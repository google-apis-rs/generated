#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*complete*](resources/projects/struct.CompleteRequestBuilder.html)\n      * [client_events](resources/projects/client_events/struct.ClientEventsActions.html)\n        * [*create*](resources/projects/client_events/struct.CreateRequestBuilder.html)\n      * [companies](resources/projects/companies/struct.CompaniesActions.html)\n        * [*create*](resources/projects/companies/struct.CreateRequestBuilder.html), [*delete*](resources/projects/companies/struct.DeleteRequestBuilder.html), [*get*](resources/projects/companies/struct.GetRequestBuilder.html), [*list*](resources/projects/companies/struct.ListRequestBuilder.html), [*patch*](resources/projects/companies/struct.PatchRequestBuilder.html)\n      * [jobs](resources/projects/jobs/struct.JobsActions.html)\n        * [*batchDelete*](resources/projects/jobs/struct.BatchDeleteRequestBuilder.html), [*create*](resources/projects/jobs/struct.CreateRequestBuilder.html), [*delete*](resources/projects/jobs/struct.DeleteRequestBuilder.html), [*get*](resources/projects/jobs/struct.GetRequestBuilder.html), [*list*](resources/projects/jobs/struct.ListRequestBuilder.html), [*patch*](resources/projects/jobs/struct.PatchRequestBuilder.html), [*search*](resources/projects/jobs/struct.SearchRequestBuilder.html), [*searchForAlert*](resources/projects/jobs/struct.SearchForAlertRequestBuilder.html)\n      * [operations](resources/projects/operations/struct.OperationsActions.html)\n        * [*get*](resources/projects/operations/struct.GetRequestBuilder.html)\n"]
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
    pub struct ApplicationInfo {
        #[doc = "Optional but at least one of uris, emails or instruction must be specified. Use this field to specify email address(es) to which resumes or applications can be sent. The maximum number of allowed characters for each entry is 255."]
        #[serde(
            rename = "emails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub emails: ::std::option::Option<Vec<String>>,
        #[doc = "Optional but at least one of uris, emails or instruction must be specified. Use this field to provide instructions, such as \"Mail your application to ...\", that a candidate can follow to apply for the job. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 3,000."]
        #[serde(
            rename = "instruction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instruction: ::std::option::Option<String>,
        #[doc = "Optional but at least one of uris, emails or instruction must be specified. Use this URI field to direct an applicant to a website, for example to link to an online application form. The maximum number of allowed characters for each entry is 2,000."]
        #[serde(
            rename = "uris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uris: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationInfo {
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
    pub struct BatchDeleteJobsRequest {
        #[doc = "Required. The filter string specifies the jobs to be deleted. Supported operator: =, AND The fields eligible for filtering are: * `companyName` (Required) * `requisitionId` (Required) Sample Query: companyName = \"projects/api-test-project/companies/123\" AND requisitionId = \"req-1\""]
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
        #[doc = "Bucket range on which histogram was performed for the numeric field, that is, the count represents number of jobs in this range."]
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
    pub struct ClientEvent {
        #[doc = "Required. The timestamp of the event."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. A unique identifier, generated by the client application. This `event_id` is used to establish the relationship between different events (see parent_event_id)."]
        #[serde(
            rename = "eventId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_id: ::std::option::Option<String>,
        #[doc = "Optional. Extra information about this event. Used for storing information with no matching field in event payload, for example, user application specific context or details. At most 20 keys are supported. The maximum total size of all keys and values is 2 KB."]
        #[serde(
            rename = "extraInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extra_info: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A event issued when a job seeker interacts with the application that implements Cloud Talent Solution."]
        #[serde(
            rename = "jobEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_event: ::std::option::Option<crate::schemas::JobEvent>,
        #[doc = "Optional. The event_id of an event that resulted in the current event. For example, a Job view event usually follows a parent impression event: A job seeker first does a search where a list of jobs appears (impression). The job seeker then selects a result and views the description of a particular job (Job view)."]
        #[serde(
            rename = "parentEventId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_event_id: ::std::option::Option<String>,
        #[doc = "Required. A unique ID generated in the API responses. It can be found in ResponseMetadata.request_id."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ClientEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommuteFilter {
        #[doc = "Optional. If true, jobs without \"precise\" addresses (street level addresses or GPS coordinates) might also be returned. For city and coarser level addresses, text matching is used. If this field is set to false or is not specified, only jobs that include precise addresses are returned by Commute Search. Note: If `allow_imprecise_addresses` is set to true, Commute Search is not able to calculate accurate commute times to jobs with city level and coarser address information. Jobs with imprecise addresses will return a `travel_duration` time of 0 regardless of distance from the job seeker."]
        #[serde(
            rename = "allowImpreciseAddresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_imprecise_addresses: ::std::option::Option<bool>,
        #[doc = "Required. The method of transportation for which to calculate the commute time."]
        #[serde(
            rename = "commuteMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commute_method: ::std::option::Option<crate::schemas::CommuteFilterCommuteMethod>,
        #[doc = "Optional. The departure time used to calculate traffic impact, represented as google.type.TimeOfDay in local time zone. Currently traffic model is restricted to hour level resolution."]
        #[serde(
            rename = "departureTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub departure_time: ::std::option::Option<crate::schemas::TimeOfDay>,
        #[doc = "Optional. Specifies the traffic density to use when calculating commute time."]
        #[serde(
            rename = "roadTraffic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub road_traffic: ::std::option::Option<crate::schemas::CommuteFilterRoadTraffic>,
        #[doc = "Required. The latitude and longitude of the location from which to calculate the commute time."]
        #[serde(
            rename = "startCoordinates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_coordinates: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Required. The maximum travel time in seconds. The maximum allowed value is `3600s` (one hour). Format is `123s`."]
        #[serde(
            rename = "travelDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub travel_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CommuteFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommuteFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommuteFilterCommuteMethod {
        #[doc = "Commute method is not specified."]
        CommuteMethodUnspecified,
        #[doc = "Commute time is calculated based on biking time."]
        Cycling,
        #[doc = "Commute time is calculated based on driving time."]
        Driving,
        #[doc = "Commute time is calculated based on public transit including bus, metro, subway, etc."]
        Transit,
        #[doc = "Commute time is calculated based on walking time."]
        Walking,
    }
    impl CommuteFilterCommuteMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                CommuteFilterCommuteMethod::CommuteMethodUnspecified => {
                    "COMMUTE_METHOD_UNSPECIFIED"
                }
                CommuteFilterCommuteMethod::Cycling => "CYCLING",
                CommuteFilterCommuteMethod::Driving => "DRIVING",
                CommuteFilterCommuteMethod::Transit => "TRANSIT",
                CommuteFilterCommuteMethod::Walking => "WALKING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommuteFilterCommuteMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommuteFilterCommuteMethod {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommuteFilterCommuteMethod, ()> {
            Ok(match s {
                "COMMUTE_METHOD_UNSPECIFIED" => {
                    CommuteFilterCommuteMethod::CommuteMethodUnspecified
                }
                "CYCLING" => CommuteFilterCommuteMethod::Cycling,
                "DRIVING" => CommuteFilterCommuteMethod::Driving,
                "TRANSIT" => CommuteFilterCommuteMethod::Transit,
                "WALKING" => CommuteFilterCommuteMethod::Walking,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommuteFilterCommuteMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommuteFilterCommuteMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommuteFilterCommuteMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMUTE_METHOD_UNSPECIFIED" => {
                    CommuteFilterCommuteMethod::CommuteMethodUnspecified
                }
                "CYCLING" => CommuteFilterCommuteMethod::Cycling,
                "DRIVING" => CommuteFilterCommuteMethod::Driving,
                "TRANSIT" => CommuteFilterCommuteMethod::Transit,
                "WALKING" => CommuteFilterCommuteMethod::Walking,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommuteFilterCommuteMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommuteFilterCommuteMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommuteFilterRoadTraffic {
        #[doc = "Commute time calculation takes in account the peak traffic impact."]
        BusyHour,
        #[doc = "Road traffic situation is not specified."]
        RoadTrafficUnspecified,
        #[doc = "Optimal commute time without considering any traffic impact."]
        TrafficFree,
    }
    impl CommuteFilterRoadTraffic {
        pub fn as_str(self) -> &'static str {
            match self {
                CommuteFilterRoadTraffic::BusyHour => "BUSY_HOUR",
                CommuteFilterRoadTraffic::RoadTrafficUnspecified => "ROAD_TRAFFIC_UNSPECIFIED",
                CommuteFilterRoadTraffic::TrafficFree => "TRAFFIC_FREE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommuteFilterRoadTraffic {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommuteFilterRoadTraffic {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommuteFilterRoadTraffic, ()> {
            Ok(match s {
                "BUSY_HOUR" => CommuteFilterRoadTraffic::BusyHour,
                "ROAD_TRAFFIC_UNSPECIFIED" => CommuteFilterRoadTraffic::RoadTrafficUnspecified,
                "TRAFFIC_FREE" => CommuteFilterRoadTraffic::TrafficFree,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommuteFilterRoadTraffic {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommuteFilterRoadTraffic {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommuteFilterRoadTraffic {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUSY_HOUR" => CommuteFilterRoadTraffic::BusyHour,
                "ROAD_TRAFFIC_UNSPECIFIED" => CommuteFilterRoadTraffic::RoadTrafficUnspecified,
                "TRAFFIC_FREE" => CommuteFilterRoadTraffic::TrafficFree,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommuteFilterRoadTraffic {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommuteFilterRoadTraffic {
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
        pub job_location: ::std::option::Option<crate::schemas::Location>,
        #[doc = "The number of seconds required to travel to the job location from the query location. A duration of 0 seconds indicates that the job is not reachable within the requested duration, but was returned as part of an expanded query."]
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
    pub struct Company {
        #[doc = "Optional. The URI to employer's career site or careers page on the employer's web site, for example, \"https://careers.google.com\"."]
        #[serde(
            rename = "careerSiteUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub career_site_uri: ::std::option::Option<String>,
        #[doc = "Output only. Derived details about the company."]
        #[serde(
            rename = "derivedInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub derived_info: ::std::option::Option<crate::schemas::CompanyDerivedInfo>,
        #[doc = "Required. The display name of the company, for example, \"Google LLC\"."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Equal Employment Opportunity legal disclaimer text to be associated with all jobs, and typically to be displayed in all roles. The maximum number of allowed characters is 500."]
        #[serde(
            rename = "eeoText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eeo_text: ::std::option::Option<String>,
        #[doc = "Required. Client side company identifier, used to uniquely identify the company. The maximum number of allowed characters is 255."]
        #[serde(
            rename = "externalId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_id: ::std::option::Option<String>,
        #[doc = "Optional. The street address of the company's main headquarters, which may be different from the job location. The service attempts to geolocate the provided address, and populates a more specific location wherever possible in DerivedInfo.headquarters_location."]
        #[serde(
            rename = "headquartersAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headquarters_address: ::std::option::Option<String>,
        #[doc = "Optional. Set to true if it is the hiring agency that post jobs for other employers. Defaults to false if not provided."]
        #[serde(
            rename = "hiringAgency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hiring_agency: ::std::option::Option<bool>,
        #[doc = "Optional. A URI that hosts the employer's company logo."]
        #[serde(
            rename = "imageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "Optional. A list of keys of filterable Job.custom_attributes, whose corresponding `string_values` are used in keyword search. Jobs with `string_values` under these specified field keys are returned if any of the values matches the search keyword. Custom field values with parenthesis, brackets and special symbols won't be properly searchable, and those keyword queries need to be surrounded by quotes."]
        #[serde(
            rename = "keywordSearchableJobCustomAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyword_searchable_job_custom_attributes: ::std::option::Option<Vec<String>>,
        #[doc = "Required during company update. The resource name for a company. This is generated by the service when a company is created. The format is \"projects/{project_id}/companies/{company_id}\", for example, \"projects/api-test-project/companies/foo\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The employer's company size."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<crate::schemas::CompanySize>,
        #[doc = "Output only. Indicates whether a company is flagged to be suspended from public availability by the service when job content appears suspicious, abusive, or spammy."]
        #[serde(
            rename = "suspended",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suspended: ::std::option::Option<bool>,
        #[doc = "Optional. The URI representing the company's primary web site or home page, for example, \"https://www.google.com\". The maximum number of allowed characters is 255."]
        #[serde(
            rename = "websiteUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub website_uri: ::std::option::Option<String>,
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
    pub enum CompanySize {
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
    impl CompanySize {
        pub fn as_str(self) -> &'static str {
            match self {
                CompanySize::Big => "BIG",
                CompanySize::Bigger => "BIGGER",
                CompanySize::CompanySizeUnspecified => "COMPANY_SIZE_UNSPECIFIED",
                CompanySize::Giant => "GIANT",
                CompanySize::Medium => "MEDIUM",
                CompanySize::Mini => "MINI",
                CompanySize::Small => "SMALL",
                CompanySize::Smedium => "SMEDIUM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompanySize {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompanySize {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompanySize, ()> {
            Ok(match s {
                "BIG" => CompanySize::Big,
                "BIGGER" => CompanySize::Bigger,
                "COMPANY_SIZE_UNSPECIFIED" => CompanySize::CompanySizeUnspecified,
                "GIANT" => CompanySize::Giant,
                "MEDIUM" => CompanySize::Medium,
                "MINI" => CompanySize::Mini,
                "SMALL" => CompanySize::Small,
                "SMEDIUM" => CompanySize::Smedium,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompanySize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompanySize {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompanySize {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BIG" => CompanySize::Big,
                "BIGGER" => CompanySize::Bigger,
                "COMPANY_SIZE_UNSPECIFIED" => CompanySize::CompanySizeUnspecified,
                "GIANT" => CompanySize::Giant,
                "MEDIUM" => CompanySize::Medium,
                "MINI" => CompanySize::Mini,
                "SMALL" => CompanySize::Small,
                "SMEDIUM" => CompanySize::Smedium,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompanySize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompanySize {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompanyDerivedInfo {
        #[doc = "A structured headquarters location of the company, resolved from Company.hq_location if provided."]
        #[serde(
            rename = "headquartersLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headquarters_location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for CompanyDerivedInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompanyDerivedInfo {
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
        #[doc = "Optional. Compensation description. For example, could indicate equity terms or provide additional context to an estimated bonus."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. Expected number of units paid each year. If not specified, when Job.employment_types is FULLTIME, a default value is inferred based on unit. Default values: - HOURLY: 2080 - DAILY: 260 - WEEKLY: 52 - MONTHLY: 12 - ANNUAL: 1"]
        #[serde(
            rename = "expectedUnitsPerYear",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expected_units_per_year: ::std::option::Option<f64>,
        #[doc = "Optional. Compensation type. Default is CompensationUnit.COMPENSATION_TYPE_UNSPECIFIED."]
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
        #[doc = "Optional. Frequency of the specified amount. Default is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED."]
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
        #[doc = "Base compensation: Refers to the fixed amount of money paid to an employee by an employer in return for work performed. Base compensation does not include benefits, bonuses or any other potential compensation from an employer."]
        Base,
        #[doc = "Bonus."]
        Bonus,
        #[doc = "Commission."]
        Commissions,
        #[doc = "Default value."]
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
        #[doc = "Default value."]
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
        #[doc = "Optional. If set to true, jobs with unspecified compensation range fields are included."]
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
        #[doc = "Required. Specify desired `base compensation entry's` CompensationInfo.CompensationUnit."]
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
        #[doc = "Filter by `base compensation entry's` unit and amount / range. A job is a match if and only if the job contains a base CompensationEntry, and the base entry's unit matches provided compensation_units and amount or range overlaps with provided compensation_range. See CompensationInfo.CompensationEntry for definition of base compensation entry. Set exactly one units and populate range."]
        UnitAndAmount,
        #[doc = "Filter by `base compensation entry's` unit. A job is a match if and only if the job contains a base CompensationEntry and the base CompensationEntry's unit matches provided units. Populate one or more units. See CompensationInfo.CompensationEntry for definition of base compensation entry."]
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
        #[doc = "Default value."]
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
        #[doc = "Required. Type of the request, representing which field the histogramming should be performed over. A single request can only specify one histogram of each `CompensationHistogramRequestType`."]
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
        #[doc = "Histogram by job's annualized base compensation. See CompensationEntry for definition of annualized base compensation."]
        AnnualizedBase,
        #[doc = "Histogram by job's annualized total compensation. See CompensationEntry for definition of annualized total compensation."]
        AnnualizedTotal,
        #[doc = "Histogram by job's base compensation. See CompensationEntry for definition of base compensation."]
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
        #[doc = "Type of the request, corresponding to CompensationHistogramRequest.type."]
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
        #[doc = "Histogram by job's annualized base compensation. See CompensationEntry for definition of annualized base compensation."]
        AnnualizedBase,
        #[doc = "Histogram by job's annualized total compensation. See CompensationEntry for definition of annualized total compensation."]
        AnnualizedTotal,
        #[doc = "Histogram by job's base compensation. See CompensationEntry for definition of base compensation."]
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
        #[doc = "Output only. Annualized base compensation range. Computed as base compensation entry's CompensationEntry.compensation times CompensationEntry.expected_units_per_year. See CompensationEntry for explanation on compensation annualization."]
        #[serde(
            rename = "annualizedBaseCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_base_compensation_range:
            ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Output only. Annualized total compensation range. Computed as all compensation entries' CompensationEntry.compensation times CompensationEntry.expected_units_per_year. See CompensationEntry for explanation on compensation annualization."]
        #[serde(
            rename = "annualizedTotalCompensationRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annualized_total_compensation_range:
            ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Optional. Job compensation information. At most one entry can be of type CompensationInfo.CompensationType.BASE, which is referred as ** base compensation entry ** for the job."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries: ::std::option::Option<Vec<crate::schemas::CompensationEntry>>,
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
        #[doc = "Optional. The maximum amount of compensation. If left empty, the value is set to a maximal compensation value and the currency code is set to match the currency code of min_compensation."]
        #[serde(
            rename = "maxCompensation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_compensation: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Optional. The minimum amount of compensation. If left empty, the value is set to zero and the currency code is set to match the currency code of max_compensation."]
        #[serde(
            rename = "minCompensation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_compensation: ::std::option::Option<crate::schemas::Money>,
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
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
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
        #[doc = "The URI of the company image for CompletionType.COMPANY_NAME."]
        #[serde(
            rename = "imageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_uri: ::std::option::Option<String>,
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
    pub struct CreateClientEventRequest {
        #[doc = "Required. Events issued when end user interacts with customer's application that uses Cloud Talent Solution."]
        #[serde(
            rename = "clientEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_event: ::std::option::Option<crate::schemas::ClientEvent>,
    }
    impl ::google_field_selector::FieldSelector for CreateClientEventRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateClientEventRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateCompanyRequest {
        #[doc = "Required. The company to be created."]
        #[serde(
            rename = "company",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company: ::std::option::Option<crate::schemas::Company>,
    }
    impl ::google_field_selector::FieldSelector for CreateCompanyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateCompanyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateJobRequest {
        #[doc = "Required. The Job to be created."]
        #[serde(
            rename = "job",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job: ::std::option::Option<crate::schemas::Job>,
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
        #[doc = "Optional. If the `filterable` flag is true, the custom field values may be used for custom attribute filters JobQuery.custom_attribute_filter. If false, these values may not be used for custom attribute filters. Default is false."]
        #[serde(
            rename = "filterable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filterable: ::std::option::Option<bool>,
        #[doc = "Optional but exactly one of string_values or long_values must be specified. This field is used to perform number range search. (`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`. Currently at most 1 long_values is supported."]
        #[serde(
            rename = "longValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_values: ::std::option::Option<Vec<i64>>,
        #[doc = "Optional but exactly one of string_values or long_values must be specified. This field is used to perform a string match (`CASE_SENSITIVE_MATCH` or `CASE_INSENSITIVE_MATCH`) search. For filterable `string_value`s, a maximum total number of 200 values is allowed, with each `string_value` has a byte size of no more than 500B. For unfilterable `string_values`, the maximum total byte size of unfilterable `string_values` is 50KB. Empty string is not allowed."]
        #[serde(
            rename = "stringValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_values: ::std::option::Option<Vec<String>>,
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
        #[doc = "Required. Specifies the custom field key to perform a histogram on. If specified without `long_value_histogram_bucketing_option`, histogram on string values of the given `key` is triggered, otherwise histogram is performed on long values."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Optional. Specifies buckets used to perform a range histogram on Job's filterable long custom field values, or min/max value requirements."]
        #[serde(
            rename = "longValueHistogramBucketingOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_value_histogram_bucketing_option:
            ::std::option::Option<crate::schemas::NumericBucketingOption>,
        #[doc = "Optional. If set to true, the response includes the histogram value for each key as a string."]
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
        #[doc = "Stores bucketed histogram counting result or min/max values for custom attribute long values associated with `key`."]
        #[serde(
            rename = "longValueHistogramResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_value_histogram_result:
            ::std::option::Option<crate::schemas::NumericBucketingResult>,
        #[doc = "Stores a map from the values of string custom field associated with `key` to the number of jobs with that value in this histogram result."]
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
    pub struct CustomRankingInfo {
        #[doc = "Required. Controls over how important the score of CustomRankingInfo.ranking_expression gets applied to job's final ranking position. An error is thrown if not specified."]
        #[serde(
            rename = "importanceLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub importance_level:
            ::std::option::Option<crate::schemas::CustomRankingInfoImportanceLevel>,
        #[doc = "Required. Controls over how job documents get ranked on top of existing relevance score (determined by API algorithm). A combination of the ranking expression and relevance score is used to determine job's final ranking position. The syntax for this expression is a subset of Google SQL syntax. Supported operators are: +, -, *, /, where the left and right side of the operator is either a numeric Job.custom_attributes key, integer/double value or an expression that can be evaluated to a number. Parenthesis are supported to adjust calculation precedence. The expression must be < 100 characters in length. The expression is considered invalid for a job if the expression references custom attributes that are not populated on the job or if the expression results in a divide by zero. If an expression is invalid for a job, that job is demoted to the end of the results. Sample ranking expression (year + 25) * 0.25 - (freshness / 0.5)"]
        #[serde(
            rename = "rankingExpression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ranking_expression: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomRankingInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomRankingInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CustomRankingInfoImportanceLevel {
        #[doc = "The given ranking expression is of Extreme importance, and dominates job's final ranking position with existing relevance score (determined by API algorithm) ignored."]
        Extreme,
        #[doc = "The given ranking expression is of High importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm)."]
        High,
        #[doc = "Default value if the importance level is not specified."]
        ImportanceLevelUnspecified,
        #[doc = "The given ranking expression is of Low importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm)."]
        Low,
        #[doc = "The given ranking expression is of Medium importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm)."]
        Medium,
        #[doc = "The given ranking expression is of Mild importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm)."]
        Mild,
        #[doc = "The given ranking expression is of None importance, existing relevance score (determined by API algorithm) dominates job's final ranking position."]
        None,
    }
    impl CustomRankingInfoImportanceLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                CustomRankingInfoImportanceLevel::Extreme => "EXTREME",
                CustomRankingInfoImportanceLevel::High => "HIGH",
                CustomRankingInfoImportanceLevel::ImportanceLevelUnspecified => {
                    "IMPORTANCE_LEVEL_UNSPECIFIED"
                }
                CustomRankingInfoImportanceLevel::Low => "LOW",
                CustomRankingInfoImportanceLevel::Medium => "MEDIUM",
                CustomRankingInfoImportanceLevel::Mild => "MILD",
                CustomRankingInfoImportanceLevel::None => "NONE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CustomRankingInfoImportanceLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CustomRankingInfoImportanceLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CustomRankingInfoImportanceLevel, ()> {
            Ok(match s {
                "EXTREME" => CustomRankingInfoImportanceLevel::Extreme,
                "HIGH" => CustomRankingInfoImportanceLevel::High,
                "IMPORTANCE_LEVEL_UNSPECIFIED" => {
                    CustomRankingInfoImportanceLevel::ImportanceLevelUnspecified
                }
                "LOW" => CustomRankingInfoImportanceLevel::Low,
                "MEDIUM" => CustomRankingInfoImportanceLevel::Medium,
                "MILD" => CustomRankingInfoImportanceLevel::Mild,
                "NONE" => CustomRankingInfoImportanceLevel::None,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CustomRankingInfoImportanceLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CustomRankingInfoImportanceLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CustomRankingInfoImportanceLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTREME" => CustomRankingInfoImportanceLevel::Extreme,
                "HIGH" => CustomRankingInfoImportanceLevel::High,
                "IMPORTANCE_LEVEL_UNSPECIFIED" => {
                    CustomRankingInfoImportanceLevel::ImportanceLevelUnspecified
                }
                "LOW" => CustomRankingInfoImportanceLevel::Low,
                "MEDIUM" => CustomRankingInfoImportanceLevel::Medium,
                "MILD" => CustomRankingInfoImportanceLevel::Mild,
                "NONE" => CustomRankingInfoImportanceLevel::None,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CustomRankingInfoImportanceLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomRankingInfoImportanceLevel {
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
        #[doc = "Optional. A device-specific ID. The ID must be a unique identifier that distinguishes the device from other devices."]
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
        #[doc = "A bot, as opposed to a device operated by human beings, such as a web crawler."]
        Bot,
        #[doc = "The device type isn't specified."]
        DeviceTypeUnspecified,
        #[doc = "An iOS device native application."]
        Ios,
        #[doc = "A mobile device web browser, such as a phone or tablet with a Chrome browser."]
        MobileWeb,
        #[doc = "Other devices types."]
        Other,
        #[doc = "A desktop web browser, such as, Chrome, Firefox, Safari, or Internet Explorer)"]
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramFacets {
        #[doc = "Optional. Specifies compensation field-based histogram requests. Duplicate values of CompensationHistogramRequest.type are not allowed."]
        #[serde(
            rename = "compensationHistogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::CompensationHistogramRequest>>,
        #[doc = "Optional. Specifies the custom attributes histogram requests. Duplicate values of CustomAttributeHistogramRequest.key are not allowed."]
        #[serde(
            rename = "customAttributeHistogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::CustomAttributeHistogramRequest>>,
        #[doc = "Optional. Specifies the simple type of histogram facets, for example, `COMPANY_SIZE`, `EMPLOYMENT_TYPE` etc."]
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
        #[doc = "Filter by Admin1, which is a global placeholder for referring to state, province, or the particular term a country uses to define the geographic structure below the country level. Examples include states codes such as \"CA\", \"IL\", \"NY\", and provinces, such as \"BC\"."]
        Admin1,
        #[doc = "A combination of state or province code with a country code. This field differs from `JOB_ADMIN1`, which can be used in multiple countries."]
        Admin1Country,
        #[doc = "Base compensation unit."]
        BaseCompensationUnit,
        #[doc = "Filter by the Category."]
        Category,
        #[doc = "Filter by the \"city name\", \"Admin1 code\", for example, \"Mountain View, CA\" or \"New York, NY\"."]
        City,
        #[doc = "Filter by the city center GPS coordinate (latitude and longitude), for example, 37.4038522,-122.0987765. Since the coordinates of a city center can change, clients may need to refresh them periodically."]
        CityCoordinate,
        #[doc = "Company display name."]
        CompanyDisplayName,
        #[doc = "Filter by the company id field."]
        CompanyId,
        #[doc = "Filter by the company size type field, such as `BIG`, `SMALL` or `BIGGER`."]
        CompanySize,
        #[doc = "Filter by the country code of job, such as US, JP, FR."]
        Country,
        #[doc = "Filter by the date published field. Possible return values are: * PAST_24_HOURS (The past 24 hours) * PAST_3_DAYS (The past 3 days) * PAST_WEEK (The past 7 days) * PAST_MONTH (The past 30 days) * PAST_YEAR (The past 365 days)"]
        DatePublished,
        #[doc = "Filter by the required education level of the job."]
        EducationLevel,
        #[doc = "Filter by the employment type field, such as `FULL_TIME` or `PART_TIME`."]
        EmploymentType,
        #[doc = "Filter by the required experience level of the job."]
        ExperienceLevel,
        #[doc = "Filter by the language code portion of the locale field, such as \"en\" or \"fr\"."]
        Language,
        #[doc = "Filter by the locale field of a job, such as \"en-US\", \"fr-FR\". This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        Locale,
        #[doc = "The default value if search type is not specified."]
        SearchTypeUnspecified,
    }
    impl HistogramFacetsSimpleHistogramFacetsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                HistogramFacetsSimpleHistogramFacetsItems::Admin1 => "ADMIN_1",
                HistogramFacetsSimpleHistogramFacetsItems::Admin1Country => "ADMIN_1_COUNTRY",
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
                HistogramFacetsSimpleHistogramFacetsItems::Country => "COUNTRY",
                HistogramFacetsSimpleHistogramFacetsItems::DatePublished => "DATE_PUBLISHED",
                HistogramFacetsSimpleHistogramFacetsItems::EducationLevel => "EDUCATION_LEVEL",
                HistogramFacetsSimpleHistogramFacetsItems::EmploymentType => "EMPLOYMENT_TYPE",
                HistogramFacetsSimpleHistogramFacetsItems::ExperienceLevel => "EXPERIENCE_LEVEL",
                HistogramFacetsSimpleHistogramFacetsItems::Language => "LANGUAGE",
                HistogramFacetsSimpleHistogramFacetsItems::Locale => "LOCALE",
                HistogramFacetsSimpleHistogramFacetsItems::SearchTypeUnspecified => {
                    "SEARCH_TYPE_UNSPECIFIED"
                }
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
                "ADMIN_1" => HistogramFacetsSimpleHistogramFacetsItems::Admin1,
                "ADMIN_1_COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Admin1Country,
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
                "COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Country,
                "DATE_PUBLISHED" => HistogramFacetsSimpleHistogramFacetsItems::DatePublished,
                "EDUCATION_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramFacetsSimpleHistogramFacetsItems::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::ExperienceLevel,
                "LANGUAGE" => HistogramFacetsSimpleHistogramFacetsItems::Language,
                "LOCALE" => HistogramFacetsSimpleHistogramFacetsItems::Locale,
                "SEARCH_TYPE_UNSPECIFIED" => {
                    HistogramFacetsSimpleHistogramFacetsItems::SearchTypeUnspecified
                }
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
                "ADMIN_1" => HistogramFacetsSimpleHistogramFacetsItems::Admin1,
                "ADMIN_1_COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Admin1Country,
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
                "COUNTRY" => HistogramFacetsSimpleHistogramFacetsItems::Country,
                "DATE_PUBLISHED" => HistogramFacetsSimpleHistogramFacetsItems::DatePublished,
                "EDUCATION_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramFacetsSimpleHistogramFacetsItems::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramFacetsSimpleHistogramFacetsItems::ExperienceLevel,
                "LANGUAGE" => HistogramFacetsSimpleHistogramFacetsItems::Language,
                "LOCALE" => HistogramFacetsSimpleHistogramFacetsItems::Locale,
                "SEARCH_TYPE_UNSPECIFIED" => {
                    HistogramFacetsSimpleHistogramFacetsItems::SearchTypeUnspecified
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
    pub struct HistogramQuery {
        #[doc = "An expression specifies a histogram request against matching resources (for example, jobs) for searches. Expression syntax is a aggregation function call with histogram facets and other options. Available aggregation function calls are: * `count(string_histogram_facet)`: Count the number of matching entity, for each distinct attribute value. * `count(numeric_histogram_facet, list of buckets)`: Count the number of matching entity within each bucket. Data types: * Histogram facet: facet names with format a-zA-Z+. * String: string like \"any string with backslash escape for quote(\").\" * Number: whole number and floating point number like 10, -1 and -0.01. * List: list of elements with comma(,) separator surrounded by square brackets. For example, [1, 2, 3] and [\"one\", \"two\", \"three\"]. Built-in constants: * MIN (minimum number similar to java Double.MIN_VALUE) * MAX (maximum number similar to java Double.MAX_VALUE) Built-in functions: * bucket(start, end[, label]): bucket built-in function creates a bucket with range of start, end). Note that the end is exclusive. For example, bucket(1, MAX, \"positive number\") or bucket(1, 10). Job histogram facets: * company_id: histogram by [Job.distributor_company_id. * company_display_name: histogram by Job.company_display_name. * employment_type: histogram by Job.employment_types. For example, \"FULL_TIME\", \"PART_TIME\". * company_size: histogram by CompanySize, for example, \"SMALL\", \"MEDIUM\", \"BIG\". * publish_time_in_month: histogram by the Job.publish_time in months. Must specify list of numeric buckets in spec. * publish_time_in_year: histogram by the Job.publish_time in years. Must specify list of numeric buckets in spec. * degree_type: histogram by the Job.degree_type. For example, \"Bachelors\", \"Masters\". * job_level: histogram by the Job.job_level. For example, \"Entry Level\". * country: histogram by the country code of jobs. For example, \"US\", \"FR\". * admin1: histogram by the admin1 code of jobs, which is a global placeholder referring to the state, province, or the particular term a country uses to define the geographic structure below the country level. For example, \"CA\", \"IL\". * city: histogram by a combination of the \"city name, admin1 code\". For example, \"Mountain View, CA\", \"New York, NY\". * admin1_country: histogram by a combination of the \"admin1 code, country\". For example, \"CA, US\", \"IL, US\". * city_coordinate: histogram by the city center's GPS coordinates (latitude and longitude). For example, 37.4038522,-122.0987765. Since the coordinates of a city center can change, customers may need to refresh them periodically. * locale: histogram by the Job.language_code. For example, \"en-US\", \"fr-FR\". * language: histogram by the language subtag of the Job.language_code. For example, \"en\", \"fr\". * category: histogram by the JobCategory. For example, \"COMPUTER_AND_IT\", \"HEALTHCARE\". * base_compensation_unit: histogram by the CompensationUnit of base salary. For example, \"WEEKLY\", \"MONTHLY\". * base_compensation: histogram by the base salary. Must specify list of numeric buckets to group results by. * annualized_base_compensation: histogram by the base annualized salary. Must specify list of numeric buckets to group results by. * annualized_total_compensation: histogram by the total annualized salary. Must specify list of numeric buckets to group results by. * string_custom_attribute: histogram by string Job.custom_attributes. Values can be accessed via square bracket notations like string_custom_attribute[\"key1\"]. * numeric_custom_attribute: histogram by numeric Job.custom_attributes. Values can be accessed via square bracket notations like numeric_custom_attribute[\"key1\"]. Must specify list of numeric buckets to group results by. Example expressions: * count(admin1) * count(base_compensation, [bucket(1000, 10000), bucket(10000, 100000), bucket(100000, MAX)]) * count(string_custom_attribute[\"some-string-custom-attribute\"]) * count(numeric_custom_attribute[\"some-numeric-custom-attribute\"], [bucket(MIN, 0, \"negative\"), bucket(0, MAX, \"non-negative\"])"]
        #[serde(
            rename = "histogramQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HistogramQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramQuery {
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
    pub struct HistogramQueryResult {
        #[doc = "A map from the values of the facet associated with distinct values to the number of matching entries with corresponding value. The key format is: * (for string histogram) string values stored in the field. * (for named numeric bucket) name specified in `bucket()` function, like for `bucket(0, MAX, \"non-negative\")`, the key will be `non-negative`. * (for anonymous numeric bucket) range formatted as `-`, for example, `0-1000`, `MIN-0`, and `0-MAX`."]
        #[serde(
            rename = "histogram",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "Requested histogram expression."]
        #[serde(
            rename = "histogramQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HistogramQueryResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramQueryResult {
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
        #[doc = "A map from the values of field to the number of jobs with that value in this search result. Key: search type (filter names, such as the companyName). Values: the count of jobs that match the filter for this search."]
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
        #[doc = "Filter by Admin1, which is a global placeholder for referring to state, province, or the particular term a country uses to define the geographic structure below the country level. Examples include states codes such as \"CA\", \"IL\", \"NY\", and provinces, such as \"BC\"."]
        Admin1,
        #[doc = "A combination of state or province code with a country code. This field differs from `JOB_ADMIN1`, which can be used in multiple countries."]
        Admin1Country,
        #[doc = "Base compensation unit."]
        BaseCompensationUnit,
        #[doc = "Filter by the Category."]
        Category,
        #[doc = "Filter by the \"city name\", \"Admin1 code\", for example, \"Mountain View, CA\" or \"New York, NY\"."]
        City,
        #[doc = "Filter by the city center GPS coordinate (latitude and longitude), for example, 37.4038522,-122.0987765. Since the coordinates of a city center can change, clients may need to refresh them periodically."]
        CityCoordinate,
        #[doc = "Company display name."]
        CompanyDisplayName,
        #[doc = "Filter by the company id field."]
        CompanyId,
        #[doc = "Filter by the company size type field, such as `BIG`, `SMALL` or `BIGGER`."]
        CompanySize,
        #[doc = "Filter by the country code of job, such as US, JP, FR."]
        Country,
        #[doc = "Filter by the date published field. Possible return values are: * PAST_24_HOURS (The past 24 hours) * PAST_3_DAYS (The past 3 days) * PAST_WEEK (The past 7 days) * PAST_MONTH (The past 30 days) * PAST_YEAR (The past 365 days)"]
        DatePublished,
        #[doc = "Filter by the required education level of the job."]
        EducationLevel,
        #[doc = "Filter by the employment type field, such as `FULL_TIME` or `PART_TIME`."]
        EmploymentType,
        #[doc = "Filter by the required experience level of the job."]
        ExperienceLevel,
        #[doc = "Filter by the language code portion of the locale field, such as \"en\" or \"fr\"."]
        Language,
        #[doc = "Filter by the locale field of a job, such as \"en-US\", \"fr-FR\". This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        Locale,
        #[doc = "The default value if search type is not specified."]
        SearchTypeUnspecified,
    }
    impl HistogramResultSearchType {
        pub fn as_str(self) -> &'static str {
            match self {
                HistogramResultSearchType::Admin1 => "ADMIN_1",
                HistogramResultSearchType::Admin1Country => "ADMIN_1_COUNTRY",
                HistogramResultSearchType::BaseCompensationUnit => "BASE_COMPENSATION_UNIT",
                HistogramResultSearchType::Category => "CATEGORY",
                HistogramResultSearchType::City => "CITY",
                HistogramResultSearchType::CityCoordinate => "CITY_COORDINATE",
                HistogramResultSearchType::CompanyDisplayName => "COMPANY_DISPLAY_NAME",
                HistogramResultSearchType::CompanyId => "COMPANY_ID",
                HistogramResultSearchType::CompanySize => "COMPANY_SIZE",
                HistogramResultSearchType::Country => "COUNTRY",
                HistogramResultSearchType::DatePublished => "DATE_PUBLISHED",
                HistogramResultSearchType::EducationLevel => "EDUCATION_LEVEL",
                HistogramResultSearchType::EmploymentType => "EMPLOYMENT_TYPE",
                HistogramResultSearchType::ExperienceLevel => "EXPERIENCE_LEVEL",
                HistogramResultSearchType::Language => "LANGUAGE",
                HistogramResultSearchType::Locale => "LOCALE",
                HistogramResultSearchType::SearchTypeUnspecified => "SEARCH_TYPE_UNSPECIFIED",
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
                "ADMIN_1" => HistogramResultSearchType::Admin1,
                "ADMIN_1_COUNTRY" => HistogramResultSearchType::Admin1Country,
                "BASE_COMPENSATION_UNIT" => HistogramResultSearchType::BaseCompensationUnit,
                "CATEGORY" => HistogramResultSearchType::Category,
                "CITY" => HistogramResultSearchType::City,
                "CITY_COORDINATE" => HistogramResultSearchType::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => HistogramResultSearchType::CompanyDisplayName,
                "COMPANY_ID" => HistogramResultSearchType::CompanyId,
                "COMPANY_SIZE" => HistogramResultSearchType::CompanySize,
                "COUNTRY" => HistogramResultSearchType::Country,
                "DATE_PUBLISHED" => HistogramResultSearchType::DatePublished,
                "EDUCATION_LEVEL" => HistogramResultSearchType::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramResultSearchType::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramResultSearchType::ExperienceLevel,
                "LANGUAGE" => HistogramResultSearchType::Language,
                "LOCALE" => HistogramResultSearchType::Locale,
                "SEARCH_TYPE_UNSPECIFIED" => HistogramResultSearchType::SearchTypeUnspecified,
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
                "ADMIN_1" => HistogramResultSearchType::Admin1,
                "ADMIN_1_COUNTRY" => HistogramResultSearchType::Admin1Country,
                "BASE_COMPENSATION_UNIT" => HistogramResultSearchType::BaseCompensationUnit,
                "CATEGORY" => HistogramResultSearchType::Category,
                "CITY" => HistogramResultSearchType::City,
                "CITY_COORDINATE" => HistogramResultSearchType::CityCoordinate,
                "COMPANY_DISPLAY_NAME" => HistogramResultSearchType::CompanyDisplayName,
                "COMPANY_ID" => HistogramResultSearchType::CompanyId,
                "COMPANY_SIZE" => HistogramResultSearchType::CompanySize,
                "COUNTRY" => HistogramResultSearchType::Country,
                "DATE_PUBLISHED" => HistogramResultSearchType::DatePublished,
                "EDUCATION_LEVEL" => HistogramResultSearchType::EducationLevel,
                "EMPLOYMENT_TYPE" => HistogramResultSearchType::EmploymentType,
                "EXPERIENCE_LEVEL" => HistogramResultSearchType::ExperienceLevel,
                "LANGUAGE" => HistogramResultSearchType::Language,
                "LOCALE" => HistogramResultSearchType::Locale,
                "SEARCH_TYPE_UNSPECIFIED" => HistogramResultSearchType::SearchTypeUnspecified,
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
        #[doc = "Specifies compensation field-based histogram results that match HistogramFacets.compensation_histogram_requests."]
        #[serde(
            rename = "compensationHistogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_histogram_results:
            ::std::option::Option<Vec<crate::schemas::CompensationHistogramResult>>,
        #[doc = "Specifies histogram results for custom attributes that match HistogramFacets.custom_attribute_histogram_facets."]
        #[serde(
            rename = "customAttributeHistogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_histogram_results:
            ::std::option::Option<Vec<crate::schemas::CustomAttributeHistogramResult>>,
        #[doc = "Specifies histogram results that matches HistogramFacets.simple_histogram_facets."]
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
        #[doc = "Optional but strongly recommended for the best service experience. Location(s) where the employer is looking to hire for this job posting. Specifying the full street address(es) of the hiring location enables better API results, especially job searches by commute time. At most 50 locations are allowed for best search performance. If a job has more locations, it is suggested to split it into multiple jobs with unique requisition_ids (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', etc.) as multiple jobs with the same company_name, language_code and requisition_id are not allowed. If the original requisition_id must be preserved, a custom field should be used for storage. It is also suggested to group the locations that close to each other in the same job for better search experience. The maximum number of allowed characters is 500."]
        #[serde(
            rename = "addresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub addresses: ::std::option::Option<Vec<String>>,
        #[doc = "Required. At least one field within ApplicationInfo must be specified. Job application information."]
        #[serde(
            rename = "applicationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_info: ::std::option::Option<crate::schemas::ApplicationInfo>,
        #[doc = "Output only. Display name of the company listing the job."]
        #[serde(
            rename = "companyDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_display_name: ::std::option::Option<String>,
        #[doc = "Required. The resource name of the company listing the job, such as \"projects/api-test-project/companies/foo\"."]
        #[serde(
            rename = "companyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_name: ::std::option::Option<String>,
        #[doc = "Optional. Job compensation information."]
        #[serde(
            rename = "compensationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_info: ::std::option::Option<crate::schemas::CompensationInfo>,
        #[doc = "Optional. A map of fields to hold both filterable and non-filterable custom job attributes that are not covered by the provided structured fields. The keys of the map are strings up to 64 bytes and must match the pattern: a-zA-Z*. For example, key0LikeThis or KEY_1_LIKE_THIS. At most 100 filterable and at most 100 unfilterable keys are supported. For filterable `string_values`, across all keys at most 200 values are allowed, with each string no more than 255 characters. For unfilterable `string_values`, the maximum total size of `string_values` across all keys is 50KB."]
        #[serde(
            rename = "customAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attributes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CustomAttribute>,
        >,
        #[doc = "Optional. The desired education degrees for the job, such as Bachelors, Masters."]
        #[serde(
            rename = "degreeTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub degree_types: ::std::option::Option<Vec<crate::schemas::JobDegreeTypesItems>>,
        #[doc = "Optional. The department or functional area within the company with the open position. The maximum number of allowed characters is 255."]
        #[serde(
            rename = "department",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub department: ::std::option::Option<String>,
        #[doc = "Output only. Derived details about the job posting."]
        #[serde(
            rename = "derivedInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub derived_info: ::std::option::Option<crate::schemas::JobDerivedInfo>,
        #[doc = "Required. The description of the job, which typically includes a multi-paragraph description of the company and related information. Separate fields are provided on the job object for responsibilities, qualifications, and other job characteristics. Use of these separate job fields is recommended. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 100,000."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. The employment type(s) of a job, for example, full time or part time."]
        #[serde(
            rename = "employmentTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub employment_types: ::std::option::Option<Vec<crate::schemas::JobEmploymentTypesItems>>,
        #[doc = "Optional. A description of bonus, commission, and other compensation incentives associated with the job not including salary or pay. The maximum number of allowed characters is 10,000."]
        #[serde(
            rename = "incentives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub incentives: ::std::option::Option<String>,
        #[doc = "Optional. The benefits included with the job."]
        #[serde(
            rename = "jobBenefits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_benefits: ::std::option::Option<Vec<crate::schemas::JobJobBenefitsItems>>,
        #[doc = "Optional. The end timestamp of the job. Typically this field is used for contracting engagements. Invalid timestamps are ignored."]
        #[serde(
            rename = "jobEndTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_end_time: ::std::option::Option<String>,
        #[doc = "Optional. The experience level associated with the job, such as \"Entry Level\"."]
        #[serde(
            rename = "jobLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_level: ::std::option::Option<crate::schemas::JobJobLevel>,
        #[doc = "Optional. The start timestamp of the job in UTC time zone. Typically this field is used for contracting engagements. Invalid timestamps are ignored."]
        #[serde(
            rename = "jobStartTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_start_time: ::std::option::Option<String>,
        #[doc = "Optional. The language of the posting. This field is distinct from any requirements for fluency that are associated with the job. Language codes must be in BCP-47 format, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){: class=\"external\" target=\"_blank\" }. If this field is unspecified and Job.description is present, detected language code based on Job.description is assigned, otherwise defaults to 'en_US'."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Required during job update. The resource name for the job. This is generated by the service when a job is created. The format is \"projects/{project_id}/jobs/{job_id}\", for example, \"projects/api-test-project/jobs/1234\". Use of this field in job queries and API calls is preferred over the use of requisition_id since this value is unique."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp when this job posting was created."]
        #[serde(
            rename = "postingCreateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub posting_create_time: ::std::option::Option<String>,
        #[doc = "Optional but strongly recommended for the best service experience. The expiration timestamp of the job. After this timestamp, the job is marked as expired, and it no longer appears in search results. The expired job can't be deleted or listed by the DeleteJob and ListJobs APIs, but it can be retrieved with the GetJob API or updated with the UpdateJob API. An expired job can be updated and opened again by using a future expiration timestamp. Updating an expired job fails if there is another existing open job with same company_name, language_code and requisition_id. The expired jobs are retained in our system for 90 days. However, the overall expired job count cannot exceed 3 times the maximum of open jobs count over the past week, otherwise jobs with earlier expire time are cleaned first. Expired jobs are no longer accessible after they are cleaned out. Invalid timestamps are ignored, and treated as expire time not provided. Timestamp before the instant request is made is considered valid, the job will be treated as expired immediately. If this value is not provided at the time of job creation or is invalid, the job posting expires after 30 days from the job's creation time. For example, if the job was created on 2017/01/01 13:00AM UTC with an unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC. If this value is not provided on job update, it depends on the field masks set by UpdateJobRequest.update_mask. If the field masks include expiry_time, or the masks are empty meaning that every field is updated, the job posting expires after 30 days from the job's last update time. Otherwise the expiration date isn't updated."]
        #[serde(
            rename = "postingExpireTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub posting_expire_time: ::std::option::Option<String>,
        #[doc = "Optional. The timestamp this job posting was most recently published. The default value is the time the request arrives at the server. Invalid timestamps are ignored."]
        #[serde(
            rename = "postingPublishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub posting_publish_time: ::std::option::Option<String>,
        #[doc = "Optional. The job PostingRegion (for example, state, country) throughout which the job is available. If this field is set, a LocationFilter in a search query within the job region finds this job posting if an exact location match isn't specified. If this field is set to PostingRegion.NATION or PostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses to the same location level as this field is strongly recommended."]
        #[serde(
            rename = "postingRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub posting_region: ::std::option::Option<crate::schemas::JobPostingRegion>,
        #[doc = "Output only. The timestamp when this job posting was last updated."]
        #[serde(
            rename = "postingUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub posting_update_time: ::std::option::Option<String>,
        #[doc = "Optional. Options for job processing."]
        #[serde(
            rename = "processingOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processing_options: ::std::option::Option<crate::schemas::ProcessingOptions>,
        #[doc = "Optional. A promotion value of the job, as determined by the client. The value determines the sort order of the jobs returned when searching for jobs using the featured jobs search call, with higher promotional values being returned first and ties being resolved by relevance sort. Only the jobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH. Default value is 0, and negative values are treated as 0."]
        #[serde(
            rename = "promotionValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotion_value: ::std::option::Option<i32>,
        #[doc = "Optional. A description of the qualifications required to perform the job. The use of this field is recommended as an alternative to using the more general description field. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 10,000."]
        #[serde(
            rename = "qualifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qualifications: ::std::option::Option<String>,
        #[doc = "Required. The requisition ID, also referred to as the posting ID, assigned by the client to identify a job. This field is intended to be used by clients for client identification and tracking of postings. A job is not allowed to be created if there is another job with the same [company_name], language_code and requisition_id. The maximum number of allowed characters is 255."]
        #[serde(
            rename = "requisitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requisition_id: ::std::option::Option<String>,
        #[doc = "Optional. A description of job responsibilities. The use of this field is recommended as an alternative to using the more general description field. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 10,000."]
        #[serde(
            rename = "responsibilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responsibilities: ::std::option::Option<String>,
        #[doc = "Required. The title of the job, such as \"Software Engineer\" The maximum number of allowed characters is 500."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Deprecated. The job is only visible to the owner. The visibility of the job. Defaults to Visibility.ACCOUNT_ONLY if not specified."]
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
    pub enum JobDegreeTypesItems {
        #[doc = "Adult Remedial Education; Programmes providing learning experiences that build on secondary education and prepare for labour market entry and/or tertiary education. The content is broader than secondary but not as complex as tertiary education. ISCED code 4."]
        AdultRemedialEducation,
        #[doc = "Associate's or equivalent; Short first tertiary programmes that are typically practically-based, occupationally-specific and prepare for labour market entry. These programmes may also provide a pathway to other tertiary programmes. ISCED code 5."]
        AssociatesOrEquivalent,
        #[doc = "Bachelor's or equivalent; Programmes designed to provide intermediate academic and/or professional knowledge, skills and competencies leading to a first tertiary degree or equivalent qualification. ISCED code 6."]
        BachelorsOrEquivalent,
        #[doc = "Default value. Represents no degree, or early childhood education. Maps to ISCED code 0. Ex) Kindergarten"]
        DegreeTypeUnspecified,
        #[doc = "Doctoral or equivalent; Programmes designed primarily to lead to an advanced research qualification, usually concluding with the submission and defense of a substantive dissertation of publishable quality based on original research. ISCED code 8."]
        DoctoralOrEquivalent,
        #[doc = "Lower secondary education; First stage of secondary education building on primary education, typically with a more subject-oriented curriculum. ISCED code 2. Ex) Middle school"]
        LowerSecondaryEducation,
        #[doc = "Master's or equivalent; Programmes designed to provide advanced academic and/or professional knowledge, skills and competencies leading to a second tertiary degree or equivalent qualification. ISCED code 7."]
        MastersOrEquivalent,
        #[doc = "Primary education which is typically the first stage of compulsory education. ISCED code 1. Ex) Elementary school"]
        PrimaryEducation,
        #[doc = "Middle education; Second/final stage of secondary education preparing for tertiary education and/or providing skills relevant to employment. Usually with an increased range of subject options and streams. ISCED code 3. Ex) High school"]
        UpperSecondaryEducation,
    }
    impl JobDegreeTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobDegreeTypesItems::AdultRemedialEducation => "ADULT_REMEDIAL_EDUCATION",
                JobDegreeTypesItems::AssociatesOrEquivalent => "ASSOCIATES_OR_EQUIVALENT",
                JobDegreeTypesItems::BachelorsOrEquivalent => "BACHELORS_OR_EQUIVALENT",
                JobDegreeTypesItems::DegreeTypeUnspecified => "DEGREE_TYPE_UNSPECIFIED",
                JobDegreeTypesItems::DoctoralOrEquivalent => "DOCTORAL_OR_EQUIVALENT",
                JobDegreeTypesItems::LowerSecondaryEducation => "LOWER_SECONDARY_EDUCATION",
                JobDegreeTypesItems::MastersOrEquivalent => "MASTERS_OR_EQUIVALENT",
                JobDegreeTypesItems::PrimaryEducation => "PRIMARY_EDUCATION",
                JobDegreeTypesItems::UpperSecondaryEducation => "UPPER_SECONDARY_EDUCATION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobDegreeTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobDegreeTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobDegreeTypesItems, ()> {
            Ok(match s {
                "ADULT_REMEDIAL_EDUCATION" => JobDegreeTypesItems::AdultRemedialEducation,
                "ASSOCIATES_OR_EQUIVALENT" => JobDegreeTypesItems::AssociatesOrEquivalent,
                "BACHELORS_OR_EQUIVALENT" => JobDegreeTypesItems::BachelorsOrEquivalent,
                "DEGREE_TYPE_UNSPECIFIED" => JobDegreeTypesItems::DegreeTypeUnspecified,
                "DOCTORAL_OR_EQUIVALENT" => JobDegreeTypesItems::DoctoralOrEquivalent,
                "LOWER_SECONDARY_EDUCATION" => JobDegreeTypesItems::LowerSecondaryEducation,
                "MASTERS_OR_EQUIVALENT" => JobDegreeTypesItems::MastersOrEquivalent,
                "PRIMARY_EDUCATION" => JobDegreeTypesItems::PrimaryEducation,
                "UPPER_SECONDARY_EDUCATION" => JobDegreeTypesItems::UpperSecondaryEducation,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobDegreeTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobDegreeTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobDegreeTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADULT_REMEDIAL_EDUCATION" => JobDegreeTypesItems::AdultRemedialEducation,
                "ASSOCIATES_OR_EQUIVALENT" => JobDegreeTypesItems::AssociatesOrEquivalent,
                "BACHELORS_OR_EQUIVALENT" => JobDegreeTypesItems::BachelorsOrEquivalent,
                "DEGREE_TYPE_UNSPECIFIED" => JobDegreeTypesItems::DegreeTypeUnspecified,
                "DOCTORAL_OR_EQUIVALENT" => JobDegreeTypesItems::DoctoralOrEquivalent,
                "LOWER_SECONDARY_EDUCATION" => JobDegreeTypesItems::LowerSecondaryEducation,
                "MASTERS_OR_EQUIVALENT" => JobDegreeTypesItems::MastersOrEquivalent,
                "PRIMARY_EDUCATION" => JobDegreeTypesItems::PrimaryEducation,
                "UPPER_SECONDARY_EDUCATION" => JobDegreeTypesItems::UpperSecondaryEducation,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobDegreeTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobDegreeTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobEmploymentTypesItems {
        #[doc = "The job is offered as a contracted position with the understanding that it's converted into a full-time position at the end of the contract. Jobs of this type are also returned by a search for EmploymentType.CONTRACTOR jobs."]
        ContractToHire,
        #[doc = "The job is offered as a contracted, as opposed to a salaried employee, position."]
        Contractor,
        #[doc = "The default value if the employment type is not specified."]
        EmploymentTypeUnspecified,
        #[doc = "The job involves employing people in remote areas and flying them temporarily to the work site instead of relocating employees and their families permanently."]
        FlyInFlyOut,
        #[doc = "The job requires working a number of hours that constitute full time employment, typically 40 or more hours per week."]
        FullTime,
        #[doc = "The job is a fixed-term opportunity for students or entry-level job seekers to obtain on-the-job training, typically offered as a summer position."]
        Intern,
        #[doc = "The job does not fit any of the other listed types."]
        OtherEmploymentType,
        #[doc = "The job entails working fewer hours than a full time job, typically less than 40 hours a week."]
        PartTime,
        #[doc = "The job requires an employee to work on an as-needed basis with a flexible schedule."]
        PerDiem,
        #[doc = "The job is offered as a temporary employment opportunity, usually a short-term engagement."]
        Temporary,
        #[doc = "The is an opportunity for an individual to volunteer, where there's no expectation of compensation for the provided services."]
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
                JobEmploymentTypesItems::OtherEmploymentType => "OTHER_EMPLOYMENT_TYPE",
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
                "OTHER_EMPLOYMENT_TYPE" => JobEmploymentTypesItems::OtherEmploymentType,
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
                "OTHER_EMPLOYMENT_TYPE" => JobEmploymentTypesItems::OtherEmploymentType,
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
    pub enum JobJobBenefitsItems {
        #[doc = "The job includes access to programs that support child care, such as daycare."]
        ChildCare,
        #[doc = "The job includes dental services covered by a dental insurance plan."]
        Dental,
        #[doc = "The job offers specific benefits to domestic partners."]
        DomesticPartner,
        #[doc = "The job allows for a flexible work schedule."]
        FlexibleHours,
        #[doc = "Default value if the type is not specified."]
        JobBenefitUnspecified,
        #[doc = "The job includes a life insurance plan provided by the employer or available for purchase by the employee."]
        LifeInsurance,
        #[doc = "The job includes health services covered by a medical insurance plan."]
        Medical,
        #[doc = "The job allows for a leave of absence to a parent to care for a newborn child."]
        ParentalLeave,
        #[doc = "The job includes a workplace retirement plan provided by the employer or available for purchase by the employee."]
        RetirementPlan,
        #[doc = "The job allows for paid time off due to illness."]
        SickDays,
        #[doc = "The job includes paid time off for vacation."]
        Vacation,
        #[doc = "The job includes vision services covered by a vision insurance plan."]
        Vision,
    }
    impl JobJobBenefitsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobJobBenefitsItems::ChildCare => "CHILD_CARE",
                JobJobBenefitsItems::Dental => "DENTAL",
                JobJobBenefitsItems::DomesticPartner => "DOMESTIC_PARTNER",
                JobJobBenefitsItems::FlexibleHours => "FLEXIBLE_HOURS",
                JobJobBenefitsItems::JobBenefitUnspecified => "JOB_BENEFIT_UNSPECIFIED",
                JobJobBenefitsItems::LifeInsurance => "LIFE_INSURANCE",
                JobJobBenefitsItems::Medical => "MEDICAL",
                JobJobBenefitsItems::ParentalLeave => "PARENTAL_LEAVE",
                JobJobBenefitsItems::RetirementPlan => "RETIREMENT_PLAN",
                JobJobBenefitsItems::SickDays => "SICK_DAYS",
                JobJobBenefitsItems::Vacation => "VACATION",
                JobJobBenefitsItems::Vision => "VISION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobJobBenefitsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobJobBenefitsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobJobBenefitsItems, ()> {
            Ok(match s {
                "CHILD_CARE" => JobJobBenefitsItems::ChildCare,
                "DENTAL" => JobJobBenefitsItems::Dental,
                "DOMESTIC_PARTNER" => JobJobBenefitsItems::DomesticPartner,
                "FLEXIBLE_HOURS" => JobJobBenefitsItems::FlexibleHours,
                "JOB_BENEFIT_UNSPECIFIED" => JobJobBenefitsItems::JobBenefitUnspecified,
                "LIFE_INSURANCE" => JobJobBenefitsItems::LifeInsurance,
                "MEDICAL" => JobJobBenefitsItems::Medical,
                "PARENTAL_LEAVE" => JobJobBenefitsItems::ParentalLeave,
                "RETIREMENT_PLAN" => JobJobBenefitsItems::RetirementPlan,
                "SICK_DAYS" => JobJobBenefitsItems::SickDays,
                "VACATION" => JobJobBenefitsItems::Vacation,
                "VISION" => JobJobBenefitsItems::Vision,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobJobBenefitsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobJobBenefitsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobJobBenefitsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHILD_CARE" => JobJobBenefitsItems::ChildCare,
                "DENTAL" => JobJobBenefitsItems::Dental,
                "DOMESTIC_PARTNER" => JobJobBenefitsItems::DomesticPartner,
                "FLEXIBLE_HOURS" => JobJobBenefitsItems::FlexibleHours,
                "JOB_BENEFIT_UNSPECIFIED" => JobJobBenefitsItems::JobBenefitUnspecified,
                "LIFE_INSURANCE" => JobJobBenefitsItems::LifeInsurance,
                "MEDICAL" => JobJobBenefitsItems::Medical,
                "PARENTAL_LEAVE" => JobJobBenefitsItems::ParentalLeave,
                "RETIREMENT_PLAN" => JobJobBenefitsItems::RetirementPlan,
                "SICK_DAYS" => JobJobBenefitsItems::SickDays,
                "VACATION" => JobJobBenefitsItems::Vacation,
                "VISION" => JobJobBenefitsItems::Vision,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobJobBenefitsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobJobBenefitsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobJobLevel {
        #[doc = "Senior-level managers responsible for managing teams of managers."]
        Director,
        #[doc = "Entry-level individual contributors, typically with less than 2 years of experience in a similar role. Includes interns."]
        EntryLevel,
        #[doc = "Executive-level managers and above, including C-level positions."]
        Executive,
        #[doc = "Experienced individual contributors, typically with 2+ years of experience in a similar role."]
        Experienced,
        #[doc = "The default value if the level is not specified."]
        JobLevelUnspecified,
        #[doc = "Entry- to mid-level managers responsible for managing a team of people."]
        Manager,
    }
    impl JobJobLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                JobJobLevel::Director => "DIRECTOR",
                JobJobLevel::EntryLevel => "ENTRY_LEVEL",
                JobJobLevel::Executive => "EXECUTIVE",
                JobJobLevel::Experienced => "EXPERIENCED",
                JobJobLevel::JobLevelUnspecified => "JOB_LEVEL_UNSPECIFIED",
                JobJobLevel::Manager => "MANAGER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobJobLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobJobLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobJobLevel, ()> {
            Ok(match s {
                "DIRECTOR" => JobJobLevel::Director,
                "ENTRY_LEVEL" => JobJobLevel::EntryLevel,
                "EXECUTIVE" => JobJobLevel::Executive,
                "EXPERIENCED" => JobJobLevel::Experienced,
                "JOB_LEVEL_UNSPECIFIED" => JobJobLevel::JobLevelUnspecified,
                "MANAGER" => JobJobLevel::Manager,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobJobLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobJobLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobJobLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIRECTOR" => JobJobLevel::Director,
                "ENTRY_LEVEL" => JobJobLevel::EntryLevel,
                "EXECUTIVE" => JobJobLevel::Executive,
                "EXPERIENCED" => JobJobLevel::Experienced,
                "JOB_LEVEL_UNSPECIFIED" => JobJobLevel::JobLevelUnspecified,
                "MANAGER" => JobJobLevel::Manager,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobJobLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobJobLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobPostingRegion {
        #[doc = "In addition to exact location matching, job posting is returned when the LocationFilter in the search query is in the same administrative area as the returned job posting. For example, if a `ADMINISTRATIVE_AREA` job is posted in \"CA, USA\", it's returned if LocationFilter has \"Mountain View\". Administrative area refers to top-level administrative subdivision of this country. For example, US state, IT region, UK constituent nation and JP prefecture."]
        AdministrativeArea,
        #[doc = "In addition to exact location matching, job is returned when LocationFilter in search query is in the same country as this job. For example, if a `NATION_WIDE` job is posted in \"USA\", it's returned if LocationFilter has 'Mountain View'."]
        Nation,
        #[doc = "If the region is unspecified, the job is only returned if it matches the LocationFilter."]
        PostingRegionUnspecified,
        #[doc = "Job allows employees to work remotely (telecommute). If locations are provided with this value, the job is considered as having a location, but telecommuting is allowed."]
        Telecommute,
    }
    impl JobPostingRegion {
        pub fn as_str(self) -> &'static str {
            match self {
                JobPostingRegion::AdministrativeArea => "ADMINISTRATIVE_AREA",
                JobPostingRegion::Nation => "NATION",
                JobPostingRegion::PostingRegionUnspecified => "POSTING_REGION_UNSPECIFIED",
                JobPostingRegion::Telecommute => "TELECOMMUTE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobPostingRegion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobPostingRegion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobPostingRegion, ()> {
            Ok(match s {
                "ADMINISTRATIVE_AREA" => JobPostingRegion::AdministrativeArea,
                "NATION" => JobPostingRegion::Nation,
                "POSTING_REGION_UNSPECIFIED" => JobPostingRegion::PostingRegionUnspecified,
                "TELECOMMUTE" => JobPostingRegion::Telecommute,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobPostingRegion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobPostingRegion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobPostingRegion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMINISTRATIVE_AREA" => JobPostingRegion::AdministrativeArea,
                "NATION" => JobPostingRegion::Nation,
                "POSTING_REGION_UNSPECIFIED" => JobPostingRegion::PostingRegionUnspecified,
                "TELECOMMUTE" => JobPostingRegion::Telecommute,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobPostingRegion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobPostingRegion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobVisibility {
        #[doc = "The resource is only visible to the GCP account who owns it."]
        AccountOnly,
        #[doc = "The resource is visible to the owner and may be visible to other applications and processes at Google."]
        SharedWithGoogle,
        #[doc = "The resource is visible to the owner and may be visible to all other API clients."]
        SharedWithPublic,
        #[doc = "Default value."]
        VisibilityUnspecified,
    }
    impl JobVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                JobVisibility::AccountOnly => "ACCOUNT_ONLY",
                JobVisibility::SharedWithGoogle => "SHARED_WITH_GOOGLE",
                JobVisibility::SharedWithPublic => "SHARED_WITH_PUBLIC",
                JobVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
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
                "ACCOUNT_ONLY" => JobVisibility::AccountOnly,
                "SHARED_WITH_GOOGLE" => JobVisibility::SharedWithGoogle,
                "SHARED_WITH_PUBLIC" => JobVisibility::SharedWithPublic,
                "VISIBILITY_UNSPECIFIED" => JobVisibility::VisibilityUnspecified,
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
                "ACCOUNT_ONLY" => JobVisibility::AccountOnly,
                "SHARED_WITH_GOOGLE" => JobVisibility::SharedWithGoogle,
                "SHARED_WITH_PUBLIC" => JobVisibility::SharedWithPublic,
                "VISIBILITY_UNSPECIFIED" => JobVisibility::VisibilityUnspecified,
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
    pub struct JobDerivedInfo {
        #[doc = "Job categories derived from Job.title and Job.description."]
        #[serde(
            rename = "jobCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_categories:
            ::std::option::Option<Vec<crate::schemas::JobDerivedInfoJobCategoriesItems>>,
        #[doc = "Structured locations of the job, resolved from Job.addresses. locations are exactly matched to Job.addresses in the same order."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<Vec<crate::schemas::Location>>,
    }
    impl ::google_field_selector::FieldSelector for JobDerivedInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobDerivedInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobDerivedInfoJobCategoriesItems {
        #[doc = "An accounting and finance job, such as an Accountant."]
        AccountingAndFinance,
        #[doc = "An administrative and office job, such as an Administrative Assistant."]
        AdministrativeAndOffice,
        #[doc = "An advertising and marketing job, such as Marketing Manager."]
        AdvertisingAndMarketing,
        #[doc = "An animal care job, such as Veterinarian."]
        AnimalCare,
        #[doc = "An art, fashion, or design job, such as Designer."]
        ArtFashionAndDesign,
        #[doc = "A business operations job, such as Business Operations Manager."]
        BusinessOperations,
        #[doc = "A cleaning and facilities job, such as Custodial Staff."]
        CleaningAndFacilities,
        #[doc = "A computer and IT job, such as Systems Administrator."]
        ComputerAndIt,
        #[doc = "A construction job, such as General Laborer."]
        Construction,
        #[doc = "A customer service job, such s Cashier."]
        CustomerService,
        #[doc = "An education job, such as School Teacher."]
        Education,
        #[doc = "An entertainment and travel job, such as Flight Attendant."]
        EntertainmentAndTravel,
        #[doc = "A farming or outdoor job, such as Park Ranger."]
        FarmingAndOutdoors,
        #[doc = "A healthcare job, such as Registered Nurse."]
        Healthcare,
        #[doc = "A human resources job, such as Human Resources Director."]
        HumanResources,
        #[doc = "An installation, maintenance, or repair job, such as Electrician."]
        InstallationMaintenanceAndRepair,
        #[doc = "The default value if the category isn't specified."]
        JobCategoryUnspecified,
        #[doc = "A legal job, such as Law Clerk."]
        Legal,
        #[doc = "A management job, often used in conjunction with another category, such as Store Manager."]
        Management,
        #[doc = "A manufacturing or warehouse job, such as Assembly Technician."]
        ManufacturingAndWarehouse,
        #[doc = "A media, communications, or writing job, such as Media Relations."]
        MediaCommunicationsAndWriting,
        #[doc = "An oil, gas or mining job, such as Offshore Driller."]
        OilGasAndMining,
        #[doc = "A personal care and services job, such as Hair Stylist."]
        PersonalCareAndServices,
        #[doc = "A protective services job, such as Security Guard."]
        ProtectiveServices,
        #[doc = "A real estate job, such as Buyer's Agent."]
        RealEstate,
        #[doc = "A restaurant and hospitality job, such as Restaurant Server."]
        RestaurantAndHospitality,
        #[doc = "A sales and/or retail job, such Sales Associate."]
        SalesAndRetail,
        #[doc = "A science and engineering job, such as Lab Technician."]
        ScienceAndEngineering,
        #[doc = "A social services or non-profit job, such as Case Worker."]
        SocialServicesAndNonProfit,
        #[doc = "A sports, fitness, or recreation job, such as Personal Trainer."]
        SportsFitnessAndRecreation,
        #[doc = "A transportation or logistics job, such as Truck Driver."]
        TransportationAndLogistics,
    }
    impl JobDerivedInfoJobCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobDerivedInfoJobCategoriesItems::AccountingAndFinance => "ACCOUNTING_AND_FINANCE",
                JobDerivedInfoJobCategoriesItems::AdministrativeAndOffice => {
                    "ADMINISTRATIVE_AND_OFFICE"
                }
                JobDerivedInfoJobCategoriesItems::AdvertisingAndMarketing => {
                    "ADVERTISING_AND_MARKETING"
                }
                JobDerivedInfoJobCategoriesItems::AnimalCare => "ANIMAL_CARE",
                JobDerivedInfoJobCategoriesItems::ArtFashionAndDesign => "ART_FASHION_AND_DESIGN",
                JobDerivedInfoJobCategoriesItems::BusinessOperations => "BUSINESS_OPERATIONS",
                JobDerivedInfoJobCategoriesItems::CleaningAndFacilities => {
                    "CLEANING_AND_FACILITIES"
                }
                JobDerivedInfoJobCategoriesItems::ComputerAndIt => "COMPUTER_AND_IT",
                JobDerivedInfoJobCategoriesItems::Construction => "CONSTRUCTION",
                JobDerivedInfoJobCategoriesItems::CustomerService => "CUSTOMER_SERVICE",
                JobDerivedInfoJobCategoriesItems::Education => "EDUCATION",
                JobDerivedInfoJobCategoriesItems::EntertainmentAndTravel => {
                    "ENTERTAINMENT_AND_TRAVEL"
                }
                JobDerivedInfoJobCategoriesItems::FarmingAndOutdoors => "FARMING_AND_OUTDOORS",
                JobDerivedInfoJobCategoriesItems::Healthcare => "HEALTHCARE",
                JobDerivedInfoJobCategoriesItems::HumanResources => "HUMAN_RESOURCES",
                JobDerivedInfoJobCategoriesItems::InstallationMaintenanceAndRepair => {
                    "INSTALLATION_MAINTENANCE_AND_REPAIR"
                }
                JobDerivedInfoJobCategoriesItems::JobCategoryUnspecified => {
                    "JOB_CATEGORY_UNSPECIFIED"
                }
                JobDerivedInfoJobCategoriesItems::Legal => "LEGAL",
                JobDerivedInfoJobCategoriesItems::Management => "MANAGEMENT",
                JobDerivedInfoJobCategoriesItems::ManufacturingAndWarehouse => {
                    "MANUFACTURING_AND_WAREHOUSE"
                }
                JobDerivedInfoJobCategoriesItems::MediaCommunicationsAndWriting => {
                    "MEDIA_COMMUNICATIONS_AND_WRITING"
                }
                JobDerivedInfoJobCategoriesItems::OilGasAndMining => "OIL_GAS_AND_MINING",
                JobDerivedInfoJobCategoriesItems::PersonalCareAndServices => {
                    "PERSONAL_CARE_AND_SERVICES"
                }
                JobDerivedInfoJobCategoriesItems::ProtectiveServices => "PROTECTIVE_SERVICES",
                JobDerivedInfoJobCategoriesItems::RealEstate => "REAL_ESTATE",
                JobDerivedInfoJobCategoriesItems::RestaurantAndHospitality => {
                    "RESTAURANT_AND_HOSPITALITY"
                }
                JobDerivedInfoJobCategoriesItems::SalesAndRetail => "SALES_AND_RETAIL",
                JobDerivedInfoJobCategoriesItems::ScienceAndEngineering => {
                    "SCIENCE_AND_ENGINEERING"
                }
                JobDerivedInfoJobCategoriesItems::SocialServicesAndNonProfit => {
                    "SOCIAL_SERVICES_AND_NON_PROFIT"
                }
                JobDerivedInfoJobCategoriesItems::SportsFitnessAndRecreation => {
                    "SPORTS_FITNESS_AND_RECREATION"
                }
                JobDerivedInfoJobCategoriesItems::TransportationAndLogistics => {
                    "TRANSPORTATION_AND_LOGISTICS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobDerivedInfoJobCategoriesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobDerivedInfoJobCategoriesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobDerivedInfoJobCategoriesItems, ()> {
            Ok(match s {
                "ACCOUNTING_AND_FINANCE" => JobDerivedInfoJobCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => {
                    JobDerivedInfoJobCategoriesItems::AdministrativeAndOffice
                }
                "ADVERTISING_AND_MARKETING" => {
                    JobDerivedInfoJobCategoriesItems::AdvertisingAndMarketing
                }
                "ANIMAL_CARE" => JobDerivedInfoJobCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobDerivedInfoJobCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobDerivedInfoJobCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => {
                    JobDerivedInfoJobCategoriesItems::CleaningAndFacilities
                }
                "COMPUTER_AND_IT" => JobDerivedInfoJobCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobDerivedInfoJobCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobDerivedInfoJobCategoriesItems::CustomerService,
                "EDUCATION" => JobDerivedInfoJobCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => {
                    JobDerivedInfoJobCategoriesItems::EntertainmentAndTravel
                }
                "FARMING_AND_OUTDOORS" => JobDerivedInfoJobCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobDerivedInfoJobCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobDerivedInfoJobCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobDerivedInfoJobCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => {
                    JobDerivedInfoJobCategoriesItems::JobCategoryUnspecified
                }
                "LEGAL" => JobDerivedInfoJobCategoriesItems::Legal,
                "MANAGEMENT" => JobDerivedInfoJobCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => {
                    JobDerivedInfoJobCategoriesItems::ManufacturingAndWarehouse
                }
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobDerivedInfoJobCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobDerivedInfoJobCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => {
                    JobDerivedInfoJobCategoriesItems::PersonalCareAndServices
                }
                "PROTECTIVE_SERVICES" => JobDerivedInfoJobCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobDerivedInfoJobCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => {
                    JobDerivedInfoJobCategoriesItems::RestaurantAndHospitality
                }
                "SALES_AND_RETAIL" => JobDerivedInfoJobCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => {
                    JobDerivedInfoJobCategoriesItems::ScienceAndEngineering
                }
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobDerivedInfoJobCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobDerivedInfoJobCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobDerivedInfoJobCategoriesItems::TransportationAndLogistics
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobDerivedInfoJobCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobDerivedInfoJobCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobDerivedInfoJobCategoriesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNTING_AND_FINANCE" => JobDerivedInfoJobCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => {
                    JobDerivedInfoJobCategoriesItems::AdministrativeAndOffice
                }
                "ADVERTISING_AND_MARKETING" => {
                    JobDerivedInfoJobCategoriesItems::AdvertisingAndMarketing
                }
                "ANIMAL_CARE" => JobDerivedInfoJobCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobDerivedInfoJobCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobDerivedInfoJobCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => {
                    JobDerivedInfoJobCategoriesItems::CleaningAndFacilities
                }
                "COMPUTER_AND_IT" => JobDerivedInfoJobCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobDerivedInfoJobCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobDerivedInfoJobCategoriesItems::CustomerService,
                "EDUCATION" => JobDerivedInfoJobCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => {
                    JobDerivedInfoJobCategoriesItems::EntertainmentAndTravel
                }
                "FARMING_AND_OUTDOORS" => JobDerivedInfoJobCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobDerivedInfoJobCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobDerivedInfoJobCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobDerivedInfoJobCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => {
                    JobDerivedInfoJobCategoriesItems::JobCategoryUnspecified
                }
                "LEGAL" => JobDerivedInfoJobCategoriesItems::Legal,
                "MANAGEMENT" => JobDerivedInfoJobCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => {
                    JobDerivedInfoJobCategoriesItems::ManufacturingAndWarehouse
                }
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobDerivedInfoJobCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobDerivedInfoJobCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => {
                    JobDerivedInfoJobCategoriesItems::PersonalCareAndServices
                }
                "PROTECTIVE_SERVICES" => JobDerivedInfoJobCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobDerivedInfoJobCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => {
                    JobDerivedInfoJobCategoriesItems::RestaurantAndHospitality
                }
                "SALES_AND_RETAIL" => JobDerivedInfoJobCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => {
                    JobDerivedInfoJobCategoriesItems::ScienceAndEngineering
                }
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobDerivedInfoJobCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobDerivedInfoJobCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobDerivedInfoJobCategoriesItems::TransportationAndLogistics
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
    impl ::google_field_selector::FieldSelector for JobDerivedInfoJobCategoriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobDerivedInfoJobCategoriesItems {
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
    pub struct JobEvent {
        #[doc = "Required. The job name(s) associated with this event. For example, if this is an impression event, this field contains the identifiers of all jobs shown to the job seeker. If this was a view event, this field contains the identifier of the viewed job."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The type of the event (see JobEventType)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::JobEventType>,
    }
    impl ::google_field_selector::FieldSelector for JobEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobEventType {
        #[doc = "This event should be used when a company submits an application on behalf of a job seeker. This event is intended for use by staffing agencies attempting to place candidates."]
        ApplicationCompanySubmit,
        #[doc = "The job seeker or other entity interacting with the service submitted an application for a job."]
        ApplicationFinish,
        #[doc = "The job seeker or other entity interacting with the service submitted an application for a job with a single click without entering information. If a job seeker performs this action, send only this event to the service. Do not also send JobEventType.APPLICATION_START or JobEventType.APPLICATION_FINISH events."]
        ApplicationQuickSubmission,
        #[doc = "The job seeker or other entity interacting with the service performed an action to apply to a job and was redirected to a different website to complete the application."]
        ApplicationRedirect,
        #[doc = "The job seeker, or other entity interacting with the service, performs an action with a single click from the search results page to apply to a job (without viewing the details of the job posting), and is redirected to a different website to complete the application. If a candidate performs this action, send only this event to the service. Do not also send JobEventType.APPLICATION_START, JobEventType.APPLICATION_FINISH or JobEventType.VIEW events."]
        ApplicationRedirectFromSearch,
        #[doc = "The job seeker or other entity interacting with the service began the process or demonstrated the intention of applying for a job."]
        ApplicationStart,
        #[doc = "The job seeker or other entity interacting with the service began the process or demonstrated the intention of applying for a job from the search results page without viewing the details of the job posting. If sending this event, JobEventType.VIEW event shouldn't be sent."]
        ApplicationStartFromSearch,
        #[doc = "The job seeker or other entity interacting with the service demonstrated an interest in a job by bookmarking or saving it."]
        Bookmark,
        #[doc = "The job seeker or other entity interacting with the service was employed by the hiring entity (employer). Send this event only if the job seeker was hired through an application that was initiated by a search conducted through the Cloud Talent Solution service."]
        Hired,
        #[doc = "The job seeker or other entity interacting with the service has had a job rendered in their view, such as in a list of search results in a compressed or clipped format. This event is typically associated with the viewing of a jobs list on a single page by a job seeker."]
        Impression,
        #[doc = "The entity interacting with the service (for example, the job seeker), was granted an initial interview by the hiring entity (employer). This event should only be sent if the job seeker was granted an interview as part of an application that was initiated by a search conducted through / recommendation provided by the Cloud Talent Solution service."]
        InterviewGranted,
        #[doc = "The event is unspecified by other provided values."]
        JobEventTypeUnspecified,
        #[doc = "The job seeker or other entity interacting with the service showed no interest in the job."]
        NotInterested,
        #[doc = "The job seeker or other entity interacting with the service was sent a notification, such as an email alert or device notification, containing one or more jobs listings generated by the service."]
        Notification,
        #[doc = "A recruiter or staffing agency submitted an application on behalf of the candidate after interacting with the service to identify a suitable job posting."]
        SentCv,
        #[doc = "The job seeker, or other entity interacting with the service, has viewed the details of a job, including the full description. This event doesn't apply to the viewing a snippet of a job appearing as a part of the job search results. Viewing a snippet is associated with an impression)."]
        View,
        #[doc = "The job seeker or other entity interacting with the service performed an action to view a job and was redirected to a different website for job."]
        ViewRedirect,
    }
    impl JobEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                JobEventType::ApplicationCompanySubmit => "APPLICATION_COMPANY_SUBMIT",
                JobEventType::ApplicationFinish => "APPLICATION_FINISH",
                JobEventType::ApplicationQuickSubmission => "APPLICATION_QUICK_SUBMISSION",
                JobEventType::ApplicationRedirect => "APPLICATION_REDIRECT",
                JobEventType::ApplicationRedirectFromSearch => "APPLICATION_REDIRECT_FROM_SEARCH",
                JobEventType::ApplicationStart => "APPLICATION_START",
                JobEventType::ApplicationStartFromSearch => "APPLICATION_START_FROM_SEARCH",
                JobEventType::Bookmark => "BOOKMARK",
                JobEventType::Hired => "HIRED",
                JobEventType::Impression => "IMPRESSION",
                JobEventType::InterviewGranted => "INTERVIEW_GRANTED",
                JobEventType::JobEventTypeUnspecified => "JOB_EVENT_TYPE_UNSPECIFIED",
                JobEventType::NotInterested => "NOT_INTERESTED",
                JobEventType::Notification => "NOTIFICATION",
                JobEventType::SentCv => "SENT_CV",
                JobEventType::View => "VIEW",
                JobEventType::ViewRedirect => "VIEW_REDIRECT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobEventType, ()> {
            Ok(match s {
                "APPLICATION_COMPANY_SUBMIT" => JobEventType::ApplicationCompanySubmit,
                "APPLICATION_FINISH" => JobEventType::ApplicationFinish,
                "APPLICATION_QUICK_SUBMISSION" => JobEventType::ApplicationQuickSubmission,
                "APPLICATION_REDIRECT" => JobEventType::ApplicationRedirect,
                "APPLICATION_REDIRECT_FROM_SEARCH" => JobEventType::ApplicationRedirectFromSearch,
                "APPLICATION_START" => JobEventType::ApplicationStart,
                "APPLICATION_START_FROM_SEARCH" => JobEventType::ApplicationStartFromSearch,
                "BOOKMARK" => JobEventType::Bookmark,
                "HIRED" => JobEventType::Hired,
                "IMPRESSION" => JobEventType::Impression,
                "INTERVIEW_GRANTED" => JobEventType::InterviewGranted,
                "JOB_EVENT_TYPE_UNSPECIFIED" => JobEventType::JobEventTypeUnspecified,
                "NOT_INTERESTED" => JobEventType::NotInterested,
                "NOTIFICATION" => JobEventType::Notification,
                "SENT_CV" => JobEventType::SentCv,
                "VIEW" => JobEventType::View,
                "VIEW_REDIRECT" => JobEventType::ViewRedirect,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_COMPANY_SUBMIT" => JobEventType::ApplicationCompanySubmit,
                "APPLICATION_FINISH" => JobEventType::ApplicationFinish,
                "APPLICATION_QUICK_SUBMISSION" => JobEventType::ApplicationQuickSubmission,
                "APPLICATION_REDIRECT" => JobEventType::ApplicationRedirect,
                "APPLICATION_REDIRECT_FROM_SEARCH" => JobEventType::ApplicationRedirectFromSearch,
                "APPLICATION_START" => JobEventType::ApplicationStart,
                "APPLICATION_START_FROM_SEARCH" => JobEventType::ApplicationStartFromSearch,
                "BOOKMARK" => JobEventType::Bookmark,
                "HIRED" => JobEventType::Hired,
                "IMPRESSION" => JobEventType::Impression,
                "INTERVIEW_GRANTED" => JobEventType::InterviewGranted,
                "JOB_EVENT_TYPE_UNSPECIFIED" => JobEventType::JobEventTypeUnspecified,
                "NOT_INTERESTED" => JobEventType::NotInterested,
                "NOTIFICATION" => JobEventType::Notification,
                "SENT_CV" => JobEventType::SentCv,
                "VIEW" => JobEventType::View,
                "VIEW_REDIRECT" => JobEventType::ViewRedirect,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobEventType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobQuery {
        #[doc = "Optional. Allows filtering jobs by commute time with different travel methods (for example, driving or public transit). Note: This only works with COMMUTE MODE. When specified, [JobQuery.location_filters] is ignored. Currently we don't support sorting by commute time."]
        #[serde(
            rename = "commuteFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commute_filter: ::std::option::Option<crate::schemas::CommuteFilter>,
        #[doc = "Optional. This filter specifies the exact company display name of the jobs to search against. If a value isn't specified, jobs within the search results are associated with any company. If multiple values are specified, jobs within the search results may be associated with any of the specified companies. At most 20 company display name filters are allowed."]
        #[serde(
            rename = "companyDisplayNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_display_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This filter specifies the company entities to search against. If a value isn't specified, jobs are searched for against all companies. If multiple values are specified, jobs are searched against the companies specified. The format is \"projects/{project_id}/companies/{company_id}\", for example, \"projects/api-test-project/companies/foo\". At most 20 company filters are allowed."]
        #[serde(
            rename = "companyNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This search filter is applied only to Job.compensation_info. For example, if the filter is specified as \"Hourly job with per-hour compensation > $15\", only jobs meeting these criteria are searched. If a filter isn't defined, all open jobs are searched."]
        #[serde(
            rename = "compensationFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compensation_filter: ::std::option::Option<crate::schemas::CompensationFilter>,
        #[doc = "Optional. This filter specifies a structured syntax to match against the Job.custom_attributes marked as `filterable`. The syntax for this expression is a subset of SQL syntax. Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left of the operator is a custom field key and the right of the operator is a number or a quoted string. You must escape backslash (\\) and quote (\") characters. Supported functions are `LOWER([field_name])` to perform a case insensitive match and `EMPTY([field_name])` to filter on the existence of a key. Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting (for example, \"((A AND B AND C) OR NOT D) AND E\"), a maximum of 100 comparisons or functions are allowed in the expression. The expression must be < 6000 bytes in length. Sample Query: `(LOWER(driving_license)=\"class \\\"a\\\"\" OR EMPTY(driving_license)) AND driving_years > 10`"]
        #[serde(
            rename = "customAttributeFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attribute_filter: ::std::option::Option<String>,
        #[doc = "Optional. This flag controls the spell-check feature. If false, the service attempts to correct a misspelled query, for example, \"enginee\" is corrected to \"engineer\". Defaults to false: a spell check is performed."]
        #[serde(
            rename = "disableSpellCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_spell_check: ::std::option::Option<bool>,
        #[doc = "Optional. The employment type filter specifies the employment type of jobs to search against, such as EmploymentType.FULL_TIME. If a value is not specified, jobs in the search results includes any employment type. If multiple values are specified, jobs in the search results include any of the specified employment types."]
        #[serde(
            rename = "employmentTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub employment_types:
            ::std::option::Option<Vec<crate::schemas::JobQueryEmploymentTypesItems>>,
        #[doc = "Optional. This filter specifies a list of job names to be excluded during search. At most 400 excluded job names are allowed."]
        #[serde(
            rename = "excludedJobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_jobs: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The category filter specifies the categories of jobs to search against. See Category for more information. If a value is not specified, jobs from any category are searched against. If multiple values are specified, jobs from any of the specified categories are searched against."]
        #[serde(
            rename = "jobCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_categories: ::std::option::Option<Vec<crate::schemas::JobQueryJobCategoriesItems>>,
        #[doc = "Optional. This filter specifies the locale of jobs to search against, for example, \"en-US\". If a value isn't specified, the search results can contain jobs in any locale. Language codes should be in BCP-47 format, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). At most 10 language code filters are allowed."]
        #[serde(
            rename = "languageCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_codes: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The location filter specifies geo-regions containing the jobs to search against. See LocationFilter for more information. If a location value isn't specified, jobs fitting the other search criteria are retrieved regardless of where they're located. If multiple values are specified, jobs are retrieved from any of the specified locations. If different values are specified for the LocationFilter.distance_in_miles parameter, the maximum provided distance is used for all locations. At most 5 location filters are allowed."]
        #[serde(
            rename = "locationFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_filters: ::std::option::Option<Vec<crate::schemas::LocationFilter>>,
        #[doc = "Optional. Jobs published within a range specified by this filter are searched against."]
        #[serde(
            rename = "publishTimeRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_time_range: ::std::option::Option<crate::schemas::TimestampRange>,
        #[doc = "Optional. The query string that matches against the job title, description, and location fields. The maximum number of allowed characters is 255."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "The language code of query. For example, \"en-US\". This field helps to better interpret the query. If a value isn't specified, the query language code is automatically detected, which may not be accurate. Language code should be in BCP-47 format, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        #[serde(
            rename = "queryLanguageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_language_code: ::std::option::Option<String>,
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
    pub enum JobQueryEmploymentTypesItems {
        #[doc = "The job is offered as a contracted position with the understanding that it's converted into a full-time position at the end of the contract. Jobs of this type are also returned by a search for EmploymentType.CONTRACTOR jobs."]
        ContractToHire,
        #[doc = "The job is offered as a contracted, as opposed to a salaried employee, position."]
        Contractor,
        #[doc = "The default value if the employment type is not specified."]
        EmploymentTypeUnspecified,
        #[doc = "The job involves employing people in remote areas and flying them temporarily to the work site instead of relocating employees and their families permanently."]
        FlyInFlyOut,
        #[doc = "The job requires working a number of hours that constitute full time employment, typically 40 or more hours per week."]
        FullTime,
        #[doc = "The job is a fixed-term opportunity for students or entry-level job seekers to obtain on-the-job training, typically offered as a summer position."]
        Intern,
        #[doc = "The job does not fit any of the other listed types."]
        OtherEmploymentType,
        #[doc = "The job entails working fewer hours than a full time job, typically less than 40 hours a week."]
        PartTime,
        #[doc = "The job requires an employee to work on an as-needed basis with a flexible schedule."]
        PerDiem,
        #[doc = "The job is offered as a temporary employment opportunity, usually a short-term engagement."]
        Temporary,
        #[doc = "The is an opportunity for an individual to volunteer, where there's no expectation of compensation for the provided services."]
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
                JobQueryEmploymentTypesItems::OtherEmploymentType => "OTHER_EMPLOYMENT_TYPE",
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
                "OTHER_EMPLOYMENT_TYPE" => JobQueryEmploymentTypesItems::OtherEmploymentType,
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
                "OTHER_EMPLOYMENT_TYPE" => JobQueryEmploymentTypesItems::OtherEmploymentType,
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
    pub enum JobQueryJobCategoriesItems {
        #[doc = "An accounting and finance job, such as an Accountant."]
        AccountingAndFinance,
        #[doc = "An administrative and office job, such as an Administrative Assistant."]
        AdministrativeAndOffice,
        #[doc = "An advertising and marketing job, such as Marketing Manager."]
        AdvertisingAndMarketing,
        #[doc = "An animal care job, such as Veterinarian."]
        AnimalCare,
        #[doc = "An art, fashion, or design job, such as Designer."]
        ArtFashionAndDesign,
        #[doc = "A business operations job, such as Business Operations Manager."]
        BusinessOperations,
        #[doc = "A cleaning and facilities job, such as Custodial Staff."]
        CleaningAndFacilities,
        #[doc = "A computer and IT job, such as Systems Administrator."]
        ComputerAndIt,
        #[doc = "A construction job, such as General Laborer."]
        Construction,
        #[doc = "A customer service job, such s Cashier."]
        CustomerService,
        #[doc = "An education job, such as School Teacher."]
        Education,
        #[doc = "An entertainment and travel job, such as Flight Attendant."]
        EntertainmentAndTravel,
        #[doc = "A farming or outdoor job, such as Park Ranger."]
        FarmingAndOutdoors,
        #[doc = "A healthcare job, such as Registered Nurse."]
        Healthcare,
        #[doc = "A human resources job, such as Human Resources Director."]
        HumanResources,
        #[doc = "An installation, maintenance, or repair job, such as Electrician."]
        InstallationMaintenanceAndRepair,
        #[doc = "The default value if the category isn't specified."]
        JobCategoryUnspecified,
        #[doc = "A legal job, such as Law Clerk."]
        Legal,
        #[doc = "A management job, often used in conjunction with another category, such as Store Manager."]
        Management,
        #[doc = "A manufacturing or warehouse job, such as Assembly Technician."]
        ManufacturingAndWarehouse,
        #[doc = "A media, communications, or writing job, such as Media Relations."]
        MediaCommunicationsAndWriting,
        #[doc = "An oil, gas or mining job, such as Offshore Driller."]
        OilGasAndMining,
        #[doc = "A personal care and services job, such as Hair Stylist."]
        PersonalCareAndServices,
        #[doc = "A protective services job, such as Security Guard."]
        ProtectiveServices,
        #[doc = "A real estate job, such as Buyer's Agent."]
        RealEstate,
        #[doc = "A restaurant and hospitality job, such as Restaurant Server."]
        RestaurantAndHospitality,
        #[doc = "A sales and/or retail job, such Sales Associate."]
        SalesAndRetail,
        #[doc = "A science and engineering job, such as Lab Technician."]
        ScienceAndEngineering,
        #[doc = "A social services or non-profit job, such as Case Worker."]
        SocialServicesAndNonProfit,
        #[doc = "A sports, fitness, or recreation job, such as Personal Trainer."]
        SportsFitnessAndRecreation,
        #[doc = "A transportation or logistics job, such as Truck Driver."]
        TransportationAndLogistics,
    }
    impl JobQueryJobCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                JobQueryJobCategoriesItems::AccountingAndFinance => "ACCOUNTING_AND_FINANCE",
                JobQueryJobCategoriesItems::AdministrativeAndOffice => "ADMINISTRATIVE_AND_OFFICE",
                JobQueryJobCategoriesItems::AdvertisingAndMarketing => "ADVERTISING_AND_MARKETING",
                JobQueryJobCategoriesItems::AnimalCare => "ANIMAL_CARE",
                JobQueryJobCategoriesItems::ArtFashionAndDesign => "ART_FASHION_AND_DESIGN",
                JobQueryJobCategoriesItems::BusinessOperations => "BUSINESS_OPERATIONS",
                JobQueryJobCategoriesItems::CleaningAndFacilities => "CLEANING_AND_FACILITIES",
                JobQueryJobCategoriesItems::ComputerAndIt => "COMPUTER_AND_IT",
                JobQueryJobCategoriesItems::Construction => "CONSTRUCTION",
                JobQueryJobCategoriesItems::CustomerService => "CUSTOMER_SERVICE",
                JobQueryJobCategoriesItems::Education => "EDUCATION",
                JobQueryJobCategoriesItems::EntertainmentAndTravel => "ENTERTAINMENT_AND_TRAVEL",
                JobQueryJobCategoriesItems::FarmingAndOutdoors => "FARMING_AND_OUTDOORS",
                JobQueryJobCategoriesItems::Healthcare => "HEALTHCARE",
                JobQueryJobCategoriesItems::HumanResources => "HUMAN_RESOURCES",
                JobQueryJobCategoriesItems::InstallationMaintenanceAndRepair => {
                    "INSTALLATION_MAINTENANCE_AND_REPAIR"
                }
                JobQueryJobCategoriesItems::JobCategoryUnspecified => "JOB_CATEGORY_UNSPECIFIED",
                JobQueryJobCategoriesItems::Legal => "LEGAL",
                JobQueryJobCategoriesItems::Management => "MANAGEMENT",
                JobQueryJobCategoriesItems::ManufacturingAndWarehouse => {
                    "MANUFACTURING_AND_WAREHOUSE"
                }
                JobQueryJobCategoriesItems::MediaCommunicationsAndWriting => {
                    "MEDIA_COMMUNICATIONS_AND_WRITING"
                }
                JobQueryJobCategoriesItems::OilGasAndMining => "OIL_GAS_AND_MINING",
                JobQueryJobCategoriesItems::PersonalCareAndServices => "PERSONAL_CARE_AND_SERVICES",
                JobQueryJobCategoriesItems::ProtectiveServices => "PROTECTIVE_SERVICES",
                JobQueryJobCategoriesItems::RealEstate => "REAL_ESTATE",
                JobQueryJobCategoriesItems::RestaurantAndHospitality => {
                    "RESTAURANT_AND_HOSPITALITY"
                }
                JobQueryJobCategoriesItems::SalesAndRetail => "SALES_AND_RETAIL",
                JobQueryJobCategoriesItems::ScienceAndEngineering => "SCIENCE_AND_ENGINEERING",
                JobQueryJobCategoriesItems::SocialServicesAndNonProfit => {
                    "SOCIAL_SERVICES_AND_NON_PROFIT"
                }
                JobQueryJobCategoriesItems::SportsFitnessAndRecreation => {
                    "SPORTS_FITNESS_AND_RECREATION"
                }
                JobQueryJobCategoriesItems::TransportationAndLogistics => {
                    "TRANSPORTATION_AND_LOGISTICS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobQueryJobCategoriesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobQueryJobCategoriesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobQueryJobCategoriesItems, ()> {
            Ok(match s {
                "ACCOUNTING_AND_FINANCE" => JobQueryJobCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => JobQueryJobCategoriesItems::AdministrativeAndOffice,
                "ADVERTISING_AND_MARKETING" => JobQueryJobCategoriesItems::AdvertisingAndMarketing,
                "ANIMAL_CARE" => JobQueryJobCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobQueryJobCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobQueryJobCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => JobQueryJobCategoriesItems::CleaningAndFacilities,
                "COMPUTER_AND_IT" => JobQueryJobCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobQueryJobCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobQueryJobCategoriesItems::CustomerService,
                "EDUCATION" => JobQueryJobCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => JobQueryJobCategoriesItems::EntertainmentAndTravel,
                "FARMING_AND_OUTDOORS" => JobQueryJobCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobQueryJobCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobQueryJobCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobQueryJobCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => JobQueryJobCategoriesItems::JobCategoryUnspecified,
                "LEGAL" => JobQueryJobCategoriesItems::Legal,
                "MANAGEMENT" => JobQueryJobCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => {
                    JobQueryJobCategoriesItems::ManufacturingAndWarehouse
                }
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobQueryJobCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobQueryJobCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => JobQueryJobCategoriesItems::PersonalCareAndServices,
                "PROTECTIVE_SERVICES" => JobQueryJobCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobQueryJobCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => {
                    JobQueryJobCategoriesItems::RestaurantAndHospitality
                }
                "SALES_AND_RETAIL" => JobQueryJobCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => JobQueryJobCategoriesItems::ScienceAndEngineering,
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobQueryJobCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobQueryJobCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobQueryJobCategoriesItems::TransportationAndLogistics
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobQueryJobCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobQueryJobCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobQueryJobCategoriesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNTING_AND_FINANCE" => JobQueryJobCategoriesItems::AccountingAndFinance,
                "ADMINISTRATIVE_AND_OFFICE" => JobQueryJobCategoriesItems::AdministrativeAndOffice,
                "ADVERTISING_AND_MARKETING" => JobQueryJobCategoriesItems::AdvertisingAndMarketing,
                "ANIMAL_CARE" => JobQueryJobCategoriesItems::AnimalCare,
                "ART_FASHION_AND_DESIGN" => JobQueryJobCategoriesItems::ArtFashionAndDesign,
                "BUSINESS_OPERATIONS" => JobQueryJobCategoriesItems::BusinessOperations,
                "CLEANING_AND_FACILITIES" => JobQueryJobCategoriesItems::CleaningAndFacilities,
                "COMPUTER_AND_IT" => JobQueryJobCategoriesItems::ComputerAndIt,
                "CONSTRUCTION" => JobQueryJobCategoriesItems::Construction,
                "CUSTOMER_SERVICE" => JobQueryJobCategoriesItems::CustomerService,
                "EDUCATION" => JobQueryJobCategoriesItems::Education,
                "ENTERTAINMENT_AND_TRAVEL" => JobQueryJobCategoriesItems::EntertainmentAndTravel,
                "FARMING_AND_OUTDOORS" => JobQueryJobCategoriesItems::FarmingAndOutdoors,
                "HEALTHCARE" => JobQueryJobCategoriesItems::Healthcare,
                "HUMAN_RESOURCES" => JobQueryJobCategoriesItems::HumanResources,
                "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                    JobQueryJobCategoriesItems::InstallationMaintenanceAndRepair
                }
                "JOB_CATEGORY_UNSPECIFIED" => JobQueryJobCategoriesItems::JobCategoryUnspecified,
                "LEGAL" => JobQueryJobCategoriesItems::Legal,
                "MANAGEMENT" => JobQueryJobCategoriesItems::Management,
                "MANUFACTURING_AND_WAREHOUSE" => {
                    JobQueryJobCategoriesItems::ManufacturingAndWarehouse
                }
                "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                    JobQueryJobCategoriesItems::MediaCommunicationsAndWriting
                }
                "OIL_GAS_AND_MINING" => JobQueryJobCategoriesItems::OilGasAndMining,
                "PERSONAL_CARE_AND_SERVICES" => JobQueryJobCategoriesItems::PersonalCareAndServices,
                "PROTECTIVE_SERVICES" => JobQueryJobCategoriesItems::ProtectiveServices,
                "REAL_ESTATE" => JobQueryJobCategoriesItems::RealEstate,
                "RESTAURANT_AND_HOSPITALITY" => {
                    JobQueryJobCategoriesItems::RestaurantAndHospitality
                }
                "SALES_AND_RETAIL" => JobQueryJobCategoriesItems::SalesAndRetail,
                "SCIENCE_AND_ENGINEERING" => JobQueryJobCategoriesItems::ScienceAndEngineering,
                "SOCIAL_SERVICES_AND_NON_PROFIT" => {
                    JobQueryJobCategoriesItems::SocialServicesAndNonProfit
                }
                "SPORTS_FITNESS_AND_RECREATION" => {
                    JobQueryJobCategoriesItems::SportsFitnessAndRecreation
                }
                "TRANSPORTATION_AND_LOGISTICS" => {
                    JobQueryJobCategoriesItems::TransportationAndLogistics
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
    impl ::google_field_selector::FieldSelector for JobQueryJobCategoriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobQueryJobCategoriesItems {
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
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
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
    pub struct ListJobsResponse {
        #[doc = "The Jobs for a given company. The maximum number of items returned is based on the limit field provided in the request."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::Job>>,
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
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
    pub struct Location {
        #[doc = "An object representing a latitude/longitude pair."]
        #[serde(
            rename = "latLng",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "The type of a location, which corresponds to the address lines field of PostalAddress. For example, \"Downtown, Atlanta, GA, USA\" has a type of LocationType#NEIGHBORHOOD, and \"Kansas City, KS, USA\" has a type of LocationType#LOCALITY."]
        #[serde(
            rename = "locationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_type: ::std::option::Option<crate::schemas::LocationLocationType>,
        #[doc = "Postal address of the location that includes human readable information, such as postal delivery and payments addresses. Given a postal address, a postal service can deliver items to a premises, P.O. Box, or other delivery location."]
        #[serde(
            rename = "postalAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_address: ::std::option::Option<crate::schemas::PostalAddress>,
        #[doc = "Radius in miles of the job location. This value is derived from the location bounding box in which a circle with the specified radius centered from LatLng covers the area associated with the job location. For example, currently, \"Mountain View, CA, USA\" has a radius of 6.17 miles."]
        #[serde(
            rename = "radiusInMiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub radius_in_miles: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Location {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LocationLocationType {
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
        #[doc = "A sublocality is a subdivision of a locality, for example a city borough, ward, or arrondissement. Sublocalities are usually recognized by a local political authority. For example, Manhattan and Brooklyn are recognized as boroughs by the City of New York, and are therefore modeled as sublocalities."]
        SubLocality,
        #[doc = "A district or equivalent level location."]
        SubLocality1,
        #[doc = "A smaller district or equivalent level display."]
        SubLocality2,
    }
    impl LocationLocationType {
        pub fn as_str(self) -> &'static str {
            match self {
                LocationLocationType::AdministrativeArea => "ADMINISTRATIVE_AREA",
                LocationLocationType::Country => "COUNTRY",
                LocationLocationType::Locality => "LOCALITY",
                LocationLocationType::LocationTypeUnspecified => "LOCATION_TYPE_UNSPECIFIED",
                LocationLocationType::Neighborhood => "NEIGHBORHOOD",
                LocationLocationType::PostalCode => "POSTAL_CODE",
                LocationLocationType::StreetAddress => "STREET_ADDRESS",
                LocationLocationType::SubAdministrativeArea => "SUB_ADMINISTRATIVE_AREA",
                LocationLocationType::SubLocality => "SUB_LOCALITY",
                LocationLocationType::SubLocality1 => "SUB_LOCALITY_1",
                LocationLocationType::SubLocality2 => "SUB_LOCALITY_2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LocationLocationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LocationLocationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LocationLocationType, ()> {
            Ok(match s {
                "ADMINISTRATIVE_AREA" => LocationLocationType::AdministrativeArea,
                "COUNTRY" => LocationLocationType::Country,
                "LOCALITY" => LocationLocationType::Locality,
                "LOCATION_TYPE_UNSPECIFIED" => LocationLocationType::LocationTypeUnspecified,
                "NEIGHBORHOOD" => LocationLocationType::Neighborhood,
                "POSTAL_CODE" => LocationLocationType::PostalCode,
                "STREET_ADDRESS" => LocationLocationType::StreetAddress,
                "SUB_ADMINISTRATIVE_AREA" => LocationLocationType::SubAdministrativeArea,
                "SUB_LOCALITY" => LocationLocationType::SubLocality,
                "SUB_LOCALITY_1" => LocationLocationType::SubLocality1,
                "SUB_LOCALITY_2" => LocationLocationType::SubLocality2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LocationLocationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LocationLocationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LocationLocationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMINISTRATIVE_AREA" => LocationLocationType::AdministrativeArea,
                "COUNTRY" => LocationLocationType::Country,
                "LOCALITY" => LocationLocationType::Locality,
                "LOCATION_TYPE_UNSPECIFIED" => LocationLocationType::LocationTypeUnspecified,
                "NEIGHBORHOOD" => LocationLocationType::Neighborhood,
                "POSTAL_CODE" => LocationLocationType::PostalCode,
                "STREET_ADDRESS" => LocationLocationType::StreetAddress,
                "SUB_ADMINISTRATIVE_AREA" => LocationLocationType::SubAdministrativeArea,
                "SUB_LOCALITY" => LocationLocationType::SubLocality,
                "SUB_LOCALITY_1" => LocationLocationType::SubLocality1,
                "SUB_LOCALITY_2" => LocationLocationType::SubLocality2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LocationLocationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocationLocationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LocationFilter {
        #[doc = "Optional. The address name, such as \"Mountain View\" or \"Bay Area\"."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "Optional. The distance_in_miles is applied when the location being searched for is identified as a city or smaller. When the location being searched for is a state or larger, this field is ignored."]
        #[serde(
            rename = "distanceInMiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distance_in_miles: ::std::option::Option<f64>,
        #[doc = "Optional. The latitude and longitude of the geographic center from which to search. This field's ignored if `address` is provided."]
        #[serde(
            rename = "latLng",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Optional. CLDR region code of the country/region. This field may be used in two ways: 1) If telecommute preference is not set, this field is used address ambiguity of the user-input address. For example, \"Liverpool\" may refer to \"Liverpool, NY, US\" or \"Liverpool, UK\". This region code biases the address resolution toward a specific country or territory. If this field is not set, address resolution is biased toward the United States by default. 2) If telecommute preference is set to TELECOMMUTE_ALLOWED, the telecommute location filter will be limited to the region specified in this field. If this field is not set, the telecommute job locations will not be limited. See https://unicode-org.github.io/cldr-staging/charts/latest/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
        #[doc = "Optional. Allows the client to return jobs without a set location, specifically, telecommuting jobs (telecommuting is considered by the service as a special location. Job.posting_region indicates if a job permits telecommuting. If this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED, telecommuting jobs are searched, and address and lat_lng are ignored. If not set or set to TelecommutePreference.TELECOMMUTE_EXCLUDED, telecommute job are not searched. This filter can be used by itself to search exclusively for telecommuting jobs, or it can be combined with another location filter to search for a combination of job locations, such as \"Mountain View\" or \"telecommuting\" jobs. However, when used in combination with other location filters, telecommuting jobs can be treated as less relevant than other jobs in the search response."]
        #[serde(
            rename = "telecommutePreference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub telecommute_preference:
            ::std::option::Option<crate::schemas::LocationFilterTelecommutePreference>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LocationFilterTelecommutePreference {
        #[doc = "Allow telecommute jobs."]
        TelecommuteAllowed,
        #[doc = "Exclude telecommute jobs."]
        TelecommuteExcluded,
        #[doc = "Default value if the telecommute preference is not specified."]
        TelecommutePreferenceUnspecified,
    }
    impl LocationFilterTelecommutePreference {
        pub fn as_str(self) -> &'static str {
            match self {
                LocationFilterTelecommutePreference::TelecommuteAllowed => "TELECOMMUTE_ALLOWED",
                LocationFilterTelecommutePreference::TelecommuteExcluded => "TELECOMMUTE_EXCLUDED",
                LocationFilterTelecommutePreference::TelecommutePreferenceUnspecified => {
                    "TELECOMMUTE_PREFERENCE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for LocationFilterTelecommutePreference {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LocationFilterTelecommutePreference {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LocationFilterTelecommutePreference, ()> {
            Ok(match s {
                "TELECOMMUTE_ALLOWED" => LocationFilterTelecommutePreference::TelecommuteAllowed,
                "TELECOMMUTE_EXCLUDED" => LocationFilterTelecommutePreference::TelecommuteExcluded,
                "TELECOMMUTE_PREFERENCE_UNSPECIFIED" => {
                    LocationFilterTelecommutePreference::TelecommutePreferenceUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LocationFilterTelecommutePreference {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LocationFilterTelecommutePreference {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LocationFilterTelecommutePreference {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TELECOMMUTE_ALLOWED" => LocationFilterTelecommutePreference::TelecommuteAllowed,
                "TELECOMMUTE_EXCLUDED" => LocationFilterTelecommutePreference::TelecommuteExcluded,
                "TELECOMMUTE_PREFERENCE_UNSPECIFIED" => {
                    LocationFilterTelecommutePreference::TelecommutePreferenceUnspecified
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
    impl ::google_field_selector::FieldSelector for LocationFilterTelecommutePreference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocationFilterTelecommutePreference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MatchingJob {
        #[doc = "Commute information which is generated based on specified CommuteFilter."]
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
        #[doc = "A summary of the job with core information that's displayed on the search results listing page."]
        #[serde(
            rename = "jobSummary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_summary: ::std::option::Option<String>,
        #[doc = "Contains snippets of text from the Job.job_title field most closely matching a search query's keywords, if available. The matching query keywords are enclosed in HTML bold tags."]
        #[serde(
            rename = "jobTitleSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_title_snippet: ::std::option::Option<String>,
        #[doc = "Contains snippets of text from the Job.description and similar fields that most closely match a search query's keywords, if available. All HTML tags in the original fields are stripped when returned in this field, and matching query keywords are enclosed in HTML bold tags."]
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
        #[doc = "When a request spans multiple servers, a MendelDebugInput may travel with the request and take effect in all the servers. This field is a map of namespaces to NamespacedMendelDebugInput protos. In a single server, up to two NamespacedMendelDebugInput protos are applied: 1. NamespacedMendelDebugInput with the global namespace (key == \"\"). 2. NamespacedMendelDebugInput with the server's namespace. When both NamespacedMendelDebugInput protos are present, they are merged. See go/mendel-debug-forcing for more details."]
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
        #[doc = "The three-letter currency code defined in ISO 4217."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
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
        #[doc = "Set of experiment names to be absolutely forced. These experiments will be forced without evaluating the conditions."]
        #[serde(
            rename = "absolutelyForcedExpNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolutely_forced_exp_names: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment tags to be absolutely forced. The experiments with these tags will be forced without evaluating the conditions."]
        #[serde(
            rename = "absolutelyForcedExpTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolutely_forced_exp_tags: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment ids to be absolutely forced. These ids will be forced without evaluating the conditions."]
        #[serde(
            rename = "absolutelyForcedExps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolutely_forced_exps: ::std::option::Option<Vec<i32>>,
        #[doc = "Set of experiment names to be conditionally forced. These experiments will be forced only if their conditions and their parent domain's conditions are true."]
        #[serde(
            rename = "conditionallyForcedExpNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditionally_forced_exp_names: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment tags to be conditionally forced. The experiments with these tags will be forced only if their conditions and their parent domain's conditions are true."]
        #[serde(
            rename = "conditionallyForcedExpTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditionally_forced_exp_tags: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment ids to be conditionally forced. These ids will be forced only if their conditions and their parent domain's conditions are true."]
        #[serde(
            rename = "conditionallyForcedExps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditionally_forced_exps: ::std::option::Option<Vec<i32>>,
        #[doc = "If true, disable automatic enrollment selection (at all diversion points). Automatic enrollment selection means experiment selection process based on the experiment's automatic enrollment condition. This does not disable selection of forced experiments."]
        #[serde(
            rename = "disableAutomaticEnrollmentSelection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_automatic_enrollment_selection: ::std::option::Option<bool>,
        #[doc = "Set of experiment names to be disabled. If an experiment is disabled, it is never selected nor forced. If an aggregate experiment is disabled, its partitions are disabled together. If an experiment with an enrollment is disabled, the enrollment is disabled together. If a name corresponds to a domain, the domain itself and all descendant experiments and domains are disabled together."]
        #[serde(
            rename = "disableExpNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_exp_names: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment tags to be disabled. All experiments that are tagged with one or more of these tags are disabled. If an experiment is disabled, it is never selected nor forced. If an aggregate experiment is disabled, its partitions are disabled together. If an experiment with an enrollment is disabled, the enrollment is disabled together."]
        #[serde(
            rename = "disableExpTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_exp_tags: ::std::option::Option<Vec<String>>,
        #[doc = "Set of experiment ids to be disabled. If an experiment is disabled, it is never selected nor forced. If an aggregate experiment is disabled, its partitions are disabled together. If an experiment with an enrollment is disabled, the enrollment is disabled together. If an ID corresponds to a domain, the domain itself and all descendant experiments and domains are disabled together."]
        #[serde(
            rename = "disableExps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_exps: ::std::option::Option<Vec<i32>>,
        #[doc = "If true, disable manual enrollment selection (at all diversion points). Manual enrollment selection means experiment selection process based on the request's manual enrollment states (a.k.a. opt-in experiments). This does not disable selection of forced experiments."]
        #[serde(
            rename = "disableManualEnrollmentSelection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_manual_enrollment_selection: ::std::option::Option<bool>,
        #[doc = "If true, disable organic experiment selection (at all diversion points). Organic selection means experiment selection process based on traffic allocation and diversion condition evaluation. This does not disable selection of forced experiments. This is useful in cases when it is not known whether experiment selection behavior is responsible for a error or breakage. Disabling organic selection may help to isolate the cause of a given problem."]
        #[serde(
            rename = "disableOrganicSelection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_organic_selection: ::std::option::Option<bool>,
        #[doc = "Flags to force in a particular experiment state. Map from flag name to flag value."]
        #[serde(
            rename = "forcedFlags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub forced_flags: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Rollouts to force in a particular experiment state. Map from rollout name to rollout value."]
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
        #[doc = "Required. Two adjacent values form a histogram bucket. Values should be in ascending order. For example, if [5, 10, 15] are provided, four buckets are created: (-inf, 5), 5, 10), [10, 15), [15, inf). At most 20 [buckets_bound is supported."]
        #[serde(
            rename = "bucketBounds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_bounds: ::std::option::Option<Vec<f64>>,
        #[doc = "Optional. If set to true, the histogram result includes minimum/maximum value of the numeric field."]
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
        #[doc = "Count within each bucket. Its size is the length of NumericBucketingOption.bucket_bounds plus 1."]
        #[serde(
            rename = "counts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub counts: ::std::option::Option<Vec<crate::schemas::BucketizedCount>>,
        #[doc = "Stores the maximum value of the numeric field. Is populated only if [NumericBucketingOption.requires_min_max] is set to true."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "Stores the minimum value of the numeric field. Will be populated only if [NumericBucketingOption.requires_min_max] is set to true."]
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
        #[doc = "Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be \"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas)."]
        #[serde(
            rename = "addressLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_lines: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated."]
        #[serde(
            rename = "administrativeArea",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub administrative_area: ::std::option::Option<String>,
        #[doc = "Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines."]
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
        #[doc = "Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_code: ::std::option::Option<String>,
        #[doc = "Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain \"care of\" information."]
        #[serde(
            rename = "recipients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipients: ::std::option::Option<Vec<String>>,
        #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like \"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number alone, representing the \"sector code\" (Jamaica), \"delivery area indicator\" (Malawi) or \"post office indicator\" (e.g. Côte d'Ivoire)."]
        #[serde(
            rename = "sortingCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sorting_code: ::std::option::Option<String>,
        #[doc = "Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts."]
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
    pub struct ProcessingOptions {
        #[doc = "Optional. If set to `true`, the service does not attempt to resolve a more precise address for the job."]
        #[serde(
            rename = "disableStreetAddressResolution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_street_address_resolution: ::std::option::Option<bool>,
        #[doc = "Optional. Option for job HTML content sanitization. Applied fields are: * description * applicationInfo.instruction * incentives * qualifications * responsibilities HTML tags in these fields may be stripped if sanitiazation is not disabled. Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY."]
        #[serde(
            rename = "htmlSanitization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_sanitization:
            ::std::option::Option<crate::schemas::ProcessingOptionsHtmlSanitization>,
    }
    impl ::google_field_selector::FieldSelector for ProcessingOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProcessingOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProcessingOptionsHtmlSanitization {
        #[doc = "Disables sanitization on HTML input."]
        HtmlSanitizationDisabled,
        #[doc = "Default value."]
        HtmlSanitizationUnspecified,
        #[doc = "Sanitizes HTML input, only accepts bold, italic, ordered list, and unordered list markup tags."]
        SimpleFormattingOnly,
    }
    impl ProcessingOptionsHtmlSanitization {
        pub fn as_str(self) -> &'static str {
            match self {
                ProcessingOptionsHtmlSanitization::HtmlSanitizationDisabled => {
                    "HTML_SANITIZATION_DISABLED"
                }
                ProcessingOptionsHtmlSanitization::HtmlSanitizationUnspecified => {
                    "HTML_SANITIZATION_UNSPECIFIED"
                }
                ProcessingOptionsHtmlSanitization::SimpleFormattingOnly => "SIMPLE_FORMATTING_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ProcessingOptionsHtmlSanitization {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ProcessingOptionsHtmlSanitization {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ProcessingOptionsHtmlSanitization, ()> {
            Ok(match s {
                "HTML_SANITIZATION_DISABLED" => {
                    ProcessingOptionsHtmlSanitization::HtmlSanitizationDisabled
                }
                "HTML_SANITIZATION_UNSPECIFIED" => {
                    ProcessingOptionsHtmlSanitization::HtmlSanitizationUnspecified
                }
                "SIMPLE_FORMATTING_ONLY" => ProcessingOptionsHtmlSanitization::SimpleFormattingOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ProcessingOptionsHtmlSanitization {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProcessingOptionsHtmlSanitization {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProcessingOptionsHtmlSanitization {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HTML_SANITIZATION_DISABLED" => {
                    ProcessingOptionsHtmlSanitization::HtmlSanitizationDisabled
                }
                "HTML_SANITIZATION_UNSPECIFIED" => {
                    ProcessingOptionsHtmlSanitization::HtmlSanitizationUnspecified
                }
                "SIMPLE_FORMATTING_ONLY" => ProcessingOptionsHtmlSanitization::SimpleFormattingOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ProcessingOptionsHtmlSanitization {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProcessingOptionsHtmlSanitization {
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
        #[doc = "Optional. The type of device used by the job seeker at the time of the call to the service."]
        #[serde(
            rename = "deviceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_info: ::std::option::Option<crate::schemas::DeviceInfo>,
        #[doc = "Required. The client-defined scope or source of the service call, which typically is the domain on which the service has been implemented and is currently being run. For example, if the service is being run by client *Foo, Inc.*, on job board www.foo.com and career site www.bar.com, then this field is set to \"foo.com\" for use on the job board, and \"bar.com\" for use on the career site. If this field isn't available for some reason, send \"UNKNOWN\". Any improvements to the model for a particular tenant site rely on this field being set correctly to a domain. The maximum number of allowed characters is 255."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "Required. A unique session identification string. A session is defined as the duration of an end user's interaction with the service over a certain period. Obfuscate this field for privacy concerns before providing it to the service. If this field is not available for some reason, send \"UNKNOWN\". Note that any improvements to the model for a particular tenant site, rely on this field being set correctly to some unique session_id. The maximum number of allowed characters is 255."]
        #[serde(
            rename = "sessionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_id: ::std::option::Option<String>,
        #[doc = "Required. A unique user identification string, as determined by the client. To have the strongest positive impact on search quality make sure the client-level is unique. Obfuscate this field for privacy concerns before providing it to the service. If this field is not available for some reason, send \"UNKNOWN\". Note that any improvements to the model for a particular tenant site, rely on this field being set correctly to a unique user_id. The maximum number of allowed characters is 255."]
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
        #[doc = "A unique id associated with this call. This id is logged for tracking purposes."]
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
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchJobsRequest {
        #[doc = "Optional. Controls over how job documents get ranked on top of existing relevance score (determined by API algorithm)."]
        #[serde(
            rename = "customRankingInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_ranking_info: ::std::option::Option<crate::schemas::CustomRankingInfo>,
        #[doc = "Optional. Controls whether to disable exact keyword match on Job.job_title, Job.description, Job.company_display_name, Job.locations, Job.qualifications. When disable keyword match is turned off, a keyword match returns jobs that do not match given category filters when there are matching keywords. For example, the query \"program manager,\" a result is returned even if the job posting has the title \"software developer,\" which does not fall into \"program manager\" ontology, but does have \"program manager\" appearing in its description. For queries like \"cloud\" that does not contain title or location specific ontology, jobs with \"cloud\" keyword matches are returned regardless of this flag's value. Please use Company.keyword_searchable_custom_fields or Company.keyword_searchable_custom_attributes if company specific globally matched custom field/attribute string values is needed. Enabling keyword match improves recall of subsequent search requests. Defaults to false."]
        #[serde(
            rename = "disableKeywordMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_keyword_match: ::std::option::Option<bool>,
        #[doc = "Optional. Controls whether highly similar jobs are returned next to each other in the search results. Jobs are identified as highly similar based on their titles, job categories, and locations. Highly similar results are clustered so that only one representative job of the cluster is displayed to the job seeker higher up in the results, with the other jobs being displayed lower down in the results. Defaults to DiversificationLevel.SIMPLE if no value is specified."]
        #[serde(
            rename = "diversificationLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub diversification_level:
            ::std::option::Option<crate::schemas::SearchJobsRequestDiversificationLevel>,
        #[doc = "Optional. Controls whether to broaden the search when it produces sparse results. Broadened queries append results to the end of the matching results list. Defaults to false."]
        #[serde(
            rename = "enableBroadening",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_broadening: ::std::option::Option<bool>,
        #[doc = "Optional. Histogram requests for jobs matching JobQuery."]
        #[serde(
            rename = "histogramFacets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_facets: ::std::option::Option<crate::schemas::HistogramFacets>,
        #[doc = "Optional. Expression based histogram requests for jobs matching JobQuery."]
        #[serde(
            rename = "histogramQueries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_queries: ::std::option::Option<Vec<crate::schemas::HistogramQuery>>,
        #[doc = "Optional. Query used to search against jobs, such as keyword, location filters, etc."]
        #[serde(
            rename = "jobQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_query: ::std::option::Option<crate::schemas::JobQuery>,
        #[doc = "Optional. The desired job attributes returned for jobs in the search response. Defaults to JobView.SMALL if no value is specified."]
        #[serde(
            rename = "jobView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_view: ::std::option::Option<crate::schemas::SearchJobsRequestJobView>,
        #[doc = "Optional. An integer that specifies the current offset (that is, starting result location, amongst the jobs deemed by the API as relevant) in search results. This field is only considered if page_token is unset. The maximum allowed value is 5000. Otherwise an error is thrown. For example, 0 means to return results starting from the first matching job, and 10 means to return from the 11th job. This can be used for pagination, (for example, pageSize = 10 and offset = 10 means to return from the second page)."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset: ::std::option::Option<i32>,
        #[doc = "Optional. The criteria determining how search results are sorted. Default is \"relevance desc\". Supported options are: * `\"relevance desc\"`: By relevance descending, as determined by the API algorithms. Relevance thresholding of query results is only available with this ordering. * `\"posting_publish_time desc\"`: By Job.posting_publish_time descending. * `\"posting_update_time desc\"`: By Job.posting_update_time descending. * `\"title\"`: By Job.title ascending. * `\"title desc\"`: By Job.title descending. * `\"annualized_base_compensation\"`: By job's CompensationInfo.annualized_base_compensation_range ascending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"annualized_base_compensation desc\"`: By job's CompensationInfo.annualized_base_compensation_range descending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"annualized_total_compensation\"`: By job's CompensationInfo.annualized_total_compensation_range ascending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"annualized_total_compensation desc\"`: By job's CompensationInfo.annualized_total_compensation_range descending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"custom_ranking desc\"`: By the relevance score adjusted to the SearchJobsRequest.custom_ranking_info.ranking_expression with weight factor assigned by SearchJobsRequest.custom_ranking_info.importance_level in descending order. * Location sorting: Use the special syntax to order jobs by distance: \"`distance_from('Hawaii')`\": Order by distance from Hawaii. \"`distance_from(19.89, 155.5)`\": Order by distance from a coordinate. \"`distance_from('Hawaii'), distance_from('Puerto Rico')`\": Order by multiple locations. See details below. \"`distance_from('Hawaii'), distance_from(19.89, 155.5)`\": Order by multiple locations. See details below. The string can have a maximum of 256 characters. When multiple distance centers are provided, a job that is close to any of the distance centers would have a high rank. When a job has multiple locations, the job location closest to one of the distance centers will be used. Jobs that don't have locations will be ranked at the bottom. Distance is calculated with a precision of 11.3 meters (37.4 feet). Diversification strategy is still applied unless explicitly disabled in diversification_level."]
        #[serde(
            rename = "orderBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_by: ::std::option::Option<String>,
        #[doc = "Optional. A limit on the number of jobs returned in the search results. Increasing this value above the default value of 10 can increase search response time. The value can be between 1 and 100."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. The token specifying the current offset within search results. See SearchJobsResponse.next_page_token for an explanation of how to obtain the next set of query results."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Required. The meta information collected about the job searcher, used to improve the search quality of the service. The identifiers (such as `user_id`) are provided by users, and must be unique and consistent."]
        #[serde(
            rename = "requestMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_metadata: ::std::option::Option<crate::schemas::RequestMetadata>,
        #[doc = "This field is deprecated."]
        #[serde(
            rename = "requirePreciseResultSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_precise_result_size: ::std::option::Option<bool>,
        #[doc = "Optional. Mode of a search. Defaults to SearchMode.JOB_SEARCH."]
        #[serde(
            rename = "searchMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_mode: ::std::option::Option<crate::schemas::SearchJobsRequestSearchMode>,
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
    pub enum SearchJobsRequestDiversificationLevel {
        #[doc = "Disables diversification. Jobs that would normally be pushed to the last page would not have their positions altered. This may result in highly similar jobs appearing in sequence in the search results."]
        Disabled,
        #[doc = "The diversification level isn't specified. By default, jobs with this enum are ordered according to SIMPLE diversifying behavior."]
        DiversificationLevelUnspecified,
        #[doc = "Default diversifying behavior. The result list is ordered so that highly similar results are pushed to the end of the last page of search results."]
        Simple,
    }
    impl SearchJobsRequestDiversificationLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestDiversificationLevel::Disabled => "DISABLED",
                SearchJobsRequestDiversificationLevel::DiversificationLevelUnspecified => {
                    "DIVERSIFICATION_LEVEL_UNSPECIFIED"
                }
                SearchJobsRequestDiversificationLevel::Simple => "SIMPLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsRequestDiversificationLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsRequestDiversificationLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsRequestDiversificationLevel, ()> {
            Ok(match s {
                "DISABLED" => SearchJobsRequestDiversificationLevel::Disabled,
                "DIVERSIFICATION_LEVEL_UNSPECIFIED" => {
                    SearchJobsRequestDiversificationLevel::DiversificationLevelUnspecified
                }
                "SIMPLE" => SearchJobsRequestDiversificationLevel::Simple,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsRequestDiversificationLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsRequestDiversificationLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsRequestDiversificationLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISABLED" => SearchJobsRequestDiversificationLevel::Disabled,
                "DIVERSIFICATION_LEVEL_UNSPECIFIED" => {
                    SearchJobsRequestDiversificationLevel::DiversificationLevelUnspecified
                }
                "SIMPLE" => SearchJobsRequestDiversificationLevel::Simple,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequestDiversificationLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequestDiversificationLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestJobView {
        #[doc = "All available attributes are included in the search results."]
        JobViewFull,
        #[doc = "A ID only view of job, with following attributes: Job.name, Job.requisition_id, Job.language_code."]
        JobViewIdOnly,
        #[doc = "A minimal view of the job, with the following attributes: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.language_code."]
        JobViewMinimal,
        #[doc = "A small view of the job, with the following attributes in the search results: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.visibility, Job.language_code, Job.description."]
        JobViewSmall,
        #[doc = "Default value."]
        JobViewUnspecified,
    }
    impl SearchJobsRequestJobView {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestJobView::JobViewFull => "JOB_VIEW_FULL",
                SearchJobsRequestJobView::JobViewIdOnly => "JOB_VIEW_ID_ONLY",
                SearchJobsRequestJobView::JobViewMinimal => "JOB_VIEW_MINIMAL",
                SearchJobsRequestJobView::JobViewSmall => "JOB_VIEW_SMALL",
                SearchJobsRequestJobView::JobViewUnspecified => "JOB_VIEW_UNSPECIFIED",
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
                "JOB_VIEW_FULL" => SearchJobsRequestJobView::JobViewFull,
                "JOB_VIEW_ID_ONLY" => SearchJobsRequestJobView::JobViewIdOnly,
                "JOB_VIEW_MINIMAL" => SearchJobsRequestJobView::JobViewMinimal,
                "JOB_VIEW_SMALL" => SearchJobsRequestJobView::JobViewSmall,
                "JOB_VIEW_UNSPECIFIED" => SearchJobsRequestJobView::JobViewUnspecified,
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
                "JOB_VIEW_FULL" => SearchJobsRequestJobView::JobViewFull,
                "JOB_VIEW_ID_ONLY" => SearchJobsRequestJobView::JobViewIdOnly,
                "JOB_VIEW_MINIMAL" => SearchJobsRequestJobView::JobViewMinimal,
                "JOB_VIEW_SMALL" => SearchJobsRequestJobView::JobViewSmall,
                "JOB_VIEW_UNSPECIFIED" => SearchJobsRequestJobView::JobViewUnspecified,
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
    pub enum SearchJobsRequestSearchMode {
        #[doc = "The job search matches only against featured jobs (jobs with a promotionValue > 0). This method doesn't return any jobs having a promotionValue <= 0. The search results order is determined by the promotionValue (jobs with a higher promotionValue are returned higher up in the search results), with relevance being used as a tiebreaker."]
        FeaturedJobSearch,
        #[doc = "The job search matches against all jobs, and featured jobs (jobs with promotionValue > 0) are not specially handled."]
        JobSearch,
        #[doc = "The mode of the search method isn't specified. The default search behavior is identical to JOB_SEARCH search behavior."]
        SearchModeUnspecified,
    }
    impl SearchJobsRequestSearchMode {
        pub fn as_str(self) -> &'static str {
            match self {
                SearchJobsRequestSearchMode::FeaturedJobSearch => "FEATURED_JOB_SEARCH",
                SearchJobsRequestSearchMode::JobSearch => "JOB_SEARCH",
                SearchJobsRequestSearchMode::SearchModeUnspecified => "SEARCH_MODE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SearchJobsRequestSearchMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SearchJobsRequestSearchMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SearchJobsRequestSearchMode, ()> {
            Ok(match s {
                "FEATURED_JOB_SEARCH" => SearchJobsRequestSearchMode::FeaturedJobSearch,
                "JOB_SEARCH" => SearchJobsRequestSearchMode::JobSearch,
                "SEARCH_MODE_UNSPECIFIED" => SearchJobsRequestSearchMode::SearchModeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SearchJobsRequestSearchMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchJobsRequestSearchMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchJobsRequestSearchMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FEATURED_JOB_SEARCH" => SearchJobsRequestSearchMode::FeaturedJobSearch,
                "JOB_SEARCH" => SearchJobsRequestSearchMode::JobSearch,
                "SEARCH_MODE_UNSPECIFIED" => SearchJobsRequestSearchMode::SearchModeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SearchJobsRequestSearchMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchJobsRequestSearchMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchJobsResponse {
        #[doc = "If query broadening is enabled, we may append additional results from the broadened query. This number indicates how many of the jobs returned in the jobs field are from the broadened query. These results are always at the end of the jobs list. In particular, a value of 0, or if the field isn't set, all the jobs in the jobs list are from the original (without broadening) query. If this field is non-zero, subsequent requests with offset after this result set should contain all broadened results."]
        #[serde(
            rename = "broadenedQueryJobsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub broadened_query_jobs_count: ::std::option::Option<i32>,
        #[doc = "An estimation of the number of jobs that match the specified query. This number is not guaranteed to be accurate. For accurate results, see SearchJobsResponse.total_size."]
        #[serde(
            rename = "estimatedTotalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub estimated_total_size: ::std::option::Option<i32>,
        #[doc = "The histogram results that match with specified SearchJobsRequest.histogram_queries."]
        #[serde(
            rename = "histogramQueryResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_query_results:
            ::std::option::Option<Vec<crate::schemas::HistogramQueryResult>>,
        #[doc = "The histogram results that match specified SearchJobsRequest.histogram_facets."]
        #[serde(
            rename = "histogramResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_results: ::std::option::Option<crate::schemas::HistogramResults>,
        #[doc = "The location filters that the service applied to the specified query. If any filters are lat-lng based, the JobLocation.location_type is JobLocation.LocationType#LOCATION_TYPE_UNSPECIFIED."]
        #[serde(
            rename = "locationFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_filters: ::std::option::Option<Vec<crate::schemas::Location>>,
        #[doc = "The Job entities that match the specified SearchJobsRequest."]
        #[serde(
            rename = "matchingJobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matching_jobs: ::std::option::Option<Vec<crate::schemas::MatchingJob>>,
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "The token that specifies the starting position of the next page of results. This field is empty if there are no more results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The spell checking result, and correction."]
        #[serde(
            rename = "spellCorrection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spell_correction: ::std::option::Option<crate::schemas::SpellingCorrection>,
        #[doc = "The precise result count with limit 100,000."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
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
    pub struct TimeOfDay {
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        #[serde(
            rename = "hours",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hours: ::std::option::Option<i32>,
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        #[serde(
            rename = "minutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minutes: ::std::option::Option<i32>,
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        #[serde(
            rename = "seconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seconds: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TimeOfDay {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeOfDay {
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
    pub struct TimestampRange {
        #[doc = "End of the period."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Begin of the period."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimestampRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateCompanyRequest {
        #[doc = "Required. The company resource to replace the current resource in the system."]
        #[serde(
            rename = "company",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub company: ::std::option::Option<crate::schemas::Company>,
        #[doc = "Optional but strongly recommended for the best service experience. If update_mask is provided, only the specified fields in company are updated. Otherwise all the fields are updated. A field mask to specify the company fields to be updated. Only top level fields of Company are supported."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateCompanyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateCompanyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateJobRequest {
        #[doc = "Required. The Job to be updated."]
        #[serde(
            rename = "job",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "Optional but strongly recommended to be provided for the best service experience. If update_mask is provided, only the specified fields in job are updated. Otherwise all the fields are updated. A field mask to restrict the fields that are updated. Only top level fields of Job are supported."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
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
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum CompleteType {
                #[doc = "Suggest both job titles and company names."]
                Combined,
                #[doc = "Only suggest company names."]
                CompanyName,
                #[doc = "Default value."]
                CompletionTypeUnspecified,
                #[doc = "Only suggest job titles."]
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
                #[doc = "Default value."]
                CompletionScopeUnspecified,
                #[doc = "Suggestions are based on all jobs data in the system that's visible to the client"]
                Public,
                #[doc = "Suggestions are based only on the data provided by the client."]
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
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Completes the specified prefix with keyword suggestions. Intended for use by a job search auto-complete search box."]
            pub fn complete(&self, name: impl Into<String>) -> CompleteRequestBuilder {
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
                    name: name.into(),
                    company_name: None,
                    language_code: None,
                    language_codes: None,
                    page_size: None,
                    query: None,
                    r#type: None,
                    scope: None,
                }
            }
            #[doc = "Actions that can be performed on the client_events resource"]
            pub fn client_events(
                &self,
            ) -> crate::resources::projects::client_events::ClientEventsActions {
                crate::resources::projects::client_events::ClientEventsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the companies resource"]
            pub fn companies(&self) -> crate::resources::projects::companies::CompaniesActions {
                crate::resources::projects::companies::CompaniesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> crate::resources::projects::jobs::JobsActions {
                crate::resources::projects::jobs::JobsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(&self) -> crate::resources::projects::operations::OperationsActions {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::complete()](struct.ProjectsActions.html#method.complete)"]
        #[derive(Debug, Clone)]
        pub struct CompleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            company_name: Option<String>,
            language_code: Option<String>,
            language_codes: Option<Vec<String>>,
            page_size: Option<i32>,
            query: Option<String>,
            r#type: Option<crate::resources::projects::params::CompleteType>,
            scope: Option<crate::resources::projects::params::CompleteScope>,
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
            #[doc = "Optional. If provided, restricts completion to specified company. The format is \"projects/{project_id}/companies/{company_id}\", for example, \"projects/api-test-project/companies/foo\"."]
            pub fn company_name(mut self, value: impl Into<String>) -> Self {
                self.company_name = Some(value.into());
                self
            }
            #[doc = "Deprecated. Use language_codes instead. Optional. The language of the query. This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). For CompletionType.JOB_TITLE type, only open jobs with the same language_code are returned. For CompletionType.COMPANY_NAME type, only companies having open jobs with the same language_code are returned. For CompletionType.COMBINED type, only open jobs with the same language_code or companies having open jobs with the same language_code are returned. The maximum number of allowed characters is 255."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "Optional. The list of languages of the query. This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). For CompletionType.JOB_TITLE type, only open jobs with the same language_codes are returned. For CompletionType.COMPANY_NAME type, only companies having open jobs with the same language_codes are returned. For CompletionType.COMBINED type, only open jobs with the same language_codes or companies having open jobs with the same language_codes are returned. The maximum number of allowed characters is 255."]
            pub fn language_codes(mut self, value: impl Into<Vec<String>>) -> Self {
                self.language_codes = Some(value.into());
                self
            }
            #[doc = "Required. Completion result count. The maximum allowed page size is 10."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Required. The query used to generate suggestions. The maximum number of allowed characters is 255."]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
                self
            }
            #[doc = "Optional. The completion topic. The default is CompletionType.COMBINED."]
            pub fn r#type(
                mut self,
                value: crate::resources::projects::params::CompleteType,
            ) -> Self {
                self.r#type = Some(value);
                self
            }
            #[doc = "Optional. The scope of the completion. The defaults is CompletionScope.PUBLIC."]
            pub fn scope(
                mut self,
                value: crate::resources::projects::params::CompleteScope,
            ) -> Self {
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
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v3p1beta1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":complete");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("companyName", &self.company_name)]);
                req = req.query(&[("languageCode", &self.language_code)]);
                for value in self.language_codes.iter().flatten() {
                    req = req.query(&[("languageCodes", value)]);
                }
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("query", &self.query)]);
                req = req.query(&[("type", &self.r#type)]);
                req = req.query(&[("scope", &self.scope)]);
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
        pub mod client_events {
            pub mod params {}
            pub struct ClientEventsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ClientEventsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Report events issued when end user interacts with customer's application that uses Cloud Talent Solution. You may inspect the created events in [self service tools](https://console.cloud.google.com/talent-solution/overview). [Learn more](https://cloud.google.com/talent-solution/docs/management-tools) about self service tools."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateClientEventRequest,
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
            }
            #[doc = "Created via [ClientEventsActions::create()](struct.ClientEventsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CreateClientEventRequest,
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
                ) -> Result<crate::schemas::ClientEvent, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ClientEvent, crate::Error> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/clientEvents");
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
                pub fn create(
                    &self,
                    request: crate::schemas::CreateCompanyRequest,
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
                #[doc = "Deletes specified company. Prerequisite: The company has no jobs associated with it."]
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
                #[doc = "Retrieves specified company."]
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
                #[doc = "Lists all companies associated with the service account."]
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
                        require_open_jobs: None,
                    }
                }
                #[doc = "Updates specified company. Company names can't be updated. To update a company name, delete the company and all jobs associated with it, and only then re-create them."]
                pub fn patch(
                    &self,
                    request: crate::schemas::UpdateCompanyRequest,
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
            }
            #[doc = "Created via [CompaniesActions::create()](struct.CompaniesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CreateCompanyRequest,
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/companies");
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
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
            #[doc = "Created via [CompaniesActions::list()](struct.CompaniesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                require_open_jobs: Option<bool>,
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
                #[doc = "Optional. The maximum number of companies to be returned, at most 100. Default is 100 if a non-positive number is provided."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. The starting indicator from which to return results."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Optional. Set to true if the companies requested must have open jobs. Defaults to false. If true, at most page_size of companies are fetched, among which only those with open jobs are returned."]
                pub fn require_open_jobs(mut self, value: bool) -> Self {
                    self.require_open_jobs = Some(value);
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
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/companies");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("requireOpenJobs", &self.require_open_jobs)]);
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
            #[doc = "Created via [CompaniesActions::patch()](struct.CompaniesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UpdateCompanyRequest,
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
        pub mod jobs {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListJobView {
                    #[doc = "All available attributes are included in the search results."]
                    JobViewFull,
                    #[doc = "A ID only view of job, with following attributes: Job.name, Job.requisition_id, Job.language_code."]
                    JobViewIdOnly,
                    #[doc = "A minimal view of the job, with the following attributes: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.language_code."]
                    JobViewMinimal,
                    #[doc = "A small view of the job, with the following attributes in the search results: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.visibility, Job.language_code, Job.description."]
                    JobViewSmall,
                    #[doc = "Default value."]
                    JobViewUnspecified,
                }
                impl ListJobView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListJobView::JobViewFull => "JOB_VIEW_FULL",
                            ListJobView::JobViewIdOnly => "JOB_VIEW_ID_ONLY",
                            ListJobView::JobViewMinimal => "JOB_VIEW_MINIMAL",
                            ListJobView::JobViewSmall => "JOB_VIEW_SMALL",
                            ListJobView::JobViewUnspecified => "JOB_VIEW_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListJobView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListJobView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListJobView, ()> {
                        Ok(match s {
                            "JOB_VIEW_FULL" => ListJobView::JobViewFull,
                            "JOB_VIEW_ID_ONLY" => ListJobView::JobViewIdOnly,
                            "JOB_VIEW_MINIMAL" => ListJobView::JobViewMinimal,
                            "JOB_VIEW_SMALL" => ListJobView::JobViewSmall,
                            "JOB_VIEW_UNSPECIFIED" => ListJobView::JobViewUnspecified,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListJobView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListJobView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListJobView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "JOB_VIEW_FULL" => ListJobView::JobViewFull,
                            "JOB_VIEW_ID_ONLY" => ListJobView::JobViewIdOnly,
                            "JOB_VIEW_MINIMAL" => ListJobView::JobViewMinimal,
                            "JOB_VIEW_SMALL" => ListJobView::JobViewSmall,
                            "JOB_VIEW_UNSPECIFIED" => ListJobView::JobViewUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListJobView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListJobView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct JobsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> JobsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deletes a list of Jobs by filter."]
                pub fn batch_delete(
                    &self,
                    request: crate::schemas::BatchDeleteJobsRequest,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Creates a new job. Typically, the job becomes searchable within 10 seconds, but it may take up to 5 minutes."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateJobRequest,
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
                #[doc = "Deletes the specified job. Typically, the job becomes unsearchable within 10 seconds, but it may take up to 5 minutes."]
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
                #[doc = "Retrieves the specified job, whose status is OPEN or recently EXPIRED within the last 90 days."]
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
                #[doc = "Lists jobs by filter."]
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
                        job_view: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates specified job. Typically, updated contents become visible in search results within 10 seconds, but it may take up to 5 minutes."]
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
                #[doc = "Searches for jobs using the provided SearchJobsRequest. This call constrains the visibility of jobs present in the database, and only returns jobs that the caller has permission to search against."]
                pub fn search(
                    &self,
                    request: crate::schemas::SearchJobsRequest,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Searches for jobs using the provided SearchJobsRequest. This API call is intended for the use case of targeting passive job seekers (for example, job seekers who have signed up to receive email alerts about potential job opportunities), and has different algorithmic adjustments that are targeted to passive job seekers. This call constrains the visibility of jobs present in the database, and only returns jobs the caller has permission to search against."]
                pub fn search_for_alert(
                    &self,
                    request: crate::schemas::SearchJobsRequest,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
            }
            #[doc = "Created via [JobsActions::batch_delete()](struct.JobsActions.html#method.batch_delete)"]
            #[derive(Debug, Clone)]
            pub struct BatchDeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchDeleteJobsRequest,
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs:batchDelete");
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
            #[doc = "Created via [JobsActions::create()](struct.JobsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CreateJobRequest,
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
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
            #[doc = "Created via [JobsActions::delete()](struct.JobsActions.html#method.delete)"]
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
            #[doc = "Created via [JobsActions::list()](struct.JobsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                job_view: Option<crate::resources::projects::jobs::params::ListJobView>,
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
                #[doc = "Required. The filter string specifies the jobs to be enumerated. Supported operator: =, AND The fields eligible for filtering are: * `companyName` (Required) * `requisitionId` (Optional) Sample Query: * companyName = \"projects/api-test-project/companies/123\" * companyName = \"projects/api-test-project/companies/123\" AND requisitionId = \"req-1\""]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The desired job attributes returned for jobs in the search response. Defaults to JobView.JOB_VIEW_FULL if no value is specified."]
                pub fn job_view(
                    mut self,
                    value: crate::resources::projects::jobs::params::ListJobView,
                ) -> Self {
                    self.job_view = Some(value);
                    self
                }
                #[doc = "Optional. The maximum number of jobs to be returned per page of results. If job_view is set to JobView.JOB_VIEW_ID_ONLY, the maximum allowed page size is 1000. Otherwise, the maximum allowed page size is 100. Default is 100 if empty or a number < 1 is specified."]
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
                    let req = self._request(&self._path()).await?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("jobView", &self.job_view)]);
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
            #[doc = "Created via [JobsActions::search()](struct.JobsActions.html#method.search)"]
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SearchJobsRequest,
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs:search");
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
            #[doc = "Created via [JobsActions::search_for_alert()](struct.JobsActions.html#method.search_for_alert)"]
            #[derive(Debug, Clone)]
            pub struct SearchForAlertRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SearchJobsRequest,
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
                    let req = self._request(&self._path()).await?;
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs:searchForAlert");
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
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OperationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
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
            }
            #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3p1beta1/");
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
