// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{Api, Instance, Ownership, VulkanObject};
use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};

use crate::prelude::GraphicsApi;
use crate::SurfaceCreateInfo;
use vulkan_sys::*;

struct SurfaceOwnership {
    handle: VkSurfaceKHR,
    instance: Instance,
}

impl Drop for SurfaceOwnership {
    fn drop(&mut self) {
        vk::destroy_surface_khr(
            vkDestroySurfaceKHR,
            self.instance.handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct Surface {
    handle: VkSurfaceKHR,
    ownership: Ownership<SurfaceOwnership>,
}

impl Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl crate::api::traits::Surface<Api> for Surface {
    #[cfg(target_os = "windows")]
    fn new(root: <Api as GraphicsApi>::Root, create_info: SurfaceCreateInfo) -> Result<Self, ()> {
        let create_info = VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: null(),
            flags: 0,
            hinstance: create_info.hinstance as *mut _,
            hwnd: create_info.hwnd as *mut _,
        };

        let handle = vk::create_win32_surface_khr(
            vkCreateWin32SurfaceKHR,
            root.handle(),
            &create_info,
            None,
        )
        .map_err(|_| {})?;

        let ownership = Ownership::new(SurfaceOwnership {
            handle,
            instance: root,
        });

        Ok(Surface { handle, ownership })
    }
}

impl VulkanObject for Surface {
    type Handle = VkSurfaceKHR;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}
