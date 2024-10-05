use leptos::*;
//use leptos::logging::log;

//use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

pub use super::common::{Direction,BorderStyle,ContentPosition};

#[component]
pub fn Divider(
    #[prop(into, optional, default = Direction::Horizontal.into())] direction: MaybeSignal<Direction>,
    #[prop(into, optional, default = BorderStyle::Solid.into())] border_style: MaybeSignal<BorderStyle>,
    #[prop(into, optional, default = ContentPosition::Center.into())] content_position: MaybeSignal<ContentPosition>,
    #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView
{
    view! {
        <el-divider
            direction=direction.get().as_str()
            border-style=border_style.get().as_str()
            content-position=content_position.get().as_str()
            >
            {
                match children {
                    Some(children) => children(),
                    None => Fragment::new(vec![]),
                } 
            }
        </el-divider>
    }
}
