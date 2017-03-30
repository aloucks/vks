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
use std::ptr;

macro_rules! gen_instance_proc_addr_loader {
    ( $( $cond:expr => $field:ident: $ty:ident [fn $load:ident], )* ) => {
        pub struct InstanceProcAddrLoader {
            pub vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr,
            pub core_null_instance: CoreNullInstance,

            $(
                #[cfg(feature = $cond)]
                pub $field: $ty,
            )*
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
                debug_struct.field("core_null_instance", &self.core_null_instance);

                $(
                    #[cfg(feature = $cond)]
                    debug_struct.field(stringify!($field), &self.$field);
                )*

                debug_struct.finish()
            }
        }

        impl InstanceProcAddrLoader {
            pub fn new(vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr) -> Self {
                InstanceProcAddrLoader {
                    vkGetInstanceProcAddr: vkGetInstanceProcAddr,
                    core_null_instance: CoreNullInstance::new(),

                    $(
                        #[cfg(feature = $cond)]
                        $field: $ty::new(),
                    )*
                }
            }

            pub unsafe fn load_core_null_instance(&mut self) {
                self.core_null_instance.load(self.vkGetInstanceProcAddr, ptr::null_mut());
            }

            $(
                #[cfg(feature = $cond)]
                pub unsafe fn $load(&mut self, instance: VkInstance) {
                    self.$field.load(self.vkGetInstanceProcAddr, instance);
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
            pub unsafe fn load(&mut self, vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr, instance: VkInstance) {
                $(
                    $(#[$cond])*
                    { self.$symbol = mem::transmute((vkGetInstanceProcAddr)(instance, concat!(stringify!($symbol), '\x00').as_ptr() as *const c_char)); }
                )*
            }
        }
    )
}

gen_instance_proc_addr_loader!(
    "core_1_0_3" => core: Core [fn load_core],
    "khr_surface_25" => khr_surface: KHR_surface [fn load_khr_surface],
    "khr_display_21" => khr_display: KHR_display [fn load_khr_display],
    "khr_display_swapchain_9" => khr_display_swapchain: KHR_display_swapchain [fn load_khr_display_swapchain],
    "khr_xlib_surface_6" => khr_xlib_surface: KHR_xlib_surface [fn load_khr_xlib_surface],
    "khr_xcb_surface_6" => khr_xcb_surface: KHR_xcb_surface [fn load_khr_xcb_surface],
    "khr_wayland_surface_5" => khr_wayland_surface: KHR_wayland_surface [fn load_khr_wayland_surface],
    "khr_mir_surface_4" => khr_mir_surface: KHR_mir_surface [fn load_khr_mir_surface],
    "khr_android_surface_6" => khr_android_surface: KHR_android_surface [fn load_khr_android_surface],
    "khr_win32_surface_5" => khr_win32_surface: KHR_win32_surface [fn load_khr_win32_surface],
    "ext_debug_report_1" => ext_debug_report: EXT_debug_report [fn load_ext_debug_report],
    "ext_debug_marker_3" => ext_debug_marker: EXT_debug_marker [fn load_ext_debug_marker],
    "amd_draw_indirect_count_1" => amd_draw_indirect_count: AMD_draw_indirect_count [fn load_amd_draw_indirect_count],
    "nv_external_memory_capabilities_1" => nv_external_memory_capabilities: NV_external_memory_capabilities [fn load_nv_external_memory_capabilities],
    "nv_external_memory_win32_1" => nv_external_memory_win32: NV_external_memory_win32 [fn load_nv_external_memory_win32],
    "nvx_device_generated_commands_1" => nvx_device_generated_commands: NVX_device_generated_commands [fn load_nvx_device_generated_commands],
    "khr_get_physical_device_properties2_1" => khr_get_physical_device_properties2: KHR_get_physical_device_properties2 [fn load_khr_get_physical_device_properties2],
    "khr_maintenance1_1" => khr_maintenance1: KHR_maintenance1 [fn load_khr_maintenance1],
);

addr_proc_struct!(CoreNullInstance {
    pfn vkEnumerateInstanceExtensionProperties: PFN_vkEnumerateInstanceExtensionProperties,
    pfn vkEnumerateInstanceLayerProperties: PFN_vkEnumerateInstanceLayerProperties,
    pfn vkCreateInstance: PFN_vkCreateInstance,
});

addr_proc_struct!(Core {
    pfn vkDestroyInstance: PFN_vkDestroyInstance,
    pfn vkEnumeratePhysicalDevices: PFN_vkEnumeratePhysicalDevices,
    pfn vkGetPhysicalDeviceFeatures: PFN_vkGetPhysicalDeviceFeatures,
    pfn vkGetPhysicalDeviceFormatProperties: PFN_vkGetPhysicalDeviceFormatProperties,
    pfn vkGetPhysicalDeviceImageFormatProperties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    pfn vkGetPhysicalDeviceProperties: PFN_vkGetPhysicalDeviceProperties,
    pfn vkGetPhysicalDeviceQueueFamilyProperties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    pfn vkGetPhysicalDeviceMemoryProperties: PFN_vkGetPhysicalDeviceMemoryProperties,
    pfn vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    pfn vkCreateDevice: PFN_vkCreateDevice,
    pfn vkDestroyDevice: PFN_vkDestroyDevice,
    pfn vkEnumerateDeviceExtensionProperties: PFN_vkEnumerateDeviceExtensionProperties,
    pfn vkEnumerateDeviceLayerProperties: PFN_vkEnumerateDeviceLayerProperties,
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
    pfn vkGetPhysicalDeviceSparseImageFormatProperties: PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
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

#[cfg(feature = "khr_surface_25")]
addr_proc_struct!(KHR_surface {
    pfn vkDestroySurfaceKHR: PFN_vkDestroySurfaceKHR,
    pfn vkGetPhysicalDeviceSurfaceSupportKHR: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    pfn vkGetPhysicalDeviceSurfaceCapabilitiesKHR: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pfn vkGetPhysicalDeviceSurfaceFormatsKHR: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    pfn vkGetPhysicalDeviceSurfacePresentModesKHR: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
});

#[cfg(feature = "khr_display_21")]
addr_proc_struct!(KHR_display {
    pfn vkGetPhysicalDeviceDisplayPropertiesKHR: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    pfn vkGetPhysicalDeviceDisplayPlanePropertiesKHR: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    pfn vkGetDisplayPlaneSupportedDisplaysKHR: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    pfn vkGetDisplayModePropertiesKHR: PFN_vkGetDisplayModePropertiesKHR,
    pfn vkCreateDisplayModeKHR: PFN_vkCreateDisplayModeKHR,
    pfn vkGetDisplayPlaneCapabilitiesKHR: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    pfn vkCreateDisplayPlaneSurfaceKHR: PFN_vkCreateDisplayPlaneSurfaceKHR,
});

#[cfg(feature = "khr_display_swapchain_9")]
addr_proc_struct!(KHR_display_swapchain {
    pfn vkCreateSharedSwapchainsKHR: PFN_vkCreateSharedSwapchainsKHR,
});

#[cfg(feature = "khr_xlib_surface_6")]
addr_proc_struct!(KHR_xlib_surface {
    pfn vkCreateXlibSurfaceKHR: PFN_vkCreateXlibSurfaceKHR,
    pfn vkGetPhysicalDeviceXlibPresentationSupportKHR: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
});

#[cfg(feature = "khr_xcb_surface_6")]
addr_proc_struct!(KHR_xcb_surface {
    pfn vkCreateXcbSurfaceKHR: PFN_vkCreateXcbSurfaceKHR,
    pfn vkGetPhysicalDeviceXcbPresentationSupportKHR: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
});

#[cfg(feature = "khr_wayland_surface_5")]
addr_proc_struct!(KHR_wayland_surface {
    pfn vkCreateWaylandSurfaceKHR: PFN_vkCreateWaylandSurfaceKHR,
    pfn vkGetPhysicalDeviceWaylandPresentationSupportKHR: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
});

#[cfg(feature = "khr_mir_surface_4")]
addr_proc_struct!(KHR_mir_surface {
    pfn vkCreateMirSurfaceKHR: PFN_vkCreateMirSurfaceKHR,
    pfn vkGetPhysicalDeviceMirPresentationSupportKHR: PFN_vkGetPhysicalDeviceMirPresentationSupportKHR,
});

#[cfg(feature = "khr_android_surface_6")]
addr_proc_struct!(KHR_android_surface {
    pfn vkCreateAndroidSurfaceKHR: PFN_vkCreateAndroidSurfaceKHR,
});

#[cfg(feature = "khr_win32_surface_5")]
addr_proc_struct!(KHR_win32_surface {
    pfn vkCreateWin32SurfaceKHR: PFN_vkCreateWin32SurfaceKHR,
    pfn vkGetPhysicalDeviceWin32PresentationSupportKHR: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
});

#[cfg(feature = "ext_debug_report_1")]
addr_proc_struct!(EXT_debug_report {
    pfn vkCreateDebugReportCallbackEXT: PFN_vkCreateDebugReportCallbackEXT,
    pfn vkDestroyDebugReportCallbackEXT: PFN_vkDestroyDebugReportCallbackEXT,
    pfn vkDebugReportMessageEXT: PFN_vkDebugReportMessageEXT,
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

#[cfg(feature = "nv_external_memory_capabilities_1")]
addr_proc_struct!(NV_external_memory_capabilities {
    pfn vkGetPhysicalDeviceExternalImageFormatPropertiesNV: PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
});

#[cfg(feature = "nv_external_memory_win32_1")]
addr_proc_struct!(NV_external_memory_win32 {
    pfn vkGetMemoryWin32HandleNV: PFN_vkGetMemoryWin32HandleNV,
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
    pfn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX: PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX,
});

#[cfg(feature = "khr_get_physical_device_properties2_1")]
addr_proc_struct!(KHR_get_physical_device_properties2 {
    pfn vkGetPhysicalDeviceFeatures2KHR: PFN_vkGetPhysicalDeviceFeatures2KHR,
    pfn vkGetPhysicalDeviceProperties2KHR: PFN_vkGetPhysicalDeviceProperties2KHR,
    pfn vkGetPhysicalDeviceFormatProperties2KHR: PFN_vkGetPhysicalDeviceFormatProperties2KHR,
    pfn vkGetPhysicalDeviceImageFormatProperties2KHR: PFN_vkGetPhysicalDeviceImageFormatProperties2KHR,
    pfn vkGetPhysicalDeviceQueueFamilyProperties2KHR: PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR,
    pfn vkGetPhysicalDeviceMemoryProperties2KHR: PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
    pfn vkGetPhysicalDeviceSparseImageFormatProperties2KHR: PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR,
});

#[cfg(feature = "khr_maintenance1_1")]
addr_proc_struct!(KHR_maintenance1 {
    pfn vkTrimCommandPoolKHR: PFN_vkTrimCommandPoolKHR,
});
