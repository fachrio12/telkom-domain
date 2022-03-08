use yew::prelude::*;

pub struct Home {}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="uk-background-muted">
                <div class="uk-container" style="padding-bottom: 800px;">
                    <form
                        class="uk-margin-large-top"
                    >
                        
                        <div class="uk-margin">
                            <div
                                class="uk-inline uk-width-1-1 uk-border-circle"
                            >
                                <span class="uk-form-icon" uk-icon="icon: user"></span>
                                <input
                                    class="uk-input uk-form-large"
                                    type="text"
                                    placeholder="Input"
                                />
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}
