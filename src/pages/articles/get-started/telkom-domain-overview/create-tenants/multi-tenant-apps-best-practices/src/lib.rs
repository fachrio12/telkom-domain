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
                <p>
                    { "Multi-tenancy is when a single instance of software runs on a server that is accessible to multiple groups of users. Domain's Public Cloud is an example of a multi-tenant application. Your applications, settings, and connections are a single tenant, which shares resources with other tenants in the Public Cloud. To learn more about tenants, read Create Tenants. To learn more about Domain's Public Cloud, read Deploy Domain." }
                </p>
                <p>
                    { "Please note that this article is not about using multiple Domain tenants. It is about using Domain to secure your own multi-tenant application." }
                </p>
                <p>
                    { "There are several ways you can secure multi-tenant applications with Domain. You can handle your multi-tenancy needs with one of the following approaches:" }
                </p>
                <ul class="uk-list uk-list-disc">
                    <li>{ "Use multiple connections" }</li>
                    <li>{ "Identify different tenants by application" }</li>
                    <li>{ "Store tenant details in app_metadata" }</li>
                    <li>{ "Use separate Domain tenants" }</li>
                </ul>
                <div class="uk-margin-large-bottom">
                    <Alert message={String::from("Entity limits may apply. To learn more, read Entity Limit Policy. If you have an Enterprise subscription, you will not be constrained due to entity limits, but you may be constrained by a connection that already has thousands of enabled clients.")}/>
                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Multiple connections" }</h1>
                    <p>
                        { "You can use multiple connections to handle your tenants. Each connection represents and contains a different pool of users." }
                    </p>
                    <p>
                        { "Using multiple connections introduces additional layers of complexity, but there are several scenarios where the upsides of this option outweigh the downsides:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "You have different Connection-level requirements, such as varying password policies, for each of your Applications." }</li>
                        <li>{ "You have user pools from different Connections. For example, one app may have users providing username/password credentials, while another app handles Enterprise logins." }</li>
                    </ul>
                    <p>
                        { "To implement this, you can call" }
                        <span class="td-label-code td-margin-text">
                            { "/authorize" }
                        </span>
                        { "with a connection specified for the user, using the" }
                        <span class="td-label-code td-margin-text">
                            { "connection" }
                        </span>
                        { "option in the Domain SPA SDK, or by passing a" }
                        <span class="td-label-code td-margin-text">
                            { "connection" }
                        </span>
                        { "parameter to the" }
                        <span class="td-label-code td-margin-text">
                            { "authorize()" }
                        </span>
                        { "method in Domain.js." }
                    </p>
                    <Alert message={String::from("If you use Lock in your applications, Lock supports a maximum of 50 database connections per application. Enterprise connections are not affected by this limit. If you use the New Universal Login Experience, Lock is not involved and this limitation therefore does not affect you.")}/>
                    <Alert message={String::from("Entity limits may apply. To learn more, read Entity Limit Policy. If you have an Enterprise subscription, you will not be constrained due to entity limits, but you may be constrained by a connection that already has thousands of enabled clients.")}/>
                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Identify tenants by application" }</h1>

                    <p>
                        { "You can represent each of your tenants with a separate application in Domain." }
                    </p>
                    <p>
                        { "Representing each of your tenants with an application allows you to configure each one differently. You can also enable/disable connections for individual applications if your tenants have varying requirements. Doing so, however, requires you to track the tenants to which your users belong within your application. Then, when they log in, you will need to specify the application they are to use." }
                    </p>
                    <Alert message={String::from("Entity limits may apply. To learn more, read Entity Limit Policy. If you have an Enterprise subscription, you will not be constrained due to entity limits, but you may be constrained by a connection that already has thousands of enabled clients.")}/>
                    <p>
                        { "In the Management API, to enable a connection for all clients, you need to pass all the clients in that array when calling" }
                        <span class="td-label-code td-margin-text">
                            { "PATCH /api/v2/connections/:id" }
                        </span>
                        <p>
                            { ". When the clients are in the hundreds of thousands, the payload exceeds our allowed payload size for an API request and makes it impossible to do this." }
                        </p>
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Store tenant details in app_metadata" }</h1>
                    <p>
                        { "Storing tenant details in the user metadata is the simplest of the implementation scenarios we cover in this article." }
                    </p>
                    <p>
                        { "Using the identifier of your choice (e.g.," }
                        <span class="td-label-code td-margin-text">
                            { r#""tenant": "customer_12345""# }
                        </span>
                        { "), you can store tenant-related details in the" }
                        <span class="td-label-code td-margin-text">
                            { "app_metadata" }
                        </span>
                        { ". Doing so allows all of your users, regardless of which tenant to which they belong, to log in using one uniform method." }
                    </p>
                    <p>
                        { "You can check for this value in your application after users log in and are redirected. This will help you sort users." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Create separate Domain tenants for each customer" }</h1>
                    <p>
                        { "You can create a new Domain tenant for each of your application's tenants." }
                    </p>
                    <p>
                        { "We recommend that you follow this approach only if you need to share access to the Domain Dashboard with individual customers. Otherwise, one of the above solutions is a more practical and easy to manage one than attempting to manage many Domain tenant dashboards, which is also not a scalable solution as your customer base grows." }
                    </p>
                    <p>
                        { "This method requires you to use a different set of Domain credentials when calling Domain APIs to authenticate users belonging to each customer because you would be using different applications on different Domain tenants (with different Client IDs) for each of your customers." }
                    </p>
                </div>
                
            </>
        }
    }
}
