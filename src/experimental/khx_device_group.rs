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

//! [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)

use core::ptr;
use khr_surface;
use khr_swapchain;
use libc::c_void;
use vk;

pub const VK_KHX_DEVICE_GROUP_SPEC_VERSION: u32 = 1;
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME: &'static [u8; 20] = b"VK_KHX_device_group\x00";
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME_STR: &'static str = "VK_KHX_device_group";
pub const VK_MAX_DEVICE_GROUP_SIZE_KHX: usize = 32;

vks_bitflags! {
    /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkPeerMemoryFeatureFlagsKHX: u32 {
        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        const VK_PEER_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHX = 0x00000001;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHX = 0x00000002;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHX = 0x00000004;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHX = 0x00000008;
    }
}

/// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
pub type VkPeerMemoryFeatureFlagBitsKHX = VkPeerMemoryFeatureFlagsKHX;

vks_bitflags! {
    /// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkMemoryAllocateFlagsKHX: u32 {
        /// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
        const VK_MEMORY_ALLOCATE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
        const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHX = 0x00000001;
    }
}

/// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
pub type VkMemoryAllocateFlagBitsKHX = VkMemoryAllocateFlagsKHX;

vks_bitflags! {
    /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDeviceGroupPresentModeFlagsKHX: u32 {
        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        const VK_DEVICE_GROUP_PRESENT_MODE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX = 0x00000001;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX = 0x00000002;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX = 0x00000004;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX = 0x00000008;
    }
}

/// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
pub type VkDeviceGroupPresentModeFlagBitsKHX = VkDeviceGroupPresentModeFlagsKHX;

/// See [`VkMemoryAllocateFlagBitsInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryAllocateFlagsInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlagsKHX,
    pub deviceMask: u32,
}

impl Default for VkMemoryAllocateFlagsInfoKHX {
    fn default() -> Self {
        VkMemoryAllocateFlagsInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX,
            pNext: ptr::null(),
            flags: Default::default(),
            deviceMask: Default::default(),
        }
    }
}

/// See [`VkBindBufferMemoryInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindBufferMemoryInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindBufferMemoryInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub buffer: vk::VkBuffer,
    pub memory: vk::VkDeviceMemory,
    pub memoryOffset: vk::VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

impl Default for VkBindBufferMemoryInfoKHX {
    fn default() -> Self {
        VkBindBufferMemoryInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX,
            pNext: ptr::null(),
            buffer: Default::default(),
            memory: Default::default(),
            memoryOffset: Default::default(),
            deviceIndexCount: Default::default(),
            pDeviceIndices: ptr::null(),
        }
    }
}

/// See [`VkBindImageMemoryInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindImageMemoryInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindImageMemoryInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub image: vk::VkImage,
    pub memory: vk::VkDeviceMemory,
    pub memoryOffset: vk::VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub SFRRectCount: u32,
    pub pSFRRects: *const vk::VkRect2D,
}

impl Default for VkBindImageMemoryInfoKHX {
    fn default() -> Self {
        VkBindImageMemoryInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX,
            pNext: ptr::null(),
            image: Default::default(),
            memory: Default::default(),
            memoryOffset: Default::default(),
            deviceIndexCount: Default::default(),
            pDeviceIndices: ptr::null(),
            SFRRectCount: Default::default(),
            pSFRRects: ptr::null(),
        }
    }
}

/// See [`VkDeviceGroupRenderPassBeginInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupRenderPassBeginInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupRenderPassBeginInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const vk::VkRect2D,
}

impl Default for VkDeviceGroupRenderPassBeginInfoKHX {
    fn default() -> Self {
        VkDeviceGroupRenderPassBeginInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX,
            pNext: ptr::null(),
            deviceMask: Default::default(),
            deviceRenderAreaCount: Default::default(),
            pDeviceRenderAreas: ptr::null(),
        }
    }
}

/// See [`VkDeviceGroupCommandBufferBeginInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupCommandBufferBeginInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
}

impl Default for VkDeviceGroupCommandBufferBeginInfoKHX {
    fn default() -> Self {
        VkDeviceGroupCommandBufferBeginInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX,
            pNext: ptr::null(),
            deviceMask: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupSubmitInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupSubmitInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupSubmitInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}

impl Default for VkDeviceGroupSubmitInfoKHX {
    fn default() -> Self {
        VkDeviceGroupSubmitInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX,
            pNext: ptr::null(),
            waitSemaphoreCount: Default::default(),
            pWaitSemaphoreDeviceIndices: ptr::null(),
            commandBufferCount: Default::default(),
            pCommandBufferDeviceMasks: ptr::null(),
            signalSemaphoreCount: Default::default(),
            pSignalSemaphoreDeviceIndices: ptr::null(),
        }
    }
}

/// See [`VkDeviceGroupBindSparseInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupBindSparseInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupBindSparseInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

impl Default for VkDeviceGroupBindSparseInfoKHX {
    fn default() -> Self {
        VkDeviceGroupBindSparseInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX,
            pNext: ptr::null(),
            resourceDeviceIndex: Default::default(),
            memoryDeviceIndex: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentCapabilitiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupPresentCapabilitiesKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE_KHX],
    pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

impl Default for VkDeviceGroupPresentCapabilitiesKHX {
    fn default() -> Self {
        VkDeviceGroupPresentCapabilitiesKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX,
            pNext: ptr::null(),
            presentMask: Default::default(),
            modes: Default::default(),
        }
    }
}

