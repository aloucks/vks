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
use core;
use libc::{c_char, c_void};
use std::ptr;

pub const VK_KHR_DISPLAY_SPEC_VERSION: u32 = 21;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &'static [u8; 15] = b"VK_KHR_display\x00";
pub const VK_KHR_DISPLAY_EXTENSION_NAME_STR: &'static str = "VK_KHR_display";

define_non_dispatchable_handle!(
    /// See [`VkDisplayKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    struct VkDisplayKHR;
);

define_non_dispatchable_handle!(
    /// See [`VkDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    struct VkDisplayModeKHR;
);

vks_bitflags! {
    /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDisplayPlaneAlphaFlagsKHR: u32 {
        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

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

vks_bitflags! {
    /// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDisplayModeCreateFlagsKHR: u32 {
        /// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_MODE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type VkDisplayModeCreateFlagBitsKHR = VkDisplayModeCreateFlagsKHR ;

vks_bitflags! {
    /// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDisplaySurfaceCreateFlagsKHR: u32 {
        /// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
        /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const VK_DISPLAY_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
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
    pub physicalDimensions: core::VkExtent2D,
    pub physicalResolution: core::VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible: core::VkBool32,
    pub persistentContent: core::VkBool32,
}

impl Default for VkDisplayPropertiesKHR {
    fn default() -> Self {
        VkDisplayPropertiesKHR  {
            display: Default::default(),
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
    pub visibleRegion: core::VkExtent2D,
    pub refreshRate: u32,
}

/// See [`VkDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

/// See [`VkDisplayModeCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateInfoKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

impl Default for VkDisplayModeCreateInfoKHR {
    fn default() -> Self {
        VkDisplayModeCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR,
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
    pub minSrcPosition: core::VkOffset2D,
    pub maxSrcPosition: core::VkOffset2D,
    pub minSrcExtent: core::VkExtent2D,
    pub maxSrcExtent: core::VkExtent2D,
    pub minDstPosition: core::VkOffset2D,
    pub maxDstPosition: core::VkOffset2D,
    pub minDstExtent: core::VkExtent2D,
    pub maxDstExtent: core::VkExtent2D,
}

/// See [`VkDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlanePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}

/// See [`VkDisplaySurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateInfoKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha: f32,
    pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: core::VkExtent2D,
}

impl Default for VkDisplaySurfaceCreateInfoKHR {
    fn default() -> Self {
        VkDisplaySurfaceCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            displayMode: Default::default(),
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
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> core::VkResult;

/// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> core::VkResult;

/// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> core::VkResult;

/// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> core::VkResult;

/// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> core::VkResult;

/// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> core::VkResult;

/// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
/// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(instance: core::VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: core::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> core::VkResult;

    /// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: core::VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> core::VkResult;

    /// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: core::VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> core::VkResult;

    /// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkCreateDisplayModeKHR(physicalDevice: core::VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> core::VkResult;

    /// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: core::VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> core::VkResult;

    /// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> core::VkResult;
}
