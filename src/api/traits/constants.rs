// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

pub trait DeviceTypeConstants {
    const OTHER: i32;
    const INTEGRATED_GPU: i32;
    const DISCRETE_GPU: i32;
    const VIRTUAL_GPU: i32;
    const CPU: i32;
}

pub trait CompositeAlphaConstants {
    const OPAQUE: i32;
    const PRE_MULTIPLIED: i32;
    const POST_MULTIPLIED: i32;
    const INHERIT: i32;
}

pub trait ColorSpaceConstants {
    const SRGB_NONLINEAR: i32;
    const SRGB_EXT_LINEAR: i32;
    const HDR10_ST2084: i32;
    const HDR10_HLG: i32;
}

pub trait DataFormatConstants {
    const R8_UINT: i32;
    const R8_SINT: i32;
    const R8_UNORM: i32;
    const R8_SNORM: i32;

    const R8G8_UINT: i32;
    const R8G8_SINT: i32;
    const R8G8_UNORM: i32;
    const R8G8_SNORM: i32;

    const R8G8B8A8_UINT: i32;
    const R8G8B8A8_SINT: i32;
    const R8G8B8A8_UNORM: i32;
    const R8G8B8A8_SNORM: i32;
    const R8G8B8A8_UNORM_SRGB: i32;

    const B8G8R8A8_UNORM: i32;
    const B8G8R8A8_UNORM_SRGB: i32;

    const R16_UINT: i32;
    const R16_SINT: i32;
    const R16_UNORM: i32;
    const R16_SNORM: i32;

    const R16G16_UINT: i32;
    const R16G16_SINT: i32;
    const R16G16_UNORM: i32;
    const R16G16_SNORM: i32;

    const R16G16B16A16_UINT: i32;
    const R16G16B16A16_SINT: i32;
    const R16G16B16A16_UNORM: i32;
    const R16G16B16A16_SNORM: i32;
    const R16G16B16A16_SFLOAT: i32;

    const R32_UINT: i32;
    const R32_SINT: i32;
    const R32_SFLOAT: i32;

    const R32G32_UINT: i32;
    const R32G32_SINT: i32;
    const R32G32_SFLOAT: i32;

    const R32G32B32_UINT: i32;
    const R32G32B32_SINT: i32;
    const R32G32B32_SFLOAT: i32;

    const R32G32B32A32_UINT: i32;
    const R32G32B32A32_SINT: i32;
    const R32G32B32A32_SFLOAT: i32;

    const R10G10B10A2_UINT: i32;
    const R10G10B10A2_UNORM: i32;
}

pub trait PresentModeConstants {
    const IMMEDIATE: i32;
    const MAILBOX: i32;
    const FIFO: i32;
    const FIFO_RELAXED: i32;
}

pub trait BlendOpConstants {
    const ADD: i32;
    const SUBTRACT: i32;
    const REVERSE_SUBTRACT: i32;
    const MIN: i32;
    const MAX: i32;
}

pub trait BlendFactorConstants {
    const ZERO: i32;
    const ONE: i32;

    const SRC_COLOR: i32;
    const ONE_MINUS_SRC_COLOR: i32;

    const DST_COLOR: i32;
    const ONE_MINUS_DST_COLOR: i32;

    const SRC_ALPHA: i32;
    const ONE_MINUS_SRC_ALPHA: i32;

    const DST_ALPHA: i32;
    const ONE_MINUS_DST_ALPHA: i32;
}

pub trait ColorComponentConstants {
    const R: i32;
    const G: i32;
    const B: i32;
    const A: i32;
    const ALL: i32;
}

pub trait PolygonModeConstants {
    const FILL: i32;
    const LINE: i32;
}

pub trait CullModeConstants {
    const NONE: i32;
    const FRONT: i32;
    const BACK: i32;
}

pub trait FrontFaceConstants {
    const CLOCKWISE: i32;
    const COUNTER_CLOCKWISE: i32;
}

pub trait PrimitiveTopologyConstants {
    const POINT_LIST: i32;
    const LINE_LIST: i32;
    const LINE_STRIP: i32;
    const TRIANGLE_LIST: i32;
    const TRIANGLE_STRIP: i32;
    const TRIANGLE_FAN: i32;
    const LINE_LIST_WITH_ADJACENCY: i32;
    const LINE_STRIP_WITH_ADJACENCY: i32;
    const TRIANGLE_LIST_WITH_ADJACENCY: i32;
    const TRIANGLE_STRIP_WITH_ADJACENCY: i32;
    const PATCH_LIST: i32;
}
