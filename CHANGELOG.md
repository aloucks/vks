# vks Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Items listed in "Changed" sub-sections are usually breaking changes. Any additional breaking changes
in other sub-sections are prefixed with "**BREAKING**" to increase visibility.


## [Unreleased]
This release includes many breaking changes from the previous version. A detailed list of all
breaking changes is provided below.

This most obvious change, is that you can no longer select specific Vulkan revisions or extensions
via Cargo features. All such features have been removed. This has been too much of a maintenance
burden in the past. Moreover, the benefits were questionable at best, because all vks does, is
provide types and function definitions. There was no benefit in selecting just a few extensions over
all of them.

### Added
 - Vulkan 1.0.54, 1.0.55, 1.0.56
 - Extension `VK_KHR_external_memory_capabilities` (revision 1)
 - Extension `VK_KHR_external_memory` (revision 1)
 - Extension `VK_KHR_external_memory_win32` (revision 1)
 - Extension `VK_KHR_external_memory_fd` (revision 1)
 - Extension `VK_KHR_win32_keyed_mutex` (revision 1)
 - Extension `VK_KHR_external_semaphore_capabilities` (revision 1)
 - Extension `VK_KHR_external_semaphore` (revision 1)
 - Extension `VK_KHR_external_semaphore_win32` (revision 1)
 - Extension `VK_KHR_external_semaphore_fd` (revision 1)
 - Extension `VK_KHR_16bit_storage` (revision 1)
 - Extension `VK_KHR_external_fence_capabilities` (revision 1)
 - Extension `VK_KHR_external_fence` (revision 1)
 - Extension `VK_KHR_external_fence_win32` (revision 1)
 - Extension `VK_KHR_external_fence_fd` (revision 1)
 - Extension `VK_KHR_variable_pointers` (revision 1)
 - Extension `VK_KHR_dedicated_allocation` (revision 1)
 - Extension `VK_KHR_storage_buffer_storage_class` (revision 1)
 - Extension `VK_KHR_get_memory_requirements2` (revision 1)
 - Revision 3 of `VK_EXT_swapchain_colorspace`

### Fixed
 - **BREAKING**: All `PFN_` function pointer types are now `Option<T>`, otherwise it is undefined
   behaviour if the pointer is `NULL`.

### Changed
 - Vks requires at least Rust 1.20.
 - Untagged unions are now used instead of the previous work-around. The feature `unstable_rust` has been removed.
 - Most Cargo features have been removed. Every vks release will now reflect a specific Vulkan release.
 - The feature `no_function_prototypes` has been renamed to `function_prototypes` and its meaning has been inverted.
 - Core Vulkan definitions are now in the `core` module.
 - Every extension is now in a separate module.
 - Experimental extensions (currently `KHX` and `NVX`) are only available, if the `experimental` Cargo feature is enabled.
 - Experimental extensions have been moved into a separate sub-module `experimental`.
 - `instance_proc_addr_loader::CoreNullInstance` has been renamed to `CoreGlobal`.
 - All `PFN_` function pointer types are now `Option<T>`.
 - All symbols in all loaders have been prefixed with `pfn_`.
 - All loaders now define functions in addition to the existing function pointers.
   For example, write `loader.core_global.vkCreateInstance(...)` instead of
   `(loader.core_global.vkCreateInstance)(...)`.
 - Functions of the extension `VK_EXT_debug_marker` now take parameters by `*const` pointers instead
   of `*mut`. See https://github.com/KhronosGroup/Vulkan-Docs/issues/513

### Removed (`experimental`)
Several `KHX` extensions have been stabilized and are now available as regular `KHR` extensions. As
part of this change, the corresponding `KHX` extensions have been removed.

 - `VK_KHX_external_memory_capabilities`
 - `VK_KHX_external_memory_fd`
 - `VK_KHX_external_memory_win32`
 - `VK_KHX_external_memory`
 - `VK_KHX_external_semaphore_capabilities`
 - `VK_KHX_external_semaphore_fd`
 - `VK_KHX_external_semaphore_win32`
 - `VK_KHX_external_semaphore`
 - `VK_KHX_win32_keyed_mutex`


