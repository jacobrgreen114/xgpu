// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::api::traits::*;
use crate::api::vulkan::*;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::sync::OnceLock;

use crate::prelude::ApiRoot;
use crate::RootCreateInfo;
use vulkan_sys::*;

struct InstanceOwnership {
    handle: VkInstance,
    physical_devices: OnceLock<Vec<<VulkanApi as GraphicsApi>::Device>>,

    #[cfg(feature = "gpu_debugging")]
    debug_messenger: VkDebugUtilsMessengerEXT,

    #[cfg(feature = "gpu_debugging")]
    destroy_debug_utils:
        unsafe extern "C" fn(VkInstance, VkDebugUtilsMessengerEXT, *const VkAllocationCallbacks),
}

impl Drop for InstanceOwnership {
    fn drop(&mut self) {
        #[cfg(feature = "gpu_debugging")]
        vk::destroy_debug_utils_messenger_ext(
            self.destroy_debug_utils,
            self.handle,
            self.debug_messenger,
            None,
        );
        vk::destroy_instance(vkDestroyInstance, self.handle, None);
    }
}

#[derive(Clone)]
pub struct VulkanInstance {
    handle: VkInstance,
    ownership: Ownership<InstanceOwnership>,
}

impl Debug for VulkanInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .finish()
    }
}

fn get_instance_layers() -> Vec<*const std::ffi::c_char> {
    let mut vec = Vec::new();

    #[cfg(feature = "gpu_debugging")]
    vec.push(VK_KHR_VALIDATION_LAYER_NAME.as_ptr() as *const _);

    vec
}

fn get_instance_extensions() -> Vec<*const std::ffi::c_char> {
    let mut vec: Vec<*const std::ffi::c_char> = Vec::new();
    vec.reserve(8);

    #[cfg(feature = "graphics")]
    {
        vec.push(VK_KHR_SURFACE_EXTENSION_NAME.as_ptr() as *const _);

        #[cfg(target_os = "windows")]
        vec.push(VK_KHR_WIN32_SURFACE_EXTENSION_NAME.as_ptr() as *const _);

        #[cfg(not(target_os = "windows"))]
        panic!("Platform not implemented");
    }

    #[cfg(feature = "gpu_debugging")]
    vec.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const _);

    vec
}

impl ApiRoot<VulkanApi> for VulkanInstance {
    fn new(_create_info: &RootCreateInfo) -> crate::Result<Self> {
        let layers = get_instance_layers();
        let extensions = get_instance_extensions();

        let instance_create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            pApplicationInfo: std::ptr::null(),
            enabledLayerCount: layers.len() as u32,
            ppEnabledLayerNames: layers.as_ptr(),
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: extensions.as_ptr(),
        };

        let handle = vk::create_instance(vkCreateInstance, &instance_create_info, None)?;

        #[cfg(feature = "gpu_debugging")]
        let (debug_messenger, destroy_debug_utils) = {
            let create_debug_utils = {
                let create: PFN_vkCreateDebugUtilsMessengerEXT = unsafe {
                    std::mem::transmute(vk::get_instance_proc_addr(
                        vkGetInstanceProcAddr,
                        handle,
                        c"vkCreateDebugUtilsMessengerEXT",
                    ))
                };
                create.unwrap()
            };

            #[cfg(feature = "gpu_debugging")]
            let destroy_debug_utils = {
                let destroy: PFN_vkDestroyDebugUtilsMessengerEXT = unsafe {
                    std::mem::transmute(vk::get_instance_proc_addr(
                        vkGetInstanceProcAddr,
                        handle,
                        c"vkDestroyDebugUtilsMessengerEXT",
                    ))
                };

                destroy.unwrap()
            };

            let debug_utils_create_info = VkDebugUtilsMessengerCreateInfoEXT {
                sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
                pNext: std::ptr::null(),
                flags: 0,
                messageSeverity: (VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT
                    | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT
                    | VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT
                    | VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT)
                    as VkDebugUtilsMessageSeverityFlagsEXT,
                messageType: (VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
                    | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
                    | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT)
                    as VkDebugUtilsMessageTypeFlagsEXT,
                pfnUserCallback: Some(debug_utils_callback),
                pUserData: std::ptr::null_mut(),
            };

            let debug_messenger = vk::create_debug_utils_messenger_ext(
                create_debug_utils,
                handle,
                &debug_utils_create_info,
                None,
            )?;

            (debug_messenger, destroy_debug_utils)
        };

