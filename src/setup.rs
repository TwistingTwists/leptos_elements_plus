use leptos::*;
use leptos_meta::{Script,Style};

use include_flate::flate;

flate!(pub static JS_STR: str from "js_dist/index.js");
flate!(pub static CSS_STR: str from "js_dist/index.css");

/// Add this component to your app to initialize element-plus
#[component]
pub fn ElementPlusSetup() -> impl IntoView {
    let js_str: &str = &JS_STR;
    let css_str: &str = &CSS_STR;
    view! {
        <Script type_="module">{js_str}</Script>
        <Style>{css_str}</Style>
    }
}
