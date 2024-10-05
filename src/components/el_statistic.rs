use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

#[component]
pub fn Statistic(
    #[prop(into, optional)] value: Option<i32>,
    #[prop(into, optional)] decimal_separator: Option<String>,
    #[prop(into, optional)] formatter: Option<Callback<i32, String>>,
    #[prop(into, optional)] group_separator: Option<String>,
    #[prop(into, optional)] precision: Option<i32>,
    #[prop(into, optional)] prefix: Option<String>,
    #[prop(into, optional)] suffix: Option<String>,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] value_style: Option<String>,
    ) -> impl IntoView
{
    view! {
        <el-statistic
            value=value
            decimal-separator=decimal_separator
            group-separator=group_separator
            precision=precision
            prefix=prefix
            suffix=suffix
            title=title
            value-style=value_style
            prop:formatter=move||{
                if let Some(formatter) = formatter {
                    let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|value:JsValue| {
                        if let Some(value) = value.as_f64() {
                            formatter.call(value as i32).into()
                        } else {
                            value
                        }
                    });
                    closure.into_js_value()
                } else {
                    JsValue::UNDEFINED
                }
            }
            >
        </el-statistic>
    }
}

#[component]
pub fn Countdown(
    #[prop(into)] value: MaybeSignal<f64>,
    #[prop(into, optional)] format: Option<String>,
    #[prop(into, optional)] prefix: Option<String>,
    #[prop(into, optional)] suffix: Option<String>,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] value_style: Option<String>,
    //#[prop(into, optional)] on_change: Option<Callback<f64>>,
    #[prop(into, optional)] on_finish: Option<Callback<()>>,
    ) -> impl IntoView
{
    view! {
        <el-countdown
            prop:value=value
            format=format
            prefix=prefix
            suffix=suffix
            title=title
            value-style=value_style
            /*
            on:ce-change=move |ev: leptos::ev::CustomEvent| {
                if let Some(on_change) = on_change {
                }
            }
            */
            on:ce-finish=move |_ev: leptos::ev::CustomEvent| {
                if let Some(on_finish) = on_finish {
                    on_finish.call(());
                }
            }
            >
        </el-countdown>
    }
}
