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
    pub struct Checksum {
        #[doc = "The SHA256 hash of the client state; that is, of the sorted list of all\nhashes present in the database."]
        #[serde(rename = "sha256", default)]
        pub sha_256: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for Checksum {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ClientInfo {
        #[doc = "A client ID that (hopefully) uniquely identifies the client implementation\nof the Safe Browsing API."]
        #[serde(rename = "clientId", default)]
        pub client_id: ::std::option::Option<String>,
        #[doc = "The version of the client implementation."]
        #[serde(rename = "clientVersion", default)]
        pub client_version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ClientInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConstraintsSupportedCompressionsItems {
        CompressionTypeUnspecified,
        Raw,
        Rice,
    }
    impl ConstraintsSupportedCompressionsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ConstraintsSupportedCompressionsItems::CompressionTypeUnspecified => {
                    "COMPRESSION_TYPE_UNSPECIFIED"
                }
                ConstraintsSupportedCompressionsItems::Raw => "RAW",
                ConstraintsSupportedCompressionsItems::Rice => "RICE",
            }
        }
    }
    impl ::std::fmt::Display for ConstraintsSupportedCompressionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConstraintsSupportedCompressionsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConstraintsSupportedCompressionsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPRESSION_TYPE_UNSPECIFIED" => {
                    ConstraintsSupportedCompressionsItems::CompressionTypeUnspecified
                }
                "RAW" => ConstraintsSupportedCompressionsItems::Raw,
                "RICE" => ConstraintsSupportedCompressionsItems::Rice,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ConstraintsSupportedCompressionsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Constraints {
        #[doc = "A client's physical location, expressed as a ISO 31166-1 alpha-2\nregion code."]
        #[serde(rename = "deviceLocation", default)]
        pub device_location: ::std::option::Option<String>,
        #[doc = "Requests the lists for a specific language. Expects ISO 639 alpha-2\nformat."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "Sets the maximum number of entries that the client is willing to have\nin the local database. This should be a power of 2 between 2**10 and\n2**20. If zero, no database size limit is set."]
        #[serde(rename = "maxDatabaseEntries", default)]
        pub max_database_entries: ::std::option::Option<i32>,
        #[doc = "The maximum size in number of entries. The update will not contain more\nentries than this value.  This should be a power of 2 between 2**10 and\n2**20.  If zero, no update size limit is set."]
        #[serde(rename = "maxUpdateEntries", default)]
        pub max_update_entries: ::std::option::Option<i32>,
        #[doc = "Requests the list for a specific geographic location. If not set the\nserver may pick that value based on the user's IP address. Expects ISO\n3166-1 alpha-2 format."]
        #[serde(rename = "region", default)]
        pub region: ::std::option::Option<String>,
        #[doc = "The compression types supported by the client."]
        #[serde(rename = "supportedCompressions", default)]
        pub supported_compressions:
            ::std::option::Option<Vec<crate::schemas::ConstraintsSupportedCompressionsItems>>,
    }
    impl ::field_selector::FieldSelector for Constraints {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct FetchThreatListUpdatesRequest {
        #[doc = "The client metadata."]
        #[serde(rename = "client", default)]
        pub client: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The requested threat list updates."]
        #[serde(rename = "listUpdateRequests", default)]
        pub list_update_requests: ::std::option::Option<Vec<crate::schemas::ListUpdateRequest>>,
    }
    impl ::field_selector::FieldSelector for FetchThreatListUpdatesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct FetchThreatListUpdatesResponse {
        #[doc = "The list updates requested by the clients."]
        #[serde(rename = "listUpdateResponses", default)]
        pub list_update_responses: ::std::option::Option<Vec<crate::schemas::ListUpdateResponse>>,
        #[doc = "The minimum duration the client must wait before issuing any update\nrequest. If this field is not set clients may update as soon as they want."]
        #[serde(rename = "minimumWaitDuration", default)]
        pub minimum_wait_duration: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for FetchThreatListUpdatesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct FindFullHashesRequest {
        #[doc = "Client metadata associated with callers of higher-level APIs built on top\nof the client's implementation."]
        #[serde(rename = "apiClient", default)]
        pub api_client: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The client metadata."]
        #[serde(rename = "client", default)]
        pub client: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The current client states for each of the client's local threat lists."]
        #[serde(rename = "clientStates", default)]
        pub client_states: ::std::option::Option<Vec<crate::bytes::Bytes>>,
        #[doc = "The lists and hashes to be checked."]
        #[serde(rename = "threatInfo", default)]
        pub threat_info: ::std::option::Option<crate::schemas::ThreatInfo>,
    }
    impl ::field_selector::FieldSelector for FindFullHashesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct FindFullHashesResponse {
        #[doc = "The full hashes that matched the requested prefixes."]
        #[serde(rename = "matches", default)]
        pub matches: ::std::option::Option<Vec<crate::schemas::ThreatMatch>>,
        #[doc = "The minimum duration the client must wait before issuing any find hashes\nrequest. If this field is not set, clients can issue a request as soon as\nthey want."]
        #[serde(rename = "minimumWaitDuration", default)]
        pub minimum_wait_duration: ::std::option::Option<String>,
        #[doc = "For requested entities that did not match the threat list, how long to\ncache the response."]
        #[serde(rename = "negativeCacheDuration", default)]
        pub negative_cache_duration: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for FindFullHashesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct FindThreatMatchesRequest {
        #[doc = "The client metadata."]
        #[serde(rename = "client", default)]
        pub client: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The lists and entries to be checked for matches."]
        #[serde(rename = "threatInfo", default)]
        pub threat_info: ::std::option::Option<crate::schemas::ThreatInfo>,
    }
    impl ::field_selector::FieldSelector for FindThreatMatchesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct FindThreatMatchesResponse {
        #[doc = "The threat list matches."]
        #[serde(rename = "matches", default)]
        pub matches: ::std::option::Option<Vec<crate::schemas::ThreatMatch>>,
    }
    impl ::field_selector::FieldSelector for FindThreatMatchesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ListThreatListsResponse {
        #[doc = "The lists available for download by the client."]
        #[serde(rename = "threatLists", default)]
        pub threat_lists: ::std::option::Option<Vec<crate::schemas::ThreatListDescriptor>>,
    }
    impl ::field_selector::FieldSelector for ListThreatListsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateRequestPlatformType {
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ListUpdateRequestPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateRequestPlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ListUpdateRequestPlatformType::Windows => "WINDOWS",
                ListUpdateRequestPlatformType::Linux => "LINUX",
                ListUpdateRequestPlatformType::Android => "ANDROID",
                ListUpdateRequestPlatformType::Osx => "OSX",
                ListUpdateRequestPlatformType::Ios => "IOS",
                ListUpdateRequestPlatformType::AnyPlatform => "ANY_PLATFORM",
                ListUpdateRequestPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ListUpdateRequestPlatformType::Chrome => "CHROME",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateRequestPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateRequestPlatformType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateRequestPlatformType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ListUpdateRequestPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => ListUpdateRequestPlatformType::Windows,
                "LINUX" => ListUpdateRequestPlatformType::Linux,
                "ANDROID" => ListUpdateRequestPlatformType::Android,
                "OSX" => ListUpdateRequestPlatformType::Osx,
                "IOS" => ListUpdateRequestPlatformType::Ios,
                "ANY_PLATFORM" => ListUpdateRequestPlatformType::AnyPlatform,
                "ALL_PLATFORMS" => ListUpdateRequestPlatformType::AllPlatforms,
                "CHROME" => ListUpdateRequestPlatformType::Chrome,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ListUpdateRequestPlatformType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateRequestThreatEntryType {
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
        #[doc = "An executable program."]
        Executable,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "Filename."]
        Filename,
        #[doc = "CERT"]
        Cert,
    }
    impl ListUpdateRequestThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateRequestThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ListUpdateRequestThreatEntryType::Url => "URL",
                ListUpdateRequestThreatEntryType::Executable => "EXECUTABLE",
                ListUpdateRequestThreatEntryType::IpRange => "IP_RANGE",
                ListUpdateRequestThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ListUpdateRequestThreatEntryType::Filename => "FILENAME",
                ListUpdateRequestThreatEntryType::Cert => "CERT",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateRequestThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateRequestThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateRequestThreatEntryType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ListUpdateRequestThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ListUpdateRequestThreatEntryType::Url,
                "EXECUTABLE" => ListUpdateRequestThreatEntryType::Executable,
                "IP_RANGE" => ListUpdateRequestThreatEntryType::IpRange,
                "CHROME_EXTENSION" => ListUpdateRequestThreatEntryType::ChromeExtension,
                "FILENAME" => ListUpdateRequestThreatEntryType::Filename,
                "CERT" => ListUpdateRequestThreatEntryType::Cert,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ListUpdateRequestThreatEntryType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateRequestThreatType {
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ListUpdateRequestThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateRequestThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ListUpdateRequestThreatType::Malware => "MALWARE",
                ListUpdateRequestThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ListUpdateRequestThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
                ListUpdateRequestThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ListUpdateRequestThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ListUpdateRequestThreatType::ApiAbuse => "API_ABUSE",
                ListUpdateRequestThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ListUpdateRequestThreatType::CsdWhitelist => "CSD_WHITELIST",
                ListUpdateRequestThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ListUpdateRequestThreatType::ClientIncident => "CLIENT_INCIDENT",
                ListUpdateRequestThreatType::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ListUpdateRequestThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ListUpdateRequestThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ListUpdateRequestThreatType::Suspicious => "SUSPICIOUS",
                ListUpdateRequestThreatType::TrickToBill => "TRICK_TO_BILL",
                ListUpdateRequestThreatType::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateRequestThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateRequestThreatType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateRequestThreatType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_TYPE_UNSPECIFIED" => ListUpdateRequestThreatType::ThreatTypeUnspecified,
                "MALWARE" => ListUpdateRequestThreatType::Malware,
                "SOCIAL_ENGINEERING" => ListUpdateRequestThreatType::SocialEngineering,
                "UNWANTED_SOFTWARE" => ListUpdateRequestThreatType::UnwantedSoftware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ListUpdateRequestThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ListUpdateRequestThreatType::SocialEngineeringInternal
                }
                "API_ABUSE" => ListUpdateRequestThreatType::ApiAbuse,
                "MALICIOUS_BINARY" => ListUpdateRequestThreatType::MaliciousBinary,
                "CSD_WHITELIST" => ListUpdateRequestThreatType::CsdWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ListUpdateRequestThreatType::CsdDownloadWhitelist,
                "CLIENT_INCIDENT" => ListUpdateRequestThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ListUpdateRequestThreatType::ClientIncidentWhitelist,
                "APK_MALWARE_OFFLINE" => ListUpdateRequestThreatType::ApkMalwareOffline,
                "SUBRESOURCE_FILTER" => ListUpdateRequestThreatType::SubresourceFilter,
                "SUSPICIOUS" => ListUpdateRequestThreatType::Suspicious,
                "TRICK_TO_BILL" => ListUpdateRequestThreatType::TrickToBill,
                "HIGH_CONFIDENCE_ALLOWLIST" => ListUpdateRequestThreatType::HighConfidenceAllowlist,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ListUpdateRequestThreatType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ListUpdateRequest {
        #[doc = "The constraints associated with this request."]
        #[serde(rename = "constraints", default)]
        pub constraints: ::std::option::Option<crate::schemas::Constraints>,
        #[doc = "The type of platform at risk by entries present in the list."]
        #[serde(rename = "platformType", default)]
        pub platform_type: ::std::option::Option<crate::schemas::ListUpdateRequestPlatformType>,
        #[doc = "The current state of the client for the requested list (the encrypted\nclient state that was received from the last successful list update)."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The types of entries present in the list."]
        #[serde(rename = "threatEntryType", default)]
        pub threat_entry_type:
            ::std::option::Option<crate::schemas::ListUpdateRequestThreatEntryType>,
        #[doc = "The type of threat posed by entries present in the list."]
        #[serde(rename = "threatType", default)]
        pub threat_type: ::std::option::Option<crate::schemas::ListUpdateRequestThreatType>,
    }
    impl ::field_selector::FieldSelector for ListUpdateRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponsePlatformType {
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ListUpdateResponsePlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponsePlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ListUpdateResponsePlatformType::Windows => "WINDOWS",
                ListUpdateResponsePlatformType::Linux => "LINUX",
                ListUpdateResponsePlatformType::Android => "ANDROID",
                ListUpdateResponsePlatformType::Osx => "OSX",
                ListUpdateResponsePlatformType::Ios => "IOS",
                ListUpdateResponsePlatformType::AnyPlatform => "ANY_PLATFORM",
                ListUpdateResponsePlatformType::AllPlatforms => "ALL_PLATFORMS",
                ListUpdateResponsePlatformType::Chrome => "CHROME",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponsePlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponsePlatformType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponsePlatformType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ListUpdateResponsePlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => ListUpdateResponsePlatformType::Windows,
                "LINUX" => ListUpdateResponsePlatformType::Linux,
                "ANDROID" => ListUpdateResponsePlatformType::Android,
                "OSX" => ListUpdateResponsePlatformType::Osx,
                "IOS" => ListUpdateResponsePlatformType::Ios,
                "ANY_PLATFORM" => ListUpdateResponsePlatformType::AnyPlatform,
                "ALL_PLATFORMS" => ListUpdateResponsePlatformType::AllPlatforms,
                "CHROME" => ListUpdateResponsePlatformType::Chrome,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ListUpdateResponsePlatformType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponseResponseType {
        #[doc = "Unknown."]
        ResponseTypeUnspecified,
        #[doc = "Partial updates are applied to the client's existing local database."]
        PartialUpdate,
        #[doc = "Full updates replace the client's entire local database. This means\nthat either the client was seriously out-of-date or the client is\nbelieved to be corrupt."]
        FullUpdate,
    }
    impl ListUpdateResponseResponseType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponseResponseType::ResponseTypeUnspecified => {
                    "RESPONSE_TYPE_UNSPECIFIED"
                }
                ListUpdateResponseResponseType::PartialUpdate => "PARTIAL_UPDATE",
                ListUpdateResponseResponseType::FullUpdate => "FULL_UPDATE",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponseResponseType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponseResponseType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponseResponseType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RESPONSE_TYPE_UNSPECIFIED" => {
                    ListUpdateResponseResponseType::ResponseTypeUnspecified
                }
                "PARTIAL_UPDATE" => ListUpdateResponseResponseType::PartialUpdate,
                "FULL_UPDATE" => ListUpdateResponseResponseType::FullUpdate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ListUpdateResponseResponseType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponseThreatEntryType {
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
        #[doc = "An executable program."]
        Executable,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "Filename."]
        Filename,
        #[doc = "CERT"]
        Cert,
    }
    impl ListUpdateResponseThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponseThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ListUpdateResponseThreatEntryType::Url => "URL",
                ListUpdateResponseThreatEntryType::Executable => "EXECUTABLE",
                ListUpdateResponseThreatEntryType::IpRange => "IP_RANGE",
                ListUpdateResponseThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ListUpdateResponseThreatEntryType::Filename => "FILENAME",
                ListUpdateResponseThreatEntryType::Cert => "CERT",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponseThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponseThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponseThreatEntryType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ListUpdateResponseThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ListUpdateResponseThreatEntryType::Url,
                "EXECUTABLE" => ListUpdateResponseThreatEntryType::Executable,
                "IP_RANGE" => ListUpdateResponseThreatEntryType::IpRange,
                "CHROME_EXTENSION" => ListUpdateResponseThreatEntryType::ChromeExtension,
                "FILENAME" => ListUpdateResponseThreatEntryType::Filename,
                "CERT" => ListUpdateResponseThreatEntryType::Cert,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ListUpdateResponseThreatEntryType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponseThreatType {
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ListUpdateResponseThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponseThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ListUpdateResponseThreatType::Malware => "MALWARE",
                ListUpdateResponseThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ListUpdateResponseThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
                ListUpdateResponseThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ListUpdateResponseThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ListUpdateResponseThreatType::ApiAbuse => "API_ABUSE",
                ListUpdateResponseThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ListUpdateResponseThreatType::CsdWhitelist => "CSD_WHITELIST",
                ListUpdateResponseThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ListUpdateResponseThreatType::ClientIncident => "CLIENT_INCIDENT",
                ListUpdateResponseThreatType::ClientIncidentWhitelist => {
                    "CLIENT_INCIDENT_WHITELIST"
                }
                ListUpdateResponseThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ListUpdateResponseThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ListUpdateResponseThreatType::Suspicious => "SUSPICIOUS",
                ListUpdateResponseThreatType::TrickToBill => "TRICK_TO_BILL",
                ListUpdateResponseThreatType::HighConfidenceAllowlist => {
                    "HIGH_CONFIDENCE_ALLOWLIST"
                }
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponseThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponseThreatType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponseThreatType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_TYPE_UNSPECIFIED" => ListUpdateResponseThreatType::ThreatTypeUnspecified,
                "MALWARE" => ListUpdateResponseThreatType::Malware,
                "SOCIAL_ENGINEERING" => ListUpdateResponseThreatType::SocialEngineering,
                "UNWANTED_SOFTWARE" => ListUpdateResponseThreatType::UnwantedSoftware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ListUpdateResponseThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ListUpdateResponseThreatType::SocialEngineeringInternal
                }
                "API_ABUSE" => ListUpdateResponseThreatType::ApiAbuse,
                "MALICIOUS_BINARY" => ListUpdateResponseThreatType::MaliciousBinary,
                "CSD_WHITELIST" => ListUpdateResponseThreatType::CsdWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ListUpdateResponseThreatType::CsdDownloadWhitelist,
                "CLIENT_INCIDENT" => ListUpdateResponseThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => {
                    ListUpdateResponseThreatType::ClientIncidentWhitelist
                }
                "APK_MALWARE_OFFLINE" => ListUpdateResponseThreatType::ApkMalwareOffline,
                "SUBRESOURCE_FILTER" => ListUpdateResponseThreatType::SubresourceFilter,
                "SUSPICIOUS" => ListUpdateResponseThreatType::Suspicious,
                "TRICK_TO_BILL" => ListUpdateResponseThreatType::TrickToBill,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    ListUpdateResponseThreatType::HighConfidenceAllowlist
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
    impl ::field_selector::FieldSelector for ListUpdateResponseThreatType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ListUpdateResponse {
        #[doc = "A set of entries to add to a local threat type's list. Repeated to allow\nfor a combination of compressed and raw data to be sent in a single\nresponse."]
        #[serde(rename = "additions", default)]
        pub additions: ::std::option::Option<Vec<crate::schemas::ThreatEntrySet>>,
        #[doc = "The expected SHA256 hash of the client state; that is, of the sorted list\nof all hashes present in the database after applying the provided update.\nIf the client state doesn't match the expected state, the client must\ndisregard this update and retry later."]
        #[serde(rename = "checksum", default)]
        pub checksum: ::std::option::Option<crate::schemas::Checksum>,
        #[doc = "The new client state, in encrypted format. Opaque to clients."]
        #[serde(rename = "newClientState", default)]
        pub new_client_state: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The platform type for which data is returned."]
        #[serde(rename = "platformType", default)]
        pub platform_type: ::std::option::Option<crate::schemas::ListUpdateResponsePlatformType>,
        #[doc = "A set of entries to remove from a local threat type's list. In practice,\nthis field is empty or contains exactly one ThreatEntrySet."]
        #[serde(rename = "removals", default)]
        pub removals: ::std::option::Option<Vec<crate::schemas::ThreatEntrySet>>,
        #[doc = "The type of response. This may indicate that an action is required by the\nclient when the response is received."]
        #[serde(rename = "responseType", default)]
        pub response_type: ::std::option::Option<crate::schemas::ListUpdateResponseResponseType>,
        #[doc = "The format of the threats."]
        #[serde(rename = "threatEntryType", default)]
        pub threat_entry_type:
            ::std::option::Option<crate::schemas::ListUpdateResponseThreatEntryType>,
        #[doc = "The threat type for which data is returned."]
        #[serde(rename = "threatType", default)]
        pub threat_type: ::std::option::Option<crate::schemas::ListUpdateResponseThreatType>,
    }
    impl ::field_selector::FieldSelector for ListUpdateResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct MetadataEntry {
        #[doc = "The metadata entry key. For JSON requests, the key is base64-encoded."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The metadata entry value. For JSON requests, the value is base64-encoded."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for MetadataEntry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct RawHashes {
        #[doc = "The number of bytes for each prefix encoded below.  This field can be\nanywhere from 4 (shortest prefix) to 32 (full SHA256 hash)."]
        #[serde(rename = "prefixSize", default)]
        pub prefix_size: ::std::option::Option<i32>,
        #[doc = "The hashes, in binary format, concatenated into one long string. Hashes are\nsorted in lexicographic order. For JSON API users, hashes are\nbase64-encoded."]
        #[serde(rename = "rawHashes", default)]
        pub raw_hashes: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for RawHashes {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct RawIndices {
        #[doc = "The indices to remove from a lexicographically-sorted local list."]
        #[serde(rename = "indices", default)]
        pub indices: ::std::option::Option<Vec<i32>>,
    }
    impl ::field_selector::FieldSelector for RawIndices {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct RiceDeltaEncoding {
        #[doc = "The encoded deltas that are encoded using the Golomb-Rice coder."]
        #[serde(rename = "encodedData", default)]
        pub encoded_data: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The offset of the first entry in the encoded data, or, if only a single\ninteger was encoded, that single integer's value. If the field is empty or\nmissing, assume zero."]
        #[serde(rename = "firstValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub first_value: ::std::option::Option<i64>,
        #[doc = "The number of entries that are delta encoded in the encoded data. If only a\nsingle integer was encoded, this will be zero and the single value will be\nstored in `first_value`."]
        #[serde(rename = "numEntries", default)]
        pub num_entries: ::std::option::Option<i32>,
        #[doc = "The Golomb-Rice parameter, which is a number between 2 and 28. This field\nis missing (that is, zero) if `num_entries` is zero."]
        #[serde(rename = "riceParameter", default)]
        pub rice_parameter: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for RiceDeltaEncoding {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatEntry {
        #[doc = "The digest of an executable in SHA256 format. The API supports both\nbinary and hex digests. For JSON requests, digests are base64-encoded."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "A hash prefix, consisting of the most significant 4-32 bytes of a SHA256\nhash. This field is in binary format. For JSON requests, hashes are\nbase64-encoded."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "A URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ThreatEntry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatEntryMetadata {
        #[doc = "The metadata entries."]
        #[serde(rename = "entries", default)]
        pub entries: ::std::option::Option<Vec<crate::schemas::MetadataEntry>>,
    }
    impl ::field_selector::FieldSelector for ThreatEntryMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatEntrySetCompressionType {
        #[doc = "Unknown."]
        CompressionTypeUnspecified,
        #[doc = "Raw, uncompressed data."]
        Raw,
        #[doc = "Rice-Golomb encoded data."]
        Rice,
    }
    impl ThreatEntrySetCompressionType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatEntrySetCompressionType::CompressionTypeUnspecified => {
                    "COMPRESSION_TYPE_UNSPECIFIED"
                }
                ThreatEntrySetCompressionType::Raw => "RAW",
                ThreatEntrySetCompressionType::Rice => "RICE",
            }
        }
    }
    impl ::std::fmt::Display for ThreatEntrySetCompressionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatEntrySetCompressionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatEntrySetCompressionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPRESSION_TYPE_UNSPECIFIED" => {
                    ThreatEntrySetCompressionType::CompressionTypeUnspecified
                }
                "RAW" => ThreatEntrySetCompressionType::Raw,
                "RICE" => ThreatEntrySetCompressionType::Rice,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatEntrySetCompressionType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatEntrySet {
        #[doc = "The compression type for the entries in this set."]
        #[serde(rename = "compressionType", default)]
        pub compression_type: ::std::option::Option<crate::schemas::ThreatEntrySetCompressionType>,
        #[doc = "The raw SHA256-formatted entries."]
        #[serde(rename = "rawHashes", default)]
        pub raw_hashes: ::std::option::Option<crate::schemas::RawHashes>,
        #[doc = "The raw removal indices for a local list."]
        #[serde(rename = "rawIndices", default)]
        pub raw_indices: ::std::option::Option<crate::schemas::RawIndices>,
        #[doc = "The encoded 4-byte prefixes of SHA256-formatted entries, using a\nGolomb-Rice encoding. The hashes are converted to uint32, sorted in\nascending order, then delta encoded and stored as encoded_data."]
        #[serde(rename = "riceHashes", default)]
        pub rice_hashes: ::std::option::Option<crate::schemas::RiceDeltaEncoding>,
        #[doc = "The encoded local, lexicographically-sorted list indices, using a\nGolomb-Rice encoding. Used for sending compressed removal indices. The\nremoval indices (uint32) are sorted in ascending order, then delta encoded\nand stored as encoded_data."]
        #[serde(rename = "riceIndices", default)]
        pub rice_indices: ::std::option::Option<crate::schemas::RiceDeltaEncoding>,
    }
    impl ::field_selector::FieldSelector for ThreatEntrySet {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatHitPlatformType {
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ThreatHitPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatHitPlatformType::PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED",
                ThreatHitPlatformType::Windows => "WINDOWS",
                ThreatHitPlatformType::Linux => "LINUX",
                ThreatHitPlatformType::Android => "ANDROID",
                ThreatHitPlatformType::Osx => "OSX",
                ThreatHitPlatformType::Ios => "IOS",
                ThreatHitPlatformType::AnyPlatform => "ANY_PLATFORM",
                ThreatHitPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ThreatHitPlatformType::Chrome => "CHROME",
            }
        }
    }
    impl ::std::fmt::Display for ThreatHitPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatHitPlatformType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatHitPlatformType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLATFORM_TYPE_UNSPECIFIED" => ThreatHitPlatformType::PlatformTypeUnspecified,
                "WINDOWS" => ThreatHitPlatformType::Windows,
                "LINUX" => ThreatHitPlatformType::Linux,
                "ANDROID" => ThreatHitPlatformType::Android,
                "OSX" => ThreatHitPlatformType::Osx,
                "IOS" => ThreatHitPlatformType::Ios,
                "ANY_PLATFORM" => ThreatHitPlatformType::AnyPlatform,
                "ALL_PLATFORMS" => ThreatHitPlatformType::AllPlatforms,
                "CHROME" => ThreatHitPlatformType::Chrome,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatHitPlatformType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatHitThreatType {
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ThreatHitThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatHitThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatHitThreatType::Malware => "MALWARE",
                ThreatHitThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatHitThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
                ThreatHitThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatHitThreatType::SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL",
                ThreatHitThreatType::ApiAbuse => "API_ABUSE",
                ThreatHitThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatHitThreatType::CsdWhitelist => "CSD_WHITELIST",
                ThreatHitThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatHitThreatType::ClientIncident => "CLIENT_INCIDENT",
                ThreatHitThreatType::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ThreatHitThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatHitThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatHitThreatType::Suspicious => "SUSPICIOUS",
                ThreatHitThreatType::TrickToBill => "TRICK_TO_BILL",
                ThreatHitThreatType::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
            }
        }
    }
    impl ::std::fmt::Display for ThreatHitThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatHitThreatType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatHitThreatType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_TYPE_UNSPECIFIED" => ThreatHitThreatType::ThreatTypeUnspecified,
                "MALWARE" => ThreatHitThreatType::Malware,
                "SOCIAL_ENGINEERING" => ThreatHitThreatType::SocialEngineering,
                "UNWANTED_SOFTWARE" => ThreatHitThreatType::UnwantedSoftware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatHitThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING_INTERNAL" => ThreatHitThreatType::SocialEngineeringInternal,
                "API_ABUSE" => ThreatHitThreatType::ApiAbuse,
                "MALICIOUS_BINARY" => ThreatHitThreatType::MaliciousBinary,
                "CSD_WHITELIST" => ThreatHitThreatType::CsdWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatHitThreatType::CsdDownloadWhitelist,
                "CLIENT_INCIDENT" => ThreatHitThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ThreatHitThreatType::ClientIncidentWhitelist,
                "APK_MALWARE_OFFLINE" => ThreatHitThreatType::ApkMalwareOffline,
                "SUBRESOURCE_FILTER" => ThreatHitThreatType::SubresourceFilter,
                "SUSPICIOUS" => ThreatHitThreatType::Suspicious,
                "TRICK_TO_BILL" => ThreatHitThreatType::TrickToBill,
                "HIGH_CONFIDENCE_ALLOWLIST" => ThreatHitThreatType::HighConfidenceAllowlist,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatHitThreatType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatHit {
        #[doc = "Client-reported identification."]
        #[serde(rename = "clientInfo", default)]
        pub client_info: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The threat entry responsible for the hit. Full hash should be reported for\nhash-based hits."]
        #[serde(rename = "entry", default)]
        pub entry: ::std::option::Option<crate::schemas::ThreatEntry>,
        #[doc = "The platform type reported."]
        #[serde(rename = "platformType", default)]
        pub platform_type: ::std::option::Option<crate::schemas::ThreatHitPlatformType>,
        #[doc = "The resources related to the threat hit."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<Vec<crate::schemas::ThreatSource>>,
        #[doc = "The threat type reported."]
        #[serde(rename = "threatType", default)]
        pub threat_type: ::std::option::Option<crate::schemas::ThreatHitThreatType>,
        #[doc = "Details about the user that encountered the threat."]
        #[serde(rename = "userInfo", default)]
        pub user_info: ::std::option::Option<crate::schemas::UserInfo>,
    }
    impl ::field_selector::FieldSelector for ThreatHit {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatInfoPlatformTypesItems {
        PlatformTypeUnspecified,
        Windows,
        Linux,
        Android,
        Osx,
        Ios,
        AnyPlatform,
        AllPlatforms,
        Chrome,
    }
    impl ThreatInfoPlatformTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatInfoPlatformTypesItems::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ThreatInfoPlatformTypesItems::Windows => "WINDOWS",
                ThreatInfoPlatformTypesItems::Linux => "LINUX",
                ThreatInfoPlatformTypesItems::Android => "ANDROID",
                ThreatInfoPlatformTypesItems::Osx => "OSX",
                ThreatInfoPlatformTypesItems::Ios => "IOS",
                ThreatInfoPlatformTypesItems::AnyPlatform => "ANY_PLATFORM",
                ThreatInfoPlatformTypesItems::AllPlatforms => "ALL_PLATFORMS",
                ThreatInfoPlatformTypesItems::Chrome => "CHROME",
            }
        }
    }
    impl ::std::fmt::Display for ThreatInfoPlatformTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatInfoPlatformTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatInfoPlatformTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ThreatInfoPlatformTypesItems::PlatformTypeUnspecified
                }
                "WINDOWS" => ThreatInfoPlatformTypesItems::Windows,
                "LINUX" => ThreatInfoPlatformTypesItems::Linux,
                "ANDROID" => ThreatInfoPlatformTypesItems::Android,
                "OSX" => ThreatInfoPlatformTypesItems::Osx,
                "IOS" => ThreatInfoPlatformTypesItems::Ios,
                "ANY_PLATFORM" => ThreatInfoPlatformTypesItems::AnyPlatform,
                "ALL_PLATFORMS" => ThreatInfoPlatformTypesItems::AllPlatforms,
                "CHROME" => ThreatInfoPlatformTypesItems::Chrome,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatInfoPlatformTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatInfoThreatEntryTypesItems {
        ThreatEntryTypeUnspecified,
        Url,
        Executable,
        IpRange,
        ChromeExtension,
        Filename,
        Cert,
    }
    impl ThreatInfoThreatEntryTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatInfoThreatEntryTypesItems::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ThreatInfoThreatEntryTypesItems::Url => "URL",
                ThreatInfoThreatEntryTypesItems::Executable => "EXECUTABLE",
                ThreatInfoThreatEntryTypesItems::IpRange => "IP_RANGE",
                ThreatInfoThreatEntryTypesItems::ChromeExtension => "CHROME_EXTENSION",
                ThreatInfoThreatEntryTypesItems::Filename => "FILENAME",
                ThreatInfoThreatEntryTypesItems::Cert => "CERT",
            }
        }
    }
    impl ::std::fmt::Display for ThreatInfoThreatEntryTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatInfoThreatEntryTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatInfoThreatEntryTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ThreatInfoThreatEntryTypesItems::ThreatEntryTypeUnspecified
                }
                "URL" => ThreatInfoThreatEntryTypesItems::Url,
                "EXECUTABLE" => ThreatInfoThreatEntryTypesItems::Executable,
                "IP_RANGE" => ThreatInfoThreatEntryTypesItems::IpRange,
                "CHROME_EXTENSION" => ThreatInfoThreatEntryTypesItems::ChromeExtension,
                "FILENAME" => ThreatInfoThreatEntryTypesItems::Filename,
                "CERT" => ThreatInfoThreatEntryTypesItems::Cert,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatInfoThreatEntryTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatInfoThreatTypesItems {
        ThreatTypeUnspecified,
        Malware,
        SocialEngineering,
        UnwantedSoftware,
        PotentiallyHarmfulApplication,
        SocialEngineeringInternal,
        ApiAbuse,
        MaliciousBinary,
        CsdWhitelist,
        CsdDownloadWhitelist,
        ClientIncident,
        ClientIncidentWhitelist,
        ApkMalwareOffline,
        SubresourceFilter,
        Suspicious,
        TrickToBill,
        HighConfidenceAllowlist,
    }
    impl ThreatInfoThreatTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatInfoThreatTypesItems::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatInfoThreatTypesItems::Malware => "MALWARE",
                ThreatInfoThreatTypesItems::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatInfoThreatTypesItems::UnwantedSoftware => "UNWANTED_SOFTWARE",
                ThreatInfoThreatTypesItems::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatInfoThreatTypesItems::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ThreatInfoThreatTypesItems::ApiAbuse => "API_ABUSE",
                ThreatInfoThreatTypesItems::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatInfoThreatTypesItems::CsdWhitelist => "CSD_WHITELIST",
                ThreatInfoThreatTypesItems::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatInfoThreatTypesItems::ClientIncident => "CLIENT_INCIDENT",
                ThreatInfoThreatTypesItems::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ThreatInfoThreatTypesItems::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatInfoThreatTypesItems::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatInfoThreatTypesItems::Suspicious => "SUSPICIOUS",
                ThreatInfoThreatTypesItems::TrickToBill => "TRICK_TO_BILL",
                ThreatInfoThreatTypesItems::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
            }
        }
    }
    impl ::std::fmt::Display for ThreatInfoThreatTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatInfoThreatTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatInfoThreatTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_TYPE_UNSPECIFIED" => ThreatInfoThreatTypesItems::ThreatTypeUnspecified,
                "MALWARE" => ThreatInfoThreatTypesItems::Malware,
                "SOCIAL_ENGINEERING" => ThreatInfoThreatTypesItems::SocialEngineering,
                "UNWANTED_SOFTWARE" => ThreatInfoThreatTypesItems::UnwantedSoftware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatInfoThreatTypesItems::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ThreatInfoThreatTypesItems::SocialEngineeringInternal
                }
                "API_ABUSE" => ThreatInfoThreatTypesItems::ApiAbuse,
                "MALICIOUS_BINARY" => ThreatInfoThreatTypesItems::MaliciousBinary,
                "CSD_WHITELIST" => ThreatInfoThreatTypesItems::CsdWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatInfoThreatTypesItems::CsdDownloadWhitelist,
                "CLIENT_INCIDENT" => ThreatInfoThreatTypesItems::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ThreatInfoThreatTypesItems::ClientIncidentWhitelist,
                "APK_MALWARE_OFFLINE" => ThreatInfoThreatTypesItems::ApkMalwareOffline,
                "SUBRESOURCE_FILTER" => ThreatInfoThreatTypesItems::SubresourceFilter,
                "SUSPICIOUS" => ThreatInfoThreatTypesItems::Suspicious,
                "TRICK_TO_BILL" => ThreatInfoThreatTypesItems::TrickToBill,
                "HIGH_CONFIDENCE_ALLOWLIST" => ThreatInfoThreatTypesItems::HighConfidenceAllowlist,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatInfoThreatTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatInfo {
        #[doc = "The platform types to be checked."]
        #[serde(rename = "platformTypes", default)]
        pub platform_types:
            ::std::option::Option<Vec<crate::schemas::ThreatInfoPlatformTypesItems>>,
        #[doc = "The threat entries to be checked."]
        #[serde(rename = "threatEntries", default)]
        pub threat_entries: ::std::option::Option<Vec<crate::schemas::ThreatEntry>>,
        #[doc = "The entry types to be checked."]
        #[serde(rename = "threatEntryTypes", default)]
        pub threat_entry_types:
            ::std::option::Option<Vec<crate::schemas::ThreatInfoThreatEntryTypesItems>>,
        #[doc = "The threat types to be checked."]
        #[serde(rename = "threatTypes", default)]
        pub threat_types: ::std::option::Option<Vec<crate::schemas::ThreatInfoThreatTypesItems>>,
    }
    impl ::field_selector::FieldSelector for ThreatInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatListDescriptorPlatformType {
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ThreatListDescriptorPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatListDescriptorPlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ThreatListDescriptorPlatformType::Windows => "WINDOWS",
                ThreatListDescriptorPlatformType::Linux => "LINUX",
                ThreatListDescriptorPlatformType::Android => "ANDROID",
                ThreatListDescriptorPlatformType::Osx => "OSX",
                ThreatListDescriptorPlatformType::Ios => "IOS",
                ThreatListDescriptorPlatformType::AnyPlatform => "ANY_PLATFORM",
                ThreatListDescriptorPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ThreatListDescriptorPlatformType::Chrome => "CHROME",
            }
        }
    }
    impl ::std::fmt::Display for ThreatListDescriptorPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatListDescriptorPlatformType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatListDescriptorPlatformType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ThreatListDescriptorPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => ThreatListDescriptorPlatformType::Windows,
                "LINUX" => ThreatListDescriptorPlatformType::Linux,
                "ANDROID" => ThreatListDescriptorPlatformType::Android,
                "OSX" => ThreatListDescriptorPlatformType::Osx,
                "IOS" => ThreatListDescriptorPlatformType::Ios,
                "ANY_PLATFORM" => ThreatListDescriptorPlatformType::AnyPlatform,
                "ALL_PLATFORMS" => ThreatListDescriptorPlatformType::AllPlatforms,
                "CHROME" => ThreatListDescriptorPlatformType::Chrome,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatListDescriptorPlatformType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatListDescriptorThreatEntryType {
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
        #[doc = "An executable program."]
        Executable,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "Filename."]
        Filename,
        #[doc = "CERT"]
        Cert,
    }
    impl ThreatListDescriptorThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatListDescriptorThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ThreatListDescriptorThreatEntryType::Url => "URL",
                ThreatListDescriptorThreatEntryType::Executable => "EXECUTABLE",
                ThreatListDescriptorThreatEntryType::IpRange => "IP_RANGE",
                ThreatListDescriptorThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ThreatListDescriptorThreatEntryType::Filename => "FILENAME",
                ThreatListDescriptorThreatEntryType::Cert => "CERT",
            }
        }
    }
    impl ::std::fmt::Display for ThreatListDescriptorThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatListDescriptorThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatListDescriptorThreatEntryType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ThreatListDescriptorThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ThreatListDescriptorThreatEntryType::Url,
                "EXECUTABLE" => ThreatListDescriptorThreatEntryType::Executable,
                "IP_RANGE" => ThreatListDescriptorThreatEntryType::IpRange,
                "CHROME_EXTENSION" => ThreatListDescriptorThreatEntryType::ChromeExtension,
                "FILENAME" => ThreatListDescriptorThreatEntryType::Filename,
                "CERT" => ThreatListDescriptorThreatEntryType::Cert,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatListDescriptorThreatEntryType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatListDescriptorThreatType {
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ThreatListDescriptorThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatListDescriptorThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatListDescriptorThreatType::Malware => "MALWARE",
                ThreatListDescriptorThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatListDescriptorThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
                ThreatListDescriptorThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatListDescriptorThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ThreatListDescriptorThreatType::ApiAbuse => "API_ABUSE",
                ThreatListDescriptorThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatListDescriptorThreatType::CsdWhitelist => "CSD_WHITELIST",
                ThreatListDescriptorThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatListDescriptorThreatType::ClientIncident => "CLIENT_INCIDENT",
                ThreatListDescriptorThreatType::ClientIncidentWhitelist => {
                    "CLIENT_INCIDENT_WHITELIST"
                }
                ThreatListDescriptorThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatListDescriptorThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatListDescriptorThreatType::Suspicious => "SUSPICIOUS",
                ThreatListDescriptorThreatType::TrickToBill => "TRICK_TO_BILL",
                ThreatListDescriptorThreatType::HighConfidenceAllowlist => {
                    "HIGH_CONFIDENCE_ALLOWLIST"
                }
            }
        }
    }
    impl ::std::fmt::Display for ThreatListDescriptorThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatListDescriptorThreatType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatListDescriptorThreatType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_TYPE_UNSPECIFIED" => ThreatListDescriptorThreatType::ThreatTypeUnspecified,
                "MALWARE" => ThreatListDescriptorThreatType::Malware,
                "SOCIAL_ENGINEERING" => ThreatListDescriptorThreatType::SocialEngineering,
                "UNWANTED_SOFTWARE" => ThreatListDescriptorThreatType::UnwantedSoftware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatListDescriptorThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ThreatListDescriptorThreatType::SocialEngineeringInternal
                }
                "API_ABUSE" => ThreatListDescriptorThreatType::ApiAbuse,
                "MALICIOUS_BINARY" => ThreatListDescriptorThreatType::MaliciousBinary,
                "CSD_WHITELIST" => ThreatListDescriptorThreatType::CsdWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatListDescriptorThreatType::CsdDownloadWhitelist,
                "CLIENT_INCIDENT" => ThreatListDescriptorThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => {
                    ThreatListDescriptorThreatType::ClientIncidentWhitelist
                }
                "APK_MALWARE_OFFLINE" => ThreatListDescriptorThreatType::ApkMalwareOffline,
                "SUBRESOURCE_FILTER" => ThreatListDescriptorThreatType::SubresourceFilter,
                "SUSPICIOUS" => ThreatListDescriptorThreatType::Suspicious,
                "TRICK_TO_BILL" => ThreatListDescriptorThreatType::TrickToBill,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    ThreatListDescriptorThreatType::HighConfidenceAllowlist
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
    impl ::field_selector::FieldSelector for ThreatListDescriptorThreatType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatListDescriptor {
        #[doc = "The platform type targeted by the list's entries."]
        #[serde(rename = "platformType", default)]
        pub platform_type: ::std::option::Option<crate::schemas::ThreatListDescriptorPlatformType>,
        #[doc = "The entry types contained in the list."]
        #[serde(rename = "threatEntryType", default)]
        pub threat_entry_type:
            ::std::option::Option<crate::schemas::ThreatListDescriptorThreatEntryType>,
        #[doc = "The threat type posed by the list's entries."]
        #[serde(rename = "threatType", default)]
        pub threat_type: ::std::option::Option<crate::schemas::ThreatListDescriptorThreatType>,
    }
    impl ::field_selector::FieldSelector for ThreatListDescriptor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatMatchPlatformType {
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ThreatMatchPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatMatchPlatformType::PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED",
                ThreatMatchPlatformType::Windows => "WINDOWS",
                ThreatMatchPlatformType::Linux => "LINUX",
                ThreatMatchPlatformType::Android => "ANDROID",
                ThreatMatchPlatformType::Osx => "OSX",
                ThreatMatchPlatformType::Ios => "IOS",
                ThreatMatchPlatformType::AnyPlatform => "ANY_PLATFORM",
                ThreatMatchPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ThreatMatchPlatformType::Chrome => "CHROME",
            }
        }
    }
    impl ::std::fmt::Display for ThreatMatchPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatMatchPlatformType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatMatchPlatformType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLATFORM_TYPE_UNSPECIFIED" => ThreatMatchPlatformType::PlatformTypeUnspecified,
                "WINDOWS" => ThreatMatchPlatformType::Windows,
                "LINUX" => ThreatMatchPlatformType::Linux,
                "ANDROID" => ThreatMatchPlatformType::Android,
                "OSX" => ThreatMatchPlatformType::Osx,
                "IOS" => ThreatMatchPlatformType::Ios,
                "ANY_PLATFORM" => ThreatMatchPlatformType::AnyPlatform,
                "ALL_PLATFORMS" => ThreatMatchPlatformType::AllPlatforms,
                "CHROME" => ThreatMatchPlatformType::Chrome,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatMatchPlatformType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatMatchThreatEntryType {
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
        #[doc = "An executable program."]
        Executable,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "Filename."]
        Filename,
        #[doc = "CERT"]
        Cert,
    }
    impl ThreatMatchThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatMatchThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ThreatMatchThreatEntryType::Url => "URL",
                ThreatMatchThreatEntryType::Executable => "EXECUTABLE",
                ThreatMatchThreatEntryType::IpRange => "IP_RANGE",
                ThreatMatchThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ThreatMatchThreatEntryType::Filename => "FILENAME",
                ThreatMatchThreatEntryType::Cert => "CERT",
            }
        }
    }
    impl ::std::fmt::Display for ThreatMatchThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatMatchThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatMatchThreatEntryType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ThreatMatchThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ThreatMatchThreatEntryType::Url,
                "EXECUTABLE" => ThreatMatchThreatEntryType::Executable,
                "IP_RANGE" => ThreatMatchThreatEntryType::IpRange,
                "CHROME_EXTENSION" => ThreatMatchThreatEntryType::ChromeExtension,
                "FILENAME" => ThreatMatchThreatEntryType::Filename,
                "CERT" => ThreatMatchThreatEntryType::Cert,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatMatchThreatEntryType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatMatchThreatType {
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ThreatMatchThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatMatchThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatMatchThreatType::Malware => "MALWARE",
                ThreatMatchThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatMatchThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
                ThreatMatchThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatMatchThreatType::SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL",
                ThreatMatchThreatType::ApiAbuse => "API_ABUSE",
                ThreatMatchThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatMatchThreatType::CsdWhitelist => "CSD_WHITELIST",
                ThreatMatchThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatMatchThreatType::ClientIncident => "CLIENT_INCIDENT",
                ThreatMatchThreatType::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ThreatMatchThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatMatchThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatMatchThreatType::Suspicious => "SUSPICIOUS",
                ThreatMatchThreatType::TrickToBill => "TRICK_TO_BILL",
                ThreatMatchThreatType::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
            }
        }
    }
    impl ::std::fmt::Display for ThreatMatchThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatMatchThreatType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatMatchThreatType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_TYPE_UNSPECIFIED" => ThreatMatchThreatType::ThreatTypeUnspecified,
                "MALWARE" => ThreatMatchThreatType::Malware,
                "SOCIAL_ENGINEERING" => ThreatMatchThreatType::SocialEngineering,
                "UNWANTED_SOFTWARE" => ThreatMatchThreatType::UnwantedSoftware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatMatchThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING_INTERNAL" => ThreatMatchThreatType::SocialEngineeringInternal,
                "API_ABUSE" => ThreatMatchThreatType::ApiAbuse,
                "MALICIOUS_BINARY" => ThreatMatchThreatType::MaliciousBinary,
                "CSD_WHITELIST" => ThreatMatchThreatType::CsdWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatMatchThreatType::CsdDownloadWhitelist,
                "CLIENT_INCIDENT" => ThreatMatchThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ThreatMatchThreatType::ClientIncidentWhitelist,
                "APK_MALWARE_OFFLINE" => ThreatMatchThreatType::ApkMalwareOffline,
                "SUBRESOURCE_FILTER" => ThreatMatchThreatType::SubresourceFilter,
                "SUSPICIOUS" => ThreatMatchThreatType::Suspicious,
                "TRICK_TO_BILL" => ThreatMatchThreatType::TrickToBill,
                "HIGH_CONFIDENCE_ALLOWLIST" => ThreatMatchThreatType::HighConfidenceAllowlist,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatMatchThreatType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatMatch {
        #[doc = "The cache lifetime for the returned match. Clients must not cache this\nresponse for more than this duration to avoid false positives."]
        #[serde(rename = "cacheDuration", default)]
        pub cache_duration: ::std::option::Option<String>,
        #[doc = "The platform type matching this threat."]
        #[serde(rename = "platformType", default)]
        pub platform_type: ::std::option::Option<crate::schemas::ThreatMatchPlatformType>,
        #[doc = "The threat matching this threat."]
        #[serde(rename = "threat", default)]
        pub threat: ::std::option::Option<crate::schemas::ThreatEntry>,
        #[doc = "Optional metadata associated with this threat."]
        #[serde(rename = "threatEntryMetadata", default)]
        pub threat_entry_metadata: ::std::option::Option<crate::schemas::ThreatEntryMetadata>,
        #[doc = "The threat entry type matching this threat."]
        #[serde(rename = "threatEntryType", default)]
        pub threat_entry_type: ::std::option::Option<crate::schemas::ThreatMatchThreatEntryType>,
        #[doc = "The threat type matching this threat."]
        #[serde(rename = "threatType", default)]
        pub threat_type: ::std::option::Option<crate::schemas::ThreatMatchThreatType>,
    }
    impl ::field_selector::FieldSelector for ThreatMatch {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatSourceType {
        #[doc = "Unknown."]
        ThreatSourceTypeUnspecified,
        #[doc = "The URL that matched the threat list (for which GetFullHash returned a\nvalid hash)."]
        MatchingUrl,
        #[doc = "The final top-level URL of the tab that the client was browsing when the\nmatch occurred."]
        TabUrl,
        #[doc = "A redirect URL that was fetched before hitting the final TAB_URL."]
        TabRedirect,
        #[doc = "A resource loaded within the final TAB_URL."]
        TabResource,
    }
    impl ThreatSourceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatSourceType::ThreatSourceTypeUnspecified => "THREAT_SOURCE_TYPE_UNSPECIFIED",
                ThreatSourceType::MatchingUrl => "MATCHING_URL",
                ThreatSourceType::TabUrl => "TAB_URL",
                ThreatSourceType::TabRedirect => "TAB_REDIRECT",
                ThreatSourceType::TabResource => "TAB_RESOURCE",
            }
        }
    }
    impl ::std::fmt::Display for ThreatSourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatSourceType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatSourceType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THREAT_SOURCE_TYPE_UNSPECIFIED" => ThreatSourceType::ThreatSourceTypeUnspecified,
                "MATCHING_URL" => ThreatSourceType::MatchingUrl,
                "TAB_URL" => ThreatSourceType::TabUrl,
                "TAB_REDIRECT" => ThreatSourceType::TabRedirect,
                "TAB_RESOURCE" => ThreatSourceType::TabResource,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ThreatSourceType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ThreatSource {
        #[doc = "The type of source reported."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::ThreatSourceType>,
        #[doc = "Referrer of the resource. Only set if the referrer is available."]
        #[serde(rename = "referrer", default)]
        pub referrer: ::std::option::Option<String>,
        #[doc = "The remote IP of the resource in ASCII format. Either IPv4 or IPv6."]
        #[serde(rename = "remoteIp", default)]
        pub remote_ip: ::std::option::Option<String>,
        #[doc = "The URL of the resource."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ThreatSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct UserInfo {
        #[doc = "The UN M.49 region code associated with the user's location."]
        #[serde(rename = "regionCode", default)]
        pub region_code: ::std::option::Option<String>,
        #[doc = "Unique user identifier defined by the client."]
        #[serde(rename = "userId", default)]
        pub user_id: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for UserInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Xgafv {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    #[doc = "Actions that can be performed on the encoded_full_hashes resource"]
    pub fn encoded_full_hashes(
        &self,
    ) -> crate::resources::encoded_full_hashes::EncodedFullHashesActions<A> {
        crate::resources::encoded_full_hashes::EncodedFullHashesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the encoded_updates resource"]
    pub fn encoded_updates(&self) -> crate::resources::encoded_updates::EncodedUpdatesActions<A> {
        crate::resources::encoded_updates::EncodedUpdatesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the full_hashes resource"]
    pub fn full_hashes(&self) -> crate::resources::full_hashes::FullHashesActions<A> {
        crate::resources::full_hashes::FullHashesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the threat_hits resource"]
    pub fn threat_hits(&self) -> crate::resources::threat_hits::ThreatHitsActions<A> {
        crate::resources::threat_hits::ThreatHitsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the threat_list_updates resource"]
    pub fn threat_list_updates(
        &self,
    ) -> crate::resources::threat_list_updates::ThreatListUpdatesActions<A> {
        crate::resources::threat_list_updates::ThreatListUpdatesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the threat_lists resource"]
    pub fn threat_lists(&self) -> crate::resources::threat_lists::ThreatListsActions<A> {
        crate::resources::threat_lists::ThreatListsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the threat_matches resource"]
    pub fn threat_matches(&self) -> crate::resources::threat_matches::ThreatMatchesActions<A> {
        crate::resources::threat_matches::ThreatMatchesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
mod resources {
    pub mod encoded_full_hashes {
        pub mod params {}
        pub struct EncodedFullHashesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> EncodedFullHashesActions<'a, A> {
            #[doc = ""]
            pub fn get(&self, encoded_request: impl Into<Vec<u8>>) -> GetRequestBuilder<A> {
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
                    encoded_request: encoded_request.into().into(),
                    client_id: None,
                    client_version: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            encoded_request: crate::bytes::Bytes,
            client_id: Option<String>,
            client_version: Option<String>,
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
            #[doc = "A client ID that (hopefully) uniquely identifies the client implementation\nof the Safe Browsing API."]
            pub fn client_id(mut self, value: impl Into<String>) -> Self {
                self.client_id = Some(value.into());
                self
            }
            #[doc = "The version of the client implementation."]
            pub fn client_version(mut self, value: impl Into<String>) -> Self {
                self.client_version = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::FindFullHashesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FindFullHashesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/encodedFullHashes/");
                {
                    let var_as_string = self.encoded_request.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientId", &self.client_id)]);
                let req = req.query(&[("clientVersion", &self.client_version)]);
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
                req
            }
        }
    }
    pub mod encoded_updates {
        pub mod params {}
        pub struct EncodedUpdatesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> EncodedUpdatesActions<'a, A> {
            #[doc = ""]
            pub fn get(&self, encoded_request: impl Into<Vec<u8>>) -> GetRequestBuilder<A> {
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
                    encoded_request: encoded_request.into().into(),
                    client_id: None,
                    client_version: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            encoded_request: crate::bytes::Bytes,
            client_id: Option<String>,
            client_version: Option<String>,
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
            #[doc = "A client ID that uniquely identifies the client implementation of the Safe\nBrowsing API."]
            pub fn client_id(mut self, value: impl Into<String>) -> Self {
                self.client_id = Some(value.into());
                self
            }
            #[doc = "The version of the client implementation."]
            pub fn client_version(mut self, value: impl Into<String>) -> Self {
                self.client_version = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/encodedUpdates/");
                {
                    let var_as_string = self.encoded_request.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientId", &self.client_id)]);
                let req = req.query(&[("clientVersion", &self.client_version)]);
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
                req
            }
        }
    }
    pub mod full_hashes {
        pub mod params {}
        pub struct FullHashesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> FullHashesActions<'a, A> {
            #[doc = "Finds the full hashes that match the requested hash prefixes."]
            pub fn find(
                &self,
                request: crate::schemas::FindFullHashesRequest,
            ) -> FindRequestBuilder<A> {
                FindRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct FindRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::FindFullHashesRequest,
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
        impl<'a, A: yup_oauth2::GetToken> FindRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::FindFullHashesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FindFullHashesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/fullHashes:find");
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
                req
            }
        }
    }
    pub mod threat_hits {
        pub mod params {}
        pub struct ThreatHitsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ThreatHitsActions<'a, A> {
            #[doc = "Reports a Safe Browsing threat list hit to Google. Only projects with\nTRUSTED_REPORTER visibility can use this method."]
            pub fn create(&self, request: crate::schemas::ThreatHit) -> CreateRequestBuilder<A> {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::ThreatHit,
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
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatHits");
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
                req
            }
        }
    }
    pub mod threat_list_updates {
        pub mod params {}
        pub struct ThreatListUpdatesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ThreatListUpdatesActions<'a, A> {
            #[doc = "Fetches the most recent threat list updates. A client can request updates\nfor multiple lists at once."]
            pub fn fetch(
                &self,
                request: crate::schemas::FetchThreatListUpdatesRequest,
            ) -> FetchRequestBuilder<A> {
                FetchRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct FetchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::FetchThreatListUpdatesRequest,
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
        impl<'a, A: yup_oauth2::GetToken> FetchRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatListUpdates:fetch");
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
                req
            }
        }
    }
    pub mod threat_lists {
        pub mod params {}
        pub struct ThreatListsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ThreatListsActions<'a, A> {
            #[doc = "Lists the Safe Browsing threat lists available for download."]
            pub fn list(&self) -> ListRequestBuilder<A> {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::ListThreatListsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListThreatListsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatLists");
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
                req
            }
        }
    }
    pub mod threat_matches {
        pub mod params {}
        pub struct ThreatMatchesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ThreatMatchesActions<'a, A> {
            #[doc = "Finds the threat entries that match the Safe Browsing lists."]
            pub fn find(
                &self,
                request: crate::schemas::FindThreatMatchesRequest,
            ) -> FindRequestBuilder<A> {
                FindRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct FindRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::FindThreatMatchesRequest,
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
        impl<'a, A: yup_oauth2::GetToken> FindRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::FindThreatMatchesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FindThreatMatchesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatMatches:find");
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
                req
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
#[allow(dead_code)]
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

#[allow(dead_code)]
struct PageIter<M, T> {
    method: M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<M, T> Iterator for PageIter<M, T>
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
} // Bytes in google apis are represented as urlsafe base64 encoded strings.
  // This defines a Bytes type that is a simple wrapper around a Vec<u8> used
  // internally to handle byte fields in google apis.
#[allow(dead_code)]
mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
