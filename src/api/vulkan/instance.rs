// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use crate::api::vulkan::*;
use std::any::type_name;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::ptr::{null, null_mut};
use std::sync::OnceLock;
use std::sync::Weak;

use crate::prelude::ApiRoot;
use crate::{ContextCreateInfo, DeviceType, RootCreateInfo, Vendor};
use vulkan_sys::*;

struct InstanceOwnership {
    handle: VkInstance,
    physical_devices: OnceLock<Vec<<Api as GraphicsApi>::Device>>,
}

impl Drop for InstanceOwnership {
    fn drop(&mut self) {
        vk::destroy_instance(vkDestroyInstance, self.handle, None);
    }
}

#[derive(Clone)]
pub struct Instance {
    handle: VkInstance,
    ownership: Ownership<InstanceOwnership>,
}

impl Debug for Instance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

fn get_instance_layers() -> Vec<*const std::ffi::c_char> {
    let mut vec = Vec::new();

    #[cfg(feature = "gpu_debugging")]
    vec.push(VK_KHR_VALIDATION_LAYER_NAME.as_ptr() as *const _);

    vec
}

fn get_instance_extensions() -> Vec<*const std::ffi::c_char> {
    let mut vec: Vec<*const std::ffi::c_char> = Vec::new();
    vec.reserve(8);

    #[cfg(feature = "graphics")]
    {
        vec.push(VK_KHR_SURFACE_EXTENSION_NAME.as_ptr() as *const _);

        #[cfg(target_os = "windows")]
        vec.push(VK_KHR_WIN32_SURFACE_EXTENSION_NAME.as_ptr() as *const _);

        #[cfg(not(target_os = "windows"))]
        panic!("Platform not implemented");
    }

    #[cfg(feature = "gpu_debugging")]
    vec.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const _);

    vec
}

impl ApiRoot<Api> for Instance {
    fn new(_create_info: RootCreateInfo) -> Result<Self> {
        let layers = get_instance_layers();
        let extensions = get_instance_extensions();

        let instance_create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: null(),
            enabledLayerCount: layers.len() as u32,
            ppEnabledLayerNames: layers.as_ptr(),
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: extensions.as_ptr(),
        };

        let handle =
            vk::create_instance(vkCreateInstance, &instance_create_info, None).map_err(|_| {})?;

        Ok(Instance {
            handle,
            ownership: Ownership::new(InstanceOwnership {
                handle,
                physical_devices: OnceLock::new(),
            }),
        })
    }

    fn devices(&self) -> &[<Api as GraphicsApi>::Device] {
        self.ownership
            .physical_devices
            .get_or_init(|| self.enumerate_physical_device().unwrap())
            .as_slice()
    }
}

impl Instance {
    fn enumerate_physical_device(&self) -> Result<Vec<<Api as GraphicsApi>::Device>> {
        vk::enumerate_physical_device(vkEnumeratePhysicalDevices, self.handle)
            .map(|v| {
                v.into_iter()
                    .map(|handle| {
                        let properties = vk::get_physical_device_properties(
                            vkGetPhysicalDeviceProperties,
                            handle,
                        );
                        PhysicalDevice::new(handle, properties)
                    })
                    .collect()
            })
            .map_err(|e| {})
    }
}

impl VulkanObject for Instance {
    type Handle = VkInstance;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

struct PhysicalDeviceOwnership {
    handle: VkPhysicalDevice,
    properties: VkPhysicalDeviceProperties,
    name: Cell<&'static str>,
}

#[derive(Clone)]
pub struct PhysicalDevice {
    handle: VkPhysicalDevice,
    ownership: Ownership<PhysicalDeviceOwnership>,
}

impl Debug for PhysicalDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .field("name", &self.name())
            .field("type", &self.device_type())
            .field("vendor", &self.vendor())
            .finish()
    }
}

impl PhysicalDevice {
    fn new(handle: VkPhysicalDevice, properties: VkPhysicalDeviceProperties) -> Self {
        let ownership = Ownership::new(PhysicalDeviceOwnership {
            handle,
            properties,
            name: Cell::new(unsafe { std::str::from_utf8_unchecked(&[]) }),
        });

        ownership.name.set(
            unsafe { std::ffi::CStr::from_ptr(ownership.properties.deviceName.as_ptr()) }
                .to_str()
                .unwrap(),
        );

        Self { handle, ownership }
    }
}

impl crate::api::traits::Device<Api> for PhysicalDevice {
    fn name(&self) -> &str {
        self.ownership.name.get()
    }

    fn device_type(&self) -> DeviceType {
        match self.ownership.properties.deviceType {
            VK_PHYSICAL_DEVICE_TYPE_OTHER => DeviceType::Unknown,
            VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => DeviceType::IntegratedGpu,
            VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => DeviceType::DiscreteGpu,
            VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => DeviceType::VirtualGpu,
            VK_PHYSICAL_DEVICE_TYPE_CPU => DeviceType::Cpu,
            _ => panic!("Invalid device type!"),
        }
    }

    fn vendor(&self) -> Vendor {
        unsafe { std::mem::transmute(self.ownership.properties.vendorID) }
    }

    fn supports_surface(&self, surface: <Api as GraphicsApi>::Surface) -> bool {
        // todo : implement queue family indexing
        vk::get_physical_device_surface_support_khr(
            vkGetPhysicalDeviceSurfaceSupportKHR,
            self.handle(),
            0,
            surface.handle(),
        )
        .unwrap()
    }

    fn get_surface_capabilities(
        &self,
        surface: <Api as GraphicsApi>::Surface,
    ) -> crate::Result<<Api as GraphicsApi>::SurfaceCapabilities> {
        vk::get_physical_device_surface_capabilities_khr(
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
            self.handle(),
            surface.handle(),
        )
        .map(|native| SurfaceCapabilities { native })
        .map_err(|_| {})
    }

    fn get_surface_formats(
        &self,
        surface: <Api as GraphicsApi>::Surface,
    ) -> Result<Vec<SurfaceFormat>> {
        vk::get_physical_device_surface_formats_khr(
            vkGetPhysicalDeviceSurfaceFormatsKHR,
            self.handle(),
            surface.handle(),
        )
        .map(|formats| formats.into_iter().map(|sf| sf.into()).collect())
        .map_err(|_| {})
    }

    fn get_surface_present_modes(
        &self,
        surface: <Api as GraphicsApi>::Surface,
    ) -> Result<Vec<PresentMode>> {
        vk::get_physical_device_surface_present_modes_khr(
            vkGetPhysicalDeviceSurfacePresentModesKHR,
            self.handle(),
            surface.handle(),
        )
        .map(|formats| formats.into_iter().map(|pm| pm.into()).collect())
        .map_err(|_| {})
    }
}

impl VulkanObject for PhysicalDevice {
    type Handle = VkPhysicalDevice;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

pub struct SurfaceCapabilities {
    native: VkSurfaceCapabilitiesKHR,
}

impl Debug for SurfaceCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.native.fmt(f)
    }
}

impl crate::api::traits::SurfaceCapabilities<Api> for SurfaceCapabilities {
    fn min_image_count(&self) -> u32 {
        self.native.minImageCount
    }

    fn max_image_count(&self) -> u32 {
        self.native.maxImageCount
    }

    fn current_extent(&self) -> Extent2D {
        self.native.currentExtent.into()
    }

    fn min_image_extent(&self) -> Extent2D {
        self.native.minImageExtent.into()
    }

    fn max_image_extent(&self) -> Extent2D {
        self.native.maxImageExtent.into()
    }

    fn max_image_array_layers(&self) -> u32 {
        self.native.maxImageArrayLayers
    }
}
