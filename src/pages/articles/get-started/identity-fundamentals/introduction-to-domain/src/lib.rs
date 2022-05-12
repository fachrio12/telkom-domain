use yew::prelude::*;

pub struct IntroductionToDomain {}

pub enum Msg {}

impl Component for IntroductionToDomain {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IntroductionToDomain {}
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

                <h1 class="uk-heading-small uk-margin-bottom">{ "Introduction To Domain" }</h1>
                    
                <div class="uk-margin-large-bottom">
                    <p>
                        { "Auth0 is an identity access management (IAM) provider. But what does this mean? If you've read Introduction to Identity and Access Management (IAM), you know an IAM solution is a gatekeeper to the resources you provide to customers as web applications, APIs, etc. The gatekeeper initiates authorization as outlined in OAuth 2.0. The addition of the OpenID Connect layer adds authentication to secure your users’ digital identities and your product." }
                    </p>
                    <p>
                        { "The Auth0 identity platform supports different application types and frameworks. Whether your application is a regular web app, a mobile app, or a machine-to-machine app, Auth0 provides configurations for the most secure authorization grant, or workflow, for each. You can read more about authorization grants and choose the one for your application in our article Which OAuth 2.0 Flow Should I Use?" }
                    </p>
                    <p>
                        { "Aside from supporting secure protocols, the Auth0 identity platform allows you to customize login services to fit your business, your technology, and your customer base. Using the Auth0 Dashboard and Management API, you can create your own Auth0 instance to authenticate and authorize your customers. You can configure login behaviors, connect your user data store, manage those users, choose an authorization grant, and establish authentication factors for a seamless, scalable product with an impactful user experience." }
                    </p>
                </div>

