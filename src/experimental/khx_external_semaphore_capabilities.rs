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

//! [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)

use core;
use libc::c_void;
use std::ptr;

pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static [u8; 39] = b"VK_KHX_external_semaphore_capabilities\x00";
pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_semaphore_capabilities";

vks_bitflags! {
    /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkExternalSemaphoreHandleTypeFlagsKHX: u32 {
        /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHX = 0x00000001;

        /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHX = 0x00000002;

        /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHX = 0x00000004;

        /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHX = 0x00000008;

        /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FENCE_FD_BIT_KHX = 0x00000010;
    }
}

/// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
pub type VkExternalSemaphoreHandleTypeFlagBitsKHX = VkExternalSemaphoreHandleTypeFlagsKHX;

vks_bitflags! {
    /// See [`VkExternalSemaphoreFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkExternalSemaphoreFeatureFlagsKHX: u32 {
        /// See [`VkExternalSemaphoreFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_FEATURE_FLAG_BITS_MAX_ENUM_KHX = 0x7fffffff;

        /// See [`VkExternalSemaphoreFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHX = 0x00000001;

        /// See [`VkExternalSemaphoreFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHX)
        const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHX = 0x00000002;
    }
}

/// See [`VkExternalSemaphoreFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHX)
pub type VkExternalSemaphoreFeatureFlagBitsKHX = VkExternalSemaphoreFeatureFlagsKHX;

/// See [`VkPhysicalDeviceExternalSemaphoreInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalSemaphoreInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHX,
}

impl Default for VkPhysicalDeviceExternalSemaphoreInfoKHX {
    fn default() -> Self {
        VkPhysicalDeviceExternalSemaphoreInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHX,
            pNext: ptr::null(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphorePropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalSemaphorePropertiesKHX {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHX,
}

impl Default for VkExternalSemaphorePropertiesKHX {
    fn default() -> Self {
        VkExternalSemaphorePropertiesKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            exportFromImportedHandleTypes: Default::default(),
            compatibleHandleTypes: Default::default(),
            externalSemaphoreFeatures: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHX)
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHX = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHX, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHX);

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHX)
    pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHX(physicalDevice: core::VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHX, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHX);
}
