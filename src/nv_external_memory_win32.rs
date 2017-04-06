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

pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static [u8; 28] = b"VK_NV_external_memory_win32\x00";
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR: &'static str = "VK_NV_external_memory_win32";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsNV,
    pub handle: win32_wrapper::HANDLE,
}

impl Default for VkImportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        VkImportMemoryWin32HandleInfoNV  {
            sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            pNext: ptr::null(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_wrapper::DWORD,
}

impl Default for VkExportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        VkExportMemoryWin32HandleInfoNV  {
            sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
        }
    }
}

pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetMemoryWin32HandleNV(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;
}
