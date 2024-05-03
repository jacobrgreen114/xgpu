// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::*;
use crate::FenceCreateInfo;
use std::fmt::Formatter;
use std::sync::Arc;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

struct DirectXFenceData {
    device: DirectXDevice,
}

#[derive(Clone)]
pub struct DirectXFence {
    fence: ID3D12Fence,
    data: Arc<DirectXFenceData>,
}

impl std::fmt::Debug for DirectXFence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.fence.as_raw())
            .finish()
    }
}

impl DirectXObject for DirectXFence {
    type Type = ID3D12Fence;

    fn handle(&self) -> &Self::Type {
        &self.fence
    }
}

impl DirectXDeviceObject for DirectXFence {
    fn device(&self) -> &DirectXDevice {
        &self.data.device
    }
}

impl crate::api::traits::Fence<DirectXApi> for DirectXFence {
    fn new(context: directx_type!(Context), create_info: FenceCreateInfo) -> crate::Result<Self> {
        let intial_value = create_info.signaled.then_some(1).unwrap_or(0);

        let fence = unsafe {
            context
                .handle()
                .CreateFence(intial_value, D3D12_FENCE_FLAG_NONE)?
        };

        let data = Arc::new(DirectXFenceData { device: context });

        Ok(Self { fence, data })
    }

    //fn signal(&self, context: directx_type!(Context), value: u64) -> crate::Result<()> {
    //    unsafe {
    //        context
    //            .queue()
    //            .handle()
    //            .Signal(self.fence.handle(), value)
    //    };
    //
    //    Ok(())
    //}
    //
    //fn wait(&self, context: directx_type!(Context), value: u64) -> crate::Result<()> {
    //    unsafe {
    //        self.fence
    //            .SetEventOnCompletion(value, context.fence_event())
    //    };
    //
    //    Ok(())
    //}
    //
    //fn get_value(&self) -> u64 {
    //    unsafe { self.fence.GetCompletedValue() }
    //}
}
