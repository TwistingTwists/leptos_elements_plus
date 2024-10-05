
use leptos::*;
//use leptos::logging::log;

//use serde::{de::DeserializeOwned, Serialize};
//use wasm_bindgen::JsValue;
//use serde_wasm_bindgen::from_value;

#[component]
pub fn Menu(
    #[prop(into, optional)] default_active: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{
    view! {
        <el-menu default-active=default_active class=class>
            {children()}
        </el-menu>
    }
}

#[component]
pub fn MenuItem (
    #[prop(into, optional)] index: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{
    view! { <el-menu-item index=index>{children()}</el-menu-item> }
}

#[component]
pub fn SubMenu (
    #[prop(into, optional)] index: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{
    view! { <el-sub-menu index=index>{children()}</el-sub-menu> }
}

#[component]
pub fn MenuItemGroup (
    #[prop(into, optional)] title: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{
    view! { <el-menu-item-group title=title>{children()}</el-menu-item-group> }
}
