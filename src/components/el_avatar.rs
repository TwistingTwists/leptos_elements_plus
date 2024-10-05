use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

pub use super::common::Size;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fit{
    Fill,
    Contain,
    #[default]
    Cover,
    None,
    ScaleDown,
}

impl Fit{
    pub fn as_str(&self) -> &'static str {
        match self{
            Fit::Fill => "fill",
            Fit::Contain => "contain",
            Fit::Cover => "cover",
            Fit::None => "none",
            Fit::ScaleDown => "scale-down",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape{
    #[default]
    Circle,
    Square,
}

impl Shape{
    pub fn as_str(&self) -> &'static str {
        match self{
            Shape::Circle => "circle",
            Shape::Square => "square",
        }
    }
}

#[component]
pub fn Avatar(
    #[prop(into, optional)] icon: MaybeSignal<String>,
    #[prop(into, optional)] src: MaybeSignal<String>,
    #[prop(into, optional)] src_set: MaybeSignal<String>,
    #[prop(into, optional)] alt: MaybeSignal<String>,
    #[prop(into, optional, default = Fit::Cover.into())] fit: MaybeSignal<Fit>,
    #[prop(into, optional, default = Size::Default_.into())] size: MaybeSignal<Size>,
    #[prop(into, optional, default = Shape::Circle.into())] shape: MaybeSignal<Shape>,
    #[prop(into, optional)] on_error: Option<Callback<i32>>,
    ) -> impl IntoView
{
    view! {
        <el-avatar
            size=size.get().as_str()
            shape=shape.get().as_str()
            src=src
            src-set=src_set
            alt=alt
            fit=fit.get().as_str()
            icon=icon
            on:ce-error=move |_: leptos::ev::CustomEvent| {
                if let Some(cb) = on_error {
                    cb.call(0);
                }
            }
            >
        </el-avatar>
    }
}
