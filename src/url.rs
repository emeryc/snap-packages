use bitvec::vec::BitVec;
use gloo_history::History;
use js_sys::Function;
use url::Url;
use wasm_bindgen::prelude::Closure;
use web_sys::{window, Event};

use crate::{compress_bitvec, load, uncompress_bitvec};

pub fn init_storage_listener() {
    let storage_event_callback = Closure::wrap(Box::new(|_event: Event| {
        let hist = gloo_history::BrowserHistory::new();
        hist.push(get_url());
    }) as Box<dyn FnMut(_)>);

    let storage_event_function: Function = storage_event_callback.into_js_value().into();

    window()
        .unwrap()
        .add_event_listener_with_callback("storage", &storage_event_function)
        .unwrap();
}

pub fn get_url() -> String {
    let name: Option<String> = load("name");
    let comment: Option<String> = load("comment");
    let contains: Option<BitVec> = load("collection");

    let mut url = Url::parse(&window().unwrap().location().href().unwrap()).unwrap();
    let mut query_pairs = url.query_pairs_mut();

    if let Some(contains) = contains {
        let compressed_bitvec = compress_bitvec(&contains);
        query_pairs.append_pair("collection", &compressed_bitvec);
    }

    if let Some(name) = name {
        query_pairs.append_pair("name", &name);
    }

    if let Some(comment) = comment {
        query_pairs.append_pair("comment", &comment);
    }
    drop(query_pairs);

    url.to_string()
}

pub fn parse_url() -> (Option<String>, Option<String>, Option<BitVec>) {
    let url = Url::parse(&window().unwrap().location().href().unwrap()).unwrap();
    let query_pairs = url.query_pairs();

    let mut name = None;
    let mut comment = None;
    let mut contains = None;

    for (key, value) in query_pairs {
        match key.as_ref() {
            "name" => name = Some(value.to_string()),
            "comment" => comment = Some(value.to_string()),
            "collection" => contains = Some(uncompress_bitvec(&value)),
            _ => (),
        }
    }

    (name, comment, contains)
}
