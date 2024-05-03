// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api;

pub mod traits;

#[cfg(all(feature = "directx", feature = "vulkan"))]
compile_error!("Only one of 'directx' or 'vulkan' features can be enabled at a time");

#[cfg(all(feature = "directx", not(target_os = "windows")))]
compile_error!("The 'directx' feature is only available on Windows");

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "directx")]
pub mod directx;

#[cfg(feature = "directx")]
pub type Api = directx::DirectXApi;

#[cfg(feature = "vulkan")]
pub type Api = vulkan::VulkanApi;
