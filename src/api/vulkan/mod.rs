// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

macro_rules! vulkan_type {
    ($name:tt) => {
        <crate::api::vulkan::VulkanApi as crate::api::traits::GraphicsApi>::$name
    };
}

mod instance;
pub use instance::*;

mod physical_device;
pub use physical_device::*;

mod surface;
pub use surface::*;

mod device;
pub use device::*;

mod command;
pub use command::*;

mod swapchain;
pub use swapchain::*;

mod image;
pub use image::*;

mod shader;
pub use shader::*;

mod framebuffer;
pub use framebuffer::*;

mod pipeline;
pub use pipeline::*;

mod fence;
pub use fence::*;

mod semaphore;
pub use semaphore::*;

use vulkan_sys::*;

use std::sync::Arc;

pub struct VulkanApi;
impl crate::api::traits::GraphicsApi for VulkanApi {
    type Root = VulkanInstance;
    type Device = VulkanPhysicalDevice;
    type DeviceProperties = VulkanPhysicalDeviceProperties;
    // type DeviceFeatures = VulkanPhysicalDeviceFeatures;
    type Surface = VulkanSurface;
    type SurfaceCapabilities = VulkanSurfaceCapabilities;
    type Context = VulkanDevice;
    type Queue = VulkanQueue;
    type CommandPool = VulkanCommandPool;
    type CommandBuffer = VulkanCommandBuffer;

    type Swapchain = VulkanSwapchain;
    type Image = VulkanImage;
    type ImageView = VulkanImageView;

    type Shader = VulkanShaderModule;

    type RenderPass = VulkanRenderPass;
    type Framebuffer = VulkanFramebuffer;

    type PipelineLayout = VulkanPipelineLayout;

    // type VertexInputState = VulkanPipelineVertexInputStateCreateInfo;
    // type InputAssemblyState = VulkanPipelineInputAssemblyStateCreateInfo;
    // type RasterizationState = VulkanPipelineRasterizationStateCreateInfo;
    type GraphicsPipeline = VulkanGraphicsPipeline;

    type Fence = VulkanFence;
    type Semaphore = VulkanSemaphore;

    type DeviceTypeConstants = constants::VulkanDeviceTypes;
    type CompositeAlphaConstants = constants::VulkanCompositeAlphaModes;
    type ColorSpaceConstants = constants::VulkanColorSpaces;
    type DataFormatConstants = constants::VulkanRenderTargetFormatConstants;
    type PresentModeConstants = constants::VulkanPresentModeConstants;
    type BlendOpConstants = constants::VulkanBlendOpConstants;
    type BlendFactorConstants = constants::VulkanBlendFactorConstants;
    type ColorComponentConstants = constants::VulkanColorComponentConstants;
    type PolygonModeConstants = constants::VulkanPolygonModeConstants;
    type CullModeConstants = constants::VulkanCullModeConstants;
    type FrontFaceConstants = constants::VulkanFrontFaceConstants;
    type PrimitiveTopologyConstants = constants::VulkanPrimitiveTopologyConstants;
}

mod constants {
    use vulkan_sys::*;

    pub struct VulkanDeviceTypes;
    impl crate::api::traits::constants::DeviceTypeConstants for VulkanDeviceTypes {
        const OTHER: i32 = VK_PHYSICAL_DEVICE_TYPE_OTHER;
        const INTEGRATED_GPU: i32 = VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU;
        const DISCRETE_GPU: i32 = VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU;
        const VIRTUAL_GPU: i32 = VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU;
        const CPU: i32 = VK_PHYSICAL_DEVICE_TYPE_CPU;
    }

    pub struct VulkanCompositeAlphaModes;
    impl crate::api::traits::constants::CompositeAlphaConstants for VulkanCompositeAlphaModes {
        const OPAQUE: i32 = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        const PRE_MULTIPLIED: i32 = VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR;
        const POST_MULTIPLIED: i32 = VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR;
        const INHERIT: i32 = VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR;
    }

