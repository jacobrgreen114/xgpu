// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

// todo - remove
#![allow(warnings)]

pub mod api;
pub mod convert;

pub mod prelude {
    pub use crate::api::traits::*;
}

use prelude::*;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};

#[cfg(feature = "vulkan")]
pub use vulkan_sys::*;

pub type Root = <api::Api as GraphicsApi>::Root;
pub type Device = <api::Api as GraphicsApi>::Device;

pub type Surface = <api::Api as GraphicsApi>::Surface;

pub type Context = <api::Api as GraphicsApi>::Context;

pub type Queue = <api::Api as GraphicsApi>::Queue;
pub type CommandPool = <api::Api as GraphicsApi>::CommandPool;
pub type CommandBuffer = <api::Api as GraphicsApi>::CommandBuffer;

pub type Swapchain = <api::Api as GraphicsApi>::Swapchain;

pub type ShaderCode<'a> = <api::Api as GraphicsApi>::ShaderCode<'a>;

pub type Shader = <api::Api as GraphicsApi>::Shader;

pub type PipelineLayout = <api::Api as GraphicsApi>::PipelineLayout;
pub type RenderPass = <api::Api as GraphicsApi>::RenderPass;

pub type VertexInputState = <api::Api as GraphicsApi>::VertexInputState;
pub type InputAssemblyState = <api::Api as GraphicsApi>::InputAssemblyState;
pub type RasterizationState = <api::Api as GraphicsApi>::RasterizationState;
pub type GraphicsPipeline = <api::Api as GraphicsApi>::GraphicsPipeline;

#[derive(Debug, Default)]
pub struct RootCreateInfo {}

#[derive(Debug)]
#[cfg(target_os = "windows")]
pub struct SurfaceCreateInfo {
    pub hwnd: windows::Win32::Foundation::HWND,
    pub hinstance: windows::Win32::Foundation::HINSTANCE,
}

impl SurfaceCreateInfo {
    pub fn new(hwnd: windows::Win32::Foundation::HWND) -> Self {
        Self {
            hwnd,
            hinstance: windows::Win32::Foundation::HINSTANCE::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ContextCreateInfo {}

#[derive(Debug, Default)]
pub struct CommandPoolCreateInfo {
    pub queue_family_index: u32,
    pub transient: bool,
    pub reset: bool,
}

#[derive(Debug, Clone)]
pub struct SwapchainCreateInfo {
    pub min_image_count: u32,
    pub format: RenderTargetFormat,
    pub colorspace: Colorspace,
    pub extent: Extent2D,
    pub composite_alpha: CompositeAlpha,
    pub present_mode: PresentMode,
}

#[derive(Debug, Clone)]
pub struct PipelineLayoutCreateInfo {}

#[derive(Debug, Clone)]
pub struct RenderPassCreateInfo {}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum ShaderStage {
    Vertex = VK_SHADER_STAGE_VERTEX_BIT,
    Fragment = VK_SHADER_STAGE_FRAGMENT_BIT,
}

#[derive(Debug, Clone)]
pub struct ShaderStageCreateInfo<'a> {
    pub module: Shader,
    pub stage: ShaderStage,
    pub entry: &'a CStr,
}

#[derive(Debug, Clone)]
pub struct GraphicsPipelineCreateInfo<'a, 'b> {
    pub shader_stages: &'a [ShaderStageCreateInfo<'b>],
    pub vertex_input_state: &'a VertexInputState,
    pub input_assembly_state: &'a InputAssemblyState,
    pub rasterization_state: &'a RasterizationState,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

// #[derive(Debug, Default)]
// pub struct DeviceQuery {}
//

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum DeviceType {
    #[cfg(feature = "vulkan")]
    Other = VK_PHYSICAL_DEVICE_TYPE_OTHER,

    #[cfg(feature = "vulkan")]
    IntegratedGpu = VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU,

    #[cfg(feature = "vulkan")]
    DiscreteGpu = VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,

    #[cfg(feature = "vulkan")]
    VirtualGpu = VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU,

    #[cfg(feature = "vulkan")]
    Cpu = VK_PHYSICAL_DEVICE_TYPE_CPU,

    #[cfg(feature = "directx")]
    Gpu,

    #[cfg(feature = "directx")]
    Software,
}

impl DeviceType {
    pub fn is_gpu(&self) -> bool {
        match self {
            #[cfg(feature = "vulkan")]
            DeviceType::IntegratedGpu | DeviceType::DiscreteGpu | DeviceType::VirtualGpu => true,
            #[cfg(feature = "directx")]
            DeviceType::Gpu => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(u32)]
pub enum Vendor {
    Nvidia = 0x10DE,
    Amd = 0x1022,
    Intel = 0x8086,
}

impl Into<u32> for Vendor {
    fn into(self) -> u32 {
        self as u32
    }
}

impl From<u32> for Vendor {
    fn from(vendor: u32) -> Self {
        unsafe { std::mem::transmute(vendor) }
    }
}

#[cfg(not(feature = "directx"))]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(i32)]
pub enum PresentMode {
    Fifo = VK_PRESENT_MODE_FIFO_KHR,
    Mailbox = VK_PRESENT_MODE_MAILBOX_KHR,
    Immediate = VK_PRESENT_MODE_IMMEDIATE_KHR,
}

#[cfg(not(feature = "directx"))]
impl Default for PresentMode {
    fn default() -> Self {
        PresentMode::Fifo
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(i32)]
pub enum RenderTargetFormat {
    R8G8B8A8_UNORM = <api::Api as GraphicsApi>::FORMAT_R8G8B8A8_UNORM,
    R8G8B8A8_UNORM_SRGB = <api::Api as GraphicsApi>::FORMAT_R8G8B8A8_UNORM_SRGB,

    B8G8R8A8_UNORM = <api::Api as GraphicsApi>::FORMAT_B8G8R8A8_UNORM,
    B8G8R8A8_UNORM_SRGB = <api::Api as GraphicsApi>::FORMAT_B8G8R8A8_UNORM_SRGB,

    R16G16B16A16_FLOAT = <api::Api as GraphicsApi>::FORMAT_R16G16B16A16_FLOAT,
}

#[cfg(not(feature = "directx"))]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(i32)]
pub enum Colorspace {
    SRGBNonLinear = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
    Extended_SRGB_Linear = VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT,
}

#[cfg(not(feature = "directx"))]
#[derive(Debug, Copy, Clone)]
pub struct SurfaceFormat {
    pub format: RenderTargetFormat,
    pub colorspace: Colorspace,
}

#[cfg(not(feature = "directx"))]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(i32)]
pub enum CompositeAlpha {
    Opaque = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
    PreMultiplied = VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR,
    PostMultiplied = VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR,
}

#[cfg(not(feature = "directx"))]
impl Default for CompositeAlpha {
    fn default() -> Self {
        Self::Opaque
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "vulkan")]
    #[error("Vulkan API error: {0}")]
    ApiError(#[from] vk::Error),

    #[cfg(feature = "directx")]
    #[error("DirectX API error: {0}")]
    WindowsError(#[from] windows::core::Error),
}
