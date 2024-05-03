// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

macro_rules! directx_type {
    ($name:tt) => {
        <crate::api::directx::DirectXApi as crate::api::traits::GraphicsApi>::$name
    };
}

mod factory;

pub use factory::*;
use windows::Win32::Graphics::Direct3D12::*;

mod adapter;
pub use adapter::*;

mod surface;
pub use surface::*;

mod device;
pub use device::*;

mod queue;
pub use queue::*;

mod pipeline;
pub use pipeline::*;

mod swapchain;
pub use swapchain::*;

mod command;
pub use command::*;

mod fence;
pub use fence::*;

use windows::Win32::Graphics::Dxgi::Common::*;

pub struct DirectXApi;

impl crate::api::traits::GraphicsApi for DirectXApi {
    type Root = DirectXFactory;
    type Device = DirectXAdapter;
    type DeviceProperties = DirectXAdapterDescription;
    // type DeviceFeatures = ();
    type Surface = DirectXSurface;
    type SurfaceCapabilities = DirectXSurfaceCapabilities;
    type Context = DirectXDevice;
    type Queue = DirectXCommandQueue;
    type CommandPool = DirectXCommandAllocator;
    type CommandBuffer = DirectXCommandList;
    type Swapchain = DirectXSwapchain;

    type RenderPass = DirectXRenderPass;

    type PipelineLayout = DirectXPipelineLayout;

    //type ShaderCode<'a> = ()<'a>;
    type Shader = DirectXShader;

    // type VertexInputState = ();
    type GraphicsPipeline = DirectXGraphicsPipeline;

    type Fence = DirectXFence;

    type DeviceTypeConstants = constants::DirectXDeviceTypes;
    type CompositeAlphaConstants = constants::DirectXCompositeAlphaModes;
    type ColorSpaceConstants = constants::DirectXColorSpaces;
    type DataFormatConstants = constants::DirectXDataFormats;
    type PresentModeConstants = constants::DirectXPresentModes;
    type BlendOpConstants = constants::DirectXBlendOps;
    type BlendFactorConstants = constants::DirectXBlendFactors;
    type ColorComponentConstants = constants::DirectXColorComponentConstants;
}

mod constants {
    use windows::Win32::Graphics::Direct3D12::*;
    use windows::Win32::Graphics::Dxgi::Common::*;

    pub struct DirectXDeviceTypes;
    impl crate::api::traits::constants::DeviceTypeConstants for DirectXDeviceTypes {
        const OTHER: i32 = 0;
        const INTEGRATED_GPU: i32 = 1;
        const DISCRETE_GPU: i32 = 2;
        const VIRTUAL_GPU: i32 = 3;
        const CPU: i32 = 4;
    }

    pub struct DirectXCompositeAlphaModes;
    impl crate::api::traits::constants::CompositeAlphaConstants for DirectXCompositeAlphaModes {
        const OPAQUE: i32 = DXGI_ALPHA_MODE_IGNORE.0;
        const PRE_MULTIPLIED: i32 = DXGI_ALPHA_MODE_PREMULTIPLIED.0;
        const POST_MULTIPLIED: i32 = DXGI_ALPHA_MODE_STRAIGHT.0;
        const INHERIT: i32 = DXGI_ALPHA_MODE_UNSPECIFIED.0;
    }

    pub struct DirectXColorSpaces;
    impl crate::api::traits::constants::ColorSpaceConstants for DirectXColorSpaces {
        const SRGB_NONLINEAR: i32 = DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709.0;
        const SRGB_EXT_LINEAR: i32 = DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709.0;
        const HDR10_ST2084: i32 = DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020.0;
        const HDR10_HLG: i32 = DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020.0;
    }

