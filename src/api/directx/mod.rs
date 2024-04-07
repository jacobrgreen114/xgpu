// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

mod factory;

use factory::*;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
    DXGI_FORMAT_R16G16B16A16_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
};

mod adapter;
use adapter::*;

mod device;
use device::*;

mod queue;
use queue::*;

mod surface;
use surface::*;

mod swapchain;
use crate::RenderTargetFormat;
use swapchain::*;

pub struct DirectXApi;

impl crate::api::traits::GraphicsApi for DirectXApi {
    type Root = factory::DirectXFactory;
    type Device = adapter::DirectXAdapter;
    type DeviceProperties = adapter::DirectXAdapterDescription;
    // type DeviceFeatures = ();
    type Surface = surface::DirectXSurface;
    // type SurfaceCapabilities = ();
    type Context = device::DirectXDevice;
    type Queue = queue::DirectXCommandQueue;
    type Swapchain = swapchain::DirectXSwapchain;
    // type ShaderCode<'a> = ()<'a>;
    // type Shader = ();
    // type VertexInputState = ();
    // type GraphicsPipeline = ();

    const FORMAT_R8G8B8A8_UNORM: i32 = DXGI_FORMAT_R8G8B8A8_UNORM.0;
    const FORMAT_R8G8B8A8_UNORM_SRGB: i32 = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.0;
    const FORMAT_B8G8R8A8_UNORM: i32 = DXGI_FORMAT_B8G8R8A8_UNORM.0;
    const FORMAT_B8G8R8A8_UNORM_SRGB: i32 = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB.0;

    const FORMAT_R16G16B16A16_FLOAT: i32 = DXGI_FORMAT_R16G16B16A16_FLOAT.0;
}

impl Into<DXGI_FORMAT> for RenderTargetFormat {
    fn into(self) -> DXGI_FORMAT {
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

pub(crate) trait DirectXDeviceObject: DirectXFactoryObject {
    fn device(&self) -> &DirectXDevice;
}
