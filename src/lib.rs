// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

// todo - remove
#![allow(warnings)]

pub mod api;
pub mod convert;
mod util;

pub mod prelude {
    pub use crate::api::traits::*;
}

mod datatypes;
pub use datatypes::*;

use prelude::*;

macro_rules! get_api_type {
    ($name:tt) => {
        <crate::api::Api as crate::api::traits::GraphicsApi>::$name
    };
}

pub type Root = get_api_type!(Root);
pub type Device = get_api_type!(Device);

pub type Surface = get_api_type!(Surface);
pub type SurfaceCapabilities = get_api_type!(SurfaceCapabilities);

pub type Context = get_api_type!(Context);

pub type Queue = get_api_type!(Queue);
pub type CommandPool = get_api_type!(CommandPool);
pub type CommandBuffer = get_api_type!(CommandBuffer);

pub type Fence = get_api_type!(Fence);

// pub type CommandBufferRecordContext = <<api::Api as GraphicsApi>::CommandBuffer as api::traits::CommandBuffer<api::Api>>::RecordContext;
// pub type RenderPassRecordContext = <<<api::Api as GraphicsApi>::CommandBuffer as api::traits::CommandBuffer<api::Api>>::RecordContext as api::traits::CommandBufferRecordContext>::RenderPassRecordContext;
//
pub type Swapchain = get_api_type!(Swapchain);
pub type Image = get_api_type!(Image);
pub type ImageView = get_api_type!(ImageView);

pub type RenderPass = get_api_type!(RenderPass);
pub type Framebuffer = get_api_type!(Framebuffer);

pub type PipelineLayout = <api::Api as GraphicsApi>::PipelineLayout;

pub enum ShaderCode<'a> {
    Static(&'static [u8]),
    Dynamic(&'a [u8]),
}

impl<'a> Into<&'a [u8]> for ShaderCode<'a> {
    fn into(self) -> &'a [u8] {
        match self {
            ShaderCode::Static(code) => code,
            ShaderCode::Dynamic(code) => code,
        }
    }
}

pub type Shader = get_api_type!(Shader);

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

#[derive(Debug)]
pub struct CommandPoolCreateInfo {
    pub transient: bool,
    pub reset: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct CommandBufferAllocateInfo {
    // pub level: CommandBufferLevel,
    // pub count: u32,
}

#[derive(Debug)]
pub struct RenderPassBeginInfo {
    // pub render_pass: RenderPass,
    // pub framebuffer: Framebuffer,
    // pub render_area: Rect2D,
    // pub clear_values: Vec<ClearValue>,
}

#[derive(Debug, Clone)]
pub struct SwapchainCreateInfo {
    pub min_image_count: u32,
    pub format: Format,
    pub colorspace: Colorspace,
    pub extent: Extent2D,
    pub composite_alpha: CompositeAlphaMode,
    pub present_mode: PresentMode,
}

#[derive(Debug, Clone)]
pub struct ImageViewCreateInfo {
    // pub view_type: ImageViewType,
    pub format: Format,
    // pub components: ComponentMapping,
    // pub subresource_range: ImageSubresourceRange,
}

/*
   Render Pass Create Info
*/

#[derive(Debug, Clone)]
pub struct RenderPassCreateInfo<'a> {
    pub attachments: &'a [AttachmentDescription],
    pub subpasses: &'a [SubpassDescription<'a>],
}

#[derive(Debug, Clone)]
pub struct AttachmentDescription {
    pub format: Format,
    // pub samples: SampleCountFlags,
    // pub load_op: AttachmentLoadOp,
    // pub store_op: AttachmentStoreOp,
    // pub stencil_load_op: AttachmentLoadOp,
    // pub stencil_store_op: AttachmentStoreOp,
    // pub initial_layout: ImageLayout,
    // pub final_layout: ImageLayout,
}

#[derive(Debug, Default, Clone)]
pub struct SubpassDescription<'a> {
    pub input_attachments: &'a [AttachmentReference],
    pub color_attachments: &'a [AttachmentReference],
    // pub resolve_attachments: Vec<AttachmentReference>,
    // pub depth_stencil_attachment: Option<AttachmentReference>,
    // pub preserve_attachments: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
}

#[derive(Debug, Clone)]
pub struct FramebufferCreateInfo<'a> {
    pub render_pass: RenderPass,
    pub extent: Extent2D,
    pub attachments: &'a [ImageView],
}

#[derive(Debug, Clone)]
pub struct PipelineLayoutCreateInfo {}

#[derive(Debug, Default, Clone)]
pub struct ShaderStages {
    pub vertex: Option<Shader>,
    pub tess_ctrl: Option<Shader>,
    pub tess_eval: Option<Shader>,
    pub geometry: Option<Shader>,
    pub fragment: Option<Shader>,
}

#[derive(Debug, Default, Clone)]
pub struct RasterizationState {
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullMode,
    pub front_face: FrontFace,
}

#[derive(Debug, Default, Clone)]
pub struct BlendAttachmentState {
    pub blend_enable: bool,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

#[derive(Debug, Default, Clone)]
pub struct BlendState<'a> {
    pub attachments: &'a [BlendAttachmentState],
}

#[derive(Debug, Clone)]
pub struct GraphicsPipelineCreateInfo<'a> {
    pub shaders: ShaderStages,
    pub topology: PrimitiveTopology,
    pub rasterization: RasterizationState,
    pub blend: BlendState<'a>,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
}

#[derive(Debug, Clone)]
pub struct FenceCreateInfo {
    pub signaled: bool,
}

#[derive(Debug, Clone)]
pub struct SemaphoreCreateInfo {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "vulkan")]
    #[error("Vulkan API error: {0}")]
    ApiError(#[from] vulkan_sys::wrapper::Error),

    #[cfg(target_os = "windows")]
    #[error("Windows error: {0}")]
    WindowsError(#[from] windows::core::Error),
}
