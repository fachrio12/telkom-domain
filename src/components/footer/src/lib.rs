use yew::prelude::*;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav
                class="uk-navbar-container"
                uk-navbar="true"
                style="border-top: .5px solid rgba(0,0,0,.2);"
            >

                <div class="uk-navbar-left uk-text-muted">
                    <ul class="uk-navbar-nav">
                        <li class="uk-navbar-item" style="font-size: 17px;">
                            { "© 2022 Telkom Domain® Inc. All Rights Reserved." }
                        </li>
                    </ul>
                </div>

                <div class="uk-navbar-right">

                    <ul class="uk-navbar-nav">
                        <li><a href="#">
                            { "Status" }
                        </a></li>
                        <span
                            class="uk-align-center"
                        >
                            { "•" }
                        </span>
                        <li><a href="#">
                            { "Legal" }
                        </a></li>
                        <span
                            class="uk-align-center"
                        >
                            { "•" }
                        </span>
                        <li><a href="#">
                            { "Privacy" }
                        </a></li>
                        <span
                            class="uk-align-center"
                        >
                            { "•" }
                        </span>
                        <li><a href="#">
                            { "Terms" }
                        </a></li>
                    </ul>

                </div>

            </nav>
        }
    }
}