    pub struct VulkanColorSpaces;
    impl crate::api::traits::constants::ColorSpaceConstants for VulkanColorSpaces {
        const SRGB_NONLINEAR: i32 = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
        const SRGB_EXT_LINEAR: i32 = VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT;
        const HDR10_ST2084: i32 = VK_COLOR_SPACE_HDR10_ST2084_EXT;
        const HDR10_HLG: i32 = VK_COLOR_SPACE_HDR10_HLG_EXT;
    }

    pub struct VulkanRenderTargetFormatConstants;
    impl crate::api::traits::constants::DataFormatConstants for VulkanRenderTargetFormatConstants {
        const R8_UINT: i32 = VK_FORMAT_R8_UINT;
        const R8_SINT: i32 = VK_FORMAT_R8_SINT;
        const R8_UNORM: i32 = VK_FORMAT_R8_UNORM;
        const R8_SNORM: i32 = VK_FORMAT_R8_SNORM;

        const R8G8_UINT: i32 = VK_FORMAT_R8G8_UINT;
        const R8G8_SINT: i32 = VK_FORMAT_R8G8_SINT;
        const R8G8_UNORM: i32 = VK_FORMAT_R8G8_UNORM;
        const R8G8_SNORM: i32 = VK_FORMAT_R8G8_SNORM;

        const R8G8B8A8_UINT: i32 = VK_FORMAT_R8G8B8A8_UINT;
        const R8G8B8A8_SINT: i32 = VK_FORMAT_R8G8B8A8_SINT;
        const R8G8B8A8_UNORM: i32 = VK_FORMAT_R8G8B8A8_UNORM;
        const R8G8B8A8_SNORM: i32 = VK_FORMAT_R8G8B8A8_SNORM;
        const R8G8B8A8_UNORM_SRGB: i32 = VK_FORMAT_R8G8B8A8_SRGB;

        const B8G8R8A8_UNORM: i32 = VK_FORMAT_B8G8R8A8_UNORM;
        const B8G8R8A8_UNORM_SRGB: i32 = VK_FORMAT_B8G8R8A8_SRGB;

        const R16_UINT: i32 = VK_FORMAT_R16_UINT;
        const R16_SINT: i32 = VK_FORMAT_R16_SINT;
        const R16_UNORM: i32 = VK_FORMAT_R16_UNORM;
        const R16_SNORM: i32 = VK_FORMAT_R16_SNORM;

        const R16G16_UINT: i32 = VK_FORMAT_R16G16_UINT;
        const R16G16_SINT: i32 = VK_FORMAT_R16G16_SINT;
        const R16G16_UNORM: i32 = VK_FORMAT_R16G16_UNORM;
        const R16G16_SNORM: i32 = VK_FORMAT_R16G16_SNORM;

        const R16G16B16A16_UINT: i32 = VK_FORMAT_R16G16B16A16_UINT;
        const R16G16B16A16_SINT: i32 = VK_FORMAT_R16G16B16A16_SINT;
        const R16G16B16A16_UNORM: i32 = VK_FORMAT_R16G16B16A16_UNORM;
        const R16G16B16A16_SNORM: i32 = VK_FORMAT_R16G16B16A16_SNORM;
        const R16G16B16A16_SFLOAT: i32 = VK_FORMAT_R16G16B16A16_SFLOAT;

        const R32_UINT: i32 = VK_FORMAT_R32_UINT;
        const R32_SINT: i32 = VK_FORMAT_R32_SINT;
        const R32_SFLOAT: i32 = VK_FORMAT_R32_SFLOAT;

        const R32G32_UINT: i32 = VK_FORMAT_R32G32_UINT;
        const R32G32_SINT: i32 = VK_FORMAT_R32G32_SINT;
        const R32G32_SFLOAT: i32 = VK_FORMAT_R32G32_SFLOAT;

        const R32G32B32_UINT: i32 = VK_FORMAT_R32G32B32_UINT;
        const R32G32B32_SINT: i32 = VK_FORMAT_R32G32B32_SINT;
        const R32G32B32_SFLOAT: i32 = VK_FORMAT_R32G32B32_SFLOAT;

