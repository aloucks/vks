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

//! [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)

use core;
use libc::c_void;
use std::ptr;

pub const VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static [u8; 36] = b"VK_KHX_external_memory_capabilities\x00";
pub const VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory_capabilities";
pub const VK_LUID_SIZE_KHX: usize = 8;

vks_bitflags! {
    /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkExternalMemoryHandleTypeFlagsKHX: u32 {
        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHX = 0x00000001;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHX = 0x00000002;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHX = 0x00000004;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHX = 0x00000008;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHX = 0x00000010;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHX = 0x00000020;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHX = 0x00000040;
    }
}

/// See [`VkExternalMemoryHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHX)
pub type VkExternalMemoryHandleTypeFlagBitsKHX = VkExternalMemoryHandleTypeFlagsKHX;

vks_bitflags! {
    /// See [`VkExternalMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkExternalMemoryFeatureFlagsKHX: u32 {
        /// See [`VkExternalMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkExternalMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHX = 0x00000001;

        /// See [`VkExternalMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHX = 0x00000002;

        /// See [`VkExternalMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHX)
        const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHX = 0x00000004;
    }
}

/// See [`VkExternalMemoryFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHX)
pub type VkExternalMemoryFeatureFlagBitsKHX = VkExternalMemoryFeatureFlagsKHX;

/// See [`VkExternalMemoryPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryPropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkExternalMemoryPropertiesKHX {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHX,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHX,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHX,
}

/// See [`VkPhysicalDeviceExternalImageFormatInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalImageFormatInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHX,
}

impl Default for VkPhysicalDeviceExternalImageFormatInfoKHX {
    fn default() -> Self {
        VkPhysicalDeviceExternalImageFormatInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHX,
            pNext: ptr::null(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalImageFormatPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalImageFormatPropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalImageFormatPropertiesKHX {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHX,
}

impl Default for VkExternalImageFormatPropertiesKHX {
    fn default() -> Self {
        VkExternalImageFormatPropertiesKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            externalMemoryProperties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceExternalBufferInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalBufferInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: core::VkBufferCreateFlags,
    pub usage: core::VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHX,
}

impl Default for VkPhysicalDeviceExternalBufferInfoKHX {
    fn default() -> Self {
        VkPhysicalDeviceExternalBufferInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHX,
            pNext: ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalBufferPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalBufferPropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalBufferPropertiesKHX {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHX,
}

impl Default for VkExternalBufferPropertiesKHX {
    fn default() -> Self {
        VkExternalBufferPropertiesKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            externalMemoryProperties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceIDPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceIDPropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceIDPropertiesKHX {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; core::VK_UUID_SIZE],
    pub driverUUID: [u8; core::VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE_KHX],
    pub deviceLUIDValid: core::VkBool32,
}

impl Default for VkPhysicalDeviceIDPropertiesKHX {
    fn default() -> Self {
        VkPhysicalDeviceIDPropertiesKHX  {
            sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            deviceUUID: Default::default(),
            driverUUID: Default::default(),
            deviceLUID: Default::default(),
            deviceLUIDValid: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceExternalBufferPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalBufferPropertiesKHX)
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHX = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHX, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHX);

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalBufferPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalBufferPropertiesKHX)
    pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHX(physicalDevice: core::VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHX, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHX);
}
