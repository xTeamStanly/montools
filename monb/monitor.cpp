#include "monitor.h"

namespace monitor {

	BOOL CALLBACK monitor_enum_proc(HMONITOR h_monitor, HDC hdc_monitor, LPRECT lprc_monitor, LPARAM dw_data) {

		std::vector<PHYSICAL_MONITOR>* all_physical_monitors = (std::vector<PHYSICAL_MONITOR>*)dw_data;

		BOOL result;
		DWORD number_of_monitors;

		result = GetNumberOfPhysicalMonitorsFromHMONITOR(h_monitor, &number_of_monitors);
		if (result == FALSE || number_of_monitors < 0) { return FALSE; }

		PHYSICAL_MONITOR* current_physical_monitors = new(std::nothrow) PHYSICAL_MONITOR[number_of_monitors]; // "new" doesn't throw exceptions
		if (current_physical_monitors == nullptr) { return FALSE; } // memory isn't allocated

		result = GetPhysicalMonitorsFromHMONITOR(h_monitor, number_of_monitors, current_physical_monitors);
		if (result == TRUE) {

			for (unsigned int i = 0; i < number_of_monitors; i++) {
				all_physical_monitors->push_back(current_physical_monitors[i]);
			}

		}

		delete[] current_physical_monitors; // cleanup memory

		return result;

	}

	void parse_args_to_monitor_params(int argc, char** argv, std::map<int, int>& monitor_params) {
		// "argc" and "argv" are always existant & map reference pointer is always not null

		bool result;
		int index;
		int brightness;

		// skip first argv (path)
		for (int i = 1; i < argc; i++) {

			std::string current_arg(argv[i]);
			
			if (current_arg[0] == '/') { // if argument starts with a slash

				current_arg.erase(0, 1); // removes the slash

				std::vector<std::string> current_tokens;
				string_util::split_string(current_arg, ':', current_tokens);

				if (current_tokens.size() < 2) { continue; }

				if (current_tokens[0] == "*") { // argyment "/*:num" means apply "num" brightness to all monitors (except the ones already in the map)

					index = -1;

				} else {

					result = string_util::parse_string_to_integer(current_tokens[0], index);
					if (result == false || index == 0 || index < -1) { continue; } // index must be greater or equal to -1, but not 0

				}

				result = string_util::parse_string_to_integer(current_tokens[1], brightness);
				if (result == false) { continue; }

			} else { // treat the argument as an integer

				result = string_util::parse_string_to_integer(current_arg, brightness);
				if (result == false) { continue; }


				// index -1 means all monitors, except those already in the map
				index = -1;

			}

			// always insert first valid <index, brightness> pair for said index
			if (monitor_params.contains(index) == false) {
				monitor_params[index] = brightness;
			}

		}

	}

	void check_monitor_brightness(PHYSICAL_MONITOR& physical_monitor, int index) {

		HANDLE physical_monitor_handle = physical_monitor.hPhysicalMonitor;

		std::wstring physical_monitor_description = string_util::trim_wchar(physical_monitor.szPhysicalMonitorDescription);
		size_t physical_monitor_description_lenght = physical_monitor_description.length();
		if (physical_monitor_description_lenght == 0) {
			// if description doesn't exist then use monitor index
			physical_monitor_description = std::to_wstring(index);
		}

		DWORD current_brightness, mininmum_brightness, maximum_brightness;
		bool result = GetMonitorBrightness(physical_monitor_handle, &mininmum_brightness, &current_brightness, &maximum_brightness);

		std::wstring output_wstring = L"Monitor (" + physical_monitor_description + L")";
		
		if (result == FALSE) { // monitor doesn't support DDC/CI
			
			output_wstring += L" doesn't support DDC/CI";
			logger::log_warning_wchar(output_wstring.c_str());

		} else {

			std::wstring progress_bar;
			result = string_util::create_progress_bar(progress_bar, mininmum_brightness, current_brightness, maximum_brightness);
			
			if (result == false) {
				
				logger::log_warning_wchar((output_wstring + L" could't render progressbar").c_str());

			} else {
				
				output_wstring += L" - ";
				output_wstring += progress_bar;
				logger::log_information_wchar(output_wstring.c_str());

			}

		}

		DestroyPhysicalMonitor(physical_monitor_handle);

	}

	void set_monitor_brightness(PHYSICAL_MONITOR& physical_monitor, int index, int brightness) {

		HANDLE physical_monitor_handle = physical_monitor.hPhysicalMonitor;

		std::wstring physical_monitor_description = string_util::trim_wchar(physical_monitor.szPhysicalMonitorDescription);
		size_t physical_monitor_description_lenght = physical_monitor_description.length();
		if (physical_monitor_description_lenght == 0) {
			// if description doesn't exist then use monitor index
			physical_monitor_description = std::to_wstring(index);
		}

		DWORD current_brightness, mininmum_brightness, maximum_brightness;
		bool result = GetMonitorBrightness(physical_monitor_handle, &mininmum_brightness, &current_brightness, &maximum_brightness);

		std::wstring output_wstring = L"Monitor (" + physical_monitor_description + L")";

		if (brightness == current_brightness) {
			
			// no need to call winapi to set brightness
			logger::log_information_wchar((output_wstring + L" no need to change brightness").c_str());

		} else if (brightness > maximum_brightness || brightness < mininmum_brightness) {

			// brightness out of range
			logger::log_warning_wchar((output_wstring + L" brightness out of range").c_str());

		} else {

			// set monitor brightness
			result = SetMonitorBrightness(physical_monitor_handle, brightness);
			
			if (result == FALSE) {
				logger::log_error_wchar((output_wstring + L" failed to set brightness").c_str());
			} else {
				logger::log_information_wchar((output_wstring + L" successfully set monitor brightness to " + std::to_wstring(brightness)).c_str());
			}

		}

	}

}