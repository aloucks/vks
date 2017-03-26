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
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(struct_field_attributes)]
#![feature(untagged_unions)]

extern crate libc;

#[macro_use]
extern crate bitflags;

#[cfg(feature = "core_1_0_3")]
mod core;

#[cfg(feature = "core_1_0_3")]
pub use core::*;

#[cfg(feature = "core_1_0_3")]
pub mod instance_proc_addr_loader;

#[cfg(feature = "core_1_0_3")]
pub use instance_proc_addr_loader::InstanceProcAddrLoader;

#[cfg(feature = "core_1_0_3")]
pub mod device_proc_addr_loader;

#[cfg(feature = "core_1_0_3")]
pub use device_proc_addr_loader::DeviceProcAddrLoader;

#[cfg(feature = "khr_surface_25")]
mod khr_surface;

#[cfg(feature = "khr_surface_25")]
pub use khr_surface::*;

#[cfg(feature = "khr_swapchain_67")]
mod khr_swapchain;

#[cfg(feature = "khr_swapchain_67")]
pub use khr_swapchain::*;

#[cfg(feature = "khr_display_21")]
mod khr_display;

#[cfg(feature = "khr_display_21")]
pub use khr_display::*;

#[cfg(feature = "khr_display_swapchain_9")]
mod khr_display_swapchain;

#[cfg(feature = "khr_display_swapchain_9")]
pub use khr_display_swapchain::*;

#[cfg(feature = "khr_xlib_surface_6")]
mod khr_xlib_surface;

#[cfg(feature = "khr_xlib_surface_6")]
pub use khr_xlib_surface::*;

#[cfg(feature = "khr_xcb_surface_6")]
mod khr_xcb_surface;

#[cfg(feature = "khr_xcb_surface_6")]
pub use khr_xcb_surface::*;

#[cfg(feature = "khr_wayland_surface_5")]
mod khr_wayland_surface;

#[cfg(feature = "khr_wayland_surface_5")]
pub use khr_wayland_surface::*;

#[cfg(feature = "khr_mir_surface_4")]
mod khr_mir_surface;

#[cfg(feature = "khr_mir_surface_4")]
pub use khr_mir_surface::*;

#[cfg(feature = "khr_android_surface_6")]
mod khr_android_surface;

#[cfg(feature = "khr_android_surface_6")]
pub use khr_android_surface::*;

#[cfg(feature = "khr_win32_surface_5")]
mod khr_win32_surface;

#[cfg(feature = "khr_win32_surface_5")]
pub use khr_win32_surface::*;

#[cfg(feature = "khr_sampler_mirror_clamp_to_edge_1")]
mod khr_sampler_mirror_clamp_to_edge;

#[cfg(feature = "khr_sampler_mirror_clamp_to_edge_1")]
pub use khr_sampler_mirror_clamp_to_edge::*;

#[cfg(feature = "ext_debug_report_1")]
mod ext_debug_report;

#[cfg(feature = "ext_debug_report_1")]
pub use ext_debug_report::*;

#[cfg(feature = "nv_glsl_shader_1")]
mod nv_glsl_shader;

#[cfg(feature = "nv_glsl_shader_1")]
pub use nv_glsl_shader::*;

#[cfg(feature = "img_filter_cubic_1")]
mod img_filter_cubic;

#[cfg(feature = "img_filter_cubic_1")]
pub use img_filter_cubic::*;

#[cfg(feature = "amd_rasterization_order_1")]
mod amd_rasterization_order;

#[cfg(feature = "amd_rasterization_order_1")]
pub use amd_rasterization_order::*;

#[cfg(feature = "amd_shader_trinary_minmax_1")]
mod amd_shader_trinary_minmax;

#[cfg(feature = "amd_shader_trinary_minmax_1")]
pub use amd_shader_trinary_minmax::*;

#[cfg(feature = "amd_shader_explicit_vertex_parameter_1")]
mod amd_shader_explicit_vertex_parameter;

#[cfg(feature = "amd_shader_explicit_vertex_parameter_1")]
pub use amd_shader_explicit_vertex_parameter::*;

#[cfg(feature = "ext_debug_marker_3")]
mod ext_debug_marker;

#[cfg(feature = "ext_debug_marker_3")]
pub use ext_debug_marker::*;

#[cfg(feature = "khr_xlib_surface_6")]
pub mod xlib_wrapper;

#[cfg(feature = "khr_xcb_surface_6")]
pub mod xcb_wrapper;

#[cfg(feature = "khr_wayland_surface_5")]
pub mod wayland_wrapper;

#[cfg(feature = "khr_mir_surface_4")]
pub mod mir_wrapper;

#[cfg(feature = "khr_android_surface_6")]
pub mod android_wrapper;

#[cfg(feature = "khr_win32_surface_5")]
pub mod win32_wrapper;
