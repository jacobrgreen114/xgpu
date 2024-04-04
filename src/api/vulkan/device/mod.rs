// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use crate::api::vulkan::{Api, Instance, Ownership, PhysicalDevice, VulkanObject};
use crate::ContextCreateInfo;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};
use std::sync::Weak;

use vulkan_sys::*;

struct DeviceOwnership {
    handle: VkDevice,
    instance: Instance,
    physical_device: PhysicalDevice,
    queues: Vec<<Api as GraphicsApi>::Queue>,
}

impl Drop for DeviceOwnership {
    fn drop(&mut self) {
        vk::destroy_device(vkDestroyDevice, self.handle, None);
    }
}

#[derive(Clone)]
pub struct Device {
    handle: VkDevice,
    ownership: Ownership<DeviceOwnership>,
}

impl Debug for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for Device {
    type Handle = VkDevice;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

fn get_device_layers() -> Vec<*const std::ffi::c_char> {
    Vec::new()
}

fn get_device_extensions() -> Vec<*const std::ffi::c_char> {
    let mut vec: Vec<*const std::ffi::c_char> = Vec::new();
    vec.reserve(8);

    #[cfg(feature = "graphics")]
    {
        vec.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr() as *const _);
    }

    vec
}

impl Context<Api> for Device {
    fn new(
        root: <Api as GraphicsApi>::Root,
        device: <Api as GraphicsApi>::Device,
        _create_info: ContextCreateInfo,
    ) -> Result<Self, ()> {
        let layers = get_device_layers();
        let extensions = get_device_extensions();

        let graphics_queue_family = 0;
        let graphics_queue_priorities: &[f32] = &[1.0];

        let queue_create_infos = &[VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            queueFamilyIndex: graphics_queue_family,
            queueCount: graphics_queue_priorities.len() as u32,
            pQueuePriorities: graphics_queue_priorities.as_ptr(),
        }];

        let features: VkPhysicalDeviceFeatures = unsafe { std::mem::zeroed() };

        let create_info = VkDeviceCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            queueCreateInfoCount: queue_create_infos.len() as u32,
            pQueueCreateInfos: queue_create_infos.as_ptr(),
            enabledLayerCount: layers.len() as u32,
            ppEnabledLayerNames: layers.as_ptr(),
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: extensions.as_ptr(),
            pEnabledFeatures: &features,
        };

        let handle = vk::create_device(vkCreateDevice, device.handle(), &create_info, None)
            .map_err(|_| {})?;

        let queue = vk::get_device_queue(vkGetDeviceQueue, handle, 0, 0);

        let ownership = Ownership::new_cyclic(|weak| DeviceOwnership {
            handle,
            instance: root,
            physical_device: device,
            queues: vec![<Api as GraphicsApi>::Queue::new(queue, weak.clone())],
        });

        Ok(Device { handle, ownership })
    }

    fn queues(&self) -> &[<Api as GraphicsApi>::Queue] {
        self.ownership.queues.as_slice()
    }
}

/*
   Queue
*/

struct QueueOwnership {
    handle: VkQueue,
    device: Weak<DeviceOwnership>,
}

#[derive(Clone)]
pub struct Queue {
    handle: VkQueue,
    ownership: Ownership<QueueOwnership>,
}

impl Queue {
    fn new(handle: VkQueue, device: Weak<DeviceOwnership>) -> Self {
        let ownership = Ownership::new(QueueOwnership { handle, device });

        Self { handle, ownership }
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl crate::api::traits::Queue<Api> for Queue {}

impl VulkanObject for Queue {
    type Handle = VkQueue;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}
