use yew::prelude::*;
use sidebar_main_menu::SidebarMainMenu;

#[derive(Clone, PartialEq, Debug)]
pub enum SidebarGetStartedState {
    MainMenu,
    GetStarted,
}

fn create_default_state() -> SidebarGetStartedState {
    SidebarGetStartedState::GetStarted
}

#[derive(Properties, PartialEq, Debug)]
pub struct SidebarGetStartedProps {
    /// The link must have a target.
    #[prop_or_else(create_default_state)]
    state: SidebarGetStartedState,
    
}

pub struct SidebarGetStarted {
    state: SidebarGetStartedState,
}

pub enum Msg {
    UpdateState(SidebarGetStartedState),
}

impl Component for SidebarGetStarted {
    type Message = Msg;
    type Properties = SidebarGetStartedProps;

    fn create(ctx: &Context<Self>) -> Self {
        // log::info!("asdfasdfasdf");
        log::info!("This is log from sidebar get started");
        let data_test = ctx.props();
        log::info!("sidebar state ====== {:?}", data_test);

        SidebarGetStarted {
            state: ctx.props().state.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateState(state) => {
                self.state = state;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("component state === {:?}", self.state);
        let is_main_menu = if self.state == SidebarGetStartedState::MainMenu {true} else {false};
        html! {
            <div class="uk-padding-small">
                {
                    if is_main_menu {
                        html! {
                            <div
                                class="uk-text-primary"
                                style="cursor: pointer;"
                                onclick={ ctx.link().callback(|_| Msg::UpdateState(SidebarGetStartedState::GetStarted)) }
                            >
                                <span
                                    class="td-text-weight-bold"
                                >
                                    { "Back" }
                                </span>
                                <span uk-icon="icon: arrow-right"></span>
                            </div>
                        }
                    } else {
                        html! {
                            <div
                                class="uk-text-primary"
                                style="cursor: pointer;"
                                onclick={ ctx.link().callback(|_| Msg::UpdateState(SidebarGetStartedState::MainMenu)) }
                            >
                                <span uk-icon="icon: arrow-left"></span>
                                <span
                                    class="td-text-weight-bold"
                                >
                                    { "Main Menu" }
                                </span>
                            </div>
                        }
                    }
                }

                {
                    if is_main_menu {
                        html! {
                            <SidebarMainMenu/>
                        }
                    } else {
                        self.view_get_started_menu()
                    }
                }


            </div>
        }
    }
}


impl SidebarGetStarted {
    fn view_get_started_menu (&self) -> Html {
        html! {
            <>
                <ul
                    class="uk-nav uk-nav-default uk-margin-medium-top uk-margin-medium-bottom"
                    style="opacity: .8;"
                >
                    
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
                            { "Get Started" }
                        </span>
                    </a></li>
                </ul>

                <div
                    class="uk-margin-medium-bottom uk-text-small td-text-weight-400"
                >
                    <div
                        class="uk-margin-small-bottom"
                        style="text-transform: uppercase;"
                    >
                        { "Start building" }
                    </div>
                    <div
                        class="uk-text-muted"
                    >
                        <div>
                            { "Quickstart" }
                            <span uk-icon="icon: arrow-right"></span>
                        </div>
                    </div>
                </div>

                <div
                    class="uk-margin-medium-bottom uk-text-small td-text-weight-400"
                >
                    <div
                        class="uk-margin-small-bottom"
                        style="text-transform: uppercase;"
                    >
                        { "Learn the basic" }
                    </div>
                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                    >
                        <div>
                            { "Identity Fundamentals" }
                            <span
                                style="float: right; padding-top: 4px;"
                                uk-icon="icon: chevron-right;  ratio: 0.85"></span>
                        </div>
                    </div>
                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                    >
                        <div>
                            { "Telkom Domain Overview" }
                            <span
                                style="float: right; padding-top: 4px;"
                                uk-icon="icon: chevron-right;  ratio: 0.85"></span>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}