#[derive(Clone, PartialEq, Debug)]
pub enum Topic {
    Home,
    IdentityFundamentals,
    DomainOverview,
    TenantSettings,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SubTopic {
    Home,
    
    IntroductionToIAM,
    IntroductionToDomain,
    AuthenticationVsAuthorization,

    DomainDashboard,
    CreateTenants,
    RegisterApis,

    SigningKeys,
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

    // Signing Keys
    RotateSigningKeys,
    RevokeSigningKeys,
    ViewSigningCertificates,
}
