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

//! [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)

use core::ptr;
use khr_surface;
use libc::{c_char, c_void};
use vk;

pub const VK_KHR_DISPLAY_SPEC_VERSION: u32 = 21;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &[u8; 15] = b"VK_KHR_display\x00";
pub const VK_KHR_DISPLAY_EXTENSION_NAME_STR: &str = "VK_KHR_display";

define_non_dispatchable_handle!(
    /// See [`VkDisplayKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayKHR)
    struct VkDisplayKHR;
);

define_non_dispatchable_handle!(
    /// See [`VkDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeKHR)
    struct VkDisplayModeKHR;
);

vks_bitflags! {
    /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDisplayPlaneAlphaFlagsKHR: u32 {
        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        const VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004;

        /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
        const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008;
    }
}

/// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
pub type VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagsKHR;

vks_bitflags! {
    /// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDisplayModeCreateFlagsKHR: u32 {
        /// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
        const VK_DISPLAY_MODE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
pub type VkDisplayModeCreateFlagBitsKHR = VkDisplayModeCreateFlagsKHR;

vks_bitflags! {
    /// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDisplaySurfaceCreateFlagsKHR: u32 {
        /// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
        const VK_DISPLAY_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
pub type VkDisplaySurfaceCreateFlagBitsKHR = VkDisplaySurfaceCreateFlagsKHR;

/// See [`VkDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPropertiesKHR {
    pub display: VkDisplayKHR,
    pub displayName: *const c_char,
    pub physicalDimensions: vk::VkExtent2D,
    pub physicalResolution: vk::VkExtent2D,
    pub supportedTransforms: khr_surface::VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible: vk::VkBool32,
    pub persistentContent: vk::VkBool32,
}

impl Default for VkDisplayPropertiesKHR {
    fn default() -> Self {
        VkDisplayPropertiesKHR {
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
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: vk::VkExtent2D,
    pub refreshRate: u32,
}

/// See [`VkDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModePropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

/// See [`VkDisplayModeCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

impl Default for VkDisplayModeCreateInfoKHR {
    fn default() -> Self {
        VkDisplayModeCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            parameters: Default::default(),
        }
    }
}

/// See [`VkDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneCapabilitiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: vk::VkOffset2D,
    pub maxSrcPosition: vk::VkOffset2D,
    pub minSrcExtent: vk::VkExtent2D,
    pub maxSrcExtent: vk::VkExtent2D,
    pub minDstPosition: vk::VkOffset2D,
    pub maxDstPosition: vk::VkOffset2D,
    pub minDstExtent: vk::VkExtent2D,
    pub maxDstExtent: vk::VkExtent2D,
}

/// See [`VkDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlanePropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}

/// See [`VkDisplaySurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: khr_surface::VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha: f32,
    pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: vk::VkExtent2D,
}

impl Default for VkDisplaySurfaceCreateInfoKHR {
    fn default() -> Self {
        VkDisplaySurfaceCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
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
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> vk::VkResult>;

/// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> vk::VkResult>;

/// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
pub type PFN_vkGetDisplayModePropertiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> vk::VkResult>;

/// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
pub type PFN_vkCreateDisplayModeKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> vk::VkResult>;

/// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> vk::VkResult>;

/// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: vk::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: vk::VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> vk::VkResult;

    /// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: vk::VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> vk::VkResult;

    /// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: vk::VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> vk::VkResult;

    /// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
    pub fn vkCreateDisplayModeKHR(physicalDevice: vk::VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> vk::VkResult;

    /// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: vk::VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> vk::VkResult;

    /// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: vk::VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult;
}
