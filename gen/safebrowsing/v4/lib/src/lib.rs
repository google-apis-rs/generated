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
    impl ::google_field_selector::FieldSelector for Checksum {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Checksum {
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
    pub struct ClientInfo {
        #[doc = "A client ID that (hopefully) uniquely identifies the client implementation\nof the Safe Browsing API."]
        #[serde(rename = "clientId", default)]
        pub client_id: ::std::option::Option<String>,
        #[doc = "The version of the client implementation."]
        #[serde(rename = "clientVersion", default)]
        pub client_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ClientInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientInfo {
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
    impl ::google_field_selector::FieldSelector for Constraints {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Constraints {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConstraintsSupportedCompressionsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConstraintsSupportedCompressionsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
    impl ::google_field_selector::FieldSelector for ConstraintsSupportedCompressionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConstraintsSupportedCompressionsItems {
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
    pub struct Empty;
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
    pub struct FetchThreatListUpdatesRequest {
        #[doc = "The client metadata."]
        #[serde(rename = "client", default)]
        pub client: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The requested threat list updates."]
        #[serde(rename = "listUpdateRequests", default)]
        pub list_update_requests: ::std::option::Option<Vec<crate::schemas::ListUpdateRequest>>,
    }
    impl ::google_field_selector::FieldSelector for FetchThreatListUpdatesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FetchThreatListUpdatesRequest {
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
    pub struct FetchThreatListUpdatesResponse {
        #[doc = "The list updates requested by the clients."]
        #[serde(rename = "listUpdateResponses", default)]
        pub list_update_responses: ::std::option::Option<Vec<crate::schemas::ListUpdateResponse>>,
        #[doc = "The minimum duration the client must wait before issuing any update\nrequest. If this field is not set clients may update as soon as they want."]
        #[serde(rename = "minimumWaitDuration", default)]
        pub minimum_wait_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FetchThreatListUpdatesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FetchThreatListUpdatesResponse {
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
    impl ::google_field_selector::FieldSelector for FindFullHashesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindFullHashesRequest {
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
    impl ::google_field_selector::FieldSelector for FindFullHashesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindFullHashesResponse {
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
    pub struct FindThreatMatchesRequest {
        #[doc = "The client metadata."]
        #[serde(rename = "client", default)]
        pub client: ::std::option::Option<crate::schemas::ClientInfo>,
        #[doc = "The lists and entries to be checked for matches."]
        #[serde(rename = "threatInfo", default)]
        pub threat_info: ::std::option::Option<crate::schemas::ThreatInfo>,
    }
    impl ::google_field_selector::FieldSelector for FindThreatMatchesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindThreatMatchesRequest {
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
    pub struct FindThreatMatchesResponse {
        #[doc = "The threat list matches."]
        #[serde(rename = "matches", default)]
        pub matches: ::std::option::Option<Vec<crate::schemas::ThreatMatch>>,
    }
    impl ::google_field_selector::FieldSelector for FindThreatMatchesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindThreatMatchesResponse {
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
    pub struct ListThreatListsResponse {
        #[doc = "The lists available for download by the client."]
        #[serde(rename = "threatLists", default)]
        pub threat_lists: ::std::option::Option<Vec<crate::schemas::ThreatListDescriptor>>,
    }
    impl ::google_field_selector::FieldSelector for ListThreatListsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListThreatListsResponse {
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
    impl ::google_field_selector::FieldSelector for ListUpdateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateRequestPlatformType {
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to Chrome."]
        Chrome,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
    }
    impl ListUpdateRequestPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateRequestPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ListUpdateRequestPlatformType::Android => "ANDROID",
                ListUpdateRequestPlatformType::AnyPlatform => "ANY_PLATFORM",
                ListUpdateRequestPlatformType::Chrome => "CHROME",
                ListUpdateRequestPlatformType::Ios => "IOS",
                ListUpdateRequestPlatformType::Linux => "LINUX",
                ListUpdateRequestPlatformType::Osx => "OSX",
                ListUpdateRequestPlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ListUpdateRequestPlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateRequestPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateRequestPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateRequestPlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => ListUpdateRequestPlatformType::AllPlatforms,
                "ANDROID" => ListUpdateRequestPlatformType::Android,
                "ANY_PLATFORM" => ListUpdateRequestPlatformType::AnyPlatform,
                "CHROME" => ListUpdateRequestPlatformType::Chrome,
                "IOS" => ListUpdateRequestPlatformType::Ios,
                "LINUX" => ListUpdateRequestPlatformType::Linux,
                "OSX" => ListUpdateRequestPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ListUpdateRequestPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => ListUpdateRequestPlatformType::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListUpdateRequestPlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateRequestPlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateRequestThreatEntryType {
        #[doc = "CERT"]
        Cert,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "An executable program."]
        Executable,
        #[doc = "Filename."]
        Filename,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
    }
    impl ListUpdateRequestThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateRequestThreatEntryType::Cert => "CERT",
                ListUpdateRequestThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ListUpdateRequestThreatEntryType::Executable => "EXECUTABLE",
                ListUpdateRequestThreatEntryType::Filename => "FILENAME",
                ListUpdateRequestThreatEntryType::IpRange => "IP_RANGE",
                ListUpdateRequestThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ListUpdateRequestThreatEntryType::Url => "URL",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateRequestThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateRequestThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateRequestThreatEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CERT" => ListUpdateRequestThreatEntryType::Cert,
                "CHROME_EXTENSION" => ListUpdateRequestThreatEntryType::ChromeExtension,
                "EXECUTABLE" => ListUpdateRequestThreatEntryType::Executable,
                "FILENAME" => ListUpdateRequestThreatEntryType::Filename,
                "IP_RANGE" => ListUpdateRequestThreatEntryType::IpRange,
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ListUpdateRequestThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ListUpdateRequestThreatEntryType::Url,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListUpdateRequestThreatEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateRequestThreatEntryType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateRequestThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
    }
    impl ListUpdateRequestThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateRequestThreatType::ApiAbuse => "API_ABUSE",
                ListUpdateRequestThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ListUpdateRequestThreatType::ClientIncident => "CLIENT_INCIDENT",
                ListUpdateRequestThreatType::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ListUpdateRequestThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ListUpdateRequestThreatType::CsdWhitelist => "CSD_WHITELIST",
                ListUpdateRequestThreatType::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
                ListUpdateRequestThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ListUpdateRequestThreatType::Malware => "MALWARE",
                ListUpdateRequestThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ListUpdateRequestThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ListUpdateRequestThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ListUpdateRequestThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ListUpdateRequestThreatType::Suspicious => "SUSPICIOUS",
                ListUpdateRequestThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ListUpdateRequestThreatType::TrickToBill => "TRICK_TO_BILL",
                ListUpdateRequestThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateRequestThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateRequestThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateRequestThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => ListUpdateRequestThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => ListUpdateRequestThreatType::ApkMalwareOffline,
                "CLIENT_INCIDENT" => ListUpdateRequestThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ListUpdateRequestThreatType::ClientIncidentWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ListUpdateRequestThreatType::CsdDownloadWhitelist,
                "CSD_WHITELIST" => ListUpdateRequestThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => ListUpdateRequestThreatType::HighConfidenceAllowlist,
                "MALICIOUS_BINARY" => ListUpdateRequestThreatType::MaliciousBinary,
                "MALWARE" => ListUpdateRequestThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ListUpdateRequestThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => ListUpdateRequestThreatType::SocialEngineering,
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ListUpdateRequestThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => ListUpdateRequestThreatType::SubresourceFilter,
                "SUSPICIOUS" => ListUpdateRequestThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => ListUpdateRequestThreatType::ThreatTypeUnspecified,
                "TRICK_TO_BILL" => ListUpdateRequestThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => ListUpdateRequestThreatType::UnwantedSoftware,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListUpdateRequestThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateRequestThreatType {
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
    impl ::google_field_selector::FieldSelector for ListUpdateResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponsePlatformType {
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to Chrome."]
        Chrome,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
    }
    impl ListUpdateResponsePlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponsePlatformType::AllPlatforms => "ALL_PLATFORMS",
                ListUpdateResponsePlatformType::Android => "ANDROID",
                ListUpdateResponsePlatformType::AnyPlatform => "ANY_PLATFORM",
                ListUpdateResponsePlatformType::Chrome => "CHROME",
                ListUpdateResponsePlatformType::Ios => "IOS",
                ListUpdateResponsePlatformType::Linux => "LINUX",
                ListUpdateResponsePlatformType::Osx => "OSX",
                ListUpdateResponsePlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ListUpdateResponsePlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponsePlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponsePlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponsePlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => ListUpdateResponsePlatformType::AllPlatforms,
                "ANDROID" => ListUpdateResponsePlatformType::Android,
                "ANY_PLATFORM" => ListUpdateResponsePlatformType::AnyPlatform,
                "CHROME" => ListUpdateResponsePlatformType::Chrome,
                "IOS" => ListUpdateResponsePlatformType::Ios,
                "LINUX" => ListUpdateResponsePlatformType::Linux,
                "OSX" => ListUpdateResponsePlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ListUpdateResponsePlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => ListUpdateResponsePlatformType::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListUpdateResponsePlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateResponsePlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponseResponseType {
        #[doc = "Full updates replace the client's entire local database. This means\nthat either the client was seriously out-of-date or the client is\nbelieved to be corrupt."]
        FullUpdate,
        #[doc = "Partial updates are applied to the client's existing local database."]
        PartialUpdate,
        #[doc = "Unknown."]
        ResponseTypeUnspecified,
    }
    impl ListUpdateResponseResponseType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponseResponseType::FullUpdate => "FULL_UPDATE",
                ListUpdateResponseResponseType::PartialUpdate => "PARTIAL_UPDATE",
                ListUpdateResponseResponseType::ResponseTypeUnspecified => {
                    "RESPONSE_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponseResponseType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponseResponseType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponseResponseType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULL_UPDATE" => ListUpdateResponseResponseType::FullUpdate,
                "PARTIAL_UPDATE" => ListUpdateResponseResponseType::PartialUpdate,
                "RESPONSE_TYPE_UNSPECIFIED" => {
                    ListUpdateResponseResponseType::ResponseTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for ListUpdateResponseResponseType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateResponseResponseType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponseThreatEntryType {
        #[doc = "CERT"]
        Cert,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "An executable program."]
        Executable,
        #[doc = "Filename."]
        Filename,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
    }
    impl ListUpdateResponseThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponseThreatEntryType::Cert => "CERT",
                ListUpdateResponseThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ListUpdateResponseThreatEntryType::Executable => "EXECUTABLE",
                ListUpdateResponseThreatEntryType::Filename => "FILENAME",
                ListUpdateResponseThreatEntryType::IpRange => "IP_RANGE",
                ListUpdateResponseThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ListUpdateResponseThreatEntryType::Url => "URL",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponseThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponseThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponseThreatEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CERT" => ListUpdateResponseThreatEntryType::Cert,
                "CHROME_EXTENSION" => ListUpdateResponseThreatEntryType::ChromeExtension,
                "EXECUTABLE" => ListUpdateResponseThreatEntryType::Executable,
                "FILENAME" => ListUpdateResponseThreatEntryType::Filename,
                "IP_RANGE" => ListUpdateResponseThreatEntryType::IpRange,
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ListUpdateResponseThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ListUpdateResponseThreatEntryType::Url,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListUpdateResponseThreatEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateResponseThreatEntryType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListUpdateResponseThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
    }
    impl ListUpdateResponseThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListUpdateResponseThreatType::ApiAbuse => "API_ABUSE",
                ListUpdateResponseThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ListUpdateResponseThreatType::ClientIncident => "CLIENT_INCIDENT",
                ListUpdateResponseThreatType::ClientIncidentWhitelist => {
                    "CLIENT_INCIDENT_WHITELIST"
                }
                ListUpdateResponseThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ListUpdateResponseThreatType::CsdWhitelist => "CSD_WHITELIST",
                ListUpdateResponseThreatType::HighConfidenceAllowlist => {
                    "HIGH_CONFIDENCE_ALLOWLIST"
                }
                ListUpdateResponseThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ListUpdateResponseThreatType::Malware => "MALWARE",
                ListUpdateResponseThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ListUpdateResponseThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ListUpdateResponseThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ListUpdateResponseThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ListUpdateResponseThreatType::Suspicious => "SUSPICIOUS",
                ListUpdateResponseThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ListUpdateResponseThreatType::TrickToBill => "TRICK_TO_BILL",
                ListUpdateResponseThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
    }
    impl ::std::fmt::Display for ListUpdateResponseThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListUpdateResponseThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListUpdateResponseThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => ListUpdateResponseThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => ListUpdateResponseThreatType::ApkMalwareOffline,
                "CLIENT_INCIDENT" => ListUpdateResponseThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => {
                    ListUpdateResponseThreatType::ClientIncidentWhitelist
                }
                "CSD_DOWNLOAD_WHITELIST" => ListUpdateResponseThreatType::CsdDownloadWhitelist,
                "CSD_WHITELIST" => ListUpdateResponseThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    ListUpdateResponseThreatType::HighConfidenceAllowlist
                }
                "MALICIOUS_BINARY" => ListUpdateResponseThreatType::MaliciousBinary,
                "MALWARE" => ListUpdateResponseThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ListUpdateResponseThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => ListUpdateResponseThreatType::SocialEngineering,
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ListUpdateResponseThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => ListUpdateResponseThreatType::SubresourceFilter,
                "SUSPICIOUS" => ListUpdateResponseThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => ListUpdateResponseThreatType::ThreatTypeUnspecified,
                "TRICK_TO_BILL" => ListUpdateResponseThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => ListUpdateResponseThreatType::UnwantedSoftware,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListUpdateResponseThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUpdateResponseThreatType {
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
    pub struct MetadataEntry {
        #[doc = "The metadata entry key. For JSON requests, the key is base64-encoded."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The metadata entry value. For JSON requests, the value is base64-encoded."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for MetadataEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetadataEntry {
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
    pub struct RawHashes {
        #[doc = "The number of bytes for each prefix encoded below.  This field can be\nanywhere from 4 (shortest prefix) to 32 (full SHA256 hash)."]
        #[serde(rename = "prefixSize", default)]
        pub prefix_size: ::std::option::Option<i32>,
        #[doc = "The hashes, in binary format, concatenated into one long string. Hashes are\nsorted in lexicographic order. For JSON API users, hashes are\nbase64-encoded."]
        #[serde(rename = "rawHashes", default)]
        pub raw_hashes: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for RawHashes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RawHashes {
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
    pub struct RawIndices {
        #[doc = "The indices to remove from a lexicographically-sorted local list."]
        #[serde(rename = "indices", default)]
        pub indices: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for RawIndices {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RawIndices {
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
    impl ::google_field_selector::FieldSelector for RiceDeltaEncoding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RiceDeltaEncoding {
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
    impl ::google_field_selector::FieldSelector for ThreatEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatEntry {
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
    pub struct ThreatEntryMetadata {
        #[doc = "The metadata entries."]
        #[serde(rename = "entries", default)]
        pub entries: ::std::option::Option<Vec<crate::schemas::MetadataEntry>>,
    }
    impl ::google_field_selector::FieldSelector for ThreatEntryMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatEntryMetadata {
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
    impl ::google_field_selector::FieldSelector for ThreatEntrySet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatEntrySet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatEntrySetCompressionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatEntrySetCompressionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
    impl ::google_field_selector::FieldSelector for ThreatEntrySetCompressionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatEntrySetCompressionType {
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
    impl ::google_field_selector::FieldSelector for ThreatHit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatHit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatHitPlatformType {
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to Chrome."]
        Chrome,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
    }
    impl ThreatHitPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatHitPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ThreatHitPlatformType::Android => "ANDROID",
                ThreatHitPlatformType::AnyPlatform => "ANY_PLATFORM",
                ThreatHitPlatformType::Chrome => "CHROME",
                ThreatHitPlatformType::Ios => "IOS",
                ThreatHitPlatformType::Linux => "LINUX",
                ThreatHitPlatformType::Osx => "OSX",
                ThreatHitPlatformType::PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED",
                ThreatHitPlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::fmt::Display for ThreatHitPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatHitPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatHitPlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => ThreatHitPlatformType::AllPlatforms,
                "ANDROID" => ThreatHitPlatformType::Android,
                "ANY_PLATFORM" => ThreatHitPlatformType::AnyPlatform,
                "CHROME" => ThreatHitPlatformType::Chrome,
                "IOS" => ThreatHitPlatformType::Ios,
                "LINUX" => ThreatHitPlatformType::Linux,
                "OSX" => ThreatHitPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => ThreatHitPlatformType::PlatformTypeUnspecified,
                "WINDOWS" => ThreatHitPlatformType::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatHitPlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatHitPlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatHitThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
    }
    impl ThreatHitThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatHitThreatType::ApiAbuse => "API_ABUSE",
                ThreatHitThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatHitThreatType::ClientIncident => "CLIENT_INCIDENT",
                ThreatHitThreatType::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ThreatHitThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatHitThreatType::CsdWhitelist => "CSD_WHITELIST",
                ThreatHitThreatType::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
                ThreatHitThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatHitThreatType::Malware => "MALWARE",
                ThreatHitThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatHitThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatHitThreatType::SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL",
                ThreatHitThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatHitThreatType::Suspicious => "SUSPICIOUS",
                ThreatHitThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatHitThreatType::TrickToBill => "TRICK_TO_BILL",
                ThreatHitThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
    }
    impl ::std::fmt::Display for ThreatHitThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatHitThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatHitThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => ThreatHitThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => ThreatHitThreatType::ApkMalwareOffline,
                "CLIENT_INCIDENT" => ThreatHitThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ThreatHitThreatType::ClientIncidentWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatHitThreatType::CsdDownloadWhitelist,
                "CSD_WHITELIST" => ThreatHitThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => ThreatHitThreatType::HighConfidenceAllowlist,
                "MALICIOUS_BINARY" => ThreatHitThreatType::MaliciousBinary,
                "MALWARE" => ThreatHitThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatHitThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => ThreatHitThreatType::SocialEngineering,
                "SOCIAL_ENGINEERING_INTERNAL" => ThreatHitThreatType::SocialEngineeringInternal,
                "SUBRESOURCE_FILTER" => ThreatHitThreatType::SubresourceFilter,
                "SUSPICIOUS" => ThreatHitThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => ThreatHitThreatType::ThreatTypeUnspecified,
                "TRICK_TO_BILL" => ThreatHitThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => ThreatHitThreatType::UnwantedSoftware,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatHitThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatHitThreatType {
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
    impl ::google_field_selector::FieldSelector for ThreatInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatInfoPlatformTypesItems {
        AllPlatforms,
        Android,
        AnyPlatform,
        Chrome,
        Ios,
        Linux,
        Osx,
        PlatformTypeUnspecified,
        Windows,
    }
    impl ThreatInfoPlatformTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatInfoPlatformTypesItems::AllPlatforms => "ALL_PLATFORMS",
                ThreatInfoPlatformTypesItems::Android => "ANDROID",
                ThreatInfoPlatformTypesItems::AnyPlatform => "ANY_PLATFORM",
                ThreatInfoPlatformTypesItems::Chrome => "CHROME",
                ThreatInfoPlatformTypesItems::Ios => "IOS",
                ThreatInfoPlatformTypesItems::Linux => "LINUX",
                ThreatInfoPlatformTypesItems::Osx => "OSX",
                ThreatInfoPlatformTypesItems::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ThreatInfoPlatformTypesItems::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::fmt::Display for ThreatInfoPlatformTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatInfoPlatformTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatInfoPlatformTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => ThreatInfoPlatformTypesItems::AllPlatforms,
                "ANDROID" => ThreatInfoPlatformTypesItems::Android,
                "ANY_PLATFORM" => ThreatInfoPlatformTypesItems::AnyPlatform,
                "CHROME" => ThreatInfoPlatformTypesItems::Chrome,
                "IOS" => ThreatInfoPlatformTypesItems::Ios,
                "LINUX" => ThreatInfoPlatformTypesItems::Linux,
                "OSX" => ThreatInfoPlatformTypesItems::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ThreatInfoPlatformTypesItems::PlatformTypeUnspecified
                }
                "WINDOWS" => ThreatInfoPlatformTypesItems::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatInfoPlatformTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatInfoPlatformTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatInfoThreatEntryTypesItems {
        Cert,
        ChromeExtension,
        Executable,
        Filename,
        IpRange,
        ThreatEntryTypeUnspecified,
        Url,
    }
    impl ThreatInfoThreatEntryTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatInfoThreatEntryTypesItems::Cert => "CERT",
                ThreatInfoThreatEntryTypesItems::ChromeExtension => "CHROME_EXTENSION",
                ThreatInfoThreatEntryTypesItems::Executable => "EXECUTABLE",
                ThreatInfoThreatEntryTypesItems::Filename => "FILENAME",
                ThreatInfoThreatEntryTypesItems::IpRange => "IP_RANGE",
                ThreatInfoThreatEntryTypesItems::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ThreatInfoThreatEntryTypesItems::Url => "URL",
            }
        }
    }
    impl ::std::fmt::Display for ThreatInfoThreatEntryTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatInfoThreatEntryTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatInfoThreatEntryTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CERT" => ThreatInfoThreatEntryTypesItems::Cert,
                "CHROME_EXTENSION" => ThreatInfoThreatEntryTypesItems::ChromeExtension,
                "EXECUTABLE" => ThreatInfoThreatEntryTypesItems::Executable,
                "FILENAME" => ThreatInfoThreatEntryTypesItems::Filename,
                "IP_RANGE" => ThreatInfoThreatEntryTypesItems::IpRange,
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ThreatInfoThreatEntryTypesItems::ThreatEntryTypeUnspecified
                }
                "URL" => ThreatInfoThreatEntryTypesItems::Url,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatInfoThreatEntryTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatInfoThreatEntryTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatInfoThreatTypesItems {
        ApiAbuse,
        ApkMalwareOffline,
        ClientIncident,
        ClientIncidentWhitelist,
        CsdDownloadWhitelist,
        CsdWhitelist,
        HighConfidenceAllowlist,
        MaliciousBinary,
        Malware,
        PotentiallyHarmfulApplication,
        SocialEngineering,
        SocialEngineeringInternal,
        SubresourceFilter,
        Suspicious,
        ThreatTypeUnspecified,
        TrickToBill,
        UnwantedSoftware,
    }
    impl ThreatInfoThreatTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatInfoThreatTypesItems::ApiAbuse => "API_ABUSE",
                ThreatInfoThreatTypesItems::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatInfoThreatTypesItems::ClientIncident => "CLIENT_INCIDENT",
                ThreatInfoThreatTypesItems::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ThreatInfoThreatTypesItems::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatInfoThreatTypesItems::CsdWhitelist => "CSD_WHITELIST",
                ThreatInfoThreatTypesItems::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
                ThreatInfoThreatTypesItems::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatInfoThreatTypesItems::Malware => "MALWARE",
                ThreatInfoThreatTypesItems::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatInfoThreatTypesItems::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatInfoThreatTypesItems::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ThreatInfoThreatTypesItems::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatInfoThreatTypesItems::Suspicious => "SUSPICIOUS",
                ThreatInfoThreatTypesItems::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatInfoThreatTypesItems::TrickToBill => "TRICK_TO_BILL",
                ThreatInfoThreatTypesItems::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
    }
    impl ::std::fmt::Display for ThreatInfoThreatTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatInfoThreatTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatInfoThreatTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => ThreatInfoThreatTypesItems::ApiAbuse,
                "APK_MALWARE_OFFLINE" => ThreatInfoThreatTypesItems::ApkMalwareOffline,
                "CLIENT_INCIDENT" => ThreatInfoThreatTypesItems::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ThreatInfoThreatTypesItems::ClientIncidentWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatInfoThreatTypesItems::CsdDownloadWhitelist,
                "CSD_WHITELIST" => ThreatInfoThreatTypesItems::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => ThreatInfoThreatTypesItems::HighConfidenceAllowlist,
                "MALICIOUS_BINARY" => ThreatInfoThreatTypesItems::MaliciousBinary,
                "MALWARE" => ThreatInfoThreatTypesItems::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatInfoThreatTypesItems::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => ThreatInfoThreatTypesItems::SocialEngineering,
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ThreatInfoThreatTypesItems::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => ThreatInfoThreatTypesItems::SubresourceFilter,
                "SUSPICIOUS" => ThreatInfoThreatTypesItems::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => ThreatInfoThreatTypesItems::ThreatTypeUnspecified,
                "TRICK_TO_BILL" => ThreatInfoThreatTypesItems::TrickToBill,
                "UNWANTED_SOFTWARE" => ThreatInfoThreatTypesItems::UnwantedSoftware,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatInfoThreatTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatInfoThreatTypesItems {
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
    impl ::google_field_selector::FieldSelector for ThreatListDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatListDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatListDescriptorPlatformType {
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to Chrome."]
        Chrome,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
    }
    impl ThreatListDescriptorPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatListDescriptorPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ThreatListDescriptorPlatformType::Android => "ANDROID",
                ThreatListDescriptorPlatformType::AnyPlatform => "ANY_PLATFORM",
                ThreatListDescriptorPlatformType::Chrome => "CHROME",
                ThreatListDescriptorPlatformType::Ios => "IOS",
                ThreatListDescriptorPlatformType::Linux => "LINUX",
                ThreatListDescriptorPlatformType::Osx => "OSX",
                ThreatListDescriptorPlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                ThreatListDescriptorPlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::fmt::Display for ThreatListDescriptorPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatListDescriptorPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatListDescriptorPlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => ThreatListDescriptorPlatformType::AllPlatforms,
                "ANDROID" => ThreatListDescriptorPlatformType::Android,
                "ANY_PLATFORM" => ThreatListDescriptorPlatformType::AnyPlatform,
                "CHROME" => ThreatListDescriptorPlatformType::Chrome,
                "IOS" => ThreatListDescriptorPlatformType::Ios,
                "LINUX" => ThreatListDescriptorPlatformType::Linux,
                "OSX" => ThreatListDescriptorPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    ThreatListDescriptorPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => ThreatListDescriptorPlatformType::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatListDescriptorPlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatListDescriptorPlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatListDescriptorThreatEntryType {
        #[doc = "CERT"]
        Cert,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "An executable program."]
        Executable,
        #[doc = "Filename."]
        Filename,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
    }
    impl ThreatListDescriptorThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatListDescriptorThreatEntryType::Cert => "CERT",
                ThreatListDescriptorThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ThreatListDescriptorThreatEntryType::Executable => "EXECUTABLE",
                ThreatListDescriptorThreatEntryType::Filename => "FILENAME",
                ThreatListDescriptorThreatEntryType::IpRange => "IP_RANGE",
                ThreatListDescriptorThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ThreatListDescriptorThreatEntryType::Url => "URL",
            }
        }
    }
    impl ::std::fmt::Display for ThreatListDescriptorThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatListDescriptorThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatListDescriptorThreatEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CERT" => ThreatListDescriptorThreatEntryType::Cert,
                "CHROME_EXTENSION" => ThreatListDescriptorThreatEntryType::ChromeExtension,
                "EXECUTABLE" => ThreatListDescriptorThreatEntryType::Executable,
                "FILENAME" => ThreatListDescriptorThreatEntryType::Filename,
                "IP_RANGE" => ThreatListDescriptorThreatEntryType::IpRange,
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ThreatListDescriptorThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ThreatListDescriptorThreatEntryType::Url,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatListDescriptorThreatEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatListDescriptorThreatEntryType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatListDescriptorThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
    }
    impl ThreatListDescriptorThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatListDescriptorThreatType::ApiAbuse => "API_ABUSE",
                ThreatListDescriptorThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatListDescriptorThreatType::ClientIncident => "CLIENT_INCIDENT",
                ThreatListDescriptorThreatType::ClientIncidentWhitelist => {
                    "CLIENT_INCIDENT_WHITELIST"
                }
                ThreatListDescriptorThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatListDescriptorThreatType::CsdWhitelist => "CSD_WHITELIST",
                ThreatListDescriptorThreatType::HighConfidenceAllowlist => {
                    "HIGH_CONFIDENCE_ALLOWLIST"
                }
                ThreatListDescriptorThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatListDescriptorThreatType::Malware => "MALWARE",
                ThreatListDescriptorThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatListDescriptorThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatListDescriptorThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                ThreatListDescriptorThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatListDescriptorThreatType::Suspicious => "SUSPICIOUS",
                ThreatListDescriptorThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatListDescriptorThreatType::TrickToBill => "TRICK_TO_BILL",
                ThreatListDescriptorThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
    }
    impl ::std::fmt::Display for ThreatListDescriptorThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatListDescriptorThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatListDescriptorThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => ThreatListDescriptorThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => ThreatListDescriptorThreatType::ApkMalwareOffline,
                "CLIENT_INCIDENT" => ThreatListDescriptorThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => {
                    ThreatListDescriptorThreatType::ClientIncidentWhitelist
                }
                "CSD_DOWNLOAD_WHITELIST" => ThreatListDescriptorThreatType::CsdDownloadWhitelist,
                "CSD_WHITELIST" => ThreatListDescriptorThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    ThreatListDescriptorThreatType::HighConfidenceAllowlist
                }
                "MALICIOUS_BINARY" => ThreatListDescriptorThreatType::MaliciousBinary,
                "MALWARE" => ThreatListDescriptorThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatListDescriptorThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => ThreatListDescriptorThreatType::SocialEngineering,
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    ThreatListDescriptorThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => ThreatListDescriptorThreatType::SubresourceFilter,
                "SUSPICIOUS" => ThreatListDescriptorThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => ThreatListDescriptorThreatType::ThreatTypeUnspecified,
                "TRICK_TO_BILL" => ThreatListDescriptorThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => ThreatListDescriptorThreatType::UnwantedSoftware,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatListDescriptorThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatListDescriptorThreatType {
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
    impl ::google_field_selector::FieldSelector for ThreatMatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatMatch {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatMatchPlatformType {
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[doc = "Threat posed to Android."]
        Android,
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[doc = "Threat posed to Chrome."]
        Chrome,
        #[doc = "Threat posed to iOS."]
        Ios,
        #[doc = "Threat posed to Linux."]
        Linux,
        #[doc = "Threat posed to OS X."]
        Osx,
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[doc = "Threat posed to Windows."]
        Windows,
    }
    impl ThreatMatchPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatMatchPlatformType::AllPlatforms => "ALL_PLATFORMS",
                ThreatMatchPlatformType::Android => "ANDROID",
                ThreatMatchPlatformType::AnyPlatform => "ANY_PLATFORM",
                ThreatMatchPlatformType::Chrome => "CHROME",
                ThreatMatchPlatformType::Ios => "IOS",
                ThreatMatchPlatformType::Linux => "LINUX",
                ThreatMatchPlatformType::Osx => "OSX",
                ThreatMatchPlatformType::PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED",
                ThreatMatchPlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::fmt::Display for ThreatMatchPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatMatchPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatMatchPlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => ThreatMatchPlatformType::AllPlatforms,
                "ANDROID" => ThreatMatchPlatformType::Android,
                "ANY_PLATFORM" => ThreatMatchPlatformType::AnyPlatform,
                "CHROME" => ThreatMatchPlatformType::Chrome,
                "IOS" => ThreatMatchPlatformType::Ios,
                "LINUX" => ThreatMatchPlatformType::Linux,
                "OSX" => ThreatMatchPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => ThreatMatchPlatformType::PlatformTypeUnspecified,
                "WINDOWS" => ThreatMatchPlatformType::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatMatchPlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatMatchPlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatMatchThreatEntryType {
        #[doc = "CERT"]
        Cert,
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[doc = "An executable program."]
        Executable,
        #[doc = "Filename."]
        Filename,
        #[doc = "An IP range."]
        IpRange,
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[doc = "A URL."]
        Url,
    }
    impl ThreatMatchThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatMatchThreatEntryType::Cert => "CERT",
                ThreatMatchThreatEntryType::ChromeExtension => "CHROME_EXTENSION",
                ThreatMatchThreatEntryType::Executable => "EXECUTABLE",
                ThreatMatchThreatEntryType::Filename => "FILENAME",
                ThreatMatchThreatEntryType::IpRange => "IP_RANGE",
                ThreatMatchThreatEntryType::ThreatEntryTypeUnspecified => {
                    "THREAT_ENTRY_TYPE_UNSPECIFIED"
                }
                ThreatMatchThreatEntryType::Url => "URL",
            }
        }
    }
    impl ::std::fmt::Display for ThreatMatchThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatMatchThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatMatchThreatEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CERT" => ThreatMatchThreatEntryType::Cert,
                "CHROME_EXTENSION" => ThreatMatchThreatEntryType::ChromeExtension,
                "EXECUTABLE" => ThreatMatchThreatEntryType::Executable,
                "FILENAME" => ThreatMatchThreatEntryType::Filename,
                "IP_RANGE" => ThreatMatchThreatEntryType::IpRange,
                "THREAT_ENTRY_TYPE_UNSPECIFIED" => {
                    ThreatMatchThreatEntryType::ThreatEntryTypeUnspecified
                }
                "URL" => ThreatMatchThreatEntryType::Url,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatMatchThreatEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatMatchThreatEntryType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatMatchThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats.\nThis enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[doc = "Malware threat type."]
        Malware,
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial\nwill not be shown for patterns from this list."]
        SubresourceFilter,
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
    }
    impl ThreatMatchThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatMatchThreatType::ApiAbuse => "API_ABUSE",
                ThreatMatchThreatType::ApkMalwareOffline => "APK_MALWARE_OFFLINE",
                ThreatMatchThreatType::ClientIncident => "CLIENT_INCIDENT",
                ThreatMatchThreatType::ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST",
                ThreatMatchThreatType::CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST",
                ThreatMatchThreatType::CsdWhitelist => "CSD_WHITELIST",
                ThreatMatchThreatType::HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST",
                ThreatMatchThreatType::MaliciousBinary => "MALICIOUS_BINARY",
                ThreatMatchThreatType::Malware => "MALWARE",
                ThreatMatchThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                ThreatMatchThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
                ThreatMatchThreatType::SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL",
                ThreatMatchThreatType::SubresourceFilter => "SUBRESOURCE_FILTER",
                ThreatMatchThreatType::Suspicious => "SUSPICIOUS",
                ThreatMatchThreatType::ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED",
                ThreatMatchThreatType::TrickToBill => "TRICK_TO_BILL",
                ThreatMatchThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
    }
    impl ::std::fmt::Display for ThreatMatchThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatMatchThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatMatchThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => ThreatMatchThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => ThreatMatchThreatType::ApkMalwareOffline,
                "CLIENT_INCIDENT" => ThreatMatchThreatType::ClientIncident,
                "CLIENT_INCIDENT_WHITELIST" => ThreatMatchThreatType::ClientIncidentWhitelist,
                "CSD_DOWNLOAD_WHITELIST" => ThreatMatchThreatType::CsdDownloadWhitelist,
                "CSD_WHITELIST" => ThreatMatchThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => ThreatMatchThreatType::HighConfidenceAllowlist,
                "MALICIOUS_BINARY" => ThreatMatchThreatType::MaliciousBinary,
                "MALWARE" => ThreatMatchThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    ThreatMatchThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => ThreatMatchThreatType::SocialEngineering,
                "SOCIAL_ENGINEERING_INTERNAL" => ThreatMatchThreatType::SocialEngineeringInternal,
                "SUBRESOURCE_FILTER" => ThreatMatchThreatType::SubresourceFilter,
                "SUSPICIOUS" => ThreatMatchThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => ThreatMatchThreatType::ThreatTypeUnspecified,
                "TRICK_TO_BILL" => ThreatMatchThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => ThreatMatchThreatType::UnwantedSoftware,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatMatchThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatMatchThreatType {
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
    impl ::google_field_selector::FieldSelector for ThreatSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThreatSourceType {
        #[doc = "The URL that matched the threat list (for which GetFullHash returned a\nvalid hash)."]
        MatchingUrl,
        #[doc = "A redirect URL that was fetched before hitting the final TAB_URL."]
        TabRedirect,
        #[doc = "A resource loaded within the final TAB_URL."]
        TabResource,
        #[doc = "The final top-level URL of the tab that the client was browsing when the\nmatch occurred."]
        TabUrl,
        #[doc = "Unknown."]
        ThreatSourceTypeUnspecified,
    }
    impl ThreatSourceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThreatSourceType::MatchingUrl => "MATCHING_URL",
                ThreatSourceType::TabRedirect => "TAB_REDIRECT",
                ThreatSourceType::TabResource => "TAB_RESOURCE",
                ThreatSourceType::TabUrl => "TAB_URL",
                ThreatSourceType::ThreatSourceTypeUnspecified => "THREAT_SOURCE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for ThreatSourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThreatSourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreatSourceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MATCHING_URL" => ThreatSourceType::MatchingUrl,
                "TAB_REDIRECT" => ThreatSourceType::TabRedirect,
                "TAB_RESOURCE" => ThreatSourceType::TabResource,
                "TAB_URL" => ThreatSourceType::TabUrl,
                "THREAT_SOURCE_TYPE_UNSPECIFIED" => ThreatSourceType::ThreatSourceTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThreatSourceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThreatSourceType {
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
    pub struct UserInfo {
        #[doc = "The UN M.49 region code associated with the user's location."]
        #[serde(rename = "regionCode", default)]
        pub region_code: ::std::option::Option<String>,
        #[doc = "Unique user identifier defined by the client."]
        #[serde(rename = "userId", default)]
        pub user_id: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for UserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserInfo {
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
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the encoded_full_hashes resource"]
    pub fn encoded_full_hashes(
        &self,
    ) -> crate::resources::encoded_full_hashes::EncodedFullHashesActions {
        crate::resources::encoded_full_hashes::EncodedFullHashesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the encoded_updates resource"]
    pub fn encoded_updates(&self) -> crate::resources::encoded_updates::EncodedUpdatesActions {
        crate::resources::encoded_updates::EncodedUpdatesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the full_hashes resource"]
    pub fn full_hashes(&self) -> crate::resources::full_hashes::FullHashesActions {
        crate::resources::full_hashes::FullHashesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the threat_hits resource"]
    pub fn threat_hits(&self) -> crate::resources::threat_hits::ThreatHitsActions {
        crate::resources::threat_hits::ThreatHitsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the threat_list_updates resource"]
    pub fn threat_list_updates(
        &self,
    ) -> crate::resources::threat_list_updates::ThreatListUpdatesActions {
        crate::resources::threat_list_updates::ThreatListUpdatesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the threat_lists resource"]
    pub fn threat_lists(&self) -> crate::resources::threat_lists::ThreatListsActions {
        crate::resources::threat_lists::ThreatListsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the threat_matches resource"]
    pub fn threat_matches(&self) -> crate::resources::threat_matches::ThreatMatchesActions {
        crate::resources::threat_matches::ThreatMatchesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod encoded_full_hashes {
        pub mod params {}
        pub struct EncodedFullHashesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> EncodedFullHashesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn get(&self, encoded_request: impl Into<Vec<u8>>) -> GetRequestBuilder {
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
                    encoded_request: encoded_request.into().into(),
                    client_id: None,
                    client_version: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> GetRequestBuilder<'a> {
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::FindFullHashesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FindFullHashesResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
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
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod encoded_updates {
        pub mod params {}
        pub struct EncodedUpdatesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> EncodedUpdatesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn get(&self, encoded_request: impl Into<Vec<u8>>) -> GetRequestBuilder {
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
                    encoded_request: encoded_request.into().into(),
                    client_id: None,
                    client_version: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> GetRequestBuilder<'a> {
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
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
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod full_hashes {
        pub mod params {}
        pub struct FullHashesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> FullHashesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Finds the full hashes that match the requested hash prefixes."]
            pub fn find(
                &self,
                request: crate::schemas::FindFullHashesRequest,
            ) -> FindRequestBuilder {
                FindRequestBuilder {
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
        #[derive(Debug, Clone)]
        pub struct FindRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> FindRequestBuilder<'a> {
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::FindFullHashesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FindFullHashesResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/fullHashes:find");
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
    pub mod threat_hits {
        pub mod params {}
        pub struct ThreatHitsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatHitsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Reports a Safe Browsing threat list hit to Google. Only projects with\nTRUSTED_REPORTER visibility can use this method."]
            pub fn create(&self, request: crate::schemas::ThreatHit) -> CreateRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatHits");
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
    pub mod threat_list_updates {
        pub mod params {}
        pub struct ThreatListUpdatesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatListUpdatesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Fetches the most recent threat list updates. A client can request updates\nfor multiple lists at once."]
            pub fn fetch(
                &self,
                request: crate::schemas::FetchThreatListUpdatesRequest,
            ) -> FetchRequestBuilder {
                FetchRequestBuilder {
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
        #[derive(Debug, Clone)]
        pub struct FetchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> FetchRequestBuilder<'a> {
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FetchThreatListUpdatesResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatListUpdates:fetch");
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
    pub mod threat_lists {
        pub mod params {}
        pub struct ThreatListsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatListsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Lists the Safe Browsing threat lists available for download."]
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::ListThreatListsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListThreatListsResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatLists");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod threat_matches {
        pub mod params {}
        pub struct ThreatMatchesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatMatchesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Finds the threat entries that match the Safe Browsing lists."]
            pub fn find(
                &self,
                request: crate::schemas::FindThreatMatchesRequest,
            ) -> FindRequestBuilder {
                FindRequestBuilder {
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
        #[derive(Debug, Clone)]
        pub struct FindRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> FindRequestBuilder<'a> {
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
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
            ) -> Result<crate::schemas::FindThreatMatchesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FindThreatMatchesResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatMatches:find");
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
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest(err) => err
                .get_ref()
                .and_then(|err| err.downcast_ref::<::serde_json::Error>()),
            Error::Other(_) => None,
        }
    }
}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::Reqwest(err)
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
// Bytes in google apis are represented as urlsafe base64 encoded strings.
// This defines a Bytes type that is a simple wrapper around a Vec<u8> used
// internally to handle byte fields in google apis.
pub mod bytes {
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
