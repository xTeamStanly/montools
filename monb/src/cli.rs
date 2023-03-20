pub mod regexes {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub const MAX_NAMES: [&'static str; 3] = ["max", "maximum", "maximal"];
    pub const MIN_NAMES: [&'static str; 3] = ["min", "minimum", "minimal"];

    const BVALUE_RAW: Lazy<String> = Lazy::new(|| {
        r"((\+|-)?(([0-9]+)(/([0-9]+))?(%)?|{{MIN}}|{{MAX}}))"
            .replace(r"{{MIN}}", MIN_NAMES.join("|").as_str())
            .replace(r"{{MAX}}", MAX_NAMES.join("|").as_str())
    });
    const BVALUE: Lazy<String> = Lazy::new(|| { r"^{{BVALUE}}$".replace(r"{{BVALUE}}", BVALUE_RAW.as_str()) });
    pub const BVALUE_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(&BVALUE).expect("Invalid `brightness argument` regex") });

    const BARG: Lazy<String> = Lazy::new(|| { r"^(/?([0-9]+|\*|all):)?{{BVALUE}}$".replace(r"{{BVALUE}}", &BVALUE_RAW) });
    pub const BARG_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(&BARG).expect("Invalid `brightness string` regex`") });
}

pub mod flags {
    pub const ZERO: &'static str = "zero_flag";
    pub const COLOR: &'static str = "color_flag";
    pub const VERBOSE: &'static str = "verbose_flag";

    #[derive(Debug)]
    pub struct Flags {
        pub zero: bool,
        pub color: bool,
        pub verbose: bool
    }
}

pub mod arguments {
    pub const BRIGHTNESS_ARGUMENTS: &'static str = "brightness_arguments";
    pub const BRIGHTNESS_VALUE: &'static str = "brightness_value";
    pub const MONITOR_INDICES: &'static str = "monitor_indices";

    pub const PROGRESSBAR_LENGTH: &'static str = "progressbar_length";
    pub const DEFAULT_PROGRESSBAR_LENGTH: usize = 20;
    pub const DEFAULT_PROGRESSBAR_LENGTH_STR: &'static str = "20";
    pub const MAX_PROGRESSBAR_LENGTH: usize = 200;
    pub const MIN_PROGRESSBAR_LENGTH: usize = 10;

    pub const PROGRESSBAR_STYLE: &'static str = "progressbar_style";
    pub const DEFAULT_PROGRESSBAR_STYLE: &'static str = "wsl_arrow";


    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum BScope {
        All,
        Index(usize)
    }

    #[derive(Debug, PartialEq)]
    pub enum BAction {
        Set,
        Inc,
        Dec
    }

    #[derive(Debug, PartialEq)]
    pub struct BValue {
        pub action: BAction,
        pub brightness: usize
    }

    #[derive(Debug, PartialEq)]
    pub struct BArg {
        pub scope: BScope,
        pub value: BValue
    }

}
