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

//! [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)

use core::ptr;
use khr_surface;
use libc::c_void;
use mir_types;
use vk;

pub const VK_KHR_MIR_SURFACE_SPEC_VERSION: u32 = 4;
pub const VK_KHR_MIR_SURFACE_EXTENSION_NAME: &[u8; 19] = b"VK_KHR_mir_surface\x00";
pub const VK_KHR_MIR_SURFACE_EXTENSION_NAME_STR: &str = "VK_KHR_mir_surface";

bitflags! {
    /// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkMirSurfaceCreateFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
pub type VkMirSurfaceCreateFlagBitsKHR = VkMirSurfaceCreateFlagsKHR;

/// See [`VkMirSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMirSurfaceCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMirSurfaceCreateFlagsKHR,
    pub connection: *mut mir_types::MirConnection,
    pub mirSurface: *mut mir_types::MirSurface,
}

impl Default for VkMirSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkMirSurfaceCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            connection: ptr::null_mut(),
            mirSurface: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateMirSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMirSurfaceKHR)
pub type PFN_vkCreateMirSurfaceKHR = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkMirSurfaceCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceMirPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMirPresentationSupportKHR)
pub type PFN_vkGetPhysicalDeviceMirPresentationSupportKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut mir_types::MirConnection) -> vk::VkBool32>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateMirSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMirSurfaceKHR)
    pub fn vkCreateMirSurfaceKHR(instance: vk::VkInstance, pCreateInfo: *const VkMirSurfaceCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceMirPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMirPresentationSupportKHR)
    pub fn vkGetPhysicalDeviceMirPresentationSupportKHR(physicalDevice: vk::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut mir_types::MirConnection) -> vk::VkBool32;
}
