use yew::prelude::*;

pub struct IdentityFundamentalsHome {}

pub enum Msg {}

impl Component for IdentityFundamentalsHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IdentityFundamentalsHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Identity Fundamentals" }</h1>
                <p class="uk-margin-large-bottom">
                    { "Explore topics related to the fundamentals of identity and access management." }
                </p>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn the Basics" }</h1>
                    <table class="uk-table uk-table-divider">
                        <thead>
                            <tr>
                                <th class="uk-text-emphasis">{ "Read..." }</th>
                                <th class="uk-text-emphasis">{ "To learn..." }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    <a class="uk-link-text uk-text-primary" href="#">{ "Introduction to Identity and Access Management (IAM)" }</a>
                                </td>
                                <td>{ "Basic concepts of IAM." }</td>
                            </tr>
                            <tr>
                                <td>
                                    <a class="uk-link-text uk-text-primary" href="#">{ "Authentication vs. Authorization" }</a>
                                </td>
                                <td>{ "About the differences between authentication and authorization." }</td>
                            </tr>
                            <tr>
                                <td>
                                    <a class="uk-link-text uk-text-primary" href="#">{ "Glossary" }</a>
                                </td>
                                <td>{ "Definitions of various terms related to identity." }</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                
            </>
        }
    }
}
