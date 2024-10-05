
use leptos::*;
//use leptos::logging::log;

//use serde::{de::DeserializeOwned, Serialize};
//use wasm_bindgen::JsValue;
use serde_wasm_bindgen::from_value;

#[component]
pub fn Tabs(
    #[prop(into, optional)] value: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] width: Option<AttributeValue>,
    #[prop(into, optional)] height: Option<AttributeValue>,
    #[prop(into, optional)] on_tab_change: Option<Callback<String>>,
    children: Children
    ) -> impl IntoView
{
    view! {
        <el-tabs
            value=value
            class=class
            width=width
            height=height
            sytle=style
            on:tab-change=move |event: leptos::ev::CustomEvent| {
                if let Ok(value) = from_value::<String>(event.detail()) {
                    if let Some(cb) = on_tab_change {
                        cb.call(value);
                    }
                }
            }
        >

            {children()}
        </el-tabs>
    }
}

#[component]
pub fn TabPane (
    #[prop(into, optional)] label: Option<AttributeValue>,
    #[prop(into, optional)] name: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{
    view! {
        <el-tab-pane label=label name=name>
            {children()}
        </el-tab-pane>
    }
}
