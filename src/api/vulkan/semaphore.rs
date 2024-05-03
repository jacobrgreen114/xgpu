// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::GraphicsApi;
use crate::api::vulkan::*;

use crate::SemaphoreCreateInfo;
use vulkan_sys::*;

struct VulkanSemaphoreOwnership {
    handle: VkSemaphore,
    device: VulkanDevice,
}

impl Drop for VulkanSemaphoreOwnership {
    fn drop(&mut self) {
        wrapper::destroy_semaphore(vkDestroySemaphore, self.device.handle(), self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanSemaphore {
    handle: VkSemaphore,
    ownership: Ownership<VulkanSemaphoreOwnership>,
}

impl std::fmt::Debug for VulkanSemaphore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanSemaphore {
    type Handle = VkSemaphore;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanDeviceObject for VulkanSemaphore {
    fn device(&self) -> &VulkanDevice {
        &self.ownership.device
    }
}

impl crate::api::traits::Semaphore<VulkanApi> for VulkanSemaphore {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: SemaphoreCreateInfo,
    ) -> crate::Result<Self> {
        let create_info = VkSemaphoreCreateInfo {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
        };
        let handle =
            wrapper::create_semaphore(vkCreateSemaphore, context.handle(), &create_info, None)?;

        let ownership = Ownership::new(VulkanSemaphoreOwnership {
            handle,
            device: context,
        });

        Ok(Self { handle, ownership })
    }
}
