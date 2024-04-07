// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::*;
use crate::prelude::GraphicsApi;

use vulkan_sys::*;

use std::fmt::Debug;

/*
   Graphics Pipeline
*/

pub struct GraphicsPipelineOwnership {
    handle: VkPipeline,
    device: VulkanDevice,
}

impl Drop for GraphicsPipelineOwnership {
    fn drop(&mut self) {
        unsafe {
            vk::destroy_pipeline(vkDestroyPipeline, self.device.handle(), self.handle, None);
        }
    }
}

#[derive(Clone)]
pub struct VulkanGraphicsPipeline {
    handle: VkPipeline,
    ownership: Ownership<GraphicsPipelineOwnership>,
}

impl Debug for VulkanGraphicsPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanGraphicsPipeline {
    type Handle = VkPipeline;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

pub trait VulkanGraphicsPipelineExt: Sized {
    fn create_pipelines(
        device: VulkanDevice,
        create_info: &[VkGraphicsPipelineCreateInfo],
    ) -> crate::Result<Vec<Self>>;
}

impl VulkanGraphicsPipelineExt for VulkanGraphicsPipeline {
    fn create_pipelines(
        device: VulkanDevice,
        create_info: &[VkGraphicsPipelineCreateInfo],
    ) -> crate::Result<Vec<Self>> {
        let handles = vk::create_graphics_pipelines(
            vkCreateGraphicsPipelines,
            device.handle(),
            std::ptr::null_mut(),
            create_info,
            None,
        )?;

        Ok(handles
            .into_iter()
            .map(|handle| {
                let ownership = Ownership::new(GraphicsPipelineOwnership {
                    handle,
                    device: device.clone(),
                });

                Self { handle, ownership }
            })
            .collect())
    }
}

impl crate::api::traits::GraphicsPipeline<VulkanApi> for VulkanGraphicsPipeline {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: crate::GraphicsPipelineCreateInfo,
    ) -> crate::Result<Self> {
        let shaders: Vec<VkPipelineShaderStageCreateInfo> = create_info
            .shader_stages
            .iter()
            .map(|stage| VkPipelineShaderStageCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                stage: stage.stage.into(),
                module: stage.module.handle(),
                pName: stage.entry.as_ptr(),
                pSpecializationInfo: std::ptr::null(),
            })
            .collect();

        let create_info = VkGraphicsPipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            stageCount: shaders.len() as u32,
            pStages: shaders.as_ptr(),
            pVertexInputState: create_info.vertex_input_state.native(),
            pInputAssemblyState: create_info.input_assembly_state.native(),
            pTessellationState: std::ptr::null(),
            pViewportState: &VIEWPORT_STATE,
            pRasterizationState: create_info.rasterization_state.native(),
            pMultisampleState: &MULTISAMPLE_STATE,
            pDepthStencilState: &DEPTH_STENCIL_STATE,
            pColorBlendState: &COLOR_BLEND_STATE,
            pDynamicState: &DYNAMIC_STATE,
            layout: create_info.layout.handle(),
            renderPass: create_info.render_pass.handle(),
            subpass: 0,
            basePipelineHandle: std::ptr::null_mut(),
            basePipelineIndex: 0,
        };

        Ok(Self::create_pipelines(context, &[create_info])?
            .pop()
            .unwrap())
    }
}

static VIEWPORT_STATE: VkPipelineViewportStateCreateInfo = VkPipelineViewportStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    pNext: std::ptr::null(),
    flags: 0,
    viewportCount: 1,
    pViewports: std::ptr::null(),
    scissorCount: 1,
    pScissors: std::ptr::null(),
};

static MULTISAMPLE_STATE: VkPipelineMultisampleStateCreateInfo =
    VkPipelineMultisampleStateCreateInfo {
        sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        rasterizationSamples: VK_SAMPLE_COUNT_1_BIT,
        sampleShadingEnable: VK_FALSE,
        minSampleShading: 0.0,
        pSampleMask: std::ptr::null(),
        alphaToCoverageEnable: VK_FALSE,
        alphaToOneEnable: VK_FALSE,
    };

static DEPTH_STENCIL_STATE: VkPipelineDepthStencilStateCreateInfo =
    VkPipelineDepthStencilStateCreateInfo {
        sType: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        depthTestEnable: VK_FALSE,
        depthWriteEnable: VK_FALSE,
        depthCompareOp: VK_COMPARE_OP_NEVER,
        depthBoundsTestEnable: VK_FALSE,
        stencilTestEnable: VK_FALSE,
        front: VkStencilOpState {
            failOp: 0,
            passOp: 0,
            depthFailOp: 0,
            compareOp: 0,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        },
        back: VkStencilOpState {
            failOp: 0,
            passOp: 0,
            depthFailOp: 0,
            compareOp: 0,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        },
        minDepthBounds: 0.0,
        maxDepthBounds: 0.0,
    };

static COLOR_BLEND_ATTACHMENT: &[VkPipelineColorBlendAttachmentState] =
    &[VkPipelineColorBlendAttachmentState {
        blendEnable: VK_TRUE,
        srcColorBlendFactor: VK_BLEND_FACTOR_SRC_ALPHA,
        dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        colorBlendOp: VK_BLEND_OP_ADD,
        srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
        alphaBlendOp: VK_BLEND_OP_ADD,
        colorWriteMask: (VK_COLOR_COMPONENT_R_BIT
            | VK_COLOR_COMPONENT_G_BIT
            | VK_COLOR_COMPONENT_B_BIT
            | VK_COLOR_COMPONENT_A_BIT) as VkColorComponentFlags,
    }];

static COLOR_BLEND_STATE: VkPipelineColorBlendStateCreateInfo =
    VkPipelineColorBlendStateCreateInfo {
        sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        logicOpEnable: VK_FALSE,
        logicOp: VK_LOGIC_OP_CLEAR,
        attachmentCount: COLOR_BLEND_ATTACHMENT.len() as u32,
        pAttachments: COLOR_BLEND_ATTACHMENT.as_ptr(),
        blendConstants: [1.0; 4],
    };

static DYNAMIC_STATES: &[VkDynamicState] = &[VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR];

static DYNAMIC_STATE: VkPipelineDynamicStateCreateInfo = VkPipelineDynamicStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    pNext: std::ptr::null(),
    flags: 0,
    dynamicStateCount: DYNAMIC_STATES.len() as u32,
    pDynamicStates: DYNAMIC_STATES.as_ptr(),
};
