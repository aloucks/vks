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

pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME: &'static [u8; 20] = b"VK_KHR_maintenance1\x00";
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME_STR: &'static str = "VK_KHR_maintenance1";

bitflags! {
    #[repr(C)]
    pub flags VkCommandPoolTrimFlagsKHR: u32 {
        const VK_COMMAND_POOL_TRIM_DUMMY_KHR = 0x00000000,
    }
}
pub type VkCommandPoolTrimFlagBitsKHR = VkCommandPoolTrimFlagsKHR;

pub type PFN_vkTrimCommandPoolKHR = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
}
