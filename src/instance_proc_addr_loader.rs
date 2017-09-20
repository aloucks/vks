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

use amd_draw_indirect_count;
use core;
use ext_acquire_xlib_display;
use ext_debug_marker;
use ext_debug_report;
use ext_direct_mode_display;
use ext_discard_rectangles;
use ext_display_control;
use ext_display_surface_counter;
use ext_hdr_metadata;
use google_display_timing;
use khr_android_surface;
use khr_descriptor_update_template;
use khr_display;
use khr_display_swapchain;
use khr_external_fence_capabilities;
use khr_external_fence_fd;
use khr_external_fence_win32;
use khr_external_memory_capabilities;
use khr_external_memory_fd;
use khr_external_memory_win32;
use khr_external_semaphore_capabilities;
use khr_external_semaphore_fd;
use khr_external_semaphore_win32;
use khr_get_memory_requirements2;
use khr_get_physical_device_properties2;
use khr_get_surface_capabilities2;
use khr_maintenance1;
use khr_mir_surface;
use khr_push_descriptor;
use khr_shared_presentable_image;
use khr_surface;
use khr_swapchain;
use khr_wayland_surface;
use khr_win32_surface;
use khr_xcb_surface;
use khr_xlib_surface;
use libc::{c_char, c_int, c_void};
use mir_types;
use mvk_ios_surface;
use mvk_macos_surface;
use nn_vi_surface;
use nv_clip_space_w_scaling;
use nv_external_memory_capabilities;
use nv_external_memory_win32;
use std::fmt;
use std::mem;
use std::ptr;
use wayland_types;
use win32_types;
use xcb_types;
use xlib_types;

#[cfg(feature = "experimental")]
use experimental::*;

