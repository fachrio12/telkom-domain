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
    AuthenticationVsAuthorization,

    DomainDashboard,
    CreateTenants,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SubTopic2 {
    Home,
    ActivityAbout,
}
