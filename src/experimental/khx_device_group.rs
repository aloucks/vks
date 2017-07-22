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

use core;
use khr_surface;
use khr_swapchain;
use libc::c_void;
use std::ptr;

pub const VK_KHX_DEVICE_GROUP_SPEC_VERSION: u32 = 1;
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME: &'static [u8; 20] = b"VK_KHX_device_group\x00";
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME_STR: &'static str = "VK_KHX_device_group";
pub const VK_MAX_DEVICE_GROUP_SIZE_KHX: usize = 32;

vks_bitflags! {
    /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkPeerMemoryFeatureFlagsKHX: u32 {
        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PEER_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHX = 0x00000001;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHX = 0x00000002;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHX = 0x00000004;

        /// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHX = 0x00000008;
    }
}

/// See [`VkPeerMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPeerMemoryFeatureFlagBitsKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type VkPeerMemoryFeatureFlagBitsKHX = VkPeerMemoryFeatureFlagsKHX;

vks_bitflags! {
    /// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkMemoryAllocateFlagsKHX: u32 {
        /// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_MEMORY_ALLOCATE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHX = 0x00000001;
    }
}

/// See [`VkMemoryAllocateFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type VkMemoryAllocateFlagBitsKHX = VkMemoryAllocateFlagsKHX;

vks_bitflags! {
    /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDeviceGroupPresentModeFlagsKHX: u32 {
        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_DEVICE_GROUP_PRESENT_MODE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX = 0x00000001;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX = 0x00000002;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX = 0x00000004;

        /// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX = 0x00000008;
    }
}

/// See [`VkDeviceGroupPresentModeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentModeFlagBitsKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type VkDeviceGroupPresentModeFlagBitsKHX = VkDeviceGroupPresentModeFlagsKHX;

/// See [`VkMemoryAllocateFlagBitsInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateFlagBitsInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryAllocateFlagsInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlagsKHX,
    pub deviceMask: u32,
}

impl Default for VkMemoryAllocateFlagsInfoKHX {
    fn default() -> Self {
        VkMemoryAllocateFlagsInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX,
            pNext: ptr::null(),
            flags: Default::default(),
            deviceMask: Default::default(),
        }
    }
}

/// See [`VkBindBufferMemoryInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindBufferMemoryInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindBufferMemoryInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub buffer: core::VkBuffer,
    pub memory: core::VkDeviceMemory,
    pub memoryOffset: core::VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

impl Default for VkBindBufferMemoryInfoKHX {
    fn default() -> Self {
        VkBindBufferMemoryInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX,
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
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindImageMemoryInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub image: core::VkImage,
    pub memory: core::VkDeviceMemory,
    pub memoryOffset: core::VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub SFRRectCount: u32,
    pub pSFRRects: *const core::VkRect2D,
}

impl Default for VkBindImageMemoryInfoKHX {
    fn default() -> Self {
        VkBindImageMemoryInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX,
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
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupRenderPassBeginInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const core::VkRect2D,
}

impl Default for VkDeviceGroupRenderPassBeginInfoKHX {
    fn default() -> Self {
        VkDeviceGroupRenderPassBeginInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX,
            pNext: ptr::null(),
            deviceMask: Default::default(),
            deviceRenderAreaCount: Default::default(),
            pDeviceRenderAreas: ptr::null(),
        }
    }
}

/// See [`VkDeviceGroupCommandBufferBeginInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupCommandBufferBeginInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
}

impl Default for VkDeviceGroupCommandBufferBeginInfoKHX {
    fn default() -> Self {
        VkDeviceGroupCommandBufferBeginInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX,
            pNext: ptr::null(),
            deviceMask: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupSubmitInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupSubmitInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupSubmitInfoKHX {
    pub sType: core::VkStructureType,
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
        VkDeviceGroupSubmitInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX,
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
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupBindSparseInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

impl Default for VkDeviceGroupBindSparseInfoKHX {
    fn default() -> Self {
        VkDeviceGroupBindSparseInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX,
            pNext: ptr::null(),
            resourceDeviceIndex: Default::default(),
            memoryDeviceIndex: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupPresentCapabilitiesKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupPresentCapabilitiesKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE_KHX],
    pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

impl Default for VkDeviceGroupPresentCapabilitiesKHX {
    fn default() -> Self {
        VkDeviceGroupPresentCapabilitiesKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX,
            pNext: ptr::null(),
            presentMask: Default::default(),
            modes: Default::default(),
        }
    }
}

/// See [`VkImageSwapchainCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSwapchainCreateInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSwapchainCreateInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: khr_swapchain::VkSwapchainKHR,
}

impl Default for VkImageSwapchainCreateInfoKHX {
    fn default() -> Self {
        VkImageSwapchainCreateInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX,
            pNext: ptr::null(),
            swapchain: Default::default(),
        }
    }
}

/// See [`VkBindImageMemorySwapchainInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindImageMemorySwapchainInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindImageMemorySwapchainInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: khr_swapchain::VkSwapchainKHR,
    pub imageIndex: u32,
}

