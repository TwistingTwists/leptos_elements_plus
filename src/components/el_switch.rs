use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

use super::common::{Size};

#[component]
pub fn Switch(
    #[prop(into)] value: RwSignal<bool>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] loading: MaybeSignal<bool>,
    #[prop(into, optional)] size: Option<Size>,
    #[prop(into, optional)] width: Option<i32>,
    #[prop(into, optional)] inline_prompt: Option<bool>,
    #[prop(into, optional)] active_icon: Option<String>,
    #[prop(into, optional)] inactive_icon: Option<String>,
    #[prop(into, optional)] active_action_icon: Option<String>,
    #[prop(into, optional)] inactive_action_icon: Option<String>,
    #[prop(into, optional)] active_text: Option<String>,
    #[prop(into, optional)] inactive_text: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] tabindex: Option<i32>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] before_change: Option<Callback<(),bool>>,
    #[prop(into, optional)] on_change: Option<Callback<bool>>,
    ) -> impl IntoView
{
    view! {
        <el-switch
            disabled=disabled
            loading=loading
            size=size.unwrap_or(Size::Default_).as_str()
            width=width
            inline-prompt=inline_prompt.unwrap_or(false)
            active-icon=active_icon
            inactive-icon=inactive_icon
            active-action-icon=active_action_icon
            inactive-action-icon=inactive_action_icon
            active-text=active_text
            inactive-text=inactive_text
            name=name
            id=id
            tabindex=tabindex
            label=label
            prop:modelValue=value
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                let array = Array::from(&ev.detail());
                if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                    return;
                }
                let v = array.get(0);
                if let Some(v) = v.as_bool() {
                    value.set(v);
                }
            }
            prop:beforeChange=move||{
                if let Some(before_change) = before_change {
                    let closure: Closure<dyn FnMut()->JsValue> = Closure::new(move|| {
                        before_change.call(()).into()
                    });
                    closure.into_js_value()
                } else {
                    JsValue::UNDEFINED
                }
            }
            on:ce-change=move |ev: leptos::ev::CustomEvent| {
                if let Some(on_change) = on_change {
                    let array = Array::from(&ev.detail());
                    if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                        return;
                    }
                    let v = array.get(0);
                    if let Some(v) = v.as_bool() {
                        on_change.call(v);
                    }
                }
            }
            >
        </el-switch>
    }
}
