// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::GraphicsApi;
use crate::api::vulkan::*;

use vulkan_sys::*;

use crate::FenceCreateInfo;
use std::fmt::{Debug, Formatter};

struct FenceOwnership {
    handle: VkFence,
    device: VulkanDevice,
}

impl Drop for FenceOwnership {
    fn drop(&mut self) {
        wrapper::destroy_fence(vkDestroyFence, self.device.handle(), self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanFence {
    handle: VkFence,
    ownership: Ownership<FenceOwnership>,
}

impl Debug for VulkanFence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanFence {
    type Handle = VkFence;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanDeviceObject for VulkanFence {
    fn device(&self) -> &VulkanDevice {
        &self.ownership.device
    }
}

impl crate::api::traits::Fence<VulkanApi> for VulkanFence {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: FenceCreateInfo,
    ) -> crate::Result<Self> {
        let flags = {
            let mut flags = 0;
            create_info
                .signaled
                .then(|| flags |= VK_FENCE_CREATE_SIGNALED_BIT);
            flags
        };

        let info = VkFenceCreateInfo {
            sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: flags as VkFenceCreateFlags,
        };

        let handle = wrapper::create_fence(vkCreateFence, context.handle(), &info, None)?;

        let ownership = Ownership::new(FenceOwnership {
            handle,
            device: context,
        });

        Ok(VulkanFence { handle, ownership })
    }
}
