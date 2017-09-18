# vks

Vulkan FFI bindings and symbol loader for Rust

Latest supported Vulkan specification: 1.0.54 + all extensions

If you are looking for a safe and more Rust-like interface, checkout the [dacite] project.

[![build status](https://gitlab.com/dennis-hamester/vks/badges/master/build.svg)](https://gitlab.com/dennis-hamester/vks)
[![vks on crates.io](https://img.shields.io/crates/v/vks.svg)](https://crates.io/crates/vks)
[![vks on docs.rs](https://docs.rs/vks/badge.svg)](https://docs.rs/vks)
[![](https://tokei.rs/b1/gitlab/dennis-hamester/vks?category=lines)](https://gitlab.com/dennis-hamester/vks)
[![vks license](https://img.shields.io/badge/license-ISC-blue.svg)](LICENSE)

[dacite]: https://gitlab.com/dennis-hamester/dacite

## Usage

Vks is available on [crates.io](https://crates.io/crates/vks). Add this to your `Cargo.toml`:

```toml
[dependencies]
vks = "0.19"
```

### Windows

On Windows, linking vks requires `vulkan-1.lib`, if the feature `function_prototypes` is enabled.
Make sure the environment variable `VULKAN_SDK` points to the root of the LunarG Vulkan SDK. This is
the default, if you use the Vulkan SDK installer.

## Cargo Features

### `function_prototypes` Feature

Enabling this feature, will cause all function definitions to be included. Linking against Vulkan
(`libvulkan.so.1` or `vulkan-1.dll`) is required in this case.

It is recommended to not use this feature, but instead load the Vulkan library dynamically at
runtime and acquire the symbol `vkGetInstanceProcAddr`. With just this symbol, you can then use
`InstanceProcAddrLoader` (and by extension, `DeviceProcAddrLoader`) to load all remaining function
pointers.

### `experimental` Feature

Vks includes support for experimental Vulkan extensions (recognizable by the prefix `KHX` or
similar), but gates them behind a feature. The reason is, that these extensions can change in a
backwards-incompatible way, or even be removed in future Vulkan releases.

Be aware, that vks updates might break your code, if you use this feature. Everything behind this
feature will be ignored in terms of Semantic Versioning requirements.

You should not ship code that depends on these extensions or uses the `experimental` feature.

## Loader

Vks includes two convenience Vulkan symbol loaders: `InstanceProcAddrLoader` and
`DeviceProcAddrLoader`. Both support all available core Vulkan functions as well as extension
function pointers.

## Supported Extensions

### `KHR` Extensions

| Extension | Revision |
| --- | --- |
| `VK_KHR_16bit_storage` | 1 |
| `VK_KHR_android_surface` | 6 |
| `VK_KHR_dedicated_allocation` | 1 |
| `VK_KHR_descriptor_update_template` | 1 |
| `VK_KHR_display_swapchain` | 9 |
| `VK_KHR_display` | 21 |
| `VK_KHR_external_fence_capabilities` | 1 |
| `VK_KHR_external_fence_fd` | 1 |
| `VK_KHR_external_fence_win32` | 1 |
| `VK_KHR_external_fence` | 1 |
| `VK_KHR_external_memory_capabilities` | 1 |
| `VK_KHR_external_memory_fd` | 1 |
| `VK_KHR_external_memory_win32` | 1 |
| `VK_KHR_external_memory` | 1 |
| `VK_KHR_external_semaphore_capabilities` | 1 |
| `VK_KHR_external_semaphore_fd` | 1 |
| `VK_KHR_external_semaphore_win32` | 1 |
| `VK_KHR_external_semaphore` | 1 |
| `VK_KHR_get_memory_requirements2` | 1 |
| `VK_KHR_get_physical_device_properties2` | 1 |
| `VK_KHR_get_surface_capabilities2` | 1 |
| `VK_KHR_incremental_present` | 1 |
| `VK_KHR_maintenance1` | 1 |
| `VK_KHR_mir_surface` | 4 |
| `VK_KHR_push_descriptor` | 1 |
| `VK_KHR_sampler_mirror_clamp_to_edge` | 1 |
| `VK_KHR_shader_draw_parameters` | 1 |
| `VK_KHR_shared_presentable_image` | 1 |
| `VK_KHR_storage_buffer_storage_class` | 1 |
| `VK_KHR_surface` | 25 |
| `VK_KHR_swapchain` | 68 |
| `VK_KHR_variable_pointers` | 1 |
| `VK_KHR_wayland_surface` | 6 |
| `VK_KHR_win32_keyed_mutex` | 1 |
| `VK_KHR_win32_surface` | 6 |
| `VK_KHR_xcb_surface` | 6 |
| `VK_KHR_xlib_surface` | 6 |

### `EXT` Extensions

| Extension | Revision |
| --- | --- |
| `VK_EXT_acquire_xlib_display` | 1 |
| `VK_EXT_blend_operation_advanced` | 2 |
| `VK_EXT_debug_marker` | 4 |
| `VK_EXT_debug_report` | 8 |
| `VK_EXT_direct_mode_display` | 1 |
| `VK_EXT_discard_rectangles` | 1 |
| `VK_EXT_display_control` | 1 |
| `VK_EXT_display_surface_counter` | 1 |
| `VK_EXT_hdr_metadata` | 1 |
| `VK_EXT_sampler_filter_minmax` | 1 |
| `VK_EXT_shader_subgroup_ballot` | 1 |
| `VK_EXT_shader_subgroup_vote` | 1 |
| `VK_EXT_swapchain_colorspace` | 2 |
| `VK_EXT_validation_flags` | 1 |

### `AMD` Extensions

| Extension | Revision |
| --- | --- |
| `VK_AMD_draw_indirect_count` | 1 |
| `VK_AMD_gcn_shader` | 1 |
| `VK_AMD_gpu_shader_half_float` | 1 |
| `VK_AMD_gpu_shader_int16` | 1 |
| `VK_AMD_negative_viewport_height` | 1 |
| `VK_AMD_rasterization_order` | 1 |
| `VK_AMD_shader_ballot` | 1 |
| `VK_AMD_shader_explicit_vertex_parameter` | 1 |
| `VK_AMD_shader_trinary_minmax` | 1 |
| `VK_AMD_texture_gather_bias_lod` | 1 |

### `GOOGLE` Extensions

| Extension | Revision |
| --- | --- |
| `VK_GOOGLE_display_timing` | 1 |

### `IMG` Extensions

| Extension | Revision |
| --- | --- |
| `VK_IMG_filter_cubic` | 1 |
| `VK_IMG_format_pvrtc` | 1 |

### `MVK` Extensions

| Extension | Revision |
| --- | --- |
| `VK_MVK_ios_surface` | 2 |
| `VK_MVK_macos_surface` | 2 |

### `NN` Extensions

| Extension | Revision |
| --- | --- |
| `VK_NN_vi_surface` | 1 |

### `NV` Extensions

| Extension | Revision |
| --- | --- |
| `VK_NV_clip_space_w_scaling` | 1 |
| `VK_NV_dedicated_allocation` | 1 |
| `VK_NV_external_memory_capabilities` | 1 |
| `VK_NV_external_memory_win32` | 1 |
| `VK_NV_external_memory` | 1 |
| `VK_NV_fill_rectangle` | 1 |
| `VK_NV_fragment_coverage_to_color` | 1 |
| `VK_NV_framebuffer_mixed_samples` | 1 |
| `VK_NV_geometry_shader_passthrough` | 1 |
| `VK_NV_glsl_shader` | 1 |
| `VK_NV_sample_mask_override_coverage` | 1 |
| `VK_NV_viewport_array2` | 1 |
| `VK_NV_viewport_swizzle` | 1 |
| `VK_NV_win32_keyed_mutex` | 1 |

### `KHX` Extensions

You must enable the `experimental` feature to use any of these extensions.

| Extension | Revision |
| --- | --- |
| `VK_KHX_device_group_creation` | 1 |
| `VK_KHX_device_group` | 1 |
| `VK_KHX_multiview` | 1 |

### `NVX` Extensions

You must enable the `experimental` feature to use any of these extensions.

| Extension | Revision |
| --- | --- |
| `VK_NVX_device_generated_commands` | 1 |
| `VK_NVX_multiview_per_view_attributes` | 1 |

## License

Vks is licensed under the ISC license:

```
Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
```
