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

//! [`VK_AMD_texture_gather_bias_lod`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_texture_gather_bias_lod)

use core;
use libc::c_void;
use std::ptr;

pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static [u8; 31] = b"VK_AMD_texture_gather_bias_lod\x00";
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME_STR: &'static str = "VK_AMD_texture_gather_bias_lod";

/// See [`VkTextureLODGatherFormatPropertiesAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkTextureLODGatherFormatPropertiesAMD)
/// and extension [`VK_AMD_texture_gather_bias_lod`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_texture_gather_bias_lod)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub supportsTextureGatherLODBiasAMD: core::VkBool32,
}

impl Default for VkTextureLODGatherFormatPropertiesAMD {
    fn default() -> Self {
        VkTextureLODGatherFormatPropertiesAMD {
            sType: core::VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
            pNext: ptr::null_mut(),
            supportsTextureGatherLODBiasAMD: Default::default(),
        }
    }
}
