
use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue/*, closure::Closure*/};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
use serde_wasm_bindgen::from_value;

pub use super::common::Size;

#[component]
pub fn InputNumber(
    #[prop(into)] value: RwSignal<i32>,
    #[prop(into, optional)] max: Option<AttributeValue>,
    #[prop(into, optional)] min: Option<AttributeValue>,
    #[prop(into, optional, default = 1.into())] step: MaybeSignal<i32>,
    #[prop(into, optional)] step_strictly: MaybeSignal<bool>,
    #[prop(into, optional)] precision: MaybeSignal<i32>,
    #[prop(into, optional, default = Size::Default_.into())] size: Size,
    #[prop(into, optional)] readonly: MaybeSignal<bool>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional, default = true.into())] controls: MaybeSignal<bool>,
    #[prop(into, optional)] controls_position: Option<AttributeValue>,
    #[prop(into, optional)] name: Option<AttributeValue>,
    #[prop(into, optional)] label: Option<AttributeValue>,
    #[prop(into, optional)] placeholder: Option<AttributeValue>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] value_on_clear: MaybeSignal<i32>,
    ) -> impl IntoView
{
    view! {
        <el-input-number
            prop:modelValue=value
            placeholder=placeholder
            disabled=disabled
            size=size.as_str()
            name=name
            readonly=readonly
            max=max
            min=min
            step=step
            label=label
            value-on-clear=value_on_clear
            id=id
            controls=controls
            controls-position=controls_position
            step-strictly=step_strictly
            precision=precision
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                let array = Array::from(&ev.detail());
                if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                    value.set(0);
                    return;
                }
                let v = array.get(0);
                if let Ok(v) = from_value::<i32>(v) {
                    value.set(v);
                } else {
                    log!("Input: ce-update:modelValue: failed to parse value");
                }
            }
            >
        </el-input-number>
    }
}

