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

macro_rules! gen_device_proc_addr_loader {
    ( $( $cond:expr => $field:ident: $ty:ident [fn $load:ident], )* ) => {
        pub struct DeviceProcAddrLoader {
            pub vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,

            $(
                #[cfg(feature = $cond)]
                pub $field: $ty,
            )*
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
                    #[cfg(feature = $cond)]
                    debug_struct.field(stringify!($field), &self.$field);
                )*

                debug_struct.finish()
            }
        }

        impl DeviceProcAddrLoader {
            pub fn new(vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr) -> Self {
                DeviceProcAddrLoader {
                    vkGetDeviceProcAddr: vkGetDeviceProcAddr,

                    $(
                        #[cfg(feature = $cond)]
                        $field: $ty::new(),
                    )*
                }
            }

            $(
                #[cfg(feature = $cond)]
                pub unsafe fn $load(&mut self, device: VkDevice) {
                    self.$field.load(self.vkGetDeviceProcAddr, device);
                }
            )*
        }
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

gen_device_proc_addr_loader!(
    "core_1_0_3" => core: Core [fn load_core],
    "khr_display_swapchain_9" => khr_display_swapchain: KHR_display_swapchain [fn load_khr_display_swapchain],
    "ext_debug_marker_3" => ext_debug_marker: EXT_debug_marker [fn load_ext_debug_marker],
    "amd_draw_indirect_count_1" => amd_draw_indirect_count: AMD_draw_indirect_count [fn load_amd_draw_indirect_count],
    "nvx_device_generated_commands_1" => nvx_device_generated_commands: NVX_device_generated_commands [fn load_nvx_device_generated_commands],
    "khr_maintenance1_1" => khr_maintenance1: KHR_maintenance1 [fn load_khr_maintenance1],
    "ext_display_control_1" => ext_display_control: EXT_display_control [fn load_ext_display_control],
    "khr_push_descriptor_1" => khr_push_descriptor: KHR_push_descriptor [fn load_khr_push_descriptor],
    "khr_descriptor_update_template_1" => khr_descriptor_update_template: KHR_descriptor_update_template [fn load_khr_descriptor_update_template],
    "khx_device_group_1" => khx_device_group: KHX_device_group [fn load_khx_device_group],
    "khx_external_memory_win32_1" => khx_external_memory_win32: KHX_external_memory_win32 [fn load_khx_external_memory_win32],
    "khx_external_memory_fd_1" => khx_external_memory_fd: KHX_external_memory_fd [fn load_khx_external_memory_fd],
);

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

#[cfg(feature = "amd_draw_indirect_count_1")]
addr_proc_struct!(AMD_draw_indirect_count {
    pfn vkCmdDrawIndirectCountAMD: PFN_vkCmdDrawIndirectCountAMD,
    pfn vkCmdDrawIndexedIndirectCountAMD: PFN_vkCmdDrawIndexedIndirectCountAMD,
});

#[cfg(feature = "nvx_device_generated_commands_1")]
addr_proc_struct!(NVX_device_generated_commands {
    pfn vkCmdProcessCommandsNVX: PFN_vkCmdProcessCommandsNVX,
    pfn vkCmdReserveSpaceForCommandsNVX: PFN_vkCmdReserveSpaceForCommandsNVX,
    pfn vkCreateIndirectCommandsLayoutNVX: PFN_vkCreateIndirectCommandsLayoutNVX,
    pfn vkDestroyIndirectCommandsLayoutNVX: PFN_vkDestroyIndirectCommandsLayoutNVX,
    pfn vkCreateObjectTableNVX: PFN_vkCreateObjectTableNVX,
    pfn vkDestroyObjectTableNVX: PFN_vkDestroyObjectTableNVX,
    pfn vkRegisterObjectsNVX: PFN_vkRegisterObjectsNVX,
    pfn vkUnregisterObjectsNVX: PFN_vkUnregisterObjectsNVX,
});

#[cfg(feature = "khr_maintenance1_1")]
addr_proc_struct!(KHR_maintenance1 {
    pfn vkTrimCommandPoolKHR: PFN_vkTrimCommandPoolKHR,
});

#[cfg(feature = "ext_display_control_1")]
addr_proc_struct!(EXT_display_control {
    pfn vkDisplayPowerControlEXT: PFN_vkDisplayPowerControlEXT,
    pfn vkRegisterDeviceEventEXT: PFN_vkRegisterDeviceEventEXT,
    pfn vkRegisterDisplayEventEXT: PFN_vkRegisterDisplayEventEXT,
    pfn vkGetSwapchainCounterEXT: PFN_vkGetSwapchainCounterEXT,
});

#[cfg(feature = "khr_push_descriptor_1")]
addr_proc_struct!(KHR_push_descriptor {
    pfn vkCmdPushDescriptorSetKHR: PFN_vkCmdPushDescriptorSetKHR,
});

#[cfg(feature = "khr_descriptor_update_template_1")]
addr_proc_struct!(KHR_descriptor_update_template {
    pfn vkCreateDescriptorUpdateTemplateKHR: PFN_vkCreateDescriptorUpdateTemplateKHR,
    pfn vkDestroyDescriptorUpdateTemplateKHR: PFN_vkDestroyDescriptorUpdateTemplateKHR,
    pfn vkUpdateDescriptorSetWithTemplateKHR: PFN_vkUpdateDescriptorSetWithTemplateKHR,
    pfn vkCmdPushDescriptorSetWithTemplateKHR: PFN_vkCmdPushDescriptorSetWithTemplateKHR,
});

#[cfg(feature = "khx_device_group_1")]
addr_proc_struct!(KHX_device_group {
    pfn vkGetDeviceGroupPeerMemoryFeaturesKHX: PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX,
    pfn vkBindBufferMemory2KHX: PFN_vkBindBufferMemory2KHX,
    pfn vkBindImageMemory2KHX: PFN_vkBindImageMemory2KHX,
    pfn vkCmdSetDeviceMaskKHX: PFN_vkCmdSetDeviceMaskKHX,
    pfn vkGetDeviceGroupPresentCapabilitiesKHX: PFN_vkGetDeviceGroupPresentCapabilitiesKHX,
    pfn vkGetDeviceGroupSurfacePresentModesKHX: PFN_vkGetDeviceGroupSurfacePresentModesKHX,
    pfn vkAcquireNextImage2KHX: PFN_vkAcquireNextImage2KHX,
    pfn vkCmdDispatchBaseKHX: PFN_vkCmdDispatchBaseKHX,
});

#[cfg(feature = "khx_external_memory_win32_1")]
addr_proc_struct!(KHX_external_memory_win32 {
    pfn vkGetMemoryWin32HandleKHX: PFN_vkGetMemoryWin32HandleKHX,
    pfn vkGetMemoryWin32HandlePropertiesKHX: PFN_vkGetMemoryWin32HandlePropertiesKHX,
});

#[cfg(feature = "khx_external_memory_fd_1")]
addr_proc_struct!(KHX_external_memory_fd {
    pfn vkGetMemoryFdKHX: PFN_vkGetMemoryFdKHX,
    pfn vkGetMemoryFdPropertiesKHX: PFN_vkGetMemoryFdPropertiesKHX,
});
