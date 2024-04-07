// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

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

mod shader;
pub use shader::*;

mod pipeline;
pub use pipeline::*;

// mod graphics_pipeline;
// pub use graphics_pipeline::*;

use vulkan_sys::*;

use std::sync::Arc;

pub struct VulkanApi;
impl crate::api::traits::GraphicsApi for VulkanApi {
    type Root = VulkanInstance;
    type Device = VulkanPhysicalDevice;
    type DeviceProperties = VulkanPhysicalDeviceProperties;
    type DeviceFeatures = VulkanPhysicalDeviceFeatures;
    type Surface = VulkanSurface;
    type SurfaceCapabilities = VulkanSurfaceCapabilities;
    type Context = VulkanDevice;
    type Queue = VulkanQueue;
    type CommandPool = VulkanCommandPool;
    type CommandBuffer = VulkanCommandBuffer;
    type Swapchain = VulkanSwapchain;

    type ShaderCode<'a> = VulkanShaderCode<'a>;
    type Shader = VulkanShaderModule;

    type PipelineLayout = VulkanPipelineLayout;
    type RenderPass = VulkanRenderPass;

    type VertexInputState = VulkanPipelineVertexInputStateCreateInfo;
    type InputAssemblyState = VulkanPipelineInputAssemblyStateCreateInfo;
    type RasterizationState = VulkanPipelineRasterizationStateCreateInfo;
    type GraphicsPipeline = VulkanGraphicsPipeline;

    const FORMAT_R8G8B8A8_UNORM: i32 = VK_FORMAT_R8G8B8A8_UNORM;
    const FORMAT_R8G8B8A8_UNORM_SRGB: i32 = VK_FORMAT_R8G8B8A8_SRGB;
    const FORMAT_B8G8R8A8_UNORM: i32 = VK_FORMAT_B8G8R8A8_UNORM;
    const FORMAT_B8G8R8A8_UNORM_SRGB: i32 = VK_FORMAT_B8G8R8A8_SRGB;
    const FORMAT_R16G16B16A16_FLOAT: i32 = VK_FORMAT_R16G16B16A16_SFLOAT;
}

pub trait VulkanObject {
    type Handle;

    fn handle(&self) -> Self::Handle;
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

impl Into<VkCompositeAlphaFlagBitsKHR> for crate::CompositeAlpha {
    fn into(self) -> VkCompositeAlphaFlagBitsKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<VkFormat> for crate::RenderTargetFormat {
    fn into(self) -> VkFormat {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkFormat> for crate::RenderTargetFormat {
    fn from(value: VkFormat) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<VkColorSpaceKHR> for crate::Colorspace {
    fn into(self) -> VkColorSpaceKHR {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<VkColorSpaceKHR> for crate::Colorspace {
    fn from(value: VkColorSpaceKHR) -> Self {
        unsafe { std::mem::transmute(value) }
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

impl Into<VkShaderStageFlagBits> for crate::ShaderStage {
    fn into(self) -> VkShaderStageFlagBits {
        unsafe { std::mem::transmute(self) }
    }
}
