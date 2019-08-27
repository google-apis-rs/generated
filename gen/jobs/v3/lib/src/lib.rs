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
        #[doc = "Optional but at least one of uris,\nemails or instruction must be\nspecified.\n\nUse this field to specify email address(es) to which resumes or\napplications can be sent.\n\nThe maximum number of allowed characters for each entry is 255."]
        #[serde(rename = "emails", default)]
        pub emails: ::std::option::Option<Vec<String>>,
        #[doc = "Optional but at least one of uris,\nemails or instruction must be\nspecified.\n\nUse this field to provide instructions, such as \"Mail your application\nto ...\", that a candidate can follow to apply for the job.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 3,000."]
        #[serde(rename = "instruction", default)]
        pub instruction: ::std::option::Option<String>,
        #[doc = "Optional but at least one of uris,\nemails or instruction must be\nspecified.\n\nUse this URI field to direct an applicant to a website, for example to\nlink to an online application form.\n\nThe maximum number of allowed characters for each entry is 2,000."]
        #[serde(rename = "uris", default)]
        pub uris: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ApplicationInfo {
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
    pub struct BatchDeleteJobsRequest {
        #[doc = "Required. The filter string specifies the jobs to be deleted.\n\nSupported operator: =, AND\n\nThe fields eligible for filtering are:\n\n* `companyName` (Required)\n* `requisitionId` (Required)\n\nSample Query: companyName = \"projects/api-test-project/companies/123\" AND\nrequisitionId = \"req-1\""]
        #[serde(rename = "filter", default)]
        pub filter: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BatchDeleteJobsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BucketRange {
        #[doc = "Starting value of the bucket range."]
        #[serde(rename = "from", default)]
        pub from: ::std::option::Option<f64>,
        #[doc = "Ending value of the bucket range."]
        #[serde(rename = "to", default)]
        pub to: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for BucketRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BucketizedCount {
        #[doc = "Number of jobs whose numeric field value fall into `range`."]
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<i32>,
        #[doc = "Bucket range on which histogram was performed for the numeric field,\nthat is, the count represents number of jobs in this range."]
        #[serde(rename = "range", default)]
        pub range: ::std::option::Option<crate::schemas::BucketRange>,
    }
    impl ::field_selector::FieldSelector for BucketizedCount {
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
    pub struct ClientEvent {
        #[doc = "Required. The timestamp of the event."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. A unique identifier, generated by the client application. This `event_id`\nis used to establish the relationship between different events\n(see parent_event_id)."]
        #[serde(rename = "eventId", default)]
        pub event_id: ::std::option::Option<String>,
        #[doc = "Optional. Extra information about this event. Used for storing information with no\nmatching field in event payload, for example, user application specific\ncontext or details.\n\nAt most 20 keys are supported. The maximum total size of all keys and\nvalues is 2 KB."]
        #[serde(rename = "extraInfo", default)]
        pub extra_info: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A event issued when a job seeker interacts with the application that\nimplements Cloud Talent Solution."]
        #[serde(rename = "jobEvent", default)]
        pub job_event: ::std::option::Option<crate::schemas::JobEvent>,
        #[doc = "Optional. The event_id of an event that resulted in the current event. For example, a\nJob view event usually follows a parent\nimpression event: A job seeker first does a\nsearch where a list of jobs appears\n(impression). The job seeker then selects a\nresult and views the description of a particular job (Job\nview)."]
        #[serde(rename = "parentEventId", default)]
        pub parent_event_id: ::std::option::Option<String>,
        #[doc = "Required. A unique ID generated in the API responses. It can be found in\nResponseMetadata.request_id."]
        #[serde(rename = "requestId", default)]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ClientEvent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommuteFilter {
        #[doc = "Optional. If true, jobs without \"precise\" addresses (street level addresses or GPS\ncoordinates) might also be returned. For city and coarser level addresses,\ntext matching is used. If this field is set to false or is not specified,\nonly jobs that include precise addresses are returned by Commute\nSearch.\n\nNote: If `allow_imprecise_addresses` is set to true, Commute Search is not\nable to calculate accurate commute times to jobs with city level and\ncoarser address information. Jobs with imprecise addresses will return a\n`travel_duration` time of 0 regardless of distance from the job seeker."]
        #[serde(rename = "allowImpreciseAddresses", default)]
        pub allow_imprecise_addresses: ::std::option::Option<bool>,
        #[doc = "Required. The method of transportation for which to calculate the commute time."]
        #[serde(rename = "commuteMethod", default)]
        pub commute_method: ::std::option::Option<crate::schemas::CommuteFilterCommuteMethod>,
        #[doc = "Optional. The departure time used to calculate traffic impact, represented as\ngoogle.type.TimeOfDay in local time zone.\n\nCurrently traffic model is restricted to hour level resolution."]
        #[serde(rename = "departureTime", default)]
        pub departure_time: ::std::option::Option<crate::schemas::TimeOfDay>,
        #[doc = "Optional. Specifies the traffic density to use when calculating commute time."]
        #[serde(rename = "roadTraffic", default)]
        pub road_traffic: ::std::option::Option<crate::schemas::CommuteFilterRoadTraffic>,
        #[doc = "Required. The latitude and longitude of the location from which to calculate the\ncommute time."]
        #[serde(rename = "startCoordinates", default)]
        pub start_coordinates: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Required. The maximum travel time in seconds. The maximum allowed value is `3600s`\n(one hour). Format is `123s`."]
        #[serde(rename = "travelDuration", default)]
        pub travel_duration: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommuteFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommuteFilterCommuteMethod {
        #[doc = "Commute method is not specified."]
        CommuteMethodUnspecified,
        #[doc = "Commute time is calculated based on driving time."]
        Driving,
        #[doc = "Commute time is calculated based on public transit including bus, metro,\nsubway, etc."]
        Transit,
    }
    impl CommuteFilterCommuteMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                CommuteFilterCommuteMethod::CommuteMethodUnspecified => {
                    "COMMUTE_METHOD_UNSPECIFIED"
                }
                CommuteFilterCommuteMethod::Driving => "DRIVING",
                CommuteFilterCommuteMethod::Transit => "TRANSIT",
            }
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
                "DRIVING" => CommuteFilterCommuteMethod::Driving,
                "TRANSIT" => CommuteFilterCommuteMethod::Transit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for CommuteFilterCommuteMethod {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CommuteFilterRoadTraffic {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommuteInfo {
        #[doc = "Location used as the destination in the commute calculation."]
        #[serde(rename = "jobLocation", default)]
        pub job_location: ::std::option::Option<crate::schemas::Location>,
        #[doc = "The number of seconds required to travel to the job location from the\nquery location. A duration of 0 seconds indicates that the job is not\nreachable within the requested duration, but was returned as part of an\nexpanded query."]
        #[serde(rename = "travelDuration", default)]
        pub travel_duration: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommuteInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Company {
        #[doc = "Optional. The URI to employer's career site or careers page on the employer's web\nsite, for example, \"https://careers.google.com\"."]
        #[serde(rename = "careerSiteUri", default)]
        pub career_site_uri: ::std::option::Option<String>,
        #[doc = "Output only. Derived details about the company."]
        #[serde(rename = "derivedInfo", default)]
        pub derived_info: ::std::option::Option<crate::schemas::CompanyDerivedInfo>,
        #[doc = "Required. The display name of the company, for example, \"Google LLC\"."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Equal Employment Opportunity legal disclaimer text to be\nassociated with all jobs, and typically to be displayed in all\nroles.\n\nThe maximum number of allowed characters is 500."]
        #[serde(rename = "eeoText", default)]
        pub eeo_text: ::std::option::Option<String>,
        #[doc = "Required. Client side company identifier, used to uniquely identify the\ncompany.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "externalId", default)]
        pub external_id: ::std::option::Option<String>,
        #[doc = "Optional. The street address of the company's main headquarters, which may be\ndifferent from the job location. The service attempts\nto geolocate the provided address, and populates a more specific\nlocation wherever possible in DerivedInfo.headquarters_location."]
        #[serde(rename = "headquartersAddress", default)]
        pub headquarters_address: ::std::option::Option<String>,
        #[doc = "Optional. Set to true if it is the hiring agency that post jobs for other\nemployers.\n\nDefaults to false if not provided."]
        #[serde(rename = "hiringAgency", default)]
        pub hiring_agency: ::std::option::Option<bool>,
        #[doc = "Optional. A URI that hosts the employer's company logo."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "Optional. A list of keys of filterable Job.custom_attributes, whose\ncorresponding `string_values` are used in keyword search. Jobs with\n`string_values` under these specified field keys are returned if any\nof the values matches the search keyword. Custom field values with\nparenthesis, brackets and special symbols won't be properly searchable,\nand those keyword queries need to be surrounded by quotes."]
        #[serde(rename = "keywordSearchableJobCustomAttributes", default)]
        pub keyword_searchable_job_custom_attributes: ::std::option::Option<Vec<String>>,
        #[doc = "Required during company update.\n\nThe resource name for a company. This is generated by the service when a\ncompany is created.\n\nThe format is \"projects/{project_id}/companies/{company_id}\", for example,\n\"projects/api-test-project/companies/foo\"."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The employer's company size."]
        #[serde(rename = "size", default)]
        pub size: ::std::option::Option<crate::schemas::CompanySize>,
        #[doc = "Output only. Indicates whether a company is flagged to be suspended from\npublic availability by the service when job content appears suspicious,\nabusive, or spammy."]
        #[serde(rename = "suspended", default)]
        pub suspended: ::std::option::Option<bool>,
        #[doc = "Optional. The URI representing the company's primary web site or home page,\nfor example, \"https://www.google.com\".\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "websiteUri", default)]
        pub website_uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Company {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompanySize {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompanyDerivedInfo {
        #[doc = "A structured headquarters location of the company, resolved from\nCompany.hq_location if provided."]
        #[serde(rename = "headquartersLocation", default)]
        pub headquarters_location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::field_selector::FieldSelector for CompanyDerivedInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationEntry {
        #[doc = "Optional. Compensation amount."]
        #[serde(rename = "amount", default)]
        pub amount: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Optional. Compensation description.  For example, could\nindicate equity terms or provide additional context to an estimated\nbonus."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. Expected number of units paid each year. If not specified, when\nJob.employment_types is FULLTIME, a default value is inferred\nbased on unit. Default values:\n\n* HOURLY: 2080\n* DAILY: 260\n* WEEKLY: 52\n* MONTHLY: 12\n* ANNUAL: 1"]
        #[serde(rename = "expectedUnitsPerYear", default)]
        pub expected_units_per_year: ::std::option::Option<f64>,
        #[doc = "Optional. Compensation type.\n\nDefault is CompensationUnit.COMPENSATION_TYPE_UNSPECIFIED."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::CompensationEntryType>,
        #[doc = "Optional. Compensation range."]
        #[serde(rename = "range", default)]
        pub range: ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Optional. Frequency of the specified amount.\n\nDefault is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED."]
        #[serde(rename = "unit", default)]
        pub unit: ::std::option::Option<crate::schemas::CompensationEntryUnit>,
    }
    impl ::field_selector::FieldSelector for CompensationEntry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompensationEntryType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompensationEntryUnit {
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
    pub struct CompensationFilter {
        #[doc = "Optional. If set to true, jobs with unspecified compensation range fields are\nincluded."]
        #[serde(rename = "includeJobsWithUnspecifiedCompensationRange", default)]
        pub include_jobs_with_unspecified_compensation_range: ::std::option::Option<bool>,
        #[doc = "Required. Type of filter."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::CompensationFilterType>,
        #[doc = "Optional. Compensation range."]
        #[serde(rename = "range", default)]
        pub range: ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Required. Specify desired `base compensation entry's`\nCompensationInfo.CompensationUnit."]
        #[serde(rename = "units", default)]
        pub units: ::std::option::Option<Vec<crate::schemas::CompensationFilterUnitsItems>>,
    }
    impl ::field_selector::FieldSelector for CompensationFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompensationFilterType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompensationFilterUnitsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationHistogramRequest {
        #[doc = "Required. Numeric histogram options, like buckets, whether include min or max value."]
        #[serde(rename = "bucketingOption", default)]
        pub bucketing_option: ::std::option::Option<crate::schemas::NumericBucketingOption>,
        #[doc = "Required. Type of the request, representing which field the histogramming should be\nperformed over. A single request can only specify one histogram of each\n`CompensationHistogramRequestType`."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::CompensationHistogramRequestType>,
    }
    impl ::field_selector::FieldSelector for CompensationHistogramRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompensationHistogramRequestType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationHistogramResult {
        #[doc = "Type of the request, corresponding to\nCompensationHistogramRequest.type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::CompensationHistogramResultType>,
        #[doc = "Histogram result."]
        #[serde(rename = "result", default)]
        pub result: ::std::option::Option<crate::schemas::NumericBucketingResult>,
    }
    impl ::field_selector::FieldSelector for CompensationHistogramResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompensationHistogramResultType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompensationInfo {
        #[doc = "Output only. Annualized base compensation range. Computed as\nbase compensation entry's CompensationEntry.compensation times\nCompensationEntry.expected_units_per_year.\n\nSee CompensationEntry for explanation on compensation annualization."]
        #[serde(rename = "annualizedBaseCompensationRange", default)]
        pub annualized_base_compensation_range:
            ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Output only. Annualized total compensation range. Computed as\nall compensation entries' CompensationEntry.compensation times\nCompensationEntry.expected_units_per_year.\n\nSee CompensationEntry for explanation on compensation annualization."]
        #[serde(rename = "annualizedTotalCompensationRange", default)]
        pub annualized_total_compensation_range:
            ::std::option::Option<crate::schemas::CompensationRange>,
        #[doc = "Optional. Job compensation information.\n\nAt most one entry can be of type\nCompensationInfo.CompensationType.BASE, which is\nreferred as ** base compensation entry ** for the job."]
        #[serde(rename = "entries", default)]
        pub entries: ::std::option::Option<Vec<crate::schemas::CompensationEntry>>,
    }
    impl ::field_selector::FieldSelector for CompensationInfo {
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
    pub struct CompensationRange {
        #[doc = "Optional. The maximum amount of compensation. If left empty, the value is set\nto a maximal compensation value and the currency code is set to\nmatch the currency code of\nmin_compensation."]
        #[serde(rename = "maxCompensation", default)]
        pub max_compensation: ::std::option::Option<crate::schemas::Money>,
        #[doc = "Optional. The minimum amount of compensation. If left empty, the value is set\nto zero and the currency code is set to match the\ncurrency code of max_compensation."]
        #[serde(rename = "minCompensation", default)]
        pub min_compensation: ::std::option::Option<crate::schemas::Money>,
    }
    impl ::field_selector::FieldSelector for CompensationRange {
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
    pub struct CompleteQueryResponse {
        #[doc = "Results of the matching job/company candidates."]
        #[serde(rename = "completionResults", default)]
        pub completion_results: ::std::option::Option<Vec<crate::schemas::CompletionResult>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
    }
    impl ::field_selector::FieldSelector for CompleteQueryResponse {
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
    pub struct CompletionResult {
        #[doc = "The URI of the company image for CompletionType.COMPANY_NAME."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "The completion topic."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::CompletionResultType>,
        #[doc = "The suggestion for the query."]
        #[serde(rename = "suggestion", default)]
        pub suggestion: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CompletionResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for CompletionResultType {
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
    pub struct CreateClientEventRequest {
        #[doc = "Required. Events issued when end user interacts with customer's application that\nuses Cloud Talent Solution."]
        #[serde(rename = "clientEvent", default)]
        pub client_event: ::std::option::Option<crate::schemas::ClientEvent>,
    }
    impl ::field_selector::FieldSelector for CreateClientEventRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateCompanyRequest {
        #[doc = "Required. The company to be created."]
        #[serde(rename = "company", default)]
        pub company: ::std::option::Option<crate::schemas::Company>,
    }
    impl ::field_selector::FieldSelector for CreateCompanyRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateJobRequest {
        #[doc = "Required. The Job to be created."]
        #[serde(rename = "job", default)]
        pub job: ::std::option::Option<crate::schemas::Job>,
    }
    impl ::field_selector::FieldSelector for CreateJobRequest {
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
    pub struct CustomAttribute {
        #[doc = "Optional. If the `filterable` flag is true, custom field values are searchable.\nIf false, values are not searchable.\n\nDefault is false."]
        #[serde(rename = "filterable", default)]
        pub filterable: ::std::option::Option<bool>,
        #[doc = "Optional but exactly one of string_values or long_values must\nbe specified.\n\nThis field is used to perform number range search.\n(`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`.\n\nCurrently at most 1 long_values is supported."]
        #[serde(rename = "longValues", default)]
        pub long_values: ::std::option::Option<Vec<i64>>,
        #[doc = "Optional but exactly one of string_values or long_values must\nbe specified.\n\nThis field is used to perform a string match (`CASE_SENSITIVE_MATCH` or\n`CASE_INSENSITIVE_MATCH`) search.\nFor filterable `string_value`s, a maximum total number of 200 values\nis allowed, with each `string_value` has a byte size of no more than\n500B. For unfilterable `string_values`, the maximum total byte size of\nunfilterable `string_values` is 50KB.\n\nEmpty string is not allowed."]
        #[serde(rename = "stringValues", default)]
        pub string_values: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for CustomAttribute {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomAttributeHistogramRequest {
        #[doc = "Required. Specifies the custom field key to perform a histogram on. If specified\nwithout `long_value_histogram_bucketing_option`, histogram on string values\nof the given `key` is triggered, otherwise histogram is performed on long\nvalues."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Optional. Specifies buckets used to perform a range histogram on Job's\nfilterable long custom field values, or min/max value requirements."]
        #[serde(rename = "longValueHistogramBucketingOption", default)]
        pub long_value_histogram_bucketing_option:
            ::std::option::Option<crate::schemas::NumericBucketingOption>,
        #[doc = "Optional. If set to true, the response includes the histogram value for\neach key as a string."]
        #[serde(rename = "stringValueHistogram", default)]
        pub string_value_histogram: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for CustomAttributeHistogramRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomAttributeHistogramResult {
        #[doc = "Stores the key of custom attribute the histogram is performed on."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Stores bucketed histogram counting result or min/max values for\ncustom attribute long values associated with `key`."]
        #[serde(rename = "longValueHistogramResult", default)]
        pub long_value_histogram_result:
            ::std::option::Option<crate::schemas::NumericBucketingResult>,
        #[doc = "Stores a map from the values of string custom field associated\nwith `key` to the number of jobs with that value in this histogram result."]
        #[serde(rename = "stringValueHistogramResult", default)]
        pub string_value_histogram_result:
            ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
    }
    impl ::field_selector::FieldSelector for CustomAttributeHistogramResult {
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
    pub struct DeviceInfo {
        #[doc = "Optional. Type of the device."]
        #[serde(rename = "deviceType", default)]
        pub device_type: ::std::option::Option<crate::schemas::DeviceInfoDeviceType>,
        #[doc = "Optional. A device-specific ID. The ID must be a unique identifier that\ndistinguishes the device from other devices."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DeviceInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for DeviceInfoDeviceType {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramFacets {
        #[doc = "Optional. Specifies compensation field-based histogram requests.\nDuplicate values of CompensationHistogramRequest.type are not allowed."]
        #[serde(rename = "compensationHistogramFacets", default)]
        pub compensation_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::CompensationHistogramRequest>>,
        #[doc = "Optional. Specifies the custom attributes histogram requests.\nDuplicate values of CustomAttributeHistogramRequest.key are not\nallowed."]
        #[serde(rename = "customAttributeHistogramFacets", default)]
        pub custom_attribute_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::CustomAttributeHistogramRequest>>,
        #[doc = "Optional. Specifies the simple type of histogram facets, for example,\n`COMPANY_SIZE`, `EMPLOYMENT_TYPE` etc."]
        #[serde(rename = "simpleHistogramFacets", default)]
        pub simple_histogram_facets:
            ::std::option::Option<Vec<crate::schemas::HistogramFacetsSimpleHistogramFacetsItems>>,
    }
    impl ::field_selector::FieldSelector for HistogramFacets {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        Country,
        DatePublished,
        EducationLevel,
        EmploymentType,
        ExperienceLevel,
        Language,
        Locale,
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
    impl ::field_selector::FieldSelector for HistogramFacetsSimpleHistogramFacetsItems {
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
    pub struct HistogramResult {
        #[doc = "The Histogram search filters."]
        #[serde(rename = "searchType", default)]
        pub search_type: ::std::option::Option<crate::schemas::HistogramResultSearchType>,
        #[doc = "A map from the values of field to the number of jobs with that value\nin this search result.\n\nKey: search type (filter names, such as the companyName).\n\nValues: the count of jobs that match the filter for this search."]
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
    }
    impl ::field_selector::FieldSelector for HistogramResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        #[doc = "Filter by the country code of job, such as US, JP, FR."]
        Country,
        #[doc = "Filter by the date published field. Possible return values are:\n\n* PAST_24_HOURS (The past 24 hours)\n* PAST_3_DAYS (The past 3 days)\n* PAST_WEEK (The past 7 days)\n* PAST_MONTH (The past 30 days)\n* PAST_YEAR (The past 365 days)"]
        DatePublished,
        #[doc = "Filter by the required education level of the job."]
        EducationLevel,
        #[doc = "Filter by the employment type field, such as `FULL_TIME` or `PART_TIME`."]
        EmploymentType,
        #[doc = "Filter by the required experience level of the job."]
        ExperienceLevel,
        #[doc = "Filter by the language code portion of the locale field, such as \"en\" or\n\"fr\"."]
        Language,
        #[doc = "Filter by the locale field of a job, such as \"en-US\", \"fr-FR\".\n\nThis is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
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
    impl ::field_selector::FieldSelector for HistogramResultSearchType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramResults {
        #[doc = "Specifies compensation field-based histogram results that match\nHistogramFacets.compensation_histogram_requests."]
        #[serde(rename = "compensationHistogramResults", default)]
        pub compensation_histogram_results:
            ::std::option::Option<Vec<crate::schemas::CompensationHistogramResult>>,
        #[doc = "Specifies histogram results for custom attributes that match\nHistogramFacets.custom_attribute_histogram_facets."]
        #[serde(rename = "customAttributeHistogramResults", default)]
        pub custom_attribute_histogram_results:
            ::std::option::Option<Vec<crate::schemas::CustomAttributeHistogramResult>>,
        #[doc = "Specifies histogram results that matches\nHistogramFacets.simple_histogram_facets."]
        #[serde(rename = "simpleHistogramResults", default)]
        pub simple_histogram_results: ::std::option::Option<Vec<crate::schemas::HistogramResult>>,
    }
    impl ::field_selector::FieldSelector for HistogramResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Job {
        #[doc = "Optional but strongly recommended for the best service experience.\n\nLocation(s) where the employer is looking to hire for this job posting.\n\nSpecifying the full street address(es) of the hiring location enables\nbetter API results, especially job searches by commute time.\n\nAt most 50 locations are allowed for best search performance. If a job has\nmore locations, it is suggested to split it into multiple jobs with unique\nrequisition_ids (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', etc.) as\nmultiple jobs with the same company_name, language_code and\nrequisition_id are not allowed. If the original requisition_id must\nbe preserved, a custom field should be used for storage. It is also\nsuggested to group the locations that close to each other in the same job\nfor better search experience.\n\nThe maximum number of allowed characters is 500."]
        #[serde(rename = "addresses", default)]
        pub addresses: ::std::option::Option<Vec<String>>,
        #[doc = "Required. At least one field within ApplicationInfo must be specified.\n\nJob application information."]
        #[serde(rename = "applicationInfo", default)]
        pub application_info: ::std::option::Option<crate::schemas::ApplicationInfo>,
        #[doc = "Output only. Display name of the company listing the job."]
        #[serde(rename = "companyDisplayName", default)]
        pub company_display_name: ::std::option::Option<String>,
        #[doc = "Required. The resource name of the company listing the job, such as\n\"projects/api-test-project/companies/foo\"."]
        #[serde(rename = "companyName", default)]
        pub company_name: ::std::option::Option<String>,
        #[doc = "Optional. Job compensation information."]
        #[serde(rename = "compensationInfo", default)]
        pub compensation_info: ::std::option::Option<crate::schemas::CompensationInfo>,
        #[doc = "Optional. A map of fields to hold both filterable and non-filterable custom job\nattributes that are not covered by the provided structured fields.\n\nThe keys of the map are strings up to 64 bytes and must match the\npattern: a-zA-Z*. For example, key0LikeThis or\nKEY_1_LIKE_THIS.\n\nAt most 100 filterable and at most 100 unfilterable keys are supported.\nFor filterable `string_values`, across all keys at most 200 values are\nallowed, with each string no more than 255 characters. For unfilterable\n`string_values`, the maximum total size of `string_values` across all keys\nis 50KB."]
        #[serde(rename = "customAttributes", default)]
        pub custom_attributes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CustomAttribute>,
        >,
        #[doc = "Optional. The desired education degrees for the job, such as Bachelors, Masters."]
        #[serde(rename = "degreeTypes", default)]
        pub degree_types: ::std::option::Option<Vec<crate::schemas::JobDegreeTypesItems>>,
        #[doc = "Optional. The department or functional area within the company with the open\nposition.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "department", default)]
        pub department: ::std::option::Option<String>,
        #[doc = "Output only. Derived details about the job posting."]
        #[serde(rename = "derivedInfo", default)]
        pub derived_info: ::std::option::Option<crate::schemas::JobDerivedInfo>,
        #[doc = "Required. The description of the job, which typically includes a multi-paragraph\ndescription of the company and related information. Separate fields are\nprovided on the job object for responsibilities,\nqualifications, and other job characteristics. Use of\nthese separate job fields is recommended.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 100,000."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. The employment type(s) of a job, for example,\nfull time or\npart time."]
        #[serde(rename = "employmentTypes", default)]
        pub employment_types: ::std::option::Option<Vec<crate::schemas::JobEmploymentTypesItems>>,
        #[doc = "Optional. A description of bonus, commission, and other compensation\nincentives associated with the job not including salary or pay.\n\nThe maximum number of allowed characters is 10,000."]
        #[serde(rename = "incentives", default)]
        pub incentives: ::std::option::Option<String>,
        #[doc = "Optional. The benefits included with the job."]
        #[serde(rename = "jobBenefits", default)]
        pub job_benefits: ::std::option::Option<Vec<crate::schemas::JobJobBenefitsItems>>,
        #[doc = "Optional. The end timestamp of the job. Typically this field is used for contracting\nengagements. Invalid timestamps are ignored."]
        #[serde(rename = "jobEndTime", default)]
        pub job_end_time: ::std::option::Option<String>,
        #[doc = "Optional. The experience level associated with the job, such as \"Entry Level\"."]
        #[serde(rename = "jobLevel", default)]
        pub job_level: ::std::option::Option<crate::schemas::JobJobLevel>,
        #[doc = "Optional. The start timestamp of the job in UTC time zone. Typically this field\nis used for contracting engagements. Invalid timestamps are ignored."]
        #[serde(rename = "jobStartTime", default)]
        pub job_start_time: ::std::option::Option<String>,
        #[doc = "Optional. The language of the posting. This field is distinct from\nany requirements for fluency that are associated with the job.\n\nLanguage codes must be in BCP-47 format, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){:\nclass=\"external\" target=\"_blank\" }.\n\nIf this field is unspecified and Job.description is present, detected\nlanguage code based on Job.description is assigned, otherwise\ndefaults to 'en_US'."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Required during job update.\n\nThe resource name for the job. This is generated by the service when a\njob is created.\n\nThe format is \"projects/{project_id}/jobs/{job_id}\",\nfor example, \"projects/api-test-project/jobs/1234\".\n\nUse of this field in job queries and API calls is preferred over the use of\nrequisition_id since this value is unique."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp when this job posting was created."]
        #[serde(rename = "postingCreateTime", default)]
        pub posting_create_time: ::std::option::Option<String>,
        #[doc = "Optional but strongly recommended for the best service\nexperience.\n\nThe expiration timestamp of the job. After this timestamp, the\njob is marked as expired, and it no longer appears in search results. The\nexpired job can't be deleted or listed by the DeleteJob and\nListJobs APIs, but it can be retrieved with the GetJob API or\nupdated with the UpdateJob API. An expired job can be updated and\nopened again by using a future expiration timestamp. Updating an expired\njob fails if there is another existing open job with same company_name,\nlanguage_code and requisition_id.\n\nThe expired jobs are retained in our system for 90 days. However, the\noverall expired job count cannot exceed 3 times the maximum of open jobs\ncount over the past week, otherwise jobs with earlier expire time are\ncleaned first. Expired jobs are no longer accessible after they are cleaned\nout.\n\nInvalid timestamps are ignored, and treated as expire time not provided.\n\nTimestamp before the instant request is made is considered valid, the job\nwill be treated as expired immediately.\n\nIf this value is not provided at the time of job creation or is invalid,\nthe job posting expires after 30 days from the job's creation time. For\nexample, if the job was created on 2017/01/01 13:00AM UTC with an\nunspecified expiration date, the job expires after 2017/01/31 13:00AM UTC.\n\nIf this value is not provided on job update, it depends on the field masks\nset by UpdateJobRequest.update_mask. If the field masks include\nexpiry_time, or the masks are empty meaning that every field is\nupdated, the job posting expires after 30 days from the job's last\nupdate time. Otherwise the expiration date isn't updated."]
        #[serde(rename = "postingExpireTime", default)]
        pub posting_expire_time: ::std::option::Option<String>,
        #[doc = "Optional. The timestamp this job posting was most recently published. The default\nvalue is the time the request arrives at the server. Invalid timestamps are\nignored."]
        #[serde(rename = "postingPublishTime", default)]
        pub posting_publish_time: ::std::option::Option<String>,
        #[doc = "Optional. The job PostingRegion (for example, state, country) throughout which\nthe job is available. If this field is set, a\nLocationFilter in a search query within the job region\nfinds this job posting if an exact location match isn't specified.\nIf this field is set to PostingRegion.NATION or\nPostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses\nto the same location level as this field is strongly recommended."]
        #[serde(rename = "postingRegion", default)]
        pub posting_region: ::std::option::Option<crate::schemas::JobPostingRegion>,
        #[doc = "Output only. The timestamp when this job posting was last updated."]
        #[serde(rename = "postingUpdateTime", default)]
        pub posting_update_time: ::std::option::Option<String>,
        #[doc = "Optional. Options for job processing."]
        #[serde(rename = "processingOptions", default)]
        pub processing_options: ::std::option::Option<crate::schemas::ProcessingOptions>,
        #[doc = "Optional. A promotion value of the job, as determined by the client.\nThe value determines the sort order of the jobs returned when searching for\njobs using the featured jobs search call, with higher promotional values\nbeing returned first and ties being resolved by relevance sort. Only the\njobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH.\n\nDefault value is 0, and negative values are treated as 0."]
        #[serde(rename = "promotionValue", default)]
        pub promotion_value: ::std::option::Option<i32>,
        #[doc = "Optional. A description of the qualifications required to perform the\njob. The use of this field is recommended\nas an alternative to using the more general description field.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 10,000."]
        #[serde(rename = "qualifications", default)]
        pub qualifications: ::std::option::Option<String>,
        #[doc = "Required. The requisition ID, also referred to as the posting ID, assigned by the\nclient to identify a job. This field is intended to be used by clients\nfor client identification and tracking of postings. A job is not allowed\nto be created if there is another job with the same [company_name],\nlanguage_code and requisition_id.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "requisitionId", default)]
        pub requisition_id: ::std::option::Option<String>,
        #[doc = "Optional. A description of job responsibilities. The use of this field is\nrecommended as an alternative to using the more general description\nfield.\n\nThis field accepts and sanitizes HTML input, and also accepts\nbold, italic, ordered list, and unordered list markup tags.\n\nThe maximum number of allowed characters is 10,000."]
        #[serde(rename = "responsibilities", default)]
        pub responsibilities: ::std::option::Option<String>,
        #[doc = "Required. The title of the job, such as \"Software Engineer\"\n\nThe maximum number of allowed characters is 500."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "Optional. The visibility of the job.\n\nDefaults to Visibility.ACCOUNT_ONLY if not specified."]
        #[serde(rename = "visibility", default)]
        pub visibility: ::std::option::Option<crate::schemas::JobVisibility>,
    }
    impl ::field_selector::FieldSelector for Job {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobDegreeTypesItems {
        AdultRemedialEducation,
        AssociatesOrEquivalent,
        BachelorsOrEquivalent,
        DegreeTypeUnspecified,
        DoctoralOrEquivalent,
        LowerSecondaryEducation,
        MastersOrEquivalent,
        PrimaryEducation,
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
    impl ::field_selector::FieldSelector for JobDegreeTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        OtherEmploymentType,
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
                JobEmploymentTypesItems::OtherEmploymentType => "OTHER_EMPLOYMENT_TYPE",
                JobEmploymentTypesItems::PartTime => "PART_TIME",
                JobEmploymentTypesItems::PerDiem => "PER_DIEM",
                JobEmploymentTypesItems::Temporary => "TEMPORARY",
                JobEmploymentTypesItems::Volunteer => "VOLUNTEER",
            }
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
    impl ::field_selector::FieldSelector for JobEmploymentTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobJobBenefitsItems {
        ChildCare,
        Dental,
        DomesticPartner,
        FlexibleHours,
        JobBenefitUnspecified,
        LifeInsurance,
        Medical,
        ParentalLeave,
        RetirementPlan,
        SickDays,
        Vacation,
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
    impl ::field_selector::FieldSelector for JobJobBenefitsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobJobLevel {
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
    impl ::field_selector::FieldSelector for JobJobLevel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobPostingRegion {
        #[doc = "In addition to exact location matching, job posting is returned when the\nLocationFilter in the search query is in the same administrative area\nas the returned job posting. For example, if a `ADMINISTRATIVE_AREA` job\nis posted in \"CA, USA\", it's returned if LocationFilter has\n\"Mountain View\".\n\nAdministrative area refers to top-level administrative subdivision of this\ncountry. For example, US state, IT region, UK constituent nation and\nJP prefecture."]
        AdministrativeArea,
        #[doc = "In addition to exact location matching, job is returned when\nLocationFilter in search query is in the same country as this job.\nFor example, if a `NATION_WIDE` job is posted in \"USA\", it's\nreturned if LocationFilter has 'Mountain View'."]
        Nation,
        #[doc = "If the region is unspecified, the job is only returned if it\nmatches the LocationFilter."]
        PostingRegionUnspecified,
        #[doc = "Job allows employees to work remotely (telecommute).\nIf locations are provided with this value, the job is\nconsidered as having a location, but telecommuting is allowed."]
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
    impl ::field_selector::FieldSelector for JobPostingRegion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobVisibility {
        #[doc = "The resource is only visible to the GCP account who owns it."]
        AccountOnly,
        #[doc = "The resource is visible to the owner and may be visible to other\napplications and processes at Google."]
        SharedWithGoogle,
        #[doc = "The resource is visible to the owner and may be visible to all other API\nclients."]
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
    impl ::field_selector::FieldSelector for JobVisibility {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobDerivedInfo {
        #[doc = "Job categories derived from Job.title and Job.description."]
        #[serde(rename = "jobCategories", default)]
        pub job_categories:
            ::std::option::Option<Vec<crate::schemas::JobDerivedInfoJobCategoriesItems>>,
        #[doc = "Structured locations of the job, resolved from Job.addresses.\n\nlocations are exactly matched to Job.addresses in the same\norder."]
        #[serde(rename = "locations", default)]
        pub locations: ::std::option::Option<Vec<crate::schemas::Location>>,
    }
    impl ::field_selector::FieldSelector for JobDerivedInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobDerivedInfoJobCategoriesItems {
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
    impl ::field_selector::FieldSelector for JobDerivedInfoJobCategoriesItems {
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
    pub struct JobEvent {
        #[doc = "Required. The job name(s) associated with this event.\nFor example, if this is an impression event,\nthis field contains the identifiers of all jobs shown to the job seeker.\nIf this was a view event, this field contains the\nidentifier of the viewed job."]
        #[serde(rename = "jobs", default)]
        pub jobs: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The type of the event (see JobEventType)."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::JobEventType>,
    }
    impl ::field_selector::FieldSelector for JobEvent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobEventType {
        #[doc = "This event should be used when a company submits an application\non behalf of a job seeker. This event is intended for use by staffing\nagencies attempting to place candidates."]
        ApplicationCompanySubmit,
        #[doc = "The job seeker or other entity interacting with the service\nsubmitted an application for a job."]
        ApplicationFinish,
        #[doc = "The job seeker or other entity interacting with the service\nsubmitted an application for a job with a single click without\nentering information. If a job seeker performs this action, send only\nthis event to the service. Do not also send\nJobEventType.APPLICATION_START or JobEventType.APPLICATION_FINISH\nevents."]
        ApplicationQuickSubmission,
        #[doc = "The job seeker or other entity interacting with the service\nperformed an action to apply to a job and was redirected to a different\nwebsite to complete the application."]
        ApplicationRedirect,
        #[doc = "The job seeker, or other entity interacting with the service, performs an\naction with a single click from the search results page to apply to a job\n(without viewing the details of the job posting), and is redirected\nto a different website to complete the application. If a candidate\nperforms this action, send only this event to the service. Do not also\nsend JobEventType.APPLICATION_START,\nJobEventType.APPLICATION_FINISH or JobEventType.VIEW events."]
        ApplicationRedirectFromSearch,
        #[doc = "The job seeker or other entity interacting with the service\nbegan the process or demonstrated the intention of applying for a job."]
        ApplicationStart,
        #[doc = "The job seeker or other entity interacting with the service began the\nprocess or demonstrated the intention of applying for a job from the\nsearch results page without viewing the details of the job posting.\nIf sending this event, JobEventType.VIEW event shouldn't be sent."]
        ApplicationStartFromSearch,
        #[doc = "The job seeker or other entity interacting with the service demonstrated\nan interest in a job by bookmarking or saving it."]
        Bookmark,
        #[doc = "The job seeker or other entity interacting with the service was\nemployed by the hiring entity (employer). Send this event\nonly if the job seeker was hired through an application that was\ninitiated by a search conducted through the Cloud Talent Solution\nservice."]
        Hired,
        #[doc = "The job seeker or other entity interacting with the service has\nhad a job rendered in their view, such as in a list of search results in\na compressed or clipped format. This event is typically associated with\nthe viewing of a jobs list on a single page by a job seeker."]
        Impression,
        #[doc = "The entity interacting with the service (for example, the job seeker),\nwas granted an initial interview by the hiring entity (employer). This\nevent should only be sent if the job seeker was granted an interview as\npart of an application that was initiated by a search conducted through /\nrecommendation provided by the Cloud Talent Solution service."]
        InterviewGranted,
        #[doc = "The event is unspecified by other provided values."]
        JobEventTypeUnspecified,
        #[doc = "The job seeker or other entity interacting with the service showed\nno interest in the job."]
        NotInterested,
        #[doc = "The job seeker or other entity interacting with the service was\nsent a notification, such as an email alert or device notification,\ncontaining one or more jobs listings generated by the service."]
        Notification,
        #[doc = "A recruiter or staffing agency submitted an application on behalf of the\ncandidate after interacting with the service to identify a suitable job\nposting."]
        SentCv,
        #[doc = "The job seeker, or other entity interacting with the service, has\nviewed the details of a job, including the full description. This\nevent doesn't apply to the viewing a snippet of a job appearing as a\npart of the job search results. Viewing a snippet is associated with an\nimpression)."]
        View,
        #[doc = "The job seeker or other entity interacting with the service\nperformed an action to view a job and was redirected to a different\nwebsite for job."]
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
    impl ::field_selector::FieldSelector for JobEventType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobQuery {
        #[doc = "Optional. Allows filtering jobs by commute time with different travel methods (for\nexample, driving or public transit). Note: This only works with COMMUTE\nMODE. When specified, [JobQuery.location_filters] is\nignored.\n\nCurrently we don't support sorting by commute time."]
        #[serde(rename = "commuteFilter", default)]
        pub commute_filter: ::std::option::Option<crate::schemas::CommuteFilter>,
        #[doc = "Optional. This filter specifies the exact company display\nname of the jobs to search against.\n\nIf a value isn't specified, jobs within the search results are\nassociated with any company.\n\nIf multiple values are specified, jobs within the search results may be\nassociated with any of the specified companies.\n\nAt most 20 company display name filters are allowed."]
        #[serde(rename = "companyDisplayNames", default)]
        pub company_display_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This filter specifies the company entities to search against.\n\nIf a value isn't specified, jobs are searched for against all\ncompanies.\n\nIf multiple values are specified, jobs are searched against the\ncompanies specified.\n\nThe format is \"projects/{project_id}/companies/{company_id}\", for example,\n\"projects/api-test-project/companies/foo\".\n\nAt most 20 company filters are allowed."]
        #[serde(rename = "companyNames", default)]
        pub company_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. This search filter is applied only to\nJob.compensation_info. For example, if the filter is specified\nas \"Hourly job with per-hour compensation > $15\", only jobs meeting\nthese criteria are searched. If a filter isn't defined, all open jobs\nare searched."]
        #[serde(rename = "compensationFilter", default)]
        pub compensation_filter: ::std::option::Option<crate::schemas::CompensationFilter>,
        #[doc = "Optional. This filter specifies a structured syntax to match against the\nJob.custom_attributes marked as `filterable`.\n\nThe syntax for this expression is a subset of SQL syntax.\n\nSupported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the\nleft of the operator is a custom field key and the right of the operator\nis a number or a quoted string. You must escape backslash (\\) and\nquote (\") characters.\n\nSupported functions are `LOWER([field_name])` to\nperform a case insensitive match and `EMPTY([field_name])` to filter on the\nexistence of a key.\n\nBoolean expressions (AND/OR/NOT) are supported up to 3 levels of\nnesting (for example, \"((A AND B AND C) OR NOT D) AND E\"), a maximum of 100\ncomparisons or functions are allowed in the expression. The expression\nmust be < 6000 bytes in length.\n\nSample Query:\n`(LOWER(driving_license)=\"class \\\"a\\\"\" OR EMPTY(driving_license)) AND driving_years > 10`"]
        #[serde(rename = "customAttributeFilter", default)]
        pub custom_attribute_filter: ::std::option::Option<String>,
        #[doc = "Optional. This flag controls the spell-check feature. If false, the\nservice attempts to correct a misspelled query,\nfor example, \"enginee\" is corrected to \"engineer\".\n\nDefaults to false: a spell check is performed."]
        #[serde(rename = "disableSpellCheck", default)]
        pub disable_spell_check: ::std::option::Option<bool>,
        #[doc = "Optional. The employment type filter specifies the employment type of jobs to\nsearch against, such as EmploymentType.FULL_TIME.\n\nIf a value is not specified, jobs in the search results includes any\nemployment type.\n\nIf multiple values are specified, jobs in the search results include\nany of the specified employment types."]
        #[serde(rename = "employmentTypes", default)]
        pub employment_types:
            ::std::option::Option<Vec<crate::schemas::JobQueryEmploymentTypesItems>>,
        #[doc = "Optional. The category filter specifies the categories of jobs to search against.\nSee Category for more information.\n\nIf a value is not specified, jobs from any category are searched against.\n\nIf multiple values are specified, jobs from any of the specified\ncategories are searched against."]
        #[serde(rename = "jobCategories", default)]
        pub job_categories: ::std::option::Option<Vec<crate::schemas::JobQueryJobCategoriesItems>>,
        #[doc = "Optional. This filter specifies the locale of jobs to search against,\nfor example, \"en-US\".\n\nIf a value isn't specified, the search results can contain jobs in any\nlocale.\n\nLanguage codes should be in BCP-47 format, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).\n\nAt most 10 language code filters are allowed."]
        #[serde(rename = "languageCodes", default)]
        pub language_codes: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The location filter specifies geo-regions containing the jobs to\nsearch against. See LocationFilter for more information.\n\nIf a location value isn't specified, jobs fitting the other search\ncriteria are retrieved regardless of where they're located.\n\nIf multiple values are specified, jobs are retrieved from any of the\nspecified locations. If different values are specified for the\nLocationFilter.distance_in_miles parameter, the maximum provided\ndistance is used for all locations.\n\nAt most 5 location filters are allowed."]
        #[serde(rename = "locationFilters", default)]
        pub location_filters: ::std::option::Option<Vec<crate::schemas::LocationFilter>>,
        #[doc = "Optional. Jobs published within a range specified by this filter are searched\nagainst."]
        #[serde(rename = "publishTimeRange", default)]
        pub publish_time_range: ::std::option::Option<crate::schemas::TimestampRange>,
        #[doc = "Optional. The query string that matches against the job title, description, and\nlocation fields.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "query", default)]
        pub query: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for JobQuery {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        OtherEmploymentType,
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
                JobQueryEmploymentTypesItems::OtherEmploymentType => "OTHER_EMPLOYMENT_TYPE",
                JobQueryEmploymentTypesItems::PartTime => "PART_TIME",
                JobQueryEmploymentTypesItems::PerDiem => "PER_DIEM",
                JobQueryEmploymentTypesItems::Temporary => "TEMPORARY",
                JobQueryEmploymentTypesItems::Volunteer => "VOLUNTEER",
            }
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
    impl ::field_selector::FieldSelector for JobQueryEmploymentTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobQueryJobCategoriesItems {
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
    impl ::field_selector::FieldSelector for JobQueryJobCategoriesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LatLng {
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for LatLng {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListCompaniesResponse {
        #[doc = "Companies for the current client."]
        #[serde(rename = "companies", default)]
        pub companies: ::std::option::Option<Vec<crate::schemas::Company>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "A token to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListCompaniesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListJobsResponse {
        #[doc = "The Jobs for a given company.\n\nThe maximum number of items returned is based on the limit field\nprovided in the request."]
        #[serde(rename = "jobs", default)]
        pub jobs: ::std::option::Option<Vec<crate::schemas::Job>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "A token to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListJobsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Location {
        #[doc = "An object representing a latitude/longitude pair."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "The type of a location, which corresponds to the address lines field of\nPostalAddress. For example, \"Downtown, Atlanta, GA, USA\" has a type of\nLocationType#NEIGHBORHOOD, and \"Kansas City, KS, USA\" has a type of\nLocationType#LOCALITY."]
        #[serde(rename = "locationType", default)]
        pub location_type: ::std::option::Option<crate::schemas::LocationLocationType>,
        #[doc = "Postal address of the location that includes human readable information,\nsuch as postal delivery and payments addresses. Given a postal address,\na postal service can deliver items to a premises, P.O. Box, or other\ndelivery location."]
        #[serde(rename = "postalAddress", default)]
        pub postal_address: ::std::option::Option<crate::schemas::PostalAddress>,
        #[doc = "Radius in miles of the job location. This value is derived from the\nlocation bounding box in which a circle with the specified radius\ncentered from LatLng covers the area associated with the job location.\nFor example, currently, \"Mountain View, CA, USA\" has a radius of\n6.17 miles."]
        #[serde(rename = "radiusInMiles", default)]
        pub radius_in_miles: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for Location {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        #[doc = "A sublocality is a subdivision of a locality, for example a city borough,\nward, or arrondissement. Sublocalities are usually recognized by a local\npolitical authority. For example, Manhattan and Brooklyn are recognized\nas boroughs by the City of New York, and are therefore modeled as\nsublocalities."]
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
    impl ::field_selector::FieldSelector for LocationLocationType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LocationFilter {
        #[doc = "Optional. The address name, such as \"Mountain View\" or \"Bay Area\"."]
        #[serde(rename = "address", default)]
        pub address: ::std::option::Option<String>,
        #[doc = "Optional. The distance_in_miles is applied when the location being searched for is\nidentified as a city or smaller. When the location being searched for is a\nstate or larger, this field is ignored."]
        #[serde(rename = "distanceInMiles", default)]
        pub distance_in_miles: ::std::option::Option<f64>,
        #[doc = "Optional. The latitude and longitude of the geographic center from which to\nsearch. This field's ignored if `address` is provided."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Optional. CLDR region code of the country/region of the address. This is used\nto address ambiguity of the user-input location, for example, \"Liverpool\"\nagainst \"Liverpool, NY, US\" or \"Liverpool, UK\".\n\nSet this field if all the jobs to search against are from a same region,\nor jobs are world-wide, but the job seeker is from a specific region.\n\nSee http://cldr.unicode.org/ and\nhttp://www.unicode.org/cldr/charts/30/supplemental/territory_information.html\nfor details. Example: \"CH\" for Switzerland."]
        #[serde(rename = "regionCode", default)]
        pub region_code: ::std::option::Option<String>,
        #[doc = "Optional. Allows the client to return jobs without a\nset location, specifically, telecommuting jobs (telecommuting is considered\nby the service as a special location.\nJob.posting_region indicates if a job permits telecommuting.\nIf this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED,\ntelecommuting jobs are searched, and address and lat_lng are\nignored. If not set or set to\nTelecommutePreference.TELECOMMUTE_EXCLUDED, telecommute job are not\nsearched.\n\nThis filter can be used by itself to search exclusively for telecommuting\njobs, or it can be combined with another location\nfilter to search for a combination of job locations,\nsuch as \"Mountain View\" or \"telecommuting\" jobs. However, when used in\ncombination with other location filters, telecommuting jobs can be\ntreated as less relevant than other jobs in the search response."]
        #[serde(rename = "telecommutePreference", default)]
        pub telecommute_preference:
            ::std::option::Option<crate::schemas::LocationFilterTelecommutePreference>,
    }
    impl ::field_selector::FieldSelector for LocationFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for LocationFilterTelecommutePreference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MatchingJob {
        #[doc = "Commute information which is generated based on specified\nCommuteFilter."]
        #[serde(rename = "commuteInfo", default)]
        pub commute_info: ::std::option::Option<crate::schemas::CommuteInfo>,
        #[doc = "Job resource that matches the specified SearchJobsRequest."]
        #[serde(rename = "job", default)]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "A summary of the job with core information that's displayed on the search\nresults listing page."]
        #[serde(rename = "jobSummary", default)]
        pub job_summary: ::std::option::Option<String>,
        #[doc = "Contains snippets of text from the Job.job_title field most\nclosely matching a search query's keywords, if available. The matching\nquery keywords are enclosed in HTML bold tags."]
        #[serde(rename = "jobTitleSnippet", default)]
        pub job_title_snippet: ::std::option::Option<String>,
        #[doc = "Contains snippets of text from the Job.description and similar\nfields that most closely match a search query's keywords, if available.\nAll HTML tags in the original fields are stripped when returned in this\nfield, and matching query keywords are enclosed in HTML bold tags."]
        #[serde(rename = "searchTextSnippet", default)]
        pub search_text_snippet: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MatchingJob {
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
    pub struct Money {
        #[doc = "The 3-letter currency code defined in ISO 4217."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999 inclusive.\nIf `units` is positive, `nanos` must be positive or zero.\nIf `units` is zero, `nanos` can be positive, zero, or negative.\nIf `units` is negative, `nanos` must be negative or zero.\nFor example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(rename = "nanos", default)]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "The whole units of the amount.\nFor example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        #[serde(rename = "units", default)]
        #[serde(with = "crate::parsed_string")]
        pub units: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for Money {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NumericBucketingOption {
        #[doc = "Required. Two adjacent values form a histogram bucket. Values should be in\nascending order. For example, if [5, 10, 15] are provided, four buckets are\ncreated: (-inf, 5), 5, 10), [10, 15), [15, inf). At most 20\n[buckets_bound is supported."]
        #[serde(rename = "bucketBounds", default)]
        pub bucket_bounds: ::std::option::Option<Vec<f64>>,
        #[doc = "Optional. If set to true, the histogram result includes minimum/maximum\nvalue of the numeric field."]
        #[serde(rename = "requiresMinMax", default)]
        pub requires_min_max: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for NumericBucketingOption {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NumericBucketingResult {
        #[doc = "Count within each bucket. Its size is the length of\nNumericBucketingOption.bucket_bounds plus 1."]
        #[serde(rename = "counts", default)]
        pub counts: ::std::option::Option<Vec<crate::schemas::BucketizedCount>>,
        #[doc = "Stores the maximum value of the numeric field. Is populated only if\n[NumericBucketingOption.requires_min_max] is set to true."]
        #[serde(rename = "maxValue", default)]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "Stores the minimum value of the numeric field. Will be populated only if\n[NumericBucketingOption.requires_min_max] is set to true."]
        #[serde(rename = "minValue", default)]
        pub min_value: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for NumericBucketingResult {
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
    pub struct PostalAddress {
        #[doc = "Unstructured address lines describing the lower levels of an address.\n\nBecause values in address_lines do not have type information and may\nsometimes contain multiple values in a single field (e.g.\n\"Austin, TX\"), it is important that the line order is clear. The order of\naddress lines should be \"envelope order\" for the country/region of the\naddress. In places where this can vary (e.g. Japan), address_language is\nused to make it explicit (e.g. \"ja\" for large-to-small ordering and\n\"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of\nan address can be selected based on the language.\n\nThe minimum permitted structural representation of an address consists\nof a region_code with all remaining information placed in the\naddress_lines. It would be possible to format such an address very\napproximately without geocoding, but no semantic reasoning could be\nmade about any of the address components until it was at least\npartially resolved.\n\nCreating an address only containing a region_code and address_lines, and\nthen geocoding is the recommended way to handle completely unstructured\naddresses (as opposed to guessing which parts of the address should be\nlocalities or administrative areas)."]
        #[serde(rename = "addressLines", default)]
        pub address_lines: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Highest administrative subdivision which is used for postal\naddresses of a country or region.\nFor example, this can be a state, a province, an oblast, or a prefecture.\nSpecifically, for Spain this is the province and not the autonomous\ncommunity (e.g. \"Barcelona\" and not \"Catalonia\").\nMany countries don't use an administrative area in postal addresses. E.g.\nin Switzerland this should be left unpopulated."]
        #[serde(rename = "administrativeArea", default)]
        pub administrative_area: ::std::option::Option<String>,
        #[doc = "Optional. BCP-47 language code of the contents of this address (if\nknown). This is often the UI language of the input form or is expected\nto match one of the languages used in the address' country/region, or their\ntransliterated equivalents.\nThis can affect formatting in certain countries, but is not critical\nto the correctness of the data and will never affect any validation or\nother non-formatting related operations.\n\nIf this value is not known, it should be omitted (rather than specifying a\npossibly incorrect default).\n\nExamples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Generally refers to the city/town portion of the address.\nExamples: US city, IT comune, UK post town.\nIn regions of the world where localities are not well defined or do not fit\ninto this structure well, leave locality empty and use address_lines."]
        #[serde(rename = "locality", default)]
        pub locality: ::std::option::Option<String>,
        #[doc = "Optional. The name of the organization at the address."]
        #[serde(rename = "organization", default)]
        pub organization: ::std::option::Option<String>,
        #[doc = "Optional. Postal code of the address. Not all countries use or require\npostal codes to be present, but where they are used, they may trigger\nadditional validation with other parts of the address (e.g. state/zip\nvalidation in the U.S.A.)."]
        #[serde(rename = "postalCode", default)]
        pub postal_code: ::std::option::Option<String>,
        #[doc = "Optional. The recipient at the address.\nThis field may, under certain circumstances, contain multiline information.\nFor example, it might contain \"care of\" information."]
        #[serde(rename = "recipients", default)]
        pub recipients: ::std::option::Option<Vec<String>>,
        #[doc = "Required. CLDR region code of the country/region of the address. This\nis never inferred and it is up to the user to ensure the value is\ncorrect. See http://cldr.unicode.org/ and\nhttp://www.unicode.org/cldr/charts/30/supplemental/territory_information.html\nfor details. Example: \"CH\" for Switzerland."]
        #[serde(rename = "regionCode", default)]
        pub region_code: ::std::option::Option<String>,
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is\nthe latest revision.\n\nAll new revisions **must** be backward compatible with old revisions."]
        #[serde(rename = "revision", default)]
        pub revision: ::std::option::Option<i32>,
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used\nin most regions. Where it is used, the value is either a string like\n\"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number\nalone, representing the \"sector code\" (Jamaica), \"delivery area indicator\"\n(Malawi) or \"post office indicator\" (e.g. C\u{f4}te d'Ivoire)."]
        #[serde(rename = "sortingCode", default)]
        pub sorting_code: ::std::option::Option<String>,
        #[doc = "Optional. Sublocality of the address.\nFor example, this can be neighborhoods, boroughs, districts."]
        #[serde(rename = "sublocality", default)]
        pub sublocality: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PostalAddress {
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
    pub struct ProcessingOptions {
        #[doc = "Optional. If set to `true`, the service does not attempt to resolve a\nmore precise address for the job."]
        #[serde(rename = "disableStreetAddressResolution", default)]
        pub disable_street_address_resolution: ::std::option::Option<bool>,
        #[doc = "Optional. Option for job HTML content sanitization. Applied fields are:\n\n* description\n* applicationInfo.instruction\n* incentives\n* qualifications\n* responsibilities\n\nHTML tags in these fields may be stripped if sanitiazation is not\ndisabled.\n\nDefaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY."]
        #[serde(rename = "htmlSanitization", default)]
        pub html_sanitization:
            ::std::option::Option<crate::schemas::ProcessingOptionsHtmlSanitization>,
    }
    impl ::field_selector::FieldSelector for ProcessingOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProcessingOptionsHtmlSanitization {
        #[doc = "Disables sanitization on HTML input."]
        HtmlSanitizationDisabled,
        #[doc = "Default value."]
        HtmlSanitizationUnspecified,
        #[doc = "Sanitizes HTML input, only accepts bold, italic, ordered list, and\nunordered list markup tags."]
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
    impl ::field_selector::FieldSelector for ProcessingOptionsHtmlSanitization {
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
    pub struct RequestMetadata {
        #[doc = "Optional. The type of device used by the job seeker at the time of the call to the\nservice."]
        #[serde(rename = "deviceInfo", default)]
        pub device_info: ::std::option::Option<crate::schemas::DeviceInfo>,
        #[doc = "Required. The client-defined scope or source of the service call, which typically\nis the domain on\nwhich the service has been implemented and is currently being run.\n\nFor example, if the service is being run by client <em>Foo, Inc.</em>, on\njob board www.foo.com and career site www.bar.com, then this field is\nset to \"foo.com\" for use on the job board, and \"bar.com\" for use on the\ncareer site.\n\nIf this field isn't available for some reason, send \"UNKNOWN\".\nAny improvements to the model for a particular tenant site rely on this\nfield being set correctly to a domain.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "domain", default)]
        pub domain: ::std::option::Option<String>,
        #[doc = "Required. A unique session identification string. A session is defined as the\nduration of an end user's interaction with the service over a certain\nperiod.\nObfuscate this field for privacy concerns before\nproviding it to the service.\n\nIf this field is not available for some reason, send \"UNKNOWN\". Note\nthat any improvements to the model for a particular tenant\nsite, rely on this field being set correctly to some unique session_id.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
        #[doc = "Required. A unique user identification string, as determined by the client.\nTo have the strongest positive impact on search quality\nmake sure the client-level is unique.\nObfuscate this field for privacy concerns before\nproviding it to the service.\n\nIf this field is not available for some reason, send \"UNKNOWN\". Note\nthat any improvements to the model for a particular tenant\nsite, rely on this field being set correctly to a unique user_id.\n\nThe maximum number of allowed characters is 255."]
        #[serde(rename = "userId", default)]
        pub user_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RequestMetadata {
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
    pub struct ResponseMetadata {
        #[doc = "A unique id associated with this call.\nThis id is logged for tracking purposes."]
        #[serde(rename = "requestId", default)]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ResponseMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchJobsRequest {
        #[doc = "Optional. Controls whether to disable exact keyword match on Job.job_title,\nJob.description, Job.company_display_name, Job.locations,\nJob.qualifications. When disable keyword match is turned off, a\nkeyword match returns jobs that do not match given category filters when\nthere are matching keywords. For example, the query \"program manager,\" a\nresult is returned even if the job posting has the title \"software\ndeveloper,\" which does not fall into \"program manager\" ontology, but does\nhave \"program manager\" appearing in its description.\n\nFor queries like \"cloud\" that does not contain title or\nlocation specific ontology, jobs with \"cloud\" keyword matches are returned\nregardless of this flag's value.\n\nPlease use Company.keyword_searchable_custom_fields or\nCompany.keyword_searchable_custom_attributes if company specific\nglobally matched custom field/attribute string values is needed. Enabling\nkeyword match improves recall of subsequent search requests.\n\nDefaults to false."]
        #[serde(rename = "disableKeywordMatch", default)]
        pub disable_keyword_match: ::std::option::Option<bool>,
        #[doc = "Optional. Controls whether highly similar jobs are returned next to each other in\nthe search results. Jobs are identified as highly similar based on\ntheir titles, job categories, and locations. Highly similar results are\nclustered so that only one representative job of the cluster is\ndisplayed to the job seeker higher up in the results, with the other jobs\nbeing displayed lower down in the results.\n\nDefaults to DiversificationLevel.SIMPLE if no value\nis specified."]
        #[serde(rename = "diversificationLevel", default)]
        pub diversification_level:
            ::std::option::Option<crate::schemas::SearchJobsRequestDiversificationLevel>,
        #[doc = "Optional. Controls whether to broaden the search when it produces sparse results.\nBroadened queries append results to the end of the matching results\nlist.\n\nDefaults to false."]
        #[serde(rename = "enableBroadening", default)]
        pub enable_broadening: ::std::option::Option<bool>,
        #[doc = "Optional. Histogram requests for jobs matching JobQuery."]
        #[serde(rename = "histogramFacets", default)]
        pub histogram_facets: ::std::option::Option<crate::schemas::HistogramFacets>,
        #[doc = "Optional. Query used to search against jobs, such as keyword, location filters, etc."]
        #[serde(rename = "jobQuery", default)]
        pub job_query: ::std::option::Option<crate::schemas::JobQuery>,
        #[doc = "Optional. The desired job attributes returned for jobs in the\nsearch response. Defaults to JobView.SMALL if no value is specified."]
        #[serde(rename = "jobView", default)]
        pub job_view: ::std::option::Option<crate::schemas::SearchJobsRequestJobView>,
        #[doc = "Optional. An integer that specifies the current offset (that is, starting result\nlocation, amongst the jobs deemed by the API as relevant) in search\nresults. This field is only considered if page_token is unset.\n\nFor example, 0 means to  return results starting from the first matching\njob, and 10 means to return from the 11th job. This can be used for\npagination, (for example, pageSize = 10 and offset = 10 means to return\nfrom the second page)."]
        #[serde(rename = "offset", default)]
        pub offset: ::std::option::Option<i32>,
        #[doc = "Optional. The criteria determining how search results are sorted. Default is\n\"relevance desc\".\n\nSupported options are:\n\n* `\"relevance desc\"`: By relevance descending, as determined by the API\n  algorithms. Relevance thresholding of query results is only available\n  with this ordering.\n* `\"posting_publish_time desc\"`: By Job.posting_publish_time\n  descending.\n* `\"posting_update_time desc\"`: By Job.posting_update_time\n  descending.\n* `\"title\"`: By Job.title ascending.\n* `\"title desc\"`: By Job.title descending.\n* `\"annualized_base_compensation\"`: By job's\n  CompensationInfo.annualized_base_compensation_range ascending. Jobs\n  whose annualized base compensation is unspecified are put at the end of\n  search results.\n* `\"annualized_base_compensation desc\"`: By job's\n  CompensationInfo.annualized_base_compensation_range descending. Jobs\n  whose annualized base compensation is unspecified are put at the end of\n  search results.\n* `\"annualized_total_compensation\"`: By job's\n  CompensationInfo.annualized_total_compensation_range ascending. Jobs\n  whose annualized base compensation is unspecified are put at the end of\n  search results.\n* `\"annualized_total_compensation desc\"`: By job's\n  CompensationInfo.annualized_total_compensation_range descending. Jobs\n  whose annualized base compensation is unspecified are put at the end of\n  search results."]
        #[serde(rename = "orderBy", default)]
        pub order_by: ::std::option::Option<String>,
        #[doc = "Optional. A limit on the number of jobs returned in the search results.\nIncreasing this value above the default value of 10 can increase search\nresponse time. The value can be between 1 and 100."]
        #[serde(rename = "pageSize", default)]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. The token specifying the current offset within\nsearch results. See SearchJobsResponse.next_page_token for\nan explanation of how to obtain the next set of query results."]
        #[serde(rename = "pageToken", default)]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Required. The meta information collected about the job searcher, used to improve the\nsearch quality of the service. The identifiers (such as `user_id`) are\nprovided by users, and must be unique and consistent."]
        #[serde(rename = "requestMetadata", default)]
        pub request_metadata: ::std::option::Option<crate::schemas::RequestMetadata>,
        #[doc = "Optional. Controls if the search job request requires the return of a precise\ncount of the first 300 results. Setting this to `true` ensures\nconsistency in the number of results per page. Best practice is to set this\nvalue to true if a client allows users to jump directly to a\nnon-sequential search results page.\n\nEnabling this flag may adversely impact performance.\n\nDefaults to false."]
        #[serde(rename = "requirePreciseResultSize", default)]
        pub require_precise_result_size: ::std::option::Option<bool>,
        #[doc = "Optional. Mode of a search.\n\nDefaults to SearchMode.JOB_SEARCH."]
        #[serde(rename = "searchMode", default)]
        pub search_mode: ::std::option::Option<crate::schemas::SearchJobsRequestSearchMode>,
    }
    impl ::field_selector::FieldSelector for SearchJobsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestDiversificationLevel {
        #[doc = "Disables diversification. Jobs that would normally be pushed to the last\npage would not have their positions altered. This may result in highly\nsimilar jobs appearing in sequence in the search results."]
        Disabled,
        #[doc = "The diversification level isn't specified. By default, jobs with this\nenum are ordered according to SIMPLE diversifying behavior."]
        DiversificationLevelUnspecified,
        #[doc = "Default diversifying behavior. The result list is ordered so that\nhighly similar results are pushed to the end of the last page of search\nresults."]
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
    impl ::field_selector::FieldSelector for SearchJobsRequestDiversificationLevel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestJobView {
        #[doc = "All available attributes are included in the search results."]
        JobViewFull,
        #[doc = "A ID only view of job, with following attributes:\nJob.name, Job.requisition_id, Job.language_code."]
        JobViewIdOnly,
        #[doc = "A minimal view of the job, with the following attributes:\nJob.name, Job.requisition_id, Job.title,\nJob.company_name, Job.DerivedInfo.locations, Job.language_code."]
        JobViewMinimal,
        #[doc = "A small view of the job, with the following attributes in the search\nresults: Job.name, Job.requisition_id, Job.title,\nJob.company_name, Job.DerivedInfo.locations, Job.visibility,\nJob.language_code, Job.description."]
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
    impl ::field_selector::FieldSelector for SearchJobsRequestJobView {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchJobsRequestSearchMode {
        #[doc = "The job search matches only against featured jobs (jobs with a\npromotionValue > 0). This method doesn't return any jobs having a\npromotionValue <= 0. The search results order is determined by the\npromotionValue (jobs with a higher promotionValue are returned higher up\nin the search results), with relevance being used as a tiebreaker."]
        FeaturedJobSearch,
        #[doc = "The job search matches against all jobs, and featured jobs\n(jobs with promotionValue > 0) are not specially handled."]
        JobSearch,
        #[doc = "The mode of the search method isn't specified."]
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
    impl ::field_selector::FieldSelector for SearchJobsRequestSearchMode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchJobsResponse {
        #[doc = "If query broadening is enabled, we may append additional results from the\nbroadened query. This number indicates how many of the jobs returned in the\njobs field are from the broadened query. These results are always at the\nend of the jobs list. In particular, a value of 0, or if the field isn't\nset, all the jobs in the jobs list are from the original\n(without broadening) query. If this field is non-zero, subsequent requests\nwith offset after this result set should contain all broadened results."]
        #[serde(rename = "broadenedQueryJobsCount", default)]
        pub broadened_query_jobs_count: ::std::option::Option<i32>,
        #[doc = "An estimation of the number of jobs that match the specified query.\n\nThis number is not guaranteed to be accurate. For accurate results,\nsee enable_precise_result_size."]
        #[serde(rename = "estimatedTotalSize", default)]
        pub estimated_total_size: ::std::option::Option<i32>,
        #[doc = "The histogram results that match specified\nSearchJobsRequest.histogram_facets."]
        #[serde(rename = "histogramResults", default)]
        pub histogram_results: ::std::option::Option<crate::schemas::HistogramResults>,
        #[doc = "The location filters that the service applied to the specified query. If\nany filters are lat-lng based, the JobLocation.location_type is\nJobLocation.LocationType#LOCATION_TYPE_UNSPECIFIED."]
        #[serde(rename = "locationFilters", default)]
        pub location_filters: ::std::option::Option<Vec<crate::schemas::Location>>,
        #[doc = "The Job entities that match the specified SearchJobsRequest."]
        #[serde(rename = "matchingJobs", default)]
        pub matching_jobs: ::std::option::Option<Vec<crate::schemas::MatchingJob>>,
        #[doc = "Additional information for the API invocation, such as the request\ntracking id."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetadata>,
        #[doc = "The token that specifies the starting position of the next page of results.\nThis field is empty if there are no more results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The spell checking result, and correction."]
        #[serde(rename = "spellCorrection", default)]
        pub spell_correction: ::std::option::Option<crate::schemas::SpellingCorrection>,
        #[doc = "The precise result count, which is available only if the client set\nenable_precise_result_size to `true`, or if the response\nis the last page of results. Otherwise, the value is `-1`."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for SearchJobsResponse {
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
    pub struct SpellingCorrection {
        #[doc = "Indicates if the query was corrected by the spell checker."]
        #[serde(rename = "corrected", default)]
        pub corrected: ::std::option::Option<bool>,
        #[doc = "Correction output consisting of the corrected keyword string."]
        #[serde(rename = "correctedText", default)]
        pub corrected_text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SpellingCorrection {
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
    pub struct TimeOfDay {
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose\nto allow the value \"24:00:00\" for scenarios like business closing time."]
        #[serde(rename = "hours", default)]
        pub hours: ::std::option::Option<i32>,
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        #[serde(rename = "minutes", default)]
        pub minutes: ::std::option::Option<i32>,
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        #[serde(rename = "nanos", default)]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may\nallow the value 60 if it allows leap-seconds."]
        #[serde(rename = "seconds", default)]
        pub seconds: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for TimeOfDay {
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
    pub struct TimestampRange {
        #[doc = "End of the period."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Begin of the period."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TimestampRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateCompanyRequest {
        #[doc = "Required. The company resource to replace the current resource in the system."]
        #[serde(rename = "company", default)]
        pub company: ::std::option::Option<crate::schemas::Company>,
        #[doc = "Optional but strongly recommended for the best service\nexperience.\n\nIf update_mask is provided, only the specified fields in\ncompany are updated. Otherwise all the fields are updated.\n\nA field mask to specify the company fields to be updated. Only\ntop level fields of Company are supported."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateCompanyRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateJobRequest {
        #[doc = "Required. The Job to be updated."]
        #[serde(rename = "job", default)]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "Optional but strongly recommended to be provided for the best service\nexperience.\n\nIf update_mask is provided, only the specified fields in\njob are updated. Otherwise all the fields are updated.\n\nA field mask to restrict the fields that are updated. Only\ntop level fields of Job are supported."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateJobRequest {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions<A> {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod projects {
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
            impl ::field_selector::FieldSelector for CompleteType {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
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
            impl ::field_selector::FieldSelector for CompleteScope {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct ProjectsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
            #[doc = "Completes the specified prefix with keyword suggestions.\nIntended for use by a job search auto-complete search box."]
            pub fn complete(&self, name: impl Into<String>) -> CompleteRequestBuilder<A> {
                CompleteRequestBuilder {
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
            ) -> crate::resources::projects::client_events::ClientEventsActions<A> {
                crate::resources::projects::client_events::ClientEventsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the companies resource"]
            pub fn companies(&self) -> crate::resources::projects::companies::CompaniesActions<A> {
                crate::resources::projects::companies::CompaniesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> crate::resources::projects::jobs::JobsActions<A> {
                crate::resources::projects::jobs::JobsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CompleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> CompleteRequestBuilder<'a, A> {
            #[doc = "Optional. If provided, restricts completion to specified company.\n\nThe format is \"projects/{project_id}/companies/{company_id}\", for example,\n\"projects/api-test-project/companies/foo\"."]
            pub fn company_name(mut self, value: impl Into<String>) -> Self {
                self.company_name = Some(value.into());
                self
            }
            #[doc = "Deprecated. Use language_codes instead.\n\nOptional.\n\nThe language of the query. This is\nthe BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).\n\nFor CompletionType.JOB_TITLE type, only open jobs with the same\nlanguage_code are returned.\n\nFor CompletionType.COMPANY_NAME type,\nonly companies having open jobs with the same language_code are\nreturned.\n\nFor CompletionType.COMBINED type, only open jobs with the same\nlanguage_code or companies having open jobs with the same\nlanguage_code are returned.\n\nThe maximum number of allowed characters is 255."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "Optional. The list of languages of the query. This is\nthe BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\n[Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).\n\nFor CompletionType.JOB_TITLE type, only open jobs with the same\nlanguage_codes are returned.\n\nFor CompletionType.COMPANY_NAME type,\nonly companies having open jobs with the same language_codes are\nreturned.\n\nFor CompletionType.COMBINED type, only open jobs with the same\nlanguage_codes or companies having open jobs with the same\nlanguage_codes are returned.\n\nThe maximum number of allowed characters is 255."]
            pub fn language_codes(mut self, value: impl Into<Vec<String>>) -> Self {
                self.language_codes = Some(value.into());
                self
            }
            #[doc = "Required. Completion result count.\n\nThe maximum allowed page size is 10."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Required. The query used to generate suggestions.\n\nThe maximum number of allowed characters is 255."]
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
            ) -> Result<crate::schemas::CompleteQueryResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::CompleteQueryResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://jobs.googleapis.com/".to_owned();
                output.push_str("v3/");
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
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("companyName", &self.company_name)]);
                let req = req.query(&[("languageCode", &self.language_code)]);
                let req = req.query(&[("languageCodes", &self.language_codes)]);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        pub mod client_events {
            pub mod params {}
            pub struct ClientEventsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ClientEventsActions<'a, A> {
                #[doc = "Report events issued when end user interacts with customer's application\nthat uses Cloud Talent Solution. You may inspect the created events in\n[self service\ntools](https://console.cloud.google.com/talent-solution/overview).\n[Learn\nmore](https://cloud.google.com/talent-solution/docs/management-tools)\nabout self service tools."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateClientEventRequest,
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
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::ClientEvent, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ClientEvent, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
        pub mod companies {
            pub mod params {}
            pub struct CompaniesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> CompaniesActions<'a, A> {
                #[doc = "Creates a new company entity."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateCompanyRequest,
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
                #[doc = "Deletes specified company.\nPrerequisite: The company has no jobs associated with it."]
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
                #[doc = "Retrieves specified company."]
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
                #[doc = "Lists all companies associated with the service account."]
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
                        require_open_jobs: None,
                    }
                }
                #[doc = "Updates specified company. Company names can't be updated. To update a\ncompany name, delete the company and all jobs associated with it, and only\nthen re-create them."]
                pub fn patch(
                    &self,
                    request: crate::schemas::UpdateCompanyRequest,
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
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Company, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Company, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
                ) -> Result<crate::schemas::Company, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Company, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
                #[doc = "Optional. Set to true if the companies requested must have open jobs.\n\nDefaults to false.\n\nIf true, at most page_size of companies are fetched, among which\nonly those with open jobs are returned."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_companies<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "companies").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "companies")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_companies_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Company> {
                    self.fields = Some(concat!("nextPageToken,", "companies").to_owned());
                    crate::iter::PageItemIter::new(self, "companies")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_companies_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Company> {
                    self.fields = Some(concat!("nextPageToken,", "companies", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "companies")
                }
                pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = T::field_selector();
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
                pub fn iter_standard(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCompaniesResponse>
                {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCompaniesResponse>
                {
                    self.fields = Some("*".to_owned());
                    crate::iter::PageIter::new(self)
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
                ) -> Result<crate::schemas::ListCompaniesResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListCompaniesResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("requireOpenJobs", &self.require_open_jobs)]);
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Company, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Company, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
        pub mod jobs {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListJobView {
                    JobViewFull,
                    JobViewIdOnly,
                    JobViewMinimal,
                    JobViewSmall,
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
                impl ::field_selector::FieldSelector for ListJobView {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct JobsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> JobsActions<'a, A> {
                #[doc = "Deletes a list of Jobs by filter."]
                pub fn batch_delete(
                    &self,
                    request: crate::schemas::BatchDeleteJobsRequest,
                    parent: impl Into<String>,
                ) -> BatchDeleteRequestBuilder<A> {
                    BatchDeleteRequestBuilder {
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
                #[doc = "Creates a new job.\n\nTypically, the job becomes searchable within 10 seconds, but it may take\nup to 5 minutes."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateJobRequest,
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
                #[doc = "Deletes the specified job.\n\nTypically, the job becomes unsearchable within 10 seconds, but it may take\nup to 5 minutes."]
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
                #[doc = "Retrieves the specified job, whose status is OPEN or recently EXPIRED\nwithin the last 90 days."]
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
                #[doc = "Lists jobs by filter."]
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
                        job_view: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates specified job.\n\nTypically, updated contents become visible in search results within 10\nseconds, but it may take up to 5 minutes."]
                pub fn patch(
                    &self,
                    request: crate::schemas::UpdateJobRequest,
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
                    }
                }
                #[doc = "Searches for jobs using the provided SearchJobsRequest.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs that the caller has\npermission to search against."]
                pub fn search(
                    &self,
                    request: crate::schemas::SearchJobsRequest,
                    parent: impl Into<String>,
                ) -> SearchRequestBuilder<A> {
                    SearchRequestBuilder {
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
                #[doc = "Searches for jobs using the provided SearchJobsRequest.\n\nThis API call is intended for the use case of targeting passive job\nseekers (for example, job seekers who have signed up to receive email\nalerts about potential job opportunities), and has different algorithmic\nadjustments that are targeted to passive job seekers.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs the caller has\npermission to search against."]
                pub fn search_for_alert(
                    &self,
                    request: crate::schemas::SearchJobsRequest,
                    parent: impl Into<String>,
                ) -> SearchForAlertRequestBuilder<A> {
                    SearchForAlertRequestBuilder {
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
            }
            #[derive(Debug, Clone)]
            pub struct BatchDeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> BatchDeleteRequestBuilder<'a, A> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Required. The filter string specifies the jobs to be enumerated.\n\nSupported operator: =, AND\n\nThe fields eligible for filtering are:\n\n* `companyName` (Required)\n* `requisitionId` (Optional)\n\nSample Query:\n\n* companyName = \"projects/api-test-project/companies/123\"\n* companyName = \"projects/api-test-project/companies/123\" AND requisitionId\n  = \"req-1\""]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The desired job attributes returned for jobs in the\nsearch response. Defaults to JobView.JOB_VIEW_FULL if no value is\nspecified."]
                pub fn job_view(
                    mut self,
                    value: crate::resources::projects::jobs::params::ListJobView,
                ) -> Self {
                    self.job_view = Some(value);
                    self
                }
                #[doc = "Optional. The maximum number of jobs to be returned per page of results.\n\nIf job_view is set to JobView.JOB_VIEW_ID_ONLY, the maximum allowed\npage size is 1000. Otherwise, the maximum allowed page size is 100.\n\nDefault is 100 if empty or a number < 1 is specified."]
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
                pub fn iter_jobs<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "jobs").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_jobs_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.fields = Some(concat!("nextPageToken,", "jobs").to_owned());
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_jobs_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.fields = Some(concat!("nextPageToken,", "jobs", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = T::field_selector();
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
                pub fn iter_standard(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
                    self.fields = Some("*".to_owned());
                    crate::iter::PageIter::new(self)
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
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("jobView", &self.job_view)]);
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::SearchJobsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::SearchJobsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct SearchForAlertRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> SearchForAlertRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::SearchJobsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::SearchJobsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://jobs.googleapis.com/".to_owned();
                    output.push_str("v3/");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
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
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
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
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
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
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
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
                                return Some(Err(format!(
                                    "no {} field found in iter response",
                                    self.items_field
                                )
                                .into()))
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
