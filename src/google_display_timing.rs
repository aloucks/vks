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

pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static [u8; 25] = b"VK_GOOGLE_display_timing\x00";
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME_STR: &'static str = "VK_GOOGLE_display_timing";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkRefreshCycleDurationGOOGLE {
    pub refreshDuration: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPastPresentationTimingGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
    pub actualPresentTime: u64,
    pub earliestPresentTime: u64,
    pub presentMargin: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentTimeGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentTimesInfoGOOGLE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pTimes: *const VkPresentTimeGOOGLE,
}

pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> VkResult;
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetRefreshCycleDurationGOOGLE(device: VkDevice, swapchain: VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> VkResult;
    pub fn vkGetPastPresentationTimingGOOGLE(device: VkDevice, swapchain: VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> VkResult;
}
