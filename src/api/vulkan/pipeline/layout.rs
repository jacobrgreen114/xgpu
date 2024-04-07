// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use std::fmt::Debug;

use crate::api::vulkan::*;
use crate::*;
use vulkan_sys::*;

struct VulkanPipelineLayoutOwnership {
    handle: VkPipelineLayout,
    device: VulkanDevice,
}

impl Drop for VulkanPipelineLayoutOwnership {
    fn drop(&mut self) {
        vk::destroy_pipeline_layout(
            vkDestroyPipelineLayout,
            self.device.handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct VulkanPipelineLayout {
    handle: VkPipelineLayout,
    ownership: Ownership<VulkanPipelineLayoutOwnership>,
}

impl VulkanObject for VulkanPipelineLayout {
    type Handle = VkPipelineLayout;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl Debug for VulkanPipelineLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl crate::api::traits::PipelineLayout<VulkanApi> for VulkanPipelineLayout {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: PipelineLayoutCreateInfo,
    ) -> crate::Result<Self> {
        let create_info = VkPipelineLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            setLayoutCount: 0,
            pSetLayouts: std::ptr::null(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: std::ptr::null(),
        };

        let handle = vk::create_pipeline_layout(
            vkCreatePipelineLayout,
            context.handle(),
            &create_info,
            None,
        )?;

        let ownership = Ownership::new(VulkanPipelineLayoutOwnership {
            handle,
            device: context,
        });

        Ok(Self { handle, ownership })
    }
}