## [0.19.1] - 2017-07-01

### Added
 - Vulkan 1.0.53
 - Add extension `VK_EXT_sampler_filter_minmax` (revision 1)
 - Add extension `VK_AMD_gpu_shader_int16` (revision 1)
 - Add extension `VK_EXT_blend_operation_advanced` (revision 2)
 - Add extension `VK_NV_fragment_coverage_to_color` (revision 1)
 - Add extension `VK_NV_framebuffer_mixed_samples` (revision 1)
 - Add extension `VK_NV_fill_rectangle` (revision 1)


## [0.19.0] - 2017-06-25
This release contains breaking changes.

### Fixed
 - **BREAKING**: Non-dispatchable Vulkan objects, bitflags and enumerations are no longer newtypes.
   See issue #111.


## [0.18.0] - 2017-06-16
This release contains breaking changes.

### Fixed
 - **BREAKING**: Fixed a rather serious bug on 32-bit targets: non-dispatchable Vulkan objects are
   now represented with a `u64` newtype. Before this change, vks used pointers as a representation,
   which is incorrect on 32-bit platforms. This bug basically made vks completely unusable on such
   platforms.  Default (as in uninitialized or NULL) non-dispatchable objects must now be created
   via the `Default` trait, or the `null()` function of that type, instead of
   `std::ptr::null_mut()`.

### Changed
 - `InstanceProcAddrLoader` and `DeviceProcAddrLoader` can no longer be created outside of vks. This
   ensures they are used in a forward-compatible way. All extension sub-structs have been changed as
   well.


## [0.17.0] - 2017-06-05
This release contains breaking changes.

### Added
 - Vulkan 1.0.51
 - Support revision 6 of `VK_KHR_win32_surface`.

### Changed
 - Values of `VkIndirectCommandsTokenTypeNVX` and `VkObjectEntryTypeNVX` from extension
   `VK_NVX_device_generated_commands` now include the word `TYPE` in their names.


## [0.16.1] - 2017-05-31

### Added
 - New Cargo feature `no_function_prototypes`: Enabling this feature, will cause all function
   definitions to be omitted. Linking against Vulkan is not required in this case. This feature is
   especially useful, if you load Vulkan dynamically at runtime and don't need the function
   definitions anyway.
 - `VULKAN_LIBRARY_NAME` was added, which contains either `libvulkan.so.1` or `vulkan-1.dll`,
   depending on the target platform.


## [0.16.0] - 2017-05-28
This release contains breaking changes.

### Added
 - Using `vks` on Windows should now be easier, if the environment variable `VULKAN_SDK` points to
   the root directory of the LunarG Vulkan SDK.

### Changed
 - Define `VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT` only if `VK_KHR_surface` is enabled.
 - Define `VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT` only if `VK_KHR_swapchain` is enabled.


## [0.15.0] - 2017-05-21
This release contains breaking changes.

### Added
 - Vulkan 1.0.50
 - Add `VK_VALIDATION_CHECK_SHADERS_EXT` to `VkValidationCheckEXT`.
 - Support revision 8 of `VK_EXT_debug_report`. Revision 7 is not explicitly supported, because
   there is no official `vulkan.h` with just that revision.
 - Add extension `VK_AMD_texture_gather_bias_lod` (revision 1).

### Fixed
 - Reverted b4ff4af. The symbol `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT` is now
   available starting with revision 2 of `VK_EXT_debug_report`.

### Changed
 - All `*_EXTENSION_SPEC_VERSION` have been renamed to `*_SPEC_VERSION`.


## [0.14.1] - 2017-05-21

### Fixed
 - Add missing `VK_EXT_DEBUG_REPORT_EXTENSION_SPEC_VERSION` for revision 6 of `VK_EXT_debug_report`.


