pub mod regexes {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub const MAX_NAMES: [&'static str; 3] = ["max", "maximum", "maximal"];
    pub const MIN_NAMES: [&'static str; 3] = ["min", "minimum", "minimal"];

    pub const SCOPE_GROUP: &'static str         = "scope";
    pub const VALUE_GROUP: &'static str         = "value";
    pub const ACTION_GROUP: &'static str        = "action";
    pub const BRIGHTNESS_GROUP: &'static str    = "brightness";
    pub const DENOMINATOR_GROUP: &'static str   = "denominator";
    pub const PERCENTAGE_GROUP: &'static str    = "percentage";
    pub const MIN_GROUP: &'static str           = "min";
    pub const MAX_GROUP: &'static str           = "max";

    const BARG: Lazy<String> = Lazy::new(|| {
        r"(?i)^/?(?:(?P<{{SCOPE_GROUP}}>[0-9]+|\*|all):)?(?P<{{VALUE_GROUP}}>(?P<{{ACTION_GROUP}}>\+|-)?(?:(?:(?P<{{BRIGHTNESS_GROUP}}>[0-9]+)(?:/(?P<{{DENOMINATOR_GROUP}}>[0-9]+))?(?P<{{PERCENTAGE_GROUP}}>%)?)|(?P<{{MIN_GROUP}}>{{MIN}})|(?P<{{MAX_GROUP}}>{{MAX}})))?$"
        /* more readable formatted version:
            (?x) # verbose mode
            (?i) # case insensitive mode
            ^

            /?
            (?:
                (? <{{SCOPE_GROUP}}> [0-9]+|\*|all) :
            )?

            (? <{{VALUE_GROUP}}>
                (? <{{ACTION_GROUP}}> \+|-)?

                (?:
                    (?:
                        (? <{{BRIGHTNESS_GROUP}}> [0-9]+)

                        (?:
                            / (? <{{DENOMINATOR_GROUP}}> [0-9]+)
                        )?

                        (? <{{PERCENTAGE_GROUP}}> %)?
                    )

                    |

                    (? <{{MIN_GROUP}}> {{MIN}})

                    |

                    (? <{{MAX_GROUP}}> {{MAX}})
                )
            )?

            $"
            */
            .replace(r"{{SCOPE_GROUP}}", SCOPE_GROUP)
            .replace(r"{{VALUE_GROUP}}", VALUE_GROUP)
            .replace(r"{{ACTION_GROUP}}", ACTION_GROUP)
            .replace(r"{{BRIGHTNESS_GROUP}}", BRIGHTNESS_GROUP)
            .replace(r"{{DENOMINATOR_GROUP}}", DENOMINATOR_GROUP)
            .replace(r"{{PERCENTAGE_GROUP}}", PERCENTAGE_GROUP)
            .replace(r"{{MIN_GROUP}}", MIN_GROUP)
            .replace(r"{{MAX_GROUP}}", MAX_GROUP)
            .replace(r"{{MIN}}", MIN_NAMES.join("|").as_str())
            .replace(r"{{MAX}}", MAX_NAMES.join("|").as_str())
    });
    pub const BARG_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(&BARG).expect("Invalid `brightness string` regex`") });
}


pub mod params {
    use std::str::FromStr;
    use std::num::IntErrorKind;
    use clap::ArgMatches;
    use const_format::concatcp;
    use once_cell::sync::Lazy;
    use strum::IntoEnumIterator;

    use crate::progressbar::{ProgressBarInfo, ProgressBarType};
    use crate::parser::parse_bargs;


    pub const FLAG_ZERO_ID: &'static str            = "FLAG_ZERO";
    pub const FLAG_ZERO_NAME: &'static str          = "ZERO";
    pub const FLAG_ZERO_SHORT_NAME: char            = 'z';
    pub const FLAG_ZERO_LONG_NAME: &'static str     = "zero";
    pub const FLAG_ZERO_HELP: &'static str          = "Enables zero-based monitor enumeration.";

    pub const FLAG_COLOR_ID: &'static str           = "FLAG_COLOR";
    pub const FLAG_COLOR_NAME: &'static str         = "COLOR";
    pub const FLAG_COLOR_LONG_NAME: &'static str    = "nocolor";
    pub const FLAG_COLOR_HELP: &'static str         = "Disables colored terminal output.";

