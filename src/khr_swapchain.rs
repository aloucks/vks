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
use libc::c_void;
use std::ptr;

pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 68;
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static [u8; 17] = b"VK_KHR_swapchain\x00";
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME_STR: &'static str = "VK_KHR_swapchain";

define_non_dispatchable_handle!(
    /// See [`VkSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    struct VkSwapchainKHR;
);

vks_bitflags! {
    /// See [`VkSwapchainCreateFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateFlagBitsKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkSwapchainCreateFlagsKHR: u32 {
        /// See [`VkSwapchainCreateFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateFlagBitsKHR)
        /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const VK_SWAPCHAIN_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

        #[cfg(feature = "experimental")]
        /// See [`VkSwapchainCreateFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateFlagBitsKHR)
        /// and extensions [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain),
        /// [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX = 0x00000001;
    }
}

/// See [`VkSwapchainCreateFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateFlagBitsKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
pub type VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagsKHR;

/// See [`VkSwapchainCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateInfoKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: core::VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: core::VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: core::VkImageUsageFlags,
    pub imageSharingMode: core::VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: core::VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

impl Default for VkSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkSwapchainCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            minImageCount: Default::default(),
            imageFormat: Default::default(),
            imageColorSpace: Default::default(),
            imageExtent: Default::default(),
            imageArrayLayers: Default::default(),
            imageUsage: Default::default(),
            imageSharingMode: Default::default(),
            queueFamilyIndexCount: Default::default(),
            pQueueFamilyIndices: ptr::null(),
            preTransform: Default::default(),
            compositeAlpha: Default::default(),
            presentMode: Default::default(),
            clipped: Default::default(),
            oldSwapchain: Default::default(),
        }
    }
}

/// See [`VkPresentInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentInfoKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const core::VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut core::VkResult,
}

impl Default for VkPresentInfoKHR {
    fn default() -> Self {
        VkPresentInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: ptr::null(),
            waitSemaphoreCount: Default::default(),
            pWaitSemaphores: ptr::null(),
            swapchainCount: Default::default(),
            pSwapchains: ptr::null(),
            pImageIndices: ptr::null(),
            pResults: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSwapchainKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(device: core::VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> core::VkResult;

/// See [`vkDestroySwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySwapchainKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(device: core::VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const core::VkAllocationCallbacks);

/// See [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainImagesKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(device: core::VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut core::VkImage) -> core::VkResult;

/// See [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(device: core::VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: core::VkSemaphore, fence: core::VkFence, pImageIndex: *mut u32) -> core::VkResult;

/// See [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
/// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(queue: core::VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSwapchainKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn vkCreateSwapchainKHR(device: core::VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> core::VkResult;

    /// See [`vkDestroySwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySwapchainKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn vkDestroySwapchainKHR(device: core::VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const core::VkAllocationCallbacks);

    /// See [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainImagesKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn vkGetSwapchainImagesKHR(device: core::VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut core::VkImage) -> core::VkResult;

    /// See [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn vkAcquireNextImageKHR(device: core::VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: core::VkSemaphore, fence: core::VkFence, pImageIndex: *mut u32) -> core::VkResult;

    /// See [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn vkQueuePresentKHR(queue: core::VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> core::VkResult;
}
