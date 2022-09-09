#pragma once

// https://stackoverflow.com/a/9158263
#define ASCII_COLOR_RESET       "\033[0m"
#define ASCII_COLOR_BLACK       "\033[30m"              /* Black */
#define ASCII_COLOR_RED         "\033[31m"              /* Red */
#define ASCII_COLOR_GREEN       "\033[32m"              /* Green */
#define ASCII_COLOR_YELLOW      "\033[33m"              /* Yellow */
#define ASCII_COLOR_BLUE        "\033[34m"              /* Blue */
#define ASCII_COLOR_MAGENTA     "\033[35m"              /* Magenta */
#define ASCII_COLOR_CYAN        "\033[36m"              /* Cyan */
#define ASCII_COLOR_WHITE       "\033[37m"              /* White */
#define ASCII_COLOR_BOLDBLACK   "\033[1m\033[30m"       /* Bold Black */
#define ASCII_COLOR_BOLDRED     "\033[1m\033[31m"       /* Bold Red */
#define ASCII_COLOR_BOLDGREEN   "\033[1m\033[32m"       /* Bold Green */
#define ASCII_COLOR_BOLDYELLOW  "\033[1m\033[33m"       /* Bold Yellow */
#define ASCII_COLOR_BOLDBLUE    "\033[1m\033[34m"       /* Bold Blue */
#define ASCII_COLOR_BOLDMAGENTA "\033[1m\033[35m"       /* Bold Magenta */
#define ASCII_COLOR_BOLDCYAN    "\033[1m\033[36m"       /* Bold Cyan */
#define ASCII_COLOR_BOLDWHITE   "\033[1m\033[37m"       /* Bold White */

namespace colors {
    
    // https://stackoverflow.com/a/49929936

    /* Enum to store Foreground colors */
    typedef enum FG_COLORS {
        FG_BLACK = 0,
        FG_BLUE = 1,
        FG_GREEN = 2,
        FG_CYAN = 3,
        FG_RED = 4,
        FG_MAGENTA = 5,
        FG_BROWN = 6,
        FG_LIGHTGRAY = 7,
        FG_GRAY = 8,
        FG_LIGHTBLUE = 9,
        FG_LIGHTGREEN = 10,
        FG_LIGHTCYAN = 11,
        FG_LIGHTRED = 12,
        FG_LIGHTMAGENTA = 13,
        FG_YELLOW = 14,
        FG_WHITE = 15
    } FG_COLORS;

    /* Enum to store Background colors */
    typedef enum BG_COLORS {
        BG_NAVYBLUE = 16,
        BG_GREEN = 32,
        BG_TEAL = 48,
        BG_MAROON = 64,
        BG_PURPLE = 80,
        BG_OLIVE = 96,
        BG_SILVER = 112,
        BG_GRAY = 128,
        BG_BLUE = 144,
        BG_LIME = 160,
        BG_CYAN = 176,
        BG_RED = 192,
        BG_MAGENTA = 208,
        BG_YELLOW = 224,
        BG_WHITE = 240
    } BG_COLORS;

}