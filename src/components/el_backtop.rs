use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

#[component]
pub fn Backtop(
    #[prop(into, optional)] target: MaybeSignal<String>,
    #[prop(into, optional)] visibility_height: MaybeSignal<i32>,
    #[prop(into, optional)] right: MaybeSignal<i32>,
    #[prop(into, optional)] bottom: MaybeSignal<i32>,
    #[prop(into, optional)] on_click: Option<Callback<i32>>,
    #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView
{
    view! {
        <el-backtop
            target=target
            visibility-height=visibility_height
            right=right
            bottom=bottom
            on:ce-click=move |_: leptos::ev::CustomEvent| {
                log!("Backtop clicked");
                if let Some(cb) = on_click {
                    cb.call(0);
                }
            }
            >
            {
                match children {
                    Some(children) => children(),
                    None => Fragment::new(vec![]),
                } 
            }
        </el-backtop>
    }
}
