#![doc = "# Resources and Methods\n    * [pagespeedapi](resources/pagespeedapi/struct.PagespeedapiActions.html)\n      * [*runpagespeed*](resources/pagespeedapi/struct.RunpagespeedRequestBuilder.html)\n"]
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
    pub struct PagespeedApiFormatStringV2 {
        #[doc = "List of arguments for the format string."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<crate::schemas::PagespeedApiFormatStringV2ArgsItems>>,
        #[doc = "A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiFormatStringV2 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiFormatStringV2 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV2ArgsItems {
        #[doc = "The placeholder key for this arg, as a string."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot."]
        #[serde(
            rename = "rects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rects: ::std::option::Option<
            Vec<crate::schemas::PagespeedApiFormatStringV2ArgsItemsRectsItems>,
        >,
        #[doc = "Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments."]
        #[serde(
            rename = "secondary_rects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secondary_rects: ::std::option::Option<
            Vec<crate::schemas::PagespeedApiFormatStringV2ArgsItemsSecondaryRectsItems>,
        >,
        #[doc = "Argument value, as a localized string."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiFormatStringV2ArgsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiFormatStringV2ArgsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV2ArgsItemsRectsItems {
        #[doc = "The height of the rect."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The left coordinate of the rect, in page coordinates."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<i32>,
        #[doc = "The top coordinate of the rect, in page coordinates."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<i32>,
        #[doc = "The width of the rect."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiFormatStringV2ArgsItemsRectsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiFormatStringV2ArgsItemsRectsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV2ArgsItemsSecondaryRectsItems {
        #[doc = "The height of the rect."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The left coordinate of the rect, in page coordinates."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<i32>,
        #[doc = "The top coordinate of the rect, in page coordinates."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<i32>,
        #[doc = "The width of the rect."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for PagespeedApiFormatStringV2ArgsItemsSecondaryRectsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for PagespeedApiFormatStringV2ArgsItemsSecondaryRectsItems
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
    pub struct PagespeedApiImageV2 {
        #[doc = "Image data base64 encoded."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Height of screenshot in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Unique string key, if any, identifying this image."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Mime type of image data (e.g. \"image/jpeg\")."]
        #[serde(
            rename = "mime_type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The region of the page that is captured by this image, with dimensions measured in CSS pixels."]
        #[serde(
            rename = "page_rect",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_rect: ::std::option::Option<crate::schemas::PagespeedApiImageV2PageRect>,
        #[doc = "Width of screenshot in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiImageV2 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiImageV2 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiImageV2PageRect {
        #[doc = "The height of the rect."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The left coordinate of the rect, in page coordinates."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<i32>,
        #[doc = "The top coordinate of the rect, in page coordinates."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<i32>,
        #[doc = "The width of the rect."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PagespeedApiImageV2PageRect {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagespeedApiImageV2PageRect {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Result {
        #[doc = "The captcha verify result"]
        #[serde(
            rename = "captchaResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_result: ::std::option::Option<String>,
        #[doc = "Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server."]
        #[serde(
            rename = "formattedResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_results: ::std::option::Option<crate::schemas::ResultFormattedResults>,
        #[doc = "Canonicalized and final URL for the document, after following page redirects (if any)."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "List of rules that were specified in the request, but which the server did not know how to instantiate."]
        #[serde(
            rename = "invalidRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invalid_rules: ::std::option::Option<Vec<String>>,
        #[doc = "Kind of result."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc."]
        #[serde(
            rename = "pageStats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_stats: ::std::option::Option<crate::schemas::ResultPageStats>,
        #[doc = "Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error."]
        #[serde(
            rename = "responseCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_code: ::std::option::Option<i32>,
        #[doc = "A map with one entry for each rule group in these results."]
        #[serde(
            rename = "ruleGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule_groups: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::ResultRuleGroupsAdditionalProperties,
            >,
        >,
        #[doc = "Base64-encoded screenshot of the page that was analyzed."]
        #[serde(
            rename = "screenshot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screenshot: ::std::option::Option<crate::schemas::PagespeedApiImageV2>,
        #[doc = "Title of the page, as displayed in the browser's title bar."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The version of PageSpeed used to generate these results."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::ResultVersion>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultFormattedResults {
        #[doc = "The locale of the formattedResults, e.g. \"en_US\"."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server."]
        #[serde(
            rename = "ruleResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule_results: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::ResultFormattedResultsRuleResultsAdditionalProperties,
            >,
        >,
    }
    impl ::google_field_selector::FieldSelector for ResultFormattedResults {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultFormattedResults {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalProperties {
        #[doc = "List of rule groups that this rule belongs to. Each entry in the list is one of \"SPEED\" or \"USABILITY\"."]
        #[serde(
            rename = "groups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub groups: ::std::option::Option<Vec<String>>,
        #[doc = "Localized name of the rule, intended for presentation to a user."]
        #[serde(
            rename = "localizedRuleName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub localized_rule_name: ::std::option::Option<String>,
        #[doc = "The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal."]
        #[serde(
            rename = "ruleImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule_impact: ::std::option::Option<f64>,
        #[doc = "A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so."]
        #[serde(
            rename = "summary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary: ::std::option::Option<crate::schemas::PagespeedApiFormatStringV2>,
        #[doc = "List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details."]
        #[serde(
            rename = "urlBlocks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_blocks: ::std::option::Option<
            Vec<
                crate::schemas::ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems,
            >,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalProperties
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ResultFormattedResultsRuleResultsAdditionalProperties
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
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems { # [ doc = "Heading to be displayed with the list of URLs." ] # [ serde ( rename = "header" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub header : :: std :: option :: Option < crate :: schemas :: PagespeedApiFormatStringV2 > , # [ doc = "List of entries that provide information about URLs in the url block. Optional." ] # [ serde ( rename = "urls" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub urls : :: std :: option :: Option < Vec < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems > > , }
    impl ::google_field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems
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
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems {
        #[doc = "List of entries that provide additional details about a single URL. Optional."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<crate::schemas::PagespeedApiFormatStringV2>>,
        #[doc = "A format string that gives information about the URL, and a list of arguments for that format string."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<crate::schemas::PagespeedApiFormatStringV2>,
    }
    impl ::google_field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems
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
    pub struct ResultPageStats {
        #[doc = "Number of uncompressed response bytes for CSS resources on the page."]
        #[serde(
            rename = "cssResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub css_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of response bytes for flash resources on the page."]
        #[serde(
            rename = "flashResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub flash_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of uncompressed response bytes for the main HTML document and all iframes on the page."]
        #[serde(
            rename = "htmlResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub html_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of response bytes for image resources on the page."]
        #[serde(
            rename = "imageResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub image_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of uncompressed response bytes for JS resources on the page."]
        #[serde(
            rename = "javascriptResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub javascript_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of CSS resources referenced by the page."]
        #[serde(
            rename = "numberCssResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_css_resources: ::std::option::Option<i32>,
        #[doc = "Number of unique hosts referenced by the page."]
        #[serde(
            rename = "numberHosts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_hosts: ::std::option::Option<i32>,
        #[doc = "Number of JavaScript resources referenced by the page."]
        #[serde(
            rename = "numberJsResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_js_resources: ::std::option::Option<i32>,
        #[doc = "Number of HTTP resources loaded by the page."]
        #[serde(
            rename = "numberResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_resources: ::std::option::Option<i32>,
        #[doc = "Number of static (i.e. cacheable) resources on the page."]
        #[serde(
            rename = "numberStaticResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_static_resources: ::std::option::Option<i32>,
        #[doc = "Number of response bytes for other resources on the page."]
        #[serde(
            rename = "otherResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub other_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page."]
        #[serde(
            rename = "textResponseBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub text_response_bytes: ::std::option::Option<i64>,
        #[doc = "Total size of all request bytes sent by the page."]
        #[serde(
            rename = "totalRequestBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_request_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ResultPageStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultPageStats {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultRuleGroupsAdditionalProperties {
        #[doc = "The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable). A high score indicates little room for improvement, while a lower score indicates more room for improvement."]
        #[serde(
            rename = "score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ResultRuleGroupsAdditionalProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultRuleGroupsAdditionalProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultVersion {
        #[doc = "The major version number of PageSpeed used to generate these results."]
        #[serde(
            rename = "major",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major: ::std::option::Option<i32>,
        #[doc = "The minor version number of PageSpeed used to generate these results."]
        #[serde(
            rename = "minor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minor: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ResultVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultVersion {
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
            pub enum RunpagespeedStrategy {
                #[doc = "Fetch and analyze the URL for desktop browsers"]
                Desktop,
                #[doc = "Fetch and analyze the URL for mobile devices"]
                Mobile,
            }
            impl RunpagespeedStrategy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RunpagespeedStrategy::Desktop => "desktop",
                        RunpagespeedStrategy::Mobile => "mobile",
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
                        "desktop" => RunpagespeedStrategy::Desktop,
                        "mobile" => RunpagespeedStrategy::Mobile,
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
                        "desktop" => RunpagespeedStrategy::Desktop,
                        "mobile" => RunpagespeedStrategy::Mobile,
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
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PagespeedapiActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information."]
            pub fn runpagespeed(&self, url: impl Into<String>) -> RunpagespeedRequestBuilder {
                RunpagespeedRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    url: url.into(),
                    filter_third_party_resources: None,
                    locale: None,
                    rule: None,
                    screenshot: None,
                    strategy: None,
                }
            }
        }
        #[doc = "Created via [PagespeedapiActions::runpagespeed()](struct.PagespeedapiActions.html#method.runpagespeed)"]
        #[derive(Debug, Clone)]
        pub struct RunpagespeedRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            url: String,
            filter_third_party_resources: Option<bool>,
            locale: Option<String>,
            rule: Option<Vec<String>>,
            screenshot: Option<bool>,
            strategy: Option<crate::resources::pagespeedapi::params::RunpagespeedStrategy>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> RunpagespeedRequestBuilder<'a> {
            #[doc = "Indicates if third party resources should be filtered out before PageSpeed analysis."]
            pub fn filter_third_party_resources(mut self, value: bool) -> Self {
                self.filter_third_party_resources = Some(value);
                self
            }
            #[doc = "The locale used to localize formatted results"]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "A PageSpeed rule to run; if none are given, all rules are run"]
            pub fn rule(mut self, value: impl Into<Vec<String>>) -> Self {
                self.rule = Some(value.into());
                self
            }
            #[doc = "Indicates if binary data containing a screenshot should be included"]
            pub fn screenshot(mut self, value: bool) -> Self {
                self.screenshot = Some(value);
                self
            }
            #[doc = "The analysis strategy to use"]
            pub fn strategy(
                mut self,
                value: crate::resources::pagespeedapi::params::RunpagespeedStrategy,
            ) -> Self {
                self.strategy = Some(value);
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
            ) -> Result<crate::schemas::Result, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Result, crate::Error> {
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
                let mut output = "https://www.googleapis.com/pagespeedonline/v2/".to_owned();
                output.push_str("runPagespeed");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("url", &self.url)]);
                let req = req.query(&[(
                    "filter_third_party_resources",
                    &self.filter_third_party_resources,
                )]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("rule", &self.rule)]);
                let req = req.query(&[("screenshot", &self.screenshot)]);
                let req = req.query(&[("strategy", &self.strategy)]);
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
