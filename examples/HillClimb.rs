use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Grid, Button};

#[cfg(windows)]
mod windows {
    pub(crate) use windows::Win32::{
        Foundation::{CHAR, MAX_PATH},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
    };
}

pub fn get_pid(process_name: &str) -> process_memory::Pid {
    /// A helper function to turn a CHAR array to a String
    fn utf8_to_string(bytes: &[windows::CHAR]) -> String {
        use std::ffi::CStr;
        unsafe {
            CStr::from_ptr(bytes.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned()
        }
    }

    let mut entry = windows::PROCESSENTRY32 {
        dwSize: std::mem::size_of::<windows::PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [windows::CHAR(0); windows::MAX_PATH as usize],
    };
    unsafe {
        // On Error return 0 as the pid. Maybe this function should instead return itself a Result
        // to indicate if a pid has been found?
        let snapshot = if let Ok(snapshot) =
            windows::CreateToolhelp32Snapshot(windows::TH32CS_SNAPPROCESS, 0)
        {
            snapshot
        } else {
            return 0;
        };
        if windows::Process32First(snapshot, &mut entry) == true {
            while windows::Process32Next(snapshot, &mut entry) == true {
                if utf8_to_string(&entry.szExeFile) == process_name {
                    return entry.th32ProcessID;
                }
            }
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    use process_memory::*;
    let process_handle = get_pid("HillClimbRacing.exe").try_into_process_handle()?;

}
