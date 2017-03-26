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

use ::*;
use libc::{c_char, c_void};
use std::fmt;
use std::mem;

pub struct DeviceProcAddrLoader {
    pub vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    pub core: Core,

    #[cfg(feature = "khr_display_swapchain_9")]
    pub khr_display_swapchain: KHR_display_swapchain,

    #[cfg(feature = "ext_debug_marker_3")]
    pub ext_debug_marker: EXT_debug_marker,
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
        debug_struct.field("core", &self.core);

        #[cfg(feature = "khr_display_swapchain_9")]
        debug_struct.field("khr_display_swapchain", &self.khr_display_swapchain);

        #[cfg(feature = "ext_debug_marker_3")]
        debug_struct.field("ext_debug_marker", &self.ext_debug_marker);

        debug_struct.finish()
    }
}

impl DeviceProcAddrLoader {
    pub fn new(vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr) -> Self {
        DeviceProcAddrLoader {
            vkGetDeviceProcAddr: vkGetDeviceProcAddr,
            core: Core::new(),

            #[cfg(feature = "khr_display_swapchain_9")]
            khr_display_swapchain: KHR_display_swapchain::new(),

            #[cfg(feature = "ext_debug_marker_3")]
            ext_debug_marker: EXT_debug_marker::new(),
        }
    }

    pub unsafe fn load_core(&mut self, device: VkDevice) {
        self.core.load(self.vkGetDeviceProcAddr, device);
    }

    #[cfg(feature = "khr_display_swapchain_9")]
    pub unsafe fn load_khr_display_swapchain(&mut self, device: VkDevice) {
        self.khr_display_swapchain.load(self.vkGetDeviceProcAddr, device);
    }

    #[cfg(feature = "ext_debug_marker_3")]
    pub unsafe fn load_ext_debug_marker(&mut self, device: VkDevice) {
        self.ext_debug_marker.load(self.vkGetDeviceProcAddr, device);
    }
}

