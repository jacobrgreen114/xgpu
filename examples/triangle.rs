// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

extern crate glfw;

use std::ptr::null_mut;
use xgpu::prelude::*;

macro_rules! scoped_timer {
    ($name:expr) => {
        let _timer = crate::ScopedTimer::new($name);
    };
}

fn main() {
    // scoped_timer!("main");

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

    let root = {
        scoped_timer!("root");
        xgpu::Root::new(xgpu::RootCreateInfo {}).unwrap()
    };
    dbg!(&root);

    let surface = {
        scoped_timer!("surface");
        #[cfg(target_os = "windows")]
        xgpu::Surface::new(
            root.clone(),
            xgpu::SurfaceCreateInfo {
                hinstance: null_mut(),
                hwnd: window.get_win32_window() as *mut _,
            },
        )
        .unwrap()
    };
    // dbg!(&surface);

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
    let selected_format = surface_formats.first().unwrap();

    let present_modes = device.get_surface_present_modes(surface.clone()).unwrap();
    // dbg!(&present_modes);
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

    let swapchain = {
        let create_info = xgpu::SwapchainCreateInfo {
            min_image_count: 2.clamp(
                capabilities.min_image_count(),
                capabilities.max_image_count(),
            ),
            format: selected_format.format,
            colorspace: selected_format.colorspace,
            extent: capabilities.current_extent(),
            composite_alpha: xgpu::CompositeAlpha::Opaque,
            present_mode: selected_present_mode.clone(),
        };
        dbg!(&create_info);

        scoped_timer!("swapchain");
        xgpu::Swapchain::new(surface.clone(), context.clone(), &create_info).unwrap()
    };

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
        println!("{}: {:?}", self.name, self.start.elapsed());
    }
}
