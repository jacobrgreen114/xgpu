// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{Ownership, VulkanApi, VulkanInstance, VulkanObject};
use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};

use crate::prelude::GraphicsApi;
use crate::{Extent2D, SurfaceCreateInfo};
use vulkan_sys::*;

struct SurfaceOwnership {
    handle: VkSurfaceKHR,
    instance: VulkanInstance,
}

impl Drop for SurfaceOwnership {
    fn drop(&mut self) {
        wrapper::destroy_surface_khr(
            vkDestroySurfaceKHR,
            self.instance.handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct VulkanSurface {
    handle: VkSurfaceKHR,
    ownership: Ownership<SurfaceOwnership>,
}

impl Debug for VulkanSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl crate::api::traits::Surface<VulkanApi> for VulkanSurface {
    #[cfg(target_os = "windows")]
    fn new(
        root: <VulkanApi as GraphicsApi>::Root,
        create_info: SurfaceCreateInfo,
    ) -> crate::Result<Self> {
        let create_info = VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: null(),
            flags: 0,
            hinstance: create_info.hinstance,
            hwnd: create_info.hwnd,
        };

        let handle = wrapper::create_win32_surface_khr(
            vkCreateWin32SurfaceKHR,
            root.handle(),
            &create_info,
            None,
        )?;

        let ownership = Ownership::new(SurfaceOwnership {
            handle,
            instance: root,
        });

        Ok(VulkanSurface { handle, ownership })
    }
}

impl VulkanObject for VulkanSurface {
    type Handle = VkSurfaceKHR;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}
