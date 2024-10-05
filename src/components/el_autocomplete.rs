
use leptos::*;
use leptos::logging::log;

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{JsValue, closure::Closure};
use serde_wasm_bindgen::from_value;

pub trait FilterAble {
    fn get_filter_value(&self) -> &str;
}

#[component]
pub fn Autocomplete<T>(
    #[prop(into)] suggestions: MaybeSignal<Vec<T>>,
    #[prop(into, optional)] value_key: Option<AttributeValue>,
    #[prop(into, optional)] placeholder: Option<AttributeValue>,
    #[prop(into, optional)] on_selected: Option<Callback<T>>,
    ) -> impl IntoView
    where T: Serialize + DeserializeOwned + 'static + Clone + FilterAble
{

    let value = create_rw_signal(JsValue::from(""));

    let suggestions = suggestions.get();

    let closure = Closure::<dyn FnMut(JsValue,JsValue)>::new(move|query_string: JsValue,call_back: JsValue|{
           log!("Autocomplete: query_string: {:?}",query_string);
           let query_string: String = from_value(query_string).unwrap_or_default();

           let filtered_suggestions: Vec<&T> = suggestions.iter().filter(|item|{
               item.get_filter_value().contains(&query_string)
           }).collect();

           let cb: js_sys::Function = js_sys::Function::from(call_back);
           if let Ok(data) = serde_wasm_bindgen::to_value(&filtered_suggestions){
               let _ = cb.call1(&JsValue::NULL, &data);
           }
        }).into_js_value();
           
    view! {
        <el-autocomplete

            prop:fetchSuggestions=closure
            clearable
            placeholder=placeholder
            value-key=value_key
            class="inline-input w-50"
            prop:modelValue=value
            trigger-on-focus=true
            on:el-select=move |event: leptos::ev::CustomEvent| {
                let value = event.detail();
                let data: T;
                if let Ok(d) = from_value(value) {
                    data = d;
                    if let Some(cb) = on_selected {
                        cb.call(data);
                    }
                }
            }

            on:ce-change=move |ev: leptos::ev::CustomEvent| {
                log!("Autocomplete: change {:?}", ev.detail());
            }

            on:ce-select=move |ev: leptos::ev::CustomEvent| {
                log!("Autocomplete: select {:?}", ev.detail());
            }

            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                value.set(ev.detail());
            }
        >
        </el-autocomplete>
    }
}
