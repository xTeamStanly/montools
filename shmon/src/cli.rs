pub mod regexes {
    use once_cell::sync::Lazy;
    use regex::Regex;

    const DURATION_RAW: &'static str = "^([0-9]+)(s|sec|m|min|h)?$";
    pub const DURATION_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(&DURATION_RAW)
        .expect("Invalid `duration argument` regex")
    });
}

pub mod params {
    use std::str::FromStr;
    use std::fmt::Display;

    pub const FLAG_COLOR_ID: &'static str           = "FLAG_COLOR";
    pub const FLAG_COLOR_NAME: &'static str         = "COLOR";
    pub const FLAG_COLOR_LONG_NAME: &'static str    = "nocolor";
    pub const FLAG_COLOR_HELP: &'static str         = "Disables colored terminal output.";

    pub const FLAG_VERBOSE_ID: &'static str         = "FLAG_VERBOSE";
    pub const FLAG_VERBOSE_NAME: &'static str       = "VERBOSE";
    pub const FLAG_VERBOSE_SHORT_NAME: char         = 'v';
    pub const FLAG_VERBOSE_LONG_NAME: &'static str  = "verbose";
    pub const FLAG_VERBOSE_HELP: &'static str       = "Prints debug information during execution.";

    pub const ARG_DURATION_ID: &'static str         = "ARG_DURATION";
    pub const ARG_DURATION_NAME: &'static str       = "DURATION";
    pub const ARG_DURATION_HELP: &'static str       = concat!(
        "Time duration before the displays are turned off.", '\n',
        "Format: `[POSITIVE INTEGER][UNIT]`.", '\n',
        "UNIT includes `s`, `sec`, `m`, `min`, `h` representing seconds, minutes and hours, respectively.", '\n',
        "If the UNIT is not provided, default unit (seconds) will be used.", '\n',
        "Default value (if not set) is 2 seconds."
    );

    #[derive(Debug, Clone, Default)]
    pub enum Unit {
        #[default]
        Seconds,

        Minutes,
        Hours
    }

    impl Into<&'static str> for Unit {
        fn into(self) -> &'static str {
            match self {
                Self::Seconds   => "second(s)",
                Self::Minutes   => "minute(s)",
                Self::Hours     => "hour(s)"
            }
        }
    }

    impl FromStr for Unit {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().trim() {
                "s" | "sec" => Ok(Self::Seconds),
                "m" | "min" => Ok(Self::Minutes),
                "h" => Ok(Self::Hours),
                _ => Err(format!("Invalid duration unit: `{}`", s))
            }
        }
    }

    #[derive(Debug)]
    pub struct Duration {
        pub value: usize,
        pub unit: Option<Unit>
    }

    impl TryInto<std::time::Duration> for Duration {

        type Error = String;

        fn try_into(self) -> Result<std::time::Duration, Self::Error> {
            let duration_value: f64 = match self.unit.unwrap_or_default() {
                Unit::Seconds   => self.value,
                Unit::Minutes   => self.value * 60,
                Unit::Hours     => self.value * 60 * 60,
            } as f64;

            std::time::Duration::try_from_secs_f64(duration_value)
                .map_err(|_| format!("Invalid duration `{}`", self.value))
        }
    }

    impl Display for Duration {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let unit_str: &'static str = self.unit.clone().unwrap_or_default().into();
            write!(f, "Sleeping starts in {} {}", self.value, unit_str)
        }
    }

    impl Default for Duration {
        fn default() -> Self {
            Self { value: 2, unit: Some(Unit::Seconds) }
        }
    }
}
