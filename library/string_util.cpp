#include "string_util.h"

#define PROGRESS_BAR_START_VALUE 0
#define PROGRESS_BAR_END_VALUE 20

namespace library {
	namespace string_util {

	// https://chromium.googlesource.com/chromium/src/base/+/refs/heads/main/strings/string_split_internal.h
		void split_string(std::string input, char delimeter, std::vector<std::string>& output, bool trim_whitespace) {

			if (input.empty() == true) { return; }

			size_t start = 0;
			std::string token;

			while (start != std::string::npos) {

				size_t end = input.find_first_of(delimeter, start);
				if (end == std::string::npos) {
					token = input.substr(start);
					start = std::string::npos;
				} else {
					token = input.substr(start, end - start);
					start = end + 1;
				}

				if (trim_whitespace == true) { token = trim_string(token); }
				if (token.empty() == false) { output.push_back(token); }


			}

		}

		// https://chromium.googlesource.com/chromium/src/base/+/refs/heads/main/strings/string_util_internal.h
		std::string trim_string(std::string input, char delimeter) {

			const size_t last_char = input.length() - 1;
			const char first_good_char = input.find_first_not_of(delimeter);
			const char last_good_char = input.find_last_not_of(delimeter);

			if (input.empty() == true || first_good_char == std::string::npos || last_good_char == std::string::npos) { return ""; }

			return input.assign(input.data() + first_good_char, last_good_char - first_good_char + 1);

		}

		std::wstring trim_wstring(std::wstring input, char delimeter) {

			const size_t last_char = input.length() - 1;
			const char first_good_char = input.find_first_not_of(delimeter);
			const char last_good_char = input.find_last_not_of(delimeter);

			if (input.empty() == true || first_good_char == std::wstring::npos || last_good_char == std::wstring::npos) { return std::wstring(); }

			return input.assign(input.data() + first_good_char, last_good_char - first_good_char + 1);

		}

		std::wstring trim_wchar(const wchar_t* input, char delimeter) {
			return trim_wstring(std::wstring(input), delimeter);
		}

		bool parse_string_to_integer(std::string& input, int& output) {

			size_t input_length = input.length();
			if (input_length == 0) { return false; }

			const char* input_cstring = input.c_str();
			const char* input_cstring_nullterminator = input_cstring + input_length;

			std::from_chars_result conversion_result = std::from_chars(input_cstring, input_cstring + input_length, output);

			if (
				conversion_result.ec == std::errc::invalid_argument ||		// not a number
				conversion_result.ec == std::errc::result_out_of_range ||	// number not in range (too big/small)
				conversion_result.ptr != input_cstring_nullterminator		// whole string is not a number
				) {
				return false;
			}

			return true;

		}

		// (p5.prototype.map) https://github.com/processing/p5.js/blob/v1.4.2/src/math/calculation.js
		float p5_map(float n, float start1, float stop1, float start2, float stop2) {
			return (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
		}

		// this function will rearrange "minimum", "current" and "maximum" if the values are not in a logical order
		bool create_progress_bar(std::wstring& progress_bar, unsigned int minimum, unsigned int current, unsigned int maximum) {

			std::vector<unsigned int> values { minimum , current, maximum };

			std::sort(values.begin(), values.end());

			minimum = values[0];
			current = values[1];
			maximum = values[2];

			unsigned int range = maximum - minimum;
			if (range == 0) { return false; } // fail only if range is zero (division by zero)

			unsigned int percentage;
			progress_bar.clear(); // empty the progress bar wstring

			progress_bar += L"[";

			// map values from 0 to PROGRESS_BAR_END_VALUE
			float current_float = p5_map(current, minimum, maximum, PROGRESS_BAR_START_VALUE, PROGRESS_BAR_END_VALUE);

			if (minimum == 0 && maximum == 100) {

				// do not remap current brightness
				percentage = current;

			} else {

				// map values from 0 to 100%
				percentage = p5_map(current_float, PROGRESS_BAR_START_VALUE, PROGRESS_BAR_END_VALUE, 0, 100);

			}

			current = (int)current_float;

			for (unsigned int i = PROGRESS_BAR_START_VALUE; i < PROGRESS_BAR_END_VALUE; i++) {
				if (i < current) {
					progress_bar += L"=";
				} else {
					progress_bar += L" ";
				}
			}

			progress_bar += L"] ";
			progress_bar += std::to_wstring(percentage);
			progress_bar += L"%";

			return true;

		};

	}
}