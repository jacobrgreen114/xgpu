// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

macro_rules! data_format {
    ($name:tt) => {
        <<crate::api::Api as crate::api::traits::GraphicsApi>::DataFormatConstants as crate::api::traits::constants::DataFormatConstants>::$name
    };
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(i32)]
pub enum Format {
    R8_UINT = data_format!(R8_UINT),
    R8_SINT = data_format!(R8_SINT),
    R8_UNORM = data_format!(R8_UNORM),
    R8_SNORM = data_format!(R8_SNORM),

    R8G8_UINT = data_format!(R8G8_UINT),
    R8G8_SINT = data_format!(R8G8_SINT),
    R8G8_UNORM = data_format!(R8G8_UNORM),
    R8G8_SNORM = data_format!(R8G8_SNORM),

    R8G8B8A8_UINT = data_format!(R8G8B8A8_UINT),
    R8G8B8A8_SINT = data_format!(R8G8B8A8_SINT),
    R8G8B8A8_UNORM = data_format!(R8G8B8A8_UNORM),
    R8G8B8A8_SNORM = data_format!(R8G8B8A8_SNORM),
    R8G8B8A8_UNORM_SRGB = data_format!(R8G8B8A8_UNORM_SRGB),

    B8G8R8A8_UNORM = data_format!(B8G8R8A8_UNORM),
    B8G8R8A8_UNORM_SRGB = data_format!(B8G8R8A8_UNORM_SRGB),

    R16_UINT = data_format!(R16_UINT),
    R16_SINT = data_format!(R16_SINT),
    R16_UNORM = data_format!(R16_UNORM),
    R16_SNORM = data_format!(R16_SNORM),

    R16G16_UINT = data_format!(R16G16_UINT),
    R16G16_SINT = data_format!(R16G16_SINT),
    R16G16_UNORM = data_format!(R16G16_UNORM),
    R16G16_SNORM = data_format!(R16G16_SNORM),

    R16G16B16A16_UINT = data_format!(R16G16B16A16_UINT),
    R16G16B16A16_SINT = data_format!(R16G16B16A16_SINT),
    R16G16B16A16_UNORM = data_format!(R16G16B16A16_UNORM),
    R16G16B16A16_SNORM = data_format!(R16G16B16A16_SNORM),
    R16G16B16A16_SFLOAT = data_format!(R16G16B16A16_SFLOAT),

    R32_UINT = data_format!(R32_UINT),
    R32_SINT = data_format!(R32_SINT),
    R32_SFLOAT = data_format!(R32_SFLOAT),

    R32G32_UINT = data_format!(R32G32_UINT),
    R32G32_SINT = data_format!(R32G32_SINT),
    R32G32_SFLOAT = data_format!(R32G32_SFLOAT),

    R32G32B32_UINT = data_format!(R32G32B32_UINT),
    R32G32B32_SINT = data_format!(R32G32B32_SINT),
    R32G32B32_SFLOAT = data_format!(R32G32B32_SFLOAT),

    R32G32B32A32_UINT = data_format!(R32G32B32A32_UINT),
    R32G32B32A32_SINT = data_format!(R32G32B32A32_SINT),
    R32G32B32A32_SFLOAT = data_format!(R32G32B32A32_SFLOAT),

    R10G10B10A2_UINT = data_format!(R10G10B10A2_UINT),
    R10G10B10A2_UNORM = data_format!(R10G10B10A2_UNORM),
}
