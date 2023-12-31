//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

pub mod color;
pub mod components;
pub mod image;
pub mod style;

pub mod widget {
    #![allow(dead_code)]
    use crate::style::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
    pub type Text<'a> = iced::widget::Text<'a, Renderer>;
    pub type Tooltip<'a> = iced::widget::Tooltip<'a, Renderer>;
    pub type ProgressBar = iced::widget::ProgressBar<Renderer>;
    pub type PickList<'a, Message> = iced::widget::PickList<'a, Message, Renderer>;
    pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Renderer>;
    pub type Svg = iced::widget::Svg<Renderer>;
}
