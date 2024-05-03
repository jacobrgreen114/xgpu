// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

use crate::api::directx::*;
use crate::api::traits::*;
use crate::*;

use crate::util::FailFast;
use std::sync::Arc;

struct DirectXSwapchainData {
    context: <DirectXApi as GraphicsApi>::Context,
    surface: <DirectXApi as GraphicsApi>::Surface,
}

#[derive(Clone)]
pub struct DirectXSwapchain {
    swapchain: IDXGISwapChain1,
    data: Arc<DirectXSwapchainData>,
}

impl std::fmt::Debug for DirectXSwapchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.swapchain.as_raw())
            .finish()
    }
}

impl crate::api::traits::Swapchain<DirectXApi> for DirectXSwapchain {
    fn new(
        surface: <DirectXApi as GraphicsApi>::Surface,
        context: <DirectXApi as GraphicsApi>::Context,
        create_info: &SwapchainCreateInfo,
    ) -> crate::Result<Self> {
        let swapchain_desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: create_info.extent.width,
            Height: create_info.extent.height,
            Format: create_info.format.into(),
            Stereo: FALSE,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: create_info.min_image_count,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED, // todo: figure out alpha mode
            Flags: 0,
        };

        let swapchain = unsafe {
            context.factory().handle().CreateSwapChainForHwnd(
                context.queues()[0].handle(),
                *surface.handle(),
                &swapchain_desc,
                None,
                None,
            )
        }
        .fail_fast()?;

        let data = Arc::new(DirectXSwapchainData { context, surface });

        Ok(Self { swapchain, data })
    }
}