    pub struct DirectXDataFormats;
    impl crate::api::traits::constants::DataFormatConstants for DirectXDataFormats {
        const R8_UINT: i32 = DXGI_FORMAT_R8_UINT.0;
        const R8_SINT: i32 = DXGI_FORMAT_R8_SINT.0;
        const R8_UNORM: i32 = DXGI_FORMAT_R8_UNORM.0;
        const R8_SNORM: i32 = DXGI_FORMAT_R8_SNORM.0;
        const R8G8_UINT: i32 = DXGI_FORMAT_R8G8_UINT.0;
        const R8G8_SINT: i32 = DXGI_FORMAT_R8G8_SINT.0;
        const R8G8_UNORM: i32 = DXGI_FORMAT_R8G8_UNORM.0;
        const R8G8_SNORM: i32 = DXGI_FORMAT_R8G8_SNORM.0;
        const R8G8B8A8_UINT: i32 = DXGI_FORMAT_R8G8B8A8_UINT.0;
        const R8G8B8A8_SINT: i32 = DXGI_FORMAT_R8G8B8A8_SINT.0;
        const R8G8B8A8_UNORM: i32 = DXGI_FORMAT_R8G8B8A8_UNORM.0;
        const R8G8B8A8_SNORM: i32 = DXGI_FORMAT_R8G8B8A8_SNORM.0;
        const R8G8B8A8_UNORM_SRGB: i32 = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.0;

        const B8G8R8A8_UNORM: i32 = DXGI_FORMAT_B8G8R8A8_UNORM.0;
        const B8G8R8A8_UNORM_SRGB: i32 = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB.0;
        const R16_UINT: i32 = DXGI_FORMAT_R16_UINT.0;
        const R16_SINT: i32 = DXGI_FORMAT_R16_SINT.0;
        const R16_UNORM: i32 = DXGI_FORMAT_R16_UNORM.0;
        const R16_SNORM: i32 = DXGI_FORMAT_R16_SNORM.0;
        const R16G16_UINT: i32 = DXGI_FORMAT_R16G16_UINT.0;
        const R16G16_SINT: i32 = DXGI_FORMAT_R16G16_SINT.0;
        const R16G16_UNORM: i32 = DXGI_FORMAT_R16G16_UNORM.0;
        const R16G16_SNORM: i32 = DXGI_FORMAT_R16G16_SNORM.0;
        const R16G16B16A16_UINT: i32 = DXGI_FORMAT_R16G16B16A16_UINT.0;
        const R16G16B16A16_SINT: i32 = DXGI_FORMAT_R16G16B16A16_SINT.0;
        const R16G16B16A16_UNORM: i32 = DXGI_FORMAT_R16G16B16A16_UNORM.0;
        const R16G16B16A16_SNORM: i32 = DXGI_FORMAT_R16G16B16A16_SNORM.0;

        const R16G16B16A16_SFLOAT: i32 = DXGI_FORMAT_R16G16B16A16_FLOAT.0;
        const R32_UINT: i32 = DXGI_FORMAT_R32_UINT.0;
        const R32_SINT: i32 = DXGI_FORMAT_R32_SINT.0;
        const R32_SFLOAT: i32 = DXGI_FORMAT_R32_FLOAT.0;
        const R32G32_UINT: i32 = DXGI_FORMAT_R32G32_UINT.0;
        const R32G32_SINT: i32 = DXGI_FORMAT_R32G32_SINT.0;
        const R32G32_SFLOAT: i32 = DXGI_FORMAT_R32G32_FLOAT.0;
        const R32G32B32_UINT: i32 = DXGI_FORMAT_R32G32B32_UINT.0;
        const R32G32B32_SINT: i32 = DXGI_FORMAT_R32G32B32_SINT.0;
        const R32G32B32_SFLOAT: i32 = DXGI_FORMAT_R32G32B32_FLOAT.0;
        const R32G32B32A32_UINT: i32 = DXGI_FORMAT_R32G32B32A32_UINT.0;
        const R32G32B32A32_SINT: i32 = DXGI_FORMAT_R32G32B32A32_SINT.0;
        const R32G32B32A32_SFLOAT: i32 = DXGI_FORMAT_R32G32B32A32_FLOAT.0;
        const R10G10B10A2_UINT: i32 = DXGI_FORMAT_R10G10B10A2_UINT.0;
        const R10G10B10A2_UNORM: i32 = DXGI_FORMAT_R10G10B10A2_UNORM.0;
    }

