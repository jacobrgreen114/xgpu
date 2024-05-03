// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{Ownership, VulkanApi, VulkanDevice, VulkanDeviceObject, VulkanObject};
use crate::prelude::GraphicsApi;
use crate::{CommandBufferAllocateInfo, RenderPassBeginInfo};
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

impl crate::api::traits::CommandBuffer<VulkanApi> for VulkanCommandBuffer {
    // type RecordContext = VulkanCommandBufferRecordContext;
    //
    fn allocate(
        pool: <VulkanApi as GraphicsApi>::CommandPool,
        create_info: CommandBufferAllocateInfo,
    ) -> crate::Result<Self> {
        let device = pool.device();

        let allocate_info = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: std::ptr::null(),
            commandPool: pool.handle(),
            level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
        };

        let handle = wrapper::allocate_command_buffers(
            vkAllocateCommandBuffers,
            device.handle(),
            &allocate_info,
        )?
        .pop()
        .unwrap();

        let ownership = Ownership::new(VulkanCommandBufferOwnership {
            handle,
            device: device.clone(),
        });

        Ok(Self { handle, ownership })
    }

    // fn allocate(
    //     pool: <VulkanApi as GraphicsApi>::CommandPool,
    //     create_info: CommandBufferAllocateInfo,
    // ) -> crate::Result<Vec<Self>> {
    //     let device = pool.device();
    //
    //     let allocate_info = VkCommandBufferAllocateInfo {
    //         sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    //         pNext: std::ptr::null(),
    //         commandPool: pool.handle(),
    //         level: create_info.level as _,
    //         commandBufferCount: create_info.count,
    //     };
    //
    //     let command_buffers = vk::allocate_command_buffers(
    //         vkAllocateCommandBuffers,
    //         device.handle(),
    //         &allocate_info,
    //     )?;
    //
    //     Ok(command_buffers
    //         .into_iter()
    //         .map(|handle| {
    //             let ownership = Ownership::new(VulkanCommandBufferOwnership {
    //                 handle,
    //                 device: device.clone(),
    //             });
    //
    //             Self { handle, ownership }
    //         })
    //         .collect())
    // }

    //
    // fn record<T, F>(&mut self, f: F) -> crate::Result<T>
    // where
    //     F: FnOnce(Self::RecordContext) -> T,
    // {
    //     let begin_info = VkCommandBufferBeginInfo {
    //         sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    //         pNext: std::ptr::null(),
    //         flags: 0,
    //         pInheritanceInfo: std::ptr::null(),
    //     };
    //
    //     vk::begin_command_buffer(vkBeginCommandBuffer, self.handle(), &begin_info)?;
    //
    //     let context = VulkanCommandBufferRecordContext {
    //         buffer: self.handle(),
    //     };
    //
    //     let result = f(context);
    //
    //     vk::end_command_buffer(vkEndCommandBuffer, self.handle())?;
    //
    //     Ok(result)
    // }
}

// pub struct VulkanCommandBufferRecordContext {
//     buffer: VkCommandBuffer,
// }
//
// impl crate::api::traits::CommandBufferRecordContext for VulkanCommandBufferRecordContext {
//     type RenderPassRecordContext = VulkanRenderPassRecordContext;
//
//     fn render_pass<T, F>(&self, begin_info_: RenderPassBeginInfo, f: F) -> crate::Result<T>
//     where
//         F: FnOnce(Self::RenderPassRecordContext) -> T,
//     {
//         let render_pass = begin_info_.framebuffer.render_pass();
//
//         let begin_info = VkRenderPassBeginInfo {
//             sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
//             pNext: std::ptr::null(),
//             renderPass: render_pass.handle(),
//             framebuffer: begin_info_.framebuffer.handle(),
//             renderArea: begin_info_.render_area.into(),
//             clearValueCount: 0,
//             pClearValues: std::ptr::null(),
//         };
//
//         vk::cmd_begin_render_pass(
//             vkCmdBeginRenderPass,
//             self.buffer,
//             &begin_info,
//             VK_SUBPASS_CONTENTS_INLINE,
//         );
//
//         let result = f(VulkanRenderPassRecordContext {
//             buffer: self.buffer,
//         });
//
//         vk::cmd_end_render_pass(vkCmdEndRenderPass, self.buffer);
//
//         Ok(result)
//     }
// }
//
// pub struct VulkanRenderPassRecordContext {
//     buffer: VkCommandBuffer,
// }
//
// impl crate::api::traits::RenderPassRecordContext for VulkanRenderPassRecordContext {}
