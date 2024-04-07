// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::adapter::DirectXAdapter;
use crate::api::directx::DirectXObject;
use std::fmt::Debug;
use std::sync::Arc;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

struct FactoryData {
    adapters: Vec<DirectXAdapter>,
}

#[derive(Clone)]
pub struct DirectXFactory {
    factory: IDXGIFactory7,
    data: Arc<FactoryData>,
}

impl Debug for DirectXFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.factory.as_raw())
            .field("adapters", &self.data.adapters)
            .finish()
    }
}

impl DirectXObject for DirectXFactory {
    type Type = IDXGIFactory7;

    fn handle(&self) -> &Self::Type {
        &self.factory
    }
}

impl crate::api::traits::ApiRoot<crate::api::directx::DirectXApi> for DirectXFactory {
    fn new(create_info: crate::RootCreateInfo) -> crate::Result<Self> {
        let dxgi_factory_flags = if cfg!(feature = "gpu_debugging") {
            DXGI_CREATE_FACTORY_DEBUG
        } else {
            0
        };

        let factory: IDXGIFactory7 = unsafe { CreateDXGIFactory2(dxgi_factory_flags) }?;

        let mut adapters = Vec::new();

        for i in 0.. {
            let adapter = unsafe { factory.EnumAdapters1(i) };

            let adapter = match adapter {
                Ok(adapter) => adapter,
                Err(_) => break,
            };

            let mut desc = DXGI_ADAPTER_DESC1::default();
            unsafe { adapter.GetDesc1(&mut desc)? };

            adapters.push(DirectXAdapter::new(adapter, desc));
        }

        let data = Arc::new(FactoryData { adapters });

        Ok(Self { factory, data })
    }

    fn devices(&self) -> &[DirectXAdapter] {
        &self.data.adapters
    }
}
