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

pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: u32 = 2;
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &'static [u8; 19] = b"VK_MVK_ios_surface\x00";
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_MVK_ios_surface";

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkIOSSurfaceCreateFlagsMVK: u32 {
        const VK_IOS_SURFACE_CREATE_DUMMY_MVK = 0x00000000,
    }
}
pub type VkIOSSurfaceCreateFlagBitsMVK = VkIOSSurfaceCreateFlagsMVK;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateIOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}
