pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Jwk {
        #[serde(rename = "keys", default)]
        pub keys: ::std::option::Option<Vec<crate::schemas::JwkKeysItems>>,
    }
    impl ::google_field_selector::FieldSelector for Jwk {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Jwk {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct JwkKeysItems {
        #[serde(rename = "alg", default)]
        pub alg: ::std::option::Option<String>,
        #[serde(rename = "e", default)]
        pub e: ::std::option::Option<String>,
        #[serde(rename = "kid", default)]
        pub kid: ::std::option::Option<String>,
        #[serde(rename = "kty", default)]
        pub kty: ::std::option::Option<String>,
        #[serde(rename = "n", default)]
        pub n: ::std::option::Option<String>,
        #[serde(rename = "use", default)]
        pub r#use: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JwkKeysItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JwkKeysItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Tokeninfo {
        #[doc = "The access type granted with this token. It can be offline or online."]
        #[serde(rename = "access_type", default)]
        pub access_type: ::std::option::Option<String>,
        #[doc = "Who is the intended audience for this token. In general the same as issued_to."]
        #[serde(rename = "audience", default)]
        pub audience: ::std::option::Option<String>,
        #[doc = "The email address of the user. Present only if the email scope is present in the request."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The expiry time of the token, as number of seconds left until expiry."]
        #[serde(rename = "expires_in", default)]
        pub expires_in: ::std::option::Option<i32>,
        #[doc = "To whom was the token issued to. In general the same as audience."]
        #[serde(rename = "issued_to", default)]
        pub issued_to: ::std::option::Option<String>,
        #[doc = "The space separated list of scopes granted to this token."]
        #[serde(rename = "scope", default)]
        pub scope: ::std::option::Option<String>,
        #[doc = "The token handle associated with this token."]
        #[serde(rename = "token_handle", default)]
        pub token_handle: ::std::option::Option<String>,
        #[doc = "The obfuscated user id."]
        #[serde(rename = "user_id", default)]
        pub user_id: ::std::option::Option<String>,
        #[doc = "Boolean flag which is true if the email address is verified. Present only if the email scope is present in the request."]
        #[serde(rename = "verified_email", default)]
        pub verified_email: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Tokeninfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Tokeninfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Userinfoplus {
        #[doc = "The user's email address."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The user's last name."]
        #[serde(rename = "family_name", default)]
        pub family_name: ::std::option::Option<String>,
        #[doc = "The user's gender."]
        #[serde(rename = "gender", default)]
        pub gender: ::std::option::Option<String>,
        #[doc = "The user's first name."]
        #[serde(rename = "given_name", default)]
        pub given_name: ::std::option::Option<String>,
        #[doc = "The hosted domain e.g. example.com if the user is Google apps user."]
        #[serde(rename = "hd", default)]
        pub hd: ::std::option::Option<String>,
        #[doc = "The obfuscated ID of the user."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "URL of the profile page."]
        #[serde(rename = "link", default)]
        pub link: ::std::option::Option<String>,
        #[doc = "The user's preferred locale."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The user's full name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "URL of the user's picture image."]
        #[serde(rename = "picture", default)]
        pub picture: ::std::option::Option<String>,
        #[doc = "Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address."]
        #[serde(rename = "verified_email", default)]
        pub verified_email: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Userinfoplus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Userinfoplus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: A,
}
impl<A> Client<A>
where
    A: ::google_api_auth::GetAccessToken,
{
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth,
        }
    }
    #[doc = "Actions that can be performed on the userinfo resource"]
    pub fn userinfo(&self) -> crate::resources::userinfo::UserinfoActions<A> {
        crate::resources::userinfo::UserinfoActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = ""]
    pub fn get_cert_for_open_id_connect(&self) -> GetCertForOpenIdConnectRequestBuilder<A> {
        GetCertForOpenIdConnectRequestBuilder {
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
    #[doc = ""]
    pub fn tokeninfo(&self) -> TokeninfoRequestBuilder<A> {
        TokeninfoRequestBuilder {
            reqwest: &self.reqwest,
            auth: &self.auth,
            alt: None,
            fields: None,
            key: None,
            oauth_token: None,
            pretty_print: None,
            quota_user: None,
            user_ip: None,
            access_token: None,
            id_token: None,
            token_handle: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct GetCertForOpenIdConnectRequestBuilder<'a, A> {
    pub(crate) reqwest: &'a ::reqwest::Client,
    pub(crate) auth: &'a A,
    alt: Option<crate::params::Alt>,
    fields: Option<String>,
    key: Option<String>,
    oauth_token: Option<String>,
    pretty_print: Option<bool>,
    quota_user: Option<String>,
    user_ip: Option<String>,
}
impl<'a, A: ::google_api_auth::GetAccessToken> GetCertForOpenIdConnectRequestBuilder<'a, A> {
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    #[doc = "OAuth 2.0 token for the current user."]
    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
        self.oauth_token = Some(value.into());
        self
    }
    #[doc = "Returns response with indentations and line breaks."]
    pub fn pretty_print(mut self, value: bool) -> Self {
        self.pretty_print = Some(value);
        self
    }
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
        self.quota_user = Some(value.into());
        self
    }
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub fn user_ip(mut self, value: impl Into<String>) -> Self {
        self.user_ip = Some(value.into());
        self
    }
    #[doc = r" Execute the given operation. The fields requested are"]
    #[doc = r" determined by the FieldSelector attribute of the return type."]
    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
    #[doc = r" are not generic over the return type and deserialize the"]
    #[doc = r" response into an auto-generated struct will all possible"]
    #[doc = r" fields."]
    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
    {
        let fields = ::google_field_selector::to_string::<T>();
        let fields: Option<String> = if fields.is_empty() {
            None
        } else {
            Some(fields)
        };
        self.execute_with_fields(fields)
    }
    #[doc = r" Execute the given operation. This will not provide any"]
    #[doc = r" `fields` selector indicating that the server will determine"]
    #[doc = r" the fields returned. This typically includes the most common"]
    #[doc = r" fields, but it will not include every possible attribute of"]
    #[doc = r" the response resource."]
    pub fn execute_with_default_fields(
        self,
    ) -> Result<crate::schemas::Jwk, Box<dyn ::std::error::Error>> {
        self.execute_with_fields(None::<&str>)
    }
    #[doc = r" Execute the given operation. This will provide a `fields`"]
    #[doc = r" selector of `*`. This will include every attribute of the"]
    #[doc = r" response resource and should be limited to use during"]
    #[doc = r" development or debugging."]
    pub fn execute_with_all_fields(
        self,
    ) -> Result<crate::schemas::Jwk, Box<dyn ::std::error::Error>> {
        self.execute_with_fields(Some("*"))
    }
    #[doc = r" Execute the given operation. This will use the `fields`"]
    #[doc = r" selector provided and will deserialize the response into"]
    #[doc = r" whatever return value is provided."]
    pub fn execute_with_fields<T, F>(
        mut self,
        fields: Option<F>,
    ) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned,
        F: Into<String>,
    {
        self.fields = fields.map(Into::into);
        self._execute()
    }
    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned,
    {
        let req = self._request(&self._path())?;
        Ok(req.send()?.error_for_status()?.json()?)
    }
    fn _path(&self) -> String {
        let mut output = "https://www.googleapis.com/".to_owned();
        output.push_str("oauth2/v2/certs");
        output
    }
    fn _request(
        &self,
        path: &str,
    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
        let req = self.reqwest.request(::reqwest::Method::GET, path);
        let req = req.query(&[("alt", &self.alt)]);
        let req = req.query(&[("fields", &self.fields)]);
        let req = req.query(&[("key", &self.key)]);
        let req = req.query(&[("oauth_token", &self.oauth_token)]);
        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
        let req = req.query(&[("quotaUser", &self.quota_user)]);
        let req = req.query(&[("userIp", &self.user_ip)]);
        let req = req.bearer_auth(self.auth.access_token()?);
        Ok(req)
    }
}
#[derive(Debug, Clone)]
pub struct TokeninfoRequestBuilder<'a, A> {
    pub(crate) reqwest: &'a ::reqwest::Client,
    pub(crate) auth: &'a A,
    access_token: Option<String>,
    id_token: Option<String>,
    token_handle: Option<String>,
    alt: Option<crate::params::Alt>,
    fields: Option<String>,
    key: Option<String>,
    oauth_token: Option<String>,
    pretty_print: Option<bool>,
    quota_user: Option<String>,
    user_ip: Option<String>,
}
impl<'a, A: ::google_api_auth::GetAccessToken> TokeninfoRequestBuilder<'a, A> {
    #[doc = ""]
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }
    #[doc = ""]
    pub fn id_token(mut self, value: impl Into<String>) -> Self {
        self.id_token = Some(value.into());
        self
    }
    #[doc = ""]
    pub fn token_handle(mut self, value: impl Into<String>) -> Self {
        self.token_handle = Some(value.into());
        self
    }
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    #[doc = "OAuth 2.0 token for the current user."]
    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
        self.oauth_token = Some(value.into());
        self
    }
    #[doc = "Returns response with indentations and line breaks."]
    pub fn pretty_print(mut self, value: bool) -> Self {
        self.pretty_print = Some(value);
        self
    }
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
        self.quota_user = Some(value.into());
        self
    }
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub fn user_ip(mut self, value: impl Into<String>) -> Self {
        self.user_ip = Some(value.into());
        self
    }
    #[doc = r" Execute the given operation. The fields requested are"]
    #[doc = r" determined by the FieldSelector attribute of the return type."]
    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
    #[doc = r" are not generic over the return type and deserialize the"]
    #[doc = r" response into an auto-generated struct will all possible"]
    #[doc = r" fields."]
    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
    {
        let fields = ::google_field_selector::to_string::<T>();
        let fields: Option<String> = if fields.is_empty() {
            None
        } else {
            Some(fields)
        };
        self.execute_with_fields(fields)
    }
    #[doc = r" Execute the given operation. This will not provide any"]
    #[doc = r" `fields` selector indicating that the server will determine"]
    #[doc = r" the fields returned. This typically includes the most common"]
    #[doc = r" fields, but it will not include every possible attribute of"]
    #[doc = r" the response resource."]
    pub fn execute_with_default_fields(
        self,
    ) -> Result<crate::schemas::Tokeninfo, Box<dyn ::std::error::Error>> {
        self.execute_with_fields(None::<&str>)
    }
    #[doc = r" Execute the given operation. This will provide a `fields`"]
    #[doc = r" selector of `*`. This will include every attribute of the"]
    #[doc = r" response resource and should be limited to use during"]
    #[doc = r" development or debugging."]
    pub fn execute_with_all_fields(
        self,
    ) -> Result<crate::schemas::Tokeninfo, Box<dyn ::std::error::Error>> {
        self.execute_with_fields(Some("*"))
    }
    #[doc = r" Execute the given operation. This will use the `fields`"]
    #[doc = r" selector provided and will deserialize the response into"]
    #[doc = r" whatever return value is provided."]
    pub fn execute_with_fields<T, F>(
        mut self,
        fields: Option<F>,
    ) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned,
        F: Into<String>,
    {
        self.fields = fields.map(Into::into);
        self._execute()
    }
    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned,
    {
        let req = self._request(&self._path())?;
        Ok(req.send()?.error_for_status()?.json()?)
    }
    fn _path(&self) -> String {
        let mut output = "https://www.googleapis.com/".to_owned();
        output.push_str("oauth2/v2/tokeninfo");
        output
    }
    fn _request(
        &self,
        path: &str,
    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
        let req = self.reqwest.request(::reqwest::Method::POST, path);
        let req = req.query(&[("access_token", &self.access_token)]);
        let req = req.query(&[("id_token", &self.id_token)]);
        let req = req.query(&[("token_handle", &self.token_handle)]);
        let req = req.query(&[("alt", &self.alt)]);
        let req = req.query(&[("fields", &self.fields)]);
        let req = req.query(&[("key", &self.key)]);
        let req = req.query(&[("oauth_token", &self.oauth_token)]);
        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
        let req = req.query(&[("quotaUser", &self.quota_user)]);
        let req = req.query(&[("userIp", &self.user_ip)]);
        let req = req.bearer_auth(self.auth.access_token()?);
        Ok(req)
    }
}
pub mod resources {
    pub mod userinfo {
        pub mod params {}
        pub struct UserinfoActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> UserinfoActions<'a, A> {
            #[doc = ""]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
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
            #[doc = "Actions that can be performed on the v_2 resource"]
            pub fn v_2(&self) -> crate::resources::userinfo::v_2::V2Actions<A> {
                crate::resources::userinfo::v_2::V2Actions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Userinfoplus, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Userinfoplus, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("oauth2/v2/userinfo");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        pub mod v_2 {
            pub mod params {}
            pub struct V2Actions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> V2Actions<'a, A> {
                #[doc = "Actions that can be performed on the me resource"]
                pub fn me(&self) -> crate::resources::userinfo::v_2::me::MeActions<A> {
                    crate::resources::userinfo::v_2::me::MeActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            pub mod me {
                pub mod params {}
                pub struct MeActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> MeActions<'a, A> {
                    #[doc = ""]
                    pub fn get(&self) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
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
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Deprecated. Please use quotaUser instead."]
                    pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                        self.user_ip = Some(value.into());
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::Userinfoplus, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Userinfoplus, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://www.googleapis.com/".to_owned();
                        output.push_str("userinfo/v2/me");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
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
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
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
