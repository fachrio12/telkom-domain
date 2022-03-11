use yew::prelude::*;
use yew_router::prelude::*;
use logo::Logo;
use router::Route;

pub struct Navtop {}

pub enum Msg {}

impl Component for Navtop {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navtop {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let history = use_history().unwrap();

        // let onclick = Callback::once(move |_| history.push(Route::Secure));

        html! {
            <div uk-sticky="sel-target: .uk-navbar-container; cls-active: uk-navbar-sticky">
                <nav
                    class="uk-navbar-container uk-navbar"
                    style="border-bottom: .5px solid rgba(0,0,0,.2);"
                    // uk-navbar={true}
                >
                    <div class="uk-navbar-left">

                        <ul class="uk-navbar-nav">
                            <li class="uk-navbar-item uk-logo">
                                <Logo width=45/>
                                <span class="uk-text-bold uk-margin-small-right">
                                    { "Telkom Domain" }
                                </span>
                                {"docs"}
                            </li>
                        </ul>
                
                    </div>

                    <div class="uk-navbar-right">

                        <ul class="uk-navbar-nav">
                            <li class="uk-active">
                                <Link<Route> to={Route::Articles}>
                                    { "Articles" }
                                </Link<Route>>
                                // <a
                                //     href="javascript:void(0)"
                                //     // {onclick}
                                // >
                                //     { "Articles" }
                                // </a>
                            </li>
                            <li>
                                <a href="#">{ "Quickstart" }</a>
                            </li>
                            <li>
                                <a href="#">{ "Domain APIs" }</a>
                            </li>
                        </ul>

                        // <ul class="uk-navbar-nav">
                        //     <li>
                        //         <a href="#">
                        //             <span class="uk-icon uk-margin-small-right" uk-icon="icon: star"></span>
                        //             { "Features" }
                        //         </a>
                        //     </li>
                        // </ul>

                        // <div class="uk-navbar-item">
                        //     <div>{ "Some " }<a href="#">{ "Link" }</a></div>
                        // </div>

                        // <div class="uk-navbar-item">
                        //     <form action="javascript:void(0)">
                        //         <input class="uk-input uk-form-width-small" type="text" placeholder="Input" />
                        //         <button class="uk-button uk-button-default">{ "Button" }</button>
                        //     </form>
                        // </div>

                    </div>
                </nav>
            </div>
        }
    }
}