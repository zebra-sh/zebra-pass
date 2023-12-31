//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::scrollable, BorderRadius};

#[derive(Default, Clone)]
pub struct Scrollable {}
impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        scrollable::Scrollbar {
            background: None,
            border_width: 0.0,
            border_color: palette.primary,
            border_radius: BorderRadius::from(palette.radius),
            scroller: scrollable::Scroller {
                color: palette.secondary,
                border_radius: BorderRadius::from(palette.radius),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_hovered: bool) -> scrollable::Scrollbar {
        let active = self.active(style);
        scrollable::Scrollbar { ..active }
    }
}
