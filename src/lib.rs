#![feature(let_chains)]

extern crate alloc;

#[macro_use]
extern crate napi_derive;

pub mod find;

static TARGET_TRIPLE: &str = include_str!(concat!(env!("OUT_DIR"), "/triple.txt"));

#[napi]
pub fn get_target_triple() -> napi::Result<String> {
    Ok(TARGET_TRIPLE.to_string())
}
