# vks

**Caution**: Don't use this library on i686 platforms. There is a bug in vks, which makes this
library completely unusable on such platforms. See issue #111 for more information.

[![build status](https://gitlab.com/dennis-hamester/vks/badges/master/build.svg)](https://gitlab.com/dennis-hamester/vks)

Vulkan FFI bindings and symbol loader for Rust

Latest supported Vulkan specification: 1.0.53 + all extensions

If you are looking for a safe and more Rust-like interface, checkout the [dacite] project.

[dacite]: https://gitlab.com/dennis-hamester/dacite

## Usage

Vks is available on [crates.io](https://crates.io/crates/vks). Add this to your `Cargo.toml`:

```toml
[dependencies]
vks = "0.19"
```

### Windows

On Windows, linking vks requires `vulkan-1.lib`, if the feature `no_function_prototypes` is not
enabled. Make sure the environment variable `VULKAN_SDK` points to the root of the LunarG Vulkan
SDK. This is the default, if you use the Vulkan SDK installer.

## Cargo Features

Vks supports very fine grained compile-time configuration via Cargo features. The version of the
core specification and the revision for each extension can be selected individually. Dependencies
between available features are modelled as well. This means, selecting nothing but a single
extension revision (i.e. via `cargo build --no-default-features --features khr_wayland_surface_5`)
will still pull in the lowest core specification, that supported that extension (i.e. `core_1_0_3`),
as well as possibly other dependencies (i.e. `khr_surface_25`).

The `default` feature will always select the latest fully supported core Vulkan specification as
well as all extensions, that are supported and have been defined up to that point. Basically,
`default` will select everything, except for features, which are incomplete and still in
development.

### `no_function_prototypes` Feature

Enabling this feature, will cause all function definitions to be omitted. Linking against Vulkan is
not required in this case. This feature is especially useful, if you load Vulkan dynamically at
runtime and don't need the function definitions anyway.

### `unstable_rust` Feature

This feature is optional and not included in the `default` feature, because it requires a nightly
version of the Rust compiler. The feature enables `untagged_unions` and `struct_field_attributes`.

When `untagged_unions` and `struct_field_attributes` become available in stable Rust, we will remove
the current work-arounds and switch to what is now behind `unstable_rust`.

### `vk_*` Features

All of these features select a specific core specification as well as all extensions, that were
defined up to that point. Features in this category have the form `vk_a_b_c`, where `a`, `b`, and
`c` refer to the Vulkan specification. For example, `vk_1_0_32` will select the core Vulkan 1.0.32
specification and all extensions, that existed at that point. The earliest version that can be
selected is 1.0.3 (via `vk_1_0_3`). The most recent version is selected by the `default` feature.

### `core_*` Features

These features select only the core specification without any extensions. They have the same form
and range of valid versions as the `vk_*` features, except for the `core_` prefix. The earliest
version that can be selected is 1.0.3 (via `core_1_0_3`). The most recent version is selected by the
`core` feature.

### Extension Features

Every extension maps to a Cargo feature by removing the `VK_` prefix from its lowercase name. For
example, the feature `amd_negative_viewport_height` corresponds to the extension
`VK_AMD_negative_viewport_height`. Features that are formed this way, will always select the most recent
extension revision, as well as the lowest compatible core specification (usually 1.0.3 via
`core_1_0_3`) and possibly other dependencies.

Specific revisions can be selected by appending `_x` to the feature, where `x` is the desired
revision. For example, `ext_debug_report_3` selects revision 3 of `VK_EXT_debug_report`, instead of
the newest revision.

## Loader

Vks includes two convenience Vulkan symbol loaders: `InstanceProcAddrLoader` and
`DeviceProcAddrLoader`. Both support all available core Vulkan functions including extension
function pointers. The exact set of function pointers depends on the enabled Cargo features.

## Supported Vulkan Specifications

 * Latest: 1.0.53
 * Earliest: 1.0.3

Every version in between is supported as well.

 * Version 1.0.52 has not been released publicly and is also not supported by vks.

## Supported Extensions

### `KHR` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_KHR_android_surface` | 6 |
| `VK_KHR_descriptor_update_template` | 1 |
| `VK_KHR_display_swapchain` | 9 |
| `VK_KHR_display` | 21 |
| `VK_KHR_get_physical_device_properties2` | 1 |
| `VK_KHR_get_surface_capabilities2` | 1 |
| `VK_KHR_incremental_present` | 1 |
| `VK_KHR_maintenance1` | 1 |
| `VK_KHR_mir_surface` | 4 |
| `VK_KHR_push_descriptor` | 1 |
| `VK_KHR_sampler_mirror_clamp_to_edge` | 1 |
| `VK_KHR_shader_draw_parameters` | 1 |
| `VK_KHR_shared_presentable_image` | 1 |
| `VK_KHR_surface` | 25 |
| `VK_KHR_swapchain` | 67, 68 |
| `VK_KHR_wayland_surface` | 5, 6 |
| `VK_KHR_win32_surface` | 5, 6 |
| `VK_KHR_xcb_surface` | 6 |
| `VK_KHR_xlib_surface` | 6 |

### `EXT` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_EXT_acquire_xlib_display` | 1 |
| `VK_EXT_debug_marker` | 3, 4 |
| `VK_EXT_debug_report` | 1, 2, 3, 4, 5, 6, 8 |
| `VK_EXT_direct_mode_display` | 1 |
| `VK_EXT_discard_rectangles` | 1 |
| `VK_EXT_display_control` | 1 |
| `VK_EXT_display_surface_counter` | 1 |
| `VK_EXT_hdr_metadata` | 1 |
| `VK_EXT_shader_subgroup_ballot` | 1 |
| `VK_EXT_shader_subgroup_vote` | 1 |
| `VK_EXT_swapchain_colorspace` | 2 |
| `VK_EXT_validation_flags` | 1 |

### `NV` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NV_clip_space_w_scaling` | 1 |
| `VK_NV_dedicated_allocation` | 1 |
| `VK_NV_external_memory_capabilities` | 1 |
| `VK_NV_external_memory_win32` | 1 |
| `VK_NV_external_memory` | 1 |
| `VK_NV_geometry_shader_passthrough` | 1 |
| `VK_NV_glsl_shader` | 1 |
| `VK_NV_sample_mask_override_coverage` | 1 |
| `VK_NV_viewport_array2` | 1 |
| `VK_NV_viewport_swizzle` | 1 |
| `VK_NV_win32_keyed_mutex` | 1 |

### `IMG` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_IMG_filter_cubic` | 1 |
| `VK_IMG_format_pvrtc` | 1 |

### `AMD` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_AMD_draw_indirect_count` | 1 |
| `VK_AMD_gcn_shader` | 1 |
| `VK_AMD_gpu_shader_half_float` | 1 |
| `VK_AMD_negative_viewport_height` | 1 |
| `VK_AMD_rasterization_order` | 1 |
| `VK_AMD_shader_ballot` | 1 |
| `VK_AMD_shader_explicit_vertex_parameter` | 1 |
| `VK_AMD_shader_trinary_minmax` | 1 |
| `VK_AMD_texture_gather_bias_lod` | 1 |

### `NVX` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NVX_device_generated_commands` | 1 |
| `VK_NVX_multiview_per_view_attributes` | 1 |

### `NN` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NN_vi_surface` | 1 |

### `KHX` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_KHX_device_group_creation` | 1 |
| `VK_KHX_device_group` | 1 |
| `VK_KHX_external_memory_capabilities` | 1 |
| `VK_KHX_external_memory_fd` | 1 |
| `VK_KHX_external_memory_win32` | 1 |
| `VK_KHX_external_memory` | 1 |
| `VK_KHX_external_semaphore_capabilities` | 1 |
| `VK_KHX_external_semaphore_fd` | 1 |
| `VK_KHX_external_semaphore_win32` | 1 |
| `VK_KHX_external_semaphore` | 1 |
| `VK_KHX_multiview` | 1 |
| `VK_KHX_win32_keyed_mutex` | 1 |

### `MVK` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_MVK_ios_surface` | 2 |
| `VK_MVK_macos_surface` | 2 |

### `GOOGLE` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_GOOGLE_display_timing` | 1 |

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
