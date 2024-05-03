// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{Ownership, VulkanApi, VulkanDevice, VulkanDeviceObject, VulkanObject};

use vulkan_sys::*;

use crate::prelude::GraphicsApi;
use crate::CommandPoolCreateInfo;
use std::fmt::Debug;

struct VulkanCommandPoolOwnership {
    handle: VkCommandPool,
    device: VulkanDevice,
}

impl Drop for VulkanCommandPoolOwnership {
    fn drop(&mut self) {
        wrapper::destroy_command_pool(
            vkDestroyCommandPool,
            self.device.handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct VulkanCommandPool {
    handle: VkCommandPool,
    ownership: Ownership<VulkanCommandPoolOwnership>,
}

impl Debug for VulkanCommandPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanCommandPool {
    type Handle = VkCommandPool;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanDeviceObject for VulkanCommandPool {
    fn device(&self) -> &VulkanDevice {
        &self.ownership.device
    }
}

impl crate::api::traits::CommandPool<VulkanApi> for VulkanCommandPool {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: CommandPoolCreateInfo,
    ) -> crate::Result<Self> {
        let mut flags = {
            let mut f = 0;
            create_info
                .transient
                .then(|| f |= VK_COMMAND_POOL_CREATE_TRANSIENT_BIT);
            create_info
                .reset
                .then(|| f |= VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT);
            f as VkCommandPoolCreateFlags
        };

        let create_info = VkCommandPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: std::ptr::null(),
            flags,
            queueFamilyIndex: 0,
        };

        let handle = wrapper::create_command_pool(
            vkCreateCommandPool,
            context.handle(),
            &create_info,
            None,
        )?;

        let ownership = Ownership::new(VulkanCommandPoolOwnership {
            handle,
            device: context.clone(),
        });

        Ok(VulkanCommandPool { handle, ownership })
    }
}
