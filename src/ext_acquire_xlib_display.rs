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

pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static [u8; 28] = b"VK_EXT_acquire_xlib_display\x00";
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME_STR: &'static str = "VK_EXT_acquire_xlib_display";

pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut ::xlib_wrapper::Display, display: VkDisplayKHR) -> VkResult;
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut ::xlib_wrapper::Display, rrOutput: ::xlib_wrapper::RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    pub fn vkAcquireXlibDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut ::xlib_wrapper::Display, display: VkDisplayKHR) -> VkResult;
    pub fn vkGetRandROutputDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut ::xlib_wrapper::Display, rrOutput: ::xlib_wrapper::RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;
}
