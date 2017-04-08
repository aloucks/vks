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

pub const VK_KHR_MIR_SURFACE_EXTENSION_SPEC_VERSION: u32 = 4;
pub const VK_KHR_MIR_SURFACE_EXTENSION_NAME: &'static [u8; 19] = b"VK_KHR_mir_surface\x00";
pub const VK_KHR_MIR_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_mir_surface";

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkMirSurfaceCreateFlagsKHR: u32 {
        const VK_MIR_SURFACE_CREATE_DUMMY = 0x00000000,
    }
}
pub type VkMirSurfaceCreateFlagBitsKHR = VkMirSurfaceCreateFlagsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMirSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMirSurfaceCreateFlagsKHR,
    pub connection: *mut mir_wrapper::MirConnection,
    pub mirSurface: *mut mir_wrapper::MirSurface,
}

pub type PFN_vkCreateMirSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkMirSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceMirPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut mir_wrapper::MirConnection) -> VkBool32;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateMirSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkMirSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceMirPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut mir_wrapper::MirConnection) -> VkBool32;
}
