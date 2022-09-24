use yew::prelude::*;


#[derive(Properties, PartialEq, Debug)]
pub struct AlertProps {
    #[prop_or(String::from("This box has no messages"))]
    pub message: String,
}

pub struct Alert2 {
    message: String,
}

pub enum Msg {}


impl Component for Alert2 {
    type Message = Msg;
    type Properties = AlertProps;

    fn create(ctx: &Context<Self>) -> Self {
        Alert2 {
            message: ctx.props().message.to_owned(),
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
            <div
                class="uk-alert"
                style="border-radius: 5px; position: relative; overflow: hidden; margin-bottom: 16px;"
            >
                <div uk-grid="true">
                    <div class="uk-width-auto" style="padding-left: 45px;">
                        <span uk-icon="icon: warning; ratio: 1.5;"></span>
                    </div>
                    <div class="uk-width-expand" style="padding-left: 25px;">
                        { self.message.to_owned() }
                    </div>
                </div>
                <div
                    style="
                        position: absolute;
                        left: 0;
                        top: 0;
                        background: yellow;
                        width: 5px;
                        height: 100%;
                    "
                />
            </div>
        }
    }
}
