pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ActivityActivityType {
        #[doc = "ActivityType will never have this value in the response. Using this type in\nthe request will result in an error."]
        ActivityTypeUnspecified,
        #[doc = "Used when the activity resulted out of a visitor viewing a page."]
        Pageview,
        #[doc = "Used when the activity resulted out of a visitor using an application on a\nmobile device."]
        Screenview,
        #[doc = "Used to denote that a goal type activity."]
        Goal,
        #[doc = "An e-commerce transaction was performed by the visitor on the page."]
        Ecommerce,
        #[doc = "Used when the activity is an event."]
        Event,
    }
    impl ActivityActivityType {
        pub fn as_str(self) -> &'static str {
            match self {
                ActivityActivityType::ActivityTypeUnspecified => "ACTIVITY_TYPE_UNSPECIFIED",
                ActivityActivityType::Pageview => "PAGEVIEW",
                ActivityActivityType::Screenview => "SCREENVIEW",
                ActivityActivityType::Goal => "GOAL",
                ActivityActivityType::Ecommerce => "ECOMMERCE",
                ActivityActivityType::Event => "EVENT",
            }
        }
    }
    impl ::std::fmt::Display for ActivityActivityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ActivityActivityType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ActivityActivityType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVITY_TYPE_UNSPECIFIED" => ActivityActivityType::ActivityTypeUnspecified,
                "PAGEVIEW" => ActivityActivityType::Pageview,
                "SCREENVIEW" => ActivityActivityType::Screenview,
                "GOAL" => ActivityActivityType::Goal,
                "ECOMMERCE" => ActivityActivityType::Ecommerce,
                "EVENT" => ActivityActivityType::Event,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Activity {
        #[doc = "Timestamp of the activity."]
        #[serde(rename = "activityTime", default)]
        pub activity_time: Option<String>,
        #[doc = "Type of this activity."]
        #[serde(rename = "activityType", default)]
        pub activity_type: Option<crate::schemas::ActivityActivityType>,
        #[doc = "This will be set if `activity_type` equals `SCREEN_VIEW`."]
        #[serde(rename = "appview", default)]
        pub appview: Option<crate::schemas::ScreenviewData>,
        #[doc = "For manual campaign tracking, it is the value of the utm_campaign campaign\ntracking parameter. For AdWords autotagging, it is the name(s) of the\nonline ad campaign(s) you use for the property. If you use neither, its\nvalue is (not set)."]
        #[serde(rename = "campaign", default)]
        pub campaign: Option<String>,
        #[doc = "The Channel Group associated with an end user's session for this View\n(defined by the View's Channel Groupings)."]
        #[serde(rename = "channelGrouping", default)]
        pub channel_grouping: Option<String>,
        #[doc = "A list of all custom dimensions associated with this activity."]
        #[serde(rename = "customDimension", default)]
        pub custom_dimension: Option<Vec<crate::schemas::CustomDimension>>,
        #[doc = "This will be set if `activity_type` equals `ECOMMERCE`."]
        #[serde(rename = "ecommerce", default)]
        pub ecommerce: Option<crate::schemas::EcommerceData>,
        #[doc = "This field contains all the details pertaining to an event and will be\nset if `activity_type` equals `EVENT`."]
        #[serde(rename = "event", default)]
        pub event: Option<crate::schemas::EventData>,
        #[doc = "This field contains a list of all the goals that were reached in this\nactivity when `activity_type` equals `GOAL`."]
        #[serde(rename = "goals", default)]
        pub goals: Option<crate::schemas::GoalSetData>,
        #[doc = "The hostname from which the tracking request was made."]
        #[serde(rename = "hostname", default)]
        pub hostname: Option<String>,
        #[doc = "For manual campaign tracking, it is the value of the utm_term campaign\ntracking parameter. For AdWords traffic, it contains the best matching\ntargeting criteria. For the display network, where multiple targeting\ncriteria could have caused the ad to show up, it returns the best matching\ntargeting criteria as selected by Ads. This could be display_keyword, site\nplacement, boomuserlist, user_interest, age, or gender. Otherwise its value\nis (not set)."]
        #[serde(rename = "keyword", default)]
        pub keyword: Option<String>,
        #[doc = "The first page in users' sessions, or the landing page."]
        #[serde(rename = "landingPagePath", default)]
        pub landing_page_path: Option<String>,
        #[doc = "The type of referrals. For manual campaign tracking, it is the value of the\nutm_medium campaign tracking parameter. For AdWords autotagging, it is cpc.\nIf users came from a search engine detected by Google Analytics, it is\norganic. If the referrer is not a search engine, it is referral. If users\ncame directly to the property and document.referrer is empty, its value is\n(none)."]
        #[serde(rename = "medium", default)]
        pub medium: Option<String>,
        #[doc = "This will be set if `activity_type` equals `PAGEVIEW`. This field\ncontains all the details about the visitor and the page that was visited."]
        #[serde(rename = "pageview", default)]
        pub pageview: Option<crate::schemas::PageviewData>,
        #[doc = "The source of referrals. For manual campaign tracking, it is the value of\nthe utm_source campaign tracking parameter. For AdWords autotagging, it is\ngoogle. If you use neither, it is the domain of the source\n(e.g., document.referrer) referring the users. It may also contain a port\naddress. If users arrived without a referrer, its value is (direct)."]
        #[serde(rename = "source", default)]
        pub source: Option<String>,
    }
    impl ::field_selector::FieldSelector for Activity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CohortType {
        #[doc = "If unspecified it's treated as `FIRST_VISIT_DATE`."]
        UnspecifiedCohortType,
        #[doc = "Cohorts that are selected based on first visit date."]
        FirstVisitDate,
    }
    impl CohortType {
        pub fn as_str(self) -> &'static str {
            match self {
                CohortType::UnspecifiedCohortType => "UNSPECIFIED_COHORT_TYPE",
                CohortType::FirstVisitDate => "FIRST_VISIT_DATE",
            }
        }
    }
    impl ::std::fmt::Display for CohortType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CohortType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CohortType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED_COHORT_TYPE" => CohortType::UnspecifiedCohortType,
                "FIRST_VISIT_DATE" => CohortType::FirstVisitDate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Cohort {
        #[doc = "This is used for `FIRST_VISIT_DATE` cohort, the cohort selects users\nwhose first visit date is between start date and end date defined in the\nDateRange. The date ranges should be aligned for cohort requests. If the\nrequest contains `ga:cohortNthDay` it should be exactly one day long,\nif `ga:cohortNthWeek` it should be aligned to the week boundary (starting\nat Sunday and ending Saturday), and for `ga:cohortNthMonth` the date range\nshould be aligned to the month (starting at the first and ending on the\nlast day of the month).\nFor LTV requests there are no such restrictions.\nYou do not need to supply a date range for the\n`reportsRequest.dateRanges` field."]
        #[serde(rename = "dateRange", default)]
        pub date_range: Option<crate::schemas::DateRange>,
        #[doc = "A unique name for the cohort. If not defined name will be auto-generated\nwith values cohort_[1234...]."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Type of the cohort. The only supported type as of now is\n`FIRST_VISIT_DATE`. If this field is unspecified the cohort is treated\nas `FIRST_VISIT_DATE` type cohort."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::CohortType>,
    }
    impl ::field_selector::FieldSelector for Cohort {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CohortGroup {
        #[doc = "The definition for the cohort."]
        #[serde(rename = "cohorts", default)]
        pub cohorts: Option<Vec<crate::schemas::Cohort>>,
        #[doc = "Enable Life Time Value (LTV).  LTV measures lifetime value for users\nacquired through different channels.\nPlease see:\n[Cohort Analysis](https://support.google.com/analytics/answer/6074676) and\n[Lifetime Value](https://support.google.com/analytics/answer/6182550)\nIf the value of lifetimeValue is false:\n\n* The metric values are similar to the values in the web interface cohort\n  report.\n* The cohort definition date ranges must be aligned to the calendar week\n  and month. i.e. while requesting `ga:cohortNthWeek` the `startDate` in\n  the cohort definition should be a Sunday and the `endDate` should be the\n  following Saturday, and for `ga:cohortNthMonth`, the `startDate`\n  should be the 1st of the month and `endDate` should be the last day\n  of the month.\n\nWhen the lifetimeValue is true:\n\n* The metric values will correspond to the values in the web interface\n  LifeTime value report.\n* The Lifetime Value report shows you how user value (Revenue) and\n  engagement (Appviews, Goal Completions, Sessions, and Session Duration)\n  grow during the 90 days after a user is acquired.\n* The metrics are calculated as a cumulative average per user per the time\n  increment.\n* The cohort definition date ranges need not be aligned to the calendar\n  week and month boundaries.\n* The `viewId` must be an\n  [app view\n  ID](https://support.google.com/analytics/answer/2649553#WebVersusAppViews)"]
        #[serde(rename = "lifetimeValue", default)]
        pub lifetime_value: Option<bool>,
    }
    impl ::field_selector::FieldSelector for CohortGroup {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ColumnHeader {
        #[doc = "The dimension names in the response."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: Option<Vec<String>>,
        #[doc = "Metric headers for the metrics in the response."]
        #[serde(rename = "metricHeader", default)]
        pub metric_header: Option<crate::schemas::MetricHeader>,
    }
    impl ::field_selector::FieldSelector for ColumnHeader {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Slot number of custom dimension."]
        #[serde(rename = "index", default)]
        pub index: Option<i32>,
        #[doc = "Value of the custom dimension. Default value (i.e. empty string) indicates\nclearing sesion/visitor scope custom dimension value."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomDimension {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DateRange {
        #[doc = "The end date for the query in the format `YYYY-MM-DD`."]
        #[serde(rename = "endDate", default)]
        pub end_date: Option<String>,
        #[doc = "The start date for the query in the format `YYYY-MM-DD`."]
        #[serde(rename = "startDate", default)]
        pub start_date: Option<String>,
    }
    impl ::field_selector::FieldSelector for DateRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DateRangeValues {
        #[doc = "The values of each pivot region."]
        #[serde(rename = "pivotValueRegions", default)]
        pub pivot_value_regions: Option<Vec<crate::schemas::PivotValueRegion>>,
        #[doc = "Each value corresponds to each Metric in the request."]
        #[serde(rename = "values", default)]
        pub values: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for DateRangeValues {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Dimension {
        #[doc = "If non-empty, we place dimension values into buckets after string to\nint64. Dimension values that are not the string representation of an\nintegral value will be converted to zero.  The bucket values have to be in\nincreasing order.  Each bucket is closed on the lower end, and open on the\nupper end. The \"first\" bucket includes all values less than the first\nboundary, the \"last\" bucket includes all values up to infinity. Dimension\nvalues that fall in a bucket get transformed to a new dimension value. For\nexample, if one gives a list of \"0, 1, 3, 4, 7\", then we return the\nfollowing buckets:\n\n* bucket #1: values < 0, dimension value \"<0\"\n* bucket #2: values in [0,1), dimension value \"0\"\n* bucket #3: values in [1,3), dimension value \"1-2\"\n* bucket #4: values in [3,4), dimension value \"3\"\n* bucket #5: values in [4,7), dimension value \"4-6\"\n* bucket #6: values >= 7, dimension value \"7+\"\n\nNOTE: If you are applying histogram mutation on any dimension, and using\nthat dimension in sort, you will want to use the sort type\n`HISTOGRAM_BUCKET` for that purpose. Without that the dimension values\nwill be sorted according to dictionary\n(lexicographic) order. For example the ascending dictionary order is:\n\n\"<50\", \"1001+\", \"121-1000\", \"50-120\"\n\nAnd the ascending `HISTOGRAM_BUCKET` order is:\n\n\"<50\", \"50-120\", \"121-1000\", \"1001+\"\n\nThe client has to explicitly request `\"orderType\": \"HISTOGRAM_BUCKET\"`\nfor a histogram-mutated dimension."]
        #[serde(rename = "histogramBuckets", default)]
        pub histogram_buckets: Option<Vec<i64>>,
        #[doc = "Name of the dimension to fetch, for example `ga:browser`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Dimension {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionFilterOperator {
        #[doc = "If the match type is unspecified, it is treated as a `REGEXP`."]
        OperatorUnspecified,
        #[doc = "The match expression is treated as a regular expression. All match types\nare not treated as regular expressions."]
        Regexp,
        #[doc = "Matches the value which begin with the match expression provided."]
        BeginsWith,
        #[doc = "Matches the values which end with the match expression provided."]
        EndsWith,
        #[doc = "Substring match."]
        Partial,
        #[doc = "The value should match the match expression entirely."]
        Exact,
        #[doc = "Integer comparison filters.\ncase sensitivity is ignored for these and the expression\nis assumed to be a string representing an integer.\nFailure conditions:\n\n* If expression is not a valid int64, the client should expect\n  an error.\n* Input dimensions that are not valid int64 values will never match the\n  filter."]
        NumericEqual,
        #[doc = "Checks if the dimension is numerically greater than the match\nexpression. Read the description for `NUMERIC_EQUALS` for restrictions."]
        NumericGreaterThan,
        #[doc = "Checks if the dimension is numerically less than the match expression.\nRead the description for `NUMERIC_EQUALS` for restrictions."]
        NumericLessThan,
        #[doc = "This option is used to specify a dimension filter whose expression can\ntake any value from a selected list of values. This helps avoiding\nevaluating multiple exact match dimension filters which are OR'ed for\nevery single response row. For example:\n\n````text\nexpressions: [\"A\", \"B\", \"C\"]\n````\n\nAny response row whose dimension has it is value as A, B or C, matches\nthis DimensionFilter."]
        InList,
    }
    impl DimensionFilterOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionFilterOperator::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
                DimensionFilterOperator::Regexp => "REGEXP",
                DimensionFilterOperator::BeginsWith => "BEGINS_WITH",
                DimensionFilterOperator::EndsWith => "ENDS_WITH",
                DimensionFilterOperator::Partial => "PARTIAL",
                DimensionFilterOperator::Exact => "EXACT",
                DimensionFilterOperator::NumericEqual => "NUMERIC_EQUAL",
                DimensionFilterOperator::NumericGreaterThan => "NUMERIC_GREATER_THAN",
                DimensionFilterOperator::NumericLessThan => "NUMERIC_LESS_THAN",
                DimensionFilterOperator::InList => "IN_LIST",
            }
        }
    }
    impl ::std::fmt::Display for DimensionFilterOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionFilterOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionFilterOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATOR_UNSPECIFIED" => DimensionFilterOperator::OperatorUnspecified,
                "REGEXP" => DimensionFilterOperator::Regexp,
                "BEGINS_WITH" => DimensionFilterOperator::BeginsWith,
                "ENDS_WITH" => DimensionFilterOperator::EndsWith,
                "PARTIAL" => DimensionFilterOperator::Partial,
                "EXACT" => DimensionFilterOperator::Exact,
                "NUMERIC_EQUAL" => DimensionFilterOperator::NumericEqual,
                "NUMERIC_GREATER_THAN" => DimensionFilterOperator::NumericGreaterThan,
                "NUMERIC_LESS_THAN" => DimensionFilterOperator::NumericLessThan,
                "IN_LIST" => DimensionFilterOperator::InList,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DimensionFilter {
        #[doc = "Should the match be case sensitive? Default is false."]
        #[serde(rename = "caseSensitive", default)]
        pub case_sensitive: Option<bool>,
        #[doc = "The dimension to filter on. A DimensionFilter must contain a dimension."]
        #[serde(rename = "dimensionName", default)]
        pub dimension_name: Option<String>,
        #[doc = "Strings or regular expression to match against. Only the first value of\nthe list is used for comparison unless the operator is `IN_LIST`.\nIf `IN_LIST` operator, then the entire list is used to filter the\ndimensions as explained in the description of the `IN_LIST` operator."]
        #[serde(rename = "expressions", default)]
        pub expressions: Option<Vec<String>>,
        #[doc = "Logical `NOT` operator. If this boolean is set to true, then the matching\ndimension values will be excluded in the report. The default is false."]
        #[serde(rename = "not", default)]
        pub not: Option<bool>,
        #[doc = "How to match the dimension to the expression. The default is REGEXP."]
        #[serde(rename = "operator", default)]
        pub operator: Option<crate::schemas::DimensionFilterOperator>,
    }
    impl ::field_selector::FieldSelector for DimensionFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionFilterClauseOperator {
        #[doc = "Unspecified operator. It is treated as an `OR`."]
        OperatorUnspecified,
        #[doc = "The logical `OR` operator."]
        Or,
        #[doc = "The logical `AND` operator."]
        And,
    }
    impl DimensionFilterClauseOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionFilterClauseOperator::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
                DimensionFilterClauseOperator::Or => "OR",
                DimensionFilterClauseOperator::And => "AND",
            }
        }
    }
    impl ::std::fmt::Display for DimensionFilterClauseOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionFilterClauseOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionFilterClauseOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATOR_UNSPECIFIED" => DimensionFilterClauseOperator::OperatorUnspecified,
                "OR" => DimensionFilterClauseOperator::Or,
                "AND" => DimensionFilterClauseOperator::And,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DimensionFilterClause {
        #[doc = "The repeated set of filters. They are logically combined based on the\noperator specified."]
        #[serde(rename = "filters", default)]
        pub filters: Option<Vec<crate::schemas::DimensionFilter>>,
        #[doc = "The operator for combining multiple dimension filters. If unspecified, it\nis treated as an `OR`."]
        #[serde(rename = "operator", default)]
        pub operator: Option<crate::schemas::DimensionFilterClauseOperator>,
    }
    impl ::field_selector::FieldSelector for DimensionFilterClause {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DynamicSegment {
        #[doc = "The name of the dynamic segment."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Session Segment to select sessions to include in the segment."]
        #[serde(rename = "sessionSegment", default)]
        pub session_segment: Option<crate::schemas::SegmentDefinition>,
        #[doc = "User Segment to select users to include in the segment."]
        #[serde(rename = "userSegment", default)]
        pub user_segment: Option<crate::schemas::SegmentDefinition>,
    }
    impl ::field_selector::FieldSelector for DynamicSegment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EcommerceDataActionType {
        #[doc = "Action type is not known."]
        Unknown,
        #[doc = "Click through of product lists."]
        Click,
        #[doc = "Product detail views."]
        DetailsView,
        #[doc = "Add product(s) to cart."]
        AddToCart,
        #[doc = "Remove product(s) from cart."]
        RemoveFromCart,
        #[doc = "Check out."]
        Checkout,
        #[doc = "Completed purchase."]
        Payment,
        #[doc = "Refund of purchase."]
        Refund,
        #[doc = "Checkout options."]
        CheckoutOption,
    }
    impl EcommerceDataActionType {
        pub fn as_str(self) -> &'static str {
            match self {
                EcommerceDataActionType::Unknown => "UNKNOWN",
                EcommerceDataActionType::Click => "CLICK",
                EcommerceDataActionType::DetailsView => "DETAILS_VIEW",
                EcommerceDataActionType::AddToCart => "ADD_TO_CART",
                EcommerceDataActionType::RemoveFromCart => "REMOVE_FROM_CART",
                EcommerceDataActionType::Checkout => "CHECKOUT",
                EcommerceDataActionType::Payment => "PAYMENT",
                EcommerceDataActionType::Refund => "REFUND",
                EcommerceDataActionType::CheckoutOption => "CHECKOUT_OPTION",
            }
        }
    }
    impl ::std::fmt::Display for EcommerceDataActionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EcommerceDataActionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EcommerceDataActionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => EcommerceDataActionType::Unknown,
                "CLICK" => EcommerceDataActionType::Click,
                "DETAILS_VIEW" => EcommerceDataActionType::DetailsView,
                "ADD_TO_CART" => EcommerceDataActionType::AddToCart,
                "REMOVE_FROM_CART" => EcommerceDataActionType::RemoveFromCart,
                "CHECKOUT" => EcommerceDataActionType::Checkout,
                "PAYMENT" => EcommerceDataActionType::Payment,
                "REFUND" => EcommerceDataActionType::Refund,
                "CHECKOUT_OPTION" => EcommerceDataActionType::CheckoutOption,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EcommerceDataEcommerceType {
        #[doc = "Used when the e-commerce activity type is unspecified."]
        EcommerceTypeUnspecified,
        #[doc = "Used when activity has classic (non-enhanced) e-commerce information."]
        Classic,
        #[doc = "Used when activity has enhanced e-commerce information."]
        Enhanced,
    }
    impl EcommerceDataEcommerceType {
        pub fn as_str(self) -> &'static str {
            match self {
                EcommerceDataEcommerceType::EcommerceTypeUnspecified => {
                    "ECOMMERCE_TYPE_UNSPECIFIED"
                }
                EcommerceDataEcommerceType::Classic => "CLASSIC",
                EcommerceDataEcommerceType::Enhanced => "ENHANCED",
            }
        }
    }
    impl ::std::fmt::Display for EcommerceDataEcommerceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EcommerceDataEcommerceType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EcommerceDataEcommerceType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ECOMMERCE_TYPE_UNSPECIFIED" => {
                    EcommerceDataEcommerceType::EcommerceTypeUnspecified
                }
                "CLASSIC" => EcommerceDataEcommerceType::Classic,
                "ENHANCED" => EcommerceDataEcommerceType::Enhanced,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EcommerceData {
        #[doc = "Action associated with this e-commerce action."]
        #[serde(rename = "actionType", default)]
        pub action_type: Option<crate::schemas::EcommerceDataActionType>,
        #[doc = "The type of this e-commerce activity."]
        #[serde(rename = "ecommerceType", default)]
        pub ecommerce_type: Option<crate::schemas::EcommerceDataEcommerceType>,
        #[doc = "Details of the products in this transaction."]
        #[serde(rename = "products", default)]
        pub products: Option<Vec<crate::schemas::ProductData>>,
        #[doc = "Transaction details of this e-commerce action."]
        #[serde(rename = "transaction", default)]
        pub transaction: Option<crate::schemas::TransactionData>,
    }
    impl ::field_selector::FieldSelector for EcommerceData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EventData {
        #[doc = "Type of interaction with the object. Eg: 'play'."]
        #[serde(rename = "eventAction", default)]
        pub event_action: Option<String>,
        #[doc = "The object on the page that was interacted with. Eg: 'Video'."]
        #[serde(rename = "eventCategory", default)]
        pub event_category: Option<String>,
        #[doc = "Number of such events in this activity."]
        #[serde(rename = "eventCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub event_count: Option<i64>,
        #[doc = "Label attached with the event."]
        #[serde(rename = "eventLabel", default)]
        pub event_label: Option<String>,
        #[doc = "Numeric value associated with the event."]
        #[serde(rename = "eventValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub event_value: Option<i64>,
    }
    impl ::field_selector::FieldSelector for EventData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetReportsRequest {
        #[doc = "Requests, each request will have a separate response.\nThere can be a maximum of 5 requests. All requests should have the same\n`dateRanges`, `viewId`, `segments`, `samplingLevel`, and `cohortGroup`."]
        #[serde(rename = "reportRequests", default)]
        pub report_requests: Option<Vec<crate::schemas::ReportRequest>>,
        #[doc = "Enables\n[resource based\nquotas](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4),\n(defaults to `False`). If this field is set to `True` the\nper view (profile) quotas are governed by the computational\ncost of the request. Note that using cost based quotas will\nhigher enable sampling rates. (10 Million for `SMALL`,\n100M for `LARGE`. See the\n[limits and quotas\ndocumentation](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4)\nfor details."]
        #[serde(rename = "useResourceQuotas", default)]
        pub use_resource_quotas: Option<bool>,
    }
    impl ::field_selector::FieldSelector for GetReportsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetReportsResponse {
        #[doc = "The amount of resource quota tokens deducted to execute the query. Includes\nall responses."]
        #[serde(rename = "queryCost", default)]
        pub query_cost: Option<i32>,
        #[doc = "Responses corresponding to each of the request."]
        #[serde(rename = "reports", default)]
        pub reports: Option<Vec<crate::schemas::Report>>,
        #[doc = "The amount of resource quota remaining for the property."]
        #[serde(rename = "resourceQuotasRemaining", default)]
        pub resource_quotas_remaining: Option<crate::schemas::ResourceQuotasRemaining>,
    }
    impl ::field_selector::FieldSelector for GetReportsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoalData {
        #[doc = "URL of the page where this goal was completed."]
        #[serde(rename = "goalCompletionLocation", default)]
        pub goal_completion_location: Option<String>,
        #[doc = "Total number of goal completions in this activity."]
        #[serde(rename = "goalCompletions", default)]
        #[serde(with = "crate::parsed_string")]
        pub goal_completions: Option<i64>,
        #[doc = "This identifies the goal as configured for the profile."]
        #[serde(rename = "goalIndex", default)]
        pub goal_index: Option<i32>,
        #[doc = "Name of the goal."]
        #[serde(rename = "goalName", default)]
        pub goal_name: Option<String>,
        #[doc = "URL of the page one step prior to the goal completion."]
        #[serde(rename = "goalPreviousStep1", default)]
        pub goal_previous_step_1: Option<String>,
        #[doc = "URL of the page two steps prior to the goal completion."]
        #[serde(rename = "goalPreviousStep2", default)]
        pub goal_previous_step_2: Option<String>,
        #[doc = "URL of the page three steps prior to the goal completion."]
        #[serde(rename = "goalPreviousStep3", default)]
        pub goal_previous_step_3: Option<String>,
        #[doc = "Value in this goal."]
        #[serde(rename = "goalValue", default)]
        pub goal_value: Option<f64>,
    }
    impl ::field_selector::FieldSelector for GoalData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoalSetData {
        #[doc = "All the goals that were reached in the current activity."]
        #[serde(rename = "goals", default)]
        pub goals: Option<Vec<crate::schemas::GoalData>>,
    }
    impl ::field_selector::FieldSelector for GoalSetData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricFormattingType {
        #[doc = "Metric type is unspecified."]
        MetricTypeUnspecified,
        #[doc = "Integer metric."]
        Integer,
        #[doc = "Float metric."]
        Float,
        #[doc = "Currency metric."]
        Currency,
        #[doc = "Percentage metric."]
        Percent,
        #[doc = "Time metric in `HH:MM:SS` format."]
        Time,
    }
    impl MetricFormattingType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricFormattingType::MetricTypeUnspecified => "METRIC_TYPE_UNSPECIFIED",
                MetricFormattingType::Integer => "INTEGER",
                MetricFormattingType::Float => "FLOAT",
                MetricFormattingType::Currency => "CURRENCY",
                MetricFormattingType::Percent => "PERCENT",
                MetricFormattingType::Time => "TIME",
            }
        }
    }
    impl ::std::fmt::Display for MetricFormattingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricFormattingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricFormattingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METRIC_TYPE_UNSPECIFIED" => MetricFormattingType::MetricTypeUnspecified,
                "INTEGER" => MetricFormattingType::Integer,
                "FLOAT" => MetricFormattingType::Float,
                "CURRENCY" => MetricFormattingType::Currency,
                "PERCENT" => MetricFormattingType::Percent,
                "TIME" => MetricFormattingType::Time,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Metric {
        #[doc = "An alias for the metric expression is an alternate name for the\nexpression. The alias can be used for filtering and sorting. This field\nis optional and is useful if the expression is not a single metric but\na complex expression which cannot be used in filtering and sorting.\nThe alias is also used in the response column header."]
        #[serde(rename = "alias", default)]
        pub alias: Option<String>,
        #[doc = "A metric expression in the request. An expression is constructed from one\nor more metrics and numbers. Accepted operators include: Plus (+), Minus\n(-), Negation (Unary -), Divided by (/), Multiplied by (*), Parenthesis,\nPositive cardinal numbers (0-9), can include decimals and is limited to\n1024 characters. Example `ga:totalRefunds/ga:users`, in most cases the\nmetric expression is just a single metric name like `ga:users`.\nAdding mixed `MetricType` (E.g., `CURRENCY` + `PERCENTAGE`) metrics\nwill result in unexpected results."]
        #[serde(rename = "expression", default)]
        pub expression: Option<String>,
        #[doc = "Specifies how the metric expression should be formatted, for example\n`INTEGER`."]
        #[serde(rename = "formattingType", default)]
        pub formatting_type: Option<crate::schemas::MetricFormattingType>,
    }
    impl ::field_selector::FieldSelector for Metric {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricFilterOperator {
        #[doc = "If the operator is not specified, it is treated as `EQUAL`."]
        OperatorUnspecified,
        #[doc = "Should the value of the metric be exactly equal to the comparison value."]
        Equal,
        #[doc = "Should the value of the metric be less than to the comparison value."]
        LessThan,
        #[doc = "Should the value of the metric be greater than to the comparison value."]
        GreaterThan,
        #[doc = "Validates if the metric is missing.\nDoesn't take comparisonValue into account."]
        IsMissing,
    }
    impl MetricFilterOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricFilterOperator::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
                MetricFilterOperator::Equal => "EQUAL",
                MetricFilterOperator::LessThan => "LESS_THAN",
                MetricFilterOperator::GreaterThan => "GREATER_THAN",
                MetricFilterOperator::IsMissing => "IS_MISSING",
            }
        }
    }
    impl ::std::fmt::Display for MetricFilterOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricFilterOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricFilterOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATOR_UNSPECIFIED" => MetricFilterOperator::OperatorUnspecified,
                "EQUAL" => MetricFilterOperator::Equal,
                "LESS_THAN" => MetricFilterOperator::LessThan,
                "GREATER_THAN" => MetricFilterOperator::GreaterThan,
                "IS_MISSING" => MetricFilterOperator::IsMissing,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricFilter {
        #[doc = "The value to compare against."]
        #[serde(rename = "comparisonValue", default)]
        pub comparison_value: Option<String>,
        #[doc = "The metric that will be filtered on. A metricFilter must contain a metric\nname. A metric name can be an alias earlier defined as a metric or it can\nalso be a metric expression."]
        #[serde(rename = "metricName", default)]
        pub metric_name: Option<String>,
        #[doc = "Logical `NOT` operator. If this boolean is set to true, then the matching\nmetric values will be excluded in the report. The default is false."]
        #[serde(rename = "not", default)]
        pub not: Option<bool>,
        #[doc = "Is the metric `EQUAL`, `LESS_THAN` or `GREATER_THAN` the\ncomparisonValue, the default is `EQUAL`. If the operator is\n`IS_MISSING`, checks if the metric is missing and would ignore the\ncomparisonValue."]
        #[serde(rename = "operator", default)]
        pub operator: Option<crate::schemas::MetricFilterOperator>,
    }
    impl ::field_selector::FieldSelector for MetricFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricFilterClauseOperator {
        #[doc = "Unspecified operator. It is treated as an `OR`."]
        OperatorUnspecified,
        #[doc = "The logical `OR` operator."]
        Or,
        #[doc = "The logical `AND` operator."]
        And,
    }
    impl MetricFilterClauseOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricFilterClauseOperator::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
                MetricFilterClauseOperator::Or => "OR",
                MetricFilterClauseOperator::And => "AND",
            }
        }
    }
    impl ::std::fmt::Display for MetricFilterClauseOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricFilterClauseOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricFilterClauseOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATOR_UNSPECIFIED" => MetricFilterClauseOperator::OperatorUnspecified,
                "OR" => MetricFilterClauseOperator::Or,
                "AND" => MetricFilterClauseOperator::And,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricFilterClause {
        #[doc = "The repeated set of filters. They are logically combined based on the\noperator specified."]
        #[serde(rename = "filters", default)]
        pub filters: Option<Vec<crate::schemas::MetricFilter>>,
        #[doc = "The operator for combining multiple metric filters. If unspecified, it is\ntreated as an `OR`."]
        #[serde(rename = "operator", default)]
        pub operator: Option<crate::schemas::MetricFilterClauseOperator>,
    }
    impl ::field_selector::FieldSelector for MetricFilterClause {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricHeader {
        #[doc = "Headers for the metrics in the response."]
        #[serde(rename = "metricHeaderEntries", default)]
        pub metric_header_entries: Option<Vec<crate::schemas::MetricHeaderEntry>>,
        #[doc = "Headers for the pivots in the response."]
        #[serde(rename = "pivotHeaders", default)]
        pub pivot_headers: Option<Vec<crate::schemas::PivotHeader>>,
    }
    impl ::field_selector::FieldSelector for MetricHeader {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricHeaderEntryType {
        #[doc = "Metric type is unspecified."]
        MetricTypeUnspecified,
        #[doc = "Integer metric."]
        Integer,
        #[doc = "Float metric."]
        Float,
        #[doc = "Currency metric."]
        Currency,
        #[doc = "Percentage metric."]
        Percent,
        #[doc = "Time metric in `HH:MM:SS` format."]
        Time,
    }
    impl MetricHeaderEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricHeaderEntryType::MetricTypeUnspecified => "METRIC_TYPE_UNSPECIFIED",
                MetricHeaderEntryType::Integer => "INTEGER",
                MetricHeaderEntryType::Float => "FLOAT",
                MetricHeaderEntryType::Currency => "CURRENCY",
                MetricHeaderEntryType::Percent => "PERCENT",
                MetricHeaderEntryType::Time => "TIME",
            }
        }
    }
    impl ::std::fmt::Display for MetricHeaderEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricHeaderEntryType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricHeaderEntryType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METRIC_TYPE_UNSPECIFIED" => MetricHeaderEntryType::MetricTypeUnspecified,
                "INTEGER" => MetricHeaderEntryType::Integer,
                "FLOAT" => MetricHeaderEntryType::Float,
                "CURRENCY" => MetricHeaderEntryType::Currency,
                "PERCENT" => MetricHeaderEntryType::Percent,
                "TIME" => MetricHeaderEntryType::Time,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricHeaderEntry {
        #[doc = "The name of the header."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The type of the metric, for example `INTEGER`."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::MetricHeaderEntryType>,
    }
    impl ::field_selector::FieldSelector for MetricHeaderEntry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OrFiltersForSegment {
        #[doc = "List of segment filters to be combined with a `OR` operator."]
        #[serde(rename = "segmentFilterClauses", default)]
        pub segment_filter_clauses: Option<Vec<crate::schemas::SegmentFilterClause>>,
    }
    impl ::field_selector::FieldSelector for OrFiltersForSegment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OrderByOrderType {
        #[doc = "Unspecified order type will be treated as sort based on value."]
        OrderTypeUnspecified,
        #[doc = "The sort order is based on the value of the chosen column; looks only at\nthe first date range."]
        Value,
        #[doc = "The sort order is based on the difference of the values of the chosen\ncolumn between the first two date ranges.  Usable only if there are\nexactly two date ranges."]
        Delta,
        #[doc = "The sort order is based on weighted value of the chosen column.  If\ncolumn has n/d format, then weighted value of this ratio will\nbe `(n + totals.n)/(d + totals.d)` Usable only for metrics that\nrepresent ratios."]
        Smart,
        #[doc = "Histogram order type is applicable only to dimension columns with\nnon-empty histogram-buckets."]
        HistogramBucket,
        #[doc = "If the dimensions are fixed length numbers, ordinary sort would just\nwork fine. `DIMENSION_AS_INTEGER` can be used if the dimensions are\nvariable length numbers."]
        DimensionAsInteger,
    }
    impl OrderByOrderType {
        pub fn as_str(self) -> &'static str {
            match self {
                OrderByOrderType::OrderTypeUnspecified => "ORDER_TYPE_UNSPECIFIED",
                OrderByOrderType::Value => "VALUE",
                OrderByOrderType::Delta => "DELTA",
                OrderByOrderType::Smart => "SMART",
                OrderByOrderType::HistogramBucket => "HISTOGRAM_BUCKET",
                OrderByOrderType::DimensionAsInteger => "DIMENSION_AS_INTEGER",
            }
        }
    }
    impl ::std::fmt::Display for OrderByOrderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OrderByOrderType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OrderByOrderType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ORDER_TYPE_UNSPECIFIED" => OrderByOrderType::OrderTypeUnspecified,
                "VALUE" => OrderByOrderType::Value,
                "DELTA" => OrderByOrderType::Delta,
                "SMART" => OrderByOrderType::Smart,
                "HISTOGRAM_BUCKET" => OrderByOrderType::HistogramBucket,
                "DIMENSION_AS_INTEGER" => OrderByOrderType::DimensionAsInteger,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OrderBySortOrder {
        #[doc = "If the sort order is unspecified, the default is ascending."]
        SortOrderUnspecified,
        #[doc = "Ascending sort. The field will be sorted in an ascending manner."]
        Ascending,
        #[doc = "Descending sort. The field will be sorted in a descending manner."]
        Descending,
    }
    impl OrderBySortOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                OrderBySortOrder::SortOrderUnspecified => "SORT_ORDER_UNSPECIFIED",
                OrderBySortOrder::Ascending => "ASCENDING",
                OrderBySortOrder::Descending => "DESCENDING",
            }
        }
    }
    impl ::std::fmt::Display for OrderBySortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OrderBySortOrder {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OrderBySortOrder {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SORT_ORDER_UNSPECIFIED" => OrderBySortOrder::SortOrderUnspecified,
                "ASCENDING" => OrderBySortOrder::Ascending,
                "DESCENDING" => OrderBySortOrder::Descending,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OrderBy {
        #[doc = "The field which to sort by. The default sort order is ascending. Example:\n`ga:browser`.\nNote, that you can only specify one field for sort here. For example,\n`ga:browser, ga:city` is not valid."]
        #[serde(rename = "fieldName", default)]
        pub field_name: Option<String>,
        #[doc = "The order type. The default orderType is `VALUE`."]
        #[serde(rename = "orderType", default)]
        pub order_type: Option<crate::schemas::OrderByOrderType>,
        #[doc = "The sorting order for the field."]
        #[serde(rename = "sortOrder", default)]
        pub sort_order: Option<crate::schemas::OrderBySortOrder>,
    }
    impl ::field_selector::FieldSelector for OrderBy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PageviewData {
        #[doc = "The URL of the page that the visitor viewed."]
        #[serde(rename = "pagePath", default)]
        pub page_path: Option<String>,
        #[doc = "The title of the page that the visitor viewed."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: Option<String>,
    }
    impl ::field_selector::FieldSelector for PageviewData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Pivot {
        #[doc = "DimensionFilterClauses are logically combined with an `AND` operator: only\ndata that is included by all these DimensionFilterClauses contributes to\nthe values in this pivot region. Dimension filters can be used to restrict\nthe columns shown in the pivot region. For example if you have\n`ga:browser` as the requested dimension in the pivot region, and you\nspecify key filters to restrict `ga:browser` to only \"IE\" or \"Firefox\",\nthen only those two browsers would show up as columns."]
        #[serde(rename = "dimensionFilterClauses", default)]
        pub dimension_filter_clauses: Option<Vec<crate::schemas::DimensionFilterClause>>,
        #[doc = "A list of dimensions to show as pivot columns. A Pivot can have a maximum\nof 4 dimensions. Pivot dimensions are part of the restriction on the\ntotal number of dimensions allowed in the request."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: Option<Vec<crate::schemas::Dimension>>,
        #[doc = "Specifies the maximum number of groups to return.\nThe default value is 10, also the maximum value is 1,000."]
        #[serde(rename = "maxGroupCount", default)]
        pub max_group_count: Option<i32>,
        #[doc = "The pivot metrics. Pivot metrics are part of the\nrestriction on total number of metrics allowed in the request."]
        #[serde(rename = "metrics", default)]
        pub metrics: Option<Vec<crate::schemas::Metric>>,
        #[doc = "If k metrics were requested, then the response will contain some\ndata-dependent multiple of k columns in the report.  E.g., if you pivoted\non the dimension `ga:browser` then you'd get k columns for \"Firefox\", k\ncolumns for \"IE\", k columns for \"Chrome\", etc. The ordering of the groups\nof columns is determined by descending order of \"total\" for the first of\nthe k values.  Ties are broken by lexicographic ordering of the first\npivot dimension, then lexicographic ordering of the second pivot\ndimension, and so on.  E.g., if the totals for the first value for\nFirefox, IE, and Chrome were 8, 2, 8, respectively, the order of columns\nwould be Chrome, Firefox, IE.\n\nThe following let you choose which of the groups of k columns are\nincluded in the response."]
        #[serde(rename = "startGroup", default)]
        pub start_group: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Pivot {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PivotHeader {
        #[doc = "A single pivot section header."]
        #[serde(rename = "pivotHeaderEntries", default)]
        pub pivot_header_entries: Option<Vec<crate::schemas::PivotHeaderEntry>>,
        #[doc = "The total number of groups for this pivot."]
        #[serde(rename = "totalPivotGroupsCount", default)]
        pub total_pivot_groups_count: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PivotHeader {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PivotHeaderEntry {
        #[doc = "The name of the dimensions in the pivot response."]
        #[serde(rename = "dimensionNames", default)]
        pub dimension_names: Option<Vec<String>>,
        #[doc = "The values for the dimensions in the pivot."]
        #[serde(rename = "dimensionValues", default)]
        pub dimension_values: Option<Vec<String>>,
        #[doc = "The metric header for the metric in the pivot."]
        #[serde(rename = "metric", default)]
        pub metric: Option<crate::schemas::MetricHeaderEntry>,
    }
    impl ::field_selector::FieldSelector for PivotHeaderEntry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PivotValueRegion {
        #[doc = "The values of the metrics in each of the pivot regions."]
        #[serde(rename = "values", default)]
        pub values: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for PivotValueRegion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ProductData {
        #[doc = "The total revenue from purchased product items."]
        #[serde(rename = "itemRevenue", default)]
        pub item_revenue: Option<f64>,
        #[doc = "The product name, supplied by the e-commerce tracking application, for\nthe purchased items."]
        #[serde(rename = "productName", default)]
        pub product_name: Option<String>,
        #[doc = "Total number of this product units in the transaction."]
        #[serde(rename = "productQuantity", default)]
        #[serde(with = "crate::parsed_string")]
        pub product_quantity: Option<i64>,
        #[doc = "Unique code that represents the product."]
        #[serde(rename = "productSku", default)]
        pub product_sku: Option<String>,
    }
    impl ::field_selector::FieldSelector for ProductData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Report {
        #[doc = "The column headers."]
        #[serde(rename = "columnHeader", default)]
        pub column_header: Option<crate::schemas::ColumnHeader>,
        #[doc = "Response data."]
        #[serde(rename = "data", default)]
        pub data: Option<crate::schemas::ReportData>,
        #[doc = "Page token to retrieve the next page of results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for Report {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReportData {
        #[doc = "The last time the data in the report was refreshed. All the hits received\nbefore this timestamp are included in the calculation of the report."]
        #[serde(rename = "dataLastRefreshed", default)]
        pub data_last_refreshed: Option<String>,
        #[doc = "Indicates if response to this request is golden or not. Data is\ngolden when the exact same request will not produce any new results if\nasked at a later point in time."]
        #[serde(rename = "isDataGolden", default)]
        pub is_data_golden: Option<bool>,
        #[doc = "Minimum and maximum values seen over all matching rows. These are both\nempty when `hideValueRanges` in the request is false, or when\nrowCount is zero."]
        #[serde(rename = "maximums", default)]
        pub maximums: Option<Vec<crate::schemas::DateRangeValues>>,
        #[doc = "Minimum and maximum values seen over all matching rows. These are both\nempty when `hideValueRanges` in the request is false, or when\nrowCount is zero."]
        #[serde(rename = "minimums", default)]
        pub minimums: Option<Vec<crate::schemas::DateRangeValues>>,
        #[doc = "Total number of matching rows for this query."]
        #[serde(rename = "rowCount", default)]
        pub row_count: Option<i32>,
        #[doc = "There's one ReportRow for every unique combination of dimensions."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<crate::schemas::ReportRow>>,
        #[doc = "If the results are\n[sampled](https://support.google.com/analytics/answer/2637192),\nthis returns the total number of samples read, one entry per date range.\nIf the results are not sampled this field will not be defined. See\n[developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)\nfor details."]
        #[serde(rename = "samplesReadCounts", default)]
        pub samples_read_counts: Option<Vec<i64>>,
        #[doc = "If the results are\n[sampled](https://support.google.com/analytics/answer/2637192),\nthis returns the total number of\nsamples present, one entry per date range. If the results are not sampled\nthis field will not be defined. See\n[developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)\nfor details."]
        #[serde(rename = "samplingSpaceSizes", default)]
        pub sampling_space_sizes: Option<Vec<i64>>,
        #[doc = "For each requested date range, for the set of all rows that match\nthe query, every requested value format gets a total. The total\nfor a value format is computed by first totaling the metrics\nmentioned in the value format and then evaluating the value\nformat as a scalar expression.  E.g., The \"totals\" for\n`3 / (ga:sessions + 2)` we compute\n`3 / ((sum of all relevant ga:sessions) + 2)`.\nTotals are computed before pagination."]
        #[serde(rename = "totals", default)]
        pub totals: Option<Vec<crate::schemas::DateRangeValues>>,
    }
    impl ::field_selector::FieldSelector for ReportData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReportRequestSamplingLevel {
        #[doc = "If the `samplingLevel` field is unspecified the `DEFAULT` sampling level\nis used."]
        SamplingUnspecified,
        #[doc = "Returns response with a sample size that balances speed and\naccuracy."]
        Default,
        #[doc = "It returns a fast response with a smaller sampling size."]
        Small,
        #[doc = "Returns a more accurate response using a large sampling size. But this\nmay result in response being slower."]
        Large,
    }
    impl ReportRequestSamplingLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportRequestSamplingLevel::SamplingUnspecified => "SAMPLING_UNSPECIFIED",
                ReportRequestSamplingLevel::Default => "DEFAULT",
                ReportRequestSamplingLevel::Small => "SMALL",
                ReportRequestSamplingLevel::Large => "LARGE",
            }
        }
    }
    impl ::std::fmt::Display for ReportRequestSamplingLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportRequestSamplingLevel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportRequestSamplingLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SAMPLING_UNSPECIFIED" => ReportRequestSamplingLevel::SamplingUnspecified,
                "DEFAULT" => ReportRequestSamplingLevel::Default,
                "SMALL" => ReportRequestSamplingLevel::Small,
                "LARGE" => ReportRequestSamplingLevel::Large,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReportRequest {
        #[doc = "Cohort group associated with this request. If there is a cohort group\nin the request the `ga:cohort` dimension must be present.\nEvery [ReportRequest](#ReportRequest) within a `batchGet` method must\ncontain the same `cohortGroup` definition."]
        #[serde(rename = "cohortGroup", default)]
        pub cohort_group: Option<crate::schemas::CohortGroup>,
        #[doc = "Date ranges in the request. The request can have a maximum of 2 date\nranges. The response will contain a set of metric values for each\ncombination of the dimensions for each date range in the request. So, if\nthere are two date ranges, there will be two set of metric values, one for\nthe original date range and one for the second date range.\nThe `reportRequest.dateRanges` field should not be specified for cohorts\nor Lifetime value requests.\nIf a date range is not provided, the default date range is (startDate:\ncurrent date - 7 days, endDate: current date - 1 day). Every\n[ReportRequest](#ReportRequest) within a `batchGet` method must\ncontain the same `dateRanges` definition."]
        #[serde(rename = "dateRanges", default)]
        pub date_ranges: Option<Vec<crate::schemas::DateRange>>,
        #[doc = "The dimension filter clauses for filtering Dimension Values. They are\nlogically combined with the `AND` operator. Note that filtering occurs\nbefore any dimensions are aggregated, so that the returned metrics\nrepresent the total for only the relevant dimensions."]
        #[serde(rename = "dimensionFilterClauses", default)]
        pub dimension_filter_clauses: Option<Vec<crate::schemas::DimensionFilterClause>>,
        #[doc = "The dimensions requested.\nRequests can have a total of 9 dimensions."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: Option<Vec<crate::schemas::Dimension>>,
        #[doc = "Dimension or metric filters that restrict the data returned for your\nrequest. To use the `filtersExpression`, supply a dimension or metric on\nwhich to filter, followed by the filter expression. For example, the\nfollowing expression selects `ga:browser` dimension which starts with\nFirefox; `ga:browser=~^Firefox`. For more information on dimensions\nand metric filters, see\n[Filters\nreference](https://developers.google.com/analytics/devguides/reporting/core/v3/reference#filters)."]
        #[serde(rename = "filtersExpression", default)]
        pub filters_expression: Option<String>,
        #[doc = "If set to true, hides the total of all metrics for all the matching rows,\nfor every date range. The default false and will return the totals."]
        #[serde(rename = "hideTotals", default)]
        pub hide_totals: Option<bool>,
        #[doc = "If set to true, hides the minimum and maximum across all matching rows.\nThe default is false and the value ranges are returned."]
        #[serde(rename = "hideValueRanges", default)]
        pub hide_value_ranges: Option<bool>,
        #[doc = "If set to false, the response does not include rows if all the retrieved\nmetrics are equal to zero. The default is false which will exclude these\nrows."]
        #[serde(rename = "includeEmptyRows", default)]
        pub include_empty_rows: Option<bool>,
        #[doc = "The metric filter clauses. They are logically combined with the `AND`\noperator.  Metric filters look at only the first date range and not the\ncomparing date range. Note that filtering on metrics occurs after the\nmetrics are aggregated."]
        #[serde(rename = "metricFilterClauses", default)]
        pub metric_filter_clauses: Option<Vec<crate::schemas::MetricFilterClause>>,
        #[doc = "The metrics requested.\nRequests must specify at least one metric. Requests can have a\ntotal of 10 metrics."]
        #[serde(rename = "metrics", default)]
        pub metrics: Option<Vec<crate::schemas::Metric>>,
        #[doc = "Sort order on output rows. To compare two rows, the elements of the\nfollowing are applied in order until a difference is found.  All date\nranges in the output get the same row order."]
        #[serde(rename = "orderBys", default)]
        pub order_bys: Option<Vec<crate::schemas::OrderBy>>,
        #[doc = "Page size is for paging and specifies the maximum number of returned rows.\nPage size should be >= 0. A query returns the default of 1,000 rows.\nThe Analytics Core Reporting API returns a maximum of 100,000 rows per\nrequest, no matter how many you ask for. It can also return fewer rows\nthan requested, if there aren't as many dimension segments as you expect.\nFor instance, there are fewer than 300 possible values for `ga:country`,\nso when segmenting only by country, you can't get more than 300 rows,\neven if you set `pageSize` to a higher value."]
        #[serde(rename = "pageSize", default)]
        pub page_size: Option<i32>,
        #[doc = "A continuation token to get the next page of the results. Adding this to\nthe request will return the rows after the pageToken. The pageToken should\nbe the value returned in the nextPageToken parameter in the response to\nthe GetReports request."]
        #[serde(rename = "pageToken", default)]
        pub page_token: Option<String>,
        #[doc = "The pivot definitions. Requests can have a maximum of 2 pivots."]
        #[serde(rename = "pivots", default)]
        pub pivots: Option<Vec<crate::schemas::Pivot>>,
        #[doc = "The desired report\n[sample](https://support.google.com/analytics/answer/2637192) size.\nIf the the `samplingLevel` field is unspecified the `DEFAULT` sampling\nlevel is used. Every [ReportRequest](#ReportRequest) within a\n`batchGet` method must contain the same `samplingLevel` definition. See\n[developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)\nfor details."]
        #[serde(rename = "samplingLevel", default)]
        pub sampling_level: Option<crate::schemas::ReportRequestSamplingLevel>,
        #[doc = "Segment the data returned for the request. A segment definition helps look\nat a subset of the segment request. A request can contain up to four\nsegments. Every [ReportRequest](#ReportRequest) within a\n`batchGet` method must contain the same `segments` definition. Requests\nwith segments must have the `ga:segment` dimension."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::Segment>>,
        #[doc = "The Analytics\n[view ID](https://support.google.com/analytics/answer/1009618)\nfrom which to retrieve data. Every [ReportRequest](#ReportRequest)\nwithin a `batchGet` method must contain the same `viewId`."]
        #[serde(rename = "viewId", default)]
        pub view_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ReportRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReportRow {
        #[doc = "List of requested dimensions."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: Option<Vec<String>>,
        #[doc = "List of metrics for each requested DateRange."]
        #[serde(rename = "metrics", default)]
        pub metrics: Option<Vec<crate::schemas::DateRangeValues>>,
    }
    impl ::field_selector::FieldSelector for ReportRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResourceQuotasRemaining {
        #[doc = "Daily resource quota remaining remaining."]
        #[serde(rename = "dailyQuotaTokensRemaining", default)]
        pub daily_quota_tokens_remaining: Option<i32>,
        #[doc = "Hourly resource quota tokens remaining."]
        #[serde(rename = "hourlyQuotaTokensRemaining", default)]
        pub hourly_quota_tokens_remaining: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ResourceQuotasRemaining {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ScreenviewData {
        #[doc = "The application name."]
        #[serde(rename = "appName", default)]
        pub app_name: Option<String>,
        #[doc = "Mobile manufacturer or branded name. Eg: \"Google\", \"Apple\" etc."]
        #[serde(rename = "mobileDeviceBranding", default)]
        pub mobile_device_branding: Option<String>,
        #[doc = "Mobile device model. Eg: \"Pixel\", \"iPhone\" etc."]
        #[serde(rename = "mobileDeviceModel", default)]
        pub mobile_device_model: Option<String>,
        #[doc = "The name of the screen."]
        #[serde(rename = "screenName", default)]
        pub screen_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ScreenviewData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SearchUserActivityRequestActivityTypesItems {}
    impl SearchUserActivityRequestActivityTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for SearchUserActivityRequestActivityTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SearchUserActivityRequestActivityTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SearchUserActivityRequestActivityTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchUserActivityRequest {
        #[doc = "Set of all activity types being requested. Only acvities matching these\ntypes will be returned in the response. If empty, all activies will be\nreturned."]
        #[serde(rename = "activityTypes", default)]
        pub activity_types:
            Option<Vec<crate::schemas::SearchUserActivityRequestActivityTypesItems>>,
        #[doc = "Date range for which to retrieve the user activity. If a date range is not\nprovided, the default date range is (startDate: current date - 7 days,\nendDate: current date - 1 day)."]
        #[serde(rename = "dateRange", default)]
        pub date_range: Option<crate::schemas::DateRange>,
        #[doc = "Page size is for paging and specifies the maximum number of returned rows.\nPage size should be > 0. If the value is 0 or if the field isn't specified,\nthe request returns the default of 1000 rows per page."]
        #[serde(rename = "pageSize", default)]
        pub page_size: Option<i32>,
        #[doc = "A continuation token to get the next page of the results. Adding this to\nthe request will return the rows after the pageToken. The pageToken should\nbe the value returned in the nextPageToken parameter in the response to\nthe [SearchUserActivityRequest](#SearchUserActivityRequest) request."]
        #[serde(rename = "pageToken", default)]
        pub page_token: Option<String>,
        #[doc = "Required. Unique user Id to query for. Every\n[SearchUserActivityRequest](#SearchUserActivityRequest) must contain this\nfield."]
        #[serde(rename = "user", default)]
        pub user: Option<crate::schemas::User>,
        #[doc = "Required. The Analytics\n[view ID](https://support.google.com/analytics/answer/1009618)\nfrom which to retrieve data. Every\n[SearchUserActivityRequest](#SearchUserActivityRequest) must contain the\n`viewId`."]
        #[serde(rename = "viewId", default)]
        pub view_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchUserActivityRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchUserActivityResponse {
        #[doc = "This token should be passed to\n[SearchUserActivityRequest](#SearchUserActivityRequest) to retrieve the\nnext page."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "This field represents the\n[sampling rate](https://support.google.com/analytics/answer/2637192) for\nthe given request and is a number between 0.0 to 1.0. See\n[developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)\nfor details."]
        #[serde(rename = "sampleRate", default)]
        pub sample_rate: Option<f64>,
        #[doc = "Each record represents a session (device details, duration, etc)."]
        #[serde(rename = "sessions", default)]
        pub sessions: Option<Vec<crate::schemas::UserActivitySession>>,
        #[doc = "Total rows returned by this query (across different pages)."]
        #[serde(rename = "totalRows", default)]
        pub total_rows: Option<i32>,
    }
    impl ::field_selector::FieldSelector for SearchUserActivityResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Segment {
        #[doc = "A dynamic segment definition in the request."]
        #[serde(rename = "dynamicSegment", default)]
        pub dynamic_segment: Option<crate::schemas::DynamicSegment>,
        #[doc = "The segment ID of a built-in or custom segment, for example `gaid::-3`."]
        #[serde(rename = "segmentId", default)]
        pub segment_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Segment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentDefinition {
        #[doc = "A segment is defined by a set of segment filters which are combined\ntogether with a logical `AND` operation."]
        #[serde(rename = "segmentFilters", default)]
        pub segment_filters: Option<Vec<crate::schemas::SegmentFilter>>,
    }
    impl ::field_selector::FieldSelector for SegmentDefinition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SegmentDimensionFilterOperator {
        #[doc = "If the match type is unspecified, it is treated as a REGEXP."]
        OperatorUnspecified,
        #[doc = "The match expression is treated as a regular expression. All other match\ntypes are not treated as regular expressions."]
        Regexp,
        #[doc = "Matches the values which begin with the match expression provided."]
        BeginsWith,
        #[doc = "Matches the values which end with the match expression provided."]
        EndsWith,
        #[doc = "Substring match."]
        Partial,
        #[doc = "The value should match the match expression entirely."]
        Exact,
        #[doc = "This option is used to specify a dimension filter whose expression can\ntake any value from a selected list of values. This helps avoiding\nevaluating multiple exact match dimension filters which are OR'ed for\nevery single response row. For example:\n\n````text\nexpressions: [\"A\", \"B\", \"C\"]\n````\n\nAny response row whose dimension has it is value as A, B or C, matches\nthis DimensionFilter."]
        InList,
        #[doc = "Integer comparison filters.\ncase sensitivity is ignored for these and the expression\nis assumed to be a string representing an integer.\nFailure conditions:\n\n* if expression is not a valid int64, the client should expect\n  an error.\n* input dimensions that are not valid int64 values will never match the\n  filter.\n\nChecks if the dimension is numerically less than the match expression."]
        NumericLessThan,
        #[doc = "Checks if the dimension is numerically greater than the match\nexpression."]
        NumericGreaterThan,
        #[doc = "Checks if the dimension is numerically between the minimum and maximum\nof the match expression, boundaries excluded."]
        NumericBetween,
    }
    impl SegmentDimensionFilterOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                SegmentDimensionFilterOperator::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
                SegmentDimensionFilterOperator::Regexp => "REGEXP",
                SegmentDimensionFilterOperator::BeginsWith => "BEGINS_WITH",
                SegmentDimensionFilterOperator::EndsWith => "ENDS_WITH",
                SegmentDimensionFilterOperator::Partial => "PARTIAL",
                SegmentDimensionFilterOperator::Exact => "EXACT",
                SegmentDimensionFilterOperator::InList => "IN_LIST",
                SegmentDimensionFilterOperator::NumericLessThan => "NUMERIC_LESS_THAN",
                SegmentDimensionFilterOperator::NumericGreaterThan => "NUMERIC_GREATER_THAN",
                SegmentDimensionFilterOperator::NumericBetween => "NUMERIC_BETWEEN",
            }
        }
    }
    impl ::std::fmt::Display for SegmentDimensionFilterOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SegmentDimensionFilterOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SegmentDimensionFilterOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATOR_UNSPECIFIED" => SegmentDimensionFilterOperator::OperatorUnspecified,
                "REGEXP" => SegmentDimensionFilterOperator::Regexp,
                "BEGINS_WITH" => SegmentDimensionFilterOperator::BeginsWith,
                "ENDS_WITH" => SegmentDimensionFilterOperator::EndsWith,
                "PARTIAL" => SegmentDimensionFilterOperator::Partial,
                "EXACT" => SegmentDimensionFilterOperator::Exact,
                "IN_LIST" => SegmentDimensionFilterOperator::InList,
                "NUMERIC_LESS_THAN" => SegmentDimensionFilterOperator::NumericLessThan,
                "NUMERIC_GREATER_THAN" => SegmentDimensionFilterOperator::NumericGreaterThan,
                "NUMERIC_BETWEEN" => SegmentDimensionFilterOperator::NumericBetween,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentDimensionFilter {
        #[doc = "Should the match be case sensitive, ignored for `IN_LIST` operator."]
        #[serde(rename = "caseSensitive", default)]
        pub case_sensitive: Option<bool>,
        #[doc = "Name of the dimension for which the filter is being applied."]
        #[serde(rename = "dimensionName", default)]
        pub dimension_name: Option<String>,
        #[doc = "The list of expressions, only the first element is used for all operators"]
        #[serde(rename = "expressions", default)]
        pub expressions: Option<Vec<String>>,
        #[doc = "Maximum comparison values for `BETWEEN` match type."]
        #[serde(rename = "maxComparisonValue", default)]
        pub max_comparison_value: Option<String>,
        #[doc = "Minimum comparison values for `BETWEEN` match type."]
        #[serde(rename = "minComparisonValue", default)]
        pub min_comparison_value: Option<String>,
        #[doc = "The operator to use to match the dimension with the expressions."]
        #[serde(rename = "operator", default)]
        pub operator: Option<crate::schemas::SegmentDimensionFilterOperator>,
    }
    impl ::field_selector::FieldSelector for SegmentDimensionFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentFilter {
        #[doc = "If true, match the complement of simple or sequence segment.\nFor example, to match all visits not from \"New York\", we can define the\nsegment as follows:\n\n````text\n  \"sessionSegment\": {\n    \"segmentFilters\": [{\n      \"simpleSegment\" :{\n        \"orFiltersForSegment\": [{\n          \"segmentFilterClauses\":[{\n            \"dimensionFilter\": {\n              \"dimensionName\": \"ga:city\",\n              \"expressions\": [\"New York\"]\n            }\n          }]\n        }]\n      },\n      \"not\": \"True\"\n    }]\n  },````"]
        #[serde(rename = "not", default)]
        pub not: Option<bool>,
        #[doc = "Sequence conditions consist of one or more steps, where each step is\ndefined by one or more dimension/metric conditions. Multiple steps can\nbe combined with special sequence operators."]
        #[serde(rename = "sequenceSegment", default)]
        pub sequence_segment: Option<crate::schemas::SequenceSegment>,
        #[doc = "A Simple segment conditions consist of one or more dimension/metric\nconditions that can be combined"]
        #[serde(rename = "simpleSegment", default)]
        pub simple_segment: Option<crate::schemas::SimpleSegment>,
    }
    impl ::field_selector::FieldSelector for SegmentFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentFilterClause {
        #[doc = "Dimension Filter for the segment definition."]
        #[serde(rename = "dimensionFilter", default)]
        pub dimension_filter: Option<crate::schemas::SegmentDimensionFilter>,
        #[doc = "Metric Filter for the segment definition."]
        #[serde(rename = "metricFilter", default)]
        pub metric_filter: Option<crate::schemas::SegmentMetricFilter>,
        #[doc = "Matches the complement (`!`) of the filter."]
        #[serde(rename = "not", default)]
        pub not: Option<bool>,
    }
    impl ::field_selector::FieldSelector for SegmentFilterClause {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SegmentMetricFilterOperator {
        #[doc = "Unspecified operator is treated as `LESS_THAN` operator."]
        UnspecifiedOperator,
        #[doc = "Checks if the metric value is less than comparison value."]
        LessThan,
        #[doc = "Checks if the metric value is greater than comparison value."]
        GreaterThan,
        #[doc = "Equals operator."]
        Equal,
        #[doc = "For between operator, both the minimum and maximum are exclusive.\nWe will use `LT` and `GT` for comparison."]
        Between,
    }
    impl SegmentMetricFilterOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                SegmentMetricFilterOperator::UnspecifiedOperator => "UNSPECIFIED_OPERATOR",
                SegmentMetricFilterOperator::LessThan => "LESS_THAN",
                SegmentMetricFilterOperator::GreaterThan => "GREATER_THAN",
                SegmentMetricFilterOperator::Equal => "EQUAL",
                SegmentMetricFilterOperator::Between => "BETWEEN",
            }
        }
    }
    impl ::std::fmt::Display for SegmentMetricFilterOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SegmentMetricFilterOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SegmentMetricFilterOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED_OPERATOR" => SegmentMetricFilterOperator::UnspecifiedOperator,
                "LESS_THAN" => SegmentMetricFilterOperator::LessThan,
                "GREATER_THAN" => SegmentMetricFilterOperator::GreaterThan,
                "EQUAL" => SegmentMetricFilterOperator::Equal,
                "BETWEEN" => SegmentMetricFilterOperator::Between,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SegmentMetricFilterScope {
        #[doc = "If the scope is unspecified, it defaults to the condition scope,\n`USER` or `SESSION` depending on if the segment is trying to choose\nusers or sessions."]
        UnspecifiedScope,
        #[doc = "Product scope."]
        Product,
        #[doc = "Hit scope."]
        Hit,
        #[doc = "Session scope."]
        Session,
        #[doc = "User scope."]
        User,
    }
    impl SegmentMetricFilterScope {
        pub fn as_str(self) -> &'static str {
            match self {
                SegmentMetricFilterScope::UnspecifiedScope => "UNSPECIFIED_SCOPE",
                SegmentMetricFilterScope::Product => "PRODUCT",
                SegmentMetricFilterScope::Hit => "HIT",
                SegmentMetricFilterScope::Session => "SESSION",
                SegmentMetricFilterScope::User => "USER",
            }
        }
    }
    impl ::std::fmt::Display for SegmentMetricFilterScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SegmentMetricFilterScope {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SegmentMetricFilterScope {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED_SCOPE" => SegmentMetricFilterScope::UnspecifiedScope,
                "PRODUCT" => SegmentMetricFilterScope::Product,
                "HIT" => SegmentMetricFilterScope::Hit,
                "SESSION" => SegmentMetricFilterScope::Session,
                "USER" => SegmentMetricFilterScope::User,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentMetricFilter {
        #[doc = "The value to compare against. If the operator is `BETWEEN`, this value is\ntreated as minimum comparison value."]
        #[serde(rename = "comparisonValue", default)]
        pub comparison_value: Option<String>,
        #[doc = "Max comparison value is only used for `BETWEEN` operator."]
        #[serde(rename = "maxComparisonValue", default)]
        pub max_comparison_value: Option<String>,
        #[doc = "The metric that will be filtered on. A `metricFilter` must contain a\nmetric name."]
        #[serde(rename = "metricName", default)]
        pub metric_name: Option<String>,
        #[doc = "Specifies is the operation to perform to compare the metric. The default\nis `EQUAL`."]
        #[serde(rename = "operator", default)]
        pub operator: Option<crate::schemas::SegmentMetricFilterOperator>,
        #[doc = "Scope for a metric defines the level at which that metric is defined.  The\nspecified metric scope must be equal to or greater than its primary scope\nas defined in the data model. The primary scope is defined by if the\nsegment is selecting users or sessions."]
        #[serde(rename = "scope", default)]
        pub scope: Option<crate::schemas::SegmentMetricFilterScope>,
    }
    impl ::field_selector::FieldSelector for SegmentMetricFilter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SegmentSequenceStepMatchType {
        #[doc = "Unspecified match type is treated as precedes."]
        UnspecifiedMatchType,
        #[doc = "Operator indicates that the previous step precedes the next step."]
        Precedes,
        #[doc = "Operator indicates that the previous step immediately precedes the next\nstep."]
        ImmediatelyPrecedes,
    }
    impl SegmentSequenceStepMatchType {
        pub fn as_str(self) -> &'static str {
            match self {
                SegmentSequenceStepMatchType::UnspecifiedMatchType => "UNSPECIFIED_MATCH_TYPE",
                SegmentSequenceStepMatchType::Precedes => "PRECEDES",
                SegmentSequenceStepMatchType::ImmediatelyPrecedes => "IMMEDIATELY_PRECEDES",
            }
        }
    }
    impl ::std::fmt::Display for SegmentSequenceStepMatchType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SegmentSequenceStepMatchType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SegmentSequenceStepMatchType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED_MATCH_TYPE" => SegmentSequenceStepMatchType::UnspecifiedMatchType,
                "PRECEDES" => SegmentSequenceStepMatchType::Precedes,
                "IMMEDIATELY_PRECEDES" => SegmentSequenceStepMatchType::ImmediatelyPrecedes,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentSequenceStep {
        #[doc = "Specifies if the step immediately precedes or can be any time before the\nnext step."]
        #[serde(rename = "matchType", default)]
        pub match_type: Option<crate::schemas::SegmentSequenceStepMatchType>,
        #[doc = "A sequence is specified with a list of Or grouped filters which are\ncombined with `AND` operator."]
        #[serde(rename = "orFiltersForSegment", default)]
        pub or_filters_for_segment: Option<Vec<crate::schemas::OrFiltersForSegment>>,
    }
    impl ::field_selector::FieldSelector for SegmentSequenceStep {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SequenceSegment {
        #[doc = "If set, first step condition must match the first hit of the visitor (in\nthe date range)."]
        #[serde(rename = "firstStepShouldMatchFirstHit", default)]
        pub first_step_should_match_first_hit: Option<bool>,
        #[doc = "The list of steps in the sequence."]
        #[serde(rename = "segmentSequenceSteps", default)]
        pub segment_sequence_steps: Option<Vec<crate::schemas::SegmentSequenceStep>>,
    }
    impl ::field_selector::FieldSelector for SequenceSegment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SimpleSegment {
        #[doc = "A list of segment filters groups which are combined with logical `AND`\noperator."]
        #[serde(rename = "orFiltersForSegment", default)]
        pub or_filters_for_segment: Option<Vec<crate::schemas::OrFiltersForSegment>>,
    }
    impl ::field_selector::FieldSelector for SimpleSegment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TransactionData {
        #[doc = "The transaction ID, supplied by the e-commerce tracking method, for the\npurchase in the shopping cart."]
        #[serde(rename = "transactionId", default)]
        pub transaction_id: Option<String>,
        #[doc = "The total sale revenue (excluding shipping and tax) of the transaction."]
        #[serde(rename = "transactionRevenue", default)]
        pub transaction_revenue: Option<f64>,
        #[doc = "Total cost of shipping."]
        #[serde(rename = "transactionShipping", default)]
        pub transaction_shipping: Option<f64>,
        #[doc = "Total tax for the transaction."]
        #[serde(rename = "transactionTax", default)]
        pub transaction_tax: Option<f64>,
    }
    impl ::field_selector::FieldSelector for TransactionData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserType {
        #[doc = "When the User Id Type is not specified, the default type used will be\nCLIENT_ID."]
        UserIdTypeUnspecified,
        #[doc = "A single user, like a signed-in user account, that may interact with\ncontent across one or more devices and / or browser instances."]
        UserId,
        #[doc = "Analytics assigned client_id."]
        ClientId,
    }
    impl UserType {
        pub fn as_str(self) -> &'static str {
            match self {
                UserType::UserIdTypeUnspecified => "USER_ID_TYPE_UNSPECIFIED",
                UserType::UserId => "USER_ID",
                UserType::ClientId => "CLIENT_ID",
            }
        }
    }
    impl ::std::fmt::Display for UserType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_ID_TYPE_UNSPECIFIED" => UserType::UserIdTypeUnspecified,
                "USER_ID" => UserType::UserId,
                "CLIENT_ID" => UserType::ClientId,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct User {
        #[doc = "Type of the user in the request. The field `userId` is associated with this\ntype."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::UserType>,
        #[doc = "Unique Id of the user for which the data is being requested."]
        #[serde(rename = "userId", default)]
        pub user_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for User {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UserActivitySession {
        #[doc = "Represents a detailed view into each of the activity in this session."]
        #[serde(rename = "activities", default)]
        pub activities: Option<Vec<crate::schemas::Activity>>,
        #[doc = "The data source of a hit. By default, hits sent from analytics.js are\nreported as \"web\" and hits sent from the mobile SDKs are reported as \"app\".\nThese values can be overridden in the Measurement Protocol."]
        #[serde(rename = "dataSource", default)]
        pub data_source: Option<String>,
        #[doc = "The type of device used: \"mobile\", \"tablet\" etc."]
        #[serde(rename = "deviceCategory", default)]
        pub device_category: Option<String>,
        #[doc = "Platform on which the activity happened: \"android\", \"ios\" etc."]
        #[serde(rename = "platform", default)]
        pub platform: Option<String>,
        #[doc = "Date of this session in ISO-8601 format."]
        #[serde(rename = "sessionDate", default)]
        pub session_date: Option<String>,
        #[doc = "Unique ID of the session."]
        #[serde(rename = "sessionId", default)]
        pub session_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for UserActivitySession {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    #[doc = "Actions that can be performed on the reports resource"]
    pub fn reports(&self) -> crate::reports::ReportsActions<A> {
        crate::reports::ReportsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the user_activity resource"]
    pub fn user_activity(&self) -> crate::user_activity::UserActivityActions<A> {
        crate::user_activity::UserActivityActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod reports {
    pub mod params {}
    pub struct ReportsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ReportsActions<'a, A> {
        #[doc = "Returns the Analytics data."]
        pub fn batch_get(
            &self,
            request: crate::schemas::GetReportsRequest,
        ) -> BatchGetRequestBuilder<A> {
            BatchGetRequestBuilder {
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct BatchGetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::GetReportsRequest,
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
    impl<'a, A: yup_oauth2::GetToken> BatchGetRequestBuilder<'a, A> {
        #[doc = "OAuth access token."]
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GetReportsResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://analyticsreporting.googleapis.com/".to_owned();
            output.push_str("v4/reports:batchGet");
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/analytics"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod user_activity {
    pub mod params {}
    pub struct UserActivityActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> UserActivityActions<'a, A> {
        #[doc = "Returns User Activity data."]
        pub fn search(
            &self,
            request: crate::schemas::SearchUserActivityRequest,
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct SearchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::SearchUserActivityRequest,
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
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::SearchUserActivityResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://analyticsreporting.googleapis.com/".to_owned();
            output.push_str("v4/userActivity:search");
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/analytics"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
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
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
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
        let req = req.body(::reqwest::Body::sized(reader, content_length));
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
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
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

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}