## [0.14.0] - 2017-05-12

### Added
 - Vulkan 1.0.49
 - Add extension `VK_KHR_get_surface_capabilities2` (revision 1)
 - Add extension `VK_KHR_shared_presentable_image` (revision 1)


## [0.13.1] - 2017-05-07

### Fixed
 - Vulkan reference links for bitflags now point to the `*FlagBits` enum instead of the `*Flags`
   typedef.


## [0.13.0] - 2017-05-07
This release contains breaking changes.

### Added
 - Vulkan 1.0.48
 - All symbols are now documented with links to the official Vulkan reference.

### Fixed
 - Link against `vulkan-1` on Windows.
 - Fixed two `Default` implementations, which assigned incorrect `sType` values
   (`VkImageMemoryBarrier` and `VkRenderPassBeginInfo`).
 - Add missing type `VkInstanceCreateFlagBits`.

### Changed
 - Renamed project from `vk-sys` to `vks`.
 - The `new` function of `InstanceProcAddrLoader` has been made parameter-less, which initialized
   the field `vkGetInstanceProcAddr` with a null pointer. The previous behaviour was moved to the
   new function `from_get_instance_proc_addr`. `Default` has been implemented as well.
 - `DeviceProcAddrLoader` was changed also accordingly.


## [0.12.0] - 2017-04-08

### Added
 - Vulkan 1.0.47
 - Implement `Default` trait for all structs and bitflags.

### Fixed
 - Define `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT` in `VK_EXT_debug_report`
   revision 1 instead of 2. The corresponding struct `VkDebugReportCallbackCreateInfoEXT` existed
   already in revision 1.


## [0.11.0] - 2017-04-02

### Added
 - Vulkan 1.0.46
 - Stable Rust can now build vks!
 - Reimplemented `untagged_unions` and `struct_field_attributes` with stable Rust. This is enabled
   by default. The old behaviour is still available by selecting the Cargo feature `unstable_rust`.
 - Add `VK_KHR_swapchain` functions to `InstanceProcAddrLoader` and `DeviceProcAddrLoader`
 - Add `NV_external_memory_win32` functions to `DeviceProcAddrLoader`
 - Add extension `VK_KHR_incremental_present` (revision 1)
 - Support revision 6 of `VK_EXT_debug_report`
 - Add version related functions and constants: `VK_API_VERSION_1_0`, `VK_HEADER_VERSION`,
   `vk_version_major`, `vk_version_minor`, `vk_version_patch`, `vk_make_version`
 - Implement `Debug` for `VkClearAttachment`
 - `VK_KHR_descriptor_update_template`, `VK_KHX_external_memory_win32` and `VK_KHR_display` now use
   the correct (libc) `c_void` type


## [0.10.0] - 2017-04-02
This release contains breaking changes.

### Added
 - Vulkan 1.0.41, 1.0.42, 1.0.43, 1.0.44, 1.0.45
 - Add extension `VK_KHR_push_descriptor` (revision 1)
 - Add extension `VK_KHR_descriptor_update_template` (revision 1)
 - Add extension `VK_KHX_multiview` (revision 1)
 - Add extension `VK_KHX_device_group` (revision 1)
 - Add extension `VK_KHX_device_group_creation` (revision 1)
 - Add extension `VK_KHX_external_memory_capabilities` (revision 1)
 - Add extension `VK_KHX_external_memory` (revision 1)
 - Add extension `VK_KHX_external_memory_win32` (revision 1)
 - Add extension `VK_KHX_external_memory_fd` (revision 1)
 - Add extension `VK_KHX_win32_keyed_mutex` (revision 1)
 - Add extension `VK_KHX_external_semaphore_capabilities` (revision 1)
 - Add extension `VK_KHX_external_semaphore` (revision 1)
 - Add extension `VK_KHX_external_semaphore_win32` (revision 1)
 - Add extension `VK_KHX_external_semaphore_fd` (revision 1)
 - Add extension `VK_NV_clip_space_w_scaling` (revision 1)
 - Add extension `VK_NV_sample_mask_override_coverage` (revision 1)
 - Add extension `VK_NV_geometry_shader_passthrough` (revision 1)
 - Add extension `VK_NV_viewport_array2` (revision 1)
 - Add extension `VK_NVX_multiview_per_view_attributes` (revision 1)
 - Add extension `VK_NV_viewport_swizzle` (revision 1)
 - Add extension `VK_EXT_discard_rectangles` (revision 1)
 - Add extension `VK_MVK_ios_surface` (revision 2)
 - Add extension `VK_MVK_macos_surface` (revision 2)
 - Support revision 6 of `VK_KHR_wayland_surface`
 - Add extension `VK_GOOGLE_display_timing` (revision 1)
 - Add extension `VK_EXT_hdr_metadata` (revision 1)
 - Add extension `VK_EXT_swapchain_colorspace` (revision 2)

