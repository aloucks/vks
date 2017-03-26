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

#[repr(C)]
pub struct VkSurfaceKHR_T(c_void);
pub type VkSurfaceKHR = *mut VkSurfaceKHR_T;

cenum!(VkColorSpaceKHR: u32 {
    const VK_COLORSPACE_SRGB_NONLINEAR_KHR = 0,

    #[cfg(feature = "core_1_0_13")]
    const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
});

cenum!(VkPresentModeKHR: u32 {
    const VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
    const VK_PRESENT_MODE_MAILBOX_KHR = 1,
    const VK_PRESENT_MODE_FIFO_KHR = 2,
    const VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
});

bitflags! {
    #[repr(C)]
    pub flags VkSurfaceTransformFlagsKHR: u32 {
        const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
        const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
        const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
        const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
        const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
        const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100,
    }
}
pub type VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagsKHR;

bitflags! {
    #[repr(C)]
    pub flags VkCompositeAlphaFlagsKHR: u32 {
        const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
        const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
        const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
        const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
    }
}
pub type VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
}
