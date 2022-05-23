#[derive(Clone, PartialEq, Debug)]
pub enum Topic {
    Home,
    IdentityFundamentals,
    DomainOverview,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SubTopic {
    Home,
    
    IntroductionToIAM,
    IntroductionToDomain,
    AuthenticationVsAuthorization,

    DomainDashboard,
    CreateTenants,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SubTopic2 {
    Home,
    ActivityAbout,

    // CREATE TENANTS
    CreateMultipleTenants,
    MultipleTenantsToSingleSubscription,
    SetUpMultipleEnvironments,
    MultiTenantBestPractices,
}
