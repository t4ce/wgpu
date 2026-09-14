use core::ops::Range;

use super::{Api, Buffer, DeviceResult, Resource};

#[derive(Debug)]
pub struct CommandBuffer {
    _private: (),
}

impl crate::CommandEncoder for CommandBuffer {
    type A = Api;

    unsafe fn begin_encoding(&mut self, label: crate::Label) -> DeviceResult<()> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn discard_encoding(&mut self) {}
    unsafe fn end_encoding(&mut self) -> DeviceResult<CommandBuffer> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn reset_all<I>(&mut self, command_buffers: I) {}

    unsafe fn transition_buffers<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = crate::BufferBarrier<'a, Buffer>>,
    {
        panic!("TRUEOS transition_buffers is not implemented")
    }

    unsafe fn transition_textures<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = crate::TextureBarrier<'a, Resource>>,
    {
        panic!("TRUEOS transition_textures is not implemented")
    }

    unsafe fn clear_buffer(&mut self, buffer: &Buffer, range: crate::MemoryRange) {
        panic!("TRUEOS clear_buffer is not implemented")
    }

    unsafe fn copy_buffer_to_buffer<T>(&mut self, src: &Buffer, dst: &Buffer, regions: T)
    where
        T: Iterator<Item = crate::BufferCopy>,
    {
        panic!("TRUEOS copy_buffer_to_buffer is not implemented")
    }

    #[cfg(webgl)]
    unsafe fn copy_external_image_to_texture<T>(
        &mut self,
        src: &wgt::CopyExternalImageSourceInfo,
        dst: &Resource,
        dst_premultiplication: bool,
        regions: T,
    ) where
        T: Iterator<Item = crate::TextureCopy>,
    {
        panic!("TRUEOS copy_external_image_to_texture is not implemented")
    }

    unsafe fn copy_texture_to_texture<T>(
        &mut self,
        src: &Resource,
        src_usage: wgt::TextureUses,
        dst: &Resource,
        regions: T,
    ) {
        panic!("TRUEOS copy_texture_to_texture is not implemented")
    }

    unsafe fn copy_buffer_to_texture<T>(&mut self, src: &Buffer, dst: &Resource, regions: T) {
        panic!("TRUEOS copy_buffer_to_texture is not implemented")
    }

    unsafe fn copy_texture_to_buffer<T>(
        &mut self,
        src: &Resource,
        src_usage: wgt::TextureUses,
        dst: &Buffer,
        regions: T,
    ) {
        panic!("TRUEOS copy_texture_to_buffer is not implemented")
    }

    unsafe fn begin_query(&mut self, set: &Resource, index: u32) {
        panic!("TRUEOS begin_query is not implemented")
    }
    unsafe fn end_query(&mut self, set: &Resource, index: u32) {
        panic!("TRUEOS end_query is not implemented")
    }
    unsafe fn write_timestamp(&mut self, set: &Resource, index: u32) {
        panic!("TRUEOS write_timestamp is not implemented")
    }
    unsafe fn read_acceleration_structure_compact_size(
        &mut self,
        acceleration_structure: &Resource,
        buf: &Buffer,
    ) {
        panic!("TRUEOS read_acceleration_structure_compact_size is not implemented")
    }
    unsafe fn reset_queries(&mut self, set: &Resource, range: Range<u32>) {
        panic!("TRUEOS reset_queries is not implemented")
    }
    unsafe fn copy_query_results(
        &mut self,
        set: &Resource,
        range: Range<u32>,
        buffer: &Buffer,
        offset: wgt::BufferAddress,
        stride: wgt::BufferSize,
    ) {
        panic!("TRUEOS copy_query_results is not implemented")
    }

    unsafe fn begin_render_pass(
        &mut self,
        desc: &crate::RenderPassDescriptor<Resource, Resource>,
    ) -> DeviceResult<()> {
        Err(crate::DeviceError::Unexpected)
    }
    unsafe fn end_render_pass(&mut self) {
        panic!("TRUEOS end_render_pass is not implemented")
    }

    unsafe fn set_bind_group(
        &mut self,
        layout: &Resource,
        index: u32,
        group: &Resource,
        dynamic_offsets: &[wgt::DynamicOffset],
    ) {
        panic!("TRUEOS set_bind_group is not implemented")
    }
    unsafe fn set_immediates(&mut self, layout: &Resource, offset_bytes: u32, data: &[u32]) {
        panic!("TRUEOS set_immediates is not implemented")
    }

    unsafe fn insert_debug_marker(&mut self, label: &str) {}
    unsafe fn begin_debug_marker(&mut self, group_label: &str) {}
    unsafe fn end_debug_marker(&mut self) {}

