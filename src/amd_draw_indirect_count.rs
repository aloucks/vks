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

//! [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)

use vk;

pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
pub const VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &[u8; 27] = b"VK_AMD_draw_indirect_count\x00";
pub const VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME_STR: &str = "VK_AMD_draw_indirect_count";

/// See [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
pub type PFN_vkCmdDrawIndirectCountAMD = Option<unsafe extern "system" fn(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, countBuffer: vk::VkBuffer, countBufferOffset: vk::VkDeviceSize, maxDrawCount: u32, stride: u32)>;

/// See [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = Option<unsafe extern "system" fn(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, countBuffer: vk::VkBuffer, countBufferOffset: vk::VkDeviceSize, maxDrawCount: u32, stride: u32)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
    pub fn vkCmdDrawIndirectCountAMD(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, countBuffer: vk::VkBuffer, countBufferOffset: vk::VkDeviceSize, maxDrawCount: u32, stride: u32);

    /// See [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
    pub fn vkCmdDrawIndexedIndirectCountAMD(commandBuffer: vk::VkCommandBuffer, buffer: vk::VkBuffer, offset: vk::VkDeviceSize, countBuffer: vk::VkBuffer, countBufferOffset: vk::VkDeviceSize, maxDrawCount: u32, stride: u32);
}
