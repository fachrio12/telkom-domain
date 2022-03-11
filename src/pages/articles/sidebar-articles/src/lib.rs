use yew::prelude::*;
use logo_icon::LogoIcon;

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
            <div class="uk-width-1-2@s uk-width-2-5@m uk-padding">
                <ul class="uk-nav uk-nav-default uk-margin-medium-top">
                    // <li class="uk-active"><a href="#">{ "Active" }</a></li>
                    <li><a href="#">
                        // <i
                        //     class="fa-solid fa-bolt uk-text-primary uk-margin-small-right uk-text-default"
                        // >
                        // </i>
                        <LogoIcon width=45 />
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Home" }
                        </span>
                    </a></li>
                    <li><a href="#">
                        <i
                            class="fa-solid fa-bolt uk-text-primary uk-margin-small-right uk-text-default"
                        >
                        </i>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Get Started" }
                        </span>
                    </a></li>
                    <li><a href="#">
                        <i
                            class="fa-solid fa-unlock uk-text-primary uk-margin-small-right uk-text-default"
                        >
                        </i>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Authenticate" }
                        </span>
                    </a></li>
                    <li><a href="#">
                        <i
                            class="fa-solid fa-user-gear uk-text-primary uk-margin-small-right uk-text-default"
                        >
                        </i>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Manage Users" }
                        </span>
                    </a></li>
                </ul>
            </div>
        }
    }
}
