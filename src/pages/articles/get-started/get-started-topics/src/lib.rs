#[derive(Clone, PartialEq, Debug)]
pub enum Topic {
    Home,
    IdentityFundamentals,
    // TelkomDomainOverview,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SubTopic {
    Home,
    IntroductionToIAM,
    AuthenticationVsAuthorization,
}