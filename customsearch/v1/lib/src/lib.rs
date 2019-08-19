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
    pub struct Context {
        #[serde(rename = "facets", default)]
        pub facets: Option<Vec<Vec<crate::schemas::ContextFacetsItemsItems>>>,
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Context {
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
    pub struct ContextFacetsItemsItems {
        #[serde(rename = "anchor", default)]
        pub anchor: Option<String>,
        #[serde(rename = "label", default)]
        pub label: Option<String>,
        #[serde(rename = "label_with_op", default)]
        pub label_with_op: Option<String>,
    }
    impl ::field_selector::FieldSelector for ContextFacetsItemsItems {
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
    pub struct Promotion {
        #[serde(rename = "bodyLines", default)]
        pub body_lines: Option<Vec<crate::schemas::PromotionBodyLinesItems>>,
        #[serde(rename = "displayLink", default)]
        pub display_link: Option<String>,
        #[serde(rename = "htmlTitle", default)]
        pub html_title: Option<String>,
        #[serde(rename = "image", default)]
        pub image: Option<crate::schemas::PromotionImage>,
        #[serde(rename = "link", default)]
        pub link: Option<String>,
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Promotion {
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
    pub struct PromotionBodyLinesItems {
        #[serde(rename = "htmlTitle", default)]
        pub html_title: Option<String>,
        #[serde(rename = "link", default)]
        pub link: Option<String>,
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for PromotionBodyLinesItems {
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
    pub struct PromotionImage {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "source", default)]
        pub source: Option<String>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PromotionImage {
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
    pub struct Query {
        #[serde(rename = "count", default)]
        pub count: Option<i32>,
        #[serde(rename = "cr", default)]
        pub cr: Option<String>,
        #[serde(rename = "cx", default)]
        pub cx: Option<String>,
        #[serde(rename = "dateRestrict", default)]
        pub date_restrict: Option<String>,
        #[serde(rename = "disableCnTwTranslation", default)]
        pub disable_cn_tw_translation: Option<String>,
        #[serde(rename = "exactTerms", default)]
        pub exact_terms: Option<String>,
        #[serde(rename = "excludeTerms", default)]
        pub exclude_terms: Option<String>,
        #[serde(rename = "fileType", default)]
        pub file_type: Option<String>,
        #[serde(rename = "filter", default)]
        pub filter: Option<String>,
        #[serde(rename = "gl", default)]
        pub gl: Option<String>,
        #[serde(rename = "googleHost", default)]
        pub google_host: Option<String>,
        #[serde(rename = "highRange", default)]
        pub high_range: Option<String>,
        #[serde(rename = "hl", default)]
        pub hl: Option<String>,
        #[serde(rename = "hq", default)]
        pub hq: Option<String>,
        #[serde(rename = "imgColorType", default)]
        pub img_color_type: Option<String>,
        #[serde(rename = "imgDominantColor", default)]
        pub img_dominant_color: Option<String>,
        #[serde(rename = "imgSize", default)]
        pub img_size: Option<String>,
        #[serde(rename = "imgType", default)]
        pub img_type: Option<String>,
        #[serde(rename = "inputEncoding", default)]
        pub input_encoding: Option<String>,
        #[serde(rename = "language", default)]
        pub language: Option<String>,
        #[serde(rename = "linkSite", default)]
        pub link_site: Option<String>,
        #[serde(rename = "lowRange", default)]
        pub low_range: Option<String>,
        #[serde(rename = "orTerms", default)]
        pub or_terms: Option<String>,
        #[serde(rename = "outputEncoding", default)]
        pub output_encoding: Option<String>,
        #[serde(rename = "relatedSite", default)]
        pub related_site: Option<String>,
        #[serde(rename = "rights", default)]
        pub rights: Option<String>,
        #[serde(rename = "safe", default)]
        pub safe: Option<String>,
        #[serde(rename = "searchTerms", default)]
        pub search_terms: Option<String>,
        #[serde(rename = "searchType", default)]
        pub search_type: Option<String>,
        #[serde(rename = "siteSearch", default)]
        pub site_search: Option<String>,
        #[serde(rename = "siteSearchFilter", default)]
        pub site_search_filter: Option<String>,
        #[serde(rename = "sort", default)]
        pub sort: Option<String>,
        #[serde(rename = "startIndex", default)]
        pub start_index: Option<i32>,
        #[serde(rename = "startPage", default)]
        pub start_page: Option<i32>,
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[serde(rename = "totalResults", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_results: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Query {
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
    pub struct Result {
        #[serde(rename = "cacheId", default)]
        pub cache_id: Option<String>,
        #[serde(rename = "displayLink", default)]
        pub display_link: Option<String>,
        #[serde(rename = "fileFormat", default)]
        pub file_format: Option<String>,
        #[serde(rename = "formattedUrl", default)]
        pub formatted_url: Option<String>,
        #[serde(rename = "htmlFormattedUrl", default)]
        pub html_formatted_url: Option<String>,
        #[serde(rename = "htmlSnippet", default)]
        pub html_snippet: Option<String>,
        #[serde(rename = "htmlTitle", default)]
        pub html_title: Option<String>,
        #[serde(rename = "image", default)]
        pub image: Option<crate::schemas::ResultImage>,
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[serde(rename = "labels", default)]
        pub labels: Option<Vec<crate::schemas::ResultLabelsItems>>,
        #[serde(rename = "link", default)]
        pub link: Option<String>,
        #[serde(rename = "mime", default)]
        pub mime: Option<String>,
        #[serde(rename = "pagemap", default)]
        pub pagemap: Option<
            ::std::collections::BTreeMap<
                String,
                Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
            >,
        >,
        #[serde(rename = "snippet", default)]
        pub snippet: Option<String>,
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Result {
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
    pub struct ResultImage {
        #[serde(rename = "byteSize", default)]
        pub byte_size: Option<i32>,
        #[serde(rename = "contextLink", default)]
        pub context_link: Option<String>,
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "thumbnailHeight", default)]
        pub thumbnail_height: Option<i32>,
        #[serde(rename = "thumbnailLink", default)]
        pub thumbnail_link: Option<String>,
        #[serde(rename = "thumbnailWidth", default)]
        pub thumbnail_width: Option<i32>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ResultImage {
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
    pub struct ResultLabelsItems {
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[serde(rename = "label_with_op", default)]
        pub label_with_op: Option<String>,
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ResultLabelsItems {
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
    pub struct Search {
        #[serde(rename = "context", default)]
        pub context: Option<crate::schemas::Context>,
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Result>>,
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[serde(rename = "promotions", default)]
        pub promotions: Option<Vec<crate::schemas::Promotion>>,
        #[serde(rename = "queries", default)]
        pub queries: Option<::std::collections::BTreeMap<String, Vec<crate::schemas::Query>>>,
        #[serde(rename = "searchInformation", default)]
        pub search_information: Option<crate::schemas::SearchSearchInformation>,
        #[serde(rename = "spelling", default)]
        pub spelling: Option<crate::schemas::SearchSpelling>,
        #[serde(rename = "url", default)]
        pub url: Option<crate::schemas::SearchUrl>,
    }
    impl ::field_selector::FieldSelector for Search {
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
    pub struct SearchSearchInformation {
        #[serde(rename = "formattedSearchTime", default)]
        pub formatted_search_time: Option<String>,
        #[serde(rename = "formattedTotalResults", default)]
        pub formatted_total_results: Option<String>,
        #[serde(rename = "searchTime", default)]
        pub search_time: Option<f64>,
        #[serde(rename = "totalResults", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_results: Option<i64>,
    }
    impl ::field_selector::FieldSelector for SearchSearchInformation {
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
    pub struct SearchSpelling {
        #[serde(rename = "correctedQuery", default)]
        pub corrected_query: Option<String>,
        #[serde(rename = "htmlCorrectedQuery", default)]
        pub html_corrected_query: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchSpelling {
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
    pub struct SearchUrl {
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[serde(rename = "template", default)]
        pub template: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchUrl {
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
    #[doc = "Actions that can be performed on the cse resource"]
    pub fn cse(&self) -> crate::cse::CseActions<A> {
        crate::cse::CseActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod cse {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListFilter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListFilter {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListFilter {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListImgColorType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListImgColorType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListImgColorType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListImgDominantColor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListImgDominantColor {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListImgDominantColor {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListImgSize {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListImgSize {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListImgSize {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListImgType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListImgType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListImgType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListLr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListLr {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListLr {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListSafe {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListSafe {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListSafe {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListSearchType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListSearchType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListSearchType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
        impl ::std::fmt::Display for ListSiteSearchFilter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListSiteSearchFilter {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListSiteSearchFilter {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    }
    pub struct CseActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> CseActions<'a, A> {
        #[doc = "Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results."]
        pub fn list(&self, q: impl Into<String>) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
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
        pub fn siterestrict(&self) -> siterestrict::SiterestrictActions<A> {
            siterestrict::SiterestrictActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        q: String,
        c_2coff: Option<String>,
        cr: Option<String>,
        cx: Option<String>,
        date_restrict: Option<String>,
        exact_terms: Option<String>,
        exclude_terms: Option<String>,
        file_type: Option<String>,
        filter: Option<crate::cse::params::ListFilter>,
        gl: Option<String>,
        googlehost: Option<String>,
        high_range: Option<String>,
        hl: Option<String>,
        hq: Option<String>,
        img_color_type: Option<crate::cse::params::ListImgColorType>,
        img_dominant_color: Option<crate::cse::params::ListImgDominantColor>,
        img_size: Option<crate::cse::params::ListImgSize>,
        img_type: Option<crate::cse::params::ListImgType>,
        link_site: Option<String>,
        low_range: Option<String>,
        lr: Option<crate::cse::params::ListLr>,
        num: Option<u32>,
        or_terms: Option<String>,
        related_site: Option<String>,
        rights: Option<String>,
        safe: Option<crate::cse::params::ListSafe>,
        search_type: Option<crate::cse::params::ListSearchType>,
        site_search: Option<String>,
        site_search_filter: Option<crate::cse::params::ListSiteSearchFilter>,
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
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Turns off the translation between zh-CN and zh-TW."]
        pub fn c_2coff(&mut self, value: impl Into<String>) -> &mut Self {
            self.c_2coff = Some(value.into());
            self
        }
        #[doc = "Country restrict(s)."]
        pub fn cr(&mut self, value: impl Into<String>) -> &mut Self {
            self.cr = Some(value.into());
            self
        }
        #[doc = "The custom search engine ID to scope this search query"]
        pub fn cx(&mut self, value: impl Into<String>) -> &mut Self {
            self.cx = Some(value.into());
            self
        }
        #[doc = "Specifies all search results are from a time period"]
        pub fn date_restrict(&mut self, value: impl Into<String>) -> &mut Self {
            self.date_restrict = Some(value.into());
            self
        }
        #[doc = "Identifies a phrase that all documents in the search results must contain"]
        pub fn exact_terms(&mut self, value: impl Into<String>) -> &mut Self {
            self.exact_terms = Some(value.into());
            self
        }
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results"]
        pub fn exclude_terms(&mut self, value: impl Into<String>) -> &mut Self {
            self.exclude_terms = Some(value.into());
            self
        }
        #[doc = "Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ..."]
        pub fn file_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.file_type = Some(value.into());
            self
        }
        #[doc = "Controls turning on or off the duplicate content filter."]
        pub fn filter(&mut self, value: crate::cse::params::ListFilter) -> &mut Self {
            self.filter = Some(value);
            self
        }
        #[doc = "Geolocation of end user."]
        pub fn gl(&mut self, value: impl Into<String>) -> &mut Self {
            self.gl = Some(value.into());
            self
        }
        #[doc = "The local Google domain to use to perform the search."]
        pub fn googlehost(&mut self, value: impl Into<String>) -> &mut Self {
            self.googlehost = Some(value.into());
            self
        }
        #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
        pub fn high_range(&mut self, value: impl Into<String>) -> &mut Self {
            self.high_range = Some(value.into());
            self
        }
        #[doc = "Sets the user interface language."]
        pub fn hl(&mut self, value: impl Into<String>) -> &mut Self {
            self.hl = Some(value.into());
            self
        }
        #[doc = "Appends the extra query terms to the query."]
        pub fn hq(&mut self, value: impl Into<String>) -> &mut Self {
            self.hq = Some(value.into());
            self
        }
        #[doc = "Returns black and white, grayscale, or color images: mono, gray, and color."]
        pub fn img_color_type(&mut self, value: crate::cse::params::ListImgColorType) -> &mut Self {
            self.img_color_type = Some(value);
            self
        }
        #[doc = "Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown."]
        pub fn img_dominant_color(
            &mut self,
            value: crate::cse::params::ListImgDominantColor,
        ) -> &mut Self {
            self.img_dominant_color = Some(value);
            self
        }
        #[doc = "Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge."]
        pub fn img_size(&mut self, value: crate::cse::params::ListImgSize) -> &mut Self {
            self.img_size = Some(value);
            self
        }
        #[doc = "Returns images of a type, which can be one of: clipart, face, lineart, news, and photo."]
        pub fn img_type(&mut self, value: crate::cse::params::ListImgType) -> &mut Self {
            self.img_type = Some(value);
            self
        }
        #[doc = "Specifies that all search results should contain a link to a particular URL"]
        pub fn link_site(&mut self, value: impl Into<String>) -> &mut Self {
            self.link_site = Some(value.into());
            self
        }
        #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
        pub fn low_range(&mut self, value: impl Into<String>) -> &mut Self {
            self.low_range = Some(value.into());
            self
        }
        #[doc = "The language restriction for the search results"]
        pub fn lr(&mut self, value: crate::cse::params::ListLr) -> &mut Self {
            self.lr = Some(value);
            self
        }
        #[doc = "Number of search results to return"]
        pub fn num(&mut self, value: u32) -> &mut Self {
            self.num = Some(value);
            self
        }
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms"]
        pub fn or_terms(&mut self, value: impl Into<String>) -> &mut Self {
            self.or_terms = Some(value.into());
            self
        }
        #[doc = "Specifies that all search results should be pages that are related to the specified URL"]
        pub fn related_site(&mut self, value: impl Into<String>) -> &mut Self {
            self.related_site = Some(value.into());
            self
        }
        #[doc = "Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these."]
        pub fn rights(&mut self, value: impl Into<String>) -> &mut Self {
            self.rights = Some(value.into());
            self
        }
        #[doc = "Search safety level"]
        pub fn safe(&mut self, value: crate::cse::params::ListSafe) -> &mut Self {
            self.safe = Some(value);
            self
        }
        #[doc = "Specifies the search type: image."]
        pub fn search_type(&mut self, value: crate::cse::params::ListSearchType) -> &mut Self {
            self.search_type = Some(value);
            self
        }
        #[doc = "Specifies all search results should be pages from a given site"]
        pub fn site_search(&mut self, value: impl Into<String>) -> &mut Self {
            self.site_search = Some(value.into());
            self
        }
        #[doc = "Controls whether to include or exclude results from the site named in the as_sitesearch parameter"]
        pub fn site_search_filter(
            &mut self,
            value: crate::cse::params::ListSiteSearchFilter,
        ) -> &mut Self {
            self.site_search_filter = Some(value);
            self
        }
        #[doc = "The sort expression to apply to the results"]
        pub fn sort(&mut self, value: impl Into<String>) -> &mut Self {
            self.sort = Some(value.into());
            self
        }
        #[doc = "The index of the first result to return"]
        pub fn start(&mut self, value: u32) -> &mut Self {
            self.start = Some(value);
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
        pub fn execute_debug(self) -> Result<crate::schemas::Search, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/customsearch/".to_owned();
            output.push_str("v1");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
            req
        }
    }
    pub mod siterestrict {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListFilter {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListFilter {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListImgColorType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgColorType {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgColorType {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListImgDominantColor {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgDominantColor {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgDominantColor {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListImgSize {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgSize {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgSize {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListImgType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgType {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgType {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListLr {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListLr {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListLr {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListSafe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSafe {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSafe {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListSearchType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSearchType {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSearchType {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
            impl ::std::fmt::Display for ListSiteSearchFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSiteSearchFilter {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSiteSearchFilter {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        }
        pub struct SiterestrictActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SiterestrictActions<'a, A> {
            #[doc = "Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results. Uses a small set of url patterns."]
            pub fn list(&self, q: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            q: String,
            c_2coff: Option<String>,
            cr: Option<String>,
            cx: Option<String>,
            date_restrict: Option<String>,
            exact_terms: Option<String>,
            exclude_terms: Option<String>,
            file_type: Option<String>,
            filter: Option<crate::siterestrict::params::ListFilter>,
            gl: Option<String>,
            googlehost: Option<String>,
            high_range: Option<String>,
            hl: Option<String>,
            hq: Option<String>,
            img_color_type: Option<crate::siterestrict::params::ListImgColorType>,
            img_dominant_color: Option<crate::siterestrict::params::ListImgDominantColor>,
            img_size: Option<crate::siterestrict::params::ListImgSize>,
            img_type: Option<crate::siterestrict::params::ListImgType>,
            link_site: Option<String>,
            low_range: Option<String>,
            lr: Option<crate::siterestrict::params::ListLr>,
            num: Option<u32>,
            or_terms: Option<String>,
            related_site: Option<String>,
            rights: Option<String>,
            safe: Option<crate::siterestrict::params::ListSafe>,
            search_type: Option<crate::siterestrict::params::ListSearchType>,
            site_search: Option<String>,
            site_search_filter: Option<crate::siterestrict::params::ListSiteSearchFilter>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Turns off the translation between zh-CN and zh-TW."]
            pub fn c_2coff(&mut self, value: impl Into<String>) -> &mut Self {
                self.c_2coff = Some(value.into());
                self
            }
            #[doc = "Country restrict(s)."]
            pub fn cr(&mut self, value: impl Into<String>) -> &mut Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = "The custom search engine ID to scope this search query"]
            pub fn cx(&mut self, value: impl Into<String>) -> &mut Self {
                self.cx = Some(value.into());
                self
            }
            #[doc = "Specifies all search results are from a time period"]
            pub fn date_restrict(&mut self, value: impl Into<String>) -> &mut Self {
                self.date_restrict = Some(value.into());
                self
            }
            #[doc = "Identifies a phrase that all documents in the search results must contain"]
            pub fn exact_terms(&mut self, value: impl Into<String>) -> &mut Self {
                self.exact_terms = Some(value.into());
                self
            }
            #[doc = "Identifies a word or phrase that should not appear in any documents in the search results"]
            pub fn exclude_terms(&mut self, value: impl Into<String>) -> &mut Self {
                self.exclude_terms = Some(value.into());
                self
            }
            #[doc = "Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ..."]
            pub fn file_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.file_type = Some(value.into());
                self
            }
            #[doc = "Controls turning on or off the duplicate content filter."]
            pub fn filter(&mut self, value: crate::siterestrict::params::ListFilter) -> &mut Self {
                self.filter = Some(value);
                self
            }
            #[doc = "Geolocation of end user."]
            pub fn gl(&mut self, value: impl Into<String>) -> &mut Self {
                self.gl = Some(value.into());
                self
            }
            #[doc = "The local Google domain to use to perform the search."]
            pub fn googlehost(&mut self, value: impl Into<String>) -> &mut Self {
                self.googlehost = Some(value.into());
                self
            }
            #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
            pub fn high_range(&mut self, value: impl Into<String>) -> &mut Self {
                self.high_range = Some(value.into());
                self
            }
            #[doc = "Sets the user interface language."]
            pub fn hl(&mut self, value: impl Into<String>) -> &mut Self {
                self.hl = Some(value.into());
                self
            }
            #[doc = "Appends the extra query terms to the query."]
            pub fn hq(&mut self, value: impl Into<String>) -> &mut Self {
                self.hq = Some(value.into());
                self
            }
            #[doc = "Returns black and white, grayscale, or color images: mono, gray, and color."]
            pub fn img_color_type(
                &mut self,
                value: crate::siterestrict::params::ListImgColorType,
            ) -> &mut Self {
                self.img_color_type = Some(value);
                self
            }
            #[doc = "Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown."]
            pub fn img_dominant_color(
                &mut self,
                value: crate::siterestrict::params::ListImgDominantColor,
            ) -> &mut Self {
                self.img_dominant_color = Some(value);
                self
            }
            #[doc = "Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge."]
            pub fn img_size(
                &mut self,
                value: crate::siterestrict::params::ListImgSize,
            ) -> &mut Self {
                self.img_size = Some(value);
                self
            }
            #[doc = "Returns images of a type, which can be one of: clipart, face, lineart, news, and photo."]
            pub fn img_type(
                &mut self,
                value: crate::siterestrict::params::ListImgType,
            ) -> &mut Self {
                self.img_type = Some(value);
                self
            }
            #[doc = "Specifies that all search results should contain a link to a particular URL"]
            pub fn link_site(&mut self, value: impl Into<String>) -> &mut Self {
                self.link_site = Some(value.into());
                self
            }
            #[doc = "Creates a range in form as_nlo value..as_nhi value and attempts to append it to query"]
            pub fn low_range(&mut self, value: impl Into<String>) -> &mut Self {
                self.low_range = Some(value.into());
                self
            }
            #[doc = "The language restriction for the search results"]
            pub fn lr(&mut self, value: crate::siterestrict::params::ListLr) -> &mut Self {
                self.lr = Some(value);
                self
            }
            #[doc = "Number of search results to return"]
            pub fn num(&mut self, value: u32) -> &mut Self {
                self.num = Some(value);
                self
            }
            #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms"]
            pub fn or_terms(&mut self, value: impl Into<String>) -> &mut Self {
                self.or_terms = Some(value.into());
                self
            }
            #[doc = "Specifies that all search results should be pages that are related to the specified URL"]
            pub fn related_site(&mut self, value: impl Into<String>) -> &mut Self {
                self.related_site = Some(value.into());
                self
            }
            #[doc = "Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these."]
            pub fn rights(&mut self, value: impl Into<String>) -> &mut Self {
                self.rights = Some(value.into());
                self
            }
            #[doc = "Search safety level"]
            pub fn safe(&mut self, value: crate::siterestrict::params::ListSafe) -> &mut Self {
                self.safe = Some(value);
                self
            }
            #[doc = "Specifies the search type: image."]
            pub fn search_type(
                &mut self,
                value: crate::siterestrict::params::ListSearchType,
            ) -> &mut Self {
                self.search_type = Some(value);
                self
            }
            #[doc = "Specifies all search results should be pages from a given site"]
            pub fn site_search(&mut self, value: impl Into<String>) -> &mut Self {
                self.site_search = Some(value.into());
                self
            }
            #[doc = "Controls whether to include or exclude results from the site named in the as_sitesearch parameter"]
            pub fn site_search_filter(
                &mut self,
                value: crate::siterestrict::params::ListSiteSearchFilter,
            ) -> &mut Self {
                self.site_search_filter = Some(value);
                self
            }
            #[doc = "The sort expression to apply to the results"]
            pub fn sort(&mut self, value: impl Into<String>) -> &mut Self {
                self.sort = Some(value.into());
                self
            }
            #[doc = "The index of the first result to return"]
            pub fn start(&mut self, value: u32) -> &mut Self {
                self.start = Some(value);
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
            ) -> Result<crate::schemas::Search, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/customsearch/".to_owned();
                output.push_str("v1/siterestrict");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                req
            }
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
