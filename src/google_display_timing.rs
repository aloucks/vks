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

//! [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)

use core;
use khr_swapchain;
use libc::c_void;
use std::ptr;

pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static [u8; 25] = b"VK_GOOGLE_display_timing\x00";
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME_STR: &'static str = "VK_GOOGLE_display_timing";

/// See [`VkRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRefreshCycleDurationGOOGLE)
/// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkRefreshCycleDurationGOOGLE {
    pub refreshDuration: u64,
}

/// See [`VkPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPastPresentationTimingGOOGLE)
/// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPastPresentationTimingGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
    pub actualPresentTime: u64,
    pub earliestPresentTime: u64,
    pub presentMargin: u64,
}

/// See [`VkPresentTimeGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentTimeGOOGLE)
/// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPresentTimeGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
}

/// See [`VkPresentTimesInfoGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentTimesInfoGOOGLE)
/// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentTimesInfoGOOGLE {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pTimes: *const VkPresentTimeGOOGLE,
}

impl Default for VkPresentTimesInfoGOOGLE {
    fn default() -> Self {
        VkPresentTimesInfoGOOGLE  {
            sType: core::VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE,
            pNext: ptr::null(),
            swapchainCount: Default::default(),
            pTimes: ptr::null(),
        }
    }
}

/// See [`vkGetRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRefreshCycleDurationGOOGLE)
/// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> core::VkResult;

/// See [`vkGetPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPastPresentationTimingGOOGLE)
/// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> core::VkResult;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetRefreshCycleDurationGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRefreshCycleDurationGOOGLE)
    /// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    pub fn vkGetRefreshCycleDurationGOOGLE(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> core::VkResult;

    /// See [`vkGetPastPresentationTimingGOOGLE`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPastPresentationTimingGOOGLE)
    /// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    pub fn vkGetPastPresentationTimingGOOGLE(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> core::VkResult;
}
