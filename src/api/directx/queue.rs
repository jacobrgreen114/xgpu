// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

use crate::api::directx::{DirectXApi, DirectXObject};
use crate::prelude::GraphicsApi;
use std::sync::Arc;

struct DirectXCommandQueueData {
    // device: <DirectXApi as GraphicsApi>::Context,
}

#[derive(Clone)]
pub struct DirectXCommandQueue {
    queue: ID3D12CommandQueue,
    data: Arc<DirectXCommandQueueData>,
}

impl Debug for DirectXCommandQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.queue.as_raw())
            .finish()
    }
}

impl DirectXObject for DirectXCommandQueue {
    type Type = ID3D12CommandQueue;

    fn handle(&self) -> &Self::Type {
        &self.queue
    }
}

impl DirectXCommandQueue {
    pub(crate) fn new(queue: ID3D12CommandQueue) -> Self {
        let data = Arc::new(DirectXCommandQueueData { /* device */  });
        Self { queue, data }
    }
}

impl crate::api::traits::Queue<DirectXApi> for DirectXCommandQueue {}