    pub const FLAG_VERBOSE_ID: &'static str         = "FLAG_VERBOSE";
    pub const FLAG_VERBOSE_NAME: &'static str       = "VERBOSE";
    pub const FLAG_VERBOSE_SHORT_NAME: char         = 'v';
    pub const FLAG_VERBOSE_LONG_NAME: &'static str  = "verbose";
    pub const FLAG_VERBOSE_HELP: &'static str       = "Prints debug information during execution.";

    pub const ARG_BARGS_ID: &'static str            = "ARG_BARGS";
    pub const ARG_BARGS_NAME: &'static str          = "BRIGHTNESS ARGUMENTS";
    pub static ARG_BARGS_HELP: Lazy<String>         = Lazy::new(|| {

        let min_values: String = super::regexes::MIN_NAMES.iter()
            .map(|x| format!("`{}`", x.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        let max_values: String = super::regexes::MAX_NAMES.iter()
            .map(|x| format!("`{}`", x.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        let help_part_1: &'static str = concatcp!(
            "Optional list of brightness arguments.", '\n',

            "Brightness arguments (BArg) can either set the brightness value (setter) or get brightness value (getter).", '\n',
            "The argument consists of a scope (BScope) and a value (BValue). All brightness arguments can start", '\n',
            "with an optional forward slash (/).", '\n', '\n',

            "Getter arguments:", '\n',
                '\t', "Format: <BScope>: This type of argument is very simple.", '\n', '\n',
                '\t', "It just has a scope and it will return the brightness value of a monitor.", '\n',
                '\t', "Scope determines which monitors will be selected. Scope values include unsigned integers (indexed scope) or", '\n',
                '\t', "an asterisk (*, global scope). If a global scope is present then all indexed scopes will be ignored, because", '\n',
                '\t', "they are included into the global scope.", '\n', '\n',
            "Setter arguments:", '\n',
                '\t', "Format: (<BScope>:)?<BValue>", '\n', '\n',
                '\t', "This type of argument is a bit more complicated then a getter argument. It consists", '\n',
                '\t', "of a scope and a value. Values have an action (BAction) and a brightness value.", '\n',
                '\t', "In this type of arguments scope value is optional, assuming global scope if not provided. Brightness", '\n',
        );
        let help_part_2: String = format!("\tvalue can be an unsigned integer, {}, {}\n\tor a ratio. ", min_values, max_values);

        let help_part_3 = concat!(
                "Ratio is just two unsigned integers separated by a / character. The value", '\n',
                '\t', "can also end with %, but it is only used for ratio values. If the percentage", '\n',
                '\t', "sign is not provided the value will be a simple integer division between two", '\n',
                '\t', "unsigned integers, otherwise it will be treated as a percentage, or simply", '\n',
                '\t', "multiplied by 100.", '\n',

                '\t', "Setters support incrementing, decrementing or setting a brightness value. This is dictated", '\n',
                '\t', "by brightness action. Brightness action can be '+', '-' or empty (setter action).", '\n',
                '\t', "Increment action (+) will increment the brightness by some brightness value and", '\n',
                '\t', "decrement action (-) will decrement it. If the action isn't provided it will", '\n',
                '\t', "default to setter action that will set the monitor brightness to desired brightness value."
        );

        return format!("{}{}{}", help_part_1, help_part_2, help_part_3);
    });

    pub const ARG_PROGRESSBAR_LENGTH_ID: &'static str           = "ARG_PROGRESSBAR_LENGTH";
    pub const ARG_PROGRESSBAR_LENGTH_NAME: &'static str         = "PROGRESSBAR LENGTH";
    pub const ARG_PROGRESSBAR_LENGTH_SHORT_NAME: char           = 'l';
    pub const ARG_PROGRESSBAR_LENGTH_LONG_NAME: &'static str    = "length";
    pub const ARG_PROGRESSBAR_LENGTH_HELP: &'static str = concatcp!(
        "Sets the length of a progressbar, measured in characters.", '\n',
        "Minimal value is ", ARG_PROGRESSBAR_LENGTH_MIN, ".\n",
        "Maximal value is ", ARG_PROGRESSBAR_LENGTH_MAX, '.'
    );
    pub const ARG_PROGRESSBAR_LENGTH_DEFAULT: usize = 20;
    pub const ARG_PROGRESSBAR_LENGTH_DEFAULT_STR: &'static str = "20";
    pub const ARG_PROGRESSBAR_LENGTH_MIN: usize = 10;
    pub const ARG_PROGRESSBAR_LENGTH_MAX: usize = 200;

    pub const ARG_PROGRESSBAR_STYLE_ID: &'static str = "ARG_PROGRESSBAR_STYLE";
    pub const ARG_PROGRESSBAR_STYLE_NAME: &'static str = "PROGRESSBAR STYLE";
    pub const ARG_PROGRESSBAR_STYLE_SHORT_NAME: char = 's';
    pub const ARG_PROGRESSBAR_STYLE_LONG_NAME: &'static str = "style";
    pub static ARG_PROGRESSBAR_STYLE_HELP: Lazy<String> = Lazy::new(|| {

        let progress_bar_style_enum_variants: String = ProgressBarType::iter()
            .map(|x| format!("`{}`", x.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        format!("{}\n{} [ {} ]",
            "Sets the progressbar style.",
            "Possible styles:",
            progress_bar_style_enum_variants
        )
    });

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum BScope {
        Global,
        Index(usize)
    }

    impl ToString for BScope {
        fn to_string(&self) -> String {
            match *self {
                Self::Global => "*".to_string(),
                Self::Index(i) => i.to_string()
            }
        }
    }

    impl FromStr for BScope {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().trim() {
                "*" | "all" => Ok(Self::Global),
                potential_number => match potential_number.parse::<usize>() {
                    Ok(number) => Ok(BScope::Index(number)),
                    Err(err) => match err.kind() {
                        IntErrorKind::PosOverflow => Err(format!("Index `{}` is too big", potential_number)),
                        _ => Err(format!("Index `{}` is not a number", potential_number))
                    }
                }
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum BAction {
        Set,
        Get,
        Inc,
        Dec
    }

    impl Into<&'static str> for &BAction {
        fn into(self) -> &'static str {
            match self {
                BAction::Dec => "-",
                BAction::Inc => "+",
                _ => ""
            }
        }
    }

    impl FromStr for BAction {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().trim() {
                "+" => Ok(Self::Inc),
                "-" => Ok(Self::Dec),
                _ => Ok(Self::Set)
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct BValue {
        pub action: BAction,
        pub brightness: Option<usize>
    }

    #[derive(Debug, PartialEq)]
    pub struct BArg {
        pub scope: BScope,
        pub value: BValue
    }

    impl ToString for BArg {
        fn to_string(&self) -> String {
            let prefix: &'static str = (&self.value.action).into();
            let scope: String = self.scope.to_string();
            return format!("{}:{}{}",
                scope,
                prefix,
                match self.value.brightness {
                    Some(v) => v.to_string(),
                    None => "".into()
                }
            );
        }
    }

    impl Default for BArg {
        fn default() -> Self {
            Self {
                scope: BScope::Global,
                value: BValue {
                    action: BAction::Get,
                    brightness: None
                }
            }
        }
    }

    #[derive(Debug)]
    pub enum Getter {
        Global,
        Many(Vec<BScope>)
    }

    #[derive(Debug)]
    pub struct BArgs {
        pub getters: Option<Getter>,
        pub setters: Vec<BArg>
    }
    impl Default for BArgs {
        fn default() -> Self {
            Self {
                getters: Some(Getter::Global),
                setters: vec![]
            }
        }
    }

    #[derive(Debug)]
    pub struct Arguments {
        pub flag_zero: bool,
        pub progressbar_info: ProgressBarInfo,
        pub bargs: BArgs
    }

    impl TryFrom<&ArgMatches> for Arguments {
        type Error = String;

        fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
            Ok(Arguments {
                flag_zero: value.get_flag(FLAG_ZERO_ID),
                progressbar_info: ProgressBarInfo::try_from(value)?,
                bargs: parse_bargs(value.get_many::<String>(ARG_BARGS_ID))?
            })
        }
    }

}
