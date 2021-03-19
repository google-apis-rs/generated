#![doc = "# Resources and Methods\n    * [v_3](resources/v_3/struct.V3Actions.html)\n      * [*logImpressions*](resources/v_3/struct.LogImpressionsRequestBuilder.html), [*logPlayerReports*](resources/v_3/struct.LogPlayerReportsRequestBuilder.html), [*samplePlayableLocations*](resources/v_3/struct.SamplePlayableLocationsRequestBuilder.html)\n"]
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
    pub struct GoogleMapsPlayablelocationsV3Impression {
        #[doc = "An arbitrary, developer-defined type identifier for each type of game object used in your game. Since players interact with differ types of game objects in different ways, this field allows you to segregate impression data by type for analysis. You should assign a unique `game_object_type` ID to represent a distinct type of game object in your game. For example, 1=monster location, 2=powerup location."]
        #[serde(
            rename = "gameObjectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub game_object_type: ::std::option::Option<i32>,
        #[doc = "Required. The type of impression event."]
        #[serde(
            rename = "impressionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impression_type: ::std::option::Option<
            crate::schemas::GoogleMapsPlayablelocationsV3ImpressionImpressionType,
        >,
        #[doc = "Required. The name of the playable location."]
        #[serde(
            rename = "locationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3Impression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3Impression {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        #[doc = "Unspecified type. Do not use."]
        ImpressionTypeUnspecified,
        #[doc = "A player interacted with the playable location."]
        Interacted,
        #[doc = "The playable location was presented to a player."]
        Presented,
    }
    impl GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleMapsPlayablelocationsV3ImpressionImpressionType :: ImpressionTypeUnspecified => "IMPRESSION_TYPE_UNSPECIFIED" , GoogleMapsPlayablelocationsV3ImpressionImpressionType :: Interacted => "INTERACTED" , GoogleMapsPlayablelocationsV3ImpressionImpressionType :: Presented => "PRESENTED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleMapsPlayablelocationsV3ImpressionImpressionType, ()>
        {
            Ok(match s {
                "IMPRESSION_TYPE_UNSPECIFIED" => {
                    GoogleMapsPlayablelocationsV3ImpressionImpressionType::ImpressionTypeUnspecified
                }
                "INTERACTED" => GoogleMapsPlayablelocationsV3ImpressionImpressionType::Interacted,
                "PRESENTED" => GoogleMapsPlayablelocationsV3ImpressionImpressionType::Presented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleMapsPlayablelocationsV3ImpressionImpressionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPRESSION_TYPE_UNSPECIFIED" => {
                    GoogleMapsPlayablelocationsV3ImpressionImpressionType::ImpressionTypeUnspecified
                }
                "INTERACTED" => GoogleMapsPlayablelocationsV3ImpressionImpressionType::Interacted,
                "PRESENTED" => GoogleMapsPlayablelocationsV3ImpressionImpressionType::Presented,
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
        for GoogleMapsPlayablelocationsV3ImpressionImpressionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3ImpressionImpressionType
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
    pub struct GoogleMapsPlayablelocationsV3LogImpressionsRequest {
        #[doc = "Required. Information about the client device. For example, device model and operating system."]
        #[serde(
            rename = "clientInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_info: ::std::option::Option<crate::schemas::GoogleMapsUnityClientInfo>,
        #[doc = "Required. Impression event details. The maximum number of impression reports that you can log at once is 50."]
        #[serde(
            rename = "impressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impressions:
            ::std::option::Option<Vec<crate::schemas::GoogleMapsPlayablelocationsV3Impression>>,
        #[doc = "Required. A string that uniquely identifies the log impressions request. This allows you to detect duplicate requests. We recommend that you use UUIDs for this value. The value must not exceed 50 characters. You should reuse the `request_id` only when retrying a request in case of failure. In this case, the request must be identical to the one that failed."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3LogImpressionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3LogImpressionsRequest {
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
    pub struct GoogleMapsPlayablelocationsV3LogImpressionsResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3LogImpressionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3LogImpressionsResponse {
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
    pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {
        #[doc = "Required. Information about the client device (for example, device model and operating system)."]
        #[serde(
            rename = "clientInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_info: ::std::option::Option<crate::schemas::GoogleMapsUnityClientInfo>,
        #[doc = "Required. Player reports. The maximum number of player reports that you can log at once is 50."]
        #[serde(
            rename = "playerReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_reports:
            ::std::option::Option<Vec<crate::schemas::GoogleMapsPlayablelocationsV3PlayerReport>>,
        #[doc = "Required. A string that uniquely identifies the log player reports request. This allows you to detect duplicate requests. We recommend that you use UUIDs for this value. The value must not exceed 50 characters. You should reuse the `request_id` only when retrying a request in the case of a failure. In that case, the request must be identical to the one that failed."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3LogPlayerReportsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {
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
    pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3LogPlayerReportsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3LogPlayerReportsResponse
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
    pub struct GoogleMapsPlayablelocationsV3PlayerReport {
        #[doc = "Language code (in BCP-47 format) indicating the language of the freeform description provided in `reason_details`. Examples are \"en\", \"en-US\" or \"ja-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Required. The name of the playable location."]
        #[serde(
            rename = "locationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_name: ::std::option::Option<String>,
        #[doc = "Required. A free-form description detailing why the playable location is considered bad."]
        #[serde(
            rename = "reasonDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason_details: ::std::option::Option<String>,
        #[doc = "Required. One or more reasons why this playable location is considered bad."]
        #[serde(
            rename = "reasons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reasons: ::std::option::Option<
            Vec<crate::schemas::GoogleMapsPlayablelocationsV3PlayerReportReasonsItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3PlayerReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3PlayerReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        #[doc = "Unspecified reason. Do not use."]
        BadLocationReasonUnspecified,
        #[doc = "The playable location isn't open to the public. For example, a private office building."]
        NotOpenToPublic,
        #[doc = "The playable location isn't accessible to pedestrians. For example, if it's in the middle of a highway."]
        NotPedestrianAccessible,
        #[doc = "The reason isn't one of the reasons in this enumeration."]
        Other,
        #[doc = "The playable location is permanently closed. For example, when a business has been shut down."]
        PermanentlyClosed,
        #[doc = "The playable location is temporarily inaccessible. For example, when a business has closed for renovations."]
        TemporarilyInaccessible,
    }
    impl GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: BadLocationReasonUnspecified => "BAD_LOCATION_REASON_UNSPECIFIED" , GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: NotOpenToPublic => "NOT_OPEN_TO_PUBLIC" , GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: NotPedestrianAccessible => "NOT_PEDESTRIAN_ACCESSIBLE" , GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: Other => "OTHER" , GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: PermanentlyClosed => "PERMANENTLY_CLOSED" , GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: TemporarilyInaccessible => "TEMPORARILY_INACCESSIBLE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleMapsPlayablelocationsV3PlayerReportReasonsItems, ()>
        {
            Ok ( match s { "BAD_LOCATION_REASON_UNSPECIFIED" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: BadLocationReasonUnspecified , "NOT_OPEN_TO_PUBLIC" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: NotOpenToPublic , "NOT_PEDESTRIAN_ACCESSIBLE" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: NotPedestrianAccessible , "OTHER" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: Other , "PERMANENTLY_CLOSED" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: PermanentlyClosed , "TEMPORARILY_INACCESSIBLE" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: TemporarilyInaccessible , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "BAD_LOCATION_REASON_UNSPECIFIED" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: BadLocationReasonUnspecified , "NOT_OPEN_TO_PUBLIC" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: NotOpenToPublic , "NOT_PEDESTRIAN_ACCESSIBLE" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: NotPedestrianAccessible , "OTHER" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: Other , "PERMANENTLY_CLOSED" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: PermanentlyClosed , "TEMPORARILY_INACCESSIBLE" => GoogleMapsPlayablelocationsV3PlayerReportReasonsItems :: TemporarilyInaccessible , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3PlayerReportReasonsItems
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
    pub struct GoogleMapsPlayablelocationsV3SampleAreaFilter {
        #[doc = "Required. The S2 cell ID of the area you want. This must be between cell level 11 and 14 (inclusive). S2 cells are 64-bit integers that identify areas on the Earth. They are hierarchical, and can therefore be used for spatial indexing. The S2 geometry library is available in a number of languages: * [C++](https://github.com/google/s2geometry) * [Java](https://github.com/google/s2-geometry-library-java) * [Go](https://github.com/golang/geo) * [Python](https://github.com/google/s2geometry/tree/master/src/python)"]
        #[serde(
            rename = "s2CellId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub s_2_cell_id: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3SampleAreaFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3SampleAreaFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SampleCriterion {
        #[doc = "Specifies which `PlayableLocation` fields are returned. `name` (which is used for logging impressions), `center_point` and `place_id` (or `plus_code`) are always returned. The following fields are omitted unless you specify them here: * snapped_point * types Note: The more fields you include, the more expensive in terms of data and associated latency your query will be."]
        #[serde(
            rename = "fieldsToReturn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields_to_return: ::std::option::Option<String>,
        #[doc = "Specifies filtering options, and specifies what will be included in the result set."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter:
            ::std::option::Option<crate::schemas::GoogleMapsPlayablelocationsV3SampleFilter>,
        #[doc = "Required. An arbitrary, developer-defined identifier of the type of game object that the playable location is used for. This field allows you to specify criteria per game object type when searching for playable locations. You should assign a unique `game_object_type` ID across all `request_criteria` to represent a distinct type of game object. For example, 1=monster location, 2=powerup location. The response contains a map."]
        #[serde(
            rename = "gameObjectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub game_object_type: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3SampleCriterion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3SampleCriterion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SampleFilter {
        #[doc = "Restricts the set of playable locations to just the [types](/maps/documentation/gaming/tt/types) that you want."]
        #[serde(
            rename = "includedTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_types: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies the maximum number of playable locations to return. This value must not be greater than 1000. The default value is 100. Only the top-ranking playable locations are returned."]
        #[serde(
            rename = "maxLocationCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_location_count: ::std::option::Option<i32>,
        #[doc = "A set of options that control the spacing between playable locations. By default the minimum distance between locations is 200m."]
        #[serde(
            rename = "spacing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spacing: ::std::option::Option<
            crate::schemas::GoogleMapsPlayablelocationsV3SampleSpacingOptions,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3SampleFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3SampleFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocation {
        #[doc = "Required. The latitude and longitude associated with the center of the playable location. By default, the set of playable locations returned from SamplePlayableLocations use center-point coordinates."]
        #[serde(
            rename = "centerPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub center_point: ::std::option::Option<crate::schemas::GoogleTypeLatLng>,
        #[doc = "Required. The name of this playable location."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A [place ID] (https://developers.google.com/places/place-id)"]
        #[serde(
            rename = "placeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub place_id: ::std::option::Option<String>,
        #[doc = "A [plus code] (http://openlocationcode.com)"]
        #[serde(
            rename = "plusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plus_code: ::std::option::Option<String>,
        #[doc = "The playable location's coordinates, snapped to the sidewalk of the nearest road, if a nearby road exists."]
        #[serde(
            rename = "snappedPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapped_point: ::std::option::Option<crate::schemas::GoogleTypeLatLng>,
        #[doc = "A collection of [Playable Location Types](/maps/documentation/gaming/tt/types) for this playable location. The first type in the collection is the primary type. Type information might not be available for all playable locations."]
        #[serde(
            rename = "types",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub types: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3SamplePlayableLocation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3SamplePlayableLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationList {
        #[doc = "A list of playable locations for this game object type."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<
            Vec<crate::schemas::GoogleMapsPlayablelocationsV3SamplePlayableLocation>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3SamplePlayableLocationList
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3SamplePlayableLocationList
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest {
        #[doc = "Required. Specifies the area to search within for playable locations."]
        #[serde(
            rename = "areaFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub area_filter:
            ::std::option::Option<crate::schemas::GoogleMapsPlayablelocationsV3SampleAreaFilter>,
        #[doc = "Required. Specifies one or more (up to 5) criteria for filtering the returned playable locations."]
        #[serde(
            rename = "criteria",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub criteria: ::std::option::Option<
            Vec<crate::schemas::GoogleMapsPlayablelocationsV3SampleCriterion>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse {
        #[doc = "Each PlayableLocation object corresponds to a game_object_type specified in the request."]
        #[serde(
            rename = "locationsPerGameObjectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations_per_game_object_type: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::GoogleMapsPlayablelocationsV3SamplePlayableLocationList,
            >,
        >,
        #[doc = "Required. Specifies the \"time-to-live\" for the set of playable locations. You can use this value to determine how long to cache the set of playable locations. After this length of time, your back-end game server should issue a new SamplePlayableLocations request to get a fresh set of playable locations (because for example, they might have been removed, a park might have closed for the day, a business might have closed permanently)."]
        #[serde(
            rename = "ttl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ttl: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleMapsPlayablelocationsV3SampleSpacingOptions {
        #[doc = "Required. The minimum spacing between any two playable locations, measured in meters. The minimum value is 30. The maximum value is 1000. Inputs will be rounded up to the next 10 meter interval. The default value is 200m. Set this field to remove tight clusters of playable locations. Note: The spacing is a greedy algorithm. It optimizes for selecting the highest ranking locations first, not to maximize the number of locations selected. Consider the following scenario: * Rank: A: 2, B: 1, C: 3. * Distance: A--200m--B--200m--C If spacing=250, it will pick the highest ranked location [B], not [A, C]. Note: Spacing works within the game object type itself, as well as the previous ones. Suppose three game object types, each with the following spacing: * X: 400m, Y: undefined, Z: 200m. 1. Add locations for X, within 400m of each other. 2. Add locations for Y, without any spacing. 3. Finally, add locations for Z within 200m of each other as well X and Y. The distance diagram between those locations end up as: * From->To. * X->X: 400m * Y->X, Y->Y: unspecified. * Z->X, Z->Y, Z->Z: 200m."]
        #[serde(
            rename = "minSpacingMeters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_spacing_meters: ::std::option::Option<f64>,
        #[doc = "Specifies whether the minimum spacing constraint applies to the center-point or to the snapped point of playable locations. The default value is `CENTER_POINT`. If a snapped point is not available for a playable location, its center-point is used instead. Set this to the point type used in your game."]
        #[serde(
            rename = "pointType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point_type: ::std::option::Option<
            crate::schemas::GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsPlayablelocationsV3SampleSpacingOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsPlayablelocationsV3SampleSpacingOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        #[doc = "The geographic coordinates correspond to the center of the location."]
        CenterPoint,
        #[doc = "Unspecified point type. Do not use this value."]
        PointTypeUnspecified,
        #[doc = "The geographic coordinates correspond to the location snapped to the sidewalk of the nearest road (when a nearby road exists)."]
        SnappedPoint,
    }
    impl GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType :: CenterPoint => "CENTER_POINT" , GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType :: PointTypeUnspecified => "POINT_TYPE_UNSPECIFIED" , GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType :: SnappedPoint => "SNAPPED_POINT" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType, ()>
        {
            Ok(match s {
                "CENTER_POINT" => {
                    GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType::CenterPoint
                }
                "POINT_TYPE_UNSPECIFIED" => {
                    GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType::PointTypeUnspecified
                }
                "SNAPPED_POINT" => {
                    GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType::SnappedPoint
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER_POINT" => {
                    GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType::CenterPoint
                }
                "POINT_TYPE_UNSPECIFIED" => {
                    GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType::PointTypeUnspecified
                }
                "SNAPPED_POINT" => {
                    GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType::SnappedPoint
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
    impl ::google_field_selector::FieldSelector
        for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleMapsPlayablelocationsV3SampleSpacingOptionsPointType
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
    pub struct GoogleMapsUnityClientInfo {
        #[doc = "API client name and version. For example, the SDK calling the API. The exact format is up to the client."]
        #[serde(
            rename = "apiClient",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_client: ::std::option::Option<String>,
        #[doc = "Application ID, such as the package name on Android and the bundle identifier on iOS platforms."]
        #[serde(
            rename = "applicationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_id: ::std::option::Option<String>,
        #[doc = "Application version number, such as \"1.2.3\". The exact format is application-dependent."]
        #[serde(
            rename = "applicationVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_version: ::std::option::Option<String>,
        #[doc = "Device model as reported by the device. The exact format is platform-dependent."]
        #[serde(
            rename = "deviceModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_model: ::std::option::Option<String>,
        #[doc = "Language code (in BCP-47 format) indicating the UI language of the client. Examples are \"en\", \"en-US\" or \"ja-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Operating system name and version as reported by the OS. For example, \"Mac OS X 10.10.4\". The exact format is platform-dependent."]
        #[serde(
            rename = "operatingSystem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operating_system: ::std::option::Option<String>,
        #[doc = "Build number/version of the operating system. e.g., the contents of android.os.Build.ID in Android, or the contents of sysctl \"kern.osversion\" in iOS."]
        #[serde(
            rename = "operatingSystemBuild",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operating_system_build: ::std::option::Option<String>,
        #[doc = "Platform where the application is running."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<crate::schemas::GoogleMapsUnityClientInfoPlatform>,
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsUnityClientInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsUnityClientInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleMapsUnityClientInfoPlatform {
        #[doc = "Android"]
        Android,
        #[doc = "Development environment."]
        Editor,
        #[doc = "iOS"]
        Ios,
        #[doc = "Linux"]
        Linux,
        #[doc = "macOS."]
        MacOs,
        #[doc = "Unspecified or unknown OS."]
        PlatformUnspecified,
        #[doc = "WebGL."]
        WebGl,
        #[doc = "Windows."]
        Windows,
    }
    impl GoogleMapsUnityClientInfoPlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleMapsUnityClientInfoPlatform::Android => "ANDROID",
                GoogleMapsUnityClientInfoPlatform::Editor => "EDITOR",
                GoogleMapsUnityClientInfoPlatform::Ios => "IOS",
                GoogleMapsUnityClientInfoPlatform::Linux => "LINUX",
                GoogleMapsUnityClientInfoPlatform::MacOs => "MAC_OS",
                GoogleMapsUnityClientInfoPlatform::PlatformUnspecified => "PLATFORM_UNSPECIFIED",
                GoogleMapsUnityClientInfoPlatform::WebGl => "WEB_GL",
                GoogleMapsUnityClientInfoPlatform::Windows => "WINDOWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleMapsUnityClientInfoPlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleMapsUnityClientInfoPlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleMapsUnityClientInfoPlatform, ()> {
            Ok(match s {
                "ANDROID" => GoogleMapsUnityClientInfoPlatform::Android,
                "EDITOR" => GoogleMapsUnityClientInfoPlatform::Editor,
                "IOS" => GoogleMapsUnityClientInfoPlatform::Ios,
                "LINUX" => GoogleMapsUnityClientInfoPlatform::Linux,
                "MAC_OS" => GoogleMapsUnityClientInfoPlatform::MacOs,
                "PLATFORM_UNSPECIFIED" => GoogleMapsUnityClientInfoPlatform::PlatformUnspecified,
                "WEB_GL" => GoogleMapsUnityClientInfoPlatform::WebGl,
                "WINDOWS" => GoogleMapsUnityClientInfoPlatform::Windows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleMapsUnityClientInfoPlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleMapsUnityClientInfoPlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleMapsUnityClientInfoPlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => GoogleMapsUnityClientInfoPlatform::Android,
                "EDITOR" => GoogleMapsUnityClientInfoPlatform::Editor,
                "IOS" => GoogleMapsUnityClientInfoPlatform::Ios,
                "LINUX" => GoogleMapsUnityClientInfoPlatform::Linux,
                "MAC_OS" => GoogleMapsUnityClientInfoPlatform::MacOs,
                "PLATFORM_UNSPECIFIED" => GoogleMapsUnityClientInfoPlatform::PlatformUnspecified,
                "WEB_GL" => GoogleMapsUnityClientInfoPlatform::WebGl,
                "WINDOWS" => GoogleMapsUnityClientInfoPlatform::Windows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleMapsUnityClientInfoPlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleMapsUnityClientInfoPlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleTypeLatLng {
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        #[serde(
            rename = "latitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        #[serde(
            rename = "longitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeLatLng {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeLatLng {
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
    #[doc = "Actions that can be performed on the v_3 resource"]
    pub fn v_3(&self) -> crate::resources::v_3::V3Actions {
        crate::resources::v_3::V3Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod v_3 {
        pub mod params {}
        pub struct V3Actions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V3Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Logs new events when playable locations are displayed, and when they are interacted with. Impressions are not partially saved; either all impressions are saved and this request succeeds, or no impressions are saved, and this request fails."]
            pub fn log_impressions(
                &self,
                request: crate::schemas::GoogleMapsPlayablelocationsV3LogImpressionsRequest,
            ) -> LogImpressionsRequestBuilder {
                LogImpressionsRequestBuilder {
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
            #[doc = "Logs bad playable location reports submitted by players. Reports are not partially saved; either all reports are saved and this request succeeds, or no reports are saved, and this request fails."]
            pub fn log_player_reports(
                &self,
                request: crate::schemas::GoogleMapsPlayablelocationsV3LogPlayerReportsRequest,
            ) -> LogPlayerReportsRequestBuilder {
                LogPlayerReportsRequestBuilder {
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
            #[doc = "Returns a set of playable locations that lie within a specified area, that satisfy optional filter criteria. Note: Identical `SamplePlayableLocations` requests can return different results as the state of the world changes over time."]
            pub fn sample_playable_locations(
                &self,
                request : crate :: schemas :: GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest,
            ) -> SamplePlayableLocationsRequestBuilder {
                SamplePlayableLocationsRequestBuilder {
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
        #[doc = "Created via [V3Actions::log_impressions()](struct.V3Actions.html#method.log_impressions)"]
        #[derive(Debug, Clone)]
        pub struct LogImpressionsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleMapsPlayablelocationsV3LogImpressionsRequest,
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
        impl<'a> LogImpressionsRequestBuilder<'a> {
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
                crate::schemas::GoogleMapsPlayablelocationsV3LogImpressionsResponse,
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
                crate::schemas::GoogleMapsPlayablelocationsV3LogImpressionsResponse,
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
                let mut output = "https://playablelocations.googleapis.com/".to_owned();
                output.push_str("v3:logImpressions");
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
        #[doc = "Created via [V3Actions::log_player_reports()](struct.V3Actions.html#method.log_player_reports)"]
        #[derive(Debug, Clone)]
        pub struct LogPlayerReportsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleMapsPlayablelocationsV3LogPlayerReportsRequest,
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
        impl<'a> LogPlayerReportsRequestBuilder<'a> {
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
                crate::schemas::GoogleMapsPlayablelocationsV3LogPlayerReportsResponse,
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
                crate::schemas::GoogleMapsPlayablelocationsV3LogPlayerReportsResponse,
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
                let mut output = "https://playablelocations.googleapis.com/".to_owned();
                output.push_str("v3:logPlayerReports");
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
        #[doc = "Created via [V3Actions::sample_playable_locations()](struct.V3Actions.html#method.sample_playable_locations)"]
        #[derive(Debug, Clone)]
        pub struct SamplePlayableLocationsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest,
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
        impl<'a> SamplePlayableLocationsRequestBuilder<'a> {
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
                crate::schemas::GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse,
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
                crate::schemas::GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse,
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
                let mut output = "https://playablelocations.googleapis.com/".to_owned();
                output.push_str("v3:samplePlayableLocations");
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
