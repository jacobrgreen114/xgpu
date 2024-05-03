// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::GraphicsApi;
use crate::api::vulkan::{device, surface, Ownership, VulkanApi, VulkanImage, VulkanObject};
use crate::{CompositeAlphaMode, SwapchainCreateInfo};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;

use vulkan_sys::*;

struct SwapchainOwnership {
    handle: VkSwapchainKHR,
    surface: surface::VulkanSurface,
    device: device::VulkanDevice,
    images: Vec<VulkanImage>,
}

impl Drop for SwapchainOwnership {
    fn drop(&mut self) {
        wrapper::destroy_swapchain_khr(
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

impl VulkanSwapchain {
    pub fn destroy(self) -> Result<(), ()> {
        Arc::try_unwrap(self.ownership).map(|_| ()).map_err(|_| ())
    }
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
        context: <VulkanApi as GraphicsApi>::Context,
        surface: <VulkanApi as GraphicsApi>::Surface,
        create_info: &SwapchainCreateInfo,
    ) -> crate::Result<Self> {
        let create_info = VkSwapchainCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: std::ptr::null(),
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
            pQueueFamilyIndices: std::ptr::null(),
            preTransform: VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
            compositeAlpha: create_info.composite_alpha.into(),
            presentMode: create_info.present_mode.into(),
            clipped: 0,
            oldSwapchain: std::ptr::null_mut(),
        };

        let handle = wrapper::create_swapchain_khr(
            vkCreateSwapchainKHR,
            context.handle(),
            &create_info,
            None,
        )?;

        // todo : do something with swapchain images
        let image_handles =
            wrapper::get_swapchain_images_khr(vkGetSwapchainImagesKHR, context.handle(), handle)?;

        let images = image_handles
            .into_iter()
            .map(|handle| VulkanImage::swapchain(handle))
            .collect();

        let ownership = Ownership::new(SwapchainOwnership {
            handle,
            surface,
            device: context,
            images,
        });

        Ok(VulkanSwapchain { handle, ownership })
    }

    fn images(&self) -> &[<VulkanApi as GraphicsApi>::Image] {
        &self.ownership.images
    }

    fn acquire_next_image(
        &self,
        timeout: Option<Duration>,
        semaphore: Option<<VulkanApi as GraphicsApi>::Semaphore>,
        fence: Option<<VulkanApi as GraphicsApi>::Fence>,
    ) -> crate::Result<u32> {
        let timeout = timeout
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(u64::MAX);

        let semaphore = semaphore
            .map(|semaphore| semaphore.handle())
            .unwrap_or(std::ptr::null_mut());

        let fence = fence
            .map(|fence| fence.handle())
            .unwrap_or(std::ptr::null_mut());

        Ok(wrapper::acquire_next_image_khr(
            vkAcquireNextImageKHR,
            self.ownership.device.handle(),
            self.handle,
            timeout,
            Some(semaphore),
            Some(fence),
        )?)
    }
}

impl VulkanObject for VulkanSwapchain {
    type Handle = VkSwapchainKHR;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}
