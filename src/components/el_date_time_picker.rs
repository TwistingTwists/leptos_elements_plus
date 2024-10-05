use leptos::*;
//use leptos::logging::log;

use js_sys::Date;
//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;
pub use super::common::Size;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type{
    Year,
    Month,
    Date,
    #[default]
    DateTime,
    Week,
}

impl Type{
    pub fn as_str(&self) -> &'static str{
        match self {
            Type::Year => "year",
            Type::Month => "month",
            Type::Date => "date",
            Type::DateTime => "datetime",
            Type::Week => "week",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeType{
    Dates,
    #[default]
    DateTimeRange,
    DateRange,
    MonthRange,
}

impl RangeType{
    pub fn as_str(&self) -> &'static str{
        match self {
            RangeType::Dates => "dates",
            RangeType::DateTimeRange => "datetimerange",
            RangeType::DateRange => "daterange",
            RangeType::MonthRange => "monthrange",
        }
    }
}

#[component]
pub fn DateTimePicker(
    #[prop(into)] value: RwSignal<Date>,
    #[prop(into, optional, default = Type::DateTime.into())] type_: Type,
    #[prop(into, optional)] readonly: MaybeSignal<bool>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] editable: MaybeSignal<bool>,
    #[prop(into, optional)] clearable: MaybeSignal<bool>,
    #[prop(into, optional, default = Size::Default_.into())] size: Size,
    #[prop(into, optional)] placeholder: Option<AttributeValue>,
    #[prop(into, optional)] arrow_control: MaybeSignal<bool>,
    #[prop(into, optional, default = "YYYY-MM-DD HH:mm:ss".into())] format: MaybeSignal<String>,
    #[prop(into, optional)] popper_class: Option<AttributeValue>,
    #[prop(into, optional, default = Date::new_0().into())] default_value: MaybeSignal<Date>,
    #[prop(into, optional)] value_format: Option<AttributeValue>,
    #[prop(into, optional)] date_format: Option<AttributeValue>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] name: Option<AttributeValue>,
    #[prop(into, optional)] prefix_icon: Option<AttributeValue>,
    #[prop(into, optional)] clear_icon: Option<AttributeValue>,
    //#[prop(into, optional)] shortcuts: Option<AttributeValue>,
    //#[prop(into, optional)] cell_class_name: Option<AttributeValue>,
    #[prop(into, optional)] teleported: MaybeSignal<bool>,
    #[prop(into, optional)] disabled_date: Option<Callback<Date,bool>>,
    //#[prop(into, optional)] on_change: Option<Callback<Date>>,
    ) -> impl IntoView
{
    view! {
        <el-date-picker
            prop:modelValue=value
            type=type_.as_str()
            placeholder=placeholder
            readonly=readonly
            disabled=disabled
            editable=editable
            clearable=clearable
            size=size.as_str()
            arrow-control=arrow_control
            format=format
            popper-class=popper_class
            prop:default-value=default_value
            value-format=value_format
            date-format=date_format
            id=id
            name=name
            prefix-icon=prefix_icon
            clear-icon=clear_icon
            //shortcuts=shortcuts
            //cell-class-name=cell_class_name
            teleported=teleported
            prop:disabledDate=move||{
                if let Some(disabled_date) = disabled_date {
                    let closure: Closure<dyn FnMut(Date)->bool> = Closure::new(move|date| {
                        disabled_date.call(date)
                    });
                    closure.into_js_value()
                }else{
                    JsValue::UNDEFINED
                }
            }
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                value.set(Date::new(&ev.detail()));
            }
            />
    }
}

#[component]
pub fn DateTimeRangePicker(
    #[prop(into)] values: RwSignal<Vec<Date>>,
    #[prop(into, optional, default = RangeType::DateTimeRange.into())] type_: RangeType,
    #[prop(into, optional)] readonly: MaybeSignal<bool>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] editable: MaybeSignal<bool>,
    #[prop(into, optional)] clearable: MaybeSignal<bool>,
    #[prop(into, optional, default = Size::Default_.into())] size: Size,
    #[prop(into, optional)] start_placeholder: Option<AttributeValue>,
    #[prop(into, optional)] end_placeholder: Option<AttributeValue>,
    #[prop(into, optional)] arrow_control: MaybeSignal<bool>,
    #[prop(into, optional, default = "YYYY-MM-DD HH:mm:ss".into())] format: MaybeSignal<String>,
    #[prop(into, optional)] popper_class: Option<AttributeValue>,
    #[prop(into, optional)] range_separator: Option<AttributeValue>,
    #[prop(into, optional, default = Date::new_0().into())] default_value: MaybeSignal<Date>,
    #[prop(into, optional, default = Date::new_0().into())] default_time: MaybeSignal<Date>,
    #[prop(into, optional)] value_format: Option<AttributeValue>,
    #[prop(into, optional)] date_format: Option<AttributeValue>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] name: Option<AttributeValue>,
    #[prop(into, optional)] unlink_panels: MaybeSignal<bool>,
    #[prop(into, optional)] prefix_icon: Option<AttributeValue>,
    #[prop(into, optional)] clear_icon: Option<AttributeValue>,
    //#[prop(into, optional)] shortcuts: Option<AttributeValue>,
    //#[prop(into, optional)] cell_class_name: Option<AttributeValue>,
    #[prop(into, optional)] teleported: MaybeSignal<bool>,
    #[prop(into, optional)] disabled_date: Option<Callback<Date,bool>>,
    //#[prop(into, optional)] on_change: Option<Callback<Date>>,
    ) -> impl IntoView
{
    view! {
        <el-date-picker
            prop:modelValue=move||{
                let array = Array::new();
                for value in values.get(){
                    array.push(&value);
                }
                array
            }
            type=type_.as_str()
            readonly=readonly
            disabled=disabled
            editable=editable
            clearable=clearable
            size=size.as_str()
            start-placeholder=start_placeholder
            end-placeholder=end_placeholder
            arrow-control=arrow_control
            format=format
            popper-class=popper_class
            range-separator=range_separator
            prop:default-value=default_value
            prop:default-time=default_time
            value-format=value_format
            date-format=date_format
            id=id
            name=name
            unlink-panels=unlink_panels
            prefix-icon=prefix_icon
            clear-icon=clear_icon
            //shortcuts=shortcuts
            //cell-class-name=cell_class_name
            teleported=teleported
            prop:disabledDate=move||{
                if let Some(disabled_date) = disabled_date {
                    let closure: Closure<dyn FnMut(Date)->bool> = Closure::new(move|date| {
                        disabled_date.call(date)
                    });
                    closure.into_js_value()
                }else{
                    JsValue::UNDEFINED
                }
            }
            on:ce-update:modelValue=move |ev: leptos::ev::CustomEvent| {
                let array = Array::from(&ev.detail());
                if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                    values.set(Vec::new());
                    return;
                }
                let array = Array::from(&array.get(0));
                if array == JsValue::UNDEFINED.into() || array.length() == 0 {
                    values.set(Vec::new());
                    return;
                }
                let mut vec = Vec::new();
                for i in 0..array.length(){
                    vec.push(Date::new(&array.get(i)));
                }
                values.set(vec);
            }
            />
    }
}
