#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [dashboards](resources/projects/dashboards/struct.DashboardsActions.html)\n        * [*create*](resources/projects/dashboards/struct.CreateRequestBuilder.html), [*delete*](resources/projects/dashboards/struct.DeleteRequestBuilder.html), [*get*](resources/projects/dashboards/struct.GetRequestBuilder.html), [*list*](resources/projects/dashboards/struct.ListRequestBuilder.html), [*patch*](resources/projects/dashboards/struct.PatchRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View and write monitoring data for all of your Google and third-party Cloud and API projects\n\n`https://www.googleapis.com/auth/monitoring`"]
    pub const MONITORING: &str = "https://www.googleapis.com/auth/monitoring";
    #[doc = "View monitoring data for all of your Google Cloud and third-party projects\n\n`https://www.googleapis.com/auth/monitoring.read`"]
    pub const MONITORING_READ: &str = "https://www.googleapis.com/auth/monitoring.read";
    #[doc = "Publish metric data to your Google Cloud projects\n\n`https://www.googleapis.com/auth/monitoring.write`"]
    pub const MONITORING_WRITE: &str = "https://www.googleapis.com/auth/monitoring.write";
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
    pub struct Aggregation {
        #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored."]
        #[serde(
            rename = "alignmentPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alignment_period: ::std::option::Option<String>,
        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
        #[serde(
            rename = "crossSeriesReducer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_series_reducer:
            ::std::option::Option<crate::schemas::AggregationCrossSeriesReducer>,
        #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
        #[serde(
            rename = "groupByFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_by_fields: ::std::option::Option<Vec<String>>,
        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
        #[serde(
            rename = "perSeriesAligner",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub per_series_aligner: ::std::option::Option<crate::schemas::AggregationPerSeriesAligner>,
    }
    impl ::google_field_selector::FieldSelector for Aggregation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Aggregation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AggregationCrossSeriesReducer {
        #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
        ReduceCount,
        #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
        ReduceCountFalse,
        #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
        ReduceCountTrue,
        #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
        ReduceFractionTrue,
        #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
        ReduceMax,
        #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
        ReduceMean,
        #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
        ReduceMin,
        #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
        ReduceNone,
        #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile05,
        #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile50,
        #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile95,
        #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile99,
        #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
        ReduceStddev,
        #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
        ReduceSum,
    }
    impl AggregationCrossSeriesReducer {
        pub fn as_str(self) -> &'static str {
            match self {
                AggregationCrossSeriesReducer::ReduceCount => "REDUCE_COUNT",
                AggregationCrossSeriesReducer::ReduceCountFalse => "REDUCE_COUNT_FALSE",
                AggregationCrossSeriesReducer::ReduceCountTrue => "REDUCE_COUNT_TRUE",
                AggregationCrossSeriesReducer::ReduceFractionTrue => "REDUCE_FRACTION_TRUE",
                AggregationCrossSeriesReducer::ReduceMax => "REDUCE_MAX",
                AggregationCrossSeriesReducer::ReduceMean => "REDUCE_MEAN",
                AggregationCrossSeriesReducer::ReduceMin => "REDUCE_MIN",
                AggregationCrossSeriesReducer::ReduceNone => "REDUCE_NONE",
                AggregationCrossSeriesReducer::ReducePercentile05 => "REDUCE_PERCENTILE_05",
                AggregationCrossSeriesReducer::ReducePercentile50 => "REDUCE_PERCENTILE_50",
                AggregationCrossSeriesReducer::ReducePercentile95 => "REDUCE_PERCENTILE_95",
                AggregationCrossSeriesReducer::ReducePercentile99 => "REDUCE_PERCENTILE_99",
                AggregationCrossSeriesReducer::ReduceStddev => "REDUCE_STDDEV",
                AggregationCrossSeriesReducer::ReduceSum => "REDUCE_SUM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AggregationCrossSeriesReducer {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AggregationCrossSeriesReducer {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AggregationCrossSeriesReducer, ()> {
            Ok(match s {
                "REDUCE_COUNT" => AggregationCrossSeriesReducer::ReduceCount,
                "REDUCE_COUNT_FALSE" => AggregationCrossSeriesReducer::ReduceCountFalse,
                "REDUCE_COUNT_TRUE" => AggregationCrossSeriesReducer::ReduceCountTrue,
                "REDUCE_FRACTION_TRUE" => AggregationCrossSeriesReducer::ReduceFractionTrue,
                "REDUCE_MAX" => AggregationCrossSeriesReducer::ReduceMax,
                "REDUCE_MEAN" => AggregationCrossSeriesReducer::ReduceMean,
                "REDUCE_MIN" => AggregationCrossSeriesReducer::ReduceMin,
                "REDUCE_NONE" => AggregationCrossSeriesReducer::ReduceNone,
                "REDUCE_PERCENTILE_05" => AggregationCrossSeriesReducer::ReducePercentile05,
                "REDUCE_PERCENTILE_50" => AggregationCrossSeriesReducer::ReducePercentile50,
                "REDUCE_PERCENTILE_95" => AggregationCrossSeriesReducer::ReducePercentile95,
                "REDUCE_PERCENTILE_99" => AggregationCrossSeriesReducer::ReducePercentile99,
                "REDUCE_STDDEV" => AggregationCrossSeriesReducer::ReduceStddev,
                "REDUCE_SUM" => AggregationCrossSeriesReducer::ReduceSum,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AggregationCrossSeriesReducer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AggregationCrossSeriesReducer {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AggregationCrossSeriesReducer {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REDUCE_COUNT" => AggregationCrossSeriesReducer::ReduceCount,
                "REDUCE_COUNT_FALSE" => AggregationCrossSeriesReducer::ReduceCountFalse,
                "REDUCE_COUNT_TRUE" => AggregationCrossSeriesReducer::ReduceCountTrue,
                "REDUCE_FRACTION_TRUE" => AggregationCrossSeriesReducer::ReduceFractionTrue,
                "REDUCE_MAX" => AggregationCrossSeriesReducer::ReduceMax,
                "REDUCE_MEAN" => AggregationCrossSeriesReducer::ReduceMean,
                "REDUCE_MIN" => AggregationCrossSeriesReducer::ReduceMin,
                "REDUCE_NONE" => AggregationCrossSeriesReducer::ReduceNone,
                "REDUCE_PERCENTILE_05" => AggregationCrossSeriesReducer::ReducePercentile05,
                "REDUCE_PERCENTILE_50" => AggregationCrossSeriesReducer::ReducePercentile50,
                "REDUCE_PERCENTILE_95" => AggregationCrossSeriesReducer::ReducePercentile95,
                "REDUCE_PERCENTILE_99" => AggregationCrossSeriesReducer::ReducePercentile99,
                "REDUCE_STDDEV" => AggregationCrossSeriesReducer::ReduceStddev,
                "REDUCE_SUM" => AggregationCrossSeriesReducer::ReduceSum,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AggregationCrossSeriesReducer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregationCrossSeriesReducer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AggregationPerSeriesAligner {
        #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
        AlignCount,
        #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
        AlignCountFalse,
        #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
        AlignCountTrue,
        #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
        AlignDelta,
        #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
        AlignFractionTrue,
        #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignInterpolate,
        #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignMax,
        #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
        AlignMean,
        #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignMin,
        #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
        AlignNextOlder,
        #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
        AlignNone,
        #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentChange,
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile05,
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile50,
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile95,
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile99,
        #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
        AlignRate,
        #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
        AlignStddev,
        #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignSum,
    }
    impl AggregationPerSeriesAligner {
        pub fn as_str(self) -> &'static str {
            match self {
                AggregationPerSeriesAligner::AlignCount => "ALIGN_COUNT",
                AggregationPerSeriesAligner::AlignCountFalse => "ALIGN_COUNT_FALSE",
                AggregationPerSeriesAligner::AlignCountTrue => "ALIGN_COUNT_TRUE",
                AggregationPerSeriesAligner::AlignDelta => "ALIGN_DELTA",
                AggregationPerSeriesAligner::AlignFractionTrue => "ALIGN_FRACTION_TRUE",
                AggregationPerSeriesAligner::AlignInterpolate => "ALIGN_INTERPOLATE",
                AggregationPerSeriesAligner::AlignMax => "ALIGN_MAX",
                AggregationPerSeriesAligner::AlignMean => "ALIGN_MEAN",
                AggregationPerSeriesAligner::AlignMin => "ALIGN_MIN",
                AggregationPerSeriesAligner::AlignNextOlder => "ALIGN_NEXT_OLDER",
                AggregationPerSeriesAligner::AlignNone => "ALIGN_NONE",
                AggregationPerSeriesAligner::AlignPercentChange => "ALIGN_PERCENT_CHANGE",
                AggregationPerSeriesAligner::AlignPercentile05 => "ALIGN_PERCENTILE_05",
                AggregationPerSeriesAligner::AlignPercentile50 => "ALIGN_PERCENTILE_50",
                AggregationPerSeriesAligner::AlignPercentile95 => "ALIGN_PERCENTILE_95",
                AggregationPerSeriesAligner::AlignPercentile99 => "ALIGN_PERCENTILE_99",
                AggregationPerSeriesAligner::AlignRate => "ALIGN_RATE",
                AggregationPerSeriesAligner::AlignStddev => "ALIGN_STDDEV",
                AggregationPerSeriesAligner::AlignSum => "ALIGN_SUM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AggregationPerSeriesAligner {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AggregationPerSeriesAligner {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AggregationPerSeriesAligner, ()> {
            Ok(match s {
                "ALIGN_COUNT" => AggregationPerSeriesAligner::AlignCount,
                "ALIGN_COUNT_FALSE" => AggregationPerSeriesAligner::AlignCountFalse,
                "ALIGN_COUNT_TRUE" => AggregationPerSeriesAligner::AlignCountTrue,
                "ALIGN_DELTA" => AggregationPerSeriesAligner::AlignDelta,
                "ALIGN_FRACTION_TRUE" => AggregationPerSeriesAligner::AlignFractionTrue,
                "ALIGN_INTERPOLATE" => AggregationPerSeriesAligner::AlignInterpolate,
                "ALIGN_MAX" => AggregationPerSeriesAligner::AlignMax,
                "ALIGN_MEAN" => AggregationPerSeriesAligner::AlignMean,
                "ALIGN_MIN" => AggregationPerSeriesAligner::AlignMin,
                "ALIGN_NEXT_OLDER" => AggregationPerSeriesAligner::AlignNextOlder,
                "ALIGN_NONE" => AggregationPerSeriesAligner::AlignNone,
                "ALIGN_PERCENT_CHANGE" => AggregationPerSeriesAligner::AlignPercentChange,
                "ALIGN_PERCENTILE_05" => AggregationPerSeriesAligner::AlignPercentile05,
                "ALIGN_PERCENTILE_50" => AggregationPerSeriesAligner::AlignPercentile50,
                "ALIGN_PERCENTILE_95" => AggregationPerSeriesAligner::AlignPercentile95,
                "ALIGN_PERCENTILE_99" => AggregationPerSeriesAligner::AlignPercentile99,
                "ALIGN_RATE" => AggregationPerSeriesAligner::AlignRate,
                "ALIGN_STDDEV" => AggregationPerSeriesAligner::AlignStddev,
                "ALIGN_SUM" => AggregationPerSeriesAligner::AlignSum,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AggregationPerSeriesAligner {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AggregationPerSeriesAligner {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AggregationPerSeriesAligner {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALIGN_COUNT" => AggregationPerSeriesAligner::AlignCount,
                "ALIGN_COUNT_FALSE" => AggregationPerSeriesAligner::AlignCountFalse,
                "ALIGN_COUNT_TRUE" => AggregationPerSeriesAligner::AlignCountTrue,
                "ALIGN_DELTA" => AggregationPerSeriesAligner::AlignDelta,
                "ALIGN_FRACTION_TRUE" => AggregationPerSeriesAligner::AlignFractionTrue,
                "ALIGN_INTERPOLATE" => AggregationPerSeriesAligner::AlignInterpolate,
                "ALIGN_MAX" => AggregationPerSeriesAligner::AlignMax,
                "ALIGN_MEAN" => AggregationPerSeriesAligner::AlignMean,
                "ALIGN_MIN" => AggregationPerSeriesAligner::AlignMin,
                "ALIGN_NEXT_OLDER" => AggregationPerSeriesAligner::AlignNextOlder,
                "ALIGN_NONE" => AggregationPerSeriesAligner::AlignNone,
                "ALIGN_PERCENT_CHANGE" => AggregationPerSeriesAligner::AlignPercentChange,
                "ALIGN_PERCENTILE_05" => AggregationPerSeriesAligner::AlignPercentile05,
                "ALIGN_PERCENTILE_50" => AggregationPerSeriesAligner::AlignPercentile50,
                "ALIGN_PERCENTILE_95" => AggregationPerSeriesAligner::AlignPercentile95,
                "ALIGN_PERCENTILE_99" => AggregationPerSeriesAligner::AlignPercentile99,
                "ALIGN_RATE" => AggregationPerSeriesAligner::AlignRate,
                "ALIGN_STDDEV" => AggregationPerSeriesAligner::AlignStddev,
                "ALIGN_SUM" => AggregationPerSeriesAligner::AlignSum,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AggregationPerSeriesAligner {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregationPerSeriesAligner {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Axis {
        #[doc = "The label of the axis."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The axis scale. By default, a linear scale is used."]
        #[serde(
            rename = "scale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale: ::std::option::Option<crate::schemas::AxisScale>,
    }
    impl ::google_field_selector::FieldSelector for Axis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Axis {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AxisScale {
        #[doc = "Linear scale."]
        Linear,
        #[doc = "Logarithmic scale (base 10)."]
        Log10,
        #[doc = "Scale is unspecified. The view will default to LINEAR."]
        ScaleUnspecified,
    }
    impl AxisScale {
        pub fn as_str(self) -> &'static str {
            match self {
                AxisScale::Linear => "LINEAR",
                AxisScale::Log10 => "LOG10",
                AxisScale::ScaleUnspecified => "SCALE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AxisScale {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AxisScale {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AxisScale, ()> {
            Ok(match s {
                "LINEAR" => AxisScale::Linear,
                "LOG10" => AxisScale::Log10,
                "SCALE_UNSPECIFIED" => AxisScale::ScaleUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AxisScale {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AxisScale {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AxisScale {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINEAR" => AxisScale::Linear,
                "LOG10" => AxisScale::Log10,
                "SCALE_UNSPECIFIED" => AxisScale::ScaleUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AxisScale {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AxisScale {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ChartOptions {
        #[doc = "The chart mode."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::ChartOptionsMode>,
    }
    impl ::google_field_selector::FieldSelector for ChartOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ChartOptionsMode {
        #[doc = "The chart distinguishes data series using different color. Line colors may get reused when there are many lines in the chart."]
        Color,
        #[doc = "Mode is unspecified. The view will default to COLOR."]
        ModeUnspecified,
        #[doc = "The chart displays statistics such as average, median, 95th percentile, and more."]
        Stats,
        #[doc = "The chart uses the Stackdriver x-ray mode, in which each data set is plotted using the same semi-transparent color."]
        XRay,
    }
    impl ChartOptionsMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ChartOptionsMode::Color => "COLOR",
                ChartOptionsMode::ModeUnspecified => "MODE_UNSPECIFIED",
                ChartOptionsMode::Stats => "STATS",
                ChartOptionsMode::XRay => "X_RAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ChartOptionsMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ChartOptionsMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ChartOptionsMode, ()> {
            Ok(match s {
                "COLOR" => ChartOptionsMode::Color,
                "MODE_UNSPECIFIED" => ChartOptionsMode::ModeUnspecified,
                "STATS" => ChartOptionsMode::Stats,
                "X_RAY" => ChartOptionsMode::XRay,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ChartOptionsMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ChartOptionsMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChartOptionsMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLOR" => ChartOptionsMode::Color,
                "MODE_UNSPECIFIED" => ChartOptionsMode::ModeUnspecified,
                "STATS" => ChartOptionsMode::Stats,
                "X_RAY" => ChartOptionsMode::XRay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ChartOptionsMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartOptionsMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Column {
        #[doc = "The relative weight of this column. The column weight is used to adjust the width of columns on the screen (relative to peers). Greater the weight, greater the width of the column on the screen. If omitted, a value of 1 is used while rendering."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub weight: ::std::option::Option<i64>,
        #[doc = "The display widgets arranged vertically in this column."]
        #[serde(
            rename = "widgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub widgets: ::std::option::Option<Vec<crate::schemas::Widget>>,
    }
    impl ::google_field_selector::FieldSelector for Column {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Column {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColumnLayout {
        #[doc = "The columns of content to display."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<Vec<crate::schemas::Column>>,
    }
    impl ::google_field_selector::FieldSelector for ColumnLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ColumnLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Dashboard {
        #[doc = "The content is divided into equally spaced columns and the widgets are arranged vertically."]
        #[serde(
            rename = "columnLayout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_layout: ::std::option::Option<crate::schemas::ColumnLayout>,
        #[doc = "Required. The mutable, human-readable name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. An etag is returned in the response to GetDashboard, and users are expected to put that etag in the request to UpdateDashboard to ensure that their change will be applied to the same version of the Dashboard configuration. The field should not be passed during dashboard creation."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Content is arranged with a basic layout that re-flows a simple list of informational elements like widgets or tiles."]
        #[serde(
            rename = "gridLayout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub grid_layout: ::std::option::Option<crate::schemas::GridLayout>,
        #[doc = "Immutable. The resource name of the dashboard."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The content is divided into equally spaced rows and the widgets are arranged horizontally."]
        #[serde(
            rename = "rowLayout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_layout: ::std::option::Option<crate::schemas::RowLayout>,
    }
    impl ::google_field_selector::FieldSelector for Dashboard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dashboard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataSet {
        #[doc = "A template string for naming TimeSeries in the resulting data set. This should be a string with interpolations of the form ${label_name}, which will resolve to the label's value."]
        #[serde(
            rename = "legendTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legend_template: ::std::option::Option<String>,
        #[doc = "Optional. The lower bound on data point frequency for this data set, implemented by specifying the minimum alignment period to use in a time series query For example, if the data is published once every 10 minutes, the min_alignment_period should be at least 10 minutes. It would not make sense to fetch and align data at one minute intervals."]
        #[serde(
            rename = "minAlignmentPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_alignment_period: ::std::option::Option<String>,
        #[doc = "How this data should be plotted on the chart."]
        #[serde(
            rename = "plotType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plot_type: ::std::option::Option<crate::schemas::DataSetPlotType>,
        #[doc = "Required. Fields for querying time series data from the Stackdriver metrics API."]
        #[serde(
            rename = "timeSeriesQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_query: ::std::option::Option<crate::schemas::TimeSeriesQuery>,
    }
    impl ::google_field_selector::FieldSelector for DataSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataSetPlotType {
        #[doc = "The data is plotted as a heatmap. The series being plotted must have a DISTRIBUTION value type. The value of each bucket in the distribution is displayed as a color. This type is not currently available in the Stackdriver Monitoring application."]
        Heatmap,
        #[doc = "The data is plotted as a set of lines (one line per series)."]
        Line,
        #[doc = "Plot type is unspecified. The view will default to LINE."]
        PlotTypeUnspecified,
        #[doc = "The data is plotted as a set of filled areas (one area per series), with the areas stacked vertically (the base of each area is the top of its predecessor, and the base of the first area is the X axis). Since the areas do not overlap, each is filled with a different opaque color."]
        StackedArea,
        #[doc = "The data is plotted as a set of rectangular boxes (one box per series), with the boxes stacked vertically (the base of each box is the top of its predecessor, and the base of the first box is the X axis). Since the boxes do not overlap, each is filled with a different opaque color."]
        StackedBar,
    }
    impl DataSetPlotType {
        pub fn as_str(self) -> &'static str {
            match self {
                DataSetPlotType::Heatmap => "HEATMAP",
                DataSetPlotType::Line => "LINE",
                DataSetPlotType::PlotTypeUnspecified => "PLOT_TYPE_UNSPECIFIED",
                DataSetPlotType::StackedArea => "STACKED_AREA",
                DataSetPlotType::StackedBar => "STACKED_BAR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DataSetPlotType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DataSetPlotType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DataSetPlotType, ()> {
            Ok(match s {
                "HEATMAP" => DataSetPlotType::Heatmap,
                "LINE" => DataSetPlotType::Line,
                "PLOT_TYPE_UNSPECIFIED" => DataSetPlotType::PlotTypeUnspecified,
                "STACKED_AREA" => DataSetPlotType::StackedArea,
                "STACKED_BAR" => DataSetPlotType::StackedBar,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DataSetPlotType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataSetPlotType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataSetPlotType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HEATMAP" => DataSetPlotType::Heatmap,
                "LINE" => DataSetPlotType::Line,
                "PLOT_TYPE_UNSPECIFIED" => DataSetPlotType::PlotTypeUnspecified,
                "STACKED_AREA" => DataSetPlotType::StackedArea,
                "STACKED_BAR" => DataSetPlotType::StackedBar,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DataSetPlotType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSetPlotType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DroppedLabels {
        #[doc = "Map from label to its value, for all labels dropped in any aggregation."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for DroppedLabels {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DroppedLabels {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Field {
        #[doc = "The field cardinality."]
        #[serde(
            rename = "cardinality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cardinality: ::std::option::Option<crate::schemas::FieldCardinality>,
        #[doc = "The string value of the default value of this field. Proto2 syntax only."]
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_value: ::std::option::Option<String>,
        #[doc = "The field JSON name."]
        #[serde(
            rename = "jsonName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_name: ::std::option::Option<String>,
        #[doc = "The field type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::FieldKind>,
        #[doc = "The field name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The field number."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "The index of the field type in Type.oneofs, for message or enumeration types. The first type has index 1; zero means the type is not in the list."]
        #[serde(
            rename = "oneofIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oneof_index: ::std::option::Option<i32>,
        #[doc = "The protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Whether to use alternative packed wire representation."]
        #[serde(
            rename = "packed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packed: ::std::option::Option<bool>,
        #[doc = "The field type URL, without the scheme, for message or enumeration types. Example: \"type.googleapis.com/google.protobuf.Timestamp\"."]
        #[serde(
            rename = "typeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Field {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Field {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldCardinality {
        #[doc = "For optional fields."]
        CardinalityOptional,
        #[doc = "For repeated fields."]
        CardinalityRepeated,
        #[doc = "For required fields. Proto2 syntax only."]
        CardinalityRequired,
        #[doc = "For fields with unknown cardinality."]
        CardinalityUnknown,
    }
    impl FieldCardinality {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldCardinality::CardinalityOptional => "CARDINALITY_OPTIONAL",
                FieldCardinality::CardinalityRepeated => "CARDINALITY_REPEATED",
                FieldCardinality::CardinalityRequired => "CARDINALITY_REQUIRED",
                FieldCardinality::CardinalityUnknown => "CARDINALITY_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FieldCardinality {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FieldCardinality {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FieldCardinality, ()> {
            Ok(match s {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FieldCardinality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldCardinality {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldCardinality {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FieldCardinality {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldCardinality {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldKind {
        #[doc = "Field type bool."]
        TypeBool,
        #[doc = "Field type bytes."]
        TypeBytes,
        #[doc = "Field type double."]
        TypeDouble,
        #[doc = "Field type enum."]
        TypeEnum,
        #[doc = "Field type fixed32."]
        TypeFixed32,
        #[doc = "Field type fixed64."]
        TypeFixed64,
        #[doc = "Field type float."]
        TypeFloat,
        #[doc = "Field type group. Proto2 syntax only, and deprecated."]
        TypeGroup,
        #[doc = "Field type int32."]
        TypeInt32,
        #[doc = "Field type int64."]
        TypeInt64,
        #[doc = "Field type message."]
        TypeMessage,
        #[doc = "Field type sfixed32."]
        TypeSfixed32,
        #[doc = "Field type sfixed64."]
        TypeSfixed64,
        #[doc = "Field type sint32."]
        TypeSint32,
        #[doc = "Field type sint64."]
        TypeSint64,
        #[doc = "Field type string."]
        TypeString,
        #[doc = "Field type uint32."]
        TypeUint32,
        #[doc = "Field type uint64."]
        TypeUint64,
        #[doc = "Field type unknown."]
        TypeUnknown,
    }
    impl FieldKind {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldKind::TypeBool => "TYPE_BOOL",
                FieldKind::TypeBytes => "TYPE_BYTES",
                FieldKind::TypeDouble => "TYPE_DOUBLE",
                FieldKind::TypeEnum => "TYPE_ENUM",
                FieldKind::TypeFixed32 => "TYPE_FIXED32",
                FieldKind::TypeFixed64 => "TYPE_FIXED64",
                FieldKind::TypeFloat => "TYPE_FLOAT",
                FieldKind::TypeGroup => "TYPE_GROUP",
                FieldKind::TypeInt32 => "TYPE_INT32",
                FieldKind::TypeInt64 => "TYPE_INT64",
                FieldKind::TypeMessage => "TYPE_MESSAGE",
                FieldKind::TypeSfixed32 => "TYPE_SFIXED32",
                FieldKind::TypeSfixed64 => "TYPE_SFIXED64",
                FieldKind::TypeSint32 => "TYPE_SINT32",
                FieldKind::TypeSint64 => "TYPE_SINT64",
                FieldKind::TypeString => "TYPE_STRING",
                FieldKind::TypeUint32 => "TYPE_UINT32",
                FieldKind::TypeUint64 => "TYPE_UINT64",
                FieldKind::TypeUnknown => "TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FieldKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FieldKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FieldKind, ()> {
            Ok(match s {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FieldKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FieldKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GaugeView {
        #[doc = "The lower bound for this gauge chart. The value of the chart should always be greater than or equal to this."]
        #[serde(
            rename = "lowerBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lower_bound: ::std::option::Option<f64>,
        #[doc = "The upper bound for this gauge chart. The value of the chart should always be less than or equal to this."]
        #[serde(
            rename = "upperBound",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upper_bound: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GaugeView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GaugeView {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GridLayout {
        #[doc = "The number of columns into which the view's width is divided. If omitted or set to zero, a system default will be used while rendering."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub columns: ::std::option::Option<i64>,
        #[doc = "The informational elements that are arranged into the columns row-first."]
        #[serde(
            rename = "widgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub widgets: ::std::option::Option<Vec<crate::schemas::Widget>>,
    }
    impl ::google_field_selector::FieldSelector for GridLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GridLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListDashboardsResponse {
        #[doc = "The list of requested dashboards."]
        #[serde(
            rename = "dashboards",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dashboards: ::std::option::Option<Vec<crate::schemas::Dashboard>>,
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListDashboardsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDashboardsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Option {
        #[doc = "The option's name. For protobuf built-in options (options defined in descriptor.proto), this is the short name. For example, \"map_entry\". For custom options, it should be the fully-qualified name. For example, \"google.api.http\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The option's value packed in an Any message. If the value is a primitive, the corresponding wrapper type defined in google/protobuf/wrappers.proto should be used. If the value is an enum, it should be stored as an int32 value using the google.protobuf.Int32Value type."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Option {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Option {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PickTimeSeriesFilter {
        #[doc = "How to use the ranking to select time series that pass through the filter."]
        #[serde(
            rename = "direction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direction: ::std::option::Option<crate::schemas::PickTimeSeriesFilterDirection>,
        #[doc = "How many time series to allow to pass through the filter."]
        #[serde(
            rename = "numTimeSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_time_series: ::std::option::Option<i32>,
        #[doc = "ranking_method is applied to each time series independently to produce the value which will be used to compare the time series to other time series."]
        #[serde(
            rename = "rankingMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ranking_method:
            ::std::option::Option<crate::schemas::PickTimeSeriesFilterRankingMethod>,
    }
    impl ::google_field_selector::FieldSelector for PickTimeSeriesFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PickTimeSeriesFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PickTimeSeriesFilterDirection {
        #[doc = "Pass the lowest num_time_series ranking inputs."]
        Bottom,
        #[doc = "Not allowed. You must specify a different Direction if you specify a PickTimeSeriesFilter."]
        DirectionUnspecified,
        #[doc = "Pass the highest num_time_series ranking inputs."]
        Top,
    }
    impl PickTimeSeriesFilterDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                PickTimeSeriesFilterDirection::Bottom => "BOTTOM",
                PickTimeSeriesFilterDirection::DirectionUnspecified => "DIRECTION_UNSPECIFIED",
                PickTimeSeriesFilterDirection::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PickTimeSeriesFilterDirection {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PickTimeSeriesFilterDirection {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PickTimeSeriesFilterDirection, ()> {
            Ok(match s {
                "BOTTOM" => PickTimeSeriesFilterDirection::Bottom,
                "DIRECTION_UNSPECIFIED" => PickTimeSeriesFilterDirection::DirectionUnspecified,
                "TOP" => PickTimeSeriesFilterDirection::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PickTimeSeriesFilterDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PickTimeSeriesFilterDirection {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PickTimeSeriesFilterDirection {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM" => PickTimeSeriesFilterDirection::Bottom,
                "DIRECTION_UNSPECIFIED" => PickTimeSeriesFilterDirection::DirectionUnspecified,
                "TOP" => PickTimeSeriesFilterDirection::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PickTimeSeriesFilterDirection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PickTimeSeriesFilterDirection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PickTimeSeriesFilterRankingMethod {
        #[doc = "Select the most recent value."]
        MethodLatest,
        #[doc = "Select the maximum value."]
        MethodMax,
        #[doc = "Select the mean of all values."]
        MethodMean,
        #[doc = "Select the minimum value."]
        MethodMin,
        #[doc = "Compute the sum of all values."]
        MethodSum,
        #[doc = "Not allowed. You must specify a different Method if you specify a PickTimeSeriesFilter."]
        MethodUnspecified,
    }
    impl PickTimeSeriesFilterRankingMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                PickTimeSeriesFilterRankingMethod::MethodLatest => "METHOD_LATEST",
                PickTimeSeriesFilterRankingMethod::MethodMax => "METHOD_MAX",
                PickTimeSeriesFilterRankingMethod::MethodMean => "METHOD_MEAN",
                PickTimeSeriesFilterRankingMethod::MethodMin => "METHOD_MIN",
                PickTimeSeriesFilterRankingMethod::MethodSum => "METHOD_SUM",
                PickTimeSeriesFilterRankingMethod::MethodUnspecified => "METHOD_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PickTimeSeriesFilterRankingMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PickTimeSeriesFilterRankingMethod {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PickTimeSeriesFilterRankingMethod, ()> {
            Ok(match s {
                "METHOD_LATEST" => PickTimeSeriesFilterRankingMethod::MethodLatest,
                "METHOD_MAX" => PickTimeSeriesFilterRankingMethod::MethodMax,
                "METHOD_MEAN" => PickTimeSeriesFilterRankingMethod::MethodMean,
                "METHOD_MIN" => PickTimeSeriesFilterRankingMethod::MethodMin,
                "METHOD_SUM" => PickTimeSeriesFilterRankingMethod::MethodSum,
                "METHOD_UNSPECIFIED" => PickTimeSeriesFilterRankingMethod::MethodUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PickTimeSeriesFilterRankingMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PickTimeSeriesFilterRankingMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PickTimeSeriesFilterRankingMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METHOD_LATEST" => PickTimeSeriesFilterRankingMethod::MethodLatest,
                "METHOD_MAX" => PickTimeSeriesFilterRankingMethod::MethodMax,
                "METHOD_MEAN" => PickTimeSeriesFilterRankingMethod::MethodMean,
                "METHOD_MIN" => PickTimeSeriesFilterRankingMethod::MethodMin,
                "METHOD_SUM" => PickTimeSeriesFilterRankingMethod::MethodSum,
                "METHOD_UNSPECIFIED" => PickTimeSeriesFilterRankingMethod::MethodUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PickTimeSeriesFilterRankingMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PickTimeSeriesFilterRankingMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RatioPart {
        #[doc = "By default, the raw time series data is returned. Use this field to combine multiple time series for different views of the data."]
        #[serde(
            rename = "aggregation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregation: ::std::option::Option<crate::schemas::Aggregation>,
        #[doc = "Required. The monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies the metric types, resources, and projects to query."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RatioPart {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RatioPart {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Row {
        #[doc = "The relative weight of this row. The row weight is used to adjust the height of rows on the screen (relative to peers). Greater the weight, greater the height of the row on the screen. If omitted, a value of 1 is used while rendering."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub weight: ::std::option::Option<i64>,
        #[doc = "The display widgets arranged horizontally in this row."]
        #[serde(
            rename = "widgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub widgets: ::std::option::Option<Vec<crate::schemas::Widget>>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RowLayout {
        #[doc = "The rows of content to display."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
    }
    impl ::google_field_selector::FieldSelector for RowLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RowLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Scorecard {
        #[doc = "Will cause the scorecard to show a gauge chart."]
        #[serde(
            rename = "gaugeView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gauge_view: ::std::option::Option<crate::schemas::GaugeView>,
        #[doc = "Will cause the scorecard to show a spark chart."]
        #[serde(
            rename = "sparkChartView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spark_chart_view: ::std::option::Option<crate::schemas::SparkChartView>,
        #[doc = "The thresholds used to determine the state of the scorecard given the time series' current value. For an actual value x, the scorecard is in a danger state if x is less than or equal to a danger threshold that triggers below, or greater than or equal to a danger threshold that triggers above. Similarly, if x is above/below a warning threshold that triggers above/below, then the scorecard is in a warning state - unless x also puts it in a danger state. (Danger trumps warning.)As an example, consider a scorecard with the following four thresholds: {  value: 90,  category: 'DANGER',  trigger: 'ABOVE', }, {  value: 70,  category: 'WARNING',  trigger: 'ABOVE', }, {  value: 10,  category: 'DANGER',  trigger: 'BELOW', }, {  value: 20,  category: 'WARNING',  trigger: 'BELOW', }Then: values less than or equal to 10 would put the scorecard in a DANGER state, values greater than 10 but less than or equal to 20 a WARNING state, values strictly between 20 and 70 an OK state, values greater than or equal to 70 but less than 90 a WARNING state, and values greater than or equal to 90 a DANGER state."]
        #[serde(
            rename = "thresholds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thresholds: ::std::option::Option<Vec<crate::schemas::Threshold>>,
        #[doc = "Required. Fields for querying time series data from the Stackdriver metrics API."]
        #[serde(
            rename = "timeSeriesQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_query: ::std::option::Option<crate::schemas::TimeSeriesQuery>,
    }
    impl ::google_field_selector::FieldSelector for Scorecard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Scorecard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SourceContext {
        #[doc = "The path-qualified name of the .proto file that contained the associated protobuf element. For example: \"google/protobuf/source_context.proto\"."]
        #[serde(
            rename = "fileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SpanContext {
        #[doc = "The resource name of the span. The format is:\nprojects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID]\n[TRACE_ID] is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array.[SPAN_ID] is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array."]
        #[serde(
            rename = "spanName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub span_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpanContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpanContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SparkChartView {
        #[doc = "The lower bound on data point frequency in the chart implemented by specifying the minimum alignment period to use in a time series query. For example, if the data is published once every 10 minutes it would not make sense to fetch and align data at one minute intervals. This field is optional and exists only as a hint."]
        #[serde(
            rename = "minAlignmentPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_alignment_period: ::std::option::Option<String>,
        #[doc = "Required. The type of sparkchart to show in this chartView."]
        #[serde(
            rename = "sparkChartType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spark_chart_type: ::std::option::Option<crate::schemas::SparkChartViewSparkChartType>,
    }
    impl ::google_field_selector::FieldSelector for SparkChartView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SparkChartView {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SparkChartViewSparkChartType {
        #[doc = "The sparkbar will be rendered as a small bar chart."]
        SparkBar,
        #[doc = "Not allowed in well-formed requests."]
        SparkChartTypeUnspecified,
        #[doc = "The sparkline will be rendered as a small line chart."]
        SparkLine,
    }
    impl SparkChartViewSparkChartType {
        pub fn as_str(self) -> &'static str {
            match self {
                SparkChartViewSparkChartType::SparkBar => "SPARK_BAR",
                SparkChartViewSparkChartType::SparkChartTypeUnspecified => {
                    "SPARK_CHART_TYPE_UNSPECIFIED"
                }
                SparkChartViewSparkChartType::SparkLine => "SPARK_LINE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SparkChartViewSparkChartType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SparkChartViewSparkChartType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SparkChartViewSparkChartType, ()> {
            Ok(match s {
                "SPARK_BAR" => SparkChartViewSparkChartType::SparkBar,
                "SPARK_CHART_TYPE_UNSPECIFIED" => {
                    SparkChartViewSparkChartType::SparkChartTypeUnspecified
                }
                "SPARK_LINE" => SparkChartViewSparkChartType::SparkLine,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SparkChartViewSparkChartType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SparkChartViewSparkChartType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SparkChartViewSparkChartType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SPARK_BAR" => SparkChartViewSparkChartType::SparkBar,
                "SPARK_CHART_TYPE_UNSPECIFIED" => {
                    SparkChartViewSparkChartType::SparkChartTypeUnspecified
                }
                "SPARK_LINE" => SparkChartViewSparkChartType::SparkLine,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SparkChartViewSparkChartType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SparkChartViewSparkChartType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Text {
        #[doc = "The text content to be displayed."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "How the text content is formatted."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::TextFormat>,
    }
    impl ::google_field_selector::FieldSelector for Text {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Text {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TextFormat {
        #[doc = "Format is unspecified. Defaults to MARKDOWN."]
        FormatUnspecified,
        #[doc = "The text contains Markdown formatting."]
        Markdown,
        #[doc = "The text contains no special formatting."]
        Raw,
    }
    impl TextFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                TextFormat::FormatUnspecified => "FORMAT_UNSPECIFIED",
                TextFormat::Markdown => "MARKDOWN",
                TextFormat::Raw => "RAW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TextFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TextFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TextFormat, ()> {
            Ok(match s {
                "FORMAT_UNSPECIFIED" => TextFormat::FormatUnspecified,
                "MARKDOWN" => TextFormat::Markdown,
                "RAW" => TextFormat::Raw,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TextFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TextFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TextFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORMAT_UNSPECIFIED" => TextFormat::FormatUnspecified,
                "MARKDOWN" => TextFormat::Markdown,
                "RAW" => TextFormat::Raw,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TextFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Threshold {
        #[doc = "The state color for this threshold. Color is not allowed in a XyChart."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::ThresholdColor>,
        #[doc = "The direction for the current threshold. Direction is not allowed in a XyChart."]
        #[serde(
            rename = "direction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direction: ::std::option::Option<crate::schemas::ThresholdDirection>,
        #[doc = "A label for the threshold."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The value of the threshold. The value should be defined in the native scale of the metric."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Threshold {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Threshold {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThresholdColor {
        #[doc = "Color is unspecified. Not allowed in well-formed requests."]
        ColorUnspecified,
        #[doc = "Crossing the threshold is \"emergency\" behavior."]
        Red,
        #[doc = "Crossing the threshold is \"concerning\" behavior."]
        Yellow,
    }
    impl ThresholdColor {
        pub fn as_str(self) -> &'static str {
            match self {
                ThresholdColor::ColorUnspecified => "COLOR_UNSPECIFIED",
                ThresholdColor::Red => "RED",
                ThresholdColor::Yellow => "YELLOW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ThresholdColor {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ThresholdColor {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ThresholdColor, ()> {
            Ok(match s {
                "COLOR_UNSPECIFIED" => ThresholdColor::ColorUnspecified,
                "RED" => ThresholdColor::Red,
                "YELLOW" => ThresholdColor::Yellow,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ThresholdColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThresholdColor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThresholdColor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLOR_UNSPECIFIED" => ThresholdColor::ColorUnspecified,
                "RED" => ThresholdColor::Red,
                "YELLOW" => ThresholdColor::Yellow,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThresholdColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThresholdColor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThresholdDirection {
        #[doc = "The threshold will be considered crossed if the actual value is above the threshold value."]
        Above,
        #[doc = "The threshold will be considered crossed if the actual value is below the threshold value."]
        Below,
        #[doc = "Not allowed in well-formed requests."]
        DirectionUnspecified,
    }
    impl ThresholdDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                ThresholdDirection::Above => "ABOVE",
                ThresholdDirection::Below => "BELOW",
                ThresholdDirection::DirectionUnspecified => "DIRECTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ThresholdDirection {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ThresholdDirection {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ThresholdDirection, ()> {
            Ok(match s {
                "ABOVE" => ThresholdDirection::Above,
                "BELOW" => ThresholdDirection::Below,
                "DIRECTION_UNSPECIFIED" => ThresholdDirection::DirectionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ThresholdDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThresholdDirection {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThresholdDirection {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABOVE" => ThresholdDirection::Above,
                "BELOW" => ThresholdDirection::Below,
                "DIRECTION_UNSPECIFIED" => ThresholdDirection::DirectionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThresholdDirection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThresholdDirection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeSeriesFilter {
        #[doc = "By default, the raw time series data is returned. Use this field to combine multiple time series for different views of the data."]
        #[serde(
            rename = "aggregation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregation: ::std::option::Option<crate::schemas::Aggregation>,
        #[doc = "Required. The monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies the metric types, resources, and projects to query."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Ranking based time series filter."]
        #[serde(
            rename = "pickTimeSeriesFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pick_time_series_filter: ::std::option::Option<crate::schemas::PickTimeSeriesFilter>,
        #[doc = "Apply a second aggregation after aggregation is applied."]
        #[serde(
            rename = "secondaryAggregation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secondary_aggregation: ::std::option::Option<crate::schemas::Aggregation>,
    }
    impl ::google_field_selector::FieldSelector for TimeSeriesFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeSeriesFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeSeriesFilterRatio {
        #[doc = "The denominator of the ratio."]
        #[serde(
            rename = "denominator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub denominator: ::std::option::Option<crate::schemas::RatioPart>,
        #[doc = "The numerator of the ratio."]
        #[serde(
            rename = "numerator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numerator: ::std::option::Option<crate::schemas::RatioPart>,
        #[doc = "Ranking based time series filter."]
        #[serde(
            rename = "pickTimeSeriesFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pick_time_series_filter: ::std::option::Option<crate::schemas::PickTimeSeriesFilter>,
        #[doc = "Apply a second aggregation after the ratio is computed."]
        #[serde(
            rename = "secondaryAggregation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secondary_aggregation: ::std::option::Option<crate::schemas::Aggregation>,
    }
    impl ::google_field_selector::FieldSelector for TimeSeriesFilterRatio {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeSeriesFilterRatio {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeSeriesQuery {
        #[doc = "Filter parameters to fetch time series."]
        #[serde(
            rename = "timeSeriesFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_filter: ::std::option::Option<crate::schemas::TimeSeriesFilter>,
        #[doc = "Parameters to fetch a ratio between two time series filters."]
        #[serde(
            rename = "timeSeriesFilterRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_filter_ratio: ::std::option::Option<crate::schemas::TimeSeriesFilterRatio>,
        #[doc = "A query used to fetch time series."]
        #[serde(
            rename = "timeSeriesQueryLanguage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series_query_language: ::std::option::Option<String>,
        #[doc = "The unit of data contained in fetched time series. If non-empty, this unit will override any unit that accompanies fetched data. The format is the same as the unit (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors) field in MetricDescriptor."]
        #[serde(
            rename = "unitOverride",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit_override: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimeSeriesQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeSeriesQuery {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Type {
        #[doc = "The list of fields."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::Field>>,
        #[doc = "The fully qualified message name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The list of types appearing in oneof definitions in this type."]
        #[serde(
            rename = "oneofs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oneofs: ::std::option::Option<Vec<String>>,
        #[doc = "The protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::TypeSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Type {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Type {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TypeSyntax {
        #[doc = "Syntax proto2."]
        SyntaxProto2,
        #[doc = "Syntax proto3."]
        SyntaxProto3,
    }
    impl TypeSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                TypeSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                TypeSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TypeSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TypeSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TypeSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TypeSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TypeSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TypeSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TypeSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TypeSyntax {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Widget {
        #[doc = "A blank space."]
        #[serde(
            rename = "blank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blank: ::std::option::Option<crate::schemas::Empty>,
        #[doc = "A scorecard summarizing time series data."]
        #[serde(
            rename = "scorecard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scorecard: ::std::option::Option<crate::schemas::Scorecard>,
        #[doc = "A raw string or markdown displaying textual content."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<crate::schemas::Text>,
        #[doc = "Optional. The title of the widget."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "A chart of time series data."]
        #[serde(
            rename = "xyChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xy_chart: ::std::option::Option<crate::schemas::XyChart>,
    }
    impl ::google_field_selector::FieldSelector for Widget {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Widget {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct XyChart {
        #[doc = "Display options for the chart."]
        #[serde(
            rename = "chartOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_options: ::std::option::Option<crate::schemas::ChartOptions>,
        #[doc = "Required. The data displayed in this chart."]
        #[serde(
            rename = "dataSets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_sets: ::std::option::Option<Vec<crate::schemas::DataSet>>,
        #[doc = "Threshold lines drawn horizontally across the chart."]
        #[serde(
            rename = "thresholds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thresholds: ::std::option::Option<Vec<crate::schemas::Threshold>>,
        #[doc = "The duration used to display a comparison chart. A comparison chart simultaneously shows values from two similar-length time periods (e.g., week-over-week metrics). The duration must be positive, and it can only be applied to charts with data sets of LINE plot type."]
        #[serde(
            rename = "timeshiftDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeshift_duration: ::std::option::Option<String>,
        #[doc = "The properties applied to the X axis."]
        #[serde(
            rename = "xAxis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_axis: ::std::option::Option<crate::schemas::Axis>,
        #[doc = "The properties applied to the Y axis."]
        #[serde(
            rename = "yAxis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y_axis: ::std::option::Option<crate::schemas::Axis>,
    }
    impl ::google_field_selector::FieldSelector for XyChart {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for XyChart {
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
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
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the dashboards resource"]
            pub fn dashboards(&self) -> crate::resources::projects::dashboards::DashboardsActions {
                crate::resources::projects::dashboards::DashboardsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod dashboards {
            pub mod params {}
            pub struct DashboardsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DashboardsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new custom dashboard.This method requires the monitoring.dashboards.create permission on the specified project. For more information, see Google Cloud IAM (https://cloud.google.com/iam)."]
                pub fn create(
                    &self,
                    request: crate::schemas::Dashboard,
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
                #[doc = "Deletes an existing custom dashboard.This method requires the monitoring.dashboards.delete permission on the specified dashboard. For more information, see Google Cloud IAM (https://cloud.google.com/iam)."]
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
                #[doc = "Fetches a specific dashboard.This method requires the monitoring.dashboards.get permission on the specified dashboard. For more information, see Google Cloud IAM (https://cloud.google.com/iam)."]
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
                #[doc = "Lists the existing dashboards.This method requires the monitoring.dashboards.list permission on the specified project. For more information, see Google Cloud IAM (https://cloud.google.com/iam)."]
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
                    }
                }
                #[doc = "Replaces an existing custom dashboard with a new definition.This method requires the monitoring.dashboards.update permission on the specified dashboard. For more information, see Google Cloud IAM (https://cloud.google.com/iam)."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Dashboard,
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
            #[doc = "Created via [DashboardsActions::create()](struct.DashboardsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Dashboard,
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
                ) -> Result<crate::schemas::Dashboard, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Dashboard, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://monitoring.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/dashboards");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
            #[doc = "Created via [DashboardsActions::delete()](struct.DashboardsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://monitoring.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
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
            #[doc = "Created via [DashboardsActions::get()](struct.DashboardsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                ) -> Result<crate::schemas::Dashboard, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Dashboard, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://monitoring.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
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
            #[doc = "Created via [DashboardsActions::list()](struct.DashboardsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
                #[doc = "A positive number that is the maximum number of results to return. If unspecified, a default of 1000 is used."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
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
                pub fn iter_dashboards<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_dashboards_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_dashboards_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Dashboard> {
                    self.iter_dashboards_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_dashboards_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Dashboard> {
                    self.iter_dashboards_with_fields(Some("*"))
                }
                pub fn iter_dashboards_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "dashboards").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "dashboards")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDashboardsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDashboardsResponse>
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
                ) -> Result<crate::schemas::ListDashboardsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDashboardsResponse, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://monitoring.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/dashboards");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [DashboardsActions::patch()](struct.DashboardsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Dashboard,
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
                ) -> Result<crate::schemas::Dashboard, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Dashboard, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://monitoring.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
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
