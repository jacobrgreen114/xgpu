// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

use crate::api::directx::factory::DirectXFactory;
use crate::api::directx::queue::DirectXCommandQueue;
use crate::api::directx::{DirectXApi, DirectXFactoryObject, DirectXObject};
use crate::ContextCreateInfo;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

use crate::api::traits::GraphicsApi;

struct DirectXDeviceData {
    factory: directx_type!(Root),
    adapter: directx_type!(Device),
    queues: Vec<directx_type!(Queue)>,
    // #[cfg(feature = "validation")]
    // debug: ID3D12Debug,
}

#[derive(Clone)]
pub struct DirectXDevice {
    device: ID3D12Device,
    data: Arc<DirectXDeviceData>,
}

impl DirectXObject for DirectXDevice {
    type Type = ID3D12Device;

    fn handle(&self) -> &Self::Type {
        &self.device
    }
}

impl DirectXFactoryObject for DirectXDevice {
    fn factory(&self) -> &DirectXFactory {
        &self.data.factory
    }
}

impl Debug for DirectXDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.device.as_raw())
            .finish()
    }
}

#[cfg(feature = "validation")]
static mut DEBUG: Option<ID3D12Debug> = None;

impl crate::api::traits::Context<DirectXApi> for DirectXDevice {
    fn new(
        root: directx_type!(Root),
        device: directx_type!(Device),
        create_info: ContextCreateInfo,
    ) -> crate::Result<Self> {
        unsafe {
            #[cfg(feature = "validation")]
            DEBUG.get_or_insert_with(|| unsafe {
                let mut debug: Option<ID3D12Debug> = None;
                D3D12GetDebugInterface(&mut debug).unwrap();
                let debug = debug.unwrap();
                debug.EnableDebugLayer();
                debug
            });
        }

        let dev = {
            let mut dev: Option<ID3D12Device> = None;
            unsafe { D3D12CreateDevice(device.handle(), D3D_FEATURE_LEVEL_12_0, &mut dev)? };
            dev.unwrap()
        };

        let mut feature_data = D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: Default::default(),
            Support1: Default::default(),
            Support2: Default::default(),
        };

        let supported = unsafe {
            dev.CheckFeatureSupport(
                D3D12_FEATURE_FORMAT_SUPPORT,
                &mut feature_data as *mut _ as *mut _,
                std::mem::size_of_val(&feature_data) as u32,
            )
        }
        .ok()
        .map_or_else(|| true, |_| false);

        // todo : implement support for multiple queues

        let mut queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            NodeMask: 0,
        };

        let queue: ID3D12CommandQueue = unsafe { dev.CreateCommandQueue(&queue_desc)? };

        let queue = DirectXCommandQueue::new(queue);

        let data = Arc::new(DirectXDeviceData {
            factory: root,
            adapter: device,
            queues: vec![queue],
            // #[cfg(feature = "validation")]
            // debug,
        });

        Ok(Self { device: dev, data })
    }

    fn queues(&self) -> &[directx_type!(Queue)] {
        &self.data.queues
    }
}
