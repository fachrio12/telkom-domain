use yew::prelude::*;

pub struct IdentityFundamentals {}

pub enum Msg {}

impl Component for IdentityFundamentals {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IdentityFundamentals {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "IDENTITY FUNDAMENTALS" }</div>
        }
    }
}
