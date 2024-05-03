// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use crate::api::vulkan::{
    Ownership, VulkanApi, VulkanInstance, VulkanInstanceObject, VulkanObject, VulkanPhysicalDevice,
};
use crate::ContextCreateInfo;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};
use std::sync::Weak;

use vulkan_sys::*;

struct VulkanDeviceOwnership {
    handle: VkDevice,
    instance: VulkanInstance,
    physical_device: VulkanPhysicalDevice,
    queues: Vec<<VulkanApi as GraphicsApi>::Queue>,
}

impl Drop for VulkanDeviceOwnership {
    fn drop(&mut self) {
        wrapper::destroy_device(vkDestroyDevice, self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanDevice {
    handle: VkDevice,
    ownership: Ownership<VulkanDeviceOwnership>,
}

impl Debug for VulkanDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanDevice {
    type Handle = VkDevice;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanInstanceObject for VulkanDevice {
    fn instance(&self) -> &VulkanInstance {
        &self.ownership.instance
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

impl Context<VulkanApi> for VulkanDevice {
    fn new(
        root: <VulkanApi as GraphicsApi>::Root,
        device: <VulkanApi as GraphicsApi>::Device,
        _create_info: ContextCreateInfo,
    ) -> crate::Result<Self> {
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

        let handle = wrapper::create_device(vkCreateDevice, device.handle(), &create_info, None)?;

        let queue = wrapper::get_device_queue(vkGetDeviceQueue, handle, 0, 0);

        let ownership = Ownership::new_cyclic(|weak| VulkanDeviceOwnership {
            handle,
            instance: root,
            physical_device: device,
            queues: vec![<VulkanApi as GraphicsApi>::Queue::new(queue, weak.clone())],
        });

        Ok(VulkanDevice { handle, ownership })
    }

    fn queues(&self) -> &[<VulkanApi as GraphicsApi>::Queue] {
        self.ownership.queues.as_slice()
    }
}

/*
   Queue
*/

struct QueueOwnership {
    handle: VkQueue,
    device: Weak<VulkanDeviceOwnership>,
}

#[derive(Clone)]
pub struct VulkanQueue {
    handle: VkQueue,
    ownership: Ownership<QueueOwnership>,
}

impl VulkanQueue {
    fn new(handle: VkQueue, device: Weak<VulkanDeviceOwnership>) -> Self {
        let ownership = Ownership::new(QueueOwnership { handle, device });

        Self { handle, ownership }
    }
}

impl Debug for VulkanQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl crate::api::traits::Queue<VulkanApi> for VulkanQueue {}

impl VulkanObject for VulkanQueue {
    type Handle = VkQueue;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}
