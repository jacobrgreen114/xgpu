// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

pub mod constants;

use crate::*;
use std::fmt::Debug;
use std::time::Duration;

// todo : implement thin types to prevent unnecessary atomic reference counting

pub trait GraphicsApi: Sized {
    type Root: ApiRoot<Self>;

    type Device: Device<Self>;
    type DeviceProperties: DeviceProperties<Self>;

    // type DeviceFeatures: DeviceFeatures<Self>;

    type Surface: Surface<Self>;
    type SurfaceCapabilities: SurfaceCapabilities<Self>;

    type Context: Context<Self>;
    type Queue: Queue<Self>;
    type CommandPool: CommandPool<Self>;
    type CommandBuffer: CommandBuffer<Self>;

    type Swapchain: Swapchain<Self>;
    type Image: Image<Self>;
    type ImageView: ImageView<Self>;

    type Shader: Shader<Self>;

    type RenderPass: RenderPass<Self>;
    type Framebuffer: Framebuffer<Self>;

    type PipelineLayout: PipelineLayout<Self>;

    // type VertexInputState: VertexInputStateCreateInfo<Self>;
    // type InputAssemblyState: InputAssemblyStateCreateInfo<Self>;
    // type RasterizationState: RasterizationStateCreateInfo<Self>;
    type GraphicsPipeline: GraphicsPipeline<Self>;
    //
    type Fence: Fence<Self>;
    type Semaphore: Semaphore<Self>;

    type DeviceTypeConstants: constants::DeviceTypeConstants;
    type CompositeAlphaConstants: constants::CompositeAlphaConstants;
    type ColorSpaceConstants: constants::ColorSpaceConstants;
    type DataFormatConstants: constants::DataFormatConstants;
    type PresentModeConstants: constants::PresentModeConstants;
    type BlendOpConstants: constants::BlendOpConstants;
    type BlendFactorConstants: constants::BlendFactorConstants;
    type ColorComponentConstants: constants::ColorComponentConstants;
    type PolygonModeConstants: constants::PolygonModeConstants;
    type CullModeConstants: constants::CullModeConstants;
    type FrontFaceConstants: constants::FrontFaceConstants;
    type PrimitiveTopologyConstants: constants::PrimitiveTopologyConstants;
}

pub trait ApiRoot<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(create_info: &RootCreateInfo) -> crate::Result<Self>;
    fn devices(&self) -> &[A::Device];
}

pub trait Device<A: GraphicsApi>: Sized + Debug + Clone {
    fn properties(&self) -> &A::DeviceProperties;
    // fn features(&self) -> &A::DeviceFeatures;
    //

    fn supports_surface(&self, surface: A::Surface) -> bool;

    fn get_surface_capabilities(
        &self,
        surface: A::Surface,
    ) -> crate::Result<A::SurfaceCapabilities>;

    fn get_surface_formats(&self, surface: A::Surface) -> crate::Result<Vec<SurfaceFormat>>;

    fn get_surface_present_modes(&self, surface: A::Surface) -> crate::Result<Vec<PresentMode>>;
}

pub trait DeviceProperties<A: GraphicsApi>: Sized + Debug {
    fn name(&self) -> &str;
    fn device_type(&self) -> DeviceType;
    fn vendor(&self) -> Vendor;
}

#[cfg(not(feature = "directx_api"))]
pub trait DeviceFeatures<A: GraphicsApi>: Sized + Debug {}

// #[cfg(not(feature = "directx"))]
pub trait Surface<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(root: A::Root, create_info: SurfaceCreateInfo) -> crate::Result<Self>;
}

pub trait SurfaceCapabilities<A: GraphicsApi>: Sized + Debug {
    fn min_image_count(&self) -> u32;
    fn max_image_count(&self) -> u32;
    fn current_extent(&self) -> Extent2D;
    fn min_image_extent(&self) -> Extent2D;
    fn max_image_extent(&self) -> Extent2D;
    fn max_image_array_layers(&self) -> u32;
}

// #[cfg(not(feature = "directx"))]
pub trait Context<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(root: A::Root, device: A::Device, create_info: ContextCreateInfo)
        -> crate::Result<Self>;

    fn queues(&self) -> &[<A as GraphicsApi>::Queue];
}

// #[cfg(not(feature = "directx"))]
pub trait Queue<A: GraphicsApi>: Sized + Debug + Clone {}

pub trait CommandPool<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: CommandPoolCreateInfo) -> crate::Result<Self>;
}

pub trait CommandBuffer<A: GraphicsApi>: Sized + Debug + Clone {
    // type RecordContext: CommandBufferRecordContext;

    fn allocate(
        pool: A::CommandPool,
        create_info: CommandBufferAllocateInfo,
    ) -> crate::Result<Self>;
    //
    // fn record<T, F>(&mut self, f: F) -> crate::Result<T>
    // where
    //     F: FnOnce(Self::RecordContext) -> T;
}

// pub trait CommandBufferRecordContext: Sized {
//     type RenderPassRecordContext: RenderPassRecordContext;
//
//     fn render_pass<T, F>(&self, begin_info: RenderPassBeginInfo, f: F) -> crate::Result<T>
//     where
//         F: FnOnce(Self::RenderPassRecordContext) -> T;
// }
//
// pub trait RenderPassRecordContext: Sized {}
//
pub trait Swapchain<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(
        context: A::Context,
        surface: A::Surface,
        create_info: &SwapchainCreateInfo,
    ) -> crate::Result<Self>;

    fn images(&self) -> &[A::Image];

    fn acquire_next_image(
        &self,
        timeout: Option<Duration>,
        semaphore: Option<A::Semaphore>,
        fence: Option<A::Fence>,
    ) -> crate::Result<u32>;
}

pub trait Image<A: GraphicsApi>: Sized + Debug + Clone {}

pub trait ImageView<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(
        context: A::Context,
        image: A::Image,
        create_info: ImageViewCreateInfo,
    ) -> crate::Result<Self>;
}

//
pub trait RenderPass<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: RenderPassCreateInfo) -> crate::Result<Self>;
}

pub trait Framebuffer<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: FramebufferCreateInfo) -> crate::Result<Self>;
}

pub trait PipelineLayout<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: PipelineLayoutCreateInfo) -> crate::Result<Self>;
}

pub trait Shader<A: GraphicsApi>: Sized + Debug + Clone {
    // fn new<'a>(context: A::Context, code: &A::ShaderCode<'a>) -> crate::Result<Self>;

    fn from_code(context: A::Context, code: ShaderCode) -> crate::Result<Self>;
}

// pub trait VertexInputStateCreateInfo<A: GraphicsApi>: Sized + Debug + Default {}
// pub trait InputAssemblyStateCreateInfo<A: GraphicsApi>: Sized + Debug + Default {}
// pub trait RasterizationStateCreateInfo<A: GraphicsApi>: Sized + Debug + Default {}
//
pub trait GraphicsPipeline<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: GraphicsPipelineCreateInfo) -> crate::Result<Self>;
}

pub trait Fence<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: FenceCreateInfo) -> crate::Result<Self>;
}

pub trait Semaphore<A: GraphicsApi>: Sized + Debug + Clone {
    fn new(context: A::Context, create_info: SemaphoreCreateInfo) -> crate::Result<Self>;
}
