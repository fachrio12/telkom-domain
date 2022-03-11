use yew::prelude::*;
use sidebar_articles::SidebarArticles;

pub struct ArticlesHome {}

pub enum Msg {}

impl Component for ArticlesHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ArticlesHome {}
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
                    <div class="uk-width-1-4@m">
                        <div
                            class="td-border-right-light"
                        >
                            <SidebarArticles/>
                        </div>
                    </div>
                    <div class="uk-width-expand@m">
                        <div
                            // class="uk-card uk-card-default uk-card-body"
                        >
                            { "Expand" }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
