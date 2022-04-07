use yew::prelude::*;

pub struct IntroductionToIAM {}

pub enum Msg {}

impl Component for IntroductionToIAM {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IntroductionToIAM {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>

                <h1 class="uk-heading-small uk-margin-large-bottom">{ "Introduction to Identity and Access Management (IAM)" }</h1>
                
                <h2 class="uk-heading-small uk-margin-large-bottom">{ "What is identity and access management (IAM)? " }</h2>
                <p>
                    { "Identity and access management provides control over user validation and resource access. Commonly known as IAM, this technology ensures that the right people access the right digital resources at the right time and for the right reasons." }
                </p>

                
            </>
        }
    }
}
