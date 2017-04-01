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

pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static [u8; 39] = b"VK_KHX_external_semaphore_capabilities\x00";
pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_semaphore_capabilities";

bitflags! {
    #[repr(C)]
    pub flags VkExternalSemaphoreHandleTypeFlagsKHX: u32 {
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHX = 0x00000001,
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHX = 0x00000002,
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHX = 0x00000004,
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHX = 0x00000008,
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FENCE_FD_BIT_KHX = 0x00000010,
    }
}
pub type VkExternalSemaphoreHandleTypeFlagBitsKHX = VkExternalSemaphoreHandleTypeFlagsKHX;

bitflags! {
    #[repr(C)]
    pub flags VkExternalSemaphoreFeatureFlagsKHX: u32 {
        const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHX = 0x00000001,
        const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHX = 0x00000002,
    }
}
pub type VkExternalSemaphoreFeatureFlagBitsKHX = VkExternalSemaphoreFeatureFlagsKHX;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalSemaphorePropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHX,
}

pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHX, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHX);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHX(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHX, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHX);
}