macro_rules! addr_proc_struct {
    ($name:ident { $( $(#[$cond:meta])* pfn $symbol:ident: $ty:ty, )* }) => (
        pub struct $name {
            $(
                $(#[$cond])*
                pub $symbol: $ty,
            )*
        }

        impl Copy for $name { }

        impl Clone for $name {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut debug_struct = f.debug_struct(stringify!($name));
                $(
                    $(#[$cond])*
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
                        $(#[$cond])*
                        $symbol: unsafe { mem::transmute(0usize) },
                    )*
                }
            }

            #[allow(unused_variables)]
            pub unsafe fn load(&mut self, vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr, device: VkDevice) {
                $(
                    $(#[$cond])*
                    { self.$symbol = mem::transmute((vkGetDeviceProcAddr)(device, concat!(stringify!($symbol), '\x00').as_ptr() as *const c_char)); }
                )*
            }
        }
    )
}

addr_proc_struct!(Core {
    pfn vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    pfn vkDestroyDevice: PFN_vkDestroyDevice,
    pfn vkGetDeviceQueue: PFN_vkGetDeviceQueue,
    pfn vkQueueSubmit: PFN_vkQueueSubmit,
    pfn vkQueueWaitIdle: PFN_vkQueueWaitIdle,
    pfn vkDeviceWaitIdle: PFN_vkDeviceWaitIdle,
    pfn vkAllocateMemory: PFN_vkAllocateMemory,
    pfn vkFreeMemory: PFN_vkFreeMemory,
    pfn vkMapMemory: PFN_vkMapMemory,
    pfn vkUnmapMemory: PFN_vkUnmapMemory,
    pfn vkFlushMappedMemoryRanges: PFN_vkFlushMappedMemoryRanges,
    pfn vkInvalidateMappedMemoryRanges: PFN_vkInvalidateMappedMemoryRanges,
    pfn vkGetDeviceMemoryCommitment: PFN_vkGetDeviceMemoryCommitment,
    pfn vkBindBufferMemory: PFN_vkBindBufferMemory,
    pfn vkBindImageMemory: PFN_vkBindImageMemory,
    pfn vkGetBufferMemoryRequirements: PFN_vkGetBufferMemoryRequirements,
    pfn vkGetImageMemoryRequirements: PFN_vkGetImageMemoryRequirements,
    pfn vkGetImageSparseMemoryRequirements: PFN_vkGetImageSparseMemoryRequirements,
    pfn vkQueueBindSparse: PFN_vkQueueBindSparse,
    pfn vkCreateFence: PFN_vkCreateFence,
    pfn vkDestroyFence: PFN_vkDestroyFence,
    pfn vkResetFences: PFN_vkResetFences,
    pfn vkGetFenceStatus: PFN_vkGetFenceStatus,
    pfn vkWaitForFences: PFN_vkWaitForFences,
    pfn vkCreateSemaphore: PFN_vkCreateSemaphore,
    pfn vkDestroySemaphore: PFN_vkDestroySemaphore,
    pfn vkCreateEvent: PFN_vkCreateEvent,
    pfn vkDestroyEvent: PFN_vkDestroyEvent,
    pfn vkGetEventStatus: PFN_vkGetEventStatus,
    pfn vkSetEvent: PFN_vkSetEvent,
    pfn vkResetEvent: PFN_vkResetEvent,
    pfn vkCreateQueryPool: PFN_vkCreateQueryPool,
    pfn vkDestroyQueryPool: PFN_vkDestroyQueryPool,
    pfn vkGetQueryPoolResults: PFN_vkGetQueryPoolResults,
    pfn vkCreateBuffer: PFN_vkCreateBuffer,
    pfn vkDestroyBuffer: PFN_vkDestroyBuffer,
    pfn vkCreateBufferView: PFN_vkCreateBufferView,
    pfn vkDestroyBufferView: PFN_vkDestroyBufferView,
    pfn vkCreateImage: PFN_vkCreateImage,
    pfn vkDestroyImage: PFN_vkDestroyImage,
    pfn vkGetImageSubresourceLayout: PFN_vkGetImageSubresourceLayout,
    pfn vkCreateImageView: PFN_vkCreateImageView,
    pfn vkDestroyImageView: PFN_vkDestroyImageView,
    pfn vkCreateShaderModule: PFN_vkCreateShaderModule,
    pfn vkDestroyShaderModule: PFN_vkDestroyShaderModule,
    pfn vkCreatePipelineCache: PFN_vkCreatePipelineCache,
    pfn vkDestroyPipelineCache: PFN_vkDestroyPipelineCache,
    pfn vkGetPipelineCacheData: PFN_vkGetPipelineCacheData,
    pfn vkMergePipelineCaches: PFN_vkMergePipelineCaches,
    pfn vkCreateGraphicsPipelines: PFN_vkCreateGraphicsPipelines,
    pfn vkCreateComputePipelines: PFN_vkCreateComputePipelines,
    pfn vkDestroyPipeline: PFN_vkDestroyPipeline,
    pfn vkCreatePipelineLayout: PFN_vkCreatePipelineLayout,
    pfn vkDestroyPipelineLayout: PFN_vkDestroyPipelineLayout,
    pfn vkCreateSampler: PFN_vkCreateSampler,
    pfn vkDestroySampler: PFN_vkDestroySampler,
    pfn vkCreateDescriptorSetLayout: PFN_vkCreateDescriptorSetLayout,
    pfn vkDestroyDescriptorSetLayout: PFN_vkDestroyDescriptorSetLayout,
    pfn vkCreateDescriptorPool: PFN_vkCreateDescriptorPool,
    pfn vkDestroyDescriptorPool: PFN_vkDestroyDescriptorPool,
    pfn vkResetDescriptorPool: PFN_vkResetDescriptorPool,
    pfn vkAllocateDescriptorSets: PFN_vkAllocateDescriptorSets,
    pfn vkFreeDescriptorSets: PFN_vkFreeDescriptorSets,
    pfn vkUpdateDescriptorSets: PFN_vkUpdateDescriptorSets,
    pfn vkCreateFramebuffer: PFN_vkCreateFramebuffer,
    pfn vkDestroyFramebuffer: PFN_vkDestroyFramebuffer,
    pfn vkCreateRenderPass: PFN_vkCreateRenderPass,
    pfn vkDestroyRenderPass: PFN_vkDestroyRenderPass,
    pfn vkGetRenderAreaGranularity: PFN_vkGetRenderAreaGranularity,
    pfn vkCreateCommandPool: PFN_vkCreateCommandPool,
    pfn vkDestroyCommandPool: PFN_vkDestroyCommandPool,
    pfn vkResetCommandPool: PFN_vkResetCommandPool,
    pfn vkAllocateCommandBuffers: PFN_vkAllocateCommandBuffers,
    pfn vkFreeCommandBuffers: PFN_vkFreeCommandBuffers,
    pfn vkBeginCommandBuffer: PFN_vkBeginCommandBuffer,
    pfn vkEndCommandBuffer: PFN_vkEndCommandBuffer,
    pfn vkResetCommandBuffer: PFN_vkResetCommandBuffer,
    pfn vkCmdBindPipeline: PFN_vkCmdBindPipeline,
    pfn vkCmdSetViewport: PFN_vkCmdSetViewport,
    pfn vkCmdSetScissor: PFN_vkCmdSetScissor,
    pfn vkCmdSetLineWidth: PFN_vkCmdSetLineWidth,
    pfn vkCmdSetDepthBias: PFN_vkCmdSetDepthBias,
    pfn vkCmdSetBlendConstants: PFN_vkCmdSetBlendConstants,
    pfn vkCmdSetDepthBounds: PFN_vkCmdSetDepthBounds,
    pfn vkCmdSetStencilCompareMask: PFN_vkCmdSetStencilCompareMask,
    pfn vkCmdSetStencilWriteMask: PFN_vkCmdSetStencilWriteMask,
    pfn vkCmdSetStencilReference: PFN_vkCmdSetStencilReference,
    pfn vkCmdBindDescriptorSets: PFN_vkCmdBindDescriptorSets,
    pfn vkCmdBindIndexBuffer: PFN_vkCmdBindIndexBuffer,
    pfn vkCmdBindVertexBuffers: PFN_vkCmdBindVertexBuffers,
    pfn vkCmdDraw: PFN_vkCmdDraw,
    pfn vkCmdDrawIndexed: PFN_vkCmdDrawIndexed,
    pfn vkCmdDrawIndirect: PFN_vkCmdDrawIndirect,
    pfn vkCmdDrawIndexedIndirect: PFN_vkCmdDrawIndexedIndirect,
    pfn vkCmdDispatch: PFN_vkCmdDispatch,
    pfn vkCmdDispatchIndirect: PFN_vkCmdDispatchIndirect,
    pfn vkCmdCopyBuffer: PFN_vkCmdCopyBuffer,
    pfn vkCmdCopyImage: PFN_vkCmdCopyImage,
    pfn vkCmdBlitImage: PFN_vkCmdBlitImage,
    pfn vkCmdCopyBufferToImage: PFN_vkCmdCopyBufferToImage,
    pfn vkCmdCopyImageToBuffer: PFN_vkCmdCopyImageToBuffer,
    pfn vkCmdUpdateBuffer: PFN_vkCmdUpdateBuffer,
    pfn vkCmdFillBuffer: PFN_vkCmdFillBuffer,
    pfn vkCmdClearColorImage: PFN_vkCmdClearColorImage,
    pfn vkCmdClearDepthStencilImage: PFN_vkCmdClearDepthStencilImage,
    pfn vkCmdClearAttachments: PFN_vkCmdClearAttachments,
    pfn vkCmdResolveImage: PFN_vkCmdResolveImage,
    pfn vkCmdSetEvent: PFN_vkCmdSetEvent,
    pfn vkCmdResetEvent: PFN_vkCmdResetEvent,
    pfn vkCmdWaitEvents: PFN_vkCmdWaitEvents,
    pfn vkCmdPipelineBarrier: PFN_vkCmdPipelineBarrier,
    pfn vkCmdBeginQuery: PFN_vkCmdBeginQuery,
    pfn vkCmdEndQuery: PFN_vkCmdEndQuery,
    pfn vkCmdResetQueryPool: PFN_vkCmdResetQueryPool,
    pfn vkCmdWriteTimestamp: PFN_vkCmdWriteTimestamp,
    pfn vkCmdCopyQueryPoolResults: PFN_vkCmdCopyQueryPoolResults,
    pfn vkCmdPushConstants: PFN_vkCmdPushConstants,
    pfn vkCmdBeginRenderPass: PFN_vkCmdBeginRenderPass,
    pfn vkCmdNextSubpass: PFN_vkCmdNextSubpass,
    pfn vkCmdEndRenderPass: PFN_vkCmdEndRenderPass,
    pfn vkCmdExecuteCommands: PFN_vkCmdExecuteCommands,
});

#[cfg(feature = "khr_display_swapchain_9")]
addr_proc_struct!(KHR_display_swapchain {
    pfn vkCreateSharedSwapchainsKHR: PFN_vkCreateSharedSwapchainsKHR,
});

#[cfg(feature = "ext_debug_marker_3")]
addr_proc_struct!(EXT_debug_marker {
    pfn vkDebugMarkerSetObjectTagEXT: PFN_vkDebugMarkerSetObjectTagEXT,
    pfn vkDebugMarkerSetObjectNameEXT: PFN_vkDebugMarkerSetObjectNameEXT,
    pfn vkCmdDebugMarkerBeginEXT: PFN_vkCmdDebugMarkerBeginEXT,
    pfn vkCmdDebugMarkerEndEXT: PFN_vkCmdDebugMarkerEndEXT,
    pfn vkCmdDebugMarkerInsertEXT: PFN_vkCmdDebugMarkerInsertEXT,
});
