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
}