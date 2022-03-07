use yew::prelude::*;
use yew::{
    Component, Context, html, Html, NodeRef,
};
use yew_router::prelude::*;

use router::{
    Route,
    switch,
};
use navbar::Navbar;


pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Navbar/>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}