### Fixed
 - Fix building various features:
   `ext_debug_marker_3`, `ext_acquire_xlib_display`, `ext_acquire_xlib_display_1`,
   `khx_external_memory_win32`, `khx_external_memory_win32_1`, `khx_external_semaphore_win32`,
   `khx_external_semaphore_win32_1`, `nv_external_memory_win32`, `nv_external_memory_win32_1`,
   `nv_win32_keyed_mutex`, `nv_win32_keyed_mutex_1`

### Changed
 - The parameters `x`, `y`, and `z` of `vkCmdDispatch` have been renamed to `groupCountX`,
   `groupCountY`, and `groupCountZ`.


## [0.9.0] - 2017-04-01

### Added
 - Vulkan 1.0.36, 1.0.37, 1.0.38, 1.0.39, 1.0.40
 - Support revision 4 of `VK_EXT_debug_report`
 - Add extension `VK_KHR_get_physical_device_properties2` (revision 1)
 - Add extension `VK_KHR_shader_draw_parameters` (revision 1)
 - Add extension `VK_KHR_maintenance1` (revision 1)
 - Add extension `VK_NN_vi_surface` (revision 1)
 - Add extension `VK_EXT_shader_subgroup_ballot` (revision 1)
 - Add extension `VK_EXT_shader_subgroup_vote` (revision 1)
 - Add extension `VK_EXT_direct_mode_display` (revision 1)
 - Add extension `VK_EXT_acquire_xlib_display` (revision 1)
 - Add extension `VK_EXT_display_surface_counter` (revision 1)
 - Add extension `VK_EXT_display_control` (revision 1)
 - Support revision 4 of `VK_EXT_debug_marker`
 - Support revision 5 of `VK_EXT_debug_report`


## [0.8.2] - 2017-03-30

### Fixed
 - Add missing field `indexType` to `VkObjectTableIndexBufferEntryNVX`


## [0.8.1] - 2017-03-29

### Fixed
 - Add functions from `NVX_device_generated_commands` to InstanceProcAddrLoader
 - Add functions from `NVX_device_generated_commands` to DeviceProcAddrLoader


## [0.8.0] - 2017-03-28

### Added
 - Vulkan 1.0.31, 1.0.32, 1.0.33, 1.0.34, 1.0.35
 - New extension `VK_AMD_negative_viewport_height` (revision 1)
 - New extension `VK_AMD_shader_ballot` (revision 1)
 - New extension `VK_NVX_device_generated_commands` (revision 1)


## [0.7.0] - 2017-03-27

### Added
 - Vulkan 1.0.26, 1.0.27, 1.0.28, 1.0.29, 1.0.30
 - New extension `VK_EXT_validation_flags` (revision 1)
 - New extension `VK_AMD_gpu_shader_half_float` (revision 1)


## [0.6.1] - 2017-03-27

### Fixed
 - Revert `VK_EXT_debug_report` back to revision 3. It seems this was an accident in the Vulkan
   1.0.25 specification, which was corrected in 1.0.26.
 - Fix version and name constants for `VK_AMD_draw_indirect_count`.


