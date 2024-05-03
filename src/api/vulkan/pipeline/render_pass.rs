// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use std::fmt::Debug;

use crate::api::vulkan::*;
use crate::*;
use vulkan_sys::*;

use crate::util::stack_vec::*;

struct RenderPassOwnership {
    handle: VkRenderPass,
    device: VulkanDevice,
}

impl Drop for RenderPassOwnership {
    fn drop(&mut self) {
        wrapper::destroy_render_pass(vkDestroyRenderPass, self.device.handle(), self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanRenderPass {
    handle: VkRenderPass,
    ownership: Ownership<RenderPassOwnership>,
}

impl std::fmt::Debug for VulkanRenderPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanRenderPass {
    type Handle = VkRenderPass;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanDeviceObject for VulkanRenderPass {
    fn device(&self) -> &VulkanDevice {
        &self.ownership.device
    }
}

const MAX_ATTACHMENTS: usize = 16;
const MAX_SUBPASSES: usize = 16;
const MAX_ATTACHMENTS_PER_SUBPASS: usize = 16;

impl crate::api::traits::RenderPass<VulkanApi> for VulkanRenderPass {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: RenderPassCreateInfo,
    ) -> Result<Self> {
        let attachments: Vec<_> = create_info
            .attachments
            .iter()
            .cloned()
            .map(Into::into)
            .collect();

        let input_attachments: Vec<Vec<VkAttachmentReference>> = create_info
            .subpasses
            .iter()
            .map(|subpass| {
                subpass
                    .input_attachments
                    .iter()
                    .map(|attachment| attachment.clone().into())
                    .collect()
            })
            .collect();

        let color_attachments: Vec<Vec<VkAttachmentReference>> = create_info
            .subpasses
            .iter()
            .map(|subpass| {
                subpass
                    .color_attachments
                    .iter()
                    .map(|attachment| attachment.clone().into())
                    .collect()
            })
            .collect();

        // let resolve_attachments: Vec<Vec<VkAttachmentReference>> = create_info
        //     .subpasses
        //     .iter()
        //     .map(|subpass| {
        //         subpass
        //             .resolve_attachments
        //             .iter()
        //             .map(|attachment| attachment.clone().into())
        //             .collect()
        //     })
        //     .collect();

        // let depth_stencil_attachments: Vec<Option<VkAttachmentReference>> = create_info
        //     .subpasses
        //     .iter()
        //     .map(|subpass| {
        //         subpass
        //             .depth_stencil_attachment
        //             .as_ref()
        //             .map(|attachment| attachment.clone().into())
        //     })
        //     .collect();

        let subpasses: Vec<_> = create_info
            .subpasses
            .iter()
            .enumerate()
            .map(|(i, subpass)| VkSubpassDescription {
                flags: 0,
                pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
                inputAttachmentCount: input_attachments[i].len() as u32,
                pInputAttachments: input_attachments[i].as_ptr(),
                colorAttachmentCount: color_attachments[i].len() as u32,
                pColorAttachments: color_attachments[i].as_ptr(),
                pResolveAttachments: std::ptr::null(),
                pDepthStencilAttachment: std::ptr::null(),
                preserveAttachmentCount: 0,
                pPreserveAttachments: std::ptr::null(),
            })
            .collect();

        let create_info = VkRenderPassCreateInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            subpassCount: subpasses.len() as u32,
            pSubpasses: subpasses.as_ptr(),
            dependencyCount: 0,
            pDependencies: std::ptr::null(),
        };

        let handle =
            wrapper::create_render_pass(vkCreateRenderPass, context.handle(), &create_info, None)?;

        let ownership = Ownership::new(RenderPassOwnership {
            handle,
            device: context,
        });

        Ok(Self { handle, ownership })
    }
}

impl Into<VkAttachmentDescription> for AttachmentDescription {
    fn into(self) -> VkAttachmentDescription {
        VkAttachmentDescription {
            flags: 0,
            format: self.format.into(),
            samples: VK_SAMPLE_COUNT_1_BIT,
            loadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            storeOp: VK_ATTACHMENT_STORE_OP_STORE,
            stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
            finalLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        }
    }
}

impl Into<VkAttachmentReference> for AttachmentReference {
    fn into(self) -> VkAttachmentReference {
        VkAttachmentReference {
            attachment: self.attachment,
            layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        }
    }
}
