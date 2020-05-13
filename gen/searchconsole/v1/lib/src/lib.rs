#![doc = "# Resources and Methods\n    * [url_testing_tools](resources/url_testing_tools/struct.UrlTestingToolsActions.html)\n      * [mobile_friendly_test](resources/url_testing_tools/mobile_friendly_test/struct.MobileFriendlyTestActions.html)\n        * [*run*](resources/url_testing_tools/mobile_friendly_test/struct.RunRequestBuilder.html)\n"]
pub mod scopes {}
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
        #[doc = "Image data in format determined by the mime type. Currently, the format\nwill always be \"image/png\", but this might change in the future."]
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
        #[doc = "Viewsport is not specified using the meta viewport tag. [Learn more]\n(https://support.google.com/webmasters/answer/6352293#viewport_not_configured)."]
        ConfigureViewport,
        #[doc = "Viewport defined to a fixed width. [Learn more]\n(https://support.google.com/webmasters/answer/6352293#fixed-width_viewport)."]
        FixedWidthViewport,
        #[doc = "Unknown rule. Sorry, we don't have any description for the rule that was\nbroken."]
        MobileFriendlyRuleUnspecified,
        #[doc = "Content not sized to viewport. [Learn more]\n(https://support.google.com/webmasters/answer/6352293#content_not_sized_to_viewport)."]
        SizeContentToViewport,
        #[doc = "Touch elements are too close to each other. [Learn more]\n(https://support.google.com/webmasters/answer/6352293#touch_elements_too_close)."]
        TapTargetsTooClose,
        #[doc = "Font size is too small for easy reading on a small screen. [Learn More]\n(https://support.google.com/webmasters/answer/6352293#small_font_size)."]
        UseLegibleFontSizes,
        #[doc = "Plugins incompatible with mobile devices are being used. [Learn more]\n(https://support.google.com/webmasters/answer/6352293#flash_usage)."]
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
            Ok ( match s { "MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendly , "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendlyTestResultUnspecified , "NOT_MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: NotMobileFriendly , _ => return Err ( ( ) ) , } )
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
            Ok ( match value { "MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendly , "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED" => RunMobileFriendlyTestResponseMobileFriendliness :: MobileFriendlyTestResultUnspecified , "NOT_MOBILE_FRIENDLY" => RunMobileFriendlyTestResponseMobileFriendliness :: NotMobileFriendly , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
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
        #[doc = "Inspection terminated in an error state. This indicates a problem in\nGoogle's infrastructure, not a user error. Please try again later."]
        InternalError,
        #[doc = "Google can not access the URL because of a user error such as a robots.txt\nblockage, a 403 or 500 code etc. Please make sure that the URL provided is\naccessible by Googlebot and is not password protected."]
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest,
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
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
                crate :: resources :: url_testing_tools :: mobile_friendly_test :: MobileFriendlyTestActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://searchconsole.googleapis.com/".to_owned();
                    output.push_str("v1/urlTestingTools/mobileFriendlyTest:run");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
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
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
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
