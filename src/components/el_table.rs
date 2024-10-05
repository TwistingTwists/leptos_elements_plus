
use leptos::*;
use leptos::logging::log;

use serde::Serialize;
use wasm_bindgen::JsValue;

#[component]
pub fn Table<T>(
    #[prop(into)] data: MaybeSignal<Vec<T>>,
    #[prop(into, optional)] stripe: bool,
    #[prop(into, optional)] border: bool,
    #[prop(into, optional)] fixed: bool,
    #[prop(into, optional)] width: Option<AttributeValue>,
    #[prop(into, optional)] height: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
    where T: Serialize + 'static + Clone + std::fmt::Debug
{
    view! {
        <el-table
            prop:data=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&data) {
                    data
                } else {
                    log!("TableV2: data is not serializable");
                    JsValue::UNDEFINED
                }
            }

            width=width
            height=height
            stripe=stripe
            border=border
            fixed=fixed
        >
            {children()}
        </el-table>
    }
}

#[component]
pub fn TableColumn(
    #[prop(into)] prop: MaybeSignal<String>,
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(into, optional)] width: Option<AttributeValue>,
    ) -> impl IntoView
{
    view! { <el-table-column prop=prop label=label width=width></el-table-column> }
}