/// See [`VkImageSwapchainCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSwapchainCreateInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSwapchainCreateInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: khr_swapchain::VkSwapchainKHR,
}

impl Default for VkImageSwapchainCreateInfoKHX {
    fn default() -> Self {
        VkImageSwapchainCreateInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX,
            pNext: ptr::null(),
            swapchain: Default::default(),
        }
    }
}

/// See [`VkBindImageMemorySwapchainInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindImageMemorySwapchainInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindImageMemorySwapchainInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: khr_swapchain::VkSwapchainKHR,
    pub imageIndex: u32,
}

impl Default for VkBindImageMemorySwapchainInfoKHX {
    fn default() -> Self {
        VkBindImageMemorySwapchainInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX,
            pNext: ptr::null(),
            swapchain: Default::default(),
            imageIndex: Default::default(),
        }
    }
}

/// See [`VkAcquireNextImageInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAcquireNextImageInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAcquireNextImageInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: khr_swapchain::VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: vk::VkSemaphore,
    pub fence: vk::VkFence,
    pub deviceMask: u32,
}

impl Default for VkAcquireNextImageInfoKHX {
    fn default() -> Self {
        VkAcquireNextImageInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX,
            pNext: ptr::null(),
            swapchain: Default::default(),
            timeout: Default::default(),
            semaphore: Default::default(),
            fence: Default::default(),
            deviceMask: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupPresentInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupPresentInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHX,
}

impl Default for VkDeviceGroupPresentInfoKHX {
    fn default() -> Self {
        VkDeviceGroupPresentInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX,
            pNext: ptr::null(),
            swapchainCount: Default::default(),
            pDeviceMasks: ptr::null(),
            mode: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupSwapchainCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupSwapchainCreateInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupSwapchainCreateInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

impl Default for VkDeviceGroupSwapchainCreateInfoKHX {
    fn default() -> Self {
        VkDeviceGroupSwapchainCreateInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX,
            pNext: ptr::null(),
            modes: Default::default(),
        }
    }
}

/// See [`vkGetDeviceGroupPeerMemoryFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPeerMemoryFeaturesKHX)
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX = Option<unsafe extern "system" fn(device: vk::VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX)>;

/// See [`vkBindBufferMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory2KHX)
pub type PFN_vkBindBufferMemory2KHX = Option<unsafe extern "system" fn(device: vk::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> vk::VkResult>;

/// See [`vkBindImageMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory2KHX)
pub type PFN_vkBindImageMemory2KHX = Option<unsafe extern "system" fn(device: vk::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> vk::VkResult>;

/// See [`vkCmdSetDeviceMaskKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDeviceMaskKHX)
pub type PFN_vkCmdSetDeviceMaskKHX = Option<unsafe extern "system" fn(commandBuffer: vk::VkCommandBuffer, deviceMask: u32)>;

/// See [`vkGetDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPresentCapabilitiesKHX)
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHX = Option<unsafe extern "system" fn(device: vk::VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> vk::VkResult>;

/// See [`vkGetDeviceGroupSurfacePresentModesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupSurfacePresentModesKHX)
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHX = Option<unsafe extern "system" fn(device: vk::VkDevice, surface: khr_surface::VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> vk::VkResult>;

/// See [`vkAcquireNextImage2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImage2KHX)
pub type PFN_vkAcquireNextImage2KHX = Option<unsafe extern "system" fn(device: vk::VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> vk::VkResult>;

/// See [`vkCmdDispatchBaseKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchBaseKHX)
pub type PFN_vkCmdDispatchBaseKHX = Option<unsafe extern "system" fn(commandBuffer: vk::VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32)>;

/// See [`vkGetPhysicalDevicePresentRectanglesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDevicePresentRectanglesKHX)
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHX = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut vk::VkRect2D) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetDeviceGroupPeerMemoryFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPeerMemoryFeaturesKHX)
    pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX(device: vk::VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);

    /// See [`vkBindBufferMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory2KHX)
    pub fn vkBindBufferMemory2KHX(device: vk::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> vk::VkResult;

    /// See [`vkBindImageMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory2KHX)
    pub fn vkBindImageMemory2KHX(device: vk::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> vk::VkResult;

    /// See [`vkCmdSetDeviceMaskKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDeviceMaskKHX)
    pub fn vkCmdSetDeviceMaskKHX(commandBuffer: vk::VkCommandBuffer, deviceMask: u32);

    /// See [`vkGetDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPresentCapabilitiesKHX)
    pub fn vkGetDeviceGroupPresentCapabilitiesKHX(device: vk::VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> vk::VkResult;

    /// See [`vkGetDeviceGroupSurfacePresentModesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupSurfacePresentModesKHX)
    pub fn vkGetDeviceGroupSurfacePresentModesKHX(device: vk::VkDevice, surface: khr_surface::VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> vk::VkResult;

    /// See [`vkAcquireNextImage2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImage2KHX)
    pub fn vkAcquireNextImage2KHX(device: vk::VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> vk::VkResult;

    /// See [`vkCmdDispatchBaseKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchBaseKHX)
    pub fn vkCmdDispatchBaseKHX(commandBuffer: vk::VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

    /// See [`vkGetPhysicalDevicePresentRectanglesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDevicePresentRectanglesKHX)
    pub fn vkGetPhysicalDevicePresentRectanglesKHX(physicalDevice: vk::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut vk::VkRect2D) -> vk::VkResult;
}
