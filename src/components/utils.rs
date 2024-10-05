
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use js_sys::{Object,Reflect};
use leptos::web_sys::Node;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = element_plus, js_name = convert)]
    pub fn convert(node: Node)-> JsValue;
}

pub fn set_property<T: Into<JsValue>>(object: &Object, key: &str, value: T) {
    let _ = Reflect::set(object, &JsValue::from_str(key), &value.into());
}
