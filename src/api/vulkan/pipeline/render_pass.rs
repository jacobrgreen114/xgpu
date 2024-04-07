// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use std::fmt::Debug;

use crate::api::vulkan::*;
use crate::*;
use vulkan_sys::*;

struct RenderPassOwnership {
    handle: VkRenderPass,
    device: VulkanDevice,
}

impl Drop for RenderPassOwnership {
    fn drop(&mut self) {
        vk::destroy_render_pass(vkDestroyRenderPass, self.device.handle(), self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanRenderPass {
    handle: VkRenderPass,
    ownership: Ownership<RenderPassOwnership>,
}

impl Debug for VulkanRenderPass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl crate::api::traits::RenderPass<VulkanApi> for VulkanRenderPass {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: RenderPassCreateInfo,
    ) -> Result<Self> {
        let attachments = [VkAttachmentDescription {
            flags: 0,
            format: VK_FORMAT_R8G8B8A8_UNORM,
            samples: VK_SAMPLE_COUNT_1_BIT,
            loadOp: VK_ATTACHMENT_LOAD_OP_CLEAR,
            storeOp: VK_ATTACHMENT_STORE_OP_STORE,
            stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
            finalLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        }];

        let color_attachments = [VkAttachmentReference {
            attachment: 0,
            layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpasses = [VkSubpassDescription {
            flags: 0,
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: 0,
            pInputAttachments: std::ptr::null(),
            colorAttachmentCount: color_attachments.len() as u32,
            pColorAttachments: color_attachments.as_ptr(),
            pResolveAttachments: std::ptr::null(),
            pDepthStencilAttachment: std::ptr::null(),
            preserveAttachmentCount: 0,
            pPreserveAttachments: std::ptr::null(),
        }];

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
            vk::create_render_pass(vkCreateRenderPass, context.handle(), &create_info, None)?;

        let ownership = Ownership::new(RenderPassOwnership {
            handle,
            device: context,
        });

        Ok(Self { handle, ownership })
    }
}
