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
use libc::{c_char, c_void};

pub const VK_KHR_DISPLAY_EXTENSION_SPEC_VERSION: u32 = 21;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &'static [u8; 15] = b"VK_KHR_display\x00";
pub const VK_KHR_DISPLAY_EXTENSION_NAME_STR: &'static str = "VK_KHR_display";

#[repr(C)]
pub struct VkDisplayKHR_T(c_void);
pub type VkDisplayKHR = *mut VkDisplayKHR_T;

#[repr(C)]
pub struct VkDisplayModeKHR_T(c_void);
pub type VkDisplayModeKHR = *mut VkDisplayModeKHR_T;

bitflags! {
    #[repr(C)]
    pub flags VkDisplayPlaneAlphaFlagsKHR : u32 {
        const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
        const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002,
        const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004,
        const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008,
    }
}
pub type VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagsKHR;

bitflags! {
    #[repr(C)]
    pub flags VkDisplayModeCreateFlagsKHR: u32 {
        const VK_DISPLAY_MODE_CREATE_DUMMY_KHR = 0x00000000,
    }
}
pub type VkDisplayModeCreateFlagBitsKHR = VkDisplayModeCreateFlagsKHR ;

bitflags! {
    #[repr(C)]
    pub flags VkDisplaySurfaceCreateFlagsKHR: u32 {
        const VK_DISPLAY_SURFACE_CREATE_DUMMY_KHR = 0x00000000,
    }
}
pub type VkDisplaySurfaceCreateFlagBitsKHR = VkDisplaySurfaceCreateFlagsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPropertiesKHR {
    pub display: VkDisplayKHR,
    pub displayName: *const c_char,
    pub physicalDimensions: VkExtent2D,
    pub physicalResolution: VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible: VkBool32,
    pub persistentContent: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: VkExtent2D,
    pub refreshRate: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: VkOffset2D,
    pub maxSrcPosition: VkOffset2D,
    pub minSrcExtent: VkExtent2D,
    pub maxSrcExtent: VkExtent2D,
    pub minDstPosition: VkOffset2D,
    pub maxDstPosition: VkOffset2D,
    pub minDstExtent: VkExtent2D,
    pub maxDstExtent: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha: f32,
    pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: VkExtent2D,
}

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
    pub fn vkCreateDisplayModeKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}
