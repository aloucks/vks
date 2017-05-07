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
use std::ptr;

pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static [u8; 39] = b"VK_KHX_external_semaphore_capabilities\x00";
pub const VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_semaphore_capabilities";

bitflags! {
    /// See [`VkExternalSemaphoreHandleTypeFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagsKHX)
    /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkExternalSemaphoreHandleTypeFlagsKHX: u32 {
        /// See [`VkExternalSemaphoreHandleTypeFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHX = 0x00000001,

        /// See [`VkExternalSemaphoreHandleTypeFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHX = 0x00000002,

        /// See [`VkExternalSemaphoreHandleTypeFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHX = 0x00000004,

        /// See [`VkExternalSemaphoreHandleTypeFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHX = 0x00000008,

        /// See [`VkExternalSemaphoreHandleTypeFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FENCE_FD_BIT_KHX = 0x00000010,
    }
}

/// See [`VkExternalSemaphoreHandleTypeFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHX)
/// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
pub type VkExternalSemaphoreHandleTypeFlagBitsKHX = VkExternalSemaphoreHandleTypeFlagsKHX;

bitflags! {
    /// See [`VkExternalSemaphoreFeatureFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagsKHX)
    /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkExternalSemaphoreFeatureFlagsKHX: u32 {
        /// See [`VkExternalSemaphoreFeatureFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHX = 0x00000001,

        /// See [`VkExternalSemaphoreFeatureFlagsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagsKHX)
        /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
        const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHX = 0x00000002,
    }
}

/// See [`VkExternalSemaphoreFeatureFlagBitsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHX)
/// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
pub type VkExternalSemaphoreFeatureFlagBitsKHX = VkExternalSemaphoreFeatureFlagsKHX;

/// See [`VkPhysicalDeviceExternalSemaphoreInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalSemaphoreInfoKHX)
/// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHX,
}

impl Default for VkPhysicalDeviceExternalSemaphoreInfoKHX {
    fn default() -> Self {
        VkPhysicalDeviceExternalSemaphoreInfoKHX  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHX,
            pNext: ptr::null(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphorePropertiesKHX)
/// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalSemaphorePropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHX,
}

impl Default for VkExternalSemaphorePropertiesKHX {
    fn default() -> Self {
        VkExternalSemaphorePropertiesKHX  {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            exportFromImportedHandleTypes: Default::default(),
            compatibleHandleTypes: Default::default(),
            externalSemaphoreFeatures: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHX)
/// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHX, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHX);

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHX)
    /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
    pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHX(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHX, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHX);
}
