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

//! [`VK_EXT_validation_flags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_validation_flags)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_validation_flags\x00";
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME_STR: &str = "VK_EXT_validation_flags";

vks_enum! {
    /// See [`VkValidationCheckEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkValidationCheckEXT)
    pub VkValidationCheckEXT: u32 {
        const ALL_EXT = 0;
        const SHADERS_EXT = 1;
    }
}

/// See [`VkValidationFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkValidationFlagsEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkValidationFlagsEXT {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub disabledValidationCheckCount: u32,
    pub pDisabledValidationChecks: *mut VkValidationCheckEXT,
}

impl Default for VkValidationFlagsEXT {
    fn default() -> Self {
        VkValidationFlagsEXT {
            sType: vk::VkStructureType::VALIDATION_FLAGS_EXT,
            pNext: ptr::null(),
            disabledValidationCheckCount: Default::default(),
            pDisabledValidationChecks: ptr::null_mut(),
        }
    }
}
