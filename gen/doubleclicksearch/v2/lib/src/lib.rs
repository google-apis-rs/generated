#![doc = "# Resources and Methods\n    * [conversion](resources/conversion/struct.ConversionActions.html)\n      * [*get*](resources/conversion/struct.GetRequestBuilder.html), [*insert*](resources/conversion/struct.InsertRequestBuilder.html), [*update*](resources/conversion/struct.UpdateRequestBuilder.html), [*updateAvailability*](resources/conversion/struct.UpdateAvailabilityRequestBuilder.html)\n    * [reports](resources/reports/struct.ReportsActions.html)\n      * [*generate*](resources/reports/struct.GenerateRequestBuilder.html), [*get*](resources/reports/struct.GetRequestBuilder.html), [*getFile*](resources/reports/struct.GetFileRequestBuilder.html), [*request*](resources/reports/struct.RequestRequestBuilder.html)\n    * [saved_columns](resources/saved_columns/struct.SavedColumnsActions.html)\n      * [*list*](resources/saved_columns/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your advertising data in DoubleClick Search\n\n`https://www.googleapis.com/auth/doubleclicksearch`"]
    pub const DOUBLECLICKSEARCH: &str = "https://www.googleapis.com/auth/doubleclicksearch";
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
    pub struct Availability {
        #[doc = "DS advertiser ID."]
        #[serde(
            rename = "advertiserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub advertiser_id: ::std::option::Option<i64>,
        #[doc = "DS agency ID."]
        #[serde(
            rename = "agencyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: ::std::option::Option<i64>,
        #[doc = "The time by which all conversions have been uploaded, in epoch millis UTC."]
        #[serde(
            rename = "availabilityTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub availability_timestamp: ::std::option::Option<i64>,
        #[doc = "The numeric segmentation identifier (for example, DoubleClick Search\nFloodlight activity ID)."]
        #[serde(
            rename = "segmentationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub segmentation_id: ::std::option::Option<i64>,
        #[doc = "The friendly segmentation identifier (for example, DoubleClick Search\nFloodlight activity name)."]
        #[serde(
            rename = "segmentationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segmentation_name: ::std::option::Option<String>,
        #[doc = "The segmentation type that this availability is for (its default value is\n`FLOODLIGHT`)."]
        #[serde(
            rename = "segmentationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segmentation_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Availability {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Availability {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Conversion {
        #[doc = "DS ad group ID."]
        #[serde(
            rename = "adGroupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ad_group_id: ::std::option::Option<i64>,
        #[doc = "DS ad ID."]
        #[serde(
            rename = "adId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ad_id: ::std::option::Option<i64>,
        #[doc = "DS advertiser ID."]
        #[serde(
            rename = "advertiserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub advertiser_id: ::std::option::Option<i64>,
        #[doc = "DS agency ID."]
        #[serde(
            rename = "agencyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: ::std::option::Option<i64>,
        #[doc = "Available to advertisers only after contacting DoubleClick Search customer\nsupport."]
        #[serde(
            rename = "attributionModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribution_model: ::std::option::Option<String>,
        #[doc = "DS campaign ID."]
        #[serde(
            rename = "campaignId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub campaign_id: ::std::option::Option<i64>,
        #[doc = "Sales channel for the product. Acceptable values are: <ul>\n\n<li>\"`local`\": a physical store</li> <li>\"`online`\":\nan online store</li> </ul>"]
        #[serde(
            rename = "channel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel: ::std::option::Option<String>,
        #[doc = "DS click ID for the conversion."]
        #[serde(
            rename = "clickId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub click_id: ::std::option::Option<String>,
        #[doc = "For offline conversions, advertisers provide this ID. Advertisers can\nspecify any ID that is meaningful to them. Each conversion in a request\nmust specify a unique ID, and the combination of ID and timestamp must be\nunique amongst all conversions within the advertiser.<br> For online\nconversions, DS copies the `dsConversionId` or\n`floodlightOrderId` into this property depending on the\nadvertiser's Floodlight instructions."]
        #[serde(
            rename = "conversionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_id: ::std::option::Option<String>,
        #[doc = "The time at which the conversion was last modified, in epoch millis UTC."]
        #[serde(
            rename = "conversionModifiedTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub conversion_modified_timestamp: ::std::option::Option<i64>,
        #[doc = "The time at which the conversion took place, in epoch millis UTC."]
        #[serde(
            rename = "conversionTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_timestamp: ::std::option::Option<String>,
        #[doc = "Available to advertisers only after contacting DoubleClick Search customer\nsupport."]
        #[serde(
            rename = "countMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count_millis: ::std::option::Option<i64>,
        #[doc = "DS criterion (keyword) ID."]
        #[serde(
            rename = "criterionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub criterion_id: ::std::option::Option<i64>,
        #[doc = "The currency code for the conversion's revenue. Should be in ISO 4217\nalphabetic (3-char) format."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Custom dimensions for the conversion, which can be used to filter data in a\nreport."]
        #[serde(
            rename = "customDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_dimension: ::std::option::Option<Vec<crate::schemas::CustomDimension>>,
        #[doc = "Custom metrics for the conversion."]
        #[serde(
            rename = "customMetric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_metric: ::std::option::Option<Vec<crate::schemas::CustomMetric>>,
        #[doc = "The type of device on which the conversion occurred."]
        #[serde(
            rename = "deviceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_type: ::std::option::Option<String>,
        #[doc = "ID that DoubleClick Search generates for each conversion."]
        #[serde(
            rename = "dsConversionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ds_conversion_id: ::std::option::Option<i64>,
        #[doc = "DS engine account ID."]
        #[serde(
            rename = "engineAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub engine_account_id: ::std::option::Option<i64>,
        #[doc = "The Floodlight order ID provided by the advertiser for the conversion."]
        #[serde(
            rename = "floodlightOrderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub floodlight_order_id: ::std::option::Option<String>,
        #[doc = "ID that DS generates and uses to uniquely identify the inventory account\nthat contains the product."]
        #[serde(
            rename = "inventoryAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub inventory_account_id: ::std::option::Option<i64>,
        #[doc = "The country registered for the Merchant Center feed that contains the\nproduct. Use an ISO 3166 code to specify a country."]
        #[serde(
            rename = "productCountry",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_country: ::std::option::Option<String>,
        #[doc = "DS product group ID."]
        #[serde(
            rename = "productGroupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub product_group_id: ::std::option::Option<i64>,
        #[doc = "The product ID (SKU)."]
        #[serde(
            rename = "productId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_id: ::std::option::Option<String>,
        #[doc = "The language registered for the Merchant Center feed that contains the\nproduct. Use an ISO 639 code to specify a language."]
        #[serde(
            rename = "productLanguage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_language: ::std::option::Option<String>,
        #[doc = "The quantity of this conversion, in millis."]
        #[serde(
            rename = "quantityMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub quantity_millis: ::std::option::Option<i64>,
        #[doc = "The type of the conversion, that is, either `ACTION` or\n`TRANSACTION`. An `ACTION` conversion is an action by\nthe user that has no monetarily quantifiable value, while a\n`TRANSACTION` conversion is an action that does have a\nmonetarily quantifiable value. Examples are email list signups\n(`ACTION`) versus ecommerce purchases\n(`TRANSACTION`)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The revenue amount of this `TRANSACTION` conversion, in micros\n(value multiplied by 1000000, no decimal). For example, to specify a\nrevenue value of \"10\"  enter \"10000000\" (10 million) in your request."]
        #[serde(
            rename = "revenueMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revenue_micros: ::std::option::Option<String>,
        #[doc = "The numeric segmentation identifier (for example, DoubleClick Search\nFloodlight activity ID)."]
        #[serde(
            rename = "segmentationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub segmentation_id: ::std::option::Option<i64>,
        #[doc = "The friendly segmentation identifier (for example, DoubleClick Search\nFloodlight activity name)."]
        #[serde(
            rename = "segmentationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segmentation_name: ::std::option::Option<String>,
        #[doc = "The segmentation type of this conversion (for example,\n`FLOODLIGHT`)."]
        #[serde(
            rename = "segmentationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segmentation_type: ::std::option::Option<String>,
        #[doc = "The state of the conversion, that is, either `ACTIVE` or\n`REMOVED`. Note: state DELETED is deprecated."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "The ID of the local store for which the product was advertised. Applicable\nonly when the channel is \"`local`\"."]
        #[serde(
            rename = "storeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub store_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Conversion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Conversion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ConversionList {
        #[doc = "The conversions being requested."]
        #[serde(
            rename = "conversion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion: ::std::option::Option<Vec<crate::schemas::Conversion>>,
        #[doc = "Identifies this as a ConversionList resource. Value: the fixed string\n<code>doubleclicksearch#conversionList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConversionList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConversionList {
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
    pub struct CustomDimension {
        #[doc = "Custom dimension name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Custom dimension value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomMetric {
        #[doc = "Custom metric name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Custom metric numeric value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for CustomMetric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomMetric {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Report {
        #[doc = "Asynchronous report only. Contains a list of generated report files once\nthe report has successfully completed."]
        #[serde(
            rename = "files",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files: ::std::option::Option<Vec<crate::schemas::ReportFilesItems>>,
        #[doc = "Asynchronous report only. Id of the report."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Asynchronous report only. True if and only if the report has completed\nsuccessfully and the report files are ready to be downloaded."]
        #[serde(
            rename = "isReportReady",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_report_ready: ::std::option::Option<bool>,
        #[doc = "Identifies this as a Report resource. Value: the fixed string\n`doubleclicksearch#report`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The request that created the report. Optional fields not specified in the\noriginal request are filled with default values."]
        #[serde(
            rename = "request",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request: ::std::option::Option<crate::schemas::ReportRequest>,
        #[doc = "The number of report rows generated by the report, not including headers."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
        #[doc = "Synchronous report only. Generated report rows."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::ReportRow>>,
        #[doc = "The currency code of all monetary values produced in the report, including\nvalues that are set by users (e.g., keyword bid settings) and metrics\n(e.g., cost and revenue). The currency code of a report is determined by\nthe `statisticsCurrency` field of the report request."]
        #[serde(
            rename = "statisticsCurrencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statistics_currency_code: ::std::option::Option<String>,
        #[doc = "If all statistics of the report are sourced from the same time zone, this\nwould be it. Otherwise the field is unset."]
        #[serde(
            rename = "statisticsTimeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statistics_time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Report {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Report {
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
    pub struct ReportFilesItems {
        #[doc = "The size of this report file in bytes."]
        #[serde(
            rename = "byteCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub byte_count: ::std::option::Option<i64>,
        #[doc = "Use this url to download the report file."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportFilesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportFilesItems {
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
    pub struct ReportApiColumnSpec {
        #[doc = "Name of a DoubleClick Search column to include in the report."]
        #[serde(
            rename = "columnName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_name: ::std::option::Option<String>,
        #[doc = "Segments a report by a custom dimension. The report must be scoped to an\nadvertiser or lower, and the custom dimension must already be set up in\nDoubleClick Search. The custom dimension name, which appears in DoubleClick\nSearch, is case sensitive.  \nIf used in a conversion report, returns the\nvalue of the specified custom dimension for the given conversion, if set.\nThis column does not segment the conversion report."]
        #[serde(
            rename = "customDimensionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_dimension_name: ::std::option::Option<String>,
        #[doc = "Name of a custom metric to include in the report. The report must be scoped\nto an advertiser or lower, and the custom metric must already be set up in\nDoubleClick Search. The custom metric name, which appears in DoubleClick\nSearch, is case sensitive."]
        #[serde(
            rename = "customMetricName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_metric_name: ::std::option::Option<String>,
        #[doc = "Inclusive day in YYYY-MM-DD format. When provided, this overrides the\noverall time range of the report for this column only. Must be provided\ntogether with `startDate`."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<String>,
        #[doc = "Synchronous report only. Set to `true` to group by this column.\nDefaults to `false`."]
        #[serde(
            rename = "groupByColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_by_column: ::std::option::Option<bool>,
        #[doc = "Text used to identify this column in the report output; defaults to\n`columnName` or `savedColumnName` when not specified.\nThis can be used to prevent collisions between DoubleClick Search columns\nand saved columns with the same name."]
        #[serde(
            rename = "headerText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_text: ::std::option::Option<String>,
        #[doc = "The platform that is used to provide data for the custom dimension.\nAcceptable values are \"floodlight\"."]
        #[serde(
            rename = "platformSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_source: ::std::option::Option<String>,
        #[doc = "Returns metrics only for a specific type of product activity. Accepted\nvalues are: <ul> <li>\"`sold`\": returns metrics only for products\nthat were sold</li> <li>\"`advertised`\": returns metrics only for\nproducts that were advertised in a Shopping campaign, and that might or\nmight not have been sold</li> </ul>"]
        #[serde(
            rename = "productReportPerspective",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_report_perspective: ::std::option::Option<String>,
        #[doc = "Name of a saved column to include in the report. The report must be scoped\nat advertiser or lower, and this saved column must already be created in\nthe DoubleClick Search UI."]
        #[serde(
            rename = "savedColumnName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saved_column_name: ::std::option::Option<String>,
        #[doc = "Inclusive date in YYYY-MM-DD format. When provided, this overrides the\noverall time range of the report for this column only. Must be provided\ntogether with `endDate`."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportApiColumnSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportApiColumnSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportRequest {
        #[doc = "The columns to include in the report. This includes both DoubleClick Search\ncolumns and saved columns. For DoubleClick Search columns, only the\n`columnName` parameter is required. For saved columns only the\n`savedColumnName` parameter is required. Both\n`columnName` and `savedColumnName` cannot be set in\nthe same stanza.  \nThe maximum number of columns per request is 300."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<Vec<crate::schemas::ReportApiColumnSpec>>,
        #[doc = "Format that the report should be returned in. Currently `csv` or\n`tsv` is supported."]
        #[serde(
            rename = "downloadFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_format: ::std::option::Option<String>,
        #[doc = "A list of filters to be applied to the report.  \nThe maximum number of filters per request is 300."]
        #[serde(
            rename = "filters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filters: ::std::option::Option<Vec<crate::schemas::ReportRequestFiltersItems>>,
        #[doc = "Determines if removed entities should be included in the report. Defaults\nto `false`. Deprecated, please use\n`includeRemovedEntities` instead."]
        #[serde(
            rename = "includeDeletedEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_deleted_entities: ::std::option::Option<bool>,
        #[doc = "Determines if removed entities should be included in the report. Defaults\nto `false`."]
        #[serde(
            rename = "includeRemovedEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_removed_entities: ::std::option::Option<bool>,
        #[doc = "Asynchronous report only. The maximum number of rows per report file. A\nlarge report is split into many files based on this field. Acceptable\nvalues are `1000000` to `100000000`, inclusive."]
        #[serde(
            rename = "maxRowsPerFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_rows_per_file: ::std::option::Option<i32>,
        #[doc = "Synchronous report only. A list of columns and directions defining sorting\nto be performed on the report rows.  \nThe maximum number of orderings per request is 300."]
        #[serde(
            rename = "orderBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_by: ::std::option::Option<Vec<crate::schemas::ReportRequestOrderByItems>>,
        #[doc = "The reportScope is a set of IDs that are used to determine which subset of\nentities will be returned in the report. The full lineage of IDs from the\nlowest scoped level desired up through agency is required."]
        #[serde(
            rename = "reportScope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_scope: ::std::option::Option<crate::schemas::ReportRequestReportScope>,
        #[doc = "Determines the type of rows that are returned in the report. For example,\nif you specify `reportType: keyword`, each row in the report\nwill contain data about a keyword. See the [Types of\nReports](/search-ads/v2/report-types/) reference for the columns that are\navailable for each type."]
        #[serde(
            rename = "reportType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_type: ::std::option::Option<String>,
        #[doc = "Synchronous report only. The maximum number of rows to return; additional\nrows are dropped. Acceptable values are `0` to\n`10000`, inclusive. Defaults to `10000`."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
        #[doc = "Synchronous report only. Zero-based index of the first row to return.\nAcceptable values are `0` to `50000`, inclusive.\nDefaults to `0`."]
        #[serde(
            rename = "startRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_row: ::std::option::Option<i32>,
        #[doc = "Specifies the currency in which monetary will be returned. Possible values\nare: `usd`, `agency` (valid if the report is scoped\nto agency or lower), `advertiser` (valid if the report is scoped\nto * advertiser or lower), or `account` (valid if the report is\nscoped to engine account or lower)."]
        #[serde(
            rename = "statisticsCurrency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statistics_currency: ::std::option::Option<String>,
        #[doc = "If metrics are requested in a report, this argument will be used to\nrestrict the metrics to a specific time range."]
        #[serde(
            rename = "timeRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_range: ::std::option::Option<crate::schemas::ReportRequestTimeRange>,
        #[doc = "If `true`, the report would only be created if all the requested\nstat data are sourced from a single timezone. Defaults to\n`false`."]
        #[serde(
            rename = "verifySingleTimeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verify_single_time_zone: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportRequestFiltersItems {
        #[doc = "Column to perform the filter on. This can be a DoubleClick Search column\nor a saved column."]
        #[serde(
            rename = "column",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column: ::std::option::Option<crate::schemas::ReportApiColumnSpec>,
        #[doc = "Operator to use in the filter. See the filter reference for a list of\navailable operators."]
        #[serde(
            rename = "operator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator: ::std::option::Option<String>,
        #[doc = "A list of values to filter the column value against.  \nThe maximum number of filter values per request is 300."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for ReportRequestFiltersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRequestFiltersItems {
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
    pub struct ReportRequestOrderByItems {
        #[doc = "Column to perform the sort on. This can be a DoubleClick Search-defined\ncolumn or a saved column."]
        #[serde(
            rename = "column",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column: ::std::option::Option<crate::schemas::ReportApiColumnSpec>,
        #[doc = "The sort direction, which is either `ascending` or\n`descending`."]
        #[serde(
            rename = "sortOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_order: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportRequestOrderByItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRequestOrderByItems {
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
    pub struct ReportRequestReportScope {
        #[doc = "DS ad group ID."]
        #[serde(
            rename = "adGroupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ad_group_id: ::std::option::Option<i64>,
        #[doc = "DS ad ID."]
        #[serde(
            rename = "adId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ad_id: ::std::option::Option<i64>,
        #[doc = "DS advertiser ID."]
        #[serde(
            rename = "advertiserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub advertiser_id: ::std::option::Option<i64>,
        #[doc = "DS agency ID."]
        #[serde(
            rename = "agencyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: ::std::option::Option<i64>,
        #[doc = "DS campaign ID."]
        #[serde(
            rename = "campaignId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub campaign_id: ::std::option::Option<i64>,
        #[doc = "DS engine account ID."]
        #[serde(
            rename = "engineAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub engine_account_id: ::std::option::Option<i64>,
        #[doc = "DS keyword ID."]
        #[serde(
            rename = "keywordId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub keyword_id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ReportRequestReportScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRequestReportScope {
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
    pub struct ReportRequestTimeRange {
        #[doc = "Inclusive UTC timestamp in RFC format, e.g.,\n`2013-07-16T10:16:23.555Z`. See additional references on how\nchanged attribute reports work."]
        #[serde(
            rename = "changedAttributesSinceTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub changed_attributes_since_timestamp: ::std::option::Option<String>,
        #[doc = "Inclusive UTC timestamp in RFC format, e.g.,\n`2013-07-16T10:16:23.555Z`. See additional references on how\nchanged metrics reports work."]
        #[serde(
            rename = "changedMetricsSinceTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub changed_metrics_since_timestamp: ::std::option::Option<String>,
        #[doc = "Inclusive date in YYYY-MM-DD format."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<String>,
        #[doc = "Inclusive date in YYYY-MM-DD format."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportRequestTimeRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRequestTimeRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    pub type ReportRow = ::std::collections::BTreeMap<String, ::serde_json::Value>;
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SavedColumn {
        #[doc = "Identifies this as a SavedColumn resource. Value: the fixed string\n<code>doubleclicksearch#savedColumn</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The type of data this saved column will produce."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The name of the saved column."]
        #[serde(
            rename = "savedColumnName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saved_column_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SavedColumn {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SavedColumn {
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
    pub struct SavedColumnList {
        #[doc = "The saved columns being requested."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::SavedColumn>>,
        #[doc = "Identifies this as a SavedColumnList resource. Value: the fixed string\n<code>doubleclicksearch#savedColumnList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SavedColumnList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SavedColumnList {
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
    pub struct UpdateAvailabilityRequest {
        #[doc = "The availabilities being requested."]
        #[serde(
            rename = "availabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availabilities: ::std::option::Option<Vec<crate::schemas::Availability>>,
    }
    impl ::google_field_selector::FieldSelector for UpdateAvailabilityRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateAvailabilityRequest {
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
    pub struct UpdateAvailabilityResponse {
        #[doc = "The availabilities being returned."]
        #[serde(
            rename = "availabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availabilities: ::std::option::Option<Vec<crate::schemas::Availability>>,
    }
    impl ::google_field_selector::FieldSelector for UpdateAvailabilityResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateAvailabilityResponse {
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
    #[doc = "Actions that can be performed on the conversion resource"]
    pub fn conversion(&self) -> crate::resources::conversion::ConversionActions {
        crate::resources::conversion::ConversionActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the reports resource"]
    pub fn reports(&self) -> crate::resources::reports::ReportsActions {
        crate::resources::reports::ReportsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the saved_columns resource"]
    pub fn saved_columns(&self) -> crate::resources::saved_columns::SavedColumnsActions {
        crate::resources::saved_columns::SavedColumnsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod conversion {
        pub mod params {}
        pub struct ConversionActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ConversionActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves a list of conversions from a DoubleClick Search engine account."]
            pub fn get(
                &self,
                agency_id: i64,
                advertiser_id: i64,
                engine_account_id: i64,
                end_date: i32,
                row_count: i32,
                start_date: i32,
                start_row: u32,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    agency_id,
                    advertiser_id,
                    engine_account_id,
                    end_date,
                    row_count,
                    start_date,
                    start_row,
                    ad_group_id: None,
                    ad_id: None,
                    campaign_id: None,
                    criterion_id: None,
                }
            }
            #[doc = "Inserts a batch of new conversions into DoubleClick Search."]
            pub fn insert(&self, request: crate::schemas::ConversionList) -> InsertRequestBuilder {
                InsertRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Updates a batch of conversions in DoubleClick Search."]
            pub fn update(&self, request: crate::schemas::ConversionList) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Updates the availabilities of a batch of floodlight activities in\nDoubleClick Search."]
            pub fn update_availability(
                &self,
                request: crate::schemas::UpdateAvailabilityRequest,
            ) -> UpdateAvailabilityRequestBuilder {
                UpdateAvailabilityRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[doc = "Created via [ConversionActions::get()](struct.ConversionActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            agency_id: i64,
            advertiser_id: i64,
            engine_account_id: i64,
            end_date: i32,
            row_count: i32,
            start_date: i32,
            start_row: u32,
            ad_group_id: Option<i64>,
            ad_id: Option<i64>,
            campaign_id: Option<i64>,
            criterion_id: Option<i64>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "Numeric ID of the ad group."]
            pub fn ad_group_id(mut self, value: i64) -> Self {
                self.ad_group_id = Some(value);
                self
            }
            #[doc = "Numeric ID of the ad."]
            pub fn ad_id(mut self, value: i64) -> Self {
                self.ad_id = Some(value);
                self
            }
            #[doc = "Numeric ID of the campaign."]
            pub fn campaign_id(mut self, value: i64) -> Self {
                self.campaign_id = Some(value);
                self
            }
            #[doc = "Numeric ID of the criterion."]
            pub fn criterion_id(mut self, value: i64) -> Self {
                self.criterion_id = Some(value);
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
            ) -> Result<crate::schemas::ConversionList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ConversionList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/agency/");
                {
                    let var_as_string = self.agency_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/advertiser/");
                {
                    let var_as_string = self.advertiser_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/engine/");
                {
                    let var_as_string = self.engine_account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/conversion");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("endDate", &self.end_date)]);
                let req = req.query(&[("rowCount", &self.row_count)]);
                let req = req.query(&[("startDate", &self.start_date)]);
                let req = req.query(&[("startRow", &self.start_row)]);
                let req = req.query(&[("adGroupId", &self.ad_group_id)]);
                let req = req.query(&[("adId", &self.ad_id)]);
                let req = req.query(&[("campaignId", &self.campaign_id)]);
                let req = req.query(&[("criterionId", &self.criterion_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ConversionActions::insert()](struct.ConversionActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ConversionList,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ConversionList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ConversionList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/conversion");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ConversionActions::update()](struct.ConversionActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ConversionList,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ConversionList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ConversionList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/conversion");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ConversionActions::update_availability()](struct.ConversionActions.html#method.update_availability)"]
        #[derive(Debug, Clone)]
        pub struct UpdateAvailabilityRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::UpdateAvailabilityRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> UpdateAvailabilityRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::UpdateAvailabilityResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UpdateAvailabilityResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/conversion/updateAvailability");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
    pub mod reports {
        pub mod params {}
        pub struct ReportsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ReportsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Generates and returns a report immediately."]
            pub fn generate(
                &self,
                request: crate::schemas::ReportRequest,
            ) -> GenerateRequestBuilder {
                GenerateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Polls for the status of a report request."]
            pub fn get(&self, report_id: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    report_id: report_id.into(),
                }
            }
            #[doc = "Downloads a report file encoded in UTF-8."]
            pub fn get_file(
                &self,
                report_id: impl Into<String>,
                report_fragment: i32,
            ) -> GetFileRequestBuilder {
                GetFileRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    report_id: report_id.into(),
                    report_fragment,
                }
            }
            #[doc = "Inserts a report request into the reporting system."]
            pub fn request(&self, request: crate::schemas::ReportRequest) -> RequestRequestBuilder {
                RequestRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[doc = "Created via [ReportsActions::generate()](struct.ReportsActions.html#method.generate)"]
        #[derive(Debug, Clone)]
        pub struct GenerateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ReportRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GenerateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Report, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Report, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/reports/generate");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ReportsActions::get()](struct.ReportsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            report_id: String,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Report, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Report, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/reports/");
                {
                    let var_as_str = &self.report_id;
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
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ReportsActions::get_file()](struct.ReportsActions.html#method.get_file)"]
        #[derive(Debug, Clone)]
        pub struct GetFileRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            report_id: String,
            report_fragment: i32,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetFileRequestBuilder<'a> {
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
            fn _download_path(&self) -> String {
                let mut output = "https://www.googleapis.com/download/".to_owned();
                output.push_str("doubleclicksearch/v2/reports/");
                {
                    let var_as_str = &self.report_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/files/");
                {
                    let var_as_string = self.report_fragment.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            pub fn download<W>(mut self, output: &mut W) -> Result<u64, crate::Error>
            where
                W: ::std::io::Write + ?Sized,
            {
                self.alt = Some(crate::params::Alt::Media);
                Ok(
                    crate::error_from_response(self._request(&self._path())?.send()?)?
                        .copy_to(output)?,
                )
            }
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/reports/");
                {
                    let var_as_str = &self.report_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/files/");
                {
                    let var_as_string = self.report_fragment.to_string();
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
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ReportsActions::request()](struct.ReportsActions.html#method.request)"]
        #[derive(Debug, Clone)]
        pub struct RequestRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ReportRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> RequestRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Report, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Report, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/reports");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
    pub mod saved_columns {
        pub mod params {}
        pub struct SavedColumnsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SavedColumnsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieve the list of saved columns for a specified advertiser."]
            pub fn list(&self, agency_id: i64, advertiser_id: i64) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    agency_id,
                    advertiser_id,
                }
            }
        }
        #[doc = "Created via [SavedColumnsActions::list()](struct.SavedColumnsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            agency_id: i64,
            advertiser_id: i64,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SavedColumnList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SavedColumnList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("doubleclicksearch/v2/agency/");
                {
                    let var_as_string = self.agency_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/advertiser/");
                {
                    let var_as_string = self.advertiser_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/savedcolumns");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