        const R32G32B32A32_UINT: i32 = VK_FORMAT_R32G32B32A32_UINT;
        const R32G32B32A32_SINT: i32 = VK_FORMAT_R32G32B32A32_SINT;
        const R32G32B32A32_SFLOAT: i32 = VK_FORMAT_R32G32B32A32_SFLOAT;

        const R10G10B10A2_UINT: i32 = VK_FORMAT_A2B10G10R10_UINT_PACK32;
        const R10G10B10A2_UNORM: i32 = VK_FORMAT_A2B10G10R10_UNORM_PACK32;
    }

    pub struct VulkanPresentModeConstants;
    impl crate::api::traits::constants::PresentModeConstants for VulkanPresentModeConstants {
        const IMMEDIATE: i32 = VK_PRESENT_MODE_IMMEDIATE_KHR;
        const MAILBOX: i32 = VK_PRESENT_MODE_MAILBOX_KHR;
        const FIFO: i32 = VK_PRESENT_MODE_FIFO_KHR;
        const FIFO_RELAXED: i32 = VK_PRESENT_MODE_FIFO_RELAXED_KHR;
    }

    pub struct VulkanBlendOpConstants;
    impl crate::api::traits::constants::BlendOpConstants for VulkanBlendOpConstants {
        const ADD: i32 = VK_BLEND_OP_ADD;
        const SUBTRACT: i32 = VK_BLEND_OP_SUBTRACT;
        const REVERSE_SUBTRACT: i32 = VK_BLEND_OP_REVERSE_SUBTRACT;
        const MIN: i32 = VK_BLEND_OP_MIN;
        const MAX: i32 = VK_BLEND_OP_MAX;
    }

    pub struct VulkanBlendFactorConstants;
    impl crate::api::traits::constants::BlendFactorConstants for VulkanBlendFactorConstants {
        const ZERO: i32 = VK_BLEND_FACTOR_ZERO;
        const ONE: i32 = VK_BLEND_FACTOR_ONE;
        const SRC_COLOR: i32 = VK_BLEND_FACTOR_SRC_COLOR;
        const ONE_MINUS_SRC_COLOR: i32 = VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR;
        const DST_COLOR: i32 = VK_BLEND_FACTOR_DST_COLOR;
        const ONE_MINUS_DST_COLOR: i32 = VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR;
        const SRC_ALPHA: i32 = VK_BLEND_FACTOR_SRC_ALPHA;
        const ONE_MINUS_SRC_ALPHA: i32 = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
        const DST_ALPHA: i32 = VK_BLEND_FACTOR_DST_ALPHA;
        const ONE_MINUS_DST_ALPHA: i32 = VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA;
        // const CONSTANT_COLOR: i32 = VK_BLEND_FACTOR_CONSTANT_COLOR;
        // const ONE_MINUS_CONSTANT_COLOR: i32 = VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR;
        // const CONSTANT_ALPHA: i32 = VK_BLEND_FACTOR_CONSTANT_ALPHA;
        // const ONE_MINUS_CONSTANT_ALPHA: i32 = VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA;
        // const SRC_ALPHA_SATURATE: i32 = VK_BLEND_FACTOR_SRC_ALPHA_SATURATE;
        // const SRC1_COLOR: i32 = VK_BLEND_FACTOR_SRC1_COLOR;
        // const ONE_MINUS_SRC1_COLOR: i32 = VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR;
        // const SRC1_ALPHA: i32 = VK_BLEND_FACTOR_SRC1_ALPHA;
        // const ONE_MINUS_SRC1_ALPHA: i32 = VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA;
    }

