use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

/*
type 	type of Tag 	^[enum]'success' | 'info' | 'warning' | 'danger' | '' 	''
closable 	whether Tag can be removed 	^[boolean] 	false
disable-transitions 	whether to disable animations 	^[boolean] 	false
hit 	whether Tag has a highlighted border 	^[boolean] 	false
color 	background color of the Tag 	^[string] 	''
size 	size of Tag 	^[enum]'large' | 'default' | 'small' | '' 	''
effect 	theme of Tag 	^[enum]'dark' | 'light' | 'plain' 	light
round 	whether Tag is rounded 	^[boolean] 	false
*/

use super::common::{Size};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Dark,
    #[default]
    Light,
    Plain,
}

impl Effect {
    pub fn as_str(&self) -> &'static str {
        match self {
            Effect::Dark => "dark",
            Effect::Light => "light",
            Effect::Plain => "plain",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Success,
    Warning,
    Danger,
    #[default]
    Info,
}

impl Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Type::Success => "success",
            Type::Warning => "warning",
            Type::Info => "info",
            Type::Danger => "danger",
        }
    }
}

#[component]
pub fn Tag(
    #[prop(into, optional)] type_: Option<Type>,
    #[prop(into, optional)] closable: Option<bool>,
    #[prop(into, optional)] disable_transitions: Option<bool>,
    #[prop(into, optional)] hit: Option<bool>,
    #[prop(into, optional)] color: Option<String>,
    #[prop(into, optional)] size: Option<Size>,
    #[prop(into, optional)] effect: Option<Effect>,
    #[prop(into, optional)] round: Option<bool>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView
{
    view! {
        <el-tag
            type=type_.unwrap_or_default().as_str()
            closable=closable.unwrap_or(false)
            disable-transitions=disable_transitions.unwrap_or(false)
            hit=hit.unwrap_or(false)
            color=color
            size=size.unwrap_or_default().as_str()
            effect=effect.unwrap_or_default().as_str()
            round=round.unwrap_or(false)
            on:ce-click=move |_ev: leptos::ev::CustomEvent| {
                if let Some(on_click) = on_click {
                    on_click.call(());
                }
            }
            on:ce-close=move |_ev: leptos::ev::CustomEvent| {
                if let Some(on_close) = on_close {
                    on_close.call(());
                }
            }
            >
            {
                match children {
                    Some(children) => children(),
                    None => Fragment::new(vec![]),
                } 
            }
        </el-tag>
    }
}
