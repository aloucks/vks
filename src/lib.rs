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

#[cfg(not(feature = "unstable_rust"))]
pub mod union_field;

// pub mod instance_proc_addr_loader;
// pub use instance_proc_addr_loader::InstanceProcAddrLoader;
// pub mod device_proc_addr_loader;
// pub use device_proc_addr_loader::DeviceProcAddrLoader;

pub mod core;
pub mod amd_draw_indirect_count;
pub mod amd_gcn_shader;
pub mod amd_gpu_shader_half_float;
pub mod amd_gpu_shader_int16;
pub mod amd_negative_viewport_height;
pub mod amd_rasterization_order;
pub mod amd_shader_ballot;
pub mod amd_shader_explicit_vertex_parameter;
pub mod amd_shader_trinary_minmax;
pub mod amd_texture_gather_bias_lod;
pub mod ext_acquire_xlib_display;
pub mod ext_blend_operation_advanced;
pub mod ext_debug_marker;
pub mod ext_debug_report;
pub mod ext_direct_mode_display;
pub mod ext_discard_rectangles;
pub mod ext_display_control;
pub mod ext_display_surface_counter;
pub mod ext_hdr_metadata;
pub mod ext_sampler_filter_minmax;
pub mod ext_shader_subgroup_ballot;
pub mod ext_shader_subgroup_vote;
pub mod ext_swapchain_colorspace;
pub mod ext_validation_flags;
pub mod google_display_timing;
pub mod img_filter_cubic;
pub mod img_format_pvrtc;
pub mod khr_android_surface;
pub mod khr_descriptor_update_template;
pub mod khr_display;
pub mod khr_display_swapchain;
pub mod khr_get_physical_device_properties2;
pub mod khr_get_surface_capabilities2;
pub mod khr_incremental_present;
pub mod khr_maintenance1;
pub mod khr_mir_surface;
pub mod khr_push_descriptor;
pub mod khr_sampler_mirror_clamp_to_edge;
pub mod khr_shader_draw_parameters;
pub mod khr_shared_presentable_image;
pub mod khr_surface;
pub mod khr_swapchain;
pub mod khr_wayland_surface;
pub mod khr_win32_surface;
pub mod khr_xcb_surface;
pub mod khr_xlib_surface;
pub mod mvk_ios_surface;
pub mod mvk_macos_surface;
pub mod nn_vi_surface;
pub mod nv_clip_space_w_scaling;
pub mod nv_dedicated_allocation;
pub mod nv_external_memory;
pub mod nv_external_memory_capabilities;
pub mod nv_external_memory_win32;
pub mod nv_fill_rectangle;
pub mod nv_fragment_coverage_to_color;
pub mod nv_framebuffer_mixed_samples;
pub mod nv_geometry_shader_passthrough;
pub mod nv_glsl_shader;
pub mod nv_sample_mask_override_coverage;
pub mod nv_viewport_array2;
pub mod nv_viewport_swizzle;
pub mod nv_win32_keyed_mutex;

pub mod android_wrapper;
pub mod mir_wrapper;
pub mod wayland_wrapper;
pub mod win32_wrapper;
pub mod xcb_wrapper;
pub mod xlib_wrapper;

#[cfg(feature = "experimental")] pub mod nvx_device_generated_commands;
#[cfg(feature = "experimental")] pub mod nvx_multiview_per_view_attributes;
#[cfg(feature = "experimental")] pub mod khx_device_group;
#[cfg(feature = "experimental")] pub mod khx_device_group_creation;
#[cfg(feature = "experimental")] pub mod khx_external_memory;
#[cfg(feature = "experimental")] pub mod khx_external_memory_capabilities;
#[cfg(feature = "experimental")] pub mod khx_external_memory_fd;
#[cfg(feature = "experimental")] pub mod khx_external_memory_win32;
#[cfg(feature = "experimental")] pub mod khx_external_semaphore;
#[cfg(feature = "experimental")] pub mod khx_external_semaphore_capabilities;
#[cfg(feature = "experimental")] pub mod khx_external_semaphore_fd;
#[cfg(feature = "experimental")] pub mod khx_external_semaphore_win32;
#[cfg(feature = "experimental")] pub mod khx_multiview;
#[cfg(feature = "experimental")] pub mod khx_win32_keyed_mutex;

pub use version::*;

#[cfg(windows)]
pub const VULKAN_LIBRARY_NAME: &'static str = "vulkan-1.dll";

#[cfg(not(windows))]
pub const VULKAN_LIBRARY_NAME: &'static str = "libvulkan.so.1";
