// Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

//! Vulkan FFI bindings and symbol loader

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "unstable_rust", feature(untagged_unions))]

extern crate libc;

mod version;

#[macro_use]
mod cenum;

#[macro_use]
mod handle;

#[macro_use]
mod vks_bitflags;

pub mod core;

pub use version::*;
// pub mod instance_proc_addr_loader;
// pub use instance_proc_addr_loader::InstanceProcAddrLoader;
// pub mod device_proc_addr_loader;
// pub use device_proc_addr_loader::DeviceProcAddrLoader;
#[cfg(not(feature = "unstable_rust"))] pub mod union_field;

mod amd_draw_indirect_count;
pub use amd_draw_indirect_count::*;
mod amd_gcn_shader;
pub use amd_gcn_shader::*;
mod amd_gpu_shader_half_float;
pub use amd_gpu_shader_half_float::*;
mod amd_gpu_shader_int16;
pub use amd_gpu_shader_int16::*;
mod amd_negative_viewport_height;
pub use amd_negative_viewport_height::*;
mod amd_rasterization_order;
pub use amd_rasterization_order::*;
mod amd_shader_ballot;
pub use amd_shader_ballot::*;
mod amd_shader_explicit_vertex_parameter;
pub use amd_shader_explicit_vertex_parameter::*;
mod amd_shader_trinary_minmax;
pub use amd_shader_trinary_minmax::*;
mod amd_texture_gather_bias_lod;
pub use amd_texture_gather_bias_lod::*;
mod ext_acquire_xlib_display;
pub use ext_acquire_xlib_display::*;
mod ext_blend_operation_advanced;
pub use ext_blend_operation_advanced::*;
mod ext_debug_marker;
pub use ext_debug_marker::*;
mod ext_debug_report;
pub use ext_debug_report::*;
mod ext_direct_mode_display;
pub use ext_direct_mode_display::*;
mod ext_discard_rectangles;
pub use ext_discard_rectangles::*;
mod ext_display_control;
pub use ext_display_control::*;
mod ext_display_surface_counter;
pub use ext_display_surface_counter::*;
mod ext_hdr_metadata;
pub use ext_hdr_metadata::*;
mod ext_sampler_filter_minmax;
pub use ext_sampler_filter_minmax::*;
mod ext_shader_subgroup_ballot;
pub use ext_shader_subgroup_ballot::*;
mod ext_shader_subgroup_vote;
pub use ext_shader_subgroup_vote::*;
mod ext_swapchain_colorspace;
pub use ext_swapchain_colorspace::*;
mod ext_validation_flags;
pub use ext_validation_flags::*;
mod google_display_timing;
pub use google_display_timing::*;
mod img_filter_cubic;
pub use img_filter_cubic::*;
mod img_format_pvrtc;
pub use img_format_pvrtc::*;
mod khr_android_surface;
pub mod android_wrapper;
pub use khr_android_surface::*;
mod khr_descriptor_update_template;
pub use khr_descriptor_update_template::*;
mod khr_display;
pub use khr_display::*;
mod khr_display_swapchain;
pub use khr_display_swapchain::*;
mod khr_get_physical_device_properties2;
pub use khr_get_physical_device_properties2::*;
mod khr_get_surface_capabilities2;
pub use khr_get_surface_capabilities2::*;
mod khr_incremental_present;
pub use khr_incremental_present::*;
mod khr_maintenance1;
pub use khr_maintenance1::*;
mod khr_mir_surface;
pub mod mir_wrapper;
pub use khr_mir_surface::*;
mod khr_push_descriptor;
pub use khr_push_descriptor::*;
mod khr_sampler_mirror_clamp_to_edge;
pub use khr_sampler_mirror_clamp_to_edge::*;
mod khr_shader_draw_parameters;
pub use khr_shader_draw_parameters::*;
mod khr_shared_presentable_image;
pub use khr_shared_presentable_image::*;
mod khr_surface;
pub use khr_surface::*;
mod khr_swapchain;
pub use khr_swapchain::*;
mod khr_wayland_surface;
pub mod wayland_wrapper;
pub use khr_wayland_surface::*;
mod khr_win32_surface;
pub mod win32_wrapper;
pub use khr_win32_surface::*;
mod khr_xcb_surface;
pub mod xcb_wrapper;
pub use khr_xcb_surface::*;
mod khr_xlib_surface;
pub mod xlib_wrapper;
pub use khr_xlib_surface::*;
mod mvk_ios_surface;
pub use mvk_ios_surface::*;
mod mvk_macos_surface;
pub use mvk_macos_surface::*;
mod nn_vi_surface;
pub use nn_vi_surface::*;
mod nv_clip_space_w_scaling;
pub use nv_clip_space_w_scaling::*;
mod nv_dedicated_allocation;
pub use nv_dedicated_allocation::*;
mod nv_external_memory;
pub use nv_external_memory::*;
mod nv_external_memory_capabilities;
pub use nv_external_memory_capabilities::*;
mod nv_external_memory_win32;
pub use nv_external_memory_win32::*;
mod nv_fill_rectangle;
pub use nv_fill_rectangle::*;
mod nv_fragment_coverage_to_color;
pub use nv_fragment_coverage_to_color::*;
mod nv_framebuffer_mixed_samples;
pub use nv_framebuffer_mixed_samples::*;
mod nv_geometry_shader_passthrough;
pub use nv_geometry_shader_passthrough::*;
mod nv_glsl_shader;
pub use nv_glsl_shader::*;
mod nv_sample_mask_override_coverage;
pub use nv_sample_mask_override_coverage::*;
mod nv_viewport_array2;
pub use nv_viewport_array2::*;
mod nv_viewport_swizzle;
pub use nv_viewport_swizzle::*;
mod nv_win32_keyed_mutex;
pub use nv_win32_keyed_mutex::*;

#[cfg(feature = "experimental")] mod nvx_device_generated_commands;
#[cfg(feature = "experimental")] pub use nvx_device_generated_commands::*;
#[cfg(feature = "experimental")] mod nvx_multiview_per_view_attributes;
#[cfg(feature = "experimental")] pub use nvx_multiview_per_view_attributes::*;
#[cfg(feature = "experimental")] mod khx_device_group;
#[cfg(feature = "experimental")] pub use khx_device_group::*;
#[cfg(feature = "experimental")] mod khx_device_group_creation;
#[cfg(feature = "experimental")] pub use khx_device_group_creation::*;
#[cfg(feature = "experimental")] mod khx_external_memory;
#[cfg(feature = "experimental")] pub use khx_external_memory::*;
#[cfg(feature = "experimental")] mod khx_external_memory_capabilities;
#[cfg(feature = "experimental")] pub use khx_external_memory_capabilities::*;
#[cfg(feature = "experimental")] mod khx_external_memory_fd;
#[cfg(feature = "experimental")] pub use khx_external_memory_fd::*;
#[cfg(feature = "experimental")] mod khx_external_memory_win32;
#[cfg(feature = "experimental")] pub use khx_external_memory_win32::*;
#[cfg(feature = "experimental")] mod khx_external_semaphore;
#[cfg(feature = "experimental")] pub use khx_external_semaphore::*;
#[cfg(feature = "experimental")] mod khx_external_semaphore_capabilities;
#[cfg(feature = "experimental")] pub use khx_external_semaphore_capabilities::*;
#[cfg(feature = "experimental")] mod khx_external_semaphore_fd;
#[cfg(feature = "experimental")] pub use khx_external_semaphore_fd::*;
#[cfg(feature = "experimental")] mod khx_external_semaphore_win32;
#[cfg(feature = "experimental")] pub use khx_external_semaphore_win32::*;
#[cfg(feature = "experimental")] mod khx_multiview;
#[cfg(feature = "experimental")] pub use khx_multiview::*;
#[cfg(feature = "experimental")] mod khx_win32_keyed_mutex;
#[cfg(feature = "experimental")] pub use khx_win32_keyed_mutex::*;

#[cfg(windows)]
pub const VULKAN_LIBRARY_NAME: &'static str = "vulkan-1.dll";

#[cfg(not(windows))]
pub const VULKAN_LIBRARY_NAME: &'static str = "libvulkan.so.1";
