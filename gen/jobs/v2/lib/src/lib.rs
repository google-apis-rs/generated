#![doc = "# Resources and Methods\n    * [companies](resources/companies/struct.CompaniesActions.html)\n      * [*create*](resources/companies/struct.CreateRequestBuilder.html), [*delete*](resources/companies/struct.DeleteRequestBuilder.html), [*get*](resources/companies/struct.GetRequestBuilder.html), [*list*](resources/companies/struct.ListRequestBuilder.html), [*patch*](resources/companies/struct.PatchRequestBuilder.html)\n      * [jobs](resources/companies/jobs/struct.JobsActions.html)\n        * [*list*](resources/companies/jobs/struct.ListRequestBuilder.html)\n    * [jobs](resources/jobs/struct.JobsActions.html)\n      * [*batchDelete*](resources/jobs/struct.BatchDeleteRequestBuilder.html), [*create*](resources/jobs/struct.CreateRequestBuilder.html), [*delete*](resources/jobs/struct.DeleteRequestBuilder.html), [*deleteByFilter*](resources/jobs/struct.DeleteByFilterRequestBuilder.html), [*get*](resources/jobs/struct.GetRequestBuilder.html), [*histogram*](resources/jobs/struct.HistogramRequestBuilder.html), [*list*](resources/jobs/struct.ListRequestBuilder.html), [*patch*](resources/jobs/struct.PatchRequestBuilder.html), [*search*](resources/jobs/struct.SearchRequestBuilder.html), [*searchForAlert*](resources/jobs/struct.SearchForAlertRequestBuilder.html)\n    * [v_2](resources/v_2/struct.V2Actions.html)\n      * [*complete*](resources/v_2/struct.CompleteRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "Manage job postings\n\n`https://www.googleapis.com/auth/jobs`"]
    pub const JOBS: &str = "https://www.googleapis.com/auth/jobs";
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
    pub struct BatchDeleteJobsRequest {
        #[doc = "Required. The filter string specifies the jobs to be deleted.\n\nSupported operator: =, AND\n\nThe fields eligible for filtering are:\n\n* `companyName` (Required)\n* `requisitionId` (Required)\n\nSample Query: companyName = \"companies/123\" AND requisitionId = \"req-1\""]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BatchDeleteJobsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchDeleteJobsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BucketRange {
        #[doc = "Starting value of the bucket range."]
        #[serde(
            rename = "from",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from: ::std::option::Option<f64>,
        #[doc = "Ending value of the bucket range."]
        #[serde(
            rename = "to",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub to: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for BucketRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BucketizedCount {
        #[doc = "Number of jobs whose numeric field value fall into `range`."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Bucket range on which histogram was performed for the numeric field,\nthat is, the count represents number of jobs in this range."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::BucketRange>,
    }
    impl ::google_field_selector::FieldSelector for BucketizedCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketizedCount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommuteInfo {
        #[doc = "Location used as the destination in the commute calculation."]
        #[serde(
            rename = "jobLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_location: ::std::option::Option<crate::schemas::JobLocation>,
        #[doc = "The number of seconds required to travel to the job location from the query\nlocation. A duration of 0 seconds indicates that the job is not\nreachable within the requested duration, but was returned as part of an\nexpanded query."]
        #[serde(
            rename = "travelDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub travel_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CommuteInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommuteInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommutePreference {
        #[doc = "Optional. If `true`, jobs without street level addresses may also be returned.\nFor city level addresses, the city center is used. For state and coarser\nlevel addresses, text matching is used.\nIf this field is set to `false` or is not specified, only jobs that include\nstreet level addresses will be returned by commute search."]
        #[serde(
            rename = "allowNonStreetLevelAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_non_street_level_address: ::std::option::Option<bool>,
        #[doc = "Optional. The departure hour to use to calculate traffic impact. Accepts an\ninteger between 0 and 23, representing the hour in the time zone of the\nstart_location. Must not be present if road_traffic is specified."]
        #[serde(
            rename = "departureHourLocal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub departure_hour_local: ::std::option::Option<i32>,
        #[doc = "Required. The method of transportation for which to calculate the commute time."]
        #[serde(
            rename = "method",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method: ::std::option::Option<crate::schemas::CommutePreferenceMethod>,
        #[doc = "Optional. Specifies the traffic density to use when calculating commute time.\nMust not be present if departure_hour_local is specified."]
        #[serde(
            rename = "roadTraffic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub road_traffic: ::std::option::Option<crate::schemas::CommutePreferenceRoadTraffic>,
        #[doc = "Required. The latitude and longitude of the location from which to calculate the\ncommute time."]
        #[serde(
            rename = "startLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_location: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Required. The maximum travel time in seconds. The maximum allowed value is `3600s`\n(one hour). Format is `123s`."]
        #[serde(
            rename = "travelTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub travel_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CommutePreference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommutePreference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommutePreferenceMethod {
        #[doc = "Commute method is not specified."]
        CommuteMethodUnspecified,
        #[doc = "Commute time is calculated based on driving time."]
        Driving,
        #[doc = "Commute time is calculated based on public transit including bus, metro,\nsubway, etc."]
        Transit,
    }
    impl CommutePreferenceMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                CommutePreferenceMethod::CommuteMethodUnspecified => "COMMUTE_METHOD_UNSPECIFIED",
                CommutePreferenceMethod::Driving => "DRIVING",
                CommutePreferenceMethod::Transit => "TRANSIT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommutePreferenceMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommutePreferenceMethod {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommutePreferenceMethod, ()> {
            Ok(match s {
                "COMMUTE_METHOD_UNSPECIFIED" => CommutePreferenceMethod::CommuteMethodUnspecified,
                "DRIVING" => CommutePreferenceMethod::Driving,
                "TRANSIT" => CommutePreferenceMethod::Transit,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommutePreferenceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommutePreferenceMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommutePreferenceMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMUTE_METHOD_UNSPECIFIED" => CommutePreferenceMethod::CommuteMethodUnspecified,
                "DRIVING" => CommutePreferenceMethod::Driving,
                "TRANSIT" => CommutePreferenceMethod::Transit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommutePreferenceMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommutePreferenceMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommutePreferenceRoadTraffic {
        #[doc = "Commute time calculation takes in account the peak traffic impact."]
        BusyHour,
        #[doc = "Road traffic situation is not specified."]
        RoadTrafficUnspecified,
        #[doc = "Optimal commute time without considering any traffic impact."]
        TrafficFree,
    }
    impl CommutePreferenceRoadTraffic {
        pub fn as_str(self) -> &'static str {
            match self {
                CommutePreferenceRoadTraffic::BusyHour => "BUSY_HOUR",
                CommutePreferenceRoadTraffic::RoadTrafficUnspecified => "ROAD_TRAFFIC_UNSPECIFIED",
                CommutePreferenceRoadTraffic::TrafficFree => "TRAFFIC_FREE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommutePreferenceRoadTraffic {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommutePreferenceRoadTraffic {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommutePreferenceRoadTraffic, ()> {
            Ok(match s {
                "BUSY_HOUR" => CommutePreferenceRoadTraffic::BusyHour,
                "ROAD_TRAFFIC_UNSPECIFIED" => CommutePreferenceRoadTraffic::RoadTrafficUnspecified,
                "TRAFFIC_FREE" => CommutePreferenceRoadTraffic::TrafficFree,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommutePreferenceRoadTraffic {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommutePreferenceRoadTraffic {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommutePreferenceRoadTraffic {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUSY_HOUR" => CommutePreferenceRoadTraffic::BusyHour,
                "ROAD_TRAFFIC_UNSPECIFIED" => CommutePreferenceRoadTraffic::RoadTrafficUnspecified,
                "TRAFFIC_FREE" => CommutePreferenceRoadTraffic::TrafficFree,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommutePreferenceRoadTraffic {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommutePreferenceRoadTraffic {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Company {
        #[doc = "Optional. The URL to employer's career site or careers page on the employer's web\nsite."]
        #[serde(
            rename = "careerPageLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub career_page_link: ::std::option::Option<String>,
        #[doc = "Optional. Identifiers external to the application that help to further identify\nthe employer."]
        #[serde(
            rename = "companyInfoSources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_info_sources: ::std::option::Option<Vec<crate::schemas::CompanyInfoSource>>,
        #[doc = "Optional. The employer's company size."]
        #[serde(
            rename = "companySize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_size: ::std::option::Option<crate::schemas::CompanyCompanySize>,
        #[doc = "Deprecated. Do not use this field.\n\nOptional.\n\nThis field is no longer used. Any value set to it is ignored."]
        #[serde(
            rename = "disableLocationOptimization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_location_optimization: ::std::option::Option<bool>,
        #[doc = "Required. The name of the employer to be displayed with the job,\nfor example, \"Google, LLC.\"."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. The unique company identifier provided by the client to identify an\nemployer for billing purposes. Recommended practice is to use\nthe distributor_company_id.\n\nDefaults to same value as distributor_company_id when a value\nis not provided."]
        #[serde(
            rename = "distributorBillingCompanyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distributor_billing_company_id: ::std::option::Option<String>,
        #[doc = "Required. A client's company identifier, used to uniquely identify the\ncompany. If an employer has a subsidiary or sub-brand, such as \"Alphabet\"\nand \"Google\", which the client wishes to use as the company displayed on\nthe job. Best practice is to create a distinct company identifier for each\ndistinct brand displayed.\n\nThe maximum number of allowed characters is 255."]
        #[serde(
            rename = "distributorCompanyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distributor_company_id: ::std::option::Option<String>,
        #[doc = "Optional. Equal Employment Opportunity legal disclaimer text to be\nassociated with all jobs, and typically to be displayed in all\nroles.\n\nThe maximum number of allowed characters is 500."]
        #[serde(
            rename = "eeoText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eeo_text: ::std::option::Option<String>,
        #[doc = "Optional. Set to true if it is the hiring agency that post jobs for other\nemployers.\n\nDefaults to false if not provided."]
        #[serde(
            rename = "hiringAgency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hiring_agency: ::std::option::Option<bool>,
        #[doc = "Optional. The street address of the company's main headquarters, which may be\ndifferent from the job location. The service attempts\nto geolocate the provided address, and populates a more specific\nlocation wherever possible in structured_company_hq_location."]
        #[serde(
            rename = "hqLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq_location: ::std::option::Option<String>,
        #[doc = "Optional. A URL that hosts the employer's company logo. If provided,\nthe logo image should be squared at 80x80 pixels.\n\nThe url must be a Google Photos or Google Album url.\nOnly images in these Google sub-domains are accepted."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "Optional. A list of keys of filterable Job.custom_attributes, whose\ncorresponding `string_values` are used in keyword search. Jobs with\n`string_values` under these specified field keys are returned if any\nof the values matches the search keyword. Custom field values with\nparenthesis, brackets and special symbols might not be properly searchable,\nand those keyword queries need to be surrounded by quotes."]
        #[serde(
            rename = "keywordSearchableCustomAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyword_searchable_custom_attributes: ::std::option::Option<Vec<String>>,
        #[doc = "Deprecated. Use keyword_searchable_custom_attributes instead.\n\nOptional.\n\nA list of filterable custom fields that should be used in keyword\nsearch. The jobs of this company are returned if any of these custom\nfields matches the search keyword. Custom field values with parenthesis,\nbrackets and special symbols might not be properly searchable, and those\nkeyword queries need to be surrounded by quotes."]
        #[serde(
            rename = "keywordSearchableCustomFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyword_searchable_custom_fields: ::std::option::Option<Vec<i32>>,
        #[doc = "Required during company update.\n\nThe resource name for a company. This is generated by the service when a\ncompany is created, for example,\n\"companies/0000aaaa-1111-bbbb-2222-cccc3333dddd\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. A structured headquarters location of the company,\nresolved from hq_location if possible."]
        #[serde(
            rename = "structuredCompanyHqLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub structured_company_hq_location: ::std::option::Option<crate::schemas::JobLocation>,
        #[doc = "Output only. Indicates whether a company is flagged to be suspended from public\navailability by the service when job content appears suspicious,\nabusive, or spammy."]
        #[serde(
            rename = "suspended",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suspended: ::std::option::Option<bool>,
        #[doc = "Deprecated. Use display_name instead.\n\nRequired.\n\nThe name of the employer to be displayed with the job,\nfor example, \"Google, LLC.\"."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Optional. The URL representing the company's primary web site or home page,\nsuch as, \"www.google.com\"."]
        #[serde(
            rename = "website",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub website: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Company {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Company {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompanyCompanySize {
        #[doc = "The company has between 1,000 and 4,999 employees."]
        Big,
        #[doc = "The company has between 5,000 and 9,999 employees."]
        Bigger,
        #[doc = "Default value if the size is not specified."]
        CompanySizeUnspecified,
        #[doc = "The company has 10,000 or more employees."]
        Giant,
        #[doc = "The company has between 500 and 999 employees."]
        Medium,
        #[doc = "The company has less than 50 employees."]
        Mini,
        #[doc = "The company has between 50 and 99 employees."]
        Small,
        #[doc = "The company has between 100 and 499 employees."]
        Smedium,
    }
    impl CompanyCompanySize {
        pub fn as_str(self) -> &'static str {
            match self {
                CompanyCompanySize::Big => "BIG",
                CompanyCompanySize::Bigger => "BIGGER",
                CompanyCompanySize::CompanySizeUnspecified => "COMPANY_SIZE_UNSPECIFIED",
                CompanyCompanySize::Giant => "GIANT",
                CompanyCompanySize::Medium => "MEDIUM",
                CompanyCompanySize::Mini => "MINI",
                CompanyCompanySize::Small => "SMALL",
                CompanyCompanySize::Smedium => "SMEDIUM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompanyCompanySize {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompanyCompanySize {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompanyCompanySize, ()> {
            Ok(match s {
                "BIG" => CompanyCompanySize::Big,
                "BIGGER" => CompanyCompanySize::Bigger,
                "COMPANY_SIZE_UNSPECIFIED" => CompanyCompanySize::CompanySizeUnspecified,
                "GIANT" => CompanyCompanySize::Giant,
                "MEDIUM" => CompanyCompanySize::Medium,
                "MINI" => CompanyCompanySize::Mini,
                "SMALL" => CompanyCompanySize::Small,
                "SMEDIUM" => CompanyCompanySize::Smedium,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompanyCompanySize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompanyCompanySize {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompanyCompanySize {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BIG" => CompanyCompanySize::Big,
                "BIGGER" => CompanyCompanySize::Bigger,
                "COMPANY_SIZE_UNSPECIFIED" => CompanyCompanySize::CompanySizeUnspecified,
                "GIANT" => CompanyCompanySize::Giant,
                "MEDIUM" => CompanyCompanySize::Medium,
                "MINI" => CompanyCompanySize::Mini,
                "SMALL" => CompanyCompanySize::Small,
                "SMEDIUM" => CompanyCompanySize::Smedium,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompanyCompanySize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompanyCompanySize {
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
    pub struct CompanyInfoSource {
        #[doc = "Optional. The Google's Knowledge Graph value for the employer's company."]
        #[serde(
            rename = "freebaseMid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub freebase_mid: ::std::option::Option<String>,
        #[doc = "Optional. The numeric identifier for the employer's Google+ business page."]
        #[serde(
            rename = "gplusId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gplus_id: ::std::option::Option<String>,
        #[doc = "Optional. The numeric identifier for the employer's headquarters on Google Maps,\nnamely, the Google Maps CID (cell id)."]
        #[serde(
            rename = "mapsCid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maps_cid: ::std::option::Option<String>,
        #[doc = "Optional. A Google identifier that does not match any of the other types."]
        #[serde(
            rename = "unknownTypeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unknown_type_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CompanyInfoSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompanyInfoSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationEntry {
        #[doc = "Optional. Compensation amount."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Optional. Compensation description.  For example, could\nindicate equity terms or provide additional context to an estimated\nbonus."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. Expected number of units paid each year. If not specified, when\nJob.employment_types is FULLTIME, a default value is inferred\nbased on unit. Default values:\n\n* HOURLY: 2080\n* DAILY: 260\n* WEEKLY: 52\n* MONTHLY: 12\n* ANNUAL: 1"]
        #[serde(
            rename = "expectedUnitsPerYear",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expected_units_per_year: ::std::option::Option<f64>,
        #[doc = "Required. Compensation type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CompensationEntryType>,
        #[doc = "Optional. Compensation range."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Optional. Frequency of the specified amount.\n\nDefault is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<crate::schemas::CompensationEntryUnit>,
    }
    impl ::google_field_selector::FieldSelector for CompensationEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationEntryType {
        #[doc = "Base compensation: Refers to the fixed amount of money paid to an\nemployee by an employer in return for work performed. Base compensation\ndoes not include benefits, bonuses or any other potential compensation\nfrom an employer."]
        Base,
        #[doc = "Bonus."]
        Bonus,
        #[doc = "Commission."]
        Commissions,
        #[doc = "Default value. Equivalent to OTHER_COMPENSATION_TYPE."]
        CompensationTypeUnspecified,
        #[doc = "Equity."]
        Equity,
        #[doc = "Other compensation type."]
        OtherCompensationType,
        #[doc = "Profit sharing."]
        ProfitSharing,
        #[doc = "Signing bonus."]
        SigningBonus,
        #[doc = "Tips."]
        Tips,
    }
    impl CompensationEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationEntryType::Base => "BASE",
                CompensationEntryType::Bonus => "BONUS",
                CompensationEntryType::Commissions => "COMMISSIONS",
                CompensationEntryType::CompensationTypeUnspecified => {
                    "COMPENSATION_TYPE_UNSPECIFIED"
                }
                CompensationEntryType::Equity => "EQUITY",
                CompensationEntryType::OtherCompensationType => "OTHER_COMPENSATION_TYPE",
                CompensationEntryType::ProfitSharing => "PROFIT_SHARING",
                CompensationEntryType::SigningBonus => "SIGNING_BONUS",
                CompensationEntryType::Tips => "TIPS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationEntryType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationEntryType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationEntryType, ()> {
            Ok(match s {
                "BASE" => CompensationEntryType::Base,
                "BONUS" => CompensationEntryType::Bonus,
                "COMMISSIONS" => CompensationEntryType::Commissions,
                "COMPENSATION_TYPE_UNSPECIFIED" => {
                    CompensationEntryType::CompensationTypeUnspecified
                }
                "EQUITY" => CompensationEntryType::Equity,
                "OTHER_COMPENSATION_TYPE" => CompensationEntryType::OtherCompensationType,
                "PROFIT_SHARING" => CompensationEntryType::ProfitSharing,
                "SIGNING_BONUS" => CompensationEntryType::SigningBonus,
                "TIPS" => CompensationEntryType::Tips,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASE" => CompensationEntryType::Base,
                "BONUS" => CompensationEntryType::Bonus,
                "COMMISSIONS" => CompensationEntryType::Commissions,
                "COMPENSATION_TYPE_UNSPECIFIED" => {
                    CompensationEntryType::CompensationTypeUnspecified
                }
                "EQUITY" => CompensationEntryType::Equity,
                "OTHER_COMPENSATION_TYPE" => CompensationEntryType::OtherCompensationType,
                "PROFIT_SHARING" => CompensationEntryType::ProfitSharing,
                "SIGNING_BONUS" => CompensationEntryType::SigningBonus,
                "TIPS" => CompensationEntryType::Tips,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompensationEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationEntryType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationEntryUnit {
        #[doc = "Default value. Equivalent to OTHER_COMPENSATION_UNIT."]
        CompensationUnitUnspecified,
        #[doc = "Daily."]
        Daily,
        #[doc = "Hourly."]
        Hourly,
        #[doc = "Monthly."]
        Monthly,
        #[doc = "One time."]
        OneTime,
        #[doc = "Other compensation units."]
        OtherCompensationUnit,
        #[doc = "Weekly"]
        Weekly,
        #[doc = "Yearly."]
        Yearly,
    }
    impl CompensationEntryUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationEntryUnit::CompensationUnitUnspecified => {
                    "COMPENSATION_UNIT_UNSPECIFIED"
                }
                CompensationEntryUnit::Daily => "DAILY",
                CompensationEntryUnit::Hourly => "HOURLY",
                CompensationEntryUnit::Monthly => "MONTHLY",
                CompensationEntryUnit::OneTime => "ONE_TIME",
                CompensationEntryUnit::OtherCompensationUnit => "OTHER_COMPENSATION_UNIT",
                CompensationEntryUnit::Weekly => "WEEKLY",
                CompensationEntryUnit::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationEntryUnit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationEntryUnit {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationEntryUnit, ()> {
            Ok(match s {
                "COMPENSATION_UNIT_UNSPECIFIED" => {
                    CompensationEntryUnit::CompensationUnitUnspecified
                }
                "DAILY" => CompensationEntryUnit::Daily,
                "HOURLY" => CompensationEntryUnit::Hourly,
                "MONTHLY" => CompensationEntryUnit::Monthly,
                "ONE_TIME" => CompensationEntryUnit::OneTime,
                "OTHER_COMPENSATION_UNIT" => CompensationEntryUnit::OtherCompensationUnit,
                "WEEKLY" => CompensationEntryUnit::Weekly,
                "YEARLY" => CompensationEntryUnit::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationEntryUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationEntryUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationEntryUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPENSATION_UNIT_UNSPECIFIED" => {
                    CompensationEntryUnit::CompensationUnitUnspecified
                }
                "DAILY" => CompensationEntryUnit::Daily,
                "HOURLY" => CompensationEntryUnit::Hourly,
                "MONTHLY" => CompensationEntryUnit::Monthly,
                "ONE_TIME" => CompensationEntryUnit::OneTime,
                "OTHER_COMPENSATION_UNIT" => CompensationEntryUnit::OtherCompensationUnit,
                "WEEKLY" => CompensationEntryUnit::Weekly,
                "YEARLY" => CompensationEntryUnit::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompensationEntryUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationEntryUnit {
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
    pub struct CompensationFilter {
        #[doc = "Optional. Whether to include jobs whose compensation range is unspecified."]
        #[serde(
            rename = "includeJobsWithUnspecifiedCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_jobs_with_unspecified_compensation_range: ::std::option::Option<bool>,
        #[doc = "Required. Type of filter."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CompensationFilterType>,
        #[doc = "Optional. Compensation range."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Required. Specify desired `base compensation entry's`\nCompensationInfo.CompensationUnit."]
        #[serde(
            rename = "units",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub units: ::std::option::Option<Vec<crate::schemas::CompensationFilterUnitsItems>>,
    }
    impl ::google_field_selector::FieldSelector for CompensationFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationFilterType {
        #[doc = "Filter by annualized base compensation amount and `base compensation entry's` unit. Populate range and zero or more units."]
        AnnualizedBaseAmount,
        #[doc = "Filter by annualized total compensation amount and `base compensation entry's` unit . Populate range and zero or more units."]
        AnnualizedTotalAmount,
        #[doc = "Filter type unspecified. Position holder, INVALID, should never be used."]
        FilterTypeUnspecified,
        #[doc = "Filter by `base compensation entry's` unit and amount / range. A job\nis a match if and only if the job contains a base CompensationEntry, and\nthe base entry's unit matches provided compensation_units and amount\nor range overlaps with provided compensation_range.\n\nSee CompensationInfo.CompensationEntry for definition of\nbase compensation entry.\n\nSet exactly one units and populate range."]
        UnitAndAmount,
        #[doc = "Filter by `base compensation entry's` unit. A job is a match if and\nonly if the job contains a base CompensationEntry and the base\nCompensationEntry's unit matches provided units.\nPopulate one or more units.\n\nSee CompensationInfo.CompensationEntry for definition of\nbase compensation entry."]
        UnitOnly,
    }
    impl CompensationFilterType {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationFilterType::AnnualizedBaseAmount => "ANNUALIZED_BASE_AMOUNT",
                CompensationFilterType::AnnualizedTotalAmount => "ANNUALIZED_TOTAL_AMOUNT",
                CompensationFilterType::FilterTypeUnspecified => "FILTER_TYPE_UNSPECIFIED",
                CompensationFilterType::UnitAndAmount => "UNIT_AND_AMOUNT",
                CompensationFilterType::UnitOnly => "UNIT_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationFilterType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationFilterType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationFilterType, ()> {
            Ok(match s {
                "ANNUALIZED_BASE_AMOUNT" => CompensationFilterType::AnnualizedBaseAmount,
                "ANNUALIZED_TOTAL_AMOUNT" => CompensationFilterType::AnnualizedTotalAmount,
                "FILTER_TYPE_UNSPECIFIED" => CompensationFilterType::FilterTypeUnspecified,
                "UNIT_AND_AMOUNT" => CompensationFilterType::UnitAndAmount,
                "UNIT_ONLY" => CompensationFilterType::UnitOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationFilterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationFilterType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationFilterType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNUALIZED_BASE_AMOUNT" => CompensationFilterType::AnnualizedBaseAmount,
                "ANNUALIZED_TOTAL_AMOUNT" => CompensationFilterType::AnnualizedTotalAmount,
                "FILTER_TYPE_UNSPECIFIED" => CompensationFilterType::FilterTypeUnspecified,
                "UNIT_AND_AMOUNT" => CompensationFilterType::UnitAndAmount,
                "UNIT_ONLY" => CompensationFilterType::UnitOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompensationFilterType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationFilterType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationFilterUnitsItems {
        CompensationUnitUnspecified,
        Daily,
        Hourly,
        Monthly,
        OneTime,
        OtherCompensationUnit,
        Weekly,
        Yearly,
    }
    impl CompensationFilterUnitsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationFilterUnitsItems::CompensationUnitUnspecified => {
                    "COMPENSATION_UNIT_UNSPECIFIED"
                }
                CompensationFilterUnitsItems::Daily => "DAILY",
                CompensationFilterUnitsItems::Hourly => "HOURLY",
                CompensationFilterUnitsItems::Monthly => "MONTHLY",
                CompensationFilterUnitsItems::OneTime => "ONE_TIME",
                CompensationFilterUnitsItems::OtherCompensationUnit => "OTHER_COMPENSATION_UNIT",
                CompensationFilterUnitsItems::Weekly => "WEEKLY",
                CompensationFilterUnitsItems::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationFilterUnitsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationFilterUnitsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationFilterUnitsItems, ()> {
            Ok(match s {
                "COMPENSATION_UNIT_UNSPECIFIED" => {
                    CompensationFilterUnitsItems::CompensationUnitUnspecified
                }
                "DAILY" => CompensationFilterUnitsItems::Daily,
                "HOURLY" => CompensationFilterUnitsItems::Hourly,
                "MONTHLY" => CompensationFilterUnitsItems::Monthly,
                "ONE_TIME" => CompensationFilterUnitsItems::OneTime,
                "OTHER_COMPENSATION_UNIT" => CompensationFilterUnitsItems::OtherCompensationUnit,
                "WEEKLY" => CompensationFilterUnitsItems::Weekly,
                "YEARLY" => CompensationFilterUnitsItems::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationFilterUnitsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationFilterUnitsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationFilterUnitsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPENSATION_UNIT_UNSPECIFIED" => {
                    CompensationFilterUnitsItems::CompensationUnitUnspecified
                }
                "DAILY" => CompensationFilterUnitsItems::Daily,
                "HOURLY" => CompensationFilterUnitsItems::Hourly,
                "MONTHLY" => CompensationFilterUnitsItems::Monthly,
                "ONE_TIME" => CompensationFilterUnitsItems::OneTime,
                "OTHER_COMPENSATION_UNIT" => CompensationFilterUnitsItems::OtherCompensationUnit,
                "WEEKLY" => CompensationFilterUnitsItems::Weekly,
                "YEARLY" => CompensationFilterUnitsItems::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompensationFilterUnitsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationFilterUnitsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationHistogramRequest {
        #[doc = "Required. Numeric histogram options, like buckets, whether include min or max value."]
        #[serde(
            rename = "bucketingOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucketing_option: ::std::option::Option<crate::schemas::NumericBucketingOption>,
        #[doc = "Required. Type of the request, representing which field the histogramming should be\nperformed over. A single request can only specify one histogram of each\n`CompensationHistogramRequestType`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CompensationHistogramRequestType>,
    }
    impl ::google_field_selector::FieldSelector for CompensationHistogramRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationHistogramRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationHistogramRequestType {
        #[doc = "Histogram by job's annualized base compensation. See CompensationEntry\nfor definition of annualized base compensation."]
        AnnualizedBase,
        #[doc = "Histogram by job's annualized total compensation. See CompensationEntry\nfor definition of annualized total compensation."]
        AnnualizedTotal,
        #[doc = "Histogram by job's base compensation. See CompensationEntry for\ndefinition of base compensation."]
        Base,
        #[doc = "Default value. Invalid."]
        CompensationHistogramRequestTypeUnspecified,
    }
    impl CompensationHistogramRequestType {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationHistogramRequestType::AnnualizedBase => "ANNUALIZED_BASE",
                CompensationHistogramRequestType::AnnualizedTotal => "ANNUALIZED_TOTAL",
                CompensationHistogramRequestType::Base => "BASE",
                CompensationHistogramRequestType::CompensationHistogramRequestTypeUnspecified => {
                    "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationHistogramRequestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationHistogramRequestType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationHistogramRequestType, ()> {
            Ok(match s {
                "ANNUALIZED_BASE" => CompensationHistogramRequestType::AnnualizedBase,
                "ANNUALIZED_TOTAL" => CompensationHistogramRequestType::AnnualizedTotal,
                "BASE" => CompensationHistogramRequestType::Base,
                "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED" => {
                    CompensationHistogramRequestType::CompensationHistogramRequestTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationHistogramRequestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationHistogramRequestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationHistogramRequestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNUALIZED_BASE" => CompensationHistogramRequestType::AnnualizedBase,
                "ANNUALIZED_TOTAL" => CompensationHistogramRequestType::AnnualizedTotal,
                "BASE" => CompensationHistogramRequestType::Base,
                "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED" => {
                    CompensationHistogramRequestType::CompensationHistogramRequestTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for CompensationHistogramRequestType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationHistogramRequestType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationHistogramResult {
        #[doc = "Type of the request, corresponding to\nCompensationHistogramRequest.type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CompensationHistogramResultType>,
        #[doc = "Histogram result."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<crate::schemas::NumericBucketingResult>,
    }
    impl ::google_field_selector::FieldSelector for CompensationHistogramResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationHistogramResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationHistogramResultType {
        #[doc = "Histogram by job's annualized base compensation. See CompensationEntry\nfor definition of annualized base compensation."]
        AnnualizedBase,
        #[doc = "Histogram by job's annualized total compensation. See CompensationEntry\nfor definition of annualized total compensation."]
        AnnualizedTotal,
        #[doc = "Histogram by job's base compensation. See CompensationEntry for\ndefinition of base compensation."]
        Base,
        #[doc = "Default value. Invalid."]
        CompensationHistogramRequestTypeUnspecified,
    }
    impl CompensationHistogramResultType {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationHistogramResultType::AnnualizedBase => "ANNUALIZED_BASE",
                CompensationHistogramResultType::AnnualizedTotal => "ANNUALIZED_TOTAL",
                CompensationHistogramResultType::Base => "BASE",
                CompensationHistogramResultType::CompensationHistogramRequestTypeUnspecified => {
                    "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationHistogramResultType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationHistogramResultType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationHistogramResultType, ()> {
            Ok(match s {
                "ANNUALIZED_BASE" => CompensationHistogramResultType::AnnualizedBase,
                "ANNUALIZED_TOTAL" => CompensationHistogramResultType::AnnualizedTotal,
                "BASE" => CompensationHistogramResultType::Base,
                "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED" => {
                    CompensationHistogramResultType::CompensationHistogramRequestTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationHistogramResultType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationHistogramResultType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationHistogramResultType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNUALIZED_BASE" => CompensationHistogramResultType::AnnualizedBase,
                "ANNUALIZED_TOTAL" => CompensationHistogramResultType::AnnualizedTotal,
                "BASE" => CompensationHistogramResultType::Base,
                "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED" => {
                    CompensationHistogramResultType::CompensationHistogramRequestTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for CompensationHistogramResultType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationHistogramResultType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationInfo {
        #[doc = "Deprecated. Use entries instead.\n\nOptional.\n\nThe amount of compensation or pay for the job.\nAs an alternative, compensation_amount_min and\ncompensation_amount_max may be used to define a range of\ncompensation."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Output only. Annualized base compensation range. Computed as\nbase compensation entry's CompensationEntry.compensation times\nCompensationEntry.expected_units_per_year.\n\nSee CompensationEntry for explanation on compensation annualization."]
        #[serde(
            rename = "annualizedBaseCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_base_compensation_range:
            ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Output only. Annualized total compensation range. Computed as\nall compensation entries' CompensationEntry.compensation times\nCompensationEntry.expected_units_per_year.\n\nSee CompensationEntry for explanation on compensation annualization."]
        #[serde(
            rename = "annualizedTotalCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_total_compensation_range:
            ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Optional. Job compensation information.\n\nAt most one entry can be of type\nCompensationInfo.CompensationType.BASE, which is\nreferred as ** base compensation entry ** for the job."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries: ::std::option::Option<Vec<crate::schemas::CompensationEntry>>,
        #[doc = "Deprecated. Use entries instead.\n\nOptional.\n\nAn upper bound on a range for compensation or pay for the job.\nThe currency type is specified in compensation_amount."]
        #[serde(
            rename = "max",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Deprecated. Use entries instead.\n\nOptional.\n\nA lower bound on a range for compensation or pay for the job.\nThe currency type is specified in compensation_amount."]
        #[serde(
            rename = "min",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Deprecated. Use entries instead.\n\nOptional.\n\nType of job compensation."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CompensationInfoType>,
    }
    impl ::google_field_selector::FieldSelector for CompensationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompensationInfoType {
        #[doc = "The job compensation is quoted based solely on commission."]
        Commission,
        #[doc = "The job compensation is quoted by the number of hours worked."]
        Hourly,
        #[doc = "The default value if the type is not specified."]
        JobCompensationTypeUnspecified,
        #[doc = "The job compensation is not quoted according to the listed compensation\noptions."]
        OtherType,
        #[doc = "The job compensation is quoted by project completion."]
        PerProject,
        #[doc = "The job compensation is quoted on an annual basis."]
        Salary,
    }
    impl CompensationInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                CompensationInfoType::Commission => "COMMISSION",
                CompensationInfoType::Hourly => "HOURLY",
                CompensationInfoType::JobCompensationTypeUnspecified => {
                    "JOB_COMPENSATION_TYPE_UNSPECIFIED"
                }
                CompensationInfoType::OtherType => "OTHER_TYPE",
                CompensationInfoType::PerProject => "PER_PROJECT",
                CompensationInfoType::Salary => "SALARY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompensationInfoType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompensationInfoType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompensationInfoType, ()> {
            Ok(match s {
                "COMMISSION" => CompensationInfoType::Commission,
                "HOURLY" => CompensationInfoType::Hourly,
                "JOB_COMPENSATION_TYPE_UNSPECIFIED" => {
                    CompensationInfoType::JobCompensationTypeUnspecified
                }
                "OTHER_TYPE" => CompensationInfoType::OtherType,
                "PER_PROJECT" => CompensationInfoType::PerProject,
                "SALARY" => CompensationInfoType::Salary,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompensationInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompensationInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompensationInfoType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMISSION" => CompensationInfoType::Commission,
                "HOURLY" => CompensationInfoType::Hourly,
                "JOB_COMPENSATION_TYPE_UNSPECIFIED" => {
                    CompensationInfoType::JobCompensationTypeUnspecified
                }
                "OTHER_TYPE" => CompensationInfoType::OtherType,
                "PER_PROJECT" => CompensationInfoType::PerProject,
                "SALARY" => CompensationInfoType::Salary,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompensationInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationInfoType {
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
    pub struct CompensationRange {
        #[doc = "Optional. The maximum amount of compensation. If left empty, the value is set\nto a maximal compensation value and the currency code is set to\nmatch the currency code of\nmin_compensation."]
        #[serde(
            rename = "max",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Optional. The minimum amount of compensation. If left empty, the value is set\nto zero and the currency code is set to match the\ncurrency code of max_compensation."]
        #[serde(
            rename = "min",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min: ::std::option::Option<crate::schemas::Money>,
    }
    impl ::google_field_selector::FieldSelector for CompensationRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompensationRange {
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
    pub struct CompleteQueryResponse {
        #[doc = "Results of the matching job/company candidates."]
        #[serde(
            rename = "completionResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completion_results: ::std::option::Option<Vec<crate::schemas::CompletionResult>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
    }
    impl ::google_field_selector::FieldSelector for CompleteQueryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompleteQueryResponse {
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
    pub struct CompletionResult {
        #[doc = "The URL for the company logo if `type=COMPANY_NAME`."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The completion topic."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CompletionResultType>,
        #[doc = "The suggestion for the query."]
        #[serde(
            rename = "suggestion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggestion: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CompletionResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompletionResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompletionResultType {
        #[doc = "Suggest both job titles and company names."]
        Combined,
        #[doc = "Only suggest company names."]
        CompanyName,
        #[doc = "Default value."]
        CompletionTypeUnspecified,
        #[doc = "Only suggest job titles."]
        JobTitle,
    }
    impl CompletionResultType {
        pub fn as_str(self) -> &'static str {
            match self {
                CompletionResultType::Combined => "COMBINED",
                CompletionResultType::CompanyName => "COMPANY_NAME",
                CompletionResultType::CompletionTypeUnspecified => "COMPLETION_TYPE_UNSPECIFIED",
                CompletionResultType::JobTitle => "JOB_TITLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompletionResultType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompletionResultType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompletionResultType, ()> {
            Ok(match s {
                "COMBINED" => CompletionResultType::Combined,
                "COMPANY_NAME" => CompletionResultType::CompanyName,
                "COMPLETION_TYPE_UNSPECIFIED" => CompletionResultType::CompletionTypeUnspecified,
                "JOB_TITLE" => CompletionResultType::JobTitle,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompletionResultType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompletionResultType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompletionResultType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMBINED" => CompletionResultType::Combined,
                "COMPANY_NAME" => CompletionResultType::CompanyName,
                "COMPLETION_TYPE_UNSPECIFIED" => CompletionResultType::CompletionTypeUnspecified,
                "JOB_TITLE" => CompletionResultType::JobTitle,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompletionResultType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompletionResultType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateJobRequest {
        #[doc = "Deprecated. Please use processing_options. This flag is ignored if\nprocessing_options is set.\n\nOptional.\n\nIf set to `true`, the service does not attempt to resolve a\nmore precise address for the job."]
        #[serde(
            rename = "disableStreetAddressResolution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_street_address_resolution: ::std::option::Option<bool>,
        #[doc = "Required. The Job to be created."]
        #[serde(
            rename = "job",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "Optional. Options for job processing."]
        #[serde(
            rename = "processingOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processing_options: ::std::option::Option<crate::schemas::JobProcessingOptions>,
    }
    impl ::google_field_selector::FieldSelector for CreateJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateJobRequest {
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
    pub struct CustomAttribute {
        #[doc = "Optional. If the `filterable` flag is true, custom field values are searchable.\nIf false, values are not searchable.\n\nDefault is false."]
        #[serde(
            rename = "filterable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filterable: ::std::option::Option<bool>,
        #[doc = "Optional but at least one of string_values or long_value must\nbe specified.\n\nThis field is used to perform number range search.\n(`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`. For\n`long_value`, a value between Long.MIN and Long.MAX is allowed."]
        #[serde(
            rename = "longValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub long_value: ::std::option::Option<i64>,
        #[doc = "Optional but at least one of string_values or long_value must\nbe specified.\n\nThis field is used to perform a string match (`CASE_SENSITIVE_MATCH` or\n`CASE_INSENSITIVE_MATCH`) search.\nFor filterable `string_values`, a maximum total number of 200 values\nis allowed, with each `string_value` has a byte size of no more than\n255B. For unfilterable `string_values`, the maximum total byte size of\nunfilterable `string_values` is 50KB.\n\nEmpty strings are not allowed."]
        #[serde(
            rename = "stringValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_values: ::std::option::Option<crate::schemas::StringValues>,
    }
    impl ::google_field_selector::FieldSelector for CustomAttribute {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomAttribute {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomAttributeHistogramRequest {
        #[doc = "Required. Specifies the custom field key to perform a histogram on. If specified\nwithout `long_value_histogram_bucketing_option`, histogram on string values\nof the given `key` is triggered, otherwise histogram is performed on long\nvalues."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Optional. Specifies buckets used to perform a range histogram on Job's\nfilterable long custom field values, or min/max value requirements."]
        #[serde(
            rename = "longValueHistogramBucketingOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_value_histogram_bucketing_option:
            ::std::option::Option<crate::schemas::NumericBucketingOption>,
        #[doc = "Optional. If set to true, the response will include the histogram value for\neach key as a string."]
        #[serde(
            rename = "stringValueHistogram",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value_histogram: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CustomAttributeHistogramRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomAttributeHistogramRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomAttributeHistogramResult {
        #[doc = "Stores the key of custom attribute the histogram is performed on."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Stores bucketed histogram counting result or min/max values for\ncustom attribute long values associated with `key`."]
        #[serde(
            rename = "longValueHistogramResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_value_histogram_result:
            ::std::option::Option<crate::schemas::NumericBucketingResult>,
        #[doc = "Stores a map from the values of string custom field associated\nwith `key` to the number of jobs with that value in this histogram result."]
        #[serde(
            rename = "stringValueHistogramResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value_histogram_result:
            ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
    }
    impl ::google_field_selector::FieldSelector for CustomAttributeHistogramResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomAttributeHistogramResult {
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
    pub struct CustomField {
        #[doc = "Optional. The values of the custom data."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CustomField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomField {
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
    pub struct CustomFieldFilter {
        #[doc = "Required. The query strings for the filter."]
        #[serde(
            rename = "queries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queries: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The type of filter.\nDefaults to FilterType.OR."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CustomFieldFilterType>,
    }
    impl ::google_field_selector::FieldSelector for CustomFieldFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomFieldFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CustomFieldFilterType {
        #[doc = "Search for a match with all queries."]
        And,
        #[doc = "Default value."]
        FilterTypeUnspecified,
        #[doc = "Negate the set of filter values for the search."]
        Not,
        #[doc = "Search for a match with any query."]
        Or,
    }
    impl CustomFieldFilterType {
        pub fn as_str(self) -> &'static str {
            match self {
                CustomFieldFilterType::And => "AND",
                CustomFieldFilterType::FilterTypeUnspecified => "FILTER_TYPE_UNSPECIFIED",
                CustomFieldFilterType::Not => "NOT",
                CustomFieldFilterType::Or => "OR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CustomFieldFilterType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CustomFieldFilterType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CustomFieldFilterType, ()> {
            Ok(match s {
                "AND" => CustomFieldFilterType::And,
                "FILTER_TYPE_UNSPECIFIED" => CustomFieldFilterType::FilterTypeUnspecified,
                "NOT" => CustomFieldFilterType::Not,
                "OR" => CustomFieldFilterType::Or,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CustomFieldFilterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CustomFieldFilterType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CustomFieldFilterType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => CustomFieldFilterType::And,
                "FILTER_TYPE_UNSPECIFIED" => CustomFieldFilterType::FilterTypeUnspecified,
                "NOT" => CustomFieldFilterType::Not,
                "OR" => CustomFieldFilterType::Or,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CustomFieldFilterType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomFieldFilterType {
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
    pub struct Date {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month, or 0\nif specifying a year by itself or a year and month where the day is not\nsignificant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of year. Must be from 1 to 12, or 0 if specifying a year without a\nmonth and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999, or 0 if specifying a date without\na year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Date {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Date {
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
    pub struct DeleteJobsByFilterRequest {
        #[doc = "Optional. If set to true, this call waits for all processing steps to complete\nbefore the job is cleaned up. Otherwise, the call returns while some\nsteps are still taking place asynchronously, hence faster."]
        #[serde(
            rename = "disableFastProcess",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_fast_process: ::std::option::Option<bool>,
        #[doc = "Required. Restrictions on the scope of the delete request."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::Filter>,
    }
    impl ::google_field_selector::FieldSelector for DeleteJobsByFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteJobsByFilterRequest {
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
    pub struct DeviceInfo {
        #[doc = "Optional. Type of the device."]
        #[serde(
            rename = "deviceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_type: ::std::option::Option<crate::schemas::DeviceInfoDeviceType>,
        #[doc = "Optional. A device-specific ID. The ID must be a unique identifier that distinguishes\nthe device from other devices."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeviceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceInfoDeviceType {
        #[doc = "An Android device native application."]
        Android,
        #[doc = "A bot, as opposed to a device operated by human beings, such as a web\ncrawler."]
        Bot,
        #[doc = "The device type isn't specified."]
        DeviceTypeUnspecified,
        #[doc = "An iOS device native application."]
        Ios,
        #[doc = "A mobile device web browser, such as a phone or tablet with a Chrome\nbrowser."]
        MobileWeb,
        #[doc = "Other devices types."]
        Other,
        #[doc = "A desktop web browser, such as, Chrome, Firefox, Safari, or Internet\nExplorer)"]
        Web,
    }
    impl DeviceInfoDeviceType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceInfoDeviceType::Android => "ANDROID",
                DeviceInfoDeviceType::Bot => "BOT",
                DeviceInfoDeviceType::DeviceTypeUnspecified => "DEVICE_TYPE_UNSPECIFIED",
                DeviceInfoDeviceType::Ios => "IOS",
                DeviceInfoDeviceType::MobileWeb => "MOBILE_WEB",
                DeviceInfoDeviceType::Other => "OTHER",
                DeviceInfoDeviceType::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceInfoDeviceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceInfoDeviceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceInfoDeviceType, ()> {
            Ok(match s {
                "ANDROID" => DeviceInfoDeviceType::Android,
                "BOT" => DeviceInfoDeviceType::Bot,
                "DEVICE_TYPE_UNSPECIFIED" => DeviceInfoDeviceType::DeviceTypeUnspecified,
                "IOS" => DeviceInfoDeviceType::Ios,
                "MOBILE_WEB" => DeviceInfoDeviceType::MobileWeb,
                "OTHER" => DeviceInfoDeviceType::Other,
                "WEB" => DeviceInfoDeviceType::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceInfoDeviceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceInfoDeviceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceInfoDeviceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => DeviceInfoDeviceType::Android,
                "BOT" => DeviceInfoDeviceType::Bot,
                "DEVICE_TYPE_UNSPECIFIED" => DeviceInfoDeviceType::DeviceTypeUnspecified,
                "IOS" => DeviceInfoDeviceType::Ios,
                "MOBILE_WEB" => DeviceInfoDeviceType::MobileWeb,
                "OTHER" => DeviceInfoDeviceType::Other,
                "WEB" => DeviceInfoDeviceType::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceInfoDeviceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceInfoDeviceType {
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
    pub struct ExtendedCompensationFilter {
        #[doc = "Optional. Compensation range."]
        #[serde(
            rename = "compensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_range:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfoCompensationRange>,
        #[doc = "Required. Specify desired `base compensation entry's`\nExtendedCompensationInfo.CompensationUnit."]
        #[serde(
            rename = "compensationUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_units: ::std::option::Option<
            Vec<crate::schemas::ExtendedCompensationFilterCompensationUnitsItems>,
        >,
        #[doc = "Optional. Specify currency in 3-letter\n[ISO 4217](https://www.iso.org/iso-4217-currency-codes.html) format. If\nunspecified, jobs are returned regardless of currency."]
        #[serde(
            rename = "currency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency: ::std::option::Option<String>,
        #[doc = "Optional. Whether to include jobs whose compensation range is unspecified."]
        #[serde(
            rename = "includeJobWithUnspecifiedCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_job_with_unspecified_compensation_range: ::std::option::Option<bool>,
        #[doc = "Required. Type of filter."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ExtendedCompensationFilterType>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExtendedCompensationFilterCompensationUnitsItems {
        Daily,
        ExtendedCompensationUnitUnspecified,
        Hourly,
        Monthly,
        OneTime,
        OtherCompensationUnit,
        Weekly,
        Yearly,
    }
    impl ExtendedCompensationFilterCompensationUnitsItems {
        pub fn as_str(self) -> &'static str {
            match self { ExtendedCompensationFilterCompensationUnitsItems :: Daily => "DAILY" , ExtendedCompensationFilterCompensationUnitsItems :: ExtendedCompensationUnitUnspecified => "EXTENDED_COMPENSATION_UNIT_UNSPECIFIED" , ExtendedCompensationFilterCompensationUnitsItems :: Hourly => "HOURLY" , ExtendedCompensationFilterCompensationUnitsItems :: Monthly => "MONTHLY" , ExtendedCompensationFilterCompensationUnitsItems :: OneTime => "ONE_TIME" , ExtendedCompensationFilterCompensationUnitsItems :: OtherCompensationUnit => "OTHER_COMPENSATION_UNIT" , ExtendedCompensationFilterCompensationUnitsItems :: Weekly => "WEEKLY" , ExtendedCompensationFilterCompensationUnitsItems :: Yearly => "YEARLY" , }
        }
    }
    impl ::std::convert::AsRef<str> for ExtendedCompensationFilterCompensationUnitsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExtendedCompensationFilterCompensationUnitsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ExtendedCompensationFilterCompensationUnitsItems, ()> {
            Ok ( match s { "DAILY" => ExtendedCompensationFilterCompensationUnitsItems :: Daily , "EXTENDED_COMPENSATION_UNIT_UNSPECIFIED" => ExtendedCompensationFilterCompensationUnitsItems :: ExtendedCompensationUnitUnspecified , "HOURLY" => ExtendedCompensationFilterCompensationUnitsItems :: Hourly , "MONTHLY" => ExtendedCompensationFilterCompensationUnitsItems :: Monthly , "ONE_TIME" => ExtendedCompensationFilterCompensationUnitsItems :: OneTime , "OTHER_COMPENSATION_UNIT" => ExtendedCompensationFilterCompensationUnitsItems :: OtherCompensationUnit , "WEEKLY" => ExtendedCompensationFilterCompensationUnitsItems :: Weekly , "YEARLY" => ExtendedCompensationFilterCompensationUnitsItems :: Yearly , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for ExtendedCompensationFilterCompensationUnitsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExtendedCompensationFilterCompensationUnitsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExtendedCompensationFilterCompensationUnitsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "DAILY" => ExtendedCompensationFilterCompensationUnitsItems :: Daily , "EXTENDED_COMPENSATION_UNIT_UNSPECIFIED" => ExtendedCompensationFilterCompensationUnitsItems :: ExtendedCompensationUnitUnspecified , "HOURLY" => ExtendedCompensationFilterCompensationUnitsItems :: Hourly , "MONTHLY" => ExtendedCompensationFilterCompensationUnitsItems :: Monthly , "ONE_TIME" => ExtendedCompensationFilterCompensationUnitsItems :: OneTime , "OTHER_COMPENSATION_UNIT" => ExtendedCompensationFilterCompensationUnitsItems :: OtherCompensationUnit , "WEEKLY" => ExtendedCompensationFilterCompensationUnitsItems :: Weekly , "YEARLY" => ExtendedCompensationFilterCompensationUnitsItems :: Yearly , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationFilterCompensationUnitsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationFilterCompensationUnitsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExtendedCompensationFilterType {
        #[doc = "Filter by annualized base compensation amount and `base compensation entry's` unit. Populate compensation_range and zero or more\ncompensation_units."]
        AnnualizedBaseAmount,
        #[doc = "Filter by annualized total compensation amount and `base compensation entry's` unit . Populate compensation_range and zero or more\ncompensation_units."]
        AnnualizedTotalAmount,
        #[doc = "Filter type unspecified. Position holder, INVALID, should never be used."]
        FilterTypeUnspecified,
        #[doc = "Filter by `base compensation entry's` unit and amount / range. A job\nis a match if and only if the job contains a base CompensationEntry, and\nthe base entry's unit matches provided compensation_units and amount\nor range overlaps with provided compensation_range.\n\nSee ExtendedCompensationInfo.CompensationEntry for definition of\nbase compensation entry.\n\nSet exactly one\ncompensation_units and populate\ncompensation_range."]
        UnitAndAmount,
        #[doc = "Filter by `base compensation entry's` unit. A job is a match if and\nonly if the job contains a base CompensationEntry and the base\nCompensationEntry's unit matches provided compensation_units.\nPopulate one or more compensation_units.\n\nSee ExtendedCompensationInfo.CompensationEntry for definition of\nbase compensation entry."]
        UnitOnly,
    }
    impl ExtendedCompensationFilterType {
        pub fn as_str(self) -> &'static str {
            match self {
                ExtendedCompensationFilterType::AnnualizedBaseAmount => "ANNUALIZED_BASE_AMOUNT",
                ExtendedCompensationFilterType::AnnualizedTotalAmount => "ANNUALIZED_TOTAL_AMOUNT",
                ExtendedCompensationFilterType::FilterTypeUnspecified => "FILTER_TYPE_UNSPECIFIED",
                ExtendedCompensationFilterType::UnitAndAmount => "UNIT_AND_AMOUNT",
                ExtendedCompensationFilterType::UnitOnly => "UNIT_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ExtendedCompensationFilterType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExtendedCompensationFilterType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ExtendedCompensationFilterType, ()> {
            Ok(match s {
                "ANNUALIZED_BASE_AMOUNT" => ExtendedCompensationFilterType::AnnualizedBaseAmount,
                "ANNUALIZED_TOTAL_AMOUNT" => ExtendedCompensationFilterType::AnnualizedTotalAmount,
                "FILTER_TYPE_UNSPECIFIED" => ExtendedCompensationFilterType::FilterTypeUnspecified,
                "UNIT_AND_AMOUNT" => ExtendedCompensationFilterType::UnitAndAmount,
                "UNIT_ONLY" => ExtendedCompensationFilterType::UnitOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ExtendedCompensationFilterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExtendedCompensationFilterType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExtendedCompensationFilterType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNUALIZED_BASE_AMOUNT" => ExtendedCompensationFilterType::AnnualizedBaseAmount,
                "ANNUALIZED_TOTAL_AMOUNT" => ExtendedCompensationFilterType::AnnualizedTotalAmount,
                "FILTER_TYPE_UNSPECIFIED" => ExtendedCompensationFilterType::FilterTypeUnspecified,
                "UNIT_AND_AMOUNT" => ExtendedCompensationFilterType::UnitAndAmount,
                "UNIT_ONLY" => ExtendedCompensationFilterType::UnitOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationFilterType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationFilterType {
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
    pub struct ExtendedCompensationInfo {
        #[doc = "Output only. Annualized base compensation range."]
        #[serde(
            rename = "annualizedBaseCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_base_compensation_range:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfoCompensationRange>,
        #[doc = "Output only. Indicates annualized base compensation range cannot be derived, due to\nthe job's base compensation entry cannot be annualized.\nSee CompensationEntry for explanation on annualization and base\ncompensation entry."]
        #[serde(
            rename = "annualizedBaseCompensationUnspecified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_base_compensation_unspecified: ::std::option::Option<bool>,
        #[doc = "Output only. Annualized total compensation range."]
        #[serde(
            rename = "annualizedTotalCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_total_compensation_range:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfoCompensationRange>,
        #[doc = "Output only. Indicates annualized total compensation range cannot be derived, due to\nthe job's all CompensationEntry cannot be annualized.\nSee CompensationEntry for explanation on annualization and base\ncompensation entry."]
        #[serde(
            rename = "annualizedTotalCompensationUnspecified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_total_compensation_unspecified: ::std::option::Option<bool>,
        #[doc = "Optional. A 3-letter [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html)\ncurrency code."]
        #[serde(
            rename = "currency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency: ::std::option::Option<String>,
        #[doc = "Optional. Job compensation information.\n\nAt most one entry can be of type\nExtendedCompensationInfo.CompensationType.BASE, which is\nreferred as ** base compensation entry ** for the job."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries:
            ::std::option::Option<Vec<crate::schemas::ExtendedCompensationInfoCompensationEntry>>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationInfo {
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
    pub struct ExtendedCompensationInfoCompensationEntry {
        #[doc = "Optional. Monetary amount."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount: ::std::option::Option<crate::schemas::ExtendedCompensationInfoDecimal>,
        #[doc = "Optional. Compensation description."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. Expected number of units paid each year. If not specified, when\nJob.employment_types is FULLTIME, a default value is inferred\nbased on unit. Default values:\n\n* HOURLY: 2080\n* DAILY: 260\n* WEEKLY: 52\n* MONTHLY: 12\n* ANNUAL: 1"]
        #[serde(
            rename = "expectedUnitsPerYear",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expected_units_per_year:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfoDecimal>,
        #[doc = "Required. Compensation type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfoCompensationEntryType>,
        #[doc = "Optional. Compensation range."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::ExtendedCompensationInfoCompensationRange>,
        #[doc = "Optional. Frequency of the specified amount.\n\nDefault is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfoCompensationEntryUnit>,
        #[doc = "Optional. Indicates compensation amount and range are unset."]
        #[serde(
            rename = "unspecified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unspecified: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationInfoCompensationEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationInfoCompensationEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExtendedCompensationInfoCompensationEntryType {
        #[doc = "Base compensation: Refers to the fixed amount of money paid to an\nemployee by an employer in return for work performed. Base compensation\ndoes not include benefits, bonuses or any other potential compensation\nfrom an employer."]
        Base,
        #[doc = "Bonus."]
        Bonus,
        #[doc = "Commission."]
        Commissions,
        #[doc = "Equity."]
        Equity,
        #[doc = "Default value. Equivalent to OTHER_COMPENSATION_TYPE."]
        ExtendedCompensationTypeUnspecified,
        #[doc = "Other compensation type."]
        OtherCompensationType,
        #[doc = "Profit sharing."]
        ProfitSharing,
        #[doc = "Signing bonus."]
        SigningBonus,
        #[doc = "Tips."]
        Tips,
    }
    impl ExtendedCompensationInfoCompensationEntryType {
        pub fn as_str(self) -> &'static str {
            match self { ExtendedCompensationInfoCompensationEntryType :: Base => "BASE" , ExtendedCompensationInfoCompensationEntryType :: Bonus => "BONUS" , ExtendedCompensationInfoCompensationEntryType :: Commissions => "COMMISSIONS" , ExtendedCompensationInfoCompensationEntryType :: Equity => "EQUITY" , ExtendedCompensationInfoCompensationEntryType :: ExtendedCompensationTypeUnspecified => "EXTENDED_COMPENSATION_TYPE_UNSPECIFIED" , ExtendedCompensationInfoCompensationEntryType :: OtherCompensationType => "OTHER_COMPENSATION_TYPE" , ExtendedCompensationInfoCompensationEntryType :: ProfitSharing => "PROFIT_SHARING" , ExtendedCompensationInfoCompensationEntryType :: SigningBonus => "SIGNING_BONUS" , ExtendedCompensationInfoCompensationEntryType :: Tips => "TIPS" , }
        }
    }
    impl ::std::convert::AsRef<str> for ExtendedCompensationInfoCompensationEntryType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExtendedCompensationInfoCompensationEntryType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ExtendedCompensationInfoCompensationEntryType, ()> {
            Ok ( match s { "BASE" => ExtendedCompensationInfoCompensationEntryType :: Base , "BONUS" => ExtendedCompensationInfoCompensationEntryType :: Bonus , "COMMISSIONS" => ExtendedCompensationInfoCompensationEntryType :: Commissions , "EQUITY" => ExtendedCompensationInfoCompensationEntryType :: Equity , "EXTENDED_COMPENSATION_TYPE_UNSPECIFIED" => ExtendedCompensationInfoCompensationEntryType :: ExtendedCompensationTypeUnspecified , "OTHER_COMPENSATION_TYPE" => ExtendedCompensationInfoCompensationEntryType :: OtherCompensationType , "PROFIT_SHARING" => ExtendedCompensationInfoCompensationEntryType :: ProfitSharing , "SIGNING_BONUS" => ExtendedCompensationInfoCompensationEntryType :: SigningBonus , "TIPS" => ExtendedCompensationInfoCompensationEntryType :: Tips , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for ExtendedCompensationInfoCompensationEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExtendedCompensationInfoCompensationEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExtendedCompensationInfoCompensationEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "BASE" => ExtendedCompensationInfoCompensationEntryType :: Base , "BONUS" => ExtendedCompensationInfoCompensationEntryType :: Bonus , "COMMISSIONS" => ExtendedCompensationInfoCompensationEntryType :: Commissions , "EQUITY" => ExtendedCompensationInfoCompensationEntryType :: Equity , "EXTENDED_COMPENSATION_TYPE_UNSPECIFIED" => ExtendedCompensationInfoCompensationEntryType :: ExtendedCompensationTypeUnspecified , "OTHER_COMPENSATION_TYPE" => ExtendedCompensationInfoCompensationEntryType :: OtherCompensationType , "PROFIT_SHARING" => ExtendedCompensationInfoCompensationEntryType :: ProfitSharing , "SIGNING_BONUS" => ExtendedCompensationInfoCompensationEntryType :: SigningBonus , "TIPS" => ExtendedCompensationInfoCompensationEntryType :: Tips , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationInfoCompensationEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationInfoCompensationEntryType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExtendedCompensationInfoCompensationEntryUnit {
        #[doc = "Daily."]
        Daily,
        #[doc = "Default value. Equivalent to OTHER_COMPENSATION_UNIT."]
        ExtendedCompensationUnitUnspecified,
        #[doc = "Hourly."]
        Hourly,
        #[doc = "Monthly."]
        Monthly,
        #[doc = "One time."]
        OneTime,
        #[doc = "Other compensation units."]
        OtherCompensationUnit,
        #[doc = "Weekly"]
        Weekly,
        #[doc = "Yearly."]
        Yearly,
    }
    impl ExtendedCompensationInfoCompensationEntryUnit {
        pub fn as_str(self) -> &'static str {
            match self { ExtendedCompensationInfoCompensationEntryUnit :: Daily => "DAILY" , ExtendedCompensationInfoCompensationEntryUnit :: ExtendedCompensationUnitUnspecified => "EXTENDED_COMPENSATION_UNIT_UNSPECIFIED" , ExtendedCompensationInfoCompensationEntryUnit :: Hourly => "HOURLY" , ExtendedCompensationInfoCompensationEntryUnit :: Monthly => "MONTHLY" , ExtendedCompensationInfoCompensationEntryUnit :: OneTime => "ONE_TIME" , ExtendedCompensationInfoCompensationEntryUnit :: OtherCompensationUnit => "OTHER_COMPENSATION_UNIT" , ExtendedCompensationInfoCompensationEntryUnit :: Weekly => "WEEKLY" , ExtendedCompensationInfoCompensationEntryUnit :: Yearly => "YEARLY" , }
        }
    }
    impl ::std::convert::AsRef<str> for ExtendedCompensationInfoCompensationEntryUnit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExtendedCompensationInfoCompensationEntryUnit {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ExtendedCompensationInfoCompensationEntryUnit, ()> {
            Ok ( match s { "DAILY" => ExtendedCompensationInfoCompensationEntryUnit :: Daily , "EXTENDED_COMPENSATION_UNIT_UNSPECIFIED" => ExtendedCompensationInfoCompensationEntryUnit :: ExtendedCompensationUnitUnspecified , "HOURLY" => ExtendedCompensationInfoCompensationEntryUnit :: Hourly , "MONTHLY" => ExtendedCompensationInfoCompensationEntryUnit :: Monthly , "ONE_TIME" => ExtendedCompensationInfoCompensationEntryUnit :: OneTime , "OTHER_COMPENSATION_UNIT" => ExtendedCompensationInfoCompensationEntryUnit :: OtherCompensationUnit , "WEEKLY" => ExtendedCompensationInfoCompensationEntryUnit :: Weekly , "YEARLY" => ExtendedCompensationInfoCompensationEntryUnit :: Yearly , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for ExtendedCompensationInfoCompensationEntryUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExtendedCompensationInfoCompensationEntryUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExtendedCompensationInfoCompensationEntryUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "DAILY" => ExtendedCompensationInfoCompensationEntryUnit :: Daily , "EXTENDED_COMPENSATION_UNIT_UNSPECIFIED" => ExtendedCompensationInfoCompensationEntryUnit :: ExtendedCompensationUnitUnspecified , "HOURLY" => ExtendedCompensationInfoCompensationEntryUnit :: Hourly , "MONTHLY" => ExtendedCompensationInfoCompensationEntryUnit :: Monthly , "ONE_TIME" => ExtendedCompensationInfoCompensationEntryUnit :: OneTime , "OTHER_COMPENSATION_UNIT" => ExtendedCompensationInfoCompensationEntryUnit :: OtherCompensationUnit , "WEEKLY" => ExtendedCompensationInfoCompensationEntryUnit :: Weekly , "YEARLY" => ExtendedCompensationInfoCompensationEntryUnit :: Yearly , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationInfoCompensationEntryUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationInfoCompensationEntryUnit {
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
    pub struct ExtendedCompensationInfoCompensationRange {
        #[doc = "Required. Maximum value."]
        #[serde(
            rename = "max",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max: ::std::option::Option<crate::schemas::ExtendedCompensationInfoDecimal>,
        #[doc = "Required. Minimum value."]
        #[serde(
            rename = "min",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min: ::std::option::Option<crate::schemas::ExtendedCompensationInfoDecimal>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationInfoCompensationRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationInfoCompensationRange {
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
    pub struct ExtendedCompensationInfoDecimal {
        #[doc = "Micro (10^-6) units.\nThe value must be between -999,999 and +999,999 inclusive.\nIf `units` is positive, `micros` must be positive or zero.\nIf `units` is zero, `micros` can be positive, zero, or negative.\nIf `units` is negative, `micros` must be negative or zero.\nFor example -1.75 is represented as `units`=-1 and `micros`=-750,000."]
        #[serde(
            rename = "micros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub micros: ::std::option::Option<i32>,
        #[doc = "Whole units."]
        #[serde(
            rename = "units",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub units: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedCompensationInfoDecimal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedCompensationInfoDecimal {
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
    pub struct Filter {
        #[doc = "Required. The requisition ID (or posting ID) assigned by the client to identify a\njob. This is intended for client identification and tracking of\nlistings.\nname takes precedence over this field\nThe maximum number of allowed characters is 225."]
        #[serde(
            rename = "requisitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requisition_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Filter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Filter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GetHistogramRequest {
        #[doc = "Optional. Controls whether to broaden the search to avoid too few results for a\ngiven query in instances where a search has sparse results. Results from a\nbroadened query is a superset of the results from the original query.\n\nDefaults to false."]
        #[serde(
            rename = "allowBroadening",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_broadening: ::std::option::Option<bool>,
        #[doc = "Deprecated. Use query instead.\n\nOptional.\n\nRestrictions on the scope of the histogram."]
        #[serde(
            rename = "filters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filters: ::std::option::Option<crate::schemas::JobFilters>,
        #[doc = "Optional. Query used to search against jobs, such as keyword, location filters, etc."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<crate::schemas::JobQuery>,
        #[doc = "Meta information, such as `user_id`, collected from the job searcher or\nother entity conducting a job search, is used to improve the service's\nsearch quality. Users determine identifier values, which must be\nunique and consist."]
        #[serde(
            rename = "requestMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_metadata: ::std::option::Option<crate::schemas::RequestMetadata>,
        #[doc = "Required. A list of facets that specify the histogram data to be calculated\nagainst and returned.\n\nHistogram response times can be slow, and counts\ncan be approximations. This call may be temporarily or permanently removed\nprior to the production release of Cloud Talent Solution."]
        #[serde(
            rename = "searchTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_types:
            ::std::option::Option<Vec<crate::schemas::GetHistogramRequestSearchTypesItems>>,
    }
    impl ::google_field_selector::FieldSelector for GetHistogramRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetHistogramRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GetHistogramRequestSearchTypesItems {
        Admin1,
        Admin1Country,
        BaseCompensationUnit,
        Category,
        City,
        CityCoordinate,
        CompanyDisplayName,
        CompanyId,
        CompanySize,
        CompanyTitle,
        Country,
        CustomField1,
        CustomField10,
        CustomField11,
        CustomField12,
        CustomField13,
        CustomField14,
        CustomField15,
        CustomField16,
        CustomField17,
        CustomField18,
        CustomField19,
        CustomField2,
        CustomField20,
        CustomField3,
        CustomField4,
        CustomField5,
        CustomField6,
        CustomField7,
        CustomField8,
        CustomField9,
        DatePublished,
        EducationLevel,
        EmploymentType,
        ExperienceLevel,
        JobFieldUnspecified,
        Language,
        Locale,
    }
    impl GetHistogramRequestSearchTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GetHistogramRequestSearchTypesItems::Admin1 => "ADMIN1",
                GetHistogramRequestSearchTypesItems::Admin1Country => "ADMIN1_COUNTRY",
                GetHistogramRequestSearchTypesItems::BaseCompensationUnit => {
                    "BASE_COMPENSATION_UNIT"
                }
                GetHistogramRequestSearchTypesItems::Category => "CATEGORY",
                GetHistogramRequestSearchTypesItems::City => "CITY",
                GetHistogramRequestSearchTypesItems::CityCoordinate => "CITY_COORDINATE",
                GetHistogramRequestSearchTypesItems::CompanyDisplayName => "COMPANY_DISPLAY_NAME",
                GetHistogramRequestSearchTypesItems::CompanyId => "COMPANY_ID",
                GetHistogramRequestSearchTypesItems::CompanySize => "COMPANY_SIZE",
                GetHistogramRequestSearchTypesItems::CompanyTitle => "COMPANY_TITLE",
                GetHistogramRequestSearchTypesItems::Country => "COUNTRY",
                GetHistogramRequestSearchTypesItems::CustomField1 => "CUSTOM_FIELD_1",
                GetHistogramRequestSearchTypesItems::CustomField10 => "CUSTOM_FIELD_10",
                GetHistogramRequestSearchTypesItems::CustomField11 => "CUSTOM_FIELD_11",
                GetHistogramRequestSearchTypesItems::CustomField12 => "CUSTOM_FIELD_12",
                GetHistogramRequestSearchTypesItems::CustomField13 => "CUSTOM_FIELD_13",
                GetHistogramRequestSearchTypesItems::CustomField14 => "CUSTOM_FIELD_14",
                GetHistogramRequestSearchTypesItems::CustomField15 => "CUSTOM_FIELD_15",
                GetHistogramRequestSearchTypesItems::CustomField16 => "CUSTOM_FIELD_16",
                GetHistogramRequestSearchTypesItems::CustomField17 => "CUSTOM_FIELD_17",
                GetHistogramRequestSearchTypesItems::CustomField18 => "CUSTOM_FIELD_18",
                GetHistogramRequestSearchTypesItems::CustomField19 => "CUSTOM_FIELD_19",
                GetHistogramRequestSearchTypesItems::CustomField2 => "CUSTOM_FIELD_2",
                GetHistogramRequestSearchTypesItems::CustomField20 => "CUSTOM_FIELD_20",
                GetHistogramRequestSearchTypesItems::CustomField3 => "CUSTOM_FIELD_3",
                GetHistogramRequestSearchTypesItems::CustomField4 => "CUSTOM_FIELD_4",
                GetHistogramRequestSearchTypesItems::CustomField5 => "CUSTOM_FIELD_5",
                GetHistogramRequestSearchTypesItems::CustomField6 => "CUSTOM_FIELD_6",
                GetHistogramRequestSearchTypesItems::CustomField7 => "CUSTOM_FIELD_7",
                GetHistogramRequestSearchTypesItems::CustomField8 => "CUSTOM_FIELD_8",
                GetHistogramRequestSearchTypesItems::CustomField9 => "CUSTOM_FIELD_9",
                GetHistogramRequestSearchTypesItems::DatePublished => "DATE_PUBLISHED",
                GetHistogramRequestSearchTypesItems::EducationLevel => "EDUCATION_LEVEL",
                GetHistogramRequestSearchTypesItems::EmploymentType => "EMPLOYMENT_TYPE",
                GetHistogramRequestSearchTypesItems::ExperienceLevel => "EXPERIENCE_LEVEL",
                GetHistogramRequestSearchTypesItems::JobFieldUnspecified => "JOB_FIELD_UNSPECIFIED",
                GetHistogramRequestSearchTypesItems::Language => "LANGUAGE",
                GetHistogramRequestSearchTypesItems::Locale => "LOCALE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GetHistogramRequestSearchTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GetHistogramRequestSearchTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GetHistogramRequestSearchTypesItems, ()> {
            Ok(match s {
                "ADMIN1" => GetHistogramRequestSearchTypesItems::Admin1,
                "ADMIN1_COUNTRY" => GetHistogramRequestSearchTypesItems::Admin1Country,
                "BASE_COMPENSATION_UNIT" => {
                    GetHistogramRequestSearchTypesItems::BaseCompensationUnit
                }
                "CATEGORY" => GetHistogramRequestSearchTypesItems::Category,
                "CITY" => GetHistogramRequestSearchTypesItems::City,
                "CITY_COORDINATE" => GetHistogramRequestSearchTypesItems::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => GetHistogramRequestSearchTypesItems::CompanyDisplayName,
                "COMPANY_ID" => GetHistogramRequestSearchTypesItems::CompanyId,
                "COMPANY_SIZE" => GetHistogramRequestSearchTypesItems::CompanySize,
                "COMPANY_TITLE" => GetHistogramRequestSearchTypesItems::CompanyTitle,
                "COUNTRY" => GetHistogramRequestSearchTypesItems::Country,
                "CUSTOM_FIELD_1" => GetHistogramRequestSearchTypesItems::CustomField1,
                "CUSTOM_FIELD_10" => GetHistogramRequestSearchTypesItems::CustomField10,
                "CUSTOM_FIELD_11" => GetHistogramRequestSearchTypesItems::CustomField11,
                "CUSTOM_FIELD_12" => GetHistogramRequestSearchTypesItems::CustomField12,
                "CUSTOM_FIELD_13" => GetHistogramRequestSearchTypesItems::CustomField13,
                "CUSTOM_FIELD_14" => GetHistogramRequestSearchTypesItems::CustomField14,
                "CUSTOM_FIELD_15" => GetHistogramRequestSearchTypesItems::CustomField15,
                "CUSTOM_FIELD_16" => GetHistogramRequestSearchTypesItems::CustomField16,
                "CUSTOM_FIELD_17" => GetHistogramRequestSearchTypesItems::CustomField17,
                "CUSTOM_FIELD_18" => GetHistogramRequestSearchTypesItems::CustomField18,
                "CUSTOM_FIELD_19" => GetHistogramRequestSearchTypesItems::CustomField19,
                "CUSTOM_FIELD_2" => GetHistogramRequestSearchTypesItems::CustomField2,
                "CUSTOM_FIELD_20" => GetHistogramRequestSearchTypesItems::CustomField20,
                "CUSTOM_FIELD_3" => GetHistogramRequestSearchTypesItems::CustomField3,
                "CUSTOM_FIELD_4" => GetHistogramRequestSearchTypesItems::CustomField4,
                "CUSTOM_FIELD_5" => GetHistogramRequestSearchTypesItems::CustomField5,
                "CUSTOM_FIELD_6" => GetHistogramRequestSearchTypesItems::CustomField6,
                "CUSTOM_FIELD_7" => GetHistogramRequestSearchTypesItems::CustomField7,
                "CUSTOM_FIELD_8" => GetHistogramRequestSearchTypesItems::CustomField8,
                "CUSTOM_FIELD_9" => GetHistogramRequestSearchTypesItems::CustomField9,
                "DATE_PUBLISHED" => GetHistogramRequestSearchTypesItems::DatePublished,
                "EDUCATION_LEVEL" => GetHistogramRequestSearchTypesItems::EducationLevel,
                "EMPLOYMENT_TYPE" => GetHistogramRequestSearchTypesItems::EmploymentType,
                "EXPERIENCE_LEVEL" => GetHistogramRequestSearchTypesItems::ExperienceLevel,
                "JOB_FIELD_UNSPECIFIED" => GetHistogramRequestSearchTypesItems::JobFieldUnspecified,
                "LANGUAGE" => GetHistogramRequestSearchTypesItems::Language,
                "LOCALE" => GetHistogramRequestSearchTypesItems::Locale,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GetHistogramRequestSearchTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetHistogramRequestSearchTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetHistogramRequestSearchTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN1" => GetHistogramRequestSearchTypesItems::Admin1,
                "ADMIN1_COUNTRY" => GetHistogramRequestSearchTypesItems::Admin1Country,
                "BASE_COMPENSATION_UNIT" => {
                    GetHistogramRequestSearchTypesItems::BaseCompensationUnit
                }
                "CATEGORY" => GetHistogramRequestSearchTypesItems::Category,
                "CITY" => GetHistogramRequestSearchTypesItems::City,
                "CITY_COORDINATE" => GetHistogramRequestSearchTypesItems::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => GetHistogramRequestSearchTypesItems::CompanyDisplayName,
                "COMPANY_ID" => GetHistogramRequestSearchTypesItems::CompanyId,
                "COMPANY_SIZE" => GetHistogramRequestSearchTypesItems::CompanySize,
                "COMPANY_TITLE" => GetHistogramRequestSearchTypesItems::CompanyTitle,
                "COUNTRY" => GetHistogramRequestSearchTypesItems::Country,
                "CUSTOM_FIELD_1" => GetHistogramRequestSearchTypesItems::CustomField1,
                "CUSTOM_FIELD_10" => GetHistogramRequestSearchTypesItems::CustomField10,
                "CUSTOM_FIELD_11" => GetHistogramRequestSearchTypesItems::CustomField11,
                "CUSTOM_FIELD_12" => GetHistogramRequestSearchTypesItems::CustomField12,
                "CUSTOM_FIELD_13" => GetHistogramRequestSearchTypesItems::CustomField13,
                "CUSTOM_FIELD_14" => GetHistogramRequestSearchTypesItems::CustomField14,
                "CUSTOM_FIELD_15" => GetHistogramRequestSearchTypesItems::CustomField15,
                "CUSTOM_FIELD_16" => GetHistogramRequestSearchTypesItems::CustomField16,
                "CUSTOM_FIELD_17" => GetHistogramRequestSearchTypesItems::CustomField17,
                "CUSTOM_FIELD_18" => GetHistogramRequestSearchTypesItems::CustomField18,
                "CUSTOM_FIELD_19" => GetHistogramRequestSearchTypesItems::CustomField19,
                "CUSTOM_FIELD_2" => GetHistogramRequestSearchTypesItems::CustomField2,
                "CUSTOM_FIELD_20" => GetHistogramRequestSearchTypesItems::CustomField20,
                "CUSTOM_FIELD_3" => GetHistogramRequestSearchTypesItems::CustomField3,
                "CUSTOM_FIELD_4" => GetHistogramRequestSearchTypesItems::CustomField4,
                "CUSTOM_FIELD_5" => GetHistogramRequestSearchTypesItems::CustomField5,
                "CUSTOM_FIELD_6" => GetHistogramRequestSearchTypesItems::CustomField6,
                "CUSTOM_FIELD_7" => GetHistogramRequestSearchTypesItems::CustomField7,
                "CUSTOM_FIELD_8" => GetHistogramRequestSearchTypesItems::CustomField8,
                "CUSTOM_FIELD_9" => GetHistogramRequestSearchTypesItems::CustomField9,
                "DATE_PUBLISHED" => GetHistogramRequestSearchTypesItems::DatePublished,
                "EDUCATION_LEVEL" => GetHistogramRequestSearchTypesItems::EducationLevel,
                "EMPLOYMENT_TYPE" => GetHistogramRequestSearchTypesItems::EmploymentType,
                "EXPERIENCE_LEVEL" => GetHistogramRequestSearchTypesItems::ExperienceLevel,
                "JOB_FIELD_UNSPECIFIED" => GetHistogramRequestSearchTypesItems::JobFieldUnspecified,
                "LANGUAGE" => GetHistogramRequestSearchTypesItems::Language,
                "LOCALE" => GetHistogramRequestSearchTypesItems::Locale,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GetHistogramRequestSearchTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetHistogramRequestSearchTypesItems {
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
    pub struct GetHistogramResponse {
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "The Histogram results."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::HistogramResult>>,
    }
    impl ::google_field_selector::FieldSelector for GetHistogramResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetHistogramResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramFacets {
        #[doc = "Optional. Specifies compensation field-based histogram requests.\nDuplicate values of CompensationHistogramRequest.type are not allowed."]
        #[serde(
            rename = "compensationHistogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::CompensationHistogramRequest>>,
        #[doc = "Optional. Specifies the custom attributes histogram requests.\nDuplicate values of CustomAttributeHistogramRequest.key are not\nallowed."]
        #[serde(
            rename = "customAttributeHistogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::CustomAttributeHistogramRequest>>,
        #[doc = "Optional. Specifies the simple type of histogram facets, for example,\n`COMPANY_SIZE`, `EMPLOYMENT_TYPE` etc. This field is equivalent to\nGetHistogramRequest."]
        #[serde(
            rename = "simpleHistogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub simple_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::HistogramFacetsSimpleHistogramFacetsItems>>,
    }
    impl ::google_field_selector::FieldSelector for HistogramFacets {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramFacets {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum HistogramFacetsSimpleHistogramFacetsItems {
        Admin1,
        Admin1Country,
        BaseCompensationUnit,
        Category,
        City,
        CityCoordinate,
        CompanyDisplayName,
        CompanyId,
        CompanySize,
        CompanyTitle,
        Country,
        CustomField1,
        CustomField10,
        CustomField11,
        CustomField12,
        CustomField13,
        CustomField14,
        CustomField15,
        CustomField16,
        CustomField17,
        CustomField18,
        CustomField19,
        CustomField2,
        CustomField20,
        CustomField3,
        CustomField4,
        CustomField5,
        CustomField6,
        CustomField7,
        CustomField8,
        CustomField9,
        DatePublished,
        EducationLevel,
        EmploymentType,
        ExperienceLevel,
        JobFieldUnspecified,
        Language,
        Locale,
    }
    impl HistogramFacetsSimpleHistogramFacetsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                HistogramFacetsSimpleHistogramFacetsItems::Admin1 => "ADMIN1",
                HistogramFacetsSimpleHistogramFacetsItems::Admin1Country => "ADMIN1_COUNTRY",
                HistogramFacetsSimpleHistogramFacetsItems::BaseCompensationUnit => {
                    "BASE_COMPENSATION_UNIT"
                }
                HistogramFacetsSimpleHistogramFacetsItems::Category => "CATEGORY",
                HistogramFacetsSimpleHistogramFacetsItems::City => "CITY",
                HistogramFacetsSimpleHistogramFacetsItems::CityCoordinate => "CITY_COORDINATE",
                HistogramFacetsSimpleHistogramFacetsItems::CompanyDisplayName => {
                    "COMPANY_DISPLAY_NAME"
                }
                HistogramFacetsSimpleHistogramFacetsItems::CompanyId => "COMPANY_ID",
                HistogramFacetsSimpleHistogramFacetsItems::CompanySize => "COMPANY_SIZE",
                HistogramFacetsSimpleHistogramFacetsItems::CompanyTitle => "COMPANY_TITLE",
                HistogramFacetsSimpleHistogramFacetsItems::Country => "COUNTRY",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField1 => "CUSTOM_FIELD_1",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField10 => "CUSTOM_FIELD_10",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField11 => "CUSTOM_FIELD_11",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField12 => "CUSTOM_FIELD_12",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField13 => "CUSTOM_FIELD_13",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField14 => "CUSTOM_FIELD_14",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField15 => "CUSTOM_FIELD_15",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField16 => "CUSTOM_FIELD_16",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField17 => "CUSTOM_FIELD_17",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField18 => "CUSTOM_FIELD_18",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField19 => "CUSTOM_FIELD_19",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField2 => "CUSTOM_FIELD_2",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField20 => "CUSTOM_FIELD_20",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField3 => "CUSTOM_FIELD_3",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField4 => "CUSTOM_FIELD_4",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField5 => "CUSTOM_FIELD_5",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField6 => "CUSTOM_FIELD_6",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField7 => "CUSTOM_FIELD_7",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField8 => "CUSTOM_FIELD_8",
                HistogramFacetsSimpleHistogramFacetsItems::CustomField9 => "CUSTOM_FIELD_9",
                HistogramFacetsSimpleHistogramFacetsItems::DatePublished => "DATE_PUBLISHED",
                HistogramFacetsSimpleHistogramFacetsItems::EducationLevel => "EDUCATION_LEVEL",
                HistogramFacetsSimpleHistogramFacetsItems::EmploymentType => "EMPLOYMENT_TYPE",
                HistogramFacetsSimpleHistogramFacetsItems::ExperienceLevel => "EXPERIENCE_LEVEL",
                HistogramFacetsSimpleHistogramFacetsItems::JobFieldUnspecified => {
                    "JOB_FIELD_UNSPECIFIED"
                }
                HistogramFacetsSimpleHistogramFacetsItems::Language => "LANGUAGE",
                HistogramFacetsSimpleHistogramFacetsItems::Locale => "LOCALE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for HistogramFacetsSimpleHistogramFacetsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for HistogramFacetsSimpleHistogramFacetsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<HistogramFacetsSimpleHistogramFacetsItems, ()> {
            Ok(match s {
                "ADMIN1" => HistogramFacetsSimpleHistogramFacetsItems::Admin1,
                "ADMIN1_COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Admin1Country,
                "BASE_COMPENSATION_UNIT" => {
                    HistogramFacetsSimpleHistogramFacetsItems::BaseCompensationUnit
                }
                "CATEGORY" => HistogramFacetsSimpleHistogramFacetsItems::Category,
                "CITY" => HistogramFacetsSimpleHistogramFacetsItems::City,
                "CITY_COORDINATE" => HistogramFacetsSimpleHistogramFacetsItems::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => {
                    HistogramFacetsSimpleHistogramFacetsItems::CompanyDisplayName
                }
                "COMPANY_ID" => HistogramFacetsSimpleHistogramFacetsItems::CompanyId,
                "COMPANY_SIZE" => HistogramFacetsSimpleHistogramFacetsItems::CompanySize,
                "COMPANY_TITLE" => HistogramFacetsSimpleHistogramFacetsItems::CompanyTitle,
                "COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Country,
                "CUSTOM_FIELD_1" => HistogramFacetsSimpleHistogramFacetsItems::CustomField1,
                "CUSTOM_FIELD_10" => HistogramFacetsSimpleHistogramFacetsItems::CustomField10,
                "CUSTOM_FIELD_11" => HistogramFacetsSimpleHistogramFacetsItems::CustomField11,
                "CUSTOM_FIELD_12" => HistogramFacetsSimpleHistogramFacetsItems::CustomField12,
                "CUSTOM_FIELD_13" => HistogramFacetsSimpleHistogramFacetsItems::CustomField13,
                "CUSTOM_FIELD_14" => HistogramFacetsSimpleHistogramFacetsItems::CustomField14,
                "CUSTOM_FIELD_15" => HistogramFacetsSimpleHistogramFacetsItems::CustomField15,
                "CUSTOM_FIELD_16" => HistogramFacetsSimpleHistogramFacetsItems::CustomField16,
                "CUSTOM_FIELD_17" => HistogramFacetsSimpleHistogramFacetsItems::CustomField17,
                "CUSTOM_FIELD_18" => HistogramFacetsSimpleHistogramFacetsItems::CustomField18,
                "CUSTOM_FIELD_19" => HistogramFacetsSimpleHistogramFacetsItems::CustomField19,
                "CUSTOM_FIELD_2" => HistogramFacetsSimpleHistogramFacetsItems::CustomField2,
                "CUSTOM_FIELD_20" => HistogramFacetsSimpleHistogramFacetsItems::CustomField20,
                "CUSTOM_FIELD_3" => HistogramFacetsSimpleHistogramFacetsItems::CustomField3,
                "CUSTOM_FIELD_4" => HistogramFacetsSimpleHistogramFacetsItems::CustomField4,
                "CUSTOM_FIELD_5" => HistogramFacetsSimpleHistogramFacetsItems::CustomField5,
                "CUSTOM_FIELD_6" => HistogramFacetsSimpleHistogramFacetsItems::CustomField6,
                "CUSTOM_FIELD_7" => HistogramFacetsSimpleHistogramFacetsItems::CustomField7,
                "CUSTOM_FIELD_8" => HistogramFacetsSimpleHistogramFacetsItems::CustomField8,
                "CUSTOM_FIELD_9" => HistogramFacetsSimpleHistogramFacetsItems::CustomField9,
                "DATE_PUBLISHED" => HistogramFacetsSimpleHistogramFacetsItems::DatePublished,
                "EDUCATION_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramFacetsSimpleHistogramFacetsItems::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::ExperienceLevel,
                "JOB_FIELD_UNSPECIFIED" => {
                    HistogramFacetsSimpleHistogramFacetsItems::JobFieldUnspecified
                }
                "LANGUAGE" => HistogramFacetsSimpleHistogramFacetsItems::Language,
                "LOCALE" => HistogramFacetsSimpleHistogramFacetsItems::Locale,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for HistogramFacetsSimpleHistogramFacetsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HistogramFacetsSimpleHistogramFacetsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HistogramFacetsSimpleHistogramFacetsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN1" => HistogramFacetsSimpleHistogramFacetsItems::Admin1,
                "ADMIN1_COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Admin1Country,
                "BASE_COMPENSATION_UNIT" => {
                    HistogramFacetsSimpleHistogramFacetsItems::BaseCompensationUnit
                }
                "CATEGORY" => HistogramFacetsSimpleHistogramFacetsItems::Category,
                "CITY" => HistogramFacetsSimpleHistogramFacetsItems::City,
                "CITY_COORDINATE" => HistogramFacetsSimpleHistogramFacetsItems::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => {
                    HistogramFacetsSimpleHistogramFacetsItems::CompanyDisplayName
                }
                "COMPANY_ID" => HistogramFacetsSimpleHistogramFacetsItems::CompanyId,
                "COMPANY_SIZE" => HistogramFacetsSimpleHistogramFacetsItems::CompanySize,
                "COMPANY_TITLE" => HistogramFacetsSimpleHistogramFacetsItems::CompanyTitle,
                "COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Country,
                "CUSTOM_FIELD_1" => HistogramFacetsSimpleHistogramFacetsItems::CustomField1,
                "CUSTOM_FIELD_10" => HistogramFacetsSimpleHistogramFacetsItems::CustomField10,
                "CUSTOM_FIELD_11" => HistogramFacetsSimpleHistogramFacetsItems::CustomField11,
                "CUSTOM_FIELD_12" => HistogramFacetsSimpleHistogramFacetsItems::CustomField12,
                "CUSTOM_FIELD_13" => HistogramFacetsSimpleHistogramFacetsItems::CustomField13,
                "CUSTOM_FIELD_14" => HistogramFacetsSimpleHistogramFacetsItems::CustomField14,
                "CUSTOM_FIELD_15" => HistogramFacetsSimpleHistogramFacetsItems::CustomField15,
                "CUSTOM_FIELD_16" => HistogramFacetsSimpleHistogramFacetsItems::CustomField16,
                "CUSTOM_FIELD_17" => HistogramFacetsSimpleHistogramFacetsItems::CustomField17,
                "CUSTOM_FIELD_18" => HistogramFacetsSimpleHistogramFacetsItems::CustomField18,
                "CUSTOM_FIELD_19" => HistogramFacetsSimpleHistogramFacetsItems::CustomField19,
                "CUSTOM_FIELD_2" => HistogramFacetsSimpleHistogramFacetsItems::CustomField2,
                "CUSTOM_FIELD_20" => HistogramFacetsSimpleHistogramFacetsItems::CustomField20,
                "CUSTOM_FIELD_3" => HistogramFacetsSimpleHistogramFacetsItems::CustomField3,
                "CUSTOM_FIELD_4" => HistogramFacetsSimpleHistogramFacetsItems::CustomField4,
                "CUSTOM_FIELD_5" => HistogramFacetsSimpleHistogramFacetsItems::CustomField5,
                "CUSTOM_FIELD_6" => HistogramFacetsSimpleHistogramFacetsItems::CustomField6,
                "CUSTOM_FIELD_7" => HistogramFacetsSimpleHistogramFacetsItems::CustomField7,
                "CUSTOM_FIELD_8" => HistogramFacetsSimpleHistogramFacetsItems::CustomField8,
                "CUSTOM_FIELD_9" => HistogramFacetsSimpleHistogramFacetsItems::CustomField9,
                "DATE_PUBLISHED" => HistogramFacetsSimpleHistogramFacetsItems::DatePublished,
                "EDUCATION_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramFacetsSimpleHistogramFacetsItems::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::ExperienceLevel,
                "JOB_FIELD_UNSPECIFIED" => {
                    HistogramFacetsSimpleHistogramFacetsItems::JobFieldUnspecified
                }
                "LANGUAGE" => HistogramFacetsSimpleHistogramFacetsItems::Language,
                "LOCALE" => HistogramFacetsSimpleHistogramFacetsItems::Locale,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for HistogramFacetsSimpleHistogramFacetsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramFacetsSimpleHistogramFacetsItems {
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
    pub struct HistogramResult {
        #[doc = "The Histogram search filters."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<crate::schemas::HistogramResultSearchType>,
        #[doc = "A map from the values of field to the number of jobs with that value\nin this search result.\n\nKey: search type (filter names, such as the companyName).\n\nValues: the count of jobs that match the filter for this search."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
    }
    impl ::google_field_selector::FieldSelector for HistogramResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum HistogramResultSearchType {
        #[doc = "Filter by Admin1, which is a global placeholder for\nreferring to state, province, or the particular term a country uses to\ndefine the geographic structure below the country level.\nExamples include states codes such as \"CA\", \"IL\", \"NY\", and\nprovinces, such as \"BC\"."]
        Admin1,
        #[doc = "A combination of state or province code with a country code. This field\ndiffers from `JOB_ADMIN1`, which can be used in multiple countries."]
        Admin1Country,
        #[doc = "Base compensation unit."]
        BaseCompensationUnit,
        #[doc = "Filter by the Category."]
        Category,
        #[doc = "Filter by the \"city name\", \"Admin1 code\", for example,\n\"Mountain View, CA\" or \"New York, NY\"."]
        City,
        #[doc = "Filter by the city center GPS coordinate (latitude and longitude), for\nexample, 37.4038522,-122.0987765. Since the coordinates of a city center\ncan change, clients may need to refresh them periodically."]
        CityCoordinate,
        #[doc = "Company display name."]
        CompanyDisplayName,
        #[doc = "Filter by the company id field."]
        CompanyId,
        #[doc = "Filter by the company size type field, such as `BIG`, `SMALL` or `BIGGER`."]
        CompanySize,
        #[doc = "Deprecated. Use COMPANY_DISPLAY_NAME instead.\n\nCompany display name."]
        CompanyTitle,
        #[doc = "Filter by the country code of job, such as US, JP, FR."]
        Country,
        #[doc = "Filter by custom field 1."]
        CustomField1,
        #[doc = "Filter by custom field 10."]
        CustomField10,
        #[doc = "Filter by custom field 11."]
        CustomField11,
        #[doc = "Filter by custom field 12."]
        CustomField12,
        #[doc = "Filter by custom field 13."]
        CustomField13,
        #[doc = "Filter by custom field 14."]
        CustomField14,
        #[doc = "Filter by custom field 15."]
        CustomField15,
        #[doc = "Filter by custom field 16."]
        CustomField16,
        #[doc = "Filter by custom field 17."]
        CustomField17,
        #[doc = "Filter by custom field 18."]
        CustomField18,
        #[doc = "Filter by custom field 19."]
        CustomField19,
        #[doc = "Filter by custom field 2."]
        CustomField2,
        #[doc = "Filter by custom field 20."]
        CustomField20,
        #[doc = "Filter by custom field 3."]
        CustomField3,
        #[doc = "Filter by custom field 4."]
        CustomField4,
        #[doc = "Filter by custom field 5."]
        CustomField5,
        #[doc = "Filter by custom field 6."]
        CustomField6,
        #[doc = "Filter by custom field 7."]
        CustomField7,
        #[doc = "Filter by custom field 8."]
        CustomField8,
        #[doc = "Filter by custom field 9."]
        CustomField9,
        #[doc = "Filter by the date published field. Values are stringified\nwith TimeRange, for example, TimeRange.PAST_MONTH."]
        DatePublished,
        #[doc = "Filter by the required education level of the job."]
        EducationLevel,
        #[doc = "Filter by the employment type field, such as `FULL_TIME` or `PART_TIME`."]
        EmploymentType,
        #[doc = "Filter by the required experience level of the job."]
        ExperienceLevel,
        #[doc = "The default value if search type is not specified."]
        JobFieldUnspecified,
        #[doc = "Filter by the language code portion of the locale field, such as \"en\" or\n\"fr\"."]
        Language,
        #[doc = "Filter by the locale field of a job, such as \"en-US\", \"fr-FR\".\n\nThis is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        Locale,
    }
    impl HistogramResultSearchType {
        pub fn as_str(self) -> &'static str {
            match self {
                HistogramResultSearchType::Admin1 => "ADMIN1",
                HistogramResultSearchType::Admin1Country => "ADMIN1_COUNTRY",
                HistogramResultSearchType::BaseCompensationUnit => "BASE_COMPENSATION_UNIT",
                HistogramResultSearchType::Category => "CATEGORY",
                HistogramResultSearchType::City => "CITY",
                HistogramResultSearchType::CityCoordinate => "CITY_COORDINATE",
                HistogramResultSearchType::CompanyDisplayName => "COMPANY_DISPLAY_NAME",
                HistogramResultSearchType::CompanyId => "COMPANY_ID",
                HistogramResultSearchType::CompanySize => "COMPANY_SIZE",
                HistogramResultSearchType::CompanyTitle => "COMPANY_TITLE",
                HistogramResultSearchType::Country => "COUNTRY",
                HistogramResultSearchType::CustomField1 => "CUSTOM_FIELD_1",
                HistogramResultSearchType::CustomField10 => "CUSTOM_FIELD_10",
                HistogramResultSearchType::CustomField11 => "CUSTOM_FIELD_11",
                HistogramResultSearchType::CustomField12 => "CUSTOM_FIELD_12",
                HistogramResultSearchType::CustomField13 => "CUSTOM_FIELD_13",
                HistogramResultSearchType::CustomField14 => "CUSTOM_FIELD_14",
                HistogramResultSearchType::CustomField15 => "CUSTOM_FIELD_15",
                HistogramResultSearchType::CustomField16 => "CUSTOM_FIELD_16",
                HistogramResultSearchType::CustomField17 => "CUSTOM_FIELD_17",
                HistogramResultSearchType::CustomField18 => "CUSTOM_FIELD_18",
                HistogramResultSearchType::CustomField19 => "CUSTOM_FIELD_19",
                HistogramResultSearchType::CustomField2 => "CUSTOM_FIELD_2",
                HistogramResultSearchType::CustomField20 => "CUSTOM_FIELD_20",
                HistogramResultSearchType::CustomField3 => "CUSTOM_FIELD_3",
                HistogramResultSearchType::CustomField4 => "CUSTOM_FIELD_4",
                HistogramResultSearchType::CustomField5 => "CUSTOM_FIELD_5",
                HistogramResultSearchType::CustomField6 => "CUSTOM_FIELD_6",
                HistogramResultSearchType::CustomField7 => "CUSTOM_FIELD_7",
                HistogramResultSearchType::CustomField8 => "CUSTOM_FIELD_8",
                HistogramResultSearchType::CustomField9 => "CUSTOM_FIELD_9",
                HistogramResultSearchType::DatePublished => "DATE_PUBLISHED",
                HistogramResultSearchType::EducationLevel => "EDUCATION_LEVEL",
                HistogramResultSearchType::EmploymentType => "EMPLOYMENT_TYPE",
                HistogramResultSearchType::ExperienceLevel => "EXPERIENCE_LEVEL",
                HistogramResultSearchType::JobFieldUnspecified => "JOB_FIELD_UNSPECIFIED",
                HistogramResultSearchType::Language => "LANGUAGE",
                HistogramResultSearchType::Locale => "LOCALE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for HistogramResultSearchType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for HistogramResultSearchType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<HistogramResultSearchType, ()> {
            Ok(match s {
                "ADMIN1" => HistogramResultSearchType::Admin1,
                "ADMIN1_COUNTRY" => HistogramResultSearchType::Admin1Country,
                "BASE_COMPENSATION_UNIT" => HistogramResultSearchType::BaseCompensationUnit,
                "CATEGORY" => HistogramResultSearchType::Category,
                "CITY" => HistogramResultSearchType::City,
                "CITY_COORDINATE" => HistogramResultSearchType::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => HistogramResultSearchType::CompanyDisplayName,
                "COMPANY_ID" => HistogramResultSearchType::CompanyId,
                "COMPANY_SIZE" => HistogramResultSearchType::CompanySize,
                "COMPANY_TITLE" => HistogramResultSearchType::CompanyTitle,
                "COUNTRY" => HistogramResultSearchType::Country,
                "CUSTOM_FIELD_1" => HistogramResultSearchType::CustomField1,
                "CUSTOM_FIELD_10" => HistogramResultSearchType::CustomField10,
                "CUSTOM_FIELD_11" => HistogramResultSearchType::CustomField11,
                "CUSTOM_FIELD_12" => HistogramResultSearchType::CustomField12,
                "CUSTOM_FIELD_13" => HistogramResultSearchType::CustomField13,
                "CUSTOM_FIELD_14" => HistogramResultSearchType::CustomField14,
                "CUSTOM_FIELD_15" => HistogramResultSearchType::CustomField15,
                "CUSTOM_FIELD_16" => HistogramResultSearchType::CustomField16,
                "CUSTOM_FIELD_17" => HistogramResultSearchType::CustomField17,
                "CUSTOM_FIELD_18" => HistogramResultSearchType::CustomField18,
                "CUSTOM_FIELD_19" => HistogramResultSearchType::CustomField19,
                "CUSTOM_FIELD_2" => HistogramResultSearchType::CustomField2,
                "CUSTOM_FIELD_20" => HistogramResultSearchType::CustomField20,
                "CUSTOM_FIELD_3" => HistogramResultSearchType::CustomField3,
                "CUSTOM_FIELD_4" => HistogramResultSearchType::CustomField4,
                "CUSTOM_FIELD_5" => HistogramResultSearchType::CustomField5,
                "CUSTOM_FIELD_6" => HistogramResultSearchType::CustomField6,
                "CUSTOM_FIELD_7" => HistogramResultSearchType::CustomField7,
                "CUSTOM_FIELD_8" => HistogramResultSearchType::CustomField8,
                "CUSTOM_FIELD_9" => HistogramResultSearchType::CustomField9,
                "DATE_PUBLISHED" => HistogramResultSearchType::DatePublished,
                "EDUCATION_LEVEL" => HistogramResultSearchType::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramResultSearchType::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramResultSearchType::ExperienceLevel,
                "JOB_FIELD_UNSPECIFIED" => HistogramResultSearchType::JobFieldUnspecified,
                "LANGUAGE" => HistogramResultSearchType::Language,
                "LOCALE" => HistogramResultSearchType::Locale,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for HistogramResultSearchType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HistogramResultSearchType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HistogramResultSearchType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN1" => HistogramResultSearchType::Admin1,
                "ADMIN1_COUNTRY" => HistogramResultSearchType::Admin1Country,
                "BASE_COMPENSATION_UNIT" => HistogramResultSearchType::BaseCompensationUnit,
                "CATEGORY" => HistogramResultSearchType::Category,
                "CITY" => HistogramResultSearchType::City,
                "CITY_COORDINATE" => HistogramResultSearchType::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => HistogramResultSearchType::CompanyDisplayName,
                "COMPANY_ID" => HistogramResultSearchType::CompanyId,
                "COMPANY_SIZE" => HistogramResultSearchType::CompanySize,
                "COMPANY_TITLE" => HistogramResultSearchType::CompanyTitle,
                "COUNTRY" => HistogramResultSearchType::Country,
                "CUSTOM_FIELD_1" => HistogramResultSearchType::CustomField1,
                "CUSTOM_FIELD_10" => HistogramResultSearchType::CustomField10,
                "CUSTOM_FIELD_11" => HistogramResultSearchType::CustomField11,
                "CUSTOM_FIELD_12" => HistogramResultSearchType::CustomField12,
                "CUSTOM_FIELD_13" => HistogramResultSearchType::CustomField13,
                "CUSTOM_FIELD_14" => HistogramResultSearchType::CustomField14,
                "CUSTOM_FIELD_15" => HistogramResultSearchType::CustomField15,
                "CUSTOM_FIELD_16" => HistogramResultSearchType::CustomField16,
                "CUSTOM_FIELD_17" => HistogramResultSearchType::CustomField17,
                "CUSTOM_FIELD_18" => HistogramResultSearchType::CustomField18,
                "CUSTOM_FIELD_19" => HistogramResultSearchType::CustomField19,
                "CUSTOM_FIELD_2" => HistogramResultSearchType::CustomField2,
                "CUSTOM_FIELD_20" => HistogramResultSearchType::CustomField20,
                "CUSTOM_FIELD_3" => HistogramResultSearchType::CustomField3,
                "CUSTOM_FIELD_4" => HistogramResultSearchType::CustomField4,
                "CUSTOM_FIELD_5" => HistogramResultSearchType::CustomField5,
                "CUSTOM_FIELD_6" => HistogramResultSearchType::CustomField6,
                "CUSTOM_FIELD_7" => HistogramResultSearchType::CustomField7,
                "CUSTOM_FIELD_8" => HistogramResultSearchType::CustomField8,
                "CUSTOM_FIELD_9" => HistogramResultSearchType::CustomField9,
                "DATE_PUBLISHED" => HistogramResultSearchType::DatePublished,
                "EDUCATION_LEVEL" => HistogramResultSearchType::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramResultSearchType::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramResultSearchType::ExperienceLevel,
                "JOB_FIELD_UNSPECIFIED" => HistogramResultSearchType::JobFieldUnspecified,
                "LANGUAGE" => HistogramResultSearchType::Language,
                "LOCALE" => HistogramResultSearchType::Locale,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for HistogramResultSearchType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramResultSearchType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramResults {
        #[doc = "Specifies compensation field-based histogram results that matches\nHistogramFacets.compensation_histogram_requests."]
        #[serde(
            rename = "compensationHistogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_histogram_results:
            ::std::option::Option<Vec<crate::schemas::CompensationHistogramResult>>,
        #[doc = "Specifies histogram results for custom attributes that\nmatches HistogramFacets.custom_attribute_histogram_facets."]
        #[serde(
            rename = "customAttributeHistogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_histogram_results:
            ::std::option::Option<Vec<crate::schemas::CustomAttributeHistogramResult>>,
        #[doc = "Specifies histogram results that matches\nHistogramFacets.simple_histogram_facets."]
        #[serde(
            rename = "simpleHistogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub simple_histogram_results: ::std::option::Option<Vec<crate::schemas::HistogramResult>>,
    }
    impl ::google_field_selector::FieldSelector for HistogramResults {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramResults {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Job {
        #[doc = "Optional but at least one of application_urls,\napplication_email_list or application_instruction must be\nspecified.\n\nUse this field to specify email address(es) to which resumes or\napplications can be sent.\n\nThe maximum number of allowed characters is 255."]
        #[serde(
            rename = "applicationEmailList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_email_list: ::std::option::Option<Vec<String>>,
        #[doc = "Optional but at least one of application_urls,\napplication_email_list or application_instruction must be\nspecified.\n\nUse this field to provide instructions, such as \"Mail your application\nto ...\", that a candidate can follow to apply for the job.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 3,000."]
        #[serde(
            rename = "applicationInstruction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_instruction: ::std::option::Option<String>,
        #[doc = "Optional but at least one of application_urls,\napplication_email_list or application_instruction must be\nspecified.\n\nUse this URL field to direct an applicant to a website, for example to\nlink to an online application form.\n\nThe maximum number of allowed characters is 2,000."]
        #[serde(
            rename = "applicationUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_urls: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The benefits included with the job."]
        #[serde(
            rename = "benefits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub benefits: ::std::option::Option<Vec<crate::schemas::JobBenefitsItems>>,
        #[doc = "Output only. The name of the company listing the job."]
        #[serde(
            rename = "companyDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_display_name: ::std::option::Option<String>,
        #[doc = "Optional but one of company_name or distributor_company_id must be\nprovided.\n\nThe resource name of the company listing the job, such as\n/companies/foo. This field takes precedence over the\ndistributor-assigned company identifier, distributor_company_id."]
        #[serde(
            rename = "companyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_name: ::std::option::Option<String>,
        #[doc = "Deprecated. Use company_display_name instead.\n\nOutput only.\n\nThe name of the company listing the job."]
        #[serde(
            rename = "companyTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_title: ::std::option::Option<String>,
        #[doc = "Optional. Job compensation information."]
        #[serde(
            rename = "compensationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_info: ::std::option::Option<crate::schemas::CompensationInfo>,
        #[doc = "Output only. The timestamp when this job was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Optional. A map of fields to hold both filterable and non-filterable custom job\nattributes that are not covered by the provided structured fields.\n\nThis field is a more general combination of the deprecated id-based\nfilterable_custom_fields and string-based\nnon_filterable_custom_fields.\n\nThe keys of the map are strings up to 64 bytes and must match the\npattern: a-zA-Z*.\n\nAt most 100 filterable and at most 100 unfilterable keys are supported.\nFor filterable `string_values`, across all keys at most 200 values are\nallowed, with each string no more than 255 characters. For unfilterable\n`string_values`, the maximum total size of `string_values` across all keys\nis 50KB."]
        #[serde(
            rename = "customAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attributes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CustomAttribute>,
        >,
        #[doc = "Optional. The department or functional area within the company with the open\nposition.\n\nThe maximum number of allowed characters is 255."]
        #[serde(
            rename = "department",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub department: ::std::option::Option<String>,
        #[doc = "Required. The description of the job, which typically includes a multi-paragraph\ndescription of the company and related information. Separate fields are\nprovided on the job object for responsibilities,\nqualifications, and other job characteristics. Use of\nthese separate job fields is recommended.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 100,000."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional but one of company_name or distributor_company_id must be\nprovided.\n\nA unique company identifier used by job distributors to identify an\nemployer's company entity. company_name takes precedence over\nthis field, and is the recommended field to use to identify companies.\n\nThe maximum number of allowed characters is 255."]
        #[serde(
            rename = "distributorCompanyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distributor_company_id: ::std::option::Option<String>,
        #[doc = "Optional. The desired education level for the job, such as\n\"Bachelors\", \"Masters\", \"Doctorate\"."]
        #[serde(
            rename = "educationLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub education_levels: ::std::option::Option<Vec<crate::schemas::JobEducationLevelsItems>>,
        #[doc = "Optional. The employment type(s) of a job, for example,\nfull time or\npart time."]
        #[serde(
            rename = "employmentTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub employment_types: ::std::option::Option<Vec<crate::schemas::JobEmploymentTypesItems>>,
        #[doc = "Optional. The end date of the job in UTC time zone. Typically this field\nis used for contracting engagements.\nDates prior to 1970/1/1 and invalid date formats are ignored."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Optional but strongly recommended for the best service\nexperience.\n\nThe expiration timestamp of the job. After this timestamp, the\njob is marked as expired, and it no longer appears in search results. The\nexpired job can't be deleted or listed by the DeleteJob and\nListJobs APIs, but it can be retrieved with the GetJob API or\nupdated with the UpdateJob API. An expired job can be updated and\nopened again by using a future expiration timestamp. Updating an expired\njob fails if there is another existing open job with same\nrequisition_id, company_name and language_code.\n\nThe expired jobs are retained in our system for 90 days. However, the\noverall expired job count cannot exceed 3 times the maximum of open jobs\ncount over the past week, otherwise jobs with earlier expire time are\ncleaned first. Expired jobs are no longer accessible after they are cleaned\nout.\nThe format of this field is RFC 3339 date strings. Example:\n2000-01-01T00:00:00.999999999Z\nSee\n[https://www.ietf.org/rfc/rfc3339.txt](https://www.ietf.org/rfc/rfc3339.txt).\n\nA valid date range is between 1970-01-01T00:00:00.0Z and\n2100-12-31T23:59:59.999Z. Invalid dates are ignored and treated as expire\ntime not provided.\n\nIf this value is not provided at the time of job creation or is invalid,\nthe job posting expires after 30 days from the job's creation time. For\nexample, if the job was created on 2017/01/01 13:00AM UTC with an\nunspecified expiration date, the job expires after 2017/01/31 13:00AM UTC.\n\nIf this value is not provided but expiry_date is, expiry_date is\nused.\n\nIf this value is not provided on job update, it depends on the field masks\nset by UpdateJobRequest.update_job_fields. If the field masks include\nexpiry_time, or the masks are empty meaning that every field is\nupdated, the job posting expires after 30 days from the job's last\nupdate time. Otherwise the expiration date isn't updated."]
        #[serde(
            rename = "expireTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expire_time: ::std::option::Option<String>,
        #[doc = "Deprecated. Use expire_time instead.\n\nOptional but strongly recommended to be provided for the best service\nexperience.\n\nThe expiration date of the job in UTC time. After 12 am on this date, the\njob is marked as expired, and it no longer appears in search results.\nThe expired job can't be deleted or listed by the DeleteJob and\nListJobs APIs, but it can be retrieved with the GetJob API or\nupdated with the UpdateJob API. An expired job can be updated and\nopened again by using a future expiration date. It can also remain expired.\nUpdating an expired job to be open fails if there is another existing open\njob with same requisition_id, company_name and language_code.\n\nThe expired jobs are retained in our system for 90 days. However, the\noverall expired job count cannot exceed 3 times the maximum of open jobs\ncount over the past week, otherwise jobs with earlier expire time are\nremoved first. Expired jobs are no longer accessible after they are cleaned\nout.\n\nA valid date range is between 1970/1/1 and 2100/12/31. Invalid dates are\nignored and treated as expiry date not provided.\n\nIf this value is not provided on job creation or is invalid, the job\nposting expires after 30 days from the job's creation time. For example, if\nthe job was created on 2017/01/01 13:00AM UTC with an unspecified\nexpiration date, the job expires after 2017/01/31 13:00AM UTC.\n\nIf this value is not provided on job update, it depends on the field masks\nset by UpdateJobRequest.update_job_fields. If the field masks include\nexpiry_date, or the masks are empty meaning that every field is\nupdated, the job expires after 30 days from the job's last update time.\nOtherwise the expiration date isn't updated."]
        #[serde(
            rename = "expiryDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expiry_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Deprecated. Always use compensation_info.\n\nOptional.\n\nJob compensation information.\n\nThis field replaces compensation_info. Only\nCompensationInfo.entries or extended_compensation_info can be set,\notherwise an exception is thrown."]
        #[serde(
            rename = "extendedCompensationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extended_compensation_info:
            ::std::option::Option<crate::schemas::ExtendedCompensationInfo>,
        #[doc = "Deprecated. Use custom_attributes instead.\n\nOptional.\n\nA map of fields to hold filterable custom job attributes not captured by\nthe standard fields such as job_title, company_name, or\nlevel. These custom fields store arbitrary\nstring values, and can be used for purposes not covered by\nthe structured fields. For the best search experience, use of the\nstructured rather than custom fields is recommended.\n\nData stored in these custom fields fields are indexed and\nsearched against by keyword searches (see\nSearchJobsRequest.custom_field_filters][]).\n\nThe map key must be a number between 1-20. If an invalid key is\nprovided on job create or update, an error is returned."]
        #[serde(
            rename = "filterableCustomFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filterable_custom_fields: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CustomField>,
        >,
        #[doc = "Optional. A description of bonus, commission, and other compensation\nincentives associated with the job not including salary or pay.\n\nThe maximum number of allowed characters is 10,000."]
        #[serde(
            rename = "incentives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub incentives: ::std::option::Option<String>,
        #[doc = "Output only. Structured locations of the job, resolved from locations."]
        #[serde(
            rename = "jobLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_locations: ::std::option::Option<Vec<crate::schemas::JobLocation>>,
        #[doc = "Required. The title of the job, such as \"Software Engineer\"\n\nThe maximum number of allowed characters is 500."]
        #[serde(
            rename = "jobTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_title: ::std::option::Option<String>,
        #[doc = "Optional. The language of the posting. This field is distinct from\nany requirements for fluency that are associated with the job.\n\nLanguage codes must be in BCP-47 format, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){:\nclass=\"external\" target=\"_blank\" }.\n\nIf this field is unspecified and Job.description is present, detected\nlanguage code based on Job.description is assigned, otherwise\ndefaults to 'en_US'."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. The experience level associated with the job, such as \"Entry Level\"."]
        #[serde(
            rename = "level",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub level: ::std::option::Option<crate::schemas::JobLevel>,
        #[doc = "Optional but strongly recommended for the best service experience.\n\nLocation(s) where the emploeyer is looking to hire for this job posting.\n\nSpecifying the full street address(es) of the hiring location enables\nbetter API results, especially job searches by commute time.\n\nAt most 50 locations are allowed for best search performance. If a job has\nmore locations, it is suggested to split it into multiple jobs with unique\nrequisition_ids (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', etc.) as\nmultiple jobs with the same requisition_id, company_name and\nlanguage_code are not allowed. If the original requisition_id must\nbe preserved, a custom field should be used for storage. It is also\nsuggested to group the locations that close to each other in the same job\nfor better search experience.\n\nThe maximum number of allowed characters is 500."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<Vec<String>>,
        #[doc = "Required during job update.\n\nResource name assigned to a job by the API, for example, \"/jobs/foo\". Use\nof this field in job queries and API calls is preferred over the use of\nrequisition_id since this value is unique."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. A promotion value of the job, as determined by the client.\nThe value determines the sort order of the jobs returned when searching for\njobs using the featured jobs search call, with higher promotional values\nbeing returned first and ties being resolved by relevance sort. Only the\njobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH.\n\nDefault value is 0, and negative values are treated as 0."]
        #[serde(
            rename = "promotionValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotion_value: ::std::option::Option<i32>,
        #[doc = "Optional. The date this job was most recently published in UTC format. The default\nvalue is the time the request arrives at the server."]
        #[serde(
            rename = "publishDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Optional. A description of the qualifications required to perform the\njob. The use of this field is recommended\nas an alternative to using the more general description field.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 10,000."]
        #[serde(
            rename = "qualifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qualifications: ::std::option::Option<String>,
        #[doc = "Output only. The URL of a web page that displays job details."]
        #[serde(
            rename = "referenceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reference_url: ::std::option::Option<String>,
        #[doc = "Optional. The job Region (for example, state, country) throughout which the job\nis available. If this field is set, a\nLocationFilter in a search query within the job region\nfinds this job if an exact location match is not specified.\nIf this field is set, setting job locations\nto the same location level as this field is strongly recommended."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<crate::schemas::JobRegion>,
        #[doc = "Required. The requisition ID, also referred to as the posting ID, assigned by the\nclient to identify a job. This field is intended to be used by clients\nfor client identification and tracking of listings. A job is not allowed\nto be created if there is another job with the same requisition_id,\ncompany_name and language_code.\n\nThe maximum number of allowed characters is 255."]
        #[serde(
            rename = "requisitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requisition_id: ::std::option::Option<String>,
        #[doc = "Optional. A description of job responsibilities. The use of this field is\nrecommended as an alternative to using the more general description\nfield.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 10,000."]
        #[serde(
            rename = "responsibilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responsibilities: ::std::option::Option<String>,
        #[doc = "Optional. The start date of the job in UTC time zone. Typically this field\nis used for contracting engagements.\nDates prior to 1970/1/1 and invalid date formats are ignored."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Deprecated. Use custom_attributes instead.\n\nOptional.\n\nA map of fields to hold non-filterable custom job attributes, similar to\nfilterable_custom_fields. These fields are distinct in that the data\nin these fields are not indexed. Therefore, the client cannot search\nagainst them, nor can the client use them to list jobs.\n\nThe key of the map can be any valid string."]
        #[serde(
            rename = "unindexedCustomFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unindexed_custom_fields: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CustomField>,
        >,
        #[doc = "Output only. The timestamp when this job was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Optional. The visibility of the job.\nDefaults to JobVisibility.PRIVATE if not specified.\nCurrently only JobVisibility.PRIVATE is supported."]
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::JobVisibility>,
    }
    impl ::google_field_selector::FieldSelector for Job {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Job {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobBenefitsItems {
        ChildCare,
        Dental,
        DomesticPartner,
        FlexibleHours,
        JobBenefitTypeUnspecified,
        LifeInsurance,
        Medical,
        ParentalLeave,
        RetirementPlan,
        SickDays,
        Telecommute,
        Vacation,
        Vision,
    }
    impl JobBenefitsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobBenefitsItems::ChildCare => "CHILD_CARE",
                JobBenefitsItems::Dental => "DENTAL",
                JobBenefitsItems::DomesticPartner => "DOMESTIC_PARTNER",
                JobBenefitsItems::FlexibleHours => "FLEXIBLE_HOURS",
                JobBenefitsItems::JobBenefitTypeUnspecified => "JOB_BENEFIT_TYPE_UNSPECIFIED",
                JobBenefitsItems::LifeInsurance => "LIFE_INSURANCE",
                JobBenefitsItems::Medical => "MEDICAL",
                JobBenefitsItems::ParentalLeave => "PARENTAL_LEAVE",
                JobBenefitsItems::RetirementPlan => "RETIREMENT_PLAN",
                JobBenefitsItems::SickDays => "SICK_DAYS",
                JobBenefitsItems::Telecommute => "TELECOMMUTE",
                JobBenefitsItems::Vacation => "VACATION",
                JobBenefitsItems::Vision => "VISION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobBenefitsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobBenefitsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobBenefitsItems, ()> {
            Ok(match s {
                "CHILD_CARE" => JobBenefitsItems::ChildCare,
                "DENTAL" => JobBenefitsItems::Dental,
                "DOMESTIC_PARTNER" => JobBenefitsItems::DomesticPartner,
                "FLEXIBLE_HOURS" => JobBenefitsItems::FlexibleHours,
                "JOB_BENEFIT_TYPE_UNSPECIFIED" => JobBenefitsItems::JobBenefitTypeUnspecified,
                "LIFE_INSURANCE" => JobBenefitsItems::LifeInsurance,
                "MEDICAL" => JobBenefitsItems::Medical,
                "PARENTAL_LEAVE" => JobBenefitsItems::ParentalLeave,
                "RETIREMENT_PLAN" => JobBenefitsItems::RetirementPlan,
                "SICK_DAYS" => JobBenefitsItems::SickDays,
                "TELECOMMUTE" => JobBenefitsItems::Telecommute,
                "VACATION" => JobBenefitsItems::Vacation,
                "VISION" => JobBenefitsItems::Vision,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobBenefitsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobBenefitsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobBenefitsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHILD_CARE" => JobBenefitsItems::ChildCare,
                "DENTAL" => JobBenefitsItems::Dental,
                "DOMESTIC_PARTNER" => JobBenefitsItems::DomesticPartner,
                "FLEXIBLE_HOURS" => JobBenefitsItems::FlexibleHours,
                "JOB_BENEFIT_TYPE_UNSPECIFIED" => JobBenefitsItems::JobBenefitTypeUnspecified,
                "LIFE_INSURANCE" => JobBenefitsItems::LifeInsurance,
                "MEDICAL" => JobBenefitsItems::Medical,
                "PARENTAL_LEAVE" => JobBenefitsItems::ParentalLeave,
                "RETIREMENT_PLAN" => JobBenefitsItems::RetirementPlan,
                "SICK_DAYS" => JobBenefitsItems::SickDays,
                "TELECOMMUTE" => JobBenefitsItems::Telecommute,
                "VACATION" => JobBenefitsItems::Vacation,
                "VISION" => JobBenefitsItems::Vision,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobBenefitsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobBenefitsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobEducationLevelsItems {
        Associate,
        Bachelors,
        Doctorate,
        EducationLevelUnspecified,
        HighSchool,
        Masters,
        NoDegreeRequired,
    }
    impl JobEducationLevelsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobEducationLevelsItems::Associate => "ASSOCIATE",
                JobEducationLevelsItems::Bachelors => "BACHELORS",
                JobEducationLevelsItems::Doctorate => "DOCTORATE",
                JobEducationLevelsItems::EducationLevelUnspecified => "EDUCATION_LEVEL_UNSPECIFIED",
                JobEducationLevelsItems::HighSchool => "HIGH_SCHOOL",
                JobEducationLevelsItems::Masters => "MASTERS",
                JobEducationLevelsItems::NoDegreeRequired => "NO_DEGREE_REQUIRED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobEducationLevelsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobEducationLevelsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobEducationLevelsItems, ()> {
            Ok(match s {
                "ASSOCIATE" => JobEducationLevelsItems::Associate,
                "BACHELORS" => JobEducationLevelsItems::Bachelors,
                "DOCTORATE" => JobEducationLevelsItems::Doctorate,
                "EDUCATION_LEVEL_UNSPECIFIED" => JobEducationLevelsItems::EducationLevelUnspecified,
                "HIGH_SCHOOL" => JobEducationLevelsItems::HighSchool,
                "MASTERS" => JobEducationLevelsItems::Masters,
                "NO_DEGREE_REQUIRED" => JobEducationLevelsItems::NoDegreeRequired,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobEducationLevelsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobEducationLevelsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobEducationLevelsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASSOCIATE" => JobEducationLevelsItems::Associate,
                "BACHELORS" => JobEducationLevelsItems::Bachelors,
                "DOCTORATE" => JobEducationLevelsItems::Doctorate,
                "EDUCATION_LEVEL_UNSPECIFIED" => JobEducationLevelsItems::EducationLevelUnspecified,
                "HIGH_SCHOOL" => JobEducationLevelsItems::HighSchool,
                "MASTERS" => JobEducationLevelsItems::Masters,
                "NO_DEGREE_REQUIRED" => JobEducationLevelsItems::NoDegreeRequired,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobEducationLevelsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobEducationLevelsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobEmploymentTypesItems {
        ContractToHire,
        Contractor,
        EmploymentTypeUnspecified,
        FlyInFlyOut,
        FullTime,
        Intern,
        Other,
        PartTime,
        PerDiem,
        Temporary,
        Volunteer,
    }
    impl JobEmploymentTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobEmploymentTypesItems::ContractToHire => "CONTRACT_TO_HIRE",
                JobEmploymentTypesItems::Contractor => "CONTRACTOR",
                JobEmploymentTypesItems::EmploymentTypeUnspecified => "EMPLOYMENT_TYPE_UNSPECIFIED",
                JobEmploymentTypesItems::FlyInFlyOut => "FLY_IN_FLY_OUT",
                JobEmploymentTypesItems::FullTime => "FULL_TIME",
                JobEmploymentTypesItems::Intern => "INTERN",
                JobEmploymentTypesItems::Other => "OTHER",
                JobEmploymentTypesItems::PartTime => "PART_TIME",
                JobEmploymentTypesItems::PerDiem => "PER_DIEM",
                JobEmploymentTypesItems::Temporary => "TEMPORARY",
                JobEmploymentTypesItems::Volunteer => "VOLUNTEER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobEmploymentTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobEmploymentTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobEmploymentTypesItems, ()> {
            Ok(match s {
                "CONTRACT_TO_HIRE" => JobEmploymentTypesItems::ContractToHire,
                "CONTRACTOR" => JobEmploymentTypesItems::Contractor,
                "EMPLOYMENT_TYPE_UNSPECIFIED" => JobEmploymentTypesItems::EmploymentTypeUnspecified,
                "FLY_IN_FLY_OUT" => JobEmploymentTypesItems::FlyInFlyOut,
                "FULL_TIME" => JobEmploymentTypesItems::FullTime,
                "INTERN" => JobEmploymentTypesItems::Intern,
                "OTHER" => JobEmploymentTypesItems::Other,
                "PART_TIME" => JobEmploymentTypesItems::PartTime,
                "PER_DIEM" => JobEmploymentTypesItems::PerDiem,
                "TEMPORARY" => JobEmploymentTypesItems::Temporary,
                "VOLUNTEER" => JobEmploymentTypesItems::Volunteer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobEmploymentTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobEmploymentTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobEmploymentTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTRACT_TO_HIRE" => JobEmploymentTypesItems::ContractToHire,
                "CONTRACTOR" => JobEmploymentTypesItems::Contractor,
                "EMPLOYMENT_TYPE_UNSPECIFIED" => JobEmploymentTypesItems::EmploymentTypeUnspecified,
                "FLY_IN_FLY_OUT" => JobEmploymentTypesItems::FlyInFlyOut,
                "FULL_TIME" => JobEmploymentTypesItems::FullTime,
                "INTERN" => JobEmploymentTypesItems::Intern,
                "OTHER" => JobEmploymentTypesItems::Other,
                "PART_TIME" => JobEmploymentTypesItems::PartTime,
                "PER_DIEM" => JobEmploymentTypesItems::PerDiem,
                "TEMPORARY" => JobEmploymentTypesItems::Temporary,
                "VOLUNTEER" => JobEmploymentTypesItems::Volunteer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobEmploymentTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobEmploymentTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobLevel {
        #[doc = "Senior-level managers responsible for managing teams of managers."]
        Director,
        #[doc = "Entry-level individual contributors, typically with less than 2 years of\nexperience in a similar role. Includes interns."]
        EntryLevel,
        #[doc = "Executive-level managers and above, including C-level positions."]
        Executive,
        #[doc = "Experienced individual contributors, typically with 2+ years of\nexperience in a similar role."]
        Experienced,
        #[doc = "The default value if the level is not specified."]
        JobLevelUnspecified,
        #[doc = "Entry- to mid-level managers responsible for managing a team of people."]
        Manager,
    }
    impl JobLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                JobLevel::Director => "DIRECTOR",
                JobLevel::EntryLevel => "ENTRY_LEVEL",
                JobLevel::Executive => "EXECUTIVE",
                JobLevel::Experienced => "EXPERIENCED",
                JobLevel::JobLevelUnspecified => "JOB_LEVEL_UNSPECIFIED",
                JobLevel::Manager => "MANAGER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobLevel, ()> {
            Ok(match s {
                "DIRECTOR" => JobLevel::Director,
                "ENTRY_LEVEL" => JobLevel::EntryLevel,
                "EXECUTIVE" => JobLevel::Executive,
                "EXPERIENCED" => JobLevel::Experienced,
                "JOB_LEVEL_UNSPECIFIED" => JobLevel::JobLevelUnspecified,
                "MANAGER" => JobLevel::Manager,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIRECTOR" => JobLevel::Director,
                "ENTRY_LEVEL" => JobLevel::EntryLevel,
                "EXECUTIVE" => JobLevel::Executive,
                "EXPERIENCED" => JobLevel::Experienced,
                "JOB_LEVEL_UNSPECIFIED" => JobLevel::JobLevelUnspecified,
                "MANAGER" => JobLevel::Manager,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobRegion {
        #[doc = "In addition to exact location matching, job is returned when\nLocationFilter in search query is in the same country as this job.\nFor example, if a `NATION_WIDE` job is posted in \"USA\", it is\nreturned if LocationFilter has 'Mountain View'."]
        NationWide,
        #[doc = "If the region is unspecified, the job is only returned if it\nmatches the LocationFilter."]
        RegionUnspecified,
        #[doc = "In additiona to exact location matching, job is returned when the\nLocationFilter in search query is in the same state as this job.\nFor example, if a `STATE_WIDE` job is posted in \"CA, USA\", it is\nreturned if LocationFilter has \"Mountain View\"."]
        StateWide,
        #[doc = "Job allows employees to work remotely (telecommute).\nIf locations are provided with this value, the job is\nconsidered as having a location, but telecommuting is allowed."]
        Telecommute,
    }
    impl JobRegion {
        pub fn as_str(self) -> &'static str {
            match self {
                JobRegion::NationWide => "NATION_WIDE",
                JobRegion::RegionUnspecified => "REGION_UNSPECIFIED",
                JobRegion::StateWide => "STATE_WIDE",
                JobRegion::Telecommute => "TELECOMMUTE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobRegion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobRegion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobRegion, ()> {
            Ok(match s {
                "NATION_WIDE" => JobRegion::NationWide,
                "REGION_UNSPECIFIED" => JobRegion::RegionUnspecified,
                "STATE_WIDE" => JobRegion::StateWide,
                "TELECOMMUTE" => JobRegion::Telecommute,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobRegion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobRegion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobRegion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NATION_WIDE" => JobRegion::NationWide,
                "REGION_UNSPECIFIED" => JobRegion::RegionUnspecified,
                "STATE_WIDE" => JobRegion::StateWide,
                "TELECOMMUTE" => JobRegion::Telecommute,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobRegion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobRegion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobVisibility {
        #[doc = "The Job is visible to the owner and may be visible to other applications\nand processes at Google.\n\nNot yet supported. Use PRIVATE."]
        Google,
        #[doc = "Default value."]
        JobVisibilityUnspecified,
        #[doc = "The Job is only visible to the owner."]
        Private,
        #[doc = "The Job is visible to the owner and may be visible to all other API\nclients.\n\nNot yet supported. Use PRIVATE."]
        Public,
    }
    impl JobVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                JobVisibility::Google => "GOOGLE",
                JobVisibility::JobVisibilityUnspecified => "JOB_VISIBILITY_UNSPECIFIED",
                JobVisibility::Private => "PRIVATE",
                JobVisibility::Public => "PUBLIC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobVisibility, ()> {
            Ok(match s {
                "GOOGLE" => JobVisibility::Google,
                "JOB_VISIBILITY_UNSPECIFIED" => JobVisibility::JobVisibilityUnspecified,
                "PRIVATE" => JobVisibility::Private,
                "PUBLIC" => JobVisibility::Public,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOOGLE" => JobVisibility::Google,
                "JOB_VISIBILITY_UNSPECIFIED" => JobVisibility::JobVisibilityUnspecified,
                "PRIVATE" => JobVisibility::Private,
                "PUBLIC" => JobVisibility::Public,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobVisibility {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobFilters {
        #[doc = "Optional. The category filter specifies the categories of jobs to search against.\nSee Category for more information.\n\nIf a value is not specified, jobs from any category are searched against.\n\nIf multiple values are specified, jobs from any of the specified\ncategories are searched against."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<crate::schemas::JobFiltersCategoriesItems>>,
        #[doc = "Optional. Allows filtering jobs by commute time with different travel methods (e.g.\ndriving or public transit). Note: this only works with COMMUTE\nMODE. When specified, [JobFilters.location_filters] will be\nignored.\n\nCurrently we do not support sorting by commute time."]
        #[serde(
            rename = "commuteFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commute_filter: ::std::option::Option<crate::schemas::CommutePreference>,
        #[doc = "Optional. The company names filter specifies the company entities to search\nagainst.\n\nIf a value is not specified, jobs are searched for against all companies.\n\nIf multiple values are specified, jobs are searched against the\nspecified companies.\n\nAt most 20 company filters are allowed."]
        #[serde(
            rename = "companyNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This filter specifies the exact company titles\nof jobs to search against.\n\nIf a value is not specified, jobs within the search results can be\nassociated with any company.\n\nIf multiple values are specified, jobs within the search results may be\nassociated with any of the specified companies.\n\nAt most 20 company title filters are allowed."]
        #[serde(
            rename = "companyTitles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_titles: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This search filter is applied only to\nJob.compensation_info. For example, if the filter is specified\nas \"Hourly job with per-hour compensation > $15\", only jobs that meet\nthis criteria are searched. If a filter is not defined, all open jobs\nare searched."]
        #[serde(
            rename = "compensationFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_filter: ::std::option::Option<crate::schemas::CompensationFilter>,
        #[doc = "Optional. This filter specifies a structured syntax to match against the\nJob.custom_attributes that are marked as `filterable`.\n\nThe syntax for this expression is a subset of Google SQL syntax.\n\nSupported operators are: =, !=, <, <=, >, >= where the left of the operator\nis a custom field key and the right of the operator is a number or string\n(surrounded by quotes) value.\n\nSupported functions are LOWER(<field_name>) to\nperform case insensitive match and EMPTY(<field_name>) to filter on the\nexistence of a key.\n\nBoolean expressions (AND/OR/NOT) are supported up to 3 levels of\nnesting (For example, \"((A AND B AND C) OR NOT D) AND E\"), and there can\nbe a maximum of 100 comparisons/functions in the expression. The expression\nmust be < 3000 bytes in length.\n\nSample Query:\n(key1 = \"TEST\" OR LOWER(key1)=\"test\" OR NOT EMPTY(key1)) AND key2 > 100"]
        #[serde(
            rename = "customAttributeFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_filter: ::std::option::Option<String>,
        #[doc = "Deprecated. Use custom_attribute_filter instead.\n\nOptional.\n\nThis filter specifies searching against\ncustom field values. See Job.filterable_custom_fields for information.\nThe key value specifies a number between 1-20 (the service\nsupports 20 custom fields) corresponding to the desired custom field map\nvalue. If an invalid key is provided or specified together with\ncustom_attribute_filter, an error is thrown."]
        #[serde(
            rename = "customFieldFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_field_filters: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CustomFieldFilter>,
        >,
        #[doc = "Optional. This flag controls the spell-check feature. If false, the\nservice attempts to correct a misspelled query,\nfor example, \"enginee\" is corrected to \"engineer\".\n\nDefaults to false: a spell check is performed."]
        #[serde(
            rename = "disableSpellCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_spell_check: ::std::option::Option<bool>,
        #[doc = "Optional. The employment type filter specifies the employment type of jobs to\nsearch against, such as EmploymentType.FULL_TIME.\n\nIf a value is not specified, jobs in the search results include any\nemployment type.\n\nIf multiple values are specified, jobs in the search results include any\nof the specified employment types."]
        #[serde(
            rename = "employmentTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub employment_types:
            ::std::option::Option<Vec<crate::schemas::JobFiltersEmploymentTypesItems>>,
        #[doc = "Deprecated. Always use compensation_filter.\n\nOptional.\n\nThis search filter is applied only to\nJob.extended_compensation_info. For example, if the filter is specified\nas \"Hourly job with per-hour compensation > $15\", only jobs that meet\nthese criteria are searched. If a filter is not defined, all open jobs\nare searched."]
        #[serde(
            rename = "extendedCompensationFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extended_compensation_filter:
            ::std::option::Option<crate::schemas::ExtendedCompensationFilter>,
        #[doc = "Optional. This filter specifies the locale of jobs to search against,\nfor example, \"en-US\".\n\nIf a value is not specified, the search results may contain jobs in any\nlocale.\n\nLanguage codes should be in BCP-47 format, for example, \"en-US\" or\n\"sr-Latn\". For more information, see [Tags for Identifying\nLanguages](https://tools.ietf.org/html/bcp47).\n\nAt most 10 language code filters are allowed."]
        #[serde(
            rename = "languageCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_codes: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The location filter specifies geo-regions containing the jobs to\nsearch against. See LocationFilter for more information.\n\nIf a location value is not specified, jobs are retrieved\nfrom all locations.\n\nIf multiple values are specified, jobs are retrieved from any of the\nspecified locations. If different values are specified for the\nLocationFilter.distance_in_miles parameter, the maximum provided\ndistance is used for all locations.\n\nAt most 5 location filters are allowed."]
        #[serde(
            rename = "locationFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_filters: ::std::option::Option<Vec<crate::schemas::LocationFilter>>,
        #[doc = "Optional. Jobs published within a range specified by this filter are searched\nagainst, for example, DateRange.PAST_MONTH. If a value is not\nspecified, all open jobs are searched against regardless of the\ndate they were published."]
        #[serde(
            rename = "publishDateRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_date_range: ::std::option::Option<crate::schemas::JobFiltersPublishDateRange>,
        #[doc = "Optional. The query filter contains the keywords that match against the job\ntitle, description, and location fields.\n\nThe maximum query size is 255 bytes/characters."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "Deprecated. Do not use this field.\n\nThis flag controls whether the job search should be restricted to jobs\nowned by the current user.\n\nDefaults to false where all jobs accessible to the\nuser are searched against."]
        #[serde(
            rename = "tenantJobOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_job_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for JobFilters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobFilters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobFiltersCategoriesItems {
        AccountingAndFinance,
        AdministrativeAndOffice,
        AdvertisingAndMarketing,
        AnimalCare,
        ArtFashionAndDesign,
        BusinessOperations,
        CleaningAndFacilities,
        ComputerAndIt,
        Construction,
        CustomerService,
        Education,
        EntertainmentAndTravel,
        FarmingAndOutdoors,
        Healthcare,
        HumanResources,
        InstallationMaintenanceAndRepair,
        JobCategoryUnspecified,
        Legal,
        Management,
        ManufacturingAndWarehouse,
        MediaCommunicationsAndWriting,
        OilGasAndMining,
        PersonalCareAndServices,
        ProtectiveServices,
        RealEstate,
        RestaurantAndHospitality,
        SalesAndRetail,
        ScienceAndEngineering,
        SocialServicesAndNonProfit,
        SportsFitnessAndRecreation,
        TransportationAndLogistics,
    }
    impl JobFiltersCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobFiltersCategoriesItems::AccountingAndFinance => "ACCOUNTING_AND_FINANCE",
                JobFiltersCategoriesItems::AdministrativeAndOffice => "ADMINISTRATIVE_AND_OFFICE",
                JobFiltersCategoriesItems::AdvertisingAndMarketing => "ADVERTISING_AND_MARKETING",
                JobFiltersCategoriesItems::AnimalCare => "ANIMAL_CARE",
                JobFiltersCategoriesItems::ArtFashionAndDesign => "ART_FASHION_AND_DESIGN",
                JobFiltersCategoriesItems::BusinessOperations => "BUSINESS_OPERATIONS",
                JobFiltersCategoriesItems::CleaningAndFacilities => "CLEANING_AND_FACILITIES",
                JobFiltersCategoriesItems::ComputerAndIt => "COMPUTER_AND_IT",
                JobFiltersCategoriesItems::Construction => "CONSTRUCTION",
                JobFiltersCategoriesItems::CustomerService => "CUSTOMER_SERVICE",
                JobFiltersCategoriesItems::Education => "EDUCATION",
                JobFiltersCategoriesItems::EntertainmentAndTravel => "ENTERTAINMENT_AND_TRAVEL",
                JobFiltersCategoriesItems::FarmingAndOutdoors => "FARMING_AND_OUTDOORS",
                JobFiltersCategoriesItems::Healthcare => "HEALTHCARE",
                JobFiltersCategoriesItems::HumanResources => "HUMAN_RESOURCES",
                JobFiltersCategoriesItems::InstallationMaintenanceAndRepair => {
                    "INSTALLATION_MAINTENANCE_AND_REPAIR"
                }
                JobFiltersCategoriesItems::JobCategoryUnspecified => "JOB_CATEGORY_UNSPECIFIED",
                JobFiltersCategoriesItems::Legal => "LEGAL",
                JobFiltersCategoriesItems::Management => "MANAGEMENT",
                JobFiltersCategoriesItems::ManufacturingAndWarehouse => {
                    "MANUFACTURING_AND_WAREHOUSE"
                }
                JobFiltersCategoriesItems::MediaCommunicationsAndWriting => {
                    "MEDIA_COMMUNICATIONS_AND_WRITING"
                }
                JobFiltersCategoriesItems::OilGasAndMining => "OIL_GAS_AND_MINING",
                JobFiltersCategoriesItems::PersonalCareAndServices => "PERSONAL_CARE_AND_SERVICES",
                JobFiltersCategoriesItems::ProtectiveServices => "PROTECTIVE_SERVICES",
                JobFiltersCategoriesItems::RealEstate => "REAL_ESTATE",
                JobFiltersCategoriesItems::RestaurantAndHospitality => "RESTAURANT_AND_HOSPITALITY",
                JobFiltersCategoriesItems::SalesAndRetail => "SALES_AND_RETAIL",
                JobFiltersCategoriesItems::ScienceAndEngineering => "SCIENCE_AND_ENGINEERING",
                JobFiltersCategoriesItems::SocialServicesAndNonProfit => {
                    "SOCIAL_SERVICES_AND_NON_PROFIT"
                }
                JobFiltersCategoriesItems::SportsFitnessAndRecreation => {
                    "SPORTS_FITNESS_AND_RECREATION"
                }
                JobFiltersCategoriesItems::TransportationAndLogistics => {
                    "TRANSPORTATION_AND_LOGISTICS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobFiltersCategoriesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobFiltersCategoriesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobFiltersCategoriesItems, ()> {
            Ok(match s {
                "ACCOUNTING_AND_FINANCE" => JobFiltersCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => JobFiltersCategoriesItems::AdministrativeAndOffice,
                "ADVERTISING_AND_MARKETING" => JobFiltersCategoriesItems::AdvertisingAndMarketing,
                "ANIMAL_CARE" => JobFiltersCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobFiltersCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobFiltersCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => JobFiltersCategoriesItems::CleaningAndFacilities,
                "COMPUTER_AND_IT" => JobFiltersCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobFiltersCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobFiltersCategoriesItems::CustomerService,
                "EDUCATION" => JobFiltersCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => JobFiltersCategoriesItems::EntertainmentAndTravel,
                "FARMING_AND_OUTDOORS" => JobFiltersCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobFiltersCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobFiltersCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobFiltersCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => JobFiltersCategoriesItems::JobCategoryUnspecified,
                "LEGAL" => JobFiltersCategoriesItems::Legal,
                "MANAGEMENT" => JobFiltersCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => {
                    JobFiltersCategoriesItems::ManufacturingAndWarehouse
                }
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobFiltersCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobFiltersCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => JobFiltersCategoriesItems::PersonalCareAndServices,
                "PROTECTIVE_SERVICES" => JobFiltersCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobFiltersCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => JobFiltersCategoriesItems::RestaurantAndHospitality,
                "SALES_AND_RETAIL" => JobFiltersCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => JobFiltersCategoriesItems::ScienceAndEngineering,
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobFiltersCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobFiltersCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobFiltersCategoriesItems::TransportationAndLogistics
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobFiltersCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobFiltersCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobFiltersCategoriesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNTING_AND_FINANCE" => JobFiltersCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => JobFiltersCategoriesItems::AdministrativeAndOffice,
                "ADVERTISING_AND_MARKETING" => JobFiltersCategoriesItems::AdvertisingAndMarketing,
                "ANIMAL_CARE" => JobFiltersCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobFiltersCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobFiltersCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => JobFiltersCategoriesItems::CleaningAndFacilities,
                "COMPUTER_AND_IT" => JobFiltersCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobFiltersCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobFiltersCategoriesItems::CustomerService,
                "EDUCATION" => JobFiltersCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => JobFiltersCategoriesItems::EntertainmentAndTravel,
                "FARMING_AND_OUTDOORS" => JobFiltersCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobFiltersCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobFiltersCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobFiltersCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => JobFiltersCategoriesItems::JobCategoryUnspecified,
                "LEGAL" => JobFiltersCategoriesItems::Legal,
                "MANAGEMENT" => JobFiltersCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => {
                    JobFiltersCategoriesItems::ManufacturingAndWarehouse
                }
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobFiltersCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobFiltersCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => JobFiltersCategoriesItems::PersonalCareAndServices,
                "PROTECTIVE_SERVICES" => JobFiltersCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobFiltersCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => JobFiltersCategoriesItems::RestaurantAndHospitality,
                "SALES_AND_RETAIL" => JobFiltersCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => JobFiltersCategoriesItems::ScienceAndEngineering,
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobFiltersCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobFiltersCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobFiltersCategoriesItems::TransportationAndLogistics
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
    impl ::google_field_selector::FieldSelector for JobFiltersCategoriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobFiltersCategoriesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobFiltersEmploymentTypesItems {
        ContractToHire,
        Contractor,
        EmploymentTypeUnspecified,
        FlyInFlyOut,
        FullTime,
        Intern,
        Other,
        PartTime,
        PerDiem,
        Temporary,
        Volunteer,
    }
    impl JobFiltersEmploymentTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobFiltersEmploymentTypesItems::ContractToHire => "CONTRACT_TO_HIRE",
                JobFiltersEmploymentTypesItems::Contractor => "CONTRACTOR",
                JobFiltersEmploymentTypesItems::EmploymentTypeUnspecified => {
                    "EMPLOYMENT_TYPE_UNSPECIFIED"
                }
                JobFiltersEmploymentTypesItems::FlyInFlyOut => "FLY_IN_FLY_OUT",
                JobFiltersEmploymentTypesItems::FullTime => "FULL_TIME",
                JobFiltersEmploymentTypesItems::Intern => "INTERN",
                JobFiltersEmploymentTypesItems::Other => "OTHER",
                JobFiltersEmploymentTypesItems::PartTime => "PART_TIME",
                JobFiltersEmploymentTypesItems::PerDiem => "PER_DIEM",
                JobFiltersEmploymentTypesItems::Temporary => "TEMPORARY",
                JobFiltersEmploymentTypesItems::Volunteer => "VOLUNTEER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobFiltersEmploymentTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobFiltersEmploymentTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobFiltersEmploymentTypesItems, ()> {
            Ok(match s {
                "CONTRACT_TO_HIRE" => JobFiltersEmploymentTypesItems::ContractToHire,
                "CONTRACTOR" => JobFiltersEmploymentTypesItems::Contractor,
                "EMPLOYMENT_TYPE_UNSPECIFIED" => {
                    JobFiltersEmploymentTypesItems::EmploymentTypeUnspecified
                }
                "FLY_IN_FLY_OUT" => JobFiltersEmploymentTypesItems::FlyInFlyOut,
                "FULL_TIME" => JobFiltersEmploymentTypesItems::FullTime,
                "INTERN" => JobFiltersEmploymentTypesItems::Intern,
                "OTHER" => JobFiltersEmploymentTypesItems::Other,
                "PART_TIME" => JobFiltersEmploymentTypesItems::PartTime,
                "PER_DIEM" => JobFiltersEmploymentTypesItems::PerDiem,
                "TEMPORARY" => JobFiltersEmploymentTypesItems::Temporary,
                "VOLUNTEER" => JobFiltersEmploymentTypesItems::Volunteer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobFiltersEmploymentTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobFiltersEmploymentTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobFiltersEmploymentTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTRACT_TO_HIRE" => JobFiltersEmploymentTypesItems::ContractToHire,
                "CONTRACTOR" => JobFiltersEmploymentTypesItems::Contractor,
                "EMPLOYMENT_TYPE_UNSPECIFIED" => {
                    JobFiltersEmploymentTypesItems::EmploymentTypeUnspecified
                }
                "FLY_IN_FLY_OUT" => JobFiltersEmploymentTypesItems::FlyInFlyOut,
                "FULL_TIME" => JobFiltersEmploymentTypesItems::FullTime,
                "INTERN" => JobFiltersEmploymentTypesItems::Intern,
                "OTHER" => JobFiltersEmploymentTypesItems::Other,
                "PART_TIME" => JobFiltersEmploymentTypesItems::PartTime,
                "PER_DIEM" => JobFiltersEmploymentTypesItems::PerDiem,
                "TEMPORARY" => JobFiltersEmploymentTypesItems::Temporary,
                "VOLUNTEER" => JobFiltersEmploymentTypesItems::Volunteer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobFiltersEmploymentTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobFiltersEmploymentTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobFiltersPublishDateRange {
        #[doc = "Default value: Filtering on time is not performed."]
        DateRangeUnspecified,
        #[doc = "The past 24 hours"]
        Past24Hours,
        #[doc = "The past 3 days"]
        Past3Days,
        #[doc = "The past month (30 days)"]
        PastMonth,
        #[doc = "The past week (7 days)"]
        PastWeek,
        #[doc = "The past year (365 days)"]
        PastYear,
    }
    impl JobFiltersPublishDateRange {
        pub fn as_str(self) -> &'static str {
            match self {
                JobFiltersPublishDateRange::DateRangeUnspecified => "DATE_RANGE_UNSPECIFIED",
                JobFiltersPublishDateRange::Past24Hours => "PAST_24_HOURS",
                JobFiltersPublishDateRange::Past3Days => "PAST_3_DAYS",
                JobFiltersPublishDateRange::PastMonth => "PAST_MONTH",
                JobFiltersPublishDateRange::PastWeek => "PAST_WEEK",
                JobFiltersPublishDateRange::PastYear => "PAST_YEAR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobFiltersPublishDateRange {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobFiltersPublishDateRange {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobFiltersPublishDateRange, ()> {
            Ok(match s {
                "DATE_RANGE_UNSPECIFIED" => JobFiltersPublishDateRange::DateRangeUnspecified,
                "PAST_24_HOURS" => JobFiltersPublishDateRange::Past24Hours,
                "PAST_3_DAYS" => JobFiltersPublishDateRange::Past3Days,
                "PAST_MONTH" => JobFiltersPublishDateRange::PastMonth,
                "PAST_WEEK" => JobFiltersPublishDateRange::PastWeek,
                "PAST_YEAR" => JobFiltersPublishDateRange::PastYear,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobFiltersPublishDateRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobFiltersPublishDateRange {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobFiltersPublishDateRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATE_RANGE_UNSPECIFIED" => JobFiltersPublishDateRange::DateRangeUnspecified,
                "PAST_24_HOURS" => JobFiltersPublishDateRange::Past24Hours,
                "PAST_3_DAYS" => JobFiltersPublishDateRange::Past3Days,
                "PAST_MONTH" => JobFiltersPublishDateRange::PastMonth,
                "PAST_WEEK" => JobFiltersPublishDateRange::PastWeek,
                "PAST_YEAR" => JobFiltersPublishDateRange::PastYear,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobFiltersPublishDateRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobFiltersPublishDateRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobLocation {
        #[doc = "An object representing a latitude/longitude pair."]
        #[serde(
            rename = "latLng",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "The type of a location, which corresponds to the address lines field of\nPostalAddress. For example, \"Downtown, Atlanta, GA, USA\" has a type of\nLocationType#NEIGHBORHOOD, and \"Kansas City, KS, USA\" has a type of\nLocationType#LOCALITY."]
        #[serde(
            rename = "locationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_type: ::std::option::Option<crate::schemas::JobLocationLocationType>,
        #[doc = "Postal address of the location that includes human readable information,\nsuch as postal delivery and payments addresses. Given a postal address,\na postal service can deliver items to a premises, P.O. Box, or other\ndelivery location."]
        #[serde(
            rename = "postalAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_address: ::std::option::Option<crate::schemas::PostalAddress>,
        #[doc = "Radius in meters of the job location. This value is derived from the\nlocation bounding box in which a circle with the specified radius\ncentered from LatLng coves the area associated with the job location.\nFor example, currently, \"Mountain View, CA, USA\" has a radius of\n7885.79 meters."]
        #[serde(
            rename = "radiusMeters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub radius_meters: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for JobLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobLocationLocationType {
        #[doc = "A state or equivalent level location."]
        AdministrativeArea,
        #[doc = "A country level location."]
        Country,
        #[doc = "A city or equivalent level location."]
        Locality,
        #[doc = "Default value if the type is not specified."]
        LocationTypeUnspecified,
        #[doc = "A neighborhood level location."]
        Neighborhood,
        #[doc = "A postal code level location."]
        PostalCode,
        #[doc = "A street address level location."]
        StreetAddress,
        #[doc = "A county or equivalent level location."]
        SubAdministrativeArea,
        #[doc = "A sublocality is a subdivision of a locality, for example a city borough,\nward, or arrondissement. Sublocalities are usually recognized by a local\npolitical authority. For example, Manhattan and Brooklyn are recognized\nas boroughs by the City of New York, and are therefore modeled as\nsublocalities."]
        SubLocality,
        #[doc = "A district or equivalent level location."]
        SubLocality1,
        #[doc = "A smaller district or equivalent level display."]
        SubLocality2,
    }
    impl JobLocationLocationType {
        pub fn as_str(self) -> &'static str {
            match self {
                JobLocationLocationType::AdministrativeArea => "ADMINISTRATIVE_AREA",
                JobLocationLocationType::Country => "COUNTRY",
                JobLocationLocationType::Locality => "LOCALITY",
                JobLocationLocationType::LocationTypeUnspecified => "LOCATION_TYPE_UNSPECIFIED",
                JobLocationLocationType::Neighborhood => "NEIGHBORHOOD",
                JobLocationLocationType::PostalCode => "POSTAL_CODE",
                JobLocationLocationType::StreetAddress => "STREET_ADDRESS",
                JobLocationLocationType::SubAdministrativeArea => "SUB_ADMINISTRATIVE_AREA",
                JobLocationLocationType::SubLocality => "SUB_LOCALITY",
                JobLocationLocationType::SubLocality1 => "SUB_LOCALITY_1",
                JobLocationLocationType::SubLocality2 => "SUB_LOCALITY_2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobLocationLocationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobLocationLocationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobLocationLocationType, ()> {
            Ok(match s {
                "ADMINISTRATIVE_AREA" => JobLocationLocationType::AdministrativeArea,
                "COUNTRY" => JobLocationLocationType::Country,
                "LOCALITY" => JobLocationLocationType::Locality,
                "LOCATION_TYPE_UNSPECIFIED" => JobLocationLocationType::LocationTypeUnspecified,
                "NEIGHBORHOOD" => JobLocationLocationType::Neighborhood,
                "POSTAL_CODE" => JobLocationLocationType::PostalCode,
                "STREET_ADDRESS" => JobLocationLocationType::StreetAddress,
                "SUB_ADMINISTRATIVE_AREA" => JobLocationLocationType::SubAdministrativeArea,
                "SUB_LOCALITY" => JobLocationLocationType::SubLocality,
                "SUB_LOCALITY_1" => JobLocationLocationType::SubLocality1,
                "SUB_LOCALITY_2" => JobLocationLocationType::SubLocality2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobLocationLocationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobLocationLocationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobLocationLocationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMINISTRATIVE_AREA" => JobLocationLocationType::AdministrativeArea,
                "COUNTRY" => JobLocationLocationType::Country,
                "LOCALITY" => JobLocationLocationType::Locality,
                "LOCATION_TYPE_UNSPECIFIED" => JobLocationLocationType::LocationTypeUnspecified,
                "NEIGHBORHOOD" => JobLocationLocationType::Neighborhood,
                "POSTAL_CODE" => JobLocationLocationType::PostalCode,
                "STREET_ADDRESS" => JobLocationLocationType::StreetAddress,
                "SUB_ADMINISTRATIVE_AREA" => JobLocationLocationType::SubAdministrativeArea,
                "SUB_LOCALITY" => JobLocationLocationType::SubLocality,
                "SUB_LOCALITY_1" => JobLocationLocationType::SubLocality1,
                "SUB_LOCALITY_2" => JobLocationLocationType::SubLocality2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobLocationLocationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobLocationLocationType {
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
    pub struct JobProcessingOptions {
        #[doc = "Optional. If set to `true`, the service does not attempt to resolve a\nmore precise address for the job."]
        #[serde(
            rename = "disableStreetAddressResolution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_street_address_resolution: ::std::option::Option<bool>,
        #[doc = "Optional. Option for job HTML content sanitization. Applied fields are:\n\n* description\n* applicationInstruction\n* incentives\n* qualifications\n* responsibilities\n\nHTML tags in these fields may be stripped if sanitiazation is not disabled.\n\nDefaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY."]
        #[serde(
            rename = "htmlSanitization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_sanitization:
            ::std::option::Option<crate::schemas::JobProcessingOptionsHtmlSanitization>,
    }
    impl ::google_field_selector::FieldSelector for JobProcessingOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobProcessingOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobProcessingOptionsHtmlSanitization {
        #[doc = "Disables sanitization on HTML input."]
        HtmlSanitizationDisabled,
        #[doc = "Default value."]
        HtmlSanitizationUnspecified,
        #[doc = "Sanitizes HTML input, only accepts bold, italic, ordered list, and\nunordered list markup tags."]
        SimpleFormattingOnly,
    }
    impl JobProcessingOptionsHtmlSanitization {
        pub fn as_str(self) -> &'static str {
            match self {
                JobProcessingOptionsHtmlSanitization::HtmlSanitizationDisabled => {
                    "HTML_SANITIZATION_DISABLED"
                }
                JobProcessingOptionsHtmlSanitization::HtmlSanitizationUnspecified => {
                    "HTML_SANITIZATION_UNSPECIFIED"
                }
                JobProcessingOptionsHtmlSanitization::SimpleFormattingOnly => {
                    "SIMPLE_FORMATTING_ONLY"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobProcessingOptionsHtmlSanitization {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobProcessingOptionsHtmlSanitization {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobProcessingOptionsHtmlSanitization, ()> {
            Ok(match s {
                "HTML_SANITIZATION_DISABLED" => {
                    JobProcessingOptionsHtmlSanitization::HtmlSanitizationDisabled
                }
                "HTML_SANITIZATION_UNSPECIFIED" => {
                    JobProcessingOptionsHtmlSanitization::HtmlSanitizationUnspecified
                }
                "SIMPLE_FORMATTING_ONLY" => {
                    JobProcessingOptionsHtmlSanitization::SimpleFormattingOnly
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobProcessingOptionsHtmlSanitization {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobProcessingOptionsHtmlSanitization {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobProcessingOptionsHtmlSanitization {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HTML_SANITIZATION_DISABLED" => {
                    JobProcessingOptionsHtmlSanitization::HtmlSanitizationDisabled
                }
                "HTML_SANITIZATION_UNSPECIFIED" => {
                    JobProcessingOptionsHtmlSanitization::HtmlSanitizationUnspecified
                }
                "SIMPLE_FORMATTING_ONLY" => {
                    JobProcessingOptionsHtmlSanitization::SimpleFormattingOnly
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
    impl ::google_field_selector::FieldSelector for JobProcessingOptionsHtmlSanitization {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobProcessingOptionsHtmlSanitization {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobQuery {
        #[doc = "Optional. The category filter specifies the categories of jobs to search against.\nSee Category for more information.\n\nIf a value is not specified, jobs from any category are searched against.\n\nIf multiple values are specified, jobs from any of the specified\ncategories are searched against."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<crate::schemas::JobQueryCategoriesItems>>,
        #[doc = "Optional. Allows filtering jobs by commute time with different travel methods (for\nexample, driving or public transit). Note: This only works with COMMUTE\nMODE. When specified, [JobQuery.location_filters] is\nignored.\n\nCurrently we don't support sorting by commute time."]
        #[serde(
            rename = "commuteFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commute_filter: ::std::option::Option<crate::schemas::CommutePreference>,
        #[doc = "Optional. This filter specifies the exact company display\nname of the jobs to search against.\n\nIf a value isn't specified, jobs within the search results are\nassociated with any company.\n\nIf multiple values are specified, jobs within the search results may be\nassociated with any of the specified companies.\n\nAt most 20 company display name filters are allowed."]
        #[serde(
            rename = "companyDisplayNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_display_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This filter specifies the company entities to search against.\n\nIf a value isn't specified, jobs are searched for against all\ncompanies.\n\nIf multiple values are specified, jobs are searched against the\ncompanies specified.\n\nAt most 20 company filters are allowed."]
        #[serde(
            rename = "companyNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This search filter is applied only to\nJob.compensation_info. For example, if the filter is specified\nas \"Hourly job with per-hour compensation > $15\", only jobs meeting\nthese criteria are searched. If a filter isn't defined, all open jobs\nare searched."]
        #[serde(
            rename = "compensationFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_filter: ::std::option::Option<crate::schemas::CompensationFilter>,
        #[doc = "Optional. This filter specifies a structured syntax to match against the\nJob.custom_attributes marked as `filterable`.\n\nThe syntax for this expression is a subset of Google SQL syntax.\n\nSupported operators are: =, !=, <, <=, >, >= where the left of the operator\nis a custom field key and the right of the operator is a number or string\n(surrounded by quotes) value.\n\nSupported functions are LOWER(<field_name>) to\nperform case insensitive match and EMPTY(<field_name>) to filter on the\nexistence of a key.\n\nBoolean expressions (AND/OR/NOT) are supported up to 3 levels of\nnesting (for example, \"((A AND B AND C) OR NOT D) AND E\"), a maximum of 50\ncomparisons/functions are allowed in the expression. The expression\nmust be < 2000 characters in length.\n\nSample Query:\n(key1 = \"TEST\" OR LOWER(key1)=\"test\" OR NOT EMPTY(key1)) AND key2 > 100"]
        #[serde(
            rename = "customAttributeFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_filter: ::std::option::Option<String>,
        #[doc = "Optional. This flag controls the spell-check feature. If false, the\nservice attempts to correct a misspelled query,\nfor example, \"enginee\" is corrected to \"engineer\".\n\nDefaults to false: a spell check is performed."]
        #[serde(
            rename = "disableSpellCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_spell_check: ::std::option::Option<bool>,
        #[doc = "Optional. The employment type filter specifies the employment type of jobs to\nsearch against, such as EmploymentType.FULL_TIME.\n\nIf a value is not specified, jobs in the search results include any\nemployment type.\n\nIf multiple values are specified, jobs in the search results include\nany of the specified employment types."]
        #[serde(
            rename = "employmentTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub employment_types:
            ::std::option::Option<Vec<crate::schemas::JobQueryEmploymentTypesItems>>,
        #[doc = "Optional. This filter specifies the locale of jobs to search against,\nfor example, \"en-US\".\n\nIf a value isn't specified, the search results can contain jobs in any\nlocale.\n\nLanguage codes should be in BCP-47 format, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).\n\nAt most 10 language code filters are allowed."]
        #[serde(
            rename = "languageCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_codes: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The location filter specifies geo-regions containing the jobs to\nsearch against. See LocationFilter for more information.\n\nIf a location value isn't specified, jobs fitting the other search\ncriteria are retrieved regardless of where they're located.\n\nIf multiple values are specified, jobs are retrieved from any of the\nspecified locations. If different values are specified for the\nLocationFilter.distance_in_miles parameter, the maximum provided\ndistance is used for all locations.\n\nAt most 5 location filters are allowed."]
        #[serde(
            rename = "locationFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_filters: ::std::option::Option<Vec<crate::schemas::LocationFilter>>,
        #[doc = "Optional. Jobs published within a range specified by this filter are searched\nagainst, for example, DateRange.PAST_MONTH. If a value isn't\nspecified, all open jobs are searched against regardless of their\npublished date."]
        #[serde(
            rename = "publishDateRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_date_range: ::std::option::Option<crate::schemas::JobQueryPublishDateRange>,
        #[doc = "Optional. The query string that matches against the job title, description, and\nlocation fields.\n\nThe maximum query size is 255 bytes."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobQuery {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobQueryCategoriesItems {
        AccountingAndFinance,
        AdministrativeAndOffice,
        AdvertisingAndMarketing,
        AnimalCare,
        ArtFashionAndDesign,
        BusinessOperations,
        CleaningAndFacilities,
        ComputerAndIt,
        Construction,
        CustomerService,
        Education,
        EntertainmentAndTravel,
        FarmingAndOutdoors,
        Healthcare,
        HumanResources,
        InstallationMaintenanceAndRepair,
        JobCategoryUnspecified,
        Legal,
        Management,
        ManufacturingAndWarehouse,
        MediaCommunicationsAndWriting,
        OilGasAndMining,
        PersonalCareAndServices,
        ProtectiveServices,
        RealEstate,
        RestaurantAndHospitality,
        SalesAndRetail,
        ScienceAndEngineering,
        SocialServicesAndNonProfit,
        SportsFitnessAndRecreation,
        TransportationAndLogistics,
    }
    impl JobQueryCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobQueryCategoriesItems::AccountingAndFinance => "ACCOUNTING_AND_FINANCE",
                JobQueryCategoriesItems::AdministrativeAndOffice => "ADMINISTRATIVE_AND_OFFICE",
                JobQueryCategoriesItems::AdvertisingAndMarketing => "ADVERTISING_AND_MARKETING",
                JobQueryCategoriesItems::AnimalCare => "ANIMAL_CARE",
                JobQueryCategoriesItems::ArtFashionAndDesign => "ART_FASHION_AND_DESIGN",
                JobQueryCategoriesItems::BusinessOperations => "BUSINESS_OPERATIONS",
                JobQueryCategoriesItems::CleaningAndFacilities => "CLEANING_AND_FACILITIES",
                JobQueryCategoriesItems::ComputerAndIt => "COMPUTER_AND_IT",
                JobQueryCategoriesItems::Construction => "CONSTRUCTION",
                JobQueryCategoriesItems::CustomerService => "CUSTOMER_SERVICE",
                JobQueryCategoriesItems::Education => "EDUCATION",
                JobQueryCategoriesItems::EntertainmentAndTravel => "ENTERTAINMENT_AND_TRAVEL",
                JobQueryCategoriesItems::FarmingAndOutdoors => "FARMING_AND_OUTDOORS",
                JobQueryCategoriesItems::Healthcare => "HEALTHCARE",
                JobQueryCategoriesItems::HumanResources => "HUMAN_RESOURCES",
                JobQueryCategoriesItems::InstallationMaintenanceAndRepair => {
                    "INSTALLATION_MAINTENANCE_AND_REPAIR"
                }
                JobQueryCategoriesItems::JobCategoryUnspecified => "JOB_CATEGORY_UNSPECIFIED",
                JobQueryCategoriesItems::Legal => "LEGAL",
                JobQueryCategoriesItems::Management => "MANAGEMENT",
                JobQueryCategoriesItems::ManufacturingAndWarehouse => "MANUFACTURING_AND_WAREHOUSE",
                JobQueryCategoriesItems::MediaCommunicationsAndWriting => {
                    "MEDIA_COMMUNICATIONS_AND_WRITING"
                }
                JobQueryCategoriesItems::OilGasAndMining => "OIL_GAS_AND_MINING",
                JobQueryCategoriesItems::PersonalCareAndServices => "PERSONAL_CARE_AND_SERVICES",
                JobQueryCategoriesItems::ProtectiveServices => "PROTECTIVE_SERVICES",
                JobQueryCategoriesItems::RealEstate => "REAL_ESTATE",
                JobQueryCategoriesItems::RestaurantAndHospitality => "RESTAURANT_AND_HOSPITALITY",
                JobQueryCategoriesItems::SalesAndRetail => "SALES_AND_RETAIL",
                JobQueryCategoriesItems::ScienceAndEngineering => "SCIENCE_AND_ENGINEERING",
                JobQueryCategoriesItems::SocialServicesAndNonProfit => {
                    "SOCIAL_SERVICES_AND_NON_PROFIT"
                }
                JobQueryCategoriesItems::SportsFitnessAndRecreation => {
                    "SPORTS_FITNESS_AND_RECREATION"
                }
                JobQueryCategoriesItems::TransportationAndLogistics => {
                    "TRANSPORTATION_AND_LOGISTICS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobQueryCategoriesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobQueryCategoriesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobQueryCategoriesItems, ()> {
            Ok(match s {
                "ACCOUNTING_AND_FINANCE" => JobQueryCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => JobQueryCategoriesItems::AdministrativeAndOffice,
                "ADVERTISING_AND_MARKETING" => JobQueryCategoriesItems::AdvertisingAndMarketing,
                "ANIMAL_CARE" => JobQueryCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobQueryCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobQueryCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => JobQueryCategoriesItems::CleaningAndFacilities,
                "COMPUTER_AND_IT" => JobQueryCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobQueryCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobQueryCategoriesItems::CustomerService,
                "EDUCATION" => JobQueryCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => JobQueryCategoriesItems::EntertainmentAndTravel,
                "FARMING_AND_OUTDOORS" => JobQueryCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobQueryCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobQueryCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobQueryCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => JobQueryCategoriesItems::JobCategoryUnspecified,
                "LEGAL" => JobQueryCategoriesItems::Legal,
                "MANAGEMENT" => JobQueryCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => JobQueryCategoriesItems::ManufacturingAndWarehouse,
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobQueryCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobQueryCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => JobQueryCategoriesItems::PersonalCareAndServices,
                "PROTECTIVE_SERVICES" => JobQueryCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobQueryCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => JobQueryCategoriesItems::RestaurantAndHospitality,
                "SALES_AND_RETAIL" => JobQueryCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => JobQueryCategoriesItems::ScienceAndEngineering,
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobQueryCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobQueryCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobQueryCategoriesItems::TransportationAndLogistics
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobQueryCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobQueryCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobQueryCategoriesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNTING_AND_FINANCE" => JobQueryCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => JobQueryCategoriesItems::AdministrativeAndOffice,
                "ADVERTISING_AND_MARKETING" => JobQueryCategoriesItems::AdvertisingAndMarketing,
                "ANIMAL_CARE" => JobQueryCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobQueryCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobQueryCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => JobQueryCategoriesItems::CleaningAndFacilities,
                "COMPUTER_AND_IT" => JobQueryCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobQueryCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobQueryCategoriesItems::CustomerService,
                "EDUCATION" => JobQueryCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => JobQueryCategoriesItems::EntertainmentAndTravel,
                "FARMING_AND_OUTDOORS" => JobQueryCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobQueryCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobQueryCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobQueryCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => JobQueryCategoriesItems::JobCategoryUnspecified,
                "LEGAL" => JobQueryCategoriesItems::Legal,
                "MANAGEMENT" => JobQueryCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => JobQueryCategoriesItems::ManufacturingAndWarehouse,
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobQueryCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobQueryCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => JobQueryCategoriesItems::PersonalCareAndServices,
                "PROTECTIVE_SERVICES" => JobQueryCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobQueryCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => JobQueryCategoriesItems::RestaurantAndHospitality,
                "SALES_AND_RETAIL" => JobQueryCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => JobQueryCategoriesItems::ScienceAndEngineering,
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobQueryCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobQueryCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobQueryCategoriesItems::TransportationAndLogistics
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
    impl ::google_field_selector::FieldSelector for JobQueryCategoriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobQueryCategoriesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobQueryEmploymentTypesItems {
        ContractToHire,
        Contractor,
        EmploymentTypeUnspecified,
        FlyInFlyOut,
        FullTime,
        Intern,
        Other,
        PartTime,
        PerDiem,
        Temporary,
        Volunteer,
    }
    impl JobQueryEmploymentTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobQueryEmploymentTypesItems::ContractToHire => "CONTRACT_TO_HIRE",
                JobQueryEmploymentTypesItems::Contractor => "CONTRACTOR",
                JobQueryEmploymentTypesItems::EmploymentTypeUnspecified => {
                    "EMPLOYMENT_TYPE_UNSPECIFIED"
                }
                JobQueryEmploymentTypesItems::FlyInFlyOut => "FLY_IN_FLY_OUT",
                JobQueryEmploymentTypesItems::FullTime => "FULL_TIME",
                JobQueryEmploymentTypesItems::Intern => "INTERN",
                JobQueryEmploymentTypesItems::Other => "OTHER",
                JobQueryEmploymentTypesItems::PartTime => "PART_TIME",
                JobQueryEmploymentTypesItems::PerDiem => "PER_DIEM",
                JobQueryEmploymentTypesItems::Temporary => "TEMPORARY",
                JobQueryEmploymentTypesItems::Volunteer => "VOLUNTEER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobQueryEmploymentTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobQueryEmploymentTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobQueryEmploymentTypesItems, ()> {
            Ok(match s {
                "CONTRACT_TO_HIRE" => JobQueryEmploymentTypesItems::ContractToHire,
                "CONTRACTOR" => JobQueryEmploymentTypesItems::Contractor,
                "EMPLOYMENT_TYPE_UNSPECIFIED" => {
                    JobQueryEmploymentTypesItems::EmploymentTypeUnspecified
                }
                "FLY_IN_FLY_OUT" => JobQueryEmploymentTypesItems::FlyInFlyOut,
                "FULL_TIME" => JobQueryEmploymentTypesItems::FullTime,
                "INTERN" => JobQueryEmploymentTypesItems::Intern,
                "OTHER" => JobQueryEmploymentTypesItems::Other,
                "PART_TIME" => JobQueryEmploymentTypesItems::PartTime,
                "PER_DIEM" => JobQueryEmploymentTypesItems::PerDiem,
                "TEMPORARY" => JobQueryEmploymentTypesItems::Temporary,
                "VOLUNTEER" => JobQueryEmploymentTypesItems::Volunteer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobQueryEmploymentTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobQueryEmploymentTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobQueryEmploymentTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTRACT_TO_HIRE" => JobQueryEmploymentTypesItems::ContractToHire,
                "CONTRACTOR" => JobQueryEmploymentTypesItems::Contractor,
                "EMPLOYMENT_TYPE_UNSPECIFIED" => {
                    JobQueryEmploymentTypesItems::EmploymentTypeUnspecified
                }
                "FLY_IN_FLY_OUT" => JobQueryEmploymentTypesItems::FlyInFlyOut,
                "FULL_TIME" => JobQueryEmploymentTypesItems::FullTime,
                "INTERN" => JobQueryEmploymentTypesItems::Intern,
                "OTHER" => JobQueryEmploymentTypesItems::Other,
                "PART_TIME" => JobQueryEmploymentTypesItems::PartTime,
                "PER_DIEM" => JobQueryEmploymentTypesItems::PerDiem,
                "TEMPORARY" => JobQueryEmploymentTypesItems::Temporary,
                "VOLUNTEER" => JobQueryEmploymentTypesItems::Volunteer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobQueryEmploymentTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobQueryEmploymentTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobQueryPublishDateRange {
        #[doc = "Default value: Filtering on time is not performed."]
        DateRangeUnspecified,
        #[doc = "The past 24 hours"]
        Past24Hours,
        #[doc = "The past 3 days"]
        Past3Days,
        #[doc = "The past month (30 days)"]
        PastMonth,
        #[doc = "The past week (7 days)"]
        PastWeek,
        #[doc = "The past year (365 days)"]
        PastYear,
    }
    impl JobQueryPublishDateRange {
        pub fn as_str(self) -> &'static str {
            match self {
                JobQueryPublishDateRange::DateRangeUnspecified => "DATE_RANGE_UNSPECIFIED",
                JobQueryPublishDateRange::Past24Hours => "PAST_24_HOURS",
                JobQueryPublishDateRange::Past3Days => "PAST_3_DAYS",
                JobQueryPublishDateRange::PastMonth => "PAST_MONTH",
                JobQueryPublishDateRange::PastWeek => "PAST_WEEK",
                JobQueryPublishDateRange::PastYear => "PAST_YEAR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobQueryPublishDateRange {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobQueryPublishDateRange {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobQueryPublishDateRange, ()> {
            Ok(match s {
                "DATE_RANGE_UNSPECIFIED" => JobQueryPublishDateRange::DateRangeUnspecified,
                "PAST_24_HOURS" => JobQueryPublishDateRange::Past24Hours,
                "PAST_3_DAYS" => JobQueryPublishDateRange::Past3Days,
                "PAST_MONTH" => JobQueryPublishDateRange::PastMonth,
                "PAST_WEEK" => JobQueryPublishDateRange::PastWeek,
                "PAST_YEAR" => JobQueryPublishDateRange::PastYear,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobQueryPublishDateRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobQueryPublishDateRange {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobQueryPublishDateRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATE_RANGE_UNSPECIFIED" => JobQueryPublishDateRange::DateRangeUnspecified,
                "PAST_24_HOURS" => JobQueryPublishDateRange::Past24Hours,
                "PAST_3_DAYS" => JobQueryPublishDateRange::Past3Days,
                "PAST_MONTH" => JobQueryPublishDateRange::PastMonth,
                "PAST_WEEK" => JobQueryPublishDateRange::PastWeek,
                "PAST_YEAR" => JobQueryPublishDateRange::PastYear,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobQueryPublishDateRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobQueryPublishDateRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LatLng {
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
    impl ::google_field_selector::FieldSelector for LatLng {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LatLng {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListCompaniesResponse {
        #[doc = "Companies for the current client."]
        #[serde(
            rename = "companies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub companies: ::std::option::Option<Vec<crate::schemas::Company>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "A token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListCompaniesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCompaniesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListCompanyJobsResponse {
        #[doc = "The Jobs for a given company.\n\nThe maximum number of items returned is based on the limit field\nprovided in the request."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::Job>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "A token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of open jobs. The result will be\nempty if ListCompanyJobsRequest.include_jobs_count is not enabled\nor if no open jobs are available."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_size: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ListCompanyJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCompanyJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListJobsResponse {
        #[doc = "The Jobs for a given company.\n\nThe maximum number of items returned is based on the limit field\nprovided in the request."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::Job>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "A token to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LocationFilter {
        #[doc = "Optional. The distance_in_miles is applied when the location being searched for is\nidentified as a city or smaller. When the location being searched for is a\nstate or larger, this field is ignored."]
        #[serde(
            rename = "distanceInMiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distance_in_miles: ::std::option::Option<f64>,
        #[doc = "Optional. Allows the client to return jobs without a\nset location, specifically, telecommuting jobs (telecomuting is considered\nby the service as a special location.\nJob.allow_telecommute indicates if a job permits telecommuting.\nIf this field is true, telecommuting jobs are searched, and\nname and lat_lng are\nignored.\nThis filter can be used by itself to search exclusively for telecommuting\njobs, or it can be combined with another location\nfilter to search for a combination of job locations,\nsuch as \"Mountain View\" or \"telecommuting\" jobs. However, when used in\ncombination with other location filters, telecommuting jobs can be\ntreated as less relevant than other jobs in the search response."]
        #[serde(
            rename = "isTelecommute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_telecommute: ::std::option::Option<bool>,
        #[doc = "Optional. The latitude and longitude of the geographic center from which to\nsearch. This field is ignored if `location_name` is provided."]
        #[serde(
            rename = "latLng",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Optional. The address name, such as \"Mountain View\" or \"Bay Area\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. CLDR region code of the country/region of the address. This will be used\nto address ambiguity of the user-input location, e.g. \"Liverpool\"\nagainst \"Liverpool, NY, US\" or \"Liverpool, UK\".\n\nSet this field if all the jobs to search against are from a same region,\nor jobs are world-wide but the job seeker is from a specific region.\n\nSee http://cldr.unicode.org/ and\nhttp://www.unicode.org/cldr/charts/30/supplemental/territory_information.html\nfor details. Example: \"CH\" for Switzerland."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LocationFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocationFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MatchingJob {
        #[doc = "Commute information which is generated based on specified\nCommutePreference."]
        #[serde(
            rename = "commuteInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commute_info: ::std::option::Option<crate::schemas::CommuteInfo>,
        #[doc = "Job resource that matches the specified SearchJobsRequest."]
        #[serde(
            rename = "job",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "A summary of the job with core information that's displayed on the search\nresults listing page."]
        #[serde(
            rename = "jobSummary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_summary: ::std::option::Option<String>,
        #[doc = "Contains snippets of text from the Job.job_title field most\nclosely matching a search query's keywords, if available. The matching\nquery keywords are enclosed in HTML bold tags."]
        #[serde(
            rename = "jobTitleSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_title_snippet: ::std::option::Option<String>,
        #[doc = "Contains snippets of text from the Job.description and similar\nfields that most closely match a search query's keywords, if available.\nAll HTML tags in the original fields are stripped when returned in this\nfield, and matching query keywords are enclosed in HTML bold tags."]
        #[serde(
            rename = "searchTextSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_text_snippet: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MatchingJob {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchingJob {
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
    pub struct MendelDebugInput {
        #[doc = "When a request spans multiple servers, a MendelDebugInput may travel with\nthe request and take effect in all the servers. This field is a map of\nnamespaces to NamespacedMendelDebugInput protos. In a single server, up to\ntwo NamespacedMendelDebugInput protos are applied:\n\n1. NamespacedMendelDebugInput with the global namespace (key == \"\").\n1. NamespacedMendelDebugInput with the server's namespace.\n   When both NamespacedMendelDebugInput protos are present, they are merged.\n   See go/mendel-debug-forcing for more details."]
        #[serde(
            rename = "namespacedDebugInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespaced_debug_input: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::NamespacedDebugInput>,
        >,
    }
    impl ::google_field_selector::FieldSelector for MendelDebugInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MendelDebugInput {
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
    pub struct Money {
        #[doc = "The 3-letter currency code defined in ISO 4217."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999 inclusive.\nIf `units` is positive, `nanos` must be positive or zero.\nIf `units` is zero, `nanos` can be positive, zero, or negative.\nIf `units` is negative, `nanos` must be negative or zero.\nFor example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "The whole units of the amount.\nFor example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        #[serde(
            rename = "units",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub units: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Money {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Money {
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
    pub struct NamespacedDebugInput {
        #[doc = "Set of experiment names to be absolutely forced.\nThese experiments will be forced without evaluating the conditions."]
        #[serde(
            rename = "absolutelyForcedExpNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolutely_forced_exp_names: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment tags to be absolutely forced.\nThe experiments with these tags will be forced without evaluating the\nconditions."]
        #[serde(
            rename = "absolutelyForcedExpTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolutely_forced_exp_tags: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment ids to be absolutely forced.\nThese ids will be forced without evaluating the conditions."]
        #[serde(
            rename = "absolutelyForcedExps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolutely_forced_exps: ::std::option::Option<Vec<i32>>,
        #[doc = "Set of experiment names to be conditionally forced.\nThese experiments will be forced only if their conditions and their\nparent domain's conditions are true."]
        #[serde(
            rename = "conditionallyForcedExpNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditionally_forced_exp_names: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment tags to be conditionally forced.\nThe experiments with these tags will be forced only if their conditions\nand their parent domain's conditions are true."]
        #[serde(
            rename = "conditionallyForcedExpTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditionally_forced_exp_tags: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment ids to be conditionally forced.\nThese ids will be forced only if their conditions and their parent\ndomain's conditions are true."]
        #[serde(
            rename = "conditionallyForcedExps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditionally_forced_exps: ::std::option::Option<Vec<i32>>,
        #[doc = "If true, disable automatic enrollment selection (at all diversion\npoints). Automatic enrollment selection means experiment selection\nprocess based on the experiment's automatic enrollment condition. This\ndoes not disable selection of forced experiments."]
        #[serde(
            rename = "disableAutomaticEnrollmentSelection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_automatic_enrollment_selection: ::std::option::Option<bool>,
        #[doc = "Set of experiment names to be disabled.\nIf an experiment is disabled, it is never selected nor forced.\nIf an aggregate experiment is disabled, its partitions are disabled\ntogether. If an experiment with an enrollment is disabled, the enrollment\nis disabled together. If a name corresponds to a domain, the domain\nitself and all descendant experiments and domains are disabled together."]
        #[serde(
            rename = "disableExpNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_exp_names: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment tags to be disabled. All experiments that are tagged\nwith one or more of these tags are disabled.\nIf an experiment is disabled, it is never selected nor forced.\nIf an aggregate experiment is disabled, its partitions are disabled\ntogether. If an experiment with an enrollment is disabled, the enrollment\nis disabled together."]
        #[serde(
            rename = "disableExpTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_exp_tags: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment ids to be disabled.\nIf an experiment is disabled, it is never selected nor forced.\nIf an aggregate experiment is disabled, its partitions are disabled\ntogether. If an experiment with an enrollment is disabled, the enrollment\nis disabled together. If an ID corresponds to a domain, the domain itself\nand all descendant experiments and domains are disabled together."]
        #[serde(
            rename = "disableExps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_exps: ::std::option::Option<Vec<i32>>,
        #[doc = "If true, disable manual enrollment selection (at all diversion points).\nManual enrollment selection means experiment selection process based on\nthe request's manual enrollment states (a.k.a. opt-in experiments).\nThis does not disable selection of forced experiments."]
        #[serde(
            rename = "disableManualEnrollmentSelection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_manual_enrollment_selection: ::std::option::Option<bool>,
        #[doc = "If true, disable organic experiment selection (at all diversion points).\nOrganic selection means experiment selection process based on traffic\nallocation and diversion condition evaluation.\nThis does not disable selection of forced experiments.\n\nThis is useful in cases when it is not known whether experiment selection\nbehavior is responsible for a error or breakage. Disabling organic\nselection may help to isolate the cause of a given problem."]
        #[serde(
            rename = "disableOrganicSelection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_organic_selection: ::std::option::Option<bool>,
        #[doc = "Flags to force in a particular experiment state.\nMap from flag name to flag value."]
        #[serde(
            rename = "forcedFlags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub forced_flags: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Rollouts to force in a particular experiment state.\nMap from rollout name to rollout value."]
        #[serde(
            rename = "forcedRollouts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub forced_rollouts: ::std::option::Option<::std::collections::BTreeMap<String, bool>>,
    }
    impl ::google_field_selector::FieldSelector for NamespacedDebugInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamespacedDebugInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NumericBucketingOption {
        #[doc = "Required. Two adjacent values form a histogram bucket. Values should be in\nascending order. For example, if [5, 10, 15] are provided, four buckets are\ncreated: (-inf, 5), 5, 10), [10, 15), [15, inf). At most 20\n[buckets_bound is supported."]
        #[serde(
            rename = "bucketBounds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_bounds: ::std::option::Option<Vec<f64>>,
        #[doc = "Optional. If set to true, the histogram result includes minimum/maximum\nvalue of the numeric field."]
        #[serde(
            rename = "requiresMinMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requires_min_max: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for NumericBucketingOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumericBucketingOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NumericBucketingResult {
        #[doc = "Count within each bucket. Its size is the length of\nNumericBucketingOption.bucket_bounds plus 1."]
        #[serde(
            rename = "counts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub counts: ::std::option::Option<Vec<crate::schemas::BucketizedCount>>,
        #[doc = "Stores the maximum value of the numeric field. Will be populated only if\n[NumericBucketingOption.requires_min_max] is set to true."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "Stores the minimum value of the numeric field. Will be populated only if\n[NumericBucketingOption.requires_min_max] is set to true."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for NumericBucketingResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumericBucketingResult {
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
    pub struct PostalAddress {
        #[doc = "Unstructured address lines describing the lower levels of an address.\n\nBecause values in address_lines do not have type information and may\nsometimes contain multiple values in a single field (e.g.\n\"Austin, TX\"), it is important that the line order is clear. The order of\naddress lines should be \"envelope order\" for the country/region of the\naddress. In places where this can vary (e.g. Japan), address_language is\nused to make it explicit (e.g. \"ja\" for large-to-small ordering and\n\"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of\nan address can be selected based on the language.\n\nThe minimum permitted structural representation of an address consists\nof a region_code with all remaining information placed in the\naddress_lines. It would be possible to format such an address very\napproximately without geocoding, but no semantic reasoning could be\nmade about any of the address components until it was at least\npartially resolved.\n\nCreating an address only containing a region_code and address_lines, and\nthen geocoding is the recommended way to handle completely unstructured\naddresses (as opposed to guessing which parts of the address should be\nlocalities or administrative areas)."]
        #[serde(
            rename = "addressLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_lines: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Highest administrative subdivision which is used for postal\naddresses of a country or region.\nFor example, this can be a state, a province, an oblast, or a prefecture.\nSpecifically, for Spain this is the province and not the autonomous\ncommunity (e.g. \"Barcelona\" and not \"Catalonia\").\nMany countries don't use an administrative area in postal addresses. E.g.\nin Switzerland this should be left unpopulated."]
        #[serde(
            rename = "administrativeArea",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub administrative_area: ::std::option::Option<String>,
        #[doc = "Optional. BCP-47 language code of the contents of this address (if\nknown). This is often the UI language of the input form or is expected\nto match one of the languages used in the address' country/region, or their\ntransliterated equivalents.\nThis can affect formatting in certain countries, but is not critical\nto the correctness of the data and will never affect any validation or\nother non-formatting related operations.\n\nIf this value is not known, it should be omitted (rather than specifying a\npossibly incorrect default).\n\nExamples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Generally refers to the city/town portion of the address.\nExamples: US city, IT comune, UK post town.\nIn regions of the world where localities are not well defined or do not fit\ninto this structure well, leave locality empty and use address_lines."]
        #[serde(
            rename = "locality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locality: ::std::option::Option<String>,
        #[doc = "Optional. The name of the organization at the address."]
        #[serde(
            rename = "organization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organization: ::std::option::Option<String>,
        #[doc = "Optional. Postal code of the address. Not all countries use or require\npostal codes to be present, but where they are used, they may trigger\nadditional validation with other parts of the address (e.g. state/zip\nvalidation in the U.S.A.)."]
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_code: ::std::option::Option<String>,
        #[doc = "Optional. The recipient at the address.\nThis field may, under certain circumstances, contain multiline information.\nFor example, it might contain \"care of\" information."]
        #[serde(
            rename = "recipients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipients: ::std::option::Option<Vec<String>>,
        #[doc = "Required. CLDR region code of the country/region of the address. This\nis never inferred and it is up to the user to ensure the value is\ncorrect. See http://cldr.unicode.org/ and\nhttp://www.unicode.org/cldr/charts/30/supplemental/territory_information.html\nfor details. Example: \"CH\" for Switzerland."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is\nthe latest revision.\n\nAll new revisions **must** be backward compatible with old revisions."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used\nin most regions. Where it is used, the value is either a string like\n\"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number\nalone, representing the \"sector code\" (Jamaica), \"delivery area indicator\"\n(Malawi) or \"post office indicator\" (e.g. C\u{f4}te d'Ivoire)."]
        #[serde(
            rename = "sortingCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sorting_code: ::std::option::Option<String>,
        #[doc = "Optional. Sublocality of the address.\nFor example, this can be neighborhoods, boroughs, districts."]
        #[serde(
            rename = "sublocality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sublocality: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PostalAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PostalAddress {
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
    pub struct RequestMetadata {
        #[doc = "Optional. The type of device used by the job seeker at the time of the call to the\nservice."]
        #[serde(
            rename = "deviceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_info: ::std::option::Option<crate::schemas::DeviceInfo>,
        #[doc = "Required. The client-defined scope or source of the service call, which typically\nis the domain on\nwhich the service has been implemented and is currently being run.\n\nFor example, if the service is being run by client <em>Foo, Inc.</em>, on\njob board www.foo.com and career site www.bar.com, then this field is\nset to \"foo.com\" for use on the job board, and \"bar.com\" for use on the\ncareer site.\n\nIf this field is not available for some reason, send \"UNKNOWN\". Note that\nany improvements to the service model for a particular tenant site rely on\nthis field being set correctly to some domain."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "Required. A unique session identification string. A session is defined as the\nduration of an end user's interaction with the service over a period.\nObfuscate this field for privacy concerns before\nproviding it to the API.\n\nIf this field is not available for some reason, please send \"UNKNOWN\". Note\nthat any improvements to the service model for a particular tenant site,\nrely on this field being set correctly to some unique session_id."]
        #[serde(
            rename = "sessionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_id: ::std::option::Option<String>,
        #[doc = "Required. A unique user identification string, as determined by the client. The\nclient is responsible for ensuring client-level uniqueness of this value\nin order to have the strongest positive impact on search quality.\nObfuscate this field for privacy concerns before\nproviding it to the service.\n\nIf this field is not available for some reason, please send \"UNKNOWN\". Note\nthat any improvements to the service model for a particular tenant site,\nrely on this field being set correctly to some unique user_id."]
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RequestMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestMetadata {
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
    pub struct ResponseMetadata {
        #[doc = "Identifiers for the versions of the search algorithm used during\nthis API invocation if multiple algorithms are used.\nThe default value is empty.\nFor search response only."]
        #[serde(
            rename = "experimentIdList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub experiment_id_list: ::std::option::Option<Vec<i32>>,
        #[doc = "For search response only. Indicates the mode of a performed search."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::ResponseMetadataMode>,
        #[doc = "A unique id associated with this call.\nThis id is logged for tracking purposes."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResponseMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResponseMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ResponseMetadataMode {
        #[doc = "Deprecated. Please use the SearchJobsForAlert API.\n\nThe job search matches against jobs suited to email notifications."]
        EmailAlertSearch,
        #[doc = "The job search matches only against featured jobs (jobs with a\npromotionValue > 0). This method doesn't return any jobs having a\npromotionValue <= 0. The search results order is determined by the\npromotionValue (jobs with a higher promotionValue are returned higher up in\nthe search results), with relevance being used as a tiebreaker."]
        FeaturedJobSearch,
        #[doc = "The job search doesn't include support for featured jobs."]
        JobSearch,
        #[doc = "The mode of the search method isn't specified."]
        SearchModeUnspecified,
    }
    impl ResponseMetadataMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ResponseMetadataMode::EmailAlertSearch => "EMAIL_ALERT_SEARCH",
                ResponseMetadataMode::FeaturedJobSearch => "FEATURED_JOB_SEARCH",
                ResponseMetadataMode::JobSearch => "JOB_SEARCH",
                ResponseMetadataMode::SearchModeUnspecified => "SEARCH_MODE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ResponseMetadataMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ResponseMetadataMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ResponseMetadataMode, ()> {
            Ok(match s {
                "EMAIL_ALERT_SEARCH" => ResponseMetadataMode::EmailAlertSearch,
                "FEATURED_JOB_SEARCH" => ResponseMetadataMode::FeaturedJobSearch,
                "JOB_SEARCH" => ResponseMetadataMode::JobSearch,
                "SEARCH_MODE_UNSPECIFIED" => ResponseMetadataMode::SearchModeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ResponseMetadataMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ResponseMetadataMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ResponseMetadataMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMAIL_ALERT_SEARCH" => ResponseMetadataMode::EmailAlertSearch,
                "FEATURED_JOB_SEARCH" => ResponseMetadataMode::FeaturedJobSearch,
                "JOB_SEARCH" => ResponseMetadataMode::JobSearch,
                "SEARCH_MODE_UNSPECIFIED" => ResponseMetadataMode::SearchModeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ResponseMetadataMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResponseMetadataMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchJobsRequest {
        #[doc = "Optional. Controls whether to disable relevance thresholding. Relevance\nthresholding removes jobs that have low relevance in search results,\nfor example, removing \"Assistant to the CEO\" positions from the search\nresults of a search for \"CEO\".\n\nDisabling relevance thresholding improves the accuracy of subsequent\nsearch requests.\n\nDefaults to false."]
        #[serde(
            rename = "disableRelevanceThresholding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_relevance_thresholding: ::std::option::Option<bool>,
        #[doc = "Optional. Controls whether to broaden the search when it produces sparse results.\nBroadened queries append results to the end of the matching results\nlist.\n\nDefaults to false."]
        #[serde(
            rename = "enableBroadening",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_broadening: ::std::option::Option<bool>,
        #[doc = "Optional. Controls if the search job request requires the return of a precise\ncount of the first 300 results. Setting this to `true` ensures\nconsistency in the number of results per page. Best practice is to set this\nvalue to true if a client allows users to jump directly to a\nnon-sequential search results page.\n\nEnabling this flag may adversely impact performance.\n\nDefaults to false."]
        #[serde(
            rename = "enablePreciseResultSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_precise_result_size: ::std::option::Option<bool>,
        #[doc = "Deprecated. Use query instead.\n\nOptional.\n\nRestrictions on the scope of the search request, such as filtering\nby location."]
        #[serde(
            rename = "filters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filters: ::std::option::Option<crate::schemas::JobFilters>,
        #[doc = "Optional. Restrictions on what fields to perform histogram on, such as\n`COMPANY_SIZE` etc."]
        #[serde(
            rename = "histogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_facets: ::std::option::Option<crate::schemas::HistogramFacets>,
        #[doc = "Optional. The number of job attributes returned for jobs in the\nsearch response. Defaults to JobView.SMALL if no value is specified."]
        #[serde(
            rename = "jobView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_view: ::std::option::Option<crate::schemas::SearchJobsRequestJobView>,
        #[doc = "Required. Mode of a search."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::SearchJobsRequestMode>,
        #[doc = "Optional. An integer that specifies the current offset (that is, starting result\nlocation, amongst the jobs deemed by the API as relevant) in search\nresults. This field is only considered if page_token is unset.\n\nFor example, 0 means to  return results starting from the first matching\njob, and 10 means to return from the 11th job. This can be used for\npagination, (for example, pageSize = 10 and offset = 10 means to return\nfrom the second page)."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset: ::std::option::Option<i32>,
        #[doc = "Deprecated. Use sort_by instead.\n\nOptional.\n\nThe criteria determining how search results are sorted.\nDefaults to SortBy.RELEVANCE_DESC if no value is specified."]
        #[serde(
            rename = "orderBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_by: ::std::option::Option<crate::schemas::SearchJobsRequestOrderBy>,
        #[doc = "Optional. A limit on the number of jobs returned in the search results.\nIncreasing this value above the default value of 10 can increase search\nresponse time. The value can be between 1 and 100."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. The token specifying the current offset within\nsearch results. See SearchJobsResponse.next_page_token for\nan explanation of how to obtain the next set of query results."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Optional. Query used to search against jobs, such as keyword, location filters, etc."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<crate::schemas::JobQuery>,
        #[doc = "Required. The meta information collected about the job searcher, used to improve the\nsearch quality of the service. The identifiers, (such as `user_id`) are\nprovided by users, and must be unique and consistent."]
        #[serde(
            rename = "requestMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_metadata: ::std::option::Option<crate::schemas::RequestMetadata>,
        #[doc = "Optional. The criteria determining how search results are sorted.\nDefaults to SortBy.RELEVANCE_DESC if no value is specified."]
        #[serde(
            rename = "sortBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_by: ::std::option::Option<crate::schemas::SearchJobsRequestSortBy>,
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestJobView {
        #[doc = "All available attributes are included in the search results.\nNote: [Job.description, Job.responsibilities,\nJob.qualifications and Job.incentives are deprecated.\nThese fields are scheduled to be removed from MatchingJob.Job objects\nin the SearchJobsResponse results on 12/31/2018.\nSee the alternative MatchingJob.search_text_snippet and\nMatchingJob.job_summary fields."]
        Full,
        #[doc = "Default value."]
        JobViewUnspecified,
        #[doc = "A minimal view of the job, with the following attributes in the search\nresults: Job.name, Job.requisition_id, Job.job_title,\nJob.company_name, Job.job_locations."]
        Minimal,
        #[doc = "A small view of the job, with the following attributes in the search\nresults: Job.name, Job.requisition_id, Job.job_title,\nJob.company_name, Job.job_locations, Job.description,\nJob.visibility.\nNote: Job.description is deprecated. It is scheduled to be removed\nfrom MatchingJob.Job objects in the SearchJobsResponse results\non 12/31/2018."]
        Small,
    }
    impl SearchJobsRequestJobView {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestJobView::Full => "FULL",
                SearchJobsRequestJobView::JobViewUnspecified => "JOB_VIEW_UNSPECIFIED",
                SearchJobsRequestJobView::Minimal => "MINIMAL",
                SearchJobsRequestJobView::Small => "SMALL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsRequestJobView {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsRequestJobView {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsRequestJobView, ()> {
            Ok(match s {
                "FULL" => SearchJobsRequestJobView::Full,
                "JOB_VIEW_UNSPECIFIED" => SearchJobsRequestJobView::JobViewUnspecified,
                "MINIMAL" => SearchJobsRequestJobView::Minimal,
                "SMALL" => SearchJobsRequestJobView::Small,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsRequestJobView {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsRequestJobView {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsRequestJobView {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULL" => SearchJobsRequestJobView::Full,
                "JOB_VIEW_UNSPECIFIED" => SearchJobsRequestJobView::JobViewUnspecified,
                "MINIMAL" => SearchJobsRequestJobView::Minimal,
                "SMALL" => SearchJobsRequestJobView::Small,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequestJobView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequestJobView {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestMode {
        #[doc = "Deprecated. Please use the SearchJobsForAlert API.\n\nThe job search matches against jobs suited to email notifications."]
        EmailAlertSearch,
        #[doc = "The job search matches only against featured jobs (jobs with a\npromotionValue > 0). This method doesn't return any jobs having a\npromotionValue <= 0. The search results order is determined by the\npromotionValue (jobs with a higher promotionValue are returned higher up in\nthe search results), with relevance being used as a tiebreaker."]
        FeaturedJobSearch,
        #[doc = "The job search doesn't include support for featured jobs."]
        JobSearch,
        #[doc = "The mode of the search method isn't specified."]
        SearchModeUnspecified,
    }
    impl SearchJobsRequestMode {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestMode::EmailAlertSearch => "EMAIL_ALERT_SEARCH",
                SearchJobsRequestMode::FeaturedJobSearch => "FEATURED_JOB_SEARCH",
                SearchJobsRequestMode::JobSearch => "JOB_SEARCH",
                SearchJobsRequestMode::SearchModeUnspecified => "SEARCH_MODE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsRequestMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsRequestMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsRequestMode, ()> {
            Ok(match s {
                "EMAIL_ALERT_SEARCH" => SearchJobsRequestMode::EmailAlertSearch,
                "FEATURED_JOB_SEARCH" => SearchJobsRequestMode::FeaturedJobSearch,
                "JOB_SEARCH" => SearchJobsRequestMode::JobSearch,
                "SEARCH_MODE_UNSPECIFIED" => SearchJobsRequestMode::SearchModeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsRequestMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsRequestMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsRequestMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMAIL_ALERT_SEARCH" => SearchJobsRequestMode::EmailAlertSearch,
                "FEATURED_JOB_SEARCH" => SearchJobsRequestMode::FeaturedJobSearch,
                "JOB_SEARCH" => SearchJobsRequestMode::JobSearch,
                "SEARCH_MODE_UNSPECIFIED" => SearchJobsRequestMode::SearchModeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequestMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequestMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestOrderBy {
        #[doc = "Sort by job annualized base compensation in ascending order.\nIf job's annualized base compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedBaseCompensation,
        #[doc = "Sort by job annualized base compensation in descending order.\nIf job's annualized base compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedBaseCompensationDesc,
        #[doc = "Sort by job annualized total compensation in ascending order.\nIf job's annualized total compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedTotalCompensation,
        #[doc = "Sort by job annualized total compensation in descending order.\nIf job's annualized total compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedTotalCompensationDesc,
        #[doc = "Sort by published date descending."]
        PublishedDateDesc,
        #[doc = "By descending relevance, as determined by the API algorithms.\n\nRelevance thresholding of query results is only available for queries if\nRELEVANCE_DESC sort ordering is specified."]
        RelevanceDesc,
        #[doc = "Default value."]
        SortByUnspecified,
        #[doc = "Sort by job title ascending."]
        Title,
        #[doc = "Sort by job title descending."]
        TitleDesc,
        #[doc = "Sort by updated date descending."]
        UpdatedDateDesc,
    }
    impl SearchJobsRequestOrderBy {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestOrderBy::AnnualizedBaseCompensation => {
                    "ANNUALIZED_BASE_COMPENSATION"
                }
                SearchJobsRequestOrderBy::AnnualizedBaseCompensationDesc => {
                    "ANNUALIZED_BASE_COMPENSATION_DESC"
                }
                SearchJobsRequestOrderBy::AnnualizedTotalCompensation => {
                    "ANNUALIZED_TOTAL_COMPENSATION"
                }
                SearchJobsRequestOrderBy::AnnualizedTotalCompensationDesc => {
                    "ANNUALIZED_TOTAL_COMPENSATION_DESC"
                }
                SearchJobsRequestOrderBy::PublishedDateDesc => "PUBLISHED_DATE_DESC",
                SearchJobsRequestOrderBy::RelevanceDesc => "RELEVANCE_DESC",
                SearchJobsRequestOrderBy::SortByUnspecified => "SORT_BY_UNSPECIFIED",
                SearchJobsRequestOrderBy::Title => "TITLE",
                SearchJobsRequestOrderBy::TitleDesc => "TITLE_DESC",
                SearchJobsRequestOrderBy::UpdatedDateDesc => "UPDATED_DATE_DESC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsRequestOrderBy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsRequestOrderBy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsRequestOrderBy, ()> {
            Ok(match s {
                "ANNUALIZED_BASE_COMPENSATION" => {
                    SearchJobsRequestOrderBy::AnnualizedBaseCompensation
                }
                "ANNUALIZED_BASE_COMPENSATION_DESC" => {
                    SearchJobsRequestOrderBy::AnnualizedBaseCompensationDesc
                }
                "ANNUALIZED_TOTAL_COMPENSATION" => {
                    SearchJobsRequestOrderBy::AnnualizedTotalCompensation
                }
                "ANNUALIZED_TOTAL_COMPENSATION_DESC" => {
                    SearchJobsRequestOrderBy::AnnualizedTotalCompensationDesc
                }
                "PUBLISHED_DATE_DESC" => SearchJobsRequestOrderBy::PublishedDateDesc,
                "RELEVANCE_DESC" => SearchJobsRequestOrderBy::RelevanceDesc,
                "SORT_BY_UNSPECIFIED" => SearchJobsRequestOrderBy::SortByUnspecified,
                "TITLE" => SearchJobsRequestOrderBy::Title,
                "TITLE_DESC" => SearchJobsRequestOrderBy::TitleDesc,
                "UPDATED_DATE_DESC" => SearchJobsRequestOrderBy::UpdatedDateDesc,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsRequestOrderBy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsRequestOrderBy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsRequestOrderBy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNUALIZED_BASE_COMPENSATION" => {
                    SearchJobsRequestOrderBy::AnnualizedBaseCompensation
                }
                "ANNUALIZED_BASE_COMPENSATION_DESC" => {
                    SearchJobsRequestOrderBy::AnnualizedBaseCompensationDesc
                }
                "ANNUALIZED_TOTAL_COMPENSATION" => {
                    SearchJobsRequestOrderBy::AnnualizedTotalCompensation
                }
                "ANNUALIZED_TOTAL_COMPENSATION_DESC" => {
                    SearchJobsRequestOrderBy::AnnualizedTotalCompensationDesc
                }
                "PUBLISHED_DATE_DESC" => SearchJobsRequestOrderBy::PublishedDateDesc,
                "RELEVANCE_DESC" => SearchJobsRequestOrderBy::RelevanceDesc,
                "SORT_BY_UNSPECIFIED" => SearchJobsRequestOrderBy::SortByUnspecified,
                "TITLE" => SearchJobsRequestOrderBy::Title,
                "TITLE_DESC" => SearchJobsRequestOrderBy::TitleDesc,
                "UPDATED_DATE_DESC" => SearchJobsRequestOrderBy::UpdatedDateDesc,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequestOrderBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequestOrderBy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestSortBy {
        #[doc = "Sort by job annualized base compensation in ascending order.\nIf job's annualized base compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedBaseCompensation,
        #[doc = "Sort by job annualized base compensation in descending order.\nIf job's annualized base compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedBaseCompensationDesc,
        #[doc = "Sort by job annualized total compensation in ascending order.\nIf job's annualized total compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedTotalCompensation,
        #[doc = "Sort by job annualized total compensation in descending order.\nIf job's annualized total compensation is unspecified, they are put at\nthe end of search result."]
        AnnualizedTotalCompensationDesc,
        #[doc = "Sort by published date descending."]
        PublishedDateDesc,
        #[doc = "By descending relevance, as determined by the API algorithms.\n\nRelevance thresholding of query results is only available for queries if\nRELEVANCE_DESC sort ordering is specified."]
        RelevanceDesc,
        #[doc = "Default value."]
        SortByUnspecified,
        #[doc = "Sort by job title ascending."]
        Title,
        #[doc = "Sort by job title descending."]
        TitleDesc,
        #[doc = "Sort by updated date descending."]
        UpdatedDateDesc,
    }
    impl SearchJobsRequestSortBy {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestSortBy::AnnualizedBaseCompensation => {
                    "ANNUALIZED_BASE_COMPENSATION"
                }
                SearchJobsRequestSortBy::AnnualizedBaseCompensationDesc => {
                    "ANNUALIZED_BASE_COMPENSATION_DESC"
                }
                SearchJobsRequestSortBy::AnnualizedTotalCompensation => {
                    "ANNUALIZED_TOTAL_COMPENSATION"
                }
                SearchJobsRequestSortBy::AnnualizedTotalCompensationDesc => {
                    "ANNUALIZED_TOTAL_COMPENSATION_DESC"
                }
                SearchJobsRequestSortBy::PublishedDateDesc => "PUBLISHED_DATE_DESC",
                SearchJobsRequestSortBy::RelevanceDesc => "RELEVANCE_DESC",
                SearchJobsRequestSortBy::SortByUnspecified => "SORT_BY_UNSPECIFIED",
                SearchJobsRequestSortBy::Title => "TITLE",
                SearchJobsRequestSortBy::TitleDesc => "TITLE_DESC",
                SearchJobsRequestSortBy::UpdatedDateDesc => "UPDATED_DATE_DESC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsRequestSortBy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsRequestSortBy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsRequestSortBy, ()> {
            Ok(match s {
                "ANNUALIZED_BASE_COMPENSATION" => {
                    SearchJobsRequestSortBy::AnnualizedBaseCompensation
                }
                "ANNUALIZED_BASE_COMPENSATION_DESC" => {
                    SearchJobsRequestSortBy::AnnualizedBaseCompensationDesc
                }
                "ANNUALIZED_TOTAL_COMPENSATION" => {
                    SearchJobsRequestSortBy::AnnualizedTotalCompensation
                }
                "ANNUALIZED_TOTAL_COMPENSATION_DESC" => {
                    SearchJobsRequestSortBy::AnnualizedTotalCompensationDesc
                }
                "PUBLISHED_DATE_DESC" => SearchJobsRequestSortBy::PublishedDateDesc,
                "RELEVANCE_DESC" => SearchJobsRequestSortBy::RelevanceDesc,
                "SORT_BY_UNSPECIFIED" => SearchJobsRequestSortBy::SortByUnspecified,
                "TITLE" => SearchJobsRequestSortBy::Title,
                "TITLE_DESC" => SearchJobsRequestSortBy::TitleDesc,
                "UPDATED_DATE_DESC" => SearchJobsRequestSortBy::UpdatedDateDesc,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsRequestSortBy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsRequestSortBy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsRequestSortBy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNUALIZED_BASE_COMPENSATION" => {
                    SearchJobsRequestSortBy::AnnualizedBaseCompensation
                }
                "ANNUALIZED_BASE_COMPENSATION_DESC" => {
                    SearchJobsRequestSortBy::AnnualizedBaseCompensationDesc
                }
                "ANNUALIZED_TOTAL_COMPENSATION" => {
                    SearchJobsRequestSortBy::AnnualizedTotalCompensation
                }
                "ANNUALIZED_TOTAL_COMPENSATION_DESC" => {
                    SearchJobsRequestSortBy::AnnualizedTotalCompensationDesc
                }
                "PUBLISHED_DATE_DESC" => SearchJobsRequestSortBy::PublishedDateDesc,
                "RELEVANCE_DESC" => SearchJobsRequestSortBy::RelevanceDesc,
                "SORT_BY_UNSPECIFIED" => SearchJobsRequestSortBy::SortByUnspecified,
                "TITLE" => SearchJobsRequestSortBy::Title,
                "TITLE_DESC" => SearchJobsRequestSortBy::TitleDesc,
                "UPDATED_DATE_DESC" => SearchJobsRequestSortBy::UpdatedDateDesc,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequestSortBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequestSortBy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchJobsResponse {
        #[doc = "The commute filter the service applied to the specified query. This\ninformation is only available when query has a valid CommutePreference."]
        #[serde(
            rename = "appliedCommuteFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_commute_filter: ::std::option::Option<crate::schemas::CommutePreference>,
        #[doc = "The location filters that the service applied to the specified query. If\nany filters are lat-lng based, the JobLocation.location_type is\nJobLocation.LocationType#LOCATION_TYPE_UNSPECIFIED."]
        #[serde(
            rename = "appliedJobLocationFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_job_location_filters: ::std::option::Option<Vec<crate::schemas::JobLocation>>,
        #[doc = "An estimation of the number of jobs that match the specified query.\n\nThis number is not guaranteed to be accurate. For accurate results,\nseenenable_precise_result_size."]
        #[serde(
            rename = "estimatedTotalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub estimated_total_size: ::std::option::Option<i64>,
        #[doc = "The histogram results that match specified\nSearchJobsRequest.HistogramFacets."]
        #[serde(
            rename = "histogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_results: ::std::option::Option<crate::schemas::HistogramResults>,
        #[doc = "Corresponds to SearchJobsRequest.job_view."]
        #[serde(
            rename = "jobView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_view: ::std::option::Option<crate::schemas::SearchJobsResponseJobView>,
        #[doc = "The Job entities that match the specified SearchJobsRequest."]
        #[serde(
            rename = "matchingJobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matching_jobs: ::std::option::Option<Vec<crate::schemas::MatchingJob>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "The token that specifies the starting position of the next page of results.\nThis field is empty if there are no more results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "If query broadening is enabled, we may append additional results from the\nbroadened query. This number indicates how many of the jobs returned in the\njobs field are from the broadened query. These results are always at the\nend of the jobs list. In particular, a value of 0 means all the jobs in the\njobs list are from the original (without broadening) query. If this\nfield is non-zero, subsequent requests with offset after this result set\nshould contain all broadened results."]
        #[serde(
            rename = "numJobsFromBroadenedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_jobs_from_broadened_query: ::std::option::Option<i32>,
        #[doc = "The spell checking result, and correction."]
        #[serde(
            rename = "spellResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spell_result: ::std::option::Option<crate::schemas::SpellingCorrection>,
        #[doc = "The precise result count, which is available only if the client set\nenable_precise_result_size to `true` or if the response\nis the last page of results. Otherwise, the value will be `-1`."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_size: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsResponseJobView {
        #[doc = "All available attributes are included in the search results.\nNote: [Job.description, Job.responsibilities,\nJob.qualifications and Job.incentives are deprecated.\nThese fields are scheduled to be removed from MatchingJob.Job objects\nin the SearchJobsResponse results on 12/31/2018.\nSee the alternative MatchingJob.search_text_snippet and\nMatchingJob.job_summary fields."]
        Full,
        #[doc = "Default value."]
        JobViewUnspecified,
        #[doc = "A minimal view of the job, with the following attributes in the search\nresults: Job.name, Job.requisition_id, Job.job_title,\nJob.company_name, Job.job_locations."]
        Minimal,
        #[doc = "A small view of the job, with the following attributes in the search\nresults: Job.name, Job.requisition_id, Job.job_title,\nJob.company_name, Job.job_locations, Job.description,\nJob.visibility.\nNote: Job.description is deprecated. It is scheduled to be removed\nfrom MatchingJob.Job objects in the SearchJobsResponse results\non 12/31/2018."]
        Small,
    }
    impl SearchJobsResponseJobView {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsResponseJobView::Full => "FULL",
                SearchJobsResponseJobView::JobViewUnspecified => "JOB_VIEW_UNSPECIFIED",
                SearchJobsResponseJobView::Minimal => "MINIMAL",
                SearchJobsResponseJobView::Small => "SMALL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsResponseJobView {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsResponseJobView {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsResponseJobView, ()> {
            Ok(match s {
                "FULL" => SearchJobsResponseJobView::Full,
                "JOB_VIEW_UNSPECIFIED" => SearchJobsResponseJobView::JobViewUnspecified,
                "MINIMAL" => SearchJobsResponseJobView::Minimal,
                "SMALL" => SearchJobsResponseJobView::Small,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsResponseJobView {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsResponseJobView {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsResponseJobView {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULL" => SearchJobsResponseJobView::Full,
                "JOB_VIEW_UNSPECIFIED" => SearchJobsResponseJobView::JobViewUnspecified,
                "MINIMAL" => SearchJobsResponseJobView::Minimal,
                "SMALL" => SearchJobsResponseJobView::Small,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsResponseJobView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsResponseJobView {
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
    pub struct SpellingCorrection {
        #[doc = "Indicates if the query was corrected by the spell checker."]
        #[serde(
            rename = "corrected",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected: ::std::option::Option<bool>,
        #[doc = "Correction output consisting of the corrected keyword string."]
        #[serde(
            rename = "correctedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpellingCorrection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpellingCorrection {
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
    pub struct StringValues {
        #[doc = "Required. String values."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for StringValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringValues {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateJobRequest {
        #[doc = "Deprecated. Please use processing_options. This flag is ignored if\nprocessing_options is set.\n\nOptional.\n\nIf set to `true`, the service does not attempt resolve a more precise\naddress for the job."]
        #[serde(
            rename = "disableStreetAddressResolution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_street_address_resolution: ::std::option::Option<bool>,
        #[doc = "Required. The Job to be updated."]
        #[serde(
            rename = "job",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "Optional. Options for job processing.\n\nUpdateJobRequest.disable_street_address_resolution is ignored if this\nflag is set."]
        #[serde(
            rename = "processingOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processing_options: ::std::option::Option<crate::schemas::JobProcessingOptions>,
        #[doc = "Optional but strongly recommended to be provided for the best service\nexperience.\n\nIf update_job_fields is provided, only the specified fields in\njob are updated. Otherwise all the fields are updated.\n\nA field mask to restrict the fields that are updated. Valid values are:\n\n* jobTitle\n* employmentTypes\n* description\n* applicationUrls\n* applicationEmailList\n* applicationInstruction\n* responsibilities\n* qualifications\n* educationLevels\n* level\n* department\n* startDate\n* endDate\n* compensationInfo\n* incentives\n* languageCode\n* benefits\n* expireTime\n* customAttributes\n* visibility\n* publishDate\n* promotionValue\n* locations\n* region\n* expiryDate (deprecated)\n* filterableCustomFields (deprecated)\n* unindexedCustomFields (deprecated)"]
        #[serde(
            rename = "updateJobFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_job_fields: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateJobRequest {
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
    #[doc = "Actions that can be performed on the companies resource"]
    pub fn companies(&self) -> crate::resources::companies::CompaniesActions {
        crate::resources::companies::CompaniesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the jobs resource"]
    pub fn jobs(&self) -> crate::resources::jobs::JobsActions {
        crate::resources::jobs::JobsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_2 resource"]
    pub fn v_2(&self) -> crate::resources::v_2::V2Actions {
        crate::resources::v_2::V2Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod companies {
        pub mod params {}
        pub struct CompaniesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CompaniesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a new company entity."]
            pub fn create(&self, request: crate::schemas::Company) -> CreateRequestBuilder {
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
            #[doc = "Deletes the specified company."]
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
            #[doc = "Retrieves the specified company."]
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
            #[doc = "Lists all companies associated with a Cloud Talent Solution account."]
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
                    must_have_open_jobs: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Updates the specified company. Company names can't be updated. To update a\ncompany name, delete the company and all jobs associated with it, and only\nthen re-create them."]
            pub fn patch(
                &self,
                request: crate::schemas::Company,
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
                    update_company_fields: None,
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> crate::resources::companies::jobs::JobsActions {
                crate::resources::companies::jobs::JobsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [CompaniesActions::create()](struct.CompaniesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Company,
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
            ) -> Result<crate::schemas::Company, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Company, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/companies");
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
        #[doc = "Created via [CompaniesActions::delete()](struct.CompaniesActions.html#method.delete)"]
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
                let req = self._request(&self._path())?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [CompaniesActions::get()](struct.CompaniesActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Company, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Company, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
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
        #[doc = "Created via [CompaniesActions::list()](struct.CompaniesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            must_have_open_jobs: Option<bool>,
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
            #[doc = "Optional. Set to true if the companies request must have open jobs.\n\nDefaults to false.\n\nIf true, at most page_size of companies are fetched, among which\nonly those with open jobs are returned."]
            pub fn must_have_open_jobs(mut self, value: bool) -> Self {
                self.must_have_open_jobs = Some(value);
                self
            }
            #[doc = "Optional. The maximum number of companies to be returned, at most 100.\nDefault is 100 if a non-positive number is provided."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional. The starting indicator from which to return results."]
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_companies<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_companies_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_companies_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Company> {
                self.iter_companies_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_companies_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Company> {
                self.iter_companies_with_fields(Some("*"))
            }
            pub fn iter_companies_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "companies").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "companies")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListCompaniesResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListCompaniesResponse> {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
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
            ) -> Result<crate::schemas::ListCompaniesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListCompaniesResponse, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/companies");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("mustHaveOpenJobs", &self.must_have_open_jobs)]);
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                todo!("implement async `execute` method for `IterableMethod` trait")
            }
        }
        #[doc = "Created via [CompaniesActions::patch()](struct.CompaniesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Company,
            name: String,
            update_company_fields: Option<String>,
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
            #[doc = "Optional but strongly recommended to be provided for the best service\nexperience.\n\nIf update_company_fields is provided, only the specified fields in\ncompany are updated. Otherwise all the fields are updated.\n\nA field mask to specify the company fields to update. Valid values are:\n\n* displayName\n* website\n* imageUrl\n* companySize\n* distributorBillingCompanyId\n* companyInfoSources\n* careerPageLink\n* hiringAgency\n* hqLocation\n* eeoText\n* keywordSearchableCustomAttributes\n* title (deprecated)\n* keywordSearchableCustomFields (deprecated)"]
            pub fn update_company_fields(mut self, value: impl Into<String>) -> Self {
                self.update_company_fields = Some(value.into());
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
            ) -> Result<crate::schemas::Company, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Company, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                let req = req.query(&[("updateCompanyFields", &self.update_company_fields)]);
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
        pub mod jobs {
            pub mod params {}
            pub struct JobsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> JobsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deprecated. Use ListJobs instead.\n\nLists all jobs associated with a company."]
                pub fn list(&self, company_name: impl Into<String>) -> ListRequestBuilder {
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
                        company_name: company_name.into(),
                        ids_only: None,
                        include_jobs_count: None,
                        job_requisition_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [JobsActions::list()](struct.JobsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                company_name: String,
                ids_only: Option<bool>,
                include_jobs_count: Option<bool>,
                job_requisition_id: Option<String>,
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
                #[doc = "Optional. If set to `true`, only job ID, job requisition ID and language code will be\nreturned.\n\nA typical use is to synchronize job repositories.\n\nDefaults to false."]
                pub fn ids_only(mut self, value: bool) -> Self {
                    self.ids_only = Some(value);
                    self
                }
                #[doc = "Deprecated. Please DO NOT use this field except for small companies.\nSuggest counting jobs page by page instead.\n\nOptional.\n\nSet to true if the total number of open jobs is to be returned.\n\nDefaults to false."]
                pub fn include_jobs_count(mut self, value: bool) -> Self {
                    self.include_jobs_count = Some(value);
                    self
                }
                #[doc = "Optional. The requisition ID, also known as posting ID, assigned by the company\nto the job.\n\nThe maximum number of allowable characters is 225."]
                pub fn job_requisition_id(mut self, value: impl Into<String>) -> Self {
                    self.job_requisition_id = Some(value.into());
                    self
                }
                #[doc = "Optional. The maximum number of jobs to be returned per page of results.\n\nIf ids_only is set to true, the maximum allowed page size\nis 1000. Otherwise, the maximum allowed page size is 100.\n\nDefault is 100 if empty or a number < 1 is specified."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. The starting point of a query result."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_jobs<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_jobs_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_jobs_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.iter_jobs_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_jobs_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.iter_jobs_with_fields(Some("*"))
                }
                pub fn iter_jobs_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "jobs").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCompanyJobsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCompanyJobsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
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
                ) -> Result<crate::schemas::ListCompanyJobsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListCompanyJobsResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.company_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("idsOnly", &self.ids_only)]);
                    let req = req.query(&[("includeJobsCount", &self.include_jobs_count)]);
                    let req = req.query(&[("jobRequisitionId", &self.job_requisition_id)]);
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    todo!("implement async `execute` method for `IterableMethod` trait")
                }
            }
        }
    }
    pub mod jobs {
        pub mod params {}
        pub struct JobsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> JobsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes a list of Job postings by filter."]
            pub fn batch_delete(
                &self,
                request: crate::schemas::BatchDeleteJobsRequest,
            ) -> BatchDeleteRequestBuilder {
                BatchDeleteRequestBuilder {
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
            #[doc = "Creates a new job.\n\nTypically, the job becomes searchable within 10 seconds, but it may take\nup to 5 minutes."]
            pub fn create(
                &self,
                request: crate::schemas::CreateJobRequest,
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
            #[doc = "Deletes the specified job.\n\nTypically, the job becomes unsearchable within 10 seconds, but it may take\nup to 5 minutes."]
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
                    disable_fast_process: None,
                }
            }
            #[doc = "Deprecated. Use BatchDeleteJobs instead.\n\nDeletes the specified job by filter. You can specify whether to\nsynchronously wait for validation, indexing, and general processing to be\ncompleted before the response is returned."]
            pub fn delete_by_filter(
                &self,
                request: crate::schemas::DeleteJobsByFilterRequest,
            ) -> DeleteByFilterRequestBuilder {
                DeleteByFilterRequestBuilder {
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
            #[doc = "Retrieves the specified job, whose status is OPEN or recently EXPIRED\nwithin the last 90 days."]
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
            #[doc = "Deprecated. Use SearchJobsRequest.histogram_facets instead to make\na single call with both search and histogram.\n\nRetrieves a histogram for the given\nGetHistogramRequest. This call provides a structured\ncount of jobs that match against the search query, grouped by specified\nfacets.\n\nThis call constrains the visibility of jobs\npresent in the database, and only counts jobs the caller has\npermission to search against.\n\nFor example, use this call to generate the\nnumber of jobs in the U.S. by state."]
            pub fn histogram(
                &self,
                request: crate::schemas::GetHistogramRequest,
            ) -> HistogramRequestBuilder {
                HistogramRequestBuilder {
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
            #[doc = "Lists jobs by filter."]
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
                    filter: None,
                    ids_only: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Updates specified job.\n\nTypically, updated contents become visible in search results within 10\nseconds, but it may take up to 5 minutes."]
            pub fn patch(
                &self,
                request: crate::schemas::UpdateJobRequest,
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
                }
            }
            #[doc = "Searches for jobs using the provided SearchJobsRequest.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs that the caller has\npermission to search against."]
            pub fn search(
                &self,
                request: crate::schemas::SearchJobsRequest,
            ) -> SearchRequestBuilder {
                SearchRequestBuilder {
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
            #[doc = "Searches for jobs using the provided SearchJobsRequest.\n\nThis API call is intended for the use case of targeting passive job\nseekers (for example, job seekers who have signed up to receive email\nalerts about potential job opportunities), and has different algorithmic\nadjustments that are targeted to passive job seekers.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs the caller has\npermission to search against."]
            pub fn search_for_alert(
                &self,
                request: crate::schemas::SearchJobsRequest,
            ) -> SearchForAlertRequestBuilder {
                SearchForAlertRequestBuilder {
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
        #[doc = "Created via [JobsActions::batch_delete()](struct.JobsActions.html#method.batch_delete)"]
        #[derive(Debug, Clone)]
        pub struct BatchDeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchDeleteJobsRequest,
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
        impl<'a> BatchDeleteRequestBuilder<'a> {
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs:batchDelete");
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
        #[doc = "Created via [JobsActions::create()](struct.JobsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CreateJobRequest,
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
            ) -> Result<crate::schemas::Job, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Job, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs");
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
        #[doc = "Created via [JobsActions::delete()](struct.JobsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            disable_fast_process: Option<bool>,
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
            #[doc = "Deprecated. This field is not working anymore.\n\nOptional.\n\nIf set to true, this call waits for all processing steps to complete\nbefore the job is cleaned up. Otherwise, the call returns while some\nsteps are still taking place asynchronously, hence faster."]
            pub fn disable_fast_process(mut self, value: bool) -> Self {
                self.disable_fast_process = Some(value);
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
                let req = self._request(&self._path())?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("disableFastProcess", &self.disable_fast_process)]);
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
        #[doc = "Created via [JobsActions::delete_by_filter()](struct.JobsActions.html#method.delete_by_filter)"]
        #[derive(Debug, Clone)]
        pub struct DeleteByFilterRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::DeleteJobsByFilterRequest,
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
        impl<'a> DeleteByFilterRequestBuilder<'a> {
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs:deleteByFilter");
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
        #[doc = "Created via [JobsActions::get()](struct.JobsActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Job, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Job, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
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
        #[doc = "Created via [JobsActions::histogram()](struct.JobsActions.html#method.histogram)"]
        #[derive(Debug, Clone)]
        pub struct HistogramRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GetHistogramRequest,
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
        impl<'a> HistogramRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GetHistogramResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetHistogramResponse, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs:histogram");
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
        #[doc = "Created via [JobsActions::list()](struct.JobsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            filter: Option<String>,
            ids_only: Option<bool>,
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
            #[doc = "Required. The filter string specifies the jobs to be enumerated.\n\nSupported operator: =, AND\n\nThe fields eligible for filtering are:\n\n* `companyName` (Required)\n* `requisitionId` (Optional)\n\nSample Query:\n\n* companyName = \"companies/123\"\n* companyName = \"companies/123\" AND requisitionId = \"req-1\""]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Optional. If set to `true`, only Job.name, Job.requisition_id and\nJob.language_code will be returned.\n\nA typical use case is to synchronize job repositories.\n\nDefaults to false."]
            pub fn ids_only(mut self, value: bool) -> Self {
                self.ids_only = Some(value);
                self
            }
            #[doc = "Optional. The maximum number of jobs to be returned per page of results.\n\nIf ids_only is set to true, the maximum allowed page size\nis 1000. Otherwise, the maximum allowed page size is 100.\n\nDefault is 100 if empty or a number < 1 is specified."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional. The starting point of a query result."]
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_jobs<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_jobs_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_jobs_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                self.iter_jobs_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_jobs_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                self.iter_jobs_with_fields(Some("*"))
            }
            pub fn iter_jobs_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "jobs").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "jobs")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
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
            ) -> Result<crate::schemas::ListJobsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListJobsResponse, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("idsOnly", &self.ids_only)]);
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                todo!("implement async `execute` method for `IterableMethod` trait")
            }
        }
        #[doc = "Created via [JobsActions::patch()](struct.JobsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::UpdateJobRequest,
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
        impl<'a> PatchRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Job, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Job, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        #[doc = "Created via [JobsActions::search()](struct.JobsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SearchJobsRequest,
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
        impl<'a> SearchRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SearchJobsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchJobsResponse, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs:search");
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
        #[doc = "Created via [JobsActions::search_for_alert()](struct.JobsActions.html#method.search_for_alert)"]
        #[derive(Debug, Clone)]
        pub struct SearchForAlertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SearchJobsRequest,
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
        impl<'a> SearchForAlertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SearchJobsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchJobsResponse, crate::Error> {
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2/jobs:searchForAlert");
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
    pub mod v_2 {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum CompleteType {
                Combined,
                CompanyName,
                CompletionTypeUnspecified,
                JobTitle,
            }
            impl CompleteType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        CompleteType::Combined => "COMBINED",
                        CompleteType::CompanyName => "COMPANY_NAME",
                        CompleteType::CompletionTypeUnspecified => "COMPLETION_TYPE_UNSPECIFIED",
                        CompleteType::JobTitle => "JOB_TITLE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for CompleteType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for CompleteType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<CompleteType, ()> {
                    Ok(match s {
                        "COMBINED" => CompleteType::Combined,
                        "COMPANY_NAME" => CompleteType::CompanyName,
                        "COMPLETION_TYPE_UNSPECIFIED" => CompleteType::CompletionTypeUnspecified,
                        "JOB_TITLE" => CompleteType::JobTitle,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for CompleteType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for CompleteType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for CompleteType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "COMBINED" => CompleteType::Combined,
                        "COMPANY_NAME" => CompleteType::CompanyName,
                        "COMPLETION_TYPE_UNSPECIFIED" => CompleteType::CompletionTypeUnspecified,
                        "JOB_TITLE" => CompleteType::JobTitle,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for CompleteType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for CompleteType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum CompleteScope {
                CompletionScopeUnspecified,
                Public,
                Tenant,
            }
            impl CompleteScope {
                pub fn as_str(self) -> &'static str {
                    match self {
                        CompleteScope::CompletionScopeUnspecified => "COMPLETION_SCOPE_UNSPECIFIED",
                        CompleteScope::Public => "PUBLIC",
                        CompleteScope::Tenant => "TENANT",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for CompleteScope {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for CompleteScope {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<CompleteScope, ()> {
                    Ok(match s {
                        "COMPLETION_SCOPE_UNSPECIFIED" => CompleteScope::CompletionScopeUnspecified,
                        "PUBLIC" => CompleteScope::Public,
                        "TENANT" => CompleteScope::Tenant,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for CompleteScope {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for CompleteScope {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for CompleteScope {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "COMPLETION_SCOPE_UNSPECIFIED" => CompleteScope::CompletionScopeUnspecified,
                        "PUBLIC" => CompleteScope::Public,
                        "TENANT" => CompleteScope::Tenant,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for CompleteScope {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for CompleteScope {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct V2Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V2Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Completes the specified prefix with job keyword suggestions.\nIntended for use by a job search auto-complete search box."]
            pub fn complete(&self) -> CompleteRequestBuilder {
                CompleteRequestBuilder {
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
                    company_name: None,
                    language_code: None,
                    page_size: None,
                    query: None,
                    r#type: None,
                    scope: None,
                }
            }
        }
        #[doc = "Created via [V2Actions::complete()](struct.V2Actions.html#method.complete)"]
        #[derive(Debug, Clone)]
        pub struct CompleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            company_name: Option<String>,
            language_code: Option<String>,
            page_size: Option<i32>,
            query: Option<String>,
            r#type: Option<crate::resources::v_2::params::CompleteType>,
            scope: Option<crate::resources::v_2::params::CompleteScope>,
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
        impl<'a> CompleteRequestBuilder<'a> {
            #[doc = "Optional. If provided, restricts completion to the specified company."]
            pub fn company_name(mut self, value: impl Into<String>) -> Self {
                self.company_name = Some(value.into());
                self
            }
            #[doc = "Required. The language of the query. This is\nthe BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).\n\nFor CompletionType.JOB_TITLE type, only open jobs with same\nlanguage_code are returned.\n\nFor CompletionType.COMPANY_NAME type,\nonly companies having open jobs with same language_code are\nreturned.\n\nFor CompletionType.COMBINED type, only open jobs with same\nlanguage_code or companies having open jobs with same\nlanguage_code are returned."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "Required. Completion result count.\nThe maximum allowed page size is 10."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Required. The query used to generate suggestions."]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
                self
            }
            #[doc = "Optional. The completion topic. The default is CompletionType.COMBINED."]
            pub fn r#type(mut self, value: crate::resources::v_2::params::CompleteType) -> Self {
                self.r#type = Some(value);
                self
            }
            #[doc = "Optional. The scope of the completion. The defaults is CompletionScope.PUBLIC."]
            pub fn scope(mut self, value: crate::resources::v_2::params::CompleteScope) -> Self {
                self.scope = Some(value);
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
            ) -> Result<crate::schemas::CompleteQueryResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CompleteQueryResponse, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v2:complete");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("companyName", &self.company_name)]);
                let req = req.query(&[("languageCode", &self.language_code)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("query", &self.query)]);
                let req = req.query(&[("type", &self.r#type)]);
                let req = req.query(&[("scope", &self.scope)]);
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
