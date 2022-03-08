use yew::prelude::*;
// use yew::{
//     Component, Context, html, Html, NodeRef,
// };
use yew_router::prelude::*;

use router::{
    Route,
    switch,
};
use navtop::Navtop;


pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Navtop/>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}
