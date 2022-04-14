use yew::prelude::*;

pub struct TelkomDomainOverviewHome {}

pub enum Msg {}

impl Component for TelkomDomainOverviewHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TelkomDomainOverviewHome {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "Telkom Domain Overview" }</div>
        }
    }
}
