use yew::prelude::*;
use sidebar_get_started::SidebarGetStarted;
use get_started_home::GetStartedHome;



fn create_default_state_topic () -> String {
    String::from("Home")
}

fn create_default_state_subtopic () -> String {
    String::from("Home")
}

#[derive(Properties, PartialEq, Debug)]
pub struct GetStartedContentProps {
    #[prop_or_else(create_default_state_topic)]
    pub topic: String,
    #[prop_or_else(create_default_state_subtopic)]
    pub sub_topic: String,
}


pub struct GetStartedContent {
    topic: String,
    sub_topic: String,
}

pub enum Msg {}

impl Component for GetStartedContent {
    type Message = Msg;
    type Properties = GetStartedContentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let GetStartedContentProps {
            topic,
            sub_topic,
        } = ctx.props();

        GetStartedContent {
            topic: topic.to_owned(),
            sub_topic: sub_topic.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>

                <div
                    class="uk-grid-collapse"
                    uk-grid="true"
                >

                    <div class="uk-width-1-4@m td-border-right-light">
                        <div>
                            <SidebarGetStarted/>
                        </div>
                    </div>

                    <div class="uk-width-expand@m">
                        <div
                            class="uk-padding-large"
                        >
                            { self.view_content() }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}


impl GetStartedContent {
    fn view_content (&self) -> Html {
        match self.topic {
            _ => {
                html! {
                    <div>{ "GET STARTED CONTENT" }</div>
                }
            }
        }
    }
}
