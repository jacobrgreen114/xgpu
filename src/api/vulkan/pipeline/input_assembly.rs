// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{VulkanApi, VulkanType};
use vulkan_sys::*;

pub struct VulkanPipelineInputAssemblyStateCreateInfo {
    native: VkPipelineInputAssemblyStateCreateInfo,
}

impl Default for VulkanPipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        Self {
            native: VkPipelineInputAssemblyStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                topology: VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                primitiveRestartEnable: VK_FALSE,
            },
        }
    }
}

impl std::fmt::Debug for VulkanPipelineInputAssemblyStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.native.fmt(f)
    }
}

impl VulkanType for VulkanPipelineInputAssemblyStateCreateInfo {
    type Type = VkPipelineInputAssemblyStateCreateInfo;

    fn native(&self) -> &Self::Type {
        &self.native
    }
}

impl crate::api::traits::InputAssemblyStateCreateInfo<VulkanApi>
    for VulkanPipelineInputAssemblyStateCreateInfo
{
}