        let ownership = Ownership::new(InstanceOwnership {
            handle,
            physical_devices: OnceLock::new(),

            #[cfg(feature = "gpu_debugging")]
            debug_messenger,

            #[cfg(feature = "gpu_debugging")]
            destroy_debug_utils,
        });

        Ok(VulkanInstance { handle, ownership })
    }

    fn devices(&self) -> &[<VulkanApi as GraphicsApi>::Device] {
        self.ownership
            .physical_devices
            .get_or_init(|| self.enumerate_physical_device().unwrap())
            .as_slice()
    }
}

impl VulkanInstance {
    fn enumerate_physical_device(&self) -> crate::Result<Vec<<VulkanApi as GraphicsApi>::Device>> {
        let to_physical_device = |handle: VkPhysicalDevice| {
            let properties =
                vk::get_physical_device_properties(vkGetPhysicalDeviceProperties, handle);
            let features = vk::get_physical_device_features(vkGetPhysicalDeviceFeatures, handle);

            VulkanPhysicalDevice::new(handle, properties, features)
        };

        let convert_handles =
            |handles: Vec<VkPhysicalDevice>| handles.into_iter().map(to_physical_device).collect();

        Ok(
            vk::enumerate_physical_device(vkEnumeratePhysicalDevices, self.handle)
                .map(convert_handles)?,
        )
    }
}

impl VulkanObject for VulkanInstance {
    type Handle = VkInstance;

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

/*
   Debug Utils
*/

// todo : implement user application callbacks
#[cfg(feature = "gpu_debugging")]
unsafe extern "C" fn debug_utils_callback(
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_type: VkDebugUtilsMessageTypeFlagsEXT,
    data: *const VkDebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> VkBool32 {
    debug_utils_callback_safe(severity, message_type, &*data);
    VK_FALSE
}

#[cfg(feature = "gpu_debugging")]
struct DebugUtilsObjectNameWrapper<'a> {
    object_type: VkObjectType,
    object_name: &'a str,
}

#[cfg(feature = "gpu_debugging")]
impl Debug for DebugUtilsObjectNameWrapper<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("type", &self.object_type)
            .field("name", &self.object_name)
            .finish()
    }
}

#[cfg(feature = "gpu_debugging")]
fn create_message(data: &VkDebugUtilsMessengerCallbackDataEXT) -> String {
    // #[cfg(windows)]
    // const LINE_ENDING: &'static str = "\r\n";
    // #[cfg(not(windows))]
    // const LINE_ENDING: &'static str = "\n";

    let message_name = unsafe {
        std::ffi::CStr::from_ptr(data.pMessageIdName)
            .to_str()
            .unwrap()
    };
    let message = unsafe { std::ffi::CStr::from_ptr(data.pMessage).to_str().unwrap() };

    // let objects = unsafe { std::slice::from_raw_parts(data.pObjects, data.objectCount as usize) }
    //     .iter()
    //     .map(|object| {
    //         let object_name = unsafe {
    //             (object.pObjectName != std::ptr::null())
    //                 .then(|| {
    //                     std::ffi::CStr::from_ptr(object.pObjectName)
    //                         .to_str()
    //                         .unwrap()
    //                 })
    //                 .unwrap_or("None")
    //         };
    //         DebugUtilsObjectNameWrapper {
    //             object_type: object.objectType,
    //             object_name,
    //         }
    //     })
    //     .collect::<Vec<_>>();

    // let formatted_message = format!(
    //     "Objects = {:#?}{}{}: {}",
    //     objects, LINE_ENDING, message_name, message
    // );

    let formatted_message = format!("{}: {}", message_name, message);

    formatted_message
}

#[cfg(feature = "gpu_debugging")]
fn debug_utils_callback_safe(
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    _types: VkDebugUtilsMessageTypeFlagsEXT,
    data: &VkDebugUtilsMessengerCallbackDataEXT,
) {
    if severity == VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT {
        panic!("{}", create_message(data));
    }

    let level = match severity {
        VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT => log::Level::Trace,
        VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT => log::Level::Debug,
        VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT => log::Level::Warn,
        VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT => log::Level::Error,
        _ => panic!(),
    };

    if log::logger().enabled(&log::Metadata::builder().level(level).build()) {
        let msg = create_message(data);

        log::logger().log(
            &log::Record::builder()
                .args(format_args!("{}", msg))
                .level(level)
                .target("Vulkan")
                .build(),
        );
    }
}
