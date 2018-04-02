// Copyright (c) 2018, Dennis Hamester <dennis.hamester@startmail.com>
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

//! [`VK_NV_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_capabilities)

use vk;

pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &[u8; 35] = b"VK_NV_external_memory_capabilities\x00";
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME_STR: &str = "VK_NV_external_memory_capabilities";

bitflags! {
    /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkExternalMemoryHandleTypeFlagsNV: u32 {
        const MAX_ENUM_NV = 0x7fffffff;
        const OPAQUE_WIN32_BIT_NV = 0x00000001;
        const OPAQUE_WIN32_KMT_BIT_NV = 0x00000002;
        const D3D11_IMAGE_BIT_NV = 0x00000004;
        const D3D11_IMAGE_KMT_BIT_NV = 0x00000008;
    }
}

/// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
pub type VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagsNV;

bitflags! {
    /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkExternalMemoryFeatureFlagsNV: u32 {
        const MAX_ENUM_NV = 0x7fffffff;
        const DEDICATED_ONLY_BIT_NV = 0x00000001;
        const EXPORTABLE_BIT_NV = 0x00000002;
        const IMPORTABLE_BIT_NV = 0x00000004;
    }
}

/// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
pub type VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagsNV;

/// See [`VkExternalImageFormatPropertiesNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalImageFormatPropertiesNV)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkExternalImageFormatPropertiesNV {
    pub imageFormatProperties: vk::VkImageFormatProperties,
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

/// See [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalImageFormatPropertiesNV)
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, format: vk::VkFormat, type_: vk::VkImageType, tiling: vk::VkImageTiling, usage: vk::VkImageUsageFlags, flags: vk::VkImageCreateFlags, externalHandleType: VkExternalMemoryHandleTypeFlagsNV, pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalImageFormatPropertiesNV)
    pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(physicalDevice: vk::VkPhysicalDevice, format: vk::VkFormat, type_: vk::VkImageType, tiling: vk::VkImageTiling, usage: vk::VkImageUsageFlags, flags: vk::VkImageCreateFlags, externalHandleType: VkExternalMemoryHandleTypeFlagsNV, pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV) -> vk::VkResult;
}
