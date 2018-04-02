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

//! [`VK_KHR_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_capabilities)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &[u8; 39] = b"VK_KHR_external_semaphore_capabilities\x00";
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME_STR: &str = "VK_KHR_external_semaphore_capabilities";

pub use khr_external_memory_capabilities::VK_LUID_SIZE_KHR;

/// See [`VkPhysicalDeviceIDPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceIDPropertiesKHR)
pub use khr_external_memory_capabilities::VkPhysicalDeviceIDPropertiesKHR;

bitflags! {
    /// See [`VkExternalSemaphoreHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkExternalSemaphoreHandleTypeFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
        const OPAQUE_FD_BIT_KHR = 0x00000001;
        const OPAQUE_WIN32_BIT_KHR = 0x00000002;
        const OPAQUE_WIN32_KMT_BIT_KHR = 0x00000004;
        const D3D12_FENCE_BIT_KHR = 0x00000008;
        const SYNC_FD_BIT_KHR = 0x00000010;
    }
}

/// See [`VkExternalSemaphoreHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreHandleTypeFlagBitsKHR)
pub type VkExternalSemaphoreHandleTypeFlagBitsKHR = VkExternalSemaphoreHandleTypeFlagsKHR;

bitflags! {
    /// See [`VkExternalSemaphoreFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkExternalSemaphoreFeatureFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
        const EXPORTABLE_BIT_KHR = 0x00000001;
        const IMPORTABLE_BIT_KHR = 0x00000002;
    }
}

/// See [`VkExternalSemaphoreFeatureFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphoreFeatureFlagBitsKHR)
pub type VkExternalSemaphoreFeatureFlagBitsKHR = VkExternalSemaphoreFeatureFlagsKHR;

/// See [`VkPhysicalDeviceExternalSemaphoreInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalSemaphoreInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}

impl Default for VkPhysicalDeviceExternalSemaphoreInfoKHR {
    fn default() -> Self {
        VkPhysicalDeviceExternalSemaphoreInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR,
            pNext: ptr::null(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalSemaphorePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalSemaphorePropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalSemaphorePropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHR,
}

impl Default for VkExternalSemaphorePropertiesKHR {
    fn default() -> Self {
        VkExternalSemaphorePropertiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            exportFromImportedHandleTypes: Default::default(),
            compatibleHandleTypes: Default::default(),
            externalSemaphoreFeatures: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)
    pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(physicalDevice: vk::VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR);
}
