#![doc = "# Resources and Methods\n    * [cse](resources/cse/struct.CseActions.html)\n      * [*list*](resources/cse/struct.ListRequestBuilder.html)\n      * [siterestrict](resources/cse/siterestrict/struct.SiterestrictActions.html)\n        * [*list*](resources/cse/siterestrict/struct.ListRequestBuilder.html)\n"]
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
    pub struct Context {
        #[serde(
            rename = "facets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub facets: ::std::option::Option<Vec<Vec<crate::schemas::ContextFacetsItemsItems>>>,
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Context {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Context {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContextFacetsItemsItems {
        #[serde(
            rename = "anchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub anchor: ::std::option::Option<String>,
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[serde(
            rename = "label_with_op",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_with_op: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContextFacetsItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContextFacetsItemsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Promotion {
        #[serde(
            rename = "bodyLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body_lines: ::std::option::Option<Vec<crate::schemas::PromotionBodyLinesItems>>,
        #[serde(
            rename = "displayLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_link: ::std::option::Option<String>,
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::PromotionImage>,
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Promotion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Promotion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PromotionBodyLinesItems {
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PromotionBodyLinesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PromotionBodyLinesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PromotionImage {
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PromotionImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PromotionImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Query {
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Query {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Query {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Result {
        #[serde(
            rename = "cacheId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_id: ::std::option::Option<String>,
        #[serde(
            rename = "displayLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_link: ::std::option::Option<String>,
        #[serde(
            rename = "fileFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_format: ::std::option::Option<String>,
        #[serde(
            rename = "formattedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_url: ::std::option::Option<String>,
        #[serde(
            rename = "htmlFormattedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_formatted_url: ::std::option::Option<String>,
        #[serde(
            rename = "htmlSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_snippet: ::std::option::Option<String>,
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::ResultImage>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::ResultLabelsItems>>,
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[serde(
            rename = "mime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime: ::std::option::Option<String>,
        #[serde(
            rename = "pagemap",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pagemap: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
            >,
        >,
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<String>,
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Result {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Result {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultImage {
        #[serde(
            rename = "byteSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub byte_size: ::std::option::Option<i32>,
        #[serde(
            rename = "contextLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_link: ::std::option::Option<String>,
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[serde(
            rename = "thumbnailHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_height: ::std::option::Option<i32>,
        #[serde(
            rename = "thumbnailLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_link: ::std::option::Option<String>,
        #[serde(
            rename = "thumbnailWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_width: ::std::option::Option<i32>,
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ResultImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultLabelsItems {
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[serde(
            rename = "label_with_op",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_with_op: ::std::option::Option<String>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResultLabelsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultLabelsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Search {
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::Context>,
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Result>>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "promotions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotions: ::std::option::Option<Vec<crate::schemas::Promotion>>,
        #[serde(
            rename = "queries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queries:
            ::std::option::Option<::std::collections::BTreeMap<String, Vec<crate::schemas::Query>>>,
        #[serde(
            rename = "searchInformation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_information: ::std::option::Option<crate::schemas::SearchSearchInformation>,
        #[serde(
            rename = "spelling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spelling: ::std::option::Option<crate::schemas::SearchSpelling>,
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<crate::schemas::SearchUrl>,
    }
    impl ::google_field_selector::FieldSelector for Search {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Search {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchSearchInformation {
        #[serde(
            rename = "formattedSearchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_search_time: ::std::option::Option<String>,
        #[serde(
            rename = "formattedTotalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_total_results: ::std::option::Option<String>,
        #[serde(
            rename = "searchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_time: ::std::option::Option<f64>,
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchSearchInformation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchSearchInformation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchSpelling {
        #[serde(
            rename = "correctedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_query: ::std::option::Option<String>,
        #[serde(
            rename = "htmlCorrectedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_corrected_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchSpelling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchSpelling {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchUrl {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "template",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchUrl {
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
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
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
    #[doc = "Actions that can be performed on the cse resource"]
    pub fn cse(&self) -> crate::resources::cse::CseActions {
        crate::resources::cse::CseActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod cse {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListFilter {
                #[doc = "Turns off duplicate content filter."]
                _0,
                #[doc = "Turns on duplicate content filter."]
                _1,
            }
            impl ListFilter {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListFilter::_0 => "0",
                        ListFilter::_1 => "1",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListFilter {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListFilter {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListFilter, ()> {
                    Ok(match s {
                        "0" => ListFilter::_0,
                        "1" => ListFilter::_1,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListFilter {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListFilter {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "0" => ListFilter::_0,
                        "1" => ListFilter::_1,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListFilter {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListFilter {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgColorType {
                #[doc = "color"]
                Color,
                #[doc = "gray"]
                Gray,
                #[doc = "mono"]
                Mono,
            }
            impl ListImgColorType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgColorType::Color => "color",
                        ListImgColorType::Gray => "gray",
                        ListImgColorType::Mono => "mono",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgColorType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgColorType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgColorType, ()> {
                    Ok(match s {
                        "color" => ListImgColorType::Color,
                        "gray" => ListImgColorType::Gray,
                        "mono" => ListImgColorType::Mono,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgColorType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgColorType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgColorType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "color" => ListImgColorType::Color,
                        "gray" => ListImgColorType::Gray,
                        "mono" => ListImgColorType::Mono,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgColorType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgColorType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgDominantColor {
                #[doc = "black"]
                Black,
                #[doc = "blue"]
                Blue,
                #[doc = "brown"]
                Brown,
                #[doc = "gray"]
                Gray,
                #[doc = "green"]
                Green,
                #[doc = "orange"]
                Orange,
                #[doc = "pink"]
                Pink,
                #[doc = "purple"]
                Purple,
                #[doc = "red"]
                Red,
                #[doc = "teal"]
                Teal,
                #[doc = "white"]
                White,
                #[doc = "yellow"]
                Yellow,
            }
            impl ListImgDominantColor {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgDominantColor::Black => "black",
                        ListImgDominantColor::Blue => "blue",
                        ListImgDominantColor::Brown => "brown",
                        ListImgDominantColor::Gray => "gray",
                        ListImgDominantColor::Green => "green",
                        ListImgDominantColor::Orange => "orange",
                        ListImgDominantColor::Pink => "pink",
                        ListImgDominantColor::Purple => "purple",
                        ListImgDominantColor::Red => "red",
                        ListImgDominantColor::Teal => "teal",
                        ListImgDominantColor::White => "white",
                        ListImgDominantColor::Yellow => "yellow",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgDominantColor {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgDominantColor {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgDominantColor, ()> {
                    Ok(match s {
                        "black" => ListImgDominantColor::Black,
                        "blue" => ListImgDominantColor::Blue,
                        "brown" => ListImgDominantColor::Brown,
                        "gray" => ListImgDominantColor::Gray,
                        "green" => ListImgDominantColor::Green,
                        "orange" => ListImgDominantColor::Orange,
                        "pink" => ListImgDominantColor::Pink,
                        "purple" => ListImgDominantColor::Purple,
                        "red" => ListImgDominantColor::Red,
                        "teal" => ListImgDominantColor::Teal,
                        "white" => ListImgDominantColor::White,
                        "yellow" => ListImgDominantColor::Yellow,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgDominantColor {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgDominantColor {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgDominantColor {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "black" => ListImgDominantColor::Black,
                        "blue" => ListImgDominantColor::Blue,
                        "brown" => ListImgDominantColor::Brown,
                        "gray" => ListImgDominantColor::Gray,
                        "green" => ListImgDominantColor::Green,
                        "orange" => ListImgDominantColor::Orange,
                        "pink" => ListImgDominantColor::Pink,
                        "purple" => ListImgDominantColor::Purple,
                        "red" => ListImgDominantColor::Red,
                        "teal" => ListImgDominantColor::Teal,
                        "white" => ListImgDominantColor::White,
                        "yellow" => ListImgDominantColor::Yellow,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgDominantColor {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgDominantColor {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgSize {
                #[doc = "huge"]
                Huge,
                #[doc = "icon"]
                Icon,
                #[doc = "large"]
                Large,
                #[doc = "medium"]
                Medium,
                #[doc = "small"]
                Small,
                #[doc = "xlarge"]
                Xlarge,
                #[doc = "xxlarge"]
                Xxlarge,
            }
            impl ListImgSize {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgSize::Huge => "huge",
                        ListImgSize::Icon => "icon",
                        ListImgSize::Large => "large",
                        ListImgSize::Medium => "medium",
                        ListImgSize::Small => "small",
                        ListImgSize::Xlarge => "xlarge",
                        ListImgSize::Xxlarge => "xxlarge",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgSize {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgSize {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgSize, ()> {
                    Ok(match s {
                        "huge" => ListImgSize::Huge,
                        "icon" => ListImgSize::Icon,
                        "large" => ListImgSize::Large,
                        "medium" => ListImgSize::Medium,
                        "small" => ListImgSize::Small,
                        "xlarge" => ListImgSize::Xlarge,
                        "xxlarge" => ListImgSize::Xxlarge,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgSize {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgSize {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgSize {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "huge" => ListImgSize::Huge,
                        "icon" => ListImgSize::Icon,
                        "large" => ListImgSize::Large,
                        "medium" => ListImgSize::Medium,
                        "small" => ListImgSize::Small,
                        "xlarge" => ListImgSize::Xlarge,
                        "xxlarge" => ListImgSize::Xxlarge,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgSize {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgSize {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgType {
                #[doc = "clipart"]
                Clipart,
                #[doc = "face"]
                Face,
                #[doc = "lineart"]
                Lineart,
                #[doc = "news"]
                News,
                #[doc = "photo"]
                Photo,
            }
            impl ListImgType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgType::Clipart => "clipart",
                        ListImgType::Face => "face",
                        ListImgType::Lineart => "lineart",
                        ListImgType::News => "news",
                        ListImgType::Photo => "photo",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgType, ()> {
                    Ok(match s {
                        "clipart" => ListImgType::Clipart,
                        "face" => ListImgType::Face,
                        "lineart" => ListImgType::Lineart,
                        "news" => ListImgType::News,
                        "photo" => ListImgType::Photo,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "clipart" => ListImgType::Clipart,
                        "face" => ListImgType::Face,
                        "lineart" => ListImgType::Lineart,
                        "news" => ListImgType::News,
                        "photo" => ListImgType::Photo,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListLr {
                #[doc = "Arabic"]
                LangAr,
                #[doc = "Bulgarian"]
                LangBg,
                #[doc = "Catalan"]
                LangCa,
                #[doc = "Czech"]
                LangCs,
                #[doc = "Danish"]
                LangDa,
                #[doc = "German"]
                LangDe,
                #[doc = "Greek"]
                LangEl,
                #[doc = "English"]
                LangEn,
                #[doc = "Spanish"]
                LangEs,
                #[doc = "Estonian"]
                LangEt,
                #[doc = "Finnish"]
                LangFi,
                #[doc = "French"]
                LangFr,
                #[doc = "Croatian"]
                LangHr,
                #[doc = "Hungarian"]
                LangHu,
                #[doc = "Indonesian"]
                LangId,
                #[doc = "Icelandic"]
                LangIs,
                #[doc = "Italian"]
                LangIt,
                #[doc = "Hebrew"]
                LangIw,
                #[doc = "Japanese"]
                LangJa,
                #[doc = "Korean"]
                LangKo,
                #[doc = "Lithuanian"]
                LangLt,
                #[doc = "Latvian"]
                LangLv,
                #[doc = "Dutch"]
                LangNl,
                #[doc = "Norwegian"]
                LangNo,
                #[doc = "Polish"]
                LangPl,
                #[doc = "Portuguese"]
                LangPt,
                #[doc = "Romanian"]
                LangRo,
                #[doc = "Russian"]
                LangRu,
                #[doc = "Slovak"]
                LangSk,
                #[doc = "Slovenian"]
                LangSl,
                #[doc = "Serbian"]
                LangSr,
                #[doc = "Swedish"]
                LangSv,
                #[doc = "Turkish"]
                LangTr,
                #[doc = "Chinese (Simplified)"]
                LangZhCN,
                #[doc = "Chinese (Traditional)"]
                LangZhTW,
            }
            impl ListLr {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListLr::LangAr => "lang_ar",
                        ListLr::LangBg => "lang_bg",
                        ListLr::LangCa => "lang_ca",
                        ListLr::LangCs => "lang_cs",
                        ListLr::LangDa => "lang_da",
                        ListLr::LangDe => "lang_de",
                        ListLr::LangEl => "lang_el",
                        ListLr::LangEn => "lang_en",
                        ListLr::LangEs => "lang_es",
                        ListLr::LangEt => "lang_et",
                        ListLr::LangFi => "lang_fi",
                        ListLr::LangFr => "lang_fr",
                        ListLr::LangHr => "lang_hr",
                        ListLr::LangHu => "lang_hu",
                        ListLr::LangId => "lang_id",
                        ListLr::LangIs => "lang_is",
                        ListLr::LangIt => "lang_it",
                        ListLr::LangIw => "lang_iw",
                        ListLr::LangJa => "lang_ja",
                        ListLr::LangKo => "lang_ko",
                        ListLr::LangLt => "lang_lt",
                        ListLr::LangLv => "lang_lv",
                        ListLr::LangNl => "lang_nl",
                        ListLr::LangNo => "lang_no",
                        ListLr::LangPl => "lang_pl",
                        ListLr::LangPt => "lang_pt",
                        ListLr::LangRo => "lang_ro",
                        ListLr::LangRu => "lang_ru",
                        ListLr::LangSk => "lang_sk",
                        ListLr::LangSl => "lang_sl",
                        ListLr::LangSr => "lang_sr",
                        ListLr::LangSv => "lang_sv",
                        ListLr::LangTr => "lang_tr",
                        ListLr::LangZhCN => "lang_zh-CN",
                        ListLr::LangZhTW => "lang_zh-TW",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListLr {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListLr {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListLr, ()> {
                    Ok(match s {
                        "lang_ar" => ListLr::LangAr,
                        "lang_bg" => ListLr::LangBg,
                        "lang_ca" => ListLr::LangCa,
                        "lang_cs" => ListLr::LangCs,
                        "lang_da" => ListLr::LangDa,
                        "lang_de" => ListLr::LangDe,
                        "lang_el" => ListLr::LangEl,
                        "lang_en" => ListLr::LangEn,
                        "lang_es" => ListLr::LangEs,
                        "lang_et" => ListLr::LangEt,
                        "lang_fi" => ListLr::LangFi,
                        "lang_fr" => ListLr::LangFr,
                        "lang_hr" => ListLr::LangHr,
                        "lang_hu" => ListLr::LangHu,
                        "lang_id" => ListLr::LangId,
                        "lang_is" => ListLr::LangIs,
                        "lang_it" => ListLr::LangIt,
                        "lang_iw" => ListLr::LangIw,
                        "lang_ja" => ListLr::LangJa,
                        "lang_ko" => ListLr::LangKo,
                        "lang_lt" => ListLr::LangLt,
                        "lang_lv" => ListLr::LangLv,
                        "lang_nl" => ListLr::LangNl,
                        "lang_no" => ListLr::LangNo,
                        "lang_pl" => ListLr::LangPl,
                        "lang_pt" => ListLr::LangPt,
                        "lang_ro" => ListLr::LangRo,
                        "lang_ru" => ListLr::LangRu,
                        "lang_sk" => ListLr::LangSk,
                        "lang_sl" => ListLr::LangSl,
                        "lang_sr" => ListLr::LangSr,
                        "lang_sv" => ListLr::LangSv,
                        "lang_tr" => ListLr::LangTr,
                        "lang_zh-CN" => ListLr::LangZhCN,
                        "lang_zh-TW" => ListLr::LangZhTW,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListLr {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListLr {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListLr {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "lang_ar" => ListLr::LangAr,
                        "lang_bg" => ListLr::LangBg,
                        "lang_ca" => ListLr::LangCa,
                        "lang_cs" => ListLr::LangCs,
                        "lang_da" => ListLr::LangDa,
                        "lang_de" => ListLr::LangDe,
                        "lang_el" => ListLr::LangEl,
                        "lang_en" => ListLr::LangEn,
                        "lang_es" => ListLr::LangEs,
                        "lang_et" => ListLr::LangEt,
                        "lang_fi" => ListLr::LangFi,
                        "lang_fr" => ListLr::LangFr,
                        "lang_hr" => ListLr::LangHr,
                        "lang_hu" => ListLr::LangHu,
                        "lang_id" => ListLr::LangId,
                        "lang_is" => ListLr::LangIs,
                        "lang_it" => ListLr::LangIt,
                        "lang_iw" => ListLr::LangIw,
                        "lang_ja" => ListLr::LangJa,
                        "lang_ko" => ListLr::LangKo,
                        "lang_lt" => ListLr::LangLt,
                        "lang_lv" => ListLr::LangLv,
                        "lang_nl" => ListLr::LangNl,
                        "lang_no" => ListLr::LangNo,
                        "lang_pl" => ListLr::LangPl,
                        "lang_pt" => ListLr::LangPt,
                        "lang_ro" => ListLr::LangRo,
                        "lang_ru" => ListLr::LangRu,
                        "lang_sk" => ListLr::LangSk,
                        "lang_sl" => ListLr::LangSl,
                        "lang_sr" => ListLr::LangSr,
                        "lang_sv" => ListLr::LangSv,
                        "lang_tr" => ListLr::LangTr,
                        "lang_zh-CN" => ListLr::LangZhCN,
                        "lang_zh-TW" => ListLr::LangZhTW,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListLr {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListLr {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSafe {
                #[doc = "Enables safe search filtering."]
                Active,
                #[doc = "(Deprecated) Same as active."]
                High,
                #[doc = "(Deprecated) Same as active."]
                Medium,
                #[doc = "Disables safe search filtering."]
                Off,
            }
            impl ListSafe {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSafe::Active => "active",
                        ListSafe::High => "high",
                        ListSafe::Medium => "medium",
                        ListSafe::Off => "off",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSafe {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSafe {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSafe, ()> {
                    Ok(match s {
                        "active" => ListSafe::Active,
                        "high" => ListSafe::High,
                        "medium" => ListSafe::Medium,
                        "off" => ListSafe::Off,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSafe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSafe {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSafe {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "active" => ListSafe::Active,
                        "high" => ListSafe::High,
                        "medium" => ListSafe::Medium,
                        "off" => ListSafe::Off,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSafe {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSafe {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSearchType {
                #[doc = "custom image search"]
                Image,
            }
            impl ListSearchType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSearchType::Image => "image",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSearchType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSearchType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSearchType, ()> {
                    Ok(match s {
                        "image" => ListSearchType::Image,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSearchType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSearchType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSearchType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "image" => ListSearchType::Image,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSearchType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSearchType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSiteSearchFilter {
                #[doc = "exclude"]
                E,
                #[doc = "include"]
                I,
            }
            impl ListSiteSearchFilter {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSiteSearchFilter::E => "e",
                        ListSiteSearchFilter::I => "i",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSiteSearchFilter {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSiteSearchFilter {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSiteSearchFilter, ()> {
                    Ok(match s {
                        "e" => ListSiteSearchFilter::E,
                        "i" => ListSiteSearchFilter::I,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSiteSearchFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSiteSearchFilter {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSiteSearchFilter {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "e" => ListSiteSearchFilter::E,
                        "i" => ListSiteSearchFilter::I,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSiteSearchFilter {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSiteSearchFilter {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct CseActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CseActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results."]
            pub fn list(&self, q: impl Into<String>) -> ListRequestBuilder {
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
                    q: q.into(),
                    c_2coff: None,
                    cr: None,
                    cx: None,
                    date_restrict: None,
                    exact_terms: None,
                    exclude_terms: None,
                    file_type: None,
                    filter: None,
                    gl: None,
                    googlehost: None,
                    high_range: None,
                    hl: None,
                    hq: None,
                    img_color_type: None,
                    img_dominant_color: None,
                    img_size: None,
                    img_type: None,
                    link_site: None,
                    low_range: None,
                    lr: None,
                    num: None,
                    or_terms: None,
                    related_site: None,
                    rights: None,
                    safe: None,
                    search_type: None,
                    site_search: None,
                    site_search_filter: None,
                    sort: None,
                    start: None,
                }
            }
            #[doc = "Actions that can be performed on the siterestrict resource"]
            pub fn siterestrict(&self) -> crate::resources::cse::siterestrict::SiterestrictActions {
                crate::resources::cse::siterestrict::SiterestrictActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [CseActions::list()](struct.CseActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            q: String,
            c_2coff: Option<String>,
            cr: Option<String>,
            cx: Option<String>,
            date_restrict: Option<String>,
            exact_terms: Option<String>,
            exclude_terms: Option<String>,
            file_type: Option<String>,
            filter: Option<crate::resources::cse::params::ListFilter>,
            gl: Option<String>,
            googlehost: Option<String>,
            high_range: Option<String>,
            hl: Option<String>,
            hq: Option<String>,
            img_color_type: Option<crate::resources::cse::params::ListImgColorType>,
            img_dominant_color: Option<crate::resources::cse::params::ListImgDominantColor>,
            img_size: Option<crate::resources::cse::params::ListImgSize>,
            img_type: Option<crate::resources::cse::params::ListImgType>,
            link_site: Option<String>,
            low_range: Option<String>,
            lr: Option<crate::resources::cse::params::ListLr>,
            num: Option<u32>,
            or_terms: Option<String>,
            related_site: Option<String>,
            rights: Option<String>,
            safe: Option<crate::resources::cse::params::ListSafe>,
            search_type: Option<crate::resources::cse::params::ListSearchType>,
            site_search: Option<String>,
            site_search_filter: Option<crate::resources::cse::params::ListSiteSearchFilter>,
            sort: Option<String>,
            start: Option<u32>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Turns off the translation between zh-CN and zh-TW."]
            pub fn c_2coff(mut self, value: impl Into<String>) -> Self {
                self.c_2coff = Some(value.into());
                self
            }
            #[doc = "Country restrict(s)."]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = "The custom search engine ID to scope this search query"]
            pub fn cx(mut self, value: impl Into<String>) -> Self {
                self.cx = Some(value.into());
                self
            }
            #[doc = "Specifies all search results are from a time period"]
            pub fn date_restrict(mut self, value: impl Into<String>) -> Self {
                self.date_restrict = Some(value.into());
                self
            }
            #[doc = "Identifies a phrase that all documents in the search results must contain"]
            pub fn exact_terms(mut self, value: impl Into<String>) -> Self {
                self.exact_terms = Some(value.into());
                self
            }
            #[doc = "Identifies a word or phrase that should not appear in any documents in the search results"]
            pub fn exclude_terms(mut self, value: impl Into<String>) -> Self {
                self.exclude_terms = Some(value.into());
                self
            }
            #[doc = "Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ..."]
            pub fn file_type(mut self, value: impl Into<String>) -> Self {
                self.file_type = Some(value.into());
                self
            }
            #[doc = "Controls turning on or off the duplicate content filter."]
            pub fn filter(mut self, value: crate::resources::cse::params::ListFilter) -> Self {
                self.filter = Some(value);
                self
            }
            #[doc = "Geolocation of end user."]
            pub fn gl(mut self, value: impl Into<String>) -> Self {
                self.gl = Some(value.into());
                self
            }
            #[doc = "The local Google domain to use to perform the search."]
            pub fn googlehost(mut self, value: impl Into<String>) -> Self {
                self.googlehost = Some(value.into());
                self
            }
            #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
            pub fn high_range(mut self, value: impl Into<String>) -> Self {
                self.high_range = Some(value.into());
                self
            }
            #[doc = "Sets the user interface language."]
            pub fn hl(mut self, value: impl Into<String>) -> Self {
                self.hl = Some(value.into());
                self
            }
            #[doc = "Appends the extra query terms to the query."]
            pub fn hq(mut self, value: impl Into<String>) -> Self {
                self.hq = Some(value.into());
                self
            }
            #[doc = "Returns black and white, grayscale, or color images: mono, gray, and color."]
            pub fn img_color_type(
                mut self,
                value: crate::resources::cse::params::ListImgColorType,
            ) -> Self {
                self.img_color_type = Some(value);
                self
            }
            #[doc = "Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown."]
            pub fn img_dominant_color(
                mut self,
                value: crate::resources::cse::params::ListImgDominantColor,
            ) -> Self {
                self.img_dominant_color = Some(value);
                self
            }
            #[doc = "Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge."]
            pub fn img_size(mut self, value: crate::resources::cse::params::ListImgSize) -> Self {
                self.img_size = Some(value);
                self
            }
            #[doc = "Returns images of a type, which can be one of: clipart, face, lineart, news, and photo."]
            pub fn img_type(mut self, value: crate::resources::cse::params::ListImgType) -> Self {
                self.img_type = Some(value);
                self
            }
            #[doc = "Specifies that all search results should contain a link to a particular URL"]
            pub fn link_site(mut self, value: impl Into<String>) -> Self {
                self.link_site = Some(value.into());
                self
            }
            #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
            pub fn low_range(mut self, value: impl Into<String>) -> Self {
                self.low_range = Some(value.into());
                self
            }
            #[doc = "The language restriction for the search results"]
            pub fn lr(mut self, value: crate::resources::cse::params::ListLr) -> Self {
                self.lr = Some(value);
                self
            }
            #[doc = "Number of search results to return"]
            pub fn num(mut self, value: u32) -> Self {
                self.num = Some(value);
                self
            }
            #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms"]
            pub fn or_terms(mut self, value: impl Into<String>) -> Self {
                self.or_terms = Some(value.into());
                self
            }
            #[doc = "Specifies that all search results should be pages that are related to the specified URL"]
            pub fn related_site(mut self, value: impl Into<String>) -> Self {
                self.related_site = Some(value.into());
                self
            }
            #[doc = "Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these."]
            pub fn rights(mut self, value: impl Into<String>) -> Self {
                self.rights = Some(value.into());
                self
            }
            #[doc = "Search safety level"]
            pub fn safe(mut self, value: crate::resources::cse::params::ListSafe) -> Self {
                self.safe = Some(value);
                self
            }
            #[doc = "Specifies the search type: image."]
            pub fn search_type(
                mut self,
                value: crate::resources::cse::params::ListSearchType,
            ) -> Self {
                self.search_type = Some(value);
                self
            }
            #[doc = "Specifies all search results should be pages from a given site"]
            pub fn site_search(mut self, value: impl Into<String>) -> Self {
                self.site_search = Some(value.into());
                self
            }
            #[doc = "Controls whether to include or exclude results from the site named in the as_sitesearch parameter"]
            pub fn site_search_filter(
                mut self,
                value: crate::resources::cse::params::ListSiteSearchFilter,
            ) -> Self {
                self.site_search_filter = Some(value);
                self
            }
            #[doc = "The sort expression to apply to the results"]
            pub fn sort(mut self, value: impl Into<String>) -> Self {
                self.sort = Some(value.into());
                self
            }
            #[doc = "The index of the first result to return"]
            pub fn start(mut self, value: u32) -> Self {
                self.start = Some(value);
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
            ) -> Result<crate::schemas::Search, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Search, crate::Error> {
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
                let mut output = "https://www.googleapis.com/customsearch/".to_owned();
                output.push_str("v1");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("q", &self.q)]);
                let req = req.query(&[("c2coff", &self.c_2coff)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("cx", &self.cx)]);
                let req = req.query(&[("dateRestrict", &self.date_restrict)]);
                let req = req.query(&[("exactTerms", &self.exact_terms)]);
                let req = req.query(&[("excludeTerms", &self.exclude_terms)]);
                let req = req.query(&[("fileType", &self.file_type)]);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("gl", &self.gl)]);
                let req = req.query(&[("googlehost", &self.googlehost)]);
                let req = req.query(&[("highRange", &self.high_range)]);
                let req = req.query(&[("hl", &self.hl)]);
                let req = req.query(&[("hq", &self.hq)]);
                let req = req.query(&[("imgColorType", &self.img_color_type)]);
                let req = req.query(&[("imgDominantColor", &self.img_dominant_color)]);
                let req = req.query(&[("imgSize", &self.img_size)]);
                let req = req.query(&[("imgType", &self.img_type)]);
                let req = req.query(&[("linkSite", &self.link_site)]);
                let req = req.query(&[("lowRange", &self.low_range)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("num", &self.num)]);
                let req = req.query(&[("orTerms", &self.or_terms)]);
                let req = req.query(&[("relatedSite", &self.related_site)]);
                let req = req.query(&[("rights", &self.rights)]);
                let req = req.query(&[("safe", &self.safe)]);
                let req = req.query(&[("searchType", &self.search_type)]);
                let req = req.query(&[("siteSearch", &self.site_search)]);
                let req = req.query(&[("siteSearchFilter", &self.site_search_filter)]);
                let req = req.query(&[("sort", &self.sort)]);
                let req = req.query(&[("start", &self.start)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        pub mod siterestrict {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListFilter {
                    #[doc = "Turns off duplicate content filter."]
                    _0,
                    #[doc = "Turns on duplicate content filter."]
                    _1,
                }
                impl ListFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListFilter::_0 => "0",
                            ListFilter::_1 => "1",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListFilter {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListFilter {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListFilter, ()> {
                        Ok(match s {
                            "0" => ListFilter::_0,
                            "1" => ListFilter::_1,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListFilter {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListFilter {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListFilter {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "0" => ListFilter::_0,
                            "1" => ListFilter::_1,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListFilter {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListFilter {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgColorType {
                    #[doc = "color"]
                    Color,
                    #[doc = "gray"]
                    Gray,
                    #[doc = "mono"]
                    Mono,
                }
                impl ListImgColorType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgColorType::Color => "color",
                            ListImgColorType::Gray => "gray",
                            ListImgColorType::Mono => "mono",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgColorType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgColorType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgColorType, ()> {
                        Ok(match s {
                            "color" => ListImgColorType::Color,
                            "gray" => ListImgColorType::Gray,
                            "mono" => ListImgColorType::Mono,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgColorType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgColorType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgColorType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "color" => ListImgColorType::Color,
                            "gray" => ListImgColorType::Gray,
                            "mono" => ListImgColorType::Mono,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgColorType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgColorType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgDominantColor {
                    #[doc = "black"]
                    Black,
                    #[doc = "blue"]
                    Blue,
                    #[doc = "brown"]
                    Brown,
                    #[doc = "gray"]
                    Gray,
                    #[doc = "green"]
                    Green,
                    #[doc = "orange"]
                    Orange,
                    #[doc = "pink"]
                    Pink,
                    #[doc = "purple"]
                    Purple,
                    #[doc = "red"]
                    Red,
                    #[doc = "teal"]
                    Teal,
                    #[doc = "white"]
                    White,
                    #[doc = "yellow"]
                    Yellow,
                }
                impl ListImgDominantColor {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgDominantColor::Black => "black",
                            ListImgDominantColor::Blue => "blue",
                            ListImgDominantColor::Brown => "brown",
                            ListImgDominantColor::Gray => "gray",
                            ListImgDominantColor::Green => "green",
                            ListImgDominantColor::Orange => "orange",
                            ListImgDominantColor::Pink => "pink",
                            ListImgDominantColor::Purple => "purple",
                            ListImgDominantColor::Red => "red",
                            ListImgDominantColor::Teal => "teal",
                            ListImgDominantColor::White => "white",
                            ListImgDominantColor::Yellow => "yellow",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgDominantColor {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgDominantColor {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgDominantColor, ()> {
                        Ok(match s {
                            "black" => ListImgDominantColor::Black,
                            "blue" => ListImgDominantColor::Blue,
                            "brown" => ListImgDominantColor::Brown,
                            "gray" => ListImgDominantColor::Gray,
                            "green" => ListImgDominantColor::Green,
                            "orange" => ListImgDominantColor::Orange,
                            "pink" => ListImgDominantColor::Pink,
                            "purple" => ListImgDominantColor::Purple,
                            "red" => ListImgDominantColor::Red,
                            "teal" => ListImgDominantColor::Teal,
                            "white" => ListImgDominantColor::White,
                            "yellow" => ListImgDominantColor::Yellow,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgDominantColor {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgDominantColor {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgDominantColor {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "black" => ListImgDominantColor::Black,
                            "blue" => ListImgDominantColor::Blue,
                            "brown" => ListImgDominantColor::Brown,
                            "gray" => ListImgDominantColor::Gray,
                            "green" => ListImgDominantColor::Green,
                            "orange" => ListImgDominantColor::Orange,
                            "pink" => ListImgDominantColor::Pink,
                            "purple" => ListImgDominantColor::Purple,
                            "red" => ListImgDominantColor::Red,
                            "teal" => ListImgDominantColor::Teal,
                            "white" => ListImgDominantColor::White,
                            "yellow" => ListImgDominantColor::Yellow,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgDominantColor {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgDominantColor {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgSize {
                    #[doc = "huge"]
                    Huge,
                    #[doc = "icon"]
                    Icon,
                    #[doc = "large"]
                    Large,
                    #[doc = "medium"]
                    Medium,
                    #[doc = "small"]
                    Small,
                    #[doc = "xlarge"]
                    Xlarge,
                    #[doc = "xxlarge"]
                    Xxlarge,
                }
                impl ListImgSize {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgSize::Huge => "huge",
                            ListImgSize::Icon => "icon",
                            ListImgSize::Large => "large",
                            ListImgSize::Medium => "medium",
                            ListImgSize::Small => "small",
                            ListImgSize::Xlarge => "xlarge",
                            ListImgSize::Xxlarge => "xxlarge",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgSize {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgSize {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgSize, ()> {
                        Ok(match s {
                            "huge" => ListImgSize::Huge,
                            "icon" => ListImgSize::Icon,
                            "large" => ListImgSize::Large,
                            "medium" => ListImgSize::Medium,
                            "small" => ListImgSize::Small,
                            "xlarge" => ListImgSize::Xlarge,
                            "xxlarge" => ListImgSize::Xxlarge,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgSize {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgSize {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgSize {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "huge" => ListImgSize::Huge,
                            "icon" => ListImgSize::Icon,
                            "large" => ListImgSize::Large,
                            "medium" => ListImgSize::Medium,
                            "small" => ListImgSize::Small,
                            "xlarge" => ListImgSize::Xlarge,
                            "xxlarge" => ListImgSize::Xxlarge,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgSize {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgSize {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgType {
                    #[doc = "clipart"]
                    Clipart,
                    #[doc = "face"]
                    Face,
                    #[doc = "lineart"]
                    Lineart,
                    #[doc = "news"]
                    News,
                    #[doc = "photo"]
                    Photo,
                }
                impl ListImgType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgType::Clipart => "clipart",
                            ListImgType::Face => "face",
                            ListImgType::Lineart => "lineart",
                            ListImgType::News => "news",
                            ListImgType::Photo => "photo",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgType, ()> {
                        Ok(match s {
                            "clipart" => ListImgType::Clipart,
                            "face" => ListImgType::Face,
                            "lineart" => ListImgType::Lineart,
                            "news" => ListImgType::News,
                            "photo" => ListImgType::Photo,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "clipart" => ListImgType::Clipart,
                            "face" => ListImgType::Face,
                            "lineart" => ListImgType::Lineart,
                            "news" => ListImgType::News,
                            "photo" => ListImgType::Photo,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListLr {
                    #[doc = "Arabic"]
                    LangAr,
                    #[doc = "Bulgarian"]
                    LangBg,
                    #[doc = "Catalan"]
                    LangCa,
                    #[doc = "Czech"]
                    LangCs,
                    #[doc = "Danish"]
                    LangDa,
                    #[doc = "German"]
                    LangDe,
                    #[doc = "Greek"]
                    LangEl,
                    #[doc = "English"]
                    LangEn,
                    #[doc = "Spanish"]
                    LangEs,
                    #[doc = "Estonian"]
                    LangEt,
                    #[doc = "Finnish"]
                    LangFi,
                    #[doc = "French"]
                    LangFr,
                    #[doc = "Croatian"]
                    LangHr,
                    #[doc = "Hungarian"]
                    LangHu,
                    #[doc = "Indonesian"]
                    LangId,
                    #[doc = "Icelandic"]
                    LangIs,
                    #[doc = "Italian"]
                    LangIt,
                    #[doc = "Hebrew"]
                    LangIw,
                    #[doc = "Japanese"]
                    LangJa,
                    #[doc = "Korean"]
                    LangKo,
                    #[doc = "Lithuanian"]
                    LangLt,
                    #[doc = "Latvian"]
                    LangLv,
                    #[doc = "Dutch"]
                    LangNl,
                    #[doc = "Norwegian"]
                    LangNo,
                    #[doc = "Polish"]
                    LangPl,
                    #[doc = "Portuguese"]
                    LangPt,
                    #[doc = "Romanian"]
                    LangRo,
                    #[doc = "Russian"]
                    LangRu,
                    #[doc = "Slovak"]
                    LangSk,
                    #[doc = "Slovenian"]
                    LangSl,
                    #[doc = "Serbian"]
                    LangSr,
                    #[doc = "Swedish"]
                    LangSv,
                    #[doc = "Turkish"]
                    LangTr,
                    #[doc = "Chinese (Simplified)"]
                    LangZhCN,
                    #[doc = "Chinese (Traditional)"]
                    LangZhTW,
                }
                impl ListLr {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListLr::LangAr => "lang_ar",
                            ListLr::LangBg => "lang_bg",
                            ListLr::LangCa => "lang_ca",
                            ListLr::LangCs => "lang_cs",
                            ListLr::LangDa => "lang_da",
                            ListLr::LangDe => "lang_de",
                            ListLr::LangEl => "lang_el",
                            ListLr::LangEn => "lang_en",
                            ListLr::LangEs => "lang_es",
                            ListLr::LangEt => "lang_et",
                            ListLr::LangFi => "lang_fi",
                            ListLr::LangFr => "lang_fr",
                            ListLr::LangHr => "lang_hr",
                            ListLr::LangHu => "lang_hu",
                            ListLr::LangId => "lang_id",
                            ListLr::LangIs => "lang_is",
                            ListLr::LangIt => "lang_it",
                            ListLr::LangIw => "lang_iw",
                            ListLr::LangJa => "lang_ja",
                            ListLr::LangKo => "lang_ko",
                            ListLr::LangLt => "lang_lt",
                            ListLr::LangLv => "lang_lv",
                            ListLr::LangNl => "lang_nl",
                            ListLr::LangNo => "lang_no",
                            ListLr::LangPl => "lang_pl",
                            ListLr::LangPt => "lang_pt",
                            ListLr::LangRo => "lang_ro",
                            ListLr::LangRu => "lang_ru",
                            ListLr::LangSk => "lang_sk",
                            ListLr::LangSl => "lang_sl",
                            ListLr::LangSr => "lang_sr",
                            ListLr::LangSv => "lang_sv",
                            ListLr::LangTr => "lang_tr",
                            ListLr::LangZhCN => "lang_zh-CN",
                            ListLr::LangZhTW => "lang_zh-TW",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListLr {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListLr {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListLr, ()> {
                        Ok(match s {
                            "lang_ar" => ListLr::LangAr,
                            "lang_bg" => ListLr::LangBg,
                            "lang_ca" => ListLr::LangCa,
                            "lang_cs" => ListLr::LangCs,
                            "lang_da" => ListLr::LangDa,
                            "lang_de" => ListLr::LangDe,
                            "lang_el" => ListLr::LangEl,
                            "lang_en" => ListLr::LangEn,
                            "lang_es" => ListLr::LangEs,
                            "lang_et" => ListLr::LangEt,
                            "lang_fi" => ListLr::LangFi,
                            "lang_fr" => ListLr::LangFr,
                            "lang_hr" => ListLr::LangHr,
                            "lang_hu" => ListLr::LangHu,
                            "lang_id" => ListLr::LangId,
                            "lang_is" => ListLr::LangIs,
                            "lang_it" => ListLr::LangIt,
                            "lang_iw" => ListLr::LangIw,
                            "lang_ja" => ListLr::LangJa,
                            "lang_ko" => ListLr::LangKo,
                            "lang_lt" => ListLr::LangLt,
                            "lang_lv" => ListLr::LangLv,
                            "lang_nl" => ListLr::LangNl,
                            "lang_no" => ListLr::LangNo,
                            "lang_pl" => ListLr::LangPl,
                            "lang_pt" => ListLr::LangPt,
                            "lang_ro" => ListLr::LangRo,
                            "lang_ru" => ListLr::LangRu,
                            "lang_sk" => ListLr::LangSk,
                            "lang_sl" => ListLr::LangSl,
                            "lang_sr" => ListLr::LangSr,
                            "lang_sv" => ListLr::LangSv,
                            "lang_tr" => ListLr::LangTr,
                            "lang_zh-CN" => ListLr::LangZhCN,
                            "lang_zh-TW" => ListLr::LangZhTW,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListLr {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListLr {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListLr {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "lang_ar" => ListLr::LangAr,
                            "lang_bg" => ListLr::LangBg,
                            "lang_ca" => ListLr::LangCa,
                            "lang_cs" => ListLr::LangCs,
                            "lang_da" => ListLr::LangDa,
                            "lang_de" => ListLr::LangDe,
                            "lang_el" => ListLr::LangEl,
                            "lang_en" => ListLr::LangEn,
                            "lang_es" => ListLr::LangEs,
                            "lang_et" => ListLr::LangEt,
                            "lang_fi" => ListLr::LangFi,
                            "lang_fr" => ListLr::LangFr,
                            "lang_hr" => ListLr::LangHr,
                            "lang_hu" => ListLr::LangHu,
                            "lang_id" => ListLr::LangId,
                            "lang_is" => ListLr::LangIs,
                            "lang_it" => ListLr::LangIt,
                            "lang_iw" => ListLr::LangIw,
                            "lang_ja" => ListLr::LangJa,
                            "lang_ko" => ListLr::LangKo,
                            "lang_lt" => ListLr::LangLt,
                            "lang_lv" => ListLr::LangLv,
                            "lang_nl" => ListLr::LangNl,
                            "lang_no" => ListLr::LangNo,
                            "lang_pl" => ListLr::LangPl,
                            "lang_pt" => ListLr::LangPt,
                            "lang_ro" => ListLr::LangRo,
                            "lang_ru" => ListLr::LangRu,
                            "lang_sk" => ListLr::LangSk,
                            "lang_sl" => ListLr::LangSl,
                            "lang_sr" => ListLr::LangSr,
                            "lang_sv" => ListLr::LangSv,
                            "lang_tr" => ListLr::LangTr,
                            "lang_zh-CN" => ListLr::LangZhCN,
                            "lang_zh-TW" => ListLr::LangZhTW,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListLr {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListLr {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSafe {
                    #[doc = "Enables highest level of safe search filtering."]
                    High,
                    #[doc = "Enables moderate safe search filtering."]
                    Medium,
                    #[doc = "Disables safe search filtering."]
                    Off,
                }
                impl ListSafe {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSafe::High => "high",
                            ListSafe::Medium => "medium",
                            ListSafe::Off => "off",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSafe {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSafe {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSafe, ()> {
                        Ok(match s {
                            "high" => ListSafe::High,
                            "medium" => ListSafe::Medium,
                            "off" => ListSafe::Off,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSafe {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSafe {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSafe {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "high" => ListSafe::High,
                            "medium" => ListSafe::Medium,
                            "off" => ListSafe::Off,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListSafe {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSafe {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSearchType {
                    #[doc = "custom image search"]
                    Image,
                }
                impl ListSearchType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSearchType::Image => "image",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSearchType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSearchType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSearchType, ()> {
                        Ok(match s {
                            "image" => ListSearchType::Image,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSearchType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSearchType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSearchType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "image" => ListSearchType::Image,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListSearchType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSearchType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSiteSearchFilter {
                    #[doc = "exclude"]
                    E,
                    #[doc = "include"]
                    I,
                }
                impl ListSiteSearchFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSiteSearchFilter::E => "e",
                            ListSiteSearchFilter::I => "i",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSiteSearchFilter {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSiteSearchFilter {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSiteSearchFilter, ()> {
                        Ok(match s {
                            "e" => ListSiteSearchFilter::E,
                            "i" => ListSiteSearchFilter::I,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSiteSearchFilter {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSiteSearchFilter {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSiteSearchFilter {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "e" => ListSiteSearchFilter::E,
                            "i" => ListSiteSearchFilter::I,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListSiteSearchFilter {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSiteSearchFilter {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct SiterestrictActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SiterestrictActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results. Uses a small set of url patterns."]
                pub fn list(&self, q: impl Into<String>) -> ListRequestBuilder {
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
                        q: q.into(),
                        c_2coff: None,
                        cr: None,
                        cx: None,
                        date_restrict: None,
                        exact_terms: None,
                        exclude_terms: None,
                        file_type: None,
                        filter: None,
                        gl: None,
                        googlehost: None,
                        high_range: None,
                        hl: None,
                        hq: None,
                        img_color_type: None,
                        img_dominant_color: None,
                        img_size: None,
                        img_type: None,
                        link_site: None,
                        low_range: None,
                        lr: None,
                        num: None,
                        or_terms: None,
                        related_site: None,
                        rights: None,
                        safe: None,
                        search_type: None,
                        site_search: None,
                        site_search_filter: None,
                        sort: None,
                        start: None,
                    }
                }
            }
            #[doc = "Created via [SiterestrictActions::list()](struct.SiterestrictActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                q: String,
                c_2coff: Option<String>,
                cr: Option<String>,
                cx: Option<String>,
                date_restrict: Option<String>,
                exact_terms: Option<String>,
                exclude_terms: Option<String>,
                file_type: Option<String>,
                filter: Option<crate::resources::cse::siterestrict::params::ListFilter>,
                gl: Option<String>,
                googlehost: Option<String>,
                high_range: Option<String>,
                hl: Option<String>,
                hq: Option<String>,
                img_color_type:
                    Option<crate::resources::cse::siterestrict::params::ListImgColorType>,
                img_dominant_color:
                    Option<crate::resources::cse::siterestrict::params::ListImgDominantColor>,
                img_size: Option<crate::resources::cse::siterestrict::params::ListImgSize>,
                img_type: Option<crate::resources::cse::siterestrict::params::ListImgType>,
                link_site: Option<String>,
                low_range: Option<String>,
                lr: Option<crate::resources::cse::siterestrict::params::ListLr>,
                num: Option<u32>,
                or_terms: Option<String>,
                related_site: Option<String>,
                rights: Option<String>,
                safe: Option<crate::resources::cse::siterestrict::params::ListSafe>,
                search_type: Option<crate::resources::cse::siterestrict::params::ListSearchType>,
                site_search: Option<String>,
                site_search_filter:
                    Option<crate::resources::cse::siterestrict::params::ListSiteSearchFilter>,
                sort: Option<String>,
                start: Option<u32>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Turns off the translation between zh-CN and zh-TW."]
                pub fn c_2coff(mut self, value: impl Into<String>) -> Self {
                    self.c_2coff = Some(value.into());
                    self
                }
                #[doc = "Country restrict(s)."]
                pub fn cr(mut self, value: impl Into<String>) -> Self {
                    self.cr = Some(value.into());
                    self
                }
                #[doc = "The custom search engine ID to scope this search query"]
                pub fn cx(mut self, value: impl Into<String>) -> Self {
                    self.cx = Some(value.into());
                    self
                }
                #[doc = "Specifies all search results are from a time period"]
                pub fn date_restrict(mut self, value: impl Into<String>) -> Self {
                    self.date_restrict = Some(value.into());
                    self
                }
                #[doc = "Identifies a phrase that all documents in the search results must contain"]
                pub fn exact_terms(mut self, value: impl Into<String>) -> Self {
                    self.exact_terms = Some(value.into());
                    self
                }
                #[doc = "Identifies a word or phrase that should not appear in any documents in the search results"]
                pub fn exclude_terms(mut self, value: impl Into<String>) -> Self {
                    self.exclude_terms = Some(value.into());
                    self
                }
                #[doc = "Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ..."]
                pub fn file_type(mut self, value: impl Into<String>) -> Self {
                    self.file_type = Some(value.into());
                    self
                }
                #[doc = "Controls turning on or off the duplicate content filter."]
                pub fn filter(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListFilter,
                ) -> Self {
                    self.filter = Some(value);
                    self
                }
                #[doc = "Geolocation of end user."]
                pub fn gl(mut self, value: impl Into<String>) -> Self {
                    self.gl = Some(value.into());
                    self
                }
                #[doc = "The local Google domain to use to perform the search."]
                pub fn googlehost(mut self, value: impl Into<String>) -> Self {
                    self.googlehost = Some(value.into());
                    self
                }
                #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
                pub fn high_range(mut self, value: impl Into<String>) -> Self {
                    self.high_range = Some(value.into());
                    self
                }
                #[doc = "Sets the user interface language."]
                pub fn hl(mut self, value: impl Into<String>) -> Self {
                    self.hl = Some(value.into());
                    self
                }
                #[doc = "Appends the extra query terms to the query."]
                pub fn hq(mut self, value: impl Into<String>) -> Self {
                    self.hq = Some(value.into());
                    self
                }
                #[doc = "Returns black and white, grayscale, or color images: mono, gray, and color."]
                pub fn img_color_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgColorType,
                ) -> Self {
                    self.img_color_type = Some(value);
                    self
                }
                #[doc = "Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown."]
                pub fn img_dominant_color(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgDominantColor,
                ) -> Self {
                    self.img_dominant_color = Some(value);
                    self
                }
                #[doc = "Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge."]
                pub fn img_size(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgSize,
                ) -> Self {
                    self.img_size = Some(value);
                    self
                }
                #[doc = "Returns images of a type, which can be one of: clipart, face, lineart, news, and photo."]
                pub fn img_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgType,
                ) -> Self {
                    self.img_type = Some(value);
                    self
                }
                #[doc = "Specifies that all search results should contain a link to a particular URL"]
                pub fn link_site(mut self, value: impl Into<String>) -> Self {
                    self.link_site = Some(value.into());
                    self
                }
                #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
                pub fn low_range(mut self, value: impl Into<String>) -> Self {
                    self.low_range = Some(value.into());
                    self
                }
                #[doc = "The language restriction for the search results"]
                pub fn lr(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListLr,
                ) -> Self {
                    self.lr = Some(value);
                    self
                }
                #[doc = "Number of search results to return"]
                pub fn num(mut self, value: u32) -> Self {
                    self.num = Some(value);
                    self
                }
                #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms"]
                pub fn or_terms(mut self, value: impl Into<String>) -> Self {
                    self.or_terms = Some(value.into());
                    self
                }
                #[doc = "Specifies that all search results should be pages that are related to the specified URL"]
                pub fn related_site(mut self, value: impl Into<String>) -> Self {
                    self.related_site = Some(value.into());
                    self
                }
                #[doc = "Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these."]
                pub fn rights(mut self, value: impl Into<String>) -> Self {
                    self.rights = Some(value.into());
                    self
                }
                #[doc = "Search safety level"]
                pub fn safe(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSafe,
                ) -> Self {
                    self.safe = Some(value);
                    self
                }
                #[doc = "Specifies the search type: image."]
                pub fn search_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSearchType,
                ) -> Self {
                    self.search_type = Some(value);
                    self
                }
                #[doc = "Specifies all search results should be pages from a given site"]
                pub fn site_search(mut self, value: impl Into<String>) -> Self {
                    self.site_search = Some(value.into());
                    self
                }
                #[doc = "Controls whether to include or exclude results from the site named in the as_sitesearch parameter"]
                pub fn site_search_filter(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSiteSearchFilter,
                ) -> Self {
                    self.site_search_filter = Some(value);
                    self
                }
                #[doc = "The sort expression to apply to the results"]
                pub fn sort(mut self, value: impl Into<String>) -> Self {
                    self.sort = Some(value.into());
                    self
                }
                #[doc = "The index of the first result to return"]
                pub fn start(mut self, value: u32) -> Self {
                    self.start = Some(value);
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
                ) -> Result<crate::schemas::Search, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Search, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/customsearch/".to_owned();
                    output.push_str("v1/siterestrict");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("q", &self.q)]);
                    let req = req.query(&[("c2coff", &self.c_2coff)]);
                    let req = req.query(&[("cr", &self.cr)]);
                    let req = req.query(&[("cx", &self.cx)]);
                    let req = req.query(&[("dateRestrict", &self.date_restrict)]);
                    let req = req.query(&[("exactTerms", &self.exact_terms)]);
                    let req = req.query(&[("excludeTerms", &self.exclude_terms)]);
                    let req = req.query(&[("fileType", &self.file_type)]);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("gl", &self.gl)]);
                    let req = req.query(&[("googlehost", &self.googlehost)]);
                    let req = req.query(&[("highRange", &self.high_range)]);
                    let req = req.query(&[("hl", &self.hl)]);
                    let req = req.query(&[("hq", &self.hq)]);
                    let req = req.query(&[("imgColorType", &self.img_color_type)]);
                    let req = req.query(&[("imgDominantColor", &self.img_dominant_color)]);
                    let req = req.query(&[("imgSize", &self.img_size)]);
                    let req = req.query(&[("imgType", &self.img_type)]);
                    let req = req.query(&[("linkSite", &self.link_site)]);
                    let req = req.query(&[("lowRange", &self.low_range)]);
                    let req = req.query(&[("lr", &self.lr)]);
                    let req = req.query(&[("num", &self.num)]);
                    let req = req.query(&[("orTerms", &self.or_terms)]);
                    let req = req.query(&[("relatedSite", &self.related_site)]);
                    let req = req.query(&[("rights", &self.rights)]);
                    let req = req.query(&[("safe", &self.safe)]);
                    let req = req.query(&[("searchType", &self.search_type)]);
                    let req = req.query(&[("siteSearch", &self.site_search)]);
                    let req = req.query(&[("siteSearchFilter", &self.site_search_filter)]);
                    let req = req.query(&[("sort", &self.sort)]);
                    let req = req.query(&[("start", &self.start)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
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
