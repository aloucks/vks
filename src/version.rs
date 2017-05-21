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

#[cfg(all(feature = "core_1_0_50", not(feature = "core_1_0_51")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 50;

#[cfg(all(feature = "core_1_0_49", not(feature = "core_1_0_50")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 49;

#[cfg(all(feature = "core_1_0_48", not(feature = "core_1_0_49")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 48;

#[cfg(all(feature = "core_1_0_47", not(feature = "core_1_0_48")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 47;

#[cfg(all(feature = "core_1_0_46", not(feature = "core_1_0_47")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 46;

#[cfg(all(feature = "core_1_0_45", not(feature = "core_1_0_46")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 45;

#[cfg(all(feature = "core_1_0_44", not(feature = "core_1_0_45")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 44;

#[cfg(all(feature = "core_1_0_43", not(feature = "core_1_0_44")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 43;

#[cfg(all(feature = "core_1_0_42", not(feature = "core_1_0_43")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 42;

#[cfg(all(feature = "core_1_0_41", not(feature = "core_1_0_42")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 41;

#[cfg(all(feature = "core_1_0_40", not(feature = "core_1_0_41")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 40;

#[cfg(all(feature = "core_1_0_39", not(feature = "core_1_0_40")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 39;

#[cfg(all(feature = "core_1_0_38", not(feature = "core_1_0_39")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 38;

#[cfg(all(feature = "core_1_0_37", not(feature = "core_1_0_38")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 37;

#[cfg(all(feature = "core_1_0_36", not(feature = "core_1_0_37")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 36;

#[cfg(all(feature = "core_1_0_35", not(feature = "core_1_0_36")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 35;

#[cfg(all(feature = "core_1_0_34", not(feature = "core_1_0_35")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 34;

#[cfg(all(feature = "core_1_0_33", not(feature = "core_1_0_34")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 33;

#[cfg(all(feature = "core_1_0_32", not(feature = "core_1_0_33")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 32;

#[cfg(all(feature = "core_1_0_31", not(feature = "core_1_0_32")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 31;

#[cfg(all(feature = "core_1_0_30", not(feature = "core_1_0_31")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 30;

#[cfg(all(feature = "core_1_0_29", not(feature = "core_1_0_30")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 29;

#[cfg(all(feature = "core_1_0_28", not(feature = "core_1_0_29")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 28;

#[cfg(all(feature = "core_1_0_27", not(feature = "core_1_0_28")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 27;

#[cfg(all(feature = "core_1_0_26", not(feature = "core_1_0_27")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 26;

#[cfg(all(feature = "core_1_0_25", not(feature = "core_1_0_26")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 25;

#[cfg(all(feature = "core_1_0_24", not(feature = "core_1_0_25")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 24;

#[cfg(all(feature = "core_1_0_23", not(feature = "core_1_0_24")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 23;

#[cfg(all(feature = "core_1_0_22", not(feature = "core_1_0_23")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 22;

#[cfg(all(feature = "core_1_0_21", not(feature = "core_1_0_22")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 21;

#[cfg(all(feature = "core_1_0_20", not(feature = "core_1_0_21")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 20;

#[cfg(all(feature = "core_1_0_19", not(feature = "core_1_0_20")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 19;

#[cfg(all(feature = "core_1_0_18", not(feature = "core_1_0_19")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 18;

#[cfg(all(feature = "core_1_0_17", not(feature = "core_1_0_18")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 17;

#[cfg(all(feature = "core_1_0_16", not(feature = "core_1_0_17")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 16;

#[cfg(all(feature = "core_1_0_15", not(feature = "core_1_0_16")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 15;

#[cfg(all(feature = "core_1_0_14", not(feature = "core_1_0_15")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 14;

#[cfg(all(feature = "core_1_0_13", not(feature = "core_1_0_14")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 13;

#[cfg(all(feature = "core_1_0_12", not(feature = "core_1_0_13")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 12;

#[cfg(all(feature = "core_1_0_11", not(feature = "core_1_0_12")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 11;

#[cfg(all(feature = "core_1_0_10", not(feature = "core_1_0_11")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 10;

#[cfg(all(feature = "core_1_0_9", not(feature = "core_1_0_10")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 9;

#[cfg(all(feature = "core_1_0_8", not(feature = "core_1_0_9")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 8;

#[cfg(all(feature = "core_1_0_7", not(feature = "core_1_0_8")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 7;

#[cfg(all(feature = "core_1_0_6", not(feature = "core_1_0_7")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 6;

#[cfg(all(feature = "core_1_0_5", not(feature = "core_1_0_6")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 5;

#[cfg(all(feature = "core_1_0_4", not(feature = "core_1_0_5")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 4;

#[cfg(all(feature = "core_1_0_3", not(feature = "core_1_0_4")))]
/// See [`VK_HEADER_VERSION`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_HEADER_VERSION)
pub const VK_HEADER_VERSION: u32 = 3;
