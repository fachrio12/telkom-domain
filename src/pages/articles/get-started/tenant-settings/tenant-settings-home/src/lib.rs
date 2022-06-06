use yew::prelude::*;
use alert::Alert;

pub struct TenantSettingsHome {}

pub enum Msg {}

impl Component for TenantSettingsHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TenantSettingsHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Tenant Settings" }</h1>
                <p>
                    { "Use the Tenant Settings page in the Auth0 Dashboard at Dashboard > Settings to configure various settings related to your Auth0 tenant." }
                </p>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Recommended settings" }</h1>
                    <p>
                        { "Auth0 recommends that when you configure your tenant, set the following items at a minimum:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Specify a production tenant:" }</b>
                            <span class="td-margin-text">{ "Production tenants get higher rate limits than non-production tenants. On non-Enterprise plans, only one tenant per subscription can be set as a production tenant." }</span>
                        </li>
                        <li>
                            <b>{ "Specify a support URL and email address:" }</b>
                            <span class="td-margin-text">{ "We recommend that you host your own custom error page and configure Auth0 to use it instead of the Auth0 default. This allows you to provide more complete and customized explanations to users about what to do in the event of an error." }</span>
                        </li>
                        <li>
                            <p>
                                <b>{ "Set up a custom domain:" }</b>
                                <span class="td-margin-text">{ "We recommend that you use a custom domain name for the Universal Login page; set up the custom domain early to minimize changes you’ll need to make later. To learn more about Universal Login, read Auth0 Universal Login." }</span>
                            </p>
                            <p>
                                { "If you will use SAML connections to authenticate users against remote SAML identity providers, set up the custom domain before you configure the SAML providers because changing the domain across multiple SAML providers is cumbersome." }
                            </p>
                        </li>
                        <li>
                            <b>{ "Set Single Sign-On session timeout:" }</b>
                            <span class="td-margin-text">{ "The SSO session timeout value specifies the time until a user's session expires. By default, the value is 7 days which is the length of time users can access your Auth0-integrated applications without re-entering their credentials. To learn more, read Sessions." }</span>
                        </li>
                        <li>
                            <b>{ "Set up tenant members:" }</b>
                            <span class="td-margin-text">{ "Configure additional Auth0 Dashboard users and enable multi-factor authentication (MFA). To learn more, read Manage Dashboard Access and Manage Dashboard Access with Multi-Factor Authentication." }</span>
                        </li>
                        <li>
                            <b>{ "Disable application connections initially:" }</b>
                            <span class="td-margin-text">{ "If this setting is enabled, all configured connections are enabled for new applications you create, so users may be able to log into an application with an unintended connection. By having connections disabled by default, you can explicitly enable the connections appropriate for each application." }</span>
                        </li>
                        <li>
                            <b>{ "Enable attack protection:" }</b>
                            <span class="td-margin-text">{ "To protect against brute force attacks and breached passwords, enable and configure attack protection. To learn more, read Attack Protection." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "General tab" }</h1>
                    <p class="uk-margin-large-bottom">{ "On the General tab, you can customize basic tenant settings." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Settings" }</h1>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-general-settings.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Friendly Name:" }</b>
                            <span class="td-margin-text">{ "Name you want to be displayed to your users, usually the name of your company or organization." }</span>
                        </li>
                        <li>
                            <b>{ "Logo URL:" }</b>
                            <span class="td-margin-text">{ "URL of your logo; it should be a square. This image will appear to your users on various screens and pages." }</span>
                        </li>
                        <li>
                            <b>{ "Support Email:" }</b>
                            <span class="td-margin-text">{ "Email address used to contact your support team." }</span>
                        </li>
                        <li>
                            <b>{ "Support URL:" }</b>
                            <span class="td-margin-text">{ "Link to your company/organization support page." }</span>
                        </li>
                    </ul>
                </div>
                
                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Environment Tag" }</h1>
                    <p>
                        { "You can identify your tenant as a production, staging, or development tenant to differentiate it from other tenants. Higher rate limits apply to tenants tagged as Production with a paid subscription. To learn more, read Rate Limit Policy." }
                    </p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/Dashboard-Tenant_Settings-General-Environmental_Tag0.png"
                    />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "API Authorization Settings" }</h1>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-general-api-auth.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Default Audience:" }</b>
                            <span class="td-margin-text">{ "API identifier to use for Authorization Flows. If you enter a value, all access tokens issued by Auth0 will specify this API identifier as an audience. Setting the Default Audience is equivalent to appending this audience to every authorization request made to your tenant for every application. This will cause new behavior that might result in breaking changes for some of your applications. Please contact support if you require assistance." }</span>
                        </li>
                        <li>
                            <b>{ "Default Directory:" }</b>
                            <span class="td-margin-text">{ "Name of the default connection to be used for the Resource Owner Password Flow. Its value should be the exact name of an existing connection for one of the following strategies: auth0-adldap, ad, auth0, email, sms, waad, or adfs." }</span>
                            <span class="td-label-code td-margin-text">{ "auth0-adldap" }</span>
                            { "," }
                            <span class="td-label-code td-margin-text">{ "ad" }</span>
                            { "," }
                            <span class="td-label-code td-margin-text">{ "auth0" }</span>
                            { "," }
                            <span class="td-label-code td-margin-text">{ "email" }</span>
                            { "," }
                            <span class="td-label-code td-margin-text">{ "sms" }</span>
                            { "," }
                            <span class="td-label-code td-margin-text">{ "waad" }</span>
                            { ", or" }
                            <span class="td-label-code td-margin-text">{ "adfs" }</span>
                            { "." }
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Error Pages" }</h1>
                    <p>{ "In the event of an authorization error, you can either display a generic error page to your users or you can redirect users to your own custom error page. To learn more, read Custom Error Pages." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-general-error-pages.png"
                    />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Languages" }</h1>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-general-languages.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Default Language:" }</b>
                            <span class="td-margin-text">{ "Language your tenant will use by default." }</span>
                        </li>
                        <li>
                            <b>{ "Supported Languages:" }</b>
                            <span class="td-margin-text">{ "Languages also supported by your tenant." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Subscription" }</h1>
                    <p>{ "On the Subscription tab, you can review your current subscription and compare features of your current plan to other Auth0 subscription plans. You can also change your subscription plan. To learn more, read Manage Subscription." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/Screen_Shot_2021-05-03_at_2.55.38_PM.png"
                    />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Payment" }</h1>
                    <p class="uk-margin-large-bottom">{ "On the Payment tab, you can enter or update your billing details." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Tenant Members" }</h1>
                    <p>{ "On the Tenant Members tab, you can view a list tenant members assigned to your tenant. You may also add or remove tenant members and review their assigned roles and if they have multi-factor authentication (MFA) enabled. To learn more, read Manage Dashboard Access." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-tenant-members.png"
                    />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Custom Domains" }</h1>
                    <p>{ "On the Custom Domains tab, you can configure a custom domain to maintain a consistent user experience. When you create a custom domain, users will remain in your domain for login rather than being redirected to your auth0.com domain. To learn more, read Custom Domains." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-custom-domains.png"
                    />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Signing Keys" }</h1>
                    <p>{ "On the Signing Keys tab, you can securely manage the signing key and certificate used to sign ID tokens, access tokens, SAML assertions, and WS-Fed assertions that are sent to your applications." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-signing-keys.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Rotation Settings:" }</b>
                            <span class="td-margin-text">{ "Settings that allow you to rotate the application signing key and certificate. You can choose whether or not to revoke the signing key upon rotation. To learn more, read Signing Keys." }</span>
                            <ul class="uk-list uk-list-disc">
                                <li>
                                    <b>{ "Rotate Signing Key:" }</b>
                                    <span class="td-margin-text">{ "Rotates the signing key without revoking it; effectively, moves the current key to the previous key. All tokens signed with the previous key will still be valid until it is revoked." }</span>
                                </li>
                                <li>
                                    <b>{ "Rotate & Revoke Signing Key:" }</b>
                                    <span class="td-margin-text">{ "Rotates the signing key and then revokes it; effectively, moves the current key to the previous key, and then invalidates the previous key. Make sure you have updated your application with the next key in the queue before you rotate and revoke the current key." }</span>
                                </li>
                            </ul>
                        </li>
                        <li>
                            <b>{ "List of Valid Keys:" }</b>
                            <span class="td-margin-text">{ "List of valid application signing keys for your tenant, which are also available at the Metadata endpoint for your application. Valid keys include:" }</span>
                            <ul class="uk-list uk-list-disc">
                                <li>
                                    <b>{ "Next in queue:" }</b>
                                    <span class="td-margin-text">{ "Key that will be used when the signing key is next rotated." }</span>
                                </li>
                                <li>
                                    <b>{ "Currently used:" }</b>
                                    <span class="td-margin-text">{ "Key that is currently in use." }</span>
                                </li>
                                <li>
                                    <b>{ "Previously used:" }</b>
                                    <span class="td-margin-text">{ "Key that was previously used. Its appearance indicates that the signing key has been rotated, but the previously-used key has not yet been revoked." }</span>
                                </li>
                            </ul>
                        </li>
                        <li>
                            <b>{ "List of Revoked Keys:" }</b>
                            <span class="td-margin-text">{ "List of the last three revoked keys for your tenant. More data about revoked keys is available in tenant logs." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Advanced" }</h1>
                    <p>{ "On the Advanced tab, you can configure advanced tenant settings." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Login and Logout" }</h1>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-advanced-login-logout.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Allowed Logout URLs:" }</b>
                            <span class="td-margin-text">{ "URLs that Auth0 can redirect to after logout when no client_id is specified on the Logout endpoint invocation. Useful as a global list when Single Sign-on (SSO) is enabled. To learn more, see Logout." }</span>
                        </li>
                        <li>
                            <b>{ "Tenant Login URI:" }</b>
                            <span class="td-margin-text">{ "URI that points to a route in your application that starts the OIDC login flow by redirecting to the /authorize endpoint; it should take the form of https://mytenant.org/login. This will only be used in scenarios where Auth0 needs your tenant to start the OIDC login flow. To learn more, see Configure Default Login Routes." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Login Session Management" }</h1>
                    <p>{ "The Login Session Management settings configure the login session lifetime that represents the Auth0 Authorization Server session layer. The authorization server session layer drives single sign-on (SSO). To learn more, read Single Sign-on." }</p>
                    <p>{ "Timeouts for tokens issued by Auth0 can be configured elsewhere. Token timeouts are often used to drive the Application session layer and appear in token claims, such as in the expiration claim for OpenID Connect (OIDC) ID tokens or the lifetime assertion for SAML." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-advanced-login-session.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Inactivity timeout:" }</b>
                            <span class="td-margin-text">{ "Timeframe (in minutes) after which a user's session will expire if they haven’t interacted with the Authorization Server. It will be superseded by system limits if over 4,320 minutes (3 days) for non-Enterprise plans or 144,000 minutes (100 days) for Enterprise plans." }</span>
                        </li>
                        <li>
                            <b>{ "Require log in after:" }</b>
                            <span class="td-margin-text">{ "Timeframe (in minutes) after which a user will be required to log in again, regardless of their activity. It will be superseded by system limits if over 43,200 minutes (30 days) for non-Enterprise plans or 525,600 minutes (365 days) for Enterprise plans." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Device Flow User Code Format" }</h1>
                    <p>{ "If you are using the Device Authorization Flow, these settings configure the randomly generated user code. To learn more, read Configure Device User Code Settings." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-advanced-device-flow.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "User Code Character Set:" }</b>
                            <span class="td-margin-text">{ "Character set used to generate the user code." }</span>
                        </li>
                        <li>
                            <b>{ "User Code Mask:" }</b>
                            <span class="td-margin-text">{ "Mask used to format the user code. The mask defines the length of the user code and formats it into a friendly, readable value, allowing spaces or hyphens for readability." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Global client information" }</h1>
                    <p>{ "The Global Client ID and Global Client Secret are used to generate tokens for legacy Auth0 APIs. Typically, you will not need these values. If you need to have the global client secret changed, please contact support." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-advanced-global-client.png"
                    />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Settings" }</h1>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-advanced-settings.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Change Password Flow v2:" }</b>
                            <span class="td-margin-text">{ "When enabled, the newest version of the Change Password Flow will be used. The previous version has been deprecated, and we strongly recommend enabling v2. This flag is presented only for backward compatibility, and once enabled, you can no longer disable it. You can customize the user interface for the Change Password widget on the Universal Login > Password Reset tab in the Auth0 Dashboard." }</span>
                        </li>
                        <li>
                            <b>{ "OIDC Dynamic Application Registration:" }</b>
                            <span class="td-margin-text">{ "When enabled, third-party developers will be able to dynamically register applications for your APIs. You can also update this flag using the" }</span>
                            <span class="td-label-code td-margin-text">{ "/tenant/patch_settings" }</span>
                            <span>{ "endpoint of the Auth0 Management API. By default, this feature is disabled. To learn more, read Dynamic Client Registration." }</span>
                        </li>
                        <li>
                            <b>{ "Enable Application Connections:" }</b>
                            <span class="td-margin-text">{ "When enabled, all current connections will be enabled for any new application that is created." }</span>
                        </li>
                        <li>
                            <b>{ "Use a generic response in public signup API error message:" }</b>
                            <span class="td-margin-text">{ "When enabled, errors generated while using the public signup API will return a generic response. This helps protect against user registration enumeration by preventing bad actors from being able to guess previously-registered email addresses or usernames from reading error response codes, such as" }</span>
                            <span class="td-label-code td-margin-text">{ "user_exists" }</span>
                            <span>{ "." }</span>
                        </li>
                        <li>
                            <b>{ "Enable Publishing of Enterprise Connections Information with IdP domains:" }</b>
                            <span class="td-margin-text">{ "When enabled, it supports Home Realm Discovery and Auth0 Lock relies on a checked public file that includes enterprise connection information. If you don’t require that functionality, you can disable it." }</span>
                        </li>
                        <li>
                            <b>{ "Enable email verification flow during login for Azure AD and ADFS connections:" }</b>
                            <span class="td-margin-text">{ "When enabled, users will be presented with an email verification prompt during their first login when using Azure AD or ADFS connections." }</span>
                        </li>
                        <li>
                            <b>{ "Refresh Token Revocation Deletes Grant:" }</b>
                            <span class="td-margin-text">{ "When enabled, it deletes the underlying grant when you revoke a refresh token using the Authentication API" }</span>
                            <span class="td-label-code td-margin-text">{ "/oauth/revoke" }</span>
                            <span>{ "endpoint." }</span>
                            <Alert message={String::from("For existing tenants, this feature is enabled by default to preserve the existing behavior. For new tenants (as of 13 January 2021), this feature is disabled by default to ensure that the revocation of a refresh token will not revoke the grant. If a grant revocation is needed, a separate request must be sent using a grant revocation endpoint.")}/>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Extensibility" }</h1>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/tenant-settings/dashboard-tenant-settings-advanced-extensibility.png"
                    />
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Runtime:" }</b>
                            <span class="td-margin-text">{ "NodeJS version environment used to execute custom scripts that allow you to extend parts of Auth0's functionality; these include Actions, Rules, Hooks, and database connections. Choose the" }</span>
                            <span class="td-label-code td-margin-text">{ "node.js" }</span>
                            <span>{ "version environment you will use to execute your custom scripts. If you are migrating from an older version of" }</span>
                            <span class="td-label-code td-margin-text">{ "node.js" }</span>
                            <span>{ "that is no longer supported, read Migrate to Node.js 12." }</span>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Migrations" }</h1>
                    <p>{ "In this section, you can choose to enable or disable various migrations that are available." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Feature Previews" }</h1>
                    <p>{ "In this section, you can choose to enable or disable feature previews that are available." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-big">{ "Delete tenant or subscription" }</h1>
                    <p>{ "Deleted tenants cannot be restored and the tenant name cannot be used again when creating new tenants. To learn how to reset your tenant configuration, read Delete or Reset Tenants." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Manage Subscription" }</li>
                        <li>{ "Delete or Reset Tenants" }</li>
                        <li>{ "Manage Dashboard Access" }</li>
                        <li>{ "Signing Keys" }</li>
                        <li>{ "Custom Domains" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
