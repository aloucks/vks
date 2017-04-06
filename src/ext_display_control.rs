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
use std::ptr;

pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static [u8; 23] = b"VK_EXT_display_control\x00";
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME_STR: &'static str = "VK_EXT_display_control";

cenum!(VkDisplayPowerStateEXT: u32 {
    const VK_DISPLAY_POWER_STATE_OFF_EXT = 0,
    const VK_DISPLAY_POWER_STATE_SUSPEND_EXT = 1,
    const VK_DISPLAY_POWER_STATE_ON_EXT = 2,
});

cenum!(VkDeviceEventTypeEXT: u32 {
    const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT = 0,
});

cenum!(VkDisplayEventTypeEXT: u32 {
    const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT = 0,
});

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayPowerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub powerState: VkDisplayPowerStateEXT,
}

impl Default for VkDisplayPowerInfoEXT {
    fn default() -> Self {
        VkDisplayPowerInfoEXT  {
            sType: VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT,
            pNext: ptr::null(),
            powerState: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceEvent: VkDeviceEventTypeEXT,
}

impl Default for VkDeviceEventInfoEXT {
    fn default() -> Self {
        VkDeviceEventInfoEXT  {
            sType: VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT,
            pNext: ptr::null(),
            deviceEvent: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDisplayEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub displayEvent: VkDisplayEventTypeEXT,
}

impl Default for VkDisplayEventInfoEXT {
    fn default() -> Self {
        VkDisplayEventInfoEXT  {
            sType: VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT,
            pNext: ptr::null(),
            displayEvent: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSwapchainCounterCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}

impl Default for VkSwapchainCounterCreateInfoEXT {
    fn default() -> Self {
        VkSwapchainCounterCreateInfoEXT  {
            sType: VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
            pNext: ptr::null(),
            surfaceCounters: Default::default(),
        }
    }
}

pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(device: VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(device: VkDevice, display: VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, counter: VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkDisplayPowerControlEXT(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;
    pub fn vkRegisterDeviceEventEXT(device: VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkRegisterDisplayEventEXT(device: VkDevice, display: VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkGetSwapchainCounterEXT(device: VkDevice, swapchain: VkSwapchainKHR, counter: VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> VkResult;
}
