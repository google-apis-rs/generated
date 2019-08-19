pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateAuthUriResponse {
        #[doc = "all providers the user has once used to do federated login"]
        #[serde(rename = "allProviders", default)]
        pub all_providers: Option<Vec<String>>,
        #[doc = "The URI used by the IDP to authenticate the user."]
        #[serde(rename = "authUri", default)]
        pub auth_uri: Option<String>,
        #[doc = "True if captcha is required."]
        #[serde(rename = "captchaRequired", default)]
        pub captcha_required: Option<bool>,
        #[doc = "True if the authUri is for user's existing provider."]
        #[serde(rename = "forExistingProvider", default)]
        pub for_existing_provider: Option<bool>,
        #[doc = "The fixed string identitytoolkit#CreateAuthUriResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The provider ID of the auth URI."]
        #[serde(rename = "providerId", default)]
        pub provider_id: Option<String>,
        #[doc = "Whether the user is registered if the identifier is an email."]
        #[serde(rename = "registered", default)]
        pub registered: Option<bool>,
        #[doc = "Session ID which should be passed in the following verifyAssertion request."]
        #[serde(rename = "sessionId", default)]
        pub session_id: Option<String>,
        #[doc = "All sign-in methods this user has used."]
        #[serde(rename = "signinMethods", default)]
        pub signin_methods: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for CreateAuthUriResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteAccountResponse {
        #[doc = "The fixed string \"identitytoolkit#DeleteAccountResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeleteAccountResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DownloadAccountResponse {
        #[doc = "The fixed string \"identitytoolkit#DownloadAccountResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The next page token. To be used in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The user accounts data."]
        #[serde(rename = "users", default)]
        pub users: Option<Vec<crate::schemas::UserInfo>>,
    }
    impl ::field_selector::FieldSelector for DownloadAccountResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EmailLinkSigninResponse {
        #[doc = "The user's email."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[doc = "The STS id token to login the newly signed in user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "Whether the user is new."]
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: Option<bool>,
        #[doc = "The fixed string \"identitytoolkit#EmailLinkSigninResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The RP local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "The refresh token for the signed in user."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for EmailLinkSigninResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EmailTemplate {
        #[doc = "Email body."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "Email body format."]
        #[serde(rename = "format", default)]
        pub format: Option<String>,
        #[doc = "From address of the email."]
        #[serde(rename = "from", default)]
        pub from: Option<String>,
        #[doc = "From display name."]
        #[serde(rename = "fromDisplayName", default)]
        pub from_display_name: Option<String>,
        #[doc = "Reply-to address."]
        #[serde(rename = "replyTo", default)]
        pub reply_to: Option<String>,
        #[doc = "Subject of the email."]
        #[serde(rename = "subject", default)]
        pub subject: Option<String>,
    }
    impl ::field_selector::FieldSelector for EmailTemplate {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GetAccountInfoResponse {
        #[doc = "The fixed string \"identitytoolkit#GetAccountInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The info of the users."]
        #[serde(rename = "users", default)]
        pub users: Option<Vec<crate::schemas::UserInfo>>,
    }
    impl ::field_selector::FieldSelector for GetAccountInfoResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetOobConfirmationCodeResponse {
        #[doc = "The email address that the email is sent to."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The fixed string \"identitytoolkit#GetOobConfirmationCodeResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The code to be send to the user."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetOobConfirmationCodeResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetRecaptchaParamResponse {
        #[doc = "The fixed string \"identitytoolkit#GetRecaptchaParamResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Site key registered at recaptcha."]
        #[serde(rename = "recaptchaSiteKey", default)]
        pub recaptcha_site_key: Option<String>,
        #[doc = "The stoken field for the recaptcha widget, used to request captcha challenge."]
        #[serde(rename = "recaptchaStoken", default)]
        pub recaptcha_stoken: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetRecaptchaParamResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyCreateAuthUriRequest {
        #[doc = "The app ID of the mobile app, base64(CERT_SHA1):PACKAGE_NAME for Android, BUNDLE_ID for iOS."]
        #[serde(rename = "appId", default)]
        pub app_id: Option<String>,
        #[doc = "Explicitly specify the auth flow type. Currently only support \"CODE_FLOW\" type. The field is only used for Google provider."]
        #[serde(rename = "authFlowType", default)]
        pub auth_flow_type: Option<String>,
        #[doc = "The relying party OAuth client ID."]
        #[serde(rename = "clientId", default)]
        pub client_id: Option<String>,
        #[doc = "The opaque value used by the client to maintain context info between the authentication request and the IDP callback."]
        #[serde(rename = "context", default)]
        pub context: Option<String>,
        #[doc = "The URI to which the IDP redirects the user after the federated login flow."]
        #[serde(rename = "continueUri", default)]
        pub continue_uri: Option<String>,
        #[doc = "The query parameter that client can customize by themselves in auth url. The following parameters are reserved for server so that they cannot be customized by clients: client_id, response_type, scope, redirect_uri, state, oauth_token."]
        #[serde(rename = "customParameter", default)]
        pub custom_parameter: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The hosted domain to restrict sign-in to accounts at that domain for Google Apps hosted accounts."]
        #[serde(rename = "hostedDomain", default)]
        pub hosted_domain: Option<String>,
        #[doc = "The email or federated ID of the user."]
        #[serde(rename = "identifier", default)]
        pub identifier: Option<String>,
        #[doc = "The developer's consumer key for OpenId OAuth Extension"]
        #[serde(rename = "oauthConsumerKey", default)]
        pub oauth_consumer_key: Option<String>,
        #[doc = "Additional oauth scopes, beyond the basid user profile, that the user would be prompted to grant"]
        #[serde(rename = "oauthScope", default)]
        pub oauth_scope: Option<String>,
        #[doc = "Optional realm for OpenID protocol. The sub string \"scheme://domain:port\" of the param \"continueUri\" is used if this is not set."]
        #[serde(rename = "openidRealm", default)]
        pub openid_realm: Option<String>,
        #[doc = "The native app package for OTA installation."]
        #[serde(rename = "otaApp", default)]
        pub ota_app: Option<String>,
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        #[serde(rename = "providerId", default)]
        pub provider_id: Option<String>,
        #[doc = "The session_id passed by client."]
        #[serde(rename = "sessionId", default)]
        pub session_id: Option<String>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: Option<u64>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyCreateAuthUriRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyDeleteAccountRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "The GITKit token or STS id token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyDeleteAccountRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyDownloadAccountRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "The max number of results to return in the response."]
        #[serde(rename = "maxResults", default)]
        pub max_results: Option<u32>,
        #[doc = "The token for the next page. This should be taken from the previous response."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "Specify which project (field value is actually project id) to operate. Only used when provided credential."]
        #[serde(rename = "targetProjectId", default)]
        pub target_project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyDownloadAccountRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
        #[doc = "The email address of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Token for linking flow."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The confirmation code."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyGetAccountInfoRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "The list of emails of the users to inquiry."]
        #[serde(rename = "email", default)]
        pub email: Option<Vec<String>>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The list of local ID's of the users to inquiry."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<Vec<String>>,
        #[doc = "Privileged caller can query users by specified phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyGetAccountInfoRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyGetProjectConfigResponse {
        #[doc = "Whether to allow password user sign in or sign up."]
        #[serde(rename = "allowPasswordUser", default)]
        pub allow_password_user: Option<bool>,
        #[doc = "Browser API key, needed when making http request to Apiary."]
        #[serde(rename = "apiKey", default)]
        pub api_key: Option<String>,
        #[doc = "Authorized domains."]
        #[serde(rename = "authorizedDomains", default)]
        pub authorized_domains: Option<Vec<String>>,
        #[doc = "Change email template."]
        #[serde(rename = "changeEmailTemplate", default)]
        pub change_email_template: Option<crate::schemas::EmailTemplate>,
        #[serde(rename = "dynamicLinksDomain", default)]
        pub dynamic_links_domain: Option<String>,
        #[doc = "Whether anonymous user is enabled."]
        #[serde(rename = "enableAnonymousUser", default)]
        pub enable_anonymous_user: Option<bool>,
        #[doc = "OAuth2 provider configuration."]
        #[serde(rename = "idpConfig", default)]
        pub idp_config: Option<Vec<crate::schemas::IdpConfig>>,
        #[doc = "Legacy reset password email template."]
        #[serde(rename = "legacyResetPasswordTemplate", default)]
        pub legacy_reset_password_template: Option<crate::schemas::EmailTemplate>,
        #[doc = "Project ID of the relying party."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Reset password email template."]
        #[serde(rename = "resetPasswordTemplate", default)]
        pub reset_password_template: Option<crate::schemas::EmailTemplate>,
        #[doc = "Whether to use email sending provided by Firebear."]
        #[serde(rename = "useEmailSending", default)]
        pub use_email_sending: Option<bool>,
        #[doc = "Verify email template."]
        #[serde(rename = "verifyEmailTemplate", default)]
        pub verify_email_template: Option<crate::schemas::EmailTemplate>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyGetProjectConfigResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyResetPasswordRequest {
        #[doc = "The email address of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The new password inputted by the user."]
        #[serde(rename = "newPassword", default)]
        pub new_password: Option<String>,
        #[doc = "The old password inputted by the user."]
        #[serde(rename = "oldPassword", default)]
        pub old_password: Option<String>,
        #[doc = "The confirmation code."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyResetPasswordRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySendVerificationCodeRequest {
        #[doc = "Receipt of successful app token validation with APNS."]
        #[serde(rename = "iosReceipt", default)]
        pub ios_receipt: Option<String>,
        #[doc = "Secret delivered to iOS app via APNS."]
        #[serde(rename = "iosSecret", default)]
        pub ios_secret: Option<String>,
        #[doc = "The phone number to send the verification code to in E.164 format."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[doc = "Recaptcha solution."]
        #[serde(rename = "recaptchaToken", default)]
        pub recaptcha_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySendVerificationCodeRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySendVerificationCodeResponse {
        #[doc = "Encrypted session information"]
        #[serde(rename = "sessionInfo", default)]
        pub session_info: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySendVerificationCodeResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySetAccountInfoRequest {
        #[doc = "The captcha challenge."]
        #[serde(rename = "captchaChallenge", default)]
        pub captcha_challenge: Option<String>,
        #[doc = "Response to the captcha."]
        #[serde(rename = "captchaResponse", default)]
        pub captcha_response: Option<String>,
        #[doc = "The timestamp when the account is created."]
        #[serde(rename = "createdAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub created_at: Option<i64>,
        #[doc = "The custom attributes to be set in the user's id token."]
        #[serde(rename = "customAttributes", default)]
        pub custom_attributes: Option<String>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "The attributes users request to delete."]
        #[serde(rename = "deleteAttribute", default)]
        pub delete_attribute: Option<Vec<String>>,
        #[doc = "The IDPs the user request to delete."]
        #[serde(rename = "deleteProvider", default)]
        pub delete_provider: Option<Vec<String>>,
        #[doc = "Whether to disable the user."]
        #[serde(rename = "disableUser", default)]
        pub disable_user: Option<bool>,
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Mark the email as verified or not."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: Option<bool>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "Last login timestamp."]
        #[serde(rename = "lastLoginAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_login_at: Option<i64>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "The out-of-band code of the change email request."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: Option<String>,
        #[doc = "The new password of the user."]
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[doc = "Privileged caller can update user with specified phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[doc = "The photo url of the user."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The associated IDPs of the user."]
        #[serde(rename = "provider", default)]
        pub provider: Option<Vec<String>>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: Option<bool>,
        #[doc = "Mark the user to upgrade to federated login."]
        #[serde(rename = "upgradeToFederatedLogin", default)]
        pub upgrade_to_federated_login: Option<bool>,
        #[doc = "Timestamp in seconds for valid login token."]
        #[serde(rename = "validSince", default)]
        #[serde(with = "crate::parsed_string")]
        pub valid_since: Option<i64>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySetAccountInfoRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySetProjectConfigRequest {
        #[doc = "Whether to allow password user sign in or sign up."]
        #[serde(rename = "allowPasswordUser", default)]
        pub allow_password_user: Option<bool>,
        #[doc = "Browser API key, needed when making http request to Apiary."]
        #[serde(rename = "apiKey", default)]
        pub api_key: Option<String>,
        #[doc = "Authorized domains for widget redirect."]
        #[serde(rename = "authorizedDomains", default)]
        pub authorized_domains: Option<Vec<String>>,
        #[doc = "Change email template."]
        #[serde(rename = "changeEmailTemplate", default)]
        pub change_email_template: Option<crate::schemas::EmailTemplate>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "Whether to enable anonymous user."]
        #[serde(rename = "enableAnonymousUser", default)]
        pub enable_anonymous_user: Option<bool>,
        #[doc = "Oauth2 provider configuration."]
        #[serde(rename = "idpConfig", default)]
        pub idp_config: Option<Vec<crate::schemas::IdpConfig>>,
        #[doc = "Legacy reset password email template."]
        #[serde(rename = "legacyResetPasswordTemplate", default)]
        pub legacy_reset_password_template: Option<crate::schemas::EmailTemplate>,
        #[doc = "Reset password email template."]
        #[serde(rename = "resetPasswordTemplate", default)]
        pub reset_password_template: Option<crate::schemas::EmailTemplate>,
        #[doc = "Whether to use email sending provided by Firebear."]
        #[serde(rename = "useEmailSending", default)]
        pub use_email_sending: Option<bool>,
        #[doc = "Verify email template."]
        #[serde(rename = "verifyEmailTemplate", default)]
        pub verify_email_template: Option<crate::schemas::EmailTemplate>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySetProjectConfigRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySetProjectConfigResponse {
        #[doc = "Project ID of the relying party."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySetProjectConfigResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySignOutUserRequest {
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySignOutUserRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySignOutUserResponse {
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySignOutUserResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartySignupNewUserRequest {
        #[doc = "The captcha challenge."]
        #[serde(rename = "captchaChallenge", default)]
        pub captcha_challenge: Option<String>,
        #[doc = "Response to the captcha."]
        #[serde(rename = "captchaResponse", default)]
        pub captcha_response: Option<String>,
        #[doc = "Whether to disable the user. Only can be used by service account."]
        #[serde(rename = "disabled", default)]
        pub disabled: Option<bool>,
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Mark the email as verified or not. Only can be used by service account."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: Option<bool>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "Privileged caller can create user with specified user id."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "The new password of the user."]
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[doc = "Privileged caller can create user with specified phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[doc = "The photo url of the user."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: Option<u64>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartySignupNewUserRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyUploadAccountRequest {
        #[doc = "Whether allow overwrite existing account when user local_id exists."]
        #[serde(rename = "allowOverwrite", default)]
        pub allow_overwrite: Option<bool>,
        #[serde(rename = "blockSize", default)]
        pub block_size: Option<i32>,
        #[doc = "The following 4 fields are for standard scrypt algorithm."]
        #[serde(rename = "cpuMemCost", default)]
        pub cpu_mem_cost: Option<i32>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[serde(rename = "dkLen", default)]
        pub dk_len: Option<i32>,
        #[doc = "The password hash algorithm."]
        #[serde(rename = "hashAlgorithm", default)]
        pub hash_algorithm: Option<String>,
        #[doc = "Memory cost for hash calculation. Used by scrypt similar algorithms."]
        #[serde(rename = "memoryCost", default)]
        pub memory_cost: Option<i32>,
        #[serde(rename = "parallelization", default)]
        pub parallelization: Option<i32>,
        #[doc = "Rounds for hash calculation. Used by scrypt and similar algorithms."]
        #[serde(rename = "rounds", default)]
        pub rounds: Option<i32>,
        #[doc = "The salt separator."]
        #[serde(rename = "saltSeparator", default)]
        pub salt_separator: Option<Vec<u8>>,
        #[doc = "If true, backend will do sanity check(including duplicate email and federated id) when uploading account."]
        #[serde(rename = "sanityCheck", default)]
        pub sanity_check: Option<bool>,
        #[doc = "The key for to hash the password."]
        #[serde(rename = "signerKey", default)]
        pub signer_key: Option<Vec<u8>>,
        #[doc = "Specify which project (field value is actually project id) to operate. Only used when provided credential."]
        #[serde(rename = "targetProjectId", default)]
        pub target_project_id: Option<String>,
        #[doc = "The account info to be stored."]
        #[serde(rename = "users", default)]
        pub users: Option<Vec<crate::schemas::UserInfo>>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyUploadAccountRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        #[doc = "When it's true, automatically creates a new account if the user doesn't exist. When it's false, allows existing user to sign in normally and throws exception if the user doesn't exist."]
        #[serde(rename = "autoCreate", default)]
        pub auto_create: Option<bool>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "The GITKit token for the non-trusted IDP pending to be confirmed by the user."]
        #[serde(rename = "pendingIdToken", default)]
        pub pending_id_token: Option<String>,
        #[doc = "The post body if the request is a HTTP POST."]
        #[serde(rename = "postBody", default)]
        pub post_body: Option<String>,
        #[doc = "The URI to which the IDP redirects the user back. It may contain federated login result params added by the IDP."]
        #[serde(rename = "requestUri", default)]
        pub request_uri: Option<String>,
        #[doc = "Whether return 200 and IDP credential rather than throw exception when federated id is already linked."]
        #[serde(rename = "returnIdpCredential", default)]
        pub return_idp_credential: Option<bool>,
        #[doc = "Whether to return refresh tokens."]
        #[serde(rename = "returnRefreshToken", default)]
        pub return_refresh_token: Option<bool>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: Option<bool>,
        #[doc = "Session ID, which should match the one in previous createAuthUri request."]
        #[serde(rename = "sessionId", default)]
        pub session_id: Option<String>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: Option<u64>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: Option<bool>,
        #[doc = "The custom token to verify"]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyVerifyPasswordRequest {
        #[doc = "The captcha challenge."]
        #[serde(rename = "captchaChallenge", default)]
        pub captcha_challenge: Option<String>,
        #[doc = "Response to the captcha."]
        #[serde(rename = "captchaResponse", default)]
        pub captcha_response: Option<String>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: Option<i64>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "The password inputed by the user."]
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[doc = "The GITKit token for the non-trusted IDP, which is to be confirmed by the user."]
        #[serde(rename = "pendingIdToken", default)]
        pub pending_id_token: Option<String>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: Option<bool>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: Option<u64>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyPasswordRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
        #[serde(rename = "code", default)]
        pub code: Option<String>,
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[serde(rename = "operation", default)]
        pub operation: Option<String>,
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[doc = "The session info previously returned by IdentityToolkit-SendVerificationCode."]
        #[serde(rename = "sessionInfo", default)]
        pub session_info: Option<String>,
        #[serde(rename = "temporaryProof", default)]
        pub temporary_proof: Option<String>,
        #[serde(rename = "verificationProof", default)]
        pub verification_proof: Option<String>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: Option<bool>,
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
        #[serde(rename = "temporaryProof", default)]
        pub temporary_proof: Option<String>,
        #[serde(rename = "temporaryProofExpiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub temporary_proof_expires_in: Option<i64>,
        #[serde(rename = "verificationProof", default)]
        pub verification_proof: Option<String>,
        #[serde(rename = "verificationProofExpiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub verification_proof_expires_in: Option<i64>,
    }
    impl ::field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdpConfig {
        #[doc = "OAuth2 client ID."]
        #[serde(rename = "clientId", default)]
        pub client_id: Option<String>,
        #[doc = "Whether this IDP is enabled."]
        #[serde(rename = "enabled", default)]
        pub enabled: Option<bool>,
        #[doc = "Percent of users who will be prompted/redirected federated login for this IDP."]
        #[serde(rename = "experimentPercent", default)]
        pub experiment_percent: Option<i32>,
        #[doc = "OAuth2 provider."]
        #[serde(rename = "provider", default)]
        pub provider: Option<String>,
        #[doc = "OAuth2 client secret."]
        #[serde(rename = "secret", default)]
        pub secret: Option<String>,
        #[doc = "Whitelisted client IDs for audience check."]
        #[serde(rename = "whitelistedAudiences", default)]
        pub whitelisted_audiences: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for IdpConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Relyingparty {
        #[doc = "whether or not to install the android app on the device where the link is opened"]
        #[serde(rename = "androidInstallApp", default)]
        pub android_install_app: Option<bool>,
        #[doc = "minimum version of the app. if the version on the device is lower than this version then the user is taken to the play store to upgrade the app"]
        #[serde(rename = "androidMinimumVersion", default)]
        pub android_minimum_version: Option<String>,
        #[doc = "android package name of the android app to handle the action code"]
        #[serde(rename = "androidPackageName", default)]
        pub android_package_name: Option<String>,
        #[doc = "whether or not the app can handle the oob code without first going to web"]
        #[serde(rename = "canHandleCodeInApp", default)]
        pub can_handle_code_in_app: Option<bool>,
        #[doc = "The recaptcha response from the user."]
        #[serde(rename = "captchaResp", default)]
        pub captcha_resp: Option<String>,
        #[doc = "The recaptcha challenge presented to the user."]
        #[serde(rename = "challenge", default)]
        pub challenge: Option<String>,
        #[doc = "The url to continue to the Gitkit app"]
        #[serde(rename = "continueUrl", default)]
        pub continue_url: Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "iOS app store id to download the app if it's not already installed"]
        #[serde(rename = "iOSAppStoreId", default)]
        pub i_os_app_store_id: Option<String>,
        #[doc = "the iOS bundle id of iOS app to handle the action code"]
        #[serde(rename = "iOSBundleId", default)]
        pub i_os_bundle_id: Option<String>,
        #[doc = "The user's Gitkit login token for email change."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The fixed string \"identitytoolkit#relyingparty\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The new email if the code is for email change."]
        #[serde(rename = "newEmail", default)]
        pub new_email: Option<String>,
        #[doc = "The request type."]
        #[serde(rename = "requestType", default)]
        pub request_type: Option<String>,
        #[doc = "The IP address of the user."]
        #[serde(rename = "userIp", default)]
        pub user_ip: Option<String>,
    }
    impl ::field_selector::FieldSelector for Relyingparty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResetPasswordResponse {
        #[doc = "The user's email. If the out-of-band code is for email recovery, the user's original email."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The fixed string \"identitytoolkit#ResetPasswordResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "If the out-of-band code is for email recovery, the user's new email."]
        #[serde(rename = "newEmail", default)]
        pub new_email: Option<String>,
        #[doc = "The request type."]
        #[serde(rename = "requestType", default)]
        pub request_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for ResetPasswordResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SetAccountInfoResponse {
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "If email has been verified."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: Option<bool>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[doc = "The Gitkit id token to login the newly sign up user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The fixed string \"identitytoolkit#SetAccountInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "The new email the user attempts to change to."]
        #[serde(rename = "newEmail", default)]
        pub new_email: Option<String>,
        #[doc = "The user's hashed password."]
        #[serde(rename = "passwordHash", default)]
        pub password_hash: Option<Vec<u8>>,
        #[doc = "The photo url of the user."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The user's profiles at the associated IdPs."]
        #[serde(rename = "providerUserInfo", default)]
        pub provider_user_info:
            Option<Vec<crate::schemas::SetAccountInfoResponseProviderUserInfoItems>>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for SetAccountInfoResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SetAccountInfoResponseProviderUserInfoItems {
        #[doc = "The user's display name at the IDP."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "User's identifier at IDP."]
        #[serde(rename = "federatedId", default)]
        pub federated_id: Option<String>,
        #[doc = "The user's photo url at the IDP."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The IdP ID. For whitelisted IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        #[serde(rename = "providerId", default)]
        pub provider_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for SetAccountInfoResponseProviderUserInfoItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SignupNewUserResponse {
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[doc = "The Gitkit id token to login the newly sign up user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The fixed string \"identitytoolkit#SignupNewUserResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The RP local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for SignupNewUserResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UploadAccountResponse {
        #[doc = "The error encountered while processing the account info."]
        #[serde(rename = "error", default)]
        pub error: Option<Vec<crate::schemas::UploadAccountResponseErrorItems>>,
        #[doc = "The fixed string \"identitytoolkit#UploadAccountResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for UploadAccountResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UploadAccountResponseErrorItems {
        #[doc = "The index of the malformed account, starting from 0."]
        #[serde(rename = "index", default)]
        pub index: Option<i32>,
        #[doc = "Detailed error message for the account info."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
    }
    impl ::field_selector::FieldSelector for UploadAccountResponseErrorItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UserInfo {
        #[doc = "User creation timestamp."]
        #[serde(rename = "createdAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub created_at: Option<i64>,
        #[doc = "The custom attributes to be set in the user's id token."]
        #[serde(rename = "customAttributes", default)]
        pub custom_attributes: Option<String>,
        #[doc = "Whether the user is authenticated by the developer."]
        #[serde(rename = "customAuth", default)]
        pub custom_auth: Option<bool>,
        #[doc = "Whether the user is disabled."]
        #[serde(rename = "disabled", default)]
        pub disabled: Option<bool>,
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Whether the email has been verified."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: Option<bool>,
        #[doc = "last login timestamp."]
        #[serde(rename = "lastLoginAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_login_at: Option<i64>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "The user's hashed password."]
        #[serde(rename = "passwordHash", default)]
        pub password_hash: Option<Vec<u8>>,
        #[doc = "The timestamp when the password was last updated."]
        #[serde(rename = "passwordUpdatedAt", default)]
        pub password_updated_at: Option<f64>,
        #[doc = "User's phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[doc = "The URL of the user profile photo."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The IDP of the user."]
        #[serde(rename = "providerUserInfo", default)]
        pub provider_user_info: Option<Vec<crate::schemas::UserInfoProviderUserInfoItems>>,
        #[doc = "The user's plain text password."]
        #[serde(rename = "rawPassword", default)]
        pub raw_password: Option<String>,
        #[doc = "The user's password salt."]
        #[serde(rename = "salt", default)]
        pub salt: Option<Vec<u8>>,
        #[doc = "User's screen name at Twitter or login name at Github."]
        #[serde(rename = "screenName", default)]
        pub screen_name: Option<String>,
        #[doc = "Timestamp in seconds for valid login token."]
        #[serde(rename = "validSince", default)]
        #[serde(with = "crate::parsed_string")]
        pub valid_since: Option<i64>,
        #[doc = "Version of the user's password."]
        #[serde(rename = "version", default)]
        pub version: Option<i32>,
    }
    impl ::field_selector::FieldSelector for UserInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserInfoProviderUserInfoItems {
        #[doc = "The user's display name at the IDP."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "User's email at IDP."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "User's identifier at IDP."]
        #[serde(rename = "federatedId", default)]
        pub federated_id: Option<String>,
        #[doc = "User's phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
        #[doc = "The user's photo url at the IDP."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        #[serde(rename = "providerId", default)]
        pub provider_id: Option<String>,
        #[doc = "User's raw identifier directly returned from IDP."]
        #[serde(rename = "rawId", default)]
        pub raw_id: Option<String>,
        #[doc = "User's screen name at Twitter or login name at Github."]
        #[serde(rename = "screenName", default)]
        pub screen_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for UserInfoProviderUserInfoItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VerifyAssertionResponse {
        #[doc = "The action code."]
        #[serde(rename = "action", default)]
        pub action: Option<String>,
        #[doc = "URL for OTA app installation."]
        #[serde(rename = "appInstallationUrl", default)]
        pub app_installation_url: Option<String>,
        #[doc = "The custom scheme used by mobile app."]
        #[serde(rename = "appScheme", default)]
        pub app_scheme: Option<String>,
        #[doc = "The opaque value used by the client to maintain context info between the authentication request and the IDP callback."]
        #[serde(rename = "context", default)]
        pub context: Option<String>,
        #[doc = "The birth date of the IdP account."]
        #[serde(rename = "dateOfBirth", default)]
        pub date_of_birth: Option<String>,
        #[doc = "The display name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email returned by the IdP. NOTE: The federated login user may not own the email."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "It's true if the email is recycled."]
        #[serde(rename = "emailRecycled", default)]
        pub email_recycled: Option<bool>,
        #[doc = "The value is true if the IDP is also the email provider. It means the user owns the email."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: Option<bool>,
        #[doc = "Client error code."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[doc = "The unique ID identifies the IdP account."]
        #[serde(rename = "federatedId", default)]
        pub federated_id: Option<String>,
        #[doc = "The first name of the user."]
        #[serde(rename = "firstName", default)]
        pub first_name: Option<String>,
        #[doc = "The full name of the user."]
        #[serde(rename = "fullName", default)]
        pub full_name: Option<String>,
        #[doc = "The ID token."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "It's the identifier param in the createAuthUri request if the identifier is an email. It can be used to check whether the user input email is different from the asserted email."]
        #[serde(rename = "inputEmail", default)]
        pub input_email: Option<String>,
        #[doc = "True if it's a new user sign-in, false if it's a returning user."]
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: Option<bool>,
        #[doc = "The fixed string \"identitytoolkit#VerifyAssertionResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The language preference of the user."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
        #[doc = "The last name of the user."]
        #[serde(rename = "lastName", default)]
        pub last_name: Option<String>,
        #[doc = "The RP local ID if it's already been mapped to the IdP account identified by the federated ID."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "Whether the assertion is from a non-trusted IDP and need account linking confirmation."]
        #[serde(rename = "needConfirmation", default)]
        pub need_confirmation: Option<bool>,
        #[doc = "Whether need client to supply email to complete the federated login flow."]
        #[serde(rename = "needEmail", default)]
        pub need_email: Option<bool>,
        #[doc = "The nick name of the user."]
        #[serde(rename = "nickName", default)]
        pub nick_name: Option<String>,
        #[doc = "The OAuth2 access token."]
        #[serde(rename = "oauthAccessToken", default)]
        pub oauth_access_token: Option<String>,
        #[doc = "The OAuth2 authorization code."]
        #[serde(rename = "oauthAuthorizationCode", default)]
        pub oauth_authorization_code: Option<String>,
        #[doc = "The lifetime in seconds of the OAuth2 access token."]
        #[serde(rename = "oauthExpireIn", default)]
        pub oauth_expire_in: Option<i32>,
        #[doc = "The OIDC id token."]
        #[serde(rename = "oauthIdToken", default)]
        pub oauth_id_token: Option<String>,
        #[doc = "The user approved request token for the OpenID OAuth extension."]
        #[serde(rename = "oauthRequestToken", default)]
        pub oauth_request_token: Option<String>,
        #[doc = "The scope for the OpenID OAuth extension."]
        #[serde(rename = "oauthScope", default)]
        pub oauth_scope: Option<String>,
        #[doc = "The OAuth1 access token secret."]
        #[serde(rename = "oauthTokenSecret", default)]
        pub oauth_token_secret: Option<String>,
        #[doc = "The original email stored in the mapping storage. It's returned when the federated ID is associated to a different email."]
        #[serde(rename = "originalEmail", default)]
        pub original_email: Option<String>,
        #[doc = "The URI of the public accessible profiel picture."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. If the \"providerId\" param is set to OpenID OP identifer other than the whilte listed IdPs the OP identifier is returned. If the \"identifier\" param is federated ID in the createAuthUri request. The domain part of the federated ID is returned."]
        #[serde(rename = "providerId", default)]
        pub provider_id: Option<String>,
        #[doc = "Raw IDP-returned user info."]
        #[serde(rename = "rawUserInfo", default)]
        pub raw_user_info: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
        #[doc = "The screen_name of a Twitter user or the login name at Github."]
        #[serde(rename = "screenName", default)]
        pub screen_name: Option<String>,
        #[doc = "The timezone of the user."]
        #[serde(rename = "timeZone", default)]
        pub time_zone: Option<String>,
        #[doc = "When action is 'map', contains the idps which can be used for confirmation."]
        #[serde(rename = "verifiedProvider", default)]
        pub verified_provider: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for VerifyAssertionResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VerifyCustomTokenResponse {
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[doc = "The GITKit token for authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "True if it's a new user sign-in, false if it's a returning user."]
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: Option<bool>,
        #[doc = "The fixed string \"identitytoolkit#VerifyCustomTokenResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for VerifyCustomTokenResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VerifyPasswordResponse {
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email returned by the IdP. NOTE: The federated login user may not own the email."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: Option<i64>,
        #[doc = "The GITKit token for authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: Option<String>,
        #[doc = "The fixed string \"identitytoolkit#VerifyPasswordResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The RP local ID if it's already been mapped to the IdP account identified by the federated ID."]
        #[serde(rename = "localId", default)]
        pub local_id: Option<String>,
        #[doc = "The OAuth2 access token."]
        #[serde(rename = "oauthAccessToken", default)]
        pub oauth_access_token: Option<String>,
        #[doc = "The OAuth2 authorization code."]
        #[serde(rename = "oauthAuthorizationCode", default)]
        pub oauth_authorization_code: Option<String>,
        #[doc = "The lifetime in seconds of the OAuth2 access token."]
        #[serde(rename = "oauthExpireIn", default)]
        pub oauth_expire_in: Option<i32>,
        #[doc = "The URI of the user's photo at IdP"]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: Option<String>,
        #[doc = "Whether the email is registered."]
        #[serde(rename = "registered", default)]
        pub registered: Option<bool>,
    }
    impl ::field_selector::FieldSelector for VerifyPasswordResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the relyingparty resource"]
    pub fn relyingparty(&self) -> crate::relyingparty::RelyingpartyActions<A> {
        crate::relyingparty::RelyingpartyActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod relyingparty {
    pub mod params {}
    pub struct RelyingpartyActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> RelyingpartyActions<'a, A> {
        #[doc = "Creates the URI used by the IdP to authenticate the user."]
        pub fn create_auth_uri(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyCreateAuthUriRequest,
        ) -> CreateAuthUriRequestBuilder<A> {
            CreateAuthUriRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Delete user account."]
        pub fn delete_account(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyDeleteAccountRequest,
        ) -> DeleteAccountRequestBuilder<A> {
            DeleteAccountRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Batch download user accounts."]
        pub fn download_account(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyDownloadAccountRequest,
        ) -> DownloadAccountRequestBuilder<A> {
            DownloadAccountRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Reset password for a user."]
        pub fn email_link_signin(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyEmailLinkSigninRequest,
        ) -> EmailLinkSigninRequestBuilder<A> {
            EmailLinkSigninRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Returns the account info."]
        pub fn get_account_info(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyGetAccountInfoRequest,
        ) -> GetAccountInfoRequestBuilder<A> {
            GetAccountInfoRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Get a code for user action confirmation."]
        pub fn get_oob_confirmation_code(
            &self,
            request: crate::schemas::Relyingparty,
        ) -> GetOobConfirmationCodeRequestBuilder<A> {
            GetOobConfirmationCodeRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Get project configuration."]
        pub fn get_project_config(&self) -> GetProjectConfigRequestBuilder<A> {
            GetProjectConfigRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                delegated_project_number: None,
                project_number: None,
            }
        }
        #[doc = "Get token signing public key."]
        pub fn get_public_keys(&self) -> GetPublicKeysRequestBuilder<A> {
            GetPublicKeysRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Get recaptcha secure param."]
        pub fn get_recaptcha_param(&self) -> GetRecaptchaParamRequestBuilder<A> {
            GetRecaptchaParamRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Reset password for a user."]
        pub fn reset_password(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyResetPasswordRequest,
        ) -> ResetPasswordRequestBuilder<A> {
            ResetPasswordRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Send SMS verification code."]
        pub fn send_verification_code(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeRequest,
        ) -> SendVerificationCodeRequestBuilder<A> {
            SendVerificationCodeRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Set account info for a user."]
        pub fn set_account_info(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartySetAccountInfoRequest,
        ) -> SetAccountInfoRequestBuilder<A> {
            SetAccountInfoRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Set project configuration."]
        pub fn set_project_config(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigRequest,
        ) -> SetProjectConfigRequestBuilder<A> {
            SetProjectConfigRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Sign out user."]
        pub fn sign_out_user(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartySignOutUserRequest,
        ) -> SignOutUserRequestBuilder<A> {
            SignOutUserRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Signup new user."]
        pub fn signup_new_user(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartySignupNewUserRequest,
        ) -> SignupNewUserRequestBuilder<A> {
            SignupNewUserRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Batch upload existing user accounts."]
        pub fn upload_account(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyUploadAccountRequest,
        ) -> UploadAccountRequestBuilder<A> {
            UploadAccountRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Verifies the assertion returned by the IdP."]
        pub fn verify_assertion(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyAssertionRequest,
        ) -> VerifyAssertionRequestBuilder<A> {
            VerifyAssertionRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Verifies the developer asserted ID token."]
        pub fn verify_custom_token(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyCustomTokenRequest,
        ) -> VerifyCustomTokenRequestBuilder<A> {
            VerifyCustomTokenRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Verifies the user entered password."]
        pub fn verify_password(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPasswordRequest,
        ) -> VerifyPasswordRequestBuilder<A> {
            VerifyPasswordRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Verifies ownership of a phone number and creates/updates the user account accordingly."]
        pub fn verify_phone_number(
            &self,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest,
        ) -> VerifyPhoneNumberRequestBuilder<A> {
            VerifyPhoneNumberRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateAuthUriRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyCreateAuthUriRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateAuthUriRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::CreateAuthUriResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("createAuthUri");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteAccountRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyDeleteAccountRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteAccountRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DeleteAccountResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("deleteAccount");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DownloadAccountRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyDownloadAccountRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DownloadAccountRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DownloadAccountResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("downloadAccount");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct EmailLinkSigninRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyEmailLinkSigninRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> EmailLinkSigninRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::EmailLinkSigninResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("emailLinkSignin");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetAccountInfoRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyGetAccountInfoRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetAccountInfoRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GetAccountInfoResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("getAccountInfo");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetOobConfirmationCodeRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Relyingparty,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetOobConfirmationCodeRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GetOobConfirmationCodeResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("getOobConfirmationCode");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetProjectConfigRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        delegated_project_number: Option<String>,
        project_number: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetProjectConfigRequestBuilder<'a, A> {
        #[doc = "Delegated GCP project number of the request."]
        pub fn delegated_project_number(&mut self, value: impl Into<String>) -> &mut Self {
            self.delegated_project_number = Some(value.into());
            self
        }
        #[doc = "GCP project number of the request."]
        pub fn project_number(&mut self, value: impl Into<String>) -> &mut Self {
            self.project_number = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<
            crate::schemas::IdentitytoolkitRelyingpartyGetProjectConfigResponse,
            Box<dyn ::std::error::Error>,
        > {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("getProjectConfig");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("delegatedProjectNumber", &self.delegated_project_number)]);
            let req = req.query(&[("projectNumber", &self.project_number)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetPublicKeysRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetPublicKeysRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<::std::collections::BTreeMap<String, String>, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("publicKeys");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRecaptchaParamRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRecaptchaParamRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GetRecaptchaParamResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("getRecaptchaParam");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ResetPasswordRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyResetPasswordRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ResetPasswordRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ResetPasswordResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("resetPassword");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SendVerificationCodeRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SendVerificationCodeRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<
            crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeResponse,
            Box<dyn ::std::error::Error>,
        > {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("sendVerificationCode");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SetAccountInfoRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartySetAccountInfoRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SetAccountInfoRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::SetAccountInfoResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("setAccountInfo");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SetProjectConfigRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SetProjectConfigRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<
            crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigResponse,
            Box<dyn ::std::error::Error>,
        > {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("setProjectConfig");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SignOutUserRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartySignOutUserRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SignOutUserRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<
            crate::schemas::IdentitytoolkitRelyingpartySignOutUserResponse,
            Box<dyn ::std::error::Error>,
        > {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("signOutUser");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SignupNewUserRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartySignupNewUserRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SignupNewUserRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::SignupNewUserResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("signupNewUser");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UploadAccountRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyUploadAccountRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UploadAccountRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::UploadAccountResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("uploadAccount");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct VerifyAssertionRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyVerifyAssertionRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> VerifyAssertionRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::VerifyAssertionResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("verifyAssertion");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct VerifyCustomTokenRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyVerifyCustomTokenRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> VerifyCustomTokenRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::VerifyCustomTokenResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("verifyCustomToken");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct VerifyPasswordRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPasswordRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> VerifyPasswordRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::VerifyPasswordResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("verifyPassword");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct VerifyPhoneNumberRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> VerifyPhoneNumberRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<
            crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse,
            Box<dyn ::std::error::Error>,
        > {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output =
                "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
            output.push_str("verifyPhoneNumber");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}
