use yew::prelude::*;


#[derive(Properties, PartialEq, Debug)]
pub struct AlertProps {
    #[prop_or(String::from("This box has no messages"))]
    pub message: String,
}

pub struct Alert {
    message: String,
}

pub enum Msg {}


impl Component for Alert {
    type Message = Msg;
    type Properties = AlertProps;

    fn create(ctx: &Context<Self>) -> Self {
        Alert {
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
                style="border-radius: 5px; position: relative; overflow: hidden;"
            >
                <div uk-grid="true">
                    <div class="uk-width-auto" style="padding-left: 45px;">
                        // <i
                        //     class="fa-solid fa-memo uk-text-default"
                        // >
                        // </i>
                        <span uk-icon="icon: file-text; ratio: 1.5;"></span>
                    </div>
                    <div class="uk-width-expand">
                        { self.message.to_owned() }
                    </div>
                </div>
                <div
                    style="
                        position: absolute;
                        left: 0;
                        top: 0;
                        background: rgba(0,0,0,.5);
                        width: 5px;
                        height: 100%;
                    "
                />
            </div>
        }
    }
}
