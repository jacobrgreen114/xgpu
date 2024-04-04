// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#[cfg(feature = "vulkan")]
pub mod vulkan;

pub(crate) mod traits {
    use crate::{
        ContextCreateInfo, DeviceQuery, DeviceType, Extent2D, PresentMode, RootCreateInfo,
        SurfaceCreateInfo, SurfaceFormat, SwapchainCreateInfo, Vendor,
    };
    use std::fmt::Debug;

    pub trait GraphicsApi: Sized {
        type Root: ApiRoot<Self>;
        type Device: Device<Self>;

        type Surface: Surface<Self>;
        type SurfaceCapabilities: SurfaceCapabilities<Self>;

        type Context: Context<Self>;
        type Queue: Queue<Self>;

        type Swapchain: Swapchain<Self>;
    }

    pub trait ApiRoot<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(create_info: RootCreateInfo) -> Result<Self, ()>;
        fn devices(&self) -> &[A::Device];
    }

    pub trait Device<A: GraphicsApi>: Sized + Debug + Clone {
        fn name(&self) -> &str;
        fn device_type(&self) -> DeviceType;
        fn vendor(&self) -> Vendor;

        fn supports_surface(&self, surface: A::Surface) -> bool;
        fn get_surface_capabilities(
            &self,
            surface: A::Surface,
        ) -> crate::Result<A::SurfaceCapabilities>;

        fn get_surface_formats(&self, surface: A::Surface) -> crate::Result<Vec<SurfaceFormat>>;

        fn get_surface_present_modes(&self, surface: A::Surface)
            -> crate::Result<Vec<PresentMode>>;
    }

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

    pub trait Context<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(
            root: A::Root,
            device: A::Device,
            create_info: ContextCreateInfo,
        ) -> Result<Self, ()>;

        fn queues(&self) -> &[<A as GraphicsApi>::Queue];
    }

    pub trait Queue<A: GraphicsApi>: Sized + Debug + Clone {}

    pub trait Swapchain<A: GraphicsApi>: Sized + Debug + Clone {
        fn new(
            surface: A::Surface,
            context: A::Context,
            create_info: &SwapchainCreateInfo,
        ) -> crate::Result<Self>;
    }
}
