#![doc = "# Resources and Methods\n    * [info_types](resources/info_types/struct.InfoTypesActions.html)\n      * [*list*](resources/info_types/struct.ListRequestBuilder.html)\n    * [locations](resources/locations/struct.LocationsActions.html)\n      * [*infoTypes*](resources/locations/struct.InfoTypesRequestBuilder.html)\n    * [organizations](resources/organizations/struct.OrganizationsActions.html)\n      * [deidentify_templates](resources/organizations/deidentify_templates/struct.DeidentifyTemplatesActions.html)\n        * [*create*](resources/organizations/deidentify_templates/struct.CreateRequestBuilder.html), [*delete*](resources/organizations/deidentify_templates/struct.DeleteRequestBuilder.html), [*get*](resources/organizations/deidentify_templates/struct.GetRequestBuilder.html), [*list*](resources/organizations/deidentify_templates/struct.ListRequestBuilder.html), [*patch*](resources/organizations/deidentify_templates/struct.PatchRequestBuilder.html)\n      * [inspect_templates](resources/organizations/inspect_templates/struct.InspectTemplatesActions.html)\n        * [*create*](resources/organizations/inspect_templates/struct.CreateRequestBuilder.html), [*delete*](resources/organizations/inspect_templates/struct.DeleteRequestBuilder.html), [*get*](resources/organizations/inspect_templates/struct.GetRequestBuilder.html), [*list*](resources/organizations/inspect_templates/struct.ListRequestBuilder.html), [*patch*](resources/organizations/inspect_templates/struct.PatchRequestBuilder.html)\n      * [stored_info_types](resources/organizations/stored_info_types/struct.StoredInfoTypesActions.html)\n        * [*create*](resources/organizations/stored_info_types/struct.CreateRequestBuilder.html), [*delete*](resources/organizations/stored_info_types/struct.DeleteRequestBuilder.html), [*get*](resources/organizations/stored_info_types/struct.GetRequestBuilder.html), [*list*](resources/organizations/stored_info_types/struct.ListRequestBuilder.html), [*patch*](resources/organizations/stored_info_types/struct.PatchRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [content](resources/projects/content/struct.ContentActions.html)\n        * [*deidentify*](resources/projects/content/struct.DeidentifyRequestBuilder.html), [*inspect*](resources/projects/content/struct.InspectRequestBuilder.html), [*reidentify*](resources/projects/content/struct.ReidentifyRequestBuilder.html)\n      * [deidentify_templates](resources/projects/deidentify_templates/struct.DeidentifyTemplatesActions.html)\n        * [*create*](resources/projects/deidentify_templates/struct.CreateRequestBuilder.html), [*delete*](resources/projects/deidentify_templates/struct.DeleteRequestBuilder.html), [*get*](resources/projects/deidentify_templates/struct.GetRequestBuilder.html), [*list*](resources/projects/deidentify_templates/struct.ListRequestBuilder.html), [*patch*](resources/projects/deidentify_templates/struct.PatchRequestBuilder.html)\n      * [dlp_jobs](resources/projects/dlp_jobs/struct.DlpJobsActions.html)\n        * [*cancel*](resources/projects/dlp_jobs/struct.CancelRequestBuilder.html), [*create*](resources/projects/dlp_jobs/struct.CreateRequestBuilder.html), [*delete*](resources/projects/dlp_jobs/struct.DeleteRequestBuilder.html), [*get*](resources/projects/dlp_jobs/struct.GetRequestBuilder.html), [*list*](resources/projects/dlp_jobs/struct.ListRequestBuilder.html)\n      * [image](resources/projects/image/struct.ImageActions.html)\n        * [*redact*](resources/projects/image/struct.RedactRequestBuilder.html)\n      * [inspect_templates](resources/projects/inspect_templates/struct.InspectTemplatesActions.html)\n        * [*create*](resources/projects/inspect_templates/struct.CreateRequestBuilder.html), [*delete*](resources/projects/inspect_templates/struct.DeleteRequestBuilder.html), [*get*](resources/projects/inspect_templates/struct.GetRequestBuilder.html), [*list*](resources/projects/inspect_templates/struct.ListRequestBuilder.html), [*patch*](resources/projects/inspect_templates/struct.PatchRequestBuilder.html)\n      * [job_triggers](resources/projects/job_triggers/struct.JobTriggersActions.html)\n        * [*activate*](resources/projects/job_triggers/struct.ActivateRequestBuilder.html), [*create*](resources/projects/job_triggers/struct.CreateRequestBuilder.html), [*delete*](resources/projects/job_triggers/struct.DeleteRequestBuilder.html), [*get*](resources/projects/job_triggers/struct.GetRequestBuilder.html), [*list*](resources/projects/job_triggers/struct.ListRequestBuilder.html), [*patch*](resources/projects/job_triggers/struct.PatchRequestBuilder.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [content](resources/projects/locations/content/struct.ContentActions.html)\n          * [*deidentify*](resources/projects/locations/content/struct.DeidentifyRequestBuilder.html), [*inspect*](resources/projects/locations/content/struct.InspectRequestBuilder.html), [*reidentify*](resources/projects/locations/content/struct.ReidentifyRequestBuilder.html)\n      * [stored_info_types](resources/projects/stored_info_types/struct.StoredInfoTypesActions.html)\n        * [*create*](resources/projects/stored_info_types/struct.CreateRequestBuilder.html), [*delete*](resources/projects/stored_info_types/struct.DeleteRequestBuilder.html), [*get*](resources/projects/stored_info_types/struct.GetRequestBuilder.html), [*list*](resources/projects/stored_info_types/struct.ListRequestBuilder.html), [*patch*](resources/projects/stored_info_types/struct.PatchRequestBuilder.html)\n"]
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
    pub struct GooglePrivacyDlpV2Action {
        #[doc = "Enable email notification to project owners and editors on job's\ncompletion/failure."]
        #[serde(
            rename = "jobNotificationEmails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_notification_emails:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2JobNotificationEmails>,
        #[doc = "Publish a notification to a pubsub topic."]
        #[serde(
            rename = "pubSub",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pub_sub: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PublishToPubSub>,
        #[doc = "Publish findings to Cloud Datahub."]
        #[serde(
            rename = "publishFindingsToCloudDataCatalog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_findings_to_cloud_data_catalog: ::std::option::Option<
            crate::schemas::GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog,
        >,
        #[doc = "Publish summary to Cloud Security Command Center (Alpha)."]
        #[serde(
            rename = "publishSummaryToCscc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_summary_to_cscc:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PublishSummaryToCscc>,
        #[doc = "Save resulting findings in a provided location."]
        #[serde(
            rename = "saveFindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub save_findings: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2SaveFindings>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Action {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Action {
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
    pub struct GooglePrivacyDlpV2ActivateJobTriggerRequest;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ActivateJobTriggerRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ActivateJobTriggerRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
        #[serde(
            rename = "categoricalStatsResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categorical_stats_result:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CategoricalStatsResult>,
        #[serde(
            rename = "deltaPresenceEstimationResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delta_presence_estimation_result:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeltaPresenceEstimationResult>,
        #[serde(
            rename = "kAnonymityResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub k_anonymity_result:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2KAnonymityResult>,
        #[serde(
            rename = "kMapEstimationResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub k_map_estimation_result:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2KMapEstimationResult>,
        #[serde(
            rename = "lDiversityResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub l_diversity_result:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2LDiversityResult>,
        #[serde(
            rename = "numericalStatsResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numerical_stats_result:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2NumericalStatsResult>,
        #[doc = "Privacy metric to compute."]
        #[serde(
            rename = "requestedPrivacyMetric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_privacy_metric:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PrivacyMetric>,
        #[doc = "Input dataset to compute metrics over."]
        #[serde(
            rename = "requestedSourceTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_source_table:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
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
    pub struct GooglePrivacyDlpV2AuxiliaryTable {
        #[doc = "Quasi-identifier columns. [required]"]
        #[serde(
            rename = "quasiIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2QuasiIdField>>,
        #[doc = "The relative frequency column must contain a floating-point number\nbetween 0 and 1 (inclusive). Null values are assumed to be zero.\n[required]"]
        #[serde(
            rename = "relativeFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relative_frequency: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "Auxiliary table location. [required]"]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2AuxiliaryTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2AuxiliaryTable {
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
    pub struct GooglePrivacyDlpV2BigQueryField {
        #[doc = "Designated field in the BigQuery table."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "Source table of the field."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BigQueryField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BigQueryField {
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
    pub struct GooglePrivacyDlpV2BigQueryKey {
        #[doc = "Absolute number of the row from the beginning of the table at the time\nof scanning."]
        #[serde(
            rename = "rowNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub row_number: ::std::option::Option<i64>,
        #[doc = "Complete BigQuery table reference."]
        #[serde(
            rename = "tableReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_reference: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BigQueryKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BigQueryKey {
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
    pub struct GooglePrivacyDlpV2BigQueryOptions {
        #[doc = "References to fields excluded from scanning. This allows you to skip\ninspection of entire columns which you know have no findings."]
        #[serde(
            rename = "excludedFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_fields: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldId>>,
        #[doc = "References to fields uniquely identifying rows within the table.\nNested fields in the format, like `person.birthdate.year`, are allowed."]
        #[serde(
            rename = "identifyingFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identifying_fields:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldId>>,
        #[doc = "Max number of rows to scan. If the table has more rows than this value, the\nrest of the rows are omitted. If not set, or if set to 0, all rows will be\nscanned. Only one of rows_limit and rows_limit_percent can be specified.\nCannot be used in conjunction with TimespanConfig."]
        #[serde(
            rename = "rowsLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub rows_limit: ::std::option::Option<i64>,
        #[doc = "Max percentage of rows to scan. The rest are omitted. The number of rows\nscanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and\n100 means no limit. Defaults to 0. Only one of rows_limit and\nrows_limit_percent can be specified. Cannot be used in conjunction with\nTimespanConfig."]
        #[serde(
            rename = "rowsLimitPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows_limit_percent: ::std::option::Option<i32>,
        #[serde(
            rename = "sampleMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_method:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryOptionsSampleMethod>,
        #[doc = "Complete BigQuery table reference."]
        #[serde(
            rename = "tableReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_reference: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BigQueryOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BigQueryOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        #[doc = "Randomly pick the row to start scanning. The scanned rows are contiguous."]
        RandomStart,
        SampleMethodUnspecified,
        #[doc = "Scan from the top (default)."]
        Top,
    }
    impl GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2BigQueryOptionsSampleMethod::RandomStart => "RANDOM_START",
                GooglePrivacyDlpV2BigQueryOptionsSampleMethod::SampleMethodUnspecified => {
                    "SAMPLE_METHOD_UNSPECIFIED"
                }
                GooglePrivacyDlpV2BigQueryOptionsSampleMethod::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2BigQueryOptionsSampleMethod, ()> {
            Ok(match s {
                "RANDOM_START" => GooglePrivacyDlpV2BigQueryOptionsSampleMethod::RandomStart,
                "SAMPLE_METHOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2BigQueryOptionsSampleMethod::SampleMethodUnspecified
                }
                "TOP" => GooglePrivacyDlpV2BigQueryOptionsSampleMethod::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RANDOM_START" => GooglePrivacyDlpV2BigQueryOptionsSampleMethod::RandomStart,
                "SAMPLE_METHOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2BigQueryOptionsSampleMethod::SampleMethodUnspecified
                }
                "TOP" => GooglePrivacyDlpV2BigQueryOptionsSampleMethod::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BigQueryOptionsSampleMethod {
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
    pub struct GooglePrivacyDlpV2BigQueryTable {
        #[doc = "Dataset ID of the table."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "The Google Cloud Platform project ID of the project containing the table.\nIf omitted, project ID is inferred from the API call."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Name of the table."]
        #[serde(
            rename = "tableId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BigQueryTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BigQueryTable {
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
    pub struct GooglePrivacyDlpV2BoundingBox {
        #[doc = "Height of the bounding box in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Left coordinate of the bounding box. (0,0) is upper left."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<i32>,
        #[doc = "Top coordinate of the bounding box. (0,0) is upper left."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<i32>,
        #[doc = "Width of the bounding box in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BoundingBox {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BoundingBox {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Bucket {
        #[doc = "Upper bound of the range, exclusive; type must match min."]
        #[serde(
            rename = "max",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
        #[doc = "Lower bound of the range, inclusive. Type should be the same as max if\nused."]
        #[serde(
            rename = "min",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
        #[doc = "Replacement value for this bucket. If not provided\nthe default behavior will be to hyphenate the min-max range."]
        #[serde(
            rename = "replacementValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replacement_value: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Bucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Bucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2BucketingConfig {
        #[doc = "Set of buckets. Ranges must be non-overlapping."]
        #[serde(
            rename = "buckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buckets: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Bucket>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2BucketingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2BucketingConfig {
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
    pub struct GooglePrivacyDlpV2ByteContentItem {
        #[doc = "Content data to inspect or redact."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The type of data stored in the bytes string. Default will be TEXT_UTF8."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ByteContentItemType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ByteContentItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ByteContentItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2ByteContentItemType {
        Avro,
        BytesTypeUnspecified,
        Image,
        ImageBmp,
        ImageJpeg,
        ImagePng,
        ImageSvg,
        TextUtf8,
    }
    impl GooglePrivacyDlpV2ByteContentItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2ByteContentItemType::Avro => "AVRO",
                GooglePrivacyDlpV2ByteContentItemType::BytesTypeUnspecified => {
                    "BYTES_TYPE_UNSPECIFIED"
                }
                GooglePrivacyDlpV2ByteContentItemType::Image => "IMAGE",
                GooglePrivacyDlpV2ByteContentItemType::ImageBmp => "IMAGE_BMP",
                GooglePrivacyDlpV2ByteContentItemType::ImageJpeg => "IMAGE_JPEG",
                GooglePrivacyDlpV2ByteContentItemType::ImagePng => "IMAGE_PNG",
                GooglePrivacyDlpV2ByteContentItemType::ImageSvg => "IMAGE_SVG",
                GooglePrivacyDlpV2ByteContentItemType::TextUtf8 => "TEXT_UTF8",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2ByteContentItemType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2ByteContentItemType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2ByteContentItemType, ()> {
            Ok(match s {
                "AVRO" => GooglePrivacyDlpV2ByteContentItemType::Avro,
                "BYTES_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ByteContentItemType::BytesTypeUnspecified
                }
                "IMAGE" => GooglePrivacyDlpV2ByteContentItemType::Image,
                "IMAGE_BMP" => GooglePrivacyDlpV2ByteContentItemType::ImageBmp,
                "IMAGE_JPEG" => GooglePrivacyDlpV2ByteContentItemType::ImageJpeg,
                "IMAGE_PNG" => GooglePrivacyDlpV2ByteContentItemType::ImagePng,
                "IMAGE_SVG" => GooglePrivacyDlpV2ByteContentItemType::ImageSvg,
                "TEXT_UTF8" => GooglePrivacyDlpV2ByteContentItemType::TextUtf8,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2ByteContentItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2ByteContentItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2ByteContentItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVRO" => GooglePrivacyDlpV2ByteContentItemType::Avro,
                "BYTES_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ByteContentItemType::BytesTypeUnspecified
                }
                "IMAGE" => GooglePrivacyDlpV2ByteContentItemType::Image,
                "IMAGE_BMP" => GooglePrivacyDlpV2ByteContentItemType::ImageBmp,
                "IMAGE_JPEG" => GooglePrivacyDlpV2ByteContentItemType::ImageJpeg,
                "IMAGE_PNG" => GooglePrivacyDlpV2ByteContentItemType::ImagePng,
                "IMAGE_SVG" => GooglePrivacyDlpV2ByteContentItemType::ImageSvg,
                "TEXT_UTF8" => GooglePrivacyDlpV2ByteContentItemType::TextUtf8,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ByteContentItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ByteContentItemType {
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
    pub struct GooglePrivacyDlpV2CancelDlpJobRequest;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CancelDlpJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CancelDlpJobRequest {
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
    pub struct GooglePrivacyDlpV2CategoricalStatsConfig {
        #[doc = "Field to compute categorical stats on. All column types are\nsupported except for arrays and structs. However, it may be more\ninformative to use NumericalStats when the field type is supported,\ndepending on the data."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CategoricalStatsConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CategoricalStatsConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
        #[doc = "Total number of values in this bucket."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_size: ::std::option::Option<i64>,
        #[doc = "Total number of distinct values in this bucket."]
        #[serde(
            rename = "bucketValueCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_value_count: ::std::option::Option<i64>,
        #[doc = "Sample of value frequencies in this bucket. The total number of\nvalues returned per bucket is capped at 20."]
        #[serde(
            rename = "bucketValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_values:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2ValueFrequency>>,
        #[doc = "Lower bound on the value frequency of the values in this bucket."]
        #[serde(
            rename = "valueFrequencyLowerBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value_frequency_lower_bound: ::std::option::Option<i64>,
        #[doc = "Upper bound on the value frequency of the values in this bucket."]
        #[serde(
            rename = "valueFrequencyUpperBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value_frequency_upper_bound: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2CategoricalStatsResult {
        #[doc = "Histogram of value frequencies in the column."]
        #[serde(
            rename = "valueFrequencyHistogramBuckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_frequency_histogram_buckets: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2CategoricalStatsHistogramBucket>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CategoricalStatsResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CategoricalStatsResult {
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
    pub struct GooglePrivacyDlpV2CharacterMaskConfig {
        #[doc = "When masking a string, items in this list will be skipped when replacing.\nFor example, if your string is 555-555-5555 and you ask us to skip `-` and\nmask 5 chars with * we would produce ***-*55-5555."]
        #[serde(
            rename = "charactersToIgnore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub characters_to_ignore:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2CharsToIgnore>>,
        #[doc = "Character to mask the sensitive values\u{2014}for example, \"*\" for an\nalphabetic string such as name, or \"0\" for a numeric string such as ZIP\ncode or credit card number. String must have length 1. If not supplied, we\nwill default to \"*\" for strings, 0 for digits."]
        #[serde(
            rename = "maskingCharacter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub masking_character: ::std::option::Option<String>,
        #[doc = "Number of characters to mask. If not set, all matching chars will be\nmasked. Skipped characters do not count towards this tally."]
        #[serde(
            rename = "numberToMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_to_mask: ::std::option::Option<i32>,
        #[doc = "Mask characters in reverse order. For example, if `masking_character` is\n'0', number_to_mask is 14, and `reverse_order` is false, then\n1234-5678-9012-3456 -> 00000000000000-3456\nIf `masking_character` is '*', `number_to_mask` is 3, and `reverse_order`\nis true, then 12345 -> 12***"]
        #[serde(
            rename = "reverseOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reverse_order: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CharacterMaskConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CharacterMaskConfig {
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
    pub struct GooglePrivacyDlpV2CharsToIgnore {
        #[serde(
            rename = "charactersToSkip",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub characters_to_skip: ::std::option::Option<String>,
        #[serde(
            rename = "commonCharactersToIgnore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_characters_to_ignore: ::std::option::Option<
            crate::schemas::GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore,
        >,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CharsToIgnore {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CharsToIgnore {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        #[doc = "a-z"]
        AlphaLowerCase,
        #[doc = "A-Z"]
        AlphaUpperCase,
        CommonCharsToIgnoreUnspecified,
        #[doc = "0-9"]
        Numeric,
        #[doc = "US Punctuation, one of !\"#$%&'()*+,-./:;<=>?@[]^_`{|}~"]
        Punctuation,
        #[doc = "Whitespace character, one of [ \\t\\n\\x0B\\f\\r]"]
        Whitespace,
    }
    impl GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        pub fn as_str(self) -> &'static str {
            match self { GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: AlphaLowerCase => "ALPHA_LOWER_CASE" , GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: AlphaUpperCase => "ALPHA_UPPER_CASE" , GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: CommonCharsToIgnoreUnspecified => "COMMON_CHARS_TO_IGNORE_UNSPECIFIED" , GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Numeric => "NUMERIC" , GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Punctuation => "PUNCTUATION" , GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Whitespace => "WHITESPACE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore, ()>
        {
            Ok ( match s { "ALPHA_LOWER_CASE" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: AlphaLowerCase , "ALPHA_UPPER_CASE" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: AlphaUpperCase , "COMMON_CHARS_TO_IGNORE_UNSPECIFIED" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: CommonCharsToIgnoreUnspecified , "NUMERIC" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Numeric , "PUNCTUATION" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Punctuation , "WHITESPACE" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Whitespace , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ALPHA_LOWER_CASE" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: AlphaLowerCase , "ALPHA_UPPER_CASE" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: AlphaUpperCase , "COMMON_CHARS_TO_IGNORE_UNSPECIFIED" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: CommonCharsToIgnoreUnspecified , "NUMERIC" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Numeric , "PUNCTUATION" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Punctuation , "WHITESPACE" => GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore :: Whitespace , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnore
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
    pub struct GooglePrivacyDlpV2CloudStorageFileSet {
        #[doc = "The url, in the format `gs://<bucket>/<path>`. Trailing wildcard in the\npath is allowed."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CloudStorageFileSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CloudStorageFileSet {
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
    pub struct GooglePrivacyDlpV2CloudStorageOptions {
        #[doc = "Max number of bytes to scan from a file. If a scanned file's size is bigger\nthan this value then the rest of the bytes are omitted. Only one\nof bytes_limit_per_file and bytes_limit_per_file_percent can be specified."]
        #[serde(
            rename = "bytesLimitPerFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bytes_limit_per_file: ::std::option::Option<i64>,
        #[doc = "Max percentage of bytes to scan from a file. The rest are omitted. The\nnumber of bytes scanned is rounded down. Must be between 0 and 100,\ninclusively. Both 0 and 100 means no limit. Defaults to 0. Only one\nof bytes_limit_per_file and bytes_limit_per_file_percent can be specified."]
        #[serde(
            rename = "bytesLimitPerFilePercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bytes_limit_per_file_percent: ::std::option::Option<i32>,
        #[doc = "The set of one or more files to scan."]
        #[serde(
            rename = "fileSet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_set: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FileSet>,
        #[doc = "List of file type groups to include in the scan.\nIf empty, all files are scanned and available data format processors\nare applied. In addition, the binary content of the selected files\nis always scanned as well."]
        #[serde(
            rename = "fileTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_types: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems>,
        >,
        #[doc = "Limits the number of files to scan to this percentage of the input FileSet.\nNumber of files scanned is rounded down. Must be between 0 and 100,\ninclusively. Both 0 and 100 means no limit. Defaults to 0."]
        #[serde(
            rename = "filesLimitPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files_limit_percent: ::std::option::Option<i32>,
        #[serde(
            rename = "sampleMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_method: ::std::option::Option<
            crate::schemas::GooglePrivacyDlpV2CloudStorageOptionsSampleMethod,
        >,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CloudStorageOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CloudStorageOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        Avro,
        BinaryFile,
        FileTypeUnspecified,
        Image,
        TextFile,
    }
    impl GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::Avro => "AVRO",
                GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::BinaryFile => "BINARY_FILE",
                GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::FileTypeUnspecified => {
                    "FILE_TYPE_UNSPECIFIED"
                }
                GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::Image => "IMAGE",
                GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::TextFile => "TEXT_FILE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems, ()>
        {
            Ok(match s {
                "AVRO" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::Avro,
                "BINARY_FILE" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::BinaryFile,
                "FILE_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::FileTypeUnspecified
                }
                "IMAGE" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::Image,
                "TEXT_FILE" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::TextFile,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVRO" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::Avro,
                "BINARY_FILE" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::BinaryFile,
                "FILE_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::FileTypeUnspecified
                }
                "IMAGE" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::Image,
                "TEXT_FILE" => GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems::TextFile,
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
        for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CloudStorageOptionsFileTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        #[doc = "For each file larger than bytes_limit_per_file, randomly pick the offset\nto start scanning. The scanned bytes are contiguous."]
        RandomStart,
        SampleMethodUnspecified,
        #[doc = "Scan from the top (default)."]
        Top,
    }
    impl GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::RandomStart => "RANDOM_START",
                GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::SampleMethodUnspecified => {
                    "SAMPLE_METHOD_UNSPECIFIED"
                }
                GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2CloudStorageOptionsSampleMethod, ()> {
            Ok(match s {
                "RANDOM_START" => GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::RandomStart,
                "SAMPLE_METHOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::SampleMethodUnspecified
                }
                "TOP" => GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RANDOM_START" => GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::RandomStart,
                "SAMPLE_METHOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::SampleMethodUnspecified
                }
                "TOP" => GooglePrivacyDlpV2CloudStorageOptionsSampleMethod::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CloudStorageOptionsSampleMethod {
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
    pub struct GooglePrivacyDlpV2CloudStoragePath {
        #[doc = "A url representing a file or path (no wildcards) in Cloud Storage.\nExample: gs://[BUCKET_NAME]/dictionary.txt"]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CloudStoragePath {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CloudStoragePath {
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
    pub struct GooglePrivacyDlpV2CloudStorageRegexFileSet {
        #[doc = "The name of a Cloud Storage bucket. Required."]
        #[serde(
            rename = "bucketName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_name: ::std::option::Option<String>,
        #[doc = "A list of regular expressions matching file paths to exclude. All files in\nthe bucket that match at least one of these regular expressions will be\nexcluded from the scan.\n\nRegular expressions use RE2\n[syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found\nunder the google/re2 repository on GitHub."]
        #[serde(
            rename = "excludeRegex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_regex: ::std::option::Option<Vec<String>>,
        #[doc = "A list of regular expressions matching file paths to include. All files in\nthe bucket that match at least one of these regular expressions will be\nincluded in the set of files, except for those that also match an item in\n`exclude_regex`. Leaving this field empty will match all files by default\n(this is equivalent to including `.*` in the list).\n\nRegular expressions use RE2\n[syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found\nunder the google/re2 repository on GitHub."]
        #[serde(
            rename = "includeRegex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_regex: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CloudStorageRegexFileSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CloudStorageRegexFileSet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Color {
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "blue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "green",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub green: ::std::option::Option<f32>,
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "red",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub red: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Color {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Color {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Condition {
        #[doc = "Field within the record this condition is evaluated against. [required]"]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "Operator used to compare the field or infoType to the value. [required]"]
        #[serde(
            rename = "operator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ConditionOperator>,
        #[doc = "Value to compare against. [Required, except for `EXISTS` tests.]"]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Condition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Condition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2ConditionOperator {
        #[doc = "Equal. Attempts to match even with incompatible types."]
        EqualTo,
        #[doc = "Exists"]
        Exists,
        #[doc = "Greater than."]
        GreaterThan,
        #[doc = "Greater than or equals."]
        GreaterThanOrEquals,
        #[doc = "Less than."]
        LessThan,
        #[doc = "Less than or equals."]
        LessThanOrEquals,
        #[doc = "Not equal to. Attempts to match even with incompatible types."]
        NotEqualTo,
        RelationalOperatorUnspecified,
    }
    impl GooglePrivacyDlpV2ConditionOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2ConditionOperator::EqualTo => "EQUAL_TO",
                GooglePrivacyDlpV2ConditionOperator::Exists => "EXISTS",
                GooglePrivacyDlpV2ConditionOperator::GreaterThan => "GREATER_THAN",
                GooglePrivacyDlpV2ConditionOperator::GreaterThanOrEquals => {
                    "GREATER_THAN_OR_EQUALS"
                }
                GooglePrivacyDlpV2ConditionOperator::LessThan => "LESS_THAN",
                GooglePrivacyDlpV2ConditionOperator::LessThanOrEquals => "LESS_THAN_OR_EQUALS",
                GooglePrivacyDlpV2ConditionOperator::NotEqualTo => "NOT_EQUAL_TO",
                GooglePrivacyDlpV2ConditionOperator::RelationalOperatorUnspecified => {
                    "RELATIONAL_OPERATOR_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2ConditionOperator {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2ConditionOperator {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2ConditionOperator, ()> {
            Ok(match s {
                "EQUAL_TO" => GooglePrivacyDlpV2ConditionOperator::EqualTo,
                "EXISTS" => GooglePrivacyDlpV2ConditionOperator::Exists,
                "GREATER_THAN" => GooglePrivacyDlpV2ConditionOperator::GreaterThan,
                "GREATER_THAN_OR_EQUALS" => {
                    GooglePrivacyDlpV2ConditionOperator::GreaterThanOrEquals
                }
                "LESS_THAN" => GooglePrivacyDlpV2ConditionOperator::LessThan,
                "LESS_THAN_OR_EQUALS" => GooglePrivacyDlpV2ConditionOperator::LessThanOrEquals,
                "NOT_EQUAL_TO" => GooglePrivacyDlpV2ConditionOperator::NotEqualTo,
                "RELATIONAL_OPERATOR_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ConditionOperator::RelationalOperatorUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2ConditionOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2ConditionOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2ConditionOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EQUAL_TO" => GooglePrivacyDlpV2ConditionOperator::EqualTo,
                "EXISTS" => GooglePrivacyDlpV2ConditionOperator::Exists,
                "GREATER_THAN" => GooglePrivacyDlpV2ConditionOperator::GreaterThan,
                "GREATER_THAN_OR_EQUALS" => {
                    GooglePrivacyDlpV2ConditionOperator::GreaterThanOrEquals
                }
                "LESS_THAN" => GooglePrivacyDlpV2ConditionOperator::LessThan,
                "LESS_THAN_OR_EQUALS" => GooglePrivacyDlpV2ConditionOperator::LessThanOrEquals,
                "NOT_EQUAL_TO" => GooglePrivacyDlpV2ConditionOperator::NotEqualTo,
                "RELATIONAL_OPERATOR_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ConditionOperator::RelationalOperatorUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ConditionOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ConditionOperator {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Conditions {
        #[serde(
            rename = "conditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditions: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Condition>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Conditions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Conditions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ContentItem {
        #[doc = "Content data to inspect or redact. Replaces `type` and `data`."]
        #[serde(
            rename = "byteItem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub byte_item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ByteContentItem>,
        #[doc = "Structured content for inspection. See\nhttps://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to\nlearn more."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Table>,
        #[doc = "String data to inspect or redact."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ContentItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ContentItem {
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
    pub struct GooglePrivacyDlpV2ContentLocation {
        #[doc = "Name of the container where the finding is located.\nThe top level name is the source file name or table name. Names of some\ncommon storage containers are formatted as follows:\n\n* BigQuery tables:  `<project_id>:<dataset_id>.<table_id>`\n* Cloud Storage files: `gs://<bucket>/<path>`\n* Datastore namespace: <namespace>\n\nNested names could be absent if the embedded object has no string\nidentifier (for an example an image contained within a document)."]
        #[serde(
            rename = "containerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container_name: ::std::option::Option<String>,
        #[doc = "Findings container modification timestamp, if applicable.\nFor Google Cloud Storage contains last file modification timestamp.\nFor BigQuery table contains last_modified_time property.\nFor Datastore - not populated."]
        #[serde(
            rename = "containerTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container_timestamp: ::std::option::Option<String>,
        #[doc = "Findings container version, if available\n(\"generation\" for Google Cloud Storage)."]
        #[serde(
            rename = "containerVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container_version: ::std::option::Option<String>,
        #[doc = "Location data for document files."]
        #[serde(
            rename = "documentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_location:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DocumentLocation>,
        #[doc = "Location within an image's pixels."]
        #[serde(
            rename = "imageLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_location: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ImageLocation>,
        #[doc = "Location within a row or record of a database table."]
        #[serde(
            rename = "recordLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub record_location:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RecordLocation>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ContentLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ContentLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
        #[doc = "The DeidentifyTemplate to create."]
        #[serde(
            rename = "deidentifyTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deidentify_template:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate>,
        #[doc = "The template id can contain uppercase and lowercase letters,\nnumbers, and hyphens; that is, it must match the regular\nexpression: `[a-zA-Z\\\\d-_]+`. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
        #[serde(
            rename = "templateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
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
    pub struct GooglePrivacyDlpV2CreateDlpJobRequest {
        #[serde(
            rename = "inspectJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_job: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectJobConfig>,
        #[doc = "The job id can contain uppercase and lowercase letters,\nnumbers, and hyphens; that is, it must match the regular\nexpression: `[a-zA-Z\\\\d-_]+`. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
        #[serde(
            rename = "jobId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_id: ::std::option::Option<String>,
        #[serde(
            rename = "riskJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub risk_job:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RiskAnalysisJobConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CreateDlpJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CreateDlpJobRequest {
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
    pub struct GooglePrivacyDlpV2CreateInspectTemplateRequest {
        #[doc = "The InspectTemplate to create."]
        #[serde(
            rename = "inspectTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_template:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectTemplate>,
        #[doc = "The template id can contain uppercase and lowercase letters,\nnumbers, and hyphens; that is, it must match the regular\nexpression: `[a-zA-Z\\\\d-_]+`. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
        #[serde(
            rename = "templateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CreateInspectTemplateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CreateInspectTemplateRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2CreateJobTriggerRequest {
        #[doc = "The JobTrigger to create."]
        #[serde(
            rename = "jobTrigger",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_trigger: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2JobTrigger>,
        #[doc = "The trigger id can contain uppercase and lowercase letters,\nnumbers, and hyphens; that is, it must match the regular\nexpression: `[a-zA-Z\\\\d-_]+`. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
        #[serde(
            rename = "triggerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trigger_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CreateJobTriggerRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CreateJobTriggerRequest {
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
    pub struct GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
        #[doc = "Configuration of the storedInfoType to create."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeConfig>,
        #[doc = "The storedInfoType ID can contain uppercase and lowercase letters,\nnumbers, and hyphens; that is, it must match the regular\nexpression: `[a-zA-Z\\\\d-_]+`. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
        #[serde(
            rename = "storedInfoTypeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stored_info_type_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
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
    pub struct GooglePrivacyDlpV2CryptoDeterministicConfig {
        #[doc = "Optional. A context may be used for higher security and maintaining\nreferential integrity such that the same identifier in two different\ncontexts will be given a distinct surrogate. The context is appended to\nplaintext value being encrypted. On decryption the provided context is\nvalidated against the value used during encryption. If a context was\nprovided during encryption, same context must be provided during decryption\nas well.\n\nIf the context is not set, plaintext would be used as is for encryption.\nIf the context is set but:\n\n1. there is no record present when transforming a given value or\n1. the field is not present when transforming a given value,\n\nplaintext would be used as is for encryption.\n\nNote that case (1) is expected when an `InfoTypeTransformation` is\napplied to both structured and non-structured `ContentItem`s."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "The key used by the encryption function."]
        #[serde(
            rename = "cryptoKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoKey>,
        #[doc = "The custom info type to annotate the surrogate with.\nThis annotation will be applied to the surrogate by prefixing it with\nthe name of the custom info type followed by the number of\ncharacters comprising the surrogate. The following scheme defines the\nformat: <info type name>(<surrogate character count>):<surrogate>\n\nFor example, if the name of custom info type is 'MY_TOKEN_INFO_TYPE' and\nthe surrogate is 'abc', the full replacement value\nwill be: 'MY_TOKEN_INFO_TYPE(3):abc'\n\nThis annotation identifies the surrogate when inspecting content using the\ncustom info type 'Surrogate'. This facilitates reversal of the\nsurrogate when it occurs in free text.\n\nNote: For record transformations where the entire cell in a table is being\ntransformed, surrogates are optional to use. Surrogates are used to denote\nthe location of the token and are necessary for re-identification in free\nform text.\n\nIn order for inspection to work properly, the name of this info type must\nnot occur naturally anywhere in your data; otherwise, inspection may either\n\n* reverse a surrogate that does not correspond to an actual identifier\n* be unable to parse the surrogate and result in an error\n\nTherefore, choose your custom info type name carefully after considering\nwhat your data looks like. One way to select a name that has a high chance\nof yielding reliable detection is to include one or more unicode characters\nthat are highly improbable to exist in your data.\nFor example, assuming your data is entered from a regular ASCII keyboard,\nthe symbol with the hex code point 29DD might be used like so:\n\u{29dd}MY_TOKEN_TYPE."]
        #[serde(
            rename = "surrogateInfoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub surrogate_info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CryptoDeterministicConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CryptoDeterministicConfig {
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
    pub struct GooglePrivacyDlpV2CryptoHashConfig {
        #[doc = "The key used by the hash function."]
        #[serde(
            rename = "cryptoKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoKey>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CryptoHashConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CryptoHashConfig {
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
    pub struct GooglePrivacyDlpV2CryptoKey {
        #[serde(
            rename = "kmsWrapped",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_wrapped:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2KmsWrappedCryptoKey>,
        #[serde(
            rename = "transient",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transient: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TransientCryptoKey>,
        #[serde(
            rename = "unwrapped",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unwrapped: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2UnwrappedCryptoKey>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CryptoKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CryptoKey {
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
    pub struct GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
        #[serde(
            rename = "commonAlphabet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_alphabet: ::std::option::Option<
            crate::schemas::GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet,
        >,
        #[doc = "The 'tweak', a context may be used for higher security since the same\nidentifier in two different contexts won't be given the same surrogate. If\nthe context is not set, a default tweak will be used.\n\nIf the context is set but:\n\n1. there is no record present when transforming a given value or\n1. the field is not present when transforming a given value,\n\na default tweak will be used.\n\nNote that case (1) is expected when an `InfoTypeTransformation` is\napplied to both structured and non-structured `ContentItem`s.\nCurrently, the referenced field may be of value type integer or string.\n\nThe tweak is constructed as a sequence of bytes in big endian byte order\nsuch that:\n\n* a 64 bit integer is encoded followed by a single byte of value 1\n* a string is encoded in UTF-8 format followed by a single byte of value 2"]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "The key used by the encryption algorithm. [required]"]
        #[serde(
            rename = "cryptoKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoKey>,
        #[doc = "This is supported by mapping these to the alphanumeric characters\nthat the FFX mode natively supports. This happens before/after\nencryption/decryption.\nEach character listed must appear only once.\nNumber of characters must be in the range [2, 62].\nThis must be encoded as ASCII.\nThe order of characters does not matter."]
        #[serde(
            rename = "customAlphabet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_alphabet: ::std::option::Option<String>,
        #[doc = "The native way to select the alphabet. Must be in the range [2, 62]."]
        #[serde(
            rename = "radix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub radix: ::std::option::Option<i32>,
        #[doc = "The custom infoType to annotate the surrogate with.\nThis annotation will be applied to the surrogate by prefixing it with\nthe name of the custom infoType followed by the number of\ncharacters comprising the surrogate. The following scheme defines the\nformat: info_type_name(surrogate_character_count):surrogate\n\nFor example, if the name of custom infoType is 'MY_TOKEN_INFO_TYPE' and\nthe surrogate is 'abc', the full replacement value\nwill be: 'MY_TOKEN_INFO_TYPE(3):abc'\n\nThis annotation identifies the surrogate when inspecting content using the\ncustom infoType\n[`SurrogateType`](/dlp/docs/reference/rest/v2/InspectConfig#surrogatetype).\nThis facilitates reversal of the surrogate when it occurs in free text.\n\nIn order for inspection to work properly, the name of this infoType must\nnot occur naturally anywhere in your data; otherwise, inspection may\nfind a surrogate that does not correspond to an actual identifier.\nTherefore, choose your custom infoType name carefully after considering\nwhat your data looks like. One way to select a name that has a high chance\nof yielding reliable detection is to include one or more unicode characters\nthat are highly improbable to exist in your data.\nFor example, assuming your data is entered from a regular ASCII keyboard,\nthe symbol with the hex code point 29DD might be used like so:\n\u{29dd}MY_TOKEN_TYPE"]
        #[serde(
            rename = "surrogateInfoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub surrogate_info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        #[doc = "[0-9A-Za-z] (radix of 62)"]
        AlphaNumeric,
        FfxCommonNativeAlphabetUnspecified,
        #[doc = "[0-9A-F] (radix of 16)"]
        Hexadecimal,
        #[doc = "[0-9] (radix of 10)"]
        Numeric,
        #[doc = "[0-9A-Z] (radix of 36)"]
        UpperCaseAlphaNumeric,
    }
    impl GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        pub fn as_str(self) -> &'static str {
            match self { GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: AlphaNumeric => "ALPHA_NUMERIC" , GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: FfxCommonNativeAlphabetUnspecified => "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED" , GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: Hexadecimal => "HEXADECIMAL" , GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: Numeric => "NUMERIC" , GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: UpperCaseAlphaNumeric => "UPPER_CASE_ALPHA_NUMERIC" , }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet, ()>
        {
            Ok ( match s { "ALPHA_NUMERIC" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: AlphaNumeric , "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: FfxCommonNativeAlphabetUnspecified , "HEXADECIMAL" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: Hexadecimal , "NUMERIC" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: Numeric , "UPPER_CASE_ALPHA_NUMERIC" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: UpperCaseAlphaNumeric , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ALPHA_NUMERIC" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: AlphaNumeric , "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: FfxCommonNativeAlphabetUnspecified , "HEXADECIMAL" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: Hexadecimal , "NUMERIC" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: Numeric , "UPPER_CASE_ALPHA_NUMERIC" => GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet :: UpperCaseAlphaNumeric , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabet
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
    pub struct GooglePrivacyDlpV2CustomInfoType {
        #[doc = "Set of detection rules to apply to all findings of this CustomInfoType.\nRules are applied in order that they are specified. Not supported for the\n`surrogate_type` CustomInfoType."]
        #[serde(
            rename = "detectionRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detection_rules:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2DetectionRule>>,
        #[doc = "A list of phrases to detect as a CustomInfoType."]
        #[serde(
            rename = "dictionary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dictionary: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Dictionary>,
        #[doc = "If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding\nto be returned. It still can be used for rules matching."]
        #[serde(
            rename = "exclusionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusion_type:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CustomInfoTypeExclusionType>,
        #[doc = "CustomInfoType can either be a new infoType, or an extension of built-in\ninfoType, when the name matches one of existing infoTypes and that infoType\nis specified in `InspectContent.info_types` field. Specifying the latter\nadds findings to the one detected by the system. If built-in info type is\nnot specified in `InspectContent.info_types` list then the name is treated\nas a custom info type."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
        #[doc = "Likelihood to return for this CustomInfoType. This base value can be\naltered by a detection rule if the finding meets the criteria specified by\nthe rule. Defaults to `VERY_LIKELY` if not specified."]
        #[serde(
            rename = "likelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub likelihood:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CustomInfoTypeLikelihood>,
        #[doc = "Regular expression based CustomInfoType."]
        #[serde(
            rename = "regex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regex: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Regex>,
        #[doc = "Load an existing `StoredInfoType` resource for use in\n`InspectDataSource`. Not currently supported in `InspectContent`."]
        #[serde(
            rename = "storedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stored_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredType>,
        #[doc = "Message for detecting output from deidentification transformations that\nsupport reversing."]
        #[serde(
            rename = "surrogateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub surrogate_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2SurrogateType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CustomInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CustomInfoType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        #[doc = "A finding of this custom info type will be excluded from final results,\nbut can still affect rule execution."]
        ExclusionTypeExclude,
        #[doc = "A finding of this custom info type will not be excluded from results."]
        ExclusionTypeUnspecified,
    }
    impl GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2CustomInfoTypeExclusionType::ExclusionTypeExclude => {
                    "EXCLUSION_TYPE_EXCLUDE"
                }
                GooglePrivacyDlpV2CustomInfoTypeExclusionType::ExclusionTypeUnspecified => {
                    "EXCLUSION_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2CustomInfoTypeExclusionType, ()> {
            Ok(match s {
                "EXCLUSION_TYPE_EXCLUDE" => {
                    GooglePrivacyDlpV2CustomInfoTypeExclusionType::ExclusionTypeExclude
                }
                "EXCLUSION_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CustomInfoTypeExclusionType::ExclusionTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXCLUSION_TYPE_EXCLUDE" => {
                    GooglePrivacyDlpV2CustomInfoTypeExclusionType::ExclusionTypeExclude
                }
                "EXCLUSION_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CustomInfoTypeExclusionType::ExclusionTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CustomInfoTypeExclusionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        Likely,
        #[doc = "Some matching elements."]
        Possible,
        Unlikely,
        #[doc = "Many matching elements."]
        VeryLikely,
        #[doc = "Few matching elements."]
        VeryUnlikely,
    }
    impl GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2CustomInfoTypeLikelihood::LikelihoodUnspecified => {
                    "LIKELIHOOD_UNSPECIFIED"
                }
                GooglePrivacyDlpV2CustomInfoTypeLikelihood::Likely => "LIKELY",
                GooglePrivacyDlpV2CustomInfoTypeLikelihood::Possible => "POSSIBLE",
                GooglePrivacyDlpV2CustomInfoTypeLikelihood::Unlikely => "UNLIKELY",
                GooglePrivacyDlpV2CustomInfoTypeLikelihood::VeryLikely => "VERY_LIKELY",
                GooglePrivacyDlpV2CustomInfoTypeLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2CustomInfoTypeLikelihood, ()> {
            Ok(match s {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CustomInfoTypeLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::VeryLikely,
                "VERY_UNLIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::VeryUnlikely,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2CustomInfoTypeLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::VeryLikely,
                "VERY_UNLIKELY" => GooglePrivacyDlpV2CustomInfoTypeLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2CustomInfoTypeLikelihood {
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
    pub struct GooglePrivacyDlpV2DatastoreKey {
        #[doc = "Datastore entity key."]
        #[serde(
            rename = "entityKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Key>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DatastoreKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DatastoreKey {
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
    pub struct GooglePrivacyDlpV2DatastoreOptions {
        #[doc = "The kind to process."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2KindExpression>,
        #[doc = "A partition ID identifies a grouping of entities. The grouping is always\nby project and namespace, however the namespace ID may be empty."]
        #[serde(
            rename = "partitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_id: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PartitionId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DatastoreOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DatastoreOptions {
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
    pub struct GooglePrivacyDlpV2DateShiftConfig {
        #[doc = "Points to the field that contains the context, for example, an entity id.\nIf set, must also set method. If set, shift will be consistent for the\ngiven context."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "Causes the shift to be computed based on this key and the context. This\nresults in the same shift for the same context and crypto_key."]
        #[serde(
            rename = "cryptoKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoKey>,
        #[doc = "For example, -5 means shift date to at most 5 days back in the past.\n[Required]"]
        #[serde(
            rename = "lowerBoundDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lower_bound_days: ::std::option::Option<i32>,
        #[doc = "Range of shift in days. Actual shift will be selected at random within this\nrange (inclusive ends). Negative means shift to earlier in time. Must not\nbe more than 365250 days (1000 years) each direction.\n\nFor example, 3 means shift date to at most 3 days into the future.\n[Required]"]
        #[serde(
            rename = "upperBoundDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upper_bound_days: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DateShiftConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DateShiftConfig {
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
    pub struct GooglePrivacyDlpV2DateTime {
        #[doc = "One or more of the following must be set. All fields are optional, but\nwhen set must be valid date or time values."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[serde(
            rename = "dayOfWeek",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_of_week: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DateTimeDayOfWeek>,
        #[serde(
            rename = "time",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time: ::std::option::Option<crate::schemas::GoogleTypeTimeOfDay>,
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TimeZone>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DateTime {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DateTime {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2DateTimeDayOfWeek {
        #[doc = "The unspecified day-of-week."]
        DayOfWeekUnspecified,
        #[doc = "The day-of-week of Friday."]
        Friday,
        #[doc = "The day-of-week of Monday."]
        Monday,
        #[doc = "The day-of-week of Saturday."]
        Saturday,
        #[doc = "The day-of-week of Sunday."]
        Sunday,
        #[doc = "The day-of-week of Thursday."]
        Thursday,
        #[doc = "The day-of-week of Tuesday."]
        Tuesday,
        #[doc = "The day-of-week of Wednesday."]
        Wednesday,
    }
    impl GooglePrivacyDlpV2DateTimeDayOfWeek {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2DateTimeDayOfWeek::DayOfWeekUnspecified => {
                    "DAY_OF_WEEK_UNSPECIFIED"
                }
                GooglePrivacyDlpV2DateTimeDayOfWeek::Friday => "FRIDAY",
                GooglePrivacyDlpV2DateTimeDayOfWeek::Monday => "MONDAY",
                GooglePrivacyDlpV2DateTimeDayOfWeek::Saturday => "SATURDAY",
                GooglePrivacyDlpV2DateTimeDayOfWeek::Sunday => "SUNDAY",
                GooglePrivacyDlpV2DateTimeDayOfWeek::Thursday => "THURSDAY",
                GooglePrivacyDlpV2DateTimeDayOfWeek::Tuesday => "TUESDAY",
                GooglePrivacyDlpV2DateTimeDayOfWeek::Wednesday => "WEDNESDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2DateTimeDayOfWeek {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2DateTimeDayOfWeek {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2DateTimeDayOfWeek, ()> {
            Ok(match s {
                "DAY_OF_WEEK_UNSPECIFIED" => {
                    GooglePrivacyDlpV2DateTimeDayOfWeek::DayOfWeekUnspecified
                }
                "FRIDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Friday,
                "MONDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Monday,
                "SATURDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Saturday,
                "SUNDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Sunday,
                "THURSDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Thursday,
                "TUESDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Tuesday,
                "WEDNESDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Wednesday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2DateTimeDayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2DateTimeDayOfWeek {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2DateTimeDayOfWeek {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY_OF_WEEK_UNSPECIFIED" => {
                    GooglePrivacyDlpV2DateTimeDayOfWeek::DayOfWeekUnspecified
                }
                "FRIDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Friday,
                "MONDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Monday,
                "SATURDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Saturday,
                "SUNDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Sunday,
                "THURSDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Thursday,
                "TUESDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Tuesday,
                "WEDNESDAY" => GooglePrivacyDlpV2DateTimeDayOfWeek::Wednesday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DateTimeDayOfWeek {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DateTimeDayOfWeek {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeidentifyConfig {
        #[doc = "Treat the dataset as free-form text and apply the same free text\ntransformation everywhere."]
        #[serde(
            rename = "infoTypeTransformations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type_transformations:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoTypeTransformations>,
        #[doc = "Treat the dataset as structured. Transformations can be applied to\nspecific locations within structured datasets, such as transforming\na column within a table."]
        #[serde(
            rename = "recordTransformations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub record_transformations:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RecordTransformations>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DeidentifyConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DeidentifyConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeidentifyContentRequest {
        #[doc = "Configuration for the de-identification of the content item.\nItems specified here will override the template referenced by the\ndeidentify_template_name argument."]
        #[serde(
            rename = "deidentifyConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deidentify_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeidentifyConfig>,
        #[doc = "Optional template to use. Any configuration directly specified in\ndeidentify_config will override those set in the template. Singular fields\nthat are set in this request will replace their corresponding fields in the\ntemplate. Repeated fields are appended. Singular sub-messages and groups\nare recursively merged."]
        #[serde(
            rename = "deidentifyTemplateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deidentify_template_name: ::std::option::Option<String>,
        #[doc = "Configuration for the inspector.\nItems specified here will override the template referenced by the\ninspect_template_name argument."]
        #[serde(
            rename = "inspectConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfig>,
        #[doc = "Optional template to use. Any configuration directly specified in\ninspect_config will override those set in the template. Singular fields\nthat are set in this request will replace their corresponding fields in the\ntemplate. Repeated fields are appended. Singular sub-messages and groups\nare recursively merged."]
        #[serde(
            rename = "inspectTemplateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_template_name: ::std::option::Option<String>,
        #[doc = "The item to de-identify. Will be treated as text."]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ContentItem>,
        #[doc = "The geographic location to process de-identification. Reserved for future\nextensions."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DeidentifyContentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DeidentifyContentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeidentifyContentResponse {
        #[doc = "The de-identified item."]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ContentItem>,
        #[doc = "An overview of the changes that were made on the `item`."]
        #[serde(
            rename = "overview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overview:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TransformationOverview>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DeidentifyContentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DeidentifyContentResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeidentifyTemplate {
        #[doc = "The creation timestamp of a inspectTemplate, output only field."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "///////////// // The core content of the template  // ///////////////"]
        #[serde(
            rename = "deidentifyConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deidentify_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeidentifyConfig>,
        #[doc = "Short description (max 256 chars)."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Display name (max 256 chars)."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The template name. Output only.\n\nThe template will have one of the following formats:\n`projects/PROJECT_ID/deidentifyTemplates/TEMPLATE_ID` OR\n`organizations/ORGANIZATION_ID/deidentifyTemplates/TEMPLATE_ID`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The last update timestamp of a inspectTemplate, output only field."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DeidentifyTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DeidentifyTemplate {
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
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
        #[doc = "Several auxiliary tables can be used in the analysis. Each custom_tag\nused to tag a quasi-identifiers field must appear in exactly one\nfield of one auxiliary table."]
        #[serde(
            rename = "auxiliaryTables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auxiliary_tables:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2StatisticalTable>>,
        #[doc = "Fields considered to be quasi-identifiers. No two fields can have the\nsame tag. [required]"]
        #[serde(
            rename = "quasiIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2QuasiId>>,
        #[doc = "ISO 3166-1 alpha-2 region code to use in the statistical modeling.\nRequired if no column is tagged with a region-specific InfoType (like\nUS_ZIP_5) or a region code."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket {
        #[doc = "Number of records within these probability bounds."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_size: ::std::option::Option<i64>,
        #[doc = "Total number of distinct quasi-identifier tuple values in this bucket."]
        #[serde(
            rename = "bucketValueCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_value_count: ::std::option::Option<i64>,
        #[doc = "Sample of quasi-identifier tuple values in this bucket. The total\nnumber of classes returned per bucket is capped at 20."]
        #[serde(
            rename = "bucketValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_values: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues>,
        >,
        #[doc = "Always greater than or equal to min_probability."]
        #[serde(
            rename = "maxProbability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_probability: ::std::option::Option<f64>,
        #[doc = "Between 0 and 1."]
        #[serde(
            rename = "minProbability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_probability: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector
        for GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues {
        #[doc = "The estimated probability that a given individual sharing these\nquasi-identifier values is in the dataset. This value, typically called\n\u{3b4}, is the ratio between the number of records in the dataset with these\nquasi-identifier values, and the total number of individuals (inside\n*and* outside the dataset) with these quasi-identifier values.\nFor example, if there are 15 individuals in the dataset who share the\nsame quasi-identifier values, and an estimated 100 people in the entire\npopulation with these values, then \u{3b4} is 0.15."]
        #[serde(
            rename = "estimatedProbability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub estimated_probability: ::std::option::Option<f64>,
        #[doc = "The quasi-identifier values."]
        #[serde(
            rename = "quasiIdsValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids_values: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Value>>,
    }
    impl ::google_field_selector::FieldSelector
        for GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationResult {
        #[doc = "The intervals [min_probability, max_probability) do not overlap. If a\nvalue doesn't correspond to any such interval, the associated frequency\nis zero. For example, the following records:\n{min_probability: 0, max_probability: 0.1, frequency: 17}\n{min_probability: 0.2, max_probability: 0.3, frequency: 42}\n{min_probability: 0.3, max_probability: 0.4, frequency: 99}\nmean that there are no record with an estimated probability in [0.1, 0.2)\nnor larger or equal to 0.4."]
        #[serde(
            rename = "deltaPresenceEstimationHistogram",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delta_presence_estimation_histogram: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DeltaPresenceEstimationResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DeltaPresenceEstimationResult {
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
    pub struct GooglePrivacyDlpV2DetectionRule {
        #[doc = "Hotword-based detection rule."]
        #[serde(
            rename = "hotwordRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hotword_rule: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2HotwordRule>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DetectionRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DetectionRule {
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
    pub struct GooglePrivacyDlpV2Dictionary {
        #[doc = "Newline-delimited file of words in Cloud Storage. Only a single file\nis accepted."]
        #[serde(
            rename = "cloudStoragePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_storage_path:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CloudStoragePath>,
        #[doc = "List of words or phrases to search for."]
        #[serde(
            rename = "wordList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word_list: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2WordList>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Dictionary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Dictionary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2DlpJob {
        #[doc = "Time when the job was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Time when the job finished."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "A stream of errors encountered running the job."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Error>>,
        #[doc = "Results from inspecting a data source."]
        #[serde(
            rename = "inspectDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_details:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectDataSourceDetails>,
        #[doc = "If created by a job trigger, the resource name of the trigger that\ninstantiated the job."]
        #[serde(
            rename = "jobTriggerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_trigger_name: ::std::option::Option<String>,
        #[doc = "The server-assigned name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of job."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DlpJobType>,
        #[doc = "Results from analyzing risk of a data source."]
        #[serde(
            rename = "riskDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub risk_details:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails>,
        #[doc = "Time when the job started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "State of a job."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DlpJobState>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DlpJob {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DlpJob {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2DlpJobType {
        DlpJobTypeUnspecified,
        #[doc = "The job inspected Google Cloud for sensitive data."]
        InspectJob,
        #[doc = "The job executed a Risk Analysis computation."]
        RiskAnalysisJob,
    }
    impl GooglePrivacyDlpV2DlpJobType {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2DlpJobType::DlpJobTypeUnspecified => "DLP_JOB_TYPE_UNSPECIFIED",
                GooglePrivacyDlpV2DlpJobType::InspectJob => "INSPECT_JOB",
                GooglePrivacyDlpV2DlpJobType::RiskAnalysisJob => "RISK_ANALYSIS_JOB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2DlpJobType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2DlpJobType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2DlpJobType, ()> {
            Ok(match s {
                "DLP_JOB_TYPE_UNSPECIFIED" => GooglePrivacyDlpV2DlpJobType::DlpJobTypeUnspecified,
                "INSPECT_JOB" => GooglePrivacyDlpV2DlpJobType::InspectJob,
                "RISK_ANALYSIS_JOB" => GooglePrivacyDlpV2DlpJobType::RiskAnalysisJob,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2DlpJobType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2DlpJobType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2DlpJobType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DLP_JOB_TYPE_UNSPECIFIED" => GooglePrivacyDlpV2DlpJobType::DlpJobTypeUnspecified,
                "INSPECT_JOB" => GooglePrivacyDlpV2DlpJobType::InspectJob,
                "RISK_ANALYSIS_JOB" => GooglePrivacyDlpV2DlpJobType::RiskAnalysisJob,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DlpJobType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DlpJobType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2DlpJobState {
        #[doc = "The job was canceled before it could complete."]
        Canceled,
        #[doc = "The job is no longer running."]
        Done,
        #[doc = "The job had an error and did not complete."]
        Failed,
        JobStateUnspecified,
        #[doc = "The job has not yet started."]
        Pending,
        #[doc = "The job is currently running."]
        Running,
    }
    impl GooglePrivacyDlpV2DlpJobState {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2DlpJobState::Canceled => "CANCELED",
                GooglePrivacyDlpV2DlpJobState::Done => "DONE",
                GooglePrivacyDlpV2DlpJobState::Failed => "FAILED",
                GooglePrivacyDlpV2DlpJobState::JobStateUnspecified => "JOB_STATE_UNSPECIFIED",
                GooglePrivacyDlpV2DlpJobState::Pending => "PENDING",
                GooglePrivacyDlpV2DlpJobState::Running => "RUNNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2DlpJobState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2DlpJobState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2DlpJobState, ()> {
            Ok(match s {
                "CANCELED" => GooglePrivacyDlpV2DlpJobState::Canceled,
                "DONE" => GooglePrivacyDlpV2DlpJobState::Done,
                "FAILED" => GooglePrivacyDlpV2DlpJobState::Failed,
                "JOB_STATE_UNSPECIFIED" => GooglePrivacyDlpV2DlpJobState::JobStateUnspecified,
                "PENDING" => GooglePrivacyDlpV2DlpJobState::Pending,
                "RUNNING" => GooglePrivacyDlpV2DlpJobState::Running,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2DlpJobState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2DlpJobState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2DlpJobState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELED" => GooglePrivacyDlpV2DlpJobState::Canceled,
                "DONE" => GooglePrivacyDlpV2DlpJobState::Done,
                "FAILED" => GooglePrivacyDlpV2DlpJobState::Failed,
                "JOB_STATE_UNSPECIFIED" => GooglePrivacyDlpV2DlpJobState::JobStateUnspecified,
                "PENDING" => GooglePrivacyDlpV2DlpJobState::Pending,
                "RUNNING" => GooglePrivacyDlpV2DlpJobState::Running,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DlpJobState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DlpJobState {
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
    pub struct GooglePrivacyDlpV2DocumentLocation {
        #[doc = "Offset of the line, from the beginning of the file, where the finding\nis located."]
        #[serde(
            rename = "fileOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub file_offset: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2DocumentLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2DocumentLocation {
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
    pub struct GooglePrivacyDlpV2EntityId {
        #[doc = "Composite key indicating which field contains the entity identifier."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2EntityId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2EntityId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2Error {
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "The times the error occurred."]
        #[serde(
            rename = "timestamps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamps: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Error {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Error {
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
    pub struct GooglePrivacyDlpV2ExcludeInfoTypes {
        #[doc = "InfoType list in ExclusionRule rule drops a finding when it overlaps or\ncontained within with a finding of an infoType from this list. For\nexample, for `InspectionRuleSet.info_types` containing \"PHONE_NUMBER\"`and`exclusion_rule`containing`exclude_info_types.info_types` with\n\"EMAIL_ADDRESS\" the phone number findings are dropped if they overlap\nwith EMAIL_ADDRESS finding.\nThat leads to \"555-222-2222@example.org\" to generate only a single\nfinding, namely email address."]
        #[serde(
            rename = "infoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_types: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoType>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ExcludeInfoTypes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ExcludeInfoTypes {
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
    pub struct GooglePrivacyDlpV2ExclusionRule {
        #[doc = "Dictionary which defines the rule."]
        #[serde(
            rename = "dictionary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dictionary: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Dictionary>,
        #[doc = "Set of infoTypes for which findings would affect this rule."]
        #[serde(
            rename = "excludeInfoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_info_types:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ExcludeInfoTypes>,
        #[doc = "How the rule is applied, see MatchingType documentation for details."]
        #[serde(
            rename = "matchingType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matching_type:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ExclusionRuleMatchingType>,
        #[doc = "Regular expression which defines the rule."]
        #[serde(
            rename = "regex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regex: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Regex>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ExclusionRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ExclusionRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2ExclusionRuleMatchingType {
        #[doc = "Full match.\n\n* Dictionary: join of Dictionary results matched complete finding quote\n* Regex: all regex matches fill a finding quote start to end\n* Exclude info type: completely inside affecting info types findings"]
        MatchingTypeFullMatch,
        #[doc = "Inverse match.\n\n* Dictionary: no tokens in the finding match the dictionary\n* Regex: finding doesn't match the regex\n* Exclude info type: no intersection with affecting info types findings"]
        MatchingTypeInverseMatch,
        #[doc = "Partial match.\n\n* Dictionary: at least one of the tokens in the finding matches\n* Regex: substring of the finding matches\n* Exclude info type: intersects with affecting info types findings"]
        MatchingTypePartialMatch,
        #[doc = "Invalid."]
        MatchingTypeUnspecified,
    }
    impl GooglePrivacyDlpV2ExclusionRuleMatchingType {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeFullMatch => {
                    "MATCHING_TYPE_FULL_MATCH"
                }
                GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeInverseMatch => {
                    "MATCHING_TYPE_INVERSE_MATCH"
                }
                GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypePartialMatch => {
                    "MATCHING_TYPE_PARTIAL_MATCH"
                }
                GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeUnspecified => {
                    "MATCHING_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2ExclusionRuleMatchingType, ()> {
            Ok(match s {
                "MATCHING_TYPE_FULL_MATCH" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeFullMatch
                }
                "MATCHING_TYPE_INVERSE_MATCH" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeInverseMatch
                }
                "MATCHING_TYPE_PARTIAL_MATCH" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypePartialMatch
                }
                "MATCHING_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MATCHING_TYPE_FULL_MATCH" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeFullMatch
                }
                "MATCHING_TYPE_INVERSE_MATCH" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeInverseMatch
                }
                "MATCHING_TYPE_PARTIAL_MATCH" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypePartialMatch
                }
                "MATCHING_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ExclusionRuleMatchingType::MatchingTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ExclusionRuleMatchingType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Expressions {
        #[serde(
            rename = "conditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditions: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Conditions>,
        #[doc = "The operator to apply to the result of conditions. Default and currently\nonly supported value is `AND`."]
        #[serde(
            rename = "logicalOperator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logical_operator:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ExpressionsLogicalOperator>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Expressions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Expressions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2ExpressionsLogicalOperator {
        And,
        LogicalOperatorUnspecified,
    }
    impl GooglePrivacyDlpV2ExpressionsLogicalOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2ExpressionsLogicalOperator::And => "AND",
                GooglePrivacyDlpV2ExpressionsLogicalOperator::LogicalOperatorUnspecified => {
                    "LOGICAL_OPERATOR_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2ExpressionsLogicalOperator {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2ExpressionsLogicalOperator {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2ExpressionsLogicalOperator, ()> {
            Ok(match s {
                "AND" => GooglePrivacyDlpV2ExpressionsLogicalOperator::And,
                "LOGICAL_OPERATOR_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ExpressionsLogicalOperator::LogicalOperatorUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2ExpressionsLogicalOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2ExpressionsLogicalOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2ExpressionsLogicalOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => GooglePrivacyDlpV2ExpressionsLogicalOperator::And,
                "LOGICAL_OPERATOR_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ExpressionsLogicalOperator::LogicalOperatorUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ExpressionsLogicalOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ExpressionsLogicalOperator {
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
    pub struct GooglePrivacyDlpV2FieldId {
        #[doc = "Name describing the field."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2FieldId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2FieldId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2FieldTransformation {
        #[doc = "Only apply the transformation if the condition evaluates to true for the\ngiven `RecordCondition`. The conditions are allowed to reference fields\nthat are not used in the actual transformation. [optional]\n\nExample Use Cases:\n\n* Apply a different bucket transformation to an age column if the zip code\n  column for the same record is within a specific range.\n* Redact a field if the date of birth field is greater than 85."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RecordCondition>,
        #[doc = "Input field(s) to apply the transformation to. [required]"]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldId>>,
        #[doc = "Treat the contents of the field as free text, and selectively\ntransform content that matches an `InfoType`."]
        #[serde(
            rename = "infoTypeTransformations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type_transformations:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoTypeTransformations>,
        #[doc = "Apply the transformation to the entire field."]
        #[serde(
            rename = "primitiveTransformation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primitive_transformation:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PrimitiveTransformation>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2FieldTransformation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2FieldTransformation {
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
    pub struct GooglePrivacyDlpV2FileSet {
        #[doc = "The regex-filtered set of files to scan. Exactly one of `url` or\n`regex_file_set` must be set."]
        #[serde(
            rename = "regexFileSet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regex_file_set:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CloudStorageRegexFileSet>,
        #[doc = "The Cloud Storage url of the file(s) to scan, in the format\n`gs://<bucket>/<path>`. Trailing wildcard in the path is allowed.\n\nIf the url ends in a trailing slash, the bucket or directory represented\nby the url will be scanned non-recursively (content in sub-directories\nwill not be scanned). This means that `gs://mybucket/` is equivalent to\n`gs://mybucket/*`, and `gs://mybucket/directory/` is equivalent to\n`gs://mybucket/directory/*`.\n\nExactly one of `url` or `regex_file_set` must be set."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2FileSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2FileSet {
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
    pub struct GooglePrivacyDlpV2Finding {
        #[doc = "Timestamp when finding was detected."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The type of content that might have been found.\nProvided if `excluded_types` is false."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
        #[doc = "Confidence of how likely it is that the `info_type` is correct."]
        #[serde(
            rename = "likelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub likelihood: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FindingLikelihood>,
        #[doc = "Where the content was found."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Location>,
        #[doc = "The content that was found. Even if the content is not textual, it\nmay be converted to a textual representation here.\nProvided if `include_quote` is true and the finding is\nless than or equal to 4096 bytes long. If the finding exceeds 4096 bytes\nin length, the quote may be omitted."]
        #[serde(
            rename = "quote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quote: ::std::option::Option<String>,
        #[doc = "Contains data parsed from quotes. Only populated if include_quote was set\nto true and a supported infoType was requested. Currently supported\ninfoTypes: DATE, DATE_OF_BIRTH and TIME."]
        #[serde(
            rename = "quoteInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quote_info: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2QuoteInfo>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Finding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Finding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2FindingLikelihood {
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        Likely,
        #[doc = "Some matching elements."]
        Possible,
        Unlikely,
        #[doc = "Many matching elements."]
        VeryLikely,
        #[doc = "Few matching elements."]
        VeryUnlikely,
    }
    impl GooglePrivacyDlpV2FindingLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2FindingLikelihood::LikelihoodUnspecified => {
                    "LIKELIHOOD_UNSPECIFIED"
                }
                GooglePrivacyDlpV2FindingLikelihood::Likely => "LIKELY",
                GooglePrivacyDlpV2FindingLikelihood::Possible => "POSSIBLE",
                GooglePrivacyDlpV2FindingLikelihood::Unlikely => "UNLIKELY",
                GooglePrivacyDlpV2FindingLikelihood::VeryLikely => "VERY_LIKELY",
                GooglePrivacyDlpV2FindingLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2FindingLikelihood {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2FindingLikelihood {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2FindingLikelihood, ()> {
            Ok(match s {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2FindingLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2FindingLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2FindingLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2FindingLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2FindingLikelihood::VeryLikely,
                "VERY_UNLIKELY" => GooglePrivacyDlpV2FindingLikelihood::VeryUnlikely,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2FindingLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2FindingLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2FindingLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2FindingLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2FindingLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2FindingLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2FindingLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2FindingLikelihood::VeryLikely,
                "VERY_UNLIKELY" => GooglePrivacyDlpV2FindingLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2FindingLikelihood {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2FindingLikelihood {
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
    pub struct GooglePrivacyDlpV2FindingLimits {
        #[doc = "Configuration of findings limit given for specified infoTypes."]
        #[serde(
            rename = "maxFindingsPerInfoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_findings_per_info_type:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoTypeLimit>>,
        #[doc = "Max number of findings that will be returned for each item scanned.\nWhen set within `InspectDataSourceRequest`,\nthe maximum returned is 2000 regardless if this is set higher.\nWhen set within `InspectContentRequest`, this field is ignored."]
        #[serde(
            rename = "maxFindingsPerItem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_findings_per_item: ::std::option::Option<i32>,
        #[doc = "Max number of findings that will be returned per request/job.\nWhen set within `InspectContentRequest`, the maximum returned is 2000\nregardless if this is set higher."]
        #[serde(
            rename = "maxFindingsPerRequest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_findings_per_request: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2FindingLimits {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2FindingLimits {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2FixedSizeBucketingConfig {
        #[doc = "Size of each bucket (except for minimum and maximum buckets). So if\n`lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the\nfollowing buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60,\n60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works. [Required]."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_size: ::std::option::Option<f64>,
        #[doc = "Lower bound value of buckets. All values less than `lower_bound` are\ngrouped together into a single bucket; for example if `lower_bound` = 10,\nthen all values less than 10 are replaced with the value \u{201c}-10\u{201d}. [Required]."]
        #[serde(
            rename = "lowerBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lower_bound: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
        #[doc = "Upper bound value of buckets. All values greater than upper_bound are\ngrouped together into a single bucket; for example if `upper_bound` = 89,\nthen all values greater than 89 are replaced with the value \u{201c}89+\u{201d}.\n[Required]."]
        #[serde(
            rename = "upperBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upper_bound: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2FixedSizeBucketingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2FixedSizeBucketingConfig {
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
    pub struct GooglePrivacyDlpV2HotwordRule {
        #[doc = "Regular expression pattern defining what qualifies as a hotword."]
        #[serde(
            rename = "hotwordRegex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hotword_regex: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Regex>,
        #[doc = "Likelihood adjustment to apply to all matching findings."]
        #[serde(
            rename = "likelihoodAdjustment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub likelihood_adjustment:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2LikelihoodAdjustment>,
        #[doc = "Proximity of the finding within which the entire hotword must reside.\nThe total length of the window cannot exceed 1000 characters. Note that\nthe finding itself will be included in the window, so that hotwords may\nbe used to match substrings of the finding itself. For example, the\ncertainty of a phone number regex \"(\\d{3}) \\d{3}-\\d{4}\" could be\nadjusted upwards if the area code is known to be the local area code of\na company office using the hotword regex \"(xxx)\", where \"xxx\"\nis the area code in question."]
        #[serde(
            rename = "proximity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proximity: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Proximity>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2HotwordRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2HotwordRule {
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
    pub struct GooglePrivacyDlpV2ImageLocation {
        #[doc = "Bounding boxes locating the pixels within the image containing the finding."]
        #[serde(
            rename = "boundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_boxes:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2BoundingBox>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ImageLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ImageLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ImageRedactionConfig {
        #[doc = "Only one per info_type should be provided per request. If not\nspecified, and redact_all_text is false, the DLP API will redact all\ntext that it matches against all info_types that are found, but not\nspecified in another ImageRedactionConfig."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
        #[doc = "If true, all text found in the image, regardless whether it matches an\ninfo_type, is redacted. Only one should be provided."]
        #[serde(
            rename = "redactAllText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redact_all_text: ::std::option::Option<bool>,
        #[doc = "The color to use when redacting content from an image. If not specified,\nthe default is black."]
        #[serde(
            rename = "redactionColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redaction_color: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Color>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ImageRedactionConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ImageRedactionConfig {
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
    pub struct GooglePrivacyDlpV2InfoType {
        #[doc = "Name of the information type. Either a name of your choosing when\ncreating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying\na built-in type. InfoType names should conform to the pattern\n[a-zA-Z0-9_]{1,64}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InfoType {
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
    pub struct GooglePrivacyDlpV2InfoTypeDescription {
        #[doc = "Description of the infotype. Translated when language is provided in the\nrequest."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Human readable form of the infoType name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Internal name of the infoType."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Which parts of the API supports this InfoType."]
        #[serde(
            rename = "supportedBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_by: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InfoTypeDescription {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InfoTypeDescription {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        EnumTypeUnspecified,
        Inspect,
        RiskAnalysis,
    }
    impl GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::EnumTypeUnspecified => {
                    "ENUM_TYPE_UNSPECIFIED"
                }
                GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::Inspect => "INSPECT",
                GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::RiskAnalysis => {
                    "RISK_ANALYSIS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems, ()>
        {
            Ok(match s {
                "ENUM_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::EnumTypeUnspecified
                }
                "INSPECT" => GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::Inspect,
                "RISK_ANALYSIS" => {
                    GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::RiskAnalysis
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENUM_TYPE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::EnumTypeUnspecified
                }
                "INSPECT" => GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::Inspect,
                "RISK_ANALYSIS" => {
                    GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems::RiskAnalysis
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
        for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByItems
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
    pub struct GooglePrivacyDlpV2InfoTypeLimit {
        #[doc = "Type of information the findings limit applies to. Only one limit per\ninfo_type should be provided. If InfoTypeLimit does not have an\ninfo_type, the DLP API applies the limit against all info_types that\nare found but not specified in another InfoTypeLimit."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
        #[doc = "Max findings limit for the given infoType."]
        #[serde(
            rename = "maxFindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_findings: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InfoTypeLimit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InfoTypeLimit {
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
    pub struct GooglePrivacyDlpV2InfoTypeStats {
        #[doc = "Number of findings for this infoType."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "The type of finding this stat is for."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InfoTypeStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InfoTypeStats {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2InfoTypeTransformation {
        #[doc = "InfoTypes to apply the transformation to. An empty list will cause\nthis transformation to apply to all findings that correspond to\ninfoTypes that were requested in `InspectConfig`."]
        #[serde(
            rename = "infoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_types: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoType>>,
        #[doc = "Primitive transformation to apply to the infoType. [required]"]
        #[serde(
            rename = "primitiveTransformation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primitive_transformation:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PrimitiveTransformation>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InfoTypeTransformation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InfoTypeTransformation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2InfoTypeTransformations {
        #[doc = "Transformation for each infoType. Cannot specify more than one\nfor a given infoType. [required]"]
        #[serde(
            rename = "transformations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transformations:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoTypeTransformation>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InfoTypeTransformations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InfoTypeTransformations {
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
    pub struct GooglePrivacyDlpV2InspectConfig {
        #[doc = "List of options defining data content to scan.\nIf empty, text, images, and other content will be included."]
        #[serde(
            rename = "contentOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_options: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2InspectConfigContentOptionsItems>,
        >,
        #[doc = "CustomInfoTypes provided by the user. See\nhttps://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more."]
        #[serde(
            rename = "customInfoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_info_types:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2CustomInfoType>>,
        #[doc = "When true, excludes type information of the findings."]
        #[serde(
            rename = "excludeInfoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_info_types: ::std::option::Option<bool>,
        #[doc = "When true, a contextual quote from the data that triggered a finding is\nincluded in the response; see Finding.quote."]
        #[serde(
            rename = "includeQuote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_quote: ::std::option::Option<bool>,
        #[doc = "Restricts what info_types to look for. The values must correspond to\nInfoType values returned by ListInfoTypes or listed at\nhttps://cloud.google.com/dlp/docs/infotypes-reference.\n\nWhen no InfoTypes or CustomInfoTypes are specified in a request, the\nsystem may automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.\n\nThe special InfoType name \"ALL_BASIC\" can be used to trigger all detectors,\nbut may change over time as new InfoTypes are added. If you need precise\ncontrol and predictability as to what detectors are run you should specify\nspecific InfoTypes listed in the reference."]
        #[serde(
            rename = "infoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_types: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoType>>,
        #[serde(
            rename = "limits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub limits: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FindingLimits>,
        #[doc = "Only returns findings equal or above this threshold. The default is\nPOSSIBLE.\nSee https://cloud.google.com/dlp/docs/likelihood to learn more."]
        #[serde(
            rename = "minLikelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_likelihood:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfigMinLikelihood>,
        #[doc = "Set of rules to apply to the findings for this InspectConfig.\nExclusion rules, contained in the set are executed in the end, other\nrules are executed in the order they are specified for each info type."]
        #[serde(
            rename = "ruleSet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule_set:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InspectionRuleSet>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        ContentImage,
        ContentText,
        ContentUnspecified,
    }
    impl GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentImage => "CONTENT_IMAGE",
                GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentText => "CONTENT_TEXT",
                GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentUnspecified => {
                    "CONTENT_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2InspectConfigContentOptionsItems, ()> {
            Ok(match s {
                "CONTENT_IMAGE" => GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentImage,
                "CONTENT_TEXT" => GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentText,
                "CONTENT_UNSPECIFIED" => {
                    GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_IMAGE" => GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentImage,
                "CONTENT_TEXT" => GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentText,
                "CONTENT_UNSPECIFIED" => {
                    GooglePrivacyDlpV2InspectConfigContentOptionsItems::ContentUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectConfigContentOptionsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2InspectConfigMinLikelihood {
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        Likely,
        #[doc = "Some matching elements."]
        Possible,
        Unlikely,
        #[doc = "Many matching elements."]
        VeryLikely,
        #[doc = "Few matching elements."]
        VeryUnlikely,
    }
    impl GooglePrivacyDlpV2InspectConfigMinLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2InspectConfigMinLikelihood::LikelihoodUnspecified => {
                    "LIKELIHOOD_UNSPECIFIED"
                }
                GooglePrivacyDlpV2InspectConfigMinLikelihood::Likely => "LIKELY",
                GooglePrivacyDlpV2InspectConfigMinLikelihood::Possible => "POSSIBLE",
                GooglePrivacyDlpV2InspectConfigMinLikelihood::Unlikely => "UNLIKELY",
                GooglePrivacyDlpV2InspectConfigMinLikelihood::VeryLikely => "VERY_LIKELY",
                GooglePrivacyDlpV2InspectConfigMinLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2InspectConfigMinLikelihood, ()> {
            Ok(match s {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2InspectConfigMinLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2InspectConfigMinLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::VeryLikely,
                "VERY_UNLIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::VeryUnlikely,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2InspectConfigMinLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2InspectConfigMinLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::VeryLikely,
                "VERY_UNLIKELY" => GooglePrivacyDlpV2InspectConfigMinLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectConfigMinLikelihood {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2InspectContentRequest {
        #[doc = "Configuration for the inspector. What specified here will override\nthe template referenced by the inspect_template_name argument."]
        #[serde(
            rename = "inspectConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfig>,
        #[doc = "Optional template to use. Any configuration directly specified in\ninspect_config will override those set in the template. Singular fields\nthat are set in this request will replace their corresponding fields in the\ntemplate. Repeated fields are appended. Singular sub-messages and groups\nare recursively merged."]
        #[serde(
            rename = "inspectTemplateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_template_name: ::std::option::Option<String>,
        #[doc = "The item to inspect."]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ContentItem>,
        #[doc = "The geographic location to process content inspection. Reserved for future\nextensions."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectContentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectContentRequest {
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
    pub struct GooglePrivacyDlpV2InspectContentResponse {
        #[doc = "The findings."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectResult>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectContentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectContentResponse {
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
    pub struct GooglePrivacyDlpV2InspectDataSourceDetails {
        #[doc = "The configuration used for this job."]
        #[serde(
            rename = "requestedOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_options:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RequestedOptions>,
        #[doc = "A summary of the outcome of this inspect job."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Result>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectDataSourceDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectDataSourceDetails {
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
    pub struct GooglePrivacyDlpV2InspectJobConfig {
        #[doc = "Actions to execute at the completion of the job."]
        #[serde(
            rename = "actions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actions: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Action>>,
        #[doc = "How and what to scan for."]
        #[serde(
            rename = "inspectConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfig>,
        #[doc = "If provided, will be used as the default for all values in InspectConfig.\n`inspect_config` will be merged into the values persisted as part of the\ntemplate."]
        #[serde(
            rename = "inspectTemplateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_template_name: ::std::option::Option<String>,
        #[doc = "The data to scan."]
        #[serde(
            rename = "storageConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StorageConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectJobConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectJobConfig {
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
    pub struct GooglePrivacyDlpV2InspectResult {
        #[doc = "List of findings for an item."]
        #[serde(
            rename = "findings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub findings: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Finding>>,
        #[doc = "If true, then this item might have more findings than were returned,\nand the findings returned are an arbitrary subset of all findings.\nThe findings list might be truncated because the input items were too\nlarge, or because the server reached the maximum amount of resources\nallowed for a single API call. For best results, divide the input into\nsmaller batches."]
        #[serde(
            rename = "findingsTruncated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub findings_truncated: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectResult {
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
    pub struct GooglePrivacyDlpV2InspectTemplate {
        #[doc = "The creation timestamp of a inspectTemplate, output only field."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Short description (max 256 chars)."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Display name (max 256 chars)."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The core content of the template. Configuration of the scanning process."]
        #[serde(
            rename = "inspectConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfig>,
        #[doc = "The template name. Output only.\n\nThe template will have one of the following formats:\n`projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR\n`organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The last update timestamp of a inspectTemplate, output only field."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectTemplate {
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
    pub struct GooglePrivacyDlpV2InspectionRule {
        #[doc = "Exclusion rule."]
        #[serde(
            rename = "exclusionRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusion_rule: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ExclusionRule>,
        #[doc = "Hotword-based detection rule."]
        #[serde(
            rename = "hotwordRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hotword_rule: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2HotwordRule>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectionRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectionRule {
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
    pub struct GooglePrivacyDlpV2InspectionRuleSet {
        #[doc = "List of infoTypes this rule set is applied to."]
        #[serde(
            rename = "infoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_types: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoType>>,
        #[doc = "Set of rules to be applied to infoTypes. The rules are applied in order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InspectionRule>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2InspectionRuleSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2InspectionRuleSet {
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
    pub struct GooglePrivacyDlpV2JobNotificationEmails;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2JobNotificationEmails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2JobNotificationEmails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2JobTrigger {
        #[doc = "The creation timestamp of a triggeredJob, output only field."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "User provided description (max 256 chars)"]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Display name (max 100 chars)"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A stream of errors encountered when the trigger was activated. Repeated\nerrors may result in the JobTrigger automatically being paused.\nWill return the last 100 errors. Whenever the JobTrigger is modified\nthis list will be cleared. Output only field."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Error>>,
        #[serde(
            rename = "inspectJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_job: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectJobConfig>,
        #[doc = "The timestamp of the last time this trigger executed, output only field."]
        #[serde(
            rename = "lastRunTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_run_time: ::std::option::Option<String>,
        #[doc = "Unique resource name for the triggeredJob, assigned by the service when the\ntriggeredJob is created, for example\n`projects/dlp-test-project/jobTriggers/53234423`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A status for this trigger. [required]"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2JobTriggerStatus>,
        #[doc = "A list of triggers which will be OR'ed together. Only one in the list\nneeds to trigger for a job to be started. The list may contain only\na single Schedule trigger and must have at least one object."]
        #[serde(
            rename = "triggers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triggers: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Trigger>>,
        #[doc = "The last update timestamp of a triggeredJob, output only field."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2JobTrigger {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2JobTrigger {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2JobTriggerStatus {
        #[doc = "Trigger is cancelled and can not be resumed."]
        Cancelled,
        #[doc = "Trigger is healthy."]
        Healthy,
        #[doc = "Trigger is temporarily paused."]
        Paused,
        StatusUnspecified,
    }
    impl GooglePrivacyDlpV2JobTriggerStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2JobTriggerStatus::Cancelled => "CANCELLED",
                GooglePrivacyDlpV2JobTriggerStatus::Healthy => "HEALTHY",
                GooglePrivacyDlpV2JobTriggerStatus::Paused => "PAUSED",
                GooglePrivacyDlpV2JobTriggerStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2JobTriggerStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2JobTriggerStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2JobTriggerStatus, ()> {
            Ok(match s {
                "CANCELLED" => GooglePrivacyDlpV2JobTriggerStatus::Cancelled,
                "HEALTHY" => GooglePrivacyDlpV2JobTriggerStatus::Healthy,
                "PAUSED" => GooglePrivacyDlpV2JobTriggerStatus::Paused,
                "STATUS_UNSPECIFIED" => GooglePrivacyDlpV2JobTriggerStatus::StatusUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2JobTriggerStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2JobTriggerStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2JobTriggerStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GooglePrivacyDlpV2JobTriggerStatus::Cancelled,
                "HEALTHY" => GooglePrivacyDlpV2JobTriggerStatus::Healthy,
                "PAUSED" => GooglePrivacyDlpV2JobTriggerStatus::Paused,
                "STATUS_UNSPECIFIED" => GooglePrivacyDlpV2JobTriggerStatus::StatusUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2JobTriggerStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2JobTriggerStatus {
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
    pub struct GooglePrivacyDlpV2KAnonymityConfig {
        #[doc = "Optional message indicating that multiple rows might be associated to a\nsingle individual. If the same entity_id is associated to multiple\nquasi-identifier tuples over distinct rows, we consider the entire\ncollection of tuples as the composite quasi-identifier. This collection\nis a multiset: the order in which the different tuples appear in the\ndataset is ignored, but their frequency is taken into account.\n\nImportant note: a maximum of 1000 rows can be associated to a single\nentity ID. If more rows are associated with the same entity ID, some\nmight be ignored."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2EntityId>,
        #[doc = "Set of fields to compute k-anonymity over. When multiple fields are\nspecified, they are considered a single composite key. Structs and\nrepeated data types are not supported; however, nested fields are\nsupported so long as they are not structs themselves or nested within\na repeated field."]
        #[serde(
            rename = "quasiIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldId>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KAnonymityConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KAnonymityConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2KAnonymityEquivalenceClass {
        #[doc = "Size of the equivalence class, for example number of rows with the\nabove set of values."]
        #[serde(
            rename = "equivalenceClassSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub equivalence_class_size: ::std::option::Option<i64>,
        #[doc = "Set of values defining the equivalence class. One value per\nquasi-identifier column in the original KAnonymity metric message.\nThe order is always the same as the original request."]
        #[serde(
            rename = "quasiIdsValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids_values: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Value>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KAnonymityEquivalenceClass {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KAnonymityEquivalenceClass {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2KAnonymityHistogramBucket {
        #[doc = "Total number of equivalence classes in this bucket."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_size: ::std::option::Option<i64>,
        #[doc = "Total number of distinct equivalence classes in this bucket."]
        #[serde(
            rename = "bucketValueCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_value_count: ::std::option::Option<i64>,
        #[doc = "Sample of equivalence classes in this bucket. The total number of\nclasses returned per bucket is capped at 20."]
        #[serde(
            rename = "bucketValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_values: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2KAnonymityEquivalenceClass>,
        >,
        #[doc = "Lower bound on the size of the equivalence classes in this bucket."]
        #[serde(
            rename = "equivalenceClassSizeLowerBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub equivalence_class_size_lower_bound: ::std::option::Option<i64>,
        #[doc = "Upper bound on the size of the equivalence classes in this bucket."]
        #[serde(
            rename = "equivalenceClassSizeUpperBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub equivalence_class_size_upper_bound: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KAnonymityHistogramBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KAnonymityHistogramBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2KAnonymityResult {
        #[doc = "Histogram of k-anonymity equivalence classes."]
        #[serde(
            rename = "equivalenceClassHistogramBuckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub equivalence_class_histogram_buckets:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2KAnonymityHistogramBucket>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KAnonymityResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KAnonymityResult {
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
    pub struct GooglePrivacyDlpV2KMapEstimationConfig {
        #[doc = "Several auxiliary tables can be used in the analysis. Each custom_tag\nused to tag a quasi-identifiers column must appear in exactly one column\nof one auxiliary table."]
        #[serde(
            rename = "auxiliaryTables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auxiliary_tables:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2AuxiliaryTable>>,
        #[doc = "Fields considered to be quasi-identifiers. No two columns can have the\nsame tag. [required]"]
        #[serde(
            rename = "quasiIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2TaggedField>>,
        #[doc = "ISO 3166-1 alpha-2 region code to use in the statistical modeling.\nRequired if no column is tagged with a region-specific InfoType (like\nUS_ZIP_5) or a region code."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KMapEstimationConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KMapEstimationConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2KMapEstimationHistogramBucket {
        #[doc = "Number of records within these anonymity bounds."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_size: ::std::option::Option<i64>,
        #[doc = "Total number of distinct quasi-identifier tuple values in this bucket."]
        #[serde(
            rename = "bucketValueCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_value_count: ::std::option::Option<i64>,
        #[doc = "Sample of quasi-identifier tuple values in this bucket. The total\nnumber of classes returned per bucket is capped at 20."]
        #[serde(
            rename = "bucketValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_values: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2KMapEstimationQuasiIdValues>,
        >,
        #[doc = "Always greater than or equal to min_anonymity."]
        #[serde(
            rename = "maxAnonymity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_anonymity: ::std::option::Option<i64>,
        #[doc = "Always positive."]
        #[serde(
            rename = "minAnonymity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_anonymity: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KMapEstimationHistogramBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KMapEstimationHistogramBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
        #[doc = "The estimated anonymity for these quasi-identifier values."]
        #[serde(
            rename = "estimatedAnonymity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub estimated_anonymity: ::std::option::Option<i64>,
        #[doc = "The quasi-identifier values."]
        #[serde(
            rename = "quasiIdsValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids_values: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Value>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2KMapEstimationResult {
        #[doc = "The intervals [min_anonymity, max_anonymity] do not overlap. If a value\ndoesn't correspond to any such interval, the associated frequency is\nzero. For example, the following records:\n{min_anonymity: 1, max_anonymity: 1, frequency: 17}\n{min_anonymity: 2, max_anonymity: 3, frequency: 42}\n{min_anonymity: 5, max_anonymity: 10, frequency: 99}\nmean that there are no record with an estimated anonymity of 4, 5, or\nlarger than 10."]
        #[serde(
            rename = "kMapEstimationHistogram",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub k_map_estimation_histogram: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2KMapEstimationHistogramBucket>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KMapEstimationResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KMapEstimationResult {
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
    pub struct GooglePrivacyDlpV2Key {
        #[doc = "Entities are partitioned into subsets, currently identified by a project\nID and namespace ID.\nQueries are scoped to a single partition."]
        #[serde(
            rename = "partitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_id: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PartitionId>,
        #[doc = "The entity path.\nAn entity path consists of one or more elements composed of a kind and a\nstring or numerical identifier, which identify entities. The first\nelement identifies a *root entity*, the second element identifies\na *child* of the root entity, the third element identifies a child of the\nsecond entity, and so forth. The entities identified by all prefixes of\nthe path are called the element's *ancestors*.\n\nA path can never be empty, and a path can have at most 100 elements."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2PathElement>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Key {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Key {
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
    pub struct GooglePrivacyDlpV2KindExpression {
        #[doc = "The name of the kind."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KindExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KindExpression {
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
    pub struct GooglePrivacyDlpV2KmsWrappedCryptoKey {
        #[doc = "The resource name of the KMS CryptoKey to use for unwrapping. [required]"]
        #[serde(
            rename = "cryptoKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_key_name: ::std::option::Option<String>,
        #[doc = "The wrapped data crypto key. [required]"]
        #[serde(
            rename = "wrappedKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wrapped_key: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2KmsWrappedCryptoKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2KmsWrappedCryptoKey {
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
    pub struct GooglePrivacyDlpV2LDiversityConfig {
        #[doc = "Set of quasi-identifiers indicating how equivalence classes are\ndefined for the l-diversity computation. When multiple fields are\nspecified, they are considered a single composite key."]
        #[serde(
            rename = "quasiIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldId>>,
        #[doc = "Sensitive field for computing the l-value."]
        #[serde(
            rename = "sensitiveAttribute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sensitive_attribute: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LDiversityConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LDiversityConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2LDiversityEquivalenceClass {
        #[doc = "Size of the k-anonymity equivalence class."]
        #[serde(
            rename = "equivalenceClassSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub equivalence_class_size: ::std::option::Option<i64>,
        #[doc = "Number of distinct sensitive values in this equivalence class."]
        #[serde(
            rename = "numDistinctSensitiveValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_distinct_sensitive_values: ::std::option::Option<i64>,
        #[doc = "Quasi-identifier values defining the k-anonymity equivalence\nclass. The order is always the same as the original request."]
        #[serde(
            rename = "quasiIdsValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids_values: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Value>>,
        #[doc = "Estimated frequencies of top sensitive values."]
        #[serde(
            rename = "topSensitiveValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_sensitive_values:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2ValueFrequency>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LDiversityEquivalenceClass {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LDiversityEquivalenceClass {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2LDiversityHistogramBucket {
        #[doc = "Total number of equivalence classes in this bucket."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_size: ::std::option::Option<i64>,
        #[doc = "Total number of distinct equivalence classes in this bucket."]
        #[serde(
            rename = "bucketValueCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bucket_value_count: ::std::option::Option<i64>,
        #[doc = "Sample of equivalence classes in this bucket. The total number of\nclasses returned per bucket is capped at 20."]
        #[serde(
            rename = "bucketValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_values: ::std::option::Option<
            Vec<crate::schemas::GooglePrivacyDlpV2LDiversityEquivalenceClass>,
        >,
        #[doc = "Lower bound on the sensitive value frequencies of the equivalence\nclasses in this bucket."]
        #[serde(
            rename = "sensitiveValueFrequencyLowerBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub sensitive_value_frequency_lower_bound: ::std::option::Option<i64>,
        #[doc = "Upper bound on the sensitive value frequencies of the equivalence\nclasses in this bucket."]
        #[serde(
            rename = "sensitiveValueFrequencyUpperBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub sensitive_value_frequency_upper_bound: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LDiversityHistogramBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LDiversityHistogramBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2LDiversityResult {
        #[doc = "Histogram of l-diversity equivalence class sensitive value frequencies."]
        #[serde(
            rename = "sensitiveValueFrequencyHistogramBuckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sensitive_value_frequency_histogram_buckets:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2LDiversityHistogramBucket>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LDiversityResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LDiversityResult {
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
    pub struct GooglePrivacyDlpV2LargeCustomDictionaryConfig {
        #[doc = "Field in a BigQuery table where each cell represents a dictionary phrase."]
        #[serde(
            rename = "bigQueryField",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub big_query_field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryField>,
        #[doc = "Set of files containing newline-delimited lists of dictionary phrases."]
        #[serde(
            rename = "cloudStorageFileSet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_storage_file_set:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CloudStorageFileSet>,
        #[doc = "Location to store dictionary artifacts in Google Cloud Storage. These files\nwill only be accessible by project owners and the DLP API. If any of these\nartifacts are modified, the dictionary is considered invalid and can no\nlonger be used."]
        #[serde(
            rename = "outputPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_path: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CloudStoragePath>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LargeCustomDictionaryConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LargeCustomDictionaryConfig {
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
    pub struct GooglePrivacyDlpV2LargeCustomDictionaryStats {
        #[doc = "Approximate number of distinct phrases in the dictionary."]
        #[serde(
            rename = "approxNumPhrases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub approx_num_phrases: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LargeCustomDictionaryStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LargeCustomDictionaryStats {
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
    pub struct GooglePrivacyDlpV2LikelihoodAdjustment {
        #[doc = "Set the likelihood of a finding to a fixed value."]
        #[serde(
            rename = "fixedLikelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_likelihood: ::std::option::Option<
            crate::schemas::GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood,
        >,
        #[doc = "Increase or decrease the likelihood by the specified number of\nlevels. For example, if a finding would be `POSSIBLE` without the\ndetection rule and `relative_likelihood` is 1, then it is upgraded to\n`LIKELY`, while a value of -1 would downgrade it to `UNLIKELY`.\nLikelihood may never drop below `VERY_UNLIKELY` or exceed\n`VERY_LIKELY`, so applying an adjustment of 1 followed by an\nadjustment of -1 when base likelihood is `VERY_LIKELY` will result in\na final likelihood of `LIKELY`."]
        #[serde(
            rename = "relativeLikelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relative_likelihood: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2LikelihoodAdjustment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2LikelihoodAdjustment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        Likely,
        #[doc = "Some matching elements."]
        Possible,
        Unlikely,
        #[doc = "Many matching elements."]
        VeryLikely,
        #[doc = "Few matching elements."]
        VeryUnlikely,
    }
    impl GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::LikelihoodUnspecified => {
                    "LIKELIHOOD_UNSPECIFIED"
                }
                GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Likely => "LIKELY",
                GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Possible => "POSSIBLE",
                GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Unlikely => "UNLIKELY",
                GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::VeryLikely => "VERY_LIKELY",
                GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood, ()>
        {
            Ok(match s {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::VeryUnlikely
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELIHOOD_UNSPECIFIED" => {
                    GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::LikelihoodUnspecified
                }
                "LIKELY" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Likely,
                "POSSIBLE" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Possible,
                "UNLIKELY" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::Unlikely,
                "VERY_LIKELY" => GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood::VeryUnlikely
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
        for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihood
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
        #[doc = "List of deidentify templates, up to page_size in\nListDeidentifyTemplatesRequest."]
        #[serde(
            rename = "deidentifyTemplates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deidentify_templates:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate>>,
        #[doc = "If the next page is available then the next page token to be used\nin following ListDeidentifyTemplates request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2ListDlpJobsResponse {
        #[doc = "A list of DlpJobs that matches the specified filter in the request."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2DlpJob>>,
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListDlpJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListDlpJobsResponse {
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
    pub struct GooglePrivacyDlpV2ListInfoTypesRequest {
        #[doc = "Optional filter to only return infoTypes supported by certain parts of the\nAPI. Defaults to supported_by=INSPECT."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Optional BCP-47 language code for localized infoType friendly\nnames. If omitted, or if localized strings are not available,\nen-US strings will be returned."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListInfoTypesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListInfoTypesRequest {
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
    pub struct GooglePrivacyDlpV2ListInfoTypesResponse {
        #[doc = "Set of sensitive infoTypes."]
        #[serde(
            rename = "infoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_types:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoTypeDescription>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListInfoTypesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListInfoTypesResponse {
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
    pub struct GooglePrivacyDlpV2ListInspectTemplatesResponse {
        #[doc = "List of inspectTemplates, up to page_size in ListInspectTemplatesRequest."]
        #[serde(
            rename = "inspectTemplates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_templates:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InspectTemplate>>,
        #[doc = "If the next page is available then the next page token to be used\nin following ListInspectTemplates request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListInspectTemplatesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListInspectTemplatesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2ListJobTriggersResponse {
        #[doc = "List of triggeredJobs, up to page_size in ListJobTriggersRequest."]
        #[serde(
            rename = "jobTriggers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_triggers: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2JobTrigger>>,
        #[doc = "If the next page is available then the next page token to be used\nin following ListJobTriggers request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListJobTriggersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListJobTriggersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2ListStoredInfoTypesResponse {
        #[doc = "If the next page is available then the next page token to be used\nin following ListStoredInfoTypes request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of storedInfoTypes, up to page_size in ListStoredInfoTypesRequest."]
        #[serde(
            rename = "storedInfoTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stored_info_types:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2StoredInfoType>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ListStoredInfoTypesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ListStoredInfoTypesResponse {
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
    pub struct GooglePrivacyDlpV2Location {
        #[doc = "Zero-based byte offsets delimiting the finding.\nThese are relative to the finding's containing element.\nNote that when the content is not textual, this references\nthe UTF-8 encoded textual representation of the content.\nOmitted if content is an image."]
        #[serde(
            rename = "byteRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub byte_range: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Range>,
        #[doc = "Unicode character offsets delimiting the finding.\nThese are relative to the finding's containing element.\nProvided when the content is text."]
        #[serde(
            rename = "codepointRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub codepoint_range: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Range>,
        #[doc = "List of nested objects pointing to the precise location of the finding\nwithin the file or record."]
        #[serde(
            rename = "contentLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_locations:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2ContentLocation>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Location {
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
    pub struct GooglePrivacyDlpV2NumericalStatsConfig {
        #[doc = "Field to compute numerical stats on. Supported types are\ninteger, float, date, datetime, timestamp, time."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2NumericalStatsConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2NumericalStatsConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2NumericalStatsResult {
        #[doc = "Maximum value appearing in the column."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
        #[doc = "Minimum value appearing in the column."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
        #[doc = "List of 99 values that partition the set of field values into 100 equal\nsized buckets."]
        #[serde(
            rename = "quantileValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quantile_values: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Value>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2NumericalStatsResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2NumericalStatsResult {
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
    pub struct GooglePrivacyDlpV2OutputStorageConfig {
        #[doc = "Schema used for writing the findings for Inspect jobs. This field is only\nused for Inspect and must be unspecified for Risk jobs. Columns are derived\nfrom the `Finding` object. If appending to an existing table, any columns\nfrom the predefined schema that are missing will be added. No columns in\nthe existing table will be deleted.\n\nIf unspecified, then all available columns will be used for a new table or\nan (existing) table with no schema, and no changes will be made to an\nexisting table that has a schema."]
        #[serde(
            rename = "outputSchema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_schema: ::std::option::Option<
            crate::schemas::GooglePrivacyDlpV2OutputStorageConfigOutputSchema,
        >,
        #[doc = "Store findings in an existing table or a new table in an existing\ndataset. If table_id is not set a new one will be generated\nfor you with the following format:\ndlp_googleapis_yyyy_mm_dd_[dlp_job_id]. Pacific timezone will be used for\ngenerating the date details.\n\nFor Inspect, each column in an existing output table must have the same\nname, type, and mode of a field in the `Finding` object.\n\nFor Risk, an existing output table should be the output of a previous\nRisk analysis job run on the same source table, with the same privacy\nmetric and quasi-identifiers. Risk jobs that analyze the same table but\ncompute a different privacy metric, or use different sets of\nquasi-identifiers, cannot store their results in the same table."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2OutputStorageConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2OutputStorageConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        #[doc = "Schema containing all columns."]
        AllColumns,
        #[doc = "Basic schema including only `info_type`, `quote`, `certainty`, and\n`timestamp`."]
        BasicColumns,
        #[doc = "Schema tailored to findings from scanning Google BigQuery."]
        BigQueryColumns,
        #[doc = "Schema tailored to findings from scanning Google Datastore."]
        DatastoreColumns,
        #[doc = "Schema tailored to findings from scanning Google Cloud Storage."]
        GcsColumns,
        OutputSchemaUnspecified,
    }
    impl GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2OutputStorageConfigOutputSchema::AllColumns => "ALL_COLUMNS",
                GooglePrivacyDlpV2OutputStorageConfigOutputSchema::BasicColumns => "BASIC_COLUMNS",
                GooglePrivacyDlpV2OutputStorageConfigOutputSchema::BigQueryColumns => {
                    "BIG_QUERY_COLUMNS"
                }
                GooglePrivacyDlpV2OutputStorageConfigOutputSchema::DatastoreColumns => {
                    "DATASTORE_COLUMNS"
                }
                GooglePrivacyDlpV2OutputStorageConfigOutputSchema::GcsColumns => "GCS_COLUMNS",
                GooglePrivacyDlpV2OutputStorageConfigOutputSchema::OutputSchemaUnspecified => {
                    "OUTPUT_SCHEMA_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2OutputStorageConfigOutputSchema, ()> {
            Ok(match s {
                "ALL_COLUMNS" => GooglePrivacyDlpV2OutputStorageConfigOutputSchema::AllColumns,
                "BASIC_COLUMNS" => GooglePrivacyDlpV2OutputStorageConfigOutputSchema::BasicColumns,
                "BIG_QUERY_COLUMNS" => {
                    GooglePrivacyDlpV2OutputStorageConfigOutputSchema::BigQueryColumns
                }
                "DATASTORE_COLUMNS" => {
                    GooglePrivacyDlpV2OutputStorageConfigOutputSchema::DatastoreColumns
                }
                "GCS_COLUMNS" => GooglePrivacyDlpV2OutputStorageConfigOutputSchema::GcsColumns,
                "OUTPUT_SCHEMA_UNSPECIFIED" => {
                    GooglePrivacyDlpV2OutputStorageConfigOutputSchema::OutputSchemaUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_COLUMNS" => GooglePrivacyDlpV2OutputStorageConfigOutputSchema::AllColumns,
                "BASIC_COLUMNS" => GooglePrivacyDlpV2OutputStorageConfigOutputSchema::BasicColumns,
                "BIG_QUERY_COLUMNS" => {
                    GooglePrivacyDlpV2OutputStorageConfigOutputSchema::BigQueryColumns
                }
                "DATASTORE_COLUMNS" => {
                    GooglePrivacyDlpV2OutputStorageConfigOutputSchema::DatastoreColumns
                }
                "GCS_COLUMNS" => GooglePrivacyDlpV2OutputStorageConfigOutputSchema::GcsColumns,
                "OUTPUT_SCHEMA_UNSPECIFIED" => {
                    GooglePrivacyDlpV2OutputStorageConfigOutputSchema::OutputSchemaUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2OutputStorageConfigOutputSchema {
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
    pub struct GooglePrivacyDlpV2PartitionId {
        #[doc = "If not empty, the ID of the namespace to which the entities belong."]
        #[serde(
            rename = "namespaceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace_id: ::std::option::Option<String>,
        #[doc = "The ID of the project to which the entities belong."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2PartitionId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PartitionId {
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
    pub struct GooglePrivacyDlpV2PathElement {
        #[doc = "The auto-allocated ID of the entity.\nNever equal to zero. Values less than zero are discouraged and may not\nbe supported in the future."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "The kind of the entity.\nA kind matching regex `__.*__` is reserved/read-only.\nA kind must not contain more than 1500 bytes when UTF-8 encoded.\nCannot be `\"\"`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the entity.\nA name matching regex `__.*__` is reserved/read-only.\nA name must not be more than 1500 bytes when UTF-8 encoded.\nCannot be `\"\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2PathElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PathElement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2PrimitiveTransformation {
        #[serde(
            rename = "bucketingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucketing_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BucketingConfig>,
        #[serde(
            rename = "characterMaskConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub character_mask_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CharacterMaskConfig>,
        #[serde(
            rename = "cryptoDeterministicConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_deterministic_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoDeterministicConfig>,
        #[serde(
            rename = "cryptoHashConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_hash_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoHashConfig>,
        #[serde(
            rename = "cryptoReplaceFfxFpeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crypto_replace_ffx_fpe_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig>,
        #[serde(
            rename = "dateShiftConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_shift_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DateShiftConfig>,
        #[serde(
            rename = "fixedSizeBucketingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_size_bucketing_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FixedSizeBucketingConfig>,
        #[serde(
            rename = "redactConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redact_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RedactConfig>,
        #[serde(
            rename = "replaceConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ReplaceValueConfig>,
        #[serde(
            rename = "replaceWithInfoTypeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_with_info_type_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ReplaceWithInfoTypeConfig>,
        #[serde(
            rename = "timePartConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_part_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TimePartConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2PrimitiveTransformation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PrimitiveTransformation {
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
    pub struct GooglePrivacyDlpV2PrivacyMetric {
        #[serde(
            rename = "categoricalStatsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categorical_stats_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CategoricalStatsConfig>,
        #[serde(
            rename = "deltaPresenceEstimationConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delta_presence_estimation_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeltaPresenceEstimationConfig>,
        #[serde(
            rename = "kAnonymityConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub k_anonymity_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2KAnonymityConfig>,
        #[serde(
            rename = "kMapEstimationConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub k_map_estimation_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2KMapEstimationConfig>,
        #[serde(
            rename = "lDiversityConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub l_diversity_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2LDiversityConfig>,
        #[serde(
            rename = "numericalStatsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numerical_stats_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2NumericalStatsConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2PrivacyMetric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PrivacyMetric {
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
    pub struct GooglePrivacyDlpV2Proximity {
        #[doc = "Number of characters after the finding to consider."]
        #[serde(
            rename = "windowAfter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub window_after: ::std::option::Option<i32>,
        #[doc = "Number of characters before the finding to consider."]
        #[serde(
            rename = "windowBefore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub window_before: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Proximity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Proximity {
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
    pub struct GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog;
    impl ::google_field_selector::FieldSelector
        for GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog {
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
    pub struct GooglePrivacyDlpV2PublishSummaryToCscc;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2PublishSummaryToCscc {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PublishSummaryToCscc {
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
    pub struct GooglePrivacyDlpV2PublishToPubSub {
        #[doc = "Cloud Pub/Sub topic to send notifications to. The topic must have given\npublishing access rights to the DLP API service account executing\nthe long running DlpJob sending the notifications.\nFormat is projects/{project}/topics/{topic}."]
        #[serde(
            rename = "topic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topic: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2PublishToPubSub {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2PublishToPubSub {
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
    pub struct GooglePrivacyDlpV2QuasiId {
        #[doc = "A column can be tagged with a custom tag. In this case, the user must\nindicate an auxiliary table that contains statistical information on\nthe possible values of this column (below)."]
        #[serde(
            rename = "customTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_tag: ::std::option::Option<String>,
        #[doc = "Identifies the column. [required]"]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "If no semantic tag is indicated, we infer the statistical model from\nthe distribution of values in the input data"]
        #[serde(
            rename = "inferred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inferred: ::std::option::Option<crate::schemas::GoogleProtobufEmpty>,
        #[doc = "A column can be tagged with a InfoType to use the relevant public\ndataset as a statistical model of population, if available. We\ncurrently support US ZIP codes, region codes, ages and genders.\nTo programmatically obtain the list of supported InfoTypes, use\nListInfoTypes with the supported_by=RISK_ANALYSIS filter."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2QuasiId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2QuasiId {
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
    pub struct GooglePrivacyDlpV2QuasiIdField {
        #[serde(
            rename = "customTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_tag: ::std::option::Option<String>,
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2QuasiIdField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2QuasiIdField {
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
    pub struct GooglePrivacyDlpV2QuasiIdentifierField {
        #[serde(
            rename = "customTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_tag: ::std::option::Option<String>,
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2QuasiIdentifierField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2QuasiIdentifierField {
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
    pub struct GooglePrivacyDlpV2QuoteInfo {
        #[doc = "The date time indicated by the quote."]
        #[serde(
            rename = "dateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_time: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DateTime>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2QuoteInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2QuoteInfo {
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
    pub struct GooglePrivacyDlpV2Range {
        #[doc = "Index of the last character of the range (exclusive)."]
        #[serde(
            rename = "end",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end: ::std::option::Option<i64>,
        #[doc = "Index of the first character of the range (inclusive)."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Range {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Range {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2RecordCondition {
        #[doc = "An expression."]
        #[serde(
            rename = "expressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expressions: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Expressions>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RecordCondition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RecordCondition {
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
    pub struct GooglePrivacyDlpV2RecordKey {
        #[serde(
            rename = "bigQueryKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub big_query_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryKey>,
        #[serde(
            rename = "datastoreKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datastore_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DatastoreKey>,
        #[doc = "Values of identifying columns in the given row. Order of values matches\nthe order of field identifiers specified in the scanning request."]
        #[serde(
            rename = "idValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RecordKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RecordKey {
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
    pub struct GooglePrivacyDlpV2RecordLocation {
        #[doc = "Field id of the field containing the finding."]
        #[serde(
            rename = "fieldId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_id: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "Key of the finding."]
        #[serde(
            rename = "recordKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub record_key: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RecordKey>,
        #[doc = "Location within a `ContentItem.Table`."]
        #[serde(
            rename = "tableLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_location: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TableLocation>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RecordLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RecordLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2RecordSuppression {
        #[doc = "A condition that when it evaluates to true will result in the record being\nevaluated to be suppressed from the transformed content."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RecordCondition>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RecordSuppression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RecordSuppression {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2RecordTransformations {
        #[doc = "Transform the record by applying various field transformations."]
        #[serde(
            rename = "fieldTransformations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_transformations:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldTransformation>>,
        #[doc = "Configuration defining which records get suppressed entirely. Records that\nmatch any suppression rule are omitted from the output [optional]."]
        #[serde(
            rename = "recordSuppressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub record_suppressions:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2RecordSuppression>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RecordTransformations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RecordTransformations {
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
    pub struct GooglePrivacyDlpV2RedactConfig;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RedactConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RedactConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2RedactImageRequest {
        #[doc = "The content must be PNG, JPEG, SVG or BMP."]
        #[serde(
            rename = "byteItem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub byte_item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ByteContentItem>,
        #[doc = "The configuration for specifying what content to redact from images."]
        #[serde(
            rename = "imageRedactionConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_redaction_configs:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2ImageRedactionConfig>>,
        #[doc = "Whether the response should include findings along with the redacted\nimage."]
        #[serde(
            rename = "includeFindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_findings: ::std::option::Option<bool>,
        #[doc = "Configuration for the inspector."]
        #[serde(
            rename = "inspectConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RedactImageRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RedactImageRequest {
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
    pub struct GooglePrivacyDlpV2RedactImageResponse {
        #[doc = "If an image was being inspected and the InspectConfig's include_quote was\nset to true, then this field will include all text, if any, that was found\nin the image."]
        #[serde(
            rename = "extractedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extracted_text: ::std::option::Option<String>,
        #[doc = "The findings. Populated when include_findings in the request is true."]
        #[serde(
            rename = "inspectResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_result: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectResult>,
        #[doc = "The redacted image. The type will be the same as the original image."]
        #[serde(
            rename = "redactedImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redacted_image: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RedactImageResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RedactImageResponse {
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
    pub struct GooglePrivacyDlpV2Regex {
        #[doc = "The index of the submatch to extract as findings. When not\nspecified, the entire match is returned. No more than 3 may be included."]
        #[serde(
            rename = "groupIndexes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_indexes: ::std::option::Option<Vec<i32>>,
        #[doc = "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the\ngoogle/re2 repository on GitHub."]
        #[serde(
            rename = "pattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pattern: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Regex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Regex {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ReidentifyContentRequest {
        #[doc = "Configuration for the inspector."]
        #[serde(
            rename = "inspectConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectConfig>,
        #[doc = "Optional template to use. Any configuration directly specified in\n`inspect_config` will override those set in the template. Singular fields\nthat are set in this request will replace their corresponding fields in the\ntemplate. Repeated fields are appended. Singular sub-messages and groups\nare recursively merged."]
        #[serde(
            rename = "inspectTemplateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_template_name: ::std::option::Option<String>,
        #[doc = "The item to re-identify. Will be treated as text."]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ContentItem>,
        #[doc = "The geographic location to process content reidentification.  Reserved for\nfuture extensions."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Configuration for the re-identification of the content item.\nThis field shares the same proto message type that is used for\nde-identification, however its usage here is for the reversal of the\nprevious de-identification. Re-identification is performed by examining\nthe transformations used to de-identify the items and executing the\nreverse. This requires that only reversible transformations\nbe provided here. The reversible transformations are:\n\n* `CryptoReplaceFfxFpeConfig`"]
        #[serde(
            rename = "reidentifyConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reidentify_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeidentifyConfig>,
        #[doc = "Optional template to use. References an instance of `DeidentifyTemplate`.\nAny configuration directly specified in `reidentify_config` or\n`inspect_config` will override those set in the template. Singular fields\nthat are set in this request will replace their corresponding fields in the\ntemplate. Repeated fields are appended. Singular sub-messages and groups\nare recursively merged."]
        #[serde(
            rename = "reidentifyTemplateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reidentify_template_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ReidentifyContentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ReidentifyContentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ReidentifyContentResponse {
        #[doc = "The re-identified item."]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ContentItem>,
        #[doc = "An overview of the changes that were made to the `item`."]
        #[serde(
            rename = "overview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overview:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TransformationOverview>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ReidentifyContentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ReidentifyContentResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ReplaceValueConfig {
        #[doc = "Value to replace it with."]
        #[serde(
            rename = "newValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_value: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ReplaceValueConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ReplaceValueConfig {
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
    pub struct GooglePrivacyDlpV2ReplaceWithInfoTypeConfig;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ReplaceWithInfoTypeConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ReplaceWithInfoTypeConfig {
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
    pub struct GooglePrivacyDlpV2RequestedOptions {
        #[serde(
            rename = "jobConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectJobConfig>,
        #[doc = "If run with an InspectTemplate, a snapshot of its state at the time of\nthis run."]
        #[serde(
            rename = "snapshotInspectTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapshot_inspect_template:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectTemplate>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RequestedOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RequestedOptions {
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
    pub struct GooglePrivacyDlpV2Result {
        #[doc = "Statistics of how many instances of each info type were found during\ninspect job."]
        #[serde(
            rename = "infoTypeStats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type_stats:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2InfoTypeStats>>,
        #[doc = "Total size in bytes that were processed."]
        #[serde(
            rename = "processedBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub processed_bytes: ::std::option::Option<i64>,
        #[doc = "Estimate of the number of bytes to process."]
        #[serde(
            rename = "totalEstimatedBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_estimated_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Result {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Result {
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
    pub struct GooglePrivacyDlpV2RiskAnalysisJobConfig {
        #[doc = "Actions to execute at the completion of the job. Are executed in the order\nprovided."]
        #[serde(
            rename = "actions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actions: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Action>>,
        #[doc = "Privacy metric to compute."]
        #[serde(
            rename = "privacyMetric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privacy_metric: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PrivacyMetric>,
        #[doc = "Input dataset to compute metrics over."]
        #[serde(
            rename = "sourceTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_table: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2RiskAnalysisJobConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2RiskAnalysisJobConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Row {
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Value>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Row {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Row {
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
    pub struct GooglePrivacyDlpV2SaveFindings {
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2OutputStorageConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2SaveFindings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2SaveFindings {
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
    pub struct GooglePrivacyDlpV2Schedule {
        #[doc = "With this option a job is started a regular periodic basis. For\nexample: every day (86400 seconds).\n\nA scheduled start time will be skipped if the previous\nexecution has not ended when its scheduled time occurs.\n\nThis value must be set to a time duration greater than or equal\nto 1 day and can be no longer than 60 days."]
        #[serde(
            rename = "recurrencePeriodDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recurrence_period_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Schedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Schedule {
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
    pub struct GooglePrivacyDlpV2StatisticalTable {
        #[doc = "Quasi-identifier columns. [required]"]
        #[serde(
            rename = "quasiIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quasi_ids:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2QuasiIdentifierField>>,
        #[doc = "The relative frequency column must contain a floating-point number\nbetween 0 and 1 (inclusive). Null values are assumed to be zero.\n[required]"]
        #[serde(
            rename = "relativeFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relative_frequency: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "Auxiliary table location. [required]"]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryTable>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StatisticalTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StatisticalTable {
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
    pub struct GooglePrivacyDlpV2StorageConfig {
        #[doc = "BigQuery options specification."]
        #[serde(
            rename = "bigQueryOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub big_query_options:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2BigQueryOptions>,
        #[doc = "Google Cloud Storage options specification."]
        #[serde(
            rename = "cloudStorageOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_storage_options:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2CloudStorageOptions>,
        #[doc = "Google Cloud Datastore options specification."]
        #[serde(
            rename = "datastoreOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datastore_options:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DatastoreOptions>,
        #[serde(
            rename = "timespanConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timespan_config:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TimespanConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StorageConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StorageConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2StoredInfoType {
        #[doc = "Current version of the stored info type."]
        #[serde(
            rename = "currentVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_version:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeVersion>,
        #[doc = "Resource name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Pending versions of the stored info type. Empty if no versions are\npending."]
        #[serde(
            rename = "pendingVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_versions:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeVersion>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StoredInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StoredInfoType {
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
    pub struct GooglePrivacyDlpV2StoredInfoTypeConfig {
        #[doc = "Description of the StoredInfoType (max 256 characters)."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Display name of the StoredInfoType (max 256 characters)."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "StoredInfoType where findings are defined by a dictionary of phrases."]
        #[serde(
            rename = "largeCustomDictionary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub large_custom_dictionary:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2LargeCustomDictionaryConfig>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StoredInfoTypeConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StoredInfoTypeConfig {
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
    pub struct GooglePrivacyDlpV2StoredInfoTypeStats {
        #[doc = "StoredInfoType where findings are defined by a dictionary of phrases."]
        #[serde(
            rename = "largeCustomDictionary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub large_custom_dictionary:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2LargeCustomDictionaryStats>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StoredInfoTypeStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StoredInfoTypeStats {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2StoredInfoTypeVersion {
        #[doc = "StoredInfoType configuration."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeConfig>,
        #[doc = "Create timestamp of the version. Read-only, determined by the system\nwhen the version is created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Errors that occurred when creating this storedInfoType version, or\nanomalies detected in the storedInfoType data that render it unusable. Only\nthe five most recent errors will be displayed, with the most recent error\nappearing first.\n\n<p>For example, some of the data for stored custom dictionaries is put in\nthe user's Google Cloud Storage bucket, and if this data is modified or\ndeleted by the user or another system, the dictionary becomes invalid.\n<p>If any errors occur, fix the problem indicated by the error message and\nuse the UpdateStoredInfoType API method to create another version of the\nstoredInfoType to continue using it, reusing the same `config` if it was\nnot the source of the error."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Error>>,
        #[doc = "Stored info type version state. Read-only, updated by the system\nduring dictionary creation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeVersionState>,
        #[doc = "Statistics about this storedInfoType version."]
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeStats>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StoredInfoTypeVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StoredInfoTypeVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2StoredInfoTypeVersionState {
        #[doc = "StoredInfoType creation failed. All relevant error messages are returned in\nthe `StoredInfoTypeVersion` message."]
        Failed,
        #[doc = "StoredInfoType is no longer valid because artifacts stored in\nuser-controlled storage were modified. To fix an invalid StoredInfoType,\nuse the `UpdateStoredInfoType` method to create a new version."]
        Invalid,
        #[doc = "StoredInfoType version is being created."]
        Pending,
        #[doc = "StoredInfoType version is ready for use."]
        Ready,
        StoredInfoTypeStateUnspecified,
    }
    impl GooglePrivacyDlpV2StoredInfoTypeVersionState {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2StoredInfoTypeVersionState::Failed => "FAILED",
                GooglePrivacyDlpV2StoredInfoTypeVersionState::Invalid => "INVALID",
                GooglePrivacyDlpV2StoredInfoTypeVersionState::Pending => "PENDING",
                GooglePrivacyDlpV2StoredInfoTypeVersionState::Ready => "READY",
                GooglePrivacyDlpV2StoredInfoTypeVersionState::StoredInfoTypeStateUnspecified => {
                    "STORED_INFO_TYPE_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2StoredInfoTypeVersionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2StoredInfoTypeVersionState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2StoredInfoTypeVersionState, ()> {
            Ok(match s {
                "FAILED" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Failed,
                "INVALID" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Invalid,
                "PENDING" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Pending,
                "READY" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Ready,
                "STORED_INFO_TYPE_STATE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2StoredInfoTypeVersionState::StoredInfoTypeStateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2StoredInfoTypeVersionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2StoredInfoTypeVersionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2StoredInfoTypeVersionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FAILED" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Failed,
                "INVALID" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Invalid,
                "PENDING" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Pending,
                "READY" => GooglePrivacyDlpV2StoredInfoTypeVersionState::Ready,
                "STORED_INFO_TYPE_STATE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2StoredInfoTypeVersionState::StoredInfoTypeStateUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StoredInfoTypeVersionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StoredInfoTypeVersionState {
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
    pub struct GooglePrivacyDlpV2StoredType {
        #[doc = "Timestamp indicating when the version of the `StoredInfoType` used for\ninspection was created. Output-only field, populated by the system."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Resource name of the requested `StoredInfoType`, for example\n`organizations/433245324/storedInfoTypes/432452342` or\n`projects/project-id/storedInfoTypes/432452342`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2StoredType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2StoredType {
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
    pub struct GooglePrivacyDlpV2SummaryResult {
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2SummaryResultCode>,
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "A place for warnings or errors to show up if a transformation didn't\nwork as expected."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2SummaryResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2SummaryResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2SummaryResultCode {
        Error,
        Success,
        TransformationResultCodeUnspecified,
    }
    impl GooglePrivacyDlpV2SummaryResultCode {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2SummaryResultCode::Error => "ERROR",
                GooglePrivacyDlpV2SummaryResultCode::Success => "SUCCESS",
                GooglePrivacyDlpV2SummaryResultCode::TransformationResultCodeUnspecified => {
                    "TRANSFORMATION_RESULT_CODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2SummaryResultCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2SummaryResultCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2SummaryResultCode, ()> {
            Ok(match s {
                "ERROR" => GooglePrivacyDlpV2SummaryResultCode::Error,
                "SUCCESS" => GooglePrivacyDlpV2SummaryResultCode::Success,
                "TRANSFORMATION_RESULT_CODE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2SummaryResultCode::TransformationResultCodeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2SummaryResultCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2SummaryResultCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2SummaryResultCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => GooglePrivacyDlpV2SummaryResultCode::Error,
                "SUCCESS" => GooglePrivacyDlpV2SummaryResultCode::Success,
                "TRANSFORMATION_RESULT_CODE_UNSPECIFIED" => {
                    GooglePrivacyDlpV2SummaryResultCode::TransformationResultCodeUnspecified
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
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2SummaryResultCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2SummaryResultCode {
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
    pub struct GooglePrivacyDlpV2SurrogateType;
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2SurrogateType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2SurrogateType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Table {
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldId>>,
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2Row>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Table {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Table {
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
    pub struct GooglePrivacyDlpV2TableLocation {
        #[doc = "The zero-based index of the row where the finding is located."]
        #[serde(
            rename = "rowIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub row_index: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TableLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TableLocation {
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
    pub struct GooglePrivacyDlpV2TaggedField {
        #[doc = "A column can be tagged with a custom tag. In this case, the user must\nindicate an auxiliary table that contains statistical information on\nthe possible values of this column (below)."]
        #[serde(
            rename = "customTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_tag: ::std::option::Option<String>,
        #[doc = "Identifies the column. [required]"]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "If no semantic tag is indicated, we infer the statistical model from\nthe distribution of values in the input data"]
        #[serde(
            rename = "inferred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inferred: ::std::option::Option<crate::schemas::GoogleProtobufEmpty>,
        #[doc = "A column can be tagged with a InfoType to use the relevant public\ndataset as a statistical model of population, if available. We\ncurrently support US ZIP codes, region codes, ages and genders.\nTo programmatically obtain the list of supported InfoTypes, use\nListInfoTypes with the supported_by=RISK_ANALYSIS filter."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TaggedField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TaggedField {
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
    pub struct GooglePrivacyDlpV2TimePartConfig {
        #[serde(
            rename = "partToExtract",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub part_to_extract:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2TimePartConfigPartToExtract>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TimePartConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TimePartConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2TimePartConfigPartToExtract {
        #[doc = "[1-31]"]
        DayOfMonth,
        #[doc = "[1-7]"]
        DayOfWeek,
        #[doc = "[0-23]"]
        HourOfDay,
        #[doc = "[1-12]"]
        Month,
        TimePartUnspecified,
        #[doc = "[1-52]"]
        WeekOfYear,
        #[doc = "[0-9999]"]
        Year,
    }
    impl GooglePrivacyDlpV2TimePartConfigPartToExtract {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2TimePartConfigPartToExtract::DayOfMonth => "DAY_OF_MONTH",
                GooglePrivacyDlpV2TimePartConfigPartToExtract::DayOfWeek => "DAY_OF_WEEK",
                GooglePrivacyDlpV2TimePartConfigPartToExtract::HourOfDay => "HOUR_OF_DAY",
                GooglePrivacyDlpV2TimePartConfigPartToExtract::Month => "MONTH",
                GooglePrivacyDlpV2TimePartConfigPartToExtract::TimePartUnspecified => {
                    "TIME_PART_UNSPECIFIED"
                }
                GooglePrivacyDlpV2TimePartConfigPartToExtract::WeekOfYear => "WEEK_OF_YEAR",
                GooglePrivacyDlpV2TimePartConfigPartToExtract::Year => "YEAR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2TimePartConfigPartToExtract {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2TimePartConfigPartToExtract {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GooglePrivacyDlpV2TimePartConfigPartToExtract, ()> {
            Ok(match s {
                "DAY_OF_MONTH" => GooglePrivacyDlpV2TimePartConfigPartToExtract::DayOfMonth,
                "DAY_OF_WEEK" => GooglePrivacyDlpV2TimePartConfigPartToExtract::DayOfWeek,
                "HOUR_OF_DAY" => GooglePrivacyDlpV2TimePartConfigPartToExtract::HourOfDay,
                "MONTH" => GooglePrivacyDlpV2TimePartConfigPartToExtract::Month,
                "TIME_PART_UNSPECIFIED" => {
                    GooglePrivacyDlpV2TimePartConfigPartToExtract::TimePartUnspecified
                }
                "WEEK_OF_YEAR" => GooglePrivacyDlpV2TimePartConfigPartToExtract::WeekOfYear,
                "YEAR" => GooglePrivacyDlpV2TimePartConfigPartToExtract::Year,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2TimePartConfigPartToExtract {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2TimePartConfigPartToExtract {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2TimePartConfigPartToExtract {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY_OF_MONTH" => GooglePrivacyDlpV2TimePartConfigPartToExtract::DayOfMonth,
                "DAY_OF_WEEK" => GooglePrivacyDlpV2TimePartConfigPartToExtract::DayOfWeek,
                "HOUR_OF_DAY" => GooglePrivacyDlpV2TimePartConfigPartToExtract::HourOfDay,
                "MONTH" => GooglePrivacyDlpV2TimePartConfigPartToExtract::Month,
                "TIME_PART_UNSPECIFIED" => {
                    GooglePrivacyDlpV2TimePartConfigPartToExtract::TimePartUnspecified
                }
                "WEEK_OF_YEAR" => GooglePrivacyDlpV2TimePartConfigPartToExtract::WeekOfYear,
                "YEAR" => GooglePrivacyDlpV2TimePartConfigPartToExtract::Year,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TimePartConfigPartToExtract {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TimePartConfigPartToExtract {
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
    pub struct GooglePrivacyDlpV2TimeZone {
        #[doc = "Set only if the offset can be determined. Positive for time ahead of UTC.\nE.g. For \"UTC-9\", this value is -540."]
        #[serde(
            rename = "offsetMinutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_minutes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TimeZone {
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
    pub struct GooglePrivacyDlpV2TimespanConfig {
        #[doc = "When the job is started by a JobTrigger we will automatically figure out\na valid start_time to avoid scanning files that have not been modified\nsince the last time the JobTrigger executed. This will be based on the\ntime of the execution of the last run of the JobTrigger."]
        #[serde(
            rename = "enableAutoPopulationOfTimespanConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_auto_population_of_timespan_config: ::std::option::Option<bool>,
        #[doc = "Exclude files or rows newer than this value.\nIf set to zero, no upper time limit is applied."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Exclude files or rows older than this value."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Specification of the field containing the timestamp of scanned items.\nUsed for data sources like Datastore or BigQuery.\nIf not specified for BigQuery, table last modification timestamp\nis checked against given time span.\nThe valid data types of the timestamp field are:\nfor BigQuery - timestamp, date, datetime;\nfor Datastore - timestamp.\nDatastore entity will be scanned if the timestamp property does not exist\nor its value is empty or invalid."]
        #[serde(
            rename = "timestampField",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp_field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TimespanConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TimespanConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2TransformationOverview {
        #[doc = "Transformations applied to the dataset."]
        #[serde(
            rename = "transformationSummaries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transformation_summaries:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2TransformationSummary>>,
        #[doc = "Total size in bytes that were transformed in some way."]
        #[serde(
            rename = "transformedBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub transformed_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TransformationOverview {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TransformationOverview {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2TransformationSummary {
        #[doc = "Set if the transformation was limited to a specific FieldId."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2FieldId>,
        #[doc = "The field transformation that was applied.\nIf multiple field transformations are requested for a single field,\nthis list will contain all of them; otherwise, only one is supplied."]
        #[serde(
            rename = "fieldTransformations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_transformations:
            ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2FieldTransformation>>,
        #[doc = "Set if the transformation was limited to a specific InfoType."]
        #[serde(
            rename = "infoType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info_type: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InfoType>,
        #[doc = "The specific suppression option these stats apply to."]
        #[serde(
            rename = "recordSuppress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub record_suppress:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2RecordSuppression>,
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::GooglePrivacyDlpV2SummaryResult>>,
        #[doc = "The specific transformation these stats apply to."]
        #[serde(
            rename = "transformation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transformation:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2PrimitiveTransformation>,
        #[doc = "Total size in bytes that were transformed in some way."]
        #[serde(
            rename = "transformedBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub transformed_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TransformationSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TransformationSummary {
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
    pub struct GooglePrivacyDlpV2TransientCryptoKey {
        #[doc = "Name of the key. [required]\nThis is an arbitrary string used to differentiate different keys.\nA unique key is generated per name: two separate `TransientCryptoKey`\nprotos share the same generated key if their names are the same.\nWhen the data crypto key is generated, this name is not used in any way\n(repeating the api call will result in a different key being generated)."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2TransientCryptoKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2TransientCryptoKey {
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
    pub struct GooglePrivacyDlpV2Trigger {
        #[doc = "Create a job on a repeating basis based on the elapse of time."]
        #[serde(
            rename = "schedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schedule: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Schedule>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Trigger {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Trigger {
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
    pub struct GooglePrivacyDlpV2UnwrappedCryptoKey {
        #[doc = "A 128/192/256 bit key. [required]"]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2UnwrappedCryptoKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2UnwrappedCryptoKey {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
        #[doc = "New DeidentifyTemplate value."]
        #[serde(
            rename = "deidentifyTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deidentify_template:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate>,
        #[doc = "Mask to control which fields get updated."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
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
    pub struct GooglePrivacyDlpV2UpdateInspectTemplateRequest {
        #[doc = "New InspectTemplate value."]
        #[serde(
            rename = "inspectTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inspect_template:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2InspectTemplate>,
        #[doc = "Mask to control which fields get updated."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2UpdateInspectTemplateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2UpdateInspectTemplateRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GooglePrivacyDlpV2UpdateJobTriggerRequest {
        #[doc = "New JobTrigger value."]
        #[serde(
            rename = "jobTrigger",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_trigger: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2JobTrigger>,
        #[doc = "Mask to control which fields get updated."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2UpdateJobTriggerRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2UpdateJobTriggerRequest {
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
    pub struct GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
        #[doc = "Updated configuration for the storedInfoType. If not provided, a new\nversion of the storedInfoType will be created with the existing\nconfiguration."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2StoredInfoTypeConfig>,
        #[doc = "Mask to control which fields get updated."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2Value {
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[serde(
            rename = "dateValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_value: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[serde(
            rename = "dayOfWeekValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_of_week_value:
            ::std::option::Option<crate::schemas::GooglePrivacyDlpV2ValueDayOfWeekValue>,
        #[serde(
            rename = "floatValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub float_value: ::std::option::Option<f64>,
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub integer_value: ::std::option::Option<i64>,
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
        #[serde(
            rename = "timeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_value: ::std::option::Option<crate::schemas::GoogleTypeTimeOfDay>,
        #[serde(
            rename = "timestampValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2Value {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2Value {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GooglePrivacyDlpV2ValueDayOfWeekValue {
        #[doc = "The unspecified day-of-week."]
        DayOfWeekUnspecified,
        #[doc = "The day-of-week of Friday."]
        Friday,
        #[doc = "The day-of-week of Monday."]
        Monday,
        #[doc = "The day-of-week of Saturday."]
        Saturday,
        #[doc = "The day-of-week of Sunday."]
        Sunday,
        #[doc = "The day-of-week of Thursday."]
        Thursday,
        #[doc = "The day-of-week of Tuesday."]
        Tuesday,
        #[doc = "The day-of-week of Wednesday."]
        Wednesday,
    }
    impl GooglePrivacyDlpV2ValueDayOfWeekValue {
        pub fn as_str(self) -> &'static str {
            match self {
                GooglePrivacyDlpV2ValueDayOfWeekValue::DayOfWeekUnspecified => {
                    "DAY_OF_WEEK_UNSPECIFIED"
                }
                GooglePrivacyDlpV2ValueDayOfWeekValue::Friday => "FRIDAY",
                GooglePrivacyDlpV2ValueDayOfWeekValue::Monday => "MONDAY",
                GooglePrivacyDlpV2ValueDayOfWeekValue::Saturday => "SATURDAY",
                GooglePrivacyDlpV2ValueDayOfWeekValue::Sunday => "SUNDAY",
                GooglePrivacyDlpV2ValueDayOfWeekValue::Thursday => "THURSDAY",
                GooglePrivacyDlpV2ValueDayOfWeekValue::Tuesday => "TUESDAY",
                GooglePrivacyDlpV2ValueDayOfWeekValue::Wednesday => "WEDNESDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GooglePrivacyDlpV2ValueDayOfWeekValue {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GooglePrivacyDlpV2ValueDayOfWeekValue {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GooglePrivacyDlpV2ValueDayOfWeekValue, ()> {
            Ok(match s {
                "DAY_OF_WEEK_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ValueDayOfWeekValue::DayOfWeekUnspecified
                }
                "FRIDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Friday,
                "MONDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Monday,
                "SATURDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Saturday,
                "SUNDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Sunday,
                "THURSDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Thursday,
                "TUESDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Tuesday,
                "WEDNESDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Wednesday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GooglePrivacyDlpV2ValueDayOfWeekValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GooglePrivacyDlpV2ValueDayOfWeekValue {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GooglePrivacyDlpV2ValueDayOfWeekValue {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY_OF_WEEK_UNSPECIFIED" => {
                    GooglePrivacyDlpV2ValueDayOfWeekValue::DayOfWeekUnspecified
                }
                "FRIDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Friday,
                "MONDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Monday,
                "SATURDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Saturday,
                "SUNDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Sunday,
                "THURSDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Thursday,
                "TUESDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Tuesday,
                "WEDNESDAY" => GooglePrivacyDlpV2ValueDayOfWeekValue::Wednesday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ValueDayOfWeekValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ValueDayOfWeekValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GooglePrivacyDlpV2ValueFrequency {
        #[doc = "How many times the value is contained in the field."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "A value contained in the field in question."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::GooglePrivacyDlpV2Value>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2ValueFrequency {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2ValueFrequency {
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
    pub struct GooglePrivacyDlpV2WordList {
        #[doc = "Words or phrases defining the dictionary. The dictionary must contain\nat least one phrase and every phrase must contain at least 2 characters\nthat are letters or digits. [required]"]
        #[serde(
            rename = "words",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub words: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GooglePrivacyDlpV2WordList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePrivacyDlpV2WordList {
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
    pub struct GoogleProtobufEmpty;
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
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
    pub struct GoogleTypeDate {
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
    impl ::google_field_selector::FieldSelector for GoogleTypeDate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeDate {
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
    pub struct GoogleTypeTimeOfDay {
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose\nto allow the value \"24:00:00\" for scenarios like business closing time."]
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
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may\nallow the value 60 if it allows leap-seconds."]
        #[serde(
            rename = "seconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seconds: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeTimeOfDay {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeTimeOfDay {
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
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the info_types resource"]
    pub fn info_types(&self) -> crate::resources::info_types::InfoTypesActions {
        crate::resources::info_types::InfoTypesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the locations resource"]
    pub fn locations(&self) -> crate::resources::locations::LocationsActions {
        crate::resources::locations::LocationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the organizations resource"]
    pub fn organizations(&self) -> crate::resources::organizations::OrganizationsActions {
        crate::resources::organizations::OrganizationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
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
    pub mod info_types {
        pub mod params {}
        pub struct InfoTypesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> InfoTypesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a list of the sensitive information types that the DLP API\nsupports. See https://cloud.google.com/dlp/docs/infotypes-reference to\nlearn more."]
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
                    language_code: None,
                    location: None,
                }
            }
        }
        #[doc = "Created via [InfoTypesActions::list()](struct.InfoTypesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            filter: Option<String>,
            language_code: Option<String>,
            location: Option<String>,
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
            #[doc = "Optional filter to only return infoTypes supported by certain parts of the\nAPI. Defaults to supported_by=INSPECT."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Optional BCP-47 language code for localized infoType friendly\nnames. If omitted, or if localized strings are not available,\nen-US strings will be returned."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "The geographic location to list info types. Reserved for future\nextensions."]
            pub fn location(mut self, value: impl Into<String>) -> Self {
                self.location = Some(value.into());
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
            ) -> Result<crate::schemas::GooglePrivacyDlpV2ListInfoTypesResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GooglePrivacyDlpV2ListInfoTypesResponse, crate::Error>
            {
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
                let mut output = "https://dlp.googleapis.com/".to_owned();
                output.push_str("v2/infoTypes");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("languageCode", &self.language_code)]);
                let req = req.query(&[("location", &self.location)]);
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
    pub mod locations {
        pub mod params {}
        pub struct LocationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> LocationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a list of the sensitive information types that the DLP API\nsupports. See https://cloud.google.com/dlp/docs/infotypes-reference to\nlearn more."]
            pub fn info_types(
                &self,
                request: crate::schemas::GooglePrivacyDlpV2ListInfoTypesRequest,
                location: impl Into<String>,
            ) -> InfoTypesRequestBuilder {
                InfoTypesRequestBuilder {
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
                    location: location.into(),
                }
            }
        }
        #[doc = "Created via [LocationsActions::info_types()](struct.LocationsActions.html#method.info_types)"]
        #[derive(Debug, Clone)]
        pub struct InfoTypesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GooglePrivacyDlpV2ListInfoTypesRequest,
            location: String,
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
        impl<'a> InfoTypesRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GooglePrivacyDlpV2ListInfoTypesResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GooglePrivacyDlpV2ListInfoTypesResponse, crate::Error>
            {
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
                let mut output = "https://dlp.googleapis.com/".to_owned();
                output.push_str("v2/locations/");
                {
                    let var_as_str = &self.location;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/infoTypes");
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
    pub mod organizations {
        pub mod params {}
        pub struct OrganizationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OrganizationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the deidentify_templates resource"]
            pub fn deidentify_templates(
                &self,
            ) -> crate::resources::organizations::deidentify_templates::DeidentifyTemplatesActions
            {
                crate::resources::organizations::deidentify_templates::DeidentifyTemplatesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the inspect_templates resource"]
            pub fn inspect_templates(
                &self,
            ) -> crate::resources::organizations::inspect_templates::InspectTemplatesActions
            {
                crate::resources::organizations::inspect_templates::InspectTemplatesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the stored_info_types resource"]
            pub fn stored_info_types(
                &self,
            ) -> crate::resources::organizations::stored_info_types::StoredInfoTypesActions
            {
                crate::resources::organizations::stored_info_types::StoredInfoTypesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod deidentify_templates {
            pub mod params {}
            pub struct DeidentifyTemplatesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DeidentifyTemplatesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a DeidentifyTemplate for re-using frequently used configuration\nfor de-identifying content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest,
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
                #[doc = "Deletes a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
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
                #[doc = "Gets a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
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
                #[doc = "Lists DeidentifyTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest,
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
            #[doc = "Created via [DeidentifyTemplatesActions::create()](struct.DeidentifyTemplatesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/deidentifyTemplates");
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
            #[doc = "Created via [DeidentifyTemplatesActions::delete()](struct.DeidentifyTemplatesActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [DeidentifyTemplatesActions::get()](struct.DeidentifyTemplatesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [DeidentifyTemplatesActions::list()](struct.DeidentifyTemplatesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                order_by: Option<String>,
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
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc,update_time, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the template was created.\n* `update_time`: corresponds to time the template was last updated.\n* `name`: corresponds to template's name.\n* `display_name`: corresponds to template's display name."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by server. If zero server returns\na page of max size 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto `ListDeidentifyTemplates`."]
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
                pub fn iter_deidentify_templates<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_deidentify_templates_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_deidentify_templates_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate,
                > {
                    self.iter_deidentify_templates_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_deidentify_templates_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate,
                > {
                    self.iter_deidentify_templates_with_fields(Some("*"))
                }
                pub fn iter_deidentify_templates_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector =
                            concat!("nextPageToken,", "deidentifyTemplates").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "deidentifyTemplates")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
                > {
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
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
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
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/deidentifyTemplates");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [DeidentifyTemplatesActions::patch()](struct.DeidentifyTemplatesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
        pub mod inspect_templates {
            pub mod params {}
            pub struct InspectTemplatesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> InspectTemplatesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates an InspectTemplate for re-using frequently used configuration\nfor inspecting content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateInspectTemplateRequest,
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
                #[doc = "Deletes an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
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
                #[doc = "Gets an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
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
                #[doc = "Lists InspectTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateInspectTemplateRequest,
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
            #[doc = "Created via [InspectTemplatesActions::create()](struct.InspectTemplatesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateInspectTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/inspectTemplates");
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
            #[doc = "Created via [InspectTemplatesActions::delete()](struct.InspectTemplatesActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [InspectTemplatesActions::get()](struct.InspectTemplatesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [InspectTemplatesActions::list()](struct.InspectTemplatesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                order_by: Option<String>,
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
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc,update_time, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the template was created.\n* `update_time`: corresponds to time the template was last updated.\n* `name`: corresponds to template's name.\n* `display_name`: corresponds to template's display name."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by server. If zero server returns\na page of max size 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto `ListInspectTemplates`."]
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
                pub fn iter_inspect_templates<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_inspect_templates_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_inspect_templates_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2InspectTemplate,
                > {
                    self.iter_inspect_templates_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_inspect_templates_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2InspectTemplate,
                > {
                    self.iter_inspect_templates_with_fields(Some("*"))
                }
                pub fn iter_inspect_templates_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "inspectTemplates").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "inspectTemplates")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
                > {
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
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
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
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/inspectTemplates");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [InspectTemplatesActions::patch()](struct.InspectTemplatesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateInspectTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
        pub mod stored_info_types {
            pub mod params {}
            pub struct StoredInfoTypesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> StoredInfoTypesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a pre-built stored infoType to be used for inspection.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateStoredInfoTypeRequest,
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
                #[doc = "Deletes a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
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
                #[doc = "Gets a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
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
                #[doc = "Lists stored infoTypes.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the stored infoType by creating a new version. The existing version\nwill continue to be used until the new version is ready.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest,
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
            #[doc = "Created via [StoredInfoTypesActions::create()](struct.StoredInfoTypesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateStoredInfoTypeRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/storedInfoTypes");
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
            #[doc = "Created via [StoredInfoTypesActions::delete()](struct.StoredInfoTypesActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [StoredInfoTypesActions::get()](struct.StoredInfoTypesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [StoredInfoTypesActions::list()](struct.StoredInfoTypesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                order_by: Option<String>,
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
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc, display_name, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the most recent version of the\n  resource was created.\n* `state`: corresponds to the state of the resource.\n* `name`: corresponds to resource name.\n* `display_name`: corresponds to info type's display name."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by server. If zero server returns\na page of max size 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto `ListStoredInfoTypes`."]
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
                pub fn iter_stored_info_types<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_stored_info_types_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_stored_info_types_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2StoredInfoType>
                {
                    self.iter_stored_info_types_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_stored_info_types_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2StoredInfoType>
                {
                    self.iter_stored_info_types_with_fields(Some("*"))
                }
                pub fn iter_stored_info_types_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "storedInfoTypes").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "storedInfoTypes")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
                > {
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
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
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
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/storedInfoTypes");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [StoredInfoTypesActions::patch()](struct.StoredInfoTypesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
    }
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
            #[doc = "Actions that can be performed on the content resource"]
            pub fn content(&self) -> crate::resources::projects::content::ContentActions {
                crate::resources::projects::content::ContentActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the deidentify_templates resource"]
            pub fn deidentify_templates(
                &self,
            ) -> crate::resources::projects::deidentify_templates::DeidentifyTemplatesActions
            {
                crate::resources::projects::deidentify_templates::DeidentifyTemplatesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the dlp_jobs resource"]
            pub fn dlp_jobs(&self) -> crate::resources::projects::dlp_jobs::DlpJobsActions {
                crate::resources::projects::dlp_jobs::DlpJobsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the image resource"]
            pub fn image(&self) -> crate::resources::projects::image::ImageActions {
                crate::resources::projects::image::ImageActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the inspect_templates resource"]
            pub fn inspect_templates(
                &self,
            ) -> crate::resources::projects::inspect_templates::InspectTemplatesActions
            {
                crate::resources::projects::inspect_templates::InspectTemplatesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the job_triggers resource"]
            pub fn job_triggers(
                &self,
            ) -> crate::resources::projects::job_triggers::JobTriggersActions {
                crate::resources::projects::job_triggers::JobTriggersActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the stored_info_types resource"]
            pub fn stored_info_types(
                &self,
            ) -> crate::resources::projects::stored_info_types::StoredInfoTypesActions {
                crate::resources::projects::stored_info_types::StoredInfoTypesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod content {
            pub mod params {}
            pub struct ContentActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ContentActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "De-identifies potentially sensitive info from a ContentItem.\nThis method has limits on input size and output size.\nSee https://cloud.google.com/dlp/docs/deidentify-sensitive-data to\nlearn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated."]
                pub fn deidentify(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2DeidentifyContentRequest,
                    parent: impl Into<String>,
                ) -> DeidentifyRequestBuilder {
                    DeidentifyRequestBuilder {
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
                #[doc = "Finds potentially sensitive info in content.\nThis method has limits on input size, processing time, and output size.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.\n\nFor how to guides, see https://cloud.google.com/dlp/docs/inspecting-images\nand https://cloud.google.com/dlp/docs/inspecting-text,"]
                pub fn inspect(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2InspectContentRequest,
                    parent: impl Into<String>,
                ) -> InspectRequestBuilder {
                    InspectRequestBuilder {
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
                #[doc = "Re-identifies content that has been de-identified.\nSee\nhttps://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example\nto learn more."]
                pub fn reidentify(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2ReidentifyContentRequest,
                    parent: impl Into<String>,
                ) -> ReidentifyRequestBuilder {
                    ReidentifyRequestBuilder {
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
            #[doc = "Created via [ContentActions::deidentify()](struct.ContentActions.html#method.deidentify)"]
            #[derive(Debug, Clone)]
            pub struct DeidentifyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2DeidentifyContentRequest,
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
            impl<'a> DeidentifyRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyContentResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyContentResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/content:deidentify");
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
            #[doc = "Created via [ContentActions::inspect()](struct.ContentActions.html#method.inspect)"]
            #[derive(Debug, Clone)]
            pub struct InspectRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2InspectContentRequest,
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
            impl<'a> InspectRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectContentResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectContentResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/content:inspect");
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
            #[doc = "Created via [ContentActions::reidentify()](struct.ContentActions.html#method.reidentify)"]
            #[derive(Debug, Clone)]
            pub struct ReidentifyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2ReidentifyContentRequest,
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
            impl<'a> ReidentifyRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2ReidentifyContentResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2ReidentifyContentResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/content:reidentify");
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
        pub mod deidentify_templates {
            pub mod params {}
            pub struct DeidentifyTemplatesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DeidentifyTemplatesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a DeidentifyTemplate for re-using frequently used configuration\nfor de-identifying content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest,
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
                #[doc = "Deletes a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
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
                #[doc = "Gets a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
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
                #[doc = "Lists DeidentifyTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest,
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
            #[doc = "Created via [DeidentifyTemplatesActions::create()](struct.DeidentifyTemplatesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/deidentifyTemplates");
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
            #[doc = "Created via [DeidentifyTemplatesActions::delete()](struct.DeidentifyTemplatesActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [DeidentifyTemplatesActions::get()](struct.DeidentifyTemplatesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [DeidentifyTemplatesActions::list()](struct.DeidentifyTemplatesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                order_by: Option<String>,
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
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc,update_time, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the template was created.\n* `update_time`: corresponds to time the template was last updated.\n* `name`: corresponds to template's name.\n* `display_name`: corresponds to template's display name."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by server. If zero server returns\na page of max size 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto `ListDeidentifyTemplates`."]
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
                pub fn iter_deidentify_templates<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_deidentify_templates_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_deidentify_templates_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate,
                > {
                    self.iter_deidentify_templates_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_deidentify_templates_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate,
                > {
                    self.iter_deidentify_templates_with_fields(Some("*"))
                }
                pub fn iter_deidentify_templates_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector =
                            concat!("nextPageToken,", "deidentifyTemplates").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "deidentifyTemplates")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
                > {
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
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
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
                    crate::schemas::GooglePrivacyDlpV2ListDeidentifyTemplatesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/deidentifyTemplates");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [DeidentifyTemplatesActions::patch()](struct.DeidentifyTemplatesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DeidentifyTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
        pub mod dlp_jobs {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListType {
                    DlpJobTypeUnspecified,
                    InspectJob,
                    RiskAnalysisJob,
                }
                impl ListType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListType::DlpJobTypeUnspecified => "DLP_JOB_TYPE_UNSPECIFIED",
                            ListType::InspectJob => "INSPECT_JOB",
                            ListType::RiskAnalysisJob => "RISK_ANALYSIS_JOB",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListType, ()> {
                        Ok(match s {
                            "DLP_JOB_TYPE_UNSPECIFIED" => ListType::DlpJobTypeUnspecified,
                            "INSPECT_JOB" => ListType::InspectJob,
                            "RISK_ANALYSIS_JOB" => ListType::RiskAnalysisJob,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "DLP_JOB_TYPE_UNSPECIFIED" => ListType::DlpJobTypeUnspecified,
                            "INSPECT_JOB" => ListType::InspectJob,
                            "RISK_ANALYSIS_JOB" => ListType::RiskAnalysisJob,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct DlpJobsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DlpJobsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Starts asynchronous cancellation on a long-running DlpJob. The server\nmakes a best effort to cancel the DlpJob, but success is not\nguaranteed.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."]
                pub fn cancel(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CancelDlpJobRequest,
                    name: impl Into<String>,
                ) -> CancelRequestBuilder {
                    CancelRequestBuilder {
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
                #[doc = "Creates a new job to inspect storage or calculate risk metrics.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in inspect jobs, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateDlpJobRequest,
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
                #[doc = "Deletes a long-running DlpJob. This method indicates that the client is\nno longer interested in the DlpJob result. The job will be cancelled if\npossible.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."]
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
                #[doc = "Gets the latest state of a long-running DlpJob.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."]
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
                #[doc = "Lists DlpJobs that match the specified filter in the request.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                        r#type: None,
                    }
                }
            }
            #[doc = "Created via [DlpJobsActions::cancel()](struct.DlpJobsActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CancelDlpJobRequest,
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
            impl<'a> CancelRequestBuilder<'a> {
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":cancel");
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
            #[doc = "Created via [DlpJobsActions::create()](struct.DlpJobsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateDlpJobRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DlpJob, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DlpJob, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/dlpJobs");
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
            #[doc = "Created via [DlpJobsActions::delete()](struct.DlpJobsActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [DlpJobsActions::get()](struct.DlpJobsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DlpJob, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DlpJob, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [DlpJobsActions::list()](struct.DlpJobsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                order_by: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                r#type: Option<crate::resources::projects::dlp_jobs::params::ListType>,
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
                #[doc = "Optional. Allows filtering.\n\nSupported syntax:\n\n* Filter expressions are made up of one or more restrictions.\n* Restrictions can be combined by `AND` or `OR` logical operators. A\n  sequence of restrictions implicitly uses `AND`.\n* A restriction has the form of `<field> <operator> <value>`.\n* Supported fields/values for inspect jobs:\n  * `state` - PENDING|RUNNING|CANCELED|FINISHED|FAILED\n  * `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY\n  * `trigger_name` - The resource name of the trigger that created job.\n  * 'end_time` - Corresponds to time the job finished.\n  * 'start_time` - Corresponds to time the job finished.\n* Supported fields for risk analysis jobs:\n  * `state` - RUNNING|CANCELED|FINISHED|FAILED\n  * 'end_time` - Corresponds to time the job finished.\n  * 'start_time` - Corresponds to time the job finished.\n* The operator must be `=` or `!=`.\n\nExamples:\n\n* inspected_storage = cloud_storage AND state = done\n* inspected_storage = cloud_storage OR inspected_storage = bigquery\n* inspected_storage = cloud_storage AND (state = done OR state = canceled)\n* end_time > \"2017-12-12T00:00:00+00:00\"\n\nThe length of this field should be no more than 500 characters."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc, end_time asc, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the job was created.\n* `end_time`: corresponds to time the job ended.\n* `name`: corresponds to job's name.\n* `state`: corresponds to `state`"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The standard list page size."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The standard list page token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
                pub fn r#type(
                    mut self,
                    value: crate::resources::projects::dlp_jobs::params::ListType,
                ) -> Self {
                    self.r#type = Some(value);
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2DlpJob>
                {
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2DlpJob>
                {
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListDlpJobsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListDlpJobsResponse,
                > {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2ListDlpJobsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2ListDlpJobsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/dlpJobs");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("orderBy", &self.order_by)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("type", &self.r#type)]);
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
                    self._execute()
                }
            }
        }
        pub mod image {
            pub mod params {}
            pub struct ImageActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ImageActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Redacts potentially sensitive info from an image.\nThis method has limits on input size, processing time, and output size.\nSee https://cloud.google.com/dlp/docs/redacting-sensitive-data-images to\nlearn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated."]
                pub fn redact(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2RedactImageRequest,
                    parent: impl Into<String>,
                ) -> RedactRequestBuilder {
                    RedactRequestBuilder {
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
            #[doc = "Created via [ImageActions::redact()](struct.ImageActions.html#method.redact)"]
            #[derive(Debug, Clone)]
            pub struct RedactRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2RedactImageRequest,
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
            impl<'a> RedactRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2RedactImageResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2RedactImageResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/image:redact");
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
        pub mod inspect_templates {
            pub mod params {}
            pub struct InspectTemplatesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> InspectTemplatesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates an InspectTemplate for re-using frequently used configuration\nfor inspecting content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateInspectTemplateRequest,
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
                #[doc = "Deletes an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
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
                #[doc = "Gets an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
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
                #[doc = "Lists InspectTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateInspectTemplateRequest,
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
            #[doc = "Created via [InspectTemplatesActions::create()](struct.InspectTemplatesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateInspectTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/inspectTemplates");
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
            #[doc = "Created via [InspectTemplatesActions::delete()](struct.InspectTemplatesActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [InspectTemplatesActions::get()](struct.InspectTemplatesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [InspectTemplatesActions::list()](struct.InspectTemplatesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                order_by: Option<String>,
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
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc,update_time, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the template was created.\n* `update_time`: corresponds to time the template was last updated.\n* `name`: corresponds to template's name.\n* `display_name`: corresponds to template's display name."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by server. If zero server returns\na page of max size 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto `ListInspectTemplates`."]
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
                pub fn iter_inspect_templates<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_inspect_templates_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_inspect_templates_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2InspectTemplate,
                > {
                    self.iter_inspect_templates_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_inspect_templates_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2InspectTemplate,
                > {
                    self.iter_inspect_templates_with_fields(Some("*"))
                }
                pub fn iter_inspect_templates_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "inspectTemplates").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "inspectTemplates")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
                > {
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
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
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
                    crate::schemas::GooglePrivacyDlpV2ListInspectTemplatesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/inspectTemplates");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [InspectTemplatesActions::patch()](struct.InspectTemplatesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateInspectTemplateRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2InspectTemplate, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
        pub mod job_triggers {
            pub mod params {}
            pub struct JobTriggersActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> JobTriggersActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Activate a job trigger. Causes the immediate execute of a trigger\ninstead of waiting on the trigger event to occur."]
                pub fn activate(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2ActivateJobTriggerRequest,
                    name: impl Into<String>,
                ) -> ActivateRequestBuilder {
                    ActivateRequestBuilder {
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
                #[doc = "Creates a job trigger to run DLP actions such as scanning storage for\nsensitive information on a set schedule.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateJobTriggerRequest,
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
                #[doc = "Deletes a job trigger.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."]
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
                #[doc = "Gets a job trigger.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."]
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
                #[doc = "Lists job triggers.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a job trigger.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateJobTriggerRequest,
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
            #[doc = "Created via [JobTriggersActions::activate()](struct.JobTriggersActions.html#method.activate)"]
            #[derive(Debug, Clone)]
            pub struct ActivateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2ActivateJobTriggerRequest,
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
            impl<'a> ActivateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DlpJob, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2DlpJob, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":activate");
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
            #[doc = "Created via [JobTriggersActions::create()](struct.JobTriggersActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateJobTriggerRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2JobTrigger, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2JobTrigger, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobTriggers");
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
            #[doc = "Created via [JobTriggersActions::delete()](struct.JobTriggersActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [JobTriggersActions::get()](struct.JobTriggersActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2JobTrigger, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2JobTrigger, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [JobTriggersActions::list()](struct.JobTriggersActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                order_by: Option<String>,
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
                #[doc = "Optional. Allows filtering.\n\nSupported syntax:\n\n* Filter expressions are made up of one or more restrictions.\n* Restrictions can be combined by `AND` or `OR` logical operators. A\n  sequence of restrictions implicitly uses `AND`.\n* A restriction has the form of `<field> <operator> <value>`.\n* Supported fields/values for inspect jobs:\n  * `status` - HEALTHY|PAUSED|CANCELLED\n  * `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY\n  * 'last_run_time` - RFC 3339 formatted timestamp, surrounded by\n    quotation marks. Nanoseconds are ignored.\n  * 'error_count' - Number of errors that have occurred while running.\n* The operator must be `=` or `!=` for status and inspected_storage.\n\nExamples:\n\n* inspected_storage = cloud_storage AND status = HEALTHY\n* inspected_storage = cloud_storage OR inspected_storage = bigquery\n* inspected_storage = cloud_storage AND (state = PAUSED OR state = HEALTHY)\n* last_run_time > \"2017-12-12T00:00:00+00:00\"\n\nThe length of this field should be no more than 500 characters."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional comma separated list of triggeredJob fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc,update_time, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the JobTrigger was created.\n* `update_time`: corresponds to time the JobTrigger was last updated.\n* `last_run_time`: corresponds to the last time the JobTrigger ran.\n* `name`: corresponds to JobTrigger's name.\n* `display_name`: corresponds to JobTrigger's display name.\n* `status`: corresponds to JobTrigger's status."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by a server."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto ListJobTriggers. `order_by` field must not\nchange for subsequent calls."]
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
                pub fn iter_job_triggers<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_job_triggers_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_job_triggers_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2JobTrigger>
                {
                    self.iter_job_triggers_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_job_triggers_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2JobTrigger>
                {
                    self.iter_job_triggers_with_fields(Some("*"))
                }
                pub fn iter_job_triggers_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "jobTriggers").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "jobTriggers")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListJobTriggersResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListJobTriggersResponse,
                > {
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2ListJobTriggersResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2ListJobTriggersResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobTriggers");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [JobTriggersActions::patch()](struct.JobTriggersActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateJobTriggerRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2JobTrigger, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2JobTrigger, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the content resource"]
                pub fn content(
                    &self,
                ) -> crate::resources::projects::locations::content::ContentActions
                {
                    crate::resources::projects::locations::content::ContentActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod content {
                pub mod params {}
                pub struct ContentActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ContentActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "De-identifies potentially sensitive info from a ContentItem.\nThis method has limits on input size and output size.\nSee https://cloud.google.com/dlp/docs/deidentify-sensitive-data to\nlearn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated."]
                    pub fn deidentify(
                        &self,
                        request: crate::schemas::GooglePrivacyDlpV2DeidentifyContentRequest,
                        parent: impl Into<String>,
                        location: impl Into<String>,
                    ) -> DeidentifyRequestBuilder {
                        DeidentifyRequestBuilder {
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
                            location: location.into(),
                        }
                    }
                    #[doc = "Finds potentially sensitive info in content.\nThis method has limits on input size, processing time, and output size.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.\n\nFor how to guides, see https://cloud.google.com/dlp/docs/inspecting-images\nand https://cloud.google.com/dlp/docs/inspecting-text,"]
                    pub fn inspect(
                        &self,
                        request: crate::schemas::GooglePrivacyDlpV2InspectContentRequest,
                        parent: impl Into<String>,
                        location: impl Into<String>,
                    ) -> InspectRequestBuilder {
                        InspectRequestBuilder {
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
                            location: location.into(),
                        }
                    }
                    #[doc = "Re-identifies content that has been de-identified.\nSee\nhttps://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example\nto learn more."]
                    pub fn reidentify(
                        &self,
                        request: crate::schemas::GooglePrivacyDlpV2ReidentifyContentRequest,
                        parent: impl Into<String>,
                        location: impl Into<String>,
                    ) -> ReidentifyRequestBuilder {
                        ReidentifyRequestBuilder {
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
                            location: location.into(),
                        }
                    }
                }
                #[doc = "Created via [ContentActions::deidentify()](struct.ContentActions.html#method.deidentify)"]
                #[derive(Debug, Clone)]
                pub struct DeidentifyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GooglePrivacyDlpV2DeidentifyContentRequest,
                    parent: String,
                    location: String,
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
                impl<'a> DeidentifyRequestBuilder<'a> {
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
                        crate::schemas::GooglePrivacyDlpV2DeidentifyContentResponse,
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
                        crate::schemas::GooglePrivacyDlpV2DeidentifyContentResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let mut output = "https://dlp.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/content:deidentify");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ContentActions::inspect()](struct.ContentActions.html#method.inspect)"]
                #[derive(Debug, Clone)]
                pub struct InspectRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GooglePrivacyDlpV2InspectContentRequest,
                    parent: String,
                    location: String,
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
                impl<'a> InspectRequestBuilder<'a> {
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
                        crate::schemas::GooglePrivacyDlpV2InspectContentResponse,
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
                        crate::schemas::GooglePrivacyDlpV2InspectContentResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let mut output = "https://dlp.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/content:inspect");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ContentActions::reidentify()](struct.ContentActions.html#method.reidentify)"]
                #[derive(Debug, Clone)]
                pub struct ReidentifyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GooglePrivacyDlpV2ReidentifyContentRequest,
                    parent: String,
                    location: String,
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
                impl<'a> ReidentifyRequestBuilder<'a> {
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
                        crate::schemas::GooglePrivacyDlpV2ReidentifyContentResponse,
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
                        crate::schemas::GooglePrivacyDlpV2ReidentifyContentResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let mut output = "https://dlp.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/content:reidentify");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        pub mod stored_info_types {
            pub mod params {}
            pub struct StoredInfoTypesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> StoredInfoTypesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a pre-built stored infoType to be used for inspection.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
                pub fn create(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2CreateStoredInfoTypeRequest,
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
                #[doc = "Deletes a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
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
                #[doc = "Gets a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
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
                #[doc = "Lists stored infoTypes.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
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
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the stored infoType by creating a new version. The existing version\nwill continue to be used until the new version is ready.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest,
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
            #[doc = "Created via [StoredInfoTypesActions::create()](struct.StoredInfoTypesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2CreateStoredInfoTypeRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/storedInfoTypes");
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
            #[doc = "Created via [StoredInfoTypesActions::delete()](struct.StoredInfoTypesActions.html#method.delete)"]
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
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [StoredInfoTypesActions::get()](struct.StoredInfoTypesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
            #[doc = "Created via [StoredInfoTypesActions::list()](struct.StoredInfoTypesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                order_by: Option<String>,
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
                #[doc = "Optional comma separated list of fields to order by,\nfollowed by `asc` or `desc` postfix. This list is case-insensitive,\ndefault sorting order is ascending, redundant space characters are\ninsignificant.\n\nExample: `name asc, display_name, create_time desc`\n\nSupported fields are:\n\n* `create_time`: corresponds to time the most recent version of the\n  resource was created.\n* `state`: corresponds to the state of the resource.\n* `name`: corresponds to resource name.\n* `display_name`: corresponds to info type's display name."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Optional size of the page, can be limited by server. If zero server returns\na page of max size 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional page token to continue retrieval. Comes from previous call\nto `ListStoredInfoTypes`."]
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
                pub fn iter_stored_info_types<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_stored_info_types_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_stored_info_types_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2StoredInfoType>
                {
                    self.iter_stored_info_types_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_stored_info_types_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GooglePrivacyDlpV2StoredInfoType>
                {
                    self.iter_stored_info_types_with_fields(Some("*"))
                }
                pub fn iter_stored_info_types_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "storedInfoTypes").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "storedInfoTypes")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
                > {
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
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
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
                    crate::schemas::GooglePrivacyDlpV2ListStoredInfoTypesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/storedInfoTypes");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    self._execute()
                }
            }
            #[doc = "Created via [StoredInfoTypesActions::patch()](struct.StoredInfoTypesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest,
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
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GooglePrivacyDlpV2StoredInfoType, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://dlp.googleapis.com/".to_owned();
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
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
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

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
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
