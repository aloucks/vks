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

//! [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)

use core::ptr;
use khr_swapchain;
use libc::c_void;
use vk;

pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 9;
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static [u8; 25] = b"VK_KHR_display_swapchain\x00";
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME_STR: &'static str = "VK_KHR_display_swapchain";

/// See [`VkDisplayPresentInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPresentInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPresentInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: vk::VkRect2D,
    pub dstRect: vk::VkRect2D,
    pub persistent: vk::VkBool32,
}

impl Default for VkDisplayPresentInfoKHR {
    fn default() -> Self {
        VkDisplayPresentInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR,
            pNext: ptr::null(),
            srcRect: Default::default(),
            dstRect: Default::default(),
            persistent: Default::default(),
        }
    }
}

/// See [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
pub type PFN_vkCreateSharedSwapchainsKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, swapchainCount: u32, pCreateInfos: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSwapchains: *mut khr_swapchain::VkSwapchainKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
    pub fn vkCreateSharedSwapchainsKHR(device: vk::VkDevice, swapchainCount: u32, pCreateInfos: *const khr_swapchain::VkSwapchainCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSwapchains: *mut khr_swapchain::VkSwapchainKHR) -> vk::VkResult;
}
