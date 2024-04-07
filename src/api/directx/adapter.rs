// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::{DirectXApi, DirectXObject};
use crate::api::traits::GraphicsApi;
use crate::prelude::DeviceProperties;
use crate::Vendor;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

#[derive(Debug)]
struct DirectXAdapterData {
    description: DirectXAdapterDescription,
}

#[derive(Clone)]
pub struct DirectXAdapter {
    adapter: IDXGIAdapter1,
    data: Arc<DirectXAdapterData>,
}

impl DirectXObject for DirectXAdapter {
    type Type = IDXGIAdapter1;

    fn handle(&self) -> &Self::Type {
        &self.adapter
    }
}

impl Debug for DirectXAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.adapter.as_raw())
            .field("properties", &self.data.description)
            .finish()
    }
}

impl DirectXAdapter {
    pub(crate) fn new(adapter: IDXGIAdapter1, desc: DXGI_ADAPTER_DESC1) -> Self {
        let data = Arc::new(DirectXAdapterData {
            description: desc.into(),
        });

        Self { adapter, data }
    }
}

impl crate::api::traits::Device<crate::api::directx::DirectXApi> for DirectXAdapter {
    fn properties(&self) -> &<DirectXApi as GraphicsApi>::DeviceProperties {
        &self.data.description
    }
}

pub struct DirectXAdapterDescription {
    description: DXGI_ADAPTER_DESC1,
    name: String,
}

impl Debug for DirectXAdapterDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("name", &self.name())
            .field("vendor", &self.vendor())
            .field("device_type", &self.device_type())
            .finish()
    }
}

impl From<DXGI_ADAPTER_DESC1> for DirectXAdapterDescription {
    fn from(desc: DXGI_ADAPTER_DESC1) -> Self {
        let name = String::from_utf16(&desc.Description)
            .unwrap()
            .trim_end_matches(0 as char)
            .to_string();
        Self {
            description: desc,
            name,
        }
    }
}

impl crate::api::traits::DeviceProperties<crate::api::directx::DirectXApi>
    for DirectXAdapterDescription
{
    fn name(&self) -> &str {
        &self.name
    }

    fn vendor(&self) -> Vendor {
        self.description.VendorId.into()
    }

    fn device_type(&self) -> crate::DeviceType {
        let dev_type = self.description.Flags & 0x3;

        match dev_type {
            0 => crate::DeviceType::Gpu,
            2 => crate::DeviceType::Software,
            _ => panic!("Unknown device type: {}", dev_type),
        }
    }
}
