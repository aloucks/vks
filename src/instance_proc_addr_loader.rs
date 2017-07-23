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
use libc::{c_char, c_void};
use mvk_ios_surface;
use mvk_macos_surface;
use nn_vi_surface;
use nv_clip_space_w_scaling;
use nv_external_memory_capabilities;
use nv_external_memory_win32;
use std::fmt;
use std::mem;
use std::ptr;

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
            pub vkGetInstanceProcAddr: core::PFN_vkGetInstanceProcAddr,

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
                debug_struct.field("vkGetInstanceProcAddr", &(self.vkGetInstanceProcAddr as *mut c_void));
                debug_struct.field("core_global", &self.core_global);

                $(
                    debug_struct.field(stringify!($field), &self.$field);
                )*

                $(
                    #[cfg(feature = "experimental")]
                    debug_struct.field(stringify!($field), &self.$field);
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
                unsafe {
                    InstanceProcAddrLoader::from_get_instance_proc_addr(mem::transmute(0usize))
                }
            }

            pub fn from_get_instance_proc_addr(vkGetInstanceProcAddr: core::PFN_vkGetInstanceProcAddr) -> Self {
                unsafe {
                    let mut res: InstanceProcAddrLoader = mem::uninitialized();

                    ptr::write(&mut res.vkGetInstanceProcAddr, vkGetInstanceProcAddr);
                    ptr::write(&mut res.core_global, CoreGlobal::new());

                    $(
                        ptr::write(&mut res.$field, $ty::new());
                    )*

                    $(
                        #[cfg(feature = "experimental")]
                        ptr::write(&mut res.$exp_field, $exp_ty::new());
                    )*

                    ptr::write(&mut res.guard, ());

                    res
                }
            }

            pub unsafe fn load_core_global(&mut self) {
                self.core_global.load(self.vkGetInstanceProcAddr, ptr::null_mut());
            }

            $(
                pub unsafe fn $load(&mut self, instance: core::VkInstance) {
                    self.$field.load(self.vkGetInstanceProcAddr, instance);
                }
            )*

            $(
                #[cfg(feature = "experimental")]
                pub unsafe fn $exp_load(&mut self, instance: core::VkInstance) {
                    self.$exp_field.load(self.vkGetInstanceProcAddr, instance);
                }
            )*
        }
    }
}

macro_rules! addr_proc_struct {
    (
        $( #[$attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$symbol_attr:meta] )*
                pub $symbol:ident: $ty:ty,
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
                $(
                    debug_struct.field(stringify!($symbol), &(self.$symbol as *mut c_void));
                )*
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
                        $symbol: unsafe { mem::transmute(0usize) },
                    )*
                    guard: (),
                }
            }

            #[allow(unused_variables)]
            pub unsafe fn load(&mut self, vkGetInstanceProcAddr: ::core::PFN_vkGetInstanceProcAddr, instance: ::core::VkInstance) {
                $(
                    self.$symbol = mem::transmute((vkGetInstanceProcAddr)(instance, concat!(stringify!($symbol), '\x00').as_ptr() as *const c_char));
                )*
            }
        }
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

            /// [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
            pub khx_external_memory_capabilities: KHX_external_memory_capabilities [fn load_khx_external_memory_capabilities],

            /// [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
            pub khx_external_memory_fd: KHX_external_memory_fd [fn load_khx_external_memory_fd],

            /// [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
            pub khx_external_memory_win32: KHX_external_memory_win32 [fn load_khx_external_memory_win32],

            /// [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
            pub khx_external_semaphore_capabilities: KHX_external_semaphore_capabilities [fn load_khx_external_semaphore_capabilities],

            /// [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
            pub khx_external_semaphore_win32: KHX_external_semaphore_win32 [fn load_khx_external_semaphore_win32],

            /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
            pub nvx_device_generated_commands: NVX_device_generated_commands [fn load_nvx_device_generated_commands],
        }
    }
);

addr_proc_struct!(
    /// Core functions, which don't require a dispatchable Vulkan object
    pub struct CoreGlobal {
        /// [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
        pub vkCreateInstance: core::PFN_vkCreateInstance,

        /// [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
        pub vkEnumerateInstanceExtensionProperties: core::PFN_vkEnumerateInstanceExtensionProperties,

        /// [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
        pub vkEnumerateInstanceLayerProperties: core::PFN_vkEnumerateInstanceLayerProperties,
    }
);

