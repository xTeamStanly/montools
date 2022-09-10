#pragma once

#include "colors.h"
#include <iostream>

#define LOG_DEBUG_STRING "[DBG] "
#define LOG_INFORMATION_STRING "[INFO] "
#define LOG_WARNING_STRING "[WARN] "
#define LOG_ERROR_STRING "[ERR] "

namespace library {
    namespace logger {

        typedef enum LOG_SEVERITY {
            LOG_DEBUG = 0,
            LOG_INFORMATION = 1,
            LOG_WARNING = 2,
            LOG_ERROR = 3
        } LOG_SEVERITY;

        void log(const char* message, LOG_SEVERITY log_severity);
        void log(const char* message);
        void log_debug(const char* message);
        void log_information(const char* message);
        void log_warning(const char* message);
        void log_error(const char* message);

        void log_wchar(const wchar_t* message, LOG_SEVERITY log_severity);
        void log_wchar(const wchar_t* message);
        void log_debug_wchar(const wchar_t* message);
        void log_information_wchar(const wchar_t* message);
        void log_warning_wchar(const wchar_t* message);
        void log_error_wchar(const wchar_t* message);

    }
}