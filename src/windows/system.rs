//! Windows module implementing nosleep methods.
//! This should ideally be refactored to use the new Windows APIs.
extern crate winapi;

use log::debug;
use winapi::shared::minwindef::{ULONG, UCHAR};
use winapi::um::winbase::SetThreadExecutionState;
use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED, ES_AWAYMODE_REQUIRED};

// SystemPowerInformation struct is not expressed in WinNT.h
// https://docs.microsoft.com/en-us/windows/win32/power/system-power-information-str
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
struct _SYSTEM_POWER_INFORMATION {
    MaxIdlenessAllowed: ULONG,
    Idleness: ULONG,
    TimeRemaining: ULONG,
    CoolingMode: UCHAR
}

/// Sets ES_CONTINUOUS for the base state.
pub fn default_sleep() {
    debug!("Setting system power to default!");
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS)
    };
}

/// Sets the current thread to have properties which prevent sleep in Windows.
/// Flags ES_SYSTEM_REQUIRED and ES_AWAYMODE_REQUIRED should prevent the system from sleeping.
pub fn prevent_sleep() {
    debug!("Setting system power to not sleep!");
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED)
    };
}

/// Gets the TimeRemaining property of the idle timer.
/// This appears to be currently broken.
fn get_idle_time_remaining() -> u32 {
    // Gets information around power
    // https://docs.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-callntpowerinformation
    use winapi::um::powerbase::CallNtPowerInformation;
    // We're interested in idle timers, indicates we want the above struct
    use winapi::um::winnt::SystemPowerInformation;
    use std::ptr::null_mut; // NULL
    use std::mem::size_of; // buffer/struct size
    // Instantiate before passing?
    let mut current_information = _SYSTEM_POWER_INFORMATION {
        MaxIdlenessAllowed: 0,
        Idleness: 0,
        TimeRemaining: 0,
        CoolingMode: 0
    };
    // Fill in struct and get result code
    let result = unsafe {
        CallNtPowerInformation(
            SystemPowerInformation,
            null_mut(),
            0,
            &mut current_information as *mut _ as *mut _, // Magic from winapi author.
            size_of::<_SYSTEM_POWER_INFORMATION>() as u32
        )
    };
    if result != 0 {
        panic!("CallNtPowerInformation failed!");
    }
    return current_information.TimeRemaining;
}