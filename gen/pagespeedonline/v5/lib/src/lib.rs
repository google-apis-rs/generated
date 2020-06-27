#![doc = "# Resources and Methods\n    * [pagespeedapi](resources/pagespeedapi/struct.PagespeedapiActions.html)\n      * [*runpagespeed*](resources/pagespeedapi/struct.RunpagespeedRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Associate you with your personal info on Google\n\n`openid`"]
    pub const OPENID: &str = "openid";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AuditRefs {
        #[doc = "The category group that the audit belongs to (optional)."]
        #[serde(
            rename = "group",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<String>,
        #[doc = "The audit ref id."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The weight this audit's score has on the overall category score."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weight: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for AuditRefs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditRefs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Bucket {
        #[doc = "Upper bound for a bucket's range."]
        #[serde(
            rename = "max",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max: ::std::option::Option<i32>,
        #[doc = "Lower bound for a bucket's range."]
        #[serde(
            rename = "min",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min: ::std::option::Option<i32>,
        #[doc = "The proportion of data in this bucket."]
        #[serde(
            rename = "proportion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proportion: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Bucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Bucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Categories {
        #[doc = "The accessibility category, containing all accessibility related audits."]
        #[serde(
            rename = "accessibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accessibility: ::std::option::Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The best practices category, containing all best practices related\naudits."]
        #[serde(
            rename = "best-practices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub best_practices: ::std::option::Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The performance category, containing all performance related audits."]
        #[serde(
            rename = "performance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub performance: ::std::option::Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The Progressive-Web-App (PWA) category, containing all pwa related\naudits."]
        #[serde(
            rename = "pwa",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pwa: ::std::option::Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The Search-Engine-Optimization (SEO) category, containing all seo related\naudits."]
        #[serde(
            rename = "seo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seo: ::std::option::Option<crate::schemas::LighthouseCategoryV5>,
    }
    impl ::google_field_selector::FieldSelector for Categories {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Categories {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CategoryGroupV5 {
        #[doc = "The description of what the category is grouping"]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The human readable title of the group"]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CategoryGroupV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CategoryGroupV5 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ConfigSettings {
        #[doc = "How Lighthouse was run, e.g. from the Chrome extension or from the npm\nmodule."]
        #[serde(
            rename = "channel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel: ::std::option::Option<String>,
        #[doc = "The form factor the emulation should use."]
        #[serde(
            rename = "emulatedFormFactor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub emulated_form_factor: ::std::option::Option<String>,
        #[doc = "The locale setting."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "List of categories of audits the run should conduct."]
        #[serde(
            rename = "onlyCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub only_categories: ::std::option::Option<::serde_json::Value>,
    }
    impl ::google_field_selector::FieldSelector for ConfigSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Environment {
        #[doc = "The benchmark index number that indicates rough device class."]
        #[serde(
            rename = "benchmarkIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub benchmark_index: ::std::option::Option<f64>,
        #[doc = "The user agent string of the version of Chrome used."]
        #[serde(
            rename = "hostUserAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_user_agent: ::std::option::Option<String>,
        #[doc = "The user agent string that was sent over the network."]
        #[serde(
            rename = "networkUserAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_user_agent: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Environment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Environment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct I18N {
        #[doc = "Internationalized strings that are formatted to the locale in\nconfigSettings."]
        #[serde(
            rename = "rendererFormattedStrings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub renderer_formatted_strings:
            ::std::option::Option<crate::schemas::RendererFormattedStrings>,
    }
    impl ::google_field_selector::FieldSelector for I18N {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for I18N {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LighthouseAuditResultV5 {
        #[doc = "The description of the audit."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Freeform details section of the audit."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The value that should be displayed on the UI for this audit."]
        #[serde(
            rename = "displayValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_value: ::std::option::Option<String>,
        #[doc = "An error message from a thrown error inside the audit."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "An explanation of the errors in the audit."]
        #[serde(
            rename = "explanation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explanation: ::std::option::Option<String>,
        #[doc = "The audit's id."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A numeric value that has a meaning specific to the audit, e.g. the number\nof nodes in the DOM or the timestamp of a specific load event. More\ninformation can be found in the audit details, if present."]
        #[serde(
            rename = "numericValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numeric_value: ::std::option::Option<f64>,
        #[doc = "The score of the audit, can be null."]
        #[serde(
            rename = "score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score: ::std::option::Option<::serde_json::Value>,
        #[doc = "The enumerated score display mode."]
        #[serde(
            rename = "scoreDisplayMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_display_mode: ::std::option::Option<String>,
        #[doc = "The human readable title."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Possible warnings that occurred in the audit, can be null."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<::serde_json::Value>,
    }
    impl ::google_field_selector::FieldSelector for LighthouseAuditResultV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LighthouseAuditResultV5 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LighthouseCategoryV5 {
        #[doc = "An array of references to all the audit members of this category."]
        #[serde(
            rename = "auditRefs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_refs: ::std::option::Option<Vec<crate::schemas::AuditRefs>>,
        #[doc = "A more detailed description of the category and its importance."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The string identifier of the category."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A description for the manual audits in the category."]
        #[serde(
            rename = "manualDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manual_description: ::std::option::Option<String>,
        #[doc = "The overall score of the category, the weighted average of all its audits.\n(The category's score, can be null.)"]
        #[serde(
            rename = "score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score: ::std::option::Option<::serde_json::Value>,
        #[doc = "The human-friendly name of the category."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LighthouseCategoryV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LighthouseCategoryV5 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LighthouseResultV5 {
        #[doc = "Map of audits in the LHR."]
        #[serde(
            rename = "audits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audits: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::LighthouseAuditResultV5>,
        >,
        #[doc = "Map of categories in the LHR."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<crate::schemas::Categories>,
        #[doc = "Map of category groups in the LHR."]
        #[serde(
            rename = "categoryGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_groups: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::CategoryGroupV5>,
        >,
        #[doc = "The configuration settings for this LHR."]
        #[serde(
            rename = "configSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_settings: ::std::option::Option<crate::schemas::ConfigSettings>,
        #[doc = "Environment settings that were used when making this LHR."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<crate::schemas::Environment>,
        #[doc = "The time that this run was fetched."]
        #[serde(
            rename = "fetchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fetch_time: ::std::option::Option<String>,
        #[doc = "The final resolved url that was audited."]
        #[serde(
            rename = "finalUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_url: ::std::option::Option<String>,
        #[doc = "The internationalization strings that are required to render the LHR."]
        #[serde(
            rename = "i18n",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub i_1_8n: ::std::option::Option<crate::schemas::I18N>,
        #[doc = "The lighthouse version that was used to generate this LHR."]
        #[serde(
            rename = "lighthouseVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lighthouse_version: ::std::option::Option<String>,
        #[doc = "The original requested url."]
        #[serde(
            rename = "requestedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_url: ::std::option::Option<String>,
        #[doc = "List of all run warnings in the LHR.  Will always output to at least `[]`."]
        #[serde(
            rename = "runWarnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub run_warnings: ::std::option::Option<Vec<::serde_json::Value>>,
        #[doc = "A top-level error message that, if present, indicates a serious enough\nproblem that this Lighthouse result may need to be discarded."]
        #[serde(
            rename = "runtimeError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_error: ::std::option::Option<crate::schemas::RuntimeError>,
        #[doc = "The Stack Pack advice strings."]
        #[serde(
            rename = "stackPacks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_packs: ::std::option::Option<Vec<crate::schemas::StackPack>>,
        #[doc = "Timing information for this LHR."]
        #[serde(
            rename = "timing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timing: ::std::option::Option<crate::schemas::Timing>,
        #[doc = "The user agent that was used to run this LHR."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LighthouseResultV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LighthouseResultV5 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PagespeedApiLoadingExperienceV5 {
        #[doc = "The url, pattern or origin which the metrics are on."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The requested URL, which may differ from the resolved \"id\"."]
        #[serde(
            rename = "initial_url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_url: ::std::option::Option<String>,
        #[doc = "The map of <metrics, data>."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::UserPageLoadMetricV5>,
        >,
        #[doc = "True if the result is an origin fallback from a page, false otherwise."]
        #[serde(
            rename = "origin_fallback",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin_fallback: ::std::option::Option<bool>,
        #[doc = "The human readable speed \"category\" of the id."]
        #[serde(
            rename = "overall_category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overall_category: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiLoadingExperienceV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiLoadingExperienceV5 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PagespeedApiPagespeedResponseV5 {
        #[doc = "The UTC timestamp of this analysis."]
        #[serde(
            rename = "analysisUTCTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_utc_timestamp: ::std::option::Option<String>,
        #[doc = "The captcha verify result"]
        #[serde(
            rename = "captchaResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_result: ::std::option::Option<String>,
        #[doc = "Canonicalized and final URL for the document, after following page\nredirects (if any)."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Kind of result."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Lighthouse response for the audit url as an object."]
        #[serde(
            rename = "lighthouseResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lighthouse_result: ::std::option::Option<crate::schemas::LighthouseResultV5>,
        #[doc = "Metrics of end users' page loading experience."]
        #[serde(
            rename = "loadingExperience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub loading_experience:
            ::std::option::Option<crate::schemas::PagespeedApiLoadingExperienceV5>,
        #[doc = "Metrics of the aggregated page loading experience of the origin"]
        #[serde(
            rename = "originLoadingExperience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin_loading_experience:
            ::std::option::Option<crate::schemas::PagespeedApiLoadingExperienceV5>,
        #[doc = "The version of PageSpeed used to generate these results."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::PagespeedVersion>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiPagespeedResponseV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiPagespeedResponseV5 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedVersion {
        #[doc = "The major version number of PageSpeed used to generate these results."]
        #[serde(
            rename = "major",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major: ::std::option::Option<String>,
        #[doc = "The minor version number of PageSpeed used to generate these results."]
        #[serde(
            rename = "minor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minor: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RendererFormattedStrings {
        #[doc = "The tooltip text on an expandable chevron icon."]
        #[serde(
            rename = "auditGroupExpandTooltip",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_group_expand_tooltip: ::std::option::Option<String>,
        #[doc = "The label for the initial request in a critical request chain."]
        #[serde(
            rename = "crcInitialNavigation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crc_initial_navigation: ::std::option::Option<String>,
        #[doc = "The label for values shown in the summary of critical request chains."]
        #[serde(
            rename = "crcLongestDurationLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crc_longest_duration_label: ::std::option::Option<String>,
        #[doc = "The label shown next to an audit or metric that has had an error."]
        #[serde(
            rename = "errorLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_label: ::std::option::Option<String>,
        #[doc = "The error string shown next to an erroring audit."]
        #[serde(
            rename = "errorMissingAuditInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_missing_audit_info: ::std::option::Option<String>,
        #[doc = "The title of the lab data performance category."]
        #[serde(
            rename = "labDataTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lab_data_title: ::std::option::Option<String>,
        #[doc = "The disclaimer shown under performance explaning that the network can\nvary."]
        #[serde(
            rename = "lsPerformanceCategoryDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ls_performance_category_description: ::std::option::Option<String>,
        #[doc = "The heading shown above a list of audits that were not computerd in the\nrun."]
        #[serde(
            rename = "manualAuditsGroupTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manual_audits_group_title: ::std::option::Option<String>,
        #[doc = "The heading shown above a list of audits that do not apply to a page."]
        #[serde(
            rename = "notApplicableAuditsGroupTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not_applicable_audits_group_title: ::std::option::Option<String>,
        #[doc = "The heading for the estimated page load savings opportunity of an\naudit."]
        #[serde(
            rename = "opportunityResourceColumnLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub opportunity_resource_column_label: ::std::option::Option<String>,
        #[doc = "The heading for the estimated page load savings of opportunity audits."]
        #[serde(
            rename = "opportunitySavingsColumnLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub opportunity_savings_column_label: ::std::option::Option<String>,
        #[doc = "The heading that is shown above a list of audits that are passing."]
        #[serde(
            rename = "passedAuditsGroupTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub passed_audits_group_title: ::std::option::Option<String>,
        #[doc = "The label that explains the score gauges scale (0-49, 50-89, 90-100)."]
        #[serde(
            rename = "scorescaleLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scorescale_label: ::std::option::Option<String>,
        #[doc = "The label shown preceding important warnings that may have invalidated\nan entire report."]
        #[serde(
            rename = "toplevelWarningsMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub toplevel_warnings_message: ::std::option::Option<String>,
        #[doc = "The disclaimer shown below a performance metric value."]
        #[serde(
            rename = "varianceDisclaimer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub variance_disclaimer: ::std::option::Option<String>,
        #[doc = "The label shown above a bulleted list of warnings."]
        #[serde(
            rename = "warningHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning_header: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RendererFormattedStrings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RendererFormattedStrings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RuntimeError {
        #[doc = "The enumerated Lighthouse Error code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<String>,
        #[doc = "A human readable message explaining the error code."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RuntimeError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuntimeError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StackPack {
        #[doc = "The stack pack advice strings."]
        #[serde(
            rename = "descriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub descriptions: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The stack pack icon data uri."]
        #[serde(
            rename = "iconDataURL",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_data_url: ::std::option::Option<String>,
        #[doc = "The stack pack id."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The stack pack title."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StackPack {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StackPack {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Timing {
        #[doc = "The total duration of Lighthouse's run."]
        #[serde(
            rename = "total",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Timing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Timing {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UserPageLoadMetricV5 {
        #[doc = "The category of the specific time metric."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "Metric distributions. Proportions should sum up to 1."]
        #[serde(
            rename = "distributions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distributions: ::std::option::Option<Vec<crate::schemas::Bucket>>,
        #[doc = "Identifies the form factor of the metric being collected."]
        #[serde(
            rename = "formFactor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_factor: ::std::option::Option<String>,
        #[doc = "The median number of the metric, in millisecond."]
        #[serde(
            rename = "median",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub median: ::std::option::Option<i32>,
        #[doc = "Identifies the type of the metric."]
        #[serde(
            rename = "metricId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_id: ::std::option::Option<String>,
        #[doc = "We use this field to store certain percentile value for this metric.\nFor v4, this field contains pc50.\nFor v5, this field contains pc90."]
        #[serde(
            rename = "percentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub percentile: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UserPageLoadMetricV5 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserPageLoadMetricV5 {
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
    #[doc = "Actions that can be performed on the pagespeedapi resource"]
    pub fn pagespeedapi(&self) -> crate::resources::pagespeedapi::PagespeedapiActions {
        crate::resources::pagespeedapi::PagespeedapiActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod pagespeedapi {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RunpagespeedCategoryItems {
                Accessibility,
                BestPractices,
                CategoryUnspecified,
                Performance,
                Pwa,
                Seo,
            }
            impl RunpagespeedCategoryItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RunpagespeedCategoryItems::Accessibility => "ACCESSIBILITY",
                        RunpagespeedCategoryItems::BestPractices => "BEST_PRACTICES",
                        RunpagespeedCategoryItems::CategoryUnspecified => "CATEGORY_UNSPECIFIED",
                        RunpagespeedCategoryItems::Performance => "PERFORMANCE",
                        RunpagespeedCategoryItems::Pwa => "PWA",
                        RunpagespeedCategoryItems::Seo => "SEO",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for RunpagespeedCategoryItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for RunpagespeedCategoryItems {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<RunpagespeedCategoryItems, ()> {
                    Ok(match s {
                        "ACCESSIBILITY" => RunpagespeedCategoryItems::Accessibility,
                        "BEST_PRACTICES" => RunpagespeedCategoryItems::BestPractices,
                        "CATEGORY_UNSPECIFIED" => RunpagespeedCategoryItems::CategoryUnspecified,
                        "PERFORMANCE" => RunpagespeedCategoryItems::Performance,
                        "PWA" => RunpagespeedCategoryItems::Pwa,
                        "SEO" => RunpagespeedCategoryItems::Seo,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for RunpagespeedCategoryItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RunpagespeedCategoryItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RunpagespeedCategoryItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ACCESSIBILITY" => RunpagespeedCategoryItems::Accessibility,
                        "BEST_PRACTICES" => RunpagespeedCategoryItems::BestPractices,
                        "CATEGORY_UNSPECIFIED" => RunpagespeedCategoryItems::CategoryUnspecified,
                        "PERFORMANCE" => RunpagespeedCategoryItems::Performance,
                        "PWA" => RunpagespeedCategoryItems::Pwa,
                        "SEO" => RunpagespeedCategoryItems::Seo,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for RunpagespeedCategoryItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RunpagespeedCategoryItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RunpagespeedStrategy {
                Desktop,
                Mobile,
                StrategyUnspecified,
            }
            impl RunpagespeedStrategy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RunpagespeedStrategy::Desktop => "DESKTOP",
                        RunpagespeedStrategy::Mobile => "MOBILE",
                        RunpagespeedStrategy::StrategyUnspecified => "STRATEGY_UNSPECIFIED",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for RunpagespeedStrategy {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for RunpagespeedStrategy {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<RunpagespeedStrategy, ()> {
                    Ok(match s {
                        "DESKTOP" => RunpagespeedStrategy::Desktop,
                        "MOBILE" => RunpagespeedStrategy::Mobile,
                        "STRATEGY_UNSPECIFIED" => RunpagespeedStrategy::StrategyUnspecified,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for RunpagespeedStrategy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RunpagespeedStrategy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RunpagespeedStrategy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "DESKTOP" => RunpagespeedStrategy::Desktop,
                        "MOBILE" => RunpagespeedStrategy::Mobile,
                        "STRATEGY_UNSPECIFIED" => RunpagespeedStrategy::StrategyUnspecified,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for RunpagespeedStrategy {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RunpagespeedStrategy {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct PagespeedapiActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PagespeedapiActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Runs PageSpeed analysis on the page at the specified URL, and returns\nPageSpeed scores, a list of suggestions to make that page faster, and other\ninformation."]
            pub fn runpagespeed(&self) -> RunpagespeedRequestBuilder {
                RunpagespeedRequestBuilder {
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
                    captcha_token: None,
                    category: None,
                    locale: None,
                    strategy: None,
                    url: None,
                    utm_campaign: None,
                    utm_source: None,
                }
            }
        }
        #[doc = "Created via [PagespeedapiActions::runpagespeed()](struct.PagespeedapiActions.html#method.runpagespeed)"]
        #[derive(Debug, Clone)]
        pub struct RunpagespeedRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            captcha_token: Option<String>,
            category:
                Option<Vec<crate::resources::pagespeedapi::params::RunpagespeedCategoryItems>>,
            locale: Option<String>,
            strategy: Option<crate::resources::pagespeedapi::params::RunpagespeedStrategy>,
            url: Option<String>,
            utm_campaign: Option<String>,
            utm_source: Option<String>,
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
        impl<'a> RunpagespeedRequestBuilder<'a> {
            #[doc = "The captcha token passed when filling out a captcha."]
            pub fn captcha_token(mut self, value: impl Into<String>) -> Self {
                self.captcha_token = Some(value.into());
                self
            }
            #[doc = "A Lighthouse category to run; if none are given, only Performance category\nwill be run"]
            pub fn category(
                mut self,
                value: impl Into<Vec<crate::resources::pagespeedapi::params::RunpagespeedCategoryItems>>,
            ) -> Self {
                self.category = Some(value.into());
                self
            }
            #[doc = "The locale used to localize formatted results"]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "The analysis strategy (desktop or mobile) to use, and desktop is the\ndefault"]
            pub fn strategy(
                mut self,
                value: crate::resources::pagespeedapi::params::RunpagespeedStrategy,
            ) -> Self {
                self.strategy = Some(value);
                self
            }
            #[doc = "The URL to fetch and analyze"]
            pub fn url(mut self, value: impl Into<String>) -> Self {
                self.url = Some(value.into());
                self
            }
            #[doc = "Campaign name for analytics."]
            pub fn utm_campaign(mut self, value: impl Into<String>) -> Self {
                self.utm_campaign = Some(value.into());
                self
            }
            #[doc = "Campaign source for analytics."]
            pub fn utm_source(mut self, value: impl Into<String>) -> Self {
                self.utm_source = Some(value.into());
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
            ) -> Result<crate::schemas::PagespeedApiPagespeedResponseV5, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PagespeedApiPagespeedResponseV5, crate::Error> {
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
                let mut output = "https://pagespeedonline.googleapis.com/".to_owned();
                output.push_str("pagespeedonline/v5/runPagespeed");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("captchaToken", &self.captcha_token)]);
                let req = req.query(&[("category", &self.category)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("strategy", &self.strategy)]);
                let req = req.query(&[("url", &self.url)]);
                let req = req.query(&[("utm_campaign", &self.utm_campaign)]);
                let req = req.query(&[("utm_source", &self.utm_source)]);
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
