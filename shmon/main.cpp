#include <Windows.h>
#include "logger.h"

// put all monitors to sleep (power saving mode)
int main(int argc, char** argv) {

	HWND current_console_window_handle = GetConsoleWindow(); // grab console window handle
	if (current_console_window_handle == NULL) { // 
		library::logger::log_error("Cound't grab window handle");
		return 1;
	}

	LRESULT messageResult = SendMessageW(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, 0x0002 /* POWER OFF */);
	
	// unsuccessfull message processing
	if (messageResult == NULL) {
		library::logger::log_warning("Couldn't turn off displays");
		library::logger::log_warning("Displays maybe turned off");
		return 1;
	}

	library::logger::log("Turning displays off");
	return 0;
}