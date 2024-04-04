// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

mod device;
mod instance;
mod surface;
mod swapchain;

pub use device::*;
pub use instance::*;
pub use surface::*;
pub use swapchain::*;

use vulkan_sys::*;

use crate::api::traits::*;
use crate::*;

use std::sync::Arc;

pub struct Api;
impl GraphicsApi for Api {
    type Root = instance::Instance;
    type Device = instance::PhysicalDevice;
    type Surface = surface::Surface;
    type SurfaceCapabilities = instance::SurfaceCapabilities;
    type Context = device::Device;
    type Queue = device::Queue;
    type Swapchain = swapchain::Swapchain;
}

pub trait VulkanObject {
    type Handle;

    fn handle(&self) -> Self::Handle;
}

type Ownership<T> = Arc<T>;

impl Into<VkPresentModeKHR> for PresentMode {
    fn into(self) -> VkPresentModeKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkPresentModeKHR> for PresentMode {
    fn from(value: VkPresentModeKHR) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkCompositeAlphaFlagBitsKHR> for CompositeAlpha {
    fn into(self) -> VkCompositeAlphaFlagBitsKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<VkFormat> for Format {
    fn into(self) -> VkFormat {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkFormat> for Format {
    fn from(value: VkFormat) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkColorSpaceKHR> for Colorspace {
    fn into(self) -> VkColorSpaceKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkColorSpaceKHR> for Colorspace {
    fn from(value: VkColorSpaceKHR) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<VkSurfaceFormatKHR> for SurfaceFormat {
    fn from(value: VkSurfaceFormatKHR) -> Self {
        Self {
            format: value.format.into(),
            colorspace: value.colorSpace.into(),
        }
    }
}

impl Into<VkExtent2D> for Extent2D {
    fn into(self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.height,
        }
    }
}

impl From<VkExtent2D> for Extent2D {
    fn from(value: VkExtent2D) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