    pub struct VulkanColorComponentConstants;
    impl crate::api::traits::constants::ColorComponentConstants for VulkanColorComponentConstants {
        const R: i32 = VK_COLOR_COMPONENT_R_BIT;
        const G: i32 = VK_COLOR_COMPONENT_G_BIT;
        const B: i32 = VK_COLOR_COMPONENT_B_BIT;
        const A: i32 = VK_COLOR_COMPONENT_A_BIT;
        const ALL: i32 = VK_COLOR_COMPONENT_R_BIT
            | VK_COLOR_COMPONENT_G_BIT
            | VK_COLOR_COMPONENT_B_BIT
            | VK_COLOR_COMPONENT_A_BIT;
    }

    pub struct VulkanPolygonModeConstants;
    impl crate::api::traits::constants::PolygonModeConstants for VulkanPolygonModeConstants {
        const FILL: i32 = VK_POLYGON_MODE_FILL;
        const LINE: i32 = VK_POLYGON_MODE_LINE;
    }

    pub struct VulkanCullModeConstants;
    impl crate::api::traits::constants::CullModeConstants for VulkanCullModeConstants {
        const NONE: i32 = VK_CULL_MODE_NONE;
        const FRONT: i32 = VK_CULL_MODE_FRONT_BIT;
        const BACK: i32 = VK_CULL_MODE_BACK_BIT;
    }

    pub struct VulkanFrontFaceConstants;
    impl crate::api::traits::constants::FrontFaceConstants for VulkanFrontFaceConstants {
        const CLOCKWISE: i32 = VK_FRONT_FACE_CLOCKWISE;
        const COUNTER_CLOCKWISE: i32 = VK_FRONT_FACE_COUNTER_CLOCKWISE;
    }

    pub struct VulkanPrimitiveTopologyConstants;
    impl crate::api::traits::constants::PrimitiveTopologyConstants
        for VulkanPrimitiveTopologyConstants
    {
        const POINT_LIST: i32 = VK_PRIMITIVE_TOPOLOGY_POINT_LIST;
        const LINE_LIST: i32 = VK_PRIMITIVE_TOPOLOGY_LINE_LIST;
        const LINE_STRIP: i32 = VK_PRIMITIVE_TOPOLOGY_LINE_STRIP;
        const TRIANGLE_LIST: i32 = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
        const TRIANGLE_STRIP: i32 = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP;
        const TRIANGLE_FAN: i32 = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN;
        const LINE_LIST_WITH_ADJACENCY: i32 = VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY;
        const LINE_STRIP_WITH_ADJACENCY: i32 = VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY;
        const TRIANGLE_LIST_WITH_ADJACENCY: i32 =
            VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY;
        const TRIANGLE_STRIP_WITH_ADJACENCY: i32 =
            VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY;
        const PATCH_LIST: i32 = VK_PRIMITIVE_TOPOLOGY_PATCH_LIST;
    }
}

pub trait VulkanObject {
    type Handle;

    fn handle(&self) -> Self::Handle;
}

pub trait VulkanInstanceObject: VulkanObject {
    fn instance(&self) -> &VulkanInstance;
}

pub trait VulkanDeviceObject: VulkanInstanceObject {
    fn device(&self) -> &VulkanDevice;
}

impl<T: VulkanDeviceObject> VulkanInstanceObject for T {
    fn instance(&self) -> &VulkanInstance {
        self.device().instance()
    }
}

pub trait VulkanType {
    type Type;

    fn native(&self) -> &Self::Type;
}

type Ownership<T> = Arc<T>;

