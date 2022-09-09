#pragma once

#include <vector>
#include <string>
#include <charconv>
#include <stdexcept>
#include <algorithm>

namespace string_util {

	void split_string(std::string input, char delimeter, std::vector<std::string>& output, bool trim_whitespace = true);
	std::string trim_string(std::string, char delimeter = ' ');
	std::wstring trim_wstring(std::wstring, char delimeter = ' ');
	std::wstring trim_wchar(const wchar_t* input, char delimeter = ' ');
	bool parse_string_to_integer(std::string& input, int& output);
	bool create_progress_bar(std::wstring& progress_bar, unsigned int minimum, unsigned int current, unsigned int maximum);

}