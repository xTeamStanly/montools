#include "logger.h"

namespace library {
	namespace logger {

	// char string

		void log(const char* message, LOG_SEVERITY log_severity) {

			if (message == nullptr) {

				std::cout << ASCII_COLOR_RED << LOG_ERROR_STRING << ASCII_COLOR_RESET << "Invalid Message" << std::endl;

			} else {

				switch (log_severity) {
					case LOG_SEVERITY::LOG_DEBUG:
						std::cout << ASCII_COLOR_CYAN << LOG_DEBUG_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

					case LOG_SEVERITY::LOG_INFORMATION:
						std::cout << ASCII_COLOR_GREEN << LOG_INFORMATION_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

					case LOG_SEVERITY::LOG_WARNING:
						std::cout << ASCII_COLOR_YELLOW << LOG_WARNING_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

					case LOG_SEVERITY::LOG_ERROR:
						std::cout << ASCII_COLOR_RED << LOG_ERROR_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

				}
			}
		}

		void log(const char* message) {
			log(message, LOG_SEVERITY::LOG_INFORMATION);
		}

		void log_debug(const char* message) {
			log(message, LOG_SEVERITY::LOG_DEBUG);
		}

		void log_information(const char* message) {
			log(message, LOG_SEVERITY::LOG_INFORMATION);
		}

		void log_warning(const char* message) {
			log(message, LOG_SEVERITY::LOG_WARNING);
		}

		void log_error(const char* message) {
			log(message, LOG_SEVERITY::LOG_ERROR);
		}


		// wchar string (wstring)

		void log_wchar(const wchar_t* message, LOG_SEVERITY log_severity) {

			if (message == nullptr) {

				std::wcout << ASCII_COLOR_RED << LOG_ERROR_STRING << ASCII_COLOR_RESET << "Invalid Message" << std::endl;

			} else {

				switch (log_severity) {
					case LOG_SEVERITY::LOG_DEBUG:
						std::wcout << ASCII_COLOR_CYAN << LOG_DEBUG_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

					case LOG_SEVERITY::LOG_INFORMATION:
						std::wcout << ASCII_COLOR_GREEN << LOG_INFORMATION_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

					case LOG_SEVERITY::LOG_WARNING:
						std::wcout << ASCII_COLOR_YELLOW << LOG_WARNING_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

					case LOG_SEVERITY::LOG_ERROR:
						std::wcout << ASCII_COLOR_RED << LOG_ERROR_STRING << ASCII_COLOR_RESET << message << std::endl;
						break;

				}
			}
		}

		void log_wchar(const wchar_t* message) {
			log_wchar(message, LOG_SEVERITY::LOG_INFORMATION);
		}

		void log_debug_wchar(const wchar_t* message) {
			log_wchar(message, LOG_SEVERITY::LOG_DEBUG);
		}

		void log_information_wchar(const wchar_t* message) {
			log_wchar(message, LOG_SEVERITY::LOG_INFORMATION);
		}

		void log_warning_wchar(const wchar_t* message) {
			log_wchar(message, LOG_SEVERITY::LOG_WARNING);
		}

		void log_error_wchar(const wchar_t* message) {
			log_wchar(message, LOG_SEVERITY::LOG_ERROR);
		}

	}
}