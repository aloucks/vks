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

pub const VK_KHX_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static [u8; 29] = b"VK_KHX_external_memory_win32\x00";
pub const VK_KHX_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory_win32";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHX,
    pub handle: win32_wrapper::HANDLE,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_wrapper::DWORD,
    pub name: win32_wrapper::LPCWSTR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub memoryTypeBits: u32,
}

pub type PFN_vkGetMemoryWin32HandleKHX = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHX = unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, handle: win32_wrapper::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHX) -> VkResult;

extern "system" {
    pub fn vkGetMemoryWin32HandleKHX(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;
    pub fn vkGetMemoryWin32HandlePropertiesKHX(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, handle: win32_wrapper::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHX) -> VkResult;
}
