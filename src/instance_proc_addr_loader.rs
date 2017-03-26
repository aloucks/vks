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

pub struct InstanceProcAddrLoader {
    pub vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr,
    pub core_null_instance: CoreNullInstance,
    pub core: Core,

    #[cfg(feature = "khr_surface_25")]
    pub khr_surface: KHR_surface,

    #[cfg(feature = "khr_display_21")]
    pub khr_display: KHR_display,

    #[cfg(feature = "khr_display_swapchain_9")]
    pub khr_display_swapchain: KHR_display_swapchain,

    #[cfg(feature = "khr_xlib_surface_6")]
    pub khr_xlib_surface: KHR_xlib_surface,

    #[cfg(feature = "khr_xcb_surface_6")]
    pub khr_xcb_surface: KHR_xcb_surface,

    #[cfg(feature = "khr_wayland_surface_5")]
    pub khr_wayland_surface: KHR_wayland_surface,

    #[cfg(feature = "khr_mir_surface_4")]
    pub khr_mir_surface: KHR_mir_surface,

    #[cfg(feature = "khr_android_surface_6")]
    pub khr_android_surface: KHR_android_surface,

    #[cfg(feature = "khr_win32_surface_5")]
    pub khr_win32_surface: KHR_win32_surface,

    #[cfg(feature = "ext_debug_report_1")]
    pub ext_debug_report: EXT_debug_report,

    #[cfg(feature = "ext_debug_marker_3")]
    pub ext_debug_marker: EXT_debug_marker,

    #[cfg(feature = "amd_draw_indirect_count_1")]
    pub amd_draw_indirect_count: AMD_draw_indirect_count,
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
        debug_struct.field("core", &self.core);

        #[cfg(feature = "khr_surface_25")]
        debug_struct.field("khr_surface", &self.khr_surface);

        #[cfg(feature = "khr_display_21")]
        debug_struct.field("khr_display", &self.khr_display);

        #[cfg(feature = "khr_display_swapchain_9")]
        debug_struct.field("khr_display_swapchain", &self.khr_display_swapchain);

        #[cfg(feature = "khr_xlib_surface_6")]
        debug_struct.field("khr_xlib_surface", &self.khr_xlib_surface);

        #[cfg(feature = "khr_xcb_surface_6")]
        debug_struct.field("khr_xcb_surface", &self.khr_xcb_surface);

        #[cfg(feature = "khr_wayland_surface_5")]
        debug_struct.field("khr_wayland_surface", &self.khr_wayland_surface);

        #[cfg(feature = "khr_mir_surface_4")]
        debug_struct.field("khr_mir_surface", &self.khr_mir_surface);

        #[cfg(feature = "khr_android_surface_6")]
        debug_struct.field("khr_android_surface", &self.khr_android_surface);

        #[cfg(feature = "khr_win32_surface_5")]
        debug_struct.field("khr_win32_surface", &self.khr_win32_surface);

        #[cfg(feature = "ext_debug_report_1")]
        debug_struct.field("ext_debug_report", &self.ext_debug_report);

        #[cfg(feature = "ext_debug_marker_3")]
        debug_struct.field("ext_debug_marker", &self.ext_debug_marker);

        #[cfg(feature = "amd_draw_indirect_count_1")]
        debug_struct.field("amd_draw_indirect_count", &self.amd_draw_indirect_count);

        debug_struct.finish()
    }
}

impl InstanceProcAddrLoader {
    pub fn new(vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr) -> Self {
        InstanceProcAddrLoader {
            vkGetInstanceProcAddr: vkGetInstanceProcAddr,
            core_null_instance: CoreNullInstance::new(),
            core: Core::new(),

            #[cfg(feature = "khr_surface_25")]
            khr_surface: KHR_surface::new(),

            #[cfg(feature = "khr_display_21")]
            khr_display: KHR_display::new(),

            #[cfg(feature = "khr_display_swapchain_9")]
            khr_display_swapchain: KHR_display_swapchain::new(),

            #[cfg(feature = "khr_xlib_surface_6")]
            khr_xlib_surface: KHR_xlib_surface::new(),

            #[cfg(feature = "khr_xcb_surface_6")]
            khr_xcb_surface: KHR_xcb_surface::new(),

            #[cfg(feature = "khr_wayland_surface_5")]
            khr_wayland_surface: KHR_wayland_surface::new(),

            #[cfg(feature = "khr_mir_surface_4")]
            khr_mir_surface: KHR_mir_surface::new(),

            #[cfg(feature = "khr_android_surface_6")]
            khr_android_surface: KHR_android_surface::new(),

            #[cfg(feature = "khr_win32_surface_5")]
            khr_win32_surface: KHR_win32_surface::new(),

            #[cfg(feature = "ext_debug_report_1")]
            ext_debug_report: EXT_debug_report::new(),

            #[cfg(feature = "ext_debug_marker_3")]
            ext_debug_marker: EXT_debug_marker::new(),

            #[cfg(feature = "amd_draw_indirect_count_1")]
            amd_draw_indirect_count: AMD_draw_indirect_count::new(),
        }
    }

    pub unsafe fn load_core_null_instance(&mut self) {
        self.core_null_instance.load(self.vkGetInstanceProcAddr, ptr::null_mut());
    }

    pub unsafe fn load_core(&mut self, instance: VkInstance) {
        self.core.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_surface_25")]
    pub unsafe fn load_khr_surface(&mut self, instance: VkInstance) {
        self.khr_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_display_21")]
    pub unsafe fn load_khr_display(&mut self, instance: VkInstance) {
        self.khr_display.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_display_swapchain_9")]
    pub unsafe fn load_khr_display_swapchain(&mut self, instance: VkInstance) {
        self.khr_display_swapchain.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_xlib_surface_6")]
    pub unsafe fn load_khr_xlib_surface(&mut self, instance: VkInstance) {
        self.khr_xlib_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_xcb_surface_6")]
    pub unsafe fn load_khr_xcb_surface(&mut self, instance: VkInstance) {
        self.khr_xcb_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_wayland_surface_5")]
    pub unsafe fn load_khr_wayland_surface(&mut self, instance: VkInstance) {
        self.khr_wayland_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_mir_surface_4")]
    pub unsafe fn load_khr_mir_surface(&mut self, instance: VkInstance) {
        self.khr_mir_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_android_surface_6")]
    pub unsafe fn load_khr_android_surface(&mut self, instance: VkInstance) {
        self.khr_android_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "khr_win32_surface_5")]
    pub unsafe fn load_khr_win32_surface(&mut self, instance: VkInstance) {
        self.khr_win32_surface.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "ext_debug_report_1")]
    pub unsafe fn load_ext_debug_report(&mut self, instance: VkInstance) {
        self.ext_debug_report.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "ext_debug_marker_3")]
    pub unsafe fn load_ext_debug_marker(&mut self, instance: VkInstance) {
        self.ext_debug_marker.load(self.vkGetInstanceProcAddr, instance);
    }

    #[cfg(feature = "amd_draw_indirect_count_1")]
    pub unsafe fn load_amd_draw_indirect_count(&mut self, instance: VkInstance) {
        self.amd_draw_indirect_count.load(self.vkGetInstanceProcAddr, instance);
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
