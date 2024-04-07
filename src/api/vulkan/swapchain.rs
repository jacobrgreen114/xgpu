// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{device, surface, Ownership, VulkanApi, VulkanObject};
use crate::prelude::GraphicsApi;
use crate::{PresentMode, SwapchainCreateInfo};
use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};

use vulkan_sys::*;

struct SwapchainOwnership {
    handle: VkSwapchainKHR,
    surface: surface::VulkanSurface,
    device: device::VulkanDevice,
}

impl Drop for SwapchainOwnership {
    fn drop(&mut self) {
        vk::destroy_swapchain_khr(
            vkDestroySwapchainKHR,
            self.device.handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct VulkanSwapchain {
    handle: VkSwapchainKHR,
    ownership: Ownership<SwapchainOwnership>,
}

impl Debug for VulkanSwapchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl crate::api::traits::Swapchain<VulkanApi> for VulkanSwapchain {
    fn new(
        surface: <VulkanApi as GraphicsApi>::Surface,
        context: <VulkanApi as GraphicsApi>::Context,
        create_info: &SwapchainCreateInfo,
    ) -> crate::Result<Self> {
        let create_info = VkSwapchainCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: null(),
            flags: 0,
            surface: surface.handle(),
            minImageCount: create_info.min_image_count,
            imageFormat: create_info.format.into(),
            imageColorSpace: create_info.colorspace.into(),
            imageExtent: create_info.extent.into(),
            imageArrayLayers: 1,
            imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as _,
            imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null(),
            preTransform: VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
            compositeAlpha: create_info.composite_alpha.into(),
            presentMode: create_info.present_mode.into(),
            clipped: 0,
            oldSwapchain: null_mut(),
        };

        let handle =
            vk::create_swapchain_khr(vkCreateSwapchainKHR, context.handle(), &create_info, None)?;

        // todo : do something with swapchain images
        let images =
            vk::get_swapchain_images_khr(vkGetSwapchainImagesKHR, context.handle(), handle)?;

        let ownership = Ownership::new(SwapchainOwnership {
            handle,
            surface,
            device: context,
        });

        Ok(VulkanSwapchain { handle, ownership })
    }
}

impl VulkanObject for VulkanSwapchain {
    type Handle = VkSwapchainKHR;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}
