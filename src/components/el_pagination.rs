use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue/*, closure::Closure*/};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

#[component]
pub fn Pagination(
    #[prop(into, optional)] small: Option<AttributeValue>,
    #[prop(into, optional)] background: Option<AttributeValue>,
    #[prop(into, optional)] default_page_size: Option<AttributeValue>,
    #[prop(into, optional)] total: Option<AttributeValue>,
    #[prop(into, optional)] page_count: Option<AttributeValue>,
    #[prop(into, optional)] pager_count: Option<AttributeValue>,
    #[prop(into, optional)] default_current_page: Option<AttributeValue>,
    #[prop(into, optional)] layout: Option<AttributeValue>,
    #[prop(into, optional)] popper_class: Option<AttributeValue>,
    #[prop(into, optional)] prev_text: Option<AttributeValue>,
    #[prop(into, optional)] prev_icon: Option<AttributeValue>,
    #[prop(into, optional)] next_text: Option<AttributeValue>,
    #[prop(into, optional)] next_icon: Option<AttributeValue>,
    #[prop(into, optional)] disabled: Option<AttributeValue>,
    #[prop(into, optional)] teleported: Option<AttributeValue>,
    #[prop(into, optional)] hide_on_single_page: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] page_sizes: MaybeSignal<Vec<i32>>,
    #[prop(into, optional)] current_page: Option<RwSignal<i32>>,
    #[prop(into, optional)] page_size: Option<RwSignal<i32>>,
    ) -> impl IntoView
{
    view! {
        <el-pagination
            small=small
            background=background
            default-page-size=default_page_size
            total=total
            page-count=page_count
            pager-count=pager_count
            default-current-page=default_current_page
            layout=layout
            popper-class=popper_class
            prev-text=prev_text
            prev-icon=prev_icon
            next-text=next_text
            next-icon=next_icon
            disabled=disabled
            teleported=teleported
            hide-on-single-page=hide_on_single_page
            class=class
            prop:pageSize=move||{
                if let Some(page_size) = page_size {
                    JsValue::from(page_size.get())
                } else {
                    JsValue::UNDEFINED
                }
            }
            prop:pageSizes=move||{
                if let Ok(data) = serde_wasm_bindgen::to_value(&page_sizes) {
                    data
                } else {
                    log!("Pagination: page_sizes: serde_wasm_bindgen::to_value failed");
                    JsValue::UNDEFINED
                }
            }
            prop:currentPage=move||{
                if let Some(current_page) = current_page {
                    JsValue::from(current_page.get())
                } else {
                    JsValue::UNDEFINED
                }
            }
            on:ce-update:current-page=move|ev: leptos::ev::CustomEvent|{
                if let Some(current_page) = current_page {
                    let data= Array::from(&ev.detail());
                    if data != JsValue::UNDEFINED.into() && data.length() != 0 {
                        let data = data.get(0);
                        if let Ok(data) = serde_wasm_bindgen::from_value::<i32>(data) {
                            current_page.set(data);
                        } else {
                            log!("Pagination: current_page: serde_wasm_bindgen::from_value failed");
                        }
                    }
                }
            }
            on:ce-update:page-size=move|ev: leptos::ev::CustomEvent|{
                if let Some(page_size) = page_size {
                    let data= Array::from(&ev.detail());
                    if data != JsValue::UNDEFINED.into() && data.length() != 0 {
                        let data = data.get(0);
                        if let Ok(data) = serde_wasm_bindgen::from_value::<i32>(data) {
                            page_size.set(data);
                        } else {
                            log!("Pagination: page_size: serde_wasm_bindgen::from_value failed");
                        }
                    }
                }
            }
            >
        </el-pagination>
    }
}
