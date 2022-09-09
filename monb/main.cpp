#include <Windows.h>
#include <highlevelmonitorconfigurationapi.h>
#include <physicalmonitorenumerationapi.h>

#pragma comment(lib, "Dxva2.lib")

#include <string>
#include <iostream>
#include <charconv>
#include <stdexcept>
#include <vector>
#include <map>

#include "colors.h"
#include "monitor.h"
#include "logger.h"

int main(int argc, char** argv) {
	
	BOOL result;
	DWORD current_brightness, min_brightness, max_brightness;

	// temporary strings for output
	std::string output_string; 
	std::wstring output_wstring;


	std::vector<PHYSICAL_MONITOR> physical_monitors_vector;
	result = EnumDisplayMonitors(NULL, NULL, monitor::monitor_enum_proc, (LPARAM)&physical_monitors_vector);
	
	if (result == FALSE) {
		logger::log_error("Display enumeration error");
		return 1;
	}

	size_t number_of_physical_monitors = physical_monitors_vector.size();

	if (number_of_physical_monitors <= 0) {
		logger::log_warning("No monitors detected");
		return 1;
	}
	
	output_string = std::to_string(number_of_physical_monitors) + " monitor/s found";
	logger::log(output_string.c_str());

	if (argc < 2) {
		// display current brightness of all monitors w/ monitor name

		for (unsigned int i = 0; i < number_of_physical_monitors; i++) {
			
			PHYSICAL_MONITOR current_physical_monitor = physical_monitors_vector[i];
			monitor::check_monitor_brightness(current_physical_monitor, i);
			
		}

	} else {
		// convert all arguments into a map of <index, brightness> values
		
		std::map<int, int> monitor_params;

		monitor::parse_args_to_monitor_params(argc, argv, monitor_params);

		if (monitor_params.size() == 0) {
			
			logger::log("No valid parameters");

		} else {

			bool set_all_monitors_flag = monitor_params.contains(-1);


			if (set_all_monitors_flag == true) {

				int all_monitors_brightness = monitor_params[-1];

				// set brightness to all the monitors that aren't in the map
				for (unsigned int i = 0; i < number_of_physical_monitors; i++) {

					int monitor_index = i + 1;
					if (monitor_params.contains(monitor_index) == false) {
						monitor::set_monitor_brightness(physical_monitors_vector[i], monitor_index, all_monitors_brightness);
					}

				}

				// remove -1 from map so we aren't trying to access physical_monitors_vector[-2]
				monitor_params.erase(-1);

			}


			// set brightness for all monitors in the map
			for (const auto& [index, brightness] : monitor_params) {

				int vector_index = index - 1;
				if (vector_index >= number_of_physical_monitors || vector_index < 0) { // index out of range
					
					logger::log_warning(("Invalid monitor index (" + std::to_string(index) + ")").c_str());
					continue;

				}

				monitor::set_monitor_brightness(physical_monitors_vector[vector_index], index, brightness);

			}

		}

	}

	return 0;

}