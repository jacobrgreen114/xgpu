// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

extern crate env_logger;
extern crate glfw;

use std::panic::panic_any;
use std::ptr::null_mut;
use xgpu::prelude::*;

use windows::Win32::Foundation::HWND;
use xgpu::ShaderCode;

macro_rules! scoped_timer {
    ($name:expr) => {
        let _timer = crate::ScopedTimer::new($name);
    };
}

static VERTEX_SHADER: &[u8] = include_bytes!("../shaders/triangle.vert.spv");
static FRAGMENT_SHADER: &[u8] = include_bytes!("../shaders/triangle.frag.spv");

fn init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();
}

fn main() {
    init();

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            800,
            600,
            "xgpu triangle example",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    let init_timer = ScopedTimer::new("init");

    let root = {
        scoped_timer!("root");
        xgpu::Root::new(&xgpu::RootCreateInfo {}).unwrap()
    };

    let surface = {
        scoped_timer!("surface");
        #[cfg(target_os = "windows")]
        xgpu::Surface::new(
            root.clone(),
            xgpu::SurfaceCreateInfo::new(HWND(window.get_win32_window() as _)),
            //xgpu::SurfaceCreateInfo::new(HWND(0)),
        )
        .unwrap()
    };

    let device = {
        scoped_timer!("device");
        root.devices().first().unwrap().clone()
    };
    // dbg!(&device);

    device
        .supports_surface(surface.clone())
        .then_some(())
        .unwrap();

    let capabilities = device.get_surface_capabilities(surface.clone()).unwrap();
    // dbg!(&capabilities);

    let surface_formats = device.get_surface_formats(surface.clone()).unwrap();
    // dbg!(&surface_formats);

    let present_modes = device.get_surface_present_modes(surface.clone()).unwrap();
    // dbg!(&present_modes);

    let selected_format = surface_formats.first().unwrap().clone();
    let selected_present_mode = present_modes.first().unwrap().clone();

    let context = {
        scoped_timer!("context");
        xgpu::Context::new(root.clone(), device, xgpu::ContextCreateInfo::default()).unwrap()
    };

    let queue = {
        scoped_timer!("queue");
        context.queues().first().unwrap().clone()
    };

    let swapchain = {
        scoped_timer!("swapchain");

        const PREFFERED_BUFFER_COUNT: u32 = 2;

        let image_count = PREFFERED_BUFFER_COUNT.clamp(
            capabilities.min_image_count(),
            capabilities.max_image_count(),
        );

        xgpu::Swapchain::new(
            context.clone(),
            surface.clone(),
            &xgpu::SwapchainCreateInfo {
                min_image_count: image_count,
                format: selected_format.format,
                colorspace: selected_format.colorspace,
                extent: capabilities.current_extent(),
                composite_alpha: xgpu::CompositeAlphaMode::Opaque,
                present_mode: selected_present_mode.clone(),
            },
        )
        .unwrap()
    };

    let swapchain_images = swapchain.images();

    let swapchain_views: Vec<_> = {
        scoped_timer!("swapchain_views");
        swapchain_images
            .iter()
            .map(|image| {
                xgpu::ImageView::new(
                    context.clone(),
                    image.clone(),
                    xgpu::ImageViewCreateInfo {
                        format: selected_format.format,
                    },
                )
                .unwrap()
            })
            .collect()
    };

    let render_pass = {
        scoped_timer!("render_pass");

        let attachments = &[xgpu::AttachmentDescription {
            format: selected_format.format,
        }];

        let subpasses = &[xgpu::SubpassDescription {
            color_attachments: &[xgpu::AttachmentReference { attachment: 0 }],
            ..Default::default()
        }];

        xgpu::RenderPass::new(
            context.clone(),
            xgpu::RenderPassCreateInfo {
                attachments,
                subpasses,
            },
        )
        .unwrap()
    };

    let pipeline_layout = {
        scoped_timer!("pipeline_layout");
        xgpu::PipelineLayout::new(context.clone(), xgpu::PipelineLayoutCreateInfo {}).unwrap()
    };

    let vertex_shader = {
        scoped_timer!("vertex_shader");
        xgpu::Shader::from_code(context.clone(), ShaderCode::Static(VERTEX_SHADER)).unwrap()
    };

    let fragment_shader = {
        scoped_timer!("fragment_shader");
        xgpu::Shader::from_code(context.clone(), ShaderCode::Static(FRAGMENT_SHADER)).unwrap()
    };

    let pipeline = {
        scoped_timer!("pipeline");

        let shaders = xgpu::ShaderStages {
            vertex: Some(vertex_shader.clone()),
            fragment: Some(fragment_shader.clone()),
            ..Default::default()
        };

        let rasterization = xgpu::RasterizationState {
            polygon_mode: xgpu::PolygonMode::Fill,
            cull_mode: xgpu::CullMode::Back,
            front_face: xgpu::FrontFace::CounterClockwise,
        };

        let blend = xgpu::BlendState {
            attachments: &[xgpu::BlendAttachmentState {
                blend_enable: false,
                ..Default::default()
            }],
            ..Default::default()
        };

        let create_info = xgpu::GraphicsPipelineCreateInfo {
            shaders,
            topology: xgpu::PrimitiveTopology::TriangleList,
            rasterization,
            blend,
            layout: pipeline_layout.clone(),
            render_pass: render_pass.clone(),
            subpass: 0,
        };

        xgpu::GraphicsPipeline::new(context.clone(), create_info).unwrap()
    };

    let framebuffers: Vec<_> = {
        scoped_timer!("framebuffers");
        swapchain_views
            .iter()
            .map(|view| {
                xgpu::Framebuffer::new(
                    context.clone(),
                    xgpu::FramebufferCreateInfo {
                        render_pass: render_pass.clone(),
                        extent: capabilities.current_extent(),
                        attachments: &[view.clone()],
                    },
                )
                .unwrap()
            })
            .collect()
    };

    let command_pool = {
        scoped_timer!("command_pool");
        xgpu::CommandPool::new(
            context.clone(),
            xgpu::CommandPoolCreateInfo {
                transient: false,
                reset: true,
            },
        )
        .unwrap()
    };

    let mut buffer = {
        scoped_timer!("buffer");
        xgpu::CommandBuffer::allocate(
            command_pool.clone(),
            xgpu::CommandBufferAllocateInfo {
                // level: xgpu::CommandBufferLevel::Primary,
                // count: 1,
            },
        )
        .unwrap()
    };

    let fence = {
        scoped_timer!("fence");
        xgpu::Fence::new(context.clone(), xgpu::FenceCreateInfo { signaled: false }).unwrap()
    };

    //
    // {
    //     scoped_timer!("record");
    //
    //     let record_renderpass = |context: xgpu::RenderPassRecordContext| {};
    //
    //     let record_buffer = |context: xgpu::CommandBufferRecordContext| {
    //         context
    //             .render_pass(
    //                 xgpu::RenderPassBeginInfo {
    //                     framebuffer: framebuffers.first().unwrap().clone(),
    //                     render_area: xgpu::Rect2D {
    //                         offset: xgpu::Offset2D { x: 0, y: 0 },
    //                         extent: capabilities.current_extent(),
    //                     },
    //                 },
    //                 record_renderpass,
    //             )
    //             .unwrap();
    //     };
    //
    //     buffer.record(record_buffer).unwrap();
    // }
    //
    //
    // let render_thread_proc = || {};
    //
    // let render_thread = std::thread::spawn(render_thread_proc);

    drop(init_timer);

    window.set_key_polling(true);
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    //render_thread.join().unwrap();
}

pub struct ScopedTimer<'a> {
    name: &'a str,
    start: std::time::Instant,
}

impl<'a> ScopedTimer<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for ScopedTimer<'_> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("{}: {:?}", self.name, elapsed);
    }
}
