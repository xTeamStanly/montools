#pragma once

#include <Windows.h>
#include <highlevelmonitorconfigurationapi.h>
#include <vector>

#include "monitor.h"
#include "string_util.h"
#include "logger.h"

#include <string>
#include <charconv>
#include <stdexcept>
#include <map>

namespace monitor {

	struct monitor_param {
		int index;				// index -1 means all monitors
		int brightness;
	};

	BOOL CALLBACK monitor_enum_proc(HMONITOR h_monitor, HDC hdc_monitor, LPRECT lprc_monitor, LPARAM dw_data);
	void parse_args_to_monitor_params(int argc, char** argv, std::map<int, int>& monitor_params);
	void check_monitor_brightness(PHYSICAL_MONITOR& physical_monitor, int index);
	void set_monitor_brightness(PHYSICAL_MONITOR& physical_monitor, int index, int brightness);

}