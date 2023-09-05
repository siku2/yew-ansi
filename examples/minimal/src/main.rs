use yew::{html, Component, Context, Html};
use yew_ansi::AnsiStatic;

const OUTPUT: &str = include_str!("../../../assets/cargo-expand.txt");

pub struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <AnsiStatic text={ OUTPUT } />
        }
    }
}

pub fn main() {
    yew::Renderer::<Model>::new().render();
}
