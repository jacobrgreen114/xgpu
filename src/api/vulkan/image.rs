// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::*;
use crate::prelude::GraphicsApi;
use crate::ImageViewCreateInfo;
use std::fmt::{Debug, Formatter};
use vulkan_sys::*;

/*
   Image
*/

struct VulkanImageOwnership {
    handle: VkImage,
}

#[derive(Clone)]
pub struct VulkanImage {
    handle: VkImage,
    // ownership: Ownership<VulkanSwapchainImageOwnership>,
}

impl VulkanImage {
    pub(crate) fn swapchain(handle: VkImage) -> Self {
        Self { handle }
    }
}

impl Debug for VulkanImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanImage {
    type Handle = VkImage;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl crate::api::traits::Image<VulkanApi> for VulkanImage {}

/*
   Image View
*/

struct VulkanImageViewOwnership {
    handle: VkImageView,
    image: VulkanImage,
    device: VulkanDevice,
}

impl Drop for VulkanImageViewOwnership {
    fn drop(&mut self) {
        wrapper::destroy_image_view(vkDestroyImageView, self.device.handle(), self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanImageView {
    handle: VkImageView,
    ownership: Ownership<VulkanImageViewOwnership>,
}

impl Debug for VulkanImageView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanImageView {
    type Handle = VkImageView;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl VulkanDeviceObject for VulkanImageView {
    fn device(&self) -> &VulkanDevice {
        &self.ownership.device
    }
}

impl crate::api::traits::ImageView<VulkanApi> for VulkanImageView {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        image: <VulkanApi as GraphicsApi>::Image,
        create_info: ImageViewCreateInfo,
    ) -> crate::Result<Self> {
        let info = VkImageViewCreateInfo {
            sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            image: image.handle(),
            viewType: VK_IMAGE_VIEW_TYPE_2D,
            format: create_info.format.into(),
            components: VkComponentMapping {
                r: VK_COMPONENT_SWIZZLE_IDENTITY,
                g: VK_COMPONENT_SWIZZLE_IDENTITY,
                b: VK_COMPONENT_SWIZZLE_IDENTITY,
                a: VK_COMPONENT_SWIZZLE_IDENTITY,
            },
            subresourceRange: VkImageSubresourceRange {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT as _,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };

        let handle = wrapper::create_image_view(vkCreateImageView, context.handle(), &info, None)?;

        let ownership = Ownership::new(VulkanImageViewOwnership {
            handle,
            image,
            device: context,
        });

        Ok(VulkanImageView { handle, ownership })
    }
}
