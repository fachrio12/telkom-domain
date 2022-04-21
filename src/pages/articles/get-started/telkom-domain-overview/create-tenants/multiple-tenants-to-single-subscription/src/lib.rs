use yew::prelude::*;

pub struct MultipleTenantsToSingleSubscription {}

pub enum Msg {}

impl Component for MultipleTenantsToSingleSubscription {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MultipleTenantsToSingleSubscription {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Link Multiple Tenants to a Single Subscription" }</h1>
                <p class="uk-margin-medium-bottom">
                    { "Telkom Domain offers the ability for customers with an Enterprise subscription to link multiple tenants under a single Telkom Domain subscription (these linked tenants can also be referred to as child tenants)." }
                </p>
                <p>{ "This feature can be useful under the following situations:" }</p>
                <ul class="uk-list uk-list-disc uk-margin-medium-bottom">
                    <li>{ "Separate development and production tenants while keeping the same feature access on development tenants as available in production tenants" }</li>
                    <li>{ "Own more than one production tenant under a single Enterprise subscription" }</li>
                </ul>

                <div class="uk-margin-medium-bottom">
                    <h1 class="td-text-size-large">{ "Linking tenants" }</h1>
                    <p>
                        { "Enterprise subscription automatically allows for users to link tenants under an existing Telkom Domain subscription by selection" }
                        { r#" "Custom Agreement" "# }
                        { "option from Create Under when creating new tenants." }
                    </p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/multiple-tenants-to-single-subscription/get-started-create-tenants-child_tenants0.png"
                    />
                    <p>{ "If you need to link previously created tenants that are not currently part of your Enterprise subscription, contact your designated Technical Account Manager or reach out to Telkom Domain Support." }</p>
                </div>

                <div class="uk-margin-medium-bottom">
                    <h1 class="td-text-size-large">{ "Usage consolidation" }</h1>
                    <p>{ "Usage from linked tenants count toward the Enterprise subscription limits and are aggregated under applicable usage and quota reports." }</p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Create Multiple Tenants" }</li>
                        <li>{ "Delete or Reset Tenants" }</li>
                        <li>{ "Set Up Multiple Environments" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
