# vk-sys

[![build status](https://gitlab.com/dennis-hamester/vk-sys/badges/master/build.svg)](https://gitlab.com/dennis-hamester/vk-sys)

Vulkan bindings and symbol loader for Rust

Latest supported Vulkan specification: 1.0.40 + all extensions

For the time being, vk-sys requires a nightly version of the Rust compiler, because we use the
`struct_field_attributes` and `untagged_unions` features.

## Cargo Features

vk-sys supports very fine grained compile-time configuration via Cargo features. The version of the
core specification and the revision for each extension can be selected individually. Dependencies
between available features are modelled as well. This means, selecting nothing but a single
extension revision (i.e. via `cargo build --no-default-features --features khr_wayland_surface_5`)
will still pull in the lowest core specification, that supported that extension (i.e. `core_1_0_3`),
as well as possibly other dependencies (i.e. `khr_surface_25`).

The `default` feature will always select the latest fully supported core Vulkan specification as
well as all extensions, that are supported and have been defined up to that point. Basically,
`default` will select everything, except for features, which are incomplete and still in
development.

### `vk_*` Features

All of these features select a specific core specification as well as all extensions, that were
defined up that point. Features in this category have the form `vk_a_b_c`, where `a`, `b`, and `c`
refer to the Vulkan specification. For example, `vk_1_0_32` will select the core Vulkan 1.0.32
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

vk-sys includes two convenience Vulkan symbol loaders: `InstanceProcAddrLoader` and
`DeviceProcAddrLoader`. Both support all available core Vulkan functions including extension
function pointers. The exact set of function pointers depend on the enabled Cargo features.

## Supported Vulkan Specifications

 * Latest: 1.0.40
 * Earliest: 1.0.3

Every version in between is supported as well.

## Supported Extensions

### `KHR` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_KHR_android_surface` | 6 |
| `VK_KHR_display_swapchain` | 9 |
| `VK_KHR_display` | 21 |
| `VK_KHR_get_physical_device_properties2` | 1 |
| `VK_KHR_maintenance1` | 1 |
| `VK_KHR_mir_surface` | 4 |
| `VK_KHR_sampler_mirror_clamp_to_edge` | 1 |
| `VK_KHR_shader_draw_parameters` | 1 |
| `VK_KHR_surface` | 25 |
| `VK_KHR_swapchain` | 67, 68 |
| `VK_KHR_wayland_surface` | 5 |
| `VK_KHR_win32_surface` | 5 |
| `VK_KHR_xcb_surface` | 6 |
| `VK_KHR_xlib_surface` | 6 |

### `EXT` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_EXT_acquire_xlib_display` | 1 |
| `VK_EXT_debug_marker` | 3, 4 |
| `VK_EXT_debug_report` | 1, 2, 3, 4, 5 |
| `VK_EXT_direct_mode_display` | 1 |
| `VK_EXT_display_control` | 1 |
| `VK_EXT_display_surface_counter` | 1 |
| `VK_EXT_shader_subgroup_ballot` | 1 |
| `VK_EXT_shader_subgroup_vote` | 1 |
| `VK_EXT_validation_flags` | 1 |

### `NV` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NV_dedicated_allocation` | 1 |
| `VK_NV_external_memory_capabilities` | 1 |
| `VK_NV_external_memory_win32` | 1 |
| `VK_NV_external_memory` | 1 |
| `VK_NV_glsl_shader` | 1 |
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

### `NVX` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NVX_device_generated_commands` | 1 |

### `NN` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NN_vi_surface` | 1 |

## License

vk-sys is licensed under the ISC license:

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
