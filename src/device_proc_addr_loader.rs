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
use ext_debug_marker;
use ext_discard_rectangles;
use ext_display_control;
use ext_hdr_metadata;
use google_display_timing;
use khr_descriptor_update_template;
use khr_display_swapchain;
use khr_maintenance1;
use khr_push_descriptor;
use khr_shared_presentable_image;
use khr_swapchain;
use libc::{c_char, c_void};
use nv_clip_space_w_scaling;
use nv_external_memory_win32;
use std::fmt;
use std::mem;
use std::ptr;

#[cfg(feature = "experimental")]
use experimental::*;

macro_rules! gen_device_proc_addr_loader {
    (
        $( #[$attr:meta] )*
        pub struct DeviceProcAddrLoader {
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
        pub struct DeviceProcAddrLoader {
            pub vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr,

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

        impl Copy for DeviceProcAddrLoader { }

        impl Clone for DeviceProcAddrLoader {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl fmt::Debug for DeviceProcAddrLoader {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut debug_struct = f.debug_struct("DeviceProcAddrLoader");
                debug_struct.field("vkGetDeviceProcAddr", &(self.vkGetDeviceProcAddr as *mut c_void));

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

        impl Default for DeviceProcAddrLoader {
            fn default() -> Self {
                DeviceProcAddrLoader::new()
            }
        }

        impl DeviceProcAddrLoader {
            pub fn new() -> Self {
                unsafe {
                    DeviceProcAddrLoader::from_get_device_proc_addr(mem::transmute(0usize))
                }
            }

            pub fn from_get_device_proc_addr(vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr) -> Self {
                unsafe {
                    let mut res: DeviceProcAddrLoader = mem::uninitialized();

                    ptr::write(&mut res.vkGetDeviceProcAddr, vkGetDeviceProcAddr);

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

            $(
                pub unsafe fn $load(&mut self, device: core::VkDevice) {
                    self.$field.load(self.vkGetDeviceProcAddr, device);
                }
            )*

            $(
                #[cfg(feature = "experimental")]
                pub unsafe fn $exp_load(&mut self, device: core::VkDevice) {
                    self.$exp_field.load(self.vkGetDeviceProcAddr, device);
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
            pub unsafe fn load(&mut self, vkGetDeviceProcAddr: ::core::PFN_vkGetDeviceProcAddr, device: ::core::VkDevice) {
                $(
                    self.$symbol = mem::transmute((vkGetDeviceProcAddr)(device, concat!(stringify!($symbol), '\x00').as_ptr() as *const c_char));
                )*
            }
        }
    )
}

gen_device_proc_addr_loader!(
    pub struct DeviceProcAddrLoader {
        /// [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)
        pub core: Core [fn load_core],

        /// [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
        pub amd_draw_indirect_count: AMD_draw_indirect_count [fn load_amd_draw_indirect_count],

        /// [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
        pub ext_debug_marker: EXT_debug_marker [fn load_ext_debug_marker],

        /// [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
        pub ext_discard_rectangles: EXT_discard_rectangles [fn load_ext_discard_rectangles],

        /// [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
        pub ext_display_control: EXT_display_control [fn load_ext_display_control],

        /// [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
        pub ext_hdr_metadata: EXT_hdr_metadata [fn load_ext_hdr_metadata],

        /// [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
        pub google_display_timing: GOOGLE_display_timing [fn load_google_display_timing],

        /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
        pub khr_descriptor_update_template: KHR_descriptor_update_template [fn load_khr_descriptor_update_template],

        /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
        pub khr_display_swapchain: KHR_display_swapchain [fn load_khr_display_swapchain],

        /// [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        pub khr_maintenance1: KHR_maintenance1 [fn load_khr_maintenance1],

        /// [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
        pub khr_push_descriptor: KHR_push_descriptor [fn load_khr_push_descriptor],

        /// [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_shared_presentable_image)
        pub khr_shared_presentable_image: KHR_shared_presentable_image [fn load_khr_shared_presentable_image],

        /// [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        pub khr_swapchain: KHR_swapchain [fn load_khr_swapchain],

        /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
        pub nv_clip_space_w_scaling: NV_clip_space_w_scaling [fn load_nv_clip_space_w_scaling],

        /// [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
        pub nv_external_memory_win32: NV_external_memory_win32 [fn load_nv_external_memory_win32],

        experimental {
            /// [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
            pub khx_device_group: KHX_device_group [fn load_khx_device_group],

            /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
            pub nvx_device_generated_commands: NVX_device_generated_commands [fn load_nvx_device_generated_commands],
        }
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
    /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    pub struct KHR_display_swapchain {
        /// [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
        pub vkCreateSharedSwapchainsKHR: khr_display_swapchain::PFN_vkCreateSharedSwapchainsKHR,
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
    /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    pub struct NV_clip_space_w_scaling {
        /// [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
        pub vkCmdSetViewportWScalingNV: nv_clip_space_w_scaling::PFN_vkCmdSetViewportWScalingNV,
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

        /// [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
        pub vkRegisterObjectsNVX: nvx_device_generated_commands::PFN_vkRegisterObjectsNVX,

        /// [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
        pub vkUnregisterObjectsNVX: nvx_device_generated_commands::PFN_vkUnregisterObjectsNVX,
    }
);
