use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct LogoIconProp {
    #[prop_or(70)]
    pub width: u16,
    // #[prop_or(hsl(210, 88%, 93%))]
    // pub color: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LogoIcon {
    width: u16,
}

pub enum Msg {}

impl Component for LogoIcon {
    type Message = Msg;
    type Properties = LogoIconProp;

    fn create(ctx: &Context<Self>) -> Self {
        LogoIcon {
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
                        opacity: .65;
                        filter: sepia(20%) hue-rotate(250deg) saturate(16000%);
                    ", self.width*15/20)}
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
                        opacity: .76;
                        filter: sepia(15%) hue-rotate(168deg) saturate(16000%);
                    ", self.width*7/20)}
                />
                // <div
                //     style={format!("
                //         width: {}px;
                //         height: {}px;
                //         position: absolute;
                //         bottom: 50%;
                //         left:50%;
                //         transform-origin: 0% 100%;
                //         transform: rotate(-90deg) translate(-50%,0);
                //         background: rgba(40,40,40,.3);
                //     ", self.width*5/4, self.width*5/4)}
                // />
                // <div
                //     style={format!("
                //         width: {}px;
                //         height: {}px;
                //         position: absolute;
                //         bottom: 50%;
                //         left:50%;
                //         transform-origin: 0% 100%;
                //         transform: rotate(90deg) translate(-50%,0);
                //         background: rgba(40,40,40,.1);
                //     ", self.width*5/4, self.width*5/4)}
                // />
            </div>
        }
    }
}