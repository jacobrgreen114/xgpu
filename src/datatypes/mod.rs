// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

mod format;
pub use format::*;

#[derive(Debug, Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

macro_rules! api_device_type {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::DeviceTypeConstants as crate::api::traits::constants::DeviceTypeConstants>::$name
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum DeviceType {
    Other = api_device_type!(OTHER),
    IntegratedGpu = api_device_type!(INTEGRATED_GPU),
    DiscreteGpu = api_device_type!(DISCRETE_GPU),
    VirtualGpu = api_device_type!(VIRTUAL_GPU),
    Cpu = api_device_type!(CPU),
}

impl DeviceType {
    pub fn is_gpu(self) -> bool {
        match self {
            DeviceType::IntegratedGpu | DeviceType::DiscreteGpu | DeviceType::VirtualGpu => true,
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

// #[derive(Debug, Copy, Clone)]
// #[repr(i32)]
// pub enum CommandBufferLevel {
//     Primary = VK_COMMAND_BUFFER_LEVEL_PRIMARY,
//     Secondary = VK_COMMAND_BUFFER_LEVEL_SECONDARY,
// }

macro_rules! api_present_mode {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::PresentModeConstants as crate::api::traits::constants::PresentModeConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(i32)]
pub enum PresentMode {
    Immediate = api_present_mode!(IMMEDIATE),
    Mailbox = api_present_mode!(MAILBOX),
    Fifo = api_present_mode!(FIFO),
    FifoRelaxed = api_present_mode!(FIFO_RELAXED),
}

macro_rules! api_color_space {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::ColorSpaceConstants as crate::api::traits::constants::ColorSpaceConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(i32)]
pub enum Colorspace {
    SRGB_NONLINEAR = api_color_space!(SRGB_NONLINEAR),
    SRGB_EXT_LINEAR = api_color_space!(SRGB_EXT_LINEAR),
    HDR10_ST2084 = api_color_space!(HDR10_ST2084),
    HDR10_HLG = api_color_space!(HDR10_HLG),
}

impl Default for Colorspace {
    fn default() -> Self {
        Self::SRGB_NONLINEAR
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SurfaceFormat {
    pub format: Format,
    pub colorspace: Colorspace,
}

macro_rules! api_composite_alpha {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::CompositeAlphaConstants as crate::api::traits::constants::CompositeAlphaConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[repr(i32)]
pub enum CompositeAlphaMode {
    // Auto = i32::MAX,
    Opaque = api_composite_alpha!(OPAQUE),
    PreMultiplied = api_composite_alpha!(PRE_MULTIPLIED),
    PostMultiplied = api_composite_alpha!(POST_MULTIPLIED),
    Inherit = api_composite_alpha!(INHERIT),
}

impl Default for CompositeAlphaMode {
    fn default() -> Self {
        Self::Inherit
    }
}

macro_rules! api_blend_op {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::BlendOpConstants as crate::api::traits::constants::BlendOpConstants>::$name
    };
}

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum BlendOp {
    Add = api_blend_op!(ADD),
    Subtract = api_blend_op!(SUBTRACT),
    ReverseSubtract = api_blend_op!(REVERSE_SUBTRACT),
    Min = api_blend_op!(MIN),
    Max = api_blend_op!(MAX),
}

impl Default for BlendOp {
    fn default() -> Self {
        Self::Add
    }
}

macro_rules! api_blend_factor {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::BlendFactorConstants as crate::api::traits::constants::BlendFactorConstants>::$name
    };
}

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum BlendFactor {
    Zero = api_blend_factor!(ZERO),
    One = api_blend_factor!(ONE),
    SrcColor = api_blend_factor!(SRC_COLOR),
    OneMinusSrcColor = api_blend_factor!(ONE_MINUS_SRC_COLOR),
    DstColor = api_blend_factor!(DST_COLOR),
    OneMinusDstColor = api_blend_factor!(ONE_MINUS_DST_COLOR),
    SrcAlpha = api_blend_factor!(SRC_ALPHA),
    OneMinusSrcAlpha = api_blend_factor!(ONE_MINUS_SRC_ALPHA),
    DstAlpha = api_blend_factor!(DST_ALPHA),
    OneMinusDstAlpha = api_blend_factor!(ONE_MINUS_DST_ALPHA),
}

impl Default for BlendFactor {
    fn default() -> Self {
        Self::Zero
    }
}

macro_rules! api_color_component {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::ColorComponentConstants as crate::api::traits::constants::ColorComponentConstants>::$name
    };
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ColorComponentFlags: i32 {
        const R = api_color_component!(R);
        const G = api_color_component!(G);
        const B = api_color_component!(B);
        const A = api_color_component!(A);
        const ALL = api_color_component!(ALL);
    }
}

impl Default for crate::ColorComponentFlags {
    fn default() -> Self {
        Self::ALL
    }
}

macro_rules! api_polygon_mode {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::PolygonModeConstants as crate::api::traits::constants::PolygonModeConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum PolygonMode {
    Fill = api_polygon_mode!(FILL),
    Wireframe = api_polygon_mode!(LINE),
}

impl Default for PolygonMode {
    fn default() -> Self {
        Self::Fill
    }
}

macro_rules! api_cull_mode {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::CullModeConstants as crate::api::traits::constants::CullModeConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum CullMode {
    None = api_cull_mode!(NONE),
    Front = api_cull_mode!(FRONT),
    Back = api_cull_mode!(BACK),
}

impl Default for CullMode {
    fn default() -> Self {
        Self::Back
    }
}

macro_rules! api_front_face {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::FrontFaceConstants as crate::api::traits::constants::FrontFaceConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum FrontFace {
    Clockwise = api_front_face!(CLOCKWISE),
    CounterClockwise = api_front_face!(COUNTER_CLOCKWISE),
}

impl Default for FrontFace {
    fn default() -> Self {
        Self::CounterClockwise
    }
}

macro_rules! api_primitive_topology {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::PrimitiveTopologyConstants as crate::api::traits::constants::PrimitiveTopologyConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum PrimitiveTopology {
    PointList = api_primitive_topology!(POINT_LIST),
    LineList = api_primitive_topology!(LINE_LIST),
    LineStrip = api_primitive_topology!(LINE_STRIP),
    TriangleList = api_primitive_topology!(TRIANGLE_LIST),
    TriangleStrip = api_primitive_topology!(TRIANGLE_STRIP),
    TriangleFan = api_primitive_topology!(TRIANGLE_FAN),
    LineListWithAdjacency = api_primitive_topology!(LINE_LIST_WITH_ADJACENCY),
    LineStripWithAdjacency = api_primitive_topology!(LINE_STRIP_WITH_ADJACENCY),
    TriangleListWithAdjacency = api_primitive_topology!(TRIANGLE_LIST_WITH_ADJACENCY),
    TriangleStripWithAdjacency = api_primitive_topology!(TRIANGLE_STRIP_WITH_ADJACENCY),
    PatchList = api_primitive_topology!(PATCH_LIST),
}

impl Default for PrimitiveTopology {
    fn default() -> Self {
        Self::TriangleList
    }
}
