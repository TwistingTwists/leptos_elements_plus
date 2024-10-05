
/// (work in progress)
pub mod el_cascader;
/// Used to show feedback after an activity. 
///
/// Examples:
///```rust
/// view! {
///    <h1 class="text-2xl p-x-4">"Message"</h1>
///    <div class="flex flex-col p-6 space-y-4">
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                Message::new()
///                    .message("This is a text message".to_string())
///                    .message_type(Type::Success)
///                    .show();
///            }
///        >
///            "Success"
///        </button>
///
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                Message::new()
///                    .message("<strong>This is <i>HTML</i> string</strong>".to_string())
///                    .dangerously_use_html_string(true)
///                    .show();
///            }
///        >
///            "HTML"
///        </button>
///
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                Message::new()
///                    .message_element(view!{ <strong>"this is view message"</strong> }.into_view())
///                    .show();
///            }
///        >
///            "View"
///        </button>
///
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                Message::new()
///                    .message_function(|| view!{ <strong>"this is message function"</strong> }.into_view())
///                    .show();
///            }
///        >
///            "Function"
///        </button>
///    </div>
///}
///```
pub mod el_message;
pub mod el_drawer;
pub mod el_table;
pub mod el_table_v2;
pub mod el_autocomplete;
pub mod el_carousel;
pub mod el_tabs;
pub mod el_menu;
pub mod el_select;
pub mod el_select_v2;
pub mod el_input;
pub mod el_input_number;
pub mod el_switch;
pub mod el_slider;
/// (work in progress)
pub mod el_time_picker;
/// (work in progress)
pub mod el_time_select;
pub mod el_date_time_picker;
/// (work in progress)
pub mod el_rate;
/// (work in progress)
pub mod el_transfer;
pub mod el_alert;
pub mod el_avatar;
pub mod el_backtop;
pub mod el_divider;
/// (work in progress)
pub mod el_image;
///A set of modal boxes simulating system message box, mainly for alerting information, confirm operations and prompting messages.
///
/// Examples:
///```rust
///
///MessageBox::new()
///    .confirm_button_text("confirm".to_string())
///    .callback(|_value,action| {
///        Message::new()
///            .message(format!("action: {:?}", action))
///            .show()
///    })
///    .alert("This is a message", "Title");
///
///MessageBox::new()
///    .confirm_button_text("confirm".to_string())
///    .callback(|value,action| {
///        Message::new()
///            .message(format!("value: {}, action: {:?}", value.unwrap_or("None".to_string()), action))
///            .show()
///    })
///    .message_type(Type::Warning)
///    .prompt("proxy will permanently delete the file. Continue?", "Warning");
///
///MessageBox::new()
///    .before_close(|action| {
///        Message::new()
///            .message(format!("before close,action: {:?}", action))
///            .show()
///    })
///    .confirm_button_text("OK".to_string())
///    .cancel_button_text("Cancel".to_string())
///    .message_type(Type::Warning)
///    .prompt_then_catch("proxy will permanently delete the file. Continue?", "Warning",
///        Some(|value:Option<String>,action|{
///            Message::new()
///                .message_type(Type::Success)
///                .message(format!("Delete completed, value: {}, action: {:?}", value.unwrap_or("None".to_string()),action))
///                .show()
///        }),
///        Some(|action|{
///            Message::new()
///                .message_type(Type::Info)
///                .message(format!("Delete canceled, action: {:?}", action))
///                .show()
///        }));
///
///```
pub mod el_message_box;
/// Displays a global notification message at a corner of the page.
///
/// Examples:
///```rust
///
///let mut notification = Notification::new();
///notification.message("This notification will never close automatically. You can click the close button to close it.".to_string())
///    .duration(0)
///    .message_type(Type::Success);
///
///let show = Arc::new(Mutex::new(notification));
///let close = Arc::clone(&show);
///
///view! {
///    <h1 class="text-2xl p-x-4">"Notification"</h1>
///    <div class="flex flex-col p-6 space-y-4">
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                Notification::new()
///                    .message("This is a text message".to_string())
///                    .message_type(Type::Success)
///                    .show();
///            }
///        >
///            "Success"
///        </button>
///
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                show.lock().unwrap().show();
///            }
///        >
///            "show"
///        </button>
///
///        <button
///            type="button"
///            class="el-button el-button--primary w-32"
///            on:click=move |_|  {
///                close.lock().unwrap().close();
///            }
///        >
///            "close"
///        </button>
///
///    </div>
///}
///```
pub mod el_notification;
/// (work in progress)
pub mod el_popover;
pub mod el_progress;
pub mod el_statistic;
/// (work in progress)
pub mod el_tooltip;
/// (work in progress)
pub mod el_tree;
/// (work in progress)
pub mod el_tree_v2;
/// (work in progress)
pub mod el_tree_select;
pub mod el_pagination;
/// (work in progress)
pub mod el_tag;

//pub mod el_radio;
//pub mod el_checkbox;

pub mod common;
mod utils;