impl Into<VkPresentModeKHR> for crate::PresentMode {
    fn into(self) -> VkPresentModeKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkPresentModeKHR> for crate::PresentMode {
    fn from(value: VkPresentModeKHR) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkCompositeAlphaFlagBitsKHR> for crate::CompositeAlphaMode {
    fn into(self) -> VkCompositeAlphaFlagBitsKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<VkFormat> for crate::Format {
    fn into(self) -> VkFormat {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkFormat> for crate::Format {
    fn from(value: VkFormat) -> Self {
        use crate::Format;

        if !cfg!(debug_assertions) {
            unsafe { std::mem::transmute(value) }
        } else {
            match value {
                VK_FORMAT_R8_UINT => Format::R8_UINT,
                VK_FORMAT_R8_SINT => Format::R8_SINT,
                VK_FORMAT_R8_UNORM => Format::R8_UNORM,
                VK_FORMAT_R8_SNORM => Format::R8_SNORM,

                VK_FORMAT_R8G8_UINT => Format::R8G8_UINT,
                VK_FORMAT_R8G8_SINT => Format::R8G8_SINT,
                VK_FORMAT_R8G8_UNORM => Format::R8G8_UNORM,
                VK_FORMAT_R8G8_SNORM => Format::R8G8_SNORM,

                VK_FORMAT_R8G8B8A8_UINT => Format::R8G8B8A8_UINT,
                VK_FORMAT_R8G8B8A8_SINT => Format::R8G8B8A8_SINT,
                VK_FORMAT_R8G8B8A8_UNORM => Format::R8G8B8A8_UNORM,
                VK_FORMAT_R8G8B8A8_SNORM => Format::R8G8B8A8_SNORM,
                VK_FORMAT_R8G8B8A8_SRGB => Format::R8G8B8A8_UNORM_SRGB,

                VK_FORMAT_B8G8R8A8_UNORM => Format::B8G8R8A8_UNORM,
                VK_FORMAT_B8G8R8A8_SRGB => Format::B8G8R8A8_UNORM_SRGB,

                VK_FORMAT_R16_UINT => Format::R16_UINT,
                VK_FORMAT_R16_SINT => Format::R16_SINT,
                VK_FORMAT_R16_UNORM => Format::R16_UNORM,
                VK_FORMAT_R16_SNORM => Format::R16_SNORM,

                VK_FORMAT_R16G16_UINT => Format::R16G16_UINT,
                VK_FORMAT_R16G16_SINT => Format::R16G16_SINT,
                VK_FORMAT_R16G16_UNORM => Format::R16G16_UNORM,
                VK_FORMAT_R16G16_SNORM => Format::R16G16_SNORM,

                VK_FORMAT_R16G16B16A16_UINT => Format::R16G16B16A16_UINT,
                VK_FORMAT_R16G16B16A16_SINT => Format::R16G16B16A16_SINT,
                VK_FORMAT_R16G16B16A16_UNORM => Format::R16G16B16A16_UNORM,
                VK_FORMAT_R16G16B16A16_SNORM => Format::R16G16B16A16_SNORM,
                VK_FORMAT_R16G16B16A16_SFLOAT => Format::R16G16B16A16_SFLOAT,

                VK_FORMAT_R32_UINT => Format::R32_UINT,
                VK_FORMAT_R32_SINT => Format::R32_SINT,
                VK_FORMAT_R32_SFLOAT => Format::R32_SFLOAT,

                VK_FORMAT_R32G32_UINT => Format::R32G32_UINT,
                VK_FORMAT_R32G32_SINT => Format::R32G32_SINT,
                VK_FORMAT_R32G32_SFLOAT => Format::R32G32_SFLOAT,

                VK_FORMAT_R32G32B32_UINT => Format::R32G32B32_UINT,
                VK_FORMAT_R32G32B32_SINT => Format::R32G32B32_SINT,
                VK_FORMAT_R32G32B32_SFLOAT => Format::R32G32B32_SFLOAT,

                VK_FORMAT_R32G32B32A32_UINT => Format::R32G32B32A32_UINT,
                VK_FORMAT_R32G32B32A32_SINT => Format::R32G32B32A32_SINT,
                VK_FORMAT_R32G32B32A32_SFLOAT => Format::R32G32B32A32_SFLOAT,

                VK_FORMAT_A2B10G10R10_UINT_PACK32 => Format::R10G10B10A2_UINT,
                VK_FORMAT_A2B10G10R10_UNORM_PACK32 => Format::R10G10B10A2_UNORM,

                _ => panic!("Unknown format"),
            }
        }
    }
}

impl Into<VkColorSpaceKHR> for crate::Colorspace {
    fn into(self) -> VkColorSpaceKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkColorSpaceKHR> for crate::Colorspace {
    fn from(value: VkColorSpaceKHR) -> Self {
        use crate::Colorspace;

        if !cfg!(debug_assertions) {
            unsafe { std::mem::transmute(value) }
        } else {
            match value {
                VK_COLOR_SPACE_SRGB_NONLINEAR_KHR => Colorspace::SRGB_NONLINEAR,
                VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT => Colorspace::SRGB_EXT_LINEAR,
                VK_COLOR_SPACE_HDR10_ST2084_EXT => Colorspace::HDR10_ST2084,
                VK_COLOR_SPACE_HDR10_HLG_EXT => Colorspace::HDR10_HLG,
                _ => panic!("Unknown colorspace"),
            }
        }
    }
}

impl From<VkSurfaceFormatKHR> for crate::SurfaceFormat {
    fn from(value: VkSurfaceFormatKHR) -> Self {
        Self {
            format: value.format.into(),
            colorspace: value.colorSpace.into(),
        }
    }
}

impl Into<VkExtent2D> for crate::Extent2D {
    fn into(self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.height,
        }
    }
}

impl From<VkExtent2D> for crate::Extent2D {
    fn from(value: VkExtent2D) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl Into<VkOffset2D> for crate::Offset2D {
    fn into(self) -> VkOffset2D {
        VkOffset2D {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<VkOffset2D> for crate::Offset2D {
    fn from(value: VkOffset2D) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Into<VkRect2D> for crate::Rect2D {
    fn into(self) -> VkRect2D {
        VkRect2D {
            offset: self.offset.into(),
            extent: self.extent.into(),
        }
    }
}

impl From<VkRect2D> for crate::Rect2D {
    fn from(value: VkRect2D) -> Self {
        Self {
            offset: value.offset.into(),
            extent: value.extent.into(),
        }
    }
}

impl Into<VkPrimitiveTopology> for crate::PrimitiveTopology {
    fn into(self) -> VkPrimitiveTopology {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkPrimitiveTopology> for crate::PrimitiveTopology {
    fn from(value: VkPrimitiveTopology) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkPolygonMode> for crate::PolygonMode {
    fn into(self) -> VkPolygonMode {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkPolygonMode> for crate::PolygonMode {
    fn from(value: VkPolygonMode) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkCullModeFlags> for crate::CullMode {
    fn into(self) -> VkCullModeFlags {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkCullModeFlags> for crate::CullMode {
    fn from(value: VkCullModeFlags) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkFrontFace> for crate::FrontFace {
    fn into(self) -> VkFrontFace {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkFrontFace> for crate::FrontFace {
    fn from(value: VkFrontFace) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkBlendOp> for crate::BlendOp {
    fn into(self) -> VkBlendOp {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkBlendOp> for crate::BlendOp {
    fn from(value: VkBlendOp) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkBlendFactor> for crate::BlendFactor {
    fn into(self) -> VkBlendFactor {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkBlendFactor> for crate::BlendFactor {
    fn from(value: VkBlendFactor) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkColorComponentFlags> for crate::ColorComponentFlags {
    fn into(self) -> VkColorComponentFlags {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkColorComponentFlags> for crate::ColorComponentFlags {
    fn from(value: VkColorComponentFlags) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

// impl Into<VkShaderStageFlagBits> for crate::ShaderStage {
//     fn into(self) -> VkShaderStageFlagBits {
//         unsafe { std::mem::transmute(self) }
//     }
// }
//
// impl Into<VkCommandBufferLevel> for crate::CommandBufferLevel {
//     fn into(self) -> VkCommandBufferLevel {
//         unsafe { std::mem::transmute(self) }
//     }
// }
//
// impl From<VkCommandBufferLevel> for crate::CommandBufferLevel {
//     fn from(value: VkCommandBufferLevel) -> Self {
//         unsafe { std::mem::transmute(value) }
//     }
// }
