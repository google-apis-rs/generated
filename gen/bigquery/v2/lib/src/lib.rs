#![doc = "# Resources and Methods\n    * [datasets](resources/datasets/struct.DatasetsActions.html)\n      * [*delete*](resources/datasets/struct.DeleteRequestBuilder.html), [*get*](resources/datasets/struct.GetRequestBuilder.html), [*insert*](resources/datasets/struct.InsertRequestBuilder.html), [*list*](resources/datasets/struct.ListRequestBuilder.html), [*patch*](resources/datasets/struct.PatchRequestBuilder.html), [*update*](resources/datasets/struct.UpdateRequestBuilder.html)\n    * [jobs](resources/jobs/struct.JobsActions.html)\n      * [*cancel*](resources/jobs/struct.CancelRequestBuilder.html), [*get*](resources/jobs/struct.GetRequestBuilder.html), [*getQueryResults*](resources/jobs/struct.GetQueryResultsRequestBuilder.html), [*insert*](resources/jobs/struct.InsertRequestBuilder.html), [*list*](resources/jobs/struct.ListRequestBuilder.html), [*query*](resources/jobs/struct.QueryRequestBuilder.html)\n    * [models](resources/models/struct.ModelsActions.html)\n      * [*delete*](resources/models/struct.DeleteRequestBuilder.html), [*get*](resources/models/struct.GetRequestBuilder.html), [*list*](resources/models/struct.ListRequestBuilder.html), [*patch*](resources/models/struct.PatchRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*getServiceAccount*](resources/projects/struct.GetServiceAccountRequestBuilder.html), [*list*](resources/projects/struct.ListRequestBuilder.html)\n    * [routines](resources/routines/struct.RoutinesActions.html)\n      * [*delete*](resources/routines/struct.DeleteRequestBuilder.html), [*get*](resources/routines/struct.GetRequestBuilder.html), [*insert*](resources/routines/struct.InsertRequestBuilder.html), [*list*](resources/routines/struct.ListRequestBuilder.html), [*update*](resources/routines/struct.UpdateRequestBuilder.html)\n    * [row_access_policies](resources/row_access_policies/struct.RowAccessPoliciesActions.html)\n      * [*getIamPolicy*](resources/row_access_policies/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/row_access_policies/struct.ListRequestBuilder.html), [*setIamPolicy*](resources/row_access_policies/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/row_access_policies/struct.TestIamPermissionsRequestBuilder.html)\n    * [tabledata](resources/tabledata/struct.TabledataActions.html)\n      * [*insertAll*](resources/tabledata/struct.InsertAllRequestBuilder.html), [*list*](resources/tabledata/struct.ListRequestBuilder.html)\n    * [tables](resources/tables/struct.TablesActions.html)\n      * [*delete*](resources/tables/struct.DeleteRequestBuilder.html), [*get*](resources/tables/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/tables/struct.GetIamPolicyRequestBuilder.html), [*insert*](resources/tables/struct.InsertRequestBuilder.html), [*list*](resources/tables/struct.ListRequestBuilder.html), [*patch*](resources/tables/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/tables/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/tables/struct.TestIamPermissionsRequestBuilder.html), [*update*](resources/tables/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data in Google BigQuery\n\n`https://www.googleapis.com/auth/bigquery`"]
    pub const BIGQUERY: &str = "https://www.googleapis.com/auth/bigquery";
    #[doc = "Insert data into Google BigQuery\n\n`https://www.googleapis.com/auth/bigquery.insertdata`"]
    pub const BIGQUERY_INSERTDATA: &str = "https://www.googleapis.com/auth/bigquery.insertdata";
    #[doc = "View your data in Google BigQuery\n\n`https://www.googleapis.com/auth/bigquery.readonly`"]
    pub const BIGQUERY_READONLY: &str = "https://www.googleapis.com/auth/bigquery.readonly";
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform.read-only`"]
    pub const CLOUD_PLATFORM_READ_ONLY: &str =
        "https://www.googleapis.com/auth/cloud-platform.read-only";
    #[doc = "Manage your data and permissions in Google Cloud Storage\n\n`https://www.googleapis.com/auth/devstorage.full_control`"]
    pub const DEVSTORAGE_FULL_CONTROL: &str =
        "https://www.googleapis.com/auth/devstorage.full_control";
    #[doc = "View your data in Google Cloud Storage\n\n`https://www.googleapis.com/auth/devstorage.read_only`"]
    pub const DEVSTORAGE_READ_ONLY: &str = "https://www.googleapis.com/auth/devstorage.read_only";
    #[doc = "Manage your data in Google Cloud Storage\n\n`https://www.googleapis.com/auth/devstorage.read_write`"]
    pub const DEVSTORAGE_READ_WRITE: &str = "https://www.googleapis.com/auth/devstorage.read_write";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AggregateClassificationMetrics {
        #[doc = "Accuracy is the fraction of predictions given the correct label. For multiclass this is a micro-averaged metric."]
        #[serde(
            rename = "accuracy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accuracy: ::std::option::Option<f64>,
        #[doc = "The F1 score is an average of recall and precision. For multiclass this is a macro-averaged metric."]
        #[serde(
            rename = "f1Score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub f_1_score: ::std::option::Option<f64>,
        #[doc = "Logarithmic Loss. For multiclass this is a macro-averaged metric."]
        #[serde(
            rename = "logLoss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_loss: ::std::option::Option<f64>,
        #[doc = "Precision is the fraction of actual positive predictions that had positive actual labels. For multiclass this is a macro-averaged metric treating each class as a binary classifier."]
        #[serde(
            rename = "precision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub precision: ::std::option::Option<f64>,
        #[doc = "Recall is the fraction of actual positive labels that were given a positive prediction. For multiclass this is a macro-averaged metric."]
        #[serde(
            rename = "recall",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recall: ::std::option::Option<f64>,
        #[doc = "Area Under a ROC Curve. For multiclass this is a macro-averaged metric."]
        #[serde(
            rename = "rocAuc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub roc_auc: ::std::option::Option<f64>,
        #[doc = "Threshold at which the metrics are computed. For binary classification models this is the positive class threshold. For multi-class classfication models this is the confidence threshold."]
        #[serde(
            rename = "threshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threshold: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for AggregateClassificationMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateClassificationMetrics {
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
    pub struct Argument {
        #[doc = "Optional. Defaults to FIXED_TYPE."]
        #[serde(
            rename = "argumentKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub argument_kind: ::std::option::Option<crate::schemas::ArgumentArgumentKind>,
        #[doc = "Required unless argument_kind = ANY_TYPE."]
        #[serde(
            rename = "dataType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_type: ::std::option::Option<crate::schemas::StandardSqlDataType>,
        #[doc = "Optional. Specifies whether the argument is input or output. Can be set for procedures only."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::ArgumentMode>,
        #[doc = "Optional. The name of this argument. Can be absent for function return argument."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Argument {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Argument {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArgumentArgumentKind {
        #[doc = "The argument is any type, including struct or array, but not a table. To be added: FIXED_TABLE, ANY_TABLE"]
        AnyType,
        ArgumentKindUnspecified,
        #[doc = "The argument is a variable with fully specified type, which can be a struct or an array, but not a table."]
        FixedType,
    }
    impl ArgumentArgumentKind {
        pub fn as_str(self) -> &'static str {
            match self {
                ArgumentArgumentKind::AnyType => "ANY_TYPE",
                ArgumentArgumentKind::ArgumentKindUnspecified => "ARGUMENT_KIND_UNSPECIFIED",
                ArgumentArgumentKind::FixedType => "FIXED_TYPE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ArgumentArgumentKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ArgumentArgumentKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ArgumentArgumentKind, ()> {
            Ok(match s {
                "ANY_TYPE" => ArgumentArgumentKind::AnyType,
                "ARGUMENT_KIND_UNSPECIFIED" => ArgumentArgumentKind::ArgumentKindUnspecified,
                "FIXED_TYPE" => ArgumentArgumentKind::FixedType,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ArgumentArgumentKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArgumentArgumentKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArgumentArgumentKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANY_TYPE" => ArgumentArgumentKind::AnyType,
                "ARGUMENT_KIND_UNSPECIFIED" => ArgumentArgumentKind::ArgumentKindUnspecified,
                "FIXED_TYPE" => ArgumentArgumentKind::FixedType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ArgumentArgumentKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArgumentArgumentKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArgumentMode {
        #[doc = "The argument is input-only."]
        In,
        #[doc = "The argument is both an input and an output."]
        Inout,
        ModeUnspecified,
        #[doc = "The argument is output-only."]
        Out,
    }
    impl ArgumentMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ArgumentMode::In => "IN",
                ArgumentMode::Inout => "INOUT",
                ArgumentMode::ModeUnspecified => "MODE_UNSPECIFIED",
                ArgumentMode::Out => "OUT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ArgumentMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ArgumentMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ArgumentMode, ()> {
            Ok(match s {
                "IN" => ArgumentMode::In,
                "INOUT" => ArgumentMode::Inout,
                "MODE_UNSPECIFIED" => ArgumentMode::ModeUnspecified,
                "OUT" => ArgumentMode::Out,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ArgumentMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArgumentMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArgumentMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IN" => ArgumentMode::In,
                "INOUT" => ArgumentMode::Inout,
                "MODE_UNSPECIFIED" => ArgumentMode::ModeUnspecified,
                "OUT" => ArgumentMode::Out,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ArgumentMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArgumentMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArimaCoefficients {
        #[doc = "Auto-regressive coefficients, an array of double."]
        #[serde(
            rename = "autoRegressiveCoefficients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_regressive_coefficients: ::std::option::Option<Vec<f64>>,
        #[doc = "Intercept coefficient, just a double not an array."]
        #[serde(
            rename = "interceptCoefficient",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intercept_coefficient: ::std::option::Option<f64>,
        #[doc = "Moving-average coefficients, an array of double."]
        #[serde(
            rename = "movingAverageCoefficients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub moving_average_coefficients: ::std::option::Option<Vec<f64>>,
    }
    impl ::google_field_selector::FieldSelector for ArimaCoefficients {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaCoefficients {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArimaFittingMetrics {
        #[doc = "AIC."]
        #[serde(
            rename = "aic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aic: ::std::option::Option<f64>,
        #[doc = "Log-likelihood."]
        #[serde(
            rename = "logLikelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_likelihood: ::std::option::Option<f64>,
        #[doc = "Variance."]
        #[serde(
            rename = "variance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub variance: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for ArimaFittingMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaFittingMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArimaForecastingMetrics {
        #[doc = "Arima model fitting metrics."]
        #[serde(
            rename = "arimaFittingMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_fitting_metrics: ::std::option::Option<Vec<crate::schemas::ArimaFittingMetrics>>,
        #[doc = "Repeated as there can be many metric sets (one for each model) in auto-arima and the large-scale case."]
        #[serde(
            rename = "arimaSingleModelForecastingMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_single_model_forecasting_metrics:
            ::std::option::Option<Vec<crate::schemas::ArimaSingleModelForecastingMetrics>>,
        #[doc = "Whether Arima model fitted with drift or not. It is always false when d is not 1."]
        #[serde(
            rename = "hasDrift",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_drift: ::std::option::Option<Vec<bool>>,
        #[doc = "Non-seasonal order."]
        #[serde(
            rename = "nonSeasonalOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_seasonal_order: ::std::option::Option<Vec<crate::schemas::ArimaOrder>>,
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        #[serde(
            rename = "seasonalPeriods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seasonal_periods:
            ::std::option::Option<Vec<crate::schemas::ArimaForecastingMetricsSeasonalPeriodsItems>>,
        #[doc = "Id to differentiate different time series for the large-scale case."]
        #[serde(
            rename = "timeSeriesId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_id: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ArimaForecastingMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaForecastingMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArimaForecastingMetricsSeasonalPeriodsItems {
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[doc = "No seasonality"]
        NoSeasonality,
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        SeasonalPeriodTypeUnspecified,
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ArimaForecastingMetricsSeasonalPeriodsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ArimaForecastingMetricsSeasonalPeriodsItems::Daily => "DAILY",
                ArimaForecastingMetricsSeasonalPeriodsItems::Monthly => "MONTHLY",
                ArimaForecastingMetricsSeasonalPeriodsItems::NoSeasonality => "NO_SEASONALITY",
                ArimaForecastingMetricsSeasonalPeriodsItems::Quarterly => "QUARTERLY",
                ArimaForecastingMetricsSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified => {
                    "SEASONAL_PERIOD_TYPE_UNSPECIFIED"
                }
                ArimaForecastingMetricsSeasonalPeriodsItems::Weekly => "WEEKLY",
                ArimaForecastingMetricsSeasonalPeriodsItems::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ArimaForecastingMetricsSeasonalPeriodsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ArimaForecastingMetricsSeasonalPeriodsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ArimaForecastingMetricsSeasonalPeriodsItems, ()> {
            Ok(match s {
                "DAILY" => ArimaForecastingMetricsSeasonalPeriodsItems::Daily,
                "MONTHLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Monthly,
                "NO_SEASONALITY" => ArimaForecastingMetricsSeasonalPeriodsItems::NoSeasonality,
                "QUARTERLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Quarterly,
                "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => {
                    ArimaForecastingMetricsSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified
                }
                "WEEKLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Weekly,
                "YEARLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ArimaForecastingMetricsSeasonalPeriodsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArimaForecastingMetricsSeasonalPeriodsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArimaForecastingMetricsSeasonalPeriodsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => ArimaForecastingMetricsSeasonalPeriodsItems::Daily,
                "MONTHLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Monthly,
                "NO_SEASONALITY" => ArimaForecastingMetricsSeasonalPeriodsItems::NoSeasonality,
                "QUARTERLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Quarterly,
                "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => {
                    ArimaForecastingMetricsSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified
                }
                "WEEKLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Weekly,
                "YEARLY" => ArimaForecastingMetricsSeasonalPeriodsItems::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ArimaForecastingMetricsSeasonalPeriodsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaForecastingMetricsSeasonalPeriodsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArimaModelInfo {
        #[doc = "Arima coefficients."]
        #[serde(
            rename = "arimaCoefficients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_coefficients: ::std::option::Option<crate::schemas::ArimaCoefficients>,
        #[doc = "Arima fitting metrics."]
        #[serde(
            rename = "arimaFittingMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_fitting_metrics: ::std::option::Option<crate::schemas::ArimaFittingMetrics>,
        #[doc = "Whether Arima model fitted with drift or not. It is always false when d is not 1."]
        #[serde(
            rename = "hasDrift",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_drift: ::std::option::Option<bool>,
        #[doc = "Non-seasonal order."]
        #[serde(
            rename = "nonSeasonalOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_seasonal_order: ::std::option::Option<crate::schemas::ArimaOrder>,
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        #[serde(
            rename = "seasonalPeriods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seasonal_periods:
            ::std::option::Option<Vec<crate::schemas::ArimaModelInfoSeasonalPeriodsItems>>,
        #[doc = "The time_series_id value for this time series. It will be one of the unique values from the time_series_id_column specified during ARIMA model training. Only present when time_series_id_column training option was used."]
        #[serde(
            rename = "timeSeriesId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ArimaModelInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaModelInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArimaModelInfoSeasonalPeriodsItems {
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[doc = "No seasonality"]
        NoSeasonality,
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        SeasonalPeriodTypeUnspecified,
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ArimaModelInfoSeasonalPeriodsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ArimaModelInfoSeasonalPeriodsItems::Daily => "DAILY",
                ArimaModelInfoSeasonalPeriodsItems::Monthly => "MONTHLY",
                ArimaModelInfoSeasonalPeriodsItems::NoSeasonality => "NO_SEASONALITY",
                ArimaModelInfoSeasonalPeriodsItems::Quarterly => "QUARTERLY",
                ArimaModelInfoSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified => {
                    "SEASONAL_PERIOD_TYPE_UNSPECIFIED"
                }
                ArimaModelInfoSeasonalPeriodsItems::Weekly => "WEEKLY",
                ArimaModelInfoSeasonalPeriodsItems::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ArimaModelInfoSeasonalPeriodsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ArimaModelInfoSeasonalPeriodsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ArimaModelInfoSeasonalPeriodsItems, ()> {
            Ok(match s {
                "DAILY" => ArimaModelInfoSeasonalPeriodsItems::Daily,
                "MONTHLY" => ArimaModelInfoSeasonalPeriodsItems::Monthly,
                "NO_SEASONALITY" => ArimaModelInfoSeasonalPeriodsItems::NoSeasonality,
                "QUARTERLY" => ArimaModelInfoSeasonalPeriodsItems::Quarterly,
                "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => {
                    ArimaModelInfoSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified
                }
                "WEEKLY" => ArimaModelInfoSeasonalPeriodsItems::Weekly,
                "YEARLY" => ArimaModelInfoSeasonalPeriodsItems::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ArimaModelInfoSeasonalPeriodsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArimaModelInfoSeasonalPeriodsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArimaModelInfoSeasonalPeriodsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => ArimaModelInfoSeasonalPeriodsItems::Daily,
                "MONTHLY" => ArimaModelInfoSeasonalPeriodsItems::Monthly,
                "NO_SEASONALITY" => ArimaModelInfoSeasonalPeriodsItems::NoSeasonality,
                "QUARTERLY" => ArimaModelInfoSeasonalPeriodsItems::Quarterly,
                "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => {
                    ArimaModelInfoSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified
                }
                "WEEKLY" => ArimaModelInfoSeasonalPeriodsItems::Weekly,
                "YEARLY" => ArimaModelInfoSeasonalPeriodsItems::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ArimaModelInfoSeasonalPeriodsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaModelInfoSeasonalPeriodsItems {
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
    pub struct ArimaOrder {
        #[doc = "Order of the differencing part."]
        #[serde(
            rename = "d",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub d: ::std::option::Option<i64>,
        #[doc = "Order of the autoregressive part."]
        #[serde(
            rename = "p",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub p: ::std::option::Option<i64>,
        #[doc = "Order of the moving-average part."]
        #[serde(
            rename = "q",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub q: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ArimaOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaOrder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArimaResult {
        #[doc = "This message is repeated because there are multiple arima models fitted in auto-arima. For non-auto-arima model, its size is one."]
        #[serde(
            rename = "arimaModelInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_model_info: ::std::option::Option<Vec<crate::schemas::ArimaModelInfo>>,
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        #[serde(
            rename = "seasonalPeriods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seasonal_periods:
            ::std::option::Option<Vec<crate::schemas::ArimaResultSeasonalPeriodsItems>>,
    }
    impl ::google_field_selector::FieldSelector for ArimaResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArimaResultSeasonalPeriodsItems {
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[doc = "No seasonality"]
        NoSeasonality,
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        SeasonalPeriodTypeUnspecified,
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ArimaResultSeasonalPeriodsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ArimaResultSeasonalPeriodsItems::Daily => "DAILY",
                ArimaResultSeasonalPeriodsItems::Monthly => "MONTHLY",
                ArimaResultSeasonalPeriodsItems::NoSeasonality => "NO_SEASONALITY",
                ArimaResultSeasonalPeriodsItems::Quarterly => "QUARTERLY",
                ArimaResultSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified => {
                    "SEASONAL_PERIOD_TYPE_UNSPECIFIED"
                }
                ArimaResultSeasonalPeriodsItems::Weekly => "WEEKLY",
                ArimaResultSeasonalPeriodsItems::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ArimaResultSeasonalPeriodsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ArimaResultSeasonalPeriodsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ArimaResultSeasonalPeriodsItems, ()> {
            Ok(match s {
                "DAILY" => ArimaResultSeasonalPeriodsItems::Daily,
                "MONTHLY" => ArimaResultSeasonalPeriodsItems::Monthly,
                "NO_SEASONALITY" => ArimaResultSeasonalPeriodsItems::NoSeasonality,
                "QUARTERLY" => ArimaResultSeasonalPeriodsItems::Quarterly,
                "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => {
                    ArimaResultSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified
                }
                "WEEKLY" => ArimaResultSeasonalPeriodsItems::Weekly,
                "YEARLY" => ArimaResultSeasonalPeriodsItems::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ArimaResultSeasonalPeriodsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArimaResultSeasonalPeriodsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArimaResultSeasonalPeriodsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => ArimaResultSeasonalPeriodsItems::Daily,
                "MONTHLY" => ArimaResultSeasonalPeriodsItems::Monthly,
                "NO_SEASONALITY" => ArimaResultSeasonalPeriodsItems::NoSeasonality,
                "QUARTERLY" => ArimaResultSeasonalPeriodsItems::Quarterly,
                "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => {
                    ArimaResultSeasonalPeriodsItems::SeasonalPeriodTypeUnspecified
                }
                "WEEKLY" => ArimaResultSeasonalPeriodsItems::Weekly,
                "YEARLY" => ArimaResultSeasonalPeriodsItems::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ArimaResultSeasonalPeriodsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaResultSeasonalPeriodsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArimaSingleModelForecastingMetrics {
        #[doc = "Arima fitting metrics."]
        #[serde(
            rename = "arimaFittingMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_fitting_metrics: ::std::option::Option<crate::schemas::ArimaFittingMetrics>,
        #[doc = "Is arima model fitted with drift or not. It is always false when d is not 1."]
        #[serde(
            rename = "hasDrift",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_drift: ::std::option::Option<bool>,
        #[doc = "Non-seasonal order."]
        #[serde(
            rename = "nonSeasonalOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_seasonal_order: ::std::option::Option<crate::schemas::ArimaOrder>,
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        #[serde(
            rename = "seasonalPeriods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seasonal_periods: ::std::option::Option<
            Vec<crate::schemas::ArimaSingleModelForecastingMetricsSeasonalPeriodsItems>,
        >,
        #[doc = "The time_series_id value for this time series. It will be one of the unique values from the time_series_id_column specified during ARIMA model training. Only present when time_series_id_column training option was used."]
        #[serde(
            rename = "timeSeriesId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ArimaSingleModelForecastingMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArimaSingleModelForecastingMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[doc = "No seasonality"]
        NoSeasonality,
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        SeasonalPeriodTypeUnspecified,
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        pub fn as_str(self) -> &'static str {
            match self { ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Daily => "DAILY" , ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Monthly => "MONTHLY" , ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: NoSeasonality => "NO_SEASONALITY" , ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Quarterly => "QUARTERLY" , ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: SeasonalPeriodTypeUnspecified => "SEASONAL_PERIOD_TYPE_UNSPECIFIED" , ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Weekly => "WEEKLY" , ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Yearly => "YEARLY" , }
        }
    }
    impl ::std::convert::AsRef<str> for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ArimaSingleModelForecastingMetricsSeasonalPeriodsItems, ()>
        {
            Ok ( match s { "DAILY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Daily , "MONTHLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Monthly , "NO_SEASONALITY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: NoSeasonality , "QUARTERLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Quarterly , "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: SeasonalPeriodTypeUnspecified , "WEEKLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Weekly , "YEARLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Yearly , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "DAILY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Daily , "MONTHLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Monthly , "NO_SEASONALITY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: NoSeasonality , "QUARTERLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Quarterly , "SEASONAL_PERIOD_TYPE_UNSPECIFIED" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: SeasonalPeriodTypeUnspecified , "WEEKLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Weekly , "YEARLY" => ArimaSingleModelForecastingMetricsSeasonalPeriodsItems :: Yearly , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ArimaSingleModelForecastingMetricsSeasonalPeriodsItems
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
    pub struct AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs: ::std::option::Option<Vec<crate::schemas::AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditConfig {
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
    pub struct AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        #[serde(
            rename = "exemptedMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<crate::schemas::AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                AuditLogConfigLogType::DataRead => "DATA_READ",
                AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfigLogType {
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
    pub struct BigQueryModelTraining {
        #[doc = "[Output-only, Beta] Index of current ML training iteration. Updated during create model query job to show job progress."]
        #[serde(
            rename = "currentIteration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_iteration: ::std::option::Option<i32>,
        #[doc = "[Output-only, Beta] Expected number of iterations for the create model query job specified as num_iterations in the input query. The actual total number of iterations may be less than this number due to early stop."]
        #[serde(
            rename = "expectedTotalIterations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expected_total_iterations: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for BigQueryModelTraining {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BigQueryModelTraining {
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
    pub struct BigtableColumn {
        #[doc = "[Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels."]
        #[serde(
            rename = "encoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encoding: ::std::option::Option<String>,
        #[doc = "[Optional] If the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries."]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name: ::std::option::Option<String>,
        #[doc = "[Optional] If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels."]
        #[serde(
            rename = "onlyReadLatest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub only_read_latest: ::std::option::Option<bool>,
        #[doc = "[Required] Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifier_string field. Otherwise, a base-64 encoded value must be set to qualifier_encoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as field_name."]
        #[serde(
            rename = "qualifierEncoded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qualifier_encoded: ::std::option::Option<::google_api_bytes::Bytes>,
        #[serde(
            rename = "qualifierString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qualifier_string: ::std::option::Option<String>,
        #[doc = "[Optional] The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BigtableColumn {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BigtableColumn {
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
    pub struct BigtableColumnFamily {
        #[doc = "[Optional] Lists of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as .. Other columns can be accessed as a list through .Column field."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<Vec<crate::schemas::BigtableColumn>>,
        #[doc = "[Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it."]
        #[serde(
            rename = "encoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encoding: ::std::option::Option<String>,
        #[doc = "Identifier of the column family."]
        #[serde(
            rename = "familyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub family_id: ::std::option::Option<String>,
        #[doc = "[Optional] If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column."]
        #[serde(
            rename = "onlyReadLatest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub only_read_latest: ::std::option::Option<bool>,
        #[doc = "[Optional] The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BigtableColumnFamily {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BigtableColumnFamily {
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
    pub struct BigtableOptions {
        #[doc = "[Optional] List of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable."]
        #[serde(
            rename = "columnFamilies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_families: ::std::option::Option<Vec<crate::schemas::BigtableColumnFamily>>,
        #[doc = "[Optional] If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false."]
        #[serde(
            rename = "ignoreUnspecifiedColumnFamilies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ignore_unspecified_column_families: ::std::option::Option<bool>,
        #[doc = "[Optional] If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false."]
        #[serde(
            rename = "readRowkeyAsString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_rowkey_as_string: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for BigtableOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BigtableOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BinaryClassificationMetrics {
        #[doc = "Aggregate classification metrics."]
        #[serde(
            rename = "aggregateClassificationMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregate_classification_metrics:
            ::std::option::Option<crate::schemas::AggregateClassificationMetrics>,
        #[doc = "Binary confusion matrix at multiple thresholds."]
        #[serde(
            rename = "binaryConfusionMatrixList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binary_confusion_matrix_list:
            ::std::option::Option<Vec<crate::schemas::BinaryConfusionMatrix>>,
        #[doc = "Label representing the negative class."]
        #[serde(
            rename = "negativeLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negative_label: ::std::option::Option<String>,
        #[doc = "Label representing the positive class."]
        #[serde(
            rename = "positiveLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positive_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BinaryClassificationMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BinaryClassificationMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BinaryConfusionMatrix {
        #[doc = "The fraction of predictions given the correct label."]
        #[serde(
            rename = "accuracy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accuracy: ::std::option::Option<f64>,
        #[doc = "The equally weighted average of recall and precision."]
        #[serde(
            rename = "f1Score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub f_1_score: ::std::option::Option<f64>,
        #[doc = "Number of false samples predicted as false."]
        #[serde(
            rename = "falseNegatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub false_negatives: ::std::option::Option<i64>,
        #[doc = "Number of false samples predicted as true."]
        #[serde(
            rename = "falsePositives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub false_positives: ::std::option::Option<i64>,
        #[doc = "Threshold value used when computing each of the following metric."]
        #[serde(
            rename = "positiveClassThreshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positive_class_threshold: ::std::option::Option<f64>,
        #[doc = "The fraction of actual positive predictions that had positive actual labels."]
        #[serde(
            rename = "precision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub precision: ::std::option::Option<f64>,
        #[doc = "The fraction of actual positive labels that were given a positive prediction."]
        #[serde(
            rename = "recall",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recall: ::std::option::Option<f64>,
        #[doc = "Number of true samples predicted as false."]
        #[serde(
            rename = "trueNegatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub true_negatives: ::std::option::Option<i64>,
        #[doc = "Number of true samples predicted as true."]
        #[serde(
            rename = "truePositives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub true_positives: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for BinaryConfusionMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BinaryConfusionMatrix {
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
    pub struct Binding {
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Binding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BqmlIterationResult {
        #[doc = "[Output-only, Beta] Time taken to run the training iteration in milliseconds."]
        #[serde(
            rename = "durationMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub duration_ms: ::std::option::Option<i64>,
        #[doc = "[Output-only, Beta] Eval loss computed on the eval data at the end of the iteration. The eval loss is used for early stopping to avoid overfitting. No eval loss if eval_split_method option is specified as no_split or auto_split with input data size less than 500 rows."]
        #[serde(
            rename = "evalLoss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eval_loss: ::std::option::Option<f64>,
        #[doc = "[Output-only, Beta] Index of the ML training iteration, starting from zero for each training run."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "[Output-only, Beta] Learning rate used for this iteration, it varies for different training iterations if learn_rate_strategy option is not constant."]
        #[serde(
            rename = "learnRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_rate: ::std::option::Option<f64>,
        #[doc = "[Output-only, Beta] Training loss computed on the training data at the end of the iteration. The training loss function is defined by model type."]
        #[serde(
            rename = "trainingLoss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_loss: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for BqmlIterationResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BqmlIterationResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BqmlTrainingRun {
        #[doc = "[Output-only, Beta] List of each iteration results."]
        #[serde(
            rename = "iterationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iteration_results: ::std::option::Option<Vec<crate::schemas::BqmlIterationResult>>,
        #[doc = "[Output-only, Beta] Training run start time in milliseconds since the epoch."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "[Output-only, Beta] Different state applicable for a training run. IN PROGRESS: Training run is in progress. FAILED: Training run ended due to a non-retryable failure. SUCCEEDED: Training run successfully completed. CANCELLED: Training run cancelled by the user."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "[Output-only, Beta] Training options used by this training run. These options are mutable for subsequent training runs. Default values are explicitly stored for options not specified in the input query of the first training run. For subsequent training runs, any option not explicitly specified in the input query will be copied from the previous training run."]
        #[serde(
            rename = "trainingOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_options: ::std::option::Option<crate::schemas::BqmlTrainingRunTrainingOptions>,
    }
    impl ::google_field_selector::FieldSelector for BqmlTrainingRun {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BqmlTrainingRun {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BqmlTrainingRunTrainingOptions {
        #[serde(
            rename = "earlyStop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub early_stop: ::std::option::Option<bool>,
        #[serde(
            rename = "l1Reg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub l_1_reg: ::std::option::Option<f64>,
        #[serde(
            rename = "l2Reg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub l_2_reg: ::std::option::Option<f64>,
        #[serde(
            rename = "learnRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_rate: ::std::option::Option<f64>,
        #[serde(
            rename = "learnRateStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_rate_strategy: ::std::option::Option<String>,
        #[serde(
            rename = "lineSearchInitLearnRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_search_init_learn_rate: ::std::option::Option<f64>,
        #[serde(
            rename = "maxIteration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_iteration: ::std::option::Option<i64>,
        #[serde(
            rename = "minRelProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_rel_progress: ::std::option::Option<f64>,
        #[serde(
            rename = "warmStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warm_start: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for BqmlTrainingRunTrainingOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BqmlTrainingRunTrainingOptions {
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
    pub struct CategoricalValue {
        #[doc = "Counts of all categories for the categorical feature. If there are more than ten categories, we return top ten (by count) and return one more CategoryCount with category \"*OTHER*\" and count as aggregate counts of remaining categories."]
        #[serde(
            rename = "categoryCounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_counts: ::std::option::Option<Vec<crate::schemas::CategoryCount>>,
    }
    impl ::google_field_selector::FieldSelector for CategoricalValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CategoricalValue {
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
    pub struct CategoryCount {
        #[doc = "The name of category."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "The count of training samples matching the category within the cluster."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for CategoryCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CategoryCount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cluster {
        #[doc = "Centroid id."]
        #[serde(
            rename = "centroidId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub centroid_id: ::std::option::Option<i64>,
        #[doc = "Count of training data rows that were assigned to this cluster."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Values of highly variant features for this cluster."]
        #[serde(
            rename = "featureValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_values: ::std::option::Option<Vec<crate::schemas::FeatureValue>>,
    }
    impl ::google_field_selector::FieldSelector for Cluster {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cluster {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClusterInfo {
        #[doc = "Centroid id."]
        #[serde(
            rename = "centroidId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub centroid_id: ::std::option::Option<i64>,
        #[doc = "Cluster radius, the average distance from centroid to each point assigned to the cluster."]
        #[serde(
            rename = "clusterRadius",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_radius: ::std::option::Option<f64>,
        #[doc = "Cluster size, the total number of points assigned to the cluster."]
        #[serde(
            rename = "clusterSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cluster_size: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ClusterInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClusterInfo {
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
    pub struct Clustering {
        #[doc = "[Repeated] One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. When you cluster a table using multiple columns, the order of columns you specify is important. The order of the specified columns determines the sort order of the data."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Clustering {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Clustering {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClusteringMetrics {
        #[doc = "Information for all clusters."]
        #[serde(
            rename = "clusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clusters: ::std::option::Option<Vec<crate::schemas::Cluster>>,
        #[doc = "Davies-Bouldin index."]
        #[serde(
            rename = "daviesBouldinIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub davies_bouldin_index: ::std::option::Option<f64>,
        #[doc = "Mean of squared distances between each sample to its cluster centroid."]
        #[serde(
            rename = "meanSquaredDistance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mean_squared_distance: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for ClusteringMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClusteringMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ConfusionMatrix {
        #[doc = "Confidence threshold used when computing the entries of the confusion matrix."]
        #[serde(
            rename = "confidenceThreshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence_threshold: ::std::option::Option<f64>,
        #[doc = "One row per actual label."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
    }
    impl ::google_field_selector::FieldSelector for ConfusionMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfusionMatrix {
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
    pub struct ConnectionProperty {
        #[doc = "[Required] Name of the connection property to set."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "[Required] Value of the connection property."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConnectionProperty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConnectionProperty {
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
    pub struct CsvOptions {
        #[doc = "[Optional] Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
        #[serde(
            rename = "allowJaggedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_jagged_rows: ::std::option::Option<bool>,
        #[doc = "[Optional] Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
        #[serde(
            rename = "allowQuotedNewlines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_quoted_newlines: ::std::option::Option<bool>,
        #[doc = "[Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties."]
        #[serde(
            rename = "encoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encoding: ::std::option::Option<String>,
        #[doc = "[Optional] The separator for fields in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',')."]
        #[serde(
            rename = "fieldDelimiter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_delimiter: ::std::option::Option<String>,
        #[doc = "[Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
        #[serde(
            rename = "quote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quote: ::std::option::Option<String>,
        #[doc = "[Optional] The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped. When autodetect is on, the behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
        #[serde(
            rename = "skipLeadingRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub skip_leading_rows: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for CsvOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CsvOptions {
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
    pub struct DataSplitResult {
        #[doc = "Table reference of the evaluation data after split."]
        #[serde(
            rename = "evaluationTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluation_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "Table reference of the training data after split."]
        #[serde(
            rename = "trainingTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_table: ::std::option::Option<crate::schemas::TableReference>,
    }
    impl ::google_field_selector::FieldSelector for DataSplitResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSplitResult {
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
    pub struct Dataset {
        #[doc = "[Optional] An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER;"]
        #[serde(
            rename = "access",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access: ::std::option::Option<Vec<crate::schemas::DatasetAccessItems>>,
        #[doc = "[Output-only] The time when this dataset was created, in milliseconds since the epoch."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: ::std::option::Option<i64>,
        #[doc = "[Required] A reference that identifies the dataset."]
        #[serde(
            rename = "datasetReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_reference: ::std::option::Option<crate::schemas::DatasetReference>,
        #[serde(
            rename = "defaultEncryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_encryption_configuration:
            ::std::option::Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Optional] The default partition expiration for all partitioned tables in the dataset, in milliseconds. Once this property is set, all newly-created partitioned tables in the dataset will have an expirationMs property in the timePartitioning settings set to this value, and changing the value will only affect new tables, not existing ones. The storage in a partition will have an expiration time of its partition time plus this value. Setting this property overrides the use of defaultTableExpirationMs for partitioned tables: only one of defaultTableExpirationMs and defaultPartitionExpirationMs will be used for any new partitioned table. If you provide an explicit timePartitioning.expirationMs when creating or updating a partitioned table, that value takes precedence over the default partition expiration time indicated by this property."]
        #[serde(
            rename = "defaultPartitionExpirationMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub default_partition_expiration_ms: ::std::option::Option<i64>,
        #[doc = "[Optional] The default lifetime of all tables in the dataset, in milliseconds. The minimum value is 3600000 milliseconds (one hour). Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property."]
        #[serde(
            rename = "defaultTableExpirationMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub default_table_expiration_ms: ::std::option::Option<i64>,
        #[doc = "[Optional] A user-friendly description of the dataset."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "[Output-only] A hash of the resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "[Optional] A descriptive name for the dataset."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "[Output-only] The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "[Output-only] The resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See Creating and Updating Dataset Labels for more information."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "[Output-only] The date when this dataset or any of its tables was last modified, in milliseconds since the epoch."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: ::std::option::Option<i64>,
        #[doc = "The geographic location where the dataset should reside. The default value is US. See details at https://cloud.google.com/bigquery/docs/locations."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "[Output-only] Reserved for future use."]
        #[serde(
            rename = "satisfiesPZS",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub satisfies_pzs: ::std::option::Option<bool>,
        #[doc = "[Output-only] A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Dataset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dataset {
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
    pub struct DatasetAccessItems {
        #[doc = "[Pick one] A grant authorizing all resources of a particular type in a particular dataset access to this dataset. Only views are supported for now. The role field is not required when this field is set. If that dataset is deleted and re-created, its access needs to be granted again via an update operation."]
        #[serde(
            rename = "dataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset: ::std::option::Option<crate::schemas::DatasetAccessEntry>,
        #[doc = "[Pick one] A domain to grant access to. Any users signed in with the domain specified will be granted the specified access. Example: \"example.com\". Maps to IAM policy member \"domain:DOMAIN\"."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "[Pick one] An email address of a Google Group to grant access to. Maps to IAM policy member \"group:GROUP\"."]
        #[serde(
            rename = "groupByEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_by_email: ::std::option::Option<String>,
        #[doc = "[Pick one] Some other type of member that appears in the IAM Policy but isn't a user, group, domain, or special group."]
        #[serde(
            rename = "iamMember",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_member: ::std::option::Option<String>,
        #[doc = "[Required] An IAM role ID that should be granted to the user, group, or domain specified in this access entry. The following legacy mappings will be applied: OWNER  roles/bigquery.dataOwner WRITER  roles/bigquery.dataEditor READER  roles/bigquery.dataViewer This field will accept any of the above formats, but will return only the legacy format. For example, if you set this field to \"roles/bigquery.dataOwner\", it will be returned back as \"OWNER\"."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
        #[doc = "[Pick one] A routine from a different dataset to grant access to. Queries executed against that routine will have read access to views/tables/routines in this dataset. Only UDF is supported for now. The role field is not required when this field is set. If that routine is updated by any user, access to the routine needs to be granted again via an update operation."]
        #[serde(
            rename = "routine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub routine: ::std::option::Option<crate::schemas::RoutineReference>,
        #[doc = "[Pick one] A special group to grant access to. Possible values include: projectOwners: Owners of the enclosing project. projectReaders: Readers of the enclosing project. projectWriters: Writers of the enclosing project. allAuthenticatedUsers: All authenticated BigQuery users. Maps to similarly-named IAM members."]
        #[serde(
            rename = "specialGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub special_group: ::std::option::Option<String>,
        #[doc = "[Pick one] An email address of a user to grant access to. For example: fred@example.com. Maps to IAM policy member \"user:EMAIL\" or \"serviceAccount:EMAIL\"."]
        #[serde(
            rename = "userByEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_by_email: ::std::option::Option<String>,
        #[doc = "[Pick one] A view from a different dataset to grant access to. Queries executed against that view will have read access to tables in this dataset. The role field is not required when this field is set. If that view is updated by any user, access to the view needs to be granted again via an update operation."]
        #[serde(
            rename = "view",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view: ::std::option::Option<crate::schemas::TableReference>,
    }
    impl ::google_field_selector::FieldSelector for DatasetAccessItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatasetAccessItems {
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
    pub struct DatasetAccessEntry {
        #[doc = "[Required] The dataset this entry applies to."]
        #[serde(
            rename = "dataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset: ::std::option::Option<crate::schemas::DatasetReference>,
        #[serde(
            rename = "target_types",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_types:
            ::std::option::Option<Vec<crate::schemas::DatasetAccessEntryTargetTypesItems>>,
    }
    impl ::google_field_selector::FieldSelector for DatasetAccessEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatasetAccessEntry {
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
    pub struct DatasetAccessEntryTargetTypesItems {
        #[doc = "[Required] Which resources in the dataset this entry applies to. Currently, only views are supported, but additional target types may be added in the future. Possible values: VIEWS: This entry applies to all views in the dataset."]
        #[serde(
            rename = "targetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatasetAccessEntryTargetTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatasetAccessEntryTargetTypesItems {
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
    pub struct DatasetList {
        #[doc = "An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project."]
        #[serde(
            rename = "datasets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datasets: ::std::option::Option<Vec<crate::schemas::DatasetListDatasetsItems>>,
        #[doc = "A hash value of the results page. You can use this property to determine if the page has changed since the last request."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The list type. This property always returns the value \"bigquery#datasetList\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A token that can be used to request the next results page. This property is omitted on the final results page."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatasetList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatasetList {
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
    pub struct DatasetListDatasetsItems {
        #[doc = "The dataset reference. Use this property to access specific parts of the dataset's ID, such as project ID or dataset ID."]
        #[serde(
            rename = "datasetReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_reference: ::std::option::Option<crate::schemas::DatasetReference>,
        #[doc = "A descriptive name for the dataset, if one exists."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "The fully-qualified, unique, opaque ID of the dataset."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The resource type. This property always returns the value \"bigquery#dataset\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The labels associated with this dataset. You can use these to organize and group your datasets."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The geographic location where the data resides."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatasetListDatasetsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatasetListDatasetsItems {
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
    pub struct DatasetReference {
        #[doc = "[Required] A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "[Optional] The ID of the project containing this dataset."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatasetReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatasetReference {
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
    pub struct DestinationTableProperties {
        #[doc = "[Optional] The description for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current description is provided, the job will fail."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "[Optional] The friendly name for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current friendly name is provided, the job will fail."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "[Optional] The labels associated with this table. You can use these to organize and group your tables. This will only be used if the destination table is newly created. If the table already exists and labels are different than the current labels are provided, the job will fail."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for DestinationTableProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationTableProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DimensionalityReductionMetrics {
        #[doc = "Total percentage of variance explained by the selected principal components."]
        #[serde(
            rename = "totalExplainedVarianceRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_explained_variance_ratio: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for DimensionalityReductionMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionalityReductionMetrics {
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
    pub struct EncryptionConfiguration {
        #[doc = "[Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key."]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EncryptionConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncryptionConfiguration {
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
    pub struct Entry {
        #[doc = "Number of items being predicted as this label."]
        #[serde(
            rename = "itemCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub item_count: ::std::option::Option<i64>,
        #[doc = "The predicted label. For confidence_threshold > 0, we will also add an entry indicating the number of items under the confidence threshold."]
        #[serde(
            rename = "predictedLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predicted_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Entry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Entry {
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
    pub struct ErrorProto {
        #[doc = "Debugging information. This property is internal to Google and should not be used."]
        #[serde(
            rename = "debugInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_info: ::std::option::Option<String>,
        #[doc = "Specifies where the error occurred, if present."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "A human-readable description of the error."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "A short error code that summarizes the error."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ErrorProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorProto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EvaluationMetrics {
        #[doc = "Populated for ARIMA models."]
        #[serde(
            rename = "arimaForecastingMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_forecasting_metrics:
            ::std::option::Option<crate::schemas::ArimaForecastingMetrics>,
        #[doc = "Populated for binary classification/classifier models."]
        #[serde(
            rename = "binaryClassificationMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binary_classification_metrics:
            ::std::option::Option<crate::schemas::BinaryClassificationMetrics>,
        #[doc = "Populated for clustering models."]
        #[serde(
            rename = "clusteringMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clustering_metrics: ::std::option::Option<crate::schemas::ClusteringMetrics>,
        #[doc = "Evaluation metrics when the model is a dimensionality reduction model, which currently includes PCA."]
        #[serde(
            rename = "dimensionalityReductionMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensionality_reduction_metrics:
            ::std::option::Option<crate::schemas::DimensionalityReductionMetrics>,
        #[doc = "Populated for multi-class classification/classifier models."]
        #[serde(
            rename = "multiClassClassificationMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_class_classification_metrics:
            ::std::option::Option<crate::schemas::MultiClassClassificationMetrics>,
        #[doc = "Populated for implicit feedback type matrix factorization models."]
        #[serde(
            rename = "rankingMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ranking_metrics: ::std::option::Option<crate::schemas::RankingMetrics>,
        #[doc = "Populated for regression models and explicit feedback type matrix factorization models."]
        #[serde(
            rename = "regressionMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regression_metrics: ::std::option::Option<crate::schemas::RegressionMetrics>,
    }
    impl ::google_field_selector::FieldSelector for EvaluationMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EvaluationMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ExplainQueryStage {
        #[doc = "Number of parallel input segments completed."]
        #[serde(
            rename = "completedParallelInputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub completed_parallel_inputs: ::std::option::Option<i64>,
        #[doc = "Milliseconds the average shard spent on CPU-bound tasks."]
        #[serde(
            rename = "computeMsAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub compute_ms_avg: ::std::option::Option<i64>,
        #[doc = "Milliseconds the slowest shard spent on CPU-bound tasks."]
        #[serde(
            rename = "computeMsMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub compute_ms_max: ::std::option::Option<i64>,
        #[doc = "Relative amount of time the average shard spent on CPU-bound tasks."]
        #[serde(
            rename = "computeRatioAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compute_ratio_avg: ::std::option::Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent on CPU-bound tasks."]
        #[serde(
            rename = "computeRatioMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compute_ratio_max: ::std::option::Option<f64>,
        #[doc = "Stage end time represented as milliseconds since epoch."]
        #[serde(
            rename = "endMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_ms: ::std::option::Option<i64>,
        #[doc = "Unique ID for stage within plan."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "IDs for stages that are inputs to this stage."]
        #[serde(
            rename = "inputStages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_stages: ::std::option::Option<Vec<i64>>,
        #[doc = "Human-readable name for stage."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Number of parallel input segments to be processed."]
        #[serde(
            rename = "parallelInputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub parallel_inputs: ::std::option::Option<i64>,
        #[doc = "Milliseconds the average shard spent reading input."]
        #[serde(
            rename = "readMsAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub read_ms_avg: ::std::option::Option<i64>,
        #[doc = "Milliseconds the slowest shard spent reading input."]
        #[serde(
            rename = "readMsMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub read_ms_max: ::std::option::Option<i64>,
        #[doc = "Relative amount of time the average shard spent reading input."]
        #[serde(
            rename = "readRatioAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_ratio_avg: ::std::option::Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent reading input."]
        #[serde(
            rename = "readRatioMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_ratio_max: ::std::option::Option<f64>,
        #[doc = "Number of records read into the stage."]
        #[serde(
            rename = "recordsRead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub records_read: ::std::option::Option<i64>,
        #[doc = "Number of records written by the stage."]
        #[serde(
            rename = "recordsWritten",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub records_written: ::std::option::Option<i64>,
        #[doc = "Total number of bytes written to shuffle."]
        #[serde(
            rename = "shuffleOutputBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shuffle_output_bytes: ::std::option::Option<i64>,
        #[doc = "Total number of bytes written to shuffle and spilled to disk."]
        #[serde(
            rename = "shuffleOutputBytesSpilled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shuffle_output_bytes_spilled: ::std::option::Option<i64>,
        #[doc = "Slot-milliseconds used by the stage."]
        #[serde(
            rename = "slotMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub slot_ms: ::std::option::Option<i64>,
        #[doc = "Stage start time represented as milliseconds since epoch."]
        #[serde(
            rename = "startMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_ms: ::std::option::Option<i64>,
        #[doc = "Current status for the stage."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<String>,
        #[doc = "List of operations within the stage in dependency order (approximately chronological)."]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<Vec<crate::schemas::ExplainQueryStep>>,
        #[doc = "Milliseconds the average shard spent waiting to be scheduled."]
        #[serde(
            rename = "waitMsAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub wait_ms_avg: ::std::option::Option<i64>,
        #[doc = "Milliseconds the slowest shard spent waiting to be scheduled."]
        #[serde(
            rename = "waitMsMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub wait_ms_max: ::std::option::Option<i64>,
        #[doc = "Relative amount of time the average shard spent waiting to be scheduled."]
        #[serde(
            rename = "waitRatioAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wait_ratio_avg: ::std::option::Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent waiting to be scheduled."]
        #[serde(
            rename = "waitRatioMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wait_ratio_max: ::std::option::Option<f64>,
        #[doc = "Milliseconds the average shard spent on writing output."]
        #[serde(
            rename = "writeMsAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub write_ms_avg: ::std::option::Option<i64>,
        #[doc = "Milliseconds the slowest shard spent on writing output."]
        #[serde(
            rename = "writeMsMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub write_ms_max: ::std::option::Option<i64>,
        #[doc = "Relative amount of time the average shard spent on writing output."]
        #[serde(
            rename = "writeRatioAvg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_ratio_avg: ::std::option::Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent on writing output."]
        #[serde(
            rename = "writeRatioMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_ratio_max: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for ExplainQueryStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExplainQueryStage {
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
    pub struct ExplainQueryStep {
        #[doc = "Machine-readable operation type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Human-readable stage descriptions."]
        #[serde(
            rename = "substeps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub substeps: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ExplainQueryStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExplainQueryStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Explanation {
        #[doc = "Attribution of feature."]
        #[serde(
            rename = "attribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribution: ::std::option::Option<f64>,
        #[doc = "Full name of the feature. For non-numerical features, will be formatted like .. Overall size of feature name will always be truncated to first 120 characters."]
        #[serde(
            rename = "featureName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Explanation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Explanation {
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
    pub struct Expr {
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Expr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Expr {
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
    pub struct ExternalDataConfiguration {
        #[doc = "Try to detect schema and format options automatically. Any option specified explicitly will be honored."]
        #[serde(
            rename = "autodetect",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub autodetect: ::std::option::Option<bool>,
        #[doc = "[Optional] Additional options if sourceFormat is set to BIGTABLE."]
        #[serde(
            rename = "bigtableOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bigtable_options: ::std::option::Option<crate::schemas::BigtableOptions>,
        #[doc = "[Optional] The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats."]
        #[serde(
            rename = "compression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compression: ::std::option::Option<String>,
        #[doc = "[Optional, Trusted Tester] Connection for external data source."]
        #[serde(
            rename = "connectionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_id: ::std::option::Option<String>,
        #[doc = "Additional properties to set if sourceFormat is set to CSV."]
        #[serde(
            rename = "csvOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub csv_options: ::std::option::Option<crate::schemas::CsvOptions>,
        #[doc = "[Optional] Additional options if sourceFormat is set to GOOGLE_SHEETS."]
        #[serde(
            rename = "googleSheetsOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_sheets_options: ::std::option::Option<crate::schemas::GoogleSheetsOptions>,
        #[doc = "[Optional] Options to configure hive partitioning support."]
        #[serde(
            rename = "hivePartitioningOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hive_partitioning_options:
            ::std::option::Option<crate::schemas::HivePartitioningOptions>,
        #[doc = "[Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored."]
        #[serde(
            rename = "ignoreUnknownValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ignore_unknown_values: ::std::option::Option<bool>,
        #[doc = "[Optional] The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV, JSON, and Google Sheets. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats."]
        #[serde(
            rename = "maxBadRecords",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_bad_records: ::std::option::Option<i32>,
        #[doc = "Additional properties to set if sourceFormat is set to Parquet."]
        #[serde(
            rename = "parquetOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parquet_options: ::std::option::Option<crate::schemas::ParquetOptions>,
        #[doc = "[Optional] The schema for the data. Schema is required for CSV and JSON formats. Schema is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "[Required] The data format. For CSV files, specify \"CSV\". For Google sheets, specify \"GOOGLE_SHEETS\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro files, specify \"AVRO\". For Google Cloud Datastore backups, specify \"DATASTORE_BACKUP\". [Beta] For Google Cloud Bigtable, specify \"BIGTABLE\"."]
        #[serde(
            rename = "sourceFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_format: ::std::option::Option<String>,
        #[doc = "[Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the '*' wildcard character is not allowed."]
        #[serde(
            rename = "sourceUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_uris: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ExternalDataConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExternalDataConfiguration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FeatureValue {
        #[doc = "The categorical feature value."]
        #[serde(
            rename = "categoricalValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categorical_value: ::std::option::Option<crate::schemas::CategoricalValue>,
        #[doc = "The feature column name."]
        #[serde(
            rename = "featureColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_column: ::std::option::Option<String>,
        #[doc = "The numerical feature value. This is the centroid value for this feature."]
        #[serde(
            rename = "numericalValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numerical_value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for FeatureValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureValue {
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
    pub struct GetIamPolicyRequest {
        #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::GetPolicyOptions>,
    }
    impl ::google_field_selector::FieldSelector for GetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIamPolicyRequest {
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
    pub struct GetPolicyOptions {
        #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "requestedPolicyVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_policy_version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GetPolicyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetPolicyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GetQueryResultsResponse {
        #[doc = "Whether the query result was fetched from the query cache."]
        #[serde(
            rename = "cacheHit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_hit: ::std::option::Option<bool>,
        #[doc = "[Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "A hash of this response."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available."]
        #[serde(
            rename = "jobComplete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_complete: ::std::option::Option<bool>,
        #[doc = "Reference to the BigQuery Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults)."]
        #[serde(
            rename = "jobReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_reference: ::std::option::Option<crate::schemas::JobReference>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        #[serde(
            rename = "numDmlAffectedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_dml_affected_rows: ::std::option::Option<i64>,
        #[doc = "A token used for paging results."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. Present only when the query completes successfully."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The schema of the results. Present only when the query completes successfully."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "The total number of bytes processed for this query."]
        #[serde(
            rename = "totalBytesProcessed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: ::std::option::Option<i64>,
        #[doc = "The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. Present only when the query completes successfully."]
        #[serde(
            rename = "totalRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for GetQueryResultsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetQueryResultsResponse {
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
    pub struct GetServiceAccountResponse {
        #[doc = "The service account email address."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetServiceAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetServiceAccountResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GlobalExplanation {
        #[doc = "Class label for this set of global explanations. Will be empty/null for binary logistic and linear regression models. Sorted alphabetically in descending order."]
        #[serde(
            rename = "classLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub class_label: ::std::option::Option<String>,
        #[doc = "A list of the top global explanations. Sorted by absolute value of attribution in descending order."]
        #[serde(
            rename = "explanations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explanations: ::std::option::Option<Vec<crate::schemas::Explanation>>,
    }
    impl ::google_field_selector::FieldSelector for GlobalExplanation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GlobalExplanation {
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
    pub struct GoogleSheetsOptions {
        #[doc = "[Optional] Range of a sheet to query from. Only used when non-empty. Typical format: sheet_name!top_left_cell_id:bottom_right_cell_id For example: sheet1!A1:B20"]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<String>,
        #[doc = "[Optional] The number of rows at the top of a sheet that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows that should be skipped. When autodetect is on, behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
        #[serde(
            rename = "skipLeadingRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub skip_leading_rows: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleSheetsOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleSheetsOptions {
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
    pub struct HivePartitioningOptions {
        #[doc = "[Optional] When set, what mode of hive partitioning to use when reading data. The following modes are supported. (1) AUTO: automatically infer partition key name(s) and type(s). (2) STRINGS: automatically infer partition key name(s). All types are interpreted as strings. (3) CUSTOM: partition key schema is encoded in the source URI prefix. Not all storage formats support hive partitioning. Requesting hive partitioning on an unsupported format will lead to an error. Currently supported types include: AVRO, CSV, JSON, ORC and Parquet."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<String>,
        #[doc = "[Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified. Note that this field should only be true when creating a permanent external table or querying a temporary external table. Hive-partitioned loads with requirePartitionFilter explicitly set to true will fail."]
        #[serde(
            rename = "requirePartitionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_partition_filter: ::std::option::Option<bool>,
        #[doc = "[Optional] When hive partition detection is requested, a common prefix for all source uris should be supplied. The prefix must end immediately before the partition key encoding begins. For example, consider files following this data layout. gs://bucket/path_to_table/dt=2019-01-01/country=BR/id=7/file.avro gs://bucket/path_to_table/dt=2018-12-31/country=CA/id=3/file.avro When hive partitioning is requested with either AUTO or STRINGS detection, the common prefix can be either of gs://bucket/path_to_table or gs://bucket/path_to_table/ (trailing slash does not matter)."]
        #[serde(
            rename = "sourceUriPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_uri_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HivePartitioningOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HivePartitioningOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IterationResult {
        #[serde(
            rename = "arimaResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arima_result: ::std::option::Option<crate::schemas::ArimaResult>,
        #[doc = "Information about top clusters for clustering models."]
        #[serde(
            rename = "clusterInfos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_infos: ::std::option::Option<Vec<crate::schemas::ClusterInfo>>,
        #[doc = "Time taken to run the iteration in milliseconds."]
        #[serde(
            rename = "durationMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub duration_ms: ::std::option::Option<i64>,
        #[doc = "Loss computed on the eval data at the end of iteration."]
        #[serde(
            rename = "evalLoss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub eval_loss: ::std::option::Option<f64>,
        #[doc = "Index of the iteration, 0 based."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "Learn rate used for this iteration."]
        #[serde(
            rename = "learnRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_rate: ::std::option::Option<f64>,
        #[doc = "The information of the principal components."]
        #[serde(
            rename = "principalComponentInfos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal_component_infos:
            ::std::option::Option<Vec<crate::schemas::PrincipalComponentInfo>>,
        #[doc = "Loss computed on the training data at the end of iteration."]
        #[serde(
            rename = "trainingLoss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_loss: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for IterationResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IterationResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Job {
        #[doc = "[Required] Describes the job configuration."]
        #[serde(
            rename = "configuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub configuration: ::std::option::Option<crate::schemas::JobConfiguration>,
        #[doc = "[Output-only] A hash of this resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "[Output-only] Opaque ID field of the job"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "[Optional] Reference describing the unique-per-user name of the job."]
        #[serde(
            rename = "jobReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_reference: ::std::option::Option<crate::schemas::JobReference>,
        #[doc = "[Output-only] The type of the resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "[Output-only] A URL that can be used to access this resource again."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "[Output-only] Information about the job, including starting time and ending time of the job."]
        #[serde(
            rename = "statistics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statistics: ::std::option::Option<crate::schemas::JobStatistics>,
        #[doc = "[Output-only] The status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::JobStatus>,
        #[doc = "[Output-only] Email address of the user who ran the job."]
        #[serde(
            rename = "user_email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct JobCancelResponse {
        #[doc = "The final state of the job."]
        #[serde(
            rename = "job",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job: ::std::option::Option<crate::schemas::Job>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobCancelResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobCancelResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct JobConfiguration {
        #[doc = "[Pick one] Copies a table."]
        #[serde(
            rename = "copy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub copy: ::std::option::Option<crate::schemas::JobConfigurationTableCopy>,
        #[doc = "[Optional] If set, don't actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn't a dry run. Behavior of non-query jobs is undefined."]
        #[serde(
            rename = "dryRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dry_run: ::std::option::Option<bool>,
        #[doc = "[Pick one] Configures an extract job."]
        #[serde(
            rename = "extract",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extract: ::std::option::Option<crate::schemas::JobConfigurationExtract>,
        #[doc = "[Optional] Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
        #[serde(
            rename = "jobTimeoutMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub job_timeout_ms: ::std::option::Option<i64>,
        #[doc = "[Output-only] The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN."]
        #[serde(
            rename = "jobType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_type: ::std::option::Option<String>,
        #[doc = "The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "[Pick one] Configures a load job."]
        #[serde(
            rename = "load",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub load: ::std::option::Option<crate::schemas::JobConfigurationLoad>,
        #[doc = "[Pick one] Configures a query job."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<crate::schemas::JobConfigurationQuery>,
    }
    impl ::google_field_selector::FieldSelector for JobConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobConfiguration {
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
    pub struct JobConfigurationExtract {
        #[doc = "[Optional] The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE. The default value is NONE. DEFLATE and SNAPPY are only supported for Avro. Not applicable when extracting models."]
        #[serde(
            rename = "compression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compression: ::std::option::Option<String>,
        #[doc = "[Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON, PARQUET or AVRO for tables and ML_TF_SAVED_MODEL or ML_XGBOOST_BOOSTER for models. The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV. The default value for models is ML_TF_SAVED_MODEL."]
        #[serde(
            rename = "destinationFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_format: ::std::option::Option<String>,
        #[doc = "[Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written."]
        #[serde(
            rename = "destinationUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_uri: ::std::option::Option<String>,
        #[doc = "[Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
        #[serde(
            rename = "destinationUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_uris: ::std::option::Option<Vec<String>>,
        #[doc = "[Optional] Delimiter to use between fields in the exported data. Default is ','. Not applicable when extracting models."]
        #[serde(
            rename = "fieldDelimiter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_delimiter: ::std::option::Option<String>,
        #[doc = "[Optional] Whether to print out a header row in the results. Default is true. Not applicable when extracting models."]
        #[serde(
            rename = "printHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub print_header: ::std::option::Option<bool>,
        #[doc = "A reference to the model being exported."]
        #[serde(
            rename = "sourceModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_model: ::std::option::Option<crate::schemas::ModelReference>,
        #[doc = "A reference to the table being exported."]
        #[serde(
            rename = "sourceTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Optional] If destinationFormat is set to \"AVRO\", this flag indicates whether to enable extracting applicable column types (such as TIMESTAMP) to their corresponding AVRO logical types (timestamp-micros), instead of only using their raw types (avro-long). Not applicable when extracting models."]
        #[serde(
            rename = "useAvroLogicalTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_avro_logical_types: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for JobConfigurationExtract {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobConfigurationExtract {
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
    pub struct JobConfigurationLoad {
        #[doc = "[Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
        #[serde(
            rename = "allowJaggedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_jagged_rows: ::std::option::Option<bool>,
        #[doc = "Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
        #[serde(
            rename = "allowQuotedNewlines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_quoted_newlines: ::std::option::Option<bool>,
        #[doc = "[Optional] Indicates if we should automatically infer the options and schema for CSV and JSON sources."]
        #[serde(
            rename = "autodetect",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub autodetect: ::std::option::Option<bool>,
        #[doc = "[Beta] Clustering specification for the destination table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered."]
        #[serde(
            rename = "clustering",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clustering: ::std::option::Option<crate::schemas::Clustering>,
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(
            rename = "createDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_disposition: ::std::option::Option<String>,
        #[doc = "Defines the list of possible SQL data types to which the source decimal values are converted. This list and the precision and the scale parameters of the decimal field determine the target type. In the order of NUMERIC, BIGNUMERIC ([Preview](/products/#product-launch-stages)), and STRING, a type is picked if it is in the specified list and if it supports the precision and the scale. STRING supports all precision and scale values. If none of the listed types supports the precision and the scale, the type supporting the widest range in the specified list is picked, and if a value exceeds the supported range when reading the data, an error will be thrown. Example: Suppose the value of this field is [\"NUMERIC\", \"BIGNUMERIC\"]. If (precision,scale) is: * (38,9) -> NUMERIC; * (39,9) -> BIGNUMERIC (NUMERIC cannot hold 30 integer digits); * (38,10) -> BIGNUMERIC (NUMERIC cannot hold 10 fractional digits); * (76,38) -> BIGNUMERIC; * (77,38) -> BIGNUMERIC (error if value exeeds supported range). This field cannot contain duplicate types. The order of the types in this field is ignored. For example, [\"BIGNUMERIC\", \"NUMERIC\"] is the same as [\"NUMERIC\", \"BIGNUMERIC\"] and NUMERIC always takes precedence over BIGNUMERIC. Defaults to [\"NUMERIC\", \"STRING\"] for ORC and [\"NUMERIC\"] for the other file formats."]
        #[serde(
            rename = "decimalTargetTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub decimal_target_types: ::std::option::Option<Vec<String>>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(
            rename = "destinationEncryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_encryption_configuration:
            ::std::option::Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Required] The destination table to load the data into."]
        #[serde(
            rename = "destinationTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Beta] [Optional] Properties with which to create the destination table if it is new."]
        #[serde(
            rename = "destinationTableProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_table_properties:
            ::std::option::Option<crate::schemas::DestinationTableProperties>,
        #[doc = "[Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties."]
        #[serde(
            rename = "encoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encoding: ::std::option::Option<String>,
        #[doc = "[Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',')."]
        #[serde(
            rename = "fieldDelimiter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_delimiter: ::std::option::Option<String>,
        #[doc = "[Optional] Options to configure hive partitioning support."]
        #[serde(
            rename = "hivePartitioningOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hive_partitioning_options:
            ::std::option::Option<crate::schemas::HivePartitioningOptions>,
        #[doc = "[Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names"]
        #[serde(
            rename = "ignoreUnknownValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ignore_unknown_values: ::std::option::Option<bool>,
        #[doc = "[Optional] If sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON. For a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited GeoJSON: set to GEOJSON."]
        #[serde(
            rename = "jsonExtension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_extension: ::std::option::Option<String>,
        #[doc = "[Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV and JSON. The default value is 0, which requires that all records are valid."]
        #[serde(
            rename = "maxBadRecords",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_bad_records: ::std::option::Option<i32>,
        #[doc = "[Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value."]
        #[serde(
            rename = "nullMarker",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub null_marker: ::std::option::Option<String>,
        #[doc = "[Optional] Options to configure parquet support."]
        #[serde(
            rename = "parquetOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parquet_options: ::std::option::Option<crate::schemas::ParquetOptions>,
        #[doc = "If sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
        #[serde(
            rename = "projectionFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub projection_fields: ::std::option::Option<Vec<String>>,
        #[doc = "[Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
        #[serde(
            rename = "quote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quote: ::std::option::Option<String>,
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(
            rename = "rangePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range_partitioning: ::std::option::Option<crate::schemas::RangePartitioning>,
        #[doc = "[Optional] The schema for the destination table. The schema can be omitted if the destination table already exists, or if you're loading data from Google Cloud Datastore."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "[Deprecated] The inline schema. For CSV schemas, specify as \"Field1:Type1[,Field2:Type2]*\". For example, \"foo:STRING, bar:INTEGER, baz:FLOAT\"."]
        #[serde(
            rename = "schemaInline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_inline: ::std::option::Option<String>,
        #[doc = "[Deprecated] The format of the schemaInline property."]
        #[serde(
            rename = "schemaInlineFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_inline_format: ::std::option::Option<String>,
        #[doc = "Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
        #[serde(
            rename = "schemaUpdateOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_update_options: ::std::option::Option<Vec<String>>,
        #[doc = "[Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped."]
        #[serde(
            rename = "skipLeadingRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_leading_rows: ::std::option::Option<i32>,
        #[doc = "[Optional] The format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\". For orc, specify \"ORC\". The default value is CSV."]
        #[serde(
            rename = "sourceFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_format: ::std::option::Option<String>,
        #[doc = "[Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed."]
        #[serde(
            rename = "sourceUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(
            rename = "timePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_partitioning: ::std::option::Option<crate::schemas::TimePartitioning>,
        #[doc = "[Optional] If sourceFormat is set to \"AVRO\", indicates whether to enable interpreting logical types into their corresponding types (ie. TIMESTAMP), instead of only using their raw types (ie. INTEGER)."]
        #[serde(
            rename = "useAvroLogicalTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_avro_logical_types: ::std::option::Option<bool>,
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(
            rename = "writeDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_disposition: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobConfigurationLoad {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobConfigurationLoad {
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
    pub struct JobConfigurationQuery {
        #[doc = "[Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size."]
        #[serde(
            rename = "allowLargeResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_large_results: ::std::option::Option<bool>,
        #[doc = "[Beta] Clustering specification for the destination table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered."]
        #[serde(
            rename = "clustering",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clustering: ::std::option::Option<crate::schemas::Clustering>,
        #[doc = "Connection properties."]
        #[serde(
            rename = "connectionProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_properties: ::std::option::Option<Vec<crate::schemas::ConnectionProperty>>,
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(
            rename = "createDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_disposition: ::std::option::Option<String>,
        #[doc = "If true, creates a new session, where session id will be a server generated random id. If false, runs query with an existing session_id passed in ConnectionProperty, otherwise runs query in non-session mode."]
        #[serde(
            rename = "createSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_session: ::std::option::Option<bool>,
        #[doc = "[Optional] Specifies the default dataset to use for unqualified table names in the query. Note that this does not alter behavior of unqualified dataset names."]
        #[serde(
            rename = "defaultDataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_dataset: ::std::option::Option<crate::schemas::DatasetReference>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(
            rename = "destinationEncryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_encryption_configuration:
            ::std::option::Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Optional] Describes the table where the query results should be stored. If not present, a new table will be created to store the results. This property must be set for large results that exceed the maximum response size."]
        #[serde(
            rename = "destinationTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
        #[serde(
            rename = "flattenResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flatten_results: ::std::option::Option<bool>,
        #[doc = "[Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        #[serde(
            rename = "maximumBillingTier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum_billing_tier: ::std::option::Option<i32>,
        #[doc = "[Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        #[serde(
            rename = "maximumBytesBilled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_bytes_billed: ::std::option::Option<i64>,
        #[doc = "Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
        #[serde(
            rename = "parameterMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_mode: ::std::option::Option<String>,
        #[doc = "[Deprecated] This property is deprecated."]
        #[serde(
            rename = "preserveNulls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preserve_nulls: ::std::option::Option<bool>,
        #[doc = "[Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE."]
        #[serde(
            rename = "priority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub priority: ::std::option::Option<String>,
        #[doc = "[Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "Query parameters for standard SQL queries."]
        #[serde(
            rename = "queryParameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_parameters: ::std::option::Option<Vec<crate::schemas::QueryParameter>>,
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(
            rename = "rangePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range_partitioning: ::std::option::Option<crate::schemas::RangePartitioning>,
        #[doc = "Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
        #[serde(
            rename = "schemaUpdateOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_update_options: ::std::option::Option<Vec<String>>,
        #[doc = "[Optional] If querying an external data source outside of BigQuery, describes the data format, location and other properties of the data source. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
        #[serde(
            rename = "tableDefinitions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_definitions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::ExternalDataConfiguration>,
        >,
        #[doc = "Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(
            rename = "timePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_partitioning: ::std::option::Option<crate::schemas::TimePartitioning>,
        #[doc = "Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false."]
        #[serde(
            rename = "useLegacySql",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_legacy_sql: ::std::option::Option<bool>,
        #[doc = "[Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true."]
        #[serde(
            rename = "useQueryCache",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_query_cache: ::std::option::Option<bool>,
        #[doc = "Describes user-defined function resources used in the query."]
        #[serde(
            rename = "userDefinedFunctionResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_defined_function_resources:
            ::std::option::Option<Vec<crate::schemas::UserDefinedFunctionResource>>,
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(
            rename = "writeDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_disposition: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobConfigurationQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobConfigurationQuery {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct JobConfigurationTableCopy {
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(
            rename = "createDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_disposition: ::std::option::Option<String>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(
            rename = "destinationEncryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_encryption_configuration:
            ::std::option::Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Optional] The time when the destination table expires. Expired tables will be deleted and their storage reclaimed."]
        #[serde(
            rename = "destinationExpirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_expiration_time: ::std::option::Option<::serde_json::Value>,
        #[doc = "[Required] The destination table"]
        #[serde(
            rename = "destinationTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Optional] Supported operation types in table copy job."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<String>,
        #[doc = "[Pick one] Source table to copy."]
        #[serde(
            rename = "sourceTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Pick one] Source tables to copy."]
        #[serde(
            rename = "sourceTables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_tables: ::std::option::Option<Vec<crate::schemas::TableReference>>,
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(
            rename = "writeDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_disposition: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobConfigurationTableCopy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobConfigurationTableCopy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct JobList {
        #[doc = "A hash of this page of results."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "List of jobs that were requested."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::JobListJobsItems>>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A token to request the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct JobListJobsItems {
        #[doc = "[Full-projection-only] Specifies the job configuration."]
        #[serde(
            rename = "configuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub configuration: ::std::option::Option<crate::schemas::JobConfiguration>,
        #[doc = "A result object that will be present only if the job has failed."]
        #[serde(
            rename = "errorResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_result: ::std::option::Option<crate::schemas::ErrorProto>,
        #[doc = "Unique opaque ID of the job."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Job reference uniquely identifying the job."]
        #[serde(
            rename = "jobReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_reference: ::std::option::Option<crate::schemas::JobReference>,
        #[doc = "The resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "[Output-only] Information about the job, including starting time and ending time of the job."]
        #[serde(
            rename = "statistics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statistics: ::std::option::Option<crate::schemas::JobStatistics>,
        #[doc = "[Full-projection-only] Describes the state of the job."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::JobStatus>,
        #[doc = "[Full-projection-only] Email address of the user who ran the job."]
        #[serde(
            rename = "user_email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobListJobsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobListJobsItems {
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
    pub struct JobReference {
        #[doc = "[Required] The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
        #[serde(
            rename = "jobId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_id: ::std::option::Option<String>,
        #[doc = "The geographic location of the job. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the project containing this job."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobStatistics {
        #[doc = "[TrustedTester] [Output-only] Job progress (0.0 -> 1.0) for LOAD and EXTRACT jobs."]
        #[serde(
            rename = "completionRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completion_ratio: ::std::option::Option<f64>,
        #[doc = "[Output-only] Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: ::std::option::Option<i64>,
        #[doc = "[Output-only] End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_time: ::std::option::Option<i64>,
        #[doc = "[Output-only] Statistics for an extract job."]
        #[serde(
            rename = "extract",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extract: ::std::option::Option<crate::schemas::JobStatistics4>,
        #[doc = "[Output-only] Statistics for a load job."]
        #[serde(
            rename = "load",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub load: ::std::option::Option<crate::schemas::JobStatistics3>,
        #[doc = "[Output-only] Number of child jobs executed."]
        #[serde(
            rename = "numChildJobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_child_jobs: ::std::option::Option<i64>,
        #[doc = "[Output-only] If this is a child job, the id of the parent."]
        #[serde(
            rename = "parentJobId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_job_id: ::std::option::Option<String>,
        #[doc = "[Output-only] Statistics for a query job."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<crate::schemas::JobStatistics2>,
        #[doc = "[Output-only] Quotas which delayed this job's start time."]
        #[serde(
            rename = "quotaDeferments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota_deferments: ::std::option::Option<Vec<String>>,
        #[doc = "[Output-only] Name of the primary reservation assigned to this job. Note that this could be different than reservations reported in the reservation usage field if parent reservations were used to execute this job."]
        #[serde(
            rename = "reservation_id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reservation_id: ::std::option::Option<String>,
        #[doc = "[Output-only] Job resource usage breakdown by reservation."]
        #[serde(
            rename = "reservationUsage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reservation_usage:
            ::std::option::Option<Vec<crate::schemas::JobStatisticsReservationUsageItems>>,
        #[doc = "[Output-only] [Preview] Statistics for row-level security. Present only for query and extract jobs."]
        #[serde(
            rename = "rowLevelSecurityStatistics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_level_security_statistics:
            ::std::option::Option<crate::schemas::RowLevelSecurityStatistics>,
        #[doc = "[Output-only] Statistics for a child job of a script."]
        #[serde(
            rename = "scriptStatistics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_statistics: ::std::option::Option<crate::schemas::ScriptStatistics>,
        #[doc = "[Output-only] Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_time: ::std::option::Option<i64>,
        #[doc = "[Output-only] [Deprecated] Use the bytes processed in the query statistics instead."]
        #[serde(
            rename = "totalBytesProcessed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: ::std::option::Option<i64>,
        #[doc = "[Output-only] Slot-milliseconds for the job."]
        #[serde(
            rename = "totalSlotMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_slot_ms: ::std::option::Option<i64>,
        #[doc = "[Output-only] [Alpha] Information of the multi-statement transaction if this job is part of one."]
        #[serde(
            rename = "transactionInfoTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction_info_template: ::std::option::Option<crate::schemas::TransactionInfo>,
    }
    impl ::google_field_selector::FieldSelector for JobStatistics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatistics {
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
    pub struct JobStatisticsReservationUsageItems {
        #[doc = "[Output-only] Reservation name or \"unreserved\" for on-demand resources usage."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "[Output-only] Slot-milliseconds the job spent in the given reservation."]
        #[serde(
            rename = "slotMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub slot_ms: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for JobStatisticsReservationUsageItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatisticsReservationUsageItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobStatistics2 {
        #[doc = "[Output-only] Billing tier for the job."]
        #[serde(
            rename = "billingTier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing_tier: ::std::option::Option<i32>,
        #[doc = "[Output-only] Whether the query result was fetched from the query cache."]
        #[serde(
            rename = "cacheHit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_hit: ::std::option::Option<bool>,
        #[doc = "[Output-only] [Preview] The number of row access policies affected by a DDL statement. Present only for DROP ALL ROW ACCESS POLICIES queries."]
        #[serde(
            rename = "ddlAffectedRowAccessPolicyCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ddl_affected_row_access_policy_count: ::std::option::Option<i64>,
        #[doc = "The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): \"CREATE\": The query created the DDL target. \"SKIP\": No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. \"REPLACE\": The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. \"DROP\": The query deleted the DDL target."]
        #[serde(
            rename = "ddlOperationPerformed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ddl_operation_performed: ::std::option::Option<String>,
        #[doc = "[Output-only] The DDL target dataset. Present only for CREATE/ALTER/DROP SCHEMA queries."]
        #[serde(
            rename = "ddlTargetDataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ddl_target_dataset: ::std::option::Option<crate::schemas::DatasetReference>,
        #[doc = "The DDL target routine. Present only for CREATE/DROP FUNCTION/PROCEDURE queries."]
        #[serde(
            rename = "ddlTargetRoutine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ddl_target_routine: ::std::option::Option<crate::schemas::RoutineReference>,
        #[doc = "[Output-only] [Preview] The DDL target row access policy. Present only for CREATE/DROP ROW ACCESS POLICY queries."]
        #[serde(
            rename = "ddlTargetRowAccessPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ddl_target_row_access_policy:
            ::std::option::Option<crate::schemas::RowAccessPolicyReference>,
        #[doc = "[Output-only] The DDL target table. Present only for CREATE/DROP TABLE/VIEW and DROP ALL ROW ACCESS POLICIES queries."]
        #[serde(
            rename = "ddlTargetTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ddl_target_table: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Output-only] The original estimate of bytes processed for the job."]
        #[serde(
            rename = "estimatedBytesProcessed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub estimated_bytes_processed: ::std::option::Option<i64>,
        #[doc = "[Output-only, Beta] Information about create model query job progress."]
        #[serde(
            rename = "modelTraining",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_training: ::std::option::Option<crate::schemas::BigQueryModelTraining>,
        #[doc = "[Output-only, Beta] Deprecated; do not use."]
        #[serde(
            rename = "modelTrainingCurrentIteration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_training_current_iteration: ::std::option::Option<i32>,
        #[doc = "[Output-only, Beta] Deprecated; do not use."]
        #[serde(
            rename = "modelTrainingExpectedTotalIteration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub model_training_expected_total_iteration: ::std::option::Option<i64>,
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        #[serde(
            rename = "numDmlAffectedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_dml_affected_rows: ::std::option::Option<i64>,
        #[doc = "[Output-only] Describes execution plan for the query."]
        #[serde(
            rename = "queryPlan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_plan: ::std::option::Option<Vec<crate::schemas::ExplainQueryStage>>,
        #[doc = "[Output-only] Referenced routines (persistent user-defined functions and stored procedures) for the job."]
        #[serde(
            rename = "referencedRoutines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referenced_routines: ::std::option::Option<Vec<crate::schemas::RoutineReference>>,
        #[doc = "[Output-only] Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list."]
        #[serde(
            rename = "referencedTables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referenced_tables: ::std::option::Option<Vec<crate::schemas::TableReference>>,
        #[doc = "[Output-only] Job resource usage breakdown by reservation."]
        #[serde(
            rename = "reservationUsage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reservation_usage:
            ::std::option::Option<Vec<crate::schemas::JobStatistics2ReservationUsageItems>>,
        #[doc = "[Output-only] The schema of the results. Present only for successful dry run of non-legacy SQL queries."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "The type of query statement, if valid. Possible values (new values might be added in the future): \"SELECT\": SELECT query. \"INSERT\": INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"UPDATE\": UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"DELETE\": DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"MERGE\": MERGE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"ALTER_TABLE\": ALTER TABLE query. \"ALTER_VIEW\": ALTER VIEW query. \"ASSERT\": ASSERT condition AS 'description'. \"CREATE_FUNCTION\": CREATE FUNCTION query. \"CREATE_MODEL\": CREATE [OR REPLACE] MODEL ... AS SELECT ... . \"CREATE_PROCEDURE\": CREATE PROCEDURE query. \"CREATE_TABLE\": CREATE [OR REPLACE] TABLE without AS SELECT. \"CREATE_TABLE_AS_SELECT\": CREATE [OR REPLACE] TABLE ... AS SELECT ... . \"CREATE_VIEW\": CREATE [OR REPLACE] VIEW ... AS SELECT ... . \"DROP_FUNCTION\" : DROP FUNCTION query. \"DROP_PROCEDURE\": DROP PROCEDURE query. \"DROP_TABLE\": DROP TABLE query. \"DROP_VIEW\": DROP VIEW query."]
        #[serde(
            rename = "statementType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statement_type: ::std::option::Option<String>,
        #[doc = "[Output-only] [Beta] Describes a timeline of job execution."]
        #[serde(
            rename = "timeline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeline: ::std::option::Option<Vec<crate::schemas::QueryTimelineSample>>,
        #[doc = "[Output-only] Total bytes billed for the job."]
        #[serde(
            rename = "totalBytesBilled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_billed: ::std::option::Option<i64>,
        #[doc = "[Output-only] Total bytes processed for the job."]
        #[serde(
            rename = "totalBytesProcessed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: ::std::option::Option<i64>,
        #[doc = "[Output-only] For dry-run jobs, totalBytesProcessed is an estimate and this field specifies the accuracy of the estimate. Possible values can be: UNKNOWN: accuracy of the estimate is unknown. PRECISE: estimate is precise. LOWER_BOUND: estimate is lower bound of what the query would cost. UPPER_BOUND: estimate is upper bound of what the query would cost."]
        #[serde(
            rename = "totalBytesProcessedAccuracy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_bytes_processed_accuracy: ::std::option::Option<String>,
        #[doc = "[Output-only] Total number of partitions processed from all partitioned tables referenced in the job."]
        #[serde(
            rename = "totalPartitionsProcessed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_partitions_processed: ::std::option::Option<i64>,
        #[doc = "[Output-only] Slot-milliseconds for the job."]
        #[serde(
            rename = "totalSlotMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_slot_ms: ::std::option::Option<i64>,
        #[doc = "Standard SQL only: list of undeclared query parameters detected during a dry run validation."]
        #[serde(
            rename = "undeclaredQueryParameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub undeclared_query_parameters: ::std::option::Option<Vec<crate::schemas::QueryParameter>>,
    }
    impl ::google_field_selector::FieldSelector for JobStatistics2 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatistics2 {
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
    pub struct JobStatistics2ReservationUsageItems {
        #[doc = "[Output-only] Reservation name or \"unreserved\" for on-demand resources usage."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "[Output-only] Slot-milliseconds the job spent in the given reservation."]
        #[serde(
            rename = "slotMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub slot_ms: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for JobStatistics2ReservationUsageItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatistics2ReservationUsageItems {
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
    pub struct JobStatistics3 {
        #[doc = "[Output-only] The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data."]
        #[serde(
            rename = "badRecords",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bad_records: ::std::option::Option<i64>,
        #[doc = "[Output-only] Number of bytes of source data in a load job."]
        #[serde(
            rename = "inputFileBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub input_file_bytes: ::std::option::Option<i64>,
        #[doc = "[Output-only] Number of source files in a load job."]
        #[serde(
            rename = "inputFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub input_files: ::std::option::Option<i64>,
        #[doc = "[Output-only] Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change."]
        #[serde(
            rename = "outputBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub output_bytes: ::std::option::Option<i64>,
        #[doc = "[Output-only] Number of rows imported in a load job. Note that while an import job is in the running state, this value may change."]
        #[serde(
            rename = "outputRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub output_rows: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for JobStatistics3 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatistics3 {
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
    pub struct JobStatistics4 {
        #[doc = "[Output-only] Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field."]
        #[serde(
            rename = "destinationUriFileCounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_uri_file_counts: ::std::option::Option<Vec<i64>>,
        #[doc = "[Output-only] Number of user bytes extracted into the result. This is the byte count as computed by BigQuery for billing purposes."]
        #[serde(
            rename = "inputBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub input_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for JobStatistics4 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatistics4 {
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
    pub struct JobStatus {
        #[doc = "[Output-only] Final error result of the job. If present, indicates that the job has completed and was unsuccessful."]
        #[serde(
            rename = "errorResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_result: ::std::option::Option<crate::schemas::ErrorProto>,
        #[doc = "[Output-only] The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "[Output-only] Running state of the job."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    pub type JsonObject = ::std::collections::BTreeMap<String, crate::schemas::JsonValue>;
    pub type JsonValue = ::serde_json::Value;
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListModelsResponse {
        #[doc = "Models in the requested dataset. Only the following fields are populated: model_reference, model_type, creation_time, last_modified_time and labels."]
        #[serde(
            rename = "models",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub models: ::std::option::Option<Vec<crate::schemas::Model>>,
        #[doc = "A token to request the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListModelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListModelsResponse {
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
    pub struct ListRoutinesResponse {
        #[doc = "A token to request the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Routines in the requested dataset. Unless read_mask is set in the request, only the following fields are populated: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language."]
        #[serde(
            rename = "routines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub routines: ::std::option::Option<Vec<crate::schemas::Routine>>,
    }
    impl ::google_field_selector::FieldSelector for ListRoutinesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListRoutinesResponse {
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
    pub struct ListRowAccessPoliciesResponse {
        #[doc = "A token to request the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Row access policies on the requested table."]
        #[serde(
            rename = "rowAccessPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_access_policies: ::std::option::Option<Vec<crate::schemas::RowAccessPolicy>>,
    }
    impl ::google_field_selector::FieldSelector for ListRowAccessPoliciesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListRowAccessPoliciesResponse {
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
    pub struct LocationMetadata {
        #[doc = "The legacy BigQuery location ID, e.g. “EU” for the “europe” location. This is for any API consumers that need the legacy “US” and “EU” locations."]
        #[serde(
            rename = "legacyLocationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_location_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LocationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocationMetadata {
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
    pub struct MaterializedViewDefinition {
        #[doc = "[Optional] [TrustedTester] Enable automatic refresh of the materialized view when the base table is updated. The default value is \"true\"."]
        #[serde(
            rename = "enableRefresh",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_refresh: ::std::option::Option<bool>,
        #[doc = "[Output-only] [TrustedTester] The time when this materialized view was last modified, in milliseconds since the epoch."]
        #[serde(
            rename = "lastRefreshTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_refresh_time: ::std::option::Option<i64>,
        #[doc = "[Required] A query whose result is persisted."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "[Optional] [TrustedTester] The maximum frequency at which this materialized view will be refreshed. The default value is \"1800000\" (30 minutes)."]
        #[serde(
            rename = "refreshIntervalMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub refresh_interval_ms: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MaterializedViewDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MaterializedViewDefinition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Model {
        #[doc = "The best trial_id across all training runs."]
        #[serde(
            rename = "bestTrialId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub best_trial_id: ::std::option::Option<i64>,
        #[doc = "Output only. The time when this model was created, in millisecs since the epoch."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: ::std::option::Option<i64>,
        #[doc = "Optional. A user-friendly description of this model."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys). This shows the encryption configuration of the model data while stored in BigQuery storage. This field can be used with PatchModel to update encryption key for an already encrypted model."]
        #[serde(
            rename = "encryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_configuration:
            ::std::option::Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "Output only. A hash of this resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Optional. The time when this model expires, in milliseconds since the epoch. If not present, the model will persist indefinitely. Expired models will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created models."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time: ::std::option::Option<i64>,
        #[doc = "Output only. Input feature columns that were used to train this model."]
        #[serde(
            rename = "featureColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_columns: ::std::option::Option<Vec<crate::schemas::StandardSqlField>>,
        #[doc = "Optional. A descriptive name for this model."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "Output only. Label columns that were used to train this model. The output of the model will have a \"predicted_\" prefix to these columns."]
        #[serde(
            rename = "labelColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_columns: ::std::option::Option<Vec<crate::schemas::StandardSqlField>>,
        #[doc = "The labels associated with this model. You can use these to organize and group your models. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The time when this model was last modified, in millisecs since the epoch."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: ::std::option::Option<i64>,
        #[doc = "Output only. The geographic location where the model resides. This value is inherited from the dataset."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Required. Unique identifier for this model."]
        #[serde(
            rename = "modelReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_reference: ::std::option::Option<crate::schemas::ModelReference>,
        #[doc = "Output only. Type of the model resource."]
        #[serde(
            rename = "modelType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_type: ::std::option::Option<crate::schemas::ModelModelType>,
        #[doc = "Output only. Information for all training runs in increasing order of start_time."]
        #[serde(
            rename = "trainingRuns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_runs: ::std::option::Option<Vec<crate::schemas::TrainingRun>>,
    }
    impl ::google_field_selector::FieldSelector for Model {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Model {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ModelModelType {
        #[doc = "ARIMA model."]
        Arima,
        #[doc = "[Beta] AutoML Tables classification model."]
        AutomlClassifier,
        #[doc = "[Beta] AutoML Tables regression model."]
        AutomlRegressor,
        #[doc = "Boosted tree classifier model."]
        BoostedTreeClassifier,
        #[doc = "Boosted tree regressor model."]
        BoostedTreeRegressor,
        #[doc = "DNN classifier model."]
        DnnClassifier,
        #[doc = "DNN regressor model."]
        DnnRegressor,
        #[doc = "K-means clustering model."]
        Kmeans,
        #[doc = "Linear regression model."]
        LinearRegression,
        #[doc = "Logistic regression based classification model."]
        LogisticRegression,
        #[doc = "Matrix factorization model."]
        MatrixFactorization,
        ModelTypeUnspecified,
        #[doc = "An imported TensorFlow model."]
        Tensorflow,
    }
    impl ModelModelType {
        pub fn as_str(self) -> &'static str {
            match self {
                ModelModelType::Arima => "ARIMA",
                ModelModelType::AutomlClassifier => "AUTOML_CLASSIFIER",
                ModelModelType::AutomlRegressor => "AUTOML_REGRESSOR",
                ModelModelType::BoostedTreeClassifier => "BOOSTED_TREE_CLASSIFIER",
                ModelModelType::BoostedTreeRegressor => "BOOSTED_TREE_REGRESSOR",
                ModelModelType::DnnClassifier => "DNN_CLASSIFIER",
                ModelModelType::DnnRegressor => "DNN_REGRESSOR",
                ModelModelType::Kmeans => "KMEANS",
                ModelModelType::LinearRegression => "LINEAR_REGRESSION",
                ModelModelType::LogisticRegression => "LOGISTIC_REGRESSION",
                ModelModelType::MatrixFactorization => "MATRIX_FACTORIZATION",
                ModelModelType::ModelTypeUnspecified => "MODEL_TYPE_UNSPECIFIED",
                ModelModelType::Tensorflow => "TENSORFLOW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ModelModelType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ModelModelType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ModelModelType, ()> {
            Ok(match s {
                "ARIMA" => ModelModelType::Arima,
                "AUTOML_CLASSIFIER" => ModelModelType::AutomlClassifier,
                "AUTOML_REGRESSOR" => ModelModelType::AutomlRegressor,
                "BOOSTED_TREE_CLASSIFIER" => ModelModelType::BoostedTreeClassifier,
                "BOOSTED_TREE_REGRESSOR" => ModelModelType::BoostedTreeRegressor,
                "DNN_CLASSIFIER" => ModelModelType::DnnClassifier,
                "DNN_REGRESSOR" => ModelModelType::DnnRegressor,
                "KMEANS" => ModelModelType::Kmeans,
                "LINEAR_REGRESSION" => ModelModelType::LinearRegression,
                "LOGISTIC_REGRESSION" => ModelModelType::LogisticRegression,
                "MATRIX_FACTORIZATION" => ModelModelType::MatrixFactorization,
                "MODEL_TYPE_UNSPECIFIED" => ModelModelType::ModelTypeUnspecified,
                "TENSORFLOW" => ModelModelType::Tensorflow,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ModelModelType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ModelModelType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ModelModelType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARIMA" => ModelModelType::Arima,
                "AUTOML_CLASSIFIER" => ModelModelType::AutomlClassifier,
                "AUTOML_REGRESSOR" => ModelModelType::AutomlRegressor,
                "BOOSTED_TREE_CLASSIFIER" => ModelModelType::BoostedTreeClassifier,
                "BOOSTED_TREE_REGRESSOR" => ModelModelType::BoostedTreeRegressor,
                "DNN_CLASSIFIER" => ModelModelType::DnnClassifier,
                "DNN_REGRESSOR" => ModelModelType::DnnRegressor,
                "KMEANS" => ModelModelType::Kmeans,
                "LINEAR_REGRESSION" => ModelModelType::LinearRegression,
                "LOGISTIC_REGRESSION" => ModelModelType::LogisticRegression,
                "MATRIX_FACTORIZATION" => ModelModelType::MatrixFactorization,
                "MODEL_TYPE_UNSPECIFIED" => ModelModelType::ModelTypeUnspecified,
                "TENSORFLOW" => ModelModelType::Tensorflow,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ModelModelType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModelModelType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ModelDefinition {
        #[doc = "[Output-only, Beta] Model options used for the first training run. These options are immutable for subsequent training runs. Default values are used for any options not specified in the input query."]
        #[serde(
            rename = "modelOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_options: ::std::option::Option<crate::schemas::ModelDefinitionModelOptions>,
        #[doc = "[Output-only, Beta] Information about ml training runs, each training run comprises of multiple iterations and there may be multiple training runs for the model if warm start is used or if a user decides to continue a previously cancelled query."]
        #[serde(
            rename = "trainingRuns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_runs: ::std::option::Option<Vec<crate::schemas::BqmlTrainingRun>>,
    }
    impl ::google_field_selector::FieldSelector for ModelDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModelDefinition {
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
    pub struct ModelDefinitionModelOptions {
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "lossType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub loss_type: ::std::option::Option<String>,
        #[serde(
            rename = "modelType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ModelDefinitionModelOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModelDefinitionModelOptions {
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
    pub struct ModelReference {
        #[doc = "[Required] The ID of the dataset containing this model."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the model. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        #[serde(
            rename = "modelId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the project containing this model."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ModelReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModelReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MultiClassClassificationMetrics {
        #[doc = "Aggregate classification metrics."]
        #[serde(
            rename = "aggregateClassificationMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregate_classification_metrics:
            ::std::option::Option<crate::schemas::AggregateClassificationMetrics>,
        #[doc = "Confusion matrix at different thresholds."]
        #[serde(
            rename = "confusionMatrixList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confusion_matrix_list: ::std::option::Option<Vec<crate::schemas::ConfusionMatrix>>,
    }
    impl ::google_field_selector::FieldSelector for MultiClassClassificationMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MultiClassClassificationMetrics {
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
    pub struct ParquetOptions {
        #[doc = "[Optional] Indicates whether to use schema inference specifically for Parquet LIST logical type."]
        #[serde(
            rename = "enableListInference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_list_inference: ::std::option::Option<bool>,
        #[doc = "[Optional] Indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
        #[serde(
            rename = "enumAsString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enum_as_string: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ParquetOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParquetOptions {
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
    pub struct Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Policy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PrincipalComponentInfo {
        #[doc = "The explained_variance is pre-ordered in the descending order to compute the cumulative explained variance ratio."]
        #[serde(
            rename = "cumulativeExplainedVarianceRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cumulative_explained_variance_ratio: ::std::option::Option<f64>,
        #[doc = "Explained variance by this principal component, which is simply the eigenvalue."]
        #[serde(
            rename = "explainedVariance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explained_variance: ::std::option::Option<f64>,
        #[doc = "Explained_variance over the total explained variance."]
        #[serde(
            rename = "explainedVarianceRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explained_variance_ratio: ::std::option::Option<f64>,
        #[doc = "Id of the principal component."]
        #[serde(
            rename = "principalComponentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub principal_component_id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PrincipalComponentInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PrincipalComponentInfo {
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
    pub struct ProjectList {
        #[doc = "A hash of the page of results"]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The type of list."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A token to request the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Projects to which you have at least READ access."]
        #[serde(
            rename = "projects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub projects: ::std::option::Option<Vec<crate::schemas::ProjectListProjectsItems>>,
        #[doc = "The total number of projects in the list."]
        #[serde(
            rename = "totalItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ProjectList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectList {
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
    pub struct ProjectListProjectsItems {
        #[doc = "A descriptive name for this project."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "An opaque ID of this project."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The numeric ID of this project."]
        #[serde(
            rename = "numericId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub numeric_id: ::std::option::Option<u64>,
        #[doc = "A unique reference to this project."]
        #[serde(
            rename = "projectReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_reference: ::std::option::Option<crate::schemas::ProjectReference>,
    }
    impl ::google_field_selector::FieldSelector for ProjectListProjectsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectListProjectsItems {
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
    pub struct ProjectReference {
        #[doc = "[Required] ID of the project. Can be either the numeric ID or the assigned ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProjectReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectReference {
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
    pub struct QueryParameter {
        #[doc = "[Optional] If unset, this is a positional parameter. Otherwise, should be unique within a query."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "[Required] The type of this parameter."]
        #[serde(
            rename = "parameterType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_type: ::std::option::Option<crate::schemas::QueryParameterType>,
        #[doc = "[Required] The value of this parameter."]
        #[serde(
            rename = "parameterValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_value: ::std::option::Option<crate::schemas::QueryParameterValue>,
    }
    impl ::google_field_selector::FieldSelector for QueryParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryParameter {
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
    pub struct QueryParameterType {
        #[doc = "[Optional] The type of the array's elements, if this is an array."]
        #[serde(
            rename = "arrayType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub array_type: ::std::option::Option<Box<crate::schemas::QueryParameterType>>,
        #[doc = "[Required] The top level type of this field."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "[Optional] The types of the fields of this struct, in order, if this is a struct."]
        #[serde(
            rename = "structTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub struct_types:
            ::std::option::Option<Vec<crate::schemas::QueryParameterTypeStructTypesItems>>,
    }
    impl ::google_field_selector::FieldSelector for QueryParameterType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryParameterType {
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
    pub struct QueryParameterTypeStructTypesItems {
        #[doc = "[Optional] Human-oriented description of the field."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "[Optional] The name of this field."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "[Required] The type of this field."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::QueryParameterType>,
    }
    impl ::google_field_selector::FieldSelector for QueryParameterTypeStructTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryParameterTypeStructTypesItems {
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
    pub struct QueryParameterValue {
        #[doc = "[Optional] The array values, if this is an array type."]
        #[serde(
            rename = "arrayValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub array_values: ::std::option::Option<Vec<crate::schemas::QueryParameterValue>>,
        #[doc = "[Optional] The struct field values, in order of the struct type's declaration."]
        #[serde(
            rename = "structValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub struct_values: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::QueryParameterValue>,
        >,
        #[doc = "[Optional] The value of this value, if a simple scalar type."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QueryParameterValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryParameterValue {
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
    pub struct QueryRequest {
        #[doc = "Connection properties."]
        #[serde(
            rename = "connectionProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_properties: ::std::option::Option<Vec<crate::schemas::ConnectionProperty>>,
        #[doc = "If true, creates a new session, where session id will be a server generated random id. If false, runs query with an existing session_id passed in ConnectionProperty, otherwise runs query in non-session mode."]
        #[serde(
            rename = "createSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_session: ::std::option::Option<bool>,
        #[doc = "[Optional] Specifies the default datasetId and projectId to assume for any unqualified table names in the query. If not set, all table names in the query string must be qualified in the format 'datasetId.tableId'."]
        #[serde(
            rename = "defaultDataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_dataset: ::std::option::Option<crate::schemas::DatasetReference>,
        #[doc = "[Optional] If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false."]
        #[serde(
            rename = "dryRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dry_run: ::std::option::Option<bool>,
        #[doc = "The resource type of the request."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The geographic location where the job should run. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "[Optional] The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies."]
        #[serde(
            rename = "maxResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_results: ::std::option::Option<u32>,
        #[doc = "[Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        #[serde(
            rename = "maximumBytesBilled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_bytes_billed: ::std::option::Option<i64>,
        #[doc = "Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
        #[serde(
            rename = "parameterMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_mode: ::std::option::Option<String>,
        #[doc = "[Deprecated] This property is deprecated."]
        #[serde(
            rename = "preserveNulls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preserve_nulls: ::std::option::Option<bool>,
        #[doc = "[Required] A query string, following the BigQuery query syntax, of the query to execute. Example: \"SELECT count(f1) FROM [myProjectId:myDatasetId.myTableId]\"."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "Query parameters for Standard SQL queries."]
        #[serde(
            rename = "queryParameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_parameters: ::std::option::Option<Vec<crate::schemas::QueryParameter>>,
        #[doc = "A unique user provided identifier to ensure idempotent behavior for queries. Note that this is different from the job_id. It has the following properties: 1. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended. 2. Read only queries can ignore this token since they are nullipotent by definition. 3. For the purposes of idempotency ensured by the request_id, a request is considered duplicate of another only if they have the same request_id and are actually duplicates. When determining whether a request is a duplicate of the previous request, all parameters in the request that may affect the behavior are considered. For example, query, connection_properties, query_parameters, use_legacy_sql are parameters that affect the result and are considered when determining whether a request is a duplicate, but properties like timeout_ms don't affect the result and are thus not considered. Dry run query requests are never considered duplicate of another request. 4. When a duplicate mutating query request is detected, it returns: a. the results of the mutation if it completes successfully within the timeout. b. the running operation if it is still in progress at the end of the timeout. 5. Its lifetime is limited to 15 minutes. In other words, if two requests are sent with the same request_id, but more than 15 minutes apart, idempotency is not guaranteed."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
        #[doc = "[Optional] How long to wait for the query to complete, in milliseconds, before the request times out and returns. Note that this is only a timeout for the request, not the query. If the query takes longer to run than the timeout value, the call returns without any results and with the 'jobComplete' flag set to false. You can call GetQueryResults() to wait for the query to complete and read the results. The default value is 10000 milliseconds (10 seconds)."]
        #[serde(
            rename = "timeoutMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeout_ms: ::std::option::Option<u32>,
        #[doc = "Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false."]
        #[serde(
            rename = "useLegacySql",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_legacy_sql: ::std::option::Option<bool>,
        #[doc = "[Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true."]
        #[serde(
            rename = "useQueryCache",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_query_cache: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for QueryRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct QueryResponse {
        #[doc = "Whether the query result was fetched from the query cache."]
        #[serde(
            rename = "cacheHit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_hit: ::std::option::Option<bool>,
        #[doc = "[Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available."]
        #[serde(
            rename = "jobComplete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_complete: ::std::option::Option<bool>,
        #[doc = "Reference to the Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults)."]
        #[serde(
            rename = "jobReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_reference: ::std::option::Option<crate::schemas::JobReference>,
        #[doc = "The resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        #[serde(
            rename = "numDmlAffectedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_dml_affected_rows: ::std::option::Option<i64>,
        #[doc = "A token used for paging results."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The schema of the results. Present only when the query completes successfully."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "The total number of bytes processed for this query. If this query was a dry run, this is the number of bytes that would be processed if the query were run."]
        #[serde(
            rename = "totalBytesProcessed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: ::std::option::Option<i64>,
        #[doc = "The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results."]
        #[serde(
            rename = "totalRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for QueryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryResponse {
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
    pub struct QueryTimelineSample {
        #[doc = "Total number of units currently being processed by workers. This does not correspond directly to slot usage. This is the largest value observed since the last sample."]
        #[serde(
            rename = "activeUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub active_units: ::std::option::Option<i64>,
        #[doc = "Total parallel units of work completed by this query."]
        #[serde(
            rename = "completedUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub completed_units: ::std::option::Option<i64>,
        #[doc = "Milliseconds elapsed since the start of query execution."]
        #[serde(
            rename = "elapsedMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub elapsed_ms: ::std::option::Option<i64>,
        #[doc = "Total parallel units of work remaining for the active stages."]
        #[serde(
            rename = "pendingUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pending_units: ::std::option::Option<i64>,
        #[doc = "Cumulative slot-ms consumed by the query."]
        #[serde(
            rename = "totalSlotMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_slot_ms: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for QueryTimelineSample {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryTimelineSample {
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
    pub struct RangePartitioning {
        #[doc = "[TrustedTester] [Required] The table is partitioned by this field. The field must be a top-level NULLABLE/REQUIRED field. The only supported type is INTEGER/INT64."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<String>,
        #[doc = "[TrustedTester] [Required] Defines the ranges for range partitioning."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::RangePartitioningRange>,
    }
    impl ::google_field_selector::FieldSelector for RangePartitioning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RangePartitioning {
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
    pub struct RangePartitioningRange {
        #[doc = "[TrustedTester] [Required] The end of range partitioning, exclusive."]
        #[serde(
            rename = "end",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end: ::std::option::Option<i64>,
        #[doc = "[TrustedTester] [Required] The width of each interval."]
        #[serde(
            rename = "interval",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub interval: ::std::option::Option<i64>,
        #[doc = "[TrustedTester] [Required] The start of range partitioning, inclusive."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for RangePartitioningRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RangePartitioningRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RankingMetrics {
        #[doc = "Determines the goodness of a ranking by computing the percentile rank from the predicted confidence and dividing it by the original rank."]
        #[serde(
            rename = "averageRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub average_rank: ::std::option::Option<f64>,
        #[doc = "Calculates a precision per user for all the items by ranking them and then averages all the precisions across all the users."]
        #[serde(
            rename = "meanAveragePrecision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mean_average_precision: ::std::option::Option<f64>,
        #[doc = "Similar to the mean squared error computed in regression and explicit recommendation models except instead of computing the rating directly, the output from evaluate is computed against a preference which is 1 or 0 depending on if the rating exists or not."]
        #[serde(
            rename = "meanSquaredError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mean_squared_error: ::std::option::Option<f64>,
        #[doc = "A metric to determine the goodness of a ranking calculated from the predicted confidence by comparing it to an ideal rank measured by the original ratings."]
        #[serde(
            rename = "normalizedDiscountedCumulativeGain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_discounted_cumulative_gain: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for RankingMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RankingMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RegressionMetrics {
        #[doc = "Mean absolute error."]
        #[serde(
            rename = "meanAbsoluteError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mean_absolute_error: ::std::option::Option<f64>,
        #[doc = "Mean squared error."]
        #[serde(
            rename = "meanSquaredError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mean_squared_error: ::std::option::Option<f64>,
        #[doc = "Mean squared log error."]
        #[serde(
            rename = "meanSquaredLogError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mean_squared_log_error: ::std::option::Option<f64>,
        #[doc = "Median absolute error."]
        #[serde(
            rename = "medianAbsoluteError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub median_absolute_error: ::std::option::Option<f64>,
        #[doc = "R^2 score. This corresponds to r2_score in ML.EVALUATE."]
        #[serde(
            rename = "rSquared",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r_squared: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for RegressionMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RegressionMetrics {
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
    pub struct Routine {
        #[doc = "Optional."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<Vec<crate::schemas::Argument>>,
        #[doc = "Output only. The time when this routine was created, in milliseconds since the epoch."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: ::std::option::Option<i64>,
        #[doc = "Required. The body of the routine. For functions, this is the expression in the AS clause. If language=SQL, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, \"\\n\", y))` The definition_body is `concat(x, \"\\n\", y)` (\\n is not replaced with linebreak). If language=JAVASCRIPT, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return \"\\n\";\\n'` The definition_body is `return \"\\n\";\\n` Note that both \\n are replaced with linebreaks."]
        #[serde(
            rename = "definitionBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub definition_body: ::std::option::Option<String>,
        #[doc = "Optional. [Experimental] The description of the routine if defined."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. [Experimental] The determinism level of the JavaScript UDF if defined."]
        #[serde(
            rename = "determinismLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub determinism_level: ::std::option::Option<crate::schemas::RoutineDeterminismLevel>,
        #[doc = "Output only. A hash of this resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Optional. If language = \"JAVASCRIPT\", this field stores the path of the imported JAVASCRIPT libraries."]
        #[serde(
            rename = "importedLibraries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub imported_libraries: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Defaults to \"SQL\"."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<crate::schemas::RoutineLanguage>,
        #[doc = "Output only. The time when this routine was last modified, in milliseconds since the epoch."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: ::std::option::Option<i64>,
        #[doc = "Optional. Set only if Routine is a \"TABLE_VALUED_FUNCTION\". TODO(b/173344646) - Update return_type documentation to say it cannot be set for TABLE_VALUED_FUNCTION before preview launch."]
        #[serde(
            rename = "returnTableType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_table_type: ::std::option::Option<crate::schemas::StandardSqlTableType>,
        #[doc = "Optional if language = \"SQL\"; required otherwise. If absent, the return type is inferred from definition_body at query time in each query that references this routine. If present, then the evaluated result will be cast to the specified returned type at query time. For example, for the functions created with the following statements: * `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);` * `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));` * `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));` The return_type is `{type_kind: \"FLOAT64\"}` for `Add` and `Decrement`, and is absent for `Increment` (inferred as FLOAT64 at query time). Suppose the function `Add` is replaced by `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);` Then the inferred return type of `Increment` is automatically changed to INT64 at query time, while the return type of `Decrement` remains FLOAT64."]
        #[serde(
            rename = "returnType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_type: ::std::option::Option<crate::schemas::StandardSqlDataType>,
        #[doc = "Required. Reference describing the ID of this routine."]
        #[serde(
            rename = "routineReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub routine_reference: ::std::option::Option<crate::schemas::RoutineReference>,
        #[doc = "Required. The type of routine."]
        #[serde(
            rename = "routineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub routine_type: ::std::option::Option<crate::schemas::RoutineRoutineType>,
    }
    impl ::google_field_selector::FieldSelector for Routine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Routine {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RoutineDeterminismLevel {
        #[doc = "The determinism of the UDF is unspecified."]
        DeterminismLevelUnspecified,
        #[doc = "The UDF is deterministic, meaning that 2 function calls with the same inputs always produce the same result, even across 2 query runs."]
        Deterministic,
        #[doc = "The UDF is not deterministic."]
        NotDeterministic,
    }
    impl RoutineDeterminismLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                RoutineDeterminismLevel::DeterminismLevelUnspecified => {
                    "DETERMINISM_LEVEL_UNSPECIFIED"
                }
                RoutineDeterminismLevel::Deterministic => "DETERMINISTIC",
                RoutineDeterminismLevel::NotDeterministic => "NOT_DETERMINISTIC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RoutineDeterminismLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RoutineDeterminismLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RoutineDeterminismLevel, ()> {
            Ok(match s {
                "DETERMINISM_LEVEL_UNSPECIFIED" => {
                    RoutineDeterminismLevel::DeterminismLevelUnspecified
                }
                "DETERMINISTIC" => RoutineDeterminismLevel::Deterministic,
                "NOT_DETERMINISTIC" => RoutineDeterminismLevel::NotDeterministic,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RoutineDeterminismLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoutineDeterminismLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoutineDeterminismLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DETERMINISM_LEVEL_UNSPECIFIED" => {
                    RoutineDeterminismLevel::DeterminismLevelUnspecified
                }
                "DETERMINISTIC" => RoutineDeterminismLevel::Deterministic,
                "NOT_DETERMINISTIC" => RoutineDeterminismLevel::NotDeterministic,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RoutineDeterminismLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoutineDeterminismLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RoutineLanguage {
        #[doc = "JavaScript language."]
        Javascript,
        LanguageUnspecified,
        #[doc = "SQL language."]
        Sql,
    }
    impl RoutineLanguage {
        pub fn as_str(self) -> &'static str {
            match self {
                RoutineLanguage::Javascript => "JAVASCRIPT",
                RoutineLanguage::LanguageUnspecified => "LANGUAGE_UNSPECIFIED",
                RoutineLanguage::Sql => "SQL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RoutineLanguage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RoutineLanguage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RoutineLanguage, ()> {
            Ok(match s {
                "JAVASCRIPT" => RoutineLanguage::Javascript,
                "LANGUAGE_UNSPECIFIED" => RoutineLanguage::LanguageUnspecified,
                "SQL" => RoutineLanguage::Sql,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RoutineLanguage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoutineLanguage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoutineLanguage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JAVASCRIPT" => RoutineLanguage::Javascript,
                "LANGUAGE_UNSPECIFIED" => RoutineLanguage::LanguageUnspecified,
                "SQL" => RoutineLanguage::Sql,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RoutineLanguage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoutineLanguage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RoutineRoutineType {
        #[doc = "Stored procedure."]
        Procedure,
        RoutineTypeUnspecified,
        #[doc = "Non-builtin permanent scalar function."]
        ScalarFunction,
        #[doc = "Non-builtin permanent TVF."]
        TableValuedFunction,
    }
    impl RoutineRoutineType {
        pub fn as_str(self) -> &'static str {
            match self {
                RoutineRoutineType::Procedure => "PROCEDURE",
                RoutineRoutineType::RoutineTypeUnspecified => "ROUTINE_TYPE_UNSPECIFIED",
                RoutineRoutineType::ScalarFunction => "SCALAR_FUNCTION",
                RoutineRoutineType::TableValuedFunction => "TABLE_VALUED_FUNCTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RoutineRoutineType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RoutineRoutineType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RoutineRoutineType, ()> {
            Ok(match s {
                "PROCEDURE" => RoutineRoutineType::Procedure,
                "ROUTINE_TYPE_UNSPECIFIED" => RoutineRoutineType::RoutineTypeUnspecified,
                "SCALAR_FUNCTION" => RoutineRoutineType::ScalarFunction,
                "TABLE_VALUED_FUNCTION" => RoutineRoutineType::TableValuedFunction,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RoutineRoutineType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoutineRoutineType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoutineRoutineType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PROCEDURE" => RoutineRoutineType::Procedure,
                "ROUTINE_TYPE_UNSPECIFIED" => RoutineRoutineType::RoutineTypeUnspecified,
                "SCALAR_FUNCTION" => RoutineRoutineType::ScalarFunction,
                "TABLE_VALUED_FUNCTION" => RoutineRoutineType::TableValuedFunction,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RoutineRoutineType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoutineRoutineType {
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
    pub struct RoutineReference {
        #[doc = "[Required] The ID of the dataset containing this routine."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the project containing this routine."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
        #[serde(
            rename = "routineId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub routine_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RoutineReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoutineReference {
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
    pub struct Row {
        #[doc = "The original label of this row."]
        #[serde(
            rename = "actualLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actual_label: ::std::option::Option<String>,
        #[doc = "Info describing predicted label distribution."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries: ::std::option::Option<Vec<crate::schemas::Entry>>,
    }
    impl ::google_field_selector::FieldSelector for Row {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Row {
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
    pub struct RowAccessPolicy {
        #[doc = "Output only. The time when this row access policy was created, in milliseconds since the epoch."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<String>,
        #[doc = "Output only. A hash of this resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region=\"EU\" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0"]
        #[serde(
            rename = "filterPredicate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_predicate: ::std::option::Option<String>,
        #[doc = "Output only. The time when this row access policy was last modified, in milliseconds since the epoch."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_modified_time: ::std::option::Option<String>,
        #[doc = "Required. Reference describing the ID of this row access policy."]
        #[serde(
            rename = "rowAccessPolicyReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_access_policy_reference:
            ::std::option::Option<crate::schemas::RowAccessPolicyReference>,
    }
    impl ::google_field_selector::FieldSelector for RowAccessPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RowAccessPolicy {
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
    pub struct RowAccessPolicyReference {
        #[doc = "[Required] The ID of the dataset containing this row access policy."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the row access policy. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
        #[serde(
            rename = "policyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the project containing this row access policy."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the table containing this row access policy."]
        #[serde(
            rename = "tableId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RowAccessPolicyReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RowAccessPolicyReference {
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
    pub struct RowLevelSecurityStatistics {
        #[doc = "[Output-only] [Preview] Whether any accessed data was protected by row access policies."]
        #[serde(
            rename = "rowLevelSecurityApplied",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_level_security_applied: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RowLevelSecurityStatistics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RowLevelSecurityStatistics {
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
    pub struct ScriptStackFrame {
        #[doc = "[Output-only] One-based end column."]
        #[serde(
            rename = "endColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_column: ::std::option::Option<i32>,
        #[doc = "[Output-only] One-based end line."]
        #[serde(
            rename = "endLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_line: ::std::option::Option<i32>,
        #[doc = "[Output-only] Name of the active procedure, empty if in a top-level script."]
        #[serde(
            rename = "procedureId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub procedure_id: ::std::option::Option<String>,
        #[doc = "[Output-only] One-based start column."]
        #[serde(
            rename = "startColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_column: ::std::option::Option<i32>,
        #[doc = "[Output-only] One-based start line."]
        #[serde(
            rename = "startLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_line: ::std::option::Option<i32>,
        #[doc = "[Output-only] Text of the current statement/expression."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ScriptStackFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScriptStackFrame {
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
    pub struct ScriptStatistics {
        #[doc = "[Output-only] Whether this child job was a statement or expression."]
        #[serde(
            rename = "evaluationKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluation_kind: ::std::option::Option<String>,
        #[doc = "Stack trace showing the line/column/procedure name of each frame on the stack at the point where the current evaluation happened. The leaf frame is first, the primary script is last. Never empty."]
        #[serde(
            rename = "stackFrames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_frames: ::std::option::Option<Vec<crate::schemas::ScriptStackFrame>>,
    }
    impl ::google_field_selector::FieldSelector for ScriptStatistics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScriptStatistics {
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
    pub struct SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetIamPolicyRequest {
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
    pub struct SnapshotDefinition {
        #[doc = "[Required] Reference describing the ID of the table that is snapshotted."]
        #[serde(
            rename = "baseTableReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_table_reference: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "[Required] The time at which the base table was snapshot."]
        #[serde(
            rename = "snapshotTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapshot_time: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotDefinition {
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
    pub struct StandardSqlDataType {
        #[doc = "The type of the array's elements, if type_kind = \"ARRAY\"."]
        #[serde(
            rename = "arrayElementType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub array_element_type: ::std::option::Option<Box<crate::schemas::StandardSqlDataType>>,
        #[doc = "The fields of this struct, in order, if type_kind = \"STRUCT\"."]
        #[serde(
            rename = "structType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub struct_type: ::std::option::Option<crate::schemas::StandardSqlStructType>,
        #[doc = "Required. The top level type of this field. Can be any standard SQL data type (e.g., \"INT64\", \"DATE\", \"ARRAY\")."]
        #[serde(
            rename = "typeKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_kind: ::std::option::Option<crate::schemas::StandardSqlDataTypeTypeKind>,
    }
    impl ::google_field_selector::FieldSelector for StandardSqlDataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StandardSqlDataType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StandardSqlDataTypeTypeKind {
        #[doc = "Encoded as a list with types matching Type.array_type."]
        Array,
        #[doc = "Encoded as a decimal string."]
        Bignumeric,
        #[doc = "Encoded as a boolean \"false\" or \"true\"."]
        Bool,
        #[doc = "Encoded as a base64 string per RFC 4648, section 4."]
        Bytes,
        #[doc = "Encoded as RFC 3339 full-date format string: 1985-04-12"]
        Date,
        #[doc = "Encoded as RFC 3339 full-date \"T\" partial-time: 1985-04-12T23:20:50.52"]
        Datetime,
        #[doc = "Encoded as a number, or string \"NaN\", \"Infinity\" or \"-Infinity\"."]
        Float64,
        #[doc = "Encoded as WKT"]
        Geography,
        #[doc = "Encoded as a string in decimal format."]
        Int64,
        #[doc = "Encoded as a decimal string."]
        Numeric,
        #[doc = "Encoded as a string value."]
        String,
        #[doc = "Encoded as a list with fields of type Type.struct_type[i]. List is used because a JSON object cannot have duplicate field names."]
        Struct,
        #[doc = "Encoded as RFC 3339 partial-time format string: 23:20:50.52"]
        Time,
        #[doc = "Encoded as an RFC 3339 timestamp with mandatory \"Z\" time zone string: 1985-04-12T23:20:50.52Z"]
        Timestamp,
        #[doc = "Invalid type."]
        TypeKindUnspecified,
    }
    impl StandardSqlDataTypeTypeKind {
        pub fn as_str(self) -> &'static str {
            match self {
                StandardSqlDataTypeTypeKind::Array => "ARRAY",
                StandardSqlDataTypeTypeKind::Bignumeric => "BIGNUMERIC",
                StandardSqlDataTypeTypeKind::Bool => "BOOL",
                StandardSqlDataTypeTypeKind::Bytes => "BYTES",
                StandardSqlDataTypeTypeKind::Date => "DATE",
                StandardSqlDataTypeTypeKind::Datetime => "DATETIME",
                StandardSqlDataTypeTypeKind::Float64 => "FLOAT64",
                StandardSqlDataTypeTypeKind::Geography => "GEOGRAPHY",
                StandardSqlDataTypeTypeKind::Int64 => "INT64",
                StandardSqlDataTypeTypeKind::Numeric => "NUMERIC",
                StandardSqlDataTypeTypeKind::String => "STRING",
                StandardSqlDataTypeTypeKind::Struct => "STRUCT",
                StandardSqlDataTypeTypeKind::Time => "TIME",
                StandardSqlDataTypeTypeKind::Timestamp => "TIMESTAMP",
                StandardSqlDataTypeTypeKind::TypeKindUnspecified => "TYPE_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for StandardSqlDataTypeTypeKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StandardSqlDataTypeTypeKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StandardSqlDataTypeTypeKind, ()> {
            Ok(match s {
                "ARRAY" => StandardSqlDataTypeTypeKind::Array,
                "BIGNUMERIC" => StandardSqlDataTypeTypeKind::Bignumeric,
                "BOOL" => StandardSqlDataTypeTypeKind::Bool,
                "BYTES" => StandardSqlDataTypeTypeKind::Bytes,
                "DATE" => StandardSqlDataTypeTypeKind::Date,
                "DATETIME" => StandardSqlDataTypeTypeKind::Datetime,
                "FLOAT64" => StandardSqlDataTypeTypeKind::Float64,
                "GEOGRAPHY" => StandardSqlDataTypeTypeKind::Geography,
                "INT64" => StandardSqlDataTypeTypeKind::Int64,
                "NUMERIC" => StandardSqlDataTypeTypeKind::Numeric,
                "STRING" => StandardSqlDataTypeTypeKind::String,
                "STRUCT" => StandardSqlDataTypeTypeKind::Struct,
                "TIME" => StandardSqlDataTypeTypeKind::Time,
                "TIMESTAMP" => StandardSqlDataTypeTypeKind::Timestamp,
                "TYPE_KIND_UNSPECIFIED" => StandardSqlDataTypeTypeKind::TypeKindUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StandardSqlDataTypeTypeKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StandardSqlDataTypeTypeKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StandardSqlDataTypeTypeKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARRAY" => StandardSqlDataTypeTypeKind::Array,
                "BIGNUMERIC" => StandardSqlDataTypeTypeKind::Bignumeric,
                "BOOL" => StandardSqlDataTypeTypeKind::Bool,
                "BYTES" => StandardSqlDataTypeTypeKind::Bytes,
                "DATE" => StandardSqlDataTypeTypeKind::Date,
                "DATETIME" => StandardSqlDataTypeTypeKind::Datetime,
                "FLOAT64" => StandardSqlDataTypeTypeKind::Float64,
                "GEOGRAPHY" => StandardSqlDataTypeTypeKind::Geography,
                "INT64" => StandardSqlDataTypeTypeKind::Int64,
                "NUMERIC" => StandardSqlDataTypeTypeKind::Numeric,
                "STRING" => StandardSqlDataTypeTypeKind::String,
                "STRUCT" => StandardSqlDataTypeTypeKind::Struct,
                "TIME" => StandardSqlDataTypeTypeKind::Time,
                "TIMESTAMP" => StandardSqlDataTypeTypeKind::Timestamp,
                "TYPE_KIND_UNSPECIFIED" => StandardSqlDataTypeTypeKind::TypeKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for StandardSqlDataTypeTypeKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StandardSqlDataTypeTypeKind {
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
    pub struct StandardSqlField {
        #[doc = "Optional. The name of this field. Can be absent for struct fields."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The type of this parameter. Absent if not explicitly specified (e.g., CREATE FUNCTION statement can omit the return type; in this case the output parameter does not have this \"type\" field)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::StandardSqlDataType>,
    }
    impl ::google_field_selector::FieldSelector for StandardSqlField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StandardSqlField {
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
    pub struct StandardSqlStructType {
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::StandardSqlField>>,
    }
    impl ::google_field_selector::FieldSelector for StandardSqlStructType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StandardSqlStructType {
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
    pub struct StandardSqlTableType {
        #[doc = "The columns in this table type"]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<Vec<crate::schemas::StandardSqlField>>,
    }
    impl ::google_field_selector::FieldSelector for StandardSqlTableType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StandardSqlTableType {
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
    pub struct Streamingbuffer {
        #[doc = "[Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer."]
        #[serde(
            rename = "estimatedBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub estimated_bytes: ::std::option::Option<u64>,
        #[doc = "[Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer."]
        #[serde(
            rename = "estimatedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub estimated_rows: ::std::option::Option<u64>,
        #[doc = "[Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available."]
        #[serde(
            rename = "oldestEntryTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub oldest_entry_time: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for Streamingbuffer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Streamingbuffer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Table {
        #[doc = "[Beta] Clustering specification for the table. Must be specified with partitioning, data in the table will be first partitioned and subsequently clustered."]
        #[serde(
            rename = "clustering",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clustering: ::std::option::Option<crate::schemas::Clustering>,
        #[doc = "[Output-only] The time when this table was created, in milliseconds since the epoch."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: ::std::option::Option<i64>,
        #[doc = "[Optional] A user-friendly description of this table."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(
            rename = "encryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_configuration:
            ::std::option::Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Output-only] A hash of the table metadata. Used to ensure there were no concurrent modifications to the resource when attempting an update. Not guaranteed to change when the table contents or the fields numRows, numBytes, numLongTermBytes or lastModifiedTime change."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "[Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time: ::std::option::Option<i64>,
        #[doc = "[Optional] Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
        #[serde(
            rename = "externalDataConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_data_configuration:
            ::std::option::Option<crate::schemas::ExternalDataConfiguration>,
        #[doc = "[Optional] A descriptive name for this table."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "[Output-only] An opaque ID uniquely identifying the table."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "[Output-only] The type of the resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "[Output-only] The time when this table was last modified, in milliseconds since the epoch."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: ::std::option::Option<u64>,
        #[doc = "[Output-only] The geographic location where the table resides. This value is inherited from the dataset."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "[Optional] Materialized view definition."]
        #[serde(
            rename = "materializedView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materialized_view: ::std::option::Option<crate::schemas::MaterializedViewDefinition>,
        #[doc = "[Output-only, Beta] Present iff this table represents a ML model. Describes the training information for the model, and it is required to run 'PREDICT' queries."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<crate::schemas::ModelDefinition>,
        #[doc = "[Output-only] The size of this table in bytes, excluding any data in the streaming buffer."]
        #[serde(
            rename = "numBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_bytes: ::std::option::Option<i64>,
        #[doc = "[Output-only] The number of bytes in the table that are considered \"long-term storage\"."]
        #[serde(
            rename = "numLongTermBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_long_term_bytes: ::std::option::Option<i64>,
        #[doc = "[Output-only] [TrustedTester] The physical size of this table in bytes, excluding any data in the streaming buffer. This includes compression and storage used for time travel."]
        #[serde(
            rename = "numPhysicalBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_physical_bytes: ::std::option::Option<i64>,
        #[doc = "[Output-only] The number of rows of data in this table, excluding any data in the streaming buffer."]
        #[serde(
            rename = "numRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_rows: ::std::option::Option<u64>,
        #[doc = "[Output-only] Describes the table type. The following values are supported: TABLE: A normal BigQuery table. VIEW: A virtual table defined by a SQL query. SNAPSHOT: An immutable, read-only table that is a copy of another table. [TrustedTester] MATERIALIZED_VIEW: SQL query whose result is persisted. EXTERNAL: A table that references data stored in an external storage system, such as Google Cloud Storage. The default value is TABLE."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(
            rename = "rangePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range_partitioning: ::std::option::Option<crate::schemas::RangePartitioning>,
        #[doc = "[Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
        #[serde(
            rename = "requirePartitionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_partition_filter: ::std::option::Option<bool>,
        #[doc = "[Optional] Describes the schema of this table."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "[Output-only] A URL that can be used to access this resource again."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "[Output-only] Snapshot definition."]
        #[serde(
            rename = "snapshotDefinition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapshot_definition: ::std::option::Option<crate::schemas::SnapshotDefinition>,
        #[doc = "[Output-only] Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer."]
        #[serde(
            rename = "streamingBuffer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub streaming_buffer: ::std::option::Option<crate::schemas::Streamingbuffer>,
        #[doc = "[Required] Reference describing the ID of this table."]
        #[serde(
            rename = "tableReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_reference: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "Time-based partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(
            rename = "timePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_partitioning: ::std::option::Option<crate::schemas::TimePartitioning>,
        #[doc = "[Optional] The view definition."]
        #[serde(
            rename = "view",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view: ::std::option::Option<crate::schemas::ViewDefinition>,
    }
    impl ::google_field_selector::FieldSelector for Table {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Table {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableCell {
        #[serde(
            rename = "v",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v: ::std::option::Option<::serde_json::Value>,
    }
    impl ::google_field_selector::FieldSelector for TableCell {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCell {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableDataInsertAllRequest {
        #[doc = "[Optional] Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors."]
        #[serde(
            rename = "ignoreUnknownValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ignore_unknown_values: ::std::option::Option<bool>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The rows to insert."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::TableDataInsertAllRequestRowsItems>>,
        #[doc = "[Optional] Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist."]
        #[serde(
            rename = "skipInvalidRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_invalid_rows: ::std::option::Option<bool>,
        #[doc = "If specified, treats the destination table as a base template, and inserts the rows into an instance table named \"{destination}{templateSuffix}\". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables."]
        #[serde(
            rename = "templateSuffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_suffix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TableDataInsertAllRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableDataInsertAllRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableDataInsertAllRequestRowsItems {
        #[doc = "[Optional] A unique ID for each row. BigQuery uses this property to detect duplicate insertion requests on a best-effort basis."]
        #[serde(
            rename = "insertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_id: ::std::option::Option<String>,
        #[doc = "[Required] A JSON object that contains a row of data. The object's properties and values must match the destination table's schema."]
        #[serde(
            rename = "json",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonValue>>,
    }
    impl ::google_field_selector::FieldSelector for TableDataInsertAllRequestRowsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableDataInsertAllRequestRowsItems {
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
    pub struct TableDataInsertAllResponse {
        #[doc = "An array of errors for rows that were not inserted."]
        #[serde(
            rename = "insertErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_errors:
            ::std::option::Option<Vec<crate::schemas::TableDataInsertAllResponseInsertErrorsItems>>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TableDataInsertAllResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableDataInsertAllResponse {
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
    pub struct TableDataInsertAllResponseInsertErrorsItems {
        #[doc = "Error information for the row indicated by the index property."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "The index of the row that error applies to."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<u32>,
    }
    impl ::google_field_selector::FieldSelector for TableDataInsertAllResponseInsertErrorsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableDataInsertAllResponseInsertErrorsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableDataList {
        #[doc = "A hash of this page of results."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The resource type of the response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A token used for paging results. Providing this token instead of the startIndex parameter can help you retrieve stable results when an underlying table is changing."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Rows of results."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The total number of rows in the complete table."]
        #[serde(
            rename = "totalRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for TableDataList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableDataList {
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
    pub struct TableFieldSchema {
        #[doc = "[Optional] The categories attached to this field, used for field-level access control."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<crate::schemas::TableFieldSchemaCategories>,
        #[doc = "[Optional] The field description. The maximum length is 1,024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "[Optional] Describes the nested schema fields if the type property is set to RECORD."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::TableFieldSchema>>,
        #[doc = "[Optional] The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<String>,
        #[doc = "[Required] The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 128 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "policyTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_tags: ::std::option::Option<crate::schemas::TableFieldSchemaPolicyTags>,
        #[doc = "[Required] The field data type. Possible values include STRING, BYTES, INTEGER, INT64 (same as INTEGER), FLOAT, FLOAT64 (same as FLOAT), NUMERIC, BIGNUMERIC, BOOLEAN, BOOL (same as BOOLEAN), TIMESTAMP, DATE, TIME, DATETIME, RECORD (where RECORD indicates that the field contains a nested schema) or STRUCT (same as RECORD)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TableFieldSchema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableFieldSchema {
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
    pub struct TableFieldSchemaCategories {
        #[doc = "A list of category resource names. For example, \"projects/1/taxonomies/2/categories/3\". At most 5 categories are allowed."]
        #[serde(
            rename = "names",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TableFieldSchemaCategories {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableFieldSchemaCategories {
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
    pub struct TableFieldSchemaPolicyTags {
        #[doc = "A list of category resource names. For example, \"projects/1/location/eu/taxonomies/2/policyTags/3\". At most 1 policy tag is allowed."]
        #[serde(
            rename = "names",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TableFieldSchemaPolicyTags {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableFieldSchemaPolicyTags {
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
    pub struct TableList {
        #[doc = "A hash of this page of results."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The type of list."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A token to request the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Tables in the requested dataset."]
        #[serde(
            rename = "tables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tables: ::std::option::Option<Vec<crate::schemas::TableListTablesItems>>,
        #[doc = "The total number of tables in the dataset."]
        #[serde(
            rename = "totalItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TableList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableList {
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
    pub struct TableListTablesItems {
        #[doc = "[Beta] Clustering specification for this table, if configured."]
        #[serde(
            rename = "clustering",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clustering: ::std::option::Option<crate::schemas::Clustering>,
        #[doc = "The time when this table was created, in milliseconds since the epoch."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: ::std::option::Option<i64>,
        #[doc = "[Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time: ::std::option::Option<i64>,
        #[doc = "The user-friendly name for this table."]
        #[serde(
            rename = "friendlyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friendly_name: ::std::option::Option<String>,
        #[doc = "An opaque ID of the table"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The labels associated with this table. You can use these to organize and group your tables."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of table. Possible values are: TABLE, VIEW."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The range partitioning specification for this table, if configured."]
        #[serde(
            rename = "rangePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range_partitioning: ::std::option::Option<crate::schemas::RangePartitioning>,
        #[doc = "A reference uniquely identifying the table."]
        #[serde(
            rename = "tableReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_reference: ::std::option::Option<crate::schemas::TableReference>,
        #[doc = "The time-based partitioning specification for this table, if configured."]
        #[serde(
            rename = "timePartitioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_partitioning: ::std::option::Option<crate::schemas::TimePartitioning>,
        #[doc = "Additional details for a view."]
        #[serde(
            rename = "view",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view: ::std::option::Option<crate::schemas::TableListTablesItemsView>,
    }
    impl ::google_field_selector::FieldSelector for TableListTablesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableListTablesItems {
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
    pub struct TableListTablesItemsView {
        #[doc = "True if view is defined in legacy SQL dialect, false if in standard SQL."]
        #[serde(
            rename = "useLegacySql",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_legacy_sql: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TableListTablesItemsView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableListTablesItemsView {
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
    pub struct TableReference {
        #[doc = "[Required] The ID of the dataset containing this table."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the project containing this table."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "[Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        #[serde(
            rename = "tableId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TableReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableRow {
        #[doc = "Represents a single row in the result set, consisting of one or more fields."]
        #[serde(
            rename = "f",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub f: ::std::option::Option<Vec<crate::schemas::TableCell>>,
    }
    impl ::google_field_selector::FieldSelector for TableRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableRow {
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
    pub struct TableSchema {
        #[doc = "Describes the fields in a table."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::TableFieldSchema>>,
    }
    impl ::google_field_selector::FieldSelector for TableSchema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableSchema {
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
    pub struct TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsRequest {
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
    pub struct TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsResponse {
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
    pub struct TimePartitioning {
        #[doc = "[Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value."]
        #[serde(
            rename = "expirationMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expiration_ms: ::std::option::Option<i64>,
        #[doc = "[Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either '_PARTITIONTIME' as TIMESTAMP type, or '_PARTITIONDATE' as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<String>,
        #[doc = "[Required] The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively. When the type is not specified, the default behavior is DAY."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "requirePartitionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_partition_filter: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TimePartitioning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimePartitioning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrainingOptions {
        #[doc = "Whether to enable auto ARIMA or not."]
        #[serde(
            rename = "autoArima",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_arima: ::std::option::Option<bool>,
        #[doc = "The max value of non-seasonal p and q."]
        #[serde(
            rename = "autoArimaMaxOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub auto_arima_max_order: ::std::option::Option<i64>,
        #[doc = "Batch size for dnn models."]
        #[serde(
            rename = "batchSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub batch_size: ::std::option::Option<i64>,
        #[doc = "The data frequency of a time series."]
        #[serde(
            rename = "dataFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_frequency: ::std::option::Option<crate::schemas::TrainingOptionsDataFrequency>,
        #[doc = "The column to split data with. This column won't be used as a feature. 1. When data_split_method is CUSTOM, the corresponding column should be boolean. The rows with true value tag are eval data, and the false are training data. 2. When data_split_method is SEQ, the first DATA_SPLIT_EVAL_FRACTION rows (from smallest to largest) in the corresponding column are used as training data, and the rest are eval data. It respects the order in Orderable data types: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties"]
        #[serde(
            rename = "dataSplitColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_split_column: ::std::option::Option<String>,
        #[doc = "The fraction of evaluation data over the whole input data. The rest of data will be used as training data. The format should be double. Accurate to two decimal places. Default value is 0.2."]
        #[serde(
            rename = "dataSplitEvalFraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_split_eval_fraction: ::std::option::Option<f64>,
        #[doc = "The data split type for training and evaluation, e.g. RANDOM."]
        #[serde(
            rename = "dataSplitMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_split_method:
            ::std::option::Option<crate::schemas::TrainingOptionsDataSplitMethod>,
        #[doc = "Distance type for clustering models."]
        #[serde(
            rename = "distanceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distance_type: ::std::option::Option<crate::schemas::TrainingOptionsDistanceType>,
        #[doc = "Dropout probability for dnn models."]
        #[serde(
            rename = "dropout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dropout: ::std::option::Option<f64>,
        #[doc = "Whether to stop early when the loss doesn't improve significantly any more (compared to min_relative_progress). Used only for iterative training algorithms."]
        #[serde(
            rename = "earlyStop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub early_stop: ::std::option::Option<bool>,
        #[doc = "Feedback type that specifies which algorithm to run for matrix factorization."]
        #[serde(
            rename = "feedbackType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feedback_type: ::std::option::Option<crate::schemas::TrainingOptionsFeedbackType>,
        #[doc = "Hidden units for dnn models."]
        #[serde(
            rename = "hiddenUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_units: ::std::option::Option<Vec<i64>>,
        #[doc = "The geographical region based on which the holidays are considered in time series modeling. If a valid value is specified, then holiday effects modeling is enabled."]
        #[serde(
            rename = "holidayRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub holiday_region: ::std::option::Option<crate::schemas::TrainingOptionsHolidayRegion>,
        #[doc = "The number of periods ahead that need to be forecasted."]
        #[serde(
            rename = "horizon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub horizon: ::std::option::Option<i64>,
        #[doc = "Include drift when fitting an ARIMA model."]
        #[serde(
            rename = "includeDrift",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_drift: ::std::option::Option<bool>,
        #[doc = "Specifies the initial learning rate for the line search learn rate strategy."]
        #[serde(
            rename = "initialLearnRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_learn_rate: ::std::option::Option<f64>,
        #[doc = "Name of input label columns in training data."]
        #[serde(
            rename = "inputLabelColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_label_columns: ::std::option::Option<Vec<String>>,
        #[doc = "Item column specified for matrix factorization models."]
        #[serde(
            rename = "itemColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_column: ::std::option::Option<String>,
        #[doc = "The column used to provide the initial centroids for kmeans algorithm when kmeans_initialization_method is CUSTOM."]
        #[serde(
            rename = "kmeansInitializationColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kmeans_initialization_column: ::std::option::Option<String>,
        #[doc = "The method used to initialize the centroids for kmeans algorithm."]
        #[serde(
            rename = "kmeansInitializationMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kmeans_initialization_method:
            ::std::option::Option<crate::schemas::TrainingOptionsKmeansInitializationMethod>,
        #[doc = "L1 regularization coefficient."]
        #[serde(
            rename = "l1Regularization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub l_1_regularization: ::std::option::Option<f64>,
        #[doc = "L2 regularization coefficient."]
        #[serde(
            rename = "l2Regularization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub l_2_regularization: ::std::option::Option<f64>,
        #[doc = "Weights associated with each label class, for rebalancing the training data. Only applicable for classification models."]
        #[serde(
            rename = "labelClassWeights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_class_weights: ::std::option::Option<::std::collections::BTreeMap<String, f64>>,
        #[doc = "Learning rate in training. Used only for iterative training algorithms."]
        #[serde(
            rename = "learnRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_rate: ::std::option::Option<f64>,
        #[doc = "The strategy to determine learn rate for the current iteration."]
        #[serde(
            rename = "learnRateStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_rate_strategy:
            ::std::option::Option<crate::schemas::TrainingOptionsLearnRateStrategy>,
        #[doc = "Type of loss function used during training run."]
        #[serde(
            rename = "lossType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub loss_type: ::std::option::Option<crate::schemas::TrainingOptionsLossType>,
        #[doc = "The maximum number of iterations in training. Used only for iterative training algorithms."]
        #[serde(
            rename = "maxIterations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_iterations: ::std::option::Option<i64>,
        #[doc = "Maximum depth of a tree for boosted tree models."]
        #[serde(
            rename = "maxTreeDepth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_tree_depth: ::std::option::Option<i64>,
        #[doc = "When early_stop is true, stops training when accuracy improvement is less than 'min_relative_progress'. Used only for iterative training algorithms."]
        #[serde(
            rename = "minRelativeProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_relative_progress: ::std::option::Option<f64>,
        #[doc = "Minimum split loss for boosted tree models."]
        #[serde(
            rename = "minSplitLoss",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_split_loss: ::std::option::Option<f64>,
        #[doc = "Google Cloud Storage URI from which the model was imported. Only applicable for imported models."]
        #[serde(
            rename = "modelUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_uri: ::std::option::Option<String>,
        #[doc = "A specification of the non-seasonal part of the ARIMA model: the three components (p, d, q) are the AR order, the degree of differencing, and the MA order."]
        #[serde(
            rename = "nonSeasonalOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_seasonal_order: ::std::option::Option<crate::schemas::ArimaOrder>,
        #[doc = "Number of clusters for clustering models."]
        #[serde(
            rename = "numClusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_clusters: ::std::option::Option<i64>,
        #[doc = "Num factors specified for matrix factorization models."]
        #[serde(
            rename = "numFactors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_factors: ::std::option::Option<i64>,
        #[doc = "Optimization strategy for training linear regression models."]
        #[serde(
            rename = "optimizationStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub optimization_strategy:
            ::std::option::Option<crate::schemas::TrainingOptionsOptimizationStrategy>,
        #[doc = "Whether to preserve the input structs in output feature names. Suppose there is a struct A with field b. When false (default), the output feature name is A_b. When true, the output feature name is A.b."]
        #[serde(
            rename = "preserveInputStructs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preserve_input_structs: ::std::option::Option<bool>,
        #[doc = "Subsample fraction of the training data to grow tree to prevent overfitting for boosted tree models."]
        #[serde(
            rename = "subsample",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subsample: ::std::option::Option<f64>,
        #[doc = "Column to be designated as time series data for ARIMA model."]
        #[serde(
            rename = "timeSeriesDataColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_data_column: ::std::option::Option<String>,
        #[doc = "The time series id column that was used during ARIMA model training."]
        #[serde(
            rename = "timeSeriesIdColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_id_column: ::std::option::Option<String>,
        #[doc = "Column to be designated as time series timestamp for ARIMA model."]
        #[serde(
            rename = "timeSeriesTimestampColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_timestamp_column: ::std::option::Option<String>,
        #[doc = "User column specified for matrix factorization models."]
        #[serde(
            rename = "userColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_column: ::std::option::Option<String>,
        #[doc = "Hyperparameter for matrix factoration when implicit feedback type is specified."]
        #[serde(
            rename = "walsAlpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wals_alpha: ::std::option::Option<f64>,
        #[doc = "Whether to train a model from the last checkpoint."]
        #[serde(
            rename = "warmStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warm_start: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TrainingOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsDataFrequency {
        #[doc = "Automatically inferred from timestamps."]
        AutoFrequency,
        #[doc = "Daily data."]
        Daily,
        DataFrequencyUnspecified,
        #[doc = "Hourly data."]
        Hourly,
        #[doc = "Monthly data."]
        Monthly,
        #[doc = "Per-minute data."]
        PerMinute,
        #[doc = "Quarterly data."]
        Quarterly,
        #[doc = "Weekly data."]
        Weekly,
        #[doc = "Yearly data."]
        Yearly,
    }
    impl TrainingOptionsDataFrequency {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsDataFrequency::AutoFrequency => "AUTO_FREQUENCY",
                TrainingOptionsDataFrequency::Daily => "DAILY",
                TrainingOptionsDataFrequency::DataFrequencyUnspecified => {
                    "DATA_FREQUENCY_UNSPECIFIED"
                }
                TrainingOptionsDataFrequency::Hourly => "HOURLY",
                TrainingOptionsDataFrequency::Monthly => "MONTHLY",
                TrainingOptionsDataFrequency::PerMinute => "PER_MINUTE",
                TrainingOptionsDataFrequency::Quarterly => "QUARTERLY",
                TrainingOptionsDataFrequency::Weekly => "WEEKLY",
                TrainingOptionsDataFrequency::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsDataFrequency {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsDataFrequency {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsDataFrequency, ()> {
            Ok(match s {
                "AUTO_FREQUENCY" => TrainingOptionsDataFrequency::AutoFrequency,
                "DAILY" => TrainingOptionsDataFrequency::Daily,
                "DATA_FREQUENCY_UNSPECIFIED" => {
                    TrainingOptionsDataFrequency::DataFrequencyUnspecified
                }
                "HOURLY" => TrainingOptionsDataFrequency::Hourly,
                "MONTHLY" => TrainingOptionsDataFrequency::Monthly,
                "PER_MINUTE" => TrainingOptionsDataFrequency::PerMinute,
                "QUARTERLY" => TrainingOptionsDataFrequency::Quarterly,
                "WEEKLY" => TrainingOptionsDataFrequency::Weekly,
                "YEARLY" => TrainingOptionsDataFrequency::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsDataFrequency {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsDataFrequency {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsDataFrequency {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO_FREQUENCY" => TrainingOptionsDataFrequency::AutoFrequency,
                "DAILY" => TrainingOptionsDataFrequency::Daily,
                "DATA_FREQUENCY_UNSPECIFIED" => {
                    TrainingOptionsDataFrequency::DataFrequencyUnspecified
                }
                "HOURLY" => TrainingOptionsDataFrequency::Hourly,
                "MONTHLY" => TrainingOptionsDataFrequency::Monthly,
                "PER_MINUTE" => TrainingOptionsDataFrequency::PerMinute,
                "QUARTERLY" => TrainingOptionsDataFrequency::Quarterly,
                "WEEKLY" => TrainingOptionsDataFrequency::Weekly,
                "YEARLY" => TrainingOptionsDataFrequency::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsDataFrequency {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsDataFrequency {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsDataSplitMethod {
        #[doc = "Splits data automatically: Uses NO_SPLIT if the data size is small. Otherwise uses RANDOM."]
        AutoSplit,
        #[doc = "Splits data with the user provided tags."]
        Custom,
        DataSplitMethodUnspecified,
        #[doc = "Data split will be skipped."]
        NoSplit,
        #[doc = "Splits data randomly."]
        Random,
        #[doc = "Splits data sequentially."]
        Sequential,
    }
    impl TrainingOptionsDataSplitMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsDataSplitMethod::AutoSplit => "AUTO_SPLIT",
                TrainingOptionsDataSplitMethod::Custom => "CUSTOM",
                TrainingOptionsDataSplitMethod::DataSplitMethodUnspecified => {
                    "DATA_SPLIT_METHOD_UNSPECIFIED"
                }
                TrainingOptionsDataSplitMethod::NoSplit => "NO_SPLIT",
                TrainingOptionsDataSplitMethod::Random => "RANDOM",
                TrainingOptionsDataSplitMethod::Sequential => "SEQUENTIAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsDataSplitMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsDataSplitMethod {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsDataSplitMethod, ()> {
            Ok(match s {
                "AUTO_SPLIT" => TrainingOptionsDataSplitMethod::AutoSplit,
                "CUSTOM" => TrainingOptionsDataSplitMethod::Custom,
                "DATA_SPLIT_METHOD_UNSPECIFIED" => {
                    TrainingOptionsDataSplitMethod::DataSplitMethodUnspecified
                }
                "NO_SPLIT" => TrainingOptionsDataSplitMethod::NoSplit,
                "RANDOM" => TrainingOptionsDataSplitMethod::Random,
                "SEQUENTIAL" => TrainingOptionsDataSplitMethod::Sequential,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsDataSplitMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsDataSplitMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsDataSplitMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO_SPLIT" => TrainingOptionsDataSplitMethod::AutoSplit,
                "CUSTOM" => TrainingOptionsDataSplitMethod::Custom,
                "DATA_SPLIT_METHOD_UNSPECIFIED" => {
                    TrainingOptionsDataSplitMethod::DataSplitMethodUnspecified
                }
                "NO_SPLIT" => TrainingOptionsDataSplitMethod::NoSplit,
                "RANDOM" => TrainingOptionsDataSplitMethod::Random,
                "SEQUENTIAL" => TrainingOptionsDataSplitMethod::Sequential,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsDataSplitMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsDataSplitMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsDistanceType {
        #[doc = "Cosine distance."]
        Cosine,
        DistanceTypeUnspecified,
        #[doc = "Eculidean distance."]
        Euclidean,
    }
    impl TrainingOptionsDistanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsDistanceType::Cosine => "COSINE",
                TrainingOptionsDistanceType::DistanceTypeUnspecified => "DISTANCE_TYPE_UNSPECIFIED",
                TrainingOptionsDistanceType::Euclidean => "EUCLIDEAN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsDistanceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsDistanceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsDistanceType, ()> {
            Ok(match s {
                "COSINE" => TrainingOptionsDistanceType::Cosine,
                "DISTANCE_TYPE_UNSPECIFIED" => TrainingOptionsDistanceType::DistanceTypeUnspecified,
                "EUCLIDEAN" => TrainingOptionsDistanceType::Euclidean,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsDistanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsDistanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsDistanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COSINE" => TrainingOptionsDistanceType::Cosine,
                "DISTANCE_TYPE_UNSPECIFIED" => TrainingOptionsDistanceType::DistanceTypeUnspecified,
                "EUCLIDEAN" => TrainingOptionsDistanceType::Euclidean,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsDistanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsDistanceType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsFeedbackType {
        #[doc = "Use nonweighted-als for explicit feedback problems."]
        Explicit,
        FeedbackTypeUnspecified,
        #[doc = "Use weighted-als for implicit feedback problems."]
        Implicit,
    }
    impl TrainingOptionsFeedbackType {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsFeedbackType::Explicit => "EXPLICIT",
                TrainingOptionsFeedbackType::FeedbackTypeUnspecified => "FEEDBACK_TYPE_UNSPECIFIED",
                TrainingOptionsFeedbackType::Implicit => "IMPLICIT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsFeedbackType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsFeedbackType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsFeedbackType, ()> {
            Ok(match s {
                "EXPLICIT" => TrainingOptionsFeedbackType::Explicit,
                "FEEDBACK_TYPE_UNSPECIFIED" => TrainingOptionsFeedbackType::FeedbackTypeUnspecified,
                "IMPLICIT" => TrainingOptionsFeedbackType::Implicit,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsFeedbackType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsFeedbackType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsFeedbackType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXPLICIT" => TrainingOptionsFeedbackType::Explicit,
                "FEEDBACK_TYPE_UNSPECIFIED" => TrainingOptionsFeedbackType::FeedbackTypeUnspecified,
                "IMPLICIT" => TrainingOptionsFeedbackType::Implicit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsFeedbackType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsFeedbackType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsHolidayRegion {
        #[doc = "United Arab Emirates"]
        Ae,
        #[doc = "Argentina"]
        Ar,
        #[doc = "Austria"]
        At,
        #[doc = "Australia"]
        Au,
        #[doc = "Belgium"]
        Be,
        #[doc = "Brazil"]
        Br,
        #[doc = "Canada"]
        Ca,
        #[doc = "Switzerland"]
        Ch,
        #[doc = "Chile"]
        Cl,
        #[doc = "China"]
        Cn,
        #[doc = "Colombia"]
        Co,
        #[doc = "Czechoslovakia"]
        Cs,
        #[doc = "Czech Republic"]
        Cz,
        #[doc = "Germany"]
        De,
        #[doc = "Denmark"]
        Dk,
        #[doc = "Algeria"]
        Dz,
        #[doc = "Ecuador"]
        Ec,
        #[doc = "Estonia"]
        Ee,
        #[doc = "Egypt"]
        Eg,
        #[doc = "Europe, the Middle East and Africa."]
        Emea,
        #[doc = "Spain"]
        Es,
        #[doc = "Finland"]
        Fi,
        #[doc = "France"]
        Fr,
        #[doc = "Great Britain (United Kingdom)"]
        Gb,
        #[doc = "Global."]
        Global,
        #[doc = "Greece"]
        Gr,
        #[doc = "Hong Kong"]
        Hk,
        #[doc = "Holiday region unspecified."]
        HolidayRegionUnspecified,
        #[doc = "Hungary"]
        Hu,
        #[doc = "Indonesia"]
        Id,
        #[doc = "Ireland"]
        Ie,
        #[doc = "Israel"]
        Il,
        #[doc = "India"]
        In,
        #[doc = "Iran"]
        Ir,
        #[doc = "Italy"]
        It,
        #[doc = "Japan and Asia Pacific: Korea, Greater China, India, Australia, and New Zealand."]
        Japac,
        #[doc = "Japan"]
        Jp,
        #[doc = "Korea (South)"]
        Kr,
        #[doc = "Latin America and the Caribbean."]
        Lac,
        #[doc = "Latvia"]
        Lv,
        #[doc = "Morocco"]
        Ma,
        #[doc = "Mexico"]
        Mx,
        #[doc = "Malaysia"]
        My,
        #[doc = "North America."]
        Na,
        #[doc = "Nigeria"]
        Ng,
        #[doc = "Netherlands"]
        Nl,
        #[doc = "Norway"]
        No,
        #[doc = "New Zealand"]
        Nz,
        #[doc = "Peru"]
        Pe,
        #[doc = "Philippines"]
        Ph,
        #[doc = "Pakistan"]
        Pk,
        #[doc = "Poland"]
        Pl,
        #[doc = "Portugal"]
        Pt,
        #[doc = "Romania"]
        Ro,
        #[doc = "Serbia"]
        Rs,
        #[doc = "Russian Federation"]
        Ru,
        #[doc = "Saudi Arabia"]
        Sa,
        #[doc = "Sweden"]
        Se,
        #[doc = "Singapore"]
        Sg,
        #[doc = "Slovenia"]
        Si,
        #[doc = "Slovakia"]
        Sk,
        #[doc = "Thailand"]
        Th,
        #[doc = "Turkey"]
        Tr,
        #[doc = "Taiwan"]
        Tw,
        #[doc = "Ukraine"]
        Ua,
        #[doc = "United States"]
        Us,
        #[doc = "Venezuela"]
        Ve,
        #[doc = "Viet Nam"]
        Vn,
        #[doc = "South Africa"]
        Za,
    }
    impl TrainingOptionsHolidayRegion {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsHolidayRegion::Ae => "AE",
                TrainingOptionsHolidayRegion::Ar => "AR",
                TrainingOptionsHolidayRegion::At => "AT",
                TrainingOptionsHolidayRegion::Au => "AU",
                TrainingOptionsHolidayRegion::Be => "BE",
                TrainingOptionsHolidayRegion::Br => "BR",
                TrainingOptionsHolidayRegion::Ca => "CA",
                TrainingOptionsHolidayRegion::Ch => "CH",
                TrainingOptionsHolidayRegion::Cl => "CL",
                TrainingOptionsHolidayRegion::Cn => "CN",
                TrainingOptionsHolidayRegion::Co => "CO",
                TrainingOptionsHolidayRegion::Cs => "CS",
                TrainingOptionsHolidayRegion::Cz => "CZ",
                TrainingOptionsHolidayRegion::De => "DE",
                TrainingOptionsHolidayRegion::Dk => "DK",
                TrainingOptionsHolidayRegion::Dz => "DZ",
                TrainingOptionsHolidayRegion::Ec => "EC",
                TrainingOptionsHolidayRegion::Ee => "EE",
                TrainingOptionsHolidayRegion::Eg => "EG",
                TrainingOptionsHolidayRegion::Emea => "EMEA",
                TrainingOptionsHolidayRegion::Es => "ES",
                TrainingOptionsHolidayRegion::Fi => "FI",
                TrainingOptionsHolidayRegion::Fr => "FR",
                TrainingOptionsHolidayRegion::Gb => "GB",
                TrainingOptionsHolidayRegion::Global => "GLOBAL",
                TrainingOptionsHolidayRegion::Gr => "GR",
                TrainingOptionsHolidayRegion::Hk => "HK",
                TrainingOptionsHolidayRegion::HolidayRegionUnspecified => {
                    "HOLIDAY_REGION_UNSPECIFIED"
                }
                TrainingOptionsHolidayRegion::Hu => "HU",
                TrainingOptionsHolidayRegion::Id => "ID",
                TrainingOptionsHolidayRegion::Ie => "IE",
                TrainingOptionsHolidayRegion::Il => "IL",
                TrainingOptionsHolidayRegion::In => "IN",
                TrainingOptionsHolidayRegion::Ir => "IR",
                TrainingOptionsHolidayRegion::It => "IT",
                TrainingOptionsHolidayRegion::Japac => "JAPAC",
                TrainingOptionsHolidayRegion::Jp => "JP",
                TrainingOptionsHolidayRegion::Kr => "KR",
                TrainingOptionsHolidayRegion::Lac => "LAC",
                TrainingOptionsHolidayRegion::Lv => "LV",
                TrainingOptionsHolidayRegion::Ma => "MA",
                TrainingOptionsHolidayRegion::Mx => "MX",
                TrainingOptionsHolidayRegion::My => "MY",
                TrainingOptionsHolidayRegion::Na => "NA",
                TrainingOptionsHolidayRegion::Ng => "NG",
                TrainingOptionsHolidayRegion::Nl => "NL",
                TrainingOptionsHolidayRegion::No => "NO",
                TrainingOptionsHolidayRegion::Nz => "NZ",
                TrainingOptionsHolidayRegion::Pe => "PE",
                TrainingOptionsHolidayRegion::Ph => "PH",
                TrainingOptionsHolidayRegion::Pk => "PK",
                TrainingOptionsHolidayRegion::Pl => "PL",
                TrainingOptionsHolidayRegion::Pt => "PT",
                TrainingOptionsHolidayRegion::Ro => "RO",
                TrainingOptionsHolidayRegion::Rs => "RS",
                TrainingOptionsHolidayRegion::Ru => "RU",
                TrainingOptionsHolidayRegion::Sa => "SA",
                TrainingOptionsHolidayRegion::Se => "SE",
                TrainingOptionsHolidayRegion::Sg => "SG",
                TrainingOptionsHolidayRegion::Si => "SI",
                TrainingOptionsHolidayRegion::Sk => "SK",
                TrainingOptionsHolidayRegion::Th => "TH",
                TrainingOptionsHolidayRegion::Tr => "TR",
                TrainingOptionsHolidayRegion::Tw => "TW",
                TrainingOptionsHolidayRegion::Ua => "UA",
                TrainingOptionsHolidayRegion::Us => "US",
                TrainingOptionsHolidayRegion::Ve => "VE",
                TrainingOptionsHolidayRegion::Vn => "VN",
                TrainingOptionsHolidayRegion::Za => "ZA",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsHolidayRegion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsHolidayRegion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsHolidayRegion, ()> {
            Ok(match s {
                "AE" => TrainingOptionsHolidayRegion::Ae,
                "AR" => TrainingOptionsHolidayRegion::Ar,
                "AT" => TrainingOptionsHolidayRegion::At,
                "AU" => TrainingOptionsHolidayRegion::Au,
                "BE" => TrainingOptionsHolidayRegion::Be,
                "BR" => TrainingOptionsHolidayRegion::Br,
                "CA" => TrainingOptionsHolidayRegion::Ca,
                "CH" => TrainingOptionsHolidayRegion::Ch,
                "CL" => TrainingOptionsHolidayRegion::Cl,
                "CN" => TrainingOptionsHolidayRegion::Cn,
                "CO" => TrainingOptionsHolidayRegion::Co,
                "CS" => TrainingOptionsHolidayRegion::Cs,
                "CZ" => TrainingOptionsHolidayRegion::Cz,
                "DE" => TrainingOptionsHolidayRegion::De,
                "DK" => TrainingOptionsHolidayRegion::Dk,
                "DZ" => TrainingOptionsHolidayRegion::Dz,
                "EC" => TrainingOptionsHolidayRegion::Ec,
                "EE" => TrainingOptionsHolidayRegion::Ee,
                "EG" => TrainingOptionsHolidayRegion::Eg,
                "EMEA" => TrainingOptionsHolidayRegion::Emea,
                "ES" => TrainingOptionsHolidayRegion::Es,
                "FI" => TrainingOptionsHolidayRegion::Fi,
                "FR" => TrainingOptionsHolidayRegion::Fr,
                "GB" => TrainingOptionsHolidayRegion::Gb,
                "GLOBAL" => TrainingOptionsHolidayRegion::Global,
                "GR" => TrainingOptionsHolidayRegion::Gr,
                "HK" => TrainingOptionsHolidayRegion::Hk,
                "HOLIDAY_REGION_UNSPECIFIED" => {
                    TrainingOptionsHolidayRegion::HolidayRegionUnspecified
                }
                "HU" => TrainingOptionsHolidayRegion::Hu,
                "ID" => TrainingOptionsHolidayRegion::Id,
                "IE" => TrainingOptionsHolidayRegion::Ie,
                "IL" => TrainingOptionsHolidayRegion::Il,
                "IN" => TrainingOptionsHolidayRegion::In,
                "IR" => TrainingOptionsHolidayRegion::Ir,
                "IT" => TrainingOptionsHolidayRegion::It,
                "JAPAC" => TrainingOptionsHolidayRegion::Japac,
                "JP" => TrainingOptionsHolidayRegion::Jp,
                "KR" => TrainingOptionsHolidayRegion::Kr,
                "LAC" => TrainingOptionsHolidayRegion::Lac,
                "LV" => TrainingOptionsHolidayRegion::Lv,
                "MA" => TrainingOptionsHolidayRegion::Ma,
                "MX" => TrainingOptionsHolidayRegion::Mx,
                "MY" => TrainingOptionsHolidayRegion::My,
                "NA" => TrainingOptionsHolidayRegion::Na,
                "NG" => TrainingOptionsHolidayRegion::Ng,
                "NL" => TrainingOptionsHolidayRegion::Nl,
                "NO" => TrainingOptionsHolidayRegion::No,
                "NZ" => TrainingOptionsHolidayRegion::Nz,
                "PE" => TrainingOptionsHolidayRegion::Pe,
                "PH" => TrainingOptionsHolidayRegion::Ph,
                "PK" => TrainingOptionsHolidayRegion::Pk,
                "PL" => TrainingOptionsHolidayRegion::Pl,
                "PT" => TrainingOptionsHolidayRegion::Pt,
                "RO" => TrainingOptionsHolidayRegion::Ro,
                "RS" => TrainingOptionsHolidayRegion::Rs,
                "RU" => TrainingOptionsHolidayRegion::Ru,
                "SA" => TrainingOptionsHolidayRegion::Sa,
                "SE" => TrainingOptionsHolidayRegion::Se,
                "SG" => TrainingOptionsHolidayRegion::Sg,
                "SI" => TrainingOptionsHolidayRegion::Si,
                "SK" => TrainingOptionsHolidayRegion::Sk,
                "TH" => TrainingOptionsHolidayRegion::Th,
                "TR" => TrainingOptionsHolidayRegion::Tr,
                "TW" => TrainingOptionsHolidayRegion::Tw,
                "UA" => TrainingOptionsHolidayRegion::Ua,
                "US" => TrainingOptionsHolidayRegion::Us,
                "VE" => TrainingOptionsHolidayRegion::Ve,
                "VN" => TrainingOptionsHolidayRegion::Vn,
                "ZA" => TrainingOptionsHolidayRegion::Za,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsHolidayRegion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsHolidayRegion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsHolidayRegion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AE" => TrainingOptionsHolidayRegion::Ae,
                "AR" => TrainingOptionsHolidayRegion::Ar,
                "AT" => TrainingOptionsHolidayRegion::At,
                "AU" => TrainingOptionsHolidayRegion::Au,
                "BE" => TrainingOptionsHolidayRegion::Be,
                "BR" => TrainingOptionsHolidayRegion::Br,
                "CA" => TrainingOptionsHolidayRegion::Ca,
                "CH" => TrainingOptionsHolidayRegion::Ch,
                "CL" => TrainingOptionsHolidayRegion::Cl,
                "CN" => TrainingOptionsHolidayRegion::Cn,
                "CO" => TrainingOptionsHolidayRegion::Co,
                "CS" => TrainingOptionsHolidayRegion::Cs,
                "CZ" => TrainingOptionsHolidayRegion::Cz,
                "DE" => TrainingOptionsHolidayRegion::De,
                "DK" => TrainingOptionsHolidayRegion::Dk,
                "DZ" => TrainingOptionsHolidayRegion::Dz,
                "EC" => TrainingOptionsHolidayRegion::Ec,
                "EE" => TrainingOptionsHolidayRegion::Ee,
                "EG" => TrainingOptionsHolidayRegion::Eg,
                "EMEA" => TrainingOptionsHolidayRegion::Emea,
                "ES" => TrainingOptionsHolidayRegion::Es,
                "FI" => TrainingOptionsHolidayRegion::Fi,
                "FR" => TrainingOptionsHolidayRegion::Fr,
                "GB" => TrainingOptionsHolidayRegion::Gb,
                "GLOBAL" => TrainingOptionsHolidayRegion::Global,
                "GR" => TrainingOptionsHolidayRegion::Gr,
                "HK" => TrainingOptionsHolidayRegion::Hk,
                "HOLIDAY_REGION_UNSPECIFIED" => {
                    TrainingOptionsHolidayRegion::HolidayRegionUnspecified
                }
                "HU" => TrainingOptionsHolidayRegion::Hu,
                "ID" => TrainingOptionsHolidayRegion::Id,
                "IE" => TrainingOptionsHolidayRegion::Ie,
                "IL" => TrainingOptionsHolidayRegion::Il,
                "IN" => TrainingOptionsHolidayRegion::In,
                "IR" => TrainingOptionsHolidayRegion::Ir,
                "IT" => TrainingOptionsHolidayRegion::It,
                "JAPAC" => TrainingOptionsHolidayRegion::Japac,
                "JP" => TrainingOptionsHolidayRegion::Jp,
                "KR" => TrainingOptionsHolidayRegion::Kr,
                "LAC" => TrainingOptionsHolidayRegion::Lac,
                "LV" => TrainingOptionsHolidayRegion::Lv,
                "MA" => TrainingOptionsHolidayRegion::Ma,
                "MX" => TrainingOptionsHolidayRegion::Mx,
                "MY" => TrainingOptionsHolidayRegion::My,
                "NA" => TrainingOptionsHolidayRegion::Na,
                "NG" => TrainingOptionsHolidayRegion::Ng,
                "NL" => TrainingOptionsHolidayRegion::Nl,
                "NO" => TrainingOptionsHolidayRegion::No,
                "NZ" => TrainingOptionsHolidayRegion::Nz,
                "PE" => TrainingOptionsHolidayRegion::Pe,
                "PH" => TrainingOptionsHolidayRegion::Ph,
                "PK" => TrainingOptionsHolidayRegion::Pk,
                "PL" => TrainingOptionsHolidayRegion::Pl,
                "PT" => TrainingOptionsHolidayRegion::Pt,
                "RO" => TrainingOptionsHolidayRegion::Ro,
                "RS" => TrainingOptionsHolidayRegion::Rs,
                "RU" => TrainingOptionsHolidayRegion::Ru,
                "SA" => TrainingOptionsHolidayRegion::Sa,
                "SE" => TrainingOptionsHolidayRegion::Se,
                "SG" => TrainingOptionsHolidayRegion::Sg,
                "SI" => TrainingOptionsHolidayRegion::Si,
                "SK" => TrainingOptionsHolidayRegion::Sk,
                "TH" => TrainingOptionsHolidayRegion::Th,
                "TR" => TrainingOptionsHolidayRegion::Tr,
                "TW" => TrainingOptionsHolidayRegion::Tw,
                "UA" => TrainingOptionsHolidayRegion::Ua,
                "US" => TrainingOptionsHolidayRegion::Us,
                "VE" => TrainingOptionsHolidayRegion::Ve,
                "VN" => TrainingOptionsHolidayRegion::Vn,
                "ZA" => TrainingOptionsHolidayRegion::Za,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsHolidayRegion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsHolidayRegion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsKmeansInitializationMethod {
        #[doc = "Initializes the centroids using data specified in kmeans_initialization_column."]
        Custom,
        #[doc = "Unspecified initialization method."]
        KmeansInitializationMethodUnspecified,
        #[doc = "Initializes with kmeans++."]
        KmeansPlusPlus,
        #[doc = "Initializes the centroids randomly."]
        Random,
    }
    impl TrainingOptionsKmeansInitializationMethod {
        pub fn as_str(self) -> &'static str {
            match self { TrainingOptionsKmeansInitializationMethod :: Custom => "CUSTOM" , TrainingOptionsKmeansInitializationMethod :: KmeansInitializationMethodUnspecified => "KMEANS_INITIALIZATION_METHOD_UNSPECIFIED" , TrainingOptionsKmeansInitializationMethod :: KmeansPlusPlus => "KMEANS_PLUS_PLUS" , TrainingOptionsKmeansInitializationMethod :: Random => "RANDOM" , }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsKmeansInitializationMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsKmeansInitializationMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<TrainingOptionsKmeansInitializationMethod, ()> {
            Ok(match s {
                "CUSTOM" => TrainingOptionsKmeansInitializationMethod::Custom,
                "KMEANS_INITIALIZATION_METHOD_UNSPECIFIED" => {
                    TrainingOptionsKmeansInitializationMethod::KmeansInitializationMethodUnspecified
                }
                "KMEANS_PLUS_PLUS" => TrainingOptionsKmeansInitializationMethod::KmeansPlusPlus,
                "RANDOM" => TrainingOptionsKmeansInitializationMethod::Random,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsKmeansInitializationMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsKmeansInitializationMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsKmeansInitializationMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => TrainingOptionsKmeansInitializationMethod::Custom,
                "KMEANS_INITIALIZATION_METHOD_UNSPECIFIED" => {
                    TrainingOptionsKmeansInitializationMethod::KmeansInitializationMethodUnspecified
                }
                "KMEANS_PLUS_PLUS" => TrainingOptionsKmeansInitializationMethod::KmeansPlusPlus,
                "RANDOM" => TrainingOptionsKmeansInitializationMethod::Random,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsKmeansInitializationMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsKmeansInitializationMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsLearnRateStrategy {
        #[doc = "Use a constant learning rate."]
        Constant,
        LearnRateStrategyUnspecified,
        #[doc = "Use line search to determine learning rate."]
        LineSearch,
    }
    impl TrainingOptionsLearnRateStrategy {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsLearnRateStrategy::Constant => "CONSTANT",
                TrainingOptionsLearnRateStrategy::LearnRateStrategyUnspecified => {
                    "LEARN_RATE_STRATEGY_UNSPECIFIED"
                }
                TrainingOptionsLearnRateStrategy::LineSearch => "LINE_SEARCH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsLearnRateStrategy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsLearnRateStrategy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsLearnRateStrategy, ()> {
            Ok(match s {
                "CONSTANT" => TrainingOptionsLearnRateStrategy::Constant,
                "LEARN_RATE_STRATEGY_UNSPECIFIED" => {
                    TrainingOptionsLearnRateStrategy::LearnRateStrategyUnspecified
                }
                "LINE_SEARCH" => TrainingOptionsLearnRateStrategy::LineSearch,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsLearnRateStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsLearnRateStrategy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsLearnRateStrategy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONSTANT" => TrainingOptionsLearnRateStrategy::Constant,
                "LEARN_RATE_STRATEGY_UNSPECIFIED" => {
                    TrainingOptionsLearnRateStrategy::LearnRateStrategyUnspecified
                }
                "LINE_SEARCH" => TrainingOptionsLearnRateStrategy::LineSearch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsLearnRateStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsLearnRateStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsLossType {
        LossTypeUnspecified,
        #[doc = "Mean log loss, used for logistic regression."]
        MeanLogLoss,
        #[doc = "Mean squared loss, used for linear regression."]
        MeanSquaredLoss,
    }
    impl TrainingOptionsLossType {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsLossType::LossTypeUnspecified => "LOSS_TYPE_UNSPECIFIED",
                TrainingOptionsLossType::MeanLogLoss => "MEAN_LOG_LOSS",
                TrainingOptionsLossType::MeanSquaredLoss => "MEAN_SQUARED_LOSS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsLossType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsLossType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsLossType, ()> {
            Ok(match s {
                "LOSS_TYPE_UNSPECIFIED" => TrainingOptionsLossType::LossTypeUnspecified,
                "MEAN_LOG_LOSS" => TrainingOptionsLossType::MeanLogLoss,
                "MEAN_SQUARED_LOSS" => TrainingOptionsLossType::MeanSquaredLoss,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsLossType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsLossType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsLossType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOSS_TYPE_UNSPECIFIED" => TrainingOptionsLossType::LossTypeUnspecified,
                "MEAN_LOG_LOSS" => TrainingOptionsLossType::MeanLogLoss,
                "MEAN_SQUARED_LOSS" => TrainingOptionsLossType::MeanSquaredLoss,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TrainingOptionsLossType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsLossType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsOptimizationStrategy {
        #[doc = "Uses an iterative batch gradient descent algorithm."]
        BatchGradientDescent,
        #[doc = "Uses a normal equation to solve linear regression problem."]
        NormalEquation,
        OptimizationStrategyUnspecified,
    }
    impl TrainingOptionsOptimizationStrategy {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsOptimizationStrategy::BatchGradientDescent => {
                    "BATCH_GRADIENT_DESCENT"
                }
                TrainingOptionsOptimizationStrategy::NormalEquation => "NORMAL_EQUATION",
                TrainingOptionsOptimizationStrategy::OptimizationStrategyUnspecified => {
                    "OPTIMIZATION_STRATEGY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for TrainingOptionsOptimizationStrategy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TrainingOptionsOptimizationStrategy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TrainingOptionsOptimizationStrategy, ()> {
            Ok(match s {
                "BATCH_GRADIENT_DESCENT" => {
                    TrainingOptionsOptimizationStrategy::BatchGradientDescent
                }
                "NORMAL_EQUATION" => TrainingOptionsOptimizationStrategy::NormalEquation,
                "OPTIMIZATION_STRATEGY_UNSPECIFIED" => {
                    TrainingOptionsOptimizationStrategy::OptimizationStrategyUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TrainingOptionsOptimizationStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsOptimizationStrategy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsOptimizationStrategy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BATCH_GRADIENT_DESCENT" => {
                    TrainingOptionsOptimizationStrategy::BatchGradientDescent
                }
                "NORMAL_EQUATION" => TrainingOptionsOptimizationStrategy::NormalEquation,
                "OPTIMIZATION_STRATEGY_UNSPECIFIED" => {
                    TrainingOptionsOptimizationStrategy::OptimizationStrategyUnspecified
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
    impl ::google_field_selector::FieldSelector for TrainingOptionsOptimizationStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingOptionsOptimizationStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrainingRun {
        #[doc = "Data split result of the training run. Only set when the input data is actually split."]
        #[serde(
            rename = "dataSplitResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_split_result: ::std::option::Option<crate::schemas::DataSplitResult>,
        #[doc = "The evaluation metrics over training/eval data that were computed at the end of training."]
        #[serde(
            rename = "evaluationMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluation_metrics: ::std::option::Option<crate::schemas::EvaluationMetrics>,
        #[doc = "Global explanations for important features of the model. For multi-class models, there is one entry for each label class. For other models, there is only one entry in the list."]
        #[serde(
            rename = "globalExplanations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub global_explanations: ::std::option::Option<Vec<crate::schemas::GlobalExplanation>>,
        #[doc = "Output of each iteration run, results.size() <= max_iterations."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::IterationResult>>,
        #[doc = "The start time of this training run."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Options that were used for this training run, includes user specified and default options that were used."]
        #[serde(
            rename = "trainingOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_options: ::std::option::Option<crate::schemas::TrainingOptions>,
    }
    impl ::google_field_selector::FieldSelector for TrainingRun {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrainingRun {
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
    pub struct TransactionInfo {
        #[doc = "[Output-only] // [Alpha] Id of the transaction."]
        #[serde(
            rename = "transactionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TransactionInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TransactionInfo {
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
    pub struct UserDefinedFunctionResource {
        #[doc = "[Pick one] An inline resource that contains code for a user-defined function (UDF). Providing a inline code resource is equivalent to providing a URI for a file containing the same code."]
        #[serde(
            rename = "inlineCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_code: ::std::option::Option<String>,
        #[doc = "[Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UserDefinedFunctionResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserDefinedFunctionResource {
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
    pub struct ViewDefinition {
        #[doc = "[Required] A query that BigQuery executes when the view is referenced."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value."]
        #[serde(
            rename = "useLegacySql",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_legacy_sql: ::std::option::Option<bool>,
        #[doc = "Describes user-defined function resources used in the query."]
        #[serde(
            rename = "userDefinedFunctionResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_defined_function_resources:
            ::std::option::Option<Vec<crate::schemas::UserDefinedFunctionResource>>,
    }
    impl ::google_field_selector::FieldSelector for ViewDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ViewDefinition {
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
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
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
    #[doc = "Actions that can be performed on the datasets resource"]
    pub fn datasets(&self) -> crate::resources::datasets::DatasetsActions {
        crate::resources::datasets::DatasetsActions {
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
    #[doc = "Actions that can be performed on the models resource"]
    pub fn models(&self) -> crate::resources::models::ModelsActions {
        crate::resources::models::ModelsActions {
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
    #[doc = "Actions that can be performed on the routines resource"]
    pub fn routines(&self) -> crate::resources::routines::RoutinesActions {
        crate::resources::routines::RoutinesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the row_access_policies resource"]
    pub fn row_access_policies(
        &self,
    ) -> crate::resources::row_access_policies::RowAccessPoliciesActions {
        crate::resources::row_access_policies::RowAccessPoliciesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the tabledata resource"]
    pub fn tabledata(&self) -> crate::resources::tabledata::TabledataActions {
        crate::resources::tabledata::TabledataActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the tables resource"]
    pub fn tables(&self) -> crate::resources::tables::TablesActions {
        crate::resources::tables::TablesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod datasets {
        pub mod params {}
        pub struct DatasetsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DatasetsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name."]
            pub fn delete(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    delete_contents: None,
                }
            }
            #[doc = "Returns the dataset specified by datasetID."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                }
            }
            #[doc = "Creates a new empty dataset."]
            pub fn insert(
                &self,
                request: crate::schemas::Dataset,
                project_id: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Lists all datasets in the specified project to which you have been granted the READER dataset role."]
            pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    all: None,
                    filter: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics."]
            pub fn patch(
                &self,
                request: crate::schemas::Dataset,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                }
            }
            #[doc = "Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource."]
            pub fn update(
                &self,
                request: crate::schemas::Dataset,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                }
            }
        }
        #[doc = "Created via [DatasetsActions::delete()](struct.DatasetsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            delete_contents: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
            #[doc = "If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default is False"]
            pub fn delete_contents(mut self, value: bool) -> Self {
                self.delete_contents = Some(value);
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
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
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("deleteContents", &self.delete_contents)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [DatasetsActions::get()](struct.DatasetsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Dataset, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Dataset, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
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
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [DatasetsActions::insert()](struct.DatasetsActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Dataset,
            project_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Dataset, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Dataset, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [DatasetsActions::list()](struct.DatasetsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            all: Option<bool>,
            filter: Option<String>,
            max_results: Option<u32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Whether to list all datasets, including hidden ones"]
            pub fn all(mut self, value: bool) -> Self {
                self.all = Some(value);
                self
            }
            #[doc = "An expression for filtering the results of the request by label. The syntax is \"labels.<name>[:<value>]\". Multiple filters can be ANDed together by connecting with a space. Example: \"labels.department:receiving labels.active\". See Filtering datasets using labels for details."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The maximum number of results to return"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_datasets<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_datasets_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_datasets_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DatasetListDatasetsItems>
            {
                self.iter_datasets_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_datasets_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DatasetListDatasetsItems>
            {
                self.iter_datasets_with_fields(Some("*"))
            }
            pub fn iter_datasets_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "datasets").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "datasets")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::DatasetList> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::DatasetList> {
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
            ) -> Result<crate::schemas::DatasetList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DatasetList, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("all", &self.all)]);
                req = req.query(&[("filter", &self.filter)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
        #[doc = "Created via [DatasetsActions::patch()](struct.DatasetsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Dataset,
            project_id: String,
            dataset_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> PatchRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Dataset, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Dataset, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
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
                let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [DatasetsActions::update()](struct.DatasetsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Dataset,
            project_id: String,
            dataset_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Dataset, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Dataset, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
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
                let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod jobs {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListProjection {
                #[doc = "Includes all job data"]
                Full,
                #[doc = "Does not include the job configuration"]
                Minimal,
            }
            impl ListProjection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListProjection::Full => "full",
                        ListProjection::Minimal => "minimal",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListProjection {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListProjection {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListProjection, ()> {
                    Ok(match s {
                        "full" => ListProjection::Full,
                        "minimal" => ListProjection::Minimal,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListProjection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListProjection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListProjection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "full" => ListProjection::Full,
                        "minimal" => ListProjection::Minimal,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListProjection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListProjection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListStateFilterItems {
                #[doc = "Finished jobs"]
                Done,
                #[doc = "Pending jobs"]
                Pending,
                #[doc = "Running jobs"]
                Running,
            }
            impl ListStateFilterItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListStateFilterItems::Done => "done",
                        ListStateFilterItems::Pending => "pending",
                        ListStateFilterItems::Running => "running",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListStateFilterItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListStateFilterItems {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListStateFilterItems, ()> {
                    Ok(match s {
                        "done" => ListStateFilterItems::Done,
                        "pending" => ListStateFilterItems::Pending,
                        "running" => ListStateFilterItems::Running,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListStateFilterItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListStateFilterItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListStateFilterItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "done" => ListStateFilterItems::Done,
                        "pending" => ListStateFilterItems::Pending,
                        "running" => ListStateFilterItems::Running,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListStateFilterItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListStateFilterItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct JobsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> JobsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs."]
            pub fn cancel(
                &self,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> CancelRequestBuilder {
                CancelRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                    location: None,
                }
            }
            #[doc = "Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                    location: None,
                }
            }
            #[doc = "Retrieves the results of a query job."]
            pub fn get_query_results(
                &self,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> GetQueryResultsRequestBuilder {
                GetQueryResultsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                    location: None,
                    max_results: None,
                    page_token: None,
                    start_index: None,
                    timeout_ms: None,
                }
            }
            #[doc = "Starts a new asynchronous job. Requires the Can View project role."]
            pub fn insert(
                &self,
                request: crate::schemas::Job,
                project_id: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property."]
            pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    all_users: None,
                    max_creation_time: None,
                    max_results: None,
                    min_creation_time: None,
                    page_token: None,
                    parent_job_id: None,
                    projection: None,
                    state_filter: None,
                }
            }
            #[doc = "Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout."]
            pub fn query(
                &self,
                request: crate::schemas::QueryRequest,
                project_id: impl Into<String>,
            ) -> QueryRequestBuilder {
                QueryRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                }
            }
        }
        #[doc = "Created via [JobsActions::cancel()](struct.JobsActions.html#method.cancel)"]
        #[derive(Debug, Clone)]
        pub struct CancelRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            job_id: String,
            location: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> CancelRequestBuilder<'a> {
            #[doc = "The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
            pub fn location(mut self, value: impl Into<String>) -> Self {
                self.location = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::JobCancelResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::JobCancelResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/jobs/");
                {
                    let var_as_str = &self.job_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/cancel");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("location", &self.location)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            job_id: String,
            location: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
            pub fn location(mut self, value: impl Into<String>) -> Self {
                self.location = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            pub fn execute_with_default_fields(self) -> Result<crate::schemas::Job, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Job, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/jobs/");
                {
                    let var_as_str = &self.job_id;
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
                req = req.query(&[("location", &self.location)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [JobsActions::get_query_results()](struct.JobsActions.html#method.get_query_results)"]
        #[derive(Debug, Clone)]
        pub struct GetQueryResultsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            job_id: String,
            location: Option<String>,
            max_results: Option<u32>,
            page_token: Option<String>,
            start_index: Option<u64>,
            timeout_ms: Option<u32>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetQueryResultsRequestBuilder<'a> {
            #[doc = "The geographic location where the job should run. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
            pub fn location(mut self, value: impl Into<String>) -> Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "Maximum number of results to read"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Zero-based index of the starting row"]
            pub fn start_index(mut self, value: u64) -> Self {
                self.start_index = Some(value);
                self
            }
            #[doc = "How long to wait for the query to complete, in milliseconds, before returning. Default is 10 seconds. If the timeout passes before the job completes, the 'jobComplete' field in the response will be false"]
            pub fn timeout_ms(mut self, value: u32) -> Self {
                self.timeout_ms = Some(value);
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::GetQueryResultsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetQueryResultsResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/queries/");
                {
                    let var_as_str = &self.job_id;
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
                req = req.query(&[("location", &self.location)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("startIndex", &self.start_index)]);
                req = req.query(&[("timeoutMs", &self.timeout_ms)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [JobsActions::insert()](struct.JobsActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Job,
            project_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            fn _simple_upload_path(&self) -> String {
                let mut output = "https://bigquery.googleapis.com/".to_owned();
                output.push_str("upload/bigquery/v2/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/jobs");
                output
            }
            pub fn upload<T, R>(
                mut self,
                content: R,
                mime_type: ::mime::Mime,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                R: ::std::io::Read + ::std::io::Seek + Send + 'static,
            {
                let fields = ::google_field_selector::to_string::<T>();
                self.fields = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                let req = self._request(&self._simple_upload_path())?;
                let req = req.query(&[("uploadType", "multipart")]);
                use crate::multipart::{Part, RelatedMultiPart};
                let mut multipart = RelatedMultiPart::new();
                let request_json = ::serde_json::to_vec(&self.request)?;
                multipart.new_part(Part::new(
                    ::mime::APPLICATION_JSON,
                    Box::new(::std::io::Cursor::new(request_json)),
                ));
                multipart.new_part(Part::new(mime_type, Box::new(content)));
                let req = req.header(
                    ::reqwest::header::CONTENT_TYPE,
                    format!("multipart/related; boundary={}", multipart.boundary()),
                );
                let req = req.body(reqwest::blocking::Body::new(multipart.into_reader()));
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _resumable_upload_path(&self) -> String {
                let mut output = "https://bigquery.googleapis.com/".to_owned();
                output.push_str("resumable/upload/bigquery/v2/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/jobs");
                output
            }
            pub fn start_resumable_upload(
                self,
                mime_type: ::mime::Mime,
            ) -> Result<crate::ResumableUpload, crate::Error> {
                let req = self._request(&self._resumable_upload_path())?;
                let req = req.query(&[("uploadType", "resumable")]);
                let req = req.header(
                    ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                    mime_type.to_string(),
                );
                let req = req.json(&self.request);
                let resp = crate::error_from_response(req.send()?)?;
                let location_header =
                    resp.headers()
                        .get(::reqwest::header::LOCATION)
                        .ok_or_else(|| {
                            crate::Error::Other(
                                format!(
                                    "No LOCATION header returned when initiating resumable upload"
                                )
                                .into(),
                            )
                        })?;
                let upload_url = ::std::str::from_utf8(location_header.as_bytes())
                    .map_err(|_| {
                        crate::Error::Other(format!("Non UTF8 LOCATION header returned").into())
                    })?
                    .to_owned();
                Ok(crate::ResumableUpload::new(
                    self.reqwest.clone(),
                    upload_url,
                ))
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
            pub fn execute_with_default_fields(self) -> Result<crate::schemas::Job, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Job, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/jobs");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            all_users: Option<bool>,
            max_creation_time: Option<u64>,
            max_results: Option<u32>,
            min_creation_time: Option<u64>,
            page_token: Option<String>,
            parent_job_id: Option<String>,
            projection: Option<crate::resources::jobs::params::ListProjection>,
            state_filter: Option<Vec<crate::resources::jobs::params::ListStateFilterItems>>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Whether to display jobs owned by all users in the project. Default false"]
            pub fn all_users(mut self, value: bool) -> Self {
                self.all_users = Some(value);
                self
            }
            #[doc = "Max value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created before or at this timestamp are returned"]
            pub fn max_creation_time(mut self, value: u64) -> Self {
                self.max_creation_time = Some(value);
                self
            }
            #[doc = "Maximum number of results to return"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Min value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created after or at this timestamp are returned"]
            pub fn min_creation_time(mut self, value: u64) -> Self {
                self.min_creation_time = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "If set, retrieves only jobs whose parent is this job. Otherwise, retrieves only jobs which have no parent"]
            pub fn parent_job_id(mut self, value: impl Into<String>) -> Self {
                self.parent_job_id = Some(value.into());
                self
            }
            #[doc = "Restrict information returned to a set of selected fields"]
            pub fn projection(
                mut self,
                value: crate::resources::jobs::params::ListProjection,
            ) -> Self {
                self.projection = Some(value);
                self
            }
            #[doc = "Filter for job state"]
            pub fn state_filter(
                mut self,
                value: impl Into<Vec<crate::resources::jobs::params::ListStateFilterItems>>,
            ) -> Self {
                self.state_filter = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::JobListJobsItems> {
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::JobListJobsItems> {
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
            ) -> crate::iter::PageIter<Self, crate::schemas::JobList> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::JobList> {
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
            ) -> Result<crate::schemas::JobList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::JobList, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/jobs");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("allUsers", &self.all_users)]);
                req = req.query(&[("maxCreationTime", &self.max_creation_time)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("minCreationTime", &self.min_creation_time)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("parentJobId", &self.parent_job_id)]);
                req = req.query(&[("projection", &self.projection)]);
                for value in self.state_filter.iter().flatten() {
                    req = req.query(&[("stateFilter", value)]);
                }
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
        #[doc = "Created via [JobsActions::query()](struct.JobsActions.html#method.query)"]
        #[derive(Debug, Clone)]
        pub struct QueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::QueryRequest,
            project_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> QueryRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::QueryResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::QueryResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/queries");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod models {
        pub mod params {}
        pub struct ModelsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ModelsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes the model specified by modelId from the dataset."]
            pub fn delete(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                model_id: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    model_id: model_id.into(),
                }
            }
            #[doc = "Gets the specified model resource by model ID."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                model_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    model_id: model_id.into(),
                }
            }
            #[doc = "Lists all models in the specified dataset. Requires the READER dataset role."]
            pub fn list(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Patch specific fields in the specified model."]
            pub fn patch(
                &self,
                request: crate::schemas::Model,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                model_id: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    model_id: model_id.into(),
                }
            }
        }
        #[doc = "Created via [ModelsActions::delete()](struct.ModelsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            model_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/models/");
                {
                    let var_as_str = &self.model_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ModelsActions::get()](struct.ModelsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            model_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Model, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Model, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/models/");
                {
                    let var_as_str = &self.model_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ModelsActions::list()](struct.ModelsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            max_results: Option<u32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_models<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_models_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_models_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Model> {
                self.iter_models_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_models_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Model> {
                self.iter_models_with_fields(Some("*"))
            }
            pub fn iter_models_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "models").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "models")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListModelsResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListModelsResponse> {
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
            ) -> Result<crate::schemas::ListModelsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListModelsResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/models");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
        #[doc = "Created via [ModelsActions::patch()](struct.ModelsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Model,
            project_id: String,
            dataset_id: String,
            model_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> PatchRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Model, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Model, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/models/");
                {
                    let var_as_str = &self.model_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns the email address of the service account for your project used for interactions with Google Cloud KMS."]
            pub fn get_service_account(
                &self,
                project_id: impl Into<String>,
            ) -> GetServiceAccountRequestBuilder {
                GetServiceAccountRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Lists all projects to which you have been granted any project role."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [ProjectsActions::get_service_account()](struct.ProjectsActions.html#method.get_service_account)"]
        #[derive(Debug, Clone)]
        pub struct GetServiceAccountRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetServiceAccountRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::GetServiceAccountResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetServiceAccountResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/serviceAccount");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ProjectsActions::list()](struct.ProjectsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            max_results: Option<u32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Maximum number of results to return"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_projects<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_projects_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_projects_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ProjectListProjectsItems>
            {
                self.iter_projects_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_projects_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ProjectListProjectsItems>
            {
                self.iter_projects_with_fields(Some("*"))
            }
            pub fn iter_projects_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "projects").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "projects")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ProjectList> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ProjectList> {
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
            ) -> Result<crate::schemas::ProjectList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ProjectList, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
    pub mod routines {
        pub mod params {}
        pub struct RoutinesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> RoutinesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes the routine specified by routineId from the dataset."]
            pub fn delete(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                routine_id: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    routine_id: routine_id.into(),
                }
            }
            #[doc = "Gets the specified routine resource by routine ID."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                routine_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    routine_id: routine_id.into(),
                    read_mask: None,
                }
            }
            #[doc = "Creates a new routine in the dataset."]
            pub fn insert(
                &self,
                request: crate::schemas::Routine,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                }
            }
            #[doc = "Lists all routines in the specified dataset. Requires the READER dataset role."]
            pub fn list(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    filter: None,
                    max_results: None,
                    page_token: None,
                    read_mask: None,
                }
            }
            #[doc = "Updates information in an existing routine. The update method replaces the entire Routine resource."]
            pub fn update(
                &self,
                request: crate::schemas::Routine,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                routine_id: impl Into<String>,
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    routine_id: routine_id.into(),
                }
            }
        }
        #[doc = "Created via [RoutinesActions::delete()](struct.RoutinesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            routine_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/routines/");
                {
                    let var_as_str = &self.routine_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [RoutinesActions::get()](struct.RoutinesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            routine_id: String,
            read_mask: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "If set, only the Routine fields in the field mask are returned in the response. If unset, all Routine fields are returned."]
            pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                self.read_mask = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Routine, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Routine, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/routines/");
                {
                    let var_as_str = &self.routine_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("readMask", &self.read_mask)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [RoutinesActions::insert()](struct.RoutinesActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Routine,
            project_id: String,
            dataset_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Routine, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Routine, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/routines");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [RoutinesActions::list()](struct.RoutinesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            filter: Option<String>,
            max_results: Option<u32>,
            page_token: Option<String>,
            read_mask: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "If set, then only the Routines matching this filter are returned. The current supported form is either \"routine_type:\" or \"routineType:\", where is a RoutineType enum. Example: \"routineType:SCALAR_FUNCTION\"."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "If set, then only the Routine fields in the field mask, as well as project_id, dataset_id and routine_id, are returned in the response. If unset, then the following Routine fields are returned: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language."]
            pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                self.read_mask = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_routines<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_routines_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_routines_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Routine> {
                self.iter_routines_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_routines_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Routine> {
                self.iter_routines_with_fields(Some("*"))
            }
            pub fn iter_routines_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "routines").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "routines")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListRoutinesResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListRoutinesResponse> {
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
            ) -> Result<crate::schemas::ListRoutinesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListRoutinesResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/routines");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("filter", &self.filter)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("readMask", &self.read_mask)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
        #[doc = "Created via [RoutinesActions::update()](struct.RoutinesActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Routine,
            project_id: String,
            dataset_id: String,
            routine_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Routine, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Routine, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/routines/");
                {
                    let var_as_str = &self.routine_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod row_access_policies {
        pub mod params {}
        pub struct RowAccessPoliciesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> RowAccessPoliciesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
            pub fn get_iam_policy(
                &self,
                request: crate::schemas::GetIamPolicyRequest,
                resource: impl Into<String>,
            ) -> GetIamPolicyRequestBuilder {
                GetIamPolicyRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Lists all row access policies on the specified table."]
            pub fn list(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
            pub fn set_iam_policy(
                &self,
                request: crate::schemas::SetIamPolicyRequest,
                resource: impl Into<String>,
            ) -> SetIamPolicyRequestBuilder {
                SetIamPolicyRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning."]
            pub fn test_iam_permissions(
                &self,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: impl Into<String>,
            ) -> TestIamPermissionsRequestBuilder {
                TestIamPermissionsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    resource: resource.into(),
                }
            }
        }
        #[doc = "Created via [RowAccessPoliciesActions::get_iam_policy()](struct.RowAccessPoliciesActions.html#method.get_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct GetIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GetIamPolicyRequest,
            resource: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetIamPolicyRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Policy, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getIamPolicy");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [RowAccessPoliciesActions::list()](struct.RowAccessPoliciesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            table_id: String,
            page_size: Option<i32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_row_access_policies<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_row_access_policies_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_row_access_policies_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::RowAccessPolicy> {
                self.iter_row_access_policies_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_row_access_policies_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::RowAccessPolicy> {
                self.iter_row_access_policies_with_fields(Some("*"))
            }
            pub fn iter_row_access_policies_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "rowAccessPolicies").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "rowAccessPolicies")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListRowAccessPoliciesResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListRowAccessPoliciesResponse>
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
            ) -> Result<crate::schemas::ListRowAccessPoliciesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListRowAccessPoliciesResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/rowAccessPolicies");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
        #[doc = "Created via [RowAccessPoliciesActions::set_iam_policy()](struct.RowAccessPoliciesActions.html#method.set_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct SetIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SetIamPolicyRequest,
            resource: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> SetIamPolicyRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Policy, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":setIamPolicy");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [RowAccessPoliciesActions::test_iam_permissions()](struct.RowAccessPoliciesActions.html#method.test_iam_permissions)"]
        #[derive(Debug, Clone)]
        pub struct TestIamPermissionsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::TestIamPermissionsRequest,
            resource: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> TestIamPermissionsRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":testIamPermissions");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod tabledata {
        pub mod params {}
        pub struct TabledataActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> TabledataActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role."]
            pub fn insert_all(
                &self,
                request: crate::schemas::TableDataInsertAllRequest,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> InsertAllRequestBuilder {
                InsertAllRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                }
            }
            #[doc = "Retrieves table data from a specified set of rows. Requires the READER dataset role."]
            pub fn list(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                    max_results: None,
                    page_token: None,
                    selected_fields: None,
                    start_index: None,
                }
            }
        }
        #[doc = "Created via [TabledataActions::insert_all()](struct.TabledataActions.html#method.insert_all)"]
        #[derive(Debug, Clone)]
        pub struct InsertAllRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::TableDataInsertAllRequest,
            project_id: String,
            dataset_id: String,
            table_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertAllRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::TableDataInsertAllResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TableDataInsertAllResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/insertAll");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TabledataActions::list()](struct.TabledataActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            table_id: String,
            max_results: Option<u32>,
            page_token: Option<String>,
            selected_fields: Option<String>,
            start_index: Option<u64>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Maximum number of results to return"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, identifying the result set"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "List of fields to return (comma-separated). If unspecified, all fields are returned"]
            pub fn selected_fields(mut self, value: impl Into<String>) -> Self {
                self.selected_fields = Some(value.into());
                self
            }
            #[doc = "Zero-based index of the starting row to read"]
            pub fn start_index(mut self, value: u64) -> Self {
                self.start_index = Some(value);
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::TableDataList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TableDataList, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/data");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("selectedFields", &self.selected_fields)]);
                req = req.query(&[("startIndex", &self.start_index)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod tables {
        pub mod params {}
        pub struct TablesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> TablesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted."]
            pub fn delete(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                }
            }
            #[doc = "Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                    selected_fields: None,
                }
            }
            #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
            pub fn get_iam_policy(
                &self,
                request: crate::schemas::GetIamPolicyRequest,
                resource: impl Into<String>,
            ) -> GetIamPolicyRequestBuilder {
                GetIamPolicyRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Creates a new, empty table in the dataset."]
            pub fn insert(
                &self,
                request: crate::schemas::Table,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                }
            }
            #[doc = "Lists all tables in the specified dataset. Requires the READER dataset role."]
            pub fn list(
                &self,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics."]
            pub fn patch(
                &self,
                request: crate::schemas::Table,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                }
            }
            #[doc = "Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
            pub fn set_iam_policy(
                &self,
                request: crate::schemas::SetIamPolicyRequest,
                resource: impl Into<String>,
            ) -> SetIamPolicyRequestBuilder {
                SetIamPolicyRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning."]
            pub fn test_iam_permissions(
                &self,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: impl Into<String>,
            ) -> TestIamPermissionsRequestBuilder {
                TestIamPermissionsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    resource: resource.into(),
                }
            }
            #[doc = "Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource."]
            pub fn update(
                &self,
                request: crate::schemas::Table,
                project_id: impl Into<String>,
                dataset_id: impl Into<String>,
                table_id: impl Into<String>,
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    dataset_id: dataset_id.into(),
                    table_id: table_id.into(),
                }
            }
        }
        #[doc = "Created via [TablesActions::delete()](struct.TablesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            table_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
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
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::get()](struct.TablesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            table_id: String,
            selected_fields: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "List of fields to return (comma-separated). If unspecified, all fields are returned"]
            pub fn selected_fields(mut self, value: impl Into<String>) -> Self {
                self.selected_fields = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Table, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Table, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
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
                req = req.query(&[("selectedFields", &self.selected_fields)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::get_iam_policy()](struct.TablesActions.html#method.get_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct GetIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GetIamPolicyRequest,
            resource: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetIamPolicyRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Policy, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getIamPolicy");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::insert()](struct.TablesActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Table,
            project_id: String,
            dataset_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Table, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Table, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::list()](struct.TablesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_id: String,
            dataset_id: String,
            max_results: Option<u32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Maximum number of results to return"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Page token, returned by a previous call, to request the next page of results"]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_tables<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_tables_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_tables_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::TableListTablesItems> {
                self.iter_tables_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_tables_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::TableListTablesItems> {
                self.iter_tables_with_fields(Some("*"))
            }
            pub fn iter_tables_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "tables").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "tables")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::TableList> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::TableList> {
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
            ) -> Result<crate::schemas::TableList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TableList, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
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
        #[doc = "Created via [TablesActions::patch()](struct.TablesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Table,
            project_id: String,
            dataset_id: String,
            table_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> PatchRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Table, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Table, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
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
                let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::set_iam_policy()](struct.TablesActions.html#method.set_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct SetIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SetIamPolicyRequest,
            resource: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> SetIamPolicyRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Policy, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":setIamPolicy");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::test_iam_permissions()](struct.TablesActions.html#method.test_iam_permissions)"]
        #[derive(Debug, Clone)]
        pub struct TestIamPermissionsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::TestIamPermissionsRequest,
            resource: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> TestIamPermissionsRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":testIamPermissions");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [TablesActions::update()](struct.TablesActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Table,
            project_id: String,
            dataset_id: String,
            table_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::Table, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Table, crate::Error> {
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
                let mut output = "https://bigquery.googleapis.com/bigquery/v2/".to_owned();
                output.push_str("projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/datasets/");
                {
                    let var_as_str = &self.dataset_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tables/");
                {
                    let var_as_str = &self.table_id;
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
                let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("userIp", &self.user_ip)]);
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
pub struct ResumableUpload {
    reqwest: ::reqwest::blocking::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::blocking::Client, url: String) -> Self {
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
        let req = req.body(::reqwest::blocking::Body::sized(reader, content_length));
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
