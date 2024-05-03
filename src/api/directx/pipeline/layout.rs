// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

use crate::api::directx::*;
use crate::*;

#[derive(Clone)]
pub struct DirectXPipelineLayout {}

impl std::fmt::Debug for DirectXPipelineLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl crate::api::traits::PipelineLayout<DirectXApi> for DirectXPipelineLayout {
    fn new(
        context: directx_type!(Context),
        create_info: PipelineLayoutCreateInfo,
    ) -> crate::Result<Self> {
        Ok(Self {})
    }
}
