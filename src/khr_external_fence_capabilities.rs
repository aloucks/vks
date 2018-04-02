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

//! [`VK_KHR_external_fence_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_capabilities)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &[u8; 35] = b"VK_KHR_external_fence_capabilities\x00";
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME_STR: &str = "VK_KHR_external_fence_capabilities";

pub use khr_external_memory_capabilities::VK_LUID_SIZE_KHR;

/// See [`VkPhysicalDeviceIDPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceIDPropertiesKHR)
pub use khr_external_memory_capabilities::VkPhysicalDeviceIDPropertiesKHR;

bitflags! {
    /// See [`VkExternalFenceHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalFenceHandleTypeFlagBitsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkExternalFenceHandleTypeFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
        const OPAQUE_FD_BIT_KHR = 0x00000001;
        const OPAQUE_WIN32_BIT_KHR = 0x00000002;
        const OPAQUE_WIN32_KMT_BIT_KHR = 0x00000004;
        const SYNC_FD_BIT_KHR = 0x00000008;
    }
}

/// See [`VkExternalFenceHandleTypeFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalFenceHandleTypeFlagBitsKHR)
pub type VkExternalFenceHandleTypeFlagBitsKHR = VkExternalFenceHandleTypeFlagsKHR;

bitflags! {
    /// See [`VkExternalFenceFeatureFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalFenceFeatureFlagsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkExternalFenceFeatureFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
        const EXPORTABLE_BIT_KHR = 0x00000001;
        const IMPORTABLE_BIT_KHR = 0x00000002;
    }
}

/// See [`VkExternalFenceFeatureFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalFenceFeatureFlagsKHR)
pub type VkExternalFenceFeatureFlagBitsKHR = VkExternalFenceFeatureFlagsKHR;

/// See [`VkPhysicalDeviceExternalFenceInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceExternalFenceInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceExternalFenceInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}

impl Default for VkPhysicalDeviceExternalFenceInfoKHR {
    fn default() -> Self {
        VkPhysicalDeviceExternalFenceInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR,
            pNext: ptr::null(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkExternalFencePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalFencePropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalFencePropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
    pub externalFenceFeatures: VkExternalFenceFeatureFlagsKHR,
}

impl Default for VkExternalFencePropertiesKHR {
    fn default() -> Self {
        VkExternalFencePropertiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            exportFromImportedHandleTypes: Default::default(),
            compatibleHandleTypes: Default::default(),
            externalFenceFeatures: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalFencePropertiesKHR)
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceExternalFencePropertiesKHR)
    pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR(physicalDevice: vk::VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR);
}
