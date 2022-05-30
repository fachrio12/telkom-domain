use yew::prelude::*;

pub struct RegisterApis {}

pub enum Msg {}

impl Component for RegisterApis {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RegisterApis {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Register APIs" }</h1>
                <p class="uk-margin-medium-bottom">
                    { "Development, staging, and production environments are easy to set up in Auth0. Simply create a new tenant for each environment to guarantee isolation between them. You can easily switch between tenants using the tenant chooser from the top left menu on the Dashboard. You can also configure different administrators for each." }
                </p>
                <p>{ "Production rate limits only apply to tenants tagged as Production. Ensure your tenant's environment tag is set to Production before going live." }</p>
                <p>{ "You can name your multiple environments any way you prefer. For production environments, we strongly recommend using custom domains." }</p>
                <p
                    class="uk-margin-large-bottom"
                >{ "If you have an Enterprise subscription plan you can link tenants that which become identical to your Production account in terms of paid/upgraded features for use in a development/staging/testing environment." }</p>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Tag the environment" }</h1>
                    <p>{ "For each new tenant created, you should specify its environment. You can assign Environment Tags to your tenants to differentiate between development, staging, and production environments." }</p>
                    <p>{ "If your tenant is mixed use, choose the higher environment. For example, a tenant used for both development and production should be set to Production." }</p>
                    <ul class="uk-list uk-list-decimal">
                        <li>
                            { "To assign an Environment Tag to a tenant, go to the Dashboard > Settings > General." }
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/set-up-multiple-environments/dashboard-tenant-settings-general-settings.png"
                            />
                        </li>
                        <li>
                            { "Under Assign Environment Tag, identify your tenant's environment as Development, Staging, or Production." }
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/set-up-multiple-environments/Dashboard-Tenant_Settings-General-Environmental_Tag0.png"
                            />
                        </li>
                        <li>{ "After selecting the environment, click Save." }</li>
                    </ul>
                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Migration" }</h1>
                    <p>{ "Through the Management API v2, you can automate the migration of assets (rules, database connections, and so forth) between tenants." }</p>
                    <p>{ "For easier configuration management, save your configuration values in the Dashboard, instead of hardcoding them into your rules or db connections scripts." }</p>
                    <p>{ "For example, let's say you want to set a URL for logs. One way to do it is to hardcode it in the rule:" }</p>
                    // JSON DISPLAY ELEMENT
                    <p>{ "This code, however, is not portable since this URL will likely change from development to production." }</p>
                    <p>{ "The recommended way of working with code that you need to use/move from development to product is via Rules section. If you have not yet created a rule, you'll need to do so. (Otherwise, jump to step 4.)" }</p>
                    <ul class="uk-list uk-list-decimal">
                        <li>{ "Click Create Your First Rule." }</li>
                        <li>{ "Choose the empty rule template." }</li>
                        <li>{ "Enter a name for your new rule, and click Save." }</li>
                        <li>{ "Go to Dashboard > Rules, and scroll to the bottom of the page to set your configuration values (we will use log_url for the key name, and https://someurl/log for value), then click Create." }</li>
                        <li>
                            { "Now, you can write your rule. Edit the rule you created, enter the following code in the code area, and click Save." }
                            // JSON DISPLAY ELEMENT
                        </li>
                    </ul>
                    <p>{ "This code is portable, and when you migrate to production, you only need to change this setting instead of searching your scripts." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "AD/LDAP Connectors" }</h1>
                    <p>{ "If you use multiple Auth0 tenants with AD/LDAP, you will need to create an AD/LDAP Connection and set up an AD/LDAP Connector for each tenant. This is because each AD/LDAP Connector is tied to a specific Connection within an Auth0 tenant." }</p>
                    <p>{ "Multiple AD/LDAP Connectors can point to the same AD or LDAP directory, but each AD/LDAP Connector can only be used by one Connection within one Auth0 tenant." }</p>
                    <p>{ "If you have multiple AD/LDAP directories against which users will authenticate (for example, to support different departments or customers, each with their own directory), you can set up multiple AD/LDAP Connectors within each Auth0 tenant." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Create Multiple Tenants" }</li>
                        <li>{ "Delete or Reset Tenants" }</li>
                        <li>{ "Link Multiple Tenants to a Single Subscription" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
