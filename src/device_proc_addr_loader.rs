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
use ext_display_surface_counter;
use ext_hdr_metadata;
use google_display_timing;
use khr_descriptor_update_template;
use khr_display;
use khr_display_swapchain;
use khr_external_fence_win32;
use khr_external_memory_capabilities;
use khr_external_memory_fd;
use khr_external_memory_win32;
use khr_external_semaphore_fd;
use khr_external_semaphore_win32;
use khr_maintenance1;
use khr_push_descriptor;
use khr_shared_presentable_image;
use khr_swapchain;
use libc::{c_char, c_int, c_void};
use nv_clip_space_w_scaling;
use nv_external_memory_capabilities;
use nv_external_memory_win32;
use std::fmt;
use std::mem;
use win32_types;

#[cfg(feature = "experimental")]
use experimental::*;

#[cfg(feature = "experimental")]
use khr_surface;

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
            pub pfn_vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr,

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

                let pfn_vkGetDeviceProcAddr = self.pfn_vkGetDeviceProcAddr.map(|pfn_vkGetDeviceProcAddr| pfn_vkGetDeviceProcAddr as *mut c_void);
                debug_struct.field("vkGetDeviceProcAddr", &pfn_vkGetDeviceProcAddr);

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

        impl Default for DeviceProcAddrLoader {
            fn default() -> Self {
                DeviceProcAddrLoader::new()
            }
        }

        impl DeviceProcAddrLoader {
            pub fn new() -> Self {
                DeviceProcAddrLoader::from_get_device_proc_addr(None)
            }

            pub fn from_get_device_proc_addr(pfn_vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr) -> Self {
                DeviceProcAddrLoader {
                    pfn_vkGetDeviceProcAddr: pfn_vkGetDeviceProcAddr,
                    $( $field: $ty::new(), )*
                    $(
                        #[cfg(feature = "experimental")]
                        $exp_field: $exp_ty::new(),
                    )*
                    guard: (),
                }
            }

            /// [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
            #[inline]
            pub unsafe fn vkGetDeviceProcAddr(&self, device: core::VkDevice, pName: *const c_char) -> core::PFN_vkVoidFunction {
                let pfn_vkGetDeviceProcAddr = self.pfn_vkGetDeviceProcAddr.expect("pfn_vkGetDeviceProcAddr is None");
                (pfn_vkGetDeviceProcAddr)(device, pName)
            }

            $(
                pub unsafe fn $load(&mut self, device: core::VkDevice) {
                    self.$field.load(self.pfn_vkGetDeviceProcAddr, device);
                }
            )*

            $(
                #[cfg(feature = "experimental")]
                pub unsafe fn $exp_load(&mut self, device: core::VkDevice) {
                    self.$exp_field.load(self.pfn_vkGetDeviceProcAddr, device);
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
            pub unsafe fn load(&mut self, pfn_vkGetDeviceProcAddr: core::PFN_vkGetDeviceProcAddr, device: core::VkDevice) {
                let pfn_vkGetDeviceProcAddr = pfn_vkGetDeviceProcAddr.expect("pfn_vkGetDeviceProcAddr is None");
                $(
                    self.$symbol = (pfn_vkGetDeviceProcAddr)(device, concat!(stringify!($fn), '\x00').as_ptr() as *const c_char)
                    .map(|$symbol| mem::transmute($symbol));
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

        /// [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
        pub khr_external_fence_win32: KHR_external_fence_win32 [fn load_khr_external_fence_win32],

        /// [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
        pub khr_external_memory_fd: KHR_external_memory_fd [fn load_khr_external_memory_fd],

        /// [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
        pub khr_external_memory_win32: KHR_external_memory_win32 [fn load_khr_external_memory_win32],

        /// [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
        pub khr_external_semaphore_fd: KHR_external_semaphore_fd [fn load_khr_external_semaphore_fd],

        /// [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
        pub khr_external_semaphore_win32: KHR_external_semaphore_win32 [fn load_khr_external_semaphore_win32],

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
    pub struct AMD_draw_indirect_count {
        /// [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
        pub fn vkCmdDrawIndexedIndirectCountAMD(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, countBuffer: core::VkBuffer, countBufferOffset: core::VkDeviceSize, maxDrawCount: u32, stride: u32); [pfn_vkCmdDrawIndexedIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndexedIndirectCountAMD],

        /// [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
        pub fn vkCmdDrawIndirectCountAMD(commandBuffer: core::VkCommandBuffer, buffer: core::VkBuffer, offset: core::VkDeviceSize, countBuffer: core::VkBuffer, countBufferOffset: core::VkDeviceSize, maxDrawCount: u32, stride: u32); [pfn_vkCmdDrawIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndirectCountAMD],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub struct EXT_debug_marker {
        /// [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
        pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *mut ext_debug_marker::VkDebugMarkerMarkerInfoEXT); [pfn_vkCmdDebugMarkerBeginEXT: ext_debug_marker::PFN_vkCmdDebugMarkerBeginEXT],

        /// [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
        pub fn vkCmdDebugMarkerEndEXT(commandBuffer: core::VkCommandBuffer); [pfn_vkCmdDebugMarkerEndEXT: ext_debug_marker::PFN_vkCmdDebugMarkerEndEXT],

        /// [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
        pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *mut ext_debug_marker::VkDebugMarkerMarkerInfoEXT); [pfn_vkCmdDebugMarkerInsertEXT: ext_debug_marker::PFN_vkCmdDebugMarkerInsertEXT],

        /// [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
        pub fn vkDebugMarkerSetObjectNameEXT(device: core::VkDevice, pNameInfo: *mut ext_debug_marker::VkDebugMarkerObjectNameInfoEXT) -> core::VkResult; [pfn_vkDebugMarkerSetObjectNameEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectNameEXT],

        /// [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
        pub fn vkDebugMarkerSetObjectTagEXT(device: core::VkDevice, pTagInfo: *mut ext_debug_marker::VkDebugMarkerObjectTagInfoEXT) -> core::VkResult; [pfn_vkDebugMarkerSetObjectTagEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectTagEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    pub struct EXT_discard_rectangles {
        /// [`vkCmdSetDiscardRectangleEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDiscardRectangleEXT)
        pub fn vkCmdSetDiscardRectangleEXT(commandBuffer: core::VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const core::VkRect2D); [pfn_vkCmdSetDiscardRectangleEXT: ext_discard_rectangles::PFN_vkCmdSetDiscardRectangleEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    pub struct EXT_display_control {
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
    /// [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
    pub struct EXT_hdr_metadata {
        /// [`vkSetHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetHdrMetadataEXT)
        pub fn vkSetHdrMetadataEXT(device: core::VkDevice, swapchainCount: u32, pSwapchains: *const khr_swapchain::VkSwapchainKHR, pMetadata: *const ext_hdr_metadata::VkHdrMetadataEXT); [pfn_vkSetHdrMetadataEXT: ext_hdr_metadata::PFN_vkSetHdrMetadataEXT],
    }
);

addr_proc_struct!(
    /// [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    pub struct GOOGLE_display_timing {
        /// [`vkGetPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPastPresentationTimingGOOGLE)
        pub fn vkGetPastPresentationTimingGOOGLE(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut google_display_timing::VkPastPresentationTimingGOOGLE) -> core::VkResult; [pfn_vkGetPastPresentationTimingGOOGLE: google_display_timing::PFN_vkGetPastPresentationTimingGOOGLE],

        /// [`vkGetRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRefreshCycleDurationGOOGLE)
        pub fn vkGetRefreshCycleDurationGOOGLE(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pDisplayTimingProperties: *mut google_display_timing::VkRefreshCycleDurationGOOGLE) -> core::VkResult; [pfn_vkGetRefreshCycleDurationGOOGLE: google_display_timing::PFN_vkGetRefreshCycleDurationGOOGLE],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub struct KHR_descriptor_update_template {
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
    /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    pub struct KHR_display_swapchain {
        /// [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
        pub fn vkCreateSharedSwapchainsKHR(device: core::VkDevice, swapchainCount: u32, pCreateInfos: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSwapchains: *mut khr_swapchain::VkSwapchainKHR) -> core::VkResult; [pfn_vkCreateSharedSwapchainsKHR: khr_display_swapchain::PFN_vkCreateSharedSwapchainsKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
    pub struct KHR_external_fence_win32 {
        /// See [`vkImportFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceWin32HandleKHR)
        pub fn vkImportFenceWin32HandleKHR(device: core::VkDevice, pImportFenceWin32HandleInfo: *const khr_external_fence_win32::VkImportFenceWin32HandleInfoKHR) -> core::VkResult; [pfn_vkImportFenceWin32HandleKHR: khr_external_fence_win32::PFN_vkImportFenceWin32HandleKHR],

        /// See [`vkGetFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceWin32HandleKHR)
        pub fn vkGetFenceWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const khr_external_fence_win32::VkFenceGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetFenceWin32HandleKHR: khr_external_fence_win32::PFN_vkGetFenceWin32HandleKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
    pub struct KHR_external_memory_fd {
        /// See [`vkGetMemoryFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHR)
        pub fn vkGetMemoryFdKHR(device: core::VkDevice, pGetFdInfo: *const khr_external_memory_fd::VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult; [pfn_vkGetMemoryFdKHR: khr_external_memory_fd::PFN_vkGetMemoryFdKHR],

        /// See [`vkGetMemoryFdPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHR)
        pub fn vkGetMemoryFdPropertiesKHR(device: core::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, fd: c_int, pMemoryFdProperties: *mut khr_external_memory_fd::VkMemoryFdPropertiesKHR) -> core::VkResult; [pfn_vkGetMemoryFdPropertiesKHR: khr_external_memory_fd::PFN_vkGetMemoryFdPropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
    pub struct KHR_external_memory_win32 {
        /// See [`vkGetMemoryWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHR)
        pub fn vkGetMemoryWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const khr_external_memory_win32::VkMemoryGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetMemoryWin32HandleKHR: khr_external_memory_win32::PFN_vkGetMemoryWin32HandleKHR],

        /// See [`vkGetMemoryWin32HandlePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHR)
        pub fn vkGetMemoryWin32HandlePropertiesKHR(device: core::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, handle: win32_types::HANDLE, pMemoryWin32HandleProperties: *mut khr_external_memory_win32::VkMemoryWin32HandlePropertiesKHR) -> core::VkResult; [pfn_vkGetMemoryWin32HandlePropertiesKHR: khr_external_memory_win32::PFN_vkGetMemoryWin32HandlePropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
    pub struct KHR_external_semaphore_fd {
        /// See [`VkImportSemaphoreFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHR)
        pub fn vkImportSemaphoreFdKHR(device: core::VkDevice, pImportSemaphoreFdInfo: *const khr_external_semaphore_fd::VkImportSemaphoreFdInfoKHR) -> core::VkResult; [pfn_vkImportSemaphoreFdKHR: khr_external_semaphore_fd::PFN_vkImportSemaphoreFdKHR],

        /// See [`vkGetSemaphoreFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHR)
        pub fn vkGetSemaphoreFdKHR(device: core::VkDevice, pGetFdInfo: *const khr_external_semaphore_fd::VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult; [pfn_vkGetSemaphoreFdKHR: khr_external_semaphore_fd::PFN_vkGetSemaphoreFdKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
    pub struct KHR_external_semaphore_win32 {
        /// See [`VkImportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHR)
        pub fn vkImportSemaphoreWin32HandleKHR(device: core::VkDevice, pImportSemaphoreWin32HandleInfo: *const khr_external_semaphore_win32::VkImportSemaphoreWin32HandleInfoKHR) -> core::VkResult; [pfn_vkImportSemaphoreWin32HandleKHR: khr_external_semaphore_win32::PFN_vkImportSemaphoreWin32HandleKHR],

        /// See [`vkGetSemaphoreWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHR)
        pub fn vkGetSemaphoreWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const khr_external_semaphore_win32::VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetSemaphoreWin32HandleKHR: khr_external_semaphore_win32::PFN_vkGetSemaphoreWin32HandleKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
    pub struct KHR_maintenance1 {
        /// [`vkTrimCommandPoolKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkTrimCommandPoolKHR)
        pub fn vkTrimCommandPoolKHR(device: core::VkDevice, commandPool: core::VkCommandPool, flags: khr_maintenance1::VkCommandPoolTrimFlagsKHR); [pfn_vkTrimCommandPoolKHR: khr_maintenance1::PFN_vkTrimCommandPoolKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
    pub struct KHR_push_descriptor {
        /// [`vkCmdPushDescriptorSetKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetKHR)
        pub fn vkCmdPushDescriptorSetKHR(commandBuffer: core::VkCommandBuffer, pipelineBindPoint: core::VkPipelineBindPoint, layout: core::VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const core::VkWriteDescriptorSet); [pfn_vkCmdPushDescriptorSetKHR: khr_push_descriptor::PFN_vkCmdPushDescriptorSetKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_shared_presentable_image)
    pub struct KHR_shared_presentable_image {
        /// [`vkGetSwapchainStatusKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainStatusKHR)
        pub fn vkGetSwapchainStatusKHR(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR) -> core::VkResult; [pfn_vkGetSwapchainStatusKHR: khr_shared_presentable_image::PFN_vkGetSwapchainStatusKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub struct KHR_swapchain {
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
    /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    pub struct NV_clip_space_w_scaling {
        /// [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
        pub fn vkCmdSetViewportWScalingNV(commandBuffer: core::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const nv_clip_space_w_scaling::VkViewportWScalingNV); [pfn_vkCmdSetViewportWScalingNV: nv_clip_space_w_scaling::PFN_vkCmdSetViewportWScalingNV],
    }
);

addr_proc_struct!(
    /// [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    pub struct NV_external_memory_win32 {
        /// [`vkGetMemoryWin32HandleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleNV)
        pub fn vkGetMemoryWin32HandleNV(device: core::VkDevice, memory: core::VkDeviceMemory, handleType: nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_types::HANDLE) -> core::VkResult; [pfn_vkGetMemoryWin32HandleNV: nv_external_memory_win32::PFN_vkGetMemoryWin32HandleNV],
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub struct KHX_device_group {
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
    }
);

#[cfg(feature = "experimental")]
addr_proc_struct!(
    /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub struct NVX_device_generated_commands {
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

        /// [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
        pub fn vkRegisterObjectsNVX(device: core::VkDevice, objectTable: nvx_device_generated_commands::VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const nvx_device_generated_commands::VkObjectTableEntryNVX, pObjectIndices: *const u32) -> core::VkResult; [pfn_vkRegisterObjectsNVX: nvx_device_generated_commands::PFN_vkRegisterObjectsNVX],

        /// [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
        pub fn vkUnregisterObjectsNVX(device: core::VkDevice, objectTable: nvx_device_generated_commands::VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const nvx_device_generated_commands::VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> core::VkResult; [pfn_vkUnregisterObjectsNVX: nvx_device_generated_commands::PFN_vkUnregisterObjectsNVX],
    }
);
