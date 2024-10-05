
use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue/*, closure::Closure*/};
use js_sys::{Array,/*Object,Reflect*/};
use serde::{Serialize,Deserialize/*,de::DeserializeOwned*/};
//use serde_wasm_bindgen::from_value;

use super::common::Placement;

#[derive(Serialize, Deserialize, Clone)]
pub struct OptionsProps{
    pub label: String,
    pub value: String,
    pub options: String,
    pub disabled: String,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSize{
    #[default]
    Medium,
    Small,
    Mini,
}

impl InputSize{
    pub fn as_str(&self) -> &'static str {
        match self {
            InputSize::Medium => "medium",
            InputSize::Small => "small",
            InputSize::Mini => "mini",
        }
    }
}

#[component]
pub fn SelectV2<T>(
    #[prop(into, optional)] value: Option<RwSignal<String>>,
    #[prop(into, optional)] values: Option<RwSignal<Vec<String>>>,
    #[prop(into)] options: MaybeSignal<Vec<T>>,
    #[prop(into, optional)] props: Option<OptionsProps>,
    #[prop(into, optional)] multiple: bool,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] size: MaybeSignal<InputSize>,
    #[prop(into, optional)] clearable: MaybeSignal<bool>,
    #[prop(into, optional)] collapse_tags: MaybeSignal<bool>,
    #[prop(into, optional)] collapse_tags_tooltip: MaybeSignal<bool>,
    #[prop(into, optional)] max_collapse_tags: MaybeSignal<i32>,
    #[prop(into, optional)] multiple_limit: MaybeSignal<i32>,
    #[prop(into, optional)] name: MaybeSignal<String>,
    #[prop(into, optional)] autocomplete: MaybeSignal<String>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(into, optional)] filterable: MaybeSignal<bool>,
    #[prop(into, optional)] allow_create: MaybeSignal<bool>,
    #[prop(into, optional)] reserve_keyword: MaybeSignal<bool>,
    #[prop(into, optional, default = "No Data".into())] no_data_text: MaybeSignal<String>,
    #[prop(into, optional)] popper_class: MaybeSignal<String>,
    #[prop(into, optional)] teleported: MaybeSignal<bool>,
    #[prop(into, optional)] persistent: MaybeSignal<bool>,
    #[prop(into, optional)] automatic_dropdown: MaybeSignal<bool>,
    #[prop(into, optional, default = 170.into())] height: MaybeSignal<i32>,
    #[prop(into, optional, default = 34.into())] item_height: MaybeSignal<i32>,
    #[prop(into, optional)] scrollbar_always_on: MaybeSignal<bool>,
    #[prop(into, optional)] placement: MaybeSignal<Placement>,
    #[prop(into, optional)] on_remove_tag: Option<Callback<String>>,
    ) -> impl IntoView
    where T: Serialize + 'static + Clone + std::default::Default
{
    view! { 
        <el-select-v2
            disabled=disabled
            clearable=clearable
            collapse-tags=collapse_tags
            collapse-tags-tooltip=collapse_tags_tooltip
            max-collapse-tags=max_collapse_tags
            multiple-limit=multiple_limit
            name=name
            autocomplete=autocomplete
            placeholder=placeholder
            filterable=filterable
            allow-create=allow_create
            reserve-keyword=reserve_keyword
            no-data-text=no_data_text
            popper-class=popper_class
            teleported=teleported
            persistent=persistent
            automatic-dropdown=automatic_dropdown
            height=height
            item-height=item_height
            scrollbar-always-on=scrollbar_always_on
            size=size.get().as_str()
            placement=placement.get().as_str()
            prop:props=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&props) {
                    data
                } else {
                    log!("SelectV2: props is not serializable");
                    JsValue::UNDEFINED
                }
            }
            prop:options=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&options) {
                    data
                } else {
                    log!("SelectV2: options is not serializable");
                    JsValue::UNDEFINED
                }
            }
            prop:modelValue=move|| {
                if multiple {
                    if let Some(values) = values {
                        if let Ok(data) = serde_wasm_bindgen::to_value(&values.get()) {
                            data
                        } else {
                            log!("SelectV2: values is not serializable");
                            JsValue::UNDEFINED
                        }
                    } else {
                        JsValue::UNDEFINED
                    }
                }else{
                    if let Some(value) = value {
                        if let Ok(data) = serde_wasm_bindgen::to_value(&value.get()) {
                            data
                        } else {
                            log!("SelectV2: value is not serializable");
                            JsValue::UNDEFINED
                        }
                    } else {
                        JsValue::UNDEFINED
                    }
                }
            }
            prop:multiple=multiple
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                let data= Array::from(&ev.detail());
                if data == JsValue::UNDEFINED.into() || data.length() == 0 {
                    log!("SelectV2: data is undefined");
                    return;
                }
                let data = data.get(0);
                if data.is_array() {
                    if let Some(values) = values {
                        if let Ok(data) = serde_wasm_bindgen::from_value(data) {
                            values.set(data);
                        } else {
                            log!("SelectV2: values is not deserializable");
                        }
                    } else {
                        log!("SelectV2: values is not set");
                    }
                } else {
                    if let Some(value) = value {
                        if let Ok(data) = serde_wasm_bindgen::from_value(data) {
                            value.set(data);
                        } else {
                            log!("SelectV2: value is not deserializable");
                        }
                    } else {
                        log!("SelectV2: value is not set");
                    }
                }
            }
            on:ce-remove-tag=move |ev: leptos::ev::CustomEvent|{
                if let Some(cb) = on_remove_tag {
                    let data= Array::from(&ev.detail());
                    if data != JsValue::UNDEFINED.into() && data.length() != 0 {
                        let data = data.get(0);
                        if let Ok(d) = serde_wasm_bindgen::from_value(data) {
                            cb.call(d);
                        } else {
                            log!("SelectV2: on_remove_tag is not deserializable");
                        }
                    }
                }
            }
            >
        </el-select-v2>
    }
}
