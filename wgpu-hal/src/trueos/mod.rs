#![allow(unused_variables)]

use alloc::{string::String, vec::Vec};
use core::time::Duration;

use crate::TlasInstance;

mod buffer;
pub mod integration;
pub use buffer::Buffer;
mod command;
pub use command::CommandBuffer;

#[derive(Clone, Debug)]
pub struct Api;

#[derive(Debug)]
pub struct Context {
    probe: integration::AdapterProbe,
}
#[derive(Debug)]
pub enum Resource {}

#[derive(Debug)]
pub enum Fence {}

type DeviceResult<T> = Result<T, crate::DeviceError>;

impl crate::Api for Api {
    const VARIANT: wgt::Backend = wgt::Backend::TrueOs;

    type Instance = Context;
    type Surface = Context;
    type Adapter = Context;
    type Device = Context;

    type Queue = Context;
    type CommandEncoder = CommandBuffer;
    type CommandBuffer = CommandBuffer;

    type Buffer = Buffer;
    type Texture = Resource;
    type SurfaceTexture = Resource;
    type TextureView = Resource;
    type Sampler = Resource;
    type QuerySet = Resource;
    type Fence = Fence;
    type AccelerationStructure = Resource;
    type PipelineCache = Resource;

    type BindGroupLayout = Resource;
    type BindGroup = Resource;
    type PipelineLayout = Resource;
    type ShaderModule = Resource;
    type RenderPipeline = Resource;
    type RayTracingPipeline = Resource;
    type ComputePipeline = Resource;
}

crate::impl_dyn_resource!(Buffer, CommandBuffer, Context, Fence, Resource);

impl crate::DynAccelerationStructure for Resource {}
impl crate::DynBindGroup for Resource {}
impl crate::DynBindGroupLayout for Resource {}
impl crate::DynBuffer for Buffer {}
impl crate::DynCommandBuffer for CommandBuffer {}
impl crate::DynComputePipeline for Resource {}
impl crate::DynFence for Fence {}
impl crate::DynPipelineCache for Resource {}
impl crate::DynPipelineLayout for Resource {}
impl crate::DynQuerySet for Resource {}
impl crate::DynRenderPipeline for Resource {}
impl crate::DynRayTracingPipeline for Resource {}
impl crate::DynSampler for Resource {}
impl crate::DynShaderModule for Resource {}
impl crate::DynSurfaceTexture for Resource {}
impl crate::DynTexture for Resource {}
impl crate::DynTextureView for Resource {}

impl core::borrow::Borrow<dyn crate::DynTexture> for Resource {
    fn borrow(&self) -> &dyn crate::DynTexture {
        self
    }
}

impl crate::Instance for Context {
    type A = Api;

    unsafe fn init(_desc: &crate::InstanceDescriptor<'_>) -> Result<Self, crate::InstanceError> {
        Ok(Self {
            probe: integration::probe().map_err(crate::InstanceError::new)?,
        })
    }
    unsafe fn create_surface(
        &self,
        _display_handle: raw_window_handle::RawDisplayHandle,
        _window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<Context, crate::InstanceError> {
        Err(crate::InstanceError::new(String::from(
            "TRUEOS surface support is not implemented",
        )))
    }
    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&Context>,
    ) -> Vec<crate::ExposedAdapter<Api>> {
        Vec::new()
    }
}

impl Context {
    pub fn probe_info(&self) -> &integration::AdapterProbe {
        &self.probe
    }
}

impl crate::Surface for Context {
    type A = Api;

    unsafe fn configure(
        &self,
        _device: &Context,
        _config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        Err(crate::SurfaceError::Other(
            "TRUEOS surface support is not implemented",
        ))
    }

    unsafe fn unconfigure(&self, _device: &Context) {}

    unsafe fn acquire_texture(
        &self,
        _timeout: Option<Duration>,
        _fence: &Fence,
    ) -> Result<crate::AcquiredSurfaceTexture<Api>, crate::SurfaceError> {
        Err(crate::SurfaceError::Other(
            "TRUEOS surface support is not implemented",
        ))
    }

    unsafe fn discard_texture(&self, _texture: Resource) {}
}

impl crate::Adapter for Context {
    type A = Api;

