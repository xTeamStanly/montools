pub mod regexes {
    use once_cell::sync::Lazy;
    use regex::Regex;

    const DURATION_RAW: &'static str = "^([0-9]+)(s|sec|m|min|h)?$";
    pub const DURATION_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(&DURATION_RAW).expect("Invalid `duration argument` regex") });
}

pub mod flags {
    pub const COLOR: &'static str = "color_flag";
    pub const VERBOSE: &'static str = "verbose_flag";

    #[derive(Debug)]
    pub struct Flags {
        pub support_color: bool,
        pub verbose: bool
    }
}

pub mod arguments {
    use std::str::FromStr;
    use std::fmt::Display;

    pub const DURATION_ARGUMENT: &'static str = "duration_argument";

    #[derive(Debug, Clone)]
    pub enum Unit {
        Seconds,
        Minutes,
        Hours
    }

    impl Default for Unit {
        fn default() -> Self {
            Self::Seconds
        }
    }

    impl Into<&'static str> for Unit {
        fn into(self) -> &'static str {
            match self {
                Self::Seconds => "second(s)",
                Self::Minutes => "minute(s)",
                Self::Hours => "hour(s)"
            }
        }
    }

    impl FromStr for Unit {

        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().trim() {
                "s" | "sec" => Ok(Self::Seconds),
                "m" | "min" => Ok(Self::Minutes),
                "h" => Ok(Self::Hours),
                _ => Err("Invalid duration unit")

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
                Unit::Seconds => self.value,
                Unit::Minutes => self.value * 60,
                Unit::Hours => self.value * 60 * 60,
            } as f64;

            std::time::Duration::try_from_secs_f64(duration_value).map_err(|_| format!("Invalid duration `{}`", self.value))
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
            Self { value: 0, unit: None }
        }
    }
}
