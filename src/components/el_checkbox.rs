use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue/*, closure::Closure*/};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

pub use super::common::Size;

#[component]
pub fn Checkbox(
    #[prop(into, optional)] label: Option<AttributeValue>,
    #[prop(into, optional)] true_label: Option<AttributeValue>,
    #[prop(into, optional)] false_label: Option<AttributeValue>,
    #[prop(into, optional, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, optional, default = false.into())] border: MaybeSignal<bool>,
    #[prop(into, optional, default = Size::Default_.into())] size: MaybeSignal<Size>,
    #[prop(into, optional)] name: MaybeSignal<String>,
    #[prop(into, optional, default = false.into())] checked: MaybeSignal<bool>,
    #[prop(into, optional, default = false.into())] indeterminate: MaybeSignal<bool>,
    #[prop(into, optional)] tabindex: MaybeSignal<i32>,
    #[prop(into, optional)] id: MaybeSignal<String>,
    #[prop(into, optional, default = false.into())] controls: MaybeSignal<bool>,
    #[prop(into, optional)] on_change: Option<Callback<bool>>,
    ) -> impl IntoView
{

    //let value = create_rw_signal(JsValue::from("value1"));

    view! {
        <el-checkbox
            label=label
            true-label=true_label
            false-label=false_label
            disabled=disabled
            border=border
            size=size.get().as_str()
            name=name
            checked=checked
            indeterminate=indeterminate
            tabindex=tabindex
            id=id
            controls=controls
            on:ce-change=move|ev: leptos::ev::CustomEvent| {
                if let Some(cb) = on_change {
                    let data= Array::from(&ev.detail());
                    if data != JsValue::UNDEFINED.into() && data.length() != 0 {
                        let data = data.get(0);
                        if let Ok(d) = serde_wasm_bindgen::from_value(data) {
                            cb.call(d);
                        } else {
                            log!("Checkbox: could not deserialize data");
                        }
                    }
                }
            }
            /*
            prop:modelValue=move|| {
                value.get()
            }
            */
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                let data= Array::from(&ev.detail());
                if data != JsValue::UNDEFINED.into() && data.length() != 0 {
                    let data = data.get(0);
                    if let Ok(_d) = serde_wasm_bindgen::from_value::<bool>(data) {
                        //value.set(d.into());
                    } else {
                        log!("Checkbox: could not deserialize data");
                    }
                }
            }
            >
        </el-checkbox>
    }
}
