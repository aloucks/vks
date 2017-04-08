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

pub const VK_NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_NN_VI_SURFACE_EXTENSION_NAME: &'static [u8; 17] = b"VK_NN_vi_surface\x00";
pub const VK_NN_VI_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_NN_vi_surface";

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkViSurfaceCreateFlagsNN: u32 {
        const VK_VI_SURFACE_CREATE_DUMMY_NN = 0x00000000,
    }
}
pub type VkViSurfaceCreateFlagBitsNN = VkViSurfaceCreateFlagsNN;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkViSurfaceCreateInfoNN {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}

impl Default for VkViSurfaceCreateInfoNN {
    fn default() -> Self {
        VkViSurfaceCreateInfoNN  {
            sType: VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN,
            pNext: ptr::null(),
            flags: Default::default(),
            window: ptr::null_mut(),
        }
    }
}

pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkViSurfaceCreateInfoNN, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateViSurfaceNN(instance: VkInstance, pCreateInfo: *const VkViSurfaceCreateInfoNN, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}
