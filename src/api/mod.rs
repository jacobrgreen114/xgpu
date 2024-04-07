// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "directx")]
pub mod directx;

// todo : implement thin types to prevent unnecessary atomic reference counting

#[cfg(feature = "vulkan")]
pub type Api = api::vulkan::VulkanApi;

#[cfg(feature = "directx")]
pub type Api = api::directx::DirectXApi;

pub(crate) mod traits {
    use crate::*;
    use std::fmt::Debug;

    pub trait GraphicsApi: Sized {
        type Root: ApiRoot<Self>;

        type Device: Device<Self>;
        type DeviceProperties: DeviceProperties<Self>;

        #[cfg(not(feature = "directx"))]
        type DeviceFeatures: DeviceFeatures<Self>;

        // #[cfg(not(feature = "directx"))]
        type Surface: Surface<Self>;
        #[cfg(not(feature = "directx"))]
        type SurfaceCapabilities: SurfaceCapabilities<Self>;

        type Context: Context<Self>;
        type Queue: Queue<Self>;
        type CommandPool: CommandPool<Self>;
        type CommandBuffer: CommandBuffer<Self>;

        // #[cfg(not(feature = "directx"))]
        type Swapchain: Swapchain<Self>;

        #[cfg(not(feature = "directx"))]
        type ShaderCode<'a>: ShaderCode<Self>;
        #[cfg(not(feature = "directx"))]
        type Shader: Shader<Self>;

        type PipelineLayout: PipelineLayout<Self>;
        type RenderPass: RenderPass<Self>;

        type VertexInputState: VertexInputStateCreateInfo<Self>;
        type InputAssemblyState: InputAssemblyStateCreateInfo<Self>;
        type RasterizationState: RasterizationStateCreateInfo<Self>;
        type GraphicsPipeline: GraphicsPipeline<Self>;

        const FORMAT_R8G8B8A8_UNORM: i32;
        const FORMAT_R8G8B8A8_UNORM_SRGB: i32;

        const FORMAT_B8G8R8A8_UNORM: i32;
        const FORMAT_B8G8R8A8_UNORM_SRGB: i32;

        const FORMAT_R16G16B16A16_FLOAT: i32;
    }

    pub trait ApiRoot<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(create_info: &RootCreateInfo) -> crate::Result<Self>;
        fn devices(&self) -> &[A::Device];
    }

    pub trait Device<A: GraphicsApi>: Sized + Debug + Clone {
        fn properties(&self) -> &A::DeviceProperties;
        fn features(&self) -> &A::DeviceFeatures;

        fn supports_surface(&self, surface: A::Surface) -> bool;
        fn get_surface_capabilities(
            &self,
            surface: A::Surface,
        ) -> crate::Result<A::SurfaceCapabilities>;

        fn get_surface_formats(&self, surface: A::Surface) -> crate::Result<Vec<SurfaceFormat>>;

        fn get_surface_present_modes(&self, surface: A::Surface)
            -> crate::Result<Vec<PresentMode>>;
    }

    pub trait DeviceProperties<A: GraphicsApi>: Sized + Debug {
        fn name(&self) -> &str;
        fn device_type(&self) -> DeviceType;
        fn vendor(&self) -> Vendor;
    }

    #[cfg(not(feature = "directx"))]
    pub trait DeviceFeatures<A: GraphicsApi>: Sized + Debug {}

    // #[cfg(not(feature = "directx"))]
    pub trait Surface<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(root: A::Root, create_info: SurfaceCreateInfo) -> crate::Result<Self>;
    }

    #[cfg(not(feature = "directx"))]
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
        fn new(
            root: A::Root,
            device: A::Device,
            create_info: ContextCreateInfo,
        ) -> crate::Result<Self>;

        fn queues(&self) -> &[<A as GraphicsApi>::Queue];
    }

    // #[cfg(not(feature = "directx"))]
    pub trait Queue<A: GraphicsApi>: Sized + Debug + Clone {}

    pub trait CommandPool<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(context: A::Context, create_info: CommandPoolCreateInfo) -> crate::Result<Self>;
    }

    pub trait CommandBuffer<A: GraphicsApi>: Sized + Debug + Clone {}

    // #[cfg(not(feature = "directx"))]
    pub trait Swapchain<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(
            surface: A::Surface,
            context: A::Context,
            create_info: &SwapchainCreateInfo,
        ) -> crate::Result<Self>;
    }

    #[cfg(not(feature = "directx"))]
    pub trait ShaderCode<A: GraphicsApi>: Sized + Debug + Clone {}

    #[cfg(not(feature = "directx"))]
    pub trait Shader<A: GraphicsApi>: Sized + Debug + Clone {
        fn new<'a>(context: A::Context, code: &A::ShaderCode<'a>) -> crate::Result<Self>;
    }

    pub trait PipelineLayout<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(context: A::Context, create_info: PipelineLayoutCreateInfo) -> crate::Result<Self>;
    }

    pub trait RenderPass<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(context: A::Context, create_info: RenderPassCreateInfo) -> crate::Result<Self>;
    }

    pub trait VertexInputStateCreateInfo<A: GraphicsApi>: Sized + Debug + Default {}
    pub trait InputAssemblyStateCreateInfo<A: GraphicsApi>: Sized + Debug + Default {}
    pub trait RasterizationStateCreateInfo<A: GraphicsApi>: Sized + Debug + Default {}

    pub trait GraphicsPipeline<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(context: A::Context, create_info: GraphicsPipelineCreateInfo)
            -> crate::Result<Self>;
    }
}
