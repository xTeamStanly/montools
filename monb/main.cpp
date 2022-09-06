#include <Windows.h>
#include <highlevelmonitorconfigurationapi.h>
#include <physicalmonitorenumerationapi.h>
#pragma comment(lib, "Dxva2.lib")

#include <string>
#include <iostream>
#include <charconv>
#include <stdexcept>



int main(int argc, char** argv) {

	// check if argument is correct (brightness validation)

	// todo: multiple monitor support
	if (argc < 2) { return 1; }
		
	int brightness = -1;
	std::string brightness_string(argv[1]);	
	
	int brightness_string_length = brightness_string.length();
	const char* brightness_string_cstring = brightness_string.c_str();
	const char* brightness_string_cstring_nullterminator = brightness_string_cstring + brightness_string_length;

	std::from_chars_result conversion_result = std::from_chars(brightness_string_cstring, brightness_string_cstring + brightness_string_length, brightness);

	if (conversion_result.ec == std::errc::invalid_argument) {
		
		// not a number
		return 2;

	} else if (conversion_result.ec == std::errc::result_out_of_range) {
		
		// number out of range (too big)
		return 3;

	} else if (conversion_result.ptr != brightness_string_cstring_nullterminator) {
		
		// whole string is not a number
		return 4;

	}



	HWND windows_handle = GetDesktopWindow();
	HMONITOR monitor_handle = MonitorFromWindow(windows_handle, MONITOR_DEFAULTTOPRIMARY);

	if (monitor_handle != NULL) {
		DWORD number_of_monitors;

		// get number of physical monitors
		BOOL result_ok = GetNumberOfPhysicalMonitorsFromHMONITOR(monitor_handle, &number_of_monitors);

		if (result_ok) {

			// physical monitors array
			LPPHYSICAL_MONITOR physical_monitors = (LPPHYSICAL_MONITOR)malloc(number_of_monitors * sizeof(LPPHYSICAL_MONITOR));

			if (physical_monitors != NULL) {
				
				// get physical monitor array
				result_ok = GetPhysicalMonitorsFromHMONITOR(monitor_handle, number_of_monitors, physical_monitors);


				if (result_ok) {

					// todo: commandline args per monitor settings, just primary monitor for now

					// primary (first) physical monitor
					HANDLE physical_monitor_handle = physical_monitors[0].hPhysicalMonitor;

					DWORD min_bright, current_bright, max_bright;
					result_ok = GetMonitorBrightness(physical_monitor_handle, &min_bright, &current_bright, &max_bright);

					if (result_ok) {

						if (brightness > max_bright || brightness < min_bright) {
							return 5; // out of monitor range
						}

						if (brightness == current_bright) {
							return 6; // no need to call function
						}

						result_ok = SetMonitorBrightness(physical_monitor_handle, brightness);

						if (result_ok) { return 0; }


					}


				}





			}


			
		}






	}

	//todo: better error handling
	//todo: logger with descriptive errors

	return 7; // idk men

}