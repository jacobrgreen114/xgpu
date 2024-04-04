// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

pub mod api;

pub mod prelude {
    pub use crate::api::traits::*;
}

use prelude::*;
use vulkan_sys::{
    HINSTANCE, HWND, VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT, VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR, VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR,
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR, VK_FORMAT_B8G8R8A8_SRGB, VK_FORMAT_B8G8R8A8_UNORM,
    VK_FORMAT_R16G16B16A16_SFLOAT, VK_FORMAT_R8G8B8A8_SRGB, VK_FORMAT_R8G8B8A8_UNORM,
    VK_PRESENT_MODE_FIFO_KHR, VK_PRESENT_MODE_IMMEDIATE_KHR, VK_PRESENT_MODE_MAILBOX_KHR,
};

pub type Api = api::vulkan::Api;
pub type Root = <Api as GraphicsApi>::Root;
pub type Device = <Api as GraphicsApi>::Device;
pub type Surface = <Api as GraphicsApi>::Surface;
pub type Context = <Api as GraphicsApi>::Context;
pub type Queue = <Api as GraphicsApi>::Queue;
pub type Swapchain = <Api as GraphicsApi>::Swapchain;

#[derive(Debug, Default)]
pub struct RootCreateInfo {}

#[derive(Debug)]
#[cfg(target_os = "windows")]
pub struct SurfaceCreateInfo {
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

#[derive(Debug, Default)]
pub struct ContextCreateInfo {}

#[derive(Debug, Clone)]
pub struct SwapchainCreateInfo {
    pub min_image_count: u32,
    pub format: Format,
    pub colorspace: Colorspace,
    pub extent: Extent2D,
    pub composite_alpha: CompositeAlpha,
    pub present_mode: PresentMode,
}

type Result<T> = std::result::Result<T, ()>;

#[derive(Debug, Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default)]
pub struct DeviceQuery {}

#[derive(Debug, Copy, Clone)]
pub enum DeviceType {
    Unknown,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(u32)]
pub enum Vendor {
    Nvidia,
    Amd,
    Intel,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(i32)]
pub enum PresentMode {
    Fifo = VK_PRESENT_MODE_FIFO_KHR,
    Mailbox = VK_PRESENT_MODE_MAILBOX_KHR,
    Immediate = VK_PRESENT_MODE_IMMEDIATE_KHR,
}

impl Default for PresentMode {
    fn default() -> Self {
        PresentMode::Fifo
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(i32)]
pub enum Format {
    B8G8R8A8_UNORM = VK_FORMAT_B8G8R8A8_UNORM,
    B8G8R8A8_SRGB = VK_FORMAT_B8G8R8A8_SRGB,
    R8G8B8A8_UNORM = VK_FORMAT_R8G8B8A8_UNORM,
    R8G8B8A8_SRGB = VK_FORMAT_R8G8B8A8_SRGB,
    R16G16B16A16_SFLOAT = VK_FORMAT_R16G16B16A16_SFLOAT,
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(i32)]
pub enum Colorspace {
    SRGBNonLinear = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
    Extended_SRGB_Linear = VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT,
}

#[derive(Debug, Copy, Clone)]
pub struct SurfaceFormat {
    pub format: Format,
    pub colorspace: Colorspace,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(i32)]
pub enum CompositeAlpha {
    Opaque = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
    PreMultiplied = VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR,
    PostMultiplied = VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR,
}

impl Default for CompositeAlpha {
    fn default() -> Self {
        Self::Opaque
    }
}
