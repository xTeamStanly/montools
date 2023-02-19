use logger::{self};

#[cfg(windows)]
use windows_sys::Win32::{Foundation::{HWND, LRESULT}, UI::WindowsAndMessaging::{WM_SYSCOMMAND, SC_MONITORPOWER, SendMessageW}, System::Console::GetConsoleWindow};


// const HWND_BROADCAST: HWND = 0xFFFF as HWND;
#[cfg(windows)]
const NULL: isize = 0;

// https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand
// const MONITOR_POWERING_ON: isize = -1;      // the display is powering on
// const MONITOR_LOW_POWER: isize = 1;         // the display is going to low power
#[cfg(windows)]
const MONITOR_POWER_OFF: isize = 2;         // the display is being shut off

fn main() {
    #[cfg(windows)]
    {
        let console_handle: HWND = unsafe { GetConsoleWindow() };
        if console_handle == NULL {
            logger::log_error("Cound't grab window handle");
            return;
        }

        let message_result: LRESULT = unsafe { SendMessageW(console_handle, WM_SYSCOMMAND, SC_MONITORPOWER as usize, MONITOR_POWER_OFF) };
        if message_result != NULL {
            logger::log_warn("Couldn't turn off displays");
	    	logger::log_warn("Displays maybe turned off");
            return;
        }

        logger::log_info("Turning displays off");
    }

    #[cfg(target_os = "linux")]
    {
        match std::process::Command::new("xset").args(["dpms", "force", "off"]).spawn() {
            Ok(_) => {
                logger::log_info("Turning displays off");
            }
            Err(err) => {
                if let std::io::ErrorKind::NotFound = err.kind() {
                    logger::log_error("Command `xset` was not found");
                } else {
                    logger::log_warn("Couldn't turn off displays");
                }
            }
        }
    }
}
