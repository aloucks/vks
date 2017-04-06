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

pub const VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static [u8; 36] = b"VK_KHX_external_memory_capabilities\x00";
pub const VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory_capabilities";
pub const VK_LUID_SIZE_KHX: usize = 8;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkExternalMemoryHandleTypeFlagsKHX: u32 {
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHX = 0x00000001,
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHX = 0x00000002,
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHX = 0x00000004,
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHX = 0x00000008,
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHX = 0x00000010,
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHX = 0x00000020,
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHX = 0x00000040,
    }
}
pub type VkExternalMemoryHandleTypeFlagBitsKHX = VkExternalMemoryHandleTypeFlagsKHX;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkExternalMemoryFeatureFlagsKHX: u32 {
        const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHX = 0x00000001,
        const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHX = 0x00000002,
        const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHX = 0x00000004,
    }
}
pub type VkExternalMemoryFeatureFlagBitsKHX = VkExternalMemoryFeatureFlagsKHX;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryPropertiesKHX {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHX,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHX,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalImageFormatPropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalBufferPropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceIDPropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE_KHX],
    pub deviceLUIDValid: VkBool32,
}

pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHX, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHX);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHX(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHX, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHX);
}
