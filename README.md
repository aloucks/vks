# vk-sys

[![build status](https://gitlab.com/dennis-hamester/vk-sys/badges/master/build.svg)](https://gitlab.com/dennis-hamester/vk-sys)

Vulkan bindings and symbol loader for Rust

Latest supported Vulkan specification: 1.0.24 + all extensions

For the time being, vk-sys requires a nightly version of the Rust compiler, because we use the
`struct_field_attributes` and `untagged_unions` features.

## Cargo Features

vk-sys supports very fine grained compile-time configuration via Cargo features. The version of the
core specification and the revision for each extension can be selected individually. Dependencies
between available features are modelled as well. This means, selecting nothing but a single
extension revision (i.e. via `cargo build --no-default-extensions --features khr_wayland_surface_5`)
will still pull in the lowest core specification, that supported that extension (i.e. `core_1_0_3`),
as well as possibly other dependencies (i.e. `khr_surface_25`).

The `default` feature will always select the latest fully supported core Vulkan specification as
well as all extensions, that are supported and have been defined up to that point.

### `vk_*` Features

All of these features select all extensions, that where defined up that point.

| Feature | Description |
| --- | --- |
| `vk_1_0_25` | Vulkan 1.0.25 + all extensions |
| `vk_1_0_24` | Vulkan 1.0.24 + all extensions |
| `vk_1_0_23` | Vulkan 1.0.23 + all extensions |
| `vk_1_0_22` | Vulkan 1.0.22 + all extensions |
| `vk_1_0_21` | Vulkan 1.0.21 + all extensions |
| `vk_1_0_20` | Vulkan 1.0.20 + all extensions |
| `vk_1_0_19` | Vulkan 1.0.19 + all extensions |
| `vk_1_0_18` | Vulkan 1.0.18 + all extensions |
| `vk_1_0_17` | Vulkan 1.0.17 + all extensions |
| `vk_1_0_16` | Vulkan 1.0.16 + all extensions |
| `vk_1_0_15` | Vulkan 1.0.15 + all extensions |
| `vk_1_0_14` | Vulkan 1.0.14 + all extensions |
| `vk_1_0_13` | Vulkan 1.0.13 + all extensions |
| `vk_1_0_12` | Vulkan 1.0.12 + all extensions |
| `vk_1_0_11` | Vulkan 1.0.11 + all extensions |
| `vk_1_0_10` | Vulkan 1.0.10 + all extensions |
| `vk_1_0_9` | Vulkan 1.0.9 + all extensions |
| `vk_1_0_8` | Vulkan 1.0.8 + all extensions |
| `vk_1_0_7` | Vulkan 1.0.7 + all extensions |
| `vk_1_0_6` | Vulkan 1.0.6 + all extensions |
| `vk_1_0_5` | Vulkan 1.0.5 + all extensions |
| `vk_1_0_4` | Vulkan 1.0.4 + all extensions |
| `vk_1_0_3` | Vulkan 1.0.3 + all extensions |

### `core_*` Features

These features do not select any extensions.

| Feature | Description |
| --- | --- |
| `core` | Latest supported Vulkan specification |
| `core_1_0_25` | Vulkan 1.0.25 |
| `core_1_0_24` | Vulkan 1.0.24 |
| `core_1_0_23` | Vulkan 1.0.23 |
| `core_1_0_22` | Vulkan 1.0.22 |
| `core_1_0_21` | Vulkan 1.0.21 |
| `core_1_0_20` | Vulkan 1.0.20 |
| `core_1_0_19` | Vulkan 1.0.19 |
| `core_1_0_18` | Vulkan 1.0.18 |
| `core_1_0_17` | Vulkan 1.0.17 |
| `core_1_0_16` | Vulkan 1.0.16 |
| `core_1_0_15` | Vulkan 1.0.15 |
| `core_1_0_14` | Vulkan 1.0.14 |
| `core_1_0_13` | Vulkan 1.0.13 |
| `core_1_0_12` | Vulkan 1.0.12 |
| `core_1_0_11` | Vulkan 1.0.11 |
| `core_1_0_9` | Vulkan 1.0.9 |
| `core_1_0_8` | Vulkan 1.0.8 |
| `core_1_0_7` | Vulkan 1.0.7 |
| `core_1_0_6` | Vulkan 1.0.6 |
| `core_1_0_5` | Vulkan 1.0.5 |
| `core_1_0_4` | Vulkan 1.0.4 |
| `core_1_0_3` | Vulkan 1.0.3 |

### Extension Features
All of these features will (at least) select the lowest required Vulkan specification and possibly
other dependencies.

| Feature | Description |
| --- | --- |
| `amd_gcn_shader_1` | `VK_AMD_gcn_shader` revision 1 |
| `amd_gcn_shader` | Latest `VK_AMD_gcn_shader` revision |
| `amd_rasterization_order_1` | `VK_AMD_rasterization_order` revision 1 |
| `amd_rasterization_order` | Latest `VK_AMD_rasterization_order` revision |
| `amd_shader_explicit_vertex_parameter_1` | `VK_AMD_shader_explicit_vertex_parameter` revision 1 |
| `amd_shader_explicit_vertex_parameter` | Latest `VK_AMD_shader_explicit_vertex_parameter` revision |
| `amd_shader_trinary_minmax_1` | `VK_AMD_shader_trinary_minmax` revision 1 |
| `amd_shader_trinary_minmax` | Latest `VK_AMD_shader_trinary_minmax` revision |
| `ext_debug_marker_3` | `VK_EXT_debug_marker` revision 3 |
| `ext_debug_marker` | Latest `VK_EXT_debug_marker` revision |
| `ext_debug_report_1` | `VK_EXT_debug_report` revision 1 |
| `ext_debug_report_2` | `VK_EXT_debug_report` revision 2 |
| `ext_debug_report_3` | `VK_EXT_debug_report` revision 3 |
| `ext_debug_report_4` | `VK_EXT_debug_report` revision 4 |
| `ext_debug_report` | Latest `VK_EXT_debug_report` revision |
| `img_filter_cubic_1` | `VK_IMG_filter_cubic` revision 1 |
| `img_filter_cubic` | Latest `VK_IMG_filter_cubic` revision |
| `img_format_pvrtc_1` | `VK_IMG_format_pvrtc` revision 1 |
| `img_format_pvrtc` | Latest `VK_IMG_format_pvrtc` revision |
| `khr_android_surface_6` | `VK_KHR_android_surface` revision 6 |
| `khr_android_surface` | Latest `VK_KHR_android_surface` revision |
| `khr_display_21` | `VK_KHR_display` revision 21 |
| `khr_display_swapchain_9` | `VK_KHR_display_swapchain` revision 9 |
| `khr_display_swapchain` | Latest `VK_KHR_display_swapchain` revision |
| `khr_display` | Latest `VK_KHR_display` revision |
| `khr_mir_surface_4` | `VK_KHR_mir_surface` revision 4 |
| `khr_mir_surface` | Latest `VK_KHR_mir_surface` revision |
| `khr_sampler_mirror_clamp_to_edge_1` | `VK_KHR_sampler_mirror_clamp_to_edge` revision 1 |
| `khr_sampler_mirror_clamp_to_edge` | Latest `VK_KHR_sampler_mirror_clamp_to_edge` revision |
| `khr_surface_25` | `VK_KHR_surface` revision 25 |
| `khr_surface` | Latest `VK_KHR_surface` revision |
| `khr_swapchain_67` | `VK_KHR_swapchain` revision 67 |
| `khr_swapchain_68` | `VK_KHR_swapchain` revision 68 |
| `khr_swapchain` | Latest `VK_KHR_surface` revision |
| `khr_wayland_surface_5` | `VK_KHR_wayland_s` revision 5 |
| `khr_wayland_surface` | Latest `VK_KHR_wayland_surface` revision |
| `khr_win32_surface_5` | `VK_KHR_win32_surface` revision 5 |
| `khr_win32_surface` | Latest `VK_KHR_win32_surface` revision |
| `khr_xcb_surface_6` | `VK_KHR_xcb_surface` revision 6 |
| `khr_xcb_surface` | Latest `VK_KHR_xcb_surface` revision |
| `khr_xlib_surface_6` | `VK_KHR_xlib_surface` revision 6 |
| `khr_xlib_surface` | Latest `VK_KHR_xlib_surface` revision |
| `nv_dedicated_allocation_1` | `VK_NV_dedicated_allocation` revision 1 |
| `nv_dedicated_allocation` | Latest `VK_NV_dedicated_allocation` revision |
| `nv_glsl_shader_1` | `VK_NV_glsl_shader` revision 1 |
| `nv_glsl_shader` | Latest `VK_NV_glsl_shader` revision |

## Loader

vk-sys includes two convenience Vulkan symbol loaders: `InstanceProcAddrLoader` and
`DeviceProcAddrLoader`. Both support all available core Vulkan functions including extension
function pointers. The exact set of function pointers depend on the enabled Cargo features.

## Supported Vulkan Specifications

 * 1.0.25
 * 1.0.24
 * 1.0.23
 * 1.0.22
 * 1.0.21
 * 1.0.20
 * 1.0.19
 * 1.0.18
 * 1.0.17
 * 1.0.16
 * 1.0.15
 * 1.0.14
 * 1.0.13
 * 1.0.12
 * 1.0.11
 * 1.0.10
 * 1.0.9
 * 1.0.8
 * 1.0.7
 * 1.0.6
 * 1.0.5
 * 1.0.4
 * 1.0.3

## Supported Extensions

### `KHR` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_KHR_android_surface` | 6 |
| `VK_KHR_display_swapchain` | 9 |
| `VK_KHR_display` | 21 |
| `VK_KHR_mir_surface` | 4 |
| `VK_KHR_sampler_mirror_clamp_to_edge` | 1 |
| `VK_KHR_surface` | 25 |
| `VK_KHR_swapchain` | 67, 68 |
| `VK_KHR_wayland_surface` | 5 |
| `VK_KHR_win32_surface` | 5 |
| `VK_KHR_xcb_surface` | 6 |
| `VK_KHR_xlib_surface` | 6 |

### `EXT` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_EXT_debug_marker` | 3 |
| `VK_EXT_debug_report` | 1, 2, 3, 4 |

### `NV` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_NV_dedicated_allocation` | 1 |
| `VK_NV_glsl_shader` | 1 |

### `IMG` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_IMG_filter_cubic` | 1 |
| `VK_IMG_format_pvrtc` | 1 |

### `AMD` Extensions

| Extension | Revision(s) |
| --- | --- |
| `VK_AMD_gcn_shader` | 1 |
| `VK_AMD_rasterization_order` | 1 |
| `VK_AMD_shader_explicit_vertex_parameter` | 1 |
| `VK_AMD_shader_trinary_minmax` | 1 |

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
