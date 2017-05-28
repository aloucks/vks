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

pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 2;
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &'static [u8; 21] = b"VK_MVK_macos_surface\x00";
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_MVK_macos_surface";

bitflags! {
    /// See [`VkMacOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMacOSSurfaceCreateFlagsMVK)
    /// and extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkMacOSSurfaceCreateFlagsMVK: u32 {
        const VK_MACOS_SURFACE_CREATE_DUMMY_MVK = 0x00000000;
    }
}

/// See [`VkMacOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMacOSSurfaceCreateFlagsMVK)
/// and extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
pub type VkMacOSSurfaceCreateFlagBitsMVK = VkMacOSSurfaceCreateFlagsMVK;

/// See [`VkMacOSSurfaceCreateInfoMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMacOSSurfaceCreateInfoMVK)
/// and extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMacOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

impl Default for VkMacOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        VkMacOSSurfaceCreateInfoMVK  {
            sType: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
            pNext: ptr::null(),
            flags: Default::default(),
            pView: ptr::null(),
        }
    }
}

/// See [`vkCreateMacOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMacOSSurfaceMVK)
/// and extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

extern "system" {
    /// See [`vkCreateMacOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMacOSSurfaceMVK)
    /// and extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
    pub fn vkCreateMacOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}
