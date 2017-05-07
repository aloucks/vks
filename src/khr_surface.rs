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

pub const VK_KHR_SURFACE_EXTENSION_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &'static [u8; 15] = b"VK_KHR_surface\x00";
pub const VK_KHR_SURFACE_EXTENSION_NAME_STR: &'static str  = "VK_KHR_surface";

/// See [`VkSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
#[repr(C)]
pub struct VkSurfaceKHR_T(c_void);

/// See [`VkSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type VkSurfaceKHR = *mut VkSurfaceKHR_T;

cenum!(VkColorSpaceKHR: u32 {
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_COLORSPACE_SRGB_NONLINEAR_KHR = 0,

    #[cfg(feature = "core_1_0_13")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT = 1000104001,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT = 1000104002,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT = 1000104003,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT = 1000104004,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_BT709_LINEAR_EXT = 1000104005,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_BT709_NONLINEAR_EXT = 1000104006,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_BT2020_LINEAR_EXT = 1000104007,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_HDR10_ST2084_EXT = 1000104008,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DOLBYVISION_EXT = 1000104009,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_HDR10_HLG_EXT = 1000104010,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT = 1000104011,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT = 1000104012,

    #[cfg(feature = "ext_swapchain_colorspace_2")]
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extensions [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface),
    /// [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_PASS_THROUGH_EXT = 1000104013,
});

cenum!(VkPresentModeKHR: u32 {
    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_PRESENT_MODE_IMMEDIATE_KHR = 0,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_PRESENT_MODE_MAILBOX_KHR = 1,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_PRESENT_MODE_FIFO_KHR = 2,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
});

bitflags! {
    /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSurfaceTransformFlagsKHR: u32 {
        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,

        /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100,
    }
}

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagsKHR;

bitflags! {
    /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkCompositeAlphaFlagsKHR: u32 {
        /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,

        /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,

        /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,

        /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
    }
}

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagsKHR;

/// See [`VkSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilitiesKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

/// See [`VkSurfaceFormatKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceFormatKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

/// See [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySurfaceKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);

/// See [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;

/// See [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;

/// See [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;

/// See [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
/// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySurfaceKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;

    /// See [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;

    /// See [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;

    /// See [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
}
