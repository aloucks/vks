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

pub const VK_KHX_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static [u8; 32] = b"VK_KHX_external_semaphore_win32\x00";
pub const VK_KHX_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_semaphore_win32";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportSemaphoreWin32HandleInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHX,
    pub handle: win32_wrapper::HANDLE,
}

impl Default for VkImportSemaphoreWin32HandleInfoKHX {
    fn default() -> Self {
        VkImportSemaphoreWin32HandleInfoKHX  {
            sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHX,
            pNext: ptr::null(),
            semaphore: ptr::null_mut(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportSemaphoreWin32HandleInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_wrapper::DWORD,
    pub name: win32_wrapper::LPCWSTR,
}

impl Default for VkExportSemaphoreWin32HandleInfoKHX {
    fn default() -> Self {
        VkExportSemaphoreWin32HandleInfoKHX  {
            sType: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHX,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkD3D12FenceSubmitInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

impl Default for VkD3D12FenceSubmitInfoKHX {
    fn default() -> Self {
        VkD3D12FenceSubmitInfoKHX  {
            sType: VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHX,
            pNext: ptr::null(),
            waitSemaphoreValuesCount: Default::default(),
            pWaitSemaphoreValues: ptr::null(),
            signalSemaphoreValuesCount: Default::default(),
            pSignalSemaphoreValues: ptr::null(),
        }
    }
}

pub type PFN_vkImportSemaphoreWin32HandleKHX = unsafe extern "system" fn(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHX) -> VkResult;
pub type PFN_vkGetSemaphoreWin32HandleKHX = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, handleType: VkExternalSemaphoreHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkImportSemaphoreWin32HandleKHX(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHX) -> VkResult;
    pub fn vkGetSemaphoreWin32HandleKHX(device: VkDevice, semaphore: VkSemaphore, handleType: VkExternalSemaphoreHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;
}
