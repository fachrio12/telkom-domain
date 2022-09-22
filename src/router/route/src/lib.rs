use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    // #[at("/get-started/:s")]
    // GetStarted,

    #[at("/get-started/identity-fundamentals/identity-and-access-management")]
    IntroductionToIAM,
    #[at("/get-started/identity-fundamentals/introduction-to-domain")]
    IntroductionToDomain,
    #[at("/get-started/identity-fundamentals/authentication-and-authorization")]
    AuthenticationVsAuthorization,
    #[at("/get-started/identity-fundamentals")]
    GetStartedIdentityFundamentals,


    #[at("/get-started/")]
    GetStartedHome,


    #[at("/get-started/domain-overview/dashboard/activity")]
    ActivityAbout,
    #[at("/get-started/domain-overview/dashboard")]
    DomainDashboard,

    #[at("/get-started/domain-overview/create-tenants/create-multiple-tenants")]
    CreateMultipleTenants,
    #[at("/get-started/domain-overview/create-tenants/child-tenants")]
    MultipleTenantsToSingleSubscription,
    #[at("/get-started/domain-overview/create-tenants/set-up-multiple-environments")]
    SetUpMultipleEnvironments,
    #[at("/get-started/domain-overview/create-tenants/multi-tenant-apps-best-practices")]
    MultiTenantBestPractices,
    #[at("/get-started/domain-overview/create-tenants")]
    CreateTenants,

    #[at("/get-started/domain-overview/set-up-apis")]
    RegisterApis,

    #[at("/get-started/domain-overview")]
    GetStartedDomainOverview,


    #[at("/get-started/tenant-settings/rotate-signing-keys")]
    RotateSigningKeys,
    #[at("/get-started/tenant-settings/revoke-signing-keys")]
    RevokeSigningKeys,
    #[at("/get-started/tenant-settings/view-signing-certificates")]
    ViewSigningCertificates,
    #[at("/get-started/tenant-settings/signing-keys")]
    SigningKeys,

    #[at("/get-started/tenant-settings")]
    GetStartedTenantSettings,


    #[at("/get-started/applications/remove-applications")]
    RemoveApplications,
    #[at("/get-started/applications/application-settings")]
    ApplicationSettings,

    #[at("/get-started/applications")]
    GetStartedApplicationsInDomain,

     
    #[at("/get-started/apis/scopes")]
    Scopes,
    #[at("/get-started/apis/delete-api-permissions")]
    DeleteApiPermissions,
    #[at("/get-started/apis/add-api-permissions")]
    AddApiPermissions,
    #[at("/get-started/apis/apis-settings")]
    ApisSettings,
   
    #[at("/get-started/apis")]
    GetStartedApis,


    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// #[derive(Clone, Routable, PartialEq)]
// pub enum RouteGetStarted {
//     #[at("/get-started/")]
//     Home,
//     #[not_found]
//     #[at("get-started/404")]
//     NotFound,
// }