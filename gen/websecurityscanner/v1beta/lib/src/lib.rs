#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [scan_configs](resources/projects/scan_configs/struct.ScanConfigsActions.html)\n        * [*create*](resources/projects/scan_configs/struct.CreateRequestBuilder.html), [*delete*](resources/projects/scan_configs/struct.DeleteRequestBuilder.html), [*get*](resources/projects/scan_configs/struct.GetRequestBuilder.html), [*list*](resources/projects/scan_configs/struct.ListRequestBuilder.html), [*patch*](resources/projects/scan_configs/struct.PatchRequestBuilder.html), [*start*](resources/projects/scan_configs/struct.StartRequestBuilder.html)\n        * [scan_runs](resources/projects/scan_configs/scan_runs/struct.ScanRunsActions.html)\n          * [*get*](resources/projects/scan_configs/scan_runs/struct.GetRequestBuilder.html), [*list*](resources/projects/scan_configs/scan_runs/struct.ListRequestBuilder.html), [*stop*](resources/projects/scan_configs/scan_runs/struct.StopRequestBuilder.html)\n          * [crawled_urls](resources/projects/scan_configs/scan_runs/crawled_urls/struct.CrawledUrlsActions.html)\n            * [*list*](resources/projects/scan_configs/scan_runs/crawled_urls/struct.ListRequestBuilder.html)\n          * [finding_type_stats](resources/projects/scan_configs/scan_runs/finding_type_stats/struct.FindingTypeStatsActions.html)\n            * [*list*](resources/projects/scan_configs/scan_runs/finding_type_stats/struct.ListRequestBuilder.html)\n          * [findings](resources/projects/scan_configs/scan_runs/findings/struct.FindingsActions.html)\n            * [*get*](resources/projects/scan_configs/scan_runs/findings/struct.GetRequestBuilder.html), [*list*](resources/projects/scan_configs/scan_runs/findings/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
}
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
    pub struct Authentication {
        #[doc = "Authentication using a custom account."]
        #[serde(
            rename = "customAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_account: ::std::option::Option<crate::schemas::CustomAccount>,
        #[doc = "Authentication using a Google account."]
        #[serde(
            rename = "googleAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_account: ::std::option::Option<crate::schemas::GoogleAccount>,
        #[doc = "Authentication using Identity-Aware-Proxy (IAP)."]
        #[serde(
            rename = "iapCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iap_credential: ::std::option::Option<crate::schemas::IapCredential>,
    }
    impl ::google_field_selector::FieldSelector for Authentication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Authentication {
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
    pub struct CrawledUrl {
        #[doc = "The body of the request that was used to visit the URL."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<String>,
        #[doc = "The http method of the request that was used to visit the URL, in uppercase."]
        #[serde(
            rename = "httpMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_method: ::std::option::Option<String>,
        #[doc = "The URL that was crawled."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CrawledUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CrawledUrl {
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
    pub struct CustomAccount {
        #[doc = "Required. The login form URL of the website."]
        #[serde(
            rename = "loginUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub login_url: ::std::option::Option<String>,
        #[doc = "Required. Input only. The password of the custom account. The credential is stored encrypted and not returned in any response nor included in audit logs."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "Required. The user name of the custom account."]
        #[serde(
            rename = "username",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub username: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomAccount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomAccount {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
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
    pub struct Finding {
        #[doc = "The body of the request that triggered the vulnerability."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<String>,
        #[doc = "The description of the vulnerability."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The URL where the browser lands when the vulnerability is detected."]
        #[serde(
            rename = "finalUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_url: ::std::option::Option<String>,
        #[doc = "The type of the Finding. Detailed and up-to-date information on findings can be found here: https://cloud.google.com/security-command-center/docs/how-to-remediate-web-security-scanner"]
        #[serde(
            rename = "findingType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finding_type: ::std::option::Option<String>,
        #[doc = "An addon containing information reported for a vulnerability with an HTML form, if any."]
        #[serde(
            rename = "form",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form: ::std::option::Option<crate::schemas::Form>,
        #[doc = "If the vulnerability was originated from nested IFrame, the immediate parent IFrame is reported."]
        #[serde(
            rename = "frameUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_url: ::std::option::Option<String>,
        #[doc = "The URL produced by the server-side fuzzer and used in the request that triggered the vulnerability."]
        #[serde(
            rename = "fuzzedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fuzzed_url: ::std::option::Option<String>,
        #[doc = "The http method of the request that triggered the vulnerability, in uppercase."]
        #[serde(
            rename = "httpMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_method: ::std::option::Option<String>,
        #[doc = "The resource name of the Finding. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'. The finding IDs are generated by the system."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "An addon containing information about outdated libraries."]
        #[serde(
            rename = "outdatedLibrary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outdated_library: ::std::option::Option<crate::schemas::OutdatedLibrary>,
        #[doc = "The URL containing human-readable payload that user can leverage to reproduce the vulnerability."]
        #[serde(
            rename = "reproductionUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reproduction_url: ::std::option::Option<String>,
        #[doc = "The severity level of the reported vulnerability."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::FindingSeverity>,
        #[doc = "The tracking ID uniquely identifies a vulnerability instance across multiple ScanRuns."]
        #[serde(
            rename = "trackingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracking_id: ::std::option::Option<String>,
        #[doc = "An addon containing detailed information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
        #[serde(
            rename = "violatingResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub violating_resource: ::std::option::Option<crate::schemas::ViolatingResource>,
        #[doc = "An addon containing information about vulnerable or missing HTTP headers."]
        #[serde(
            rename = "vulnerableHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerable_headers: ::std::option::Option<crate::schemas::VulnerableHeaders>,
        #[doc = "An addon containing information about request parameters which were found to be vulnerable."]
        #[serde(
            rename = "vulnerableParameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerable_parameters: ::std::option::Option<crate::schemas::VulnerableParameters>,
        #[doc = "An addon containing information reported for an XSS, if any."]
        #[serde(
            rename = "xss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xss: ::std::option::Option<crate::schemas::Xss>,
    }
    impl ::google_field_selector::FieldSelector for Finding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Finding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FindingSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "No severity specified. The default value."]
        SeverityUnspecified,
    }
    impl FindingSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                FindingSeverity::Critical => "CRITICAL",
                FindingSeverity::High => "HIGH",
                FindingSeverity::Low => "LOW",
                FindingSeverity::Medium => "MEDIUM",
                FindingSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FindingSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FindingSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FindingSeverity, ()> {
            Ok(match s {
                "CRITICAL" => FindingSeverity::Critical,
                "HIGH" => FindingSeverity::High,
                "LOW" => FindingSeverity::Low,
                "MEDIUM" => FindingSeverity::Medium,
                "SEVERITY_UNSPECIFIED" => FindingSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FindingSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FindingSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FindingSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => FindingSeverity::Critical,
                "HIGH" => FindingSeverity::High,
                "LOW" => FindingSeverity::Low,
                "MEDIUM" => FindingSeverity::Medium,
                "SEVERITY_UNSPECIFIED" => FindingSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FindingSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindingSeverity {
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
    pub struct FindingTypeStats {
        #[doc = "The count of findings belonging to this finding type."]
        #[serde(
            rename = "findingCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finding_count: ::std::option::Option<i32>,
        #[doc = "The finding type associated with the stats."]
        #[serde(
            rename = "findingType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finding_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FindingTypeStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindingTypeStats {
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
    pub struct Form {
        #[doc = "! The URI where to send the form when it's submitted."]
        #[serde(
            rename = "actionUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_uri: ::std::option::Option<String>,
        #[doc = "! The names of form fields related to the vulnerability."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Form {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Form {
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
    pub struct GoogleAccount {
        #[doc = "Required. Input only. The password of the Google account. The credential is stored encrypted and not returned in any response nor included in audit logs."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "Required. The user name of the Google account."]
        #[serde(
            rename = "username",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub username: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAccount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAccount {
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
    pub struct Header {
        #[doc = "Header name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Header value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Header {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Header {
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
    pub struct IapCredential {
        #[doc = "Authentication configuration when Web-Security-Scanner service account is added in Identity-Aware-Proxy (IAP) access policies."]
        #[serde(
            rename = "iapTestServiceAccountInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iap_test_service_account_info:
            ::std::option::Option<crate::schemas::IapTestServiceAccountInfo>,
    }
    impl ::google_field_selector::FieldSelector for IapCredential {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IapCredential {
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
    pub struct IapTestServiceAccountInfo {
        #[doc = "Required. Describes OAuth2 Client ID of resources protected by Identity-Aware-Proxy(IAP)."]
        #[serde(
            rename = "targetAudienceClientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_audience_client_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IapTestServiceAccountInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IapTestServiceAccountInfo {
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
    pub struct ListCrawledUrlsResponse {
        #[doc = "The list of CrawledUrls returned."]
        #[serde(
            rename = "crawledUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crawled_urls: ::std::option::Option<Vec<crate::schemas::CrawledUrl>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListCrawledUrlsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCrawledUrlsResponse {
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
    pub struct ListFindingTypeStatsResponse {
        #[doc = "The list of FindingTypeStats returned."]
        #[serde(
            rename = "findingTypeStats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finding_type_stats: ::std::option::Option<Vec<crate::schemas::FindingTypeStats>>,
    }
    impl ::google_field_selector::FieldSelector for ListFindingTypeStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListFindingTypeStatsResponse {
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
    pub struct ListFindingsResponse {
        #[doc = "The list of Findings returned."]
        #[serde(
            rename = "findings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub findings: ::std::option::Option<Vec<crate::schemas::Finding>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListFindingsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListFindingsResponse {
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
    pub struct ListScanConfigsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of ScanConfigs returned."]
        #[serde(
            rename = "scanConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan_configs: ::std::option::Option<Vec<crate::schemas::ScanConfig>>,
    }
    impl ::google_field_selector::FieldSelector for ListScanConfigsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListScanConfigsResponse {
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
    pub struct ListScanRunsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of ScanRuns returned."]
        #[serde(
            rename = "scanRuns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan_runs: ::std::option::Option<Vec<crate::schemas::ScanRun>>,
    }
    impl ::google_field_selector::FieldSelector for ListScanRunsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListScanRunsResponse {
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
    pub struct OutdatedLibrary {
        #[doc = "URLs to learn more information about the vulnerabilities in the library."]
        #[serde(
            rename = "learnMoreUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_more_urls: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the outdated library."]
        #[serde(
            rename = "libraryName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub library_name: ::std::option::Option<String>,
        #[doc = "The version number."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OutdatedLibrary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OutdatedLibrary {
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
    pub struct ScanConfig {
        #[doc = "The authentication configuration. If specified, service will use the authentication configuration during scanning."]
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "The excluded URL patterns as described in https://cloud.google.com/security-command-center/docs/how-to-use-web-security-scanner#excluding_urls"]
        #[serde(
            rename = "blacklistPatterns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blacklist_patterns: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The user provided display name of the ScanConfig."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Controls export of scan configurations and results to Security Command Center."]
        #[serde(
            rename = "exportToSecurityCommandCenter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub export_to_security_command_center:
            ::std::option::Option<crate::schemas::ScanConfigExportToSecurityCommandCenter>,
        #[doc = "Latest ScanRun if available."]
        #[serde(
            rename = "latestRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_run: ::std::option::Option<crate::schemas::ScanRun>,
        #[doc = "Whether the scan config is managed by Web Security Scanner, output only."]
        #[serde(
            rename = "managedScan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_scan: ::std::option::Option<bool>,
        #[doc = "The maximum QPS during scanning. A valid value ranges from 5 to 20 inclusively. If the field is unspecified or its value is set 0, server will default to 15. Other values outside of [5, 20] range will be rejected with INVALID_ARGUMENT error."]
        #[serde(
            rename = "maxQps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_qps: ::std::option::Option<i32>,
        #[doc = "The resource name of the ScanConfig. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}'. The ScanConfig IDs are generated by the system."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The risk level selected for the scan"]
        #[serde(
            rename = "riskLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub risk_level: ::std::option::Option<crate::schemas::ScanConfigRiskLevel>,
        #[doc = "The schedule of the ScanConfig."]
        #[serde(
            rename = "schedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schedule: ::std::option::Option<crate::schemas::Schedule>,
        #[doc = "Required. The starting URLs from which the scanner finds site pages."]
        #[serde(
            rename = "startingUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub starting_urls: ::std::option::Option<Vec<String>>,
        #[doc = "Whether the scan configuration has enabled static IP address scan feature. If enabled, the scanner will access applications from static IP addresses."]
        #[serde(
            rename = "staticIpScan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub static_ip_scan: ::std::option::Option<bool>,
        #[doc = "Set of Google Cloud platforms targeted by the scan. If empty, APP_ENGINE will be used as a default."]
        #[serde(
            rename = "targetPlatforms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_platforms:
            ::std::option::Option<Vec<crate::schemas::ScanConfigTargetPlatformsItems>>,
        #[doc = "The user agent used during scanning."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<crate::schemas::ScanConfigUserAgent>,
    }
    impl ::google_field_selector::FieldSelector for ScanConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigExportToSecurityCommandCenter {
        #[doc = "Do not export results of this scan to Security Command Center."]
        Disabled,
        #[doc = "Export results of this scan to Security Command Center."]
        Enabled,
        #[doc = "Use default, which is ENABLED."]
        ExportToSecurityCommandCenterUnspecified,
    }
    impl ScanConfigExportToSecurityCommandCenter {
        pub fn as_str(self) -> &'static str {
            match self { ScanConfigExportToSecurityCommandCenter :: Disabled => "DISABLED" , ScanConfigExportToSecurityCommandCenter :: Enabled => "ENABLED" , ScanConfigExportToSecurityCommandCenter :: ExportToSecurityCommandCenterUnspecified => "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ScanConfigExportToSecurityCommandCenter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanConfigExportToSecurityCommandCenter {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanConfigExportToSecurityCommandCenter, ()> {
            Ok (match s { "DISABLED" => ScanConfigExportToSecurityCommandCenter :: Disabled , "ENABLED" => ScanConfigExportToSecurityCommandCenter :: Enabled , "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED" => ScanConfigExportToSecurityCommandCenter :: ExportToSecurityCommandCenterUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ScanConfigExportToSecurityCommandCenter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigExportToSecurityCommandCenter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigExportToSecurityCommandCenter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DISABLED" => ScanConfigExportToSecurityCommandCenter :: Disabled , "ENABLED" => ScanConfigExportToSecurityCommandCenter :: Enabled , "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED" => ScanConfigExportToSecurityCommandCenter :: ExportToSecurityCommandCenterUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanConfigExportToSecurityCommandCenter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfigExportToSecurityCommandCenter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigRiskLevel {
        #[doc = "Lower impact scanning"]
        Low,
        #[doc = "Normal scanning (Recommended)"]
        Normal,
        #[doc = "Use default, which is NORMAL."]
        RiskLevelUnspecified,
    }
    impl ScanConfigRiskLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigRiskLevel::Low => "LOW",
                ScanConfigRiskLevel::Normal => "NORMAL",
                ScanConfigRiskLevel::RiskLevelUnspecified => "RISK_LEVEL_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanConfigRiskLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanConfigRiskLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanConfigRiskLevel, ()> {
            Ok(match s {
                "LOW" => ScanConfigRiskLevel::Low,
                "NORMAL" => ScanConfigRiskLevel::Normal,
                "RISK_LEVEL_UNSPECIFIED" => ScanConfigRiskLevel::RiskLevelUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanConfigRiskLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigRiskLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigRiskLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOW" => ScanConfigRiskLevel::Low,
                "NORMAL" => ScanConfigRiskLevel::Normal,
                "RISK_LEVEL_UNSPECIFIED" => ScanConfigRiskLevel::RiskLevelUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanConfigRiskLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfigRiskLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigTargetPlatformsItems {
        #[doc = "Google App Engine service."]
        AppEngine,
        #[doc = "Google Compute Engine service."]
        Compute,
        #[doc = "The target platform is unknown. Requests with this enum value will be rejected with INVALID_ARGUMENT error."]
        TargetPlatformUnspecified,
    }
    impl ScanConfigTargetPlatformsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigTargetPlatformsItems::AppEngine => "APP_ENGINE",
                ScanConfigTargetPlatformsItems::Compute => "COMPUTE",
                ScanConfigTargetPlatformsItems::TargetPlatformUnspecified => {
                    "TARGET_PLATFORM_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanConfigTargetPlatformsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanConfigTargetPlatformsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanConfigTargetPlatformsItems, ()> {
            Ok(match s {
                "APP_ENGINE" => ScanConfigTargetPlatformsItems::AppEngine,
                "COMPUTE" => ScanConfigTargetPlatformsItems::Compute,
                "TARGET_PLATFORM_UNSPECIFIED" => {
                    ScanConfigTargetPlatformsItems::TargetPlatformUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanConfigTargetPlatformsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigTargetPlatformsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigTargetPlatformsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_ENGINE" => ScanConfigTargetPlatformsItems::AppEngine,
                "COMPUTE" => ScanConfigTargetPlatformsItems::Compute,
                "TARGET_PLATFORM_UNSPECIFIED" => {
                    ScanConfigTargetPlatformsItems::TargetPlatformUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanConfigTargetPlatformsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfigTargetPlatformsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigUserAgent {
        #[doc = "Chrome on Android."]
        ChromeAndroid,
        #[doc = "Chrome on Linux. This is the service default if unspecified."]
        ChromeLinux,
        #[doc = "Safari on IPhone."]
        SafariIphone,
        #[doc = "The user agent is unknown. Service will default to CHROME_LINUX."]
        UserAgentUnspecified,
    }
    impl ScanConfigUserAgent {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigUserAgent::ChromeAndroid => "CHROME_ANDROID",
                ScanConfigUserAgent::ChromeLinux => "CHROME_LINUX",
                ScanConfigUserAgent::SafariIphone => "SAFARI_IPHONE",
                ScanConfigUserAgent::UserAgentUnspecified => "USER_AGENT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanConfigUserAgent {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanConfigUserAgent {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanConfigUserAgent, ()> {
            Ok(match s {
                "CHROME_ANDROID" => ScanConfigUserAgent::ChromeAndroid,
                "CHROME_LINUX" => ScanConfigUserAgent::ChromeLinux,
                "SAFARI_IPHONE" => ScanConfigUserAgent::SafariIphone,
                "USER_AGENT_UNSPECIFIED" => ScanConfigUserAgent::UserAgentUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanConfigUserAgent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigUserAgent {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigUserAgent {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHROME_ANDROID" => ScanConfigUserAgent::ChromeAndroid,
                "CHROME_LINUX" => ScanConfigUserAgent::ChromeLinux,
                "SAFARI_IPHONE" => ScanConfigUserAgent::SafariIphone,
                "USER_AGENT_UNSPECIFIED" => ScanConfigUserAgent::UserAgentUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanConfigUserAgent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfigUserAgent {
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
    pub struct ScanConfigError {
        #[doc = "Indicates the reason code for a configuration failure."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ScanConfigErrorCode>,
        #[doc = "Indicates the full name of the ScanConfig field that triggers this error, for example \"scan_config.max_qps\". This field is provided for troubleshooting purposes only and its actual value can change in the future."]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ScanConfigError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfigError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigErrorCode {
        #[doc = "One of the seed URLs is an App Engine URL but we cannot validate the scan settings due to an App Engine API backend error."]
        AppengineApiBackendError,
        #[doc = "One of the seed URLs is an App Engine URL but we cannot access the App Engine API to validate scan settings."]
        AppengineApiNotAccessible,
        #[doc = "One of the seed URLs is an App Engine URL but the Default Host of the App Engine is not set."]
        AppengineDefaultHostMissing,
        #[doc = "Google corporate accounts can not be used for scanning."]
        CannotUseGoogleComAccount,
        #[doc = "The account of the scan creator can not be used for scanning."]
        CannotUseOwnerAccount,
        #[doc = "There is no error."]
        CodeUnspecified,
        #[doc = "This scan targets Compute Engine, but we cannot validate scan settings due to a Compute Engine API backend error."]
        ComputeApiBackendError,
        #[doc = "This scan targets Compute Engine, but we cannot access the Compute Engine API to validate the scan settings."]
        ComputeApiNotAccessible,
        #[doc = "The Custom Login URL does not belong to the current project."]
        CustomLoginUrlDoesNotBelongToCurrentProject,
        #[doc = "The Custom Login URL has a non-routable IP address."]
        CustomLoginUrlHasNonRoutableIpAddress,
        #[doc = "The Custom Login URL has an IP address which is not reserved for the current project."]
        CustomLoginUrlHasUnreservedIpAddress,
        #[doc = "The Custom Login URL is malformed (can not be parsed)."]
        CustomLoginUrlMalformed,
        #[doc = "The Custom Login URL is mapped to a non-routable IP address in DNS."]
        CustomLoginUrlMappedToNonRoutableAddress,
        #[doc = "The Custom Login URL is mapped to an IP address which is not reserved for the current project."]
        CustomLoginUrlMappedToUnreservedAddress,
        #[doc = "Another scan with the same name (case-sensitive) already exists."]
        DuplicateScanName,
        #[doc = "There was an error trying to authenticate to the scan target."]
        FailedToAuthenticateToTarget,
        #[doc = "Finding type value is not specified in the list findings request."]
        FindingTypeUnspecified,
        #[doc = "Scan targets Compute Engine, yet current project was not whitelisted for Google Compute Engine Scanning Alpha access."]
        ForbiddenToScanCompute,
        #[doc = "User tries to update managed scan"]
        ForbiddenUpdateToManagedScan,
        #[doc = "Indicates an internal server error. Please DO NOT USE THIS ERROR CODE unless the root cause is truly unknown."]
        InternalError,
        #[doc = "A field is set to an invalid value."]
        InvalidFieldValue,
        #[doc = "The supplied filter is malformed. For example, it can not be parsed, does not have a filter type in expression, or the same filter type appears more than once."]
        MalformedFilter,
        #[doc = "The supplied resource name is malformed (can not be parsed)."]
        MalformedResourceName,
        #[doc = "There is no error."]
        Ok,
        #[doc = "The current project is not in an active state."]
        ProjectInactive,
        #[doc = "A required field is not set."]
        RequiredField,
        #[doc = "Project id, scanconfig id, scanrun id, or finding id are not consistent with each other in resource name."]
        ResourceNameInconsistent,
        #[doc = "The scan being requested to start is already running."]
        ScanAlreadyRunning,
        #[doc = "The scan that was requested to be stopped is not running."]
        ScanNotRunning,
        #[doc = "One of the seed URLs does not belong to the current project."]
        SeedUrlDoesNotBelongToCurrentProject,
        #[doc = "One of the seed URLs has on-routable IP address."]
        SeedUrlHasNonRoutableIpAddress,
        #[doc = "One of the seed URLs has an IP address that is not reserved for the current project."]
        SeedUrlHasUnreservedIpAddress,
        #[doc = "One of the seed URLs is malformed (can not be parsed)."]
        SeedUrlMalformed,
        #[doc = "One of the seed URLs is mapped to a non-routable IP address in DNS."]
        SeedUrlMappedToNonRoutableAddress,
        #[doc = "One of the seed URLs is mapped to an IP address which is not reserved for the current project."]
        SeedUrlMappedToUnreservedAddress,
        #[doc = "The Web Security Scanner service account is not configured under the project."]
        ServiceAccountNotConfigured,
        #[doc = "A project has reached the maximum number of scans."]
        TooManyScans,
        #[doc = "Resolving the details of the current project fails."]
        UnableToResolveProjectInfo,
        #[doc = "One or more blacklist patterns were in the wrong format."]
        UnsupportedBlacklistPatternFormat,
        #[doc = "The supplied filter is not supported."]
        UnsupportedFilter,
        #[doc = "The supplied finding type is not supported. For example, we do not provide findings of the given finding type."]
        UnsupportedFindingType,
        #[doc = "The URL scheme of one or more of the supplied URLs is not supported."]
        UnsupportedUrlScheme,
    }
    impl ScanConfigErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigErrorCode::AppengineApiBackendError => "APPENGINE_API_BACKEND_ERROR",
                ScanConfigErrorCode::AppengineApiNotAccessible => "APPENGINE_API_NOT_ACCESSIBLE",
                ScanConfigErrorCode::AppengineDefaultHostMissing => {
                    "APPENGINE_DEFAULT_HOST_MISSING"
                }
                ScanConfigErrorCode::CannotUseGoogleComAccount => "CANNOT_USE_GOOGLE_COM_ACCOUNT",
                ScanConfigErrorCode::CannotUseOwnerAccount => "CANNOT_USE_OWNER_ACCOUNT",
                ScanConfigErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ScanConfigErrorCode::ComputeApiBackendError => "COMPUTE_API_BACKEND_ERROR",
                ScanConfigErrorCode::ComputeApiNotAccessible => "COMPUTE_API_NOT_ACCESSIBLE",
                ScanConfigErrorCode::CustomLoginUrlDoesNotBelongToCurrentProject => {
                    "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT"
                }
                ScanConfigErrorCode::CustomLoginUrlHasNonRoutableIpAddress => {
                    "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS"
                }
                ScanConfigErrorCode::CustomLoginUrlHasUnreservedIpAddress => {
                    "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS"
                }
                ScanConfigErrorCode::CustomLoginUrlMalformed => "CUSTOM_LOGIN_URL_MALFORMED",
                ScanConfigErrorCode::CustomLoginUrlMappedToNonRoutableAddress => {
                    "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS"
                }
                ScanConfigErrorCode::CustomLoginUrlMappedToUnreservedAddress => {
                    "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS"
                }
                ScanConfigErrorCode::DuplicateScanName => "DUPLICATE_SCAN_NAME",
                ScanConfigErrorCode::FailedToAuthenticateToTarget => {
                    "FAILED_TO_AUTHENTICATE_TO_TARGET"
                }
                ScanConfigErrorCode::FindingTypeUnspecified => "FINDING_TYPE_UNSPECIFIED",
                ScanConfigErrorCode::ForbiddenToScanCompute => "FORBIDDEN_TO_SCAN_COMPUTE",
                ScanConfigErrorCode::ForbiddenUpdateToManagedScan => {
                    "FORBIDDEN_UPDATE_TO_MANAGED_SCAN"
                }
                ScanConfigErrorCode::InternalError => "INTERNAL_ERROR",
                ScanConfigErrorCode::InvalidFieldValue => "INVALID_FIELD_VALUE",
                ScanConfigErrorCode::MalformedFilter => "MALFORMED_FILTER",
                ScanConfigErrorCode::MalformedResourceName => "MALFORMED_RESOURCE_NAME",
                ScanConfigErrorCode::Ok => "OK",
                ScanConfigErrorCode::ProjectInactive => "PROJECT_INACTIVE",
                ScanConfigErrorCode::RequiredField => "REQUIRED_FIELD",
                ScanConfigErrorCode::ResourceNameInconsistent => "RESOURCE_NAME_INCONSISTENT",
                ScanConfigErrorCode::ScanAlreadyRunning => "SCAN_ALREADY_RUNNING",
                ScanConfigErrorCode::ScanNotRunning => "SCAN_NOT_RUNNING",
                ScanConfigErrorCode::SeedUrlDoesNotBelongToCurrentProject => {
                    "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT"
                }
                ScanConfigErrorCode::SeedUrlHasNonRoutableIpAddress => {
                    "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS"
                }
                ScanConfigErrorCode::SeedUrlHasUnreservedIpAddress => {
                    "SEED_URL_HAS_UNRESERVED_IP_ADDRESS"
                }
                ScanConfigErrorCode::SeedUrlMalformed => "SEED_URL_MALFORMED",
                ScanConfigErrorCode::SeedUrlMappedToNonRoutableAddress => {
                    "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS"
                }
                ScanConfigErrorCode::SeedUrlMappedToUnreservedAddress => {
                    "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS"
                }
                ScanConfigErrorCode::ServiceAccountNotConfigured => {
                    "SERVICE_ACCOUNT_NOT_CONFIGURED"
                }
                ScanConfigErrorCode::TooManyScans => "TOO_MANY_SCANS",
                ScanConfigErrorCode::UnableToResolveProjectInfo => "UNABLE_TO_RESOLVE_PROJECT_INFO",
                ScanConfigErrorCode::UnsupportedBlacklistPatternFormat => {
                    "UNSUPPORTED_BLACKLIST_PATTERN_FORMAT"
                }
                ScanConfigErrorCode::UnsupportedFilter => "UNSUPPORTED_FILTER",
                ScanConfigErrorCode::UnsupportedFindingType => "UNSUPPORTED_FINDING_TYPE",
                ScanConfigErrorCode::UnsupportedUrlScheme => "UNSUPPORTED_URL_SCHEME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanConfigErrorCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanConfigErrorCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanConfigErrorCode, ()> {
            Ok(match s {
                "APPENGINE_API_BACKEND_ERROR" => ScanConfigErrorCode::AppengineApiBackendError,
                "APPENGINE_API_NOT_ACCESSIBLE" => ScanConfigErrorCode::AppengineApiNotAccessible,
                "APPENGINE_DEFAULT_HOST_MISSING" => {
                    ScanConfigErrorCode::AppengineDefaultHostMissing
                }
                "CANNOT_USE_GOOGLE_COM_ACCOUNT" => ScanConfigErrorCode::CannotUseGoogleComAccount,
                "CANNOT_USE_OWNER_ACCOUNT" => ScanConfigErrorCode::CannotUseOwnerAccount,
                "CODE_UNSPECIFIED" => ScanConfigErrorCode::CodeUnspecified,
                "COMPUTE_API_BACKEND_ERROR" => ScanConfigErrorCode::ComputeApiBackendError,
                "COMPUTE_API_NOT_ACCESSIBLE" => ScanConfigErrorCode::ComputeApiNotAccessible,
                "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT" => {
                    ScanConfigErrorCode::CustomLoginUrlDoesNotBelongToCurrentProject
                }
                "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlHasNonRoutableIpAddress
                }
                "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlHasUnreservedIpAddress
                }
                "CUSTOM_LOGIN_URL_MALFORMED" => ScanConfigErrorCode::CustomLoginUrlMalformed,
                "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlMappedToNonRoutableAddress
                }
                "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlMappedToUnreservedAddress
                }
                "DUPLICATE_SCAN_NAME" => ScanConfigErrorCode::DuplicateScanName,
                "FAILED_TO_AUTHENTICATE_TO_TARGET" => {
                    ScanConfigErrorCode::FailedToAuthenticateToTarget
                }
                "FINDING_TYPE_UNSPECIFIED" => ScanConfigErrorCode::FindingTypeUnspecified,
                "FORBIDDEN_TO_SCAN_COMPUTE" => ScanConfigErrorCode::ForbiddenToScanCompute,
                "FORBIDDEN_UPDATE_TO_MANAGED_SCAN" => {
                    ScanConfigErrorCode::ForbiddenUpdateToManagedScan
                }
                "INTERNAL_ERROR" => ScanConfigErrorCode::InternalError,
                "INVALID_FIELD_VALUE" => ScanConfigErrorCode::InvalidFieldValue,
                "MALFORMED_FILTER" => ScanConfigErrorCode::MalformedFilter,
                "MALFORMED_RESOURCE_NAME" => ScanConfigErrorCode::MalformedResourceName,
                "OK" => ScanConfigErrorCode::Ok,
                "PROJECT_INACTIVE" => ScanConfigErrorCode::ProjectInactive,
                "REQUIRED_FIELD" => ScanConfigErrorCode::RequiredField,
                "RESOURCE_NAME_INCONSISTENT" => ScanConfigErrorCode::ResourceNameInconsistent,
                "SCAN_ALREADY_RUNNING" => ScanConfigErrorCode::ScanAlreadyRunning,
                "SCAN_NOT_RUNNING" => ScanConfigErrorCode::ScanNotRunning,
                "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT" => {
                    ScanConfigErrorCode::SeedUrlDoesNotBelongToCurrentProject
                }
                "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlHasNonRoutableIpAddress
                }
                "SEED_URL_HAS_UNRESERVED_IP_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlHasUnreservedIpAddress
                }
                "SEED_URL_MALFORMED" => ScanConfigErrorCode::SeedUrlMalformed,
                "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlMappedToNonRoutableAddress
                }
                "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlMappedToUnreservedAddress
                }
                "SERVICE_ACCOUNT_NOT_CONFIGURED" => {
                    ScanConfigErrorCode::ServiceAccountNotConfigured
                }
                "TOO_MANY_SCANS" => ScanConfigErrorCode::TooManyScans,
                "UNABLE_TO_RESOLVE_PROJECT_INFO" => ScanConfigErrorCode::UnableToResolveProjectInfo,
                "UNSUPPORTED_BLACKLIST_PATTERN_FORMAT" => {
                    ScanConfigErrorCode::UnsupportedBlacklistPatternFormat
                }
                "UNSUPPORTED_FILTER" => ScanConfigErrorCode::UnsupportedFilter,
                "UNSUPPORTED_FINDING_TYPE" => ScanConfigErrorCode::UnsupportedFindingType,
                "UNSUPPORTED_URL_SCHEME" => ScanConfigErrorCode::UnsupportedUrlScheme,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanConfigErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPENGINE_API_BACKEND_ERROR" => ScanConfigErrorCode::AppengineApiBackendError,
                "APPENGINE_API_NOT_ACCESSIBLE" => ScanConfigErrorCode::AppengineApiNotAccessible,
                "APPENGINE_DEFAULT_HOST_MISSING" => {
                    ScanConfigErrorCode::AppengineDefaultHostMissing
                }
                "CANNOT_USE_GOOGLE_COM_ACCOUNT" => ScanConfigErrorCode::CannotUseGoogleComAccount,
                "CANNOT_USE_OWNER_ACCOUNT" => ScanConfigErrorCode::CannotUseOwnerAccount,
                "CODE_UNSPECIFIED" => ScanConfigErrorCode::CodeUnspecified,
                "COMPUTE_API_BACKEND_ERROR" => ScanConfigErrorCode::ComputeApiBackendError,
                "COMPUTE_API_NOT_ACCESSIBLE" => ScanConfigErrorCode::ComputeApiNotAccessible,
                "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT" => {
                    ScanConfigErrorCode::CustomLoginUrlDoesNotBelongToCurrentProject
                }
                "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlHasNonRoutableIpAddress
                }
                "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlHasUnreservedIpAddress
                }
                "CUSTOM_LOGIN_URL_MALFORMED" => ScanConfigErrorCode::CustomLoginUrlMalformed,
                "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlMappedToNonRoutableAddress
                }
                "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlMappedToUnreservedAddress
                }
                "DUPLICATE_SCAN_NAME" => ScanConfigErrorCode::DuplicateScanName,
                "FAILED_TO_AUTHENTICATE_TO_TARGET" => {
                    ScanConfigErrorCode::FailedToAuthenticateToTarget
                }
                "FINDING_TYPE_UNSPECIFIED" => ScanConfigErrorCode::FindingTypeUnspecified,
                "FORBIDDEN_TO_SCAN_COMPUTE" => ScanConfigErrorCode::ForbiddenToScanCompute,
                "FORBIDDEN_UPDATE_TO_MANAGED_SCAN" => {
                    ScanConfigErrorCode::ForbiddenUpdateToManagedScan
                }
                "INTERNAL_ERROR" => ScanConfigErrorCode::InternalError,
                "INVALID_FIELD_VALUE" => ScanConfigErrorCode::InvalidFieldValue,
                "MALFORMED_FILTER" => ScanConfigErrorCode::MalformedFilter,
                "MALFORMED_RESOURCE_NAME" => ScanConfigErrorCode::MalformedResourceName,
                "OK" => ScanConfigErrorCode::Ok,
                "PROJECT_INACTIVE" => ScanConfigErrorCode::ProjectInactive,
                "REQUIRED_FIELD" => ScanConfigErrorCode::RequiredField,
                "RESOURCE_NAME_INCONSISTENT" => ScanConfigErrorCode::ResourceNameInconsistent,
                "SCAN_ALREADY_RUNNING" => ScanConfigErrorCode::ScanAlreadyRunning,
                "SCAN_NOT_RUNNING" => ScanConfigErrorCode::ScanNotRunning,
                "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT" => {
                    ScanConfigErrorCode::SeedUrlDoesNotBelongToCurrentProject
                }
                "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlHasNonRoutableIpAddress
                }
                "SEED_URL_HAS_UNRESERVED_IP_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlHasUnreservedIpAddress
                }
                "SEED_URL_MALFORMED" => ScanConfigErrorCode::SeedUrlMalformed,
                "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlMappedToNonRoutableAddress
                }
                "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlMappedToUnreservedAddress
                }
                "SERVICE_ACCOUNT_NOT_CONFIGURED" => {
                    ScanConfigErrorCode::ServiceAccountNotConfigured
                }
                "TOO_MANY_SCANS" => ScanConfigErrorCode::TooManyScans,
                "UNABLE_TO_RESOLVE_PROJECT_INFO" => ScanConfigErrorCode::UnableToResolveProjectInfo,
                "UNSUPPORTED_BLACKLIST_PATTERN_FORMAT" => {
                    ScanConfigErrorCode::UnsupportedBlacklistPatternFormat
                }
                "UNSUPPORTED_FILTER" => ScanConfigErrorCode::UnsupportedFilter,
                "UNSUPPORTED_FINDING_TYPE" => ScanConfigErrorCode::UnsupportedFindingType,
                "UNSUPPORTED_URL_SCHEME" => ScanConfigErrorCode::UnsupportedUrlScheme,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanConfigErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfigErrorCode {
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
    pub struct ScanRun {
        #[doc = "The time at which the ScanRun reached termination state - that the ScanRun is either finished or stopped by user."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "If result_state is an ERROR, this field provides the primary reason for scan's termination and more details, if such are available."]
        #[serde(
            rename = "errorTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_trace: ::std::option::Option<crate::schemas::ScanRunErrorTrace>,
        #[doc = "The execution state of the ScanRun."]
        #[serde(
            rename = "executionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_state: ::std::option::Option<crate::schemas::ScanRunExecutionState>,
        #[doc = "Whether the scan run has found any vulnerabilities."]
        #[serde(
            rename = "hasVulnerabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_vulnerabilities: ::std::option::Option<bool>,
        #[doc = "The resource name of the ScanRun. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'. The ScanRun IDs are generated by the system."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The percentage of total completion ranging from 0 to 100. If the scan is in queue, the value is 0. If the scan is running, the value ranges from 0 to 100. If the scan is finished, the value is 100."]
        #[serde(
            rename = "progressPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percent: ::std::option::Option<i32>,
        #[doc = "The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
        #[serde(
            rename = "resultState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result_state: ::std::option::Option<crate::schemas::ScanRunResultState>,
        #[doc = "The time at which the ScanRun started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The number of URLs crawled during this ScanRun. If the scan is in progress, the value represents the number of URLs crawled up to now."]
        #[serde(
            rename = "urlsCrawledCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub urls_crawled_count: ::std::option::Option<i64>,
        #[doc = "The number of URLs tested during this ScanRun. If the scan is in progress, the value represents the number of URLs tested up to now. The number of URLs tested is usually larger than the number URLS crawled because typically a crawled URL is tested with multiple test payloads."]
        #[serde(
            rename = "urlsTestedCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub urls_tested_count: ::std::option::Option<i64>,
        #[doc = "A list of warnings, if such are encountered during this scan run."]
        #[serde(
            rename = "warningTraces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning_traces: ::std::option::Option<Vec<crate::schemas::ScanRunWarningTrace>>,
    }
    impl ::google_field_selector::FieldSelector for ScanRun {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRun {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunExecutionState {
        #[doc = "Represents an invalid state caused by internal server error. This value should never be returned."]
        ExecutionStateUnspecified,
        #[doc = "The scan is either finished or stopped by user."]
        Finished,
        #[doc = "The scan is waiting in the queue."]
        Queued,
        #[doc = "The scan is in progress."]
        Scanning,
    }
    impl ScanRunExecutionState {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunExecutionState::ExecutionStateUnspecified => "EXECUTION_STATE_UNSPECIFIED",
                ScanRunExecutionState::Finished => "FINISHED",
                ScanRunExecutionState::Queued => "QUEUED",
                ScanRunExecutionState::Scanning => "SCANNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanRunExecutionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanRunExecutionState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanRunExecutionState, ()> {
            Ok(match s {
                "EXECUTION_STATE_UNSPECIFIED" => ScanRunExecutionState::ExecutionStateUnspecified,
                "FINISHED" => ScanRunExecutionState::Finished,
                "QUEUED" => ScanRunExecutionState::Queued,
                "SCANNING" => ScanRunExecutionState::Scanning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanRunExecutionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunExecutionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunExecutionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXECUTION_STATE_UNSPECIFIED" => ScanRunExecutionState::ExecutionStateUnspecified,
                "FINISHED" => ScanRunExecutionState::Finished,
                "QUEUED" => ScanRunExecutionState::Queued,
                "SCANNING" => ScanRunExecutionState::Scanning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanRunExecutionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRunExecutionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunResultState {
        #[doc = "The scan finished with errors."]
        Error,
        #[doc = "The scan was terminated by user."]
        Killed,
        #[doc = "Default value. This value is returned when the ScanRun is not yet finished."]
        ResultStateUnspecified,
        #[doc = "The scan finished without errors."]
        Success,
    }
    impl ScanRunResultState {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunResultState::Error => "ERROR",
                ScanRunResultState::Killed => "KILLED",
                ScanRunResultState::ResultStateUnspecified => "RESULT_STATE_UNSPECIFIED",
                ScanRunResultState::Success => "SUCCESS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanRunResultState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanRunResultState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanRunResultState, ()> {
            Ok(match s {
                "ERROR" => ScanRunResultState::Error,
                "KILLED" => ScanRunResultState::Killed,
                "RESULT_STATE_UNSPECIFIED" => ScanRunResultState::ResultStateUnspecified,
                "SUCCESS" => ScanRunResultState::Success,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanRunResultState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunResultState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunResultState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => ScanRunResultState::Error,
                "KILLED" => ScanRunResultState::Killed,
                "RESULT_STATE_UNSPECIFIED" => ScanRunResultState::ResultStateUnspecified,
                "SUCCESS" => ScanRunResultState::Success,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanRunResultState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRunResultState {
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
    pub struct ScanRunErrorTrace {
        #[doc = "Indicates the error reason code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ScanRunErrorTraceCode>,
        #[doc = "If the scan encounters TOO_MANY_HTTP_ERRORS, this field indicates the most common HTTP error code, if such is available. For example, if this code is 404, the scan has encountered too many NOT_FOUND responses."]
        #[serde(
            rename = "mostCommonHttpErrorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub most_common_http_error_code: ::std::option::Option<i32>,
        #[doc = "If the scan encounters SCAN_CONFIG_ISSUE error, this field has the error message encountered during scan configuration validation that is performed before each scan run."]
        #[serde(
            rename = "scanConfigError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan_config_error: ::std::option::Option<crate::schemas::ScanConfigError>,
    }
    impl ::google_field_selector::FieldSelector for ScanRunErrorTrace {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRunErrorTrace {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunErrorTraceCode {
        #[doc = "Indicates an authentication error, usually due to outdated ScanConfig authentication settings."]
        AuthenticationConfigIssue,
        #[doc = "Default value is never used."]
        CodeUnspecified,
        #[doc = "Indicates that the scan run failed due to an internal server error."]
        InternalError,
        #[doc = "Indicates a scan configuration error, usually due to outdated ScanConfig settings, such as starting_urls or the DNS configuration."]
        ScanConfigIssue,
        #[doc = "Indicates a scan operation timeout, usually caused by a very large site."]
        TimedOutWhileScanning,
        #[doc = "Indicates that a scan encountered numerous errors from the web site pages. When available, most_common_http_error_code field indicates the most common HTTP error code encountered during the scan."]
        TooManyHttpErrors,
        #[doc = "Indicates that a scan encountered excessive redirects, either to authentication or some other page outside of the scan scope."]
        TooManyRedirects,
    }
    impl ScanRunErrorTraceCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunErrorTraceCode::AuthenticationConfigIssue => "AUTHENTICATION_CONFIG_ISSUE",
                ScanRunErrorTraceCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ScanRunErrorTraceCode::InternalError => "INTERNAL_ERROR",
                ScanRunErrorTraceCode::ScanConfigIssue => "SCAN_CONFIG_ISSUE",
                ScanRunErrorTraceCode::TimedOutWhileScanning => "TIMED_OUT_WHILE_SCANNING",
                ScanRunErrorTraceCode::TooManyHttpErrors => "TOO_MANY_HTTP_ERRORS",
                ScanRunErrorTraceCode::TooManyRedirects => "TOO_MANY_REDIRECTS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanRunErrorTraceCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanRunErrorTraceCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanRunErrorTraceCode, ()> {
            Ok(match s {
                "AUTHENTICATION_CONFIG_ISSUE" => ScanRunErrorTraceCode::AuthenticationConfigIssue,
                "CODE_UNSPECIFIED" => ScanRunErrorTraceCode::CodeUnspecified,
                "INTERNAL_ERROR" => ScanRunErrorTraceCode::InternalError,
                "SCAN_CONFIG_ISSUE" => ScanRunErrorTraceCode::ScanConfigIssue,
                "TIMED_OUT_WHILE_SCANNING" => ScanRunErrorTraceCode::TimedOutWhileScanning,
                "TOO_MANY_HTTP_ERRORS" => ScanRunErrorTraceCode::TooManyHttpErrors,
                "TOO_MANY_REDIRECTS" => ScanRunErrorTraceCode::TooManyRedirects,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanRunErrorTraceCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunErrorTraceCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunErrorTraceCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_CONFIG_ISSUE" => ScanRunErrorTraceCode::AuthenticationConfigIssue,
                "CODE_UNSPECIFIED" => ScanRunErrorTraceCode::CodeUnspecified,
                "INTERNAL_ERROR" => ScanRunErrorTraceCode::InternalError,
                "SCAN_CONFIG_ISSUE" => ScanRunErrorTraceCode::ScanConfigIssue,
                "TIMED_OUT_WHILE_SCANNING" => ScanRunErrorTraceCode::TimedOutWhileScanning,
                "TOO_MANY_HTTP_ERRORS" => ScanRunErrorTraceCode::TooManyHttpErrors,
                "TOO_MANY_REDIRECTS" => ScanRunErrorTraceCode::TooManyRedirects,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanRunErrorTraceCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRunErrorTraceCode {
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
    pub struct ScanRunWarningTrace {
        #[doc = "Indicates the warning code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ScanRunWarningTraceCode>,
    }
    impl ::google_field_selector::FieldSelector for ScanRunWarningTrace {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRunWarningTrace {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunWarningTraceCode {
        #[doc = "Indicates that a scan is blocked by IAP."]
        BlockedByIap,
        #[doc = "Default value is never used."]
        CodeUnspecified,
        #[doc = "Indicates that a scan discovered an unexpectedly low number of URLs. This is sometimes caused by complex navigation features or by using a single URL for numerous pages."]
        InsufficientCrawlResults,
        #[doc = "Indicates that no seed is found for a scan"]
        NoStartingUrlFoundForManagedScan,
        #[doc = "Indicates that a scan discovered too many URLs to test, or excessive redundant URLs."]
        TooManyCrawlResults,
        #[doc = "Indicates that too many tests have been generated for the scan. Customer should try reducing the number of starting URLs, increasing the QPS rate, or narrowing down the scope of the scan using the excluded patterns."]
        TooManyFuzzTasks,
    }
    impl ScanRunWarningTraceCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunWarningTraceCode::BlockedByIap => "BLOCKED_BY_IAP",
                ScanRunWarningTraceCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ScanRunWarningTraceCode::InsufficientCrawlResults => "INSUFFICIENT_CRAWL_RESULTS",
                ScanRunWarningTraceCode::NoStartingUrlFoundForManagedScan => {
                    "NO_STARTING_URL_FOUND_FOR_MANAGED_SCAN"
                }
                ScanRunWarningTraceCode::TooManyCrawlResults => "TOO_MANY_CRAWL_RESULTS",
                ScanRunWarningTraceCode::TooManyFuzzTasks => "TOO_MANY_FUZZ_TASKS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScanRunWarningTraceCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScanRunWarningTraceCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScanRunWarningTraceCode, ()> {
            Ok(match s {
                "BLOCKED_BY_IAP" => ScanRunWarningTraceCode::BlockedByIap,
                "CODE_UNSPECIFIED" => ScanRunWarningTraceCode::CodeUnspecified,
                "INSUFFICIENT_CRAWL_RESULTS" => ScanRunWarningTraceCode::InsufficientCrawlResults,
                "NO_STARTING_URL_FOUND_FOR_MANAGED_SCAN" => {
                    ScanRunWarningTraceCode::NoStartingUrlFoundForManagedScan
                }
                "TOO_MANY_CRAWL_RESULTS" => ScanRunWarningTraceCode::TooManyCrawlResults,
                "TOO_MANY_FUZZ_TASKS" => ScanRunWarningTraceCode::TooManyFuzzTasks,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScanRunWarningTraceCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunWarningTraceCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunWarningTraceCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLOCKED_BY_IAP" => ScanRunWarningTraceCode::BlockedByIap,
                "CODE_UNSPECIFIED" => ScanRunWarningTraceCode::CodeUnspecified,
                "INSUFFICIENT_CRAWL_RESULTS" => ScanRunWarningTraceCode::InsufficientCrawlResults,
                "NO_STARTING_URL_FOUND_FOR_MANAGED_SCAN" => {
                    ScanRunWarningTraceCode::NoStartingUrlFoundForManagedScan
                }
                "TOO_MANY_CRAWL_RESULTS" => ScanRunWarningTraceCode::TooManyCrawlResults,
                "TOO_MANY_FUZZ_TASKS" => ScanRunWarningTraceCode::TooManyFuzzTasks,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScanRunWarningTraceCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanRunWarningTraceCode {
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
    pub struct Schedule {
        #[doc = "Required. The duration of time between executions in days."]
        #[serde(
            rename = "intervalDurationDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interval_duration_days: ::std::option::Option<i32>,
        #[doc = "A timestamp indicates when the next run will be scheduled. The value is refreshed by the server after each run. If unspecified, it will default to current server time, which means the scan will be scheduled to start immediately."]
        #[serde(
            rename = "scheduleTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schedule_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Schedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Schedule {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StartScanRunRequest {}
    impl ::google_field_selector::FieldSelector for StartScanRunRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StartScanRunRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StopScanRunRequest {}
    impl ::google_field_selector::FieldSelector for StopScanRunRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StopScanRunRequest {
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
    pub struct ViolatingResource {
        #[doc = "The MIME type of this resource."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "URL of this violating resource."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ViolatingResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ViolatingResource {
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
    pub struct VulnerableHeaders {
        #[doc = "List of vulnerable headers."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<Vec<crate::schemas::Header>>,
        #[doc = "List of missing headers."]
        #[serde(
            rename = "missingHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub missing_headers: ::std::option::Option<Vec<crate::schemas::Header>>,
    }
    impl ::google_field_selector::FieldSelector for VulnerableHeaders {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerableHeaders {
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
    pub struct VulnerableParameters {
        #[doc = "The vulnerable parameter names."]
        #[serde(
            rename = "parameterNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for VulnerableParameters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerableParameters {
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
    pub struct Xss {
        #[doc = "An error message generated by a javascript breakage."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Stack traces leading to the point where the XSS occurred."]
        #[serde(
            rename = "stackTraces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_traces: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Xss {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xss {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the scan_configs resource"]
            pub fn scan_configs(
                &self,
            ) -> crate::resources::projects::scan_configs::ScanConfigsActions {
                crate::resources::projects::scan_configs::ScanConfigsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod scan_configs {
            pub mod params {}
            pub struct ScanConfigsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ScanConfigsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new ScanConfig."]
                pub fn create(
                    &self,
                    request: crate::schemas::ScanConfig,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes an existing ScanConfig and its child resources."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets a ScanConfig."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists ScanConfigs under a given project."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a ScanConfig. This method support partial update of a ScanConfig."]
                pub fn patch(
                    &self,
                    request: crate::schemas::ScanConfig,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Start a ScanRun according to the given ScanConfig."]
                pub fn start(
                    &self,
                    request: crate::schemas::StartScanRunRequest,
                    name: impl Into<String>,
                ) -> StartRequestBuilder {
                    StartRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Actions that can be performed on the scan_runs resource"]
                pub fn scan_runs(
                    &self,
                ) -> crate::resources::projects::scan_configs::scan_runs::ScanRunsActions
                {
                    crate::resources::projects::scan_configs::scan_runs::ScanRunsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [ScanConfigsActions::create()](struct.ScanConfigsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ScanConfig,
                parent: String,
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
            impl<'a> CreateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/scanConfigs");
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
            #[doc = "Created via [ScanConfigsActions::delete()](struct.ScanConfigsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
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
            #[doc = "Created via [ScanConfigsActions::get()](struct.ScanConfigsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
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
            #[doc = "Created via [ScanConfigsActions::list()](struct.ScanConfigsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
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
                #[doc = "The maximum number of ScanConfigs to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
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
                ) -> Result<crate::schemas::ListScanConfigsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListScanConfigsResponse, crate::Error> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/scanConfigs");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
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
            #[doc = "Created via [ScanConfigsActions::patch()](struct.ScanConfigsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ScanConfig,
                name: String,
                update_mask: Option<String>,
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
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "Required. The update mask applies to the resource. For the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
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
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    req = req.query(&[("updateMask", &self.update_mask)]);
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
            #[doc = "Created via [ScanConfigsActions::start()](struct.ScanConfigsActions.html#method.start)"]
            #[derive(Debug, Clone)]
            pub struct StartRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::StartScanRunRequest,
                name: String,
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
            impl<'a> StartRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ScanRun, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ScanRun, crate::Error> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":start");
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
            pub mod scan_runs {
                pub mod params {}
                pub struct ScanRunsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ScanRunsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets a ScanRun."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists ScanRuns under a given ScanConfig, in descending order of ScanRun stop time."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                            parent: parent.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Stops a ScanRun. The stopped ScanRun is returned."]
                    pub fn stop(
                        &self,
                        request: crate::schemas::StopScanRunRequest,
                        name: impl Into<String>,
                    ) -> StopRequestBuilder {
                        StopRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the crawled_urls resource"]                    pub fn crawled_urls (& self) -> crate :: resources :: projects :: scan_configs :: scan_runs :: crawled_urls :: CrawledUrlsActions{
                        crate :: resources :: projects :: scan_configs :: scan_runs :: crawled_urls :: CrawledUrlsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the finding_type_stats resource"]                    pub fn finding_type_stats (& self) -> crate :: resources :: projects :: scan_configs :: scan_runs :: finding_type_stats :: FindingTypeStatsActions{
                        crate :: resources :: projects :: scan_configs :: scan_runs :: finding_type_stats :: FindingTypeStatsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the findings resource"]                    pub fn findings (& self) -> crate :: resources :: projects :: scan_configs :: scan_runs :: findings :: FindingsActions{
                        crate :: resources :: projects :: scan_configs :: scan_runs :: findings :: FindingsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [ScanRunsActions::get()](struct.ScanRunsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
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
                    ) -> Result<crate::schemas::ScanRun, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ScanRun, crate::Error> {
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
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
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
                #[doc = "Created via [ScanRunsActions::list()](struct.ScanRunsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
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
                    #[doc = "The maximum number of ScanRuns to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
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
                    ) -> Result<crate::schemas::ListScanRunsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListScanRunsResponse, crate::Error>
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/scanRuns");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
                #[doc = "Created via [ScanRunsActions::stop()](struct.ScanRunsActions.html#method.stop)"]
                #[derive(Debug, Clone)]
                pub struct StopRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::StopScanRunRequest,
                    name: String,
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
                impl<'a> StopRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::ScanRun, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ScanRun, crate::Error> {
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
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":stop");
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
                pub mod crawled_urls {
                    pub mod params {}
                    pub struct CrawledUrlsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> CrawledUrlsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "List CrawledUrls under a given ScanRun."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                                parent: parent.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [CrawledUrlsActions::list()](struct.CrawledUrlsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        page_size: Option<i32>,
                        page_token: Option<String>,
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
                        #[doc = "The maximum number of CrawledUrls to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        ) -> Result<crate::schemas::ListCrawledUrlsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListCrawledUrlsResponse, crate::Error>
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://websecurityscanner.googleapis.com/".to_owned();
                            output.push_str("v1beta/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/crawledUrls");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
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
                pub mod finding_type_stats {
                    pub mod params {}
                    pub struct FindingTypeStatsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> FindingTypeStatsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "List all FindingTypeStats under a given ScanRun."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                                parent: parent.into(),
                            }
                        }
                    }
                    #[doc = "Created via [FindingTypeStatsActions::list()](struct.FindingTypeStatsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        ) -> Result<crate::schemas::ListFindingTypeStatsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListFindingTypeStatsResponse, crate::Error>
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://websecurityscanner.googleapis.com/".to_owned();
                            output.push_str("v1beta/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/findingTypeStats");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                pub mod findings {
                    pub mod params {}
                    pub struct FindingsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> FindingsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Gets a Finding."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "List Findings under a given ScanRun."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                                parent: parent.into(),
                                filter: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [FindingsActions::get()](struct.FindingsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        ) -> Result<crate::schemas::Finding, crate::Error> {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Finding, crate::Error> {
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
                            let mut output =
                                "https://websecurityscanner.googleapis.com/".to_owned();
                            output.push_str("v1beta/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [FindingsActions::list()](struct.FindingsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        filter: Option<String>,
                        page_size: Option<i32>,
                        page_token: Option<String>,
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
                        #[doc = "Required. The filter expression. The expression must be in the format: . Supported field: 'finding_type'. Supported operator: '='."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "The maximum number of Findings to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        ) -> Result<crate::schemas::ListFindingsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListFindingsResponse, crate::Error>
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://websecurityscanner.googleapis.com/".to_owned();
                            output.push_str("v1beta/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/findings");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
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