    unsafe fn set_render_pipeline(&mut self, pipeline: &Resource) {
        panic!("TRUEOS set_render_pipeline is not implemented")
    }

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: crate::BufferBinding<'a, Buffer, wgt::BufferAddress>,
        format: wgt::IndexFormat,
    ) {
        panic!("TRUEOS set_index_buffer is not implemented")
    }
    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: crate::BufferBinding<'a, Buffer, wgt::BufferAddress>,
    ) {
        panic!("TRUEOS set_vertex_buffer is not implemented")
    }
    unsafe fn set_viewport(&mut self, rect: &crate::Rect<f32>, depth_range: Range<f32>) {
        panic!("TRUEOS set_viewport is not implemented")
    }
    unsafe fn set_scissor_rect(&mut self, rect: &crate::Rect<u32>) {
        panic!("TRUEOS set_scissor_rect is not implemented")
    }
    unsafe fn set_stencil_reference(&mut self, value: u32) {
        panic!("TRUEOS set_stencil_reference is not implemented")
    }
    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]) {
        panic!("TRUEOS set_blend_constants is not implemented")
    }

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    ) {
        panic!("TRUEOS draw is not implemented")
    }
    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    ) {
        panic!("TRUEOS draw_indexed is not implemented")
    }
    unsafe fn draw_mesh_tasks(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        panic!("TRUEOS draw_mesh_tasks is not implemented")
    }
    unsafe fn draw_indirect(
        &mut self,
        buffer: &Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        panic!("TRUEOS draw_indirect is not implemented")
    }
    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        panic!("TRUEOS draw_indexed_indirect is not implemented")
    }
    unsafe fn draw_mesh_tasks_indirect(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        panic!("TRUEOS draw_mesh_tasks_indirect is not implemented")
    }
    unsafe fn draw_indirect_count(
        &mut self,
        buffer: &Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        panic!("TRUEOS draw_indirect_count is not implemented")
    }
    unsafe fn draw_indexed_indirect_count(
        &mut self,
        buffer: &Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        panic!("TRUEOS draw_indexed_indirect_count is not implemented")
    }
    unsafe fn draw_mesh_tasks_indirect_count(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &<Self::A as crate::Api>::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        panic!("TRUEOS draw_mesh_tasks_indirect_count is not implemented")
    }

    unsafe fn begin_compute_pass(&mut self, desc: &crate::ComputePassDescriptor<Resource>) {
        panic!("TRUEOS begin_compute_pass is not implemented")
    }
    unsafe fn end_compute_pass(&mut self) {
        panic!("TRUEOS end_compute_pass is not implemented")
    }

    unsafe fn set_compute_pipeline(&mut self, pipeline: &Resource) {
        panic!("TRUEOS set_compute_pipeline is not implemented")
    }

    unsafe fn dispatch_workgroups(&mut self, count: [u32; 3]) {
        panic!("TRUEOS dispatch_workgroups is not implemented")
    }
    unsafe fn dispatch_workgroups_indirect(&mut self, buffer: &Buffer, offset: wgt::BufferAddress) {
        panic!("TRUEOS dispatch_workgroups_indirect is not implemented")
    }

    unsafe fn build_acceleration_structures<'a, T>(
        &mut self,
        _descriptor_count: u32,
        descriptors: T,
    ) where
        Api: 'a,
        T: IntoIterator<Item = crate::BuildAccelerationStructureDescriptor<'a, Buffer, Resource>>,
    {
        panic!("TRUEOS build_acceleration_structures is not implemented")
    }

    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        _barriers: crate::AccelerationStructureBarrier,
    ) {
        panic!("TRUEOS place_acceleration_structure_barrier is not implemented")
    }

    unsafe fn copy_acceleration_structure_to_acceleration_structure(
        &mut self,
        src: &Resource,
        dst: &Resource,
        copy: wgt::AccelerationStructureCopy,
    ) {
        panic!("TRUEOS copy_acceleration_structure_to_acceleration_structure is not implemented")
    }

    unsafe fn set_acceleration_structure_dependencies(
        command_buffers: &[&CommandBuffer],
        dependencies: &[&Resource],
    ) {
        panic!("TRUEOS set_acceleration_structure_dependencies is not implemented")
    }

    unsafe fn begin_ray_tracing_pass(&mut self, _desc: &crate::RayTracingPassDescriptor) {
        panic!("TRUEOS begin_ray_tracing_pass is not implemented")
    }

    unsafe fn end_ray_tracing_pass(&mut self) {
        panic!("TRUEOS end_ray_tracing_pass is not implemented")
    }

    unsafe fn set_ray_tracing_pipeline(
        &mut self,
        _pipeline: &<Self::A as crate::Api>::RayTracingPipeline,
    ) {
        panic!("TRUEOS set_ray_tracing_pipeline is not implemented")
    }

    unsafe fn trace_rays(
        &mut self,
        _count: [u32; 3],
        _ray_generation_group_data: crate::PipelineGroupData<Buffer>,
        _miss_group_data: crate::PipelineGroupData<Buffer>,
        _intersection_group_data: crate::PipelineGroupData<Buffer>,
    ) {
        panic!("TRUEOS trace_rays is not implemented")
    }
}
