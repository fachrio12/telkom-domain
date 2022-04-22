use yew::prelude::*;

pub struct CreateMultipleTenants {}

pub enum Msg {}

impl Component for CreateMultipleTenants {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CreateMultipleTenants {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Create Multiple Tenants" }</h1>
                <p class="uk-margin-large-bottom">
                    { "You can configure multiple tenants to create different environments in the Telkom Domain Dashboard to allow for complex configurations. For example, you could have two separate domains (one internal and one public-facing), or you may want users to log in differently for different applications. The way to accomplish this is to create more than one Telkom Domain tenant to allow you to have separate sets of applications, connections, and users for the applications and groups of users that you need to support." }
                </p>

                <div class="uk-margin-large-bottom">
                    <ul class="uk-list uk-list-decimal">
                        <li>
                            <p>{ "Go to the Telkom Domain Dashboard, select your tenant name, and select Create Tenant." }</p>
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/create-multiple-tenants/dashboard-tenant-drop-down-menu.png"
                            />
                        </li>
                        <li>
                            <p>{ "Enter your desired Tenant Domain, select a Region, and select Create." }</p>
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/create-multiple-tenants/dashboard-tenant-menu-create-tenant.png"
                            />
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Multiple Organization Architecture" }</li>
                        <li>{ "Set Up Multiple Environments" }</li>
                        <li>{ "Delete or Reset Tenants" }</li>
                        <li>{ "Multi-Tenant Applications Best Practices" }</li>
                        <li>{ "Tenant Settings" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
