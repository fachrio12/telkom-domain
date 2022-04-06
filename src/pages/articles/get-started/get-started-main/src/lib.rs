use yew::prelude::*;
use sidebar_get_started::SidebarGetStarted;

use articles_breadcrumb::ArticlesBreadcrumb;
use get_started_content::GetStartedContent;



fn create_default_state_topic () -> String {
    String::from("Home")
}

fn create_default_state_subtopic () -> String {
    String::from("Home")
}

#[derive(Properties, PartialEq, Debug)]
pub struct GetStartedMainProps {
    #[prop_or_else(create_default_state_topic)]
    pub topic: String,
    #[prop_or_else(create_default_state_subtopic)]
    pub sub_topic: String,
}


pub struct GetStartedMain {
    topic: String,
    sub_topic: String,
}

pub enum Msg {}

impl Component for GetStartedMain {
    type Message = Msg;
    type Properties = GetStartedMainProps;

    fn create(ctx: &Context<Self>) -> Self {
        let GetStartedMainProps {
            topic,
            sub_topic,
        } = ctx.props();

        GetStartedMain {
            topic: topic.to_owned(),
            sub_topic: sub_topic.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.topic != ctx.props().topic {
            self.topic = ctx.props().topic.to_owned();
            true
        } else {
            false
        }
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
                            <SidebarGetStarted topic={ self.topic.to_owned() } />
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


impl GetStartedMain {
    fn view_content (&self) -> Html {
        match self.topic.as_str() {
            "Home" => {
                html! {
                    <>
                        <ArticlesBreadcrumb/>
                        <GetStartedContent/>
                    </>
                }
            }
            "Identity Fundamentals" => {
                html! {
                    <>
                        <ArticlesBreadcrumb/>
                        <GetStartedContent topic={ self.topic.to_owned() } />
                    </>
                }
            }
            _ => {
                html! {
                    <>
                        <ArticlesBreadcrumb/>
                        <GetStartedContent/>
                    </>
                }
            }
        }
    }
}
