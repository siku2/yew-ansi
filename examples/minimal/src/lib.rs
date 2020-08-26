use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_ansi::Ansi;

const OUTPUT: &str = include_str!("output.txt");

pub struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Ansi text=OUTPUT.to_owned() />
        }
    }
}

#[wasm_bindgen(start)]
pub fn start_app() {
    yew::start_app::<Model>();
}
