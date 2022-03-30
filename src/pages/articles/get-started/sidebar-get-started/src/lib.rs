use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SidebarGetStartedState {
    Articles,
    GetStarted,
}

fn create_default_state() -> SidebarGetStartedState {
    SidebarGetStartedState::GetStarted
}

#[derive(Properties, PartialEq)]
pub struct SidebarGetStartedProps {
    /// The link must have a target.
    #[prop_or_else(create_default_state)]
    state: SidebarGetStartedState,
    
}

pub struct SidebarGetStarted {}

pub enum Msg {}

impl Component for SidebarGetStarted {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // log::info!("asdfasdfasdf");

        SidebarGetStarted {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="uk-padding-small">
                <div class="uk-text-primary">
                    <span uk-icon="icon: arrow-left"></span>
                    <span
                        class="td-text-weight-bold"
                    >
                        { "Main Menu" }
                    </span>
                </div>

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


            </div>
        }
    }
}
