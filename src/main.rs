// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use rand;
use slint::{Model, SharedString, VecModel};
use zebra_pass::{
    bip39::mnemonic::{Language, Mnemonic},
    core::{
        bip39::{self, from_bip39_model},
        core::Core,
        passgen::PassGen,
    },
    errors::ZebraErrors,
};

slint::include_modules!();

fn handler(core: Rc<RefCell<Core>>) -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let core_ref_state = core.clone();
    let state = &core_ref_state.borrow().state.clone();
    let app = AppWindow::new()?;
    let main_window = Rc::new(app.as_weak().unwrap());

    if !state.borrow().payload.inited {
        main_window.set_route(Routers::LangChoose);
    } else {
        main_window.set_route(Routers::Lock);
    }

    main_window
        .global::<KeyChainLogic>()
        .on_request_random_words(|length_str| {
            let mut rng = rand::thread_rng();
            let count: usize = length_str.to_string().parse().unwrap_or(12);
            // TODO: make a error hanlder.
            let m = Mnemonic::gen(&mut rng, count, Language::English).unwrap();
            bip39::gen_bip39_words(&m, 3)
        });

    let keys_logic_ref = main_window.clone();
    let core_ref = core.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_create_account(move || {
            let mut core = core_ref.borrow_mut();
            let sync = keys_logic_ref.global::<KeyChainLogic>().get_sync();
            let email = keys_logic_ref.global::<KeyChainLogic>().get_email();
            let password = keys_logic_ref.global::<KeyChainLogic>().get_password();
            let words_salt = keys_logic_ref.global::<KeyChainLogic>().get_words_salt();
            let words_model = keys_logic_ref.global::<KeyChainLogic>().get_random_words();
            let m = match from_bip39_model(words_model) {
                Ok(r) => r,
                Err(_) => {
                    return LogicResult {
                        error: "bip39 words are invalid".into(),
                        response: SharedString::default(),
                        success: true,
                    }
                }
            };

            match core.init_data(sync, &email, &password, &words_salt, &m) {
                Ok(_) => LogicResult {
                    error: SharedString::default(),
                    response: SharedString::default(),
                    success: true,
                },
                Err(_) => LogicResult {
                    // TODO: make more informative errors
                    error: "Cannot init data".into(),
                    response: SharedString::default(),
                    success: false,
                },
            }
        });

    let lock_page_core_ref = core.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_unlock(move |password| {
            let mut core = lock_page_core_ref.borrow_mut();

            match core.guard.try_unlock(&password.to_string().as_bytes()) {
                Ok(_) => LogicResult {
                    error: SharedString::default(),
                    response: SharedString::default(),
                    success: true,
                },
                Err(_) => LogicResult {
                    // TODO: add more informative errors
                    error: "incorrect password".into(),
                    response: SharedString::default(),
                    success: false,
                },
            }
        });

    main_window
        .global::<GeneratorLogic>()
        .on_request_password_gen(
            move |lowercase: bool, upercase: bool, nums: bool, symbols: bool, length: i32| {
                let mut rng = rand::thread_rng();
                let pass_gen = PassGen::from(lowercase, upercase, nums, symbols);

                match pass_gen.gen(length as usize, &mut rng) {
                    Ok(password) => LogicResult {
                        error: SharedString::default(),
                        success: true,
                        response: String::from_utf8_lossy(&password).to_string().into(),
                    },
                    Err(_) => LogicResult {
                        // TODO: add more informative errors
                        error: "Invalid RNG".into(),
                        response: SharedString::default(),
                        success: false,
                    },
                }
            },
        );

    let logic_ref_add_new_element = Rc::clone(&main_window);
    main_window
        .global::<Logic>()
        .on_add_new_element(move |element| {
            let elements = logic_ref_add_new_element.global::<Logic>().get_elements();
            let mut vec_elements = elements
                .as_any()
                .downcast_ref::<Vec<Element>>()
                .unwrap()
                .clone();

            vec_elements.push(element);

            LogicElementsUpdateResult {
                error: "".into(),
                success: true,
                response: VecModel::from_slice(&vec_elements),
            }
        });

    app.run()
}

fn error_handler(error: ZebraErrors) -> Result<(), slint::PlatformError> {
    dbg!(error);
    // TODO: Show error window!
    Ok(())
}

fn main() -> Result<(), slint::PlatformError> {
    let core = match Core::new() {
        Ok(c) => c,
        Err(e) => {
            return error_handler(e);
        }
    };

    match core.sync() {
        Ok(_) => {}
        Err(e) => {
            return error_handler(e);
        }
    }

    handler(Rc::new(RefCell::new(core)))
}
