#include <Windows.h>
#include <highlevelmonitorconfigurationapi.h>
#include <physicalmonitorenumerationapi.h>
#pragma comment(lib, "Dxva2.lib")

#include <string>
#include <iostream>
#include <charconv>
#include <stdexcept>
#include <sstream>

#include "colors.h"

#include <vector>

BOOL CALLBACK MonitorEnumProc(HMONITOR hMonitor, HDC hdcMonitor, LPRECT lprcMonitor, LPARAM dwData) {

	std::vector<PHYSICAL_MONITOR>* physical_monitors_vector_ptr = (std::vector<PHYSICAL_MONITOR>*)dwData;

	BOOL result;

	DWORD number_of_monitors;
	result = GetNumberOfPhysicalMonitorsFromHMONITOR(hMonitor, &number_of_monitors);
	if (result == FALSE || number_of_monitors < 0) { return FALSE; }

	PHYSICAL_MONITOR* physical_monitor_array = new PHYSICAL_MONITOR[number_of_monitors];

	result = GetPhysicalMonitorsFromHMONITOR(hMonitor, number_of_monitors, physical_monitor_array);
	if (result == FALSE) {

		if (physical_monitor_array != nullptr) {
			delete[] physical_monitor_array; // cleanup memory
		}

		return FALSE;
	}

	for (unsigned int i = 0; i < number_of_monitors; i++) {
		physical_monitors_vector_ptr->push_back(physical_monitor_array[i]);
	}

	if (physical_monitor_array != nullptr) {
		delete[] physical_monitor_array; // cleanup memory
	}

	return TRUE;
}



int main(int argc, char** argv) {
	
	BOOL result;
	DWORD current_brightness, min_brightness, max_brightness;
	
	// console handle for coloring the output
	HANDLE console_handle = GetStdHandle(STD_OUTPUT_HANDLE);
	if(console_handle == NULL) { // system error?
		return 1;
	}

	std::vector<PHYSICAL_MONITOR> physical_monitors_vector;
	EnumDisplayMonitors(NULL, NULL, MonitorEnumProc, (LPARAM)&physical_monitors_vector);
	if (physical_monitors_vector.size() == 0) { // no monitors connected?
		return 1;
	}
	

	if (argc < 2) {
		// display current brightness of all monitors w/ monitor name

		// yellow console output
		SetConsoleTextAttribute(console_handle, colors::FG_YELLOW);

		for (PHYSICAL_MONITOR physical_monitor : physical_monitors_vector) {

			HANDLE current_physical_monitor_handle = physical_monitor.hPhysicalMonitor;
			
			std::wstring current_physical_monitor_description(physical_monitor.szPhysicalMonitorDescription);
			std::size_t current_physical_monitor_description_lenght = current_physical_monitor_description.length();
						
			result = GetMonitorBrightness(current_physical_monitor_handle, &min_brightness, &current_brightness, &max_brightness);
			if (result != FALSE) { // monitor supports DDC/CI, otherwise just skip it

				std::string progress_bar;

				unsigned int range = max_brightness - min_brightness;
				if (range > 0) { // only if range is positive and non zero, otherwise just skip it
					
					unsigned int percentage = ((current_brightness - min_brightness) * 100) / range;
					
					progress_bar += "[";

					for (unsigned int i = 0; i < 100; i += 5) {
						
						if (i < percentage) {
							progress_bar += "=";
						} else {
							progress_bar += " ";
						}

					}
					progress_bar += "] ";
					progress_bar += std::to_string(current_brightness);
					progress_bar += "%";

				}

				std::wcout << current_physical_monitor_description << std::endl;
				std::cout << progress_bar << std::endl;
				
			}

			DestroyPhysicalMonitor(current_physical_monitor_handle);
		}

		// white console output
		SetConsoleTextAttribute(console_handle, colors::FG_LIGHTGRAY);

	} else {
		// parse sent argument as integer and set correct brightness value

		// check if argument is correct (brightness validation)
		
		int brightness = -1;
		std::string brightness_string(argv[1]);
		int brightness_string_length = brightness_string.length();
		const char* brightness_string_cstring = brightness_string.c_str();
		const char* brightness_string_cstring_nullterminator = brightness_string_cstring + brightness_string_length;

		std::from_chars_result conversion_result = std::from_chars(brightness_string_cstring, brightness_string_cstring + brightness_string_length, brightness);

		if (conversion_result.ec == std::errc::invalid_argument) {
			
			// not a number
			return 1;

		} else if (conversion_result.ec == std::errc::result_out_of_range) {
			
			// number not in range (too big/small)
			return 1;

		} else if (conversion_result.ptr != brightness_string_cstring_nullterminator) {

			// whole string is not a number
			return 1;

		}

		for (PHYSICAL_MONITOR physical_monitor : physical_monitors_vector) {
			
			HANDLE current_physical_monitor_handle = physical_monitor.hPhysicalMonitor;

			result = GetMonitorBrightness(current_physical_monitor_handle, &min_brightness, &current_brightness, &max_brightness);
			if (result != FALSE) { // monitor supports DDC/CI, otherwise just skip it
				
				if (brightness == (int)current_brightness) {
					// no need to call WinAPI
					continue;
				}


				if (brightness > (int)max_brightness || brightness < (int)min_brightness) {
					// brightness out of range
					continue;
				}

				result = SetMonitorBrightness(current_physical_monitor_handle, brightness);
				if (result == FALSE) { // shouldn't happen


				}

			}
		}
	}

	return 0;

	//todo: better error handling
	//todo: logger with descriptive errors
	// std::cout << GetLastError() << std::endl;

}