// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use crate::api::vulkan::*;
use std::any::type_name;
use std::fmt::{Debug, Formatter};

use crate::convert::MapInto;
use crate::Extent2D;
use vulkan_sys::*;

/*
   Physical Device
*/

struct PhysicalDeviceOwnership {
    handle: VkPhysicalDevice,
    properties: VulkanPhysicalDeviceProperties,
    // features: VulkanPhysicalDeviceFeatures,
}

#[derive(Clone)]
pub struct VulkanPhysicalDevice {
    handle: VkPhysicalDevice,
    ownership: Ownership<PhysicalDeviceOwnership>,
}

impl Debug for VulkanPhysicalDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .field("properties", &self.ownership.properties)
            // .field("features", &self.ownership.features)
            .finish()
    }
}

impl VulkanObject for VulkanPhysicalDevice {
    type Handle = VkPhysicalDevice;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanPhysicalDevice {
    pub(crate) fn new(
        handle: VkPhysicalDevice,
        properties: VkPhysicalDeviceProperties,
        features: VkPhysicalDeviceFeatures,
    ) -> Self {
        let ownership = Ownership::new(PhysicalDeviceOwnership {
            handle,
            properties: properties.into(),
            // features: features.into(),
        });

        Self { handle, ownership }
    }
}

impl crate::api::traits::Device<VulkanApi> for VulkanPhysicalDevice {
    fn properties(&self) -> &<VulkanApi as GraphicsApi>::DeviceProperties {
        &self.ownership.properties
    }

    // fn features(&self) -> &<VulkanApi as GraphicsApi>::DeviceFeatures {
    //     &self.ownership.features
    // }

    fn supports_surface(&self, surface: <VulkanApi as GraphicsApi>::Surface) -> bool {
        // todo : implement queue family indexing
        wrapper::get_physical_device_surface_support_khr(
            vkGetPhysicalDeviceSurfaceSupportKHR,
            self.handle(),
            0,
            surface.handle(),
        )
        .unwrap()
    }

    fn get_surface_capabilities(
        &self,
        surface: <VulkanApi as GraphicsApi>::Surface,
    ) -> crate::Result<VulkanSurfaceCapabilities> {
        Ok(wrapper::get_physical_device_surface_capabilities_khr(
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
            self.handle(),
            surface.handle(),
        )
        .map_into()?)
        //.map(|capabilities| capabilities.into())?)
    }

    fn get_surface_formats(
        &self,
        surface: <VulkanApi as GraphicsApi>::Surface,
    ) -> crate::Result<Vec<crate::SurfaceFormat>> {
        Ok(wrapper::get_physical_device_surface_formats_khr(
            vkGetPhysicalDeviceSurfaceFormatsKHR,
            self.handle(),
            surface.handle(),
        )
        .map(|formats| formats.into_iter().map(|sf| sf.into()).collect())?)
    }

    fn get_surface_present_modes(
        &self,
        surface: <VulkanApi as GraphicsApi>::Surface,
    ) -> crate::Result<Vec<crate::PresentMode>> {
        Ok(wrapper::get_physical_device_surface_present_modes_khr(
            vkGetPhysicalDeviceSurfacePresentModesKHR,
            self.handle(),
            surface.handle(),
        )
        .map(|formats| formats.into_iter().map(|pm| pm.into()).collect())?)
    }
}

/*
   Physical Device Properties
*/

pub struct VulkanPhysicalDeviceProperties {
    native: VkPhysicalDeviceProperties,
}

impl From<VkPhysicalDeviceProperties> for VulkanPhysicalDeviceProperties {
    fn from(native: VkPhysicalDeviceProperties) -> Self {
        Self { native }
    }
}

impl Debug for VulkanPhysicalDeviceProperties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.native.fmt(f)
    }
}

impl crate::api::traits::DeviceProperties<VulkanApi> for VulkanPhysicalDeviceProperties {
    fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr(self.native.deviceName.as_ptr())
                .to_str()
                .unwrap()
        }
    }

    fn device_type(&self) -> crate::DeviceType {
        unsafe { std::mem::transmute(self.native.deviceType) }
    }

    fn vendor(&self) -> crate::Vendor {
        unsafe { std::mem::transmute(self.native.vendorID) }
    }
}

/*
   Physical Device Features
*/

// pub struct VulkanPhysicalDeviceFeatures {
//     native: VkPhysicalDeviceFeatures,
// }
//
// impl From<VkPhysicalDeviceFeatures> for VulkanPhysicalDeviceFeatures {
//     fn from(native: VkPhysicalDeviceFeatures) -> Self {
//         Self { native }
//     }
// }
//
// impl Debug for VulkanPhysicalDeviceFeatures {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.native.fmt(f)
//     }
// }
//
// impl crate::api::traits::DeviceFeatures<VulkanApi> for VulkanPhysicalDeviceFeatures {}

/*
   Surface Capabilities
*/

pub struct VulkanSurfaceCapabilities {
    native: VkSurfaceCapabilitiesKHR,
}

impl From<VkSurfaceCapabilitiesKHR> for VulkanSurfaceCapabilities {
    fn from(native: VkSurfaceCapabilitiesKHR) -> Self {
        Self { native }
    }
}

impl Debug for VulkanSurfaceCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("min_image_count", &self.min_image_count())
            .field("max_image_count", &self.max_image_count())
            .field("current_extent", &self.current_extent())
            .field("min_image_extent", &self.min_image_extent())
            .field("max_image_extent", &self.max_image_extent())
            .field("max_image_array_layers", &self.max_image_array_layers())
            .finish()
    }
}

impl crate::api::traits::SurfaceCapabilities<VulkanApi> for VulkanSurfaceCapabilities {
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
