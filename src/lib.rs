mod consts;
mod errors;
mod meta_models;
mod models;
#[macro_use]
mod part_command;
mod command_spec;
mod commands;
mod pass1;
mod pass2;
mod utils;

use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{meta_models::Code, pass1::Pass1};

pub fn load_from_file(path: PathBuf) -> String {
    let file = fs::read(path).expect("Unable to read file");
    let (res, _, had_errors) = encoding_rs::SHIFT_JIS.decode(&file);
    if !had_errors {
        return res.to_string();
    }

    let (res, _, had_errors) = encoding_rs::UTF_8.decode(&file);
    load(res)
}

pub fn load(mml: impl Into<String>) -> String {
    let code = Code::default();
    let mut pass1 = Pass1::new(code, mml.into());
    let _ = pass1.parse();

    return "".to_string();
}
