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
use std::ptr;

pub const VK_KHR_DISPLAY_SPEC_VERSION: u32 = 21;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &'static [u8; 15] = b"VK_KHR_display\x00";
pub const VK_KHR_DISPLAY_EXTENSION_NAME_STR: &'static str = "VK_KHR_display";

/// See [`VkDisplayKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
pub struct VkDisplayKHR_T(c_void);

/// See [`VkDisplayKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type VkDisplayKHR = *mut VkDisplayKHR_T;

/// See [`VkDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
pub struct VkDisplayModeKHR_T(c_void);

/// See [`VkDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type VkDisplayModeKHR = *mut VkDisplayModeKHR_T;

bitflags! {
    /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDisplayPlaneAlphaFlagsKHR: u32 {
        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008;
    }
}

/// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagsKHR;

bitflags! {
    /// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDisplayModeCreateFlagsKHR: u32 {
        const VK_DISPLAY_MODE_CREATE_DUMMY_KHR = 0x00000000;
    }
}

/// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type VkDisplayModeCreateFlagBitsKHR = VkDisplayModeCreateFlagsKHR ;

bitflags! {
    /// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDisplaySurfaceCreateFlagsKHR: u32 {
        const VK_DISPLAY_SURFACE_CREATE_DUMMY_KHR = 0x00000000;
    }
}

/// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type VkDisplaySurfaceCreateFlagBitsKHR = VkDisplaySurfaceCreateFlagsKHR;

/// See [`VkDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
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

impl Default for VkDisplayPropertiesKHR {
    fn default() -> Self {
        VkDisplayPropertiesKHR  {
            display: ptr::null_mut(),
            displayName: ptr::null(),
            physicalDimensions: Default::default(),
            physicalResolution: Default::default(),
            supportedTransforms: Default::default(),
            planeReorderPossible: Default::default(),
            persistentContent: Default::default(),
        }
    }
}

/// See [`VkDisplayModeParametersKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeParametersKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: VkExtent2D,
    pub refreshRate: u32,
}

/// See [`VkDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

impl Default for VkDisplayModePropertiesKHR {
    fn default() -> Self {
        VkDisplayModePropertiesKHR  {
            displayMode: ptr::null_mut(),
            parameters: Default::default(),
        }
    }
}

/// See [`VkDisplayModeCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateInfoKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

impl Default for VkDisplayModeCreateInfoKHR {
    fn default() -> Self {
        VkDisplayModeCreateInfoKHR  {
            sType: VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            parameters: Default::default(),
        }
    }
}

/// See [`VkDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneCapabilitiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
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

/// See [`VkDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlanePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}

impl Default for VkDisplayPlanePropertiesKHR {
    fn default() -> Self {
        VkDisplayPlanePropertiesKHR  {
            currentDisplay: ptr::null_mut(),
            currentStackIndex: Default::default(),
        }
    }
}

/// See [`VkDisplaySurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateInfoKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
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

impl Default for VkDisplaySurfaceCreateInfoKHR {
    fn default() -> Self {
        VkDisplaySurfaceCreateInfoKHR  {
            sType: VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            displayMode: ptr::null_mut(),
            planeIndex: Default::default(),
            planeStackIndex: Default::default(),
            transform: Default::default(),
            globalAlpha: Default::default(),
            alphaMode: Default::default(),
            imageExtent: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;

/// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;

/// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;

/// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;

/// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;

/// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;

/// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;

    /// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;

    /// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;

    /// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;

    /// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkCreateDisplayModeKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;

    /// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;

    /// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}
