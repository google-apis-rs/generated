#![doc = "# Resources and Methods\n    * [billing_accounts](resources/billing_accounts/struct.BillingAccountsActions.html)\n      * [budgets](resources/billing_accounts/budgets/struct.BudgetsActions.html)\n        * [*create*](resources/billing_accounts/budgets/struct.CreateRequestBuilder.html), [*delete*](resources/billing_accounts/budgets/struct.DeleteRequestBuilder.html), [*get*](resources/billing_accounts/budgets/struct.GetRequestBuilder.html), [*list*](resources/billing_accounts/budgets/struct.ListRequestBuilder.html), [*patch*](resources/billing_accounts/budgets/struct.PatchRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your Google Cloud Platform billing accounts\n\n`https://www.googleapis.com/auth/cloud-billing`"]
    pub const CLOUD_BILLING: &str = "https://www.googleapis.com/auth/cloud-billing";
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
    pub struct GoogleCloudBillingBudgetsV1Beta1AllUpdatesRule {
        #[doc = "Optional. When set to true, disables default notifications sent when a threshold is exceeded. Default notifications are sent to those with Billing Account Administrator and Billing Account User IAM roles for the target account."]
        #[serde(
            rename = "disableDefaultIamRecipients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_default_iam_recipients: ::std::option::Option<bool>,
        #[doc = "Optional. Targets to send notifications to when a threshold is exceeded. This is in addition to default recipients who have billing account IAM roles. The value is the full REST resource name of a monitoring notification channel with the form `projects/{project_id}/notificationChannels/{channel_id}`. A maximum of 5 channels are allowed. See https://cloud.google.com/billing/docs/how-to/budgets-notification-recipients for more details."]
        #[serde(
            rename = "monitoringNotificationChannels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitoring_notification_channels: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The name of the Pub/Sub topic where budget related messages will be published, in the form `projects/{project_id}/topics/{topic_id}`. Updates are sent at regular intervals to the topic. The topic needs to be created before the budget is created; see https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications for more details. Caller is expected to have `pubsub.topics.setIamPolicy` permission on the topic when it's set for a budget, otherwise, the API call will fail with PERMISSION_DENIED. See https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#permissions_required_for_this_task for more details on Pub/Sub roles and permissions."]
        #[serde(
            rename = "pubsubTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pubsub_topic: ::std::option::Option<String>,
        #[doc = "Optional. Required when AllUpdatesRule.pubsub_topic is set. The schema version of the notification sent to AllUpdatesRule.pubsub_topic. Only \"1.0\" is accepted. It represents the JSON schema as defined in https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format."]
        #[serde(
            rename = "schemaVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1AllUpdatesRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1AllUpdatesRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudBillingBudgetsV1Beta1Budget {
        #[doc = "Optional. Rules to apply to notifications sent based on budget spend and thresholds."]
        #[serde(
            rename = "allUpdatesRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_updates_rule:
            ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1AllUpdatesRule>,
        #[doc = "Required. Budgeted amount."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount:
            ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1BudgetAmount>,
        #[doc = "Optional. Filters that define which resources are used to compute the actual spend against the budget."]
        #[serde(
            rename = "budgetFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub budget_filter:
            ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Filter>,
        #[doc = "User data for display name in UI. Validation: <= 60 chars."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag will cause an update to overwrite other changes."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget."]
        #[serde(
            rename = "thresholdRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threshold_rules: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudBillingBudgetsV1Beta1ThresholdRule>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1Budget {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1Budget {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudBillingBudgetsV1Beta1BudgetAmount {
        #[doc = "Use the last period's actual spend as the budget for the present period. Cannot be set in combination with Filter.custom_period."]
        #[serde(
            rename = "lastPeriodAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_period_amount:
            ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1LastPeriodAmount>,
        #[doc = "A specified amount to use as the budget. `currency_code` is optional. If specified when creating a budget, it must match the currency of the billing account. If specified when updating a budget, it must match the currency_code of the existing budget. The `currency_code` is provided on output."]
        #[serde(
            rename = "specifiedAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub specified_amount: ::std::option::Option<crate::schemas::GoogleTypeMoney>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1BudgetAmount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1BudgetAmount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudBillingBudgetsV1Beta1CreateBudgetRequest {
        #[doc = "Required. Budget to create."]
        #[serde(
            rename = "budget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub budget: ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudBillingBudgetsV1Beta1CreateBudgetRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1CreateBudgetRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudBillingBudgetsV1Beta1CustomPeriod {
        #[doc = "Optional. The end date of the time period. Budgets with elapsed end date won't be processed. If unset, specifies to track all usage incurred since the start_date."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "Required. The start date must be after January 1, 2017."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::GoogleTypeDate>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1CustomPeriod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1CustomPeriod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudBillingBudgetsV1Beta1Filter {
        #[doc = "Optional. Specifies to track usage for recurring calendar period. E.g. Assume that CalendarPeriod.QUARTER is set. The budget will track usage from April 1 to June 30, when current calendar month is April, May, June. After that, it will track usage from July 1 to September 30 when current calendar month is July, August, September, and so on."]
        #[serde(
            rename = "calendarPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub calendar_period: ::std::option::Option<
            crate::schemas::GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod,
        >,
        #[doc = "Optional. If Filter.credit_types_treatment is INCLUDE_SPECIFIED_CREDITS, this is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. If Filter.credit_types_treatment is **not** INCLUDE_SPECIFIED_CREDITS, this field must be empty. See [a list of acceptable credit type values](https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type)."]
        #[serde(
            rename = "creditTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credit_types: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`."]
        #[serde(
            rename = "creditTypesTreatment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credit_types_treatment: ::std::option::Option<
            crate::schemas::GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment,
        >,
        #[doc = "Optional. Specifies to track usage from any start date (required) to any end date (optional)."]
        #[serde(
            rename = "customPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_period:
            ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1CustomPeriod>,
        #[doc = "Optional. A single label and value pair specifying that usage from only this set of labeled resources should be included in the budget. Currently, multiple entries or multiple values per entry are not allowed. If omitted, the report will include all labeled and unlabeled usage."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, Vec<::serde_json::Value>>>,
        #[doc = "Optional. A set of projects of the form `projects/{project}`, specifying that usage from only this set of projects should be included in the budget. If omitted, the report will include all usage for the billing account, regardless of which project the usage occurred on. Only zero or one project can be specified currently."]
        #[serde(
            rename = "projects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub projects: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. A set of services of the form `services/{service_id}`, specifying that usage from only this set of services should be included in the budget. If omitted, the report will include usage for all the services. The service names are available through the Catalog API: https://cloud.google.com/billing/v1/how-tos/catalog-api."]
        #[serde(
            rename = "services",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub services: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. A set of subaccounts of the form `billingAccounts/{account_id}`, specifying that usage from only this set of subaccounts should be included in the budget. If a subaccount is set to the name of the parent account, usage from the parent account will be included. If omitted, the report will include usage from the parent account and all subaccounts, if they exist."]
        #[serde(
            rename = "subaccounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subaccounts: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1Filter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1Filter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        CalendarPeriodUnspecified,
        #[doc = "A month. Month starts on the first day of each month, such as January 1, February 1, March 1, and so on."]
        Month,
        #[doc = "A quarter. Quarters start on dates January 1, April 1, July 1, and October 1 of each year."]
        Quarter,
        #[doc = "A year. Year starts on January 1."]
        Year,
    }
    impl GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::CalendarPeriodUnspecified => {
                    "CALENDAR_PERIOD_UNSPECIFIED"
                }
                GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Month => "MONTH",
                GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Quarter => "QUARTER",
                GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Year => "YEAR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod, ()>
        {
            Ok(match s {
                "CALENDAR_PERIOD_UNSPECIFIED" => {
                    GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::CalendarPeriodUnspecified
                }
                "MONTH" => GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Month,
                "QUARTER" => GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Quarter,
                "YEAR" => GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Year,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CALENDAR_PERIOD_UNSPECIFIED" => {
                    GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::CalendarPeriodUnspecified
                }
                "MONTH" => GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Month,
                "QUARTER" => GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Quarter,
                "YEAR" => GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod::Year,
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
        for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1FilterCalendarPeriod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        CreditTypesTreatmentUnspecified,
        #[doc = "All types of credit are added to the net cost to determine the spend for threshold calculations."]
        ExcludeAllCredits,
        #[doc = "All types of credit are subtracted from the gross cost to determine the spend for threshold calculations."]
        IncludeAllCredits,
        #[doc = "Credit types specified in the credit_types field are subtracted from the gross cost to determine the spend for threshold calculations."]
        IncludeSpecifiedCredits,
    }
    impl GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: CreditTypesTreatmentUnspecified => "CREDIT_TYPES_TREATMENT_UNSPECIFIED" , GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: ExcludeAllCredits => "EXCLUDE_ALL_CREDITS" , GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: IncludeAllCredits => "INCLUDE_ALL_CREDITS" , GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: IncludeSpecifiedCredits => "INCLUDE_SPECIFIED_CREDITS" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment, ()>
        {
            Ok (match s { "CREDIT_TYPES_TREATMENT_UNSPECIFIED" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: CreditTypesTreatmentUnspecified , "EXCLUDE_ALL_CREDITS" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: ExcludeAllCredits , "INCLUDE_ALL_CREDITS" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: IncludeAllCredits , "INCLUDE_SPECIFIED_CREDITS" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: IncludeSpecifiedCredits , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CREDIT_TYPES_TREATMENT_UNSPECIFIED" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: CreditTypesTreatmentUnspecified , "EXCLUDE_ALL_CREDITS" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: ExcludeAllCredits , "INCLUDE_ALL_CREDITS" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: IncludeAllCredits , "INCLUDE_SPECIFIED_CREDITS" => GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment :: IncludeSpecifiedCredits , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudBillingBudgetsV1Beta1FilterCreditTypesTreatment
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudBillingBudgetsV1Beta1LastPeriodAmount {}
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1LastPeriodAmount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1LastPeriodAmount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudBillingBudgetsV1Beta1ListBudgetsResponse {
        #[doc = "List of the budgets owned by the requested billing account."]
        #[serde(
            rename = "budgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub budgets:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget>>,
        #[doc = "If not empty, indicates that there may be more budgets that match the request; this value should be passed in a new `ListBudgetsRequest`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudBillingBudgetsV1Beta1ListBudgetsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1ListBudgetsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudBillingBudgetsV1Beta1ThresholdRule {
        #[doc = "Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set."]
        #[serde(
            rename = "spendBasis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spend_basis: ::std::option::Option<
            crate::schemas::GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis,
        >,
        #[doc = "Required. Send an alert when this threshold is exceeded. This is a 1.0-based percentage, so 0.5 = 50%. Validation: non-negative number."]
        #[serde(
            rename = "thresholdPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threshold_percent: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudBillingBudgetsV1Beta1ThresholdRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1ThresholdRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        #[doc = "Unspecified threshold basis."]
        BasisUnspecified,
        #[doc = "Use current spend as the basis for comparison against the threshold."]
        CurrentSpend,
        #[doc = "Use forecasted spend for the period as the basis for comparison against the threshold. Cannot be set in combination with Filter.custom_period."]
        ForecastedSpend,
    }
    impl GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::BasisUnspecified => {
                    "BASIS_UNSPECIFIED"
                }
                GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::CurrentSpend => {
                    "CURRENT_SPEND"
                }
                GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::ForecastedSpend => {
                    "FORECASTED_SPEND"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis, ()>
        {
            Ok(match s {
                "BASIS_UNSPECIFIED" => {
                    GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::BasisUnspecified
                }
                "CURRENT_SPEND" => {
                    GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::CurrentSpend
                }
                "FORECASTED_SPEND" => {
                    GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::ForecastedSpend
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIS_UNSPECIFIED" => {
                    GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::BasisUnspecified
                }
                "CURRENT_SPEND" => {
                    GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::CurrentSpend
                }
                "FORECASTED_SPEND" => {
                    GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis::ForecastedSpend
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
        for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudBillingBudgetsV1Beta1ThresholdRuleSpendBasis
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudBillingBudgetsV1Beta1UpdateBudgetRequest {
        #[doc = "Required. The updated budget object. The budget to update is specified by the budget name in the budget."]
        #[serde(
            rename = "budget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub budget: ::std::option::Option<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget>,
        #[doc = "Optional. Indicates which fields in the provided budget to update. Read-only fields (such as `name`) cannot be changed. If this is not provided, then only fields with non-default values from the request are updated. See https://developers.google.com/protocol-buffers/docs/proto3#default for more details about default values."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudBillingBudgetsV1Beta1UpdateBudgetRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudBillingBudgetsV1Beta1UpdateBudgetRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct GoogleTypeDate {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
    pub struct GoogleTypeMoney {
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
    impl ::google_field_selector::FieldSelector for GoogleTypeMoney {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeMoney {
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
    #[doc = "Actions that can be performed on the billing_accounts resource"]
    pub fn billing_accounts(&self) -> crate::resources::billing_accounts::BillingAccountsActions {
        crate::resources::billing_accounts::BillingAccountsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod billing_accounts {
        pub mod params {}
        pub struct BillingAccountsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BillingAccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the budgets resource"]
            pub fn budgets(&self) -> crate::resources::billing_accounts::budgets::BudgetsActions {
                crate::resources::billing_accounts::budgets::BudgetsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod budgets {
            pub mod params {}
            pub struct BudgetsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> BudgetsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new budget. See Quotas and limits for more information on the limits of the number of budgets you can create."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudBillingBudgetsV1Beta1CreateBudgetRequest,
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
                #[doc = "Deletes a budget. Returns successfully if already deleted."]
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
                #[doc = "Returns a budget. WARNING: There are some fields exposed on the Google Cloud Console that aren't available on this API. When reading from the API, you will not see these fields in the return value, though they may have been set in the Cloud Console."]
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
                #[doc = "Returns a list of budgets for a billing account. WARNING: There are some fields exposed on the Google Cloud Console that aren't available on this API. When reading from the API, you will not see these fields in the return value, though they may have been set in the Cloud Console."]
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
                #[doc = "Updates a budget and returns the updated budget. WARNING: There are some fields exposed on the Google Cloud Console that aren't available on this API. Budget fields that are not exposed in this API will not be changed by this method."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GoogleCloudBillingBudgetsV1Beta1UpdateBudgetRequest,
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
            #[doc = "Created via [BudgetsActions::create()](struct.BudgetsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudBillingBudgetsV1Beta1CreateBudgetRequest,
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
                ) -> Result<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget, crate::Error>
                {
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
                    let mut output = "https://billingbudgets.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/budgets");
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
            #[doc = "Created via [BudgetsActions::delete()](struct.BudgetsActions.html#method.delete)"]
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
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
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
                    let mut output = "https://billingbudgets.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
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
            #[doc = "Created via [BudgetsActions::get()](struct.BudgetsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget, crate::Error>
                {
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
                    let mut output = "https://billingbudgets.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
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
            #[doc = "Created via [BudgetsActions::list()](struct.BudgetsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
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
                #[doc = "Optional. The maximum number of budgets to return per page. The default and maximum value are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. The value returned by the last `ListBudgetsResponse` which indicates that this is a continuation of a prior `ListBudgets` call, and that the system should return the next page of data."]
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
                ) -> Result<
                    crate::schemas::GoogleCloudBillingBudgetsV1Beta1ListBudgetsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudBillingBudgetsV1Beta1ListBudgetsResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://billingbudgets.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/budgets");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [BudgetsActions::patch()](struct.BudgetsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudBillingBudgetsV1Beta1UpdateBudgetRequest,
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
                ) -> Result<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudBillingBudgetsV1Beta1Budget, crate::Error>
                {
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
                    let mut output = "https://billingbudgets.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
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
