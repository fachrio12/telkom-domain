use yew::prelude::*;

pub struct CreateTenantsHome {}

pub enum Msg {}

impl Component for CreateTenantsHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CreateTenantsHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Create Tenants" }</h1>
                <p class="uk-margin-medium-bottom">
                    { "We will walk through the initial steps of getting started using Telkom Domain to familiarize you with the key concepts of the Telkom Domain service. We will use the company Example-Co to help describe some of the steps involved." }
                </p>

                <div class="uk-margin-medium-bottom">
                    <h1 class="td-text-size-large">{ "Set up an Telkom Domain account" }</h1>
                    <p>{ "If you haven't already signed up for an Telkom Domain account, do so (it's free). You can either use username and password or log in with a social provider (such as Facebook, Google, or Apple)." }</p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Create a tenant and domain" }</h1>
                    <p>{ "Once you create your account you will be asked to create a tenant. Everything starts with an Telkom Domain tenant. This is where you configure your use of Telkom Domain, and the where Telkom Domain assets - such as applications, connections, and user profiles are defined, managed and stored. You access an Telkom Domain tenant via the Telkom Domain Dashboard, where you can also create additional, associated tenants. You can create more than one Telkom Domain tenant so that you can structure your tenants in a way that will isolate different domains of users and also support your Software Development Life Cycle (SDLC)." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/create-tenants/dashboard-tenant-menu-create-tenant.png"
                    />
                    <p>{ "Tenant names cannot be changed or reused once deleted. So, make sure you're happy with the name(s) before you create your Telkom Domain tenants." }</p>
                    <p>{ "Determining the level of isolation you require when it comes to your user domains is an important step, and together with your branding requirements helps you determine the number of Telkom Domain tenants needed in your environment. The number of Telkom Domain tenants you need to manage can quickly grow so consider carefully before creating multiple Telkom Domain tenants for production." }</p>
                    <p>{ "Tenant characteristics:" }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "The tenant name has to be unique. It will be used to create your personal domain." }</li>
                        <p>{ r#" "json": "format" "# }</p>
                        <li>
                            { "The tenant name can contain only lowercase alphanumeric characters and hyphens " }
                            { r#" ("-") "# }
                            { ". It cannot begin or end with a hyphen." }
                        </li>
                        <li>{ "The tenant name must be a minimum of 3 characters and a maximum of 64 characters." }</li>
                        <li>{ "The tenant name cannot be changed after creation." }</li>
                        <li>{ "You can create more than one tenant; in fact, you are encouraged to do so for each environment you may have such as development, staging, or production. To learn more, read Set Up Multiple Environments." }</li>
                    </ul>
                    <p>{ "When you name your tenant, that name becomes your Telkom Domain's domain. (Or, you can create a custom domain; see below.) This domain is the base URL that you will use to access our API and the URL where your users are redirected in order to authenticate. Telkom Domain supports these locality subdomains:" }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "US" } </li>
                        <li>{ "EU" } </li>
                        <li>{ "AU" } </li>
                        <li>{ "JP" } </li>
                    </ul>
                    <p>{ "The US is further separated into three regions: US-1, US-2, and US-3." }</p>
                    <p>{ "You cannot choose which region your tenant will reside in. When you are asked for the locality you want to use, your choice affects which regional subdomain will be assigned to you and where your data will be hosted." }</p>
                    <p>
                        { "In our example, Example-Co chose the name" }
                        <span class="td-label-code">{ "example-co" }</span>
                        { "and AU as their region. So their domain is" }
                        <span class="td-label-code">{ "example-co.au.Telkom Domain.com" }</span>
                        { "." }
                    </p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Custom domains" }</h1>
                    <p>
                        { "We recommend the use of custom domains, such as example-co.com" }
                        <span class="td-label-code">{ "example-co.com" }</span>
                        { ", in production environments to provide your users with the most secure and seamless experience. This comes with an additional cost." }
                    </p>
                    <p> { "If you have a single-tenant implementation, you can deploy your custom domain in:" } </p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "The cloud-managed by Telkom Domain" } </li>
                        <li>{ "An AWS cloud managed by you" } </li>
                    </ul>
                    <p> { "To learn more, read Custom Domains." } </p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "What's next" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Create and register applications: Now that you have an account and a domain, you need to register each application that will use our services in the Telkom Domain Dashboard. To learn more, read Applications in Telkom Domain and Create Applications." }</li>
                        <li>{ "Set up connections: Next, you need to set up how your users will authenticate during log in. Telkom Domain sits between your app and the identity provider that authenticates your users (such as Google or Facebook). The relationship between Telkom Domain and the identity provider is referred to as a connection. By using this connection layer, Telkom Domain keeps your app isolated from any changes that occur with the identity provider's implementation. To learn more, read Authentication and Authorization and Connections." }</li>
                    </ul>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Extend Telkom Domain's functionality" }</h1>
                    <p>{"Telkom Domain offers several ways to extend the platform's functionality:"}</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Actions: Actions are secure, tenant-specific, versioned functions written in Node.js that execute at certain points during the Telkom Domain runtime. Use actions to customize and extend Telkom Domain's capabilities with custom login." }</li>
                        <li>
                            <p>{ "Rules: Rules are functions written in JavaScript or C#, that are executed in Telkom Domain just after successful authentication and before control returns to your app. Rules can be chained together for modular coding and can be turned on and off individually. You can use rules for:" }</p>
                            <ul class="uk-list uk-list-disc">
                                <li>{ "Access control" }</li>
                                <li>{ "Webhooks" }</li>
                                <li>{ "Profile enrichment" }</li>
                                <li>{ "Multi-factor authentication (MFA)" }</li>
                            </ul>
                        </li>
                        <li>{ "Hooks: Hooks allow you to customize the behavior of Telkom Domain using Node.js code that is executed against extensibility points (which are comparable to webhooks that come with a server). They are secure, self-contained functions associated with specific extensibility points of the Telkom Domain platform. Telkom Domain invokes the Hooks at runtime to execute your custom logic." }</li>
                        <li>
                            { "Extensions: Telkom Domain Extensions enable you to install applications or run commands/scripts that extend the functionality of the Telkom Domain base product. You can either use one of the pre-defined extensions, provided by Telkom Domain, or create your own. Some of the actions you can do with extensions include:" }
                            <ul class="uk-list uk-list-disc">
                                    <li>{ "Manage the authorizations for users (using groups, roles, and permissions)" }</li>
                                    <li>{ "Import/export users" }</li>
                                    <li>{ "Export logs to other services" }</li>
                                    <li>{ "Deploy scripts from external repositories" }</li>
                            </ul>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Create Tenants" }</li>
                        <li>{ "Telkom Domain Dashboard" }</li>
                        <li>{ "Architecture Scenarios" }</li>
                        <li>{ "Protocols" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
