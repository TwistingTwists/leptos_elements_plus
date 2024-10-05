use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
use serde_wasm_bindgen::from_value;

pub use super::common::Size;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    TextArea,
    Password,
    Button,
    Checkbox,
    File,
    Number,
    Radio,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::TextArea => "textarea",
            InputType::Password => "password",
            InputType::Button => "button",
            InputType::Checkbox => "checkbox",
            InputType::File => "file",
            InputType::Number => "number",
            InputType::Radio => "radio",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resize {
    #[default]
    None,
    Both,
    Horizontal,
    Vertical,
}

impl Resize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Resize::None => "none",
            Resize::Both => "both",
            Resize::Horizontal => "horizontal",
            Resize::Vertical => "vertical",
        }
    }
}

#[component]
pub fn Input(
    #[prop(into, optional, default = InputType::Text.into())] type_: InputType,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] maxlength: Option<AttributeValue>,
    #[prop(into, optional)] minlength: Option<AttributeValue>,
    #[prop(into, optional)] show_word_limit: MaybeSignal<bool>,
    #[prop(into, optional)] placeholder: Option<AttributeValue>,
    #[prop(into, optional)] clearable: MaybeSignal<bool>,
    #[prop(into, optional)] formatter: Option<Callback<String,String>>,
    #[prop(into, optional)] parser: Option<Callback<String,String>>,
    #[prop(into, optional)] show_password: MaybeSignal<bool>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional, default = Size::Default_.into())] size: Size,
    //#[prop(into, optional)] prefix_icon: Option<AttributeValue>,
    //#[prop(into, optional)] suffix_icon: Option<AttributeValue>,
    #[prop(into, optional, default = 2.into())] rows: MaybeSignal<u32>,
    #[prop(into, optional)] autosize: MaybeSignal<bool>,
    #[prop(into, optional)] autocomplete: Option<AttributeValue>,
    #[prop(into, optional)] name: Option<AttributeValue>,
    #[prop(into, optional)] readonly: MaybeSignal<bool>,
    #[prop(into, optional)] max: Option<AttributeValue>,
    #[prop(into, optional)] min: Option<AttributeValue>,
    #[prop(into, optional)] step: Option<AttributeValue>,
    #[prop(into, optional)] resize: Option<Resize>,
    #[prop(into, optional)] autofocus: MaybeSignal<bool>,
    #[prop(into, optional)] form: Option<AttributeValue>,
    #[prop(into, optional)] label: Option<AttributeValue>,
    #[prop(into, optional)] tabindex: Option<AttributeValue>,
    #[prop(into, optional)] input_style: Option<AttributeValue>,
    #[prop(into, optional)] on_blur: Option<Callback<()>>,
    ) -> impl IntoView
{
    view! {
        <el-input
            type=type_.as_str()
            prop:modelValue=value
            maxlength=maxlength
            minlength=minlength
            show-word-limit=show_word_limit
            placeholder=placeholder
            clearable=clearable
            prop:formatter=move||{
                if let Some(formatter) = formatter {
                    let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|v| {
                        if let Ok(v) = from_value::<String>(v) {
                            let result = formatter.call(v);
                            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::UNDEFINED)
                        } else {
                            log!("Input: formatter: failed to parse value");
                            JsValue::UNDEFINED
                        }
                    });
                    closure.into_js_value()
                }else{
                    JsValue::UNDEFINED
                }
            }
            prop:parser=move||{
                if let Some(parser) = parser {
                    let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|v| {
                        if let Ok(v) = from_value::<String>(v) {
                            let result = parser.call(v);
                            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::UNDEFINED)
                        } else {
                            log!("Input: parser: failed to parse value");
                            JsValue::UNDEFINED
                        }
                    });
                    closure.into_js_value()
                }else{
                    JsValue::UNDEFINED
                }
            }
            show-password=show_password
            disabled=disabled
            size=size.as_str()
            //prefix-icon=prefix_icon
            //suffix-icon=suffix_icon
            rows=rows
            autosize=autosize
            autocomplete=autocomplete
            name=name
            readonly=readonly
            max=max
            min=min
            step=step
            resize=resize.map(|r| r.as_str())
            autofocus=autofocus
            form=form
            label=label
            tabindex=tabindex
            input-style=input_style
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                let array = Array::from(&ev.detail());
                if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                    value.set("".to_string());
                    return;
                }
                let v = array.get(0);
                if let Ok(v) = from_value::<String>(v) {
                    value.set(v);
                } else {
                    log!("Input: ce-update:modelValue: failed to parse value");
                }
            }
            on:ce-blur=move |_ev: leptos::ev::CustomEvent| {
                if let Some(on_blur) = on_blur {
                    on_blur.call(());
                }
            }
            />
    }
}
