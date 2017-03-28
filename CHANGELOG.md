# vk-sys Changelog

## Current Git Master Branch

 * Vulkan 1.0.31, 1.0.32, 1.0.33
 * New extension `VK_AMD_negative_viewport_height` (revision 1)
 * New extension `VK_AMD_shader_ballot` (revision 1)

## Version 0.7.0, released on 27.03.2017

 * Vulkan 1.0.26, 1.0.27, 1.0.28, 1.0.29, 1.0.30
 * New extension `VK_EXT_validation_flags` (revision 1)
 * New extension `VK_AMD_gpu_shader_half_float` (revision 1)

## Version 0.6.1, released on 27.03.2017

 * Revert `VK_EXT_debug_report` back to revision 3. It seems this was an accident in the Vulkan
   1.0.25 specification, which was corrected in 1.0.26.
 * Fix version and name constants for `VK_AMD_draw_indirect_count`.

## Version 0.6.0, released on 26.03.2017

 * Vulkan 1.0.21, 1.0.22, 1.0.23, 1.0.24, 1.0.25
 * All enums have been converted to newtype structs and their values are now global constants.
   Unknown values can now be handled properly (without causing undefined behaviour). This change
   also brings back some backwards compatibility, because we don't have to remove old values, when
   an enum value gets renamed.
 * New extension `VK_IMG_format_pvrtc` (revision 1)
 * Support revision 4 of `VK_EXT_debug_report`
 * New extension `VK_AMD_draw_indirect_count` (revision 1)
 * New extension `VK_NV_external_memory_capabilities` (revision 1)
 * New extension `VK_NV_external_memory` (revision 1)
 * New extension `VK_NV_external_memory_win32` (revision 1)
 * New extension `VK_NV_win32_keyed_mutex` (revision 1)

## Version 0.5.0, released on 26.03.2017

 * Vulkan 1.0.16, 1.0.17, 1.0.18, 1.0.19, 1.0.20
 * New extension `VK_AMD_shader_trinary_minmax` (revision 1)
 * New extension `VK_AMD_shader_explicit_vertex_parameter` (revision 1)
 * New extension `VK_AMD_gcn_shader` (revision 1)
 * All extensions now depend on the core 1.0.3 specification, instead of something higher.
 * Support revision 3 of `VK_EXT_debug_report`
 * Vulkan 1.0.19: Changed type of parameter `pData` from `*const u32` to `*const c_void` in function
   `vkCmdUpdateBuffer`.
 * New extension `VK_NV_dedicated_allocation` (revision 1)

## Version 0.4.0, released on 25.03.2017

 * Vulkan 1.0.12, 1.0.13, 1.0.14, 1.0.15
 * `VK_AMD_rasterization_order` revision 1
 * `VK_KHR_surface`: Rename `VK_COLORSPACE_SRGB_NONLINEAR_KHR` to `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`
 * `VK_EXT_debug_marker` revision 3

## Version 0.3.0, released on 25.03.2017

 * Vulkan 1.0.6, 1.0.7, 1.0.8, 1.0.9, 1.0.10, 1.0.11
 * `VK_IMG_filter_cubic` revision 1
 * `instance_proc_addr_loader`: Implement Default for generated structs
 * `device_proc_addr_loader`: Implement Default for generated structs
 * Add support for [clippy](https://github.com/Manishearth/rust-clippy).
 * `VK_KHR_swapchain` revision 68

## Version 0.2.0, released on 25.03.2017

 * Vulkan 1.0.5
 * Add `*_SPEC_VERSION` constant to all extensions.
 * `VK_NV_glsl_shader` revision 1
 * Reduce required core version of `VK_EXT_debug_report` revision 2 from 1.0.4 to 1.0.3.

## Version 0.1.0, released on 25.03.2017

 * Initial release
 * Vulkan 1.0.3 and 1.0.4
 * `InstanceProcAddrLoader` and `DeviceProcAddrLoader`
 * `VK_KHR_android_surface` revision 6
 * `VK_KHR_display_swapchain` revision 9
 * `VK_KHR_display` revision 21
 * `VK_KHR_mir_surface` revision 4
 * `VK_KHR_sampler_mirror_clamp_to_edge` revision 1
 * `VK_KHR_surface` revision 25
 * `VK_KHR_swapchain` revision 67
 * `VK_KHR_wayland_surface` revision 5
 * `VK_KHR_win32_surface` revision 5
 * `VK_KHR_xcb_surface` revision 6
 * `VK_KHR_xlib_surface` revision 6
 * `VK_EXT_debug_report` revisions 1 and 2
