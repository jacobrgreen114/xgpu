// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

extern crate env_logger;
extern crate glfw;

use std::ptr::null_mut;
use xgpu::prelude::*;

use windows::Win32::Foundation::HWND;

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
    // dbg!(&root);

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
    // dbg!(&surface);

    let device = {
        scoped_timer!("device");
        root.devices().first().unwrap().clone()
    };
    // dbg!(&device);

    #[cfg(not(feature = "directx"))]
    device
        .supports_surface(surface.clone())
        .then_some(())
        .unwrap();

    #[cfg(not(feature = "directx"))]
    let capabilities = device.get_surface_capabilities(surface.clone()).unwrap();
    // dbg!(&capabilities);

    #[cfg(not(feature = "directx"))]
    let surface_formats = device.get_surface_formats(surface.clone()).unwrap();
    // dbg!(&surface_formats);

    #[cfg(not(feature = "directx"))]
    let selected_format = surface_formats.first().unwrap();

    #[cfg(not(feature = "directx"))]
    let present_modes = device.get_surface_present_modes(surface.clone()).unwrap();
    // dbg!(&present_modes);

    #[cfg(not(feature = "directx"))]
    let selected_present_mode = present_modes.first().unwrap();

    let context = {
        scoped_timer!("context");
        xgpu::Context::new(root.clone(), device, xgpu::ContextCreateInfo::default()).unwrap()
    };
    // dbg!(&context);

    let queue = {
        scoped_timer!("queue");
        context.queues()[0].clone()
    };
    // dbg!(&queue);

    let command_pool = {
        scoped_timer!("command_pool");
        xgpu::CommandPool::new(context.clone(), xgpu::CommandPoolCreateInfo::default()).unwrap()
    };

    #[cfg(not(feature = "directx"))]
    let swapchain = {
        scoped_timer!("swapchain");

        const PREFFERED_BUFFER_COUNT: u32 = 2;

        let image_count = u32::clamp(
            PREFFERED_BUFFER_COUNT,
            capabilities.min_image_count(),
            capabilities.max_image_count(),
        );

        xgpu::Swapchain::new(
            surface.clone(),
            context.clone(),
            &xgpu::SwapchainCreateInfo {
                min_image_count: image_count,
                format: selected_format.format,
                colorspace: selected_format.colorspace,
                extent: capabilities.current_extent(),
                composite_alpha: xgpu::CompositeAlpha::Opaque,
                present_mode: selected_present_mode.clone(),
            },
        )
        .unwrap()
    };

    #[cfg(not(feature = "directx"))]
    let vertex_shader = {
        scoped_timer!("vertex_shader");
        xgpu::Shader::new(context.clone(), &xgpu::ShaderCode::from(VERTEX_SHADER)).unwrap()
    };

    #[cfg(not(feature = "directx"))]
    let fragment_shader = {
        scoped_timer!("fragment_shader");
        xgpu::Shader::new(context.clone(), &xgpu::ShaderCode::from(FRAGMENT_SHADER)).unwrap()
    };

    let pipeline_layout = {
        scoped_timer!("pipeline_layout");
        xgpu::PipelineLayout::new(context.clone(), xgpu::PipelineLayoutCreateInfo {}).unwrap()
    };

    let render_pass = {
        scoped_timer!("render_pass");
        xgpu::RenderPass::new(context.clone(), xgpu::RenderPassCreateInfo {}).unwrap()
    };

    #[cfg(not(feature = "directx"))]
    let pipeline = {
        scoped_timer!("pipeline");

        let shaders = &[
            xgpu::ShaderStageCreateInfo {
                module: vertex_shader,
                stage: xgpu::ShaderStage::Vertex,
                entry: c"main",
            },
            xgpu::ShaderStageCreateInfo {
                module: fragment_shader,
                stage: xgpu::ShaderStage::Fragment,
                entry: c"main",
            },
        ];

        let vertex_input_state = xgpu::VertexInputState::default();

        let input_assembly_state = xgpu::InputAssemblyState::default();

        let rasterization_state = xgpu::RasterizationState::default();

        let create_info = xgpu::GraphicsPipelineCreateInfo {
            shader_stages: shaders,
            vertex_input_state: &vertex_input_state,
            input_assembly_state: &input_assembly_state,
            rasterization_state: &rasterization_state,
            layout: pipeline_layout,
            render_pass,
        };

        xgpu::GraphicsPipeline::new(context.clone(), create_info).unwrap()
    };
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
