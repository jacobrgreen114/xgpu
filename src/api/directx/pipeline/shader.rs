// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::directx::DirectXApi;
use crate::ShaderCode;
use std::sync::Arc;
use windows::Win32::Graphics::Direct3D12::D3D12_SHADER_BYTECODE;

enum Code {
    Static(&'static [u8]),
    Dynamic(Vec<u8>),
}

struct ShaderData {
    code: Code,
    device: directx_type!(Context),
}

#[derive(Clone)]
pub struct DirectXShader {
    data: Arc<ShaderData>,
}

impl DirectXShader {
    pub(crate) fn code(&self) -> &[u8] {
        match &self.data.code {
            Code::Static(code) => code,
            Code::Dynamic(code) => code,
        }
    }
}

impl Into<D3D12_SHADER_BYTECODE> for DirectXShader {
    fn into(self) -> D3D12_SHADER_BYTECODE {
        let code = self.code();
        D3D12_SHADER_BYTECODE {
            pShaderBytecode: code.as_ptr() as *const _,
            BytecodeLength: code.len(),
        }
    }
}

impl std::fmt::Debug for DirectXShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl crate::api::traits::Shader<DirectXApi> for DirectXShader {
    fn from_code(context: directx_type!(Context), code: ShaderCode) -> crate::Result<Self> {
        let code = match code {
            ShaderCode::Static(code) => Code::Static(code),
            ShaderCode::Dynamic(code) => Code::Dynamic(code.to_vec()),
        };

        let data = Arc::new(ShaderData {
            code,
            device: context,
        });

        Ok(Self { data })
    }
}
