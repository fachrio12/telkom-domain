use yew::prelude::*;
use yew_router::prelude::*;
use logo_icon::LogoIcon;
use route::Route;

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
                <ul
                    class="uk-nav uk-nav-default uk-margin-medium-top"
                    style="opacity: .8;"
                >
                    // <li class="uk-active"><a href="#">{ "Active" }</a></li>
                    <li><a href="#" class="uk-padding-remove-top" style="margin-bottom: -5px;">
                        // <i
                        //     class="fa-solid fa-bolt uk-text-primary uk-margin-small-right uk-text-default"
                        // >
                        // </i>
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 5px;"
                        >
                            <LogoIcon width=33 />
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Home" }
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-bolt uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            <Link<Route> to={Route::GetStartedHome}
                                classes="uk-text-emphasis"
                            >
                                { "Get Started" }
                            </Link<Route>>
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-unlock uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Authenticate" }
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-user-gear uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Manage Users" }
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-sliders uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Customize" }
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-shield uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Secure" }
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-square-poll-vertical uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Deploy and Monitor" }
                        </span>
                    </a></li>
                    <li><a href="#" class="uk-padding-remove-top">
                        <span
                            class="uk-text-center"
                            style="width: 45px; margin-top: 3px;"
                        >
                            <i
                                class="fa-solid fa-life-ring uk-text-primary uk-text-default"
                            >
                            </i>
                        </span>
                        <span
                            class="uk-text-secondary td-text-size-big td-text-weight-bold"
                        >
                            { "Troubleshoot" }
                        </span>
                    </a></li>
                </ul>
            </div>
        }
    }
}