addr_proc_struct!(
    /// [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)
    pub struct Core {
        /// [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
        pub vkAllocateCommandBuffers: core::PFN_vkAllocateCommandBuffers,

        /// [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
        pub vkAllocateDescriptorSets: core::PFN_vkAllocateDescriptorSets,

        /// [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
        pub vkAllocateMemory: core::PFN_vkAllocateMemory,

        /// [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
        pub vkBeginCommandBuffer: core::PFN_vkBeginCommandBuffer,

        /// [`vkBindBufferMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory)
        pub vkBindBufferMemory: core::PFN_vkBindBufferMemory,

        /// [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
        pub vkBindImageMemory: core::PFN_vkBindImageMemory,

        /// [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
        pub vkCmdBeginQuery: core::PFN_vkCmdBeginQuery,

        /// [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
        pub vkCmdBeginRenderPass: core::PFN_vkCmdBeginRenderPass,

        /// [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
        pub vkCmdBindDescriptorSets: core::PFN_vkCmdBindDescriptorSets,

        /// [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
        pub vkCmdBindIndexBuffer: core::PFN_vkCmdBindIndexBuffer,

        /// [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
        pub vkCmdBindPipeline: core::PFN_vkCmdBindPipeline,

        /// [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
        pub vkCmdBindVertexBuffers: core::PFN_vkCmdBindVertexBuffers,

        /// [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
        pub vkCmdBlitImage: core::PFN_vkCmdBlitImage,

        /// [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
        pub vkCmdClearAttachments: core::PFN_vkCmdClearAttachments,

        /// [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
        pub vkCmdClearColorImage: core::PFN_vkCmdClearColorImage,

        /// [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
        pub vkCmdClearDepthStencilImage: core::PFN_vkCmdClearDepthStencilImage,

        /// [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
        pub vkCmdCopyBuffer: core::PFN_vkCmdCopyBuffer,

        /// [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
        pub vkCmdCopyBufferToImage: core::PFN_vkCmdCopyBufferToImage,

        /// [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
        pub vkCmdCopyImage: core::PFN_vkCmdCopyImage,

        /// [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
        pub vkCmdCopyImageToBuffer: core::PFN_vkCmdCopyImageToBuffer,

        /// [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
        pub vkCmdCopyQueryPoolResults: core::PFN_vkCmdCopyQueryPoolResults,

        /// [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
        pub vkCmdDispatch: core::PFN_vkCmdDispatch,

        /// [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
        pub vkCmdDispatchIndirect: core::PFN_vkCmdDispatchIndirect,

        /// [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
        pub vkCmdDraw: core::PFN_vkCmdDraw,

        /// [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
        pub vkCmdDrawIndexed: core::PFN_vkCmdDrawIndexed,

        /// [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
        pub vkCmdDrawIndexedIndirect: core::PFN_vkCmdDrawIndexedIndirect,

        /// [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
        pub vkCmdDrawIndirect: core::PFN_vkCmdDrawIndirect,

        /// [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
        pub vkCmdEndQuery: core::PFN_vkCmdEndQuery,

        /// [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
        pub vkCmdEndRenderPass: core::PFN_vkCmdEndRenderPass,

        /// [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
        pub vkCmdExecuteCommands: core::PFN_vkCmdExecuteCommands,

        /// [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
        pub vkCmdFillBuffer: core::PFN_vkCmdFillBuffer,

        /// [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
        pub vkCmdNextSubpass: core::PFN_vkCmdNextSubpass,

        /// [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
        pub vkCmdPipelineBarrier: core::PFN_vkCmdPipelineBarrier,

        /// [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
        pub vkCmdPushConstants: core::PFN_vkCmdPushConstants,

        /// [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
        pub vkCmdResetEvent: core::PFN_vkCmdResetEvent,

        /// [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
        pub vkCmdResetQueryPool: core::PFN_vkCmdResetQueryPool,

        /// [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
        pub vkCmdResolveImage: core::PFN_vkCmdResolveImage,

        /// [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
        pub vkCmdSetBlendConstants: core::PFN_vkCmdSetBlendConstants,

        /// [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
        pub vkCmdSetDepthBias: core::PFN_vkCmdSetDepthBias,

        /// [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
        pub vkCmdSetDepthBounds: core::PFN_vkCmdSetDepthBounds,

        /// [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
        pub vkCmdSetEvent: core::PFN_vkCmdSetEvent,

        /// [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
        pub vkCmdSetLineWidth: core::PFN_vkCmdSetLineWidth,

        /// [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
        pub vkCmdSetScissor: core::PFN_vkCmdSetScissor,

        /// [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
        pub vkCmdSetStencilCompareMask: core::PFN_vkCmdSetStencilCompareMask,

        /// [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
        pub vkCmdSetStencilReference: core::PFN_vkCmdSetStencilReference,

        /// [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
        pub vkCmdSetStencilWriteMask: core::PFN_vkCmdSetStencilWriteMask,

        /// [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
        pub vkCmdSetViewport: core::PFN_vkCmdSetViewport,

        /// [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
        pub vkCmdUpdateBuffer: core::PFN_vkCmdUpdateBuffer,

        /// [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
        pub vkCmdWaitEvents: core::PFN_vkCmdWaitEvents,

        /// [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
        pub vkCmdWriteTimestamp: core::PFN_vkCmdWriteTimestamp,

        /// [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
        pub vkCreateBuffer: core::PFN_vkCreateBuffer,

        /// [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
        pub vkCreateBufferView: core::PFN_vkCreateBufferView,

        /// [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
        pub vkCreateCommandPool: core::PFN_vkCreateCommandPool,

        /// [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
        pub vkCreateComputePipelines: core::PFN_vkCreateComputePipelines,

        /// [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
        pub vkCreateDescriptorPool: core::PFN_vkCreateDescriptorPool,

        /// [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
        pub vkCreateDescriptorSetLayout: core::PFN_vkCreateDescriptorSetLayout,

        /// [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
        pub vkCreateDevice: core::PFN_vkCreateDevice,

        /// [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
        pub vkCreateEvent: core::PFN_vkCreateEvent,

        /// [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
        pub vkCreateFence: core::PFN_vkCreateFence,

        /// [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
        pub vkCreateFramebuffer: core::PFN_vkCreateFramebuffer,

        /// [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
        pub vkCreateGraphicsPipelines: core::PFN_vkCreateGraphicsPipelines,

        /// [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
        pub vkCreateImage: core::PFN_vkCreateImage,

        /// [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
        pub vkCreateImageView: core::PFN_vkCreateImageView,

        /// [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
        pub vkCreatePipelineCache: core::PFN_vkCreatePipelineCache,

        /// [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
        pub vkCreatePipelineLayout: core::PFN_vkCreatePipelineLayout,

        /// [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
        pub vkCreateQueryPool: core::PFN_vkCreateQueryPool,

        /// [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
        pub vkCreateRenderPass: core::PFN_vkCreateRenderPass,

        /// [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
        pub vkCreateSampler: core::PFN_vkCreateSampler,

        /// [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
        pub vkCreateSemaphore: core::PFN_vkCreateSemaphore,

        /// [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
        pub vkCreateShaderModule: core::PFN_vkCreateShaderModule,

        /// [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBuffer)
        pub vkDestroyBuffer: core::PFN_vkDestroyBuffer,

        /// [`vkDestroyBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBufferView)
        pub vkDestroyBufferView: core::PFN_vkDestroyBufferView,

        /// [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyCommandPool)
        pub vkDestroyCommandPool: core::PFN_vkDestroyCommandPool,

        /// [`vkDestroyDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorPool)
        pub vkDestroyDescriptorPool: core::PFN_vkDestroyDescriptorPool,

        /// [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorSetLayout)
        pub vkDestroyDescriptorSetLayout: core::PFN_vkDestroyDescriptorSetLayout,

        /// [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDevice)
        pub vkDestroyDevice: core::PFN_vkDestroyDevice,

        /// [`vkDestroyEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyEvent)
        pub vkDestroyEvent: core::PFN_vkDestroyEvent,

        /// [`vkDestroyFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFence)
        pub vkDestroyFence: core::PFN_vkDestroyFence,

        /// [`vkDestroyFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFramebuffer)
        pub vkDestroyFramebuffer: core::PFN_vkDestroyFramebuffer,

        /// [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImage)
        pub vkDestroyImage: core::PFN_vkDestroyImage,

        /// [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImageView)
        pub vkDestroyImageView: core::PFN_vkDestroyImageView,

        /// [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyInstance)
        pub vkDestroyInstance: core::PFN_vkDestroyInstance,

        /// [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipeline)
        pub vkDestroyPipeline: core::PFN_vkDestroyPipeline,

        /// [`vkDestroyPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineCache)
        pub vkDestroyPipelineCache: core::PFN_vkDestroyPipelineCache,

        /// [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineLayout)
        pub vkDestroyPipelineLayout: core::PFN_vkDestroyPipelineLayout,

        /// [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyQueryPool)
        pub vkDestroyQueryPool: core::PFN_vkDestroyQueryPool,

        /// [`vkDestroyRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyRenderPass)
        pub vkDestroyRenderPass: core::PFN_vkDestroyRenderPass,

        /// [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySampler)
        pub vkDestroySampler: core::PFN_vkDestroySampler,

        /// [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySemaphore)
        pub vkDestroySemaphore: core::PFN_vkDestroySemaphore,

        /// [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyShaderModule)
        pub vkDestroyShaderModule: core::PFN_vkDestroyShaderModule,

        /// [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
        pub vkDeviceWaitIdle: core::PFN_vkDeviceWaitIdle,

        /// [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
        pub vkEndCommandBuffer: core::PFN_vkEndCommandBuffer,

        /// [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
        pub vkEnumerateDeviceExtensionProperties: core::PFN_vkEnumerateDeviceExtensionProperties,

        /// [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
        pub vkEnumerateDeviceLayerProperties: core::PFN_vkEnumerateDeviceLayerProperties,

        /// [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
        pub vkEnumeratePhysicalDevices: core::PFN_vkEnumeratePhysicalDevices,

        /// [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
        pub vkFlushMappedMemoryRanges: core::PFN_vkFlushMappedMemoryRanges,

        /// [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers)
        pub vkFreeCommandBuffers: core::PFN_vkFreeCommandBuffers,

        /// [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
        pub vkFreeDescriptorSets: core::PFN_vkFreeDescriptorSets,

        /// [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeMemory)
        pub vkFreeMemory: core::PFN_vkFreeMemory,

        /// [`vkGetBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements)
        pub vkGetBufferMemoryRequirements: core::PFN_vkGetBufferMemoryRequirements,

        /// [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
        pub vkGetDeviceMemoryCommitment: core::PFN_vkGetDeviceMemoryCommitment,

        /// [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
        pub vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr,

        /// [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
        pub vkGetDeviceQueue: core::PFN_vkGetDeviceQueue,

        /// [`vkGetEventStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetEventStatus)
        pub vkGetEventStatus: core::PFN_vkGetEventStatus,

        /// [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
        pub vkGetFenceStatus: core::PFN_vkGetFenceStatus,

        /// [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
        pub vkGetImageMemoryRequirements: core::PFN_vkGetImageMemoryRequirements,

        /// [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
        pub vkGetImageSparseMemoryRequirements: core::PFN_vkGetImageSparseMemoryRequirements,

        /// [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
        pub vkGetImageSubresourceLayout: core::PFN_vkGetImageSubresourceLayout,

        /// [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
        pub vkGetPhysicalDeviceFeatures: core::PFN_vkGetPhysicalDeviceFeatures,

        /// [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
        pub vkGetPhysicalDeviceFormatProperties: core::PFN_vkGetPhysicalDeviceFormatProperties,

        /// [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
        pub vkGetPhysicalDeviceImageFormatProperties: core::PFN_vkGetPhysicalDeviceImageFormatProperties,

        /// [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
        pub vkGetPhysicalDeviceMemoryProperties: core::PFN_vkGetPhysicalDeviceMemoryProperties,

        /// [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
        pub vkGetPhysicalDeviceProperties: core::PFN_vkGetPhysicalDeviceProperties,

        /// [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
        pub vkGetPhysicalDeviceQueueFamilyProperties: core::PFN_vkGetPhysicalDeviceQueueFamilyProperties,

        /// [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
        pub vkGetPhysicalDeviceSparseImageFormatProperties: core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties,

        /// [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
        pub vkGetPipelineCacheData: core::PFN_vkGetPipelineCacheData,

        /// [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
        pub vkGetQueryPoolResults: core::PFN_vkGetQueryPoolResults,

        /// [`vkGetRenderAreaGranularity`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRenderAreaGranularity)
        pub vkGetRenderAreaGranularity: core::PFN_vkGetRenderAreaGranularity,

        /// [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
        pub vkInvalidateMappedMemoryRanges: core::PFN_vkInvalidateMappedMemoryRanges,

        /// [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
        pub vkMapMemory: core::PFN_vkMapMemory,

        /// [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
        pub vkMergePipelineCaches: core::PFN_vkMergePipelineCaches,

        /// [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
        pub vkQueueBindSparse: core::PFN_vkQueueBindSparse,

        /// [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
        pub vkQueueSubmit: core::PFN_vkQueueSubmit,

        /// [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
        pub vkQueueWaitIdle: core::PFN_vkQueueWaitIdle,

        /// [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
        pub vkResetCommandBuffer: core::PFN_vkResetCommandBuffer,

        /// [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
        pub vkResetCommandPool: core::PFN_vkResetCommandPool,

        /// [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
        pub vkResetDescriptorPool: core::PFN_vkResetDescriptorPool,

        /// [`vkResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetEvent)
        pub vkResetEvent: core::PFN_vkResetEvent,

        /// [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
        pub vkResetFences: core::PFN_vkResetFences,

        /// [`vkSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetEvent)
        pub vkSetEvent: core::PFN_vkSetEvent,

        /// [`vkUnmapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnmapMemory)
        pub vkUnmapMemory: core::PFN_vkUnmapMemory,

        /// [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
        pub vkUpdateDescriptorSets: core::PFN_vkUpdateDescriptorSets,

        /// [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
        pub vkWaitForFences: core::PFN_vkWaitForFences,
    }
);

