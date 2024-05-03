// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::{DirectXApi, DirectXObject};
use crate::api::traits::GraphicsApi;
use crate::prelude::{DeviceProperties, SurfaceCapabilities};
use crate::{Colorspace, Extent2D, Format, PresentMode, SurfaceFormat, Vendor};
use std::any::type_name;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Dxgi::*, Win32::UI::WindowsAndMessaging::*,
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

impl crate::api::traits::Device<DirectXApi> for DirectXAdapter {
    fn properties(&self) -> &directx_type!(DeviceProperties) {
        &self.data.description
    }

    fn supports_surface(&self, surface: directx_type!(Surface)) -> bool {
        // actually implement this
        true
    }

    fn get_surface_capabilities(
        &self,
        surface: directx_type!(Surface),
    ) -> crate::Result<directx_type!(SurfaceCapabilities)> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(*surface.handle(), &mut rect) }?;

        Ok(DirectXSurfaceCapabilities {
            current_extent: Extent2D {
                width: (rect.right - rect.left) as u32,
                height: (rect.bottom - rect.top) as u32,
            },
        })
    }

    fn get_surface_formats(
        &self,
        surface: directx_type!(Surface),
    ) -> crate::Result<Vec<SurfaceFormat>> {
        Ok(vec![
            BGRA8_UNORM,
            BGRA8_SRGB,
            RGBA8_UNORM,
            RGBA8_SRGB,
            RGBA16F_SRGB_LINEAR,
            RGB10A2_ST2084,
            RGB10A2_SRGB,
        ])
    }

    fn get_surface_present_modes(
        &self,
        surface: directx_type!(Surface),
    ) -> crate::Result<Vec<PresentMode>> {
        Ok(vec![
            PresentMode::Fifo,
            PresentMode::FifoRelaxed,
            PresentMode::Mailbox,
            PresentMode::Immediate,
        ])
    }
}

const BGRA8_UNORM: SurfaceFormat = SurfaceFormat {
    format: Format::B8G8R8A8_UNORM,
    colorspace: Colorspace::SRGB_NONLINEAR,
};

const BGRA8_SRGB: SurfaceFormat = SurfaceFormat {
    format: Format::B8G8R8A8_UNORM,
    colorspace: Colorspace::SRGB_NONLINEAR,
};

const RGBA8_UNORM: SurfaceFormat = SurfaceFormat {
    format: Format::R8G8B8A8_UNORM,
    colorspace: Colorspace::SRGB_NONLINEAR,
};

const RGBA8_SRGB: SurfaceFormat = SurfaceFormat {
    format: Format::R8G8B8A8_UNORM,
    colorspace: Colorspace::SRGB_NONLINEAR,
};

const RGBA16F_SRGB_LINEAR: SurfaceFormat = SurfaceFormat {
    format: Format::R16G16B16A16_SFLOAT,
    colorspace: Colorspace::SRGB_EXT_LINEAR,
};

const RGB10A2_ST2084: SurfaceFormat = SurfaceFormat {
    format: Format::R10G10B10A2_UNORM,
    colorspace: Colorspace::HDR10_ST2084,
};

const RGB10A2_SRGB: SurfaceFormat = SurfaceFormat {
    format: Format::R10G10B10A2_UNORM,
    colorspace: Colorspace::SRGB_NONLINEAR,
};

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

impl crate::api::traits::DeviceProperties<DirectXApi> for DirectXAdapterDescription {
    fn name(&self) -> &str {
        &self.name
    }

    fn device_type(&self) -> crate::DeviceType {
        todo!()
    }

    fn vendor(&self) -> Vendor {
        self.description.VendorId.into()
    }
}

/*
   Surface Capabilities
*/

pub struct DirectXSurfaceCapabilities {
    current_extent: Extent2D,
}

impl Debug for DirectXSurfaceCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("min_image_count", &self.min_image_count())
            .field("max_image_count", &self.max_image_count())
            .field("current_extent", &self.current_extent())
            .field("min_image_extent", &self.min_image_extent())
            .field("max_image_extent", &self.max_image_extent())
            .field("max_image_array_layers", &self.max_image_array_layers())
            .finish()
    }
}

impl crate::api::traits::SurfaceCapabilities<DirectXApi> for DirectXSurfaceCapabilities {
    fn min_image_count(&self) -> u32 {
        2
    }

    fn max_image_count(&self) -> u32 {
        DXGI_MAX_SWAP_CHAIN_BUFFERS
    }

    fn current_extent(&self) -> Extent2D {
        self.current_extent
    }

    fn min_image_extent(&self) -> Extent2D {
        self.current_extent
    }

    fn max_image_extent(&self) -> Extent2D {
        self.current_extent
    }

    fn max_image_array_layers(&self) -> u32 {
        1
    }
}
