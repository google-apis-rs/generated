#![doc = "# Resources and Methods\n    * [encoded_full_hashes](resources/encoded_full_hashes/struct.EncodedFullHashesActions.html)\n      * [*get*](resources/encoded_full_hashes/struct.GetRequestBuilder.html)\n    * [encoded_updates](resources/encoded_updates/struct.EncodedUpdatesActions.html)\n      * [*get*](resources/encoded_updates/struct.GetRequestBuilder.html)\n    * [full_hashes](resources/full_hashes/struct.FullHashesActions.html)\n      * [*find*](resources/full_hashes/struct.FindRequestBuilder.html)\n    * [threat_hits](resources/threat_hits/struct.ThreatHitsActions.html)\n      * [*create*](resources/threat_hits/struct.CreateRequestBuilder.html)\n    * [threat_list_updates](resources/threat_list_updates/struct.ThreatListUpdatesActions.html)\n      * [*fetch*](resources/threat_list_updates/struct.FetchRequestBuilder.html)\n    * [threat_lists](resources/threat_lists/struct.ThreatListsActions.html)\n      * [*list*](resources/threat_lists/struct.ListRequestBuilder.html)\n    * [threat_matches](resources/threat_matches/struct.ThreatMatchesActions.html)\n      * [*find*](resources/threat_matches/struct.FindRequestBuilder.html)\n"]
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleProtobufEmpty {}
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
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
    pub struct GoogleSecuritySafebrowsingV4Checksum {
        #[doc = "The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database."]
        #[serde(
            rename = "sha256",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_256: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4Checksum {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4Checksum {
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
    pub struct GoogleSecuritySafebrowsingV4ClientInfo {
        #[doc = "A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "The version of the client implementation."]
        #[serde(
            rename = "clientVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ClientInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ClientInfo {
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
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest { # [ doc = "The client metadata." ] # [ serde ( rename = "client" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub client : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4ClientInfo > , # [ doc = "The requested threat list updates." ] # [ serde ( rename = "listUpdateRequests" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub list_update_requests : :: std :: option :: Option < Vec < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest
    {
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
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest { # [ doc = "The constraints associated with this request." ] # [ serde ( rename = "constraints" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub constraints : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints > , # [ doc = "The type of platform at risk by entries present in the list." ] # [ serde ( rename = "platformType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub platform_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType > , # [ doc = "The current state of the client for the requested list (the encrypted client state that was received from the last successful list update)." ] # [ serde ( rename = "state" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub state : :: std :: option :: Option < :: google_api_bytes :: Bytes > , # [ doc = "The types of entries present in the list." ] # [ serde ( rename = "threatEntryType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub threat_entry_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType > , # [ doc = "The type of threat posed by entries present in the list." ] # [ serde ( rename = "threatType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub threat_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType {
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
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: AllPlatforms => "ALL_PLATFORMS" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Android => "ANDROID" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: AnyPlatform => "ANY_PLATFORM" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Chrome => "CHROME" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Ios => "IOS" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Linux => "LINUX" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Osx => "OSX" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Windows => "WINDOWS" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType,
            (),
        > {
            Ok ( match s { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Linux , "OSX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Windows , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Linux , "OSX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType :: Windows , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType {
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
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Cert => "CERT" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: ChromeExtension => "CHROME_EXTENSION" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Executable => "EXECUTABLE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Filename => "FILENAME" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: IpRange => "IP_RANGE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: ThreatEntryTypeUnspecified => "THREAT_ENTRY_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Url => "URL" , }
        }
    }
    impl :: std :: convert :: AsRef < str > for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { fn as_ref ( & self ) -> & str { self . as_str ( ) } }
    impl :: std :: str :: FromStr for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { type Err = ( ) ; fn from_str ( s : & str ) -> :: std :: result :: Result < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType , ( ) > { Ok ( match s { "CERT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Url , _ => return Err ( ( ) ) , } ) } }
    impl :: std :: fmt :: Display for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { fn fmt ( & self , f : & mut std :: fmt :: Formatter < '_ > ) -> :: std :: fmt :: Result { f . write_str ( self . as_str ( ) ) } }
    impl :: serde :: Serialize for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { fn serialize < S > ( & self , serializer : S ) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str ( self . as_str ( ) ) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { fn deserialize < D > ( deserializer : D ) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize ( deserializer ) ? ; Ok ( match value { "CERT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType :: Url , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } ) } }
    impl :: google_field_selector :: FieldSelector for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { fn fields ( ) -> Vec < :: google_field_selector :: Field > { Vec :: new ( ) } }
    impl :: google_field_selector :: ToFieldType for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryType { fn field_type ( ) -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
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
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
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
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ApiAbuse => "API_ABUSE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ApkMalwareOffline => "APK_MALWARE_OFFLINE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ClientIncident => "CLIENT_INCIDENT" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: CsdWhitelist => "CSD_WHITELIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: MaliciousBinary => "MALICIOUS_BINARY" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: Malware => "MALWARE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: PotentiallyHarmfulApplication => "POTENTIALLY_HARMFUL_APPLICATION" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SocialEngineering => "SOCIAL_ENGINEERING" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SubresourceFilter => "SUBRESOURCE_FILTER" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: Suspicious => "SUSPICIOUS" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: TrickToBill => "TRICK_TO_BILL" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: UnwantedSoftware => "UNWANTED_SOFTWARE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType,
            (),
        > {
            Ok ( match s { "API_ABUSE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: UnwantedSoftware , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "API_ABUSE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType :: UnwantedSoftware , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatType
    {
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
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints { # [ doc = "A client's physical location, expressed as a ISO 31166-1 alpha-2 region code." ] # [ serde ( rename = "deviceLocation" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub device_location : :: std :: option :: Option < String > , # [ doc = "Requests the lists for a specific language. Expects ISO 639 alpha-2 format." ] # [ serde ( rename = "language" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub language : :: std :: option :: Option < String > , # [ doc = "Sets the maximum number of entries that the client is willing to have in the local database for the specified list. This should be a power of 2 between 2**10 and 2**20. If zero, no database size limit is set." ] # [ serde ( rename = "maxDatabaseEntries" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub max_database_entries : :: std :: option :: Option < i32 > , # [ doc = "The maximum size in number of entries. The update will not contain more entries than this value. This should be a power of 2 between 2**10 and 2**20. If zero, no update size limit is set." ] # [ serde ( rename = "maxUpdateEntries" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub max_update_entries : :: std :: option :: Option < i32 > , # [ doc = "Requests the list for a specific geographic location. If not set the server may pick that value based on the user's IP address. Expects ISO 3166-1 alpha-2 format." ] # [ serde ( rename = "region" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub region : :: std :: option :: Option < String > , # [ doc = "The compression types supported by the client." ] # [ serde ( rename = "supportedCompressions" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub supported_compressions : :: std :: option :: Option < Vec < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems
    {
        #[doc = "Unknown."]
        CompressionTypeUnspecified,
        #[doc = "Raw, uncompressed data."]
        Raw,
        #[doc = "Rice-Golomb encoded data."]
        Rice,
    }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { pub fn as_str ( self ) -> & 'static str { match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: CompressionTypeUnspecified => "COMPRESSION_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: Raw => "RAW" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: Rice => "RICE" , } } }
    impl :: std :: convert :: AsRef < str > for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { fn as_ref ( & self ) -> & str { self . as_str ( ) } }
    impl :: std :: str :: FromStr for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { type Err = ( ) ; fn from_str ( s : & str ) -> :: std :: result :: Result < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems , ( ) > { Ok ( match s { "COMPRESSION_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: CompressionTypeUnspecified , "RAW" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: Raw , "RICE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: Rice , _ => return Err ( ( ) ) , } ) } }
    impl :: std :: fmt :: Display for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { fn fmt ( & self , f : & mut std :: fmt :: Formatter < '_ > ) -> :: std :: fmt :: Result { f . write_str ( self . as_str ( ) ) } }
    impl :: serde :: Serialize for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { fn serialize < S > ( & self , serializer : S ) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str ( self . as_str ( ) ) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { fn deserialize < D > ( deserializer : D ) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize ( deserializer ) ? ; Ok ( match value { "COMPRESSION_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: CompressionTypeUnspecified , "RAW" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: Raw , "RICE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems :: Rice , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } ) } }
    impl :: google_field_selector :: FieldSelector for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { fn fields ( ) -> Vec < :: google_field_selector :: Field > { Vec :: new ( ) } }
    impl :: google_field_selector :: ToFieldType for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsItems { fn field_type ( ) -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
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
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse { # [ doc = "The list updates requested by the clients. The number of responses here may be less than the number of requests sent by clients. This is the case, for example, if the server has no updates for a particular list." ] # [ serde ( rename = "listUpdateResponses" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub list_update_responses : :: std :: option :: Option < Vec < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse > > , # [ doc = "The minimum duration the client must wait before issuing any update request. If this field is not set clients may update as soon as they want." ] # [ serde ( rename = "minimumWaitDuration" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub minimum_wait_duration : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse
    {
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
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse { # [ doc = "A set of entries to add to a local threat type's list. Repeated to allow for a combination of compressed and raw data to be sent in a single response." ] # [ serde ( rename = "additions" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub additions : :: std :: option :: Option < Vec < crate :: schemas :: GoogleSecuritySafebrowsingV4ThreatEntrySet > > , # [ doc = "The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided update. If the client state doesn't match the expected state, the client must disregard this update and retry later." ] # [ serde ( rename = "checksum" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub checksum : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4Checksum > , # [ doc = "The new client state, in encrypted format. Opaque to clients." ] # [ serde ( rename = "newClientState" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub new_client_state : :: std :: option :: Option < :: google_api_bytes :: Bytes > , # [ doc = "The platform type for which data is returned." ] # [ serde ( rename = "platformType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub platform_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType > , # [ doc = "A set of entries to remove from a local threat type's list. In practice, this field is empty or contains exactly one ThreatEntrySet." ] # [ serde ( rename = "removals" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub removals : :: std :: option :: Option < Vec < crate :: schemas :: GoogleSecuritySafebrowsingV4ThreatEntrySet > > , # [ doc = "The type of response. This may indicate that an action is required by the client when the response is received." ] # [ serde ( rename = "responseType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub response_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType > , # [ doc = "The format of the threats." ] # [ serde ( rename = "threatEntryType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub threat_entry_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType > , # [ doc = "The threat type for which data is returned." ] # [ serde ( rename = "threatType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub threat_type : :: std :: option :: Option < crate :: schemas :: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType {
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
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: AllPlatforms => "ALL_PLATFORMS" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Android => "ANDROID" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: AnyPlatform => "ANY_PLATFORM" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Chrome => "CHROME" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Ios => "IOS" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Linux => "LINUX" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Osx => "OSX" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Windows => "WINDOWS" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        type Err = ();        fn from_str ( s : & str ) -> :: std :: result :: Result < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType , ( ) >{
            Ok ( match s { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Linux , "OSX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Windows , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Linux , "OSX" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType :: Windows , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType {
        #[doc = "Full updates replace the client's entire local database. This means that either the client was seriously out-of-date or the client is believed to be corrupt."]
        FullUpdate,
        #[doc = "Partial updates are applied to the client's existing local database."]
        PartialUpdate,
        #[doc = "Unknown."]
        ResponseTypeUnspecified,
    }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: FullUpdate => "FULL_UPDATE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: PartialUpdate => "PARTIAL_UPDATE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: ResponseTypeUnspecified => "RESPONSE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        type Err = ();        fn from_str ( s : & str ) -> :: std :: result :: Result < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType , ( ) >{
            Ok ( match s { "FULL_UPDATE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: FullUpdate , "PARTIAL_UPDATE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: PartialUpdate , "RESPONSE_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: ResponseTypeUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FULL_UPDATE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: FullUpdate , "PARTIAL_UPDATE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: PartialUpdate , "RESPONSE_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType :: ResponseTypeUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType
    {
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
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Cert => "CERT" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: ChromeExtension => "CHROME_EXTENSION" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Executable => "EXECUTABLE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Filename => "FILENAME" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: IpRange => "IP_RANGE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: ThreatEntryTypeUnspecified => "THREAT_ENTRY_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Url => "URL" , }
        }
    }
    impl :: std :: convert :: AsRef < str > for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { fn as_ref ( & self ) -> & str { self . as_str ( ) } }
    impl :: std :: str :: FromStr for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { type Err = ( ) ; fn from_str ( s : & str ) -> :: std :: result :: Result < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType , ( ) > { Ok ( match s { "CERT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Url , _ => return Err ( ( ) ) , } ) } }
    impl :: std :: fmt :: Display for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { fn fmt ( & self , f : & mut std :: fmt :: Formatter < '_ > ) -> :: std :: fmt :: Result { f . write_str ( self . as_str ( ) ) } }
    impl :: serde :: Serialize for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { fn serialize < S > ( & self , serializer : S ) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str ( self . as_str ( ) ) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { fn deserialize < D > ( deserializer : D ) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize ( deserializer ) ? ; Ok ( match value { "CERT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType :: Url , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } ) } }
    impl :: google_field_selector :: FieldSelector for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { fn fields ( ) -> Vec < :: google_field_selector :: Field > { Vec :: new ( ) } }
    impl :: google_field_selector :: ToFieldType for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryType { fn field_type ( ) -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
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
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
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
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ApiAbuse => "API_ABUSE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ApkMalwareOffline => "APK_MALWARE_OFFLINE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ClientIncident => "CLIENT_INCIDENT" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: CsdWhitelist => "CSD_WHITELIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: MaliciousBinary => "MALICIOUS_BINARY" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: Malware => "MALWARE" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: PotentiallyHarmfulApplication => "POTENTIALLY_HARMFUL_APPLICATION" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SocialEngineering => "SOCIAL_ENGINEERING" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SubresourceFilter => "SUBRESOURCE_FILTER" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: Suspicious => "SUSPICIOUS" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: TrickToBill => "TRICK_TO_BILL" , GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: UnwantedSoftware => "UNWANTED_SOFTWARE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType,
            (),
        > {
            Ok ( match s { "API_ABUSE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: UnwantedSoftware , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "API_ABUSE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType :: UnwantedSoftware , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatType
    {
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
    pub struct GoogleSecuritySafebrowsingV4FindFullHashesRequest {
        #[doc = "Client metadata associated with callers of higher-level APIs built on top of the client's implementation."]
        #[serde(
            rename = "apiClient",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_client:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ClientInfo>,
        #[doc = "The client metadata."]
        #[serde(
            rename = "client",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client: ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ClientInfo>,
        #[doc = "The current client states for each of the client's local threat lists."]
        #[serde(
            rename = "clientStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_states: ::std::option::Option<Vec<::google_api_bytes::Bytes>>,
        #[doc = "The lists and hashes to be checked."]
        #[serde(
            rename = "threatInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_info:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4FindFullHashesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4FindFullHashesRequest {
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
    pub struct GoogleSecuritySafebrowsingV4FindFullHashesResponse {
        #[doc = "The full hashes that matched the requested prefixes."]
        #[serde(
            rename = "matches",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matches:
            ::std::option::Option<Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatMatch>>,
        #[doc = "The minimum duration the client must wait before issuing any find hashes request. If this field is not set, clients can issue a request as soon as they want."]
        #[serde(
            rename = "minimumWaitDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_wait_duration: ::std::option::Option<String>,
        #[doc = "For requested entities that did not match the threat list, how long to cache the response."]
        #[serde(
            rename = "negativeCacheDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negative_cache_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4FindFullHashesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4FindFullHashesResponse {
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
    pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {
        #[doc = "The client metadata."]
        #[serde(
            rename = "client",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client: ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ClientInfo>,
        #[doc = "The lists and entries to be checked for matches."]
        #[serde(
            rename = "threatInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_info:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatInfo>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FindThreatMatchesRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {
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
    pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {
        #[doc = "The threat list matches."]
        #[serde(
            rename = "matches",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matches:
            ::std::option::Option<Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatMatch>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4FindThreatMatchesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4FindThreatMatchesResponse
    {
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
    pub struct GoogleSecuritySafebrowsingV4ListThreatListsResponse {
        #[doc = "The lists available for download by the client."]
        #[serde(
            rename = "threatLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_lists: ::std::option::Option<
            Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatListDescriptor>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ListThreatListsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ListThreatListsResponse {
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
    pub struct GoogleSecuritySafebrowsingV4RawHashes {
        #[doc = "The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash)."]
        #[serde(
            rename = "prefixSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prefix_size: ::std::option::Option<i32>,
        #[doc = "The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded."]
        #[serde(
            rename = "rawHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_hashes: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4RawHashes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4RawHashes {
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
    pub struct GoogleSecuritySafebrowsingV4RawIndices {
        #[doc = "The indices to remove from a lexicographically-sorted local list."]
        #[serde(
            rename = "indices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indices: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4RawIndices {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4RawIndices {
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
    pub struct GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
        #[doc = "The encoded deltas that are encoded using the Golomb-Rice coder."]
        #[serde(
            rename = "encodedData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encoded_data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero."]
        #[serde(
            rename = "firstValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub first_value: ::std::option::Option<i64>,
        #[doc = "The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`."]
        #[serde(
            rename = "numEntries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_entries: ::std::option::Option<i32>,
        #[doc = "The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero."]
        #[serde(
            rename = "riceParameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rice_parameter: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatEntry {
        #[doc = "The digest of an executable in SHA256 format. The API supports both binary and hex digests. For JSON requests, digests are base64-encoded."]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A hash prefix, consisting of the most significant 4-32 bytes of a SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded."]
        #[serde(
            rename = "hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hash: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatEntry {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
        #[doc = "The metadata entries."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries: ::std::option::Option<
            Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {
        #[doc = "The metadata entry key. For JSON requests, the key is base64-encoded."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The metadata entry value. For JSON requests, the value is base64-encoded."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry
    {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatEntrySet {
        #[doc = "The compression type for the entries in this set."]
        #[serde(
            rename = "compressionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compression_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType,
        >,
        #[doc = "The raw SHA256-formatted entries."]
        #[serde(
            rename = "rawHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_hashes:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4RawHashes>,
        #[doc = "The raw removal indices for a local list."]
        #[serde(
            rename = "rawIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_indices:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4RawIndices>,
        #[doc = "The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data."]
        #[serde(
            rename = "riceHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rice_hashes:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4RiceDeltaEncoding>,
        #[doc = "The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data."]
        #[serde(
            rename = "riceIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rice_indices:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4RiceDeltaEncoding>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatEntrySet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatEntrySet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        #[doc = "Unknown."]
        CompressionTypeUnspecified,
        #[doc = "Raw, uncompressed data."]
        Raw,
        #[doc = "Rice-Golomb encoded data."]
        Rice,
    }
    impl GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: CompressionTypeUnspecified => "COMPRESSION_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: Raw => "RAW" , GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: Rice => "RICE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType, ()>
        {
            Ok ( match s { "COMPRESSION_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: CompressionTypeUnspecified , "RAW" => GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: Raw , "RICE" => GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: Rice , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "COMPRESSION_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: CompressionTypeUnspecified , "RAW" => GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: Raw , "RICE" => GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType :: Rice , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionType
    {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatHit {
        #[doc = "Client-reported identification."]
        #[serde(
            rename = "clientInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_info:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ClientInfo>,
        #[doc = "The threat entry responsible for the hit. Full hash should be reported for hash-based hits."]
        #[serde(
            rename = "entry",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry: ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatEntry>,
        #[doc = "The platform type reported."]
        #[serde(
            rename = "platformType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatHitPlatformType,
        >,
        #[doc = "The resources related to the threat hit."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<
            Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatHitThreatSource>,
        >,
        #[doc = "The threat type reported."]
        #[serde(
            rename = "threatType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_type:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatHitThreatType>,
        #[doc = "Details about the user that encountered the threat."]
        #[serde(
            rename = "userInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_info:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatHitUserInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatHit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatHit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
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
    impl GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::AllPlatforms => "ALL_PLATFORMS",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Android => "ANDROID",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::AnyPlatform => "ANY_PLATFORM",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Chrome => "CHROME",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Ios => "IOS",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Linux => "LINUX",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Osx => "OSX",
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatHitPlatformType, ()> {
            Ok(match s {
                "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::AllPlatforms,
                "ANDROID" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Android,
                "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::AnyPlatform,
                "CHROME" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Chrome,
                "IOS" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Ios,
                "LINUX" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Linux,
                "OSX" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatHitPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Windows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::AllPlatforms,
                "ANDROID" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Android,
                "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::AnyPlatform,
                "CHROME" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Chrome,
                "IOS" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Ios,
                "LINUX" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Linux,
                "OSX" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatHitPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatHitPlatformType::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatHitPlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
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
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
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
    impl GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::ApiAbuse => "API_ABUSE",
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::ApkMalwareOffline => {
                    "APK_MALWARE_OFFLINE"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::ClientIncident => {
                    "CLIENT_INCIDENT"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::ClientIncidentWhitelist => {
                    "CLIENT_INCIDENT_WHITELIST"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::CsdDownloadWhitelist => {
                    "CSD_DOWNLOAD_WHITELIST"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::CsdWhitelist => "CSD_WHITELIST",
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::HighConfidenceAllowlist => {
                    "HIGH_CONFIDENCE_ALLOWLIST"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::MaliciousBinary => {
                    "MALICIOUS_BINARY"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::Malware => "MALWARE",
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::PotentiallyHarmfulApplication => {
                    "POTENTIALLY_HARMFUL_APPLICATION"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::SocialEngineering => {
                    "SOCIAL_ENGINEERING"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::SocialEngineeringInternal => {
                    "SOCIAL_ENGINEERING_INTERNAL"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::SubresourceFilter => {
                    "SUBRESOURCE_FILTER"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::Suspicious => "SUSPICIOUS",
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::ThreatTypeUnspecified => {
                    "THREAT_TYPE_UNSPECIFIED"
                }
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::TrickToBill => "TRICK_TO_BILL",
                GoogleSecuritySafebrowsingV4ThreatHitThreatType::UnwantedSoftware => {
                    "UNWANTED_SOFTWARE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatHitThreatType, ()> {
            Ok(match s {
                "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ApkMalwareOffline
                }
                "CLIENT_INCIDENT" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ClientIncident
                }
                "CLIENT_INCIDENT_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ClientIncidentWhitelist
                }
                "CSD_DOWNLOAD_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::CsdDownloadWhitelist
                }
                "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::HighConfidenceAllowlist
                }
                "MALICIOUS_BINARY" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::MaliciousBinary
                }
                "MALWARE" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::SocialEngineering
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::SubresourceFilter
                }
                "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ThreatTypeUnspecified
                }
                "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::UnwantedSoftware
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ApkMalwareOffline
                }
                "CLIENT_INCIDENT" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ClientIncident
                }
                "CLIENT_INCIDENT_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ClientIncidentWhitelist
                }
                "CSD_DOWNLOAD_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::CsdDownloadWhitelist
                }
                "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::HighConfidenceAllowlist
                }
                "MALICIOUS_BINARY" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::MaliciousBinary
                }
                "MALWARE" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::SocialEngineering
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::SubresourceFilter
                }
                "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::ThreatTypeUnspecified
                }
                "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatHitThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => {
                    GoogleSecuritySafebrowsingV4ThreatHitThreatType::UnwantedSoftware
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
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatHitThreatType {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
        #[doc = "The type of source reported."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType,
        >,
        #[doc = "Referrer of the resource. Only set if the referrer is available."]
        #[serde(
            rename = "referrer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referrer: ::std::option::Option<String>,
        #[doc = "The remote IP of the resource in ASCII format. Either IPv4 or IPv6."]
        #[serde(
            rename = "remoteIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remote_ip: ::std::option::Option<String>,
        #[doc = "The URL of the resource."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        #[doc = "The URL that matched the threat list (for which GetFullHash returned a valid hash)."]
        MatchingUrl,
        #[doc = "A redirect URL that was fetched before hitting the final TAB_URL."]
        TabRedirect,
        #[doc = "A resource loaded within the final TAB_URL."]
        TabResource,
        #[doc = "The final top-level URL of the tab that the client was browsing when the match occurred."]
        TabUrl,
        #[doc = "Unknown."]
        ThreatSourceTypeUnspecified,
    }
    impl GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: MatchingUrl => "MATCHING_URL" , GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabRedirect => "TAB_REDIRECT" , GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabResource => "TAB_RESOURCE" , GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabUrl => "TAB_URL" , GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: ThreatSourceTypeUnspecified => "THREAT_SOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType, ()>
        {
            Ok ( match s { "MATCHING_URL" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: MatchingUrl , "TAB_REDIRECT" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabRedirect , "TAB_RESOURCE" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabResource , "TAB_URL" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabUrl , "THREAT_SOURCE_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: ThreatSourceTypeUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "MATCHING_URL" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: MatchingUrl , "TAB_REDIRECT" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabRedirect , "TAB_RESOURCE" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabResource , "TAB_URL" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: TabUrl , "THREAT_SOURCE_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType :: ThreatSourceTypeUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceType
    {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
        #[doc = "The UN M.49 region code associated with the user's location."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
        #[doc = "Unique user identifier defined by the client."]
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatInfo {
        #[doc = "The platform types to be checked."]
        #[serde(
            rename = "platformTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_types: ::std::option::Option<
            Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems>,
        >,
        #[doc = "The threat entries to be checked."]
        #[serde(
            rename = "threatEntries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_entries:
            ::std::option::Option<Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatEntry>>,
        #[doc = "The entry types to be checked."]
        #[serde(
            rename = "threatEntryTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_entry_types: ::std::option::Option<
            Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems>,
        >,
        #[doc = "The threat types to be checked."]
        #[serde(
            rename = "threatTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_types: ::std::option::Option<
            Vec<crate::schemas::GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
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
    impl GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: AllPlatforms => "ALL_PLATFORMS" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Android => "ANDROID" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: AnyPlatform => "ANY_PLATFORM" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Chrome => "CHROME" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Ios => "IOS" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Linux => "LINUX" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Osx => "OSX" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Windows => "WINDOWS" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems, ()>
        {
            Ok ( match s { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Linux , "OSX" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Windows , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Linux , "OSX" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems :: Windows , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems {
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
    impl GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Cert => "CERT" , GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: ChromeExtension => "CHROME_EXTENSION" , GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Executable => "EXECUTABLE" , GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Filename => "FILENAME" , GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: IpRange => "IP_RANGE" , GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: ThreatEntryTypeUnspecified => "THREAT_ENTRY_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Url => "URL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems, ()>
        {
            Ok ( match s { "CERT" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Url , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "CERT" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems :: Url , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
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
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
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
    impl GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ApiAbuse => "API_ABUSE" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ApkMalwareOffline => "APK_MALWARE_OFFLINE" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ClientIncident => "CLIENT_INCIDENT" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: CsdWhitelist => "CSD_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: MaliciousBinary => "MALICIOUS_BINARY" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: Malware => "MALWARE" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: PotentiallyHarmfulApplication => "POTENTIALLY_HARMFUL_APPLICATION" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SocialEngineering => "SOCIAL_ENGINEERING" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SubresourceFilter => "SUBRESOURCE_FILTER" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: Suspicious => "SUSPICIOUS" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: TrickToBill => "TRICK_TO_BILL" , GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: UnwantedSoftware => "UNWANTED_SOFTWARE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems, ()>
        {
            Ok ( match s { "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: UnwantedSoftware , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems :: UnwantedSoftware , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesItems
    {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatListDescriptor {
        #[doc = "The platform type targeted by the list's entries."]
        #[serde(
            rename = "platformType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType,
        >,
        #[doc = "The entry types contained in the list."]
        #[serde(
            rename = "threatEntryType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_entry_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType,
        >,
        #[doc = "The threat type posed by the list's entries."]
        #[serde(
            rename = "threatType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatListDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatListDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType {
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
    impl GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: AllPlatforms => "ALL_PLATFORMS" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Android => "ANDROID" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: AnyPlatform => "ANY_PLATFORM" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Chrome => "CHROME" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Ios => "IOS" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Linux => "LINUX" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Osx => "OSX" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Windows => "WINDOWS" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType, ()>
        {
            Ok ( match s { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Linux , "OSX" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Windows , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ALL_PLATFORMS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: AllPlatforms , "ANDROID" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Android , "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: AnyPlatform , "CHROME" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Chrome , "IOS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Ios , "LINUX" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Linux , "OSX" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Osx , "PLATFORM_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: PlatformTypeUnspecified , "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType :: Windows , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType {
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
    impl GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Cert => "CERT" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: ChromeExtension => "CHROME_EXTENSION" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Executable => "EXECUTABLE" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Filename => "FILENAME" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: IpRange => "IP_RANGE" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: ThreatEntryTypeUnspecified => "THREAT_ENTRY_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Url => "URL" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType,
            (),
        > {
            Ok ( match s { "CERT" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Url , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "CERT" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType :: Url , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
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
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
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
    impl GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ApiAbuse => "API_ABUSE" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ApkMalwareOffline => "APK_MALWARE_OFFLINE" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ClientIncident => "CLIENT_INCIDENT" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: CsdWhitelist => "CSD_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: MaliciousBinary => "MALICIOUS_BINARY" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: Malware => "MALWARE" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: PotentiallyHarmfulApplication => "POTENTIALLY_HARMFUL_APPLICATION" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SocialEngineering => "SOCIAL_ENGINEERING" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SubresourceFilter => "SUBRESOURCE_FILTER" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: Suspicious => "SUSPICIOUS" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: TrickToBill => "TRICK_TO_BILL" , GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: UnwantedSoftware => "UNWANTED_SOFTWARE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType, ()>
        {
            Ok ( match s { "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: UnwantedSoftware , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ApiAbuse , "APK_MALWARE_OFFLINE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ApkMalwareOffline , "CLIENT_INCIDENT" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ClientIncident , "CLIENT_INCIDENT_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ClientIncidentWhitelist , "CSD_DOWNLOAD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: CsdDownloadWhitelist , "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: CsdWhitelist , "HIGH_CONFIDENCE_ALLOWLIST" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: HighConfidenceAllowlist , "MALICIOUS_BINARY" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: MaliciousBinary , "MALWARE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: Malware , "POTENTIALLY_HARMFUL_APPLICATION" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: PotentiallyHarmfulApplication , "SOCIAL_ENGINEERING" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SocialEngineering , "SOCIAL_ENGINEERING_INTERNAL" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SocialEngineeringInternal , "SUBRESOURCE_FILTER" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: SubresourceFilter , "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: Suspicious , "THREAT_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: ThreatTypeUnspecified , "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: TrickToBill , "UNWANTED_SOFTWARE" => GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType :: UnwantedSoftware , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatType
    {
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
    pub struct GoogleSecuritySafebrowsingV4ThreatMatch {
        #[doc = "The cache lifetime for the returned match. Clients must not cache this response for more than this duration to avoid false positives."]
        #[serde(
            rename = "cacheDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_duration: ::std::option::Option<String>,
        #[doc = "The platform type matching this threat."]
        #[serde(
            rename = "platformType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatMatchPlatformType,
        >,
        #[doc = "The threat matching this threat."]
        #[serde(
            rename = "threat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat: ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatEntry>,
        #[doc = "Optional metadata associated with this threat."]
        #[serde(
            rename = "threatEntryMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_entry_metadata:
            ::std::option::Option<crate::schemas::GoogleSecuritySafebrowsingV4ThreatEntryMetadata>,
        #[doc = "The threat entry type matching this threat."]
        #[serde(
            rename = "threatEntryType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_entry_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType,
        >,
        #[doc = "The threat type matching this threat."]
        #[serde(
            rename = "threatType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threat_type: ::std::option::Option<
            crate::schemas::GoogleSecuritySafebrowsingV4ThreatMatchThreatType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatMatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatMatch {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
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
    impl GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::AllPlatforms => {
                    "ALL_PLATFORMS"
                }
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Android => "ANDROID",
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::AnyPlatform => "ANY_PLATFORM",
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Chrome => "CHROME",
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Ios => "IOS",
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Linux => "LINUX",
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Osx => "OSX",
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::PlatformTypeUnspecified => {
                    "PLATFORM_TYPE_UNSPECIFIED"
                }
                GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatMatchPlatformType, ()>
        {
            Ok(match s {
                "ALL_PLATFORMS" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::AllPlatforms
                }
                "ANDROID" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Android,
                "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::AnyPlatform,
                "CHROME" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Chrome,
                "IOS" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Ios,
                "LINUX" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Linux,
                "OSX" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Windows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_PLATFORMS" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::AllPlatforms
                }
                "ANDROID" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Android,
                "ANY_PLATFORM" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::AnyPlatform,
                "CHROME" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Chrome,
                "IOS" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Ios,
                "LINUX" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Linux,
                "OSX" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Osx,
                "PLATFORM_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::PlatformTypeUnspecified
                }
                "WINDOWS" => GoogleSecuritySafebrowsingV4ThreatMatchPlatformType::Windows,
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
        for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatMatchPlatformType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
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
    impl GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Cert => "CERT" , GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: ChromeExtension => "CHROME_EXTENSION" , GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Executable => "EXECUTABLE" , GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Filename => "FILENAME" , GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: IpRange => "IP_RANGE" , GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: ThreatEntryTypeUnspecified => "THREAT_ENTRY_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Url => "URL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType, ()>
        {
            Ok ( match s { "CERT" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Url , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "CERT" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Cert , "CHROME_EXTENSION" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: ChromeExtension , "EXECUTABLE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Executable , "FILENAME" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Filename , "IP_RANGE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: IpRange , "THREAT_ENTRY_TYPE_UNSPECIFIED" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: ThreatEntryTypeUnspecified , "URL" => GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType :: Url , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
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
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
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
    impl GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: ApiAbuse => "API_ABUSE" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: ApkMalwareOffline => "APK_MALWARE_OFFLINE" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: ClientIncident => "CLIENT_INCIDENT" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: ClientIncidentWhitelist => "CLIENT_INCIDENT_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: CsdDownloadWhitelist => "CSD_DOWNLOAD_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: CsdWhitelist => "CSD_WHITELIST" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: HighConfidenceAllowlist => "HIGH_CONFIDENCE_ALLOWLIST" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: MaliciousBinary => "MALICIOUS_BINARY" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: Malware => "MALWARE" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: PotentiallyHarmfulApplication => "POTENTIALLY_HARMFUL_APPLICATION" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: SocialEngineering => "SOCIAL_ENGINEERING" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: SocialEngineeringInternal => "SOCIAL_ENGINEERING_INTERNAL" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: SubresourceFilter => "SUBRESOURCE_FILTER" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: Suspicious => "SUSPICIOUS" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: ThreatTypeUnspecified => "THREAT_TYPE_UNSPECIFIED" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: TrickToBill => "TRICK_TO_BILL" , GoogleSecuritySafebrowsingV4ThreatMatchThreatType :: UnwantedSoftware => "UNWANTED_SOFTWARE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleSecuritySafebrowsingV4ThreatMatchThreatType, ()> {
            Ok(match s {
                "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ApkMalwareOffline
                }
                "CLIENT_INCIDENT" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ClientIncident
                }
                "CLIENT_INCIDENT_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ClientIncidentWhitelist
                }
                "CSD_DOWNLOAD_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::CsdDownloadWhitelist
                }
                "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::HighConfidenceAllowlist
                }
                "MALICIOUS_BINARY" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::MaliciousBinary
                }
                "MALWARE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::SocialEngineering
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::SubresourceFilter
                }
                "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ThreatTypeUnspecified
                }
                "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::UnwantedSoftware
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_ABUSE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ApiAbuse,
                "APK_MALWARE_OFFLINE" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ApkMalwareOffline
                }
                "CLIENT_INCIDENT" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ClientIncident
                }
                "CLIENT_INCIDENT_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ClientIncidentWhitelist
                }
                "CSD_DOWNLOAD_WHITELIST" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::CsdDownloadWhitelist
                }
                "CSD_WHITELIST" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::CsdWhitelist,
                "HIGH_CONFIDENCE_ALLOWLIST" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::HighConfidenceAllowlist
                }
                "MALICIOUS_BINARY" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::MaliciousBinary
                }
                "MALWARE" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::Malware,
                "POTENTIALLY_HARMFUL_APPLICATION" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::PotentiallyHarmfulApplication
                }
                "SOCIAL_ENGINEERING" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::SocialEngineering
                }
                "SOCIAL_ENGINEERING_INTERNAL" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::SocialEngineeringInternal
                }
                "SUBRESOURCE_FILTER" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::SubresourceFilter
                }
                "SUSPICIOUS" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::Suspicious,
                "THREAT_TYPE_UNSPECIFIED" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::ThreatTypeUnspecified
                }
                "TRICK_TO_BILL" => GoogleSecuritySafebrowsingV4ThreatMatchThreatType::TrickToBill,
                "UNWANTED_SOFTWARE" => {
                    GoogleSecuritySafebrowsingV4ThreatMatchThreatType::UnwantedSoftware
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
    impl ::google_field_selector::FieldSelector for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSecuritySafebrowsingV4ThreatMatchThreatType {
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
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
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
        #[doc = "Created via [EncodedFullHashesActions::get()](struct.EncodedFullHashesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            encoded_request: ::google_api_bytes::Bytes,
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
            #[doc = "A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API."]
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
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FindFullHashesResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FindFullHashesResponse,
                crate::Error,
            > {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("clientId", &self.client_id)]);
                req = req.query(&[("clientVersion", &self.client_version)]);
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
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
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
        #[doc = "Created via [EncodedUpdatesActions::get()](struct.EncodedUpdatesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            encoded_request: ::google_api_bytes::Bytes,
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
            #[doc = "A client ID that uniquely identifies the client implementation of the Safe Browsing API."]
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
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse,
                crate::Error,
            > {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("clientId", &self.client_id)]);
                req = req.query(&[("clientVersion", &self.client_version)]);
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
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> FullHashesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Finds the full hashes that match the requested hash prefixes."]
            pub fn find(
                &self,
                request: crate::schemas::GoogleSecuritySafebrowsingV4FindFullHashesRequest,
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
        #[doc = "Created via [FullHashesActions::find()](struct.FullHashesActions.html#method.find)"]
        #[derive(Debug, Clone)]
        pub struct FindRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleSecuritySafebrowsingV4FindFullHashesRequest,
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
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FindFullHashesResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FindFullHashesResponse,
                crate::Error,
            > {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/fullHashes:find");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatHitsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Reports a Safe Browsing threat list hit to Google. Only projects with TRUSTED_REPORTER visibility can use this method."]
            pub fn create(
                &self,
                request: crate::schemas::GoogleSecuritySafebrowsingV4ThreatHit,
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
                }
            }
        }
        #[doc = "Created via [ThreatHitsActions::create()](struct.ThreatHitsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleSecuritySafebrowsingV4ThreatHit,
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
            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatHits");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatListUpdatesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Fetches the most recent threat list updates. A client can request updates for multiple lists at once."]
            pub fn fetch(
                &self,
                request: crate::schemas::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest,
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
        #[doc = "Created via [ThreatListUpdatesActions::fetch()](struct.ThreatListUpdatesActions.html#method.fetch)"]
        #[derive(Debug, Clone)]
        pub struct FetchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest,
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
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse,
                crate::Error,
            > {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatListUpdates:fetch");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
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
        #[doc = "Created via [ThreatListsActions::list()](struct.ThreatListsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4ListThreatListsResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4ListThreatListsResponse,
                crate::Error,
            > {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatLists");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ThreatMatchesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Finds the threat entries that match the Safe Browsing lists."]
            pub fn find(
                &self,
                request: crate::schemas::GoogleSecuritySafebrowsingV4FindThreatMatchesRequest,
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
        #[doc = "Created via [ThreatMatchesActions::find()](struct.ThreatMatchesActions.html#method.find)"]
        #[derive(Debug, Clone)]
        pub struct FindRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleSecuritySafebrowsingV4FindThreatMatchesRequest,
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
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FindThreatMatchesResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleSecuritySafebrowsingV4FindThreatMatchesResponse,
                crate::Error,
            > {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://safebrowsing.googleapis.com/".to_owned();
                output.push_str("v4/threatMatches:find");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
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