    pub struct DirectXPresentModes;
    impl crate::api::traits::constants::PresentModeConstants for DirectXPresentModes {
        const IMMEDIATE: i32 = 0;
        const MAILBOX: i32 = 1;
        const FIFO: i32 = 2;
        const FIFO_RELAXED: i32 = 3;
    }

    pub struct DirectXBlendOps;
    impl crate::api::traits::constants::BlendOpConstants for DirectXBlendOps {
        const ADD: i32 = D3D12_BLEND_OP_ADD.0;
        const SUBTRACT: i32 = D3D12_BLEND_OP_SUBTRACT.0;
        const REVERSE_SUBTRACT: i32 = D3D12_BLEND_OP_REV_SUBTRACT.0;
        const MIN: i32 = D3D12_BLEND_OP_MIN.0;
        const MAX: i32 = D3D12_BLEND_OP_MAX.0;
    }

    pub struct DirectXBlendFactors;
    impl crate::api::traits::constants::BlendFactorConstants for DirectXBlendFactors {
        const ZERO: i32 = D3D12_BLEND_ZERO.0;
        const ONE: i32 = D3D12_BLEND_ONE.0;
        const SRC_COLOR: i32 = D3D12_BLEND_SRC_COLOR.0;
        const ONE_MINUS_SRC_COLOR: i32 = D3D12_BLEND_INV_SRC_COLOR.0;
        const DST_COLOR: i32 = D3D12_BLEND_DEST_COLOR.0;
        const ONE_MINUS_DST_COLOR: i32 = D3D12_BLEND_INV_DEST_COLOR.0;
        const SRC_ALPHA: i32 = D3D12_BLEND_SRC_ALPHA.0;
        const ONE_MINUS_SRC_ALPHA: i32 = D3D12_BLEND_INV_SRC_ALPHA.0;
        const DST_ALPHA: i32 = D3D12_BLEND_DEST_ALPHA.0;
        const ONE_MINUS_DST_ALPHA: i32 = D3D12_BLEND_INV_DEST_ALPHA.0;
    }

    pub struct DirectXColorComponentConstants;
    impl crate::api::traits::constants::ColorComponentConstants for DirectXColorComponentConstants {
        const R: i32 = D3D12_COLOR_WRITE_ENABLE_RED.0;
        const G: i32 = D3D12_COLOR_WRITE_ENABLE_GREEN.0;
        const B: i32 = D3D12_COLOR_WRITE_ENABLE_BLUE.0;
        const A: i32 = D3D12_COLOR_WRITE_ENABLE_ALPHA.0;
        const ALL: i32 = D3D12_COLOR_WRITE_ENABLE_ALL.0;
    }
}

impl Into<DXGI_FORMAT> for crate::Format {
    fn into(self) -> DXGI_FORMAT {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<DXGI_ALPHA_MODE> for crate::CompositeAlphaMode {
    fn into(self) -> DXGI_ALPHA_MODE {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<D3D12_BLEND_OP> for crate::BlendOp {
    fn into(self) -> D3D12_BLEND_OP {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<D3D12_BLEND> for crate::BlendFactor {
    fn into(self) -> D3D12_BLEND {
        unsafe { std::mem::transmute(self) }
    }
}

pub(crate) trait DirectXObject {
    type Type;

    fn handle(&self) -> &Self::Type;
}

pub(crate) trait DirectXFactoryObject: DirectXObject {
    fn factory(&self) -> &DirectXFactory;
}

pub(crate) trait DirectXDeviceObject: DirectXObject {
    fn device(&self) -> &DirectXDevice;
}

// fn test() {
//     let dec = D3D12_ROOT_SIGNATURE_DESC {
//         NumParameters: 0,
//         pParameters: std::ptr::null(),
//         NumStaticSamplers: 0,
//         pStaticSamplers: std::ptr::null(),
//         Flags: 0,
//     };
// }
//
