use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;
pub use super::common::Type;

#[component]
pub fn Alert(
    #[prop(into, optional)] title: MaybeSignal<String>,
    #[prop(into, optional, default = Type::Info.into())] type_: MaybeSignal<Type>,
    #[prop(into, optional)] description: MaybeSignal<String>,
    #[prop(into, optional, default = true.into())] closable: MaybeSignal<bool>,
    #[prop(into, optional, default = false.into())] center: MaybeSignal<bool>,
    #[prop(into, optional)] close_text: MaybeSignal<String>,
    #[prop(into, optional, default = false.into())] show_icon: MaybeSignal<bool>,
    #[prop(into, optional)] on_close: Option<Callback<i32>>,
    ) -> impl IntoView
{
    view! { 
        <el-alert
            title=title
            type=type_.get().as_str()
            description=description
            prop:closable=closable
            center=center
            close-text=close_text
            show-icon=show_icon
            on:ce-close=move|_: leptos::ev::CustomEvent| {
                if let Some(cb) = on_close {
                    cb.call(0);
                }
            }
            >
        </el-alert>
    }
}
