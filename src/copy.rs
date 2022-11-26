use web_sys::MouseEvent;
use yew::{function_component, html, use_callback, use_state, Html};

use crate::url;

#[function_component]
pub fn CopyButton() -> Html {
    // When the button is clicked, turn into a text field and copy the text.
    let url_state = use_state(|| "".to_string());

    let callback = {
        let url_state = url_state.clone();

        use_callback(
            move |event: MouseEvent, _| {
                event.prevent_default();
                let url = url::get_url();
                copy(&url);
                url_state.set(url);
            },
            (),
        )
    };

    if url_state.is_empty() {
        html! {
            <div class="share_area">
                <button class="share" onclick={callback}>{"Copy Share Link"}</button>
            </div>
        }
    } else {
        html! {
            <div class="share_area">
                <div class="share_url">{&*url_state}</div>
                <button class="share" onclick={callback}>{"Copy Share Link"}</button>
            </div>
        }
    }
}

#[cfg(web_sys_unstable_apis)]
fn copy(text: &str) {
    use web_sys::window;

    let _ = window()
        .unwrap()
        .navigator()
        .clipboard()
        .unwrap()
        .write_text(text);
}

#[cfg(not(web_sys_unstable_apis))]
fn copy(_text: &str) {}
