// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{
    Ownership, VulkanApi, VulkanDevice, VulkanDeviceObject, VulkanObject, VulkanRenderPass,
};

use vulkan_sys::*;

struct VulkanFramebufferOwnership {
    handle: VkFramebuffer,
    render_pass: VulkanRenderPass,
}

impl Drop for VulkanFramebufferOwnership {
    fn drop(&mut self) {
        wrapper::destroy_framebuffer(
            vkDestroyFramebuffer,
            self.render_pass.device().handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct VulkanFramebuffer {
    handle: VkFramebuffer,
    ownership: Ownership<VulkanFramebufferOwnership>,
}

impl std::fmt::Debug for VulkanFramebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanFramebuffer {
    type Handle = VkFramebuffer;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanDeviceObject for VulkanFramebuffer {
    fn device(&self) -> &VulkanDevice {
        &self.ownership.render_pass.device()
    }
}

impl VulkanFramebuffer {
    pub fn render_pass(&self) -> &VulkanRenderPass {
        &self.ownership.render_pass
    }
}

impl crate::api::traits::Framebuffer<VulkanApi> for VulkanFramebuffer {
    fn new(
        context: <VulkanApi as crate::prelude::GraphicsApi>::Context,
        create_info: crate::FramebufferCreateInfo,
    ) -> crate::Result<Self> {
        let attachments: Vec<_> = create_info
            .attachments
            .iter()
            .map(|attachment| attachment.handle())
            .collect();

        let framebuffer_info = VkFramebufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            renderPass: create_info.render_pass.handle(),
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            width: create_info.extent.width,
            height: create_info.extent.height,
            layers: 1,
        };

        let handle = wrapper::create_framebuffer(
            vkCreateFramebuffer,
            context.handle(),
            &framebuffer_info,
            None,
        )?;

        let ownership = Ownership::new(VulkanFramebufferOwnership {
            handle,
            render_pass: create_info.render_pass,
        });

        Ok(Self { handle, ownership })
    }
}
