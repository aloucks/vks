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
use libc::c_void;

pub const VK_KHX_DEVICE_GROUP_SPEC_VERSION: u32 = 1;
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME: &'static [u8; 20] = b"VK_KHX_device_group\x00";
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME_STR: &'static str = "VK_KHX_device_group";
pub const VK_MAX_DEVICE_GROUP_SIZE_KHX: usize = 32;

bitflags! {
    #[repr(C)]
    pub flags VkPeerMemoryFeatureFlagsKHX: u32 {
        const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHX = 0x00000001,
        const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHX = 0x00000002,
        const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHX = 0x00000004,
        const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHX = 0x00000008,
    }
}
pub type VkPeerMemoryFeatureFlagBitsKHX = VkPeerMemoryFeatureFlagsKHX;

bitflags! {
    #[repr(C)]
    pub flags VkMemoryAllocateFlagsKHX: u32 {
        const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHX = 0x00000001,
    }
}
pub type VkMemoryAllocateFlagBitsKHX = VkMemoryAllocateFlagsKHX;

bitflags! {
    #[repr(C)]
    pub flags VkDeviceGroupPresentModeFlagsKHX: u32 {
        const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX = 0x00000001,
        const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX = 0x00000002,
        const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX = 0x00000004,
        const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX = 0x00000008,
    }
}
pub type VkDeviceGroupPresentModeFlagBitsKHX = VkDeviceGroupPresentModeFlagsKHX;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryAllocateFlagsInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlagsKHX,
    pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindBufferMemoryInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindImageMemoryInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub SFRRectCount: u32,
    pub pSFRRects: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupRenderPassBeginInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupSubmitInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupBindSparseInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupPresentCapabilitiesKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE_KHX],
    pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSwapchainCreateInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindImageMemorySwapchainInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAcquireNextImageInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: VkSemaphore,
    pub fence: VkFence,
    pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupPresentInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupSwapchainCreateInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX = unsafe extern "system" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);
pub type PFN_vkBindBufferMemory2KHX = unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> VkResult;
pub type PFN_vkBindImageMemory2KHX = unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> VkResult;
pub type PFN_vkCmdSetDeviceMaskKHX = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHX = unsafe extern "system" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> VkResult;
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHX = unsafe extern "system" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> VkResult;
pub type PFN_vkAcquireNextImage2KHX = unsafe extern "system" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> VkResult;
pub type PFN_vkCmdDispatchBaseKHX = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);
    pub fn vkBindBufferMemory2KHX(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> VkResult;
    pub fn vkBindImageMemory2KHX(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> VkResult;
    pub fn vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32);
    pub fn vkGetDeviceGroupPresentCapabilitiesKHX(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> VkResult;
    pub fn vkGetDeviceGroupSurfacePresentModesKHX(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> VkResult;
    pub fn vkAcquireNextImage2KHX(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> VkResult;
    pub fn vkCmdDispatchBaseKHX(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    pub fn vkGetPhysicalDevicePresentRectanglesKHX(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;
}
