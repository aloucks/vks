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

//! [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharedPresentSurfaceCapabilitiesKHR)

use core::ptr;
use khr_swapchain;
use libc::c_void;
use vk;

pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static [u8; 32] = b"VK_KHR_shared_presentable_image\x00";
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME_STR: &'static str = "VK_KHR_shared_presentable_image";

/// See [`VkSharedPresentSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharedPresentSurfaceCapabilitiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub sharedPresentSupportedUsageFlags: vk::VkImageUsageFlags,
}

impl Default for VkSharedPresentSurfaceCapabilitiesKHR {
    fn default() -> Self {
        VkSharedPresentSurfaceCapabilitiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
            pNext: ptr::null_mut(),
            sharedPresentSupportedUsageFlags: Default::default(),
        }
    }
}

/// See [`PFN_vkGetSwapchainStatusKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkGetSwapchainStatusKHR)
pub type PFN_vkGetSwapchainStatusKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`PFN_vkGetSwapchainStatusKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkGetSwapchainStatusKHR)
    pub fn vkGetSwapchainStatusKHR(device: vk::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR) -> vk::VkResult;
}