macro_rules! gen_instance_proc_addr_loader {
    (
        $( #[$attr:meta] )*
        pub struct InstanceProcAddrLoader {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident: $ty:ident [fn $load:ident],
            )*

            experimental {
                $(
                    $( #[$exp_field_attr:meta] )*
                    pub $exp_field:ident: $exp_ty:ident [fn $exp_load:ident],
                )*
            }
        }
    ) => {
        $( #[$attr] )*
        pub struct InstanceProcAddrLoader {
            pub pfn_vkGetInstanceProcAddr: core::PFN_vkGetInstanceProcAddr,

            /// Core functions, which don't require a dispatchable Vulkan object
            pub core_global: CoreGlobal,

            $(
                $( #[$field_attr] )*
                pub $field: $ty,
            )*

            $(
                #[cfg(feature = "experimental")]
                $( #[$exp_field_attr] )*
                pub $exp_field: $exp_ty,
            )*

            #[allow(dead_code)]
            guard: (),
        }

        impl Copy for InstanceProcAddrLoader { }

        impl Clone for InstanceProcAddrLoader {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl fmt::Debug for InstanceProcAddrLoader {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut debug_struct = f.debug_struct("InstanceProcAddrLoader");

                let pfn_vkGetInstanceProcAddr = self.pfn_vkGetInstanceProcAddr.map(|pfn_vkGetInstanceProcAddr| pfn_vkGetInstanceProcAddr as *mut c_void);
                debug_struct.field("pfn_vkGetInstanceProcAddr", &pfn_vkGetInstanceProcAddr);

                debug_struct.field("core_global", &self.core_global);

                $(
                    debug_struct.field(stringify!($field), &self.$field);
                )*

                $(
                    #[cfg(feature = "experimental")]
                    debug_struct.field(stringify!($exp_field), &self.$exp_field);
                )*

                debug_struct.finish()
            }
        }

        impl Default for InstanceProcAddrLoader {
            fn default() -> Self {
                InstanceProcAddrLoader::new()
            }
        }

        impl InstanceProcAddrLoader {
            pub fn new() -> Self {
                InstanceProcAddrLoader::from_get_instance_proc_addr(None)
            }

            pub fn from_get_instance_proc_addr(pfn_vkGetInstanceProcAddr: core::PFN_vkGetInstanceProcAddr) -> Self {
                InstanceProcAddrLoader {
                    pfn_vkGetInstanceProcAddr: pfn_vkGetInstanceProcAddr,
                    core_global: CoreGlobal::new(),
                    $( $field: $ty::new(), )*
                    $(
                        #[cfg(feature = "experimental")]
                        $exp_field: $exp_ty::new(),
                    )*
                    guard: (),
                }
            }

            /// [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetInstanceProcAddr)
            #[inline]
            pub unsafe fn vkGetInstanceProcAddr(&self, instance: core::VkInstance, pName: *const c_char) -> core::PFN_vkVoidFunction {
                let pfn_vkGetInstanceProcAddr = self.pfn_vkGetInstanceProcAddr.expect("pfn_vkGetInstanceProcAddr is None");
                (pfn_vkGetInstanceProcAddr)(instance, pName)
            }

            pub unsafe fn load_core_global(&mut self) {
                self.core_global.load(self.pfn_vkGetInstanceProcAddr, ptr::null_mut());
            }

            $(
                pub unsafe fn $load(&mut self, instance: core::VkInstance) {
                    self.$field.load(self.pfn_vkGetInstanceProcAddr, instance);
                }
            )*

            $(
                #[cfg(feature = "experimental")]
                pub unsafe fn $exp_load(&mut self, instance: core::VkInstance) {
                    self.$exp_field.load(self.pfn_vkGetInstanceProcAddr, instance);
                }
            )*
        }
    }
}

macro_rules! addr_proc_struct {
    (
        $( #[$attr:meta] )*
        pub struct $name:ident [$ext_field:ident] {
            $(
                $( #[$symbol_attr:meta] )*
                pub fn $fn:ident( $( $arg:ident: $arg_ty:ty ),* ) $( -> $fn_ret:ty )* ; [$symbol:ident: $ty:ty],
            )*
        }
    ) => (
        $( #[$attr] )*
        pub struct $name {
            $(
                $( #[$symbol_attr] )*
                pub $symbol: $ty,
            )*

            #[allow(dead_code)]
            guard: (),
        }

        impl Copy for $name {}

        impl Clone for $name {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut debug_struct = f.debug_struct(stringify!($name));
                $( {
                    let $symbol = self.$symbol.map(|$symbol| $symbol as *mut c_void);
                    debug_struct.field(stringify!($symbol), &$symbol);
                } )*
                debug_struct.finish()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                $name {
                    $(
                        $symbol: None,
                    )*
                    guard: (),
                }
            }

            $(
                #[inline]
                $( #[$symbol_attr] )*
                #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
                pub unsafe fn $fn(&self, $( $arg: $arg_ty ),* ) $( -> $fn_ret )* {
                    let $symbol = self.$symbol.expect(concat!(stringify!($symbol), " is None"));
                    ($symbol)($( $arg ),*)
                }
            )*

            #[allow(unused_variables)]
            pub unsafe fn load(&mut self, pfn_vkGetInstanceProcAddr: core::PFN_vkGetInstanceProcAddr, instance: core::VkInstance) {
                let pfn_vkGetInstanceProcAddr = pfn_vkGetInstanceProcAddr.expect("pfn_vkGetInstanceProcAddr is None");
                $(
                    self.$symbol = (pfn_vkGetInstanceProcAddr)(instance, concat!(stringify!($fn), '\x00').as_ptr() as *const c_char)
                        .map(|$symbol| mem::transmute($symbol));
                )*
            }
        }

        $(
            impl InstanceProcAddrLoader {
                #[inline]
                $( #[$symbol_attr] )*
                #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
                pub unsafe fn $fn(&self, $( $arg: $arg_ty ),* ) $( -> $fn_ret )* {
                    self.$ext_field.$fn($( $arg ),*)
                }
            }
        )*
    )
}

gen_instance_proc_addr_loader!(
    pub struct InstanceProcAddrLoader {
        /// [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)
        pub core: Core [fn load_core],

        /// [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
        pub amd_draw_indirect_count: AMD_draw_indirect_count [fn load_amd_draw_indirect_count],

        /// [`VK_EXT_acquire_xlib_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_acquire_xlib_display)
        pub ext_acquire_xlib_display: EXT_acquire_xlib_display [fn load_ext_acquire_xlib_display],

        /// [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
        pub ext_debug_marker: EXT_debug_marker [fn load_ext_debug_marker],

        /// [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        pub ext_debug_report: EXT_debug_report [fn load_ext_debug_report],

        /// [`VK_EXT_direct_mode_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_direct_mode_display)
        pub ext_direct_mode_display: EXT_direct_mode_display [fn load_ext_direct_mode_display],

        /// [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
        pub ext_discard_rectangles: EXT_discard_rectangles [fn load_ext_discard_rectangles],

        /// [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
        pub ext_display_control: EXT_display_control [fn load_ext_display_control],

        /// [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
        pub ext_display_surface_counter: EXT_display_surface_counter [fn load_ext_display_surface_counter],

        /// [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
        pub ext_hdr_metadata: EXT_hdr_metadata [fn load_ext_hdr_metadata],

        /// [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
        pub google_display_timing: GOOGLE_display_timing [fn load_google_display_timing],

        /// [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)
        pub khr_android_surface: KHR_android_surface [fn load_khr_android_surface],

        /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
        pub khr_descriptor_update_template: KHR_descriptor_update_template [fn load_khr_descriptor_update_template],

        /// [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        pub khr_display: KHR_display [fn load_khr_display],

        /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
        pub khr_display_swapchain: KHR_display_swapchain [fn load_khr_display_swapchain],

        /// [`VK_KHR_external_fence_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_capabilities)
        pub khr_external_fence_capabilities: KHR_external_fence_capabilities [fn load_khr_external_fence_capabilities],

        /// [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)
        pub khr_external_fence_fd: KHR_external_fence_fd [fn load_khr_external_fence_fd],

        /// [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
        pub khr_external_fence_win32: KHR_external_fence_win32 [fn load_khr_external_fence_win32],

        /// [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)
        pub khr_external_memory_capabilities: KHR_external_memory_capabilities [fn load_khr_external_memory_capabilities],

        /// [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
        pub khr_external_memory_fd: KHR_external_memory_fd [fn load_khr_external_memory_fd],

        /// [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
        pub khr_external_memory_win32: KHR_external_memory_win32 [fn load_khr_external_memory_win32],

        /// [`VK_KHR_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_capabilities)
        pub khr_external_semaphore_capabilities: KHR_external_semaphore_capabilities [fn load_khr_external_semaphore_capabilities],

        /// [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
        pub khr_external_semaphore_fd: KHR_external_semaphore_fd [fn load_khr_external_semaphore_fd],

        /// [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
        pub khr_external_semaphore_win32: KHR_external_semaphore_win32 [fn load_khr_external_semaphore_win32],

        /// [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        pub khr_get_memory_requirements2: KHR_get_memory_requirements2 [fn load_khr_get_memory_requirements2],

        /// [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        pub khr_get_physical_device_properties2: KHR_get_physical_device_properties2 [fn load_khr_get_physical_device_properties2],

        /// [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)
        pub khr_get_surface_capabilities2: KHR_get_surface_capabilities2 [fn load_khr_get_surface_capabilities2],

        /// [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        pub khr_maintenance1: KHR_maintenance1 [fn load_khr_maintenance1],

        /// [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)
        pub khr_mir_surface: KHR_mir_surface [fn load_khr_mir_surface],

        /// [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
        pub khr_push_descriptor: KHR_push_descriptor [fn load_khr_push_descriptor],

        /// [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_shared_presentable_image)
        pub khr_shared_presentable_image: KHR_shared_presentable_image [fn load_khr_shared_presentable_image],

        /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        pub khr_surface: KHR_surface [fn load_khr_surface],

        /// [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        pub khr_swapchain: KHR_swapchain [fn load_khr_swapchain],

        /// [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
        pub khr_wayland_surface: KHR_wayland_surface [fn load_khr_wayland_surface],

        /// [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)
        pub khr_win32_surface: KHR_win32_surface [fn load_khr_win32_surface],

        /// [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
        pub khr_xcb_surface: KHR_xcb_surface [fn load_khr_xcb_surface],

        /// [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
        pub khr_xlib_surface: KHR_xlib_surface [fn load_khr_xlib_surface],

        /// [`VK_MVK_ios_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_ios_surface)
        pub mvk_ios_surface: MVK_ios_surface [fn load_mvk_ios_surface],

        /// [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
        pub mvk_macos_surface: MVK_macos_surface [fn load_mvk_macos_surface],

        /// [`VK_NN_vi_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NN_vi_surface)
        pub nn_vi_surface: NN_vi_surface [fn load_nn_vi_surface],

        /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
        pub nv_clip_space_w_scaling: NV_clip_space_w_scaling [fn load_nv_clip_space_w_scaling],

        /// [`VK_NV_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_capabilities)
        pub nv_external_memory_capabilities: NV_external_memory_capabilities [fn load_nv_external_memory_capabilities],

        /// [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
        pub nv_external_memory_win32: NV_external_memory_win32 [fn load_nv_external_memory_win32],

        experimental {
            /// [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
            pub khx_device_group: KHX_device_group [fn load_khx_device_group],

            /// [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)
            pub khx_device_group_creation: KHX_device_group_creation [fn load_khx_device_group_creation],

            /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
            pub nvx_device_generated_commands: NVX_device_generated_commands [fn load_nvx_device_generated_commands],
        }
    }
);

addr_proc_struct!(
    /// Core functions, which don't require a dispatchable Vulkan object
    pub struct CoreGlobal [core_global] {
        /// [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
        pub fn vkCreateInstance(pCreateInfo: *const core::VkInstanceCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pInstance: *mut core::VkInstance) -> core::VkResult; [pfn_vkCreateInstance: core::PFN_vkCreateInstance],

        /// [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
        pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut core::VkExtensionProperties) -> core::VkResult; [pfn_vkEnumerateInstanceExtensionProperties: core::PFN_vkEnumerateInstanceExtensionProperties],

        /// [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
        pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut core::VkLayerProperties) -> core::VkResult; [pfn_vkEnumerateInstanceLayerProperties: core::PFN_vkEnumerateInstanceLayerProperties],
    }
);

addr_proc_struct!(
    /// [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)
    pub struct Core [core] {
        /// [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
        pub fn vkAllocateCommandBuffers(device: core::VkDevice, pAllocateInfo: *const core::VkCommandBufferAllocateInfo, pCommandBuffers: *mut core::VkCommandBuffer) -> core::VkResult; [pfn_vkAllocateCommandBuffers: core::PFN_vkAllocateCommandBuffers],

        /// [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
        pub fn vkAllocateDescriptorSets(device: core::VkDevice, pAllocateInfo: *const core::VkDescriptorSetAllocateInfo, pDescriptorSets: *mut core::VkDescriptorSet) -> core::VkResult; [pfn_vkAllocateDescriptorSets: core::PFN_vkAllocateDescriptorSets],

        /// [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
        pub fn vkAllocateMemory(device: core::VkDevice, pAllocateInfo: *const core::VkMemoryAllocateInfo, pAllocator: *const core::VkAllocationCallbacks, pMemory: *mut core::VkDeviceMemory) -> core::VkResult; [pfn_vkAllocateMemory: core::PFN_vkAllocateMemory],

        /// [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
        pub fn vkBeginCommandBuffer(commandBuffer: core::VkCommandBuffer, pBeginInfo: *const core::VkCommandBufferBeginInfo) -> core::VkResult; [pfn_vkBeginCommandBuffer: core::PFN_vkBeginCommandBuffer],

        /// [`vkBindBufferMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory)
        pub fn vkBindBufferMemory(device: core::VkDevice, buffer: core::VkBuffer, memory: core::VkDeviceMemory, memoryOffset: core::VkDeviceSize) -> core::VkResult; [pfn_vkBindBufferMemory: core::PFN_vkBindBufferMemory],

        /// [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
        pub fn vkBindImageMemory(device: core::VkDevice, image: core::VkImage, memory: core::VkDeviceMemory, memoryOffset: core::VkDeviceSize) -> core::VkResult; [pfn_vkBindImageMemory: core::PFN_vkBindImageMemory],

        /// [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
        pub fn vkCmdBeginQuery(commandBuffer: core::VkCommandBuffer, queryPool: core::VkQueryPool, query: u32, flags: core::VkQueryControlFlags); [pfn_vkCmdBeginQuery: core::PFN_vkCmdBeginQuery],

        /// [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
        pub fn vkCmdBeginRenderPass(commandBuffer: core::VkCommandBuffer, pRenderPassBegin: *const core::VkRenderPassBeginInfo, contents: core::VkSubpassContents); [pfn_vkCmdBeginRenderPass: core::PFN_vkCmdBeginRenderPass],

        /// [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
        pub fn vkCmdBindDescriptorSets(commandBuffer: core::VkCommandBuffer, pipelineBindPoint: core::VkPipelineBindPoint, layout: core::VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const core::VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32); [pfn_vkCmdBindDescriptorSets: core::PFN_vkCmdBindDescriptorSets],

        /// [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
        pub fn vkCmdBindIndexBuffer(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, indexType: core::VkIndexType); [pfn_vkCmdBindIndexBuffer: core::PFN_vkCmdBindIndexBuffer],

        /// [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
        pub fn vkCmdBindPipeline(commandBuffer: core::VkCommandBuffer, pipelineBindPoint: core::VkPipelineBindPoint, pipeline: core::VkPipeline); [pfn_vkCmdBindPipeline: core::PFN_vkCmdBindPipeline],

        /// [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
        pub fn vkCmdBindVertexBuffers(commandBuffer: core::VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const core::VkBuffer, pOffsets: *const core::VkDeviceSize); [pfn_vkCmdBindVertexBuffers: core::PFN_vkCmdBindVertexBuffers],

        /// [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
        pub fn vkCmdBlitImage(commandBuffer: core::VkCommandBuffer, srcImage: core::VkImage, srcImageLayout: core::VkImageLayout, dstImage: core::VkImage, dstImageLayout: core::VkImageLayout, regionCount: u32, pRegions: *const core::VkImageBlit, filter: core::VkFilter); [pfn_vkCmdBlitImage: core::PFN_vkCmdBlitImage],

        /// [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
        pub fn vkCmdClearAttachments(commandBuffer: core::VkCommandBuffer, attachmentCount: u32, pAttachments: *const core::VkClearAttachment, rectCount: u32, pRects: *const core::VkClearRect); [pfn_vkCmdClearAttachments: core::PFN_vkCmdClearAttachments],

        /// [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
        pub fn vkCmdClearColorImage(commandBuffer: core::VkCommandBuffer, image: core::VkImage, imageLayout: core::VkImageLayout, pColor: *const core::VkClearColorValue, rangeCount: u32, pRanges: *const core::VkImageSubresourceRange); [pfn_vkCmdClearColorImage: core::PFN_vkCmdClearColorImage],

        /// [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
        pub fn vkCmdClearDepthStencilImage(commandBuffer: core::VkCommandBuffer, image: core::VkImage, imageLayout: core::VkImageLayout, pDepthStencil: *const core::VkClearDepthStencilValue, rangeCount: u32, pRanges: *const core::VkImageSubresourceRange); [pfn_vkCmdClearDepthStencilImage: core::PFN_vkCmdClearDepthStencilImage],

        /// [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
        pub fn vkCmdCopyBuffer(commandBuffer: core::VkCommandBuffer, srcBuffer: core::VkBuffer, dstBuffer: core::VkBuffer, regionCount: u32, pRegions: *const core::VkBufferCopy); [pfn_vkCmdCopyBuffer: core::PFN_vkCmdCopyBuffer],

        /// [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
        pub fn vkCmdCopyBufferToImage(commandBuffer: core::VkCommandBuffer, srcBuffer: core::VkBuffer, dstImage: core::VkImage, dstImageLayout: core::VkImageLayout, regionCount: u32, pRegions: *const core::VkBufferImageCopy); [pfn_vkCmdCopyBufferToImage: core::PFN_vkCmdCopyBufferToImage],

        /// [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
        pub fn vkCmdCopyImage(commandBuffer: core::VkCommandBuffer, srcImage: core::VkImage, srcImageLayout: core::VkImageLayout, dstImage: core::VkImage, dstImageLayout: core::VkImageLayout, regionCount: u32, pRegions: *const core::VkImageCopy); [pfn_vkCmdCopyImage: core::PFN_vkCmdCopyImage],

        /// [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
        pub fn vkCmdCopyImageToBuffer(commandBuffer: core::VkCommandBuffer, srcImage: core::VkImage, srcImageLayout: core::VkImageLayout, dstBuffer: core::VkBuffer, regionCount: u32, pRegions: *const core::VkBufferImageCopy); [pfn_vkCmdCopyImageToBuffer: core::PFN_vkCmdCopyImageToBuffer],

        /// [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
        pub fn vkCmdCopyQueryPoolResults(commandBuffer: core::VkCommandBuffer, queryPool: core::VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: core::VkBuffer, dstOffset: core::VkDeviceSize, stride: core::VkDeviceSize, flags: core::VkQueryResultFlags); [pfn_vkCmdCopyQueryPoolResults: core::PFN_vkCmdCopyQueryPoolResults],

        /// [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
        pub fn vkCmdDispatch(commandBuffer: core::VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32); [pfn_vkCmdDispatch: core::PFN_vkCmdDispatch],

        /// [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
        pub fn vkCmdDispatchIndirect(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize); [pfn_vkCmdDispatchIndirect: core::PFN_vkCmdDispatchIndirect],

        /// [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
        pub fn vkCmdDraw(commandBuffer: core::VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32); [pfn_vkCmdDraw: core::PFN_vkCmdDraw],

        /// [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
        pub fn vkCmdDrawIndexed(commandBuffer: core::VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32); [pfn_vkCmdDrawIndexed: core::PFN_vkCmdDrawIndexed],

        /// [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
        pub fn vkCmdDrawIndexedIndirect(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, drawCount: u32, stride: u32); [pfn_vkCmdDrawIndexedIndirect: core::PFN_vkCmdDrawIndexedIndirect],

        /// [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
        pub fn vkCmdDrawIndirect(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, drawCount: u32, stride: u32); [pfn_vkCmdDrawIndirect: core::PFN_vkCmdDrawIndirect],

        /// [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
        pub fn vkCmdEndQuery(commandBuffer: core::VkCommandBuffer, queryPool: core::VkQueryPool, query: u32); [pfn_vkCmdEndQuery: core::PFN_vkCmdEndQuery],

        /// [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
        pub fn vkCmdEndRenderPass(commandBuffer: core::VkCommandBuffer); [pfn_vkCmdEndRenderPass: core::PFN_vkCmdEndRenderPass],

        /// [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
        pub fn vkCmdExecuteCommands(commandBuffer: core::VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const core::VkCommandBuffer); [pfn_vkCmdExecuteCommands: core::PFN_vkCmdExecuteCommands],

        /// [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
        pub fn vkCmdFillBuffer(commandBuffer: core::VkCommandBuffer, dstBuffer: core::VkBuffer, dstOffset: core::VkDeviceSize, size: core::VkDeviceSize, data: u32); [pfn_vkCmdFillBuffer: core::PFN_vkCmdFillBuffer],

        /// [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
        pub fn vkCmdNextSubpass(commandBuffer: core::VkCommandBuffer, contents: core::VkSubpassContents); [pfn_vkCmdNextSubpass: core::PFN_vkCmdNextSubpass],

        /// [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
        pub fn vkCmdPipelineBarrier(commandBuffer: core::VkCommandBuffer, srcStageMask: core::VkPipelineStageFlags, dstStageMask: core::VkPipelineStageFlags, dependencyFlags: core::VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const core::VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const core::VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const core::VkImageMemoryBarrier); [pfn_vkCmdPipelineBarrier: core::PFN_vkCmdPipelineBarrier],

        /// [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
        pub fn vkCmdPushConstants(commandBuffer: core::VkCommandBuffer, layout: core::VkPipelineLayout, stageFlags: core::VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void); [pfn_vkCmdPushConstants: core::PFN_vkCmdPushConstants],

        /// [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
        pub fn vkCmdResetEvent(commandBuffer: core::VkCommandBuffer, event: core::VkEvent, stageMask: core::VkPipelineStageFlags); [pfn_vkCmdResetEvent: core::PFN_vkCmdResetEvent],

        /// [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
        pub fn vkCmdResetQueryPool(commandBuffer: core::VkCommandBuffer, queryPool: core::VkQueryPool, firstQuery: u32, queryCount: u32); [pfn_vkCmdResetQueryPool: core::PFN_vkCmdResetQueryPool],

        /// [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
        pub fn vkCmdResolveImage(commandBuffer: core::VkCommandBuffer, srcImage: core::VkImage, srcImageLayout: core::VkImageLayout, dstImage: core::VkImage, dstImageLayout: core::VkImageLayout, regionCount: u32, pRegions: *const core::VkImageResolve); [pfn_vkCmdResolveImage: core::PFN_vkCmdResolveImage],

        /// [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
        pub fn vkCmdSetBlendConstants(commandBuffer: core::VkCommandBuffer, blendConstants: *const f32); [pfn_vkCmdSetBlendConstants: core::PFN_vkCmdSetBlendConstants],

        /// [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
        pub fn vkCmdSetDepthBias(commandBuffer: core::VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32); [pfn_vkCmdSetDepthBias: core::PFN_vkCmdSetDepthBias],

        /// [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
        pub fn vkCmdSetDepthBounds(commandBuffer: core::VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32); [pfn_vkCmdSetDepthBounds: core::PFN_vkCmdSetDepthBounds],

        /// [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
        pub fn vkCmdSetEvent(commandBuffer: core::VkCommandBuffer, event: core::VkEvent, stageMask: core::VkPipelineStageFlags); [pfn_vkCmdSetEvent: core::PFN_vkCmdSetEvent],

        /// [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
        pub fn vkCmdSetLineWidth(commandBuffer: core::VkCommandBuffer, lineWidth: f32); [pfn_vkCmdSetLineWidth: core::PFN_vkCmdSetLineWidth],

        /// [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
        pub fn vkCmdSetScissor(commandBuffer: core::VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const core::VkRect2D); [pfn_vkCmdSetScissor: core::PFN_vkCmdSetScissor],

        /// [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
        pub fn vkCmdSetStencilCompareMask(commandBuffer: core::VkCommandBuffer, faceMask: core::VkStencilFaceFlags, compareMask: u32); [pfn_vkCmdSetStencilCompareMask: core::PFN_vkCmdSetStencilCompareMask],

        /// [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
        pub fn vkCmdSetStencilReference(commandBuffer: core::VkCommandBuffer, faceMask: core::VkStencilFaceFlags, reference: u32); [pfn_vkCmdSetStencilReference: core::PFN_vkCmdSetStencilReference],

        /// [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
        pub fn vkCmdSetStencilWriteMask(commandBuffer: core::VkCommandBuffer, faceMask: core::VkStencilFaceFlags, writeMask: u32); [pfn_vkCmdSetStencilWriteMask: core::PFN_vkCmdSetStencilWriteMask],

        /// [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
        pub fn vkCmdSetViewport(commandBuffer: core::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const core::VkViewport); [pfn_vkCmdSetViewport: core::PFN_vkCmdSetViewport],

        /// [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
        pub fn vkCmdUpdateBuffer(commandBuffer: core::VkCommandBuffer, dstBuffer: core::VkBuffer, dstOffset: core::VkDeviceSize, dataSize: core::VkDeviceSize, pData: *const c_void); [pfn_vkCmdUpdateBuffer: core::PFN_vkCmdUpdateBuffer],

        /// [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
        pub fn vkCmdWaitEvents(commandBuffer: core::VkCommandBuffer, eventCount: u32, pEvents: *const core::VkEvent, srcStageMask: core::VkPipelineStageFlags, dstStageMask: core::VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const core::VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const core::VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const core::VkImageMemoryBarrier); [pfn_vkCmdWaitEvents: core::PFN_vkCmdWaitEvents],

        /// [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
        pub fn vkCmdWriteTimestamp(commandBuffer: core::VkCommandBuffer, pipelineStage: core::VkPipelineStageFlagBits, queryPool: core::VkQueryPool, query: u32); [pfn_vkCmdWriteTimestamp: core::PFN_vkCmdWriteTimestamp],

        /// [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
        pub fn vkCreateBuffer(device: core::VkDevice, pCreateInfo: *const core::VkBufferCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pBuffer: *mut core::VkBuffer) -> core::VkResult; [pfn_vkCreateBuffer: core::PFN_vkCreateBuffer],

        /// [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
        pub fn vkCreateBufferView(device: core::VkDevice, pCreateInfo: *const core::VkBufferViewCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pView: *mut core::VkBufferView) -> core::VkResult; [pfn_vkCreateBufferView: core::PFN_vkCreateBufferView],

        /// [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
        pub fn vkCreateCommandPool(device: core::VkDevice, pCreateInfo: *const core::VkCommandPoolCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pCommandPool: *mut core::VkCommandPool) -> core::VkResult; [pfn_vkCreateCommandPool: core::PFN_vkCreateCommandPool],

        /// [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
        pub fn vkCreateComputePipelines(device: core::VkDevice, pipelineCache: core::VkPipelineCache, createInfoCount: u32, pCreateInfos: *const core::VkComputePipelineCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pPipelines: *mut core::VkPipeline) -> core::VkResult; [pfn_vkCreateComputePipelines: core::PFN_vkCreateComputePipelines],

        /// [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
        pub fn vkCreateDescriptorPool(device: core::VkDevice, pCreateInfo: *const core::VkDescriptorPoolCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pDescriptorPool: *mut core::VkDescriptorPool) -> core::VkResult; [pfn_vkCreateDescriptorPool: core::PFN_vkCreateDescriptorPool],

        /// [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
        pub fn vkCreateDescriptorSetLayout(device: core::VkDevice, pCreateInfo: *const core::VkDescriptorSetLayoutCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pSetLayout: *mut core::VkDescriptorSetLayout) -> core::VkResult; [pfn_vkCreateDescriptorSetLayout: core::PFN_vkCreateDescriptorSetLayout],

        /// [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
        pub fn vkCreateDevice(physicalDevice: core::VkPhysicalDevice, pCreateInfo: *const core::VkDeviceCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pDevice: *mut core::VkDevice) -> core::VkResult; [pfn_vkCreateDevice: core::PFN_vkCreateDevice],

        /// [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
        pub fn vkCreateEvent(device: core::VkDevice, pCreateInfo: *const core::VkEventCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pEvent: *mut core::VkEvent) -> core::VkResult; [pfn_vkCreateEvent: core::PFN_vkCreateEvent],

        /// [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
        pub fn vkCreateFence(device: core::VkDevice, pCreateInfo: *const core::VkFenceCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult; [pfn_vkCreateFence: core::PFN_vkCreateFence],

        /// [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
        pub fn vkCreateFramebuffer(device: core::VkDevice, pCreateInfo: *const core::VkFramebufferCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pFramebuffer: *mut core::VkFramebuffer) -> core::VkResult; [pfn_vkCreateFramebuffer: core::PFN_vkCreateFramebuffer],

        /// [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
        pub fn vkCreateGraphicsPipelines(device: core::VkDevice, pipelineCache: core::VkPipelineCache, createInfoCount: u32, pCreateInfos: *const core::VkGraphicsPipelineCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pPipelines: *mut core::VkPipeline) -> core::VkResult; [pfn_vkCreateGraphicsPipelines: core::PFN_vkCreateGraphicsPipelines],

        /// [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
        pub fn vkCreateImage(device: core::VkDevice, pCreateInfo: *const core::VkImageCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pImage: *mut core::VkImage) -> core::VkResult; [pfn_vkCreateImage: core::PFN_vkCreateImage],

        /// [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
        pub fn vkCreateImageView(device: core::VkDevice, pCreateInfo: *const core::VkImageViewCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pView: *mut core::VkImageView) -> core::VkResult; [pfn_vkCreateImageView: core::PFN_vkCreateImageView],

        /// [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
        pub fn vkCreatePipelineCache(device: core::VkDevice, pCreateInfo: *const core::VkPipelineCacheCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pPipelineCache: *mut core::VkPipelineCache) -> core::VkResult; [pfn_vkCreatePipelineCache: core::PFN_vkCreatePipelineCache],

        /// [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
        pub fn vkCreatePipelineLayout(device: core::VkDevice, pCreateInfo: *const core::VkPipelineLayoutCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pPipelineLayout: *mut core::VkPipelineLayout) -> core::VkResult; [pfn_vkCreatePipelineLayout: core::PFN_vkCreatePipelineLayout],

        /// [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
        pub fn vkCreateQueryPool(device: core::VkDevice, pCreateInfo: *const core::VkQueryPoolCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pQueryPool: *mut core::VkQueryPool) -> core::VkResult; [pfn_vkCreateQueryPool: core::PFN_vkCreateQueryPool],

        /// [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
        pub fn vkCreateRenderPass(device: core::VkDevice, pCreateInfo: *const core::VkRenderPassCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pRenderPass: *mut core::VkRenderPass) -> core::VkResult; [pfn_vkCreateRenderPass: core::PFN_vkCreateRenderPass],

        /// [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
        pub fn vkCreateSampler(device: core::VkDevice, pCreateInfo: *const core::VkSamplerCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pSampler: *mut core::VkSampler) -> core::VkResult; [pfn_vkCreateSampler: core::PFN_vkCreateSampler],

        /// [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
        pub fn vkCreateSemaphore(device: core::VkDevice, pCreateInfo: *const core::VkSemaphoreCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pSemaphore: *mut core::VkSemaphore) -> core::VkResult; [pfn_vkCreateSemaphore: core::PFN_vkCreateSemaphore],

        /// [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
        pub fn vkCreateShaderModule(device: core::VkDevice, pCreateInfo: *const core::VkShaderModuleCreateInfo, pAllocator: *const core::VkAllocationCallbacks, pShaderModule: *mut core::VkShaderModule) -> core::VkResult; [pfn_vkCreateShaderModule: core::PFN_vkCreateShaderModule],

        /// [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBuffer)
        pub fn vkDestroyBuffer(device: core::VkDevice, buffer: core::VkBuffer, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyBuffer: core::PFN_vkDestroyBuffer],

        /// [`vkDestroyBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBufferView)
        pub fn vkDestroyBufferView(device: core::VkDevice, bufferView: core::VkBufferView, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyBufferView: core::PFN_vkDestroyBufferView],

        /// [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyCommandPool)
        pub fn vkDestroyCommandPool(device: core::VkDevice, commandPool: core::VkCommandPool, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyCommandPool: core::PFN_vkDestroyCommandPool],

        /// [`vkDestroyDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorPool)
        pub fn vkDestroyDescriptorPool(device: core::VkDevice, descriptorPool: core::VkDescriptorPool, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyDescriptorPool: core::PFN_vkDestroyDescriptorPool],

        /// [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorSetLayout)
        pub fn vkDestroyDescriptorSetLayout(device: core::VkDevice, descriptorSetLayout: core::VkDescriptorSetLayout, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyDescriptorSetLayout: core::PFN_vkDestroyDescriptorSetLayout],

        /// [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDevice)
        pub fn vkDestroyDevice(device: core::VkDevice, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyDevice: core::PFN_vkDestroyDevice],

        /// [`vkDestroyEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyEvent)
        pub fn vkDestroyEvent(device: core::VkDevice, event: core::VkEvent, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyEvent: core::PFN_vkDestroyEvent],

        /// [`vkDestroyFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFence)
        pub fn vkDestroyFence(device: core::VkDevice, fence: core::VkFence, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyFence: core::PFN_vkDestroyFence],

        /// [`vkDestroyFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFramebuffer)
        pub fn vkDestroyFramebuffer(device: core::VkDevice, framebuffer: core::VkFramebuffer, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyFramebuffer: core::PFN_vkDestroyFramebuffer],

        /// [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImage)
        pub fn vkDestroyImage(device: core::VkDevice, image: core::VkImage, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyImage: core::PFN_vkDestroyImage],

        /// [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImageView)
        pub fn vkDestroyImageView(device: core::VkDevice, imageView: core::VkImageView, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyImageView: core::PFN_vkDestroyImageView],

        /// [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyInstance)
        pub fn vkDestroyInstance(instance: core::VkInstance, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyInstance: core::PFN_vkDestroyInstance],

        /// [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipeline)
        pub fn vkDestroyPipeline(device: core::VkDevice, pipeline: core::VkPipeline, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyPipeline: core::PFN_vkDestroyPipeline],

        /// [`vkDestroyPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineCache)
        pub fn vkDestroyPipelineCache(device: core::VkDevice, pipelineCache: core::VkPipelineCache, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyPipelineCache: core::PFN_vkDestroyPipelineCache],

        /// [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineLayout)
        pub fn vkDestroyPipelineLayout(device: core::VkDevice, pipelineLayout: core::VkPipelineLayout, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyPipelineLayout: core::PFN_vkDestroyPipelineLayout],

        /// [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyQueryPool)
        pub fn vkDestroyQueryPool(device: core::VkDevice, queryPool: core::VkQueryPool, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyQueryPool: core::PFN_vkDestroyQueryPool],

        /// [`vkDestroyRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyRenderPass)
        pub fn vkDestroyRenderPass(device: core::VkDevice, renderPass: core::VkRenderPass, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyRenderPass: core::PFN_vkDestroyRenderPass],

        /// [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySampler)
        pub fn vkDestroySampler(device: core::VkDevice, sampler: core::VkSampler, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroySampler: core::PFN_vkDestroySampler],

        /// [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySemaphore)
        pub fn vkDestroySemaphore(device: core::VkDevice, semaphore: core::VkSemaphore, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroySemaphore: core::PFN_vkDestroySemaphore],

        /// [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyShaderModule)
        pub fn vkDestroyShaderModule(device: core::VkDevice, shaderModule: core::VkShaderModule, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyShaderModule: core::PFN_vkDestroyShaderModule],

        /// [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
        pub fn vkDeviceWaitIdle(device: core::VkDevice) -> core::VkResult; [pfn_vkDeviceWaitIdle: core::PFN_vkDeviceWaitIdle],

        /// [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
        pub fn vkEndCommandBuffer(commandBuffer: core::VkCommandBuffer) -> core::VkResult; [pfn_vkEndCommandBuffer: core::PFN_vkEndCommandBuffer],

        /// [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
        pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: core::VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut core::VkExtensionProperties) -> core::VkResult; [pfn_vkEnumerateDeviceExtensionProperties: core::PFN_vkEnumerateDeviceExtensionProperties],

        /// [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
        pub fn vkEnumerateDeviceLayerProperties(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut core::VkLayerProperties) -> core::VkResult; [pfn_vkEnumerateDeviceLayerProperties: core::PFN_vkEnumerateDeviceLayerProperties],

        /// [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
        pub fn vkEnumeratePhysicalDevices(instance: core::VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut core::VkPhysicalDevice) -> core::VkResult; [pfn_vkEnumeratePhysicalDevices: core::PFN_vkEnumeratePhysicalDevices],

        /// [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
        pub fn vkFlushMappedMemoryRanges(device: core::VkDevice, memoryRangeCount: u32, pMemoryRanges: *const core::VkMappedMemoryRange) -> core::VkResult; [pfn_vkFlushMappedMemoryRanges: core::PFN_vkFlushMappedMemoryRanges],

        /// [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers)
        pub fn vkFreeCommandBuffers(device: core::VkDevice, commandPool: core::VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const core::VkCommandBuffer); [pfn_vkFreeCommandBuffers: core::PFN_vkFreeCommandBuffers],

        /// [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
        pub fn vkFreeDescriptorSets(device: core::VkDevice, descriptorPool: core::VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const core::VkDescriptorSet) -> core::VkResult; [pfn_vkFreeDescriptorSets: core::PFN_vkFreeDescriptorSets],

        /// [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeMemory)
        pub fn vkFreeMemory(device: core::VkDevice, memory: core::VkDeviceMemory, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkFreeMemory: core::PFN_vkFreeMemory],

        /// [`vkGetBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements)
        pub fn vkGetBufferMemoryRequirements(device: core::VkDevice, buffer: core::VkBuffer, pMemoryRequirements: *mut core::VkMemoryRequirements); [pfn_vkGetBufferMemoryRequirements: core::PFN_vkGetBufferMemoryRequirements],

        /// [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
        pub fn vkGetDeviceMemoryCommitment(device: core::VkDevice, memory: core::VkDeviceMemory, pCommittedMemoryInBytes: *mut core::VkDeviceSize); [pfn_vkGetDeviceMemoryCommitment: core::PFN_vkGetDeviceMemoryCommitment],

        /// [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
        pub fn vkGetDeviceProcAddr(device: core::VkDevice, pName: *const c_char) -> core::PFN_vkVoidFunction; [pfn_vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr],

        /// [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
        pub fn vkGetDeviceQueue(device: core::VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut core::VkQueue); [pfn_vkGetDeviceQueue: core::PFN_vkGetDeviceQueue],

        /// [`vkGetEventStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetEventStatus)
        pub fn vkGetEventStatus(device: core::VkDevice, event: core::VkEvent) -> core::VkResult; [pfn_vkGetEventStatus: core::PFN_vkGetEventStatus],

        /// [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
        pub fn vkGetFenceStatus(device: core::VkDevice, fence: core::VkFence) -> core::VkResult; [pfn_vkGetFenceStatus: core::PFN_vkGetFenceStatus],

        /// [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
        pub fn vkGetImageMemoryRequirements(device: core::VkDevice, image: core::VkImage, pMemoryRequirements: *mut core::VkMemoryRequirements); [pfn_vkGetImageMemoryRequirements: core::PFN_vkGetImageMemoryRequirements],

        /// [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
        pub fn vkGetImageSparseMemoryRequirements(device: core::VkDevice, image: core::VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut core::VkSparseImageMemoryRequirements); [pfn_vkGetImageSparseMemoryRequirements: core::PFN_vkGetImageSparseMemoryRequirements],

        /// [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
        pub fn vkGetImageSubresourceLayout(device: core::VkDevice, image: core::VkImage, pSubresource: *const core::VkImageSubresource, pLayout: *mut core::VkSubresourceLayout); [pfn_vkGetImageSubresourceLayout: core::PFN_vkGetImageSubresourceLayout],

        /// [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
        pub fn vkGetPhysicalDeviceFeatures(physicalDevice: core::VkPhysicalDevice, pFeatures: *mut core::VkPhysicalDeviceFeatures); [pfn_vkGetPhysicalDeviceFeatures: core::PFN_vkGetPhysicalDeviceFeatures],

        /// [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
        pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: core::VkPhysicalDevice, format: core::VkFormat, pFormatProperties: *mut core::VkFormatProperties); [pfn_vkGetPhysicalDeviceFormatProperties: core::PFN_vkGetPhysicalDeviceFormatProperties],

        /// [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
        pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: core::VkPhysicalDevice, format: core::VkFormat, type_: core::VkImageType, tiling: core::VkImageTiling, usage: core::VkImageUsageFlags, flags: core::VkImageCreateFlags, pImageFormatProperties: *mut core::VkImageFormatProperties) -> core::VkResult; [pfn_vkGetPhysicalDeviceImageFormatProperties: core::PFN_vkGetPhysicalDeviceImageFormatProperties],

        /// [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
        pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: core::VkPhysicalDevice, pMemoryProperties: *mut core::VkPhysicalDeviceMemoryProperties); [pfn_vkGetPhysicalDeviceMemoryProperties: core::PFN_vkGetPhysicalDeviceMemoryProperties],

        /// [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
        pub fn vkGetPhysicalDeviceProperties(physicalDevice: core::VkPhysicalDevice, pProperties: *mut core::VkPhysicalDeviceProperties); [pfn_vkGetPhysicalDeviceProperties: core::PFN_vkGetPhysicalDeviceProperties],

        /// [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
        pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: core::VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut core::VkQueueFamilyProperties); [pfn_vkGetPhysicalDeviceQueueFamilyProperties: core::PFN_vkGetPhysicalDeviceQueueFamilyProperties],

        /// [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
        pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: core::VkPhysicalDevice, format: core::VkFormat, type_: core::VkImageType, samples: core::VkSampleCountFlagBits, usage: core::VkImageUsageFlags, tiling: core::VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut core::VkSparseImageFormatProperties); [pfn_vkGetPhysicalDeviceSparseImageFormatProperties: core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties],

        /// [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
        pub fn vkGetPipelineCacheData(device: core::VkDevice, pipelineCache: core::VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> core::VkResult; [pfn_vkGetPipelineCacheData: core::PFN_vkGetPipelineCacheData],

        /// [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
        pub fn vkGetQueryPoolResults(device: core::VkDevice, queryPool: core::VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: core::VkDeviceSize, flags: core::VkQueryResultFlags) -> core::VkResult; [pfn_vkGetQueryPoolResults: core::PFN_vkGetQueryPoolResults],

        /// [`vkGetRenderAreaGranularity`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRenderAreaGranularity)
        pub fn vkGetRenderAreaGranularity(device: core::VkDevice, renderPass: core::VkRenderPass, pGranularity: *mut core::VkExtent2D); [pfn_vkGetRenderAreaGranularity: core::PFN_vkGetRenderAreaGranularity],

        /// [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
        pub fn vkInvalidateMappedMemoryRanges(device: core::VkDevice, memoryRangeCount: u32, pMemoryRanges: *const core::VkMappedMemoryRange) -> core::VkResult; [pfn_vkInvalidateMappedMemoryRanges: core::PFN_vkInvalidateMappedMemoryRanges],

        /// [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
        pub fn vkMapMemory(device: core::VkDevice, memory: core::VkDeviceMemory, offset: core::VkDeviceSize, size: core::VkDeviceSize, flags: core::VkMemoryMapFlags, ppData: *mut *mut c_void) -> core::VkResult; [pfn_vkMapMemory: core::PFN_vkMapMemory],

        /// [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
        pub fn vkMergePipelineCaches(device: core::VkDevice, dstCache: core::VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const core::VkPipelineCache) -> core::VkResult; [pfn_vkMergePipelineCaches: core::PFN_vkMergePipelineCaches],

        /// [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
        pub fn vkQueueBindSparse(queue: core::VkQueue, bindInfoCount: u32, pBindInfo: *const core::VkBindSparseInfo, fence: core::VkFence) -> core::VkResult; [pfn_vkQueueBindSparse: core::PFN_vkQueueBindSparse],

        /// [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
        pub fn vkQueueSubmit(queue: core::VkQueue, submitCount: u32, pSubmits: *const core::VkSubmitInfo, fence: core::VkFence) -> core::VkResult; [pfn_vkQueueSubmit: core::PFN_vkQueueSubmit],

        /// [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
        pub fn vkQueueWaitIdle(queue: core::VkQueue) -> core::VkResult; [pfn_vkQueueWaitIdle: core::PFN_vkQueueWaitIdle],

        /// [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
        pub fn vkResetCommandBuffer(commandBuffer: core::VkCommandBuffer, flags: core::VkCommandBufferResetFlags) -> core::VkResult; [pfn_vkResetCommandBuffer: core::PFN_vkResetCommandBuffer],

        /// [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
        pub fn vkResetCommandPool(device: core::VkDevice, commandPool: core::VkCommandPool, flags: core::VkCommandPoolResetFlags) -> core::VkResult; [pfn_vkResetCommandPool: core::PFN_vkResetCommandPool],

        /// [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
        pub fn vkResetDescriptorPool(device: core::VkDevice, descriptorPool: core::VkDescriptorPool, flags: core::VkDescriptorPoolResetFlags) -> core::VkResult; [pfn_vkResetDescriptorPool: core::PFN_vkResetDescriptorPool],

        /// [`vkResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetEvent)
        pub fn vkResetEvent(device: core::VkDevice, event: core::VkEvent) -> core::VkResult; [pfn_vkResetEvent: core::PFN_vkResetEvent],

        /// [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
        pub fn vkResetFences(device: core::VkDevice, fenceCount: u32, pFences: *const core::VkFence) -> core::VkResult; [pfn_vkResetFences: core::PFN_vkResetFences],

        /// [`vkSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetEvent)
        pub fn vkSetEvent(device: core::VkDevice, event: core::VkEvent) -> core::VkResult; [pfn_vkSetEvent: core::PFN_vkSetEvent],

        /// [`vkUnmapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnmapMemory)
        pub fn vkUnmapMemory(device: core::VkDevice, memory: core::VkDeviceMemory); [pfn_vkUnmapMemory: core::PFN_vkUnmapMemory],

        /// [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
        pub fn vkUpdateDescriptorSets(device: core::VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const core::VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const core::VkCopyDescriptorSet); [pfn_vkUpdateDescriptorSets: core::PFN_vkUpdateDescriptorSets],

        /// [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
        pub fn vkWaitForFences(device: core::VkDevice, fenceCount: u32, pFences: *const core::VkFence, waitAll: core::VkBool32, timeout: u64) -> core::VkResult; [pfn_vkWaitForFences: core::PFN_vkWaitForFences],
    }
);

addr_proc_struct!(
    /// [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
    pub struct AMD_draw_indirect_count [amd_draw_indirect_count] {
        /// [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
        pub fn vkCmdDrawIndexedIndirectCountAMD(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, countBuffer: core::VkBuffer, countBufferOffset: core::VkDeviceSize, maxDrawCount: u32, stride: u32); [pfn_vkCmdDrawIndexedIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndexedIndirectCountAMD],

        /// [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
        pub fn vkCmdDrawIndirectCountAMD(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, countBuffer: core::VkBuffer, countBufferOffset: core::VkDeviceSize, maxDrawCount: u32, stride: u32); [pfn_vkCmdDrawIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndirectCountAMD],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_acquire_xlib_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_acquire_xlib_display)
    pub struct EXT_acquire_xlib_display [ext_acquire_xlib_display] {
        /// [`vkAcquireXlibDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireXlibDisplayEXT)
        pub fn vkAcquireXlibDisplayEXT(physicalDevice: core::VkPhysicalDevice, dpy: *mut xlib_types::Display, display: khr_display::VkDisplayKHR) -> core::VkResult; [pfn_vkAcquireXlibDisplayEXT: ext_acquire_xlib_display::PFN_vkAcquireXlibDisplayEXT],

        /// [`vkGetRandROutputDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRandROutputDisplayEXT)
        pub fn vkGetRandROutputDisplayEXT(physicalDevice: core::VkPhysicalDevice, dpy: *mut xlib_types::Display, rrOutput: xlib_types::RROutput, pDisplay: *mut khr_display::VkDisplayKHR) -> core::VkResult; [pfn_vkGetRandROutputDisplayEXT: ext_acquire_xlib_display::PFN_vkGetRandROutputDisplayEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub struct EXT_debug_marker [ext_debug_marker] {
        /// [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
        pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *const ext_debug_marker::VkDebugMarkerMarkerInfoEXT); [pfn_vkCmdDebugMarkerBeginEXT: ext_debug_marker::PFN_vkCmdDebugMarkerBeginEXT],

        /// [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
        pub fn vkCmdDebugMarkerEndEXT(commandBuffer: core::VkCommandBuffer); [pfn_vkCmdDebugMarkerEndEXT: ext_debug_marker::PFN_vkCmdDebugMarkerEndEXT],

        /// [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
        pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *const ext_debug_marker::VkDebugMarkerMarkerInfoEXT); [pfn_vkCmdDebugMarkerInsertEXT: ext_debug_marker::PFN_vkCmdDebugMarkerInsertEXT],

        /// [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
        pub fn vkDebugMarkerSetObjectNameEXT(device: core::VkDevice, pNameInfo: *const ext_debug_marker::VkDebugMarkerObjectNameInfoEXT) -> core::VkResult; [pfn_vkDebugMarkerSetObjectNameEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectNameEXT],

        /// [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
        pub fn vkDebugMarkerSetObjectTagEXT(device: core::VkDevice, pTagInfo: *const ext_debug_marker::VkDebugMarkerObjectTagInfoEXT) -> core::VkResult; [pfn_vkDebugMarkerSetObjectTagEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectTagEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub struct EXT_debug_report [ext_debug_report] {
        /// [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
        pub fn vkCreateDebugReportCallbackEXT(instance: core::VkInstance, pCreateInfo: *const ext_debug_report::VkDebugReportCallbackCreateInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pCallback: *mut ext_debug_report::VkDebugReportCallbackEXT) -> core::VkResult; [pfn_vkCreateDebugReportCallbackEXT: ext_debug_report::PFN_vkCreateDebugReportCallbackEXT],

        /// [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
        pub fn vkDebugReportMessageEXT(instance: core::VkInstance, flags: ext_debug_report::VkDebugReportFlagsEXT, objectType: ext_debug_report::VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char); [pfn_vkDebugReportMessageEXT: ext_debug_report::PFN_vkDebugReportMessageEXT],

        /// [`vkDestroyDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDebugReportCallbackEXT)
        pub fn vkDestroyDebugReportCallbackEXT(instance: core::VkInstance, callback: ext_debug_report::VkDebugReportCallbackEXT, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyDebugReportCallbackEXT: ext_debug_report::PFN_vkDestroyDebugReportCallbackEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_direct_mode_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_direct_mode_display)
    pub struct EXT_direct_mode_display [ext_direct_mode_display] {
        /// [`vkReleaseDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkReleaseDisplayEXT)
        pub fn vkReleaseDisplayEXT(physicalDevice: core::VkPhysicalDevice, display: khr_display::VkDisplayKHR) -> core::VkResult; [pfn_vkReleaseDisplayEXT: ext_direct_mode_display::PFN_vkReleaseDisplayEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    pub struct EXT_discard_rectangles [ext_discard_rectangles] {
        /// [`vkCmdSetDiscardRectangleEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDiscardRectangleEXT)
        pub fn vkCmdSetDiscardRectangleEXT(commandBuffer: core::VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const core::VkRect2D); [pfn_vkCmdSetDiscardRectangleEXT: ext_discard_rectangles::PFN_vkCmdSetDiscardRectangleEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    pub struct EXT_display_control [ext_display_control] {
        /// [`vkDisplayPowerControlEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDisplayPowerControlEXT)
        pub fn vkDisplayPowerControlEXT(device: core::VkDevice, display: khr_display::VkDisplayKHR, pDisplayPowerInfo: *const ext_display_control::VkDisplayPowerInfoEXT) -> core::VkResult; [pfn_vkDisplayPowerControlEXT: ext_display_control::PFN_vkDisplayPowerControlEXT],

        /// [`vkGetSwapchainCounterEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainCounterEXT)
        pub fn vkGetSwapchainCounterEXT(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, counter: ext_display_surface_counter::VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> core::VkResult; [pfn_vkGetSwapchainCounterEXT: ext_display_control::PFN_vkGetSwapchainCounterEXT],

        /// [`vkRegisterDeviceEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDeviceEventEXT)
        pub fn vkRegisterDeviceEventEXT(device: core::VkDevice, pDeviceEventInfo: *const ext_display_control::VkDeviceEventInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult; [pfn_vkRegisterDeviceEventEXT: ext_display_control::PFN_vkRegisterDeviceEventEXT],

        /// [`vkRegisterDisplayEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDisplayEventEXT)
        pub fn vkRegisterDisplayEventEXT(device: core::VkDevice, display: khr_display::VkDisplayKHR, pDisplayEventInfo: *const ext_display_control::VkDisplayEventInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult; [pfn_vkRegisterDisplayEventEXT: ext_display_control::PFN_vkRegisterDisplayEventEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
    pub struct EXT_display_surface_counter [ext_display_surface_counter] {
        /// [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2EXT)
        pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceCapabilities: *mut ext_display_surface_counter::VkSurfaceCapabilities2EXT) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfaceCapabilities2EXT: ext_display_surface_counter::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
    pub struct EXT_hdr_metadata [ext_hdr_metadata] {
        /// [`vkSetHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetHdrMetadataEXT)
        pub fn vkSetHdrMetadataEXT(device: core::VkDevice, swapchainCount: u32, pSwapchains: *const khr_swapchain::VkSwapchainKHR, pMetadata: *const ext_hdr_metadata::VkHdrMetadataEXT); [pfn_vkSetHdrMetadataEXT: ext_hdr_metadata::PFN_vkSetHdrMetadataEXT],
    }
);

addr_proc_struct!(
    /// [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    pub struct GOOGLE_display_timing [google_display_timing] {
        /// [`vkGetPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPastPresentationTimingGOOGLE)
        pub fn vkGetPastPresentationTimingGOOGLE(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut google_display_timing::VkPastPresentationTimingGOOGLE) -> core::VkResult; [pfn_vkGetPastPresentationTimingGOOGLE: google_display_timing::PFN_vkGetPastPresentationTimingGOOGLE],

        /// [`vkGetRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRefreshCycleDurationGOOGLE)
        pub fn vkGetRefreshCycleDurationGOOGLE(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pDisplayTimingProperties: *mut google_display_timing::VkRefreshCycleDurationGOOGLE) -> core::VkResult; [pfn_vkGetRefreshCycleDurationGOOGLE: google_display_timing::PFN_vkGetRefreshCycleDurationGOOGLE],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)
    pub struct KHR_android_surface [khr_android_surface] {
        /// [`vkCreateAndroidSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateAndroidSurfaceKHR)
        pub fn vkCreateAndroidSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_android_surface::VkAndroidSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateAndroidSurfaceKHR: khr_android_surface::PFN_vkCreateAndroidSurfaceKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub struct KHR_descriptor_update_template [khr_descriptor_update_template] {
        /// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
        pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: core::VkCommandBuffer, descriptorUpdateTemplate: khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR, layout: core::VkPipelineLayout, set: u32, pData: *const c_void); [pfn_vkCmdPushDescriptorSetWithTemplateKHR: khr_descriptor_update_template::PFN_vkCmdPushDescriptorSetWithTemplateKHR],

        /// [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
        pub fn vkCreateDescriptorUpdateTemplateKHR(device: core::VkDevice, pCreateInfo: *const khr_descriptor_update_template::VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR) -> core::VkResult; [pfn_vkCreateDescriptorUpdateTemplateKHR: khr_descriptor_update_template::PFN_vkCreateDescriptorUpdateTemplateKHR],

        /// [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
        pub fn vkDestroyDescriptorUpdateTemplateKHR(device: core::VkDevice, descriptorUpdateTemplate: khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyDescriptorUpdateTemplateKHR: khr_descriptor_update_template::PFN_vkDestroyDescriptorUpdateTemplateKHR],

        /// [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
        pub fn vkUpdateDescriptorSetWithTemplateKHR(device: core::VkDevice, descriptorSet: core::VkDescriptorSet, descriptorUpdateTemplate: khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR, pData: *const c_void); [pfn_vkUpdateDescriptorSetWithTemplateKHR: khr_descriptor_update_template::PFN_vkUpdateDescriptorSetWithTemplateKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub struct KHR_display [khr_display] {
        /// [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
        pub fn vkCreateDisplayModeKHR(physicalDevice: core::VkPhysicalDevice, display: khr_display::VkDisplayKHR, pCreateInfo: *const khr_display::VkDisplayModeCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pMode: *mut khr_display::VkDisplayModeKHR) -> core::VkResult; [pfn_vkCreateDisplayModeKHR: khr_display::PFN_vkCreateDisplayModeKHR],

        /// [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
        pub fn vkCreateDisplayPlaneSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_display::VkDisplaySurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateDisplayPlaneSurfaceKHR: khr_display::PFN_vkCreateDisplayPlaneSurfaceKHR],

        /// [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
        pub fn vkGetDisplayModePropertiesKHR(physicalDevice: core::VkPhysicalDevice, display: khr_display::VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut khr_display::VkDisplayModePropertiesKHR) -> core::VkResult; [pfn_vkGetDisplayModePropertiesKHR: khr_display::PFN_vkGetDisplayModePropertiesKHR],

        /// [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
        pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: core::VkPhysicalDevice, mode: khr_display::VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut khr_display::VkDisplayPlaneCapabilitiesKHR) -> core::VkResult; [pfn_vkGetDisplayPlaneCapabilitiesKHR: khr_display::PFN_vkGetDisplayPlaneCapabilitiesKHR],

        /// [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
        pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: core::VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut khr_display::VkDisplayKHR) -> core::VkResult; [pfn_vkGetDisplayPlaneSupportedDisplaysKHR: khr_display::PFN_vkGetDisplayPlaneSupportedDisplaysKHR],

        /// [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
        pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut khr_display::VkDisplayPlanePropertiesKHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceDisplayPlanePropertiesKHR: khr_display::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR],

        /// [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
        pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut khr_display::VkDisplayPropertiesKHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceDisplayPropertiesKHR: khr_display::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    pub struct KHR_display_swapchain [khr_display_swapchain] {
        /// [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
        pub fn vkCreateSharedSwapchainsKHR(device: core::VkDevice, swapchainCount: u32, pCreateInfos: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSwapchains: *mut khr_swapchain::VkSwapchainKHR) -> core::VkResult; [pfn_vkCreateSharedSwapchainsKHR: khr_display_swapchain::PFN_vkCreateSharedSwapchainsKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_fence_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_capabilities)
    pub struct KHR_external_fence_capabilities [khr_external_fence_capabilities] {
        /// See [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalFencePropertiesKHR)
        pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR(physicalDevice: core::VkPhysicalDevice, pExternalFenceInfo: *const khr_external_fence_capabilities::VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut khr_external_fence_capabilities::VkExternalFencePropertiesKHR); [pfn_vkGetPhysicalDeviceExternalFencePropertiesKHR: khr_external_fence_capabilities::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)
    pub struct KHR_external_fence_fd [khr_external_fence_fd] {
        /// See [`vkImportFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceFdKHR)
        pub fn vkImportFenceFdKHR(device: core::VkDevice, pImportFenceFdInfo: *const khr_external_fence_fd::VkImportFenceFdInfoKHR) -> core::VkResult; [pfn_vkImportFenceFdKHR: khr_external_fence_fd::PFN_vkImportFenceFdKHR],

        /// See [`vkGetFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceFdKHR)
        pub fn vkGetFenceFdKHR(device: core::VkDevice, pGetFdInfo: *const khr_external_fence_fd::VkFenceGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult; [pfn_vkGetFenceFdKHR: khr_external_fence_fd::PFN_vkGetFenceFdKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
    pub struct KHR_external_fence_win32 [khr_external_fence_win32] {
        /// See [`vkImportFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceWin32HandleKHR)
        pub fn vkImportFenceWin32HandleKHR(device: core::VkDevice, pImportFenceWin32HandleInfo: *const khr_external_fence_win32::VkImportFenceWin32HandleInfoKHR) -> core::VkResult; [pfn_vkImportFenceWin32HandleKHR: khr_external_fence_win32::PFN_vkImportFenceWin32HandleKHR],

        /// See [`vkGetFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceWin32HandleKHR)
        pub fn vkGetFenceWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const khr_external_fence_win32::VkFenceGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetFenceWin32HandleKHR: khr_external_fence_win32::PFN_vkGetFenceWin32HandleKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)
    pub struct KHR_external_memory_capabilities [khr_external_memory_capabilities] {
        /// See [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalBufferPropertiesKHR)
        pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(physicalDevice: core::VkPhysicalDevice, pExternalBufferInfo: *const khr_external_memory_capabilities::VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut khr_external_memory_capabilities::VkExternalBufferPropertiesKHR); [pfn_vkGetPhysicalDeviceExternalBufferPropertiesKHR: khr_external_memory_capabilities::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
    pub struct KHR_external_memory_fd [khr_external_memory_fd] {
        /// See [`vkGetMemoryFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHR)
        pub fn vkGetMemoryFdKHR(device: core::VkDevice, pGetFdInfo: *const khr_external_memory_fd::VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult; [pfn_vkGetMemoryFdKHR: khr_external_memory_fd::PFN_vkGetMemoryFdKHR],

        /// See [`vkGetMemoryFdPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHR)
        pub fn vkGetMemoryFdPropertiesKHR(device: core::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, fd: c_int, pMemoryFdProperties: *mut khr_external_memory_fd::VkMemoryFdPropertiesKHR) -> core::VkResult; [pfn_vkGetMemoryFdPropertiesKHR: khr_external_memory_fd::PFN_vkGetMemoryFdPropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
    pub struct KHR_external_memory_win32 [khr_external_memory_win32] {
        /// See [`vkGetMemoryWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHR)
        pub fn vkGetMemoryWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const khr_external_memory_win32::VkMemoryGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetMemoryWin32HandleKHR: khr_external_memory_win32::PFN_vkGetMemoryWin32HandleKHR],

        /// See [`vkGetMemoryWin32HandlePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHR)
        pub fn vkGetMemoryWin32HandlePropertiesKHR(device: core::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, handle: win32_types::HANDLE, pMemoryWin32HandleProperties: *mut khr_external_memory_win32::VkMemoryWin32HandlePropertiesKHR) -> core::VkResult; [pfn_vkGetMemoryWin32HandlePropertiesKHR: khr_external_memory_win32::PFN_vkGetMemoryWin32HandlePropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_capabilities)
    pub struct KHR_external_semaphore_capabilities [khr_external_semaphore_capabilities] {
        /// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)
        pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(physicalDevice: core::VkPhysicalDevice, pExternalSemaphoreInfo: *const khr_external_semaphore_capabilities::VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut khr_external_semaphore_capabilities::VkExternalSemaphorePropertiesKHR); [pfn_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR: khr_external_semaphore_capabilities::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
    pub struct KHR_external_semaphore_fd [khr_external_semaphore_fd] {
        /// See [`VkImportSemaphoreFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHR)
        pub fn vkImportSemaphoreFdKHR(device: core::VkDevice, pImportSemaphoreFdInfo: *const khr_external_semaphore_fd::VkImportSemaphoreFdInfoKHR) -> core::VkResult; [pfn_vkImportSemaphoreFdKHR: khr_external_semaphore_fd::PFN_vkImportSemaphoreFdKHR],

        /// See [`vkGetSemaphoreFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHR)
        pub fn vkGetSemaphoreFdKHR(device: core::VkDevice, pGetFdInfo: *const khr_external_semaphore_fd::VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult; [pfn_vkGetSemaphoreFdKHR: khr_external_semaphore_fd::PFN_vkGetSemaphoreFdKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
    pub struct KHR_external_semaphore_win32 [khr_external_semaphore_win32] {
        /// See [`VkImportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHR)
        pub fn vkImportSemaphoreWin32HandleKHR(device: core::VkDevice, pImportSemaphoreWin32HandleInfo: *const khr_external_semaphore_win32::VkImportSemaphoreWin32HandleInfoKHR) -> core::VkResult; [pfn_vkImportSemaphoreWin32HandleKHR: khr_external_semaphore_win32::PFN_vkImportSemaphoreWin32HandleKHR],

        /// See [`vkGetSemaphoreWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHR)
        pub fn vkGetSemaphoreWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const khr_external_semaphore_win32::VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetSemaphoreWin32HandleKHR: khr_external_semaphore_win32::PFN_vkGetSemaphoreWin32HandleKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
    pub struct KHR_get_memory_requirements2 [khr_get_memory_requirements2] {
        /// See [`vkGetImageMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements2KHR)
        pub fn vkGetImageMemoryRequirements2KHR(device: core::VkDevice, pInfo: *const khr_get_memory_requirements2::VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut khr_get_memory_requirements2::VkMemoryRequirements2KHR); [pfn_vkGetImageMemoryRequirements2KHR: khr_get_memory_requirements2::PFN_vkGetImageMemoryRequirements2KHR],

        /// See [`vkGetBufferMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements2KHR)
        pub fn vkGetBufferMemoryRequirements2KHR(device: core::VkDevice, pInfo: *const khr_get_memory_requirements2::VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut khr_get_memory_requirements2::VkMemoryRequirements2KHR); [pfn_vkGetBufferMemoryRequirements2KHR: khr_get_memory_requirements2::PFN_vkGetBufferMemoryRequirements2KHR],

        /// See [`vkGetImageSparseMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements2KHR)
        pub fn vkGetImageSparseMemoryRequirements2KHR(device: core::VkDevice, pInfo: *const khr_get_memory_requirements2::VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut khr_get_memory_requirements2::VkSparseImageMemoryRequirements2KHR); [pfn_vkGetImageSparseMemoryRequirements2KHR: khr_get_memory_requirements2::PFN_vkGetImageSparseMemoryRequirements2KHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub struct KHR_get_physical_device_properties2 [khr_get_physical_device_properties2] {
        /// [`vkGetPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures2KHR)
        pub fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: core::VkPhysicalDevice, pFeatures: *mut khr_get_physical_device_properties2::VkPhysicalDeviceFeatures2KHR); [pfn_vkGetPhysicalDeviceFeatures2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceFeatures2KHR],

        /// [`vkGetPhysicalDeviceFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties2KHR)
        pub fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: core::VkPhysicalDevice, format: core::VkFormat, pFormatProperties: *mut khr_get_physical_device_properties2::VkFormatProperties2KHR); [pfn_vkGetPhysicalDeviceFormatProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceFormatProperties2KHR],

        /// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties2KHR)
        pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice: core::VkPhysicalDevice, pImageFormatInfo: *const khr_get_physical_device_properties2::VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut khr_get_physical_device_properties2::VkImageFormatProperties2KHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceImageFormatProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR],

        /// [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties2KHR)
        pub fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: core::VkPhysicalDevice, pMemoryProperties: *mut khr_get_physical_device_properties2::VkPhysicalDeviceMemoryProperties2KHR); [pfn_vkGetPhysicalDeviceMemoryProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceMemoryProperties2KHR],

        /// [`vkGetPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties2KHR)
        pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: core::VkPhysicalDevice, pProperties: *mut khr_get_physical_device_properties2::VkPhysicalDeviceProperties2KHR); [pfn_vkGetPhysicalDeviceProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceProperties2KHR],

        /// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties2KHR)
        pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: core::VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut khr_get_physical_device_properties2::VkQueueFamilyProperties2KHR); [pfn_vkGetPhysicalDeviceQueueFamilyProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR],

        /// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties2KHR)
        pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice: core::VkPhysicalDevice, pFormatInfo: *const khr_get_physical_device_properties2::VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut khr_get_physical_device_properties2::VkSparseImageFormatProperties2KHR); [pfn_vkGetPhysicalDeviceSparseImageFormatProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)
    pub struct KHR_get_surface_capabilities2 [khr_get_surface_capabilities2] {
        /// [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2KHR)
        pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice: core::VkPhysicalDevice, pSurfaceInfo: *const khr_get_surface_capabilities2::VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut khr_get_surface_capabilities2::VkSurfaceCapabilities2KHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfaceCapabilities2KHR: khr_get_surface_capabilities2::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR],

        /// [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormats2KHR)
        pub fn vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice: core::VkPhysicalDevice, pSurfaceInfo: *const khr_get_surface_capabilities2::VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut khr_get_surface_capabilities2::VkSurfaceFormat2KHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfaceFormats2KHR: khr_get_surface_capabilities2::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
    pub struct KHR_maintenance1 [khr_maintenance1] {
        /// [`vkTrimCommandPoolKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkTrimCommandPoolKHR)
        pub fn vkTrimCommandPoolKHR(device: core::VkDevice, commandPool: core::VkCommandPool, flags: khr_maintenance1::VkCommandPoolTrimFlagsKHR); [pfn_vkTrimCommandPoolKHR: khr_maintenance1::PFN_vkTrimCommandPoolKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)
    pub struct KHR_mir_surface [khr_mir_surface] {
        /// [`vkCreateMirSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMirSurfaceKHR)
        pub fn vkCreateMirSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_mir_surface::VkMirSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateMirSurfaceKHR: khr_mir_surface::PFN_vkCreateMirSurfaceKHR],

        /// [`vkGetPhysicalDeviceMirPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMirPresentationSupportKHR)
        pub fn vkGetPhysicalDeviceMirPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut mir_types::MirConnection) -> core::VkBool32; [pfn_vkGetPhysicalDeviceMirPresentationSupportKHR: khr_mir_surface::PFN_vkGetPhysicalDeviceMirPresentationSupportKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
    pub struct KHR_push_descriptor [khr_push_descriptor] {
        /// [`vkCmdPushDescriptorSetKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetKHR)
        pub fn vkCmdPushDescriptorSetKHR(commandBuffer: core::VkCommandBuffer, pipelineBindPoint: core::VkPipelineBindPoint, layout: core::VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const core::VkWriteDescriptorSet); [pfn_vkCmdPushDescriptorSetKHR: khr_push_descriptor::PFN_vkCmdPushDescriptorSetKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_shared_presentable_image)
    pub struct KHR_shared_presentable_image [khr_shared_presentable_image] {
        /// [`vkGetSwapchainStatusKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainStatusKHR)
        pub fn vkGetSwapchainStatusKHR(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR) -> core::VkResult; [pfn_vkGetSwapchainStatusKHR: khr_shared_presentable_image::PFN_vkGetSwapchainStatusKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub struct KHR_surface [khr_surface] {
        /// [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySurfaceKHR)
        pub fn vkDestroySurfaceKHR(instance: core::VkInstance, surface: khr_surface::VkSurfaceKHR, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroySurfaceKHR: khr_surface::PFN_vkDestroySurfaceKHR],

        /// [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
        pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceCapabilities: *mut khr_surface::VkSurfaceCapabilitiesKHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfaceCapabilitiesKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR],

        /// [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
        pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut khr_surface::VkSurfaceFormatKHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfaceFormatsKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR],

        /// [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
        pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut khr_surface::VkPresentModeKHR) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfacePresentModesKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR],

        /// [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
        pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, surface: khr_surface::VkSurfaceKHR, pSupported: *mut core::VkBool32) -> core::VkResult; [pfn_vkGetPhysicalDeviceSurfaceSupportKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfaceSupportKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub struct KHR_swapchain [khr_swapchain] {
        /// [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
        pub fn vkAcquireNextImageKHR(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, timeout: u64, semaphore: core::VkSemaphore, fence: core::VkFence, pImageIndex: *mut u32) -> core::VkResult; [pfn_vkAcquireNextImageKHR: khr_swapchain::PFN_vkAcquireNextImageKHR],

        /// [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSwapchainKHR)
        pub fn vkCreateSwapchainKHR(device: core::VkDevice, pCreateInfo: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSwapchain: *mut khr_swapchain::VkSwapchainKHR) -> core::VkResult; [pfn_vkCreateSwapchainKHR: khr_swapchain::PFN_vkCreateSwapchainKHR],

        /// [`vkDestroySwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySwapchainKHR)
        pub fn vkDestroySwapchainKHR(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroySwapchainKHR: khr_swapchain::PFN_vkDestroySwapchainKHR],

        /// [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainImagesKHR)
        pub fn vkGetSwapchainImagesKHR(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut core::VkImage) -> core::VkResult; [pfn_vkGetSwapchainImagesKHR: khr_swapchain::PFN_vkGetSwapchainImagesKHR],

        /// [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
        pub fn vkQueuePresentKHR(queue: core::VkQueue, pPresentInfo: *const khr_swapchain::VkPresentInfoKHR) -> core::VkResult; [pfn_vkQueuePresentKHR: khr_swapchain::PFN_vkQueuePresentKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub struct KHR_wayland_surface [khr_wayland_surface] {
        /// [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
        pub fn vkCreateWaylandSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_wayland_surface::VkWaylandSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateWaylandSurfaceKHR: khr_wayland_surface::PFN_vkCreateWaylandSurfaceKHR],

        /// [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
        pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_types::wl_display) -> core::VkBool32; [pfn_vkGetPhysicalDeviceWaylandPresentationSupportKHR: khr_wayland_surface::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)
    pub struct KHR_win32_surface [khr_win32_surface] {
        /// [`vkCreateWin32SurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWin32SurfaceKHR)
        pub fn vkCreateWin32SurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_win32_surface::VkWin32SurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateWin32SurfaceKHR: khr_win32_surface::PFN_vkCreateWin32SurfaceKHR],

        /// [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWin32PresentationSupportKHR)
        pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32) -> core::VkBool32; [pfn_vkGetPhysicalDeviceWin32PresentationSupportKHR: khr_win32_surface::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub struct KHR_xcb_surface [khr_xcb_surface] {
        /// [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
        pub fn vkCreateXcbSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_xcb_surface::VkXcbSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateXcbSurfaceKHR: khr_xcb_surface::PFN_vkCreateXcbSurfaceKHR],

        /// [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
        pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_types::xcb_connection_t, visual_id: xcb_types::xcb_visualid_t) -> core::VkBool32; [pfn_vkGetPhysicalDeviceXcbPresentationSupportKHR: khr_xcb_surface::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    pub struct KHR_xlib_surface [khr_xlib_surface] {
        /// [`vkCreateXlibSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXlibSurfaceKHR)
        pub fn vkCreateXlibSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const khr_xlib_surface::VkXlibSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateXlibSurfaceKHR: khr_xlib_surface::PFN_vkCreateXlibSurfaceKHR],

        /// [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXlibPresentationSupportKHR)
        pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut xlib_types::Display, visualID: xlib_types::VisualID) -> core::VkBool32; [pfn_vkGetPhysicalDeviceXlibPresentationSupportKHR: khr_xlib_surface::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR],
    }
);

addr_proc_struct!(
    /// [`VK_MVK_ios_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_ios_surface)
    pub struct MVK_ios_surface [mvk_ios_surface] {
        /// [`vkCreateIOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIOSSurfaceMVK)
        pub fn vkCreateIOSSurfaceMVK(instance: core::VkInstance, pCreateInfo: *const mvk_ios_surface::VkIOSSurfaceCreateInfoMVK, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateIOSSurfaceMVK: mvk_ios_surface::PFN_vkCreateIOSSurfaceMVK],
    }
);

addr_proc_struct!(
    /// [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
    pub struct MVK_macos_surface [mvk_macos_surface] {
        /// [`vkCreateMacOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMacOSSurfaceMVK)
        pub fn vkCreateMacOSSurfaceMVK(instance: core::VkInstance, pCreateInfo: *const mvk_macos_surface::VkMacOSSurfaceCreateInfoMVK, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateMacOSSurfaceMVK: mvk_macos_surface::PFN_vkCreateMacOSSurfaceMVK],
    }
);

addr_proc_struct!(
    /// [`VK_NN_vi_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NN_vi_surface)
    pub struct NN_vi_surface [nn_vi_surface] {
        /// [`vkCreateViSurfaceNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateViSurfaceNN)
        pub fn vkCreateViSurfaceNN(instance: core::VkInstance, pCreateInfo: *const nn_vi_surface::VkViSurfaceCreateInfoNN, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult; [pfn_vkCreateViSurfaceNN: nn_vi_surface::PFN_vkCreateViSurfaceNN],
    }
);

addr_proc_struct!(
    /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    pub struct NV_clip_space_w_scaling [nv_clip_space_w_scaling] {
        /// [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
        pub fn vkCmdSetViewportWScalingNV(commandBuffer: core::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const nv_clip_space_w_scaling::VkViewportWScalingNV); [pfn_vkCmdSetViewportWScalingNV: nv_clip_space_w_scaling::PFN_vkCmdSetViewportWScalingNV],
    }
);

addr_proc_struct!(
    /// [`VK_NV_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_capabilities)
    pub struct NV_external_memory_capabilities [nv_external_memory_capabilities] {
        /// [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalImageFormatPropertiesNV)
        pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(physicalDevice: core::VkPhysicalDevice, format: core::VkFormat, type_: core::VkImageType, tiling: core::VkImageTiling, usage: core::VkImageUsageFlags, flags: core::VkImageCreateFlags, externalHandleType: nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV, pExternalImageFormatProperties: *mut nv_external_memory_capabilities::VkExternalImageFormatPropertiesNV) -> core::VkResult; [pfn_vkGetPhysicalDeviceExternalImageFormatPropertiesNV: nv_external_memory_capabilities::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV],
    }
);

addr_proc_struct!(
    /// [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    pub struct NV_external_memory_win32 [nv_external_memory_win32] {
        /// [`vkGetMemoryWin32HandleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleNV)
        pub fn vkGetMemoryWin32HandleNV(device: core::VkDevice, memory: core::VkDeviceMemory, handleType: nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetMemoryWin32HandleNV: nv_external_memory_win32::PFN_vkGetMemoryWin32HandleNV],
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub struct KHX_device_group [khx_device_group] {
        /// [`vkAcquireNextImage2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImage2KHX)
        pub fn vkAcquireNextImage2KHX(device: core::VkDevice, pAcquireInfo: *const khx_device_group::VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> core::VkResult; [pfn_vkAcquireNextImage2KHX: khx_device_group::PFN_vkAcquireNextImage2KHX],

        /// [`vkBindBufferMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory2KHX)
        pub fn vkBindBufferMemory2KHX(device: core::VkDevice, bindInfoCount: u32, pBindInfos: *const khx_device_group::VkBindBufferMemoryInfoKHX) -> core::VkResult; [pfn_vkBindBufferMemory2KHX: khx_device_group::PFN_vkBindBufferMemory2KHX],

        /// [`vkBindImageMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory2KHX)
        pub fn vkBindImageMemory2KHX(device: core::VkDevice, bindInfoCount: u32, pBindInfos: *const khx_device_group::VkBindImageMemoryInfoKHX) -> core::VkResult; [pfn_vkBindImageMemory2KHX: khx_device_group::PFN_vkBindImageMemory2KHX],

        /// [`vkCmdDispatchBaseKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchBaseKHX)
        pub fn vkCmdDispatchBaseKHX(commandBuffer: core::VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32); [pfn_vkCmdDispatchBaseKHX: khx_device_group::PFN_vkCmdDispatchBaseKHX],

        /// [`vkCmdSetDeviceMaskKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDeviceMaskKHX)
        pub fn vkCmdSetDeviceMaskKHX(commandBuffer: core::VkCommandBuffer, deviceMask: u32); [pfn_vkCmdSetDeviceMaskKHX: khx_device_group::PFN_vkCmdSetDeviceMaskKHX],

        /// [`vkGetDeviceGroupPeerMemoryFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPeerMemoryFeaturesKHX)
        pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX(device: core::VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut khx_device_group::VkPeerMemoryFeatureFlagsKHX); [pfn_vkGetDeviceGroupPeerMemoryFeaturesKHX: khx_device_group::PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX],

        /// [`vkGetDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPresentCapabilitiesKHX)
        pub fn vkGetDeviceGroupPresentCapabilitiesKHX(device: core::VkDevice, pDeviceGroupPresentCapabilities: *mut khx_device_group::VkDeviceGroupPresentCapabilitiesKHX) -> core::VkResult; [pfn_vkGetDeviceGroupPresentCapabilitiesKHX: khx_device_group::PFN_vkGetDeviceGroupPresentCapabilitiesKHX],

        /// [`vkGetDeviceGroupSurfacePresentModesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupSurfacePresentModesKHX)
        pub fn vkGetDeviceGroupSurfacePresentModesKHX(device: core::VkDevice, surface: khr_surface::VkSurfaceKHR, pModes: *mut khx_device_group::VkDeviceGroupPresentModeFlagsKHX) -> core::VkResult; [pfn_vkGetDeviceGroupSurfacePresentModesKHX: khx_device_group::PFN_vkGetDeviceGroupSurfacePresentModesKHX],

        /// [`vkGetPhysicalDevicePresentRectanglesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDevicePresentRectanglesKHX)
        pub fn vkGetPhysicalDevicePresentRectanglesKHX(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut core::VkRect2D) -> core::VkResult; [pfn_vkGetPhysicalDevicePresentRectanglesKHX: khx_device_group::PFN_vkGetPhysicalDevicePresentRectanglesKHX],
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)
    pub struct KHX_device_group_creation [khx_device_group_creation] {
        /// [`vkEnumeratePhysicalDeviceGroupsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDeviceGroupsKHX)
        pub fn vkEnumeratePhysicalDeviceGroupsKHX(instance: core::VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut khx_device_group_creation::VkPhysicalDeviceGroupPropertiesKHX) -> core::VkResult; [pfn_vkEnumeratePhysicalDeviceGroupsKHX: khx_device_group_creation::PFN_vkEnumeratePhysicalDeviceGroupsKHX],
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub struct NVX_device_generated_commands [nvx_device_generated_commands] {
        /// [`vkCmdProcessCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdProcessCommandsNVX)
        pub fn vkCmdProcessCommandsNVX(commandBuffer: core::VkCommandBuffer, pProcessCommandsInfo: *const nvx_device_generated_commands::VkCmdProcessCommandsInfoNVX); [pfn_vkCmdProcessCommandsNVX: nvx_device_generated_commands::PFN_vkCmdProcessCommandsNVX],

        /// [`vkCmdReserveSpaceForCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdReserveSpaceForCommandsNVX)
        pub fn vkCmdReserveSpaceForCommandsNVX(commandBuffer: core::VkCommandBuffer, pReserveSpaceInfo: *const nvx_device_generated_commands::VkCmdReserveSpaceForCommandsInfoNVX); [pfn_vkCmdReserveSpaceForCommandsNVX: nvx_device_generated_commands::PFN_vkCmdReserveSpaceForCommandsNVX],

        /// [`vkCreateIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIndirectCommandsLayoutNVX)
        pub fn vkCreateIndirectCommandsLayoutNVX(device: core::VkDevice, pCreateInfo: *const nvx_device_generated_commands::VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const core::VkAllocationCallbacks, pIndirectCommandsLayout: *mut nvx_device_generated_commands::VkIndirectCommandsLayoutNVX) -> core::VkResult; [pfn_vkCreateIndirectCommandsLayoutNVX: nvx_device_generated_commands::PFN_vkCreateIndirectCommandsLayoutNVX],

        /// [`vkCreateObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateObjectTableNVX)
        pub fn vkCreateObjectTableNVX(device: core::VkDevice, pCreateInfo: *const nvx_device_generated_commands::VkObjectTableCreateInfoNVX, pAllocator: *const core::VkAllocationCallbacks, pObjectTable: *mut nvx_device_generated_commands::VkObjectTableNVX) -> core::VkResult; [pfn_vkCreateObjectTableNVX: nvx_device_generated_commands::PFN_vkCreateObjectTableNVX],

        /// [`vkDestroyIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyIndirectCommandsLayoutNVX)
        pub fn vkDestroyIndirectCommandsLayoutNVX(device: core::VkDevice, indirectCommandsLayout: nvx_device_generated_commands::VkIndirectCommandsLayoutNVX, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyIndirectCommandsLayoutNVX: nvx_device_generated_commands::PFN_vkDestroyIndirectCommandsLayoutNVX],

        /// [`vkDestroyObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyObjectTableNVX)
        pub fn vkDestroyObjectTableNVX(device: core::VkDevice, objectTable: nvx_device_generated_commands::VkObjectTableNVX, pAllocator: *const core::VkAllocationCallbacks); [pfn_vkDestroyObjectTableNVX: nvx_device_generated_commands::PFN_vkDestroyObjectTableNVX],

        /// [`vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)
        pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(physicalDevice: core::VkPhysicalDevice, pFeatures: *mut nvx_device_generated_commands::VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut nvx_device_generated_commands::VkDeviceGeneratedCommandsLimitsNVX); [pfn_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX: nvx_device_generated_commands::PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX],

        /// [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
        pub fn vkRegisterObjectsNVX(device: core::VkDevice, objectTable: nvx_device_generated_commands::VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const nvx_device_generated_commands::VkObjectTableEntryNVX, pObjectIndices: *const u32) -> core::VkResult; [pfn_vkRegisterObjectsNVX: nvx_device_generated_commands::PFN_vkRegisterObjectsNVX],

        /// [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
        pub fn vkUnregisterObjectsNVX(device: core::VkDevice, objectTable: nvx_device_generated_commands::VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const nvx_device_generated_commands::VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> core::VkResult; [pfn_vkUnregisterObjectsNVX: nvx_device_generated_commands::PFN_vkUnregisterObjectsNVX],
    }
);
