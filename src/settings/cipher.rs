//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

use crate::keychain::keys::CipherOrders;

pub const DIFFICULTY: usize = 2048;

#[derive(Debug, Serialize, Deserialize)]
pub struct CipherSettings {
    // difficulty for password PBKDF2
    pub difficulty: usize,
    pub cipher_orders: Vec<CipherOrders>,
}

impl CipherSettings {
    pub fn new() -> Self {
        Self {
            difficulty: DIFFICULTY,
            cipher_orders: vec![CipherOrders::NTRUP1277, CipherOrders::AES256],
        }
    }
}