impl Default for VkBindImageMemorySwapchainInfoKHX {
    fn default() -> Self {
        VkBindImageMemorySwapchainInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX,
            pNext: ptr::null(),
            swapchain: Default::default(),
            imageIndex: Default::default(),
        }
    }
}

/// See [`VkAcquireNextImageInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAcquireNextImageInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAcquireNextImageInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: khr_swapchain::VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: core::VkSemaphore,
    pub fence: core::VkFence,
    pub deviceMask: u32,
}

impl Default for VkAcquireNextImageInfoKHX {
    fn default() -> Self {
        VkAcquireNextImageInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX,
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
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupPresentInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHX,
}

impl Default for VkDeviceGroupPresentInfoKHX {
    fn default() -> Self {
        VkDeviceGroupPresentInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX,
            pNext: ptr::null(),
            swapchainCount: Default::default(),
            pDeviceMasks: ptr::null(),
            mode: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupSwapchainCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupSwapchainCreateInfoKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupSwapchainCreateInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

impl Default for VkDeviceGroupSwapchainCreateInfoKHX {
    fn default() -> Self {
        VkDeviceGroupSwapchainCreateInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX,
            pNext: ptr::null(),
            modes: Default::default(),
        }
    }
}

/// See [`vkGetDeviceGroupPeerMemoryFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPeerMemoryFeaturesKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX = unsafe extern "system" fn(device: core::VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);

/// See [`vkBindBufferMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory2KHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkBindBufferMemory2KHX = unsafe extern "system" fn(device: core::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> core::VkResult;

/// See [`vkBindImageMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory2KHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkBindImageMemory2KHX = unsafe extern "system" fn(device: core::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> core::VkResult;

/// See [`vkCmdSetDeviceMaskKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDeviceMaskKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkCmdSetDeviceMaskKHX = unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, deviceMask: u32);

/// See [`vkGetDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPresentCapabilitiesKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHX = unsafe extern "system" fn(device: core::VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> core::VkResult;

/// See [`vkGetDeviceGroupSurfacePresentModesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupSurfacePresentModesKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHX = unsafe extern "system" fn(device: core::VkDevice, surface: khr_surface::VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> core::VkResult;

/// See [`vkAcquireNextImage2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImage2KHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkAcquireNextImage2KHX = unsafe extern "system" fn(device: core::VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> core::VkResult;

/// See [`vkCmdDispatchBaseKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchBaseKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkCmdDispatchBaseKHX = unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

/// See [`vkGetPhysicalDevicePresentRectanglesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDevicePresentRectanglesKHX)
/// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHX = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut core::VkRect2D) -> core::VkResult;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetDeviceGroupPeerMemoryFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPeerMemoryFeaturesKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX(device: core::VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);

    /// See [`vkBindBufferMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory2KHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkBindBufferMemory2KHX(device: core::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> core::VkResult;

    /// See [`vkBindImageMemory2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory2KHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkBindImageMemory2KHX(device: core::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> core::VkResult;

    /// See [`vkCmdSetDeviceMaskKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDeviceMaskKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkCmdSetDeviceMaskKHX(commandBuffer: core::VkCommandBuffer, deviceMask: u32);

    /// See [`vkGetDeviceGroupPresentCapabilitiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupPresentCapabilitiesKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkGetDeviceGroupPresentCapabilitiesKHX(device: core::VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> core::VkResult;

    /// See [`vkGetDeviceGroupSurfacePresentModesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceGroupSurfacePresentModesKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkGetDeviceGroupSurfacePresentModesKHX(device: core::VkDevice, surface: khr_surface::VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> core::VkResult;

    /// See [`vkAcquireNextImage2KHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImage2KHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkAcquireNextImage2KHX(device: core::VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> core::VkResult;

    /// See [`vkCmdDispatchBaseKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchBaseKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkCmdDispatchBaseKHX(commandBuffer: core::VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

    /// See [`vkGetPhysicalDevicePresentRectanglesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDevicePresentRectanglesKHX)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    pub fn vkGetPhysicalDevicePresentRectanglesKHX(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut core::VkRect2D) -> core::VkResult;
}
