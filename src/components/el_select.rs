
use leptos::*;
//use leptos::logging::log;
use serde_wasm_bindgen::from_value;

#[component]
pub fn Select(
    #[prop(into)] value: RwSignal<String>,
    children: Children
    ) -> impl IntoView
{
    view! {
        <el-select
            value=value
            on:el-change=move |event: leptos::ev::CustomEvent| {
                if let Ok(v) = from_value::<String>(event.detail()) {
                    value.set(v);
                }
            }
        >

            {children()}
        </el-select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(into, optional)] label: Option<AttributeValue>,
    #[prop(into, optional)] value: Option<AttributeValue>,
    #[prop(into, optional)] disabled: bool,
    ) -> impl IntoView
{
    view! { <el-option label=label value=value disabled=disabled></el-option> }
}
