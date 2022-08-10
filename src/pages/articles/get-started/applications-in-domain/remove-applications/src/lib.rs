use yew::prelude::*;


pub struct RemoveApplications {}

pub enum Msg {}

impl Component for RemoveApplications {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RemoveApplications {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Remove Applications" }</h1>
                <p>
                    { "You can remove an application using the Telkom Dashboard or the Management API. Once confirmed, this operation cannot be undone." }
                </p>

                <ul class="uk-list uk-list-decimal uk-margin-medium-bottom">
                    <li>
                        <p>
                            { "Go to Dashboard > Applications > Applications, and select the name of an application to view." }
                        </p>
                        <img
                            class="uk-margin-top uk-margin-bottom"
                            src="/assets/remove-applications/dashboard-applications-applications-list.png"
                        />
                    </li>


                    <li>
                        <p>
                            { "Scroll to the bottom of the Settings page, locate the Danger Zone, select Delete, and confirm." }
                        </p>
                        <img
                            class="uk-margin-top uk-margin-bottom"
                            src="/assets/remove-applications/dashboard-applications-applications-settings-danger-zone.png"
                        />
                    </li>
                </ul>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Application Settings" }</li>
                        <li>{ "Confidential and Public Applications" }</li>
                        <li>{ "First-Party and Third-Party Applications" }</li>
                        <li>{ "Application Grant Types" }</li>
                    </ul>
                </div>

                
            </>
        }
    }
}
