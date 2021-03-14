#![doc = "# Resources and Methods\n    * [searchanalytics](resources/searchanalytics/struct.SearchanalyticsActions.html)\n      * [*query*](resources/searchanalytics/struct.QueryRequestBuilder.html)\n    * [sitemaps](resources/sitemaps/struct.SitemapsActions.html)\n      * [*delete*](resources/sitemaps/struct.DeleteRequestBuilder.html), [*get*](resources/sitemaps/struct.GetRequestBuilder.html), [*list*](resources/sitemaps/struct.ListRequestBuilder.html), [*submit*](resources/sitemaps/struct.SubmitRequestBuilder.html)\n    * [sites](resources/sites/struct.SitesActions.html)\n      * [*add*](resources/sites/struct.AddRequestBuilder.html), [*delete*](resources/sites/struct.DeleteRequestBuilder.html), [*get*](resources/sites/struct.GetRequestBuilder.html), [*list*](resources/sites/struct.ListRequestBuilder.html)\n    * [url_testing_tools](resources/url_testing_tools/struct.UrlTestingToolsActions.html)\n      * [mobile_friendly_test](resources/url_testing_tools/mobile_friendly_test/struct.MobileFriendlyTestActions.html)\n        * [*run*](resources/url_testing_tools/mobile_friendly_test/struct.RunRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage Search Console data for your verified sites\n\n`https://www.googleapis.com/auth/webmasters`"]
    pub const WEBMASTERS: &str = "https://www.googleapis.com/auth/webmasters";
    #[doc = "View Search Console data for your verified sites\n\n`https://www.googleapis.com/auth/webmasters.readonly`"]
    pub const WEBMASTERS_READONLY: &str = "https://www.googleapis.com/auth/webmasters.readonly";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ApiDataRow {
        #[serde(
            rename = "clicks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clicks: ::std::option::Option<f64>,
        #[serde(
            rename = "ctr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ctr: ::std::option::Option<f64>,
        #[serde(
            rename = "impressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impressions: ::std::option::Option<f64>,
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for ApiDataRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiDataRow {
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
    pub struct ApiDimensionFilter {
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<crate::schemas::ApiDimensionFilterDimension>,
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[serde(
            rename = "operator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator: ::std::option::Option<crate::schemas::ApiDimensionFilterOperator>,
    }
    impl ::google_field_selector::FieldSelector for ApiDimensionFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiDimensionFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiDimensionFilterDimension {
        Country,
        Device,
        Page,
        Query,
        SearchAppearance,
    }
    impl ApiDimensionFilterDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiDimensionFilterDimension::Country => "COUNTRY",
                ApiDimensionFilterDimension::Device => "DEVICE",
                ApiDimensionFilterDimension::Page => "PAGE",
                ApiDimensionFilterDimension::Query => "QUERY",
                ApiDimensionFilterDimension::SearchAppearance => "SEARCH_APPEARANCE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApiDimensionFilterDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApiDimensionFilterDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApiDimensionFilterDimension, ()> {
            Ok(match s {
                "COUNTRY" => ApiDimensionFilterDimension::Country,
                "DEVICE" => ApiDimensionFilterDimension::Device,
                "PAGE" => ApiDimensionFilterDimension::Page,
                "QUERY" => ApiDimensionFilterDimension::Query,
                "SEARCH_APPEARANCE" => ApiDimensionFilterDimension::SearchAppearance,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApiDimensionFilterDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiDimensionFilterDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiDimensionFilterDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COUNTRY" => ApiDimensionFilterDimension::Country,
                "DEVICE" => ApiDimensionFilterDimension::Device,
                "PAGE" => ApiDimensionFilterDimension::Page,
                "QUERY" => ApiDimensionFilterDimension::Query,
                "SEARCH_APPEARANCE" => ApiDimensionFilterDimension::SearchAppearance,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApiDimensionFilterDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiDimensionFilterDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiDimensionFilterOperator {
        Contains,
        Equals,
        NotContains,
        NotEquals,
    }
    impl ApiDimensionFilterOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiDimensionFilterOperator::Contains => "CONTAINS",
                ApiDimensionFilterOperator::Equals => "EQUALS",
                ApiDimensionFilterOperator::NotContains => "NOT_CONTAINS",
                ApiDimensionFilterOperator::NotEquals => "NOT_EQUALS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApiDimensionFilterOperator {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApiDimensionFilterOperator {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApiDimensionFilterOperator, ()> {
            Ok(match s {
                "CONTAINS" => ApiDimensionFilterOperator::Contains,
                "EQUALS" => ApiDimensionFilterOperator::Equals,
                "NOT_CONTAINS" => ApiDimensionFilterOperator::NotContains,
                "NOT_EQUALS" => ApiDimensionFilterOperator::NotEquals,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApiDimensionFilterOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiDimensionFilterOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiDimensionFilterOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTAINS" => ApiDimensionFilterOperator::Contains,
                "EQUALS" => ApiDimensionFilterOperator::Equals,
                "NOT_CONTAINS" => ApiDimensionFilterOperator::NotContains,
                "NOT_EQUALS" => ApiDimensionFilterOperator::NotEquals,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApiDimensionFilterOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiDimensionFilterOperator {
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
    pub struct ApiDimensionFilterGroup {
        #[serde(
            rename = "filters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filters: ::std::option::Option<Vec<crate::schemas::ApiDimensionFilter>>,
        #[serde(
            rename = "groupType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_type: ::std::option::Option<crate::schemas::ApiDimensionFilterGroupGroupType>,
    }
    impl ::google_field_selector::FieldSelector for ApiDimensionFilterGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiDimensionFilterGroup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiDimensionFilterGroupGroupType {
        And,
    }
    impl ApiDimensionFilterGroupGroupType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiDimensionFilterGroupGroupType::And => "AND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApiDimensionFilterGroupGroupType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApiDimensionFilterGroupGroupType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApiDimensionFilterGroupGroupType, ()> {
            Ok(match s {
                "AND" => ApiDimensionFilterGroupGroupType::And,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApiDimensionFilterGroupGroupType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiDimensionFilterGroupGroupType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiDimensionFilterGroupGroupType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => ApiDimensionFilterGroupGroupType::And,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApiDimensionFilterGroupGroupType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiDimensionFilterGroupGroupType {
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
    pub struct BlockedResource {
        #[doc = "URL of the blocked resource."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BlockedResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BlockedResource {
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
    pub struct Image {
        #[doc = "Image data in format determined by the mime type. Currently, the format will always be \"image/png\", but this might change in the future."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The mime-type of the image data."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Image {
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
    pub struct MobileFriendlyIssue {
        #[doc = "Rule violated."]
        #[serde(
            rename = "rule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule: ::std::option::Option<crate::schemas::MobileFriendlyIssueRule>,
    }
    impl ::google_field_selector::FieldSelector for MobileFriendlyIssue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MobileFriendlyIssue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MobileFriendlyIssueRule {
        #[doc = "Viewsport is not specified using the meta viewport tag. [Learn more] (https://support.google.com/webmasters/answer/6352293#viewport_not_configured)."]
        ConfigureViewport,
        #[doc = "Viewport defined to a fixed width. [Learn more] (https://support.google.com/webmasters/answer/6352293#fixed-width_viewport)."]
        FixedWidthViewport,
        #[doc = "Unknown rule. Sorry, we don't have any description for the rule that was broken."]
        MobileFriendlyRuleUnspecified,
        #[doc = "Content not sized to viewport. [Learn more] (https://support.google.com/webmasters/answer/6352293#content_not_sized_to_viewport)."]
        SizeContentToViewport,
        #[doc = "Touch elements are too close to each other. [Learn more] (https://support.google.com/webmasters/answer/6352293#touch_elements_too_close)."]
        TapTargetsTooClose,
        #[doc = "Font size is too small for easy reading on a small screen. [Learn More] (https://support.google.com/webmasters/answer/6352293#small_font_size)."]
        UseLegibleFontSizes,
        #[doc = "Plugins incompatible with mobile devices are being used. [Learn more] (https://support.google.com/webmasters/answer/6352293#flash_usage)."]
        UsesIncompatiblePlugins,
    }
    impl MobileFriendlyIssueRule {
        pub fn as_str(self) -> &'static str {
            match self {
                MobileFriendlyIssueRule::ConfigureViewport => "CONFIGURE_VIEWPORT",
                MobileFriendlyIssueRule::FixedWidthViewport => "FIXED_WIDTH_VIEWPORT",
                MobileFriendlyIssueRule::MobileFriendlyRuleUnspecified => {
                    "MOBILE_FRIENDLY_RULE_UNSPECIFIED"
                }
                MobileFriendlyIssueRule::SizeContentToViewport => "SIZE_CONTENT_TO_VIEWPORT",
                MobileFriendlyIssueRule::TapTargetsTooClose => "TAP_TARGETS_TOO_CLOSE",
                MobileFriendlyIssueRule::UseLegibleFontSizes => "USE_LEGIBLE_FONT_SIZES",
                MobileFriendlyIssueRule::UsesIncompatiblePlugins => "USES_INCOMPATIBLE_PLUGINS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MobileFriendlyIssueRule {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MobileFriendlyIssueRule {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MobileFriendlyIssueRule, ()> {
            Ok(match s {
                "CONFIGURE_VIEWPORT" => MobileFriendlyIssueRule::ConfigureViewport,
                "FIXED_WIDTH_VIEWPORT" => MobileFriendlyIssueRule::FixedWidthViewport,
                "MOBILE_FRIENDLY_RULE_UNSPECIFIED" => {
                    MobileFriendlyIssueRule::MobileFriendlyRuleUnspecified
                }
                "SIZE_CONTENT_TO_VIEWPORT" => MobileFriendlyIssueRule::SizeContentToViewport,
                "TAP_TARGETS_TOO_CLOSE" => MobileFriendlyIssueRule::TapTargetsTooClose,
                "USE_LEGIBLE_FONT_SIZES" => MobileFriendlyIssueRule::UseLegibleFontSizes,
                "USES_INCOMPATIBLE_PLUGINS" => MobileFriendlyIssueRule::UsesIncompatiblePlugins,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MobileFriendlyIssueRule {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MobileFriendlyIssueRule {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MobileFriendlyIssueRule {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONFIGURE_VIEWPORT" => MobileFriendlyIssueRule::ConfigureViewport,
                "FIXED_WIDTH_VIEWPORT" => MobileFriendlyIssueRule::FixedWidthViewport,
                "MOBILE_FRIENDLY_RULE_UNSPECIFIED" => {
                    MobileFriendlyIssueRule::MobileFriendlyRuleUnspecified
                }
                "SIZE_CONTENT_TO_VIEWPORT" => MobileFriendlyIssueRule::SizeContentToViewport,
                "TAP_TARGETS_TOO_CLOSE" => MobileFriendlyIssueRule::TapTargetsTooClose,
                "USE_LEGIBLE_FONT_SIZES" => MobileFriendlyIssueRule::UseLegibleFontSizes,
                "USES_INCOMPATIBLE_PLUGINS" => MobileFriendlyIssueRule::UsesIncompatiblePlugins,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MobileFriendlyIssueRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MobileFriendlyIssueRule {
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
    pub struct ResourceIssue {
        #[doc = "Describes a blocked resource issue."]
        #[serde(
            rename = "blockedResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blocked_resource: ::std::option::Option<crate::schemas::BlockedResource>,
    }
    impl ::google_field_selector::FieldSelector for ResourceIssue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceIssue {
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
    pub struct RunMobileFriendlyTestRequest {
        #[doc = "Whether or not screenshot is requested. Default is false."]
        #[serde(
            rename = "requestScreenshot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_screenshot: ::std::option::Option<bool>,
        #[doc = "URL for inspection."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RunMobileFriendlyTestRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunMobileFriendlyTestRequest {
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
    pub struct RunMobileFriendlyTestResponse {
        #[doc = "Test verdict, whether the page is mobile friendly or not."]
        #[serde(
            rename = "mobileFriendliness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_friendliness:
            ::std::option::Option<crate::schemas::RunMobileFriendlyTestResponseMobileFriendliness>,
        #[doc = "List of mobile-usability issues."]
        #[serde(
            rename = "mobileFriendlyIssues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_friendly_issues: ::std::option::Option<Vec<crate::schemas::MobileFriendlyIssue>>,
        #[doc = "Information about embedded resources issues."]
        #[serde(
            rename = "resourceIssues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_issues: ::std::option::Option<Vec<crate::schemas::ResourceIssue>>,
        #[doc = "Screenshot of the requested URL."]
        #[serde(
            rename = "screenshot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screenshot: ::std::option::Option<crate::schemas::Image>,
        #[doc = "Final state of the test, can be either complete or an error."]
        #[serde(
            rename = "testStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_status: ::std::option::Option<crate::schemas::TestStatus>,
    }
    impl ::google_field_selector::FieldSelector for RunMobileFriendlyTestResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunMobileFriendlyTestResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RunMobileFriendlyTestResponseMobileFriendliness {
        #[doc = "The page is mobile friendly."]
        MobileFriendly,
        #[doc = "Internal error when running this test. Please try running the test again."]
        MobileFriendlyTestResultUnspecified,
        #[doc = "The page is not mobile friendly."]
        NotMobileFriendly,
    }
    impl RunMobileFriendlyTestResponseMobileFriendliness {
        pub fn as_str(self) -> &'static str {
            match self { RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendly => "MOBILE_FRIENDLY" , RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendlyTestResultUnspecified => "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED" , RunMobileFriendlyTestResponseMobileFriendliness :: NotMobileFriendly => "NOT_MOBILE_FRIENDLY" , }
        }
    }
    impl ::std::convert::AsRef<str> for RunMobileFriendlyTestResponseMobileFriendliness {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RunMobileFriendlyTestResponseMobileFriendliness {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<RunMobileFriendlyTestResponseMobileFriendliness, ()> {
            Ok (match s { "MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendly , "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendlyTestResultUnspecified , "NOT_MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: NotMobileFriendly , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for RunMobileFriendlyTestResponseMobileFriendliness {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RunMobileFriendlyTestResponseMobileFriendliness {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RunMobileFriendlyTestResponseMobileFriendliness {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendly , "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendlyTestResultUnspecified , "NOT_MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: NotMobileFriendly , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for RunMobileFriendlyTestResponseMobileFriendliness {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunMobileFriendlyTestResponseMobileFriendliness {
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
    pub struct SearchAnalyticsQueryRequest {
        #[doc = "[Optional; Default is \"auto\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see the help documentation to learn how data is calculated differently by site versus by page. **Note:** If you group or filter by page, you cannot aggregate by property. If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid."]
        #[serde(
            rename = "aggregationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregation_type:
            ::std::option::Option<crate::schemas::SearchAnalyticsQueryRequestAggregationType>,
        #[doc = "The data state to be fetched, can be full or all, the latter including full and partial data."]
        #[serde(
            rename = "dataState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_state: ::std::option::Option<crate::schemas::SearchAnalyticsQueryRequestDataState>,
        #[doc = "[Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains \"buy\"' to see only data where the query string contains the substring \"buy\" (not case-sensitive). You can filter by a dimension without grouping by it."]
        #[serde(
            rename = "dimensionFilterGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filter_groups:
            ::std::option::Option<Vec<crate::schemas::ApiDimensionFilterGroup>>,
        #[doc = "[Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions:
            ::std::option::Option<Vec<crate::schemas::SearchAnalyticsQueryRequestDimensionsItems>>,
        #[doc = "[Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<String>,
        #[doc = "[Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 25,000 (inclusive)."]
        #[serde(
            rename = "rowLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_limit: ::std::option::Option<i32>,
        #[doc = "[Optional; Default is \"web\"] The search type to filter for."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type:
            ::std::option::Option<crate::schemas::SearchAnalyticsQueryRequestSearchType>,
        #[doc = "[Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<String>,
        #[doc = "[Optional; Default is 0] Zero-based index of the first row in the response. Must be a non-negative number."]
        #[serde(
            rename = "startRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_row: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SearchAnalyticsQueryRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchAnalyticsQueryRequestAggregationType {
        Auto,
        ByPage,
        ByProperty,
    }
    impl SearchAnalyticsQueryRequestAggregationType {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchAnalyticsQueryRequestAggregationType::Auto => "AUTO",
                SearchAnalyticsQueryRequestAggregationType::ByPage => "BY_PAGE",
                SearchAnalyticsQueryRequestAggregationType::ByProperty => "BY_PROPERTY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchAnalyticsQueryRequestAggregationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchAnalyticsQueryRequestAggregationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<SearchAnalyticsQueryRequestAggregationType, ()> {
            Ok(match s {
                "AUTO" => SearchAnalyticsQueryRequestAggregationType::Auto,
                "BY_PAGE" => SearchAnalyticsQueryRequestAggregationType::ByPage,
                "BY_PROPERTY" => SearchAnalyticsQueryRequestAggregationType::ByProperty,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchAnalyticsQueryRequestAggregationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchAnalyticsQueryRequestAggregationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchAnalyticsQueryRequestAggregationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO" => SearchAnalyticsQueryRequestAggregationType::Auto,
                "BY_PAGE" => SearchAnalyticsQueryRequestAggregationType::ByPage,
                "BY_PROPERTY" => SearchAnalyticsQueryRequestAggregationType::ByProperty,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchAnalyticsQueryRequestAggregationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryRequestAggregationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchAnalyticsQueryRequestDataState {
        #[doc = "Include all data, full and partial."]
        All,
        #[doc = "Default value, should not be used."]
        DataStateUnspecified,
        #[doc = "Include full final data only, without partial."]
        Final,
    }
    impl SearchAnalyticsQueryRequestDataState {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchAnalyticsQueryRequestDataState::All => "ALL",
                SearchAnalyticsQueryRequestDataState::DataStateUnspecified => {
                    "DATA_STATE_UNSPECIFIED"
                }
                SearchAnalyticsQueryRequestDataState::Final => "FINAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchAnalyticsQueryRequestDataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchAnalyticsQueryRequestDataState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchAnalyticsQueryRequestDataState, ()> {
            Ok(match s {
                "ALL" => SearchAnalyticsQueryRequestDataState::All,
                "DATA_STATE_UNSPECIFIED" => {
                    SearchAnalyticsQueryRequestDataState::DataStateUnspecified
                }
                "FINAL" => SearchAnalyticsQueryRequestDataState::Final,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchAnalyticsQueryRequestDataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchAnalyticsQueryRequestDataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchAnalyticsQueryRequestDataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL" => SearchAnalyticsQueryRequestDataState::All,
                "DATA_STATE_UNSPECIFIED" => {
                    SearchAnalyticsQueryRequestDataState::DataStateUnspecified
                }
                "FINAL" => SearchAnalyticsQueryRequestDataState::Final,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchAnalyticsQueryRequestDataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryRequestDataState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchAnalyticsQueryRequestDimensionsItems {
        Country,
        Date,
        Device,
        Page,
        Query,
        SearchAppearance,
    }
    impl SearchAnalyticsQueryRequestDimensionsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchAnalyticsQueryRequestDimensionsItems::Country => "COUNTRY",
                SearchAnalyticsQueryRequestDimensionsItems::Date => "DATE",
                SearchAnalyticsQueryRequestDimensionsItems::Device => "DEVICE",
                SearchAnalyticsQueryRequestDimensionsItems::Page => "PAGE",
                SearchAnalyticsQueryRequestDimensionsItems::Query => "QUERY",
                SearchAnalyticsQueryRequestDimensionsItems::SearchAppearance => "SEARCH_APPEARANCE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchAnalyticsQueryRequestDimensionsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchAnalyticsQueryRequestDimensionsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<SearchAnalyticsQueryRequestDimensionsItems, ()> {
            Ok(match s {
                "COUNTRY" => SearchAnalyticsQueryRequestDimensionsItems::Country,
                "DATE" => SearchAnalyticsQueryRequestDimensionsItems::Date,
                "DEVICE" => SearchAnalyticsQueryRequestDimensionsItems::Device,
                "PAGE" => SearchAnalyticsQueryRequestDimensionsItems::Page,
                "QUERY" => SearchAnalyticsQueryRequestDimensionsItems::Query,
                "SEARCH_APPEARANCE" => SearchAnalyticsQueryRequestDimensionsItems::SearchAppearance,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchAnalyticsQueryRequestDimensionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchAnalyticsQueryRequestDimensionsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchAnalyticsQueryRequestDimensionsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COUNTRY" => SearchAnalyticsQueryRequestDimensionsItems::Country,
                "DATE" => SearchAnalyticsQueryRequestDimensionsItems::Date,
                "DEVICE" => SearchAnalyticsQueryRequestDimensionsItems::Device,
                "PAGE" => SearchAnalyticsQueryRequestDimensionsItems::Page,
                "QUERY" => SearchAnalyticsQueryRequestDimensionsItems::Query,
                "SEARCH_APPEARANCE" => SearchAnalyticsQueryRequestDimensionsItems::SearchAppearance,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchAnalyticsQueryRequestDimensionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryRequestDimensionsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchAnalyticsQueryRequestSearchType {
        Image,
        News,
        Video,
        Web,
    }
    impl SearchAnalyticsQueryRequestSearchType {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchAnalyticsQueryRequestSearchType::Image => "IMAGE",
                SearchAnalyticsQueryRequestSearchType::News => "NEWS",
                SearchAnalyticsQueryRequestSearchType::Video => "VIDEO",
                SearchAnalyticsQueryRequestSearchType::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchAnalyticsQueryRequestSearchType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchAnalyticsQueryRequestSearchType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchAnalyticsQueryRequestSearchType, ()> {
            Ok(match s {
                "IMAGE" => SearchAnalyticsQueryRequestSearchType::Image,
                "NEWS" => SearchAnalyticsQueryRequestSearchType::News,
                "VIDEO" => SearchAnalyticsQueryRequestSearchType::Video,
                "WEB" => SearchAnalyticsQueryRequestSearchType::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchAnalyticsQueryRequestSearchType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchAnalyticsQueryRequestSearchType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchAnalyticsQueryRequestSearchType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMAGE" => SearchAnalyticsQueryRequestSearchType::Image,
                "NEWS" => SearchAnalyticsQueryRequestSearchType::News,
                "VIDEO" => SearchAnalyticsQueryRequestSearchType::Video,
                "WEB" => SearchAnalyticsQueryRequestSearchType::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchAnalyticsQueryRequestSearchType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryRequestSearchType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchAnalyticsQueryResponse {
        #[doc = "How the results were aggregated."]
        #[serde(
            rename = "responseAggregationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_aggregation_type: ::std::option::Option<
            crate::schemas::SearchAnalyticsQueryResponseResponseAggregationType,
        >,
        #[doc = "A list of rows grouped by the key values in the order given in the query."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::ApiDataRow>>,
    }
    impl ::google_field_selector::FieldSelector for SearchAnalyticsQueryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchAnalyticsQueryResponseResponseAggregationType {
        Auto,
        ByPage,
        ByProperty,
    }
    impl SearchAnalyticsQueryResponseResponseAggregationType {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchAnalyticsQueryResponseResponseAggregationType::Auto => "AUTO",
                SearchAnalyticsQueryResponseResponseAggregationType::ByPage => "BY_PAGE",
                SearchAnalyticsQueryResponseResponseAggregationType::ByProperty => "BY_PROPERTY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchAnalyticsQueryResponseResponseAggregationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchAnalyticsQueryResponseResponseAggregationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<SearchAnalyticsQueryResponseResponseAggregationType, ()>
        {
            Ok(match s {
                "AUTO" => SearchAnalyticsQueryResponseResponseAggregationType::Auto,
                "BY_PAGE" => SearchAnalyticsQueryResponseResponseAggregationType::ByPage,
                "BY_PROPERTY" => SearchAnalyticsQueryResponseResponseAggregationType::ByProperty,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchAnalyticsQueryResponseResponseAggregationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchAnalyticsQueryResponseResponseAggregationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchAnalyticsQueryResponseResponseAggregationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO" => SearchAnalyticsQueryResponseResponseAggregationType::Auto,
                "BY_PAGE" => SearchAnalyticsQueryResponseResponseAggregationType::ByPage,
                "BY_PROPERTY" => SearchAnalyticsQueryResponseResponseAggregationType::ByProperty,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for SearchAnalyticsQueryResponseResponseAggregationType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAnalyticsQueryResponseResponseAggregationType {
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
    pub struct SitemapsListResponse {
        #[doc = "Contains detailed information about a specific URL submitted as a [sitemap](https://support.google.com/webmasters/answer/156184)."]
        #[serde(
            rename = "sitemap",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sitemap: ::std::option::Option<Vec<crate::schemas::WmxSitemap>>,
    }
    impl ::google_field_selector::FieldSelector for SitemapsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SitemapsListResponse {
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
    pub struct SitesListResponse {
        #[doc = "Contains permission level information about a Search Console site. For more information, see [Permissions in Search Console](https://support.google.com/webmasters/answer/2451999)."]
        #[serde(
            rename = "siteEntry",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_entry: ::std::option::Option<Vec<crate::schemas::WmxSite>>,
    }
    impl ::google_field_selector::FieldSelector for SitesListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SitesListResponse {
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
    pub struct TestStatus {
        #[doc = "Error details if applicable."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<String>,
        #[doc = "Status of the test."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::TestStatusStatus>,
    }
    impl ::google_field_selector::FieldSelector for TestStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestStatusStatus {
        #[doc = "Inspection has completed without errors."]
        Complete,
        #[doc = "Inspection terminated in an error state. This indicates a problem in Google's infrastructure, not a user error. Please try again later."]
        InternalError,
        #[doc = "Google can not access the URL because of a user error such as a robots.txt blockage, a 403 or 500 code etc. Please make sure that the URL provided is accessible by Googlebot and is not password protected."]
        PageUnreachable,
        #[doc = "Internal error when running this test. Please try running the test again."]
        TestStatusUnspecified,
    }
    impl TestStatusStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                TestStatusStatus::Complete => "COMPLETE",
                TestStatusStatus::InternalError => "INTERNAL_ERROR",
                TestStatusStatus::PageUnreachable => "PAGE_UNREACHABLE",
                TestStatusStatus::TestStatusUnspecified => "TEST_STATUS_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TestStatusStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TestStatusStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TestStatusStatus, ()> {
            Ok(match s {
                "COMPLETE" => TestStatusStatus::Complete,
                "INTERNAL_ERROR" => TestStatusStatus::InternalError,
                "PAGE_UNREACHABLE" => TestStatusStatus::PageUnreachable,
                "TEST_STATUS_UNSPECIFIED" => TestStatusStatus::TestStatusUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TestStatusStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestStatusStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestStatusStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLETE" => TestStatusStatus::Complete,
                "INTERNAL_ERROR" => TestStatusStatus::InternalError,
                "PAGE_UNREACHABLE" => TestStatusStatus::PageUnreachable,
                "TEST_STATUS_UNSPECIFIED" => TestStatusStatus::TestStatusUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TestStatusStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestStatusStatus {
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
    pub struct WmxSite {
        #[doc = "The user's permission level for the site."]
        #[serde(
            rename = "permissionLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_level: ::std::option::Option<crate::schemas::WmxSitePermissionLevel>,
        #[doc = "The URL of the site."]
        #[serde(
            rename = "siteUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WmxSite {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WmxSite {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WmxSitePermissionLevel {
        #[doc = "Full users can access all data, and perform most of the operations."]
        SiteFullUser,
        #[doc = "Owner has complete access to the site."]
        SiteOwner,
        SitePermissionLevelUnspecified,
        #[doc = "Restricted users can access most of the data, and perform some operations."]
        SiteRestrictedUser,
        #[doc = "Unverified user has no access to site's data."]
        SiteUnverifiedUser,
    }
    impl WmxSitePermissionLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                WmxSitePermissionLevel::SiteFullUser => "SITE_FULL_USER",
                WmxSitePermissionLevel::SiteOwner => "SITE_OWNER",
                WmxSitePermissionLevel::SitePermissionLevelUnspecified => {
                    "SITE_PERMISSION_LEVEL_UNSPECIFIED"
                }
                WmxSitePermissionLevel::SiteRestrictedUser => "SITE_RESTRICTED_USER",
                WmxSitePermissionLevel::SiteUnverifiedUser => "SITE_UNVERIFIED_USER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WmxSitePermissionLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WmxSitePermissionLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WmxSitePermissionLevel, ()> {
            Ok(match s {
                "SITE_FULL_USER" => WmxSitePermissionLevel::SiteFullUser,
                "SITE_OWNER" => WmxSitePermissionLevel::SiteOwner,
                "SITE_PERMISSION_LEVEL_UNSPECIFIED" => {
                    WmxSitePermissionLevel::SitePermissionLevelUnspecified
                }
                "SITE_RESTRICTED_USER" => WmxSitePermissionLevel::SiteRestrictedUser,
                "SITE_UNVERIFIED_USER" => WmxSitePermissionLevel::SiteUnverifiedUser,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WmxSitePermissionLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WmxSitePermissionLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WmxSitePermissionLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SITE_FULL_USER" => WmxSitePermissionLevel::SiteFullUser,
                "SITE_OWNER" => WmxSitePermissionLevel::SiteOwner,
                "SITE_PERMISSION_LEVEL_UNSPECIFIED" => {
                    WmxSitePermissionLevel::SitePermissionLevelUnspecified
                }
                "SITE_RESTRICTED_USER" => WmxSitePermissionLevel::SiteRestrictedUser,
                "SITE_UNVERIFIED_USER" => WmxSitePermissionLevel::SiteUnverifiedUser,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WmxSitePermissionLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WmxSitePermissionLevel {
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
    pub struct WmxSitemap {
        #[doc = "The various content types in the sitemap."]
        #[serde(
            rename = "contents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contents: ::std::option::Option<Vec<crate::schemas::WmxSitemapContent>>,
        #[doc = "Number of errors in the sitemap. These are issues with the sitemap itself that need to be fixed before it can be processed correctly."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub errors: ::std::option::Option<i64>,
        #[doc = "If true, the sitemap has not been processed."]
        #[serde(
            rename = "isPending",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_pending: ::std::option::Option<bool>,
        #[doc = "If true, the sitemap is a collection of sitemaps."]
        #[serde(
            rename = "isSitemapsIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_sitemaps_index: ::std::option::Option<bool>,
        #[doc = "Date & time in which this sitemap was last downloaded. Date format is in RFC 3339 format (yyyy-mm-dd)."]
        #[serde(
            rename = "lastDownloaded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_downloaded: ::std::option::Option<String>,
        #[doc = "Date & time in which this sitemap was submitted. Date format is in RFC 3339 format (yyyy-mm-dd)."]
        #[serde(
            rename = "lastSubmitted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_submitted: ::std::option::Option<String>,
        #[doc = "The url of the sitemap."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "The type of the sitemap. For example: `rssFeed`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::WmxSitemapType>,
        #[doc = "Number of warnings for the sitemap. These are generally non-critical issues with URLs in the sitemaps."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub warnings: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for WmxSitemap {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WmxSitemap {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WmxSitemapType {
        AtomFeed,
        NotSitemap,
        Oceanfront,
        #[doc = "Unsupported sitemap types."]
        PatternSitemap,
        RssFeed,
        Sitemap,
        UrlList,
    }
    impl WmxSitemapType {
        pub fn as_str(self) -> &'static str {
            match self {
                WmxSitemapType::AtomFeed => "ATOM_FEED",
                WmxSitemapType::NotSitemap => "NOT_SITEMAP",
                WmxSitemapType::Oceanfront => "OCEANFRONT",
                WmxSitemapType::PatternSitemap => "PATTERN_SITEMAP",
                WmxSitemapType::RssFeed => "RSS_FEED",
                WmxSitemapType::Sitemap => "SITEMAP",
                WmxSitemapType::UrlList => "URL_LIST",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WmxSitemapType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WmxSitemapType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WmxSitemapType, ()> {
            Ok(match s {
                "ATOM_FEED" => WmxSitemapType::AtomFeed,
                "NOT_SITEMAP" => WmxSitemapType::NotSitemap,
                "OCEANFRONT" => WmxSitemapType::Oceanfront,
                "PATTERN_SITEMAP" => WmxSitemapType::PatternSitemap,
                "RSS_FEED" => WmxSitemapType::RssFeed,
                "SITEMAP" => WmxSitemapType::Sitemap,
                "URL_LIST" => WmxSitemapType::UrlList,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WmxSitemapType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WmxSitemapType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WmxSitemapType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATOM_FEED" => WmxSitemapType::AtomFeed,
                "NOT_SITEMAP" => WmxSitemapType::NotSitemap,
                "OCEANFRONT" => WmxSitemapType::Oceanfront,
                "PATTERN_SITEMAP" => WmxSitemapType::PatternSitemap,
                "RSS_FEED" => WmxSitemapType::RssFeed,
                "SITEMAP" => WmxSitemapType::Sitemap,
                "URL_LIST" => WmxSitemapType::UrlList,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WmxSitemapType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WmxSitemapType {
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
    pub struct WmxSitemapContent {
        #[doc = "The number of URLs from the sitemap that were indexed (of the content type)."]
        #[serde(
            rename = "indexed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub indexed: ::std::option::Option<i64>,
        #[doc = "The specific type of content in this sitemap. For example: `web`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::WmxSitemapContentType>,
        #[doc = "The number of URLs in the sitemap (of the content type)."]
        #[serde(
            rename = "submitted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub submitted: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for WmxSitemapContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WmxSitemapContent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WmxSitemapContentType {
        AndroidApp,
        #[doc = "Unsupported content type."]
        DataFeedElement,
        Image,
        IosApp,
        Mobile,
        News,
        #[doc = "Unsupported content type."]
        Pattern,
        Video,
        Web,
    }
    impl WmxSitemapContentType {
        pub fn as_str(self) -> &'static str {
            match self {
                WmxSitemapContentType::AndroidApp => "ANDROID_APP",
                WmxSitemapContentType::DataFeedElement => "DATA_FEED_ELEMENT",
                WmxSitemapContentType::Image => "IMAGE",
                WmxSitemapContentType::IosApp => "IOS_APP",
                WmxSitemapContentType::Mobile => "MOBILE",
                WmxSitemapContentType::News => "NEWS",
                WmxSitemapContentType::Pattern => "PATTERN",
                WmxSitemapContentType::Video => "VIDEO",
                WmxSitemapContentType::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WmxSitemapContentType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WmxSitemapContentType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WmxSitemapContentType, ()> {
            Ok(match s {
                "ANDROID_APP" => WmxSitemapContentType::AndroidApp,
                "DATA_FEED_ELEMENT" => WmxSitemapContentType::DataFeedElement,
                "IMAGE" => WmxSitemapContentType::Image,
                "IOS_APP" => WmxSitemapContentType::IosApp,
                "MOBILE" => WmxSitemapContentType::Mobile,
                "NEWS" => WmxSitemapContentType::News,
                "PATTERN" => WmxSitemapContentType::Pattern,
                "VIDEO" => WmxSitemapContentType::Video,
                "WEB" => WmxSitemapContentType::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WmxSitemapContentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WmxSitemapContentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WmxSitemapContentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID_APP" => WmxSitemapContentType::AndroidApp,
                "DATA_FEED_ELEMENT" => WmxSitemapContentType::DataFeedElement,
                "IMAGE" => WmxSitemapContentType::Image,
                "IOS_APP" => WmxSitemapContentType::IosApp,
                "MOBILE" => WmxSitemapContentType::Mobile,
                "NEWS" => WmxSitemapContentType::News,
                "PATTERN" => WmxSitemapContentType::Pattern,
                "VIDEO" => WmxSitemapContentType::Video,
                "WEB" => WmxSitemapContentType::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WmxSitemapContentType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WmxSitemapContentType {
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
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
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
                "media" => Alt::Media,
                "proto" => Alt::Proto,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the searchanalytics resource"]
    pub fn searchanalytics(&self) -> crate::resources::searchanalytics::SearchanalyticsActions {
        crate::resources::searchanalytics::SearchanalyticsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the sitemaps resource"]
    pub fn sitemaps(&self) -> crate::resources::sitemaps::SitemapsActions {
        crate::resources::sitemaps::SitemapsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the sites resource"]
    pub fn sites(&self) -> crate::resources::sites::SitesActions {
        crate::resources::sites::SitesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the url_testing_tools resource"]
    pub fn url_testing_tools(&self) -> crate::resources::url_testing_tools::UrlTestingToolsActions {
        crate::resources::url_testing_tools::UrlTestingToolsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod searchanalytics {
        pub mod params {}
        pub struct SearchanalyticsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SearchanalyticsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Query your data with filters and parameters that you define. Returns zero or more rows grouped by the row keys that you define. You must define a date range of one or more days. When date is one of the group by values, any days without data are omitted from the result list. If you need to know which days have data, issue a broad date range query grouped by date for any metric, and see which day rows are returned."]
            pub fn query(
                &self,
                request: crate::schemas::SearchAnalyticsQueryRequest,
                site_url: impl Into<String>,
            ) -> QueryRequestBuilder {
                QueryRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                }
            }
        }
        #[doc = "Created via [SearchanalyticsActions::query()](struct.SearchanalyticsActions.html#method.query)"]
        #[derive(Debug, Clone)]
        pub struct QueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SearchAnalyticsQueryRequest,
            site_url: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> QueryRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::SearchAnalyticsQueryResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchAnalyticsQueryResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/searchAnalytics/query");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod sitemaps {
        pub mod params {}
        pub struct SitemapsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SitemapsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes a sitemap from this site."]
            pub fn delete(
                &self,
                site_url: impl Into<String>,
                feedpath: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                    feedpath: feedpath.into(),
                }
            }
            #[doc = "Retrieves information about a specific sitemap."]
            pub fn get(
                &self,
                site_url: impl Into<String>,
                feedpath: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                    feedpath: feedpath.into(),
                }
            }
            #[doc = "Lists the [sitemaps-entries](/webmaster-tools/v3/sitemaps) submitted for this site, or included in the sitemap index file (if `sitemapIndex` is specified in the request)."]
            pub fn list(&self, site_url: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                    sitemap_index: None,
                }
            }
            #[doc = "Submits a sitemap for a site."]
            pub fn submit(
                &self,
                site_url: impl Into<String>,
                feedpath: impl Into<String>,
            ) -> SubmitRequestBuilder {
                SubmitRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                    feedpath: feedpath.into(),
                }
            }
        }
        #[doc = "Created via [SitemapsActions::delete()](struct.SitemapsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            feedpath: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sitemaps/");
                {
                    let var_as_str = &self.feedpath;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SitemapsActions::get()](struct.SitemapsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            feedpath: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::WmxSitemap, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::WmxSitemap, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sitemaps/");
                {
                    let var_as_str = &self.feedpath;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SitemapsActions::list()](struct.SitemapsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            sitemap_index: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "A URL of a site's sitemap index. For example: `http://www.example.com/sitemapindex.xml`."]
            pub fn sitemap_index(mut self, value: impl Into<String>) -> Self {
                self.sitemap_index = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::SitemapsListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SitemapsListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sitemaps");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("sitemapIndex", &self.sitemap_index)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SitemapsActions::submit()](struct.SitemapsActions.html#method.submit)"]
        #[derive(Debug, Clone)]
        pub struct SubmitRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            feedpath: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SubmitRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sitemaps/");
                {
                    let var_as_str = &self.feedpath;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod sites {
        pub mod params {}
        pub struct SitesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SitesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Adds a site to the set of the user's sites in Search Console."]
            pub fn add(&self, site_url: impl Into<String>) -> AddRequestBuilder {
                AddRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                }
            }
            #[doc = "Removes a site from the set of the user's Search Console sites."]
            pub fn delete(&self, site_url: impl Into<String>) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                }
            }
            #[doc = "Retrieves information about specific site."]
            pub fn get(&self, site_url: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    site_url: site_url.into(),
                }
            }
            #[doc = "Lists the user's Search Console sites."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[doc = "Created via [SitesActions::add()](struct.SitesActions.html#method.add)"]
        #[derive(Debug, Clone)]
        pub struct AddRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> AddRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SitesActions::delete()](struct.SitesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SitesActions::get()](struct.SitesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            site_url: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::WmxSite, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::WmxSite, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites/");
                {
                    let var_as_str = &self.site_url;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SitesActions::list()](struct.SitesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::SitesListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SitesListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://searchconsole.googleapis.com/".to_owned();
                output.push_str("webmasters/v3/sites");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod url_testing_tools {
        pub mod params {}
        pub struct UrlTestingToolsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> UrlTestingToolsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the mobile_friendly_test resource"]
            pub fn mobile_friendly_test(
                &self,
            ) -> crate::resources::url_testing_tools::mobile_friendly_test::MobileFriendlyTestActions
            {
                crate :: resources :: url_testing_tools :: mobile_friendly_test :: MobileFriendlyTestActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
            }
        }
        pub mod mobile_friendly_test {
            pub mod params {}
            pub struct MobileFriendlyTestActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> MobileFriendlyTestActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Runs Mobile-Friendly Test for a given URL."]
                pub fn run(
                    &self,
                    request: crate::schemas::RunMobileFriendlyTestRequest,
                ) -> RunRequestBuilder {
                    RunRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                    }
                }
            }
            #[doc = "Created via [MobileFriendlyTestActions::run()](struct.MobileFriendlyTestActions.html#method.run)"]
            #[derive(Debug, Clone)]
            pub struct RunRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::RunMobileFriendlyTestRequest,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> RunRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub async fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields).await
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::RunMobileFriendlyTestResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::RunMobileFriendlyTestResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute().await
                }
                async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://searchconsole.googleapis.com/".to_owned();
                    output.push_str("v1/urlTestingTools/mobileFriendlyTest:run");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
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
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
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
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
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
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
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

            futures::task::Poll::Ready(Ok(bytes_written))
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
