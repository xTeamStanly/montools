# montools
Simple CLI Monitor Tools

## shmon - **SH**utdown **MON**itor
Very simple CLI tool for turning off monitors.

### Usage

```./shmon``` - turns off all monitors

```./shmon [DURATION]``` - turns off all monitors after the time _DURATION_ has passed

### Duration
```Format: [POSITIVE INTEGER]([UNIT])?```

Duration dictates how much time should pass before turning off all monitors.
`UNIT` includes `'s'`, `'sec'`, `'m'`, `'min'`, `'h'` representing seconds, minutes and hours, respectively.
If the UNIT is not provided, default unit (seconds) will be used.

### Argument example (shmon)

```./shmon 10 or ./shmon 10s or ./shmon 10sec ``` - will turn all displays off in 10 seconds


## monb - **MON**itor **B**rightness
Very simple CLI tool for controlling monitor brightness.

### Usage

```./monb``` - display brightness value for all monitors (same effect as `./monb *:`)

```./monb --help``` - shows the help message

```./monb [BRIGHTNESS ARGUMENT]... [FLAGS and PROGRESSBAR STYLE]...``` - tries to apply all provided brightness arguments

### FLAGS and PROGRESSBAR STYLE
```-z, --zero``` - Enables zero-based monitor enumeration

```--nocolor``` - Disables colored terminal output

```-v, --verbose``` - Prints debug information during execution

```-l, --length``` - Sets the length of a progressbar, measured in characters

```-s, --style``` - Sets the progressbar style. Possible styles: [ **`wsl`**, **`classic`**, **`arrow`**, **`wsl_arrow`**, **`filled`** ]

### Brightness Arguments
Brightness arguments (`BArg`) can either **set** the brightness value (_setter_) or **get** brightness value (_getter_).
The argument consists of a scope (`BScope`) and a value (`BValue`). All brightness arguments can start with an optional forward slash (**`/`**)

#### _Getter_ arguments
```Format: <BScope>:```
This type of argument is very simple.

It just has a scope and it will return the brightness value of a monitor.
Scope determines which monitors will be selected.
Scope values include unsigned integers (indexed scope) or an asterisk (`*`, global scope).
If a global scope is present then all indexed scopes will be ignored, because they are included
into the global scope.

Example:

```./monb 1:``` - will display brightness value for a monitor with index 1

```./monb 1: 2:``` - will display brightness value for monitors with index 1 and 2

```./monb *:``` - will display brightness value for all monitors

```./monb 1: *:``` - will display brightness value for all monitors

```./monb *: or ./monb /*: ``` - will display brightness value for all monitors


#### _Setter_ arguments
```Format: (<BScope>:)?<BValue>```

This type of argument is a bit more complicated then a _getter_ argument.
It consists of a scope and a value. Values have an action (`BAction`) and a brightness value.
In this type of arguments scope value is optional, assuming global scope if not provided.
Brightness value can be an unsigned integer, `'minimal'`, `'minimum'`, `'min'`, `'maximal'`, `'maximum'`, `'max'` or a ratio.
Ratio is just two unsigned integers separated by a `/` character. The value can also end with `%`, but it is only
used for ratio values. If the percentage sign is not provided the value will be a simple integer division between
two unsigned integers, otherwise it will be treated as a percentage, or simply multiplied by 100.

Setters support incrementing, decrementing or setting a brightness value. This is dictated by brightness action.
Brightness action can be `'+'`, `'-'` or empty (`setter` action). Increment action (`+`) will increment the brightness
by some brightness value and decrement action (`-`) will decrement it. If the action isn't provided it will default to
`setter` action that will set the monitor brightness to desired brightness value.

### Argument example (monb)

```./monb 10 or ./monb *:10 or ./monb /*:10 ``` - will set the brightness for all monitors to 10

```./monb +25 or ./monb *:+25``` - will increase the brightness for all monitors by 25

```./monb +25 1:-50``` - will increase the brightness for all monitors by 25, except the monitor with index 1, because the brightness of that monitor will be decreased by 50

```./monb max``` - will set the brightness for all monitors to 100

```./monb 2400/120``` - will set the brightness for all monitors to 20

```./monb 2/5%``` - will set the brightness for all monitors to 40

```./monb 1:20 2:30``` - will set the brightness for monitor with index 1 to 20 and brightness for monitor with index 2 to 30

```./monb 1:10 2:30 20``` - will set the brightness for monitor with index 1 to 10, monitor with index 2 to 30 and all other monitors brightness to 20

# Info
First version was written for Windows in C++ and is on the `cpp` branch.
The `cpp` branch is deprecated.
On linux, `shmon` is using `xset` command to turn off displays so Wayland support is **currently** non-existent.