                <h2 class="uk-margin-large-bottom td-text-size-huge">{ "Get Started" }</h2>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Identity Fundamentals" }</h2>
                    <p>
                        { "You don’t have to be an expert on IAM to integrate Auth0 into your application or API, but you can choose the right configuration for your use case if you know some key concepts. To learn more, read our Introduction to Identity and Access Management (IAM) article. If you still have questions about planning your implementation, review our Architecture Scenarios section for walk-throughs of real world scenarios." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Integrate with Telkom Domain" }
                    </h2>
                    <p>
                        { "To start integrating with Auth0, you can either start with our interactive Quickstart guides for initial set-up and quick configurations, or you can register your application manually in the Auth0 Dashboard. In Dashboard, you can create a tenant, or your Auth0 instance, from the ground up. If you prefer to use SDKs, Auth0 offers multiple options for each application type. To see the full offering, navigate to Auth0 Libraries." }
                    </p>
                    <p>
                        { "You can begin your configuration with general details in the Dashboard tenant settings, such as the name displayed to your users, your company logo, your callback URLs, or where Auth0 redirects your users after authentication. You can review our recommendations by reading Tenant Settings." }
                    </p>
                    <p>
                        { "Once you’ve set up the tenant, then you can create and configure your application or API. You can use the instructions in our articles Create Applications or Register APIs as a starting point." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Authenticate" }
                    </h2>
                    <p>
                        { "The vehicle of authentication is the login form, or the intermediary to allow your users access to your application. Users provide pre-determined credentials, such as username or password, in the login form to verify their digital identities." }
                    </p>
                    <p>
                        { "Auth0’s Universal Login is a login form you can customize to accommodate your brand and configure to provide secure access. Some benefits of using Universal Login are:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Passwordless login with biometrics" }</li>
                        <li>{ "Choice of multi-factor authentication methods from email, voice, or Duo" }</li>
                        <li>{ "Single Sign-on (SSO) capabilities" }</li>
                        <li>{ "Localization support" }</li>
                    </ul>
                    <p>
                        { "To learn more, read Universal Login. To find out more about available features, read New Universal Login vs. Classic Universal Login." }
                    </p>
                    <p>
                        { "Once you have a login form, you can connect your user store to Auth0. You can connect an existing database, or use a social, legal, or enterprise identity provider such as Twitter or Azure Active Directory. New users can sign up with the connection you have configured." }
                    </p>
                    <p>
                        { "Once you have a login form and user store connection, you can set protocols that work behind the scenes when users log in to your application. The most common protocols are associated with the OAuth 2.0 and OpenID Connect (OIDC) specs you may have reviewed in our Identity Fundamentals article." }
                    </p>
                    <p>
                        { "Another protocol to securely transmit information during log in comes in the form of tokens. Tokens from the Authorization Server, Auth0’s Authentication API, transmit information between entities.  When a user logs in and access is approved, the Authentication API sends an access token, an ID token, or both depending on the authentication grant you are using to create a session. Access tokens contain information about what scopes, or permissions, the requestor has in your application while ID tokens have requestor information, such as user metadata to better the user experience." }
                    </p>
                    <p>
                        { "Tokens from the Authentication API are JSON Web Tokens (JWTs) structured with:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "a header that includes the signature" }</li>
                        <li>{ "the payload that contains statements and attributes about the requestor" }</li>
                        <li>{ "the signature that verifies the token is valid" }</li>
                    </ul>
                    <p>
                        { "To learn more about tokens, read Access Tokens, ID Tokens, or JSON Web Tokens." }
                    </p>
                    <p>
                        { "Other protocols, like SAML (Security Assertion Markup Language) and WS-Fed (Web Service Federation) are used with more specific systems. SAML works with some identity providers while WS-Fed is used with Microsoft products. You can learn more by exploring the Protocols section of our documentation." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Manage users" }
                    </h2>
                    <p>
                        { "Managing user profiles and access can be time-consuming. If you choose to manage users with your Auth0 instance, you can remove some of the pain points." }
                    </p>
                    <p>
                        { "You can easily automate CRUD operations and query user profiles using Auth0 Dashboard or the Management API. You can categorize your users into categories with Auth0 Organizations to arrange your customer-base to fit your management style. To learn more, navigate to the Manage Users section of our documentation." }
                    </p>
                    <p>
                        { "Your business model may include levels of access for your users. You may want a subsection of users to have read-only permissions and another subsection with the ability to edit. Auth0’s Authorization Core allows you to implement role-based access control. You can create roles, assign roles to users, and define permissions." }
                    </p>
                    <p>
                        { "If you want to manage access based on browser behaviors, you can limit the lifetime of a session. A session, or the interaction between the requesting entity and your application or resource, has a lifetime limit. A session can end when the user closes the browser or navigates away from your webpage. You can extend sessions with refresh tokens that renew access tokens. Configure refresh tokens in the Dashboard.  To learn more, read Session Lifetime Limits and Get Refresh Tokens." }
                    </p>
                    <p>
                        { "Cookies, or strings of data, tie into the session and represent an authenticated user. Cookies allow your authenticated users to maintain a session and move between web pages without being forced to re-authenticate.  Once the browser closes, the cookie is cleared by the browser." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Customize" }
                    </h2>
                    <p>
                        { "Your brand is important, and Auth0 offers customization to make the login experience more personalized to your business. You can add your logo and color scheme to your login form as well as use a custom domain to give you ownership of the login URL. To learn more about configuration, read Custom Domains." }
                    </p>
                    <p>
                        { "Universal Login offers numerous features to configure authentication to fit your needs, like Multi-factor authentication, passwordless authentication with device biometrics, and localization. On a more granular level, you can adjust the text of prompts your user receives when an action needs to be completed. You can configure prompts for your users to signup, to enroll a device for authentication, or to send a code to an email/SMS for users to enter for verification. You can also customize email communications to welcome new users, verify enrollment, or reset passwords with email templates. To learn more, read Customize New Universal Login Text Prompts and Customize Email Templates." }
                    </p>
                    <p>
                        { "You can also configure certain events with Auth0 Actions. Actions are secure functions that execute during runtime. Actions trigger at different points in the pipeline and have a variety of uses. You could add metadata before the user signs up or redirect users to an external site. To learn more about what Actions can do for you, read Understand How Auth0 Actions Work." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Secure" }
                    </h2>
                    <p>
                        { "Malicious attacks can happen anytime. Auth0 offers several attack protection options, such as Bot Detection in combination with Google reCAPTCHA Enterprise to prevent cyber attacks. To learn more about Bot Detection configuration, read Bot Detection. " }
                    </p>
                    <p>
                        { "Even if you are using your own login page, Auth0 offers other security options you can enable in the Auth0 Dashboard: " }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Breached Password Detection " }</li>
                        <li>{ "Brute-Force Protection " }</li>
                        <li>{ "Suspicious IP Throttling " }</li>
                    </ul>
                    <p>
                        { "Breached password detection is a security measure against malicious agents with stolen credentials. Brute-force protection safeguards a targeted user account by limiting the amount of login attempts that automatically block the malicious IP and send a notification to the flagged user account. Suspicious IP throttling works where brute force protection leaves off to block traffic from any IP address that attempts rapid signups or logins." }
                    </p>
                    <p>
                        { "Other security measures depend on how you want your users to authenticate. Enabling multi-factor authentication (MFA) in Universal Login requires users to provide two or more authentication factors. With Auth0, you can customize MFA to trigger under certain circumstances, such as a user logging in from an unknown device or from a questionable IP address. To learn more about configuring MFA, read Adaptive MFA." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Deploy and monitor" }
                    </h2>
                    <p>
                        { "When you’ve finished testing your Auth0 instance and are ready to deploy, you can use our public or private cloud offerings. To learn more about available offerings, read Deployment Options. If you need a multi-tenant capable environment, you can read more about Private Cloud on AWS." }
                    </p>
                    <p>
                        { "To keep your deployment on track, we provide guidance in the form of pre-deployment recommendations, a deployment checklist, best practices, common fixes, and other tips to help make deployment as seamless as possible." }
                    </p>
                    <p>
                        { "Once you’ve established your production environment ready for users, you can be on the lookout with error tracking and alerts. The System Center Operations Manager allows you to monitor, while event logs can be exported to an analytical tool and allow you insight on trends, user behavior, or issues. " }
                    </p>
                </div>

            </>
        }
    }
}