    unsafe fn open(
        &self,
        _features: wgt::Features,
        _limits: &wgt::Limits,
        _memory_hints: &wgt::MemoryHints,
    ) -> DeviceResult<crate::OpenDevice<Api>> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> crate::TextureFormatCapabilities {
        crate::TextureFormatCapabilities::empty()
    }

    unsafe fn surface_capabilities(&self, surface: &Context) -> Option<crate::SurfaceCapabilities> {
        None
    }

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        wgt::PresentationTimestamp::INVALID_TIMESTAMP
    }

    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses {
        wgt::BufferUses::empty()
    }

    fn get_ordered_texture_usages(&self) -> wgt::TextureUses {
        wgt::TextureUses::empty()
    }
}

impl crate::Queue for Context {
    type A = Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&CommandBuffer],
        surface_textures: &[&Resource],
        (fence, fence_value): (&Fence, crate::FenceValue),
    ) -> DeviceResult<()> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn present(
        &self,
        surface: &Context,
        texture: Resource,
    ) -> Result<(), crate::SurfaceError> {
        Err(crate::SurfaceError::Other(
            "TRUEOS presentation is not implemented",
        ))
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        panic!("TRUEOS timestamps are not implemented")
    }

    unsafe fn wait_for_idle(&self) -> Result<(), crate::DeviceError> {
        Err(crate::DeviceError::Unexpected)
    }
}

impl crate::Device for Context {
    type A = Api;

    unsafe fn create_buffer(
        &self,
        _desc: &crate::BufferDescriptor,
    ) -> DeviceResult<(Buffer, wgt::BufferAddress)> {
        Err(crate::DeviceError::Unexpected)
    }

    unsafe fn destroy_buffer(&self, buffer: Buffer) {}
    unsafe fn add_raw_buffer(&self, _buffer: &Buffer) {}

    unsafe fn map_buffer(
        &self,
        _buffer: &Buffer,
        _range: crate::MemoryRange,
    ) -> DeviceResult<crate::BufferMapping> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn unmap_buffer(&self, buffer: &Buffer) {}
    unsafe fn flush_mapped_ranges<I>(&self, buffer: &Buffer, ranges: I) {}
    unsafe fn invalidate_mapped_ranges<I>(&self, buffer: &Buffer, ranges: I) {}

    unsafe fn create_texture(&self, desc: &crate::TextureDescriptor) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_texture(&self, texture: Resource) {}
    unsafe fn add_raw_texture(&self, _texture: &Resource) {}

    unsafe fn create_texture_view(
        &self,
        texture: &Resource,
        desc: &crate::TextureViewDescriptor,
    ) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_texture_view(&self, view: Resource) {}
    unsafe fn create_sampler(&self, desc: &crate::SamplerDescriptor) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_sampler(&self, sampler: Resource) {}

