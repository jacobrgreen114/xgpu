// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::vulkan::{Ownership, VulkanApi, VulkanObject};
use std::fmt::Debug;
use vulkan_sys::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct VulkanShaderCode<'a> {
    code: &'a [u8],
}

impl<'a> crate::api::traits::ShaderCode<VulkanApi> for VulkanShaderCode<'a> {}

impl<'a> From<&'a [u8]> for VulkanShaderCode<'a> {
    fn from(code: &'a [u8]) -> Self {
        Self { code }
    }
}

struct ShaderModuleOwnership {
    handle: VkShaderModule,
    device: crate::api::vulkan::device::VulkanDevice,
}

impl Drop for ShaderModuleOwnership {
    fn drop(&mut self) {
        vk::destroy_shader_module(
            vkDestroyShaderModule,
            self.device.handle(),
            self.handle,
            None,
        );
    }
}

#[derive(Clone)]
pub struct VulkanShaderModule {
    handle: VkShaderModule,
    ownership: Ownership<ShaderModuleOwnership>,
}

impl Debug for VulkanShaderModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

impl VulkanObject for VulkanShaderModule {
    type Handle = VkShaderModule;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

impl crate::api::traits::Shader<VulkanApi> for VulkanShaderModule {
    fn new(
        context: <VulkanApi as GraphicsApi>::Context,
        code: &VulkanShaderCode,
    ) -> crate::Result<Self> {
        let create_info = VkShaderModuleCreateInfo {
            sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            codeSize: code.code.len(),
            pCode: code.code.as_ptr() as *const u32,
        };

        let handle =
            vk::create_shader_module(vkCreateShaderModule, context.handle(), &create_info, None)?;

        let ownership = Ownership::new(ShaderModuleOwnership {
            handle,
            device: context,
        });

        Ok(Self { handle, ownership })
    }
}
