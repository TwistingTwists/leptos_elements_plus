use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

#[component]
pub fn TimePicker(
    ) -> impl IntoView
{
    view! { <el-time-picker></el-time-picker> }
}
