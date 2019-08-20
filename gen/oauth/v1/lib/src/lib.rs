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
    pub struct Tokeninfo {
        #[doc = "The access type granted with this token. It can be offline or online."]
        #[serde(rename = "access_type", default)]
        pub access_type: Option<String>,
        #[doc = "Who is the intended audience for this token. In general the same as issued_to."]
        #[serde(rename = "audience", default)]
        pub audience: Option<String>,
        #[doc = "The email address of the user. Present only if the email scope is present in the request."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Boolean flag which is true if the email address is verified. Present only if the email scope is present in the request."]
        #[serde(rename = "email_verified", default)]
        pub email_verified: Option<bool>,
        #[doc = "The expiry time of the token, as number of seconds left until expiry."]
        #[serde(rename = "expires_in", default)]
        pub expires_in: Option<i32>,
        #[doc = "The issue time of the token, as number of seconds."]
        #[serde(rename = "issued_at", default)]
        pub issued_at: Option<i32>,
        #[doc = "To whom was the token issued to. In general the same as audience."]
        #[serde(rename = "issued_to", default)]
        pub issued_to: Option<String>,
        #[doc = "Who issued the token."]
        #[serde(rename = "issuer", default)]
        pub issuer: Option<String>,
        #[doc = "Nonce of the id token."]
        #[serde(rename = "nonce", default)]
        pub nonce: Option<String>,
        #[doc = "The space separated list of scopes granted to this token."]
        #[serde(rename = "scope", default)]
        pub scope: Option<String>,
        #[doc = "The obfuscated user id."]
        #[serde(rename = "user_id", default)]
        pub user_id: Option<String>,
        #[doc = "Boolean flag which is true if the email address is verified. Present only if the email scope is present in the request."]
        #[serde(rename = "verified_email", default)]
        pub verified_email: Option<bool>,
    }
    impl ::field_selector::FieldSelector for Tokeninfo {
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
        pub email: Option<String>,
        #[doc = "The user's last name."]
        #[serde(rename = "family_name", default)]
        pub family_name: Option<String>,
        #[doc = "The user's gender."]
        #[serde(rename = "gender", default)]
        pub gender: Option<String>,
        #[doc = "The user's first name."]
        #[serde(rename = "given_name", default)]
        pub given_name: Option<String>,
        #[doc = "The hosted domain e.g. example.com if the user is Google apps user."]
        #[serde(rename = "hd", default)]
        pub hd: Option<String>,
        #[doc = "The obfuscated ID of the user."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "URL of the profile page."]
        #[serde(rename = "link", default)]
        pub link: Option<String>,
        #[doc = "The user's preferred locale."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[doc = "The user's full name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "URL of the user's picture image."]
        #[serde(rename = "picture", default)]
        pub picture: Option<String>,
        #[doc = "Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address."]
        #[serde(rename = "verified_email", default)]
        pub verified_email: Option<bool>,
    }
    impl ::field_selector::FieldSelector for Userinfoplus {
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
    #[doc = "Actions that can be performed on the userinfo resource"]
    pub fn userinfo(&self) -> crate::userinfo::UserinfoActions<A> {
        crate::userinfo::UserinfoActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Get token info"]
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
        }
    }
}
pub mod userinfo {
    pub mod params {}
    pub struct UserinfoActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> UserinfoActions<'a, A> {
        #[doc = "Get user info"]
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
        pub fn v_2(&self) -> v_2::V2Actions<A> {
            v_2::V2Actions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
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
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::Userinfoplus, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("oauth2/v1/userinfo");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/plus.me"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    pub mod v_2 {
        pub mod params {}
        pub struct V2Actions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> V2Actions<'a, A> {
            #[doc = "Actions that can be performed on the me resource"]
            pub fn me(&self) -> me::MeActions<A> {
                me::MeActions
            }
        }
        pub mod me {
            pub mod params {}
            pub struct MeActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> MeActions<'a, A> {
                #[doc = "Get user info"]
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
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Userinfoplus, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("userinfo/v2/me");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/plus.me"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct TokeninfoRequestBuilder<'a, A> {
    pub(crate) reqwest: &'a ::reqwest::Client,
    pub(crate) auth: &'a ::std::sync::Mutex<A>,
    access_token: Option<String>,
    id_token: Option<String>,
    alt: Option<crate::params::Alt>,
    fields: Option<String>,
    key: Option<String>,
    oauth_token: Option<String>,
    pretty_print: Option<bool>,
    quota_user: Option<String>,
    user_ip: Option<String>,
}
impl<'a, A: yup_oauth2::GetToken> TokeninfoRequestBuilder<'a, A> {
    #[doc = "The oauth2 access token"]
    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
        self.access_token = Some(value.into());
        self
    }
    #[doc = "The ID token"]
    pub fn id_token(&mut self, value: impl Into<String>) -> &mut Self {
        self.id_token = Some(value.into());
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
    pub fn execute_debug(self) -> Result<crate::schemas::Tokeninfo, Box<dyn ::std::error::Error>> {
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
        let mut output = "https://www.googleapis.com/".to_owned();
        output.push_str("oauth2/v1/tokeninfo");
        output
    }
    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
        let req = self.reqwest.request(::reqwest::Method::POST, path);
        let req = req.query(&[("access_token", &self.access_token)]);
        let req = req.query(&[("id_token", &self.id_token)]);
        let req = req.query(&[("alt", &self.alt)]);
        let req = req.query(&[("fields", &self.fields)]);
        let req = req.query(&[("key", &self.key)]);
        let req = req.query(&[("oauth_token", &self.oauth_token)]);
        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
        let req = req.query(&[("quotaUser", &self.quota_user)]);
        let req = req.query(&[("userIp", &self.user_ip)]);
        req
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
