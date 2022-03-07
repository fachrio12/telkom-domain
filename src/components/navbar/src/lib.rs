use yew::prelude::*;

pub struct Navbar {}

pub enum Msg {}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navbar {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "NAVBAR" }</div>
        }
    }
}