#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;


#[wasm_bindgen]
pub struct Message {
    kind: MessageKind,
    data: i32
}

#[wasm_bindgen]
pub enum MessageKind {
    Init,
    Error,
    Update,
}

