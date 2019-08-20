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
    pub struct AchievementConfiguration {
        #[doc = "The type of the achievement.\nPossible values are:  \n- \"STANDARD\" - Achievement is either locked or unlocked. \n- \"INCREMENTAL\" - Achievement is incremental."]
        #[serde(rename = "achievementType", default)]
        pub achievement_type: Option<String>,
        #[doc = "The draft data of the achievement."]
        #[serde(rename = "draft", default)]
        pub draft: Option<crate::schemas::AchievementConfigurationDetail>,
        #[doc = "The ID of the achievement."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The initial state of the achievement.\nPossible values are:  \n- \"HIDDEN\" - Achievement is hidden. \n- \"REVEALED\" - Achievement is revealed. \n- \"UNLOCKED\" - Achievement is unlocked."]
        #[serde(rename = "initialState", default)]
        pub initial_state: Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#achievementConfiguration."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The read-only published data of the achievement."]
        #[serde(rename = "published", default)]
        pub published: Option<crate::schemas::AchievementConfigurationDetail>,
        #[doc = "Steps to unlock. Only applicable to incremental achievements."]
        #[serde(rename = "stepsToUnlock", default)]
        pub steps_to_unlock: Option<i32>,
        #[doc = "The token for this resource."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for AchievementConfiguration {
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
    pub struct AchievementConfigurationDetail {
        #[doc = "Localized strings for the achievement description."]
        #[serde(rename = "description", default)]
        pub description: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "The icon url of this achievement. Writes to this field are ignored."]
        #[serde(rename = "iconUrl", default)]
        pub icon_url: Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#achievementConfigurationDetail."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Localized strings for the achievement name."]
        #[serde(rename = "name", default)]
        pub name: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "Point value for the achievement."]
        #[serde(rename = "pointValue", default)]
        pub point_value: Option<i32>,
        #[doc = "The sort rank of this achievement. Writes to this field are ignored."]
        #[serde(rename = "sortRank", default)]
        pub sort_rank: Option<i32>,
    }
    impl ::field_selector::FieldSelector for AchievementConfigurationDetail {
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
    pub struct AchievementConfigurationListResponse {
        #[doc = "The achievement configurations."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::AchievementConfiguration>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string games#achievementConfigurationListResponse."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The pagination token for the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for AchievementConfigurationListResponse {
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
    pub struct GamesNumberAffixConfiguration {
        #[doc = "When the language requires special treatment of \"small\" numbers (as with 2, 3, and 4 in Czech; or numbers ending 2, 3, or 4 but not 12, 13, or 14 in Polish)."]
        #[serde(rename = "few", default)]
        pub few: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "When the language requires special treatment of \"large\" numbers (as with numbers ending 11-99 in Maltese)."]
        #[serde(rename = "many", default)]
        pub many: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "When the language requires special treatment of numbers like one (as with the number 1 in English and most other languages; in Russian, any number ending in 1 but not ending in 11 is in this class)."]
        #[serde(rename = "one", default)]
        pub one: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "When the language does not require special treatment of the given quantity (as with all numbers in Chinese, or 42 in English)."]
        #[serde(rename = "other", default)]
        pub other: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "When the language requires special treatment of numbers like two (as with 2 in Welsh, or 102 in Slovenian)."]
        #[serde(rename = "two", default)]
        pub two: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "When the language requires special treatment of the number 0 (as in Arabic)."]
        #[serde(rename = "zero", default)]
        pub zero: Option<crate::schemas::LocalizedStringBundle>,
    }
    impl ::field_selector::FieldSelector for GamesNumberAffixConfiguration {
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
    pub struct GamesNumberFormatConfiguration {
        #[doc = "The curreny code string. Only used for CURRENCY format type."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: Option<String>,
        #[doc = "The number of decimal places for number. Only used for NUMERIC format type."]
        #[serde(rename = "numDecimalPlaces", default)]
        pub num_decimal_places: Option<i32>,
        #[doc = "The formatting for the number.\nPossible values are:  \n- \"NUMERIC\" - Numbers are formatted to have no digits or a fixed number of digits after the decimal point according to locale. An optional custom unit can be added.\n- \"TIME_DURATION\" - Numbers are formatted to hours, minutes and seconds.\n- \"CURRENCY\" - Numbers are formatted to currency according to locale."]
        #[serde(rename = "numberFormatType", default)]
        pub number_format_type: Option<String>,
        #[doc = "An optional suffix for the NUMERIC format type. These strings follow the same  plural rules as all Android string resources."]
        #[serde(rename = "suffix", default)]
        pub suffix: Option<crate::schemas::GamesNumberAffixConfiguration>,
    }
    impl ::field_selector::FieldSelector for GamesNumberFormatConfiguration {
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
    pub struct ImageConfiguration {
        #[doc = "The image type for the image."]
        #[serde(rename = "imageType", default)]
        pub image_type: Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#imageConfiguration."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The resource ID of resource which the image belongs to."]
        #[serde(rename = "resourceId", default)]
        pub resource_id: Option<String>,
        #[doc = "The url for this image."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for ImageConfiguration {
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
    pub struct LeaderboardConfiguration {
        #[doc = "The draft data of the leaderboard."]
        #[serde(rename = "draft", default)]
        pub draft: Option<crate::schemas::LeaderboardConfigurationDetail>,
        #[doc = "The ID of the leaderboard."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#leaderboardConfiguration."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The read-only published data of the leaderboard."]
        #[serde(rename = "published", default)]
        pub published: Option<crate::schemas::LeaderboardConfigurationDetail>,
        #[doc = "Maximum score that can be posted to this leaderboard."]
        #[serde(rename = "scoreMax", default)]
        #[serde(with = "crate::parsed_string")]
        pub score_max: Option<i64>,
        #[doc = "Minimum score that can be posted to this leaderboard."]
        #[serde(rename = "scoreMin", default)]
        #[serde(with = "crate::parsed_string")]
        pub score_min: Option<i64>,
        #[doc = "The type of the leaderboard.\nPossible values are:  \n- \"LARGER_IS_BETTER\" - Larger scores posted are ranked higher. \n- \"SMALLER_IS_BETTER\" - Smaller scores posted are ranked higher."]
        #[serde(rename = "scoreOrder", default)]
        pub score_order: Option<String>,
        #[doc = "The token for this resource."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for LeaderboardConfiguration {
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
    pub struct LeaderboardConfigurationDetail {
        #[doc = "The icon url of this leaderboard. Writes to this field are ignored."]
        #[serde(rename = "iconUrl", default)]
        pub icon_url: Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#leaderboardConfigurationDetail."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Localized strings for the leaderboard name."]
        #[serde(rename = "name", default)]
        pub name: Option<crate::schemas::LocalizedStringBundle>,
        #[doc = "The score formatting for the leaderboard."]
        #[serde(rename = "scoreFormat", default)]
        pub score_format: Option<crate::schemas::GamesNumberFormatConfiguration>,
        #[doc = "The sort rank of this leaderboard. Writes to this field are ignored."]
        #[serde(rename = "sortRank", default)]
        pub sort_rank: Option<i32>,
    }
    impl ::field_selector::FieldSelector for LeaderboardConfigurationDetail {
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
    pub struct LeaderboardConfigurationListResponse {
        #[doc = "The leaderboard configurations."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::LeaderboardConfiguration>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string games#leaderboardConfigurationListResponse."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The pagination token for the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for LeaderboardConfigurationListResponse {
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
    pub struct LocalizedString {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedString."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The locale string."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[doc = "The string value."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for LocalizedString {
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
    pub struct LocalizedStringBundle {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The locale strings."]
        #[serde(rename = "translations", default)]
        pub translations: Option<Vec<crate::schemas::LocalizedString>>,
    }
    impl ::field_selector::FieldSelector for LocalizedStringBundle {
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
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
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
                "media" => Alt::Media,
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
    #[doc = "Actions that can be performed on the achievement_configurations resource"]
    pub fn achievement_configurations(
        &self,
    ) -> crate::achievement_configurations::AchievementConfigurationsActions<A> {
        crate::achievement_configurations::AchievementConfigurationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the image_configurations resource"]
    pub fn image_configurations(
        &self,
    ) -> crate::image_configurations::ImageConfigurationsActions<A> {
        crate::image_configurations::ImageConfigurationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the leaderboard_configurations resource"]
    pub fn leaderboard_configurations(
        &self,
    ) -> crate::leaderboard_configurations::LeaderboardConfigurationsActions<A> {
        crate::leaderboard_configurations::LeaderboardConfigurationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod achievement_configurations {
    pub mod params {}
    pub struct AchievementConfigurationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AchievementConfigurationsActions<'a, A> {
        #[doc = "Delete the achievement configuration with the given ID."]
        pub fn delete(&self, achievement_id: impl Into<String>) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                achievement_id: achievement_id.into(),
            }
        }
        #[doc = "Retrieves the metadata of the achievement configuration with the given ID."]
        pub fn get(&self, achievement_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                achievement_id: achievement_id.into(),
            }
        }
        #[doc = "Insert a new achievement configuration in this application."]
        pub fn insert(
            &self,
            request: crate::schemas::AchievementConfiguration,
            application_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
                application_id: application_id.into(),
            }
        }
        #[doc = "Returns a list of the achievement configurations in this application."]
        pub fn list(&self, application_id: impl Into<String>) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                application_id: application_id.into(),
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Update the metadata of the achievement configuration with the given ID. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::AchievementConfiguration,
            achievement_id: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
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
                achievement_id: achievement_id.into(),
            }
        }
        #[doc = "Update the metadata of the achievement configuration with the given ID."]
        pub fn update(
            &self,
            request: crate::schemas::AchievementConfiguration,
            achievement_id: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
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
                achievement_id: achievement_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        achievement_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("achievements/");
            output.push_str(&self.achievement_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        achievement_id: String,
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
        ) -> Result<crate::schemas::AchievementConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("achievements/");
            output.push_str(&self.achievement_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AchievementConfiguration,
        application_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AchievementConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("applications/");
            output.push_str(&self.application_id);
            output.push_str("/achievements");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        application_id: String,
        max_results: Option<i32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The maximum number of resource configurations to return in the response, used for paging. For any response, the actual number of resources returned may be less than the specified maxResults."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "The token returned by the previous request."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
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
        pub fn iter_items<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "items")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
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
            crate::schemas::AchievementConfigurationListResponse,
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("applications/");
            output.push_str(&self.application_id);
            output.push_str("/achievements");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AchievementConfiguration,
        achievement_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AchievementConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("achievements/");
            output.push_str(&self.achievement_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AchievementConfiguration,
        achievement_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AchievementConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("achievements/");
            output.push_str(&self.achievement_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod image_configurations {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum UploadImageType {
            #[doc = "The icon image for an achievement resource."]
            AchievementIcon,
            #[doc = "The icon image for a leaderboard resource."]
            LeaderboardIcon,
        }
        impl UploadImageType {
            pub fn as_str(self) -> &'static str {
                match self {
                    UploadImageType::AchievementIcon => "ACHIEVEMENT_ICON",
                    UploadImageType::LeaderboardIcon => "LEADERBOARD_ICON",
                }
            }
        }
        impl ::std::fmt::Display for UploadImageType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UploadImageType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UploadImageType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "ACHIEVEMENT_ICON" => UploadImageType::AchievementIcon,
                    "LEADERBOARD_ICON" => UploadImageType::LeaderboardIcon,
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
    pub struct ImageConfigurationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ImageConfigurationsActions<'a, A> {
        #[doc = "Uploads an image for a resource with the given ID and image type."]
        pub fn upload(
            &self,
            resource_id: impl Into<String>,
            image_type: crate::image_configurations::params::UploadImageType,
        ) -> UploadRequestBuilder<A> {
            UploadRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                resource_id: resource_id.into(),
                image_type,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct UploadRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        resource_id: String,
        image_type: crate::image_configurations::params::UploadImageType,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UploadRequestBuilder<'a, A> {
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
        fn _simple_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("upload/games/v1configuration/images/");
            output.push_str(&self.resource_id);
            output.push_str("/imageType/");
            {
                let str_value = self.image_type.to_string();
                output.push_str(&str_value);
            }
            output
        }
        pub fn upload<T, R>(
            mut self,
            content: R,
            mime_type: ::mime::Mime,
        ) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            R: ::std::io::Read + ::std::io::Seek + Send + 'static,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._simple_upload_path());
            let req = req.query(&[("uploadType", "multipart")]);
            use crate::multipart::{Part, RelatedMultiPart};
            let mut multipart = RelatedMultiPart::new();
            let request_json = ::serde_json::to_vec(&self.request)?;
            multipart.new_part(Part::new(
                ::mime::APPLICATION_JSON,
                Box::new(::std::io::Cursor::new(request_json)),
            ));
            multipart.new_part(Part::new(mime_type, Box::new(content)));
            let req = req.header(
                ::reqwest::header::CONTENT_TYPE,
                format!("multipart/related; boundary={}", multipart.boundary()),
            );
            let req = req.body(reqwest::Body::new(multipart.into_reader()));
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _resumable_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("resumable/upload/games/v1configuration/images/");
            output.push_str(&self.resource_id);
            output.push_str("/imageType/");
            {
                let str_value = self.image_type.to_string();
                output.push_str(&str_value);
            }
            output
        }
        pub fn start_resumable_upload(
            self,
            mime_type: ::mime::Mime,
        ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._resumable_upload_path());
            let req = req.query(&[("uploadType", "resumable")]);
            let req = req.header(
                ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                mime_type.to_string(),
            );
            let resp = req.send()?.error_for_status()?;
            let location_header =
                resp.headers()
                    .get(::reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        format!("No LOCATION header returned when initiating resumable upload")
                    })?;
            let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
            Ok(crate::ResumableUpload::new(
                self.reqwest.clone(),
                upload_url,
            ))
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
        ) -> Result<crate::schemas::ImageConfiguration, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("images/");
            output.push_str(&self.resource_id);
            output.push_str("/imageType/");
            {
                let str_value = self.image_type.to_string();
                output.push_str(&str_value);
            }
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod leaderboard_configurations {
    pub mod params {}
    pub struct LeaderboardConfigurationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> LeaderboardConfigurationsActions<'a, A> {
        #[doc = "Delete the leaderboard configuration with the given ID."]
        pub fn delete(&self, leaderboard_id: impl Into<String>) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                leaderboard_id: leaderboard_id.into(),
            }
        }
        #[doc = "Retrieves the metadata of the leaderboard configuration with the given ID."]
        pub fn get(&self, leaderboard_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                leaderboard_id: leaderboard_id.into(),
            }
        }
        #[doc = "Insert a new leaderboard configuration in this application."]
        pub fn insert(
            &self,
            request: crate::schemas::LeaderboardConfiguration,
            application_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
                application_id: application_id.into(),
            }
        }
        #[doc = "Returns a list of the leaderboard configurations in this application."]
        pub fn list(&self, application_id: impl Into<String>) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                application_id: application_id.into(),
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Update the metadata of the leaderboard configuration with the given ID. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::LeaderboardConfiguration,
            leaderboard_id: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
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
                leaderboard_id: leaderboard_id.into(),
            }
        }
        #[doc = "Update the metadata of the leaderboard configuration with the given ID."]
        pub fn update(
            &self,
            request: crate::schemas::LeaderboardConfiguration,
            leaderboard_id: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
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
                leaderboard_id: leaderboard_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        leaderboard_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("leaderboards/");
            output.push_str(&self.leaderboard_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        leaderboard_id: String,
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
        ) -> Result<crate::schemas::LeaderboardConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("leaderboards/");
            output.push_str(&self.leaderboard_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::LeaderboardConfiguration,
        application_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::LeaderboardConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("applications/");
            output.push_str(&self.application_id);
            output.push_str("/leaderboards");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        application_id: String,
        max_results: Option<i32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The maximum number of resource configurations to return in the response, used for paging. For any response, the actual number of resources returned may be less than the specified maxResults."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "The token returned by the previous request."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
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
        pub fn iter_items<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "items")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
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
            crate::schemas::LeaderboardConfigurationListResponse,
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("applications/");
            output.push_str(&self.application_id);
            output.push_str("/leaderboards");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::LeaderboardConfiguration,
        leaderboard_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::LeaderboardConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("leaderboards/");
            output.push_str(&self.leaderboard_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::LeaderboardConfiguration,
        leaderboard_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::LeaderboardConfiguration, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/games/v1configuration/".to_owned();
            output.push_str("leaderboards/");
            output.push_str(&self.leaderboard_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidpublisher"])
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
