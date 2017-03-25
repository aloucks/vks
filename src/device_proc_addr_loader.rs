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
        }
    }

    pub unsafe fn load_core(&mut self, device: VkDevice) {
        self.core.load(self.vkGetDeviceProcAddr, device);
    }

    #[cfg(feature = "khr_display_swapchain_9")]
    pub unsafe fn load_khr_display_swapchain(&mut self, device: VkDevice) {
        self.khr_display_swapchain.load(self.vkGetDeviceProcAddr, device);
    }
}

macro_rules! addr_proc_struct {
    ($name:ident, { $([$cfg:expr] $symbol:ident: $ty:ty),+ }) => (
        pub struct $name {
            $(
                #[cfg(feature = $cfg)]
                pub $symbol: $ty,
            )+
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
                    #[cfg(feature = $cfg)]
                    debug_struct.field(stringify!($symbol), &(self.$symbol as *mut c_void));
                )+
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
                        #[cfg(feature = $cfg)]
                        $symbol: unsafe { mem::transmute(0usize) },
                    )*
                }
            }

            pub unsafe fn load(&mut self, vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr, device: VkDevice) {
                $(
                    #[cfg(feature = $cfg)]
                    { self.$symbol = mem::transmute((vkGetDeviceProcAddr)(device, concat!(stringify!($symbol), '\x00').as_ptr() as *const c_char)); }
                )+
            }
        }
    )
}

