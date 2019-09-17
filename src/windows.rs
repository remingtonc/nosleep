extern crate winapi;

pub fn prevent_sleep() {
    use winapi::shared::minwindef::{ULONG, UCHAR};
    // SystemPowerInformation struct is not expressed in WinNT.h
    // https://docs.microsoft.com/en-us/windows/win32/power/system-power-information-str
    #[repr(C)]
    #[derive(Debug)]
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
            &mut teststruct as *mut _ as *mut _,
            size_of::<SystemPowerInformationStruct>() as u32
        )
    };
    use std::{thread, time};
    use winapi::um::winbase::SetThreadExecutionState;
    use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED, ES_AWAYMODE_REQUIRED};
    let mut occurs = 0;
    let start = time::Instant::now();
    let sleep_time = time::Duration::from_secs(50);
    while occurs < 30 {
        println!("Setting flags and sleeping for {:?} seconds!", sleep_time);
        // Without ES_AWAYMODE_REQUIRED system still slept, necessary.
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED)
        };
        thread::sleep(sleep_time);
        println!("Still alive for {:?}!", start.elapsed());
        println!("{:?}", teststruct);
        occurs += 1;
    }
    println!("Setting system power to default!");
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS)
    };
}