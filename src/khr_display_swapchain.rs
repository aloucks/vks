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

pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_SPEC_VERSION: u32 = 9;
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static [u8; 25] = b"VK_KHR_display_swapchain\x00";
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME_STR: &'static str = "VK_KHR_display_swapchain";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}

pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateSharedSwapchainsKHR(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult;
}
