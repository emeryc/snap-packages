use itertools::Itertools;
use snap_packages::{
    all_cards,
    collection::*,
    copy::CopyButton,
    load, store,
    text::TextArea,
    url::{init_storage_listener, parse_url},
    Card, Pool,
};
use std::collections::HashMap;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let cards = all_cards();
    let contains = use_reducer(Collection::new);
    init_storage_listener();

    let pools: HashMap<Pool, Vec<Html>> = cards
        .into_iter()
        .map(|card| {
            let contains = contains.clone();
            let checked = contains.get(card.id);
            (card.pool, html! {<CardComp card={card} checked={checked.unwrap_or_default()} check={Callback::from(move |card_id: usize| {
                contains.dispatch(CollectionAction::Toggle(card_id));
            })} />})
        })
        .into_group_map();

    //let pool1 = pools.get(&Pool::Pool1).unwrap().to_owned();
    //let pool2 = pools.get(&Pool::Pool2).unwrap().to_owned();
    let pool3 = pools.get(&Pool::Pool3).unwrap().to_owned();
    let pool4 = pools.get(&Pool::Pool4).unwrap().to_owned();
    let pool5 = pools.get(&Pool::Pool5).unwrap().to_owned();

    html! {
        <div class="container">
        <div class="pools">
        <PoolComp pool_name={"Pool 3"}>
            <PoolGrid>
                {pool3}
            </PoolGrid>
        </PoolComp>
        <PoolComp pool_name={"Pool 4"}>
            <PoolGrid>
                {pool4}
            </PoolGrid>
        </PoolComp>
        <PoolComp pool_name={"Pool 5"}>
            <PoolGrid>
                {pool5}
            </PoolGrid>
        </PoolComp>
        </div>
        <div class="comments">
            <Name />
            <Comment />
            <CopyButton />
        </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct PoolProps {
    pool_name: String,
    children: Children,
}

#[function_component]
pub fn PoolComp(props: &PoolProps) -> Html {
    html! {
        <div class="pool">
          <h2>{props.pool_name.clone()}</h2>
          { props.children.clone() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct EvenGridProps {
    children: Children,
}

#[function_component]
pub fn PoolGrid(props: &EvenGridProps) -> Html {
    html! {
        <div class="grid">
          { props.children.clone() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CardProps {
    card: Card,
    checked: bool,
    check: Callback<usize>,
}

#[function_component]
pub fn CardComp(props: &CardProps) -> Html {
    let check = props.check.clone();
    let id = props.card.id;
    html! {
        <div class="card">
            <label>
                <input type="checkbox"
                checked={props.checked}
                onchange={Callback::from(move |_| check.emit(id))}/>
            {props.card.name}
        </label>
        </div>
    }
}

#[function_component]
pub fn Name() -> Html {
    let name: String = parse_url().0.or_else(|| load("name")).unwrap_or_default();

    let update_name = use_callback(
        |name: String, _| {
            store("name", &name);
        },
        (),
    );

    html! {
        <div class="name">
                {"Name: "}
                <TextArea id="name" content={name} callback={update_name}/>

        </div>
    }
}

#[function_component]
pub fn Comment() -> Html {
    let comment: String = parse_url()
        .1
        .or_else(|| load("comment"))
        .unwrap_or_default();

    let update_comment = use_callback(
        |comment: String, _| {
            store("comment", &comment);
        },
        (),
    );

    html! {
        <div class="comment">
                {"Comments: "}
                <TextArea id="comment" content={comment} callback={update_comment}/>

        </div>
    }
}
