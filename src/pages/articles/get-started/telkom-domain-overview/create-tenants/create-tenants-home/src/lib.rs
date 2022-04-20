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
                { "CREATE TENANTS HOME" }

                // <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Telkom Domain Overview" }</h1>
                // <p>
                //     { "Telkom Domain is a flexible, drop-in solution to add authentication and authorization services to your applications. Your team and organization can avoid the cost, time, and risk that come with building your own solution to authenticate and authorize users.
                //     " }
                // </p>

                // <div
                //     class="uk-margin-medium-bottom"
                // >
                //     <p>
                //         { "Take a look at just a few of Telkom Domain's use cases:" }
                //     </p>
                //     <ul class="uk-list uk-list-disc">
                //         <li>{ "You built an awesome app and you want to add user authentication and authorization. Your users should be able to log in either with a username/password or with their social accounts (such as Facebook or Twitter). You want to retrieve the user's profile after the login so you can customize the UI and apply your authorization policies." }</li>
                //         <li>{ "You built an API and you want to secure it with Telkom Domain" }</li>
                //         <li>{ "You have more than one app, and you want to implement Single Sign-on (SSO)." }</li>
                //         <li>{ "You built a JavaScript front-end app and a mobile app, and you want them both to securely access your API." }</li>
                //         <li>{ "You have a web app that needs to authenticate users using Security Assertion Markup Language (SAML)." }</li>
                //         <li>{ "You believe passwords are broken and you want your users to log in with one-time codes delivered by email or SMS." }</li>
                //         <li>{ "If one of your user's email addresses is compromised in some site's public data breach, you want to be notified, and you want to notify the users and/or block them from logging in to your app until they reset their password." }</li>
                //         <li>{ "You want to act proactively to block suspicious IP addresses if they make consecutive failed login attempts, in order to avoid DDoS attacks." }</li>
                //         <li>{ "You are part of a large organization that wants to federate your existing enterprise directory service to allow employees to log in to the various internal and third-party applications using their existing enterprise credentials." }</li>
                //         <li>{ "You don't want (or you don't know how) to implement your own user management solution. Password resets, creating, provisioning, blocking, and deleting users, and the UI to manage all these. You just want to focus on your app." }</li>
                //         <li>{ "You want to enforce multi-factor authentication (MFA) when your users want to access sensitive data." }</li>
                //         <li>{ "You are looking for an identity solution that will help you stay on top of the constantly growing compliance requirements of SOC2, GDPR, PCI DSS, HIPAA, and others." }</li>
                //         <li>{ "You want to monitor users on your site or application. You plan on using this data to create funnels, measure user retention, and improve your sign-up flow." }</li>
                //     </ul>

                //     <table class="uk-table uk-table-divider">
                //         <thead>
                //             <tr>
                //                 <th class="uk-text-emphasis">{ "Read..." }</th>
                //                 <th class="uk-text-emphasis">{ "To learn..." }</th>
                //             </tr>
                //         </thead>
                //         <tbody>
                //             <tr>
                //                 <td>
                //                     <a class="uk-link-text uk-text-primary" href="#">{ "Telkom Domain Dashboard" }</a>
                //                 </td>
                //                 <td>{ "About the Telkom Domain Dashboard and features you can access to implement authentication and authorization with your applications and APIs." }</td>
                //             </tr>
                //             <tr>
                //                 <td>
                //                     <a class="uk-link-text uk-text-primary" href="#">{ "Create Tenants" }</a>
                //                 </td>
                //                 <td>{ "How to create tenants using the Telkom Domain Dashboard or the Management API, explore creating multiple tenants and child tenants, and learn about setting up multiple environments." }</td>
                //             </tr>
                //             <tr>
                //                 <td>
                //                     <a class="uk-link-text uk-text-primary" href="#">{ "Create Applications" }</a>
                //                 </td>
                //                 <td>{ "How to set up and configure applications in the Telkom Domain Dashboard." }</td>
                //             </tr>
                //             <tr>
                //                 <td>
                //                     <a class="uk-link-text uk-text-primary" href="#">{ "Register APIs" }</a>
                //                 </td>
                //                 <td>{ "How to set up and configure APIs in the Telkom Domain Dashboard." }</td>
                //             </tr>
                //         </tbody>
                //     </table>
                // </div>

                // <div
                //     class="uk-margin-medium-bottom"
                // >
                //     <h1 class="td-text-size-large">{ "Learn more" }</h1>
                //     <ul class="uk-list uk-list-disc">
                //         <li>{ "Create Tenants" }</li>
                //         <li>{ "Telkom Domain Dashboard" }</li>
                //         <li>{ "Architecture Scenarios" }</li>
                //         <li>{ "Protocols" }</li>
                //     </ul>
                // </div>
                
            </>
        }
    }
}
