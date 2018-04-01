// Copyright (c) 2018, Dennis Hamester <dennis.hamester@startmail.com>
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
use core::fmt;
use core::mem;
use ext_debug_marker;
use ext_discard_rectangles;
use ext_display_control;
use ext_display_surface_counter;
use ext_hdr_metadata;
use google_display_timing;
use khr_descriptor_update_template;
use khr_display;
use khr_display_swapchain;
use khr_external_fence_fd;
use khr_external_fence_win32;
use khr_external_memory_capabilities;
use khr_external_memory_fd;
use khr_external_memory_win32;
use khr_external_semaphore_fd;
use khr_external_semaphore_win32;
use khr_get_memory_requirements2;
use khr_maintenance1;
use khr_push_descriptor;
use khr_shared_presentable_image;
use khr_swapchain;
use libc::{c_char, c_int, c_void};
use nv_clip_space_w_scaling;
use nv_external_memory_capabilities;
use nv_external_memory_win32;
use vk;
use win32_types;

macro_rules! gen_device_proc_addr_loader {
    (
        $( #[$attr:meta] )*
        pub struct DeviceProcAddrLoader {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident: $ty:ident [fn $load:ident],
            )*
        }
    ) => {
        $( #[$attr] )*
        pub struct DeviceProcAddrLoader {
            pub pfn_vkGetDeviceProcAddr: vk::PFN_vkGetDeviceProcAddr,

            $(
                $( #[$field_attr] )*
                pub $field: $ty,
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

            pub fn from_get_device_proc_addr(pfn_vkGetDeviceProcAddr: vk::PFN_vkGetDeviceProcAddr) -> Self {
                DeviceProcAddrLoader {
                    pfn_vkGetDeviceProcAddr: pfn_vkGetDeviceProcAddr,
                    $( $field: $ty::new(), )*
                    guard: (),
                }
            }

            /// [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
            #[inline]
            pub unsafe fn vkGetDeviceProcAddr(&self, device: vk::VkDevice, pName: *const c_char) -> vk::PFN_vkVoidFunction {
                let pfn_vkGetDeviceProcAddr = self.pfn_vkGetDeviceProcAddr.expect("pfn_vkGetDeviceProcAddr is None");
                (pfn_vkGetDeviceProcAddr)(device, pName)
            }

            $(
                pub unsafe fn $load(&mut self, device: vk::VkDevice) {
                    self.$field.load(self.pfn_vkGetDeviceProcAddr, device);
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
            pub unsafe fn load(&mut self, pfn_vkGetDeviceProcAddr: vk::PFN_vkGetDeviceProcAddr, device: vk::VkDevice) {
                let pfn_vkGetDeviceProcAddr = pfn_vkGetDeviceProcAddr.expect("pfn_vkGetDeviceProcAddr is None");
                $(
                    self.$symbol = (pfn_vkGetDeviceProcAddr)(device, concat!(stringify!($fn), '\x00').as_ptr() as *const c_char)
                    .map(|$symbol| mem::transmute($symbol));
                )*
            }
        }

        $(
            impl DeviceProcAddrLoader {
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

gen_device_proc_addr_loader!(
    pub struct DeviceProcAddrLoader {
        /// [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)
        pub vk: Vk [fn load_vk],

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

        /// [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)
        pub khr_external_fence_fd: KHR_external_fence_fd [fn load_khr_external_fence_fd],

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

        /// [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        pub khr_get_memory_requirements2: KHR_get_memory_requirements2 [fn load_khr_get_memory_requirements2],

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
    }
);

addr_proc_struct!(
    /// [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)
    pub struct Vk [vk] {
        /// [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
        pub fn vkAllocateCommandBuffers(device: vk::VkDevice, pAllocateInfo: *const vk::VkCommandBufferAllocateInfo, pCommandBuffers: *mut vk::VkCommandBuffer) -> vk::VkResult; [pfn_vkAllocateCommandBuffers: vk::PFN_vkAllocateCommandBuffers],

        /// [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
        pub fn vkAllocateDescriptorSets(device: vk::VkDevice, pAllocateInfo: *const vk::VkDescriptorSetAllocateInfo, pDescriptorSets: *mut vk::VkDescriptorSet) -> vk::VkResult; [pfn_vkAllocateDescriptorSets: vk::PFN_vkAllocateDescriptorSets],

        /// [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
        pub fn vkAllocateMemory(device: vk::VkDevice, pAllocateInfo: *const vk::VkMemoryAllocateInfo, pAllocator: *const vk::VkAllocationCallbacks, pMemory: *mut vk::VkDeviceMemory) -> vk::VkResult; [pfn_vkAllocateMemory: vk::PFN_vkAllocateMemory],

        /// [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
        pub fn vkBeginCommandBuffer(commandBuffer: vk::VkCommandBuffer, pBeginInfo: *const vk::VkCommandBufferBeginInfo) -> vk::VkResult; [pfn_vkBeginCommandBuffer: vk::PFN_vkBeginCommandBuffer],

        /// [`vkBindBufferMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory)
        pub fn vkBindBufferMemory(device: vk::VkDevice, buffer: vk::VkBuffer, memory: vk::VkDeviceMemory, memoryOffset: vk::VkDeviceSize) -> vk::VkResult; [pfn_vkBindBufferMemory: vk::PFN_vkBindBufferMemory],

        /// [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
        pub fn vkBindImageMemory(device: vk::VkDevice, image: vk::VkImage, memory: vk::VkDeviceMemory, memoryOffset: vk::VkDeviceSize) -> vk::VkResult; [pfn_vkBindImageMemory: vk::PFN_vkBindImageMemory],

        /// [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
        pub fn vkCmdBeginQuery(commandBuffer: vk::VkCommandBuffer, queryPool: vk::VkQueryPool, query: u32, flags: vk::VkQueryControlFlags); [pfn_vkCmdBeginQuery: vk::PFN_vkCmdBeginQuery],

        /// [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
        pub fn vkCmdBeginRenderPass(commandBuffer: vk::VkCommandBuffer, pRenderPassBegin: *const vk::VkRenderPassBeginInfo, contents: vk::VkSubpassContents); [pfn_vkCmdBeginRenderPass: vk::PFN_vkCmdBeginRenderPass],

        /// [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
        pub fn vkCmdBindDescriptorSets(commandBuffer: vk::VkCommandBuffer, pipelineBindPoint: vk::VkPipelineBindPoint, layout: vk::VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const vk::VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32); [pfn_vkCmdBindDescriptorSets: vk::PFN_vkCmdBindDescriptorSets],

        /// [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
        pub fn vkCmdBindIndexBuffer(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, indexType: vk::VkIndexType); [pfn_vkCmdBindIndexBuffer: vk::PFN_vkCmdBindIndexBuffer],

        /// [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
        pub fn vkCmdBindPipeline(commandBuffer: vk::VkCommandBuffer, pipelineBindPoint: vk::VkPipelineBindPoint, pipeline: vk::VkPipeline); [pfn_vkCmdBindPipeline: vk::PFN_vkCmdBindPipeline],

        /// [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
        pub fn vkCmdBindVertexBuffers(commandBuffer: vk::VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const vk::VkBuffer, pOffsets: *const vk::VkDeviceSize); [pfn_vkCmdBindVertexBuffers: vk::PFN_vkCmdBindVertexBuffers],

        /// [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
        pub fn vkCmdBlitImage(commandBuffer: vk::VkCommandBuffer, srcImage: vk::VkImage, srcImageLayout: vk::VkImageLayout, dstImage: vk::VkImage, dstImageLayout: vk::VkImageLayout, regionCount: u32, pRegions: *const vk::VkImageBlit, filter: vk::VkFilter); [pfn_vkCmdBlitImage: vk::PFN_vkCmdBlitImage],

        /// [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
        pub fn vkCmdClearAttachments(commandBuffer: vk::VkCommandBuffer, attachmentCount: u32, pAttachments: *const vk::VkClearAttachment, rectCount: u32, pRects: *const vk::VkClearRect); [pfn_vkCmdClearAttachments: vk::PFN_vkCmdClearAttachments],

        /// [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
        pub fn vkCmdClearColorImage(commandBuffer: vk::VkCommandBuffer, image: vk::VkImage, imageLayout: vk::VkImageLayout, pColor: *const vk::VkClearColorValue, rangeCount: u32, pRanges: *const vk::VkImageSubresourceRange); [pfn_vkCmdClearColorImage: vk::PFN_vkCmdClearColorImage],

        /// [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
        pub fn vkCmdClearDepthStencilImage(commandBuffer: vk::VkCommandBuffer, image: vk::VkImage, imageLayout: vk::VkImageLayout, pDepthStencil: *const vk::VkClearDepthStencilValue, rangeCount: u32, pRanges: *const vk::VkImageSubresourceRange); [pfn_vkCmdClearDepthStencilImage: vk::PFN_vkCmdClearDepthStencilImage],

        /// [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
        pub fn vkCmdCopyBuffer(commandBuffer: vk::VkCommandBuffer, srcBuffer: vk::VkBuffer, dstBuffer: vk::VkBuffer, regionCount: u32, pRegions: *const vk::VkBufferCopy); [pfn_vkCmdCopyBuffer: vk::PFN_vkCmdCopyBuffer],

        /// [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
        pub fn vkCmdCopyBufferToImage(commandBuffer: vk::VkCommandBuffer, srcBuffer: vk::VkBuffer, dstImage: vk::VkImage, dstImageLayout: vk::VkImageLayout, regionCount: u32, pRegions: *const vk::VkBufferImageCopy); [pfn_vkCmdCopyBufferToImage: vk::PFN_vkCmdCopyBufferToImage],

        /// [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
        pub fn vkCmdCopyImage(commandBuffer: vk::VkCommandBuffer, srcImage: vk::VkImage, srcImageLayout: vk::VkImageLayout, dstImage: vk::VkImage, dstImageLayout: vk::VkImageLayout, regionCount: u32, pRegions: *const vk::VkImageCopy); [pfn_vkCmdCopyImage: vk::PFN_vkCmdCopyImage],

        /// [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
        pub fn vkCmdCopyImageToBuffer(commandBuffer: vk::VkCommandBuffer, srcImage: vk::VkImage, srcImageLayout: vk::VkImageLayout, dstBuffer: vk::VkBuffer, regionCount: u32, pRegions: *const vk::VkBufferImageCopy); [pfn_vkCmdCopyImageToBuffer: vk::PFN_vkCmdCopyImageToBuffer],

        /// [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
        pub fn vkCmdCopyQueryPoolResults(commandBuffer: vk::VkCommandBuffer, queryPool: vk::VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: vk::VkBuffer, dstOffset: vk::VkDeviceSize, stride: vk::VkDeviceSize, flags: vk::VkQueryResultFlags); [pfn_vkCmdCopyQueryPoolResults: vk::PFN_vkCmdCopyQueryPoolResults],

        /// [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
        pub fn vkCmdDispatch(commandBuffer: vk::VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32); [pfn_vkCmdDispatch: vk::PFN_vkCmdDispatch],

        /// [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
        pub fn vkCmdDispatchIndirect(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize); [pfn_vkCmdDispatchIndirect: vk::PFN_vkCmdDispatchIndirect],

        /// [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
        pub fn vkCmdDraw(commandBuffer: vk::VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32); [pfn_vkCmdDraw: vk::PFN_vkCmdDraw],

        /// [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
        pub fn vkCmdDrawIndexed(commandBuffer: vk::VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32); [pfn_vkCmdDrawIndexed: vk::PFN_vkCmdDrawIndexed],

        /// [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
        pub fn vkCmdDrawIndexedIndirect(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, drawCount: u32, stride: u32); [pfn_vkCmdDrawIndexedIndirect: vk::PFN_vkCmdDrawIndexedIndirect],

        /// [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
        pub fn vkCmdDrawIndirect(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, drawCount: u32, stride: u32); [pfn_vkCmdDrawIndirect: vk::PFN_vkCmdDrawIndirect],

        /// [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
        pub fn vkCmdEndQuery(commandBuffer: vk::VkCommandBuffer, queryPool: vk::VkQueryPool, query: u32); [pfn_vkCmdEndQuery: vk::PFN_vkCmdEndQuery],

        /// [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
        pub fn vkCmdEndRenderPass(commandBuffer: vk::VkCommandBuffer); [pfn_vkCmdEndRenderPass: vk::PFN_vkCmdEndRenderPass],

        /// [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
        pub fn vkCmdExecuteCommands(commandBuffer: vk::VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const vk::VkCommandBuffer); [pfn_vkCmdExecuteCommands: vk::PFN_vkCmdExecuteCommands],

        /// [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
        pub fn vkCmdFillBuffer(commandBuffer: vk::VkCommandBuffer, dstBuffer: vk::VkBuffer, dstOffset: vk::VkDeviceSize, size: vk::VkDeviceSize, data: u32); [pfn_vkCmdFillBuffer: vk::PFN_vkCmdFillBuffer],

        /// [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
        pub fn vkCmdNextSubpass(commandBuffer: vk::VkCommandBuffer, contents: vk::VkSubpassContents); [pfn_vkCmdNextSubpass: vk::PFN_vkCmdNextSubpass],

        /// [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
        pub fn vkCmdPipelineBarrier(commandBuffer: vk::VkCommandBuffer, srcStageMask: vk::VkPipelineStageFlags, dstStageMask: vk::VkPipelineStageFlags, dependencyFlags: vk::VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const vk::VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const vk::VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const vk::VkImageMemoryBarrier); [pfn_vkCmdPipelineBarrier: vk::PFN_vkCmdPipelineBarrier],

        /// [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
        pub fn vkCmdPushConstants(commandBuffer: vk::VkCommandBuffer, layout: vk::VkPipelineLayout, stageFlags: vk::VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void); [pfn_vkCmdPushConstants: vk::PFN_vkCmdPushConstants],

        /// [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
        pub fn vkCmdResetEvent(commandBuffer: vk::VkCommandBuffer, event: vk::VkEvent, stageMask: vk::VkPipelineStageFlags); [pfn_vkCmdResetEvent: vk::PFN_vkCmdResetEvent],

        /// [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
        pub fn vkCmdResetQueryPool(commandBuffer: vk::VkCommandBuffer, queryPool: vk::VkQueryPool, firstQuery: u32, queryCount: u32); [pfn_vkCmdResetQueryPool: vk::PFN_vkCmdResetQueryPool],

        /// [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
        pub fn vkCmdResolveImage(commandBuffer: vk::VkCommandBuffer, srcImage: vk::VkImage, srcImageLayout: vk::VkImageLayout, dstImage: vk::VkImage, dstImageLayout: vk::VkImageLayout, regionCount: u32, pRegions: *const vk::VkImageResolve); [pfn_vkCmdResolveImage: vk::PFN_vkCmdResolveImage],

        /// [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
        pub fn vkCmdSetBlendConstants(commandBuffer: vk::VkCommandBuffer, blendConstants: *const f32); [pfn_vkCmdSetBlendConstants: vk::PFN_vkCmdSetBlendConstants],

        /// [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
        pub fn vkCmdSetDepthBias(commandBuffer: vk::VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32); [pfn_vkCmdSetDepthBias: vk::PFN_vkCmdSetDepthBias],

        /// [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
        pub fn vkCmdSetDepthBounds(commandBuffer: vk::VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32); [pfn_vkCmdSetDepthBounds: vk::PFN_vkCmdSetDepthBounds],

        /// [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
        pub fn vkCmdSetEvent(commandBuffer: vk::VkCommandBuffer, event: vk::VkEvent, stageMask: vk::VkPipelineStageFlags); [pfn_vkCmdSetEvent: vk::PFN_vkCmdSetEvent],

        /// [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
        pub fn vkCmdSetLineWidth(commandBuffer: vk::VkCommandBuffer, lineWidth: f32); [pfn_vkCmdSetLineWidth: vk::PFN_vkCmdSetLineWidth],

        /// [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
        pub fn vkCmdSetScissor(commandBuffer: vk::VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const vk::VkRect2D); [pfn_vkCmdSetScissor: vk::PFN_vkCmdSetScissor],

        /// [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
        pub fn vkCmdSetStencilCompareMask(commandBuffer: vk::VkCommandBuffer, faceMask: vk::VkStencilFaceFlags, compareMask: u32); [pfn_vkCmdSetStencilCompareMask: vk::PFN_vkCmdSetStencilCompareMask],

        /// [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
        pub fn vkCmdSetStencilReference(commandBuffer: vk::VkCommandBuffer, faceMask: vk::VkStencilFaceFlags, reference: u32); [pfn_vkCmdSetStencilReference: vk::PFN_vkCmdSetStencilReference],

        /// [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
        pub fn vkCmdSetStencilWriteMask(commandBuffer: vk::VkCommandBuffer, faceMask: vk::VkStencilFaceFlags, writeMask: u32); [pfn_vkCmdSetStencilWriteMask: vk::PFN_vkCmdSetStencilWriteMask],

        /// [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
        pub fn vkCmdSetViewport(commandBuffer: vk::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const vk::VkViewport); [pfn_vkCmdSetViewport: vk::PFN_vkCmdSetViewport],

        /// [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
        pub fn vkCmdUpdateBuffer(commandBuffer: vk::VkCommandBuffer, dstBuffer: vk::VkBuffer, dstOffset: vk::VkDeviceSize, dataSize: vk::VkDeviceSize, pData: *const c_void); [pfn_vkCmdUpdateBuffer: vk::PFN_vkCmdUpdateBuffer],

        /// [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
        pub fn vkCmdWaitEvents(commandBuffer: vk::VkCommandBuffer, eventCount: u32, pEvents: *const vk::VkEvent, srcStageMask: vk::VkPipelineStageFlags, dstStageMask: vk::VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const vk::VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const vk::VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const vk::VkImageMemoryBarrier); [pfn_vkCmdWaitEvents: vk::PFN_vkCmdWaitEvents],

        /// [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
        pub fn vkCmdWriteTimestamp(commandBuffer: vk::VkCommandBuffer, pipelineStage: vk::VkPipelineStageFlagBits, queryPool: vk::VkQueryPool, query: u32); [pfn_vkCmdWriteTimestamp: vk::PFN_vkCmdWriteTimestamp],

        /// [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
        pub fn vkCreateBuffer(device: vk::VkDevice, pCreateInfo: *const vk::VkBufferCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pBuffer: *mut vk::VkBuffer) -> vk::VkResult; [pfn_vkCreateBuffer: vk::PFN_vkCreateBuffer],

        /// [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
        pub fn vkCreateBufferView(device: vk::VkDevice, pCreateInfo: *const vk::VkBufferViewCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pView: *mut vk::VkBufferView) -> vk::VkResult; [pfn_vkCreateBufferView: vk::PFN_vkCreateBufferView],

        /// [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
        pub fn vkCreateCommandPool(device: vk::VkDevice, pCreateInfo: *const vk::VkCommandPoolCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pCommandPool: *mut vk::VkCommandPool) -> vk::VkResult; [pfn_vkCreateCommandPool: vk::PFN_vkCreateCommandPool],

        /// [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
        pub fn vkCreateComputePipelines(device: vk::VkDevice, pipelineCache: vk::VkPipelineCache, createInfoCount: u32, pCreateInfos: *const vk::VkComputePipelineCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pPipelines: *mut vk::VkPipeline) -> vk::VkResult; [pfn_vkCreateComputePipelines: vk::PFN_vkCreateComputePipelines],

        /// [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
        pub fn vkCreateDescriptorPool(device: vk::VkDevice, pCreateInfo: *const vk::VkDescriptorPoolCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pDescriptorPool: *mut vk::VkDescriptorPool) -> vk::VkResult; [pfn_vkCreateDescriptorPool: vk::PFN_vkCreateDescriptorPool],

        /// [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
        pub fn vkCreateDescriptorSetLayout(device: vk::VkDevice, pCreateInfo: *const vk::VkDescriptorSetLayoutCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pSetLayout: *mut vk::VkDescriptorSetLayout) -> vk::VkResult; [pfn_vkCreateDescriptorSetLayout: vk::PFN_vkCreateDescriptorSetLayout],

        /// [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
        pub fn vkCreateEvent(device: vk::VkDevice, pCreateInfo: *const vk::VkEventCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pEvent: *mut vk::VkEvent) -> vk::VkResult; [pfn_vkCreateEvent: vk::PFN_vkCreateEvent],

        /// [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
        pub fn vkCreateFence(device: vk::VkDevice, pCreateInfo: *const vk::VkFenceCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pFence: *mut vk::VkFence) -> vk::VkResult; [pfn_vkCreateFence: vk::PFN_vkCreateFence],

        /// [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
        pub fn vkCreateFramebuffer(device: vk::VkDevice, pCreateInfo: *const vk::VkFramebufferCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pFramebuffer: *mut vk::VkFramebuffer) -> vk::VkResult; [pfn_vkCreateFramebuffer: vk::PFN_vkCreateFramebuffer],

        /// [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
        pub fn vkCreateGraphicsPipelines(device: vk::VkDevice, pipelineCache: vk::VkPipelineCache, createInfoCount: u32, pCreateInfos: *const vk::VkGraphicsPipelineCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pPipelines: *mut vk::VkPipeline) -> vk::VkResult; [pfn_vkCreateGraphicsPipelines: vk::PFN_vkCreateGraphicsPipelines],

        /// [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
        pub fn vkCreateImage(device: vk::VkDevice, pCreateInfo: *const vk::VkImageCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pImage: *mut vk::VkImage) -> vk::VkResult; [pfn_vkCreateImage: vk::PFN_vkCreateImage],

        /// [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
        pub fn vkCreateImageView(device: vk::VkDevice, pCreateInfo: *const vk::VkImageViewCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pView: *mut vk::VkImageView) -> vk::VkResult; [pfn_vkCreateImageView: vk::PFN_vkCreateImageView],

        /// [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
        pub fn vkCreatePipelineCache(device: vk::VkDevice, pCreateInfo: *const vk::VkPipelineCacheCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pPipelineCache: *mut vk::VkPipelineCache) -> vk::VkResult; [pfn_vkCreatePipelineCache: vk::PFN_vkCreatePipelineCache],

        /// [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
        pub fn vkCreatePipelineLayout(device: vk::VkDevice, pCreateInfo: *const vk::VkPipelineLayoutCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pPipelineLayout: *mut vk::VkPipelineLayout) -> vk::VkResult; [pfn_vkCreatePipelineLayout: vk::PFN_vkCreatePipelineLayout],

        /// [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
        pub fn vkCreateQueryPool(device: vk::VkDevice, pCreateInfo: *const vk::VkQueryPoolCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pQueryPool: *mut vk::VkQueryPool) -> vk::VkResult; [pfn_vkCreateQueryPool: vk::PFN_vkCreateQueryPool],

        /// [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
        pub fn vkCreateRenderPass(device: vk::VkDevice, pCreateInfo: *const vk::VkRenderPassCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pRenderPass: *mut vk::VkRenderPass) -> vk::VkResult; [pfn_vkCreateRenderPass: vk::PFN_vkCreateRenderPass],

        /// [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
        pub fn vkCreateSampler(device: vk::VkDevice, pCreateInfo: *const vk::VkSamplerCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pSampler: *mut vk::VkSampler) -> vk::VkResult; [pfn_vkCreateSampler: vk::PFN_vkCreateSampler],

        /// [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
        pub fn vkCreateSemaphore(device: vk::VkDevice, pCreateInfo: *const vk::VkSemaphoreCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pSemaphore: *mut vk::VkSemaphore) -> vk::VkResult; [pfn_vkCreateSemaphore: vk::PFN_vkCreateSemaphore],

        /// [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
        pub fn vkCreateShaderModule(device: vk::VkDevice, pCreateInfo: *const vk::VkShaderModuleCreateInfo, pAllocator: *const vk::VkAllocationCallbacks, pShaderModule: *mut vk::VkShaderModule) -> vk::VkResult; [pfn_vkCreateShaderModule: vk::PFN_vkCreateShaderModule],

        /// [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBuffer)
        pub fn vkDestroyBuffer(device: vk::VkDevice, buffer: vk::VkBuffer, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyBuffer: vk::PFN_vkDestroyBuffer],

        /// [`vkDestroyBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBufferView)
        pub fn vkDestroyBufferView(device: vk::VkDevice, bufferView: vk::VkBufferView, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyBufferView: vk::PFN_vkDestroyBufferView],

        /// [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyCommandPool)
        pub fn vkDestroyCommandPool(device: vk::VkDevice, commandPool: vk::VkCommandPool, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyCommandPool: vk::PFN_vkDestroyCommandPool],

        /// [`vkDestroyDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorPool)
        pub fn vkDestroyDescriptorPool(device: vk::VkDevice, descriptorPool: vk::VkDescriptorPool, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyDescriptorPool: vk::PFN_vkDestroyDescriptorPool],

        /// [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorSetLayout)
        pub fn vkDestroyDescriptorSetLayout(device: vk::VkDevice, descriptorSetLayout: vk::VkDescriptorSetLayout, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyDescriptorSetLayout: vk::PFN_vkDestroyDescriptorSetLayout],

        /// [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDevice)
        pub fn vkDestroyDevice(device: vk::VkDevice, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyDevice: vk::PFN_vkDestroyDevice],

        /// [`vkDestroyEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyEvent)
        pub fn vkDestroyEvent(device: vk::VkDevice, event: vk::VkEvent, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyEvent: vk::PFN_vkDestroyEvent],

        /// [`vkDestroyFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFence)
        pub fn vkDestroyFence(device: vk::VkDevice, fence: vk::VkFence, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyFence: vk::PFN_vkDestroyFence],

        /// [`vkDestroyFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFramebuffer)
        pub fn vkDestroyFramebuffer(device: vk::VkDevice, framebuffer: vk::VkFramebuffer, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyFramebuffer: vk::PFN_vkDestroyFramebuffer],

        /// [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImage)
        pub fn vkDestroyImage(device: vk::VkDevice, image: vk::VkImage, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyImage: vk::PFN_vkDestroyImage],

        /// [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImageView)
        pub fn vkDestroyImageView(device: vk::VkDevice, imageView: vk::VkImageView, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyImageView: vk::PFN_vkDestroyImageView],

        /// [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipeline)
        pub fn vkDestroyPipeline(device: vk::VkDevice, pipeline: vk::VkPipeline, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyPipeline: vk::PFN_vkDestroyPipeline],

        /// [`vkDestroyPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineCache)
        pub fn vkDestroyPipelineCache(device: vk::VkDevice, pipelineCache: vk::VkPipelineCache, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyPipelineCache: vk::PFN_vkDestroyPipelineCache],

        /// [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineLayout)
        pub fn vkDestroyPipelineLayout(device: vk::VkDevice, pipelineLayout: vk::VkPipelineLayout, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyPipelineLayout: vk::PFN_vkDestroyPipelineLayout],

        /// [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyQueryPool)
        pub fn vkDestroyQueryPool(device: vk::VkDevice, queryPool: vk::VkQueryPool, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyQueryPool: vk::PFN_vkDestroyQueryPool],

        /// [`vkDestroyRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyRenderPass)
        pub fn vkDestroyRenderPass(device: vk::VkDevice, renderPass: vk::VkRenderPass, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyRenderPass: vk::PFN_vkDestroyRenderPass],

        /// [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySampler)
        pub fn vkDestroySampler(device: vk::VkDevice, sampler: vk::VkSampler, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroySampler: vk::PFN_vkDestroySampler],

        /// [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySemaphore)
        pub fn vkDestroySemaphore(device: vk::VkDevice, semaphore: vk::VkSemaphore, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroySemaphore: vk::PFN_vkDestroySemaphore],

        /// [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyShaderModule)
        pub fn vkDestroyShaderModule(device: vk::VkDevice, shaderModule: vk::VkShaderModule, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyShaderModule: vk::PFN_vkDestroyShaderModule],

        /// [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
        pub fn vkDeviceWaitIdle(device: vk::VkDevice) -> vk::VkResult; [pfn_vkDeviceWaitIdle: vk::PFN_vkDeviceWaitIdle],

        /// [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
        pub fn vkEndCommandBuffer(commandBuffer: vk::VkCommandBuffer) -> vk::VkResult; [pfn_vkEndCommandBuffer: vk::PFN_vkEndCommandBuffer],

        /// [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
        pub fn vkFlushMappedMemoryRanges(device: vk::VkDevice, memoryRangeCount: u32, pMemoryRanges: *const vk::VkMappedMemoryRange) -> vk::VkResult; [pfn_vkFlushMappedMemoryRanges: vk::PFN_vkFlushMappedMemoryRanges],

        /// [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers)
        pub fn vkFreeCommandBuffers(device: vk::VkDevice, commandPool: vk::VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const vk::VkCommandBuffer); [pfn_vkFreeCommandBuffers: vk::PFN_vkFreeCommandBuffers],

        /// [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
        pub fn vkFreeDescriptorSets(device: vk::VkDevice, descriptorPool: vk::VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const vk::VkDescriptorSet) -> vk::VkResult; [pfn_vkFreeDescriptorSets: vk::PFN_vkFreeDescriptorSets],

        /// [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeMemory)
        pub fn vkFreeMemory(device: vk::VkDevice, memory: vk::VkDeviceMemory, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkFreeMemory: vk::PFN_vkFreeMemory],

        /// [`vkGetBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements)
        pub fn vkGetBufferMemoryRequirements(device: vk::VkDevice, buffer: vk::VkBuffer, pMemoryRequirements: *mut vk::VkMemoryRequirements); [pfn_vkGetBufferMemoryRequirements: vk::PFN_vkGetBufferMemoryRequirements],

        /// [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
        pub fn vkGetDeviceMemoryCommitment(device: vk::VkDevice, memory: vk::VkDeviceMemory, pCommittedMemoryInBytes: *mut vk::VkDeviceSize); [pfn_vkGetDeviceMemoryCommitment: vk::PFN_vkGetDeviceMemoryCommitment],

        /// [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
        pub fn vkGetDeviceQueue(device: vk::VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut vk::VkQueue); [pfn_vkGetDeviceQueue: vk::PFN_vkGetDeviceQueue],

        /// [`vkGetEventStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetEventStatus)
        pub fn vkGetEventStatus(device: vk::VkDevice, event: vk::VkEvent) -> vk::VkResult; [pfn_vkGetEventStatus: vk::PFN_vkGetEventStatus],

        /// [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
        pub fn vkGetFenceStatus(device: vk::VkDevice, fence: vk::VkFence) -> vk::VkResult; [pfn_vkGetFenceStatus: vk::PFN_vkGetFenceStatus],

        /// [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
        pub fn vkGetImageMemoryRequirements(device: vk::VkDevice, image: vk::VkImage, pMemoryRequirements: *mut vk::VkMemoryRequirements); [pfn_vkGetImageMemoryRequirements: vk::PFN_vkGetImageMemoryRequirements],

        /// [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
        pub fn vkGetImageSparseMemoryRequirements(device: vk::VkDevice, image: vk::VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut vk::VkSparseImageMemoryRequirements); [pfn_vkGetImageSparseMemoryRequirements: vk::PFN_vkGetImageSparseMemoryRequirements],

        /// [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
        pub fn vkGetImageSubresourceLayout(device: vk::VkDevice, image: vk::VkImage, pSubresource: *const vk::VkImageSubresource, pLayout: *mut vk::VkSubresourceLayout); [pfn_vkGetImageSubresourceLayout: vk::PFN_vkGetImageSubresourceLayout],

        /// [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
        pub fn vkGetPipelineCacheData(device: vk::VkDevice, pipelineCache: vk::VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> vk::VkResult; [pfn_vkGetPipelineCacheData: vk::PFN_vkGetPipelineCacheData],

        /// [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
        pub fn vkGetQueryPoolResults(device: vk::VkDevice, queryPool: vk::VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: vk::VkDeviceSize, flags: vk::VkQueryResultFlags) -> vk::VkResult; [pfn_vkGetQueryPoolResults: vk::PFN_vkGetQueryPoolResults],

        /// [`vkGetRenderAreaGranularity`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRenderAreaGranularity)
        pub fn vkGetRenderAreaGranularity(device: vk::VkDevice, renderPass: vk::VkRenderPass, pGranularity: *mut vk::VkExtent2D); [pfn_vkGetRenderAreaGranularity: vk::PFN_vkGetRenderAreaGranularity],

        /// [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
        pub fn vkInvalidateMappedMemoryRanges(device: vk::VkDevice, memoryRangeCount: u32, pMemoryRanges: *const vk::VkMappedMemoryRange) -> vk::VkResult; [pfn_vkInvalidateMappedMemoryRanges: vk::PFN_vkInvalidateMappedMemoryRanges],

        /// [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
        pub fn vkMapMemory(device: vk::VkDevice, memory: vk::VkDeviceMemory, offset: vk::VkDeviceSize, size: vk::VkDeviceSize, flags: vk::VkMemoryMapFlags, ppData: *mut *mut c_void) -> vk::VkResult; [pfn_vkMapMemory: vk::PFN_vkMapMemory],

        /// [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
        pub fn vkMergePipelineCaches(device: vk::VkDevice, dstCache: vk::VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const vk::VkPipelineCache) -> vk::VkResult; [pfn_vkMergePipelineCaches: vk::PFN_vkMergePipelineCaches],

        /// [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
        pub fn vkQueueBindSparse(queue: vk::VkQueue, bindInfoCount: u32, pBindInfo: *const vk::VkBindSparseInfo, fence: vk::VkFence) -> vk::VkResult; [pfn_vkQueueBindSparse: vk::PFN_vkQueueBindSparse],

        /// [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
        pub fn vkQueueSubmit(queue: vk::VkQueue, submitCount: u32, pSubmits: *const vk::VkSubmitInfo, fence: vk::VkFence) -> vk::VkResult; [pfn_vkQueueSubmit: vk::PFN_vkQueueSubmit],

        /// [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
        pub fn vkQueueWaitIdle(queue: vk::VkQueue) -> vk::VkResult; [pfn_vkQueueWaitIdle: vk::PFN_vkQueueWaitIdle],

        /// [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
        pub fn vkResetCommandBuffer(commandBuffer: vk::VkCommandBuffer, flags: vk::VkCommandBufferResetFlags) -> vk::VkResult; [pfn_vkResetCommandBuffer: vk::PFN_vkResetCommandBuffer],

        /// [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
        pub fn vkResetCommandPool(device: vk::VkDevice, commandPool: vk::VkCommandPool, flags: vk::VkCommandPoolResetFlags) -> vk::VkResult; [pfn_vkResetCommandPool: vk::PFN_vkResetCommandPool],

        /// [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
        pub fn vkResetDescriptorPool(device: vk::VkDevice, descriptorPool: vk::VkDescriptorPool, flags: vk::VkDescriptorPoolResetFlags) -> vk::VkResult; [pfn_vkResetDescriptorPool: vk::PFN_vkResetDescriptorPool],

        /// [`vkResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetEvent)
        pub fn vkResetEvent(device: vk::VkDevice, event: vk::VkEvent) -> vk::VkResult; [pfn_vkResetEvent: vk::PFN_vkResetEvent],

        /// [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
        pub fn vkResetFences(device: vk::VkDevice, fenceCount: u32, pFences: *const vk::VkFence) -> vk::VkResult; [pfn_vkResetFences: vk::PFN_vkResetFences],

        /// [`vkSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetEvent)
        pub fn vkSetEvent(device: vk::VkDevice, event: vk::VkEvent) -> vk::VkResult; [pfn_vkSetEvent: vk::PFN_vkSetEvent],

        /// [`vkUnmapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnmapMemory)
        pub fn vkUnmapMemory(device: vk::VkDevice, memory: vk::VkDeviceMemory); [pfn_vkUnmapMemory: vk::PFN_vkUnmapMemory],

        /// [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
        pub fn vkUpdateDescriptorSets(device: vk::VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const vk::VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const vk::VkCopyDescriptorSet); [pfn_vkUpdateDescriptorSets: vk::PFN_vkUpdateDescriptorSets],

        /// [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
        pub fn vkWaitForFences(device: vk::VkDevice, fenceCount: u32, pFences: *const vk::VkFence, waitAll: vk::VkBool32, timeout: u64) -> vk::VkResult; [pfn_vkWaitForFences: vk::PFN_vkWaitForFences],
    }
);

addr_proc_struct!(
    /// [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
    pub struct AMD_draw_indirect_count [amd_draw_indirect_count] {
        /// [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
        pub fn vkCmdDrawIndexedIndirectCountAMD(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, countBuffer: vk::VkBuffer, countBufferOffset: vk::VkDeviceSize, maxDrawCount: u32, stride: u32); [pfn_vkCmdDrawIndexedIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndexedIndirectCountAMD],

        /// [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
        pub fn vkCmdDrawIndirectCountAMD(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, countBuffer: vk::VkBuffer, countBufferOffset: vk::VkDeviceSize, maxDrawCount: u32, stride: u32); [pfn_vkCmdDrawIndirectCountAMD: amd_draw_indirect_count::PFN_vkCmdDrawIndirectCountAMD],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub struct EXT_debug_marker [ext_debug_marker] {
        /// [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
        pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: vk::VkCommandBuffer, pMarkerInfo: *const ext_debug_marker::VkDebugMarkerMarkerInfoEXT); [pfn_vkCmdDebugMarkerBeginEXT: ext_debug_marker::PFN_vkCmdDebugMarkerBeginEXT],

        /// [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
        pub fn vkCmdDebugMarkerEndEXT(commandBuffer: vk::VkCommandBuffer); [pfn_vkCmdDebugMarkerEndEXT: ext_debug_marker::PFN_vkCmdDebugMarkerEndEXT],

        /// [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
        pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: vk::VkCommandBuffer, pMarkerInfo: *const ext_debug_marker::VkDebugMarkerMarkerInfoEXT); [pfn_vkCmdDebugMarkerInsertEXT: ext_debug_marker::PFN_vkCmdDebugMarkerInsertEXT],

        /// [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
        pub fn vkDebugMarkerSetObjectNameEXT(device: vk::VkDevice, pNameInfo: *const ext_debug_marker::VkDebugMarkerObjectNameInfoEXT) -> vk::VkResult; [pfn_vkDebugMarkerSetObjectNameEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectNameEXT],

        /// [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
        pub fn vkDebugMarkerSetObjectTagEXT(device: vk::VkDevice, pTagInfo: *const ext_debug_marker::VkDebugMarkerObjectTagInfoEXT) -> vk::VkResult; [pfn_vkDebugMarkerSetObjectTagEXT: ext_debug_marker::PFN_vkDebugMarkerSetObjectTagEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    pub struct EXT_discard_rectangles [ext_discard_rectangles] {
        /// [`vkCmdSetDiscardRectangleEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDiscardRectangleEXT)
        pub fn vkCmdSetDiscardRectangleEXT(commandBuffer: vk::VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const vk::VkRect2D); [pfn_vkCmdSetDiscardRectangleEXT: ext_discard_rectangles::PFN_vkCmdSetDiscardRectangleEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    pub struct EXT_display_control [ext_display_control] {
        /// [`vkDisplayPowerControlEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDisplayPowerControlEXT)
        pub fn vkDisplayPowerControlEXT(device: vk::VkDevice, display: khr_display::VkDisplayKHR, pDisplayPowerInfo: *const ext_display_control::VkDisplayPowerInfoEXT) -> vk::VkResult; [pfn_vkDisplayPowerControlEXT: ext_display_control::PFN_vkDisplayPowerControlEXT],

        /// [`vkGetSwapchainCounterEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainCounterEXT)
        pub fn vkGetSwapchainCounterEXT(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, counter: ext_display_surface_counter::VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> vk::VkResult; [pfn_vkGetSwapchainCounterEXT: ext_display_control::PFN_vkGetSwapchainCounterEXT],

        /// [`vkRegisterDeviceEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDeviceEventEXT)
        pub fn vkRegisterDeviceEventEXT(device: vk::VkDevice, pDeviceEventInfo: *const ext_display_control::VkDeviceEventInfoEXT, pAllocator: *const vk::VkAllocationCallbacks, pFence: *mut vk::VkFence) -> vk::VkResult; [pfn_vkRegisterDeviceEventEXT: ext_display_control::PFN_vkRegisterDeviceEventEXT],

        /// [`vkRegisterDisplayEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDisplayEventEXT)
        pub fn vkRegisterDisplayEventEXT(device: vk::VkDevice, display: khr_display::VkDisplayKHR, pDisplayEventInfo: *const ext_display_control::VkDisplayEventInfoEXT, pAllocator: *const vk::VkAllocationCallbacks, pFence: *mut vk::VkFence) -> vk::VkResult; [pfn_vkRegisterDisplayEventEXT: ext_display_control::PFN_vkRegisterDisplayEventEXT],
    }
);

addr_proc_struct!(
    /// [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
    pub struct EXT_hdr_metadata [ext_hdr_metadata] {
        /// [`vkSetHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetHdrMetadataEXT)
        pub fn vkSetHdrMetadataEXT(device: vk::VkDevice, swapchainCount: u32, pSwapchains: *const khr_swapchain::VkSwapchainKHR, pMetadata: *const ext_hdr_metadata::VkHdrMetadataEXT); [pfn_vkSetHdrMetadataEXT: ext_hdr_metadata::PFN_vkSetHdrMetadataEXT],
    }
);

addr_proc_struct!(
    /// [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    pub struct GOOGLE_display_timing [google_display_timing] {
        /// [`vkGetPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPastPresentationTimingGOOGLE)
        pub fn vkGetPastPresentationTimingGOOGLE(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut google_display_timing::VkPastPresentationTimingGOOGLE) -> vk::VkResult; [pfn_vkGetPastPresentationTimingGOOGLE: google_display_timing::PFN_vkGetPastPresentationTimingGOOGLE],

        /// [`vkGetRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRefreshCycleDurationGOOGLE)
        pub fn vkGetRefreshCycleDurationGOOGLE(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pDisplayTimingProperties: *mut google_display_timing::VkRefreshCycleDurationGOOGLE) -> vk::VkResult; [pfn_vkGetRefreshCycleDurationGOOGLE: google_display_timing::PFN_vkGetRefreshCycleDurationGOOGLE],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub struct KHR_descriptor_update_template [khr_descriptor_update_template] {
        /// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
        pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: vk::VkCommandBuffer, descriptorUpdateTemplate: khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR, layout: vk::VkPipelineLayout, set: u32, pData: *const c_void); [pfn_vkCmdPushDescriptorSetWithTemplateKHR: khr_descriptor_update_template::PFN_vkCmdPushDescriptorSetWithTemplateKHR],

        /// [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
        pub fn vkCreateDescriptorUpdateTemplateKHR(device: vk::VkDevice, pCreateInfo: *const khr_descriptor_update_template::VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR) -> vk::VkResult; [pfn_vkCreateDescriptorUpdateTemplateKHR: khr_descriptor_update_template::PFN_vkCreateDescriptorUpdateTemplateKHR],

        /// [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
        pub fn vkDestroyDescriptorUpdateTemplateKHR(device: vk::VkDevice, descriptorUpdateTemplate: khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroyDescriptorUpdateTemplateKHR: khr_descriptor_update_template::PFN_vkDestroyDescriptorUpdateTemplateKHR],

        /// [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
        pub fn vkUpdateDescriptorSetWithTemplateKHR(device: vk::VkDevice, descriptorSet: vk::VkDescriptorSet, descriptorUpdateTemplate: khr_descriptor_update_template::VkDescriptorUpdateTemplateKHR, pData: *const c_void); [pfn_vkUpdateDescriptorSetWithTemplateKHR: khr_descriptor_update_template::PFN_vkUpdateDescriptorSetWithTemplateKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    pub struct KHR_display_swapchain [khr_display_swapchain] {
        /// [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
        pub fn vkCreateSharedSwapchainsKHR(device: vk::VkDevice, swapchainCount: u32, pCreateInfos: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSwapchains: *mut khr_swapchain::VkSwapchainKHR) -> vk::VkResult; [pfn_vkCreateSharedSwapchainsKHR: khr_display_swapchain::PFN_vkCreateSharedSwapchainsKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)
    pub struct KHR_external_fence_fd [khr_external_fence_fd] {
        /// See [`vkImportFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceFdKHR)
        pub fn vkImportFenceFdKHR(device: vk::VkDevice, pImportFenceFdInfo: *const khr_external_fence_fd::VkImportFenceFdInfoKHR) -> vk::VkResult; [pfn_vkImportFenceFdKHR: khr_external_fence_fd::PFN_vkImportFenceFdKHR],

        /// See [`vkGetFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceFdKHR)
        pub fn vkGetFenceFdKHR(device: vk::VkDevice, pGetFdInfo: *const khr_external_fence_fd::VkFenceGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult; [pfn_vkGetFenceFdKHR: khr_external_fence_fd::PFN_vkGetFenceFdKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
    pub struct KHR_external_fence_win32 [khr_external_fence_win32] {
        /// See [`vkImportFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceWin32HandleKHR)
        pub fn vkImportFenceWin32HandleKHR(device: vk::VkDevice, pImportFenceWin32HandleInfo: *const khr_external_fence_win32::VkImportFenceWin32HandleInfoKHR) -> vk::VkResult; [pfn_vkImportFenceWin32HandleKHR: khr_external_fence_win32::PFN_vkImportFenceWin32HandleKHR],

        /// See [`vkGetFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceWin32HandleKHR)
        pub fn vkGetFenceWin32HandleKHR(device: vk::VkDevice, pGetWin32HandleInfo: *const khr_external_fence_win32::VkFenceGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult; [pfn_vkGetFenceWin32HandleKHR: khr_external_fence_win32::PFN_vkGetFenceWin32HandleKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
    pub struct KHR_external_memory_fd [khr_external_memory_fd] {
        /// See [`vkGetMemoryFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHR)
        pub fn vkGetMemoryFdKHR(device: vk::VkDevice, pGetFdInfo: *const khr_external_memory_fd::VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult; [pfn_vkGetMemoryFdKHR: khr_external_memory_fd::PFN_vkGetMemoryFdKHR],

        /// See [`vkGetMemoryFdPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHR)
        pub fn vkGetMemoryFdPropertiesKHR(device: vk::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, fd: c_int, pMemoryFdProperties: *mut khr_external_memory_fd::VkMemoryFdPropertiesKHR) -> vk::VkResult; [pfn_vkGetMemoryFdPropertiesKHR: khr_external_memory_fd::PFN_vkGetMemoryFdPropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
    pub struct KHR_external_memory_win32 [khr_external_memory_win32] {
        /// See [`vkGetMemoryWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHR)
        pub fn vkGetMemoryWin32HandleKHR(device: vk::VkDevice, pGetWin32HandleInfo: *const khr_external_memory_win32::VkMemoryGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult; [pfn_vkGetMemoryWin32HandleKHR: khr_external_memory_win32::PFN_vkGetMemoryWin32HandleKHR],

        /// See [`vkGetMemoryWin32HandlePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHR)
        pub fn vkGetMemoryWin32HandlePropertiesKHR(device: vk::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, handle: win32_types::HANDLE, pMemoryWin32HandleProperties: *mut khr_external_memory_win32::VkMemoryWin32HandlePropertiesKHR) -> vk::VkResult; [pfn_vkGetMemoryWin32HandlePropertiesKHR: khr_external_memory_win32::PFN_vkGetMemoryWin32HandlePropertiesKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
    pub struct KHR_external_semaphore_fd [khr_external_semaphore_fd] {
        /// See [`VkImportSemaphoreFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHR)
        pub fn vkImportSemaphoreFdKHR(device: vk::VkDevice, pImportSemaphoreFdInfo: *const khr_external_semaphore_fd::VkImportSemaphoreFdInfoKHR) -> vk::VkResult; [pfn_vkImportSemaphoreFdKHR: khr_external_semaphore_fd::PFN_vkImportSemaphoreFdKHR],

        /// See [`vkGetSemaphoreFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHR)
        pub fn vkGetSemaphoreFdKHR(device: vk::VkDevice, pGetFdInfo: *const khr_external_semaphore_fd::VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult; [pfn_vkGetSemaphoreFdKHR: khr_external_semaphore_fd::PFN_vkGetSemaphoreFdKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
    pub struct KHR_get_memory_requirements2 [khr_get_memory_requirements2] {
        /// See [`vkGetImageMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements2KHR)
        pub fn vkGetImageMemoryRequirements2KHR(device: vk::VkDevice, pInfo: *const khr_get_memory_requirements2::VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut khr_get_memory_requirements2::VkMemoryRequirements2KHR); [pfn_vkGetImageMemoryRequirements2KHR: khr_get_memory_requirements2::PFN_vkGetImageMemoryRequirements2KHR],

        /// See [`vkGetBufferMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements2KHR)
        pub fn vkGetBufferMemoryRequirements2KHR(device: vk::VkDevice, pInfo: *const khr_get_memory_requirements2::VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut khr_get_memory_requirements2::VkMemoryRequirements2KHR); [pfn_vkGetBufferMemoryRequirements2KHR: khr_get_memory_requirements2::PFN_vkGetBufferMemoryRequirements2KHR],

        /// See [`vkGetImageSparseMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements2KHR)
        pub fn vkGetImageSparseMemoryRequirements2KHR(device: vk::VkDevice, pInfo: *const khr_get_memory_requirements2::VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut khr_get_memory_requirements2::VkSparseImageMemoryRequirements2KHR); [pfn_vkGetImageSparseMemoryRequirements2KHR: khr_get_memory_requirements2::PFN_vkGetImageSparseMemoryRequirements2KHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
    pub struct KHR_external_semaphore_win32 [khr_external_semaphore_win32] {
        /// See [`VkImportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHR)
        pub fn vkImportSemaphoreWin32HandleKHR(device: vk::VkDevice, pImportSemaphoreWin32HandleInfo: *const khr_external_semaphore_win32::VkImportSemaphoreWin32HandleInfoKHR) -> vk::VkResult; [pfn_vkImportSemaphoreWin32HandleKHR: khr_external_semaphore_win32::PFN_vkImportSemaphoreWin32HandleKHR],

        /// See [`vkGetSemaphoreWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHR)
        pub fn vkGetSemaphoreWin32HandleKHR(device: vk::VkDevice, pGetWin32HandleInfo: *const khr_external_semaphore_win32::VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult; [pfn_vkGetSemaphoreWin32HandleKHR: khr_external_semaphore_win32::PFN_vkGetSemaphoreWin32HandleKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
    pub struct KHR_maintenance1 [khr_maintenance1] {
        /// [`vkTrimCommandPoolKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkTrimCommandPoolKHR)
        pub fn vkTrimCommandPoolKHR(device: vk::VkDevice, commandPool: vk::VkCommandPool, flags: khr_maintenance1::VkCommandPoolTrimFlagsKHR); [pfn_vkTrimCommandPoolKHR: khr_maintenance1::PFN_vkTrimCommandPoolKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
    pub struct KHR_push_descriptor [khr_push_descriptor] {
        /// [`vkCmdPushDescriptorSetKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetKHR)
        pub fn vkCmdPushDescriptorSetKHR(commandBuffer: vk::VkCommandBuffer, pipelineBindPoint: vk::VkPipelineBindPoint, layout: vk::VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const vk::VkWriteDescriptorSet); [pfn_vkCmdPushDescriptorSetKHR: khr_push_descriptor::PFN_vkCmdPushDescriptorSetKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_shared_presentable_image)
    pub struct KHR_shared_presentable_image [khr_shared_presentable_image] {
        /// [`vkGetSwapchainStatusKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainStatusKHR)
        pub fn vkGetSwapchainStatusKHR(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR) -> vk::VkResult; [pfn_vkGetSwapchainStatusKHR: khr_shared_presentable_image::PFN_vkGetSwapchainStatusKHR],
    }
);

addr_proc_struct!(
    /// [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub struct KHR_swapchain [khr_swapchain] {
        /// [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
        pub fn vkAcquireNextImageKHR(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, timeout: u64, semaphore: vk::VkSemaphore, fence: vk::VkFence, pImageIndex: *mut u32) -> vk::VkResult; [pfn_vkAcquireNextImageKHR: khr_swapchain::PFN_vkAcquireNextImageKHR],

        /// [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSwapchainKHR)
        pub fn vkCreateSwapchainKHR(device: vk::VkDevice, pCreateInfo: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSwapchain: *mut khr_swapchain::VkSwapchainKHR) -> vk::VkResult; [pfn_vkCreateSwapchainKHR: khr_swapchain::PFN_vkCreateSwapchainKHR],

        /// [`vkDestroySwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySwapchainKHR)
        pub fn vkDestroySwapchainKHR(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pAllocator: *const vk::VkAllocationCallbacks); [pfn_vkDestroySwapchainKHR: khr_swapchain::PFN_vkDestroySwapchainKHR],

        /// [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainImagesKHR)
        pub fn vkGetSwapchainImagesKHR(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut vk::VkImage) -> vk::VkResult; [pfn_vkGetSwapchainImagesKHR: khr_swapchain::PFN_vkGetSwapchainImagesKHR],

        /// [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
        pub fn vkQueuePresentKHR(queue: vk::VkQueue, pPresentInfo: *const khr_swapchain::VkPresentInfoKHR) -> vk::VkResult; [pfn_vkQueuePresentKHR: khr_swapchain::PFN_vkQueuePresentKHR],
    }
);

addr_proc_struct!(
    /// [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    pub struct NV_clip_space_w_scaling [nv_clip_space_w_scaling] {
        /// [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
        pub fn vkCmdSetViewportWScalingNV(commandBuffer: vk::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const nv_clip_space_w_scaling::VkViewportWScalingNV); [pfn_vkCmdSetViewportWScalingNV: nv_clip_space_w_scaling::PFN_vkCmdSetViewportWScalingNV],
    }
);

addr_proc_struct!(
    /// [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    pub struct NV_external_memory_win32 [nv_external_memory_win32] {
        /// [`vkGetMemoryWin32HandleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleNV)
        pub fn vkGetMemoryWin32HandleNV(device: vk::VkDevice, memory: vk::VkDeviceMemory, handleType: nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_types::HANDLE) -> vk::VkResult; [pfn_vkGetMemoryWin32HandleNV: nv_external_memory_win32::PFN_vkGetMemoryWin32HandleNV],
    }
);
