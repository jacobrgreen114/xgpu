// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{VulkanApi, VulkanType};
use vulkan_sys::*;

pub struct VulkanPipelineRasterizationStateCreateInfo {
    native: VkPipelineRasterizationStateCreateInfo,
}

impl Default for VulkanPipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        Self {
            native: VkPipelineRasterizationStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                depthClampEnable: VK_FALSE,
                rasterizerDiscardEnable: VK_FALSE,
                polygonMode: VK_POLYGON_MODE_FILL,
                cullMode: VK_CULL_MODE_BACK_BIT as VkCullModeFlags,
                frontFace: VK_FRONT_FACE_COUNTER_CLOCKWISE,
                depthBiasEnable: VK_FALSE,
                depthBiasConstantFactor: 0.0,
                depthBiasClamp: 0.0,
                depthBiasSlopeFactor: 0.0,
                lineWidth: 1.0,
            },
        }
    }
}

impl std::fmt::Debug for VulkanPipelineRasterizationStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.native.fmt(f)
    }
}

impl VulkanType for VulkanPipelineRasterizationStateCreateInfo {
    type Type = VkPipelineRasterizationStateCreateInfo;

    fn native(&self) -> &Self::Type {
        &self.native
    }
}

impl crate::api::traits::RasterizationStateCreateInfo<VulkanApi>
    for VulkanPipelineRasterizationStateCreateInfo
{
}