addr_proc_struct!(Core, {
    ["core_1_0_3"] vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    ["core_1_0_3"] vkDestroyDevice: PFN_vkDestroyDevice,
    ["core_1_0_3"] vkGetDeviceQueue: PFN_vkGetDeviceQueue,
    ["core_1_0_3"] vkQueueSubmit: PFN_vkQueueSubmit,
    ["core_1_0_3"] vkQueueWaitIdle: PFN_vkQueueWaitIdle,
    ["core_1_0_3"] vkDeviceWaitIdle: PFN_vkDeviceWaitIdle,
    ["core_1_0_3"] vkAllocateMemory: PFN_vkAllocateMemory,
    ["core_1_0_3"] vkFreeMemory: PFN_vkFreeMemory,
    ["core_1_0_3"] vkMapMemory: PFN_vkMapMemory,
    ["core_1_0_3"] vkUnmapMemory: PFN_vkUnmapMemory,
    ["core_1_0_3"] vkFlushMappedMemoryRanges: PFN_vkFlushMappedMemoryRanges,
    ["core_1_0_3"] vkInvalidateMappedMemoryRanges: PFN_vkInvalidateMappedMemoryRanges,
    ["core_1_0_3"] vkGetDeviceMemoryCommitment: PFN_vkGetDeviceMemoryCommitment,
    ["core_1_0_3"] vkBindBufferMemory: PFN_vkBindBufferMemory,
    ["core_1_0_3"] vkBindImageMemory: PFN_vkBindImageMemory,
    ["core_1_0_3"] vkGetBufferMemoryRequirements: PFN_vkGetBufferMemoryRequirements,
    ["core_1_0_3"] vkGetImageMemoryRequirements: PFN_vkGetImageMemoryRequirements,
    ["core_1_0_3"] vkGetImageSparseMemoryRequirements: PFN_vkGetImageSparseMemoryRequirements,
    ["core_1_0_3"] vkQueueBindSparse: PFN_vkQueueBindSparse,
    ["core_1_0_3"] vkCreateFence: PFN_vkCreateFence,
    ["core_1_0_3"] vkDestroyFence: PFN_vkDestroyFence,
    ["core_1_0_3"] vkResetFences: PFN_vkResetFences,
    ["core_1_0_3"] vkGetFenceStatus: PFN_vkGetFenceStatus,
    ["core_1_0_3"] vkWaitForFences: PFN_vkWaitForFences,
    ["core_1_0_3"] vkCreateSemaphore: PFN_vkCreateSemaphore,
    ["core_1_0_3"] vkDestroySemaphore: PFN_vkDestroySemaphore,
    ["core_1_0_3"] vkCreateEvent: PFN_vkCreateEvent,
    ["core_1_0_3"] vkDestroyEvent: PFN_vkDestroyEvent,
    ["core_1_0_3"] vkGetEventStatus: PFN_vkGetEventStatus,
    ["core_1_0_3"] vkSetEvent: PFN_vkSetEvent,
    ["core_1_0_3"] vkResetEvent: PFN_vkResetEvent,
    ["core_1_0_3"] vkCreateQueryPool: PFN_vkCreateQueryPool,
    ["core_1_0_3"] vkDestroyQueryPool: PFN_vkDestroyQueryPool,
    ["core_1_0_3"] vkGetQueryPoolResults: PFN_vkGetQueryPoolResults,
    ["core_1_0_3"] vkCreateBuffer: PFN_vkCreateBuffer,
    ["core_1_0_3"] vkDestroyBuffer: PFN_vkDestroyBuffer,
    ["core_1_0_3"] vkCreateBufferView: PFN_vkCreateBufferView,
    ["core_1_0_3"] vkDestroyBufferView: PFN_vkDestroyBufferView,
    ["core_1_0_3"] vkCreateImage: PFN_vkCreateImage,
    ["core_1_0_3"] vkDestroyImage: PFN_vkDestroyImage,
    ["core_1_0_3"] vkGetImageSubresourceLayout: PFN_vkGetImageSubresourceLayout,
    ["core_1_0_3"] vkCreateImageView: PFN_vkCreateImageView,
    ["core_1_0_3"] vkDestroyImageView: PFN_vkDestroyImageView,
    ["core_1_0_3"] vkCreateShaderModule: PFN_vkCreateShaderModule,
    ["core_1_0_3"] vkDestroyShaderModule: PFN_vkDestroyShaderModule,
    ["core_1_0_3"] vkCreatePipelineCache: PFN_vkCreatePipelineCache,
    ["core_1_0_3"] vkDestroyPipelineCache: PFN_vkDestroyPipelineCache,
    ["core_1_0_3"] vkGetPipelineCacheData: PFN_vkGetPipelineCacheData,
    ["core_1_0_3"] vkMergePipelineCaches: PFN_vkMergePipelineCaches,
    ["core_1_0_3"] vkCreateGraphicsPipelines: PFN_vkCreateGraphicsPipelines,
    ["core_1_0_3"] vkCreateComputePipelines: PFN_vkCreateComputePipelines,
    ["core_1_0_3"] vkDestroyPipeline: PFN_vkDestroyPipeline,
    ["core_1_0_3"] vkCreatePipelineLayout: PFN_vkCreatePipelineLayout,
    ["core_1_0_3"] vkDestroyPipelineLayout: PFN_vkDestroyPipelineLayout,
    ["core_1_0_3"] vkCreateSampler: PFN_vkCreateSampler,
    ["core_1_0_3"] vkDestroySampler: PFN_vkDestroySampler,
    ["core_1_0_3"] vkCreateDescriptorSetLayout: PFN_vkCreateDescriptorSetLayout,
    ["core_1_0_3"] vkDestroyDescriptorSetLayout: PFN_vkDestroyDescriptorSetLayout,
    ["core_1_0_3"] vkCreateDescriptorPool: PFN_vkCreateDescriptorPool,
    ["core_1_0_3"] vkDestroyDescriptorPool: PFN_vkDestroyDescriptorPool,
    ["core_1_0_3"] vkResetDescriptorPool: PFN_vkResetDescriptorPool,
    ["core_1_0_3"] vkAllocateDescriptorSets: PFN_vkAllocateDescriptorSets,
    ["core_1_0_3"] vkFreeDescriptorSets: PFN_vkFreeDescriptorSets,
    ["core_1_0_3"] vkUpdateDescriptorSets: PFN_vkUpdateDescriptorSets,
    ["core_1_0_3"] vkCreateFramebuffer: PFN_vkCreateFramebuffer,
    ["core_1_0_3"] vkDestroyFramebuffer: PFN_vkDestroyFramebuffer,
    ["core_1_0_3"] vkCreateRenderPass: PFN_vkCreateRenderPass,
    ["core_1_0_3"] vkDestroyRenderPass: PFN_vkDestroyRenderPass,
    ["core_1_0_3"] vkGetRenderAreaGranularity: PFN_vkGetRenderAreaGranularity,
    ["core_1_0_3"] vkCreateCommandPool: PFN_vkCreateCommandPool,
    ["core_1_0_3"] vkDestroyCommandPool: PFN_vkDestroyCommandPool,
    ["core_1_0_3"] vkResetCommandPool: PFN_vkResetCommandPool,
    ["core_1_0_3"] vkAllocateCommandBuffers: PFN_vkAllocateCommandBuffers,
    ["core_1_0_3"] vkFreeCommandBuffers: PFN_vkFreeCommandBuffers,
    ["core_1_0_3"] vkBeginCommandBuffer: PFN_vkBeginCommandBuffer,
    ["core_1_0_3"] vkEndCommandBuffer: PFN_vkEndCommandBuffer,
    ["core_1_0_3"] vkResetCommandBuffer: PFN_vkResetCommandBuffer,
    ["core_1_0_3"] vkCmdBindPipeline: PFN_vkCmdBindPipeline,
    ["core_1_0_3"] vkCmdSetViewport: PFN_vkCmdSetViewport,
    ["core_1_0_3"] vkCmdSetScissor: PFN_vkCmdSetScissor,
    ["core_1_0_3"] vkCmdSetLineWidth: PFN_vkCmdSetLineWidth,
    ["core_1_0_3"] vkCmdSetDepthBias: PFN_vkCmdSetDepthBias,
    ["core_1_0_3"] vkCmdSetBlendConstants: PFN_vkCmdSetBlendConstants,
    ["core_1_0_3"] vkCmdSetDepthBounds: PFN_vkCmdSetDepthBounds,
    ["core_1_0_3"] vkCmdSetStencilCompareMask: PFN_vkCmdSetStencilCompareMask,
    ["core_1_0_3"] vkCmdSetStencilWriteMask: PFN_vkCmdSetStencilWriteMask,
    ["core_1_0_3"] vkCmdSetStencilReference: PFN_vkCmdSetStencilReference,
    ["core_1_0_3"] vkCmdBindDescriptorSets: PFN_vkCmdBindDescriptorSets,
    ["core_1_0_3"] vkCmdBindIndexBuffer: PFN_vkCmdBindIndexBuffer,
    ["core_1_0_3"] vkCmdBindVertexBuffers: PFN_vkCmdBindVertexBuffers,
    ["core_1_0_3"] vkCmdDraw: PFN_vkCmdDraw,
    ["core_1_0_3"] vkCmdDrawIndexed: PFN_vkCmdDrawIndexed,
    ["core_1_0_3"] vkCmdDrawIndirect: PFN_vkCmdDrawIndirect,
    ["core_1_0_3"] vkCmdDrawIndexedIndirect: PFN_vkCmdDrawIndexedIndirect,
    ["core_1_0_3"] vkCmdDispatch: PFN_vkCmdDispatch,
    ["core_1_0_3"] vkCmdDispatchIndirect: PFN_vkCmdDispatchIndirect,
    ["core_1_0_3"] vkCmdCopyBuffer: PFN_vkCmdCopyBuffer,
    ["core_1_0_3"] vkCmdCopyImage: PFN_vkCmdCopyImage,
    ["core_1_0_3"] vkCmdBlitImage: PFN_vkCmdBlitImage,
    ["core_1_0_3"] vkCmdCopyBufferToImage: PFN_vkCmdCopyBufferToImage,
    ["core_1_0_3"] vkCmdCopyImageToBuffer: PFN_vkCmdCopyImageToBuffer,
    ["core_1_0_3"] vkCmdUpdateBuffer: PFN_vkCmdUpdateBuffer,
    ["core_1_0_3"] vkCmdFillBuffer: PFN_vkCmdFillBuffer,
    ["core_1_0_3"] vkCmdClearColorImage: PFN_vkCmdClearColorImage,
    ["core_1_0_3"] vkCmdClearDepthStencilImage: PFN_vkCmdClearDepthStencilImage,
    ["core_1_0_3"] vkCmdClearAttachments: PFN_vkCmdClearAttachments,
    ["core_1_0_3"] vkCmdResolveImage: PFN_vkCmdResolveImage,
    ["core_1_0_3"] vkCmdSetEvent: PFN_vkCmdSetEvent,
    ["core_1_0_3"] vkCmdResetEvent: PFN_vkCmdResetEvent,
    ["core_1_0_3"] vkCmdWaitEvents: PFN_vkCmdWaitEvents,
    ["core_1_0_3"] vkCmdPipelineBarrier: PFN_vkCmdPipelineBarrier,
    ["core_1_0_3"] vkCmdBeginQuery: PFN_vkCmdBeginQuery,
    ["core_1_0_3"] vkCmdEndQuery: PFN_vkCmdEndQuery,
    ["core_1_0_3"] vkCmdResetQueryPool: PFN_vkCmdResetQueryPool,
    ["core_1_0_3"] vkCmdWriteTimestamp: PFN_vkCmdWriteTimestamp,
    ["core_1_0_3"] vkCmdCopyQueryPoolResults: PFN_vkCmdCopyQueryPoolResults,
    ["core_1_0_3"] vkCmdPushConstants: PFN_vkCmdPushConstants,
    ["core_1_0_3"] vkCmdBeginRenderPass: PFN_vkCmdBeginRenderPass,
    ["core_1_0_3"] vkCmdNextSubpass: PFN_vkCmdNextSubpass,
    ["core_1_0_3"] vkCmdEndRenderPass: PFN_vkCmdEndRenderPass,
    ["core_1_0_3"] vkCmdExecuteCommands: PFN_vkCmdExecuteCommands
});

#[cfg(feature = "khr_display_swapchain_9")]
addr_proc_struct!(KHR_display_swapchain, {
    ["khr_display_swapchain_9"] vkCreateSharedSwapchainsKHR: PFN_vkCreateSharedSwapchainsKHR
});
