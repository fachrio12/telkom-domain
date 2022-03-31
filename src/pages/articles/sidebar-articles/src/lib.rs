use yew::prelude::*;
use sidebar_main_menu::SidebarMainMenu;

pub struct SidebarArticles {}

pub enum Msg {}

impl Component for SidebarArticles {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SidebarArticles {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="uk-width-1-2@s uk-width-2-5@m uk-padding-small">
                <SidebarMainMenu/>
            </div>
        }
    }
}
