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

/// See [`VK_API_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_API_VERSION_1_0)
pub const VK_API_VERSION_1_0: u32 = 0x00400000;

/// See [`VK_VERSION_MAJOR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_VERSION_MAJOR)
#[inline]
pub fn vk_version_major(version: u32) -> u32 {
    version >> 22
}

/// See [`VK_VERSION_MINOR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_VERSION_MINOR)
#[inline]
pub fn vk_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}

/// See [`VK_VERSION_PATCH`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_VERSION_PATCH)
#[inline]
pub fn vk_version_patch(version: u32) -> u32 {
    version & 0xfff
}

/// See [`VK_MAKE_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MAKE_VERSION)
#[inline]
pub fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 53;
