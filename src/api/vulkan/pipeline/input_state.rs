// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{VulkanApi, VulkanType};

use vulkan_sys::*;

use std::fmt::Debug;

pub struct VulkanPipelineVertexInputStateCreateInfo {
    native: VkPipelineVertexInputStateCreateInfo,
}

impl Default for VulkanPipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        Self {
            native: VkPipelineVertexInputStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                vertexBindingDescriptionCount: 0,
                pVertexBindingDescriptions: std::ptr::null(),
                vertexAttributeDescriptionCount: 0,
                pVertexAttributeDescriptions: std::ptr::null(),
            },
        }
    }
}

impl Debug for VulkanPipelineVertexInputStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.native.fmt(f)
    }
}

impl VulkanType for VulkanPipelineVertexInputStateCreateInfo {
    type Type = VkPipelineVertexInputStateCreateInfo;

    fn native(&self) -> &Self::Type {
        &self.native
    }
}

impl crate::api::traits::VertexInputStateCreateInfo<VulkanApi>
    for VulkanPipelineVertexInputStateCreateInfo
{
}
