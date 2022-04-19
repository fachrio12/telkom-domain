use yew::prelude::*;
use get_started_topics::{
    Topic,
    SubTopic,
    SubTopic2,
};
use sidebar_get_started::SidebarGetStarted;

use articles_breadcrumb::ArticlesBreadcrumb;
use get_started_content::GetStartedContent;



fn create_default_state_topic () -> Topic {
    Topic::Home
}

fn create_default_state_subtopic () -> SubTopic {
    SubTopic::Home
}

#[derive(Properties, PartialEq, Debug)]
pub struct GetStartedMainProps {
    #[prop_or_else(create_default_state_topic)]
    pub topic: Topic,
    #[prop_or_else(create_default_state_subtopic)]
    pub sub_topic: SubTopic,
    #[prop_or(SubTopic2::Home)]
    pub sub_topic_2: SubTopic2,
}


pub struct GetStartedMain {
    topic: Topic,
    sub_topic: SubTopic,
    sub_topic_2: SubTopic2,
}

pub enum Msg {}

impl Component for GetStartedMain {
    type Message = Msg;
    type Properties = GetStartedMainProps;

    fn create(ctx: &Context<Self>) -> Self {
        let GetStartedMainProps {
            topic,
            sub_topic,
            sub_topic_2,
        } = ctx.props();

        GetStartedMain {
            topic: topic.to_owned(),
            sub_topic: sub_topic.to_owned(),
            sub_topic_2: sub_topic_2.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.topic != ctx.props().topic || self.sub_topic != ctx.props().sub_topic {
            self.topic = ctx.props().topic.to_owned();
            self.sub_topic = ctx.props().sub_topic.to_owned();
            self.sub_topic_2 = ctx.props().sub_topic_2.to_owned();
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
                    style="min-height: 500px;"
                >

                    <div class="uk-width-1-4@m td-border-right-light">
                        <div class="uk-position-fixed uk-width-1-4@m">
                            <SidebarGetStarted topic={ self.topic.to_owned() } sub_topic={ self.sub_topic.to_owned() } sub_topic_2={ self.sub_topic_2.to_owned() } />
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
        log::info!("get started main, sub topic ====== {:?}", self.sub_topic);
        match self.topic {
            Topic::Home => {
                html! {
                    <>
                        <ArticlesBreadcrumb topic={ self.topic.to_owned() } sub_topic={ self.sub_topic.to_owned() } sub_topic_2={ self.sub_topic_2.to_owned() } />
                        <GetStartedContent/>
                    </>
                }
            }
            Topic::IdentityFundamentals => {
                html! {
                    <>
                        <ArticlesBreadcrumb topic={ self.topic.to_owned() } sub_topic={ self.sub_topic.to_owned() } sub_topic_2={ self.sub_topic_2.to_owned() } />
                        <GetStartedContent topic={ self.topic.to_owned() } sub_topic={ self.sub_topic.to_owned() } sub_topic_2={ self.sub_topic_2.to_owned() } />
                    </>
                }
            }
            Topic::DomainOverview => {
                html! {
                    <>
                        <ArticlesBreadcrumb topic={ self.topic.to_owned() } sub_topic={ self.sub_topic.to_owned() } sub_topic_2={ self.sub_topic_2.to_owned() } />
                        <GetStartedContent topic={ self.topic.to_owned() } sub_topic={ self.sub_topic.to_owned() } sub_topic_2={ self.sub_topic_2.to_owned() } />
                    </>
                }
            }
        }
    }
}