addr_proc_struct!(
    /// [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
    pub struct AMD_draw_indirect_count {
        /// [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
        pub vkCmdDrawIndexedIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndexedIndirectCountAMD,

        /// [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
        pub vkCmdDrawIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndirectCountAMD,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_acquire_xlib_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_acquire_xlib_display)
    pub struct EXT_acquire_xlib_display {
        /// [`vkAcquireXlibDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireXlibDisplayEXT)
        pub vkAcquireXlibDisplayEXT: ext_acquire_xlib_display::PFN_vkAcquireXlibDisplayEXT,

        /// [`vkGetRandROutputDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRandROutputDisplayEXT)
        pub vkGetRandROutputDisplayEXT: ext_acquire_xlib_display::PFN_vkGetRandROutputDisplayEXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub struct EXT_debug_marker {
        /// [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
        pub vkCmdDebugMarkerBeginEXT: ext_debug_marker::PFN_vkCmdDebugMarkerBeginEXT,

        /// [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
        pub vkCmdDebugMarkerEndEXT: ext_debug_marker::PFN_vkCmdDebugMarkerEndEXT,

        /// [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
        pub vkCmdDebugMarkerInsertEXT: ext_debug_marker::PFN_vkCmdDebugMarkerInsertEXT,

        /// [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
        pub vkDebugMarkerSetObjectNameEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectNameEXT,

        /// [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
        pub vkDebugMarkerSetObjectTagEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectTagEXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub struct EXT_debug_report {
        /// [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
        pub vkCreateDebugReportCallbackEXT: ext_debug_report::PFN_vkCreateDebugReportCallbackEXT,

        /// [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
        pub vkDebugReportMessageEXT: ext_debug_report::PFN_vkDebugReportMessageEXT,

        /// [`vkDestroyDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDebugReportCallbackEXT)
        pub vkDestroyDebugReportCallbackEXT: ext_debug_report::PFN_vkDestroyDebugReportCallbackEXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_direct_mode_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_direct_mode_display)
    pub struct EXT_direct_mode_display {
        /// [`vkReleaseDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkReleaseDisplayEXT)
        pub vkReleaseDisplayEXT: ext_direct_mode_display::PFN_vkReleaseDisplayEXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    pub struct EXT_discard_rectangles {
        /// [`vkCmdSetDiscardRectangleEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDiscardRectangleEXT)
        pub vkCmdSetDiscardRectangleEXT: ext_discard_rectangles::PFN_vkCmdSetDiscardRectangleEXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    pub struct EXT_display_control {
        /// [`vkDisplayPowerControlEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDisplayPowerControlEXT)
        pub vkDisplayPowerControlEXT: ext_display_control::PFN_vkDisplayPowerControlEXT,

        /// [`vkGetSwapchainCounterEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainCounterEXT)
        pub vkGetSwapchainCounterEXT: ext_display_control::PFN_vkGetSwapchainCounterEXT,

        /// [`vkRegisterDeviceEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDeviceEventEXT)
        pub vkRegisterDeviceEventEXT: ext_display_control::PFN_vkRegisterDeviceEventEXT,

        /// [`vkRegisterDisplayEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDisplayEventEXT)
        pub vkRegisterDisplayEventEXT: ext_display_control::PFN_vkRegisterDisplayEventEXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
    pub struct EXT_display_surface_counter {
        /// [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2EXT)
        pub vkGetPhysicalDeviceSurfaceCapabilities2EXT: ext_display_surface_counter::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
    }
);

addr_proc_struct!(
    /// [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
    pub struct EXT_hdr_metadata {
        /// [`vkSetHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetHdrMetadataEXT)
        pub vkSetHdrMetadataEXT: ext_hdr_metadata::PFN_vkSetHdrMetadataEXT,
    }
);

addr_proc_struct!(
    /// [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    pub struct GOOGLE_display_timing {
        /// [`vkGetPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPastPresentationTimingGOOGLE)
        pub vkGetPastPresentationTimingGOOGLE: google_display_timing::PFN_vkGetPastPresentationTimingGOOGLE,

        /// [`vkGetRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRefreshCycleDurationGOOGLE)
        pub vkGetRefreshCycleDurationGOOGLE: google_display_timing::PFN_vkGetRefreshCycleDurationGOOGLE,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)
    pub struct KHR_android_surface {
        /// [`vkCreateAndroidSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateAndroidSurfaceKHR)
        pub vkCreateAndroidSurfaceKHR: khr_android_surface::PFN_vkCreateAndroidSurfaceKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub struct KHR_descriptor_update_template {
        /// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
        pub vkCmdPushDescriptorSetWithTemplateKHR: khr_descriptor_update_template::PFN_vkCmdPushDescriptorSetWithTemplateKHR,

        /// [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
        pub vkCreateDescriptorUpdateTemplateKHR: khr_descriptor_update_template::PFN_vkCreateDescriptorUpdateTemplateKHR,

        /// [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
        pub vkDestroyDescriptorUpdateTemplateKHR: khr_descriptor_update_template::PFN_vkDestroyDescriptorUpdateTemplateKHR,

        /// [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
        pub vkUpdateDescriptorSetWithTemplateKHR: khr_descriptor_update_template::PFN_vkUpdateDescriptorSetWithTemplateKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub struct KHR_display {
        /// [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
        pub vkCreateDisplayModeKHR: khr_display::PFN_vkCreateDisplayModeKHR,

        /// [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
        pub vkCreateDisplayPlaneSurfaceKHR: khr_display::PFN_vkCreateDisplayPlaneSurfaceKHR,

        /// [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
        pub vkGetDisplayModePropertiesKHR: khr_display::PFN_vkGetDisplayModePropertiesKHR,

        /// [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
        pub vkGetDisplayPlaneCapabilitiesKHR: khr_display::PFN_vkGetDisplayPlaneCapabilitiesKHR,

        /// [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
        pub vkGetDisplayPlaneSupportedDisplaysKHR: khr_display::PFN_vkGetDisplayPlaneSupportedDisplaysKHR,

        /// [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
        pub vkGetPhysicalDeviceDisplayPlanePropertiesKHR: khr_display::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,

        /// [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
        pub vkGetPhysicalDeviceDisplayPropertiesKHR: khr_display::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    pub struct KHR_display_swapchain {
        /// [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
        pub vkCreateSharedSwapchainsKHR: khr_display_swapchain::PFN_vkCreateSharedSwapchainsKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub struct KHR_get_physical_device_properties2 {
        /// [`vkGetPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures2KHR)
        pub vkGetPhysicalDeviceFeatures2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceFeatures2KHR,

        /// [`vkGetPhysicalDeviceFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties2KHR)
        pub vkGetPhysicalDeviceFormatProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceFormatProperties2KHR,

        /// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties2KHR)
        pub vkGetPhysicalDeviceImageFormatProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR,

        /// [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties2KHR)
        pub vkGetPhysicalDeviceMemoryProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceMemoryProperties2KHR,

        /// [`vkGetPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties2KHR)
        pub vkGetPhysicalDeviceProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceProperties2KHR,

        /// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties2KHR)
        pub vkGetPhysicalDeviceQueueFamilyProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR,

        /// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties2KHR)
        pub vkGetPhysicalDeviceSparseImageFormatProperties2KHR: khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)
    pub struct KHR_get_surface_capabilities2 {
        /// [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2KHR)
        pub vkGetPhysicalDeviceSurfaceCapabilities2KHR: khr_get_surface_capabilities2::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,

        /// [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormats2KHR)
        pub vkGetPhysicalDeviceSurfaceFormats2KHR: khr_get_surface_capabilities2::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
    pub struct KHR_maintenance1 {
        /// [`vkTrimCommandPoolKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkTrimCommandPoolKHR)
        pub vkTrimCommandPoolKHR: khr_maintenance1::PFN_vkTrimCommandPoolKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)
    pub struct KHR_mir_surface {
        /// [`vkCreateMirSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMirSurfaceKHR)
        pub vkCreateMirSurfaceKHR: khr_mir_surface::PFN_vkCreateMirSurfaceKHR,

        /// [`vkGetPhysicalDeviceMirPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMirPresentationSupportKHR)
        pub vkGetPhysicalDeviceMirPresentationSupportKHR: khr_mir_surface::PFN_vkGetPhysicalDeviceMirPresentationSupportKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
    pub struct KHR_push_descriptor {
        /// [`vkCmdPushDescriptorSetKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetKHR)
        pub vkCmdPushDescriptorSetKHR: khr_push_descriptor::PFN_vkCmdPushDescriptorSetKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_shared_presentable_image)
    pub struct KHR_shared_presentable_image {
        /// [`vkGetSwapchainStatusKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainStatusKHR)
        pub vkGetSwapchainStatusKHR: khr_shared_presentable_image::PFN_vkGetSwapchainStatusKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub struct KHR_surface {
        /// [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySurfaceKHR)
        pub vkDestroySurfaceKHR: khr_surface::PFN_vkDestroySurfaceKHR,

        /// [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
        pub vkGetPhysicalDeviceSurfaceCapabilitiesKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,

        /// [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
        pub vkGetPhysicalDeviceSurfaceFormatsKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,

        /// [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
        pub vkGetPhysicalDeviceSurfacePresentModesKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,

        /// [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
        pub vkGetPhysicalDeviceSurfaceSupportKHR: khr_surface::PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub struct KHR_swapchain {
        /// [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
        pub vkAcquireNextImageKHR: khr_swapchain::PFN_vkAcquireNextImageKHR,

        /// [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSwapchainKHR)
        pub vkCreateSwapchainKHR: khr_swapchain::PFN_vkCreateSwapchainKHR,

        /// [`vkDestroySwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySwapchainKHR)
        pub vkDestroySwapchainKHR: khr_swapchain::PFN_vkDestroySwapchainKHR,

        /// [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainImagesKHR)
        pub vkGetSwapchainImagesKHR: khr_swapchain::PFN_vkGetSwapchainImagesKHR,

        /// [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
        pub vkQueuePresentKHR: khr_swapchain::PFN_vkQueuePresentKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub struct KHR_wayland_surface {
        /// [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
        pub vkCreateWaylandSurfaceKHR: khr_wayland_surface::PFN_vkCreateWaylandSurfaceKHR,

        /// [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
        pub vkGetPhysicalDeviceWaylandPresentationSupportKHR: khr_wayland_surface::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)
    pub struct KHR_win32_surface {
        /// [`vkCreateWin32SurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWin32SurfaceKHR)
        pub vkCreateWin32SurfaceKHR: khr_win32_surface::PFN_vkCreateWin32SurfaceKHR,

        /// [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWin32PresentationSupportKHR)
        pub vkGetPhysicalDeviceWin32PresentationSupportKHR: khr_win32_surface::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub struct KHR_xcb_surface {
        /// [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
        pub vkCreateXcbSurfaceKHR: khr_xcb_surface::PFN_vkCreateXcbSurfaceKHR,

        /// [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
        pub vkGetPhysicalDeviceXcbPresentationSupportKHR: khr_xcb_surface::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
    }
);

addr_proc_struct!(
    /// [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    pub struct KHR_xlib_surface {
        /// [`vkCreateXlibSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXlibSurfaceKHR)
        pub vkCreateXlibSurfaceKHR: khr_xlib_surface::PFN_vkCreateXlibSurfaceKHR,

        /// [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXlibPresentationSupportKHR)
        pub vkGetPhysicalDeviceXlibPresentationSupportKHR: khr_xlib_surface::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
    }
);

addr_proc_struct!(
    /// [`VK_MVK_ios_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_ios_surface)
    pub struct MVK_ios_surface {
        /// [`vkCreateIOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIOSSurfaceMVK)
        pub vkCreateIOSSurfaceMVK: mvk_ios_surface::PFN_vkCreateIOSSurfaceMVK,
    }
);

addr_proc_struct!(
    /// [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
    pub struct MVK_macos_surface {
        /// [`vkCreateMacOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMacOSSurfaceMVK)
        pub vkCreateMacOSSurfaceMVK: mvk_macos_surface::PFN_vkCreateMacOSSurfaceMVK,
    }
);

addr_proc_struct!(
    /// [`VK_NN_vi_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NN_vi_surface)
    pub struct NN_vi_surface {
        /// [`vkCreateViSurfaceNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateViSurfaceNN)
        pub vkCreateViSurfaceNN: nn_vi_surface::PFN_vkCreateViSurfaceNN,
    }
);

addr_proc_struct!(
    /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    pub struct NV_clip_space_w_scaling {
        /// [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
        pub vkCmdSetViewportWScalingNV: nv_clip_space_w_scaling::PFN_vkCmdSetViewportWScalingNV,
    }
);

addr_proc_struct!(
    /// [`VK_NV_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_capabilities)
    pub struct NV_external_memory_capabilities {
        /// [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalImageFormatPropertiesNV)
        pub vkGetPhysicalDeviceExternalImageFormatPropertiesNV: nv_external_memory_capabilities::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
    }
);

addr_proc_struct!(
    /// [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    pub struct NV_external_memory_win32 {
        /// [`vkGetMemoryWin32HandleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleNV)
        pub vkGetMemoryWin32HandleNV: nv_external_memory_win32::PFN_vkGetMemoryWin32HandleNV,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub struct KHX_device_group {
        /// [`vkAcquireNextImage2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImage2KHX)
        pub vkAcquireNextImage2KHX: khx_device_group::PFN_vkAcquireNextImage2KHX,

        /// [`vkBindBufferMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory2KHX)
        pub vkBindBufferMemory2KHX: khx_device_group::PFN_vkBindBufferMemory2KHX,

        /// [`vkBindImageMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory2KHX)
        pub vkBindImageMemory2KHX: khx_device_group::PFN_vkBindImageMemory2KHX,

        /// [`vkCmdDispatchBaseKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchBaseKHX)
        pub vkCmdDispatchBaseKHX: khx_device_group::PFN_vkCmdDispatchBaseKHX,

        /// [`vkCmdSetDeviceMaskKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDeviceMaskKHX)
        pub vkCmdSetDeviceMaskKHX: khx_device_group::PFN_vkCmdSetDeviceMaskKHX,

        /// [`vkGetDeviceGroupPeerMemoryFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPeerMemoryFeaturesKHX)
        pub vkGetDeviceGroupPeerMemoryFeaturesKHX: khx_device_group::PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX,

        /// [`vkGetDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPresentCapabilitiesKHX)
        pub vkGetDeviceGroupPresentCapabilitiesKHX: khx_device_group::PFN_vkGetDeviceGroupPresentCapabilitiesKHX,

        /// [`vkGetDeviceGroupSurfacePresentModesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupSurfacePresentModesKHX)
        pub vkGetDeviceGroupSurfacePresentModesKHX: khx_device_group::PFN_vkGetDeviceGroupSurfacePresentModesKHX,

        /// [`vkGetPhysicalDevicePresentRectanglesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDevicePresentRectanglesKHX)
        pub vkGetPhysicalDevicePresentRectanglesKHX: khx_device_group::PFN_vkGetPhysicalDevicePresentRectanglesKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)
    pub struct KHX_device_group_creation {
        /// [`vkEnumeratePhysicalDeviceGroupsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDeviceGroupsKHX)
        pub vkEnumeratePhysicalDeviceGroupsKHX: khx_device_group_creation::PFN_vkEnumeratePhysicalDeviceGroupsKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
    pub struct KHX_external_memory_capabilities {
        /// [`vkGetPhysicalDeviceExternalBufferPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalBufferPropertiesKHX)
        pub vkGetPhysicalDeviceExternalBufferPropertiesKHX: khx_external_memory_capabilities::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
    pub struct KHX_external_memory_fd {
        /// [`vkGetMemoryFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHX)
        pub vkGetMemoryFdKHX: khx_external_memory_fd::PFN_vkGetMemoryFdKHX,

        /// [`vkGetMemoryFdPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHX)
        pub vkGetMemoryFdPropertiesKHX: khx_external_memory_fd::PFN_vkGetMemoryFdPropertiesKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
    pub struct KHX_external_memory_win32 {
        /// [`vkGetMemoryWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHX)
        pub vkGetMemoryWin32HandleKHX: khx_external_memory_win32::PFN_vkGetMemoryWin32HandleKHX,

        /// [`vkGetMemoryWin32HandlePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHX)
        pub vkGetMemoryWin32HandlePropertiesKHX: khx_external_memory_win32::PFN_vkGetMemoryWin32HandlePropertiesKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
    pub struct KHX_external_semaphore_capabilities {
        /// [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHX)
        pub vkGetPhysicalDeviceExternalSemaphorePropertiesKHX: khx_external_semaphore_capabilities::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
    pub struct KHX_external_semaphore_win32 {
        /// [`vkGetSemaphoreWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHX)
        pub vkGetSemaphoreWin32HandleKHX: khx_external_semaphore_win32::PFN_vkGetSemaphoreWin32HandleKHX,

        /// [`vkImportSemaphoreWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportSemaphoreWin32HandleKHX)
        pub vkImportSemaphoreWin32HandleKHX: khx_external_semaphore_win32::PFN_vkImportSemaphoreWin32HandleKHX,
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub struct NVX_device_generated_commands {
        /// [`vkCmdProcessCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdProcessCommandsNVX)
        pub vkCmdProcessCommandsNVX: nvx_device_generated_commands::PFN_vkCmdProcessCommandsNVX,

        /// [`vkCmdReserveSpaceForCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdReserveSpaceForCommandsNVX)
        pub vkCmdReserveSpaceForCommandsNVX: nvx_device_generated_commands::PFN_vkCmdReserveSpaceForCommandsNVX,

        /// [`vkCreateIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIndirectCommandsLayoutNVX)
        pub vkCreateIndirectCommandsLayoutNVX: nvx_device_generated_commands::PFN_vkCreateIndirectCommandsLayoutNVX,

        /// [`vkCreateObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateObjectTableNVX)
        pub vkCreateObjectTableNVX: nvx_device_generated_commands::PFN_vkCreateObjectTableNVX,

        /// [`vkDestroyIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyIndirectCommandsLayoutNVX)
        pub vkDestroyIndirectCommandsLayoutNVX: nvx_device_generated_commands::PFN_vkDestroyIndirectCommandsLayoutNVX,

        /// [`vkDestroyObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyObjectTableNVX)
        pub vkDestroyObjectTableNVX: nvx_device_generated_commands::PFN_vkDestroyObjectTableNVX,

        /// [`vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)
        pub vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX: nvx_device_generated_commands::PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX,

        /// [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
        pub vkRegisterObjectsNVX: nvx_device_generated_commands::PFN_vkRegisterObjectsNVX,

        /// [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
        pub vkUnregisterObjectsNVX: nvx_device_generated_commands::PFN_vkUnregisterObjectsNVX,
    }
);