    unsafe fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<Context>,
    ) -> DeviceResult<CommandBuffer> {
        Err(crate::DeviceError::Unexpected)
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_bind_group_layout(&self, bg_layout: Resource) {}
    unsafe fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<Resource>,
    ) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_pipeline_layout(&self, pipeline_layout: Resource) {}
    unsafe fn create_bind_group(
        &self,
        desc: &crate::BindGroupDescriptor<Resource, Buffer, Resource, Resource, Resource>,
    ) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_bind_group(&self, group: Resource) {}

    unsafe fn create_shader_module(
        &self,
        desc: &crate::ShaderModuleDescriptor,
        shader: crate::ShaderInput,
    ) -> Result<Resource, crate::ShaderError> {
        Err(crate::DeviceError::Unexpected.into())
    }
    unsafe fn destroy_shader_module(&self, module: Resource) {}
    unsafe fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<Resource, Resource, Resource>,
    ) -> Result<Resource, crate::PipelineError> {
        Err(crate::DeviceError::Unexpected.into())
    }
    unsafe fn destroy_render_pipeline(&self, pipeline: Resource) {}
    unsafe fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<Resource, Resource, Resource>,
    ) -> Result<Resource, crate::PipelineError> {
        Err(crate::DeviceError::Unexpected.into())
    }
    unsafe fn destroy_compute_pipeline(&self, pipeline: Resource) {}
    unsafe fn create_ray_tracing_pipeline(
        &self,
        desc: &crate::RayTracingPipelineDescriptor<Resource, Resource, Resource>,
    ) -> Result<Resource, crate::PipelineError> {
        Err(crate::DeviceError::Unexpected.into())
    }
    unsafe fn destroy_ray_tracing_pipeline(&self, pipeline: Resource) {}
    unsafe fn get_raytracing_pipeline_group_data(
        &self,
        pipeline: &Resource,
        groups: core::ops::Range<u32>,
    ) -> Result<Vec<u8>, crate::DeviceError> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> Result<Resource, crate::PipelineCacheError> {
        Err(crate::DeviceError::Unexpected.into())
    }
    unsafe fn destroy_pipeline_cache(&self, cache: Resource) {}

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<crate::Label>,
    ) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_query_set(&self, set: Resource) {}
    unsafe fn create_fence(&self) -> DeviceResult<Fence> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn destroy_fence(&self, fence: Fence) {}
    unsafe fn get_fence_value(&self, fence: &Fence) -> DeviceResult<crate::FenceValue> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn wait(
        &self,
        fence: &Fence,
        value: crate::FenceValue,
        timeout: Option<Duration>,
    ) -> DeviceResult<bool> {
        Err(crate::DeviceError::Unexpected)
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        false
    }
    unsafe fn stop_graphics_debugger_capture(&self) {}
    unsafe fn create_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
    ) -> DeviceResult<Resource> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn get_acceleration_structure_build_sizes<'a>(
        &self,
        _desc: &crate::GetAccelerationStructureBuildSizesDescriptor<'a, Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        panic!("TRUEOS acceleration structures are not implemented")
    }
    unsafe fn get_acceleration_structure_device_address(
        &self,
        _acceleration_structure: &Resource,
    ) -> wgt::BufferAddress {
        panic!("TRUEOS acceleration structures are not implemented")
    }
    unsafe fn destroy_acceleration_structure(&self, _acceleration_structure: Resource) {}

    fn tlas_instance_to_bytes(&self, instance: TlasInstance, to_extend: &mut Vec<u8>) {
        panic!("TRUEOS acceleration structures are not implemented")
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        Default::default()
    }

    fn check_if_oom(&self) -> DeviceResult<()> {
        Err(crate::DeviceError::Unexpected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_has_the_trueos_variant() {
        assert_eq!(<Api as crate::Api>::VARIANT, wgt::Backend::TrueOs);
    }

    #[cfg(not(target_os = "trueos"))]
    #[test]
    fn host_probe_is_explicitly_unavailable() {
        let descriptor = crate::InstanceDescriptor {
            name: "trueos-host-test",
            flags: wgt::InstanceFlags::empty(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            telemetry: None,
            display: None,
        };
        assert!(unsafe { <Context as crate::Instance>::init(&descriptor) }.is_err());
    }

    #[test]
    fn opening_a_probed_adapter_is_explicitly_unsupported() {
        let context = Context {
            probe: integration::AdapterProbe {
                capabilities: u64::MAX,
                memory_quota: 0,
                memory_used: 0,
                epoch: 0,
            },
        };
        assert!(
            unsafe { <Context as crate::Instance>::enumerate_adapters(&context, None) }.is_empty()
        );
        let result = unsafe {
            <Context as crate::Adapter>::open(
                &context,
                wgt::Features::empty(),
                &wgt::Limits::default(),
                &wgt::MemoryHints::Performance,
            )
        };
        assert!(matches!(result, Err(crate::DeviceError::Unexpected)));
        let buffer = unsafe {
            <Context as crate::Device>::create_buffer(
                &context,
                &crate::BufferDescriptor {
                    label: None,
                    size: 16,
                    usage: wgt::BufferUses::STORAGE_READ_WRITE,
                    memory_flags: crate::MemoryFlags::empty(),
                },
            )
        };
        assert!(matches!(buffer, Err(crate::DeviceError::Unexpected)));
        assert!(unsafe { <Context as crate::Device>::create_fence(&context) }.is_err());
        assert!(unsafe { <Context as crate::Queue>::wait_for_idle(&context) }.is_err());
    }
}
