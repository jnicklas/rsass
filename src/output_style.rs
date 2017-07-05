/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputStyle {
    Expanded(u32),
    Compressed,
}

impl OutputStyle {
    pub fn byte_order_mark(self) -> &'static str {
        match self {
            OutputStyle::Compressed => "\u{feff}",
            OutputStyle::Expanded(_) => "@charset \"UTF-8\";\n",
        }
    }

    pub fn is_compressed(self) -> bool {
        match self {
            OutputStyle::Compressed => true,
            OutputStyle::Expanded(_) => false,
        }
    }

    pub fn include_comments(self) -> bool {
        match self {
            OutputStyle::Compressed => false,
            OutputStyle::Expanded(_) => true,
        }
    }

    pub fn include_trailing_semicolon(self) -> bool {
        match self {
            OutputStyle::Compressed => false,
            OutputStyle::Expanded(_) => true,
        }
    }

    pub fn item_separator(self) -> &'static str {
        match self {
            OutputStyle::Compressed => "",
            OutputStyle::Expanded(_) => "\n",
        }
    }

    pub fn property_separator(self) -> &'static str {
        match self {
            OutputStyle::Compressed => "",
            OutputStyle::Expanded(_) => " ",
        }
    }

    pub fn rule_opening_separator(self) -> &'static str {
        match self {
            OutputStyle::Compressed => "",
            OutputStyle::Expanded(_) => "\n",
        }
    }

    pub fn important_separator(self) -> &'static str {
        match self {
            OutputStyle::Compressed => "",
            OutputStyle::Expanded(_) => " ",
        }
    }

    pub fn end_of_file_separator(self) -> &'static str {
        match self {
            OutputStyle::Compressed => "\n",
            OutputStyle::Expanded(_) => "",
        }
    }

    pub fn indent(self) -> Self {
        match self {
            OutputStyle::Compressed => OutputStyle::Compressed,
            OutputStyle::Expanded(level) => OutputStyle::Expanded(level + 1),
        }
    }

    pub fn indentation(self) -> usize {
        match self {
            OutputStyle::Compressed => 0,
            OutputStyle::Expanded(level) => (level * 2) as usize,
        }
    }
}
