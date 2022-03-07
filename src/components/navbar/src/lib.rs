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
            <nav
                class="uk-navbar-container uk-navbar"
                // uk-navbar={true}
            >
                <div class="uk-navbar-left">

                    <ul class="uk-navbar-nav">
                        <li class="uk-navbar-item uk-logo">
                            <span class="uk-text-bold uk-margin-small-right">
                                { "Telkom Domain" }
                            </span>
                            {"docs"}
                        </li>
                    </ul>
            
                </div>

                <div class="uk-navbar-right">

                    // <a class="uk-navbar-item uk-logo" href="#">{ "LOGO" }</a>

                    <ul class="uk-navbar-nav">
                        <li>
                            <a href="#">
                                <span class="uk-icon uk-margin-small-right" uk-icon="icon: star"></span>
                                { "Features" }
                            </a>
                        </li>
                    </ul>

                    <div class="uk-navbar-item">
                        <div>{ "Some " }<a href="#">{ "Link" }</a></div>
                    </div>

                    <div class="uk-navbar-item">
                        <form action="javascript:void(0)">
                            <input class="uk-input uk-form-width-small" type="text" placeholder="Input" />
                            <button class="uk-button uk-button-default">{ "Button" }</button>
                        </form>
                    </div>

                </div>
            </nav>
        }
    }
}