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

//! [`VK_NN_vi_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NN_vi_surface)

use core::ptr;
use khr_surface;
use libc::c_void;
use vk;

pub const VK_NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_NN_VI_SURFACE_EXTENSION_NAME: &[u8; 17] = b"VK_NN_vi_surface\x00";
pub const VK_NN_VI_SURFACE_EXTENSION_NAME_STR: &str = "VK_NN_vi_surface";

bitflags! {
    /// See [`VkViSurfaceCreateFlagsNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViSurfaceCreateFlagsNN)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkViSurfaceCreateFlagsNN: u32 {
        const MAX_ENUM_NN = 0x7fffffff;
    }
}

/// See [`VkViSurfaceCreateFlagsNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViSurfaceCreateFlagsNN)
pub type VkViSurfaceCreateFlagBitsNN = VkViSurfaceCreateFlagsNN;

/// See [`VkViSurfaceCreateInfoNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViSurfaceCreateInfoNN)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkViSurfaceCreateInfoNN {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}

impl Default for VkViSurfaceCreateInfoNN {
    fn default() -> Self {
        VkViSurfaceCreateInfoNN {
            sType: vk::VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN,
            pNext: ptr::null(),
            flags: Default::default(),
            window: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateViSurfaceNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateViSurfaceNN)
pub type PFN_vkCreateViSurfaceNN = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkViSurfaceCreateInfoNN, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateViSurfaceNN`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateViSurfaceNN)
    pub fn vkCreateViSurfaceNN(instance: vk::VkInstance, pCreateInfo: *const VkViSurfaceCreateInfoNN, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult;
}
