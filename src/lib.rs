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

#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

// https://github.com/rust-lang-nursery/rust-clippy/issues/1254
#![cfg_attr(feature = "cargo-clippy", allow(expl_impl_clone_on_copy))]

extern crate libc;

#[macro_use]
mod cenum;

#[macro_use]
mod handle;

#[macro_use]
mod vks_bitflags;

pub mod device_proc_addr_loader;
pub mod instance_proc_addr_loader;

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
pub mod khr_get_memory_requirements2;
pub mod khr_16bit_storage;
pub mod khr_android_surface;
pub mod khr_dedicated_allocation;
pub mod khr_descriptor_update_template;
pub mod khr_display;
pub mod khr_display_swapchain;
pub mod khr_external_fence;
pub mod khr_external_fence_capabilities;
pub mod khr_external_fence_fd;
pub mod khr_external_fence_win32;
pub mod khr_external_memory;
pub mod khr_external_memory_capabilities;
pub mod khr_external_memory_fd;
pub mod khr_external_memory_win32;
pub mod khr_external_semaphore;
pub mod khr_external_semaphore_capabilities;
pub mod khr_external_semaphore_fd;
pub mod khr_external_semaphore_win32;
pub mod khr_get_physical_device_properties2;
pub mod khr_get_surface_capabilities2;
pub mod khr_incremental_present;
pub mod khr_maintenance1;
pub mod khr_mir_surface;
pub mod khr_push_descriptor;
pub mod khr_sampler_mirror_clamp_to_edge;
pub mod khr_shader_draw_parameters;
pub mod khr_shared_presentable_image;
pub mod khr_storage_buffer_storage_class;
pub mod khr_surface;
pub mod khr_swapchain;
pub mod khr_variable_pointers;
pub mod khr_wayland_surface;
pub mod khr_win32_keyed_mutex;
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

pub mod android_types;
pub mod mir_types;
pub mod wayland_types;
pub mod win32_types;
pub mod xcb_types;
pub mod xlib_types;

#[cfg(feature = "experimental")]
pub mod experimental;

pub use device_proc_addr_loader::DeviceProcAddrLoader;
pub use instance_proc_addr_loader::InstanceProcAddrLoader;

#[cfg(windows)]
pub const VULKAN_LIBRARY_NAME: &'static str = "vulkan-1.dll";

#[cfg(not(windows))]
pub const VULKAN_LIBRARY_NAME: &'static str = "libvulkan.so.1";

/// See [`VK_API_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_API_VERSION_1_0)
pub const VK_API_VERSION_1_0: u32 = 0x00400000;

/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 54;

/// See [`VK_VERSION_MAJOR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_VERSION_MAJOR)
#[inline]
pub fn vk_version_major(version: u32) -> u32 {
    version >> 22
}

/// See [`VK_VERSION_MINOR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_VERSION_MINOR)
#[inline]
pub fn vk_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}

/// See [`VK_VERSION_PATCH`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_VERSION_PATCH)
#[inline]
pub fn vk_version_patch(version: u32) -> u32 {
    version & 0xfff
}

/// See [`VK_MAKE_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MAKE_VERSION)
#[inline]
pub fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
