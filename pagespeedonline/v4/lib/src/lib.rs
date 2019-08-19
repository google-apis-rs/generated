pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV4 {
        #[doc = "List of arguments for the format string."]
        #[serde(rename = "args", default)]
        pub args: Option<Vec<crate::schemas::PagespeedApiFormatStringV4ArgsItems>>,
        #[doc = "A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'."]
        #[serde(rename = "format", default)]
        pub format: Option<String>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiFormatStringV4 {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV4ArgsItems {
        #[doc = "The placeholder key for this arg, as a string."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot."]
        #[serde(rename = "rects", default)]
        pub rects: Option<Vec<crate::schemas::PagespeedApiFormatStringV4ArgsItemsRectsItems>>,
        #[doc = "Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments."]
        #[serde(rename = "secondary_rects", default)]
        pub secondary_rects:
            Option<Vec<crate::schemas::PagespeedApiFormatStringV4ArgsItemsSecondaryRectsItems>>,
        #[doc = "Argument value, as a localized string."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiFormatStringV4ArgsItems {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV4ArgsItemsRectsItems {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "left", default)]
        pub left: Option<i32>,
        #[serde(rename = "top", default)]
        pub top: Option<i32>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiFormatStringV4ArgsItemsRectsItems {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiFormatStringV4ArgsItemsSecondaryRectsItems {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "left", default)]
        pub left: Option<i32>,
        #[serde(rename = "top", default)]
        pub top: Option<i32>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiFormatStringV4ArgsItemsSecondaryRectsItems {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiImageV4 {
        #[doc = "Image data base64 encoded."]
        #[serde(rename = "data", default)]
        pub data: Option<Vec<u8>>,
        #[doc = "Height of screenshot in pixels."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "Unique string key, if any, identifying this image."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "Mime type of image data (e.g. \"image/jpeg\")."]
        #[serde(rename = "mime_type", default)]
        pub mime_type: Option<String>,
        #[serde(rename = "page_rect", default)]
        pub page_rect: Option<crate::schemas::PagespeedApiImageV4PageRect>,
        #[doc = "Width of screenshot in pixels."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiImageV4 {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiImageV4PageRect {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "left", default)]
        pub left: Option<i32>,
        #[serde(rename = "top", default)]
        pub top: Option<i32>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiImageV4PageRect {
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
    pub struct PagespeedApiPagespeedResponseV4 {
        #[doc = "The captcha verify result"]
        #[serde(rename = "captchaResult", default)]
        pub captcha_result: Option<String>,
        #[doc = "Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server."]
        #[serde(rename = "formattedResults", default)]
        pub formatted_results:
            Option<crate::schemas::PagespeedApiPagespeedResponseV4FormattedResults>,
        #[doc = "Canonicalized and final URL for the document, after following page redirects (if any)."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "List of rules that were specified in the request, but which the server did not know how to instantiate."]
        #[serde(rename = "invalidRules", default)]
        pub invalid_rules: Option<Vec<String>>,
        #[doc = "Kind of result."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Metrics of end users' page loading experience."]
        #[serde(rename = "loadingExperience", default)]
        pub loading_experience:
            Option<crate::schemas::PagespeedApiPagespeedResponseV4LoadingExperience>,
        #[doc = "Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc."]
        #[serde(rename = "pageStats", default)]
        pub page_stats: Option<crate::schemas::PagespeedApiPagespeedResponseV4PageStats>,
        #[doc = "Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error."]
        #[serde(rename = "responseCode", default)]
        pub response_code: Option<i32>,
        #[doc = "A map with one entry for each rule group in these results."]
        #[serde(rename = "ruleGroups", default)]
        pub rule_groups: Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::PagespeedApiPagespeedResponseV4RuleGroupsAdditionalProperties,
            >,
        >,
        #[doc = "Base64-encoded screenshot of the page that was analyzed."]
        #[serde(rename = "screenshot", default)]
        pub screenshot: Option<crate::schemas::PagespeedApiImageV4>,
        #[doc = "Additional base64-encoded screenshots of the page, in various partial render states."]
        #[serde(rename = "snapshots", default)]
        pub snapshots: Option<Vec<crate::schemas::PagespeedApiImageV4>>,
        #[doc = "Title of the page, as displayed in the browser's title bar."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "The version of PageSpeed used to generate these results."]
        #[serde(rename = "version", default)]
        pub version: Option<crate::schemas::PagespeedApiPagespeedResponseV4Version>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV4 {
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
    pub struct PagespeedApiPagespeedResponseV4FormattedResults { # [ doc = "The locale of the formattedResults, e.g. \"en_US\"." ] # [ serde ( rename = "locale" , default ) ] pub locale : Option < String > , # [ doc = "Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server." ] # [ serde ( rename = "ruleResults" , default ) ] pub rule_results : Option < :: std :: collections :: BTreeMap < String , crate :: schemas :: PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalProperties > > , }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV4FormattedResults {
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
    pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalProperties { # [ doc = "Whether this rule is in 'beta'. Rules in beta are new rules that are being tested, which do not impact the overall score." ] # [ serde ( rename = "beta" , default ) ] pub beta : Option < bool > , # [ doc = "List of rule groups that this rule belongs to. Each entry in the list is one of \"SPEED\", \"USABILITY\", or \"SECURITY\"." ] # [ serde ( rename = "groups" , default ) ] pub groups : Option < Vec < String > > , # [ doc = "Localized name of the rule, intended for presentation to a user." ] # [ serde ( rename = "localizedRuleName" , default ) ] pub localized_rule_name : Option < String > , # [ doc = "The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal." ] # [ serde ( rename = "ruleImpact" , default ) ] pub rule_impact : Option < f64 > , # [ doc = "A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so." ] # [ serde ( rename = "summary" , default ) ] pub summary : Option < crate :: schemas :: PagespeedApiFormatStringV4 > , # [ doc = "List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details." ] # [ serde ( rename = "urlBlocks" , default ) ] pub url_blocks : Option < Vec < crate :: schemas :: PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems > > , }
    impl ::field_selector::FieldSelector
        for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalProperties
    {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems { # [ doc = "Heading to be displayed with the list of URLs." ] # [ serde ( rename = "header" , default ) ] pub header : Option < crate :: schemas :: PagespeedApiFormatStringV4 > , # [ doc = "List of entries that provide information about URLs in the url block. Optional." ] # [ serde ( rename = "urls" , default ) ] pub urls : Option < Vec < crate :: schemas :: PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems > > , }
    impl :: field_selector :: FieldSelector for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems { fn field_selector_with_ident ( ident : & str , selector : & mut String ) { match selector . chars ( ) . rev ( ) . nth ( 0 ) { Some ( ',' ) | None => { } , _ => selector . push_str ( "," ) , } selector . push_str ( ident ) ; selector . push_str ( "*" ) ; } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems
    {
        #[doc = "List of entries that provide additional details about a single URL. Optional."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<crate::schemas::PagespeedApiFormatStringV4>>,
        #[doc = "A format string that gives information about the URL, and a list of arguments for that format string."]
        #[serde(rename = "result", default)]
        pub result: Option<crate::schemas::PagespeedApiFormatStringV4>,
    }
    impl :: field_selector :: FieldSelector for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems { fn field_selector_with_ident ( ident : & str , selector : & mut String ) { match selector . chars ( ) . rev ( ) . nth ( 0 ) { Some ( ',' ) | None => { } , _ => selector . push_str ( "," ) , } selector . push_str ( ident ) ; selector . push_str ( "*" ) ; } }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PagespeedApiPagespeedResponseV4LoadingExperience { # [ doc = "The url, pattern or origin which the metrics are on." ] # [ serde ( rename = "id" , default ) ] pub id : Option < String > , # [ serde ( rename = "initial_url" , default ) ] pub initial_url : Option < String > , # [ serde ( rename = "metrics" , default ) ] pub metrics : Option < :: std :: collections :: BTreeMap < String , crate :: schemas :: PagespeedApiPagespeedResponseV4LoadingExperienceMetricsAdditionalProperties > > , # [ serde ( rename = "overall_category" , default ) ] pub overall_category : Option < String > , }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV4LoadingExperience {
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
    pub struct PagespeedApiPagespeedResponseV4LoadingExperienceMetricsAdditionalProperties { # [ serde ( rename = "category" , default ) ] pub category : Option < String > , # [ serde ( rename = "distributions" , default ) ] pub distributions : Option < Vec < crate :: schemas :: PagespeedApiPagespeedResponseV4LoadingExperienceMetricsAdditionalPropertiesDistributionsItems > > , # [ serde ( rename = "median" , default ) ] pub median : Option < i32 > , }
    impl ::field_selector::FieldSelector
        for PagespeedApiPagespeedResponseV4LoadingExperienceMetricsAdditionalProperties
    {
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
    pub struct PagespeedApiPagespeedResponseV4LoadingExperienceMetricsAdditionalPropertiesDistributionsItems
    {
        #[serde(rename = "max", default)]
        pub max: Option<i32>,
        #[serde(rename = "min", default)]
        pub min: Option<i32>,
        #[serde(rename = "proportion", default)]
        pub proportion: Option<f64>,
    }
    impl :: field_selector :: FieldSelector for PagespeedApiPagespeedResponseV4LoadingExperienceMetricsAdditionalPropertiesDistributionsItems { fn field_selector_with_ident ( ident : & str , selector : & mut String ) { match selector . chars ( ) . rev ( ) . nth ( 0 ) { Some ( ',' ) | None => { } , _ => selector . push_str ( "," ) , } selector . push_str ( ident ) ; selector . push_str ( "*" ) ; } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiPagespeedResponseV4PageStats {
        #[doc = "Content management system (CMS) used for the page."]
        #[serde(rename = "cms", default)]
        pub cms: Option<String>,
        #[doc = "Number of uncompressed response bytes for CSS resources on the page."]
        #[serde(rename = "cssResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub css_response_bytes: Option<i64>,
        #[doc = "Number of response bytes for flash resources on the page."]
        #[serde(rename = "flashResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub flash_response_bytes: Option<i64>,
        #[doc = "Number of uncompressed response bytes for the main HTML document and all iframes on the page."]
        #[serde(rename = "htmlResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub html_response_bytes: Option<i64>,
        #[doc = "Number of response bytes for image resources on the page."]
        #[serde(rename = "imageResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub image_response_bytes: Option<i64>,
        #[doc = "Number of uncompressed response bytes for JS resources on the page."]
        #[serde(rename = "javascriptResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub javascript_response_bytes: Option<i64>,
        #[doc = "The needed round trips to load render blocking resources"]
        #[serde(rename = "numRenderBlockingRoundTrips", default)]
        pub num_render_blocking_round_trips: Option<i32>,
        #[doc = "The needed round trips to load the full page"]
        #[serde(rename = "numTotalRoundTrips", default)]
        pub num_total_round_trips: Option<i32>,
        #[doc = "Number of CSS resources referenced by the page."]
        #[serde(rename = "numberCssResources", default)]
        pub number_css_resources: Option<i32>,
        #[doc = "Number of unique hosts referenced by the page."]
        #[serde(rename = "numberHosts", default)]
        pub number_hosts: Option<i32>,
        #[doc = "Number of JavaScript resources referenced by the page."]
        #[serde(rename = "numberJsResources", default)]
        pub number_js_resources: Option<i32>,
        #[doc = "Number of HTTP resources loaded by the page."]
        #[serde(rename = "numberResources", default)]
        pub number_resources: Option<i32>,
        #[doc = "Number of roboted resources."]
        #[serde(rename = "numberRobotedResources", default)]
        pub number_roboted_resources: Option<i32>,
        #[doc = "Number of static (i.e. cacheable) resources on the page."]
        #[serde(rename = "numberStaticResources", default)]
        pub number_static_resources: Option<i32>,
        #[doc = "Number of transient-failed resources."]
        #[serde(rename = "numberTransientFetchFailureResources", default)]
        pub number_transient_fetch_failure_resources: Option<i32>,
        #[doc = "Number of response bytes for other resources on the page."]
        #[serde(rename = "otherResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub other_response_bytes: Option<i64>,
        #[doc = "Number of over-the-wire bytes, uses the default gzip compression strategy as an estimation."]
        #[serde(rename = "overTheWireResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub over_the_wire_response_bytes: Option<i64>,
        #[doc = "List of roboted urls."]
        #[serde(rename = "robotedUrls", default)]
        pub roboted_urls: Option<Vec<String>>,
        #[doc = "Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page."]
        #[serde(rename = "textResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub text_response_bytes: Option<i64>,
        #[doc = "Total size of all request bytes sent by the page."]
        #[serde(rename = "totalRequestBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_request_bytes: Option<i64>,
        #[doc = "List of transient fetch failure urls."]
        #[serde(rename = "transientFetchFailureUrls", default)]
        pub transient_fetch_failure_urls: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV4PageStats {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiPagespeedResponseV4RuleGroupsAdditionalProperties {
        #[serde(rename = "pass", default)]
        pub pass: Option<bool>,
        #[doc = "The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable, or how much more secure). A high score indicates little room for improvement, while a lower score indicates more room for improvement."]
        #[serde(rename = "score", default)]
        pub score: Option<i32>,
    }
    impl ::field_selector::FieldSelector
        for PagespeedApiPagespeedResponseV4RuleGroupsAdditionalProperties
    {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PagespeedApiPagespeedResponseV4Version {
        #[doc = "The major version number of PageSpeed used to generate these results."]
        #[serde(rename = "major", default)]
        pub major: Option<i32>,
        #[doc = "The minor version number of PageSpeed used to generate these results."]
        #[serde(rename = "minor", default)]
        pub minor: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV4Version {
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
    #[doc = "Actions that can be performed on the pagespeedapi resource"]
    pub fn pagespeedapi(&self) -> crate::pagespeedapi::PagespeedapiActions<A> {
        crate::pagespeedapi::PagespeedapiActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod pagespeedapi {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for RunpagespeedStrategy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RunpagespeedStrategy {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RunpagespeedStrategy {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    }
    pub struct PagespeedapiActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PagespeedapiActions<'a, A> {
        #[doc = "Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information."]
        pub fn runpagespeed(&self, url: impl Into<String>) -> RunpagespeedRequestBuilder<A> {
            RunpagespeedRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
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
                snapshots: None,
                strategy: None,
                utm_campaign: None,
                utm_source: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct RunpagespeedRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        url: String,
        filter_third_party_resources: Option<bool>,
        locale: Option<String>,
        rule: Option<String>,
        screenshot: Option<bool>,
        snapshots: Option<bool>,
        strategy: Option<crate::pagespeedapi::params::RunpagespeedStrategy>,
        utm_campaign: Option<String>,
        utm_source: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RunpagespeedRequestBuilder<'a, A> {
        #[doc = "Indicates if third party resources should be filtered out before PageSpeed analysis."]
        pub fn filter_third_party_resources(&mut self, value: bool) -> &mut Self {
            self.filter_third_party_resources = Some(value);
            self
        }
        #[doc = "The locale used to localize formatted results"]
        pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
            self.locale = Some(value.into());
            self
        }
        #[doc = "A PageSpeed rule to run; if none are given, all rules are run"]
        pub fn rule(&mut self, value: impl Into<String>) -> &mut Self {
            self.rule = Some(value.into());
            self
        }
        #[doc = "Indicates if binary data containing a screenshot should be included"]
        pub fn screenshot(&mut self, value: bool) -> &mut Self {
            self.screenshot = Some(value);
            self
        }
        #[doc = "Indicates if binary data containing snapshot images should be included"]
        pub fn snapshots(&mut self, value: bool) -> &mut Self {
            self.snapshots = Some(value);
            self
        }
        #[doc = "The analysis strategy (desktop or mobile) to use, and desktop is the default"]
        pub fn strategy(
            &mut self,
            value: crate::pagespeedapi::params::RunpagespeedStrategy,
        ) -> &mut Self {
            self.strategy = Some(value);
            self
        }
        #[doc = "Campaign name for analytics."]
        pub fn utm_campaign(&mut self, value: impl Into<String>) -> &mut Self {
            self.utm_campaign = Some(value.into());
            self
        }
        #[doc = "Campaign source for analytics."]
        pub fn utm_source(&mut self, value: impl Into<String>) -> &mut Self {
            self.utm_source = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
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
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
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
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::PagespeedApiPagespeedResponseV4, Box<dyn ::std::error::Error>>
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
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/pagespeedonline/v4/".to_owned();
            output.push_str("runPagespeed");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("url", &self.url)]);
            let req = req.query(&[(
                "filter_third_party_resources",
                &self.filter_third_party_resources,
            )]);
            let req = req.query(&[("locale", &self.locale)]);
            let req = req.query(&[("rule", &self.rule)]);
            let req = req.query(&[("screenshot", &self.screenshot)]);
            let req = req.query(&[("snapshots", &self.snapshots)]);
            let req = req.query(&[("strategy", &self.strategy)]);
            let req = req.query(&[("utm_campaign", &self.utm_campaign)]);
            let req = req.query(&[("utm_source", &self.utm_source)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