## [0.6.0] - 2017-03-26
This release contains breaking changes.

### Added
 - Vulkan 1.0.21, 1.0.22, 1.0.23, 1.0.24, 1.0.25
 - New extension `VK_IMG_format_pvrtc` (revision 1)
 - Support revision 4 of `VK_EXT_debug_report`
 - New extension `VK_AMD_draw_indirect_count` (revision 1)
 - New extension `VK_NV_external_memory_capabilities` (revision 1)
 - New extension `VK_NV_external_memory` (revision 1)
 - New extension `VK_NV_external_memory_win32` (revision 1)
 - New extension `VK_NV_win32_keyed_mutex` (revision 1)

### Changed
 - All enums have been converted to newtype structs and their values are now global constants.
   Unknown values can now be handled properly (without causing undefined behaviour). This change
   also brings back some backwards compatibility, because we don't have to remove old values, when
   an enum value gets renamed.


## [0.5.0] - 2017-03-26
This release contains breaking changes.

### Added
 - Vulkan 1.0.16, 1.0.17, 1.0.18, 1.0.19, 1.0.20
 - New extension `VK_AMD_shader_trinary_minmax` (revision 1)
 - New extension `VK_AMD_shader_explicit_vertex_parameter` (revision 1)
 - New extension `VK_AMD_gcn_shader` (revision 1)
 - Support revision 3 of `VK_EXT_debug_report`
 - New extension `VK_NV_dedicated_allocation` (revision 1)

### Changed
 - All extensions now depend on the core 1.0.3 specification, instead of something higher.
 - Vulkan 1.0.19: Changed type of parameter `pData` from `*const u32` to `*const c_void` in function
   `vkCmdUpdateBuffer`.


## [0.4.0] - 2017-03-25
This release contains breaking changes.

### Added
 - Vulkan 1.0.12, 1.0.13, 1.0.14, 1.0.15
 - `VK_AMD_rasterization_order` revision 1
 - `VK_EXT_debug_marker` revision 3

### Changed
 - `VK_KHR_surface`: Rename `VK_COLORSPACE_SRGB_NONLINEAR_KHR` to `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`


## [0.3.0] - 2017-03-25

### Added
 - Vulkan 1.0.6, 1.0.7, 1.0.8, 1.0.9, 1.0.10, 1.0.11
 - `VK_IMG_filter_cubic` revision 1
 - `instance_proc_addr_loader`: Implement Default for generated structs
 - `device_proc_addr_loader`: Implement Default for generated structs
 - Add support for [clippy](https://github.com/Manishearth/rust-clippy).
 - `VK_KHR_swapchain` revision 68


## [0.2.0] - 2017-03-25

### Added
 - Vulkan 1.0.5
 - Add `*_SPEC_VERSION` constant to all extensions.
 - `VK_NV_glsl_shader` revision 1

### Fixed
 - Reduce required core version of `VK_EXT_debug_report` revision 2 from 1.0.4 to 1.0.3.


## [0.1.0] - 2017-03-25
Initial release

### Added
 - Vulkan 1.0.3 and 1.0.4
 - `InstanceProcAddrLoader` and `DeviceProcAddrLoader`
 - `VK_KHR_android_surface` revision 6
 - `VK_KHR_display_swapchain` revision 9
 - `VK_KHR_display` revision 21
 - `VK_KHR_mir_surface` revision 4
 - `VK_KHR_sampler_mirror_clamp_to_edge` revision 1
 - `VK_KHR_surface` revision 25
 - `VK_KHR_swapchain` revision 67
 - `VK_KHR_wayland_surface` revision 5
 - `VK_KHR_win32_surface` revision 5
 - `VK_KHR_xcb_surface` revision 6
 - `VK_KHR_xlib_surface` revision 6
 - `VK_EXT_debug_report` revisions 1 and 2
