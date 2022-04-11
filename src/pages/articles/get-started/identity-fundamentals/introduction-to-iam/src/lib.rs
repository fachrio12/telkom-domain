use yew::prelude::*;

pub struct IntroductionToIAM {}

pub enum Msg {}

impl Component for IntroductionToIAM {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IntroductionToIAM {}
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

                <h1 class="uk-heading-small uk-margin-large-bottom">{ "Introduction to Identity and Access Management (IAM)" }</h1>
                    
                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "What is identity and access management (IAM)? " }</h2>
                    <p>
                        { "Identity and access management provides control over user validation and resource access. Commonly known as IAM, this technology ensures that the right people access the right digital resources at the right time and for the right reasons." }
                    </p>
                </div>
                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "IAM basic concepts" }</h2>
                    <p>
                        { "To understand IAM, you must be familiar with some fundamental concepts:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            { "A digital resource is any combination of applications and data in a computer system. Examples of digital resources include web applications, APIs, platforms, devices, or databases. " }
                        </li>
                        <li>{ "The core of IAM is identity. Someone wants access to your resource. It could be a customer, employee, member, participant, and so on. In IAM, a user account is a digital identity. User accounts can also represent non-humans, such as software, Internet of Things devices, or robotics." }</li>

                        <img
                            src="/assets/introduction-to-iam/intro-iam-user-wants-resource.png"
                        />

                        <img
                            src="/assets/introduction-to-iam/IAM-verifies-access.png"
                        />
                        <li>
                            { "Authentication is the verification of a digital identity. Someone (or something) authenticates to prove that they’re the user they claim to be. " }
                        </li>
                        <li>
                            { "Authorization is the process of determining what resources a user can access.  " }
                        </li>
                    </ul>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "The difference between authentication and authorization" }</h2>

                    <p>
                        { "It’s common to confuse authentication and authorization because they seem like a single experience to users. They are two separate processes: authentication proves a user’s identity, while authorization grants or denies the user’s access to certain resources. " }
                    </p>

                    <p>
                        { "You can think of authentication and authorization as the security system for an office building. Users are the people who want to enter the building. Resources that people want to access are areas in the building: floors, rooms, and so on. " }
                    </p>

                    <p>
                        { "Authentication: When you enter the building, you must show your photo ID badge to the security guard. The guard compares the photo on the badge to your face. If they match, the guard lets you through the door to try to access different areas of the building. The guard doesn’t tell you what rooms you can access; they only get proof that you are who you claim to be. This is authentication: confirming user identity. " }
                    </p>

                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/introduction-to-iam/authentication-building.png"
                    />

                    <p>
                        { "Authorization: In this scenario, imagine the elevators and doorways in the building have key sensors for access. The chip in your badge gives you access only to the first floor, which your company occupies. If you swipe your badge to enter any other floor, your access is denied. You can access your private office but not those belonging to your colleagues. You can enter the supply room but not the server room. This is authorization: granting and denying access to different resources based on identity. " }
                    </p>

                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/introduction-to-iam/authorization-building.png"
                    />

                    <p>
                        { "To learn more about authentication and authorization, read Authentication vs. Authorization." }
                    </p>

                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "What does IAM do?" }</h2>
                    <p>{ "Identity and access management gives you control over user validation and resource access: " }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "How users become a part of your system" }</li>
                        <li>{ "What user information to store" }</li>
                        <li>{ "How users can prove their identity" }</li>
                        <li>{ "When and how often users must prove their identity" }</li>
                        <li>{ "The experience of proving identity" }</li>
                        <li>{ "Who can and cannot access different resources" }</li>
                    </ul>
                    <p>{ "You integrate IAM with your application, API, device, data store, or other technology. This integration can be very simple. For example, your web application might rely entirely on Facebook for authentication, and have an all-or-nothing authorization policy. Your app performs a simple check: if a user isn’t currently logged in to Facebook in the current browser, you direct them to do so. Once authenticated, all users can access everything in your app. " }</p>
                    <p>{ "It’s unlikely that such a simple IAM solution would meet the needs of your users, organization, industry, or compliance standards. In real life, IAM is complex. Most systems require some combination of these capabilities:" }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Seamless signup and login experiences—Smooth and professional login and signup experiences occur within your app, with your brand’s look and language. " }</li>
                        <li>{ "Multiple sources of user identities—Users expect to be able to log in using a variety of social (such as Google or Linkedin), enterprise (such as Microsoft Active Directory), and other identity providers." }</li>
                        <li>{ "Multi-factor authentication (MFA)—In an age when passwords are often stolen, requiring additional proof of identity is the new standard. Fingerprint authentication and one-time passwords are examples of common authentication methods. To learn more, read Multi-Factor Authentication (MFA)." }</li>
                        <li>{ "Step-up authentication—Access to advanced capabilities and sensitive information require stronger proof of identity than everyday tasks and data. Step-up authentication requires additional identity verification for selected areas and features. To learn more, read Add Step-up Authentication." }</li>
                        <li>{ "Attack protection—Preventing bots and bad actors from breaking into your system is fundamental to cybersecurity. To learn more, read Attack Protection." }</li>
                        <li>{ "Role-based access control (RBAC)—As the number of users grows, managing the access of each individual quickly becomes impractical. With RBAC, people who have the same role have the same access to resources. To learn more, read Role-Based Access Control. " }</li>
                    </ul>
                    <p>{ "Facing this level of complexity, many developers rely on an IAM platform like Auth0 instead of building their own solutions. " }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "How does IAM work? " }</h2>
                    <p>{ "“Identity and access management” is not one clearly defined system. IAM is a discipline and a type of framework for solving the challenge of secure access to digital resources.  There’s no limit to the different approaches for implementing an IAM system. This section explores elements and practices in common implementations. " }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Identity providers" }</h2>
                    <p>{ "In the past, the standard for identity and access management was for a system to create and manage its own identity information for its users. Each time a user wanted to use a new web application, they filled in a form to create an account. The application stored all of their information, including login credentials, and performed its own authentication whenever a user signed in. " }</p>
                    <p>{ "As the internet grew and more and more applications became available, most people amassed countless user accounts, each with its own account name and password to remember. There are many applications that continue to work this way. But many others now rely on identity providers to reduce their development and maintenance burden and their users’ effort." }</p>
                    <p>{ "An identity provider creates, maintains, and manages identity information, and can provide authentication services to other applications. For example, Google Accounts is an identity provider. They store account information such as your user name, full name, job title, and email address. Slate online magazine lets you log in with Google (or another identity provider) rather than go through the steps of entering and storing your information anew. " }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/introduction-to-iam/Slate-login.png"
                    />
                    <p>{ "Identity providers don’t share your authentication credentials with the apps that rely on them. Slate, for example, doesn’t ever see your Google password. Google only lets Slate know that you’ve proven your identity. " }</p>
                    <p>{ "Other identity providers include social media (such as Facebook or LinkedIn), enterprise (such as Microsoft Active Directory), and legal identity providers (such as Swedish BankID)." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Authentication factors" }</h2>
                    <p>{ "Authentication factors are methods for proving a user’s identity. They commonly fall into these basic types:" }</p>
                    <table class="uk-table uk-table-divider">
                        <thead>
                            <tr>
                                <th class="uk-text-emphasis">{ "Factor type" }</th>
                                <th class="uk-text-emphasis">{ "Examples" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    { "Knowledge (something you know)" }
                                </td>
                                <td>{ "Pin, password" }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Possession (something you have)" }
                                </td>
                                <td>{ "Mobile phone, encryption key device" }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Inherence (something you are)" }
                                </td>
                                <td>{ "Fingerprint, facial recognition, iris scan" }</td>
                            </tr>
                        </tbody>
                    </table>
                    <p>{ "IAM systems require one or many authentication factors to verify identity." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Authentication and authorization standards" }</h2>
                    <p>{ "Authentication and authorization standards are open specifications and protocols that provide guidance on how to:" }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Design IAM systems to manage identity" }</li>
                        <li>{ "Move personal data securely" }</li>
                        <li>{ "Decide who can access resources" }</li>
                    </ul>
                    <p>{ "These IAM industry standards are considered the most secure, reliable, and practical to implement:" }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Telkom Domain" }</h2>
                    <p>{ "Telkom Domain is a delegation protocol for accessing APIs and is the industry-standard protocol for IAM. An open authorization protocol, Telkom Domain lets an app access resources hosted by other web apps on behalf of a user without ever sharing the user’s credentials. It’s the standard that allows third-party developers to rely on large social platforms like Facebook, Google, and Twitter for login. To learn more, read OAuth 2.0 Authorization Framework." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Open ID Connect" }</h2>
                    <p>{ "A simple identity layer that sits on top of OAuth 2.0, OpenID Connect (OIDC) makes it easy to verify a user’s identity and obtain basic profile information from the identity provider. OIDC is another open standard protocol. To learn more, read OpenID Connect Protocol." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "JSON web tokens" }</h2>
                    <p>{ "JSON web tokens (JWTs) are an open standard that defines a compact and self-contained way for securely transmitting information between parties as a JSON object. JWTs can be verified and trusted because they’re digitally signed. They can be used to pass the identity of authenticated users between the identity provider and the service requesting the authentication. They also can be authenticated and encrypted. To learn more, read JSON Web Tokens." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Security Assertion Markup Language (SAML)" }</h2>
                    <p>{ "Security Assertion Markup Language (SAML) is an open-standard, XML-based data format that lets businesses communicate user authentication and authorization information to partner companies and enterprise applications that their employees use. To learn more, read SAML." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Web Services Federation (WS-Fed)" }</h2>
                    <p>{ "Developed by Microsoft and used extensively in their applications, this standard defines the way security tokens can be transported between different entities to exchange identity and authorization information. To learn more, read Web Services Federation Protocol." }</p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "Why use an IAM platform?" }</h2>
                    <p>{ "Why do so many developers choose to build on an identity and access management platform instead of building their own solution from the ground up?" }</p>
                    <p>{ "User expectations, customer requirements, and compliance standards introduce significant technical challenges. With multiple user sources, authentication factors, and open industry standards, the amount of knowledge and work required to build a typical IAM system can be enormous. A strong IAM platform has built-in support for all identity providers and authentication factors, offers APIs for easy integration with your software, and relies on the most secure industry standards for authentication and authorization." }</p>
                    <p>{ "For those who haven’t yet decided whether to build or buy an IAM solution, Build vs. Buy: Guide to Evaluating Identity Management is a useful resource." }</p>
                </div>

                
            </>
        }
    }
}
