use log::{info, debug};
use crate::cli::params::Duration;

#[cfg(windows)]
use crate::monitor::windows::turn_off;

#[cfg(target_os = "linux")]
use crate::monitor::linux::turn_off;

pub fn execute_duration(duration: Duration) -> Result<(), String> {
    if duration.value == 0 {
        turn_off();
        return Ok(());
    }

    info!("{}", duration);
    std::thread::sleep(duration.try_into()?);

    debug!("Time is up! Waking up...");
    turn_off();

    return Ok(());
}

#[cfg(windows)]
pub mod windows {
    use log::{info, warn, error};
    use windows_sys::Win32::Foundation::{HWND, LRESULT};
    use windows_sys::Win32::System::Console::GetConsoleWindow;
    use windows_sys::Win32::UI::WindowsAndMessaging::{WM_SYSCOMMAND, SC_MONITORPOWER, SendMessageW};

    // const HWND_BROADCAST: HWND = 0xFFFF as HWND;
    const NULL: isize = 0;

    // https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand
    // const MONITOR_POWERING_ON: isize = -1;   // the display is powering on
    // const MONITOR_LOW_POWER: isize = 1;      // the display is going to low power
    const MONITOR_POWER_OFF: isize = 2;         // the display is being shut off

    pub fn turn_off() {
        let console_handle: HWND = unsafe { GetConsoleWindow() };
        if console_handle == NULL {
            error!("Cound't grab window handle");
            return;
        }

        let message_result: LRESULT = unsafe {
            SendMessageW(console_handle, WM_SYSCOMMAND, SC_MONITORPOWER as usize, MONITOR_POWER_OFF)
        };
        if message_result != NULL {
            warn!("Couldn't turn off displays");
            warn!("Displays maybe turned off");
        } else {
            info!("Turning displays off");
        }
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    use log::{info, warn, error};
    use std::process::Command;
    use std::io::ErrorKind::NotFound;

    pub fn turn_off() {
        match Command::new("xset").args(["dpms", "force", "off"]).spawn() {
            Ok(_) => info!("Turning displays off"),

            Err(err) => if let NotFound = err.kind() {
                error!("Command `xset` was not found");
            } else {
                warn!("Couldn't turn off displays");
            }
        }
    }
}
