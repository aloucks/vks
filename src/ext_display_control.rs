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

//! [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)

use core;
use ext_display_surface_counter;
use khr_display;
use khr_swapchain;
use libc::c_void;
use std::ptr;

pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static [u8; 23] = b"VK_EXT_display_control\x00";
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME_STR: &'static str = "VK_EXT_display_control";

cenum!(VkDisplayPowerStateEXT: u32 {
    /// See [`VkDisplayPowerStateEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPowerStateEXT)
    const VK_DISPLAY_POWER_STATE_OFF_EXT = 0,

    /// See [`VkDisplayPowerStateEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPowerStateEXT)
    const VK_DISPLAY_POWER_STATE_SUSPEND_EXT = 1,

    /// See [`VkDisplayPowerStateEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPowerStateEXT)
    const VK_DISPLAY_POWER_STATE_ON_EXT = 2,
});

cenum!(VkDeviceEventTypeEXT: u32 {
    /// See [`VkDeviceEventTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceEventTypeEXT)
    const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT = 0,
});

cenum!(VkDisplayEventTypeEXT: u32 {
    /// See [`VkDisplayEventTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayEventTypeEXT)
    const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT = 0,
});

/// See [`VkDisplayPowerInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPowerInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPowerInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub powerState: VkDisplayPowerStateEXT,
}

impl Default for VkDisplayPowerInfoEXT {
    fn default() -> Self {
        VkDisplayPowerInfoEXT  {
            sType: core::VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT,
            pNext: ptr::null(),
            powerState: Default::default(),
        }
    }
}

/// See [`VkDeviceEventInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceEventInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceEventInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub deviceEvent: VkDeviceEventTypeEXT,
}

impl Default for VkDeviceEventInfoEXT {
    fn default() -> Self {
        VkDeviceEventInfoEXT  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT,
            pNext: ptr::null(),
            deviceEvent: Default::default(),
        }
    }
}

/// See [`VkDisplayEventInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayEventInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayEventInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub displayEvent: VkDisplayEventTypeEXT,
}

impl Default for VkDisplayEventInfoEXT {
    fn default() -> Self {
        VkDisplayEventInfoEXT  {
            sType: core::VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT,
            pNext: ptr::null(),
            displayEvent: Default::default(),
        }
    }
}

/// See [`VkSwapchainCounterCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCounterCreateInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSwapchainCounterCreateInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub surfaceCounters: ext_display_surface_counter::VkSurfaceCounterFlagsEXT,
}

impl Default for VkSwapchainCounterCreateInfoEXT {
    fn default() -> Self {
        VkSwapchainCounterCreateInfoEXT  {
            sType: core::VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
            pNext: ptr::null(),
            surfaceCounters: Default::default(),
        }
    }
}

/// See [`vkDisplayPowerControlEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDisplayPowerControlEXT)
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(device: core::VkDevice, display: khr_display::VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> core::VkResult;

/// See [`vkRegisterDeviceEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDeviceEventEXT)
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(device: core::VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult;

/// See [`vkRegisterDisplayEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDisplayEventEXT)
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(device: core::VkDevice, display: khr_display::VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult;

/// See [`vkGetSwapchainCounterEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainCounterEXT)
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, counter: ext_display_surface_counter::VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> core::VkResult;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkDisplayPowerControlEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDisplayPowerControlEXT)
    pub fn vkDisplayPowerControlEXT(device: core::VkDevice, display: khr_display::VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> core::VkResult;

    /// See [`vkRegisterDeviceEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDeviceEventEXT)
    pub fn vkRegisterDeviceEventEXT(device: core::VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult;

    /// See [`vkRegisterDisplayEventEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterDisplayEventEXT)
    pub fn vkRegisterDisplayEventEXT(device: core::VkDevice, display: khr_display::VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pFence: *mut core::VkFence) -> core::VkResult;

    /// See [`vkGetSwapchainCounterEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainCounterEXT)
    pub fn vkGetSwapchainCounterEXT(device: core::VkDevice, swapchain: khr_swapchain::VkSwapchainKHR, counter: ext_display_surface_counter::VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> core::VkResult;
}
