use yew::prelude::*;
use alert::Alert;

pub struct MultiTenantBestPractices {}

pub enum Msg {}

impl Component for MultiTenantBestPractices {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MultiTenantBestPractices {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Multi-Tenant Applications Best Practices" }</h1>
                <p class="uk-margin-large-bottom">
                    { "Multi-tenancy is when a single instance of software runs on a server that is accessible to multiple groups of users. Auth0's Public Cloud is an example of a multi-tenant application. Your applications, settings, and connections are a single tenant, which shares resources with other tenants in the Public Cloud. To learn more about tenants, read Create Tenants. To learn more about Auth0's Public Cloud, read Deploy Auth0." }
                </p>
                <p>
                    { "Please note that this article is not about using multiple Auth0 tenants. It is about using Auth0 to secure your own multi-tenant application." }
                </p>
                <p>
                    { "There are several ways you can secure multi-tenant applications with Auth0. You can handle your multi-tenancy needs with one of the following approaches:" }
                </p>
                <ul class="uk-list uk-list-disc">
                    <li>{ "Use multiple connections" }</li>
                    <li>{ "Identify different tenants by application" }</li>
                    <li>{ "Store tenant details in app_metadata" }</li>
                    <li>{ "Use separate Auth0 tenants" }</li>
                </ul>
                <Alert message={String::from("Entity limits may apply. To learn more, read Entity Limit Policy. If you have an Enterprise subscription, you will not be constrained due to entity limits, but you may be constrained by a connection that already has thousands of enabled clients.")}/>

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
