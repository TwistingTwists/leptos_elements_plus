

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

impl Alignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Success,
    Warning,
    Error,
    #[default]
    Info,
}

impl Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Type::Success => "success",
            Type::Warning => "warning",
            Type::Error => "error",
            Type::Info => "info",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size{
    #[default]
    Default_,
    Large,
    Small,
}

impl Size{
    pub fn as_str(&self) -> &'static str {
        match self{
            Size::Default_ => "default",
            Size::Large => "large",
            Size::Small => "small",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction{
    #[default]
    Horizontal,
    Vertical,
}

impl Direction{
    pub fn as_str(&self) -> &'static str {
        match self{
            Direction::Horizontal => "horizontal",
            Direction::Vertical => "vertical",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle{
    None,
    Hidden,
    Dotted,
    Dashed,
    #[default]
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl BorderStyle{
    pub fn as_str(&self) -> &'static str {
        match self{
            BorderStyle::None => "none",
            BorderStyle::Hidden => "hidden",
            BorderStyle::Dotted => "dotted",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Solid => "solid",
            BorderStyle::Double => "double",
            BorderStyle::Groove => "groove",
            BorderStyle::Ridge => "ridge",
            BorderStyle::Inset => "inset",
            BorderStyle::Outset => "outset",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentPosition{
    Left,
    Right,
    #[default]
    Center,
}

impl ContentPosition{
    pub fn as_str(&self) -> &'static str {
        match self{
            ContentPosition::Left => "left",
            ContentPosition::Right => "right",
            ContentPosition::Center => "center",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement{
    Top,
    TopStart,
    TopEnd,
    Bottom,
    #[default]
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl Placement{
    pub fn as_str(&self) -> &'static str {
        match self {
            Placement::Top => "top",
            Placement::TopStart => "top-start",
            Placement::TopEnd => "top-end",
            Placement::Bottom => "bottom",
            Placement::BottomStart => "bottom-start",
            Placement::BottomEnd => "bottom-end",
            Placement::Left => "left",
            Placement::LeftStart => "left-start",
            Placement::LeftEnd => "left-end",
            Placement::Right => "right",
            Placement::RightStart => "right-start",
            Placement::RightEnd => "right-end",
        }
    }
}

