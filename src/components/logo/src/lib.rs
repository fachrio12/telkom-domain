use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct LogoState {
    #[prop_or(70)]
    pub width: u16,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Logo {
    width: u16,
}

pub enum Msg {}

impl Component for Logo {
    type Message = Msg;
    type Properties = LogoState;

    fn create(ctx: &Context<Self>) -> Self {
        Logo {
            width: ctx.props().width,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let width = ctx.props().width;
        if self.width == width {
            false
        } else {
            self.width = width;
            true
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div
                style={format!("
                        width: {}px;
                        height: {}px;
                        display: inline-block;
                        padding: 0px;
                        border-radius: {}px;
                        background: transparent;
                        border: {}px solid rgba(0,0,0,.78);
                        overflow: hidden;
                        position: relative;
                    ", self.width, self.width, self.width, 0/22)}
            >
                <img
                    src="/assets/logo/shield.png"
                    style={format!("
                        width: {}px;
                        position: absolute;
                        top: 53%;
                        left: 50%;
                        transform: translate(-50%,-50%);
                        opacity: .9;
                    ", self.width*19/20)}
                />
                <img
                    src="/assets/logo/lock3.png"
                    style={format!("
                        width: {}px;
                        position: absolute;
                        top: 55%;
                        left: 50%;
                        transform: translate(-50%,-50%);
                        filter: grayscale(100%);
                        opacity: 1;
                    ", self.width*9/20)}
                />
                <div
                    style={format!("
                        width: {}px;
                        height: {}px;
                        position: absolute;
                        bottom: 50%;
                        left:50%;
                        transform-origin: 0% 100%;
                        transform: rotate(-90deg) translate(-50%,0);
                        background: rgba(40,40,40,.3);
                    ", self.width*5/4, self.width*5/4)}
                />
                <div
                    style={format!("
                        width: {}px;
                        height: {}px;
                        position: absolute;
                        bottom: 50%;
                        left:50%;
                        transform-origin: 0% 100%;
                        transform: rotate(90deg) translate(-50%,0);
                        background: rgba(40,40,40,.1);
                    ", self.width*5/4, self.width*5/4)}
                />
            </div>
        }
    }
}