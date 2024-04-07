// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::{DirectXApi, DirectXObject};
use crate::prelude::GraphicsApi;
use crate::SurfaceCreateInfo;

#[derive(Debug, Clone)]
pub struct DirectXSurface {
    hwnd: windows::Win32::Foundation::HWND,
}

impl DirectXObject for DirectXSurface {
    type Type = windows::Win32::Foundation::HWND;

    fn handle(&self) -> &Self::Type {
        &self.hwnd
    }
}

impl crate::api::traits::Surface<DirectXApi> for DirectXSurface {
    fn new(
        root: <DirectXApi as GraphicsApi>::Root,
        create_info: SurfaceCreateInfo,
    ) -> crate::Result<Self> {
        Ok(Self {
            hwnd: create_info.hwnd,
        })
    }
}
