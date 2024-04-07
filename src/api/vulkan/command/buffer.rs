// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{Ownership, VulkanApi, VulkanDevice, VulkanObject};
use std::fmt::{Debug, Formatter};
pub use vulkan_sys::*;

struct VulkanCommandBufferOwnership {
    handle: VkCommandBuffer,
    device: VulkanDevice,
}

#[derive(Clone)]
pub struct VulkanCommandBuffer {
    handle: VkCommandBuffer,
    ownership: Ownership<VulkanCommandBufferOwnership>,
}

impl Debug for VulkanCommandBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanCommandBuffer {
    type Handle = VkCommandBuffer;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl crate::api::traits::CommandBuffer<VulkanApi> for VulkanCommandBuffer {}
