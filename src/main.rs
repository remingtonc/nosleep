#[cfg(windows)] extern crate winapi;

fn main() {
    println!("Preventing sleep!");
    prevent_sleep();
}

#[cfg(windows)]
fn prevent_sleep() {
    use winapi::shared::minwindef::{ULONG, UCHAR};
    // SystemPowerInformation struct is not expressed in WinNT.h
    // https://docs.microsoft.com/en-us/windows/win32/power/system-power-information-str
    #[repr(C)]
    struct SystemPowerInformationStruct {
        MaxIdlenessAllowed: ULONG,
        Idleness: ULONG,
        TimeRemaining: ULONG,
        CoolingMode: UCHAR
    }
    // Gets information around power
    // https://docs.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-callntpowerinformation
    use winapi::um::powerbase::CallNtPowerInformation;
    // We're interested in idle timers, indicates we want the above struct
    use winapi::um::winnt::SystemPowerInformation;
    use std::ptr::null_mut; // NULL
    use std::mem::size_of; // buffer/struct size
    // Instantiate before passing?
    let mut teststruct = SystemPowerInformationStruct {
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
            &teststruct,
            size_of::<SystemPowerInformationStruct>() as u32
        )
    };
    println!("{}", result);
    println!("{}", teststruct);
    // Eventually loop a timer based on above
    // use winapi::um::winbase::SetThreadExecutionState;
    // use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED};
    // SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
}