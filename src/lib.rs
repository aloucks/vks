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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(struct_field_attributes)]
#![feature(untagged_unions)]

extern crate libc;

#[cfg_attr(feature = "core_1_0_3", macro_use)]
extern crate bitflags;

#[macro_use]
mod cenum;

#[cfg(feature = "core_1_0_3")] mod core;
#[cfg(feature = "core_1_0_3")] pub mod device_proc_addr_loader;
#[cfg(feature = "core_1_0_3")] pub mod instance_proc_addr_loader;
#[cfg(feature = "core_1_0_3")] pub use core::*;
#[cfg(feature = "core_1_0_3")] pub use device_proc_addr_loader::DeviceProcAddrLoader;
#[cfg(feature = "core_1_0_3")] pub use instance_proc_addr_loader::InstanceProcAddrLoader;

#[cfg(feature = "amd_draw_indirect_count_1")] mod amd_draw_indirect_count;
#[cfg(feature = "amd_draw_indirect_count_1")] pub use amd_draw_indirect_count::*;
#[cfg(feature = "amd_gcn_shader_1")] mod amd_gcn_shader;
#[cfg(feature = "amd_gcn_shader_1")] pub use amd_gcn_shader::*;
#[cfg(feature = "amd_gpu_shader_half_float_1")] mod amd_gpu_shader_half_float;
#[cfg(feature = "amd_gpu_shader_half_float_1")] pub use amd_gpu_shader_half_float::*;
#[cfg(feature = "amd_negative_viewport_height_1")] mod amd_negative_viewport_height;
#[cfg(feature = "amd_negative_viewport_height_1")] pub use amd_negative_viewport_height::*;
#[cfg(feature = "amd_rasterization_order_1")] mod amd_rasterization_order;
#[cfg(feature = "amd_rasterization_order_1")] pub use amd_rasterization_order::*;
#[cfg(feature = "amd_shader_ballot_1")] mod amd_shader_ballot;
#[cfg(feature = "amd_shader_ballot_1")] pub use amd_shader_ballot::*;
#[cfg(feature = "amd_shader_explicit_vertex_parameter_1")] mod amd_shader_explicit_vertex_parameter;
#[cfg(feature = "amd_shader_explicit_vertex_parameter_1")] pub use amd_shader_explicit_vertex_parameter::*;
#[cfg(feature = "amd_shader_trinary_minmax_1")] mod amd_shader_trinary_minmax;
#[cfg(feature = "amd_shader_trinary_minmax_1")] pub use amd_shader_trinary_minmax::*;
#[cfg(feature = "ext_acquire_xlib_display_1")] mod ext_acquire_xlib_display;
#[cfg(feature = "ext_acquire_xlib_display_1")] pub use ext_acquire_xlib_display::*;
#[cfg(feature = "ext_debug_marker_3")] mod ext_debug_marker;
#[cfg(feature = "ext_debug_marker_3")] pub use ext_debug_marker::*;
#[cfg(feature = "ext_debug_report_1")] mod ext_debug_report;
#[cfg(feature = "ext_debug_report_1")] pub use ext_debug_report::*;
#[cfg(feature = "ext_direct_mode_display_1")] mod ext_direct_mode_display;
#[cfg(feature = "ext_direct_mode_display_1")] pub use ext_direct_mode_display::*;
#[cfg(feature = "ext_discard_rectangles_1")] mod ext_discard_rectangles;
#[cfg(feature = "ext_discard_rectangles_1")] pub use ext_discard_rectangles::*;
#[cfg(feature = "ext_display_control_1")] mod ext_display_control;
#[cfg(feature = "ext_display_control_1")] pub use ext_display_control::*;
#[cfg(feature = "ext_display_surface_counter_1")] mod ext_display_surface_counter;
#[cfg(feature = "ext_display_surface_counter_1")] pub use ext_display_surface_counter::*;
#[cfg(feature = "ext_hdr_metadata_1")] mod ext_hdr_metadata;
#[cfg(feature = "ext_hdr_metadata_1")] pub use ext_hdr_metadata::*;
#[cfg(feature = "ext_shader_subgroup_ballot_1")] mod ext_shader_subgroup_ballot;
#[cfg(feature = "ext_shader_subgroup_ballot_1")] pub use ext_shader_subgroup_ballot::*;
#[cfg(feature = "ext_shader_subgroup_vote_1")] mod ext_shader_subgroup_vote;
#[cfg(feature = "ext_shader_subgroup_vote_1")] pub use ext_shader_subgroup_vote::*;
#[cfg(feature = "ext_swapchain_colorspace_2")] mod ext_swapchain_colorspace;
#[cfg(feature = "ext_swapchain_colorspace_2")] pub use ext_swapchain_colorspace::*;
#[cfg(feature = "ext_validation_flags_1")] mod ext_validation_flags;
#[cfg(feature = "ext_validation_flags_1")] pub use ext_validation_flags::*;
#[cfg(feature = "google_display_timing_1")] mod google_display_timing;
#[cfg(feature = "google_display_timing_1")] pub use google_display_timing::*;
#[cfg(feature = "img_filter_cubic_1")] mod img_filter_cubic;
#[cfg(feature = "img_filter_cubic_1")] pub use img_filter_cubic::*;
#[cfg(feature = "img_format_pvrtc_1")] mod img_format_pvrtc;
#[cfg(feature = "img_format_pvrtc_1")] pub use img_format_pvrtc::*;
#[cfg(feature = "khr_android_surface_6")] mod khr_android_surface;
#[cfg(feature = "khr_android_surface_6")] pub mod android_wrapper;
#[cfg(feature = "khr_android_surface_6")] pub use khr_android_surface::*;
#[cfg(feature = "khr_descriptor_update_template_1")] mod khr_descriptor_update_template;
#[cfg(feature = "khr_descriptor_update_template_1")] pub use khr_descriptor_update_template::*;
#[cfg(feature = "khr_display_21")] mod khr_display;
#[cfg(feature = "khr_display_21")] pub use khr_display::*;
#[cfg(feature = "khr_display_swapchain_9")] mod khr_display_swapchain;
#[cfg(feature = "khr_display_swapchain_9")] pub use khr_display_swapchain::*;
#[cfg(feature = "khr_get_physical_device_properties2_1")] mod khr_get_physical_device_properties2;
#[cfg(feature = "khr_get_physical_device_properties2_1")] pub use khr_get_physical_device_properties2::*;
#[cfg(feature = "khr_maintenance1_1")] mod khr_maintenance1;
#[cfg(feature = "khr_maintenance1_1")] pub use khr_maintenance1::*;
#[cfg(feature = "khr_mir_surface_4")] mod khr_mir_surface;
#[cfg(feature = "khr_mir_surface_4")] pub mod mir_wrapper;
#[cfg(feature = "khr_mir_surface_4")] pub use khr_mir_surface::*;
#[cfg(feature = "khr_push_descriptor_1")] mod khr_push_descriptor;
#[cfg(feature = "khr_push_descriptor_1")] pub use khr_push_descriptor::*;
#[cfg(feature = "khr_sampler_mirror_clamp_to_edge_1")] mod khr_sampler_mirror_clamp_to_edge;
#[cfg(feature = "khr_sampler_mirror_clamp_to_edge_1")] pub use khr_sampler_mirror_clamp_to_edge::*;
#[cfg(feature = "khr_shader_draw_parameters_1")] mod khr_shader_draw_parameters;
#[cfg(feature = "khr_shader_draw_parameters_1")] pub use khr_shader_draw_parameters::*;
#[cfg(feature = "khr_surface_25")] mod khr_surface;
#[cfg(feature = "khr_surface_25")] pub use khr_surface::*;
#[cfg(feature = "khr_swapchain_67")] mod khr_swapchain;
#[cfg(feature = "khr_swapchain_67")] pub use khr_swapchain::*;
#[cfg(feature = "khr_wayland_surface_5")] mod khr_wayland_surface;
#[cfg(feature = "khr_wayland_surface_5")] pub mod wayland_wrapper;
#[cfg(feature = "khr_wayland_surface_5")] pub use khr_wayland_surface::*;
#[cfg(feature = "khr_win32_surface_5")] mod khr_win32_surface;
#[cfg(any(feature = "khr_win32_surface_5",
          feature = "khx_external_memory_win32_1",
          feature = "khx_external_semaphore_win32_1",
          feature = "nv_external_memory_win32_1",
          feature = "nv_win32_keyed_mutex_1"))] pub mod win32_wrapper;
