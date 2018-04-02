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

//! [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)

use vk;

pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance1\x00";
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME_STR: &str = "VK_KHR_maintenance1";

bitflags! {
    /// See [`VkCommandPoolTrimFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolTrimFlagsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCommandPoolTrimFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkCommandPoolTrimFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolTrimFlagsKHR)
pub type VkCommandPoolTrimFlagBitsKHR = VkCommandPoolTrimFlagsKHR;

/// See [`vkTrimCommandPoolKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkTrimCommandPoolKHR)
pub type PFN_vkTrimCommandPoolKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, commandPool: vk::VkCommandPool, flags: VkCommandPoolTrimFlagsKHR)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkTrimCommandPoolKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkTrimCommandPoolKHR)
    pub fn vkTrimCommandPoolKHR(device: vk::VkDevice, commandPool: vk::VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
}
