#![doc = "# Resources and Methods\n    * [cse](resources/cse/struct.CseActions.html)\n      * [*list*](resources/cse/struct.ListRequestBuilder.html)\n      * [siterestrict](resources/cse/siterestrict/struct.SiterestrictActions.html)\n        * [*list*](resources/cse/siterestrict/struct.ListRequestBuilder.html)\n"]
pub mod scopes {}
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
    pub struct Promotion {
        #[doc = "An array of block objects for this promotion. See [Google WebSearch\nProtocol\nreference](https://developers.google.com/custom-search/docs/xml_results)\nfor more information."]
        #[serde(
            rename = "bodyLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body_lines: ::std::option::Option<Vec<crate::schemas::PromotionBodyLinesItems>>,
        #[doc = "An abridged version of this search's result URL, e.g. www.example.com."]
        #[serde(
            rename = "displayLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_link: ::std::option::Option<String>,
        #[doc = "The title of the promotion, in HTML."]
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[doc = "Image belonging to a promotion."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::PromotionImage>,
        #[doc = "The URL of the promotion."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "The title of the promotion."]
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
        #[doc = "The block object's text in HTML, if it has text."]
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[doc = "The anchor text of the block object's link, if it has a link."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "The block object's text, if it has text."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The URL of the block object's link, if it has one."]
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
        #[doc = "Image height in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "URL of the image for this promotion link."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "Image width in pixels."]
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Result {
        #[doc = "Indicates the ID of Google's cached version of the search result."]
        #[serde(
            rename = "cacheId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_id: ::std::option::Option<String>,
        #[doc = "An abridged version of this search result’s URL, e.g. www.example.com."]
        #[serde(
            rename = "displayLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_link: ::std::option::Option<String>,
        #[doc = "The file format of the search result."]
        #[serde(
            rename = "fileFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_format: ::std::option::Option<String>,
        #[doc = "The URL displayed after the snippet for each search result."]
        #[serde(
            rename = "formattedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_url: ::std::option::Option<String>,
        #[doc = "The HTML-formatted URL displayed after the snippet for each search result."]
        #[serde(
            rename = "htmlFormattedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_formatted_url: ::std::option::Option<String>,
        #[doc = "The snippet of the search result, in HTML."]
        #[serde(
            rename = "htmlSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_snippet: ::std::option::Option<String>,
        #[doc = "The title of the search result, in HTML."]
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[doc = "Image belonging to a custom search result."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::ResultImage>,
        #[doc = "A unique identifier for the type of current object. For this API, it is\n`customsearch#result.`"]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Encapsulates all information about [refinement\nlabels](https://developers.google.com/custom-search/docs/xml_results)."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::ResultLabelsItems>>,
        #[doc = "The full URL to which the search result is pointing, e.g.\nhttp://www.example.com/foo/bar."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "The MIME type of the search result."]
        #[serde(
            rename = "mime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime: ::std::option::Option<String>,
        #[doc = "Contains\n[PageMap](https://developers.google.com/custom-search/docs/structured_data#pagemaps)\ninformation for this search result."]
        #[serde(
            rename = "pagemap",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pagemap:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The snippet of the search result, in plain text."]
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<String>,
        #[doc = "The title of the search result, in plain text."]
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
        #[doc = "The size of the image, in pixels."]
        #[serde(
            rename = "byteSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub byte_size: ::std::option::Option<i32>,
        #[doc = "A URL pointing to the webpage hosting the image."]
        #[serde(
            rename = "contextLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_link: ::std::option::Option<String>,
        #[doc = "The height of the image, in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The height of the thumbnail image, in pixels."]
        #[serde(
            rename = "thumbnailHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_height: ::std::option::Option<i32>,
        #[doc = "A URL to the thumbnail image."]
        #[serde(
            rename = "thumbnailLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_link: ::std::option::Option<String>,
        #[doc = "The width of the thumbnail image, in pixels."]
        #[serde(
            rename = "thumbnailWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_width: ::std::option::Option<i32>,
        #[doc = "The width of the image, in pixels."]
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
        #[doc = "The display name of a refinement label. This is the name you should\ndisplay in your user interface."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Refinement label and the associated refinement operation."]
        #[serde(
            rename = "label_with_op",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_with_op: ::std::option::Option<String>,
        #[doc = "The name of a refinement label, which you can use to refine searches.\nDon't display this in your user interface; instead, use displayName."]
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
        #[doc = "Metadata and refinements associated with the given search engine,\nincluding:\n\n* The name of the search engine that was used for the query.\n\n* A set of [facet\n  objects](https://developers.google.com/custom-search/docs/refinements#create)\n  (refinements) you can use for refining a search."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The current set of custom search results."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Result>>,
        #[doc = "Unique identifier for the type of current object. For this API, it is\ncustomsearch#search."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The set of\n[promotions](https://developers.google.com/custom-search/docs/promotions).\nPresent only if the custom search engine's configuration files define any\npromotions for the given query."]
        #[serde(
            rename = "promotions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotions: ::std::option::Option<Vec<crate::schemas::Promotion>>,
        #[doc = "Query metadata for the previous, current, and next pages of results."]
        #[serde(
            rename = "queries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queries: ::std::option::Option<crate::schemas::SearchQueries>,
        #[doc = "Metadata about a search operation."]
        #[serde(
            rename = "searchInformation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_information: ::std::option::Option<crate::schemas::SearchSearchInformation>,
        #[doc = "Spell correction information for a query."]
        #[serde(
            rename = "spelling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spelling: ::std::option::Option<crate::schemas::SearchSpelling>,
        #[doc = "OpenSearch template and URL."]
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchQueries {
        #[doc = "Metadata representing the next page of results, if applicable."]
        #[serde(
            rename = "nextPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page: ::std::option::Option<Vec<crate::schemas::SearchQueriesNextPageItems>>,
        #[doc = "Metadata representing the previous page of results, if applicable."]
        #[serde(
            rename = "previousPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub previous_page:
            ::std::option::Option<Vec<crate::schemas::SearchQueriesPreviousPageItems>>,
        #[doc = "Metadata representing the current request."]
        #[serde(
            rename = "request",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request: ::std::option::Option<Vec<crate::schemas::SearchQueriesRequestItems>>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueries {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchQueriesNextPageItems {
        #[doc = "Number of search results returned in this set."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Restricts search results to documents originating in a particular\ncountry. You may use [Boolean\noperators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators)\nin the `cr` parameter's value.\n\nGoogle WebSearch determines the country of a document by analyzing the\nfollowing:\n\n* The top-level domain (TLD) of the document's URL.\n\n* The geographic location of the web server's IP address.\n\nSee [Country (cr) Parameter\nValues](https://developers.google.com/custom-search/docs/xml_results#countryCollections)\nfor a list of valid values for this parameter."]
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[doc = "The identifier of a custom search engine created using the Custom Search\n[Control Panel](https://cse.google.com/). This is a custom property not\ndefined in the OpenSearch spec. This parameter is **required**."]
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs based on date. Supported values include:\n\n* `d[number]`: requests results from the specified number of past days.\n\n* `w[number]`: requests results from the specified number of past weeks.\n\n* `m[number]`: requests results from the specified number of past months.\n\n* `y[number]`: requests results from the specified number of past years."]
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[doc = "Enables or disables the [Simplified and Traditional Chinese\nSearch](https://developers.google.com/custom-search/docs/xml_results#chineseSearch)\nfeature.\n\nSupported values are:\n\n* `0`: enabled (default)\n\n* `1`: disabled"]
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[doc = "Identifies a phrase that all documents in the search results must\ncontain."]
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[doc = "Identifies a word or phrase that should not appear in any documents in\nthe search results."]
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[doc = "Restricts results to files of a specified extension. Filetypes supported\nby Google include:\n\n* Adobe Portable Document Format (`pdf`)\n\n* Adobe PostScript (`ps`)\n\n* Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`)\n\n* Lotus WordPro (`lwp`)\n\n* Macwrite (`mw`)\n\n* Microsoft Excel (`xls`)\n\n* Microsoft PowerPoint (`ppt`)\n\n* Microsoft Word (`doc`)\n\n* Microsoft Works (`wks`, `wps`, `wdb`)\n\n* Microsoft Write (`wri`)\n\n* Rich Text Format (`rtf`)\n\n* Shockwave Flash (`swf`)\n\n* Text (`ans`, `txt`).\n\nAdditional filetypes may be added in the future. An up-to-date list can\nalways be found in Google's [file type\nFAQ](https://support.google.com/webmasters/answer/35287)."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[doc = "Activates or deactivates the automatic filtering of Google search\nresults. See [Automatic\nFiltering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)\nfor more information about Google's search results filters. Valid values\nfor this parameter are:\n\n* `0`: Disabled\n\n* `1`: Enabled (default)\n\n**Note**: By default, Google applies filtering to all search results to\nimprove the quality of those results."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Boosts search results whose country of origin matches the parameter\nvalue. See [Country\nCodes](https://developers.google.com/custom-search/docs/xml_results#countryCodes)\nfor a list of valid values.\n\nSpecifying a `gl` parameter value in WebSearch requests should improve\nthe relevance of results. This is particularly true for international\ncustomers and, even more specifically, for customers in English-speaking\ncountries other than the United States."]
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or\ngoogle.fr) to which the search should be limited."]
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and\n`cse:highrange` to append an inclusive search range of\n`lowRange...highRange` to the query."]
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[doc = "Specifies the interface language (host language) of your user interface.\nExplicitly setting this parameter improves the performance and the\nquality of your search results.\n\nSee the [Interface\nLanguages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)\nsection of [Internationalizing Queries and Results\nPresentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)\nfor more information, and [Supported Interface\nLanguages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages)\nfor a list of supported languages."]
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[doc = "Appends the specified query terms to the query, as if they were combined\nwith a logical `AND` operator."]
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified color type. Supported values\nare:\n\n* `mono` (black and white)\n\n* `gray` (grayscale)\n\n* `color` (color)"]
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[doc = "Restricts results to images with a specific dominant color. Supported\nvalues are:\n\n* `red`\n\n* `orange`\n\n* `yellow`\n\n* `green`\n\n* `teal`\n\n* `blue`\n\n* `purple`\n\n* `pink`\n\n* `white`\n\n* `gray`\n\n* `black`\n\n* `brown`"]
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified size. Supported values are:\n\n* `icon` (small)\n\n* `small | medium | large | xlarge` (medium)\n\n* `xxlarge` (large)\n\n* `huge` (extra-large)"]
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified type. Supported values are:\n\n* `clipart` (Clip art)\n\n* `face` (Face)\n\n* `lineart` (Line drawing)\n\n* `photo` (Photo)\n\n* `animated` (Animated)\n\n* `stock` (Stock)"]
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search requests."]
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[doc = "The language of the search results."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and\n`cse:highrange` to append an inclusive search range of\n`lowRange...highRange` to the query."]
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[doc = "Provides additional search terms to check for in a document, where each\ndocument in the search results must contain at least one of the\nadditional search terms. You can also use the [Boolean\nOR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt)\nquery term for this type of query."]
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search results."]
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[doc = "Specifies that all search results should be pages that are related to the\nspecified URL. The parameter value should be a URL."]
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[doc = "Filters based on licensing. Supported values include:\n\n* `cc_publicdomain`\n\n* `cc_attribute`\n\n* `cc_sharealike`\n\n* `cc_noncommercial`\n\n* `cc_nonderived`"]
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[doc = "Specifies the [SafeSearch\nlevel](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels)\nused for filtering out adult results. This is a custom property not\ndefined in the OpenSearch spec. Valid parameter values are:\n\n* `\"off\"`: Disable SafeSearch\n\n* `\"active\"`: Enable SafeSearch"]
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[doc = "The search terms entered by the user."]
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited\nto webpages."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs from a specified site."]
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[doc = "Specifies whether to include or exclude results from the site named in\nthe `sitesearch` parameter. Supported values are:\n\n* `i`: include content from site\n\n* `e`: exclude content from site"]
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[doc = "Specifies that results should be sorted according to the specified\nexpression. For example, sort by date."]
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[doc = "The index of the current set of search results into the total set of\nresults, where the index of the first result is 1."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The page number of this set of results, where the page length is set by\nthe `count` property."]
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[doc = "A description of the query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Estimated number of total search results. May not be accurate."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueriesNextPageItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueriesNextPageItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchQueriesPreviousPageItems {
        #[doc = "Number of search results returned in this set."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Restricts search results to documents originating in a particular\ncountry. You may use [Boolean\noperators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators)\nin the `cr` parameter's value.\n\nGoogle WebSearch determines the country of a document by analyzing the\nfollowing:\n\n* The top-level domain (TLD) of the document's URL.\n\n* The geographic location of the web server's IP address.\n\nSee [Country (cr) Parameter\nValues](https://developers.google.com/custom-search/docs/xml_results#countryCollections)\nfor a list of valid values for this parameter."]
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[doc = "The identifier of a custom search engine created using the Custom Search\n[Control Panel](https://cse.google.com/). This is a custom property not\ndefined in the OpenSearch spec. This parameter is **required**."]
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs based on date. Supported values include:\n\n* `d[number]`: requests results from the specified number of past days.\n\n* `w[number]`: requests results from the specified number of past weeks.\n\n* `m[number]`: requests results from the specified number of past months.\n\n* `y[number]`: requests results from the specified number of past years."]
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[doc = "Enables or disables the [Simplified and Traditional Chinese\nSearch](https://developers.google.com/custom-search/docs/xml_results#chineseSearch)\nfeature.\n\nSupported values are:\n\n* `0`: enabled (default)\n\n* `1`: disabled"]
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[doc = "Identifies a phrase that all documents in the search results must\ncontain."]
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[doc = "Identifies a word or phrase that should not appear in any documents in\nthe search results."]
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[doc = "Restricts results to files of a specified extension. Filetypes supported\nby Google include:\n\n* Adobe Portable Document Format (`pdf`)\n\n* Adobe PostScript (`ps`)\n\n* Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`)\n\n* Lotus WordPro (`lwp`)\n\n* Macwrite (`mw`)\n\n* Microsoft Excel (`xls`)\n\n* Microsoft PowerPoint (`ppt`)\n\n* Microsoft Word (`doc`)\n\n* Microsoft Works (`wks`, `wps`, `wdb`)\n\n* Microsoft Write (`wri`)\n\n* Rich Text Format (`rtf`)\n\n* Shockwave Flash (`swf`)\n\n* Text (`ans`, `txt`).\n\nAdditional filetypes may be added in the future. An up-to-date list can\nalways be found in Google's [file type\nFAQ](https://support.google.com/webmasters/answer/35287)."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[doc = "Activates or deactivates the automatic filtering of Google search\nresults. See [Automatic\nFiltering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)\nfor more information about Google's search results filters. Valid values\nfor this parameter are:\n\n* `0`: Disabled\n\n* `1`: Enabled (default)\n\n**Note**: By default, Google applies filtering to all search results to\nimprove the quality of those results."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Boosts search results whose country of origin matches the parameter\nvalue. See [Country\nCodes](https://developers.google.com/custom-search/docs/xml_results#countryCodes)\nfor a list of valid values.\n\nSpecifying a `gl` parameter value in WebSearch requests should improve\nthe relevance of results. This is particularly true for international\ncustomers and, even more specifically, for customers in English-speaking\ncountries other than the United States."]
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or\ngoogle.fr) to which the search should be limited."]
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and\n`cse:highrange` to append an inclusive search range of\n`lowRange...highRange` to the query."]
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[doc = "Specifies the interface language (host language) of your user interface.\nExplicitly setting this parameter improves the performance and the\nquality of your search results.\n\nSee the [Interface\nLanguages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)\nsection of [Internationalizing Queries and Results\nPresentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)\nfor more information, and [Supported Interface\nLanguages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages)\nfor a list of supported languages."]
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[doc = "Appends the specified query terms to the query, as if they were combined\nwith a logical `AND` operator."]
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified color type. Supported values\nare:\n\n* `mono` (black and white)\n\n* `gray` (grayscale)\n\n* `color` (color)"]
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[doc = "Restricts results to images with a specific dominant color. Supported\nvalues are:\n\n* `red`\n\n* `orange`\n\n* `yellow`\n\n* `green`\n\n* `teal`\n\n* `blue`\n\n* `purple`\n\n* `pink`\n\n* `white`\n\n* `gray`\n\n* `black`\n\n* `brown`"]
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified size. Supported values are:\n\n* `icon` (small)\n\n* `small | medium | large | xlarge` (medium)\n\n* `xxlarge` (large)\n\n* `huge` (extra-large)"]
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified type. Supported values are:\n\n* `clipart` (Clip art)\n\n* `face` (Face)\n\n* `lineart` (Line drawing)\n\n* `photo` (Photo)\n\n* `animated` (Animated)\n\n* `stock` (Stock)"]
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search requests."]
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[doc = "The language of the search results."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and\n`cse:highrange` to append an inclusive search range of\n`lowRange...highRange` to the query."]
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[doc = "Provides additional search terms to check for in a document, where each\ndocument in the search results must contain at least one of the\nadditional search terms. You can also use the [Boolean\nOR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt)\nquery term for this type of query."]
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search results."]
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[doc = "Specifies that all search results should be pages that are related to the\nspecified URL. The parameter value should be a URL."]
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[doc = "Filters based on licensing. Supported values include:\n\n* `cc_publicdomain`\n\n* `cc_attribute`\n\n* `cc_sharealike`\n\n* `cc_noncommercial`\n\n* `cc_nonderived`"]
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[doc = "Specifies the [SafeSearch\nlevel](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels)\nused for filtering out adult results. This is a custom property not\ndefined in the OpenSearch spec. Valid parameter values are:\n\n* `\"off\"`: Disable SafeSearch\n\n* `\"active\"`: Enable SafeSearch"]
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[doc = "The search terms entered by the user."]
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited\nto webpages."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs from a specified site."]
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[doc = "Specifies whether to include or exclude results from the site named in\nthe `sitesearch` parameter. Supported values are:\n\n* `i`: include content from site\n\n* `e`: exclude content from site"]
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[doc = "Specifies that results should be sorted according to the specified\nexpression. For example, sort by date."]
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[doc = "The index of the current set of search results into the total set of\nresults, where the index of the first result is 1."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The page number of this set of results, where the page length is set by\nthe `count` property."]
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[doc = "A description of the query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Estimated number of total search results. May not be accurate."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueriesPreviousPageItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueriesPreviousPageItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchQueriesRequestItems {
        #[doc = "Number of search results returned in this set."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Restricts search results to documents originating in a particular\ncountry. You may use [Boolean\noperators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators)\nin the `cr` parameter's value.\n\nGoogle WebSearch determines the country of a document by analyzing the\nfollowing:\n\n* The top-level domain (TLD) of the document's URL.\n\n* The geographic location of the web server's IP address.\n\nSee [Country (cr) Parameter\nValues](https://developers.google.com/custom-search/docs/xml_results#countryCollections)\nfor a list of valid values for this parameter."]
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[doc = "The identifier of a custom search engine created using the Custom Search\n[Control Panel](https://cse.google.com/). This is a custom property not\ndefined in the OpenSearch spec. This parameter is **required**."]
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs based on date. Supported values include:\n\n* `d[number]`: requests results from the specified number of past days.\n\n* `w[number]`: requests results from the specified number of past weeks.\n\n* `m[number]`: requests results from the specified number of past months.\n\n* `y[number]`: requests results from the specified number of past years."]
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[doc = "Enables or disables the [Simplified and Traditional Chinese\nSearch](https://developers.google.com/custom-search/docs/xml_results#chineseSearch)\nfeature.\n\nSupported values are:\n\n* `0`: enabled (default)\n\n* `1`: disabled"]
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[doc = "Identifies a phrase that all documents in the search results must\ncontain."]
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[doc = "Identifies a word or phrase that should not appear in any documents in\nthe search results."]
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[doc = "Restricts results to files of a specified extension. Filetypes supported\nby Google include:\n\n* Adobe Portable Document Format (`pdf`)\n\n* Adobe PostScript (`ps`)\n\n* Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`)\n\n* Lotus WordPro (`lwp`)\n\n* Macwrite (`mw`)\n\n* Microsoft Excel (`xls`)\n\n* Microsoft PowerPoint (`ppt`)\n\n* Microsoft Word (`doc`)\n\n* Microsoft Works (`wks`, `wps`, `wdb`)\n\n* Microsoft Write (`wri`)\n\n* Rich Text Format (`rtf`)\n\n* Shockwave Flash (`swf`)\n\n* Text (`ans`, `txt`).\n\nAdditional filetypes may be added in the future. An up-to-date list can\nalways be found in Google's [file type\nFAQ](https://support.google.com/webmasters/answer/35287)."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[doc = "Activates or deactivates the automatic filtering of Google search\nresults. See [Automatic\nFiltering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)\nfor more information about Google's search results filters. Valid values\nfor this parameter are:\n\n* `0`: Disabled\n\n* `1`: Enabled (default)\n\n**Note**: By default, Google applies filtering to all search results to\nimprove the quality of those results."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Boosts search results whose country of origin matches the parameter\nvalue. See [Country\nCodes](https://developers.google.com/custom-search/docs/xml_results#countryCodes)\nfor a list of valid values.\n\nSpecifying a `gl` parameter value in WebSearch requests should improve\nthe relevance of results. This is particularly true for international\ncustomers and, even more specifically, for customers in English-speaking\ncountries other than the United States."]
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or\ngoogle.fr) to which the search should be limited."]
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and\n`cse:highrange` to append an inclusive search range of\n`lowRange...highRange` to the query."]
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[doc = "Specifies the interface language (host language) of your user interface.\nExplicitly setting this parameter improves the performance and the\nquality of your search results.\n\nSee the [Interface\nLanguages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)\nsection of [Internationalizing Queries and Results\nPresentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)\nfor more information, and [Supported Interface\nLanguages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages)\nfor a list of supported languages."]
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[doc = "Appends the specified query terms to the query, as if they were combined\nwith a logical `AND` operator."]
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified color type. Supported values\nare:\n\n* `mono` (black and white)\n\n* `gray` (grayscale)\n\n* `color` (color)"]
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[doc = "Restricts results to images with a specific dominant color. Supported\nvalues are:\n\n* `red`\n\n* `orange`\n\n* `yellow`\n\n* `green`\n\n* `teal`\n\n* `blue`\n\n* `purple`\n\n* `pink`\n\n* `white`\n\n* `gray`\n\n* `black`\n\n* `brown`"]
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified size. Supported values are:\n\n* `icon` (small)\n\n* `small | medium | large | xlarge` (medium)\n\n* `xxlarge` (large)\n\n* `huge` (extra-large)"]
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified type. Supported values are:\n\n* `clipart` (Clip art)\n\n* `face` (Face)\n\n* `lineart` (Line drawing)\n\n* `photo` (Photo)\n\n* `animated` (Animated)\n\n* `stock` (Stock)"]
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search requests."]
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[doc = "The language of the search results."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and\n`cse:highrange` to append an inclusive search range of\n`lowRange...highRange` to the query."]
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[doc = "Provides additional search terms to check for in a document, where each\ndocument in the search results must contain at least one of the\nadditional search terms. You can also use the [Boolean\nOR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt)\nquery term for this type of query."]
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search results."]
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[doc = "Specifies that all search results should be pages that are related to the\nspecified URL. The parameter value should be a URL."]
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[doc = "Filters based on licensing. Supported values include:\n\n* `cc_publicdomain`\n\n* `cc_attribute`\n\n* `cc_sharealike`\n\n* `cc_noncommercial`\n\n* `cc_nonderived`"]
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[doc = "Specifies the [SafeSearch\nlevel](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels)\nused for filtering out adult results. This is a custom property not\ndefined in the OpenSearch spec. Valid parameter values are:\n\n* `\"off\"`: Disable SafeSearch\n\n* `\"active\"`: Enable SafeSearch"]
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[doc = "The search terms entered by the user."]
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited\nto webpages."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs from a specified site."]
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[doc = "Specifies whether to include or exclude results from the site named in\nthe `sitesearch` parameter. Supported values are:\n\n* `i`: include content from site\n\n* `e`: exclude content from site"]
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[doc = "Specifies that results should be sorted according to the specified\nexpression. For example, sort by date."]
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[doc = "The index of the current set of search results into the total set of\nresults, where the index of the first result is 1."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The page number of this set of results, where the page length is set by\nthe `count` property."]
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[doc = "A description of the query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Estimated number of total search results. May not be accurate."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueriesRequestItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueriesRequestItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchSearchInformation {
        #[doc = "The time taken for the server to return search results, formatted\naccording to locale style."]
        #[serde(
            rename = "formattedSearchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_search_time: ::std::option::Option<String>,
        #[doc = "The total number of search results, formatted according to locale style."]
        #[serde(
            rename = "formattedTotalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_total_results: ::std::option::Option<String>,
        #[doc = "The time taken for the server to return search results."]
        #[serde(
            rename = "searchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_time: ::std::option::Option<f64>,
        #[doc = "The total number of search results returned by the query."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_results: ::std::option::Option<String>,
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
        #[doc = "The corrected query."]
        #[serde(
            rename = "correctedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_query: ::std::option::Option<String>,
        #[doc = "The corrected query, formatted in HTML."]
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
        #[doc = "The MIME type of the OpenSearch URL template for the Custom Search API."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The actual [OpenSearch\ntemplate](http://www.opensearch.org/specifications/opensearch/1.1#opensearch_url_template_syntax)\nfor this API."]
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
            pub enum ListImgColorType {
                Color,
                Gray,
                ImgColorTypeUndefined,
                Mono,
                Trans,
            }
            impl ListImgColorType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgColorType::Color => "color",
                        ListImgColorType::Gray => "gray",
                        ListImgColorType::ImgColorTypeUndefined => "imgColorTypeUndefined",
                        ListImgColorType::Mono => "mono",
                        ListImgColorType::Trans => "trans",
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
                        "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                        "mono" => ListImgColorType::Mono,
                        "trans" => ListImgColorType::Trans,
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
                        "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                        "mono" => ListImgColorType::Mono,
                        "trans" => ListImgColorType::Trans,
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
                Black,
                Blue,
                Brown,
                Gray,
                Green,
                ImgDominantColorUndefined,
                Orange,
                Pink,
                Purple,
                Red,
                Teal,
                White,
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
                        ListImgDominantColor::ImgDominantColorUndefined => {
                            "imgDominantColorUndefined"
                        }
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
                        "imgDominantColorUndefined" => {
                            ListImgDominantColor::ImgDominantColorUndefined
                        }
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
                        "imgDominantColorUndefined" => {
                            ListImgDominantColor::ImgDominantColorUndefined
                        }
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
                Huge,
                Icon,
                ImgSizeUndefined,
                Large,
                Medium,
                Small,
                Xlarge,
                Xxlarge,
            }
            impl ListImgSize {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgSize::Huge => "HUGE",
                        ListImgSize::Icon => "ICON",
                        ListImgSize::ImgSizeUndefined => "imgSizeUndefined",
                        ListImgSize::Large => "LARGE",
                        ListImgSize::Medium => "MEDIUM",
                        ListImgSize::Small => "SMALL",
                        ListImgSize::Xlarge => "XLARGE",
                        ListImgSize::Xxlarge => "XXLARGE",
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
                        "HUGE" => ListImgSize::Huge,
                        "ICON" => ListImgSize::Icon,
                        "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                        "LARGE" => ListImgSize::Large,
                        "MEDIUM" => ListImgSize::Medium,
                        "SMALL" => ListImgSize::Small,
                        "XLARGE" => ListImgSize::Xlarge,
                        "XXLARGE" => ListImgSize::Xxlarge,
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
                        "HUGE" => ListImgSize::Huge,
                        "ICON" => ListImgSize::Icon,
                        "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                        "LARGE" => ListImgSize::Large,
                        "MEDIUM" => ListImgSize::Medium,
                        "SMALL" => ListImgSize::Small,
                        "XLARGE" => ListImgSize::Xlarge,
                        "XXLARGE" => ListImgSize::Xxlarge,
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
                Animated,
                Clipart,
                Face,
                ImgTypeUndefined,
                Lineart,
                Photo,
                Stock,
            }
            impl ListImgType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgType::Animated => "animated",
                        ListImgType::Clipart => "clipart",
                        ListImgType::Face => "face",
                        ListImgType::ImgTypeUndefined => "imgTypeUndefined",
                        ListImgType::Lineart => "lineart",
                        ListImgType::Photo => "photo",
                        ListImgType::Stock => "stock",
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
                        "animated" => ListImgType::Animated,
                        "clipart" => ListImgType::Clipart,
                        "face" => ListImgType::Face,
                        "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                        "lineart" => ListImgType::Lineart,
                        "photo" => ListImgType::Photo,
                        "stock" => ListImgType::Stock,
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
                        "animated" => ListImgType::Animated,
                        "clipart" => ListImgType::Clipart,
                        "face" => ListImgType::Face,
                        "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                        "lineart" => ListImgType::Lineart,
                        "photo" => ListImgType::Photo,
                        "stock" => ListImgType::Stock,
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
            pub enum ListSafe {
                Active,
                High,
                Medium,
                Off,
                SafeUndefined,
            }
            impl ListSafe {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSafe::Active => "active",
                        ListSafe::High => "high",
                        ListSafe::Medium => "medium",
                        ListSafe::Off => "off",
                        ListSafe::SafeUndefined => "safeUndefined",
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
                        "safeUndefined" => ListSafe::SafeUndefined,
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
                        "safeUndefined" => ListSafe::SafeUndefined,
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
                Image,
                SearchTypeUndefined,
            }
            impl ListSearchType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSearchType::Image => "image",
                        ListSearchType::SearchTypeUndefined => "searchTypeUndefined",
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
                        "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
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
                        "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
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
                E,
                I,
                SiteSearchFilterUndefined,
            }
            impl ListSiteSearchFilter {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSiteSearchFilter::E => "e",
                        ListSiteSearchFilter::I => "i",
                        ListSiteSearchFilter::SiteSearchFilterUndefined => {
                            "siteSearchFilterUndefined"
                        }
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
                        "siteSearchFilterUndefined" => {
                            ListSiteSearchFilter::SiteSearchFilterUndefined
                        }
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
                        "siteSearchFilterUndefined" => {
                            ListSiteSearchFilter::SiteSearchFilterUndefined
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CseActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns metadata about the search performed, metadata about the custom\nsearch engine used for the search, and the search results."]
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
                    q: None,
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
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            c_2coff: Option<String>,
            cr: Option<String>,
            cx: Option<String>,
            date_restrict: Option<String>,
            exact_terms: Option<String>,
            exclude_terms: Option<String>,
            file_type: Option<String>,
            filter: Option<String>,
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
            lr: Option<String>,
            num: Option<i32>,
            or_terms: Option<String>,
            q: Option<String>,
            related_site: Option<String>,
            rights: Option<String>,
            safe: Option<crate::resources::cse::params::ListSafe>,
            search_type: Option<crate::resources::cse::params::ListSearchType>,
            site_search: Option<String>,
            site_search_filter: Option<crate::resources::cse::params::ListSiteSearchFilter>,
            sort: Option<String>,
            start: Option<u32>,
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
            #[doc = "Enables or disables [Simplified and Traditional Chinese\nSearch](https://developers.google.com/custom-search/docs/xml_results#chineseSearch).\n\nThe default value for this parameter is 0 (zero), meaning that the feature\nis enabled. Supported values are:\n\n* `1`: Disabled\n\n* `0`: Enabled (default)"]
            pub fn c_2coff(mut self, value: impl Into<String>) -> Self {
                self.c_2coff = Some(value.into());
                self
            }
            #[doc = "Restricts search results to documents originating in a particular country.\nYou may use [Boolean\noperators](https://developers.google.com/custom-search/docs/xml_results_appendices#booleanOperators)\nin the cr parameter's value.\n\nGoogle Search determines the country of a document by analyzing:\n\n* the top-level domain (TLD) of the document's URL\n\n* the geographic location of the Web server's IP address\n\nSee the [Country Parameter\nValues](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCollections)\npage for a list of valid values for this parameter."]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = "The custom search engine ID to use for this request."]
            pub fn cx(mut self, value: impl Into<String>) -> Self {
                self.cx = Some(value.into());
                self
            }
            #[doc = "Restricts results to URLs based on date. Supported values include:\n\n* `d[number]`: requests results from the specified number of past days.\n\n* `w[number]`: requests results from the specified number of past weeks.\n\n* `m[number]`: requests results from the specified number of past months.\n\n* `y[number]`: requests results from the specified number of past years."]
            pub fn date_restrict(mut self, value: impl Into<String>) -> Self {
                self.date_restrict = Some(value.into());
                self
            }
            #[doc = "Identifies a phrase that all documents in the search results must contain."]
            pub fn exact_terms(mut self, value: impl Into<String>) -> Self {
                self.exact_terms = Some(value.into());
                self
            }
            #[doc = "Identifies a word or phrase that should not appear in any documents in the\nsearch results."]
            pub fn exclude_terms(mut self, value: impl Into<String>) -> Self {
                self.exclude_terms = Some(value.into());
                self
            }
            #[doc = "Restricts results to files of a specified extension. A list of file types\nindexable by Google can be found in Search Console [Help\nCenter](https://support.google.com/webmasters/answer/35287)."]
            pub fn file_type(mut self, value: impl Into<String>) -> Self {
                self.file_type = Some(value.into());
                self
            }
            #[doc = "Controls turning on or off the duplicate content filter.\n\n* See [Automatic\n  Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)\n  for more information about Google's search results filters. Note that host\n  crowding filtering applies only to multi-site searches.\n\n* By default, Google applies filtering to all search results to improve the\n  quality of those results.\n\nAcceptable values are:\n\n* `0`: Turns off duplicate content filter.\n\n* `1`: Turns on duplicate content filter."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Geolocation of end user.\n\n* The `gl` parameter value is a two-letter country code. The `gl` parameter\n  boosts search results whose country of origin matches the parameter value.\n  See the [Country\n  Codes](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCodes)\n  page for a list of valid values.\n\n* Specifying a `gl` parameter value should lead to more relevant results.\n  This is particularly true for international customers and, even more\n  specifically, for customers in English- speaking countries other than the\n  United States."]
            pub fn gl(mut self, value: impl Into<String>) -> Self {
                self.gl = Some(value.into());
                self
            }
            #[doc = "**Deprecated**. Use the `gl` parameter for a similar effect.\n\nThe local Google domain (for example, google.com, google.de, or\ngoogle.fr) to use to perform the search."]
            pub fn googlehost(mut self, value: impl Into<String>) -> Self {
                self.googlehost = Some(value.into());
                self
            }
            #[doc = "Specifies the ending value for a search range.\n\n* Use `lowRange` and `highRange` to append an inclusive search range of\n  `lowRange...highRange` to the query."]
            pub fn high_range(mut self, value: impl Into<String>) -> Self {
                self.high_range = Some(value.into());
                self
            }
            #[doc = "Sets the user interface language.\n\n* Explicitly setting this parameter improves the performance and the\n  quality of your search results.\n\n* See the [Interface\n  Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)\n  section of [Internationalizing Queries and Results\n  Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)\n  for more information, and (Supported Interface\n  Languages)[https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages]\n  for a list of supported languages."]
            pub fn hl(mut self, value: impl Into<String>) -> Self {
                self.hl = Some(value.into());
                self
            }
            #[doc = "Appends the specified query terms to the query, as if they were combined\nwith a logical AND operator."]
            pub fn hq(mut self, value: impl Into<String>) -> Self {
                self.hq = Some(value.into());
                self
            }
            #[doc = "Returns black and white, grayscale, transparent, or color images.\nAcceptable values are:\n\n* `\"color\"`\n\n* `\"gray\"`\n\n* `\"mono\"`: black and white\n\n* `\"trans\"`: transparent background"]
            pub fn img_color_type(
                mut self,
                value: crate::resources::cse::params::ListImgColorType,
            ) -> Self {
                self.img_color_type = Some(value);
                self
            }
            #[doc = "Returns images of a specific dominant color. Acceptable values are:\n\n* `\"black\"`\n\n* `\"blue\"`\n\n* `\"brown\"`\n\n* `\"gray\"`\n\n* `\"green\"`\n\n* `\"orange\"`\n\n* `\"pink\"`\n\n* `\"purple\"`\n\n* `\"red\"`\n\n* `\"teal\"`\n\n* `\"white\"`\n\n* `\"yellow\"`"]
            pub fn img_dominant_color(
                mut self,
                value: crate::resources::cse::params::ListImgDominantColor,
            ) -> Self {
                self.img_dominant_color = Some(value);
                self
            }
            #[doc = "Returns images of a specified size. Acceptable values are:\n\n* `\"huge\"`\n\n* `\"icon\"`\n\n* `\"large\"`\n\n* `\"medium\"`\n\n* `\"small\"`\n\n* `\"xlarge\"`\n\n* `\"xxlarge\"`"]
            pub fn img_size(mut self, value: crate::resources::cse::params::ListImgSize) -> Self {
                self.img_size = Some(value);
                self
            }
            #[doc = "Returns images of a type. Acceptable values are:\n\n* `\"clipart\"`\n\n* `\"face\"`\n\n* `\"lineart\"`\n\n* `\"stock\"`\n\n* `\"photo\"`\n\n* `\"animated\"`"]
            pub fn img_type(mut self, value: crate::resources::cse::params::ListImgType) -> Self {
                self.img_type = Some(value);
                self
            }
            #[doc = "Specifies that all search results should contain a link to a particular\nURL."]
            pub fn link_site(mut self, value: impl Into<String>) -> Self {
                self.link_site = Some(value.into());
                self
            }
            #[doc = "Specifies the starting value for a search range. Use `lowRange` and\n`highRange` to append an inclusive search range of `lowRange...highRange`\nto the query."]
            pub fn low_range(mut self, value: impl Into<String>) -> Self {
                self.low_range = Some(value.into());
                self
            }
            #[doc = "Restricts the search to documents written in a particular language (e.g.,\n`lr=lang_ja`).\n\nAcceptable values are:\n\n* `\"lang_ar\"`: Arabic\n\n* `\"lang_bg\"`: Bulgarian\n\n* `\"lang_ca\"`: Catalan\n\n* `\"lang_cs\"`: Czech\n\n* `\"lang_da\"`: Danish\n\n* `\"lang_de\"`: German\n\n* `\"lang_el\"`: Greek\n\n* `\"lang_en\"`: English\n\n* `\"lang_es\"`: Spanish\n\n* `\"lang_et\"`: Estonian\n\n* `\"lang_fi\"`: Finnish\n\n* `\"lang_fr\"`: French\n\n* `\"lang_hr\"`: Croatian\n\n* `\"lang_hu\"`: Hungarian\n\n* `\"lang_id\"`: Indonesian\n\n* `\"lang_is\"`: Icelandic\n\n* `\"lang_it\"`: Italian\n\n* `\"lang_iw\"`: Hebrew\n\n* `\"lang_ja\"`: Japanese\n\n* `\"lang_ko\"`: Korean\n\n* `\"lang_lt\"`: Lithuanian\n\n* `\"lang_lv\"`: Latvian\n\n* `\"lang_nl\"`: Dutch\n\n* `\"lang_no\"`: Norwegian\n\n* `\"lang_pl\"`: Polish\n\n* `\"lang_pt\"`: Portuguese\n\n* `\"lang_ro\"`: Romanian\n\n* `\"lang_ru\"`: Russian\n\n* `\"lang_sk\"`: Slovak\n\n* `\"lang_sl\"`: Slovenian\n\n* `\"lang_sr\"`: Serbian\n\n* `\"lang_sv\"`: Swedish\n\n* `\"lang_tr\"`: Turkish\n\n* `\"lang_zh-CN\"`: Chinese (Simplified)\n\n* `\"lang_zh-TW\"`: Chinese (Traditional)"]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = "Number of search results to return.\n\n* Valid values are integers between 1 and 10, inclusive."]
            pub fn num(mut self, value: i32) -> Self {
                self.num = Some(value);
                self
            }
            #[doc = "Provides additional search terms to check for in a document, where each\ndocument in the search results must contain at least one of the additional\nsearch terms."]
            pub fn or_terms(mut self, value: impl Into<String>) -> Self {
                self.or_terms = Some(value.into());
                self
            }
            #[doc = "Query"]
            pub fn q(mut self, value: impl Into<String>) -> Self {
                self.q = Some(value.into());
                self
            }
            #[doc = "Specifies that all search results should be pages that are related to the\nspecified URL."]
            pub fn related_site(mut self, value: impl Into<String>) -> Self {
                self.related_site = Some(value.into());
                self
            }
            #[doc = "Filters based on licensing. Supported values include: `cc_publicdomain`,\n`cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and\ncombinations of these. See [typical\ncombinations](https://wiki.creativecommons.org/wiki/CC_Search_integration)."]
            pub fn rights(mut self, value: impl Into<String>) -> Self {
                self.rights = Some(value.into());
                self
            }
            #[doc = "Search safety level. Acceptable values are:\n\n* `\"active\"`: Enables SafeSearch filtering.\n\n* `\"off\"`: Disables SafeSearch filtering. (default)"]
            pub fn safe(mut self, value: crate::resources::cse::params::ListSafe) -> Self {
                self.safe = Some(value);
                self
            }
            #[doc = "Specifies the search type: `image`. If unspecified, results are limited to\nwebpages.\n\nAcceptable values are:\n\n* `\"image\"`: custom image search."]
            pub fn search_type(
                mut self,
                value: crate::resources::cse::params::ListSearchType,
            ) -> Self {
                self.search_type = Some(value);
                self
            }
            #[doc = "Specifies a given site which should always be included or excluded from\nresults (see `siteSearchFilter` parameter, below)."]
            pub fn site_search(mut self, value: impl Into<String>) -> Self {
                self.site_search = Some(value.into());
                self
            }
            #[doc = "Controls whether to include or exclude results from the site named in the\n`siteSearch` parameter.\n\nAcceptable values are:\n\n* `\"e\"`: exclude\n\n* `\"i\"`: include"]
            pub fn site_search_filter(
                mut self,
                value: crate::resources::cse::params::ListSiteSearchFilter,
            ) -> Self {
                self.site_search_filter = Some(value);
                self
            }
            #[doc = "The sort expression to apply to the results."]
            pub fn sort(mut self, value: impl Into<String>) -> Self {
                self.sort = Some(value.into());
                self
            }
            #[doc = "The index of the first result to return. The default number of results per\npage is 10, so `&start=11` would start at the top of the second page of\nresults. **Note**: The JSON API will never return more than 100 results,\neven if more than 100 documents match the query, so setting the sum of\n`start + num` to a number greater than 100 will produce an error. Also note\nthat the maximum value for `num` is 10."]
            pub fn start(mut self, value: u32) -> Self {
                self.start = Some(value);
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://customsearch.googleapis.com/".to_owned();
                output.push_str("customsearch/v1");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                let req = req.query(&[("q", &self.q)]);
                let req = req.query(&[("relatedSite", &self.related_site)]);
                let req = req.query(&[("rights", &self.rights)]);
                let req = req.query(&[("safe", &self.safe)]);
                let req = req.query(&[("searchType", &self.search_type)]);
                let req = req.query(&[("siteSearch", &self.site_search)]);
                let req = req.query(&[("siteSearchFilter", &self.site_search_filter)]);
                let req = req.query(&[("sort", &self.sort)]);
                let req = req.query(&[("start", &self.start)]);
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
        pub mod siterestrict {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgColorType {
                    Color,
                    Gray,
                    ImgColorTypeUndefined,
                    Mono,
                    Trans,
                }
                impl ListImgColorType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgColorType::Color => "color",
                            ListImgColorType::Gray => "gray",
                            ListImgColorType::ImgColorTypeUndefined => "imgColorTypeUndefined",
                            ListImgColorType::Mono => "mono",
                            ListImgColorType::Trans => "trans",
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
                            "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                            "mono" => ListImgColorType::Mono,
                            "trans" => ListImgColorType::Trans,
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
                            "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                            "mono" => ListImgColorType::Mono,
                            "trans" => ListImgColorType::Trans,
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
                    Black,
                    Blue,
                    Brown,
                    Gray,
                    Green,
                    ImgDominantColorUndefined,
                    Orange,
                    Pink,
                    Purple,
                    Red,
                    Teal,
                    White,
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
                            ListImgDominantColor::ImgDominantColorUndefined => {
                                "imgDominantColorUndefined"
                            }
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
                            "imgDominantColorUndefined" => {
                                ListImgDominantColor::ImgDominantColorUndefined
                            }
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
                            "imgDominantColorUndefined" => {
                                ListImgDominantColor::ImgDominantColorUndefined
                            }
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
                    Huge,
                    Icon,
                    ImgSizeUndefined,
                    Large,
                    Medium,
                    Small,
                    Xlarge,
                    Xxlarge,
                }
                impl ListImgSize {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgSize::Huge => "HUGE",
                            ListImgSize::Icon => "ICON",
                            ListImgSize::ImgSizeUndefined => "imgSizeUndefined",
                            ListImgSize::Large => "LARGE",
                            ListImgSize::Medium => "MEDIUM",
                            ListImgSize::Small => "SMALL",
                            ListImgSize::Xlarge => "XLARGE",
                            ListImgSize::Xxlarge => "XXLARGE",
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
                            "HUGE" => ListImgSize::Huge,
                            "ICON" => ListImgSize::Icon,
                            "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                            "LARGE" => ListImgSize::Large,
                            "MEDIUM" => ListImgSize::Medium,
                            "SMALL" => ListImgSize::Small,
                            "XLARGE" => ListImgSize::Xlarge,
                            "XXLARGE" => ListImgSize::Xxlarge,
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
                            "HUGE" => ListImgSize::Huge,
                            "ICON" => ListImgSize::Icon,
                            "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                            "LARGE" => ListImgSize::Large,
                            "MEDIUM" => ListImgSize::Medium,
                            "SMALL" => ListImgSize::Small,
                            "XLARGE" => ListImgSize::Xlarge,
                            "XXLARGE" => ListImgSize::Xxlarge,
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
                    Animated,
                    Clipart,
                    Face,
                    ImgTypeUndefined,
                    Lineart,
                    Photo,
                    Stock,
                }
                impl ListImgType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgType::Animated => "animated",
                            ListImgType::Clipart => "clipart",
                            ListImgType::Face => "face",
                            ListImgType::ImgTypeUndefined => "imgTypeUndefined",
                            ListImgType::Lineart => "lineart",
                            ListImgType::Photo => "photo",
                            ListImgType::Stock => "stock",
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
                            "animated" => ListImgType::Animated,
                            "clipart" => ListImgType::Clipart,
                            "face" => ListImgType::Face,
                            "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                            "lineart" => ListImgType::Lineart,
                            "photo" => ListImgType::Photo,
                            "stock" => ListImgType::Stock,
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
                            "animated" => ListImgType::Animated,
                            "clipart" => ListImgType::Clipart,
                            "face" => ListImgType::Face,
                            "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                            "lineart" => ListImgType::Lineart,
                            "photo" => ListImgType::Photo,
                            "stock" => ListImgType::Stock,
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
                pub enum ListSafe {
                    Active,
                    High,
                    Medium,
                    Off,
                    SafeUndefined,
                }
                impl ListSafe {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSafe::Active => "active",
                            ListSafe::High => "high",
                            ListSafe::Medium => "medium",
                            ListSafe::Off => "off",
                            ListSafe::SafeUndefined => "safeUndefined",
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
                            "safeUndefined" => ListSafe::SafeUndefined,
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
                            "safeUndefined" => ListSafe::SafeUndefined,
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
                    Image,
                    SearchTypeUndefined,
                }
                impl ListSearchType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSearchType::Image => "image",
                            ListSearchType::SearchTypeUndefined => "searchTypeUndefined",
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
                            "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
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
                            "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
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
                    E,
                    I,
                    SiteSearchFilterUndefined,
                }
                impl ListSiteSearchFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSiteSearchFilter::E => "e",
                            ListSiteSearchFilter::I => "i",
                            ListSiteSearchFilter::SiteSearchFilterUndefined => {
                                "siteSearchFilterUndefined"
                            }
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
                            "siteSearchFilterUndefined" => {
                                ListSiteSearchFilter::SiteSearchFilterUndefined
                            }
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
                            "siteSearchFilterUndefined" => {
                                ListSiteSearchFilter::SiteSearchFilterUndefined
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
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SiterestrictActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns metadata about the search performed, metadata about the custom\nsearch engine used for the search, and the search results. Uses a small set\nof url patterns."]
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
                        q: None,
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
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                c_2coff: Option<String>,
                cr: Option<String>,
                cx: Option<String>,
                date_restrict: Option<String>,
                exact_terms: Option<String>,
                exclude_terms: Option<String>,
                file_type: Option<String>,
                filter: Option<String>,
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
                lr: Option<String>,
                num: Option<i32>,
                or_terms: Option<String>,
                q: Option<String>,
                related_site: Option<String>,
                rights: Option<String>,
                safe: Option<crate::resources::cse::siterestrict::params::ListSafe>,
                search_type: Option<crate::resources::cse::siterestrict::params::ListSearchType>,
                site_search: Option<String>,
                site_search_filter:
                    Option<crate::resources::cse::siterestrict::params::ListSiteSearchFilter>,
                sort: Option<String>,
                start: Option<u32>,
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
                #[doc = "Enables or disables [Simplified and Traditional Chinese\nSearch](https://developers.google.com/custom-search/docs/xml_results#chineseSearch).\n\nThe default value for this parameter is 0 (zero), meaning that the feature\nis enabled. Supported values are:\n\n* `1`: Disabled\n\n* `0`: Enabled (default)"]
                pub fn c_2coff(mut self, value: impl Into<String>) -> Self {
                    self.c_2coff = Some(value.into());
                    self
                }
                #[doc = "Restricts search results to documents originating in a particular country.\nYou may use [Boolean\noperators](https://developers.google.com/custom-search/docs/xml_results_appendices#booleanOperators)\nin the cr parameter's value.\n\nGoogle Search determines the country of a document by analyzing:\n\n* the top-level domain (TLD) of the document's URL\n\n* the geographic location of the Web server's IP address\n\nSee the [Country Parameter\nValues](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCollections)\npage for a list of valid values for this parameter."]
                pub fn cr(mut self, value: impl Into<String>) -> Self {
                    self.cr = Some(value.into());
                    self
                }
                #[doc = "The custom search engine ID to use for this request."]
                pub fn cx(mut self, value: impl Into<String>) -> Self {
                    self.cx = Some(value.into());
                    self
                }
                #[doc = "Restricts results to URLs based on date. Supported values include:\n\n* `d[number]`: requests results from the specified number of past days.\n\n* `w[number]`: requests results from the specified number of past weeks.\n\n* `m[number]`: requests results from the specified number of past months.\n\n* `y[number]`: requests results from the specified number of past years."]
                pub fn date_restrict(mut self, value: impl Into<String>) -> Self {
                    self.date_restrict = Some(value.into());
                    self
                }
                #[doc = "Identifies a phrase that all documents in the search results must contain."]
                pub fn exact_terms(mut self, value: impl Into<String>) -> Self {
                    self.exact_terms = Some(value.into());
                    self
                }
                #[doc = "Identifies a word or phrase that should not appear in any documents in the\nsearch results."]
                pub fn exclude_terms(mut self, value: impl Into<String>) -> Self {
                    self.exclude_terms = Some(value.into());
                    self
                }
                #[doc = "Restricts results to files of a specified extension. A list of file types\nindexable by Google can be found in Search Console [Help\nCenter](https://support.google.com/webmasters/answer/35287)."]
                pub fn file_type(mut self, value: impl Into<String>) -> Self {
                    self.file_type = Some(value.into());
                    self
                }
                #[doc = "Controls turning on or off the duplicate content filter.\n\n* See [Automatic\n  Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)\n  for more information about Google's search results filters. Note that host\n  crowding filtering applies only to multi-site searches.\n\n* By default, Google applies filtering to all search results to improve the\n  quality of those results.\n\nAcceptable values are:\n\n* `0`: Turns off duplicate content filter.\n\n* `1`: Turns on duplicate content filter."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Geolocation of end user.\n\n* The `gl` parameter value is a two-letter country code. The `gl` parameter\n  boosts search results whose country of origin matches the parameter value.\n  See the [Country\n  Codes](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCodes)\n  page for a list of valid values.\n\n* Specifying a `gl` parameter value should lead to more relevant results.\n  This is particularly true for international customers and, even more\n  specifically, for customers in English- speaking countries other than the\n  United States."]
                pub fn gl(mut self, value: impl Into<String>) -> Self {
                    self.gl = Some(value.into());
                    self
                }
                #[doc = "**Deprecated**. Use the `gl` parameter for a similar effect.\n\nThe local Google domain (for example, google.com, google.de, or\ngoogle.fr) to use to perform the search."]
                pub fn googlehost(mut self, value: impl Into<String>) -> Self {
                    self.googlehost = Some(value.into());
                    self
                }
                #[doc = "Specifies the ending value for a search range.\n\n* Use `lowRange` and `highRange` to append an inclusive search range of\n  `lowRange...highRange` to the query."]
                pub fn high_range(mut self, value: impl Into<String>) -> Self {
                    self.high_range = Some(value.into());
                    self
                }
                #[doc = "Sets the user interface language.\n\n* Explicitly setting this parameter improves the performance and the\n  quality of your search results.\n\n* See the [Interface\n  Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)\n  section of [Internationalizing Queries and Results\n  Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)\n  for more information, and (Supported Interface\n  Languages)[https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages]\n  for a list of supported languages."]
                pub fn hl(mut self, value: impl Into<String>) -> Self {
                    self.hl = Some(value.into());
                    self
                }
                #[doc = "Appends the specified query terms to the query, as if they were combined\nwith a logical AND operator."]
                pub fn hq(mut self, value: impl Into<String>) -> Self {
                    self.hq = Some(value.into());
                    self
                }
                #[doc = "Returns black and white, grayscale, transparent, or color images.\nAcceptable values are:\n\n* `\"color\"`\n\n* `\"gray\"`\n\n* `\"mono\"`: black and white\n\n* `\"trans\"`: transparent background"]
                pub fn img_color_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgColorType,
                ) -> Self {
                    self.img_color_type = Some(value);
                    self
                }
                #[doc = "Returns images of a specific dominant color. Acceptable values are:\n\n* `\"black\"`\n\n* `\"blue\"`\n\n* `\"brown\"`\n\n* `\"gray\"`\n\n* `\"green\"`\n\n* `\"orange\"`\n\n* `\"pink\"`\n\n* `\"purple\"`\n\n* `\"red\"`\n\n* `\"teal\"`\n\n* `\"white\"`\n\n* `\"yellow\"`"]
                pub fn img_dominant_color(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgDominantColor,
                ) -> Self {
                    self.img_dominant_color = Some(value);
                    self
                }
                #[doc = "Returns images of a specified size. Acceptable values are:\n\n* `\"huge\"`\n\n* `\"icon\"`\n\n* `\"large\"`\n\n* `\"medium\"`\n\n* `\"small\"`\n\n* `\"xlarge\"`\n\n* `\"xxlarge\"`"]
                pub fn img_size(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgSize,
                ) -> Self {
                    self.img_size = Some(value);
                    self
                }
                #[doc = "Returns images of a type. Acceptable values are:\n\n* `\"clipart\"`\n\n* `\"face\"`\n\n* `\"lineart\"`\n\n* `\"stock\"`\n\n* `\"photo\"`\n\n* `\"animated\"`"]
                pub fn img_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgType,
                ) -> Self {
                    self.img_type = Some(value);
                    self
                }
                #[doc = "Specifies that all search results should contain a link to a particular\nURL."]
                pub fn link_site(mut self, value: impl Into<String>) -> Self {
                    self.link_site = Some(value.into());
                    self
                }
                #[doc = "Specifies the starting value for a search range. Use `lowRange` and\n`highRange` to append an inclusive search range of `lowRange...highRange`\nto the query."]
                pub fn low_range(mut self, value: impl Into<String>) -> Self {
                    self.low_range = Some(value.into());
                    self
                }
                #[doc = "Restricts the search to documents written in a particular language (e.g.,\n`lr=lang_ja`).\n\nAcceptable values are:\n\n* `\"lang_ar\"`: Arabic\n\n* `\"lang_bg\"`: Bulgarian\n\n* `\"lang_ca\"`: Catalan\n\n* `\"lang_cs\"`: Czech\n\n* `\"lang_da\"`: Danish\n\n* `\"lang_de\"`: German\n\n* `\"lang_el\"`: Greek\n\n* `\"lang_en\"`: English\n\n* `\"lang_es\"`: Spanish\n\n* `\"lang_et\"`: Estonian\n\n* `\"lang_fi\"`: Finnish\n\n* `\"lang_fr\"`: French\n\n* `\"lang_hr\"`: Croatian\n\n* `\"lang_hu\"`: Hungarian\n\n* `\"lang_id\"`: Indonesian\n\n* `\"lang_is\"`: Icelandic\n\n* `\"lang_it\"`: Italian\n\n* `\"lang_iw\"`: Hebrew\n\n* `\"lang_ja\"`: Japanese\n\n* `\"lang_ko\"`: Korean\n\n* `\"lang_lt\"`: Lithuanian\n\n* `\"lang_lv\"`: Latvian\n\n* `\"lang_nl\"`: Dutch\n\n* `\"lang_no\"`: Norwegian\n\n* `\"lang_pl\"`: Polish\n\n* `\"lang_pt\"`: Portuguese\n\n* `\"lang_ro\"`: Romanian\n\n* `\"lang_ru\"`: Russian\n\n* `\"lang_sk\"`: Slovak\n\n* `\"lang_sl\"`: Slovenian\n\n* `\"lang_sr\"`: Serbian\n\n* `\"lang_sv\"`: Swedish\n\n* `\"lang_tr\"`: Turkish\n\n* `\"lang_zh-CN\"`: Chinese (Simplified)\n\n* `\"lang_zh-TW\"`: Chinese (Traditional)"]
                pub fn lr(mut self, value: impl Into<String>) -> Self {
                    self.lr = Some(value.into());
                    self
                }
                #[doc = "Number of search results to return.\n\n* Valid values are integers between 1 and 10, inclusive."]
                pub fn num(mut self, value: i32) -> Self {
                    self.num = Some(value);
                    self
                }
                #[doc = "Provides additional search terms to check for in a document, where each\ndocument in the search results must contain at least one of the additional\nsearch terms."]
                pub fn or_terms(mut self, value: impl Into<String>) -> Self {
                    self.or_terms = Some(value.into());
                    self
                }
                #[doc = "Query"]
                pub fn q(mut self, value: impl Into<String>) -> Self {
                    self.q = Some(value.into());
                    self
                }
                #[doc = "Specifies that all search results should be pages that are related to the\nspecified URL."]
                pub fn related_site(mut self, value: impl Into<String>) -> Self {
                    self.related_site = Some(value.into());
                    self
                }
                #[doc = "Filters based on licensing. Supported values include: `cc_publicdomain`,\n`cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and\ncombinations of these. See [typical\ncombinations](https://wiki.creativecommons.org/wiki/CC_Search_integration)."]
                pub fn rights(mut self, value: impl Into<String>) -> Self {
                    self.rights = Some(value.into());
                    self
                }
                #[doc = "Search safety level. Acceptable values are:\n\n* `\"active\"`: Enables SafeSearch filtering.\n\n* `\"off\"`: Disables SafeSearch filtering. (default)"]
                pub fn safe(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSafe,
                ) -> Self {
                    self.safe = Some(value);
                    self
                }
                #[doc = "Specifies the search type: `image`. If unspecified, results are limited to\nwebpages.\n\nAcceptable values are:\n\n* `\"image\"`: custom image search."]
                pub fn search_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSearchType,
                ) -> Self {
                    self.search_type = Some(value);
                    self
                }
                #[doc = "Specifies a given site which should always be included or excluded from\nresults (see `siteSearchFilter` parameter, below)."]
                pub fn site_search(mut self, value: impl Into<String>) -> Self {
                    self.site_search = Some(value.into());
                    self
                }
                #[doc = "Controls whether to include or exclude results from the site named in the\n`siteSearch` parameter.\n\nAcceptable values are:\n\n* `\"e\"`: exclude\n\n* `\"i\"`: include"]
                pub fn site_search_filter(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSiteSearchFilter,
                ) -> Self {
                    self.site_search_filter = Some(value);
                    self
                }
                #[doc = "The sort expression to apply to the results."]
                pub fn sort(mut self, value: impl Into<String>) -> Self {
                    self.sort = Some(value.into());
                    self
                }
                #[doc = "The index of the first result to return. The default number of results per\npage is 10, so `&start=11` would start at the top of the second page of\nresults. **Note**: The JSON API will never return more than 100 results,\neven if more than 100 documents match the query, so setting the sum of\n`start + num` to a number greater than 100 will produce an error. Also note\nthat the maximum value for `num` is 10."]
                pub fn start(mut self, value: u32) -> Self {
                    self.start = Some(value);
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://customsearch.googleapis.com/".to_owned();
                    output.push_str("customsearch/v1/siterestrict");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    let req = req.query(&[("q", &self.q)]);
                    let req = req.query(&[("relatedSite", &self.related_site)]);
                    let req = req.query(&[("rights", &self.rights)]);
                    let req = req.query(&[("safe", &self.safe)]);
                    let req = req.query(&[("searchType", &self.search_type)]);
                    let req = req.query(&[("siteSearch", &self.site_search)]);
                    let req = req.query(&[("siteSearchFilter", &self.site_search_filter)]);
                    let req = req.query(&[("sort", &self.sort)]);
                    let req = req.query(&[("start", &self.start)]);
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