#[cfg(feature = "khr_win32_surface_5")] pub use khr_win32_surface::*;
#[cfg(feature = "khr_xcb_surface_6")] mod khr_xcb_surface;
#[cfg(feature = "khr_xcb_surface_6")] pub mod xcb_wrapper;
#[cfg(feature = "khr_xcb_surface_6")] pub use khr_xcb_surface::*;
#[cfg(feature = "khr_xlib_surface_6")] mod khr_xlib_surface;
#[cfg(any(feature = "khr_xlib_surface_6",
          feature = "ext_acquire_xlib_display_1"))] pub mod xlib_wrapper;
#[cfg(feature = "khr_xlib_surface_6")] pub use khr_xlib_surface::*;
#[cfg(feature = "khx_device_group_1")] mod khx_device_group;
#[cfg(feature = "khx_device_group_1")] pub use khx_device_group::*;
#[cfg(feature = "khx_device_group_creation_1")] mod khx_device_group_creation;
#[cfg(feature = "khx_device_group_creation_1")] pub use khx_device_group_creation::*;
#[cfg(feature = "khx_external_memory_1")] mod khx_external_memory;
#[cfg(feature = "khx_external_memory_1")] pub use khx_external_memory::*;
#[cfg(feature = "khx_external_memory_capabilities_1")] mod khx_external_memory_capabilities;
#[cfg(feature = "khx_external_memory_capabilities_1")] pub use khx_external_memory_capabilities::*;
#[cfg(feature = "khx_external_memory_fd_1")] mod khx_external_memory_fd;
#[cfg(feature = "khx_external_memory_fd_1")] pub use khx_external_memory_fd::*;
#[cfg(feature = "khx_external_memory_win32_1")] mod khx_external_memory_win32;
#[cfg(feature = "khx_external_memory_win32_1")] pub use khx_external_memory_win32::*;
#[cfg(feature = "khx_external_semaphore_1")] mod khx_external_semaphore;
#[cfg(feature = "khx_external_semaphore_1")] pub use khx_external_semaphore::*;
#[cfg(feature = "khx_external_semaphore_capabilities_1")] mod khx_external_semaphore_capabilities;
#[cfg(feature = "khx_external_semaphore_capabilities_1")] pub use khx_external_semaphore_capabilities::*;
#[cfg(feature = "khx_external_semaphore_fd_1")] mod khx_external_semaphore_fd;
#[cfg(feature = "khx_external_semaphore_fd_1")] pub use khx_external_semaphore_fd::*;
#[cfg(feature = "khx_external_semaphore_win32_1")] mod khx_external_semaphore_win32;
#[cfg(feature = "khx_external_semaphore_win32_1")] pub use khx_external_semaphore_win32::*;
#[cfg(feature = "khx_multiview_1")] mod khx_multiview;
#[cfg(feature = "khx_multiview_1")] pub use khx_multiview::*;
#[cfg(feature = "khx_win32_keyed_mutex_1")] mod khx_win32_keyed_mutex;
#[cfg(feature = "khx_win32_keyed_mutex_1")] pub use khx_win32_keyed_mutex::*;
#[cfg(feature = "mvk_ios_surface_2")] mod mvk_ios_surface;
#[cfg(feature = "mvk_ios_surface_2")] pub use mvk_ios_surface::*;
#[cfg(feature = "mvk_macos_surface_2")] mod mvk_macos_surface;
#[cfg(feature = "mvk_macos_surface_2")] pub use mvk_macos_surface::*;
#[cfg(feature = "nn_vi_surface_1")] mod nn_vi_surface;
#[cfg(feature = "nn_vi_surface_1")] pub use nn_vi_surface::*;
#[cfg(feature = "nv_clip_space_w_scaling_1")] mod nv_clip_space_w_scaling;
#[cfg(feature = "nv_clip_space_w_scaling_1")] pub use nv_clip_space_w_scaling::*;
#[cfg(feature = "nv_dedicated_allocation_1")] mod nv_dedicated_allocation;
#[cfg(feature = "nv_dedicated_allocation_1")] pub use nv_dedicated_allocation::*;
#[cfg(feature = "nv_external_memory_1")] mod nv_external_memory;
#[cfg(feature = "nv_external_memory_1")] pub use nv_external_memory::*;
#[cfg(feature = "nv_external_memory_capabilities_1")] mod nv_external_memory_capabilities;
#[cfg(feature = "nv_external_memory_capabilities_1")] pub use nv_external_memory_capabilities::*;
#[cfg(feature = "nv_external_memory_win32_1")] mod nv_external_memory_win32;
#[cfg(feature = "nv_external_memory_win32_1")] pub use nv_external_memory_win32::*;
#[cfg(feature = "nv_geometry_shader_passthrough_1")] mod nv_geometry_shader_passthrough;
#[cfg(feature = "nv_geometry_shader_passthrough_1")] pub use nv_geometry_shader_passthrough::*;
#[cfg(feature = "nv_glsl_shader_1")] mod nv_glsl_shader;
#[cfg(feature = "nv_glsl_shader_1")] pub use nv_glsl_shader::*;
#[cfg(feature = "nv_sample_mask_override_coverage_1")] mod nv_sample_mask_override_coverage;
#[cfg(feature = "nv_sample_mask_override_coverage_1")] pub use nv_sample_mask_override_coverage::*;
#[cfg(feature = "nv_viewport_array2_1")] mod nv_viewport_array2;
#[cfg(feature = "nv_viewport_array2_1")] pub use nv_viewport_array2::*;
#[cfg(feature = "nv_viewport_swizzle_1")] mod nv_viewport_swizzle;
#[cfg(feature = "nv_viewport_swizzle_1")] pub use nv_viewport_swizzle::*;
#[cfg(feature = "nv_win32_keyed_mutex_1")] mod nv_win32_keyed_mutex;
#[cfg(feature = "nv_win32_keyed_mutex_1")] pub use nv_win32_keyed_mutex::*;
#[cfg(feature = "nvx_device_generated_commands_1")] mod nvx_device_generated_commands;
#[cfg(feature = "nvx_device_generated_commands_1")] pub use nvx_device_generated_commands::*;
#[cfg(feature = "nvx_multiview_per_view_attributes_1")] mod nvx_multiview_per_view_attributes;
#[cfg(feature = "nvx_multiview_per_view_attributes_1")] pub use nvx_multiview_per_view_attributes::*;
