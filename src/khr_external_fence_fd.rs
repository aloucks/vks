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

//! [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)

use core::ptr;
use khr_external_fence;
use khr_external_fence_capabilities;
use libc::{c_int, c_void};
use vk;

pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_external_fence_fd\x00";
pub const VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME_STR: &str = "VK_KHR_external_fence_fd";

/// See [`VkImportFenceFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportFenceFdInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportFenceFdInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub fence: vk::VkFence,
    pub flags: khr_external_fence::VkFenceImportFlagsKHR,
    pub handleType: khr_external_fence_capabilities::VkExternalFenceHandleTypeFlagBitsKHR,
    pub fd: c_int,
}

impl Default for VkImportFenceFdInfoKHR {
    fn default() -> Self {
        VkImportFenceFdInfoKHR {
            sType: vk::VkStructureType::IMPORT_FENCE_FD_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handleType: Default::default(),
            fd: Default::default(),
        }
    }
}

/// See [`VkFenceGetFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceGetFdInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceGetFdInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub fence: vk::VkFence,
    pub handleType: khr_external_fence_capabilities::VkExternalFenceHandleTypeFlagBitsKHR,
}

impl Default for VkFenceGetFdInfoKHR {
    fn default() -> Self {
        VkFenceGetFdInfoKHR {
            sType: vk::VkStructureType::FENCE_GET_FD_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`vkImportFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceFdKHR)
pub type PFN_vkImportFenceFdKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> vk::VkResult>;

/// See [`vkGetFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceFdKHR)
pub type PFN_vkGetFenceFdKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkImportFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceFdKHR)
    pub fn vkImportFenceFdKHR(device: vk::VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> vk::VkResult;

    /// See [`vkGetFenceFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceFdKHR)
    pub fn vkGetFenceFdKHR(device: vk::VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult;
}
