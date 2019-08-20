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
        #[serde(rename = "customAccount", default)]
        pub custom_account: Option<crate::schemas::CustomAccount>,
        #[doc = "Authentication using a Google account."]
        #[serde(rename = "googleAccount", default)]
        pub google_account: Option<crate::schemas::GoogleAccount>,
    }
    impl ::field_selector::FieldSelector for Authentication {
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
    pub struct CrawledUrl {
        #[doc = "Output only. The body of the request that was used to visit the URL."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "Output only. The http method of the request that was used to visit the URL, in\nuppercase."]
        #[serde(rename = "httpMethod", default)]
        pub http_method: Option<String>,
        #[doc = "Output only. The URL that was crawled."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for CrawledUrl {
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
    pub struct CustomAccount {
        #[doc = "Required. The login form URL of the website."]
        #[serde(rename = "loginUrl", default)]
        pub login_url: Option<String>,
        #[doc = "Required. Input only. The password of the custom account. The credential is stored encrypted\nand not returned in any response nor included in audit logs."]
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[doc = "Required. The user name of the custom account."]
        #[serde(rename = "username", default)]
        pub username: Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomAccount {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty;
    impl ::field_selector::FieldSelector for Empty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
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
        #[doc = "Output only. The body of the request that triggered the vulnerability."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "Output only. The description of the vulnerability."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Output only. The URL where the browser lands when the vulnerability is detected."]
        #[serde(rename = "finalUrl", default)]
        pub final_url: Option<String>,
        #[doc = "Output only. The type of the Finding.\nDetailed and up-to-date information on findings can be found here:\nhttps://cloud.google.com/security-scanner/docs/scan-result-details"]
        #[serde(rename = "findingType", default)]
        pub finding_type: Option<String>,
        #[doc = "Output only. An addon containing information reported for a vulnerability with an HTML\nform, if any."]
        #[serde(rename = "form", default)]
        pub form: Option<crate::schemas::Form>,
        #[doc = "Output only. If the vulnerability was originated from nested IFrame, the immediate\nparent IFrame is reported."]
        #[serde(rename = "frameUrl", default)]
        pub frame_url: Option<String>,
        #[doc = "Output only. The URL produced by the server-side fuzzer and used in the request that\ntriggered the vulnerability."]
        #[serde(rename = "fuzzedUrl", default)]
        pub fuzzed_url: Option<String>,
        #[doc = "Output only. The http method of the request that triggered the vulnerability, in\nuppercase."]
        #[serde(rename = "httpMethod", default)]
        pub http_method: Option<String>,
        #[doc = "Output only. The resource name of the Finding. The name follows the format of\n'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'.\nThe finding IDs are generated by the system."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Output only. An addon containing information about outdated libraries."]
        #[serde(rename = "outdatedLibrary", default)]
        pub outdated_library: Option<crate::schemas::OutdatedLibrary>,
        #[doc = "Output only. The URL containing human-readable payload that user can leverage to\nreproduce the vulnerability."]
        #[serde(rename = "reproductionUrl", default)]
        pub reproduction_url: Option<String>,
        #[doc = "Output only. The tracking ID uniquely identifies a vulnerability instance across\nmultiple ScanRuns."]
        #[serde(rename = "trackingId", default)]
        pub tracking_id: Option<String>,
        #[doc = "Output only. An addon containing detailed information regarding any resource causing the\nvulnerability such as JavaScript sources, image, audio files, etc."]
        #[serde(rename = "violatingResource", default)]
        pub violating_resource: Option<crate::schemas::ViolatingResource>,
        #[doc = "Output only. An addon containing information about vulnerable or missing HTTP headers."]
        #[serde(rename = "vulnerableHeaders", default)]
        pub vulnerable_headers: Option<crate::schemas::VulnerableHeaders>,
        #[doc = "Output only. An addon containing information about request parameters which were found\nto be vulnerable."]
        #[serde(rename = "vulnerableParameters", default)]
        pub vulnerable_parameters: Option<crate::schemas::VulnerableParameters>,
        #[doc = "Output only. An addon containing information reported for an XSS, if any."]
        #[serde(rename = "xss", default)]
        pub xss: Option<crate::schemas::Xss>,
    }
    impl ::field_selector::FieldSelector for Finding {
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
    pub struct FindingTypeStats {
        #[doc = "Output only. The count of findings belonging to this finding type."]
        #[serde(rename = "findingCount", default)]
        pub finding_count: Option<i32>,
        #[doc = "Output only. The finding type associated with the stats."]
        #[serde(rename = "findingType", default)]
        pub finding_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for FindingTypeStats {
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
    pub struct Form {
        #[doc = "! The URI where to send the form when it's submitted."]
        #[serde(rename = "actionUri", default)]
        pub action_uri: Option<String>,
        #[doc = "! The names of form fields related to the vulnerability."]
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Form {
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
    pub struct GoogleAccount {
        #[doc = "Required. Input only. The password of the Google account. The credential is stored encrypted\nand not returned in any response nor included in audit logs."]
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[doc = "Required. The user name of the Google account."]
        #[serde(rename = "username", default)]
        pub username: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleAccount {
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
    pub struct Header {
        #[doc = "Header name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Header value."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for Header {
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
    pub struct ListCrawledUrlsResponse {
        #[doc = "The list of CrawledUrls returned."]
        #[serde(rename = "crawledUrls", default)]
        pub crawled_urls: Option<Vec<crate::schemas::CrawledUrl>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListCrawledUrlsResponse {
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
    pub struct ListFindingTypeStatsResponse {
        #[doc = "The list of FindingTypeStats returned."]
        #[serde(rename = "findingTypeStats", default)]
        pub finding_type_stats: Option<Vec<crate::schemas::FindingTypeStats>>,
    }
    impl ::field_selector::FieldSelector for ListFindingTypeStatsResponse {
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
    pub struct ListFindingsResponse {
        #[doc = "The list of Findings returned."]
        #[serde(rename = "findings", default)]
        pub findings: Option<Vec<crate::schemas::Finding>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListFindingsResponse {
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
    pub struct ListScanConfigsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of ScanConfigs returned."]
        #[serde(rename = "scanConfigs", default)]
        pub scan_configs: Option<Vec<crate::schemas::ScanConfig>>,
    }
    impl ::field_selector::FieldSelector for ListScanConfigsResponse {
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
    pub struct ListScanRunsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of ScanRuns returned."]
        #[serde(rename = "scanRuns", default)]
        pub scan_runs: Option<Vec<crate::schemas::ScanRun>>,
    }
    impl ::field_selector::FieldSelector for ListScanRunsResponse {
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
    pub struct OutdatedLibrary {
        #[doc = "URLs to learn more information about the vulnerabilities in the library."]
        #[serde(rename = "learnMoreUrls", default)]
        pub learn_more_urls: Option<Vec<String>>,
        #[doc = "The name of the outdated library."]
        #[serde(rename = "libraryName", default)]
        pub library_name: Option<String>,
        #[doc = "The version number."]
        #[serde(rename = "version", default)]
        pub version: Option<String>,
    }
    impl ::field_selector::FieldSelector for OutdatedLibrary {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigExportToSecurityCommandCenter {
        #[doc = "Use default, which is ENABLED."]
        ExportToSecurityCommandCenterUnspecified,
        #[doc = "Export results of this scan to Cloud Security Command Center."]
        Enabled,
        #[doc = "Do not export results of this scan to Cloud Security Command Center."]
        Disabled,
    }
    impl ScanConfigExportToSecurityCommandCenter {
        pub fn as_str(self) -> &'static str {
            match self { ScanConfigExportToSecurityCommandCenter :: ExportToSecurityCommandCenterUnspecified => "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED" , ScanConfigExportToSecurityCommandCenter :: Enabled => "ENABLED" , ScanConfigExportToSecurityCommandCenter :: Disabled => "DISABLED" , }
        }
    }
    impl ::std::fmt::Display for ScanConfigExportToSecurityCommandCenter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigExportToSecurityCommandCenter {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigExportToSecurityCommandCenter {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED" => ScanConfigExportToSecurityCommandCenter :: ExportToSecurityCommandCenterUnspecified , "ENABLED" => ScanConfigExportToSecurityCommandCenter :: Enabled , "DISABLED" => ScanConfigExportToSecurityCommandCenter :: Disabled , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigRiskLevel {
        #[doc = "Use default, which is NORMAL."]
        RiskLevelUnspecified,
        #[doc = "Normal scanning (Recommended)"]
        Normal,
        #[doc = "Lower impact scanning"]
        Low,
    }
    impl ScanConfigRiskLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigRiskLevel::RiskLevelUnspecified => "RISK_LEVEL_UNSPECIFIED",
                ScanConfigRiskLevel::Normal => "NORMAL",
                ScanConfigRiskLevel::Low => "LOW",
            }
        }
    }
    impl ::std::fmt::Display for ScanConfigRiskLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigRiskLevel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigRiskLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RISK_LEVEL_UNSPECIFIED" => ScanConfigRiskLevel::RiskLevelUnspecified,
                "NORMAL" => ScanConfigRiskLevel::Normal,
                "LOW" => ScanConfigRiskLevel::Low,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigUserAgent {
        #[doc = "The user agent is unknown. Service will default to CHROME_LINUX."]
        UserAgentUnspecified,
        #[doc = "Chrome on Linux. This is the service default if unspecified."]
        ChromeLinux,
        #[doc = "Chrome on Android."]
        ChromeAndroid,
        #[doc = "Safari on IPhone."]
        SafariIphone,
    }
    impl ScanConfigUserAgent {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigUserAgent::UserAgentUnspecified => "USER_AGENT_UNSPECIFIED",
                ScanConfigUserAgent::ChromeLinux => "CHROME_LINUX",
                ScanConfigUserAgent::ChromeAndroid => "CHROME_ANDROID",
                ScanConfigUserAgent::SafariIphone => "SAFARI_IPHONE",
            }
        }
    }
    impl ::std::fmt::Display for ScanConfigUserAgent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigUserAgent {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigUserAgent {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_AGENT_UNSPECIFIED" => ScanConfigUserAgent::UserAgentUnspecified,
                "CHROME_LINUX" => ScanConfigUserAgent::ChromeLinux,
                "CHROME_ANDROID" => ScanConfigUserAgent::ChromeAndroid,
                "SAFARI_IPHONE" => ScanConfigUserAgent::SafariIphone,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
        #[doc = "The authentication configuration. If specified, service will use the\nauthentication configuration during scanning."]
        #[serde(rename = "authentication", default)]
        pub authentication: Option<crate::schemas::Authentication>,
        #[doc = "The blacklist URL patterns as described in\nhttps://cloud.google.com/security-scanner/docs/excluded-urls"]
        #[serde(rename = "blacklistPatterns", default)]
        pub blacklist_patterns: Option<Vec<String>>,
        #[doc = "Required. The user provided display name of the ScanConfig."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "Controls export of scan configurations and results to Cloud Security\nCommand Center."]
        #[serde(rename = "exportToSecurityCommandCenter", default)]
        pub export_to_security_command_center:
            Option<crate::schemas::ScanConfigExportToSecurityCommandCenter>,
        #[doc = "The maximum QPS during scanning. A valid value ranges from 5 to 20\ninclusively. If the field is unspecified or its value is set 0, server will\ndefault to 15. Other values outside of [5, 20] range will be rejected with\nINVALID_ARGUMENT error."]
        #[serde(rename = "maxQps", default)]
        pub max_qps: Option<i32>,
        #[doc = "The resource name of the ScanConfig. The name follows the format of\n'projects/{projectId}/scanConfigs/{scanConfigId}'. The ScanConfig IDs are\ngenerated by the system."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The risk level selected for the scan"]
        #[serde(rename = "riskLevel", default)]
        pub risk_level: Option<crate::schemas::ScanConfigRiskLevel>,
        #[doc = "The schedule of the ScanConfig."]
        #[serde(rename = "schedule", default)]
        pub schedule: Option<crate::schemas::Schedule>,
        #[doc = "Required. The starting URLs from which the scanner finds site pages."]
        #[serde(rename = "startingUrls", default)]
        pub starting_urls: Option<Vec<String>>,
        #[doc = "The user agent used during scanning."]
        #[serde(rename = "userAgent", default)]
        pub user_agent: Option<crate::schemas::ScanConfigUserAgent>,
    }
    impl ::field_selector::FieldSelector for ScanConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanConfigErrorCode {
        #[doc = "There is no error."]
        CodeUnspecified,
        #[doc = "There is no error."]
        Ok,
        #[doc = "Indicates an internal server error.\nPlease DO NOT USE THIS ERROR CODE unless the root cause is truly unknown."]
        InternalError,
        #[doc = "One of the seed URLs is an App Engine URL but we cannot validate the scan\nsettings due to an App Engine API backend error."]
        AppengineApiBackendError,
        #[doc = "One of the seed URLs is an App Engine URL but we cannot access the\nApp Engine API to validate scan settings."]
        AppengineApiNotAccessible,
        #[doc = "One of the seed URLs is an App Engine URL but the Default Host of the\nApp Engine is not set."]
        AppengineDefaultHostMissing,
        #[doc = "Google corporate accounts can not be used for scanning."]
        CannotUseGoogleComAccount,
        #[doc = "The account of the scan creator can not be used for scanning."]
        CannotUseOwnerAccount,
        #[doc = "This scan targets Compute Engine, but we cannot validate scan settings\ndue to a Compute Engine API backend error."]
        ComputeApiBackendError,
        #[doc = "This scan targets Compute Engine, but we cannot access the Compute Engine\nAPI to validate the scan settings."]
        ComputeApiNotAccessible,
        #[doc = "The Custom Login URL does not belong to the current project."]
        CustomLoginUrlDoesNotBelongToCurrentProject,
        #[doc = "The Custom Login URL is malformed (can not be parsed)."]
        CustomLoginUrlMalformed,
        #[doc = "The Custom Login URL is mapped to a non-routable IP address in DNS."]
        CustomLoginUrlMappedToNonRoutableAddress,
        #[doc = "The Custom Login URL is mapped to an IP address which is not reserved for\nthe current project."]
        CustomLoginUrlMappedToUnreservedAddress,
        #[doc = "The Custom Login URL has a non-routable IP address."]
        CustomLoginUrlHasNonRoutableIpAddress,
        #[doc = "The Custom Login URL has an IP address which is not reserved for the\ncurrent project."]
        CustomLoginUrlHasUnreservedIpAddress,
        #[doc = "Another scan with the same name (case-sensitive) already exists."]
        DuplicateScanName,
        #[doc = "A field is set to an invalid value."]
        InvalidFieldValue,
        #[doc = "There was an error trying to authenticate to the scan target."]
        FailedToAuthenticateToTarget,
        #[doc = "Finding type value is not specified in the list findings request."]
        FindingTypeUnspecified,
        #[doc = "Scan targets Compute Engine, yet current project was not whitelisted for\nGoogle Compute Engine Scanning Alpha access."]
        ForbiddenToScanCompute,
        #[doc = "The supplied filter is malformed. For example, it can not be parsed, does\nnot have a filter type in expression, or the same filter type appears\nmore than once."]
        MalformedFilter,
        #[doc = "The supplied resource name is malformed (can not be parsed)."]
        MalformedResourceName,
        #[doc = "The current project is not in an active state."]
        ProjectInactive,
        #[doc = "A required field is not set."]
        RequiredField,
        #[doc = "Project id, scanconfig id, scanrun id, or finding id are not consistent\nwith each other in resource name."]
        ResourceNameInconsistent,
        #[doc = "The scan being requested to start is already running."]
        ScanAlreadyRunning,
        #[doc = "The scan that was requested to be stopped is not running."]
        ScanNotRunning,
        #[doc = "One of the seed URLs does not belong to the current project."]
        SeedUrlDoesNotBelongToCurrentProject,
        #[doc = "One of the seed URLs is malformed (can not be parsed)."]
        SeedUrlMalformed,
        #[doc = "One of the seed URLs is mapped to a non-routable IP address in DNS."]
        SeedUrlMappedToNonRoutableAddress,
        #[doc = "One of the seed URLs is mapped to an IP address which is not reserved\nfor the current project."]
        SeedUrlMappedToUnreservedAddress,
        #[doc = "One of the seed URLs has on-routable IP address."]
        SeedUrlHasNonRoutableIpAddress,
        #[doc = "One of the seed URLs has an IP address that is not reserved\nfor the current project."]
        SeedUrlHasUnreservedIpAddress,
        #[doc = "The Cloud Security Scanner service account is not configured under the\nproject."]
        ServiceAccountNotConfigured,
        #[doc = "A project has reached the maximum number of scans."]
        TooManyScans,
        #[doc = "Resolving the details of the current project fails."]
        UnableToResolveProjectInfo,
        #[doc = "One or more blacklist patterns were in the wrong format."]
        UnsupportedBlacklistPatternFormat,
        #[doc = "The supplied filter is not supported."]
        UnsupportedFilter,
        #[doc = "The supplied finding type is not supported. For example, we do not\nprovide findings of the given finding type."]
        UnsupportedFindingType,
        #[doc = "The URL scheme of one or more of the supplied URLs is not supported."]
        UnsupportedUrlScheme,
    }
    impl ScanConfigErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanConfigErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ScanConfigErrorCode::Ok => "OK",
                ScanConfigErrorCode::InternalError => "INTERNAL_ERROR",
                ScanConfigErrorCode::AppengineApiBackendError => "APPENGINE_API_BACKEND_ERROR",
                ScanConfigErrorCode::AppengineApiNotAccessible => "APPENGINE_API_NOT_ACCESSIBLE",
                ScanConfigErrorCode::AppengineDefaultHostMissing => {
                    "APPENGINE_DEFAULT_HOST_MISSING"
                }
                ScanConfigErrorCode::CannotUseGoogleComAccount => "CANNOT_USE_GOOGLE_COM_ACCOUNT",
                ScanConfigErrorCode::CannotUseOwnerAccount => "CANNOT_USE_OWNER_ACCOUNT",
                ScanConfigErrorCode::ComputeApiBackendError => "COMPUTE_API_BACKEND_ERROR",
                ScanConfigErrorCode::ComputeApiNotAccessible => "COMPUTE_API_NOT_ACCESSIBLE",
                ScanConfigErrorCode::CustomLoginUrlDoesNotBelongToCurrentProject => {
                    "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT"
                }
                ScanConfigErrorCode::CustomLoginUrlMalformed => "CUSTOM_LOGIN_URL_MALFORMED",
                ScanConfigErrorCode::CustomLoginUrlMappedToNonRoutableAddress => {
                    "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS"
                }
                ScanConfigErrorCode::CustomLoginUrlMappedToUnreservedAddress => {
                    "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS"
                }
                ScanConfigErrorCode::CustomLoginUrlHasNonRoutableIpAddress => {
                    "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS"
                }
                ScanConfigErrorCode::CustomLoginUrlHasUnreservedIpAddress => {
                    "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS"
                }
                ScanConfigErrorCode::DuplicateScanName => "DUPLICATE_SCAN_NAME",
                ScanConfigErrorCode::InvalidFieldValue => "INVALID_FIELD_VALUE",
                ScanConfigErrorCode::FailedToAuthenticateToTarget => {
                    "FAILED_TO_AUTHENTICATE_TO_TARGET"
                }
                ScanConfigErrorCode::FindingTypeUnspecified => "FINDING_TYPE_UNSPECIFIED",
                ScanConfigErrorCode::ForbiddenToScanCompute => "FORBIDDEN_TO_SCAN_COMPUTE",
                ScanConfigErrorCode::MalformedFilter => "MALFORMED_FILTER",
                ScanConfigErrorCode::MalformedResourceName => "MALFORMED_RESOURCE_NAME",
                ScanConfigErrorCode::ProjectInactive => "PROJECT_INACTIVE",
                ScanConfigErrorCode::RequiredField => "REQUIRED_FIELD",
                ScanConfigErrorCode::ResourceNameInconsistent => "RESOURCE_NAME_INCONSISTENT",
                ScanConfigErrorCode::ScanAlreadyRunning => "SCAN_ALREADY_RUNNING",
                ScanConfigErrorCode::ScanNotRunning => "SCAN_NOT_RUNNING",
                ScanConfigErrorCode::SeedUrlDoesNotBelongToCurrentProject => {
                    "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT"
                }
                ScanConfigErrorCode::SeedUrlMalformed => "SEED_URL_MALFORMED",
                ScanConfigErrorCode::SeedUrlMappedToNonRoutableAddress => {
                    "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS"
                }
                ScanConfigErrorCode::SeedUrlMappedToUnreservedAddress => {
                    "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS"
                }
                ScanConfigErrorCode::SeedUrlHasNonRoutableIpAddress => {
                    "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS"
                }
                ScanConfigErrorCode::SeedUrlHasUnreservedIpAddress => {
                    "SEED_URL_HAS_UNRESERVED_IP_ADDRESS"
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
    impl ::std::fmt::Display for ScanConfigErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanConfigErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanConfigErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ScanConfigErrorCode::CodeUnspecified,
                "OK" => ScanConfigErrorCode::Ok,
                "INTERNAL_ERROR" => ScanConfigErrorCode::InternalError,
                "APPENGINE_API_BACKEND_ERROR" => ScanConfigErrorCode::AppengineApiBackendError,
                "APPENGINE_API_NOT_ACCESSIBLE" => ScanConfigErrorCode::AppengineApiNotAccessible,
                "APPENGINE_DEFAULT_HOST_MISSING" => {
                    ScanConfigErrorCode::AppengineDefaultHostMissing
                }
                "CANNOT_USE_GOOGLE_COM_ACCOUNT" => ScanConfigErrorCode::CannotUseGoogleComAccount,
                "CANNOT_USE_OWNER_ACCOUNT" => ScanConfigErrorCode::CannotUseOwnerAccount,
                "COMPUTE_API_BACKEND_ERROR" => ScanConfigErrorCode::ComputeApiBackendError,
                "COMPUTE_API_NOT_ACCESSIBLE" => ScanConfigErrorCode::ComputeApiNotAccessible,
                "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT" => {
                    ScanConfigErrorCode::CustomLoginUrlDoesNotBelongToCurrentProject
                }
                "CUSTOM_LOGIN_URL_MALFORMED" => ScanConfigErrorCode::CustomLoginUrlMalformed,
                "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlMappedToNonRoutableAddress
                }
                "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlMappedToUnreservedAddress
                }
                "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlHasNonRoutableIpAddress
                }
                "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS" => {
                    ScanConfigErrorCode::CustomLoginUrlHasUnreservedIpAddress
                }
                "DUPLICATE_SCAN_NAME" => ScanConfigErrorCode::DuplicateScanName,
                "INVALID_FIELD_VALUE" => ScanConfigErrorCode::InvalidFieldValue,
                "FAILED_TO_AUTHENTICATE_TO_TARGET" => {
                    ScanConfigErrorCode::FailedToAuthenticateToTarget
                }
                "FINDING_TYPE_UNSPECIFIED" => ScanConfigErrorCode::FindingTypeUnspecified,
                "FORBIDDEN_TO_SCAN_COMPUTE" => ScanConfigErrorCode::ForbiddenToScanCompute,
                "MALFORMED_FILTER" => ScanConfigErrorCode::MalformedFilter,
                "MALFORMED_RESOURCE_NAME" => ScanConfigErrorCode::MalformedResourceName,
                "PROJECT_INACTIVE" => ScanConfigErrorCode::ProjectInactive,
                "REQUIRED_FIELD" => ScanConfigErrorCode::RequiredField,
                "RESOURCE_NAME_INCONSISTENT" => ScanConfigErrorCode::ResourceNameInconsistent,
                "SCAN_ALREADY_RUNNING" => ScanConfigErrorCode::ScanAlreadyRunning,
                "SCAN_NOT_RUNNING" => ScanConfigErrorCode::ScanNotRunning,
                "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT" => {
                    ScanConfigErrorCode::SeedUrlDoesNotBelongToCurrentProject
                }
                "SEED_URL_MALFORMED" => ScanConfigErrorCode::SeedUrlMalformed,
                "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlMappedToNonRoutableAddress
                }
                "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlMappedToUnreservedAddress
                }
                "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlHasNonRoutableIpAddress
                }
                "SEED_URL_HAS_UNRESERVED_IP_ADDRESS" => {
                    ScanConfigErrorCode::SeedUrlHasUnreservedIpAddress
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
        #[doc = "Output only. Indicates the reason code for a configuration failure."]
        #[serde(rename = "code", default)]
        pub code: Option<crate::schemas::ScanConfigErrorCode>,
        #[doc = "Output only. Indicates the full name of the ScanConfig field that triggers this error,\nfor example \"scan_config.max_qps\". This field is provided for\ntroubleshooting purposes only and its actual value can change in the\nfuture."]
        #[serde(rename = "fieldName", default)]
        pub field_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ScanConfigError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunExecutionState {
        #[doc = "Represents an invalid state caused by internal server error. This value\nshould never be returned."]
        ExecutionStateUnspecified,
        #[doc = "The scan is waiting in the queue."]
        Queued,
        #[doc = "The scan is in progress."]
        Scanning,
        #[doc = "The scan is either finished or stopped by user."]
        Finished,
    }
    impl ScanRunExecutionState {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunExecutionState::ExecutionStateUnspecified => "EXECUTION_STATE_UNSPECIFIED",
                ScanRunExecutionState::Queued => "QUEUED",
                ScanRunExecutionState::Scanning => "SCANNING",
                ScanRunExecutionState::Finished => "FINISHED",
            }
        }
    }
    impl ::std::fmt::Display for ScanRunExecutionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunExecutionState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunExecutionState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXECUTION_STATE_UNSPECIFIED" => ScanRunExecutionState::ExecutionStateUnspecified,
                "QUEUED" => ScanRunExecutionState::Queued,
                "SCANNING" => ScanRunExecutionState::Scanning,
                "FINISHED" => ScanRunExecutionState::Finished,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunResultState {
        #[doc = "Default value. This value is returned when the ScanRun is not yet\nfinished."]
        ResultStateUnspecified,
        #[doc = "The scan finished without errors."]
        Success,
        #[doc = "The scan finished with errors."]
        Error,
        #[doc = "The scan was terminated by user."]
        Killed,
    }
    impl ScanRunResultState {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunResultState::ResultStateUnspecified => "RESULT_STATE_UNSPECIFIED",
                ScanRunResultState::Success => "SUCCESS",
                ScanRunResultState::Error => "ERROR",
                ScanRunResultState::Killed => "KILLED",
            }
        }
    }
    impl ::std::fmt::Display for ScanRunResultState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunResultState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunResultState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RESULT_STATE_UNSPECIFIED" => ScanRunResultState::ResultStateUnspecified,
                "SUCCESS" => ScanRunResultState::Success,
                "ERROR" => ScanRunResultState::Error,
                "KILLED" => ScanRunResultState::Killed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
        #[doc = "Output only. The time at which the ScanRun reached termination state - that the ScanRun\nis either finished or stopped by user."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Output only. If result_state is an ERROR, this field provides the primary reason for\nscan's termination and more details, if such are available."]
        #[serde(rename = "errorTrace", default)]
        pub error_trace: Option<crate::schemas::ScanRunErrorTrace>,
        #[doc = "Output only. The execution state of the ScanRun."]
        #[serde(rename = "executionState", default)]
        pub execution_state: Option<crate::schemas::ScanRunExecutionState>,
        #[doc = "Output only. Whether the scan run has found any vulnerabilities."]
        #[serde(rename = "hasVulnerabilities", default)]
        pub has_vulnerabilities: Option<bool>,
        #[doc = "Output only. The resource name of the ScanRun. The name follows the format of\n'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'.\nThe ScanRun IDs are generated by the system."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Output only. The percentage of total completion ranging from 0 to 100.\nIf the scan is in queue, the value is 0.\nIf the scan is running, the value ranges from 0 to 100.\nIf the scan is finished, the value is 100."]
        #[serde(rename = "progressPercent", default)]
        pub progress_percent: Option<i32>,
        #[doc = "Output only. The result state of the ScanRun. This field is only available after the\nexecution state reaches \"FINISHED\"."]
        #[serde(rename = "resultState", default)]
        pub result_state: Option<crate::schemas::ScanRunResultState>,
        #[doc = "Output only. The time at which the ScanRun started."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Output only. The number of URLs crawled during this ScanRun. If the scan is in progress,\nthe value represents the number of URLs crawled up to now."]
        #[serde(rename = "urlsCrawledCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub urls_crawled_count: Option<i64>,
        #[doc = "Output only. The number of URLs tested during this ScanRun. If the scan is in progress,\nthe value represents the number of URLs tested up to now. The number of\nURLs tested is usually larger than the number URLS crawled because\ntypically a crawled URL is tested with multiple test payloads."]
        #[serde(rename = "urlsTestedCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub urls_tested_count: Option<i64>,
        #[doc = "Output only. A list of warnings, if such are encountered during this scan run."]
        #[serde(rename = "warningTraces", default)]
        pub warning_traces: Option<Vec<crate::schemas::ScanRunWarningTrace>>,
    }
    impl ::field_selector::FieldSelector for ScanRun {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunErrorTraceCode {
        #[doc = "Default value is never used."]
        CodeUnspecified,
        #[doc = "Indicates that the scan run failed due to an internal server error."]
        InternalError,
        #[doc = "Indicates a scan configuration error, usually due to outdated ScanConfig\nsettings, such as starting_urls or the DNS configuration."]
        ScanConfigIssue,
        #[doc = "Indicates an authentication error, usually due to outdated ScanConfig\nauthentication settings."]
        AuthenticationConfigIssue,
        #[doc = "Indicates a scan operation timeout, usually caused by a very large site."]
        TimedOutWhileScanning,
        #[doc = "Indicates that a scan encountered excessive redirects, either to\nauthentication or some other page outside of the scan scope."]
        TooManyRedirects,
        #[doc = "Indicates that a scan encountered numerous errors from the web site\npages. When available, most_common_http_error_code field indicates the\nmost common HTTP error code encountered during the scan."]
        TooManyHttpErrors,
    }
    impl ScanRunErrorTraceCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunErrorTraceCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ScanRunErrorTraceCode::InternalError => "INTERNAL_ERROR",
                ScanRunErrorTraceCode::ScanConfigIssue => "SCAN_CONFIG_ISSUE",
                ScanRunErrorTraceCode::AuthenticationConfigIssue => "AUTHENTICATION_CONFIG_ISSUE",
                ScanRunErrorTraceCode::TimedOutWhileScanning => "TIMED_OUT_WHILE_SCANNING",
                ScanRunErrorTraceCode::TooManyRedirects => "TOO_MANY_REDIRECTS",
                ScanRunErrorTraceCode::TooManyHttpErrors => "TOO_MANY_HTTP_ERRORS",
            }
        }
    }
    impl ::std::fmt::Display for ScanRunErrorTraceCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunErrorTraceCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunErrorTraceCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ScanRunErrorTraceCode::CodeUnspecified,
                "INTERNAL_ERROR" => ScanRunErrorTraceCode::InternalError,
                "SCAN_CONFIG_ISSUE" => ScanRunErrorTraceCode::ScanConfigIssue,
                "AUTHENTICATION_CONFIG_ISSUE" => ScanRunErrorTraceCode::AuthenticationConfigIssue,
                "TIMED_OUT_WHILE_SCANNING" => ScanRunErrorTraceCode::TimedOutWhileScanning,
                "TOO_MANY_REDIRECTS" => ScanRunErrorTraceCode::TooManyRedirects,
                "TOO_MANY_HTTP_ERRORS" => ScanRunErrorTraceCode::TooManyHttpErrors,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
        #[doc = "Output only. Indicates the error reason code."]
        #[serde(rename = "code", default)]
        pub code: Option<crate::schemas::ScanRunErrorTraceCode>,
        #[doc = "Output only. If the scan encounters TOO_MANY_HTTP_ERRORS, this field indicates the most\ncommon HTTP error code, if such is available. For example, if this code is\n404, the scan has encountered too many NOT_FOUND responses."]
        #[serde(rename = "mostCommonHttpErrorCode", default)]
        pub most_common_http_error_code: Option<i32>,
        #[doc = "Output only. If the scan encounters SCAN_CONFIG_ISSUE error, this field has the error\nmessage encountered during scan configuration validation that is performed\nbefore each scan run."]
        #[serde(rename = "scanConfigError", default)]
        pub scan_config_error: Option<crate::schemas::ScanConfigError>,
    }
    impl ::field_selector::FieldSelector for ScanRunErrorTrace {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScanRunWarningTraceCode {
        #[doc = "Default value is never used."]
        CodeUnspecified,
        #[doc = "Indicates that a scan discovered an unexpectedly low number of URLs. This\nis sometimes caused by complex navigation features or by using a single\nURL for numerous pages."]
        InsufficientCrawlResults,
        #[doc = "Indicates that a scan discovered too many URLs to test, or excessive\nredundant URLs."]
        TooManyCrawlResults,
        #[doc = "Indicates that too many tests have been generated for the scan. Customer\nshould try reducing the number of starting URLs, increasing the QPS rate,\nor narrowing down the scope of the scan using the excluded patterns."]
        TooManyFuzzTasks,
        #[doc = "Indicates that a scan is blocked by IAP."]
        BlockedByIap,
    }
    impl ScanRunWarningTraceCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ScanRunWarningTraceCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ScanRunWarningTraceCode::InsufficientCrawlResults => "INSUFFICIENT_CRAWL_RESULTS",
                ScanRunWarningTraceCode::TooManyCrawlResults => "TOO_MANY_CRAWL_RESULTS",
                ScanRunWarningTraceCode::TooManyFuzzTasks => "TOO_MANY_FUZZ_TASKS",
                ScanRunWarningTraceCode::BlockedByIap => "BLOCKED_BY_IAP",
            }
        }
    }
    impl ::std::fmt::Display for ScanRunWarningTraceCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScanRunWarningTraceCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScanRunWarningTraceCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ScanRunWarningTraceCode::CodeUnspecified,
                "INSUFFICIENT_CRAWL_RESULTS" => ScanRunWarningTraceCode::InsufficientCrawlResults,
                "TOO_MANY_CRAWL_RESULTS" => ScanRunWarningTraceCode::TooManyCrawlResults,
                "TOO_MANY_FUZZ_TASKS" => ScanRunWarningTraceCode::TooManyFuzzTasks,
                "BLOCKED_BY_IAP" => ScanRunWarningTraceCode::BlockedByIap,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
        #[doc = "Output only. Indicates the warning code."]
        #[serde(rename = "code", default)]
        pub code: Option<crate::schemas::ScanRunWarningTraceCode>,
    }
    impl ::field_selector::FieldSelector for ScanRunWarningTrace {
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
    pub struct Schedule {
        #[doc = "Required. The duration of time between executions in days."]
        #[serde(rename = "intervalDurationDays", default)]
        pub interval_duration_days: Option<i32>,
        #[doc = "A timestamp indicates when the next run will be scheduled. The value is\nrefreshed by the server after each run. If unspecified, it will default\nto current server time, which means the scan will be scheduled to start\nimmediately."]
        #[serde(rename = "scheduleTime", default)]
        pub schedule_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for Schedule {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StartScanRunRequest;
    impl ::field_selector::FieldSelector for StartScanRunRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
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
    pub struct StopScanRunRequest;
    impl ::field_selector::FieldSelector for StopScanRunRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
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
        #[serde(rename = "contentType", default)]
        pub content_type: Option<String>,
        #[doc = "URL of this violating resource."]
        #[serde(rename = "resourceUrl", default)]
        pub resource_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for ViolatingResource {
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
    pub struct VulnerableHeaders {
        #[doc = "List of vulnerable headers."]
        #[serde(rename = "headers", default)]
        pub headers: Option<Vec<crate::schemas::Header>>,
        #[doc = "List of missing headers."]
        #[serde(rename = "missingHeaders", default)]
        pub missing_headers: Option<Vec<crate::schemas::Header>>,
    }
    impl ::field_selector::FieldSelector for VulnerableHeaders {
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
    pub struct VulnerableParameters {
        #[doc = "The vulnerable parameter names."]
        #[serde(rename = "parameterNames", default)]
        pub parameter_names: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for VulnerableParameters {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum XssAttackVector {
        #[doc = "Unknown attack vector."]
        AttackVectorUnspecified,
        #[doc = "The attack comes from fuzzing the browser's localStorage."]
        LocalStorage,
        #[doc = "The attack comes from fuzzing the browser's sessionStorage."]
        SessionStorage,
        #[doc = "The attack comes from fuzzing the window's name property."]
        WindowName,
        #[doc = "The attack comes from fuzzing the referrer property."]
        Referrer,
        #[doc = "The attack comes from fuzzing an input element."]
        FormInput,
        #[doc = "The attack comes from fuzzing the browser's cookies."]
        Cookie,
        #[doc = "The attack comes from hijacking the post messaging mechanism."]
        PostMessage,
        #[doc = "The attack comes from fuzzing parameters in the url."]
        GetParameters,
        #[doc = "The attack comes from fuzzing the fragment in the url."]
        UrlFragment,
        #[doc = "The attack comes from fuzzing the HTML comments."]
        HtmlComment,
        #[doc = "The attack comes from fuzzing the POST parameters."]
        PostParameters,
        #[doc = "The attack comes from fuzzing the protocol."]
        Protocol,
        #[doc = "The attack comes from the server side and is stored."]
        StoredXss,
        #[doc = "The attack is a Same-Origin Method Execution attack via a GET parameter."]
        SameOrigin,
        #[doc = "The attack payload is received from a third-party host via a URL that is\nuser-controllable"]
        UserControllableUrl,
    }
    impl XssAttackVector {
        pub fn as_str(self) -> &'static str {
            match self {
                XssAttackVector::AttackVectorUnspecified => "ATTACK_VECTOR_UNSPECIFIED",
                XssAttackVector::LocalStorage => "LOCAL_STORAGE",
                XssAttackVector::SessionStorage => "SESSION_STORAGE",
                XssAttackVector::WindowName => "WINDOW_NAME",
                XssAttackVector::Referrer => "REFERRER",
                XssAttackVector::FormInput => "FORM_INPUT",
                XssAttackVector::Cookie => "COOKIE",
                XssAttackVector::PostMessage => "POST_MESSAGE",
                XssAttackVector::GetParameters => "GET_PARAMETERS",
                XssAttackVector::UrlFragment => "URL_FRAGMENT",
                XssAttackVector::HtmlComment => "HTML_COMMENT",
                XssAttackVector::PostParameters => "POST_PARAMETERS",
                XssAttackVector::Protocol => "PROTOCOL",
                XssAttackVector::StoredXss => "STORED_XSS",
                XssAttackVector::SameOrigin => "SAME_ORIGIN",
                XssAttackVector::UserControllableUrl => "USER_CONTROLLABLE_URL",
            }
        }
    }
    impl ::std::fmt::Display for XssAttackVector {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for XssAttackVector {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for XssAttackVector {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_VECTOR_UNSPECIFIED" => XssAttackVector::AttackVectorUnspecified,
                "LOCAL_STORAGE" => XssAttackVector::LocalStorage,
                "SESSION_STORAGE" => XssAttackVector::SessionStorage,
                "WINDOW_NAME" => XssAttackVector::WindowName,
                "REFERRER" => XssAttackVector::Referrer,
                "FORM_INPUT" => XssAttackVector::FormInput,
                "COOKIE" => XssAttackVector::Cookie,
                "POST_MESSAGE" => XssAttackVector::PostMessage,
                "GET_PARAMETERS" => XssAttackVector::GetParameters,
                "URL_FRAGMENT" => XssAttackVector::UrlFragment,
                "HTML_COMMENT" => XssAttackVector::HtmlComment,
                "POST_PARAMETERS" => XssAttackVector::PostParameters,
                "PROTOCOL" => XssAttackVector::Protocol,
                "STORED_XSS" => XssAttackVector::StoredXss,
                "SAME_ORIGIN" => XssAttackVector::SameOrigin,
                "USER_CONTROLLABLE_URL" => XssAttackVector::UserControllableUrl,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
        #[doc = "The attack vector of the payload triggering this XSS."]
        #[serde(rename = "attackVector", default)]
        pub attack_vector: Option<crate::schemas::XssAttackVector>,
        #[doc = "An error message generated by a javascript breakage."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[doc = "Stack traces leading to the point where the XSS occurred."]
        #[serde(rename = "stackTraces", default)]
        pub stack_traces: Option<Vec<String>>,
        #[doc = "The reproduction url for the seeding POST request of a Stored XSS."]
        #[serde(rename = "storedXssSeedingUrl", default)]
        pub stored_xss_seeding_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for Xss {
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
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::projects::ProjectsActions<A> {
        crate::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod projects {
    pub mod params {}
    pub struct ProjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
        #[doc = "Actions that can be performed on the scan_configs resource"]
        pub fn scan_configs(&self) -> scan_configs::ScanConfigsActions<A> {
            scan_configs::ScanConfigsActions
        }
    }
    pub mod scan_configs {
        pub mod params {}
        pub struct ScanConfigsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ScanConfigsActions<'a, A> {
            #[doc = "Creates a new ScanConfig."]
            pub fn create(
                &self,
                request: crate::schemas::ScanConfig,
                parent: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
            pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
            pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
            ) -> PatchRequestBuilder<A> {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
            ) -> StartRequestBuilder<A> {
                StartRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
            pub fn scan_runs(&self) -> scan_runs::ScanRunsActions<A> {
                scan_runs::ScanRunsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::ScanConfig, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/scanConfigs");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::ScanConfig, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "The maximum number of ScanConfigs to return, can be limited by server.\nIf not specified or not positive, the implementation will select a\nreasonable value."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results to be returned. This should be a\n`next_page_token` value returned from a previous List request.\nIf unspecified, the first page of results is returned."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_scan_configs<T>(
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
                            #[serde(rename = "scanConfigs")]
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
            ) -> Result<crate::schemas::ListScanConfigsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/scanConfigs");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
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
        impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
            #[doc = "Required. The update mask applies to the resource. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
            pub fn update_mask(&mut self, value: impl Into<String>) -> &mut Self {
                self.update_mask = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::ScanConfig, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                let req = req.query(&[("updateMask", &self.update_mask)]);
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
        pub struct StartRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> StartRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::ScanRun, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output.push_str(":start");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        pub mod scan_runs {
            pub mod params {}
            pub struct ScanRunsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ScanRunsActions<'a, A> {
                #[doc = "Gets a ScanRun."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
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
                #[doc = "Lists ScanRuns under a given ScanConfig, in descending order of ScanRun\nstop time."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
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
                ) -> StopRequestBuilder<A> {
                    StopRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
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
                #[doc = "Actions that can be performed on the crawled_urls resource"]
                pub fn crawled_urls(&self) -> crawled_urls::CrawledUrlsActions<A> {
                    crawled_urls::CrawledUrlsActions
                }
                #[doc = "Actions that can be performed on the finding_type_stats resource"]
                pub fn finding_type_stats(&self) -> finding_type_stats::FindingTypeStatsActions<A> {
                    finding_type_stats::FindingTypeStatsActions
                }
                #[doc = "Actions that can be performed on the findings resource"]
                pub fn findings(&self) -> findings::FindingsActions<A> {
                    findings::FindingsActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::ScanRun, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "The maximum number of ScanRuns to return, can be limited by server.\nIf not specified or not positive, the implementation will select a\nreasonable value."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results to be returned. This should be a\n`next_page_token` value returned from a previous List request.\nIf unspecified, the first page of results is returned."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_scan_runs<T>(
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
                                #[serde(rename = "scanRuns")]
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
                ) -> Result<crate::schemas::ListScanRunsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    output.push_str(&self.parent);
                    output.push_str("/scanRuns");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
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
            pub struct StopRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> StopRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::ScanRun, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    output.push_str(&self.name);
                    output.push_str(":stop");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            pub mod crawled_urls {
                pub mod params {}
                pub struct CrawledUrlsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> CrawledUrlsActions<'a, A> {
                    #[doc = "List CrawledUrls under a given ScanRun."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
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
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "The maximum number of CrawledUrls to return, can be limited by server.\nIf not specified or not positive, the implementation will select a\nreasonable value."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results to be returned. This should be a\n`next_page_token` value returned from a previous List request.\nIf unspecified, the first page of results is returned."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_crawled_urls<T>(
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
                                    #[serde(rename = "crawledUrls")]
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
                    ) -> Result<crate::schemas::ListCrawledUrlsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        output.push_str(&self.parent);
                        output.push_str("/crawledUrls");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
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
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
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
            }
            pub mod finding_type_stats {
                pub mod params {}
                pub struct FindingTypeStatsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FindingTypeStatsActions<'a, A> {
                    #[doc = "List all FindingTypeStats under a given ScanRun."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
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
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
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
                        crate::schemas::ListFindingTypeStatsResponse,
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
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        output.push_str(&self.parent);
                        output.push_str("/findingTypeStats");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod findings {
                pub mod params {}
                pub struct FindingsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FindingsActions<'a, A> {
                    #[doc = "Gets a Finding."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
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
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
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
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Finding, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
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
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "The filter expression. The expression must be in the format: <field>\n<operator> <value>.\nSupported field: 'finding_type'.\nSupported operator: '='."]
                    pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The maximum number of Findings to return, can be limited by server.\nIf not specified or not positive, the implementation will select a\nreasonable value."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results to be returned. This should be a\n`next_page_token` value returned from a previous List request.\nIf unspecified, the first page of results is returned."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_findings<T>(
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
                                    #[serde(rename = "findings")]
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
                    ) -> Result<crate::schemas::ListFindingsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://websecurityscanner.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        output.push_str(&self.parent);
                        output.push_str("/findings");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
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
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
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
            }
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
