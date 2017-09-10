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

//! [`VK_EXT_direct_mode_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_direct_mode_display)

use core;
use khr_display;

pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static [u8; 27] = b"VK_EXT_direct_mode_display\x00";
pub const VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME_STR: &'static str = "VK_EXT_direct_mode_display";

/// See [`vkReleaseDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkReleaseDisplayEXT)
pub type PFN_vkReleaseDisplayEXT = Option<unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, display: khr_display::VkDisplayKHR) -> core::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkReleaseDisplayEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkReleaseDisplayEXT)
    pub fn vkReleaseDisplayEXT(physicalDevice: core::VkPhysicalDevice, display: khr_display::VkDisplayKHR) -> core::VkResult;
}
