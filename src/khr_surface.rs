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

//! [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)

use vk;

pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &[u8; 15] = b"VK_KHR_surface\x00";
pub const VK_KHR_SURFACE_EXTENSION_NAME_STR: &str = "VK_KHR_surface";

define_non_dispatchable_handle!(
    /// See [`VkSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceKHR)
    struct VkSurfaceKHR;
);

cenum!(VkColorSpaceKHR: u32 {
    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    const VK_COLORSPACE_SRGB_NONLINEAR_KHR = 0,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT = 1000104001,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT = 1000104002,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT = 1000104003,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT = 1000104004,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_BT709_LINEAR_EXT = 1000104005,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_BT709_NONLINEAR_EXT = 1000104006,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_BT2020_LINEAR_EXT = 1000104007,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_HDR10_ST2084_EXT = 1000104008,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_DOLBYVISION_EXT = 1000104009,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_HDR10_HLG_EXT = 1000104010,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT = 1000104011,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT = 1000104012,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_PASS_THROUGH_EXT = 1000104013,

    /// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
    /// and extension [`VK_EXT_swapchain_colorspace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_swapchain_colorspace)
    const VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT = 1000104014,
});

cenum!(VkPresentModeKHR: u32 {
    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    const VK_PRESENT_MODE_IMMEDIATE_KHR = 0,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    const VK_PRESENT_MODE_MAILBOX_KHR = 1,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    const VK_PRESENT_MODE_FIFO_KHR = 2,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    const VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    /// and extension [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharedPresentSurfaceCapabilitiesKHR)
    const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR = 1000111000,

    /// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
    /// and extension [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharedPresentSurfaceCapabilitiesKHR)
    const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
});

bitflags! {
    /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSurfaceTransformFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
        const IDENTITY_BIT_KHR = 0x00000001;
        const ROTATE_90_BIT_KHR = 0x00000002;
        const ROTATE_180_BIT_KHR = 0x00000004;
        const ROTATE_270_BIT_KHR = 0x00000008;
        const HORIZONTAL_MIRROR_BIT_KHR = 0x00000010;
        const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020;
        const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040;
        const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080;
        const INHERIT_BIT_KHR = 0x00000100;
    }
}

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub type VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagsKHR;

bitflags! {
    /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCompositeAlphaFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
        const OPAQUE_BIT_KHR = 0x00000001;
        const PRE_MULTIPLIED_BIT_KHR = 0x00000002;
        const POST_MULTIPLIED_BIT_KHR = 0x00000004;
        const INHERIT_BIT_KHR = 0x00000008;
    }
}

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub type VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagsKHR;

/// See [`VkSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilitiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: vk::VkExtent2D,
    pub minImageExtent: vk::VkExtent2D,
    pub maxImageExtent: vk::VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: vk::VkImageUsageFlags,
}

/// See [`VkSurfaceFormatKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceFormatKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSurfaceFormatKHR {
    pub format: vk::VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

/// See [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySurfaceKHR)
pub type PFN_vkDestroySurfaceKHR = Option<unsafe extern "system" fn(instance: vk::VkInstance, surface: VkSurfaceKHR, pAllocator: *const vk::VkAllocationCallbacks)>;

/// See [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut vk::VkBool32) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySurfaceKHR)
    pub fn vkDestroySurfaceKHR(instance: vk::VkInstance, surface: VkSurfaceKHR, pAllocator: *const vk::VkAllocationCallbacks);

    /// See [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: vk::VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut vk::VkBool32) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: vk::VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: vk::VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: vk::VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> vk::VkResult;
}
