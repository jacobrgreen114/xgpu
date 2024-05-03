// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use std::sync::Arc;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D12::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

use crate::api::directx::*;
use crate::convert::MapInto;
use crate::util::FailFast;
use crate::*;

struct GraphicsPipelineData {
    context: directx_type!(Context),
}

#[derive(Clone)]
pub struct DirectXGraphicsPipeline {
    pipeline: ID3D12PipelineState,
    data: Arc<GraphicsPipelineData>,
}

impl std::fmt::Debug for DirectXGraphicsPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl crate::api::traits::GraphicsPipeline<DirectXApi> for DirectXGraphicsPipeline {
    fn new(
        context: directx_type!(Context),
        create_info: GraphicsPipelineCreateInfo,
    ) -> crate::Result<Self> {
        let vs = create_info.shaders.vertex.map_into().unwrap_or_default();
        let ps = create_info.shaders.fragment.map_into().unwrap_or_default();
        let hs = create_info.shaders.tess_ctrl.map_into().unwrap_or_default();
        let ds = create_info.shaders.tess_eval.map_into().unwrap_or_default();
        let gs = create_info.shaders.geometry.map_into().unwrap_or_default();

        let input_layout = D3D12_INPUT_LAYOUT_DESC {
            pInputElementDescs: std::ptr::null(),
            NumElements: 0,
        };

        let depth_stencil_desc = D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: FALSE,
            DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ZERO,
            DepthFunc: D3D12_COMPARISON_FUNC_NEVER,
            StencilEnable: FALSE,
            StencilReadMask: 0,
            StencilWriteMask: 0,
            FrontFace: D3D12_DEPTH_STENCILOP_DESC {
                StencilFailOp: D3D12_STENCIL_OP_KEEP,
                StencilDepthFailOp: D3D12_STENCIL_OP_KEEP,
                StencilPassOp: D3D12_STENCIL_OP_KEEP,
                StencilFunc: D3D12_COMPARISON_FUNC_NEVER,
            },
            BackFace: D3D12_DEPTH_STENCILOP_DESC {
                StencilFailOp: D3D12_STENCIL_OP_KEEP,
                StencilDepthFailOp: D3D12_STENCIL_OP_KEEP,
                StencilPassOp: D3D12_STENCIL_OP_KEEP,
                StencilFunc: D3D12_COMPARISON_FUNC_NEVER,
            },
        };

        let raster_desc = D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID,
            CullMode: D3D12_CULL_MODE_BACK,
            FrontCounterClockwise: FALSE,
            DepthBias: D3D12_DEFAULT_DEPTH_BIAS,
            DepthBiasClamp: D3D12_DEFAULT_DEPTH_BIAS_CLAMP,
            SlopeScaledDepthBias: D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS,
            DepthClipEnable: TRUE,
            MultisampleEnable: FALSE,
            AntialiasedLineEnable: FALSE,
            ForcedSampleCount: 0,
            ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
        };

        let blend_desc = D3D12_BLEND_DESC {
            AlphaToCoverageEnable: FALSE,
            IndependentBlendEnable: FALSE,
            RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC {
                BlendEnable: FALSE,
                LogicOpEnable: FALSE,
                SrcBlend: D3D12_BLEND_ZERO,
                DestBlend: D3D12_BLEND_ZERO,
                BlendOp: D3D12_BLEND_OP_ADD,
                SrcBlendAlpha: D3D12_BLEND_ZERO,
                DestBlendAlpha: D3D12_BLEND_ZERO,
                BlendOpAlpha: D3D12_BLEND_OP_ADD,
                LogicOp: D3D12_LOGIC_OP_NOOP,
                RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
            }; 8],
        };

        let desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: Default::default(),
            VS: vs,
            PS: ps,
            DS: ds,
            HS: hs,
            GS: gs,
            StreamOutput: Default::default(),
            BlendState: blend_desc,
            SampleMask: 0,
            RasterizerState: raster_desc,
            DepthStencilState: depth_stencil_desc,
            InputLayout: input_layout,
            IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED,
            PrimitiveTopologyType: create_info.topology.into(),
            NumRenderTargets: 0,
            RTVFormats: [],
            DSVFormat: Default::default(),
            SampleDesc: Default::default(),
            NodeMask: 0,
            CachedPSO: Default::default(),
            Flags: D3D12_PIPELINE_STATE_FLAG_NONE,
        };

        let pipeline =
            unsafe { context.handle().CreateGraphicsPipelineState(&desc) }.fail_fast()?;

        let data = Arc::new(GraphicsPipelineData { context });

        Ok(Self { pipeline, data })
    }
}

impl Into<D3D12_PRIMITIVE_TOPOLOGY_TYPE> for PrimitiveTopology {
    fn into(self) -> D3D12_PRIMITIVE_TOPOLOGY_TYPE {
        match self {
            PrimitiveTopology::PointList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT,
            PrimitiveTopology::LineList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            PrimitiveTopology::LineStrip => D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            PrimitiveTopology::TriangleList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            PrimitiveTopology::TriangleStrip => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            PrimitiveTopology::TriangleFan => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            PrimitiveTopology::LineListWithAdjacency => D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            PrimitiveTopology::LineStripWithAdjacency => D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            PrimitiveTopology::TriangleListWithAdjacency => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            PrimitiveTopology::TriangleStripWithAdjacency => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            PrimitiveTopology::PatchList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH,
        }
    }
}
