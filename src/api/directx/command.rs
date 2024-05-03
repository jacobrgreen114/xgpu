// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::*;
use crate::{CommandBufferAllocateInfo, CommandPoolCreateInfo};
use std::fmt::Formatter;
use std::sync::Arc;

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

/*
   Command Allocator
*/

struct CommandAllocatorData {
    device: DirectXDevice,
}

#[derive(Clone)]
pub struct DirectXCommandAllocator {
    allocator: ID3D12CommandAllocator,
    data: Arc<CommandAllocatorData>,
}

impl DirectXObject for DirectXCommandAllocator {
    type Type = ID3D12CommandAllocator;

    fn handle(&self) -> &Self::Type {
        &self.allocator
    }
}

impl DirectXDeviceObject for DirectXCommandAllocator {
    fn device(&self) -> &DirectXDevice {
        &self.data.device
    }
}

impl std::fmt::Debug for DirectXCommandAllocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.allocator.as_raw())
            .finish()
    }
}

impl crate::api::traits::CommandPool<DirectXApi> for DirectXCommandAllocator {
    fn new(
        context: directx_type!(Context),
        create_info: CommandPoolCreateInfo,
    ) -> crate::Result<Self> {
        let allocator: ID3D12CommandAllocator = unsafe {
            context
                .handle()
                .CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?
        };

        let data = Arc::new(CommandAllocatorData { device: context });

        Ok(Self { allocator, data })
    }
}

/*
   Command List
*/

struct CommandListData {
    allocator: DirectXCommandAllocator,
}

#[derive(Clone)]
pub struct DirectXCommandList {
    list: ID3D12GraphicsCommandList,
    data: Arc<CommandListData>,
}

impl DirectXObject for DirectXCommandList {
    type Type = ID3D12GraphicsCommandList;

    fn handle(&self) -> &Self::Type {
        &self.list
    }
}

impl DirectXDeviceObject for DirectXCommandList {
    fn device(&self) -> &DirectXDevice {
        &self.data.allocator.device()
    }
}

impl std::fmt::Debug for DirectXCommandList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.list.as_raw())
            .finish()
    }
}

impl crate::api::traits::CommandBuffer<DirectXApi> for DirectXCommandList {
    fn allocate(
        pool: directx_type!(CommandPool),
        create_info: CommandBufferAllocateInfo,
    ) -> crate::Result<Self> {
        let device = pool.device();

        let list: ID3D12GraphicsCommandList = unsafe {
            device.handle().CreateCommandList(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                pool.handle(),
                None,
            )?
        };

        let data = Arc::new(CommandListData { allocator: pool });

        Ok(Self { list, data })
    }
}
