use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
pub fn TextArea(props: &TextAreaProps) -> Html {
    let id = props.id.clone();
    let callback = props.callback.clone();
    let onchange = Callback::from(move |event| {
        let callback = callback.clone();
        text_change(event, callback)
    });

    html! {
        <div onkeyup={onchange}
             contenteditable="true"
             id={id.clone()}>
            {&props.content}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TextAreaProps {
    pub id: String,
    pub content: String,
    pub callback: Callback<String>,
}

fn text_change(event: KeyboardEvent, callback: Callback<String>) {
    let target = event.target().unwrap();
    let target = target.unchecked_into::<HtmlElement>();
    let text = target.inner_text();
    callback.emit(text);
}
