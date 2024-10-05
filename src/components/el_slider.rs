use leptos::*;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

use super::common::{Size,Placement};

#[component]
pub fn Slider(
    #[prop(into)] value: RwSignal<i32>,
    #[prop(into, optional)] min: Option<i32>,
    #[prop(into, optional)] max: Option<i32>,
    #[prop(into, optional)] disabled: Option<bool>,
    #[prop(into, optional)] step: Option<i32>,
    #[prop(into, optional)] show_input: Option<bool>,
    #[prop(into, optional)] show_input_controls: Option<bool>,
    #[prop(into, optional)] size: Option<Size>,
    #[prop(into, optional)] input_size: Option<Size>,
    #[prop(into, optional)] show_stops: Option<bool>,
    #[prop(into, optional)] show_tooltip: Option<bool>,
    #[prop(into, optional)] format_tooltip: Option<Callback<i32,String>>,
    #[prop(into, optional)] range: Option<bool>,
    #[prop(into, optional)] vertical: Option<bool>,
    #[prop(into, optional)] height: Option<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] range_start_label: Option<String>,
    #[prop(into, optional)] range_end_label: Option<String>,
    #[prop(into, optional)] format_value_text: Option<Callback<i32,String>>,
    #[prop(into, optional)] debounce: Option<i32>,
    #[prop(into, optional)] tooltip_class: Option<String>,
    #[prop(into, optional)] placement: Option<Placement>,
    #[prop(into, optional)] on_input: Option<Callback<i32>>,
    //#[prop(into, optional)] marks: Option<Object>,
    ) -> impl IntoView
{
    view! {
        <el-slider
            min=min
            max=max
            disabled=disabled.unwrap_or(false)
            step=step
            show-input=show_input.unwrap_or(false)
            show-input-controls=show_input_controls.unwrap_or(true)
            size=size.unwrap_or(Size::Default_).as_str()
            input-size=input_size.unwrap_or(Size::Default_).as_str()
            show-stops=show_stops.unwrap_or(false)
            show-tooltip=show_tooltip.unwrap_or(true)
            range=range.unwrap_or(false)
            vertical=vertical.unwrap_or(false)
            height=height
            label=label
            range-start-label=range_start_label
            range-end-label=range_end_label
            debounce=debounce
            tooltip-class=tooltip_class
            placement=placement.unwrap_or(Placement::Top).as_str()
            //marks=marks
            prop:modelValue=value
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                if let Some(v) = ev.detail().as_f64() {
                    value.set(v as i32);
                }
            }
            prop:formatTooltip=move||{
                if let Some(format_tooltip) = format_tooltip {
                    let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|value:JsValue| {
                        if let Some(value) = value.as_f64() {
                            format_tooltip.call(value as i32).into()
                        } else {
                            value
                        }
                    });
                    closure.into_js_value()
                } else {
                    JsValue::UNDEFINED
                }
            }
            prop:formatValueText=move||{
                if let Some(format_value_text) = format_value_text {
                    let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|value:JsValue| {
                        if let Some(value) = value.as_f64() {
                            format_value_text.call(value as i32).into()
                        } else {
                            value
                        }
                    });
                    closure.into_js_value()
                } else {
                    JsValue::UNDEFINED
                }
            }
            on:ce-input=move |ev: leptos::ev::CustomEvent| {
                if let Some(on_input) = on_input {
                    let array = Array::from(&ev.detail());
                    if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                        return;
                    }
                    let value = array.get(0);
                    if let Some(value) = value.as_f64() {
                        on_input.call(value as i32);
                    }
                }
            }
            >
        </el-slider>
    }
}
