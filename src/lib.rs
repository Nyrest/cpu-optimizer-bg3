#![feature(concat_bytes)]

use std::slice;
use std::str::FromStr;

use ini::Properties;
use logger::{report_debug, report_error};
use types::ModSettings;
use windows::Win32::Foundation::{HANDLE, HINSTANCE};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Threading::{
    GetCurrentProcess, SetPriorityClass, ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS,
    HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, PROCESS_CREATION_FLAGS,
    REALTIME_PRIORITY_CLASS,
};

mod logger;
mod types;

static mut SETTINGS_MOD: ModSettings = ModSettings {
    enabled: true,
    priority: Some(HIGH_PRIORITY_CLASS),
};

static mut CURRENT_PROCESS_HANDLE: HANDLE = HANDLE(0);

#[no_mangle]
pub extern "system" fn DllMain(
    #[allow(non_snake_case)] hinstDLL: HINSTANCE,
    #[allow(non_snake_case)] fdwReason: i32,
    #[allow(non_snake_case)] _lpvReserved: isize,
) -> i32 {
    if fdwReason == 1 {
        unsafe {
            CURRENT_PROCESS_HANDLE = GetCurrentProcess();
        };
        let lib_path: [u16; 2048] = [0; 2048];
        let lib_path_len = unsafe {
            let u8slice: &mut [u16] =
                slice::from_raw_parts_mut(lib_path.as_ptr() as *mut u16, lib_path.len());
            GetModuleFileNameW(hinstDLL, u8slice.as_mut())
        };
        let mut ini_path = unsafe {
            let slice = slice::from_raw_parts(lib_path.as_ptr() as *mut u16, lib_path_len as usize);
            String::from_utf16(&slice).unwrap()
        };
        ini_path.truncate(ini_path.rfind('.').unwrap());
        ini_path.push_str(".ini");

        if std::path::Path::new(&ini_path).exists() {
            if let Ok(ini_file) = ini::Ini::load_from_file(&ini_path) {
                if let Some(section) = ini_file.section(Some("CpuOptmizerMod")) {
                    if let Some(value) = parse_ini_value(section, "enabled") {
                        unsafe { SETTINGS_MOD.enabled = value }
                    }

                    unsafe {
                        SETTINGS_MOD.priority = parse_priority_class(section.get("priority"));
                    }
                }
            } else {
                report_error(format!("Unable to parse ini file: {0}", ini_path).as_str());
            }
        } else {
            // If the ini not exists.
            // Use the default value.
        }

        unsafe {
            // Return if the mod is disabled.
            if !SETTINGS_MOD.enabled {
                return 1;
            }

            if let Some(value) = SETTINGS_MOD.priority {
                if let Err(_) = SetPriorityClass(CURRENT_PROCESS_HANDLE, value) {
                    report_error(format!("Unable to set priority class to {0}.", value.0).as_str());
                }
            }

            // Debug
            #[cfg(debug_assertions)]
            report_debug(
                format!(
                    "Enabled:{0}\nPRIORITY:{1:#?}",
                    SETTINGS_MOD.enabled, SETTINGS_MOD.priority,
                )
                .as_str(),
            );
        }
    }
    return 1;
}

fn parse_ini_value<T: FromStr + 'static>(section: &Properties, key: &str) -> Option<T> {
    match section.get(key) {
        Some(str_value) => match str_value.parse::<T>() {
            Ok(parsed_value) => return Some(parsed_value),
            Err(_) => {
                report_error(
                    format!(
                        "Founded key \"{0}\", but unable to parse \"{1}\" as a valid value.",
                        key, str_value
                    )
                    .as_str(),
                );
                return None;
            }
        },
        None => {
            report_error(format!("Key \"{0}\" is not founded", key).as_str());
            return None;
        }
    }
}

fn parse_priority_class(priority_class: Option<&str>) -> Option<PROCESS_CREATION_FLAGS> {
    match priority_class {
        Some(priority_class) => match u32::from_str(priority_class) {
            Ok(priority_class) => match priority_class {
                0 => Some(IDLE_PRIORITY_CLASS),
                1 => Some(BELOW_NORMAL_PRIORITY_CLASS),
                2 => Some(NORMAL_PRIORITY_CLASS),
                3 => Some(ABOVE_NORMAL_PRIORITY_CLASS),
                4 => Some(HIGH_PRIORITY_CLASS),
                5 => Some(REALTIME_PRIORITY_CLASS),

                _ => None,
            },
            _ => match priority_class.to_lowercase().as_str() {
                "idle" => Some(IDLE_PRIORITY_CLASS),
                "below_normal" | "below normal" => Some(BELOW_NORMAL_PRIORITY_CLASS),
                "normal" => Some(NORMAL_PRIORITY_CLASS),
                "above_normal" | "above normal" => Some(ABOVE_NORMAL_PRIORITY_CLASS),
                "high" => Some(HIGH_PRIORITY_CLASS),
                "realtime" => Some(REALTIME_PRIORITY_CLASS),
                _ => None,
            },
        },
        None => None,
    }
}
