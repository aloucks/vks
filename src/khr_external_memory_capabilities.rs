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

//! [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static [u8; 36] = b"VK_KHR_external_memory_capabilities\x00";
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME_STR: &'static str = "VK_KHR_external_memory_capabilities";

pub const VK_LUID_SIZE_KHR: usize = 8;

vks_bitflags! {
    /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
    pub struct VkExternalMemoryHandleTypeFlagsKHR: u32 {
        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = 0x00000001;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = 0x00000002;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = 0x00000004;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR = 0x00000008;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR = 0x00000010;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR = 0x00000020;

        /// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR = 0x00000040;
    }
}

/// See [`VkExternalMemoryHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsKHR)
pub type VkExternalMemoryHandleTypeFlagBitsKHR = VkExternalMemoryHandleTypeFlagsKHR;

vks_bitflags! {
    /// See [`VkExternalMemoryFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHR)
    pub struct VkExternalMemoryFeatureFlagsKHR: u32 {
        /// See [`VkExternalMemoryFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

        /// See [`VkExternalMemoryFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR = 0x00000001;

        /// See [`VkExternalMemoryFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR = 0x00000002;

        /// See [`VkExternalMemoryFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHR)
        const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR = 0x00000004;
    }
}

/// See [`VkExternalMemoryFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsKHR)
pub type VkExternalMemoryFeatureFlagBitsKHR = VkExternalMemoryFeatureFlagsKHR;

/// See [`VkExternalMemoryPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryPropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkExternalMemoryPropertiesKHR {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHR,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}

/// See [`VkPhysicalDeviceExternalImageFormatInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalImageFormatInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}

impl Default for VkPhysicalDeviceExternalImageFormatInfoKHR {
    fn default() -> Self {
        VkPhysicalDeviceExternalImageFormatInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR,
            pNext: ptr::null(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalImageFormatPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalImageFormatPropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalImageFormatPropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}

impl Default for VkExternalImageFormatPropertiesKHR {
    fn default() -> Self {
        VkExternalImageFormatPropertiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            externalMemoryProperties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceExternalBufferInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalBufferInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: vk::VkBufferCreateFlags,
    pub usage: vk::VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}

impl Default for VkPhysicalDeviceExternalBufferInfoKHR {
    fn default() -> Self {
        VkPhysicalDeviceExternalBufferInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalBufferPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalBufferPropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalBufferPropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}

impl Default for VkExternalBufferPropertiesKHR {
    fn default() -> Self {
        VkExternalBufferPropertiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            externalMemoryProperties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceIDPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceIDPropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceIDPropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; vk::VK_UUID_SIZE],
    pub driverUUID: [u8; vk::VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE_KHR],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: vk::VkBool32,
}

impl Default for VkPhysicalDeviceIDPropertiesKHR {
    fn default() -> Self {
        VkPhysicalDeviceIDPropertiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            deviceUUID: Default::default(),
            driverUUID: Default::default(),
            deviceLUID: Default::default(),
            deviceNodeMask: Default::default(),
            deviceLUIDValid: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalBufferPropertiesKHR)
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalBufferPropertiesKHR)
    pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(physicalDevice: vk::VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR);
}
