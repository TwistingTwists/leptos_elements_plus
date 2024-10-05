
use leptos::*;
//use leptos::logging::log;

//use serde::{de::DeserializeOwned, Serialize};
//use wasm_bindgen::JsValue;
//use serde_wasm_bindgen::from_value;

#[component]
pub fn Carousel(
    #[prop(into, optional)] width: Option<AttributeValue>,
    #[prop(into, optional)] height: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{
    view! {
        <el-carousel height=height width=width>
            {children()}
        </el-carousel>
    }
}

#[component]
pub fn CarouselItem(
    children: Children
    ) -> impl IntoView
{
    view! { <el-carousel-item>{children()}</el-carousel-item> }
}
