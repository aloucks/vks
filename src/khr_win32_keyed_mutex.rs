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

//! [`VK_KHR_win32_keyed_mutex`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_keyed_mutex)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
pub const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_win32_keyed_mutex\x00";
pub const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME_STR: &str = "VK_KHR_win32_keyed_mutex";

/// See [`VkWin32KeyedMutexAcquireReleaseInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32KeyedMutexAcquireReleaseInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const vk::VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeouts: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const vk::VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}

impl Default for VkWin32KeyedMutexAcquireReleaseInfoKHR {
    fn default() -> Self {
        VkWin32KeyedMutexAcquireReleaseInfoKHR {
            sType: vk::VkStructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
            pNext: ptr::null(),
            acquireCount: Default::default(),
            pAcquireSyncs: ptr::null(),
            pAcquireKeys: ptr::null(),
            pAcquireTimeouts: ptr::null(),
            releaseCount: Default::default(),
            pReleaseSyncs: ptr::null(),
            pReleaseKeys: ptr::null(),
        }
    }
}
