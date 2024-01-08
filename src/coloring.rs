use colored::{ColoredString, Colorize};

pub(crate) fn with_gray(s: &str) -> ColoredString {
    s.truecolor(169, 169, 169)
}
