extern crate winapi;

use winapi::shared::minwindef::{ULONG, UCHAR};
use winapi::um::winbase::SetThreadExecutionState;
use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED, ES_AWAYMODE_REQUIRED};

// SystemPowerInformation struct is not expressed in WinNT.h
// https://docs.microsoft.com/en-us/windows/win32/power/system-power-information-str
#[repr(C)]
#[derive(Debug)]
struct _SYSTEM_POWER_INFORMATION {
    MaxIdlenessAllowed: ULONG,
    Idleness: ULONG,
    TimeRemaining: ULONG,
    CoolingMode: UCHAR
}

pub fn default_sleep() {
    println!("Setting system power to default!");
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS)
    };
}

pub fn prevent_sleep() {
    println!("Setting system power to not sleep!");
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED)
    };
}

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
            &mut current_information as *mut _ as *mut _,
            size_of::<_SYSTEM_POWER_INFORMATION>() as u32
        )
    };
    if result != 0 {
        panic!("CallNtPowerInformation failed!");
    }
    return current_information.TimeRemaining;
}