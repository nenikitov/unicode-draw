type TuiColor = tui::style::Color;
type DrawColor = draw::style::Color;

pub struct ColorMapping(TuiColor);

impl Into<TuiColor> for ColorMapping {
    fn into(self) -> TuiColor {
        self.0
    }
}

impl From<&DrawColor> for ColorMapping {
    fn from(color: &DrawColor) -> Self {
        ColorMapping(match color {
            DrawColor::None => TuiColor::Reset,
            DrawColor::Black => TuiColor::Black,
            DrawColor::Red => TuiColor::LightRed,
            DrawColor::Green => TuiColor::LightGreen,
            DrawColor::Yellow => TuiColor::LightYellow,
            DrawColor::Blue => TuiColor::LightBlue,
            DrawColor::Magenta => TuiColor::LightMagenta,
            DrawColor::Cyan => TuiColor::LightCyan,
            DrawColor::LightGray => TuiColor::Gray,
            DrawColor::DarkGray => TuiColor::DarkGray,
            DrawColor::DarkRed => TuiColor::Red,
            DrawColor::DarkGreen => TuiColor::Green,
            DrawColor::DarkYellow => TuiColor::Yellow,
            DrawColor::DarkBlue => TuiColor::Blue,
            DrawColor::DarkMagenta => TuiColor::Magenta,
            DrawColor::DarkCyan => TuiColor::Cyan,
            DrawColor::White => TuiColor::White,
            DrawColor::Indexed { i } => TuiColor::Indexed(i.clone()),
            DrawColor::Rgb { r, g, b } => TuiColor::Rgb(r.clone(), g.clone(), b.clone())
        })
    }
}


type TuiModifier = tui::style::Modifier;
type DrawModifiers = draw::style::Modifiers;

pub struct ModifiersMapping(TuiModifier);

impl Into<TuiModifier> for ModifiersMapping {
    fn into(self) -> TuiModifier {
        self.0
    }
}

impl From<&DrawModifiers> for ModifiersMapping {
    fn from(modifier: &DrawModifiers) -> Self {
        let mut ret = TuiModifier::empty();

        if modifier.bold {
            ret |= TuiModifier::BOLD;
        }
        if modifier.italic {
            ret |= TuiModifier::ITALIC;
        }
        if modifier.reverse {
            ret |= TuiModifier::REVERSED;
        }

        ModifiersMapping(ret)
    }
}


type TuiStyle = tui::style::Style;
type DrawStyle = draw::style::Style;

pub struct StyleMapping(TuiStyle);

impl Into<TuiStyle> for StyleMapping {
    fn into(self) -> TuiStyle {
        self.0
    }
}

impl From<&DrawStyle> for StyleMapping {
    fn from(style: &DrawStyle) -> Self {
        StyleMapping(
            TuiStyle::default()
                .fg(ColorMapping::from(&style.fg).into())
                .bg(ColorMapping::from(&style.bg).into())
                .add_modifier(ModifiersMapping::from(&style.modifiers).into())
        )
    }
}


type TuiSpan<'a> = tui::text::Span<'a>;
type DrawCharacter = draw::character::Character;

pub struct CharacterMapping<'a>(TuiSpan<'a>);

impl<'a> Into<TuiSpan<'a>> for CharacterMapping<'a> {
    fn into(self) -> TuiSpan<'a> {
        self.0
    }
}

impl<'a> From<&DrawCharacter> for CharacterMapping<'a> {
    fn from(character: &DrawCharacter) -> Self {
        CharacterMapping(TuiSpan::styled(
            character.character().to_string(),
            StyleMapping::from(character.style()).into()
        ))
    }
}
