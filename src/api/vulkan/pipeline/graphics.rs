// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::*;
use crate::prelude::GraphicsApi;

use vulkan_sys::*;

use crate::BlendAttachmentState;
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
            wrapper::destroy_pipeline(vkDestroyPipeline, self.device.handle(), self.handle, None);
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
        let handles = wrapper::create_graphics_pipelines(
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
        let shader_stages = collect_shader_stages(&create_info.shaders);

        let input_state = VkPipelineVertexInputStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            vertexBindingDescriptionCount: 0,
            pVertexBindingDescriptions: std::ptr::null(),
            vertexAttributeDescriptionCount: 0,
            pVertexAttributeDescriptions: std::ptr::null(),
        };

        let input_assembly_state = VkPipelineInputAssemblyStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            topology: create_info.topology.into(),
            primitiveRestartEnable: VK_FALSE,
        };

        let viewport_state = VkPipelineViewportStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            viewportCount: 1,
            pViewports: std::ptr::null(),
            scissorCount: 1,
            pScissors: std::ptr::null(),
        };

        let rasterization_state = VkPipelineRasterizationStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            depthClampEnable: VK_FALSE,
            rasterizerDiscardEnable: VK_FALSE,
            polygonMode: create_info.rasterization.polygon_mode.into(),
            cullMode: create_info.rasterization.cull_mode.into(),
            frontFace: create_info.rasterization.front_face.into(),
            depthBiasEnable: VK_FALSE,
            depthBiasConstantFactor: 0.0,
            depthBiasClamp: 0.0,
            depthBiasSlopeFactor: 0.0,
            lineWidth: 1.0,
        };

        let multisample_state = VkPipelineMultisampleStateCreateInfo {
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

        let depth_stencil_state = VkPipelineDepthStencilStateCreateInfo {
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

        let blend_attachments: Vec<VkPipelineColorBlendAttachmentState> = create_info
            .blend
            .attachments
            .iter()
            .cloned()
            .map(Into::into)
            .collect();

        let color_blend_state = VkPipelineColorBlendStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            logicOpEnable: VK_FALSE,
            logicOp: VK_LOGIC_OP_CLEAR,
            attachmentCount: blend_attachments.len() as u32,
            pAttachments: blend_attachments.as_ptr(),
            blendConstants: [1.0; 4],
        };

        let dynamic_state = VkPipelineDynamicStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            dynamicStateCount: 2,
            pDynamicStates: [VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR].as_ptr(),
        };

        let info = VkGraphicsPipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            stageCount: shader_stages.len() as u32,
            pStages: shader_stages.as_ptr(),
            pVertexInputState: &input_state,
            pInputAssemblyState: &input_assembly_state,
            pTessellationState: std::ptr::null(),
            pViewportState: &viewport_state,
            pRasterizationState: &rasterization_state,
            pMultisampleState: &multisample_state,
            pDepthStencilState: &depth_stencil_state,
            pColorBlendState: &color_blend_state,
            pDynamicState: &dynamic_state,
            layout: create_info.layout.handle(),
            renderPass: create_info.render_pass.handle(),
            subpass: create_info.subpass,
            basePipelineHandle: std::ptr::null_mut(),
            basePipelineIndex: 0,
        };

        let pipeline = Self::create_pipelines(context, &[info])?.pop().unwrap();

        Ok(pipeline)
    }
}

impl Into<VkPipelineColorBlendAttachmentState> for BlendAttachmentState {
    fn into(self) -> VkPipelineColorBlendAttachmentState {
        VkPipelineColorBlendAttachmentState {
            blendEnable: self.blend_enable.into(),
            srcColorBlendFactor: self.src_color_blend_factor.into(),
            dstColorBlendFactor: self.dst_color_blend_factor.into(),
            colorBlendOp: self.color_blend_op.into(),
            srcAlphaBlendFactor: self.src_alpha_blend_factor.into(),
            dstAlphaBlendFactor: self.dst_alpha_blend_factor.into(),
            alphaBlendOp: self.alpha_blend_op.into(),
            colorWriteMask: self.color_write_mask.into(),
        }
    }
}

fn create_shader_stage(
    stage: VkShaderStageFlagBits,
    shader: &VulkanShaderModule,
) -> VkPipelineShaderStageCreateInfo {
    VkPipelineShaderStageCreateInfo {
        sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        stage,
        module: shader.handle(),
        pName: c"main".as_ptr(),
        pSpecializationInfo: std::ptr::null(),
    }
}

fn collect_shader_stages(shaders: &crate::ShaderStages) -> Vec<VkPipelineShaderStageCreateInfo> {
    let mut shader_stages: Vec<VkPipelineShaderStageCreateInfo> = Vec::new();
    shader_stages.reserve(5);

    shaders.vertex.as_ref().inspect(|shader| {
        shader_stages.push(create_shader_stage(VK_SHADER_STAGE_VERTEX_BIT, shader));
    });

    shaders.tess_ctrl.as_ref().inspect(|shader| {
        shader_stages.push(create_shader_stage(
            VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
            shader,
        ));
    });

    shaders.tess_eval.as_ref().inspect(|shader| {
        shader_stages.push(create_shader_stage(
            VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
            shader,
        ));
    });

    shaders.geometry.as_ref().inspect(|shader| {
        shader_stages.push(create_shader_stage(VK_SHADER_STAGE_GEOMETRY_BIT, shader));
    });

    shaders.fragment.as_ref().inspect(|shader| {
        shader_stages.push(create_shader_stage(VK_SHADER_STAGE_FRAGMENT_BIT, shader));
    });

    shader_stages
}
