//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate rust_i18n;

use rust_i18n::i18n;
use zebra_ui::theme;

i18n!("locales", fallback = "en");

fn main() {
    println!("Hello, world!");
}
