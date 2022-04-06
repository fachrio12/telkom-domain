use yew::prelude::*;



#[derive(Properties, PartialEq, Debug)]
pub struct ArticlesBreadcrumbProps {
    #[prop_or("Home".to_string())]
    pub topic: String,
}

pub struct ArticlesBreadcrumb {
    topic: String,
}

pub enum Msg {}

impl Component for ArticlesBreadcrumb {
    type Message = Msg;
    type Properties = ArticlesBreadcrumbProps;

    fn create(ctx: &Context<Self>) -> Self {
        // log::info!("This is log from articles breadcrumb");
        ArticlesBreadcrumb {
            topic: ctx.props().topic.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.topic.as_str() {
            "Home" => html! {
                <ul class="uk-breadcrumb">
                    <li><a href="#">{ "Docs" }</a></li>
                    <li><span>{ "Get Started" }</span></li>
                </ul>
            },
            _ => html! {}
        }
    }
}
