pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnalyzeEntitiesRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
    }
    impl AnalyzeEntitiesRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeEntitiesRequestEncodingType::None => "NONE",
                AnalyzeEntitiesRequestEncodingType::Utf8 => "UTF8",
                AnalyzeEntitiesRequestEncodingType::Utf16 => "UTF16",
                AnalyzeEntitiesRequestEncodingType::Utf32 => "UTF32",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeEntitiesRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeEntitiesRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeEntitiesRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeEntitiesRequestEncodingType::None,
                "UTF8" => AnalyzeEntitiesRequestEncodingType::Utf8,
                "UTF16" => AnalyzeEntitiesRequestEncodingType::Utf16,
                "UTF32" => AnalyzeEntitiesRequestEncodingType::Utf32,
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
    pub struct AnalyzeEntitiesRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: Option<crate::schemas::AnalyzeEntitiesRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitiesRequest {
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
    pub struct AnalyzeEntitiesResponse {
        #[doc = "The recognized entities in the input document."]
        #[serde(rename = "entities", default)]
        pub entities: Option<Vec<crate::schemas::Entity>>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitiesResponse {
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
    pub enum AnalyzeEntitySentimentRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
    }
    impl AnalyzeEntitySentimentRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeEntitySentimentRequestEncodingType::None => "NONE",
                AnalyzeEntitySentimentRequestEncodingType::Utf8 => "UTF8",
                AnalyzeEntitySentimentRequestEncodingType::Utf16 => "UTF16",
                AnalyzeEntitySentimentRequestEncodingType::Utf32 => "UTF32",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeEntitySentimentRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeEntitySentimentRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeEntitySentimentRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeEntitySentimentRequestEncodingType::None,
                "UTF8" => AnalyzeEntitySentimentRequestEncodingType::Utf8,
                "UTF16" => AnalyzeEntitySentimentRequestEncodingType::Utf16,
                "UTF32" => AnalyzeEntitySentimentRequestEncodingType::Utf32,
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
    pub struct AnalyzeEntitySentimentRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: Option<crate::schemas::AnalyzeEntitySentimentRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitySentimentRequest {
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
    pub struct AnalyzeEntitySentimentResponse {
        #[doc = "The recognized entities in the input document with associated sentiments."]
        #[serde(rename = "entities", default)]
        pub entities: Option<Vec<crate::schemas::Entity>>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitySentimentResponse {
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
    pub enum AnalyzeSentimentRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
    }
    impl AnalyzeSentimentRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeSentimentRequestEncodingType::None => "NONE",
                AnalyzeSentimentRequestEncodingType::Utf8 => "UTF8",
                AnalyzeSentimentRequestEncodingType::Utf16 => "UTF16",
                AnalyzeSentimentRequestEncodingType::Utf32 => "UTF32",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeSentimentRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeSentimentRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeSentimentRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeSentimentRequestEncodingType::None,
                "UTF8" => AnalyzeSentimentRequestEncodingType::Utf8,
                "UTF16" => AnalyzeSentimentRequestEncodingType::Utf16,
                "UTF32" => AnalyzeSentimentRequestEncodingType::Utf32,
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
    pub struct AnalyzeSentimentRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate sentence offsets for the\nsentence sentiment."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: Option<crate::schemas::AnalyzeSentimentRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSentimentRequest {
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
    pub struct AnalyzeSentimentResponse {
        #[doc = "The overall sentiment of the input document."]
        #[serde(rename = "documentSentiment", default)]
        pub document_sentiment: Option<crate::schemas::Sentiment>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
        #[doc = "The sentiment for all the sentences in the document."]
        #[serde(rename = "sentences", default)]
        pub sentences: Option<Vec<crate::schemas::Sentence>>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSentimentResponse {
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
    pub enum AnalyzeSyntaxRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
    }
    impl AnalyzeSyntaxRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeSyntaxRequestEncodingType::None => "NONE",
                AnalyzeSyntaxRequestEncodingType::Utf8 => "UTF8",
                AnalyzeSyntaxRequestEncodingType::Utf16 => "UTF16",
                AnalyzeSyntaxRequestEncodingType::Utf32 => "UTF32",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeSyntaxRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeSyntaxRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeSyntaxRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeSyntaxRequestEncodingType::None,
                "UTF8" => AnalyzeSyntaxRequestEncodingType::Utf8,
                "UTF16" => AnalyzeSyntaxRequestEncodingType::Utf16,
                "UTF32" => AnalyzeSyntaxRequestEncodingType::Utf32,
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
    pub struct AnalyzeSyntaxRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: Option<crate::schemas::AnalyzeSyntaxRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSyntaxRequest {
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
    pub struct AnalyzeSyntaxResponse {
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
        #[doc = "Sentences in the input document."]
        #[serde(rename = "sentences", default)]
        pub sentences: Option<Vec<crate::schemas::Sentence>>,
        #[doc = "Tokens, along with their syntactic information, in the input document."]
        #[serde(rename = "tokens", default)]
        pub tokens: Option<Vec<crate::schemas::Token>>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSyntaxResponse {
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
    pub enum AnnotateTextRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
    }
    impl AnnotateTextRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnnotateTextRequestEncodingType::None => "NONE",
                AnnotateTextRequestEncodingType::Utf8 => "UTF8",
                AnnotateTextRequestEncodingType::Utf16 => "UTF16",
                AnnotateTextRequestEncodingType::Utf32 => "UTF32",
            }
        }
    }
    impl ::std::fmt::Display for AnnotateTextRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnnotateTextRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnnotateTextRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnnotateTextRequestEncodingType::None,
                "UTF8" => AnnotateTextRequestEncodingType::Utf8,
                "UTF16" => AnnotateTextRequestEncodingType::Utf16,
                "UTF32" => AnnotateTextRequestEncodingType::Utf32,
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
    pub struct AnnotateTextRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: Option<crate::schemas::AnnotateTextRequestEncodingType>,
        #[doc = "Required. The enabled features."]
        #[serde(rename = "features", default)]
        pub features: Option<crate::schemas::Features>,
    }
    impl ::field_selector::FieldSelector for AnnotateTextRequest {
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
    pub struct AnnotateTextResponse {
        #[doc = "Categories identified in the input document."]
        #[serde(rename = "categories", default)]
        pub categories: Option<Vec<crate::schemas::ClassificationCategory>>,
        #[doc = "The overall sentiment for the document. Populated if the user enables\nAnnotateTextRequest.Features.extract_document_sentiment."]
        #[serde(rename = "documentSentiment", default)]
        pub document_sentiment: Option<crate::schemas::Sentiment>,
        #[doc = "Entities, along with their semantic information, in the input document.\nPopulated if the user enables\nAnnotateTextRequest.Features.extract_entities."]
        #[serde(rename = "entities", default)]
        pub entities: Option<Vec<crate::schemas::Entity>>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
        #[doc = "Sentences in the input document. Populated if the user enables\nAnnotateTextRequest.Features.extract_syntax."]
        #[serde(rename = "sentences", default)]
        pub sentences: Option<Vec<crate::schemas::Sentence>>,
        #[doc = "Tokens, along with their syntactic information, in the input document.\nPopulated if the user enables\nAnnotateTextRequest.Features.extract_syntax."]
        #[serde(rename = "tokens", default)]
        pub tokens: Option<Vec<crate::schemas::Token>>,
    }
    impl ::field_selector::FieldSelector for AnnotateTextResponse {
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
    pub struct ClassificationCategory {
        #[doc = "The classifier's confidence of the category. Number represents how certain\nthe classifier is that this category represents the given text."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "The name of the category representing the document, from the [predefined\ntaxonomy](/natural-language/docs/categories)."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ClassificationCategory {
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
    pub struct ClassifyTextRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: Option<crate::schemas::Document>,
    }
    impl ::field_selector::FieldSelector for ClassifyTextRequest {
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
    pub struct ClassifyTextResponse {
        #[doc = "Categories representing the input document."]
        #[serde(rename = "categories", default)]
        pub categories: Option<Vec<crate::schemas::ClassificationCategory>>,
    }
    impl ::field_selector::FieldSelector for ClassifyTextResponse {
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
    pub enum DependencyEdgeLabel {
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Abbreviation modifier"]
        Abbrev,
        #[doc = "Adjectival complement"]
        Acomp,
        #[doc = "Adverbial clause modifier"]
        Advcl,
        #[doc = "Adverbial modifier"]
        Advmod,
        #[doc = "Adjectival modifier of an NP"]
        Amod,
        #[doc = "Appositional modifier of an NP"]
        Appos,
        #[doc = "Attribute dependent of a copular verb"]
        Attr,
        #[doc = "Auxiliary (non-main) verb"]
        Aux,
        #[doc = "Passive auxiliary"]
        Auxpass,
        #[doc = "Coordinating conjunction"]
        Cc,
        #[doc = "Clausal complement of a verb or adjective"]
        Ccomp,
        #[doc = "Conjunct"]
        Conj,
        #[doc = "Clausal subject"]
        Csubj,
        #[doc = "Clausal passive subject"]
        Csubjpass,
        #[doc = "Dependency (unable to determine)"]
        Dep,
        #[doc = "Determiner"]
        Det,
        #[doc = "Discourse"]
        Discourse,
        #[doc = "Direct object"]
        Dobj,
        #[doc = "Expletive"]
        Expl,
        #[doc = "Goes with (part of a word in a text not well edited)"]
        Goeswith,
        #[doc = "Indirect object"]
        Iobj,
        #[doc = "Marker (word introducing a subordinate clause)"]
        Mark,
        #[doc = "Multi-word expression"]
        Mwe,
        #[doc = "Multi-word verbal expression"]
        Mwv,
        #[doc = "Negation modifier"]
        Neg,
        #[doc = "Noun compound modifier"]
        Nn,
        #[doc = "Noun phrase used as an adverbial modifier"]
        Npadvmod,
        #[doc = "Nominal subject"]
        Nsubj,
        #[doc = "Passive nominal subject"]
        Nsubjpass,
        #[doc = "Numeric modifier of a noun"]
        Num,
        #[doc = "Element of compound number"]
        Number,
        #[doc = "Punctuation mark"]
        P,
        #[doc = "Parataxis relation"]
        Parataxis,
        #[doc = "Participial modifier"]
        Partmod,
        #[doc = "The complement of a preposition is a clause"]
        Pcomp,
        #[doc = "Object of a preposition"]
        Pobj,
        #[doc = "Possession modifier"]
        Poss,
        #[doc = "Postverbal negative particle"]
        Postneg,
        #[doc = "Predicate complement"]
        Precomp,
        #[doc = "Preconjunt"]
        Preconj,
        #[doc = "Predeterminer"]
        Predet,
        #[doc = "Prefix"]
        Pref,
        #[doc = "Prepositional modifier"]
        Prep,
        #[doc = "The relationship between a verb and verbal morpheme"]
        Pronl,
        #[doc = "Particle"]
        Prt,
        #[doc = "Associative or possessive marker"]
        Ps,
        #[doc = "Quantifier phrase modifier"]
        Quantmod,
        #[doc = "Relative clause modifier"]
        Rcmod,
        #[doc = "Complementizer in relative clause"]
        Rcmodrel,
        #[doc = "Ellipsis without a preceding predicate"]
        Rdrop,
        #[doc = "Referent"]
        Ref,
        #[doc = "Remnant"]
        Remnant,
        #[doc = "Reparandum"]
        Reparandum,
        #[doc = "Root"]
        Root,
        #[doc = "Suffix specifying a unit of number"]
        Snum,
        #[doc = "Suffix"]
        Suff,
        #[doc = "Temporal modifier"]
        Tmod,
        #[doc = "Topic marker"]
        Topic,
        #[doc = "Clause headed by an infinite form of the verb that modifies a noun"]
        Vmod,
        #[doc = "Vocative"]
        Vocative,
        #[doc = "Open clausal complement"]
        Xcomp,
        #[doc = "Name suffix"]
        Suffix,
        #[doc = "Name title"]
        Title,
        #[doc = "Adverbial phrase modifier"]
        Advphmod,
        #[doc = "Causative auxiliary"]
        Auxcaus,
        #[doc = "Helper auxiliary"]
        Auxvv,
        #[doc = "Rentaishi (Prenominal modifier)"]
        Dtmod,
        #[doc = "Foreign words"]
        Foreign,
        #[doc = "Keyword"]
        Kw,
        #[doc = "List for chains of comparable items"]
        List,
        #[doc = "Nominalized clause"]
        Nomc,
        #[doc = "Nominalized clausal subject"]
        Nomcsubj,
        #[doc = "Nominalized clausal passive"]
        Nomcsubjpass,
        #[doc = "Compound of numeric modifier"]
        Numc,
        #[doc = "Copula"]
        Cop,
        #[doc = "Dislocated relation (for fronted/topicalized elements)"]
        Dislocated,
        #[doc = "Aspect marker"]
        Asp,
        #[doc = "Genitive modifier"]
        Gmod,
        #[doc = "Genitive object"]
        Gobj,
        #[doc = "Infinitival modifier"]
        Infmod,
        #[doc = "Measure"]
        Mes,
        #[doc = "Nominal complement of a noun"]
        Ncomp,
    }
    impl DependencyEdgeLabel {
        pub fn as_str(self) -> &'static str {
            match self {
                DependencyEdgeLabel::Unknown => "UNKNOWN",
                DependencyEdgeLabel::Abbrev => "ABBREV",
                DependencyEdgeLabel::Acomp => "ACOMP",
                DependencyEdgeLabel::Advcl => "ADVCL",
                DependencyEdgeLabel::Advmod => "ADVMOD",
                DependencyEdgeLabel::Amod => "AMOD",
                DependencyEdgeLabel::Appos => "APPOS",
                DependencyEdgeLabel::Attr => "ATTR",
                DependencyEdgeLabel::Aux => "AUX",
                DependencyEdgeLabel::Auxpass => "AUXPASS",
                DependencyEdgeLabel::Cc => "CC",
                DependencyEdgeLabel::Ccomp => "CCOMP",
                DependencyEdgeLabel::Conj => "CONJ",
                DependencyEdgeLabel::Csubj => "CSUBJ",
                DependencyEdgeLabel::Csubjpass => "CSUBJPASS",
                DependencyEdgeLabel::Dep => "DEP",
                DependencyEdgeLabel::Det => "DET",
                DependencyEdgeLabel::Discourse => "DISCOURSE",
                DependencyEdgeLabel::Dobj => "DOBJ",
                DependencyEdgeLabel::Expl => "EXPL",
                DependencyEdgeLabel::Goeswith => "GOESWITH",
                DependencyEdgeLabel::Iobj => "IOBJ",
                DependencyEdgeLabel::Mark => "MARK",
                DependencyEdgeLabel::Mwe => "MWE",
                DependencyEdgeLabel::Mwv => "MWV",
                DependencyEdgeLabel::Neg => "NEG",
                DependencyEdgeLabel::Nn => "NN",
                DependencyEdgeLabel::Npadvmod => "NPADVMOD",
                DependencyEdgeLabel::Nsubj => "NSUBJ",
                DependencyEdgeLabel::Nsubjpass => "NSUBJPASS",
                DependencyEdgeLabel::Num => "NUM",
                DependencyEdgeLabel::Number => "NUMBER",
                DependencyEdgeLabel::P => "P",
                DependencyEdgeLabel::Parataxis => "PARATAXIS",
                DependencyEdgeLabel::Partmod => "PARTMOD",
                DependencyEdgeLabel::Pcomp => "PCOMP",
                DependencyEdgeLabel::Pobj => "POBJ",
                DependencyEdgeLabel::Poss => "POSS",
                DependencyEdgeLabel::Postneg => "POSTNEG",
                DependencyEdgeLabel::Precomp => "PRECOMP",
                DependencyEdgeLabel::Preconj => "PRECONJ",
                DependencyEdgeLabel::Predet => "PREDET",
                DependencyEdgeLabel::Pref => "PREF",
                DependencyEdgeLabel::Prep => "PREP",
                DependencyEdgeLabel::Pronl => "PRONL",
                DependencyEdgeLabel::Prt => "PRT",
                DependencyEdgeLabel::Ps => "PS",
                DependencyEdgeLabel::Quantmod => "QUANTMOD",
                DependencyEdgeLabel::Rcmod => "RCMOD",
                DependencyEdgeLabel::Rcmodrel => "RCMODREL",
                DependencyEdgeLabel::Rdrop => "RDROP",
                DependencyEdgeLabel::Ref => "REF",
                DependencyEdgeLabel::Remnant => "REMNANT",
                DependencyEdgeLabel::Reparandum => "REPARANDUM",
                DependencyEdgeLabel::Root => "ROOT",
                DependencyEdgeLabel::Snum => "SNUM",
                DependencyEdgeLabel::Suff => "SUFF",
                DependencyEdgeLabel::Tmod => "TMOD",
                DependencyEdgeLabel::Topic => "TOPIC",
                DependencyEdgeLabel::Vmod => "VMOD",
                DependencyEdgeLabel::Vocative => "VOCATIVE",
                DependencyEdgeLabel::Xcomp => "XCOMP",
                DependencyEdgeLabel::Suffix => "SUFFIX",
                DependencyEdgeLabel::Title => "TITLE",
                DependencyEdgeLabel::Advphmod => "ADVPHMOD",
                DependencyEdgeLabel::Auxcaus => "AUXCAUS",
                DependencyEdgeLabel::Auxvv => "AUXVV",
                DependencyEdgeLabel::Dtmod => "DTMOD",
                DependencyEdgeLabel::Foreign => "FOREIGN",
                DependencyEdgeLabel::Kw => "KW",
                DependencyEdgeLabel::List => "LIST",
                DependencyEdgeLabel::Nomc => "NOMC",
                DependencyEdgeLabel::Nomcsubj => "NOMCSUBJ",
                DependencyEdgeLabel::Nomcsubjpass => "NOMCSUBJPASS",
                DependencyEdgeLabel::Numc => "NUMC",
                DependencyEdgeLabel::Cop => "COP",
                DependencyEdgeLabel::Dislocated => "DISLOCATED",
                DependencyEdgeLabel::Asp => "ASP",
                DependencyEdgeLabel::Gmod => "GMOD",
                DependencyEdgeLabel::Gobj => "GOBJ",
                DependencyEdgeLabel::Infmod => "INFMOD",
                DependencyEdgeLabel::Mes => "MES",
                DependencyEdgeLabel::Ncomp => "NCOMP",
            }
        }
    }
    impl ::std::fmt::Display for DependencyEdgeLabel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DependencyEdgeLabel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DependencyEdgeLabel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => DependencyEdgeLabel::Unknown,
                "ABBREV" => DependencyEdgeLabel::Abbrev,
                "ACOMP" => DependencyEdgeLabel::Acomp,
                "ADVCL" => DependencyEdgeLabel::Advcl,
                "ADVMOD" => DependencyEdgeLabel::Advmod,
                "AMOD" => DependencyEdgeLabel::Amod,
                "APPOS" => DependencyEdgeLabel::Appos,
                "ATTR" => DependencyEdgeLabel::Attr,
                "AUX" => DependencyEdgeLabel::Aux,
                "AUXPASS" => DependencyEdgeLabel::Auxpass,
                "CC" => DependencyEdgeLabel::Cc,
                "CCOMP" => DependencyEdgeLabel::Ccomp,
                "CONJ" => DependencyEdgeLabel::Conj,
                "CSUBJ" => DependencyEdgeLabel::Csubj,
                "CSUBJPASS" => DependencyEdgeLabel::Csubjpass,
                "DEP" => DependencyEdgeLabel::Dep,
                "DET" => DependencyEdgeLabel::Det,
                "DISCOURSE" => DependencyEdgeLabel::Discourse,
                "DOBJ" => DependencyEdgeLabel::Dobj,
                "EXPL" => DependencyEdgeLabel::Expl,
                "GOESWITH" => DependencyEdgeLabel::Goeswith,
                "IOBJ" => DependencyEdgeLabel::Iobj,
                "MARK" => DependencyEdgeLabel::Mark,
                "MWE" => DependencyEdgeLabel::Mwe,
                "MWV" => DependencyEdgeLabel::Mwv,
                "NEG" => DependencyEdgeLabel::Neg,
                "NN" => DependencyEdgeLabel::Nn,
                "NPADVMOD" => DependencyEdgeLabel::Npadvmod,
                "NSUBJ" => DependencyEdgeLabel::Nsubj,
                "NSUBJPASS" => DependencyEdgeLabel::Nsubjpass,
                "NUM" => DependencyEdgeLabel::Num,
                "NUMBER" => DependencyEdgeLabel::Number,
                "P" => DependencyEdgeLabel::P,
                "PARATAXIS" => DependencyEdgeLabel::Parataxis,
                "PARTMOD" => DependencyEdgeLabel::Partmod,
                "PCOMP" => DependencyEdgeLabel::Pcomp,
                "POBJ" => DependencyEdgeLabel::Pobj,
                "POSS" => DependencyEdgeLabel::Poss,
                "POSTNEG" => DependencyEdgeLabel::Postneg,
                "PRECOMP" => DependencyEdgeLabel::Precomp,
                "PRECONJ" => DependencyEdgeLabel::Preconj,
                "PREDET" => DependencyEdgeLabel::Predet,
                "PREF" => DependencyEdgeLabel::Pref,
                "PREP" => DependencyEdgeLabel::Prep,
                "PRONL" => DependencyEdgeLabel::Pronl,
                "PRT" => DependencyEdgeLabel::Prt,
                "PS" => DependencyEdgeLabel::Ps,
                "QUANTMOD" => DependencyEdgeLabel::Quantmod,
                "RCMOD" => DependencyEdgeLabel::Rcmod,
                "RCMODREL" => DependencyEdgeLabel::Rcmodrel,
                "RDROP" => DependencyEdgeLabel::Rdrop,
                "REF" => DependencyEdgeLabel::Ref,
                "REMNANT" => DependencyEdgeLabel::Remnant,
                "REPARANDUM" => DependencyEdgeLabel::Reparandum,
                "ROOT" => DependencyEdgeLabel::Root,
                "SNUM" => DependencyEdgeLabel::Snum,
                "SUFF" => DependencyEdgeLabel::Suff,
                "TMOD" => DependencyEdgeLabel::Tmod,
                "TOPIC" => DependencyEdgeLabel::Topic,
                "VMOD" => DependencyEdgeLabel::Vmod,
                "VOCATIVE" => DependencyEdgeLabel::Vocative,
                "XCOMP" => DependencyEdgeLabel::Xcomp,
                "SUFFIX" => DependencyEdgeLabel::Suffix,
                "TITLE" => DependencyEdgeLabel::Title,
                "ADVPHMOD" => DependencyEdgeLabel::Advphmod,
                "AUXCAUS" => DependencyEdgeLabel::Auxcaus,
                "AUXVV" => DependencyEdgeLabel::Auxvv,
                "DTMOD" => DependencyEdgeLabel::Dtmod,
                "FOREIGN" => DependencyEdgeLabel::Foreign,
                "KW" => DependencyEdgeLabel::Kw,
                "LIST" => DependencyEdgeLabel::List,
                "NOMC" => DependencyEdgeLabel::Nomc,
                "NOMCSUBJ" => DependencyEdgeLabel::Nomcsubj,
                "NOMCSUBJPASS" => DependencyEdgeLabel::Nomcsubjpass,
                "NUMC" => DependencyEdgeLabel::Numc,
                "COP" => DependencyEdgeLabel::Cop,
                "DISLOCATED" => DependencyEdgeLabel::Dislocated,
                "ASP" => DependencyEdgeLabel::Asp,
                "GMOD" => DependencyEdgeLabel::Gmod,
                "GOBJ" => DependencyEdgeLabel::Gobj,
                "INFMOD" => DependencyEdgeLabel::Infmod,
                "MES" => DependencyEdgeLabel::Mes,
                "NCOMP" => DependencyEdgeLabel::Ncomp,
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
    pub struct DependencyEdge {
        #[doc = "Represents the head of this token in the dependency tree.\nThis is the index of the token which has an arc going to this token.\nThe index is the position of the token in the array of tokens returned\nby the API method. If this token is a root token, then the\n`head_token_index` is its own index."]
        #[serde(rename = "headTokenIndex", default)]
        pub head_token_index: Option<i32>,
        #[doc = "The parse label for the token."]
        #[serde(rename = "label", default)]
        pub label: Option<crate::schemas::DependencyEdgeLabel>,
    }
    impl ::field_selector::FieldSelector for DependencyEdge {
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
    pub enum DocumentType {
        #[doc = "The content type is not specified."]
        TypeUnspecified,
        #[doc = "Plain text"]
        PlainText,
        #[doc = "HTML"]
        Html,
    }
    impl DocumentType {
        pub fn as_str(self) -> &'static str {
            match self {
                DocumentType::TypeUnspecified => "TYPE_UNSPECIFIED",
                DocumentType::PlainText => "PLAIN_TEXT",
                DocumentType::Html => "HTML",
            }
        }
    }
    impl ::std::fmt::Display for DocumentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DocumentType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DocumentType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => DocumentType::TypeUnspecified,
                "PLAIN_TEXT" => DocumentType::PlainText,
                "HTML" => DocumentType::Html,
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
    pub struct Document {
        #[doc = "The content of the input in string format.\nCloud audit logging exempt since it is based on user data."]
        #[serde(rename = "content", default)]
        pub content: Option<String>,
        #[doc = "The Google Cloud Storage URI where the file content is located.\nThis URI must be of the form: gs://bucket_name/object_name. For more\ndetails, see https://cloud.google.com/storage/docs/reference-uris.\nNOTE: Cloud Storage object versioning is not supported."]
        #[serde(rename = "gcsContentUri", default)]
        pub gcs_content_uri: Option<String>,
        #[doc = "The language of the document (if not specified, the language is\nautomatically detected). Both ISO and BCP-47 language codes are\naccepted.<br>\n[Language Support](/natural-language/docs/languages)\nlists currently supported languages for each API method.\nIf the language (either specified by the caller or automatically detected)\nis not supported by the called API method, an `INVALID_ARGUMENT` error\nis returned."]
        #[serde(rename = "language", default)]
        pub language: Option<String>,
        #[doc = "Required. If the type is not set or is `TYPE_UNSPECIFIED`,\nreturns an `INVALID_ARGUMENT` error."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::DocumentType>,
    }
    impl ::field_selector::FieldSelector for Document {
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
    pub enum EntityType {
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Person"]
        Person,
        #[doc = "Location"]
        Location,
        #[doc = "Organization"]
        Organization,
        #[doc = "Event"]
        Event,
        #[doc = "Artwork"]
        WorkOfArt,
        #[doc = "Consumer product"]
        ConsumerGood,
        #[doc = "Other types of entities"]
        Other,
        #[doc = "Phone number\n\nThe metadata lists the phone number, formatted according to local\nconvention, plus whichever additional elements appear in the text:\n\n* `number` - the actual number, broken down into sections as per local\n  convention\n* `national_prefix` - country code, if detected\n* `area_code` - region or area code, if detected\n* `extension` - phone extension (to be dialed after connection), if\n  detected"]
        PhoneNumber,
        #[doc = "Address\n\nThe metadata identifies the street number and locality plus whichever\nadditional elements appear in the text:\n\n* `street_number` - street number\n* `locality` - city or town\n* `street_name` - street/route name, if detected\n* `postal_code` - postal code, if detected\n* `country` - country, if detected<\n* `broad_region` - administrative area, such as the state, if detected\n* `narrow_region` - smaller administrative area, such as county, if\n  detected\n* `sublocality` - used in Asian addresses to demark a district within a\n  city, if detected"]
        Address,
        #[doc = "Date\n\nThe metadata identifies the components of the date:\n\n* `year` - four digit year, if detected\n* `month` - two digit month number, if detected\n* `day` - two digit day number, if detected"]
        Date,
        #[doc = "Number\n\nThe metadata is the number itself."]
        Number,
        #[doc = "Price\n\nThe metadata identifies the `value` and `currency`."]
        Price,
    }
    impl EntityType {
        pub fn as_str(self) -> &'static str {
            match self {
                EntityType::Unknown => "UNKNOWN",
                EntityType::Person => "PERSON",
                EntityType::Location => "LOCATION",
                EntityType::Organization => "ORGANIZATION",
                EntityType::Event => "EVENT",
                EntityType::WorkOfArt => "WORK_OF_ART",
                EntityType::ConsumerGood => "CONSUMER_GOOD",
                EntityType::Other => "OTHER",
                EntityType::PhoneNumber => "PHONE_NUMBER",
                EntityType::Address => "ADDRESS",
                EntityType::Date => "DATE",
                EntityType::Number => "NUMBER",
                EntityType::Price => "PRICE",
            }
        }
    }
    impl ::std::fmt::Display for EntityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EntityType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EntityType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => EntityType::Unknown,
                "PERSON" => EntityType::Person,
                "LOCATION" => EntityType::Location,
                "ORGANIZATION" => EntityType::Organization,
                "EVENT" => EntityType::Event,
                "WORK_OF_ART" => EntityType::WorkOfArt,
                "CONSUMER_GOOD" => EntityType::ConsumerGood,
                "OTHER" => EntityType::Other,
                "PHONE_NUMBER" => EntityType::PhoneNumber,
                "ADDRESS" => EntityType::Address,
                "DATE" => EntityType::Date,
                "NUMBER" => EntityType::Number,
                "PRICE" => EntityType::Price,
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
    pub struct Entity {
        #[doc = "The mentions of this entity in the input document. The API currently\nsupports proper noun mentions."]
        #[serde(rename = "mentions", default)]
        pub mentions: Option<Vec<crate::schemas::EntityMention>>,
        #[doc = "Metadata associated with the entity.\n\nFor most entity types, the metadata is a Wikipedia URL (`wikipedia_url`)\nand Knowledge Graph MID (`mid`), if they are available. For the metadata\nassociated with other entity types, see the Type table below."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The representative name for the entity."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The entity type."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::EntityType>,
        #[doc = "The salience score associated with the entity in the [0, 1.0] range.\n\nThe salience score for an entity provides information about the\nimportance or centrality of that entity to the entire document text.\nScores closer to 0 are less salient, while scores closer to 1.0 are highly\nsalient."]
        #[serde(rename = "salience", default)]
        pub salience: Option<f32>,
        #[doc = "For calls to AnalyzeEntitySentiment or if\nAnnotateTextRequest.Features.extract_entity_sentiment is set to\ntrue, this field will contain the aggregate sentiment expressed for this\nentity in the provided document."]
        #[serde(rename = "sentiment", default)]
        pub sentiment: Option<crate::schemas::Sentiment>,
    }
    impl ::field_selector::FieldSelector for Entity {
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
    pub enum EntityMentionType {
        #[doc = "Unknown"]
        TypeUnknown,
        #[doc = "Proper name"]
        Proper,
        #[doc = "Common noun (or noun compound)"]
        Common,
    }
    impl EntityMentionType {
        pub fn as_str(self) -> &'static str {
            match self {
                EntityMentionType::TypeUnknown => "TYPE_UNKNOWN",
                EntityMentionType::Proper => "PROPER",
                EntityMentionType::Common => "COMMON",
            }
        }
    }
    impl ::std::fmt::Display for EntityMentionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EntityMentionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EntityMentionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNKNOWN" => EntityMentionType::TypeUnknown,
                "PROPER" => EntityMentionType::Proper,
                "COMMON" => EntityMentionType::Common,
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
    pub struct EntityMention {
        #[doc = "The type of the entity mention."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::EntityMentionType>,
        #[doc = "For calls to AnalyzeEntitySentiment or if\nAnnotateTextRequest.Features.extract_entity_sentiment is set to\ntrue, this field will contain the sentiment expressed for this mention of\nthe entity in the provided document."]
        #[serde(rename = "sentiment", default)]
        pub sentiment: Option<crate::schemas::Sentiment>,
        #[doc = "The mention text."]
        #[serde(rename = "text", default)]
        pub text: Option<crate::schemas::TextSpan>,
    }
    impl ::field_selector::FieldSelector for EntityMention {
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
    pub struct Features {
        #[doc = "Classify the full document into categories. If this is true,\nthe API will use the default model which classifies into a\n[predefined taxonomy](/natural-language/docs/categories)."]
        #[serde(rename = "classifyText", default)]
        pub classify_text: Option<bool>,
        #[doc = "Extract document-level sentiment."]
        #[serde(rename = "extractDocumentSentiment", default)]
        pub extract_document_sentiment: Option<bool>,
        #[doc = "Extract entities."]
        #[serde(rename = "extractEntities", default)]
        pub extract_entities: Option<bool>,
        #[doc = "Extract entities and their associated sentiment."]
        #[serde(rename = "extractEntitySentiment", default)]
        pub extract_entity_sentiment: Option<bool>,
        #[doc = "Extract syntax information."]
        #[serde(rename = "extractSyntax", default)]
        pub extract_syntax: Option<bool>,
    }
    impl ::field_selector::FieldSelector for Features {
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
    pub enum PartOfSpeechAspect {
        #[doc = "Aspect is not applicable in the analyzed language or is not predicted."]
        AspectUnknown,
        #[doc = "Perfective"]
        Perfective,
        #[doc = "Imperfective"]
        Imperfective,
        #[doc = "Progressive"]
        Progressive,
    }
    impl PartOfSpeechAspect {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechAspect::AspectUnknown => "ASPECT_UNKNOWN",
                PartOfSpeechAspect::Perfective => "PERFECTIVE",
                PartOfSpeechAspect::Imperfective => "IMPERFECTIVE",
                PartOfSpeechAspect::Progressive => "PROGRESSIVE",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechAspect {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechAspect {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechAspect {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASPECT_UNKNOWN" => PartOfSpeechAspect::AspectUnknown,
                "PERFECTIVE" => PartOfSpeechAspect::Perfective,
                "IMPERFECTIVE" => PartOfSpeechAspect::Imperfective,
                "PROGRESSIVE" => PartOfSpeechAspect::Progressive,
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
    pub enum PartOfSpeechCase {
        #[doc = "Case is not applicable in the analyzed language or is not predicted."]
        CaseUnknown,
        #[doc = "Accusative"]
        Accusative,
        #[doc = "Adverbial"]
        Adverbial,
        #[doc = "Complementive"]
        Complementive,
        #[doc = "Dative"]
        Dative,
        #[doc = "Genitive"]
        Genitive,
        #[doc = "Instrumental"]
        Instrumental,
        #[doc = "Locative"]
        Locative,
        #[doc = "Nominative"]
        Nominative,
        #[doc = "Oblique"]
        Oblique,
        #[doc = "Partitive"]
        Partitive,
        #[doc = "Prepositional"]
        Prepositional,
        #[doc = "Reflexive"]
        ReflexiveCase,
        #[doc = "Relative"]
        RelativeCase,
        #[doc = "Vocative"]
        Vocative,
    }
    impl PartOfSpeechCase {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechCase::CaseUnknown => "CASE_UNKNOWN",
                PartOfSpeechCase::Accusative => "ACCUSATIVE",
                PartOfSpeechCase::Adverbial => "ADVERBIAL",
                PartOfSpeechCase::Complementive => "COMPLEMENTIVE",
                PartOfSpeechCase::Dative => "DATIVE",
                PartOfSpeechCase::Genitive => "GENITIVE",
                PartOfSpeechCase::Instrumental => "INSTRUMENTAL",
                PartOfSpeechCase::Locative => "LOCATIVE",
                PartOfSpeechCase::Nominative => "NOMINATIVE",
                PartOfSpeechCase::Oblique => "OBLIQUE",
                PartOfSpeechCase::Partitive => "PARTITIVE",
                PartOfSpeechCase::Prepositional => "PREPOSITIONAL",
                PartOfSpeechCase::ReflexiveCase => "REFLEXIVE_CASE",
                PartOfSpeechCase::RelativeCase => "RELATIVE_CASE",
                PartOfSpeechCase::Vocative => "VOCATIVE",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechCase {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechCase {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechCase {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CASE_UNKNOWN" => PartOfSpeechCase::CaseUnknown,
                "ACCUSATIVE" => PartOfSpeechCase::Accusative,
                "ADVERBIAL" => PartOfSpeechCase::Adverbial,
                "COMPLEMENTIVE" => PartOfSpeechCase::Complementive,
                "DATIVE" => PartOfSpeechCase::Dative,
                "GENITIVE" => PartOfSpeechCase::Genitive,
                "INSTRUMENTAL" => PartOfSpeechCase::Instrumental,
                "LOCATIVE" => PartOfSpeechCase::Locative,
                "NOMINATIVE" => PartOfSpeechCase::Nominative,
                "OBLIQUE" => PartOfSpeechCase::Oblique,
                "PARTITIVE" => PartOfSpeechCase::Partitive,
                "PREPOSITIONAL" => PartOfSpeechCase::Prepositional,
                "REFLEXIVE_CASE" => PartOfSpeechCase::ReflexiveCase,
                "RELATIVE_CASE" => PartOfSpeechCase::RelativeCase,
                "VOCATIVE" => PartOfSpeechCase::Vocative,
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
    pub enum PartOfSpeechForm {
        #[doc = "Form is not applicable in the analyzed language or is not predicted."]
        FormUnknown,
        #[doc = "Adnomial"]
        Adnomial,
        #[doc = "Auxiliary"]
        Auxiliary,
        #[doc = "Complementizer"]
        Complementizer,
        #[doc = "Final ending"]
        FinalEnding,
        #[doc = "Gerund"]
        Gerund,
        #[doc = "Realis"]
        Realis,
        #[doc = "Irrealis"]
        Irrealis,
        #[doc = "Short form"]
        Short,
        #[doc = "Long form"]
        Long,
        #[doc = "Order form"]
        Order,
        #[doc = "Specific form"]
        Specific,
    }
    impl PartOfSpeechForm {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechForm::FormUnknown => "FORM_UNKNOWN",
                PartOfSpeechForm::Adnomial => "ADNOMIAL",
                PartOfSpeechForm::Auxiliary => "AUXILIARY",
                PartOfSpeechForm::Complementizer => "COMPLEMENTIZER",
                PartOfSpeechForm::FinalEnding => "FINAL_ENDING",
                PartOfSpeechForm::Gerund => "GERUND",
                PartOfSpeechForm::Realis => "REALIS",
                PartOfSpeechForm::Irrealis => "IRREALIS",
                PartOfSpeechForm::Short => "SHORT",
                PartOfSpeechForm::Long => "LONG",
                PartOfSpeechForm::Order => "ORDER",
                PartOfSpeechForm::Specific => "SPECIFIC",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechForm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechForm {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechForm {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORM_UNKNOWN" => PartOfSpeechForm::FormUnknown,
                "ADNOMIAL" => PartOfSpeechForm::Adnomial,
                "AUXILIARY" => PartOfSpeechForm::Auxiliary,
                "COMPLEMENTIZER" => PartOfSpeechForm::Complementizer,
                "FINAL_ENDING" => PartOfSpeechForm::FinalEnding,
                "GERUND" => PartOfSpeechForm::Gerund,
                "REALIS" => PartOfSpeechForm::Realis,
                "IRREALIS" => PartOfSpeechForm::Irrealis,
                "SHORT" => PartOfSpeechForm::Short,
                "LONG" => PartOfSpeechForm::Long,
                "ORDER" => PartOfSpeechForm::Order,
                "SPECIFIC" => PartOfSpeechForm::Specific,
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
    pub enum PartOfSpeechGender {
        #[doc = "Gender is not applicable in the analyzed language or is not predicted."]
        GenderUnknown,
        #[doc = "Feminine"]
        Feminine,
        #[doc = "Masculine"]
        Masculine,
        #[doc = "Neuter"]
        Neuter,
    }
    impl PartOfSpeechGender {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechGender::GenderUnknown => "GENDER_UNKNOWN",
                PartOfSpeechGender::Feminine => "FEMININE",
                PartOfSpeechGender::Masculine => "MASCULINE",
                PartOfSpeechGender::Neuter => "NEUTER",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechGender {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechGender {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechGender {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GENDER_UNKNOWN" => PartOfSpeechGender::GenderUnknown,
                "FEMININE" => PartOfSpeechGender::Feminine,
                "MASCULINE" => PartOfSpeechGender::Masculine,
                "NEUTER" => PartOfSpeechGender::Neuter,
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
    pub enum PartOfSpeechMood {
        #[doc = "Mood is not applicable in the analyzed language or is not predicted."]
        MoodUnknown,
        #[doc = "Conditional"]
        ConditionalMood,
        #[doc = "Imperative"]
        Imperative,
        #[doc = "Indicative"]
        Indicative,
        #[doc = "Interrogative"]
        Interrogative,
        #[doc = "Jussive"]
        Jussive,
        #[doc = "Subjunctive"]
        Subjunctive,
    }
    impl PartOfSpeechMood {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechMood::MoodUnknown => "MOOD_UNKNOWN",
                PartOfSpeechMood::ConditionalMood => "CONDITIONAL_MOOD",
                PartOfSpeechMood::Imperative => "IMPERATIVE",
                PartOfSpeechMood::Indicative => "INDICATIVE",
                PartOfSpeechMood::Interrogative => "INTERROGATIVE",
                PartOfSpeechMood::Jussive => "JUSSIVE",
                PartOfSpeechMood::Subjunctive => "SUBJUNCTIVE",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechMood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechMood {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechMood {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MOOD_UNKNOWN" => PartOfSpeechMood::MoodUnknown,
                "CONDITIONAL_MOOD" => PartOfSpeechMood::ConditionalMood,
                "IMPERATIVE" => PartOfSpeechMood::Imperative,
                "INDICATIVE" => PartOfSpeechMood::Indicative,
                "INTERROGATIVE" => PartOfSpeechMood::Interrogative,
                "JUSSIVE" => PartOfSpeechMood::Jussive,
                "SUBJUNCTIVE" => PartOfSpeechMood::Subjunctive,
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
    pub enum PartOfSpeechNumber {
        #[doc = "Number is not applicable in the analyzed language or is not predicted."]
        NumberUnknown,
        #[doc = "Singular"]
        Singular,
        #[doc = "Plural"]
        Plural,
        #[doc = "Dual"]
        Dual,
    }
    impl PartOfSpeechNumber {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechNumber::NumberUnknown => "NUMBER_UNKNOWN",
                PartOfSpeechNumber::Singular => "SINGULAR",
                PartOfSpeechNumber::Plural => "PLURAL",
                PartOfSpeechNumber::Dual => "DUAL",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechNumber {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechNumber {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NUMBER_UNKNOWN" => PartOfSpeechNumber::NumberUnknown,
                "SINGULAR" => PartOfSpeechNumber::Singular,
                "PLURAL" => PartOfSpeechNumber::Plural,
                "DUAL" => PartOfSpeechNumber::Dual,
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
    pub enum PartOfSpeechPerson {
        #[doc = "Person is not applicable in the analyzed language or is not predicted."]
        PersonUnknown,
        #[doc = "First"]
        First,
        #[doc = "Second"]
        Second,
        #[doc = "Third"]
        Third,
        #[doc = "Reflexive"]
        ReflexivePerson,
    }
    impl PartOfSpeechPerson {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechPerson::PersonUnknown => "PERSON_UNKNOWN",
                PartOfSpeechPerson::First => "FIRST",
                PartOfSpeechPerson::Second => "SECOND",
                PartOfSpeechPerson::Third => "THIRD",
                PartOfSpeechPerson::ReflexivePerson => "REFLEXIVE_PERSON",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechPerson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechPerson {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechPerson {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERSON_UNKNOWN" => PartOfSpeechPerson::PersonUnknown,
                "FIRST" => PartOfSpeechPerson::First,
                "SECOND" => PartOfSpeechPerson::Second,
                "THIRD" => PartOfSpeechPerson::Third,
                "REFLEXIVE_PERSON" => PartOfSpeechPerson::ReflexivePerson,
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
    pub enum PartOfSpeechProper {
        #[doc = "Proper is not applicable in the analyzed language or is not predicted."]
        ProperUnknown,
        #[doc = "Proper"]
        Proper,
        #[doc = "Not proper"]
        NotProper,
    }
    impl PartOfSpeechProper {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechProper::ProperUnknown => "PROPER_UNKNOWN",
                PartOfSpeechProper::Proper => "PROPER",
                PartOfSpeechProper::NotProper => "NOT_PROPER",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechProper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechProper {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechProper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PROPER_UNKNOWN" => PartOfSpeechProper::ProperUnknown,
                "PROPER" => PartOfSpeechProper::Proper,
                "NOT_PROPER" => PartOfSpeechProper::NotProper,
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
    pub enum PartOfSpeechReciprocity {
        #[doc = "Reciprocity is not applicable in the analyzed language or is not\npredicted."]
        ReciprocityUnknown,
        #[doc = "Reciprocal"]
        Reciprocal,
        #[doc = "Non-reciprocal"]
        NonReciprocal,
    }
    impl PartOfSpeechReciprocity {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechReciprocity::ReciprocityUnknown => "RECIPROCITY_UNKNOWN",
                PartOfSpeechReciprocity::Reciprocal => "RECIPROCAL",
                PartOfSpeechReciprocity::NonReciprocal => "NON_RECIPROCAL",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechReciprocity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechReciprocity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechReciprocity {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RECIPROCITY_UNKNOWN" => PartOfSpeechReciprocity::ReciprocityUnknown,
                "RECIPROCAL" => PartOfSpeechReciprocity::Reciprocal,
                "NON_RECIPROCAL" => PartOfSpeechReciprocity::NonReciprocal,
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
    pub enum PartOfSpeechTag {
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Adjective"]
        Adj,
        #[doc = "Adposition (preposition and postposition)"]
        Adp,
        #[doc = "Adverb"]
        Adv,
        #[doc = "Conjunction"]
        Conj,
        #[doc = "Determiner"]
        Det,
        #[doc = "Noun (common and proper)"]
        Noun,
        #[doc = "Cardinal number"]
        Num,
        #[doc = "Pronoun"]
        Pron,
        #[doc = "Particle or other function word"]
        Prt,
        #[doc = "Punctuation"]
        Punct,
        #[doc = "Verb (all tenses and modes)"]
        Verb,
        #[doc = "Other: foreign words, typos, abbreviations"]
        X,
        #[doc = "Affix"]
        Affix,
    }
    impl PartOfSpeechTag {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechTag::Unknown => "UNKNOWN",
                PartOfSpeechTag::Adj => "ADJ",
                PartOfSpeechTag::Adp => "ADP",
                PartOfSpeechTag::Adv => "ADV",
                PartOfSpeechTag::Conj => "CONJ",
                PartOfSpeechTag::Det => "DET",
                PartOfSpeechTag::Noun => "NOUN",
                PartOfSpeechTag::Num => "NUM",
                PartOfSpeechTag::Pron => "PRON",
                PartOfSpeechTag::Prt => "PRT",
                PartOfSpeechTag::Punct => "PUNCT",
                PartOfSpeechTag::Verb => "VERB",
                PartOfSpeechTag::X => "X",
                PartOfSpeechTag::Affix => "AFFIX",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechTag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechTag {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechTag {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => PartOfSpeechTag::Unknown,
                "ADJ" => PartOfSpeechTag::Adj,
                "ADP" => PartOfSpeechTag::Adp,
                "ADV" => PartOfSpeechTag::Adv,
                "CONJ" => PartOfSpeechTag::Conj,
                "DET" => PartOfSpeechTag::Det,
                "NOUN" => PartOfSpeechTag::Noun,
                "NUM" => PartOfSpeechTag::Num,
                "PRON" => PartOfSpeechTag::Pron,
                "PRT" => PartOfSpeechTag::Prt,
                "PUNCT" => PartOfSpeechTag::Punct,
                "VERB" => PartOfSpeechTag::Verb,
                "X" => PartOfSpeechTag::X,
                "AFFIX" => PartOfSpeechTag::Affix,
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
    pub enum PartOfSpeechTense {
        #[doc = "Tense is not applicable in the analyzed language or is not predicted."]
        TenseUnknown,
        #[doc = "Conditional"]
        ConditionalTense,
        #[doc = "Future"]
        Future,
        #[doc = "Past"]
        Past,
        #[doc = "Present"]
        Present,
        #[doc = "Imperfect"]
        Imperfect,
        #[doc = "Pluperfect"]
        Pluperfect,
    }
    impl PartOfSpeechTense {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechTense::TenseUnknown => "TENSE_UNKNOWN",
                PartOfSpeechTense::ConditionalTense => "CONDITIONAL_TENSE",
                PartOfSpeechTense::Future => "FUTURE",
                PartOfSpeechTense::Past => "PAST",
                PartOfSpeechTense::Present => "PRESENT",
                PartOfSpeechTense::Imperfect => "IMPERFECT",
                PartOfSpeechTense::Pluperfect => "PLUPERFECT",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechTense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechTense {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechTense {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TENSE_UNKNOWN" => PartOfSpeechTense::TenseUnknown,
                "CONDITIONAL_TENSE" => PartOfSpeechTense::ConditionalTense,
                "FUTURE" => PartOfSpeechTense::Future,
                "PAST" => PartOfSpeechTense::Past,
                "PRESENT" => PartOfSpeechTense::Present,
                "IMPERFECT" => PartOfSpeechTense::Imperfect,
                "PLUPERFECT" => PartOfSpeechTense::Pluperfect,
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
    pub enum PartOfSpeechVoice {
        #[doc = "Voice is not applicable in the analyzed language or is not predicted."]
        VoiceUnknown,
        #[doc = "Active"]
        Active,
        #[doc = "Causative"]
        Causative,
        #[doc = "Passive"]
        Passive,
    }
    impl PartOfSpeechVoice {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechVoice::VoiceUnknown => "VOICE_UNKNOWN",
                PartOfSpeechVoice::Active => "ACTIVE",
                PartOfSpeechVoice::Causative => "CAUSATIVE",
                PartOfSpeechVoice::Passive => "PASSIVE",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechVoice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechVoice {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechVoice {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "VOICE_UNKNOWN" => PartOfSpeechVoice::VoiceUnknown,
                "ACTIVE" => PartOfSpeechVoice::Active,
                "CAUSATIVE" => PartOfSpeechVoice::Causative,
                "PASSIVE" => PartOfSpeechVoice::Passive,
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
    pub struct PartOfSpeech {
        #[doc = "The grammatical aspect."]
        #[serde(rename = "aspect", default)]
        pub aspect: Option<crate::schemas::PartOfSpeechAspect>,
        #[doc = "The grammatical case."]
        #[serde(rename = "case", default)]
        pub case: Option<crate::schemas::PartOfSpeechCase>,
        #[doc = "The grammatical form."]
        #[serde(rename = "form", default)]
        pub form: Option<crate::schemas::PartOfSpeechForm>,
        #[doc = "The grammatical gender."]
        #[serde(rename = "gender", default)]
        pub gender: Option<crate::schemas::PartOfSpeechGender>,
        #[doc = "The grammatical mood."]
        #[serde(rename = "mood", default)]
        pub mood: Option<crate::schemas::PartOfSpeechMood>,
        #[doc = "The grammatical number."]
        #[serde(rename = "number", default)]
        pub number: Option<crate::schemas::PartOfSpeechNumber>,
        #[doc = "The grammatical person."]
        #[serde(rename = "person", default)]
        pub person: Option<crate::schemas::PartOfSpeechPerson>,
        #[doc = "The grammatical properness."]
        #[serde(rename = "proper", default)]
        pub proper: Option<crate::schemas::PartOfSpeechProper>,
        #[doc = "The grammatical reciprocity."]
        #[serde(rename = "reciprocity", default)]
        pub reciprocity: Option<crate::schemas::PartOfSpeechReciprocity>,
        #[doc = "The part of speech tag."]
        #[serde(rename = "tag", default)]
        pub tag: Option<crate::schemas::PartOfSpeechTag>,
        #[doc = "The grammatical tense."]
        #[serde(rename = "tense", default)]
        pub tense: Option<crate::schemas::PartOfSpeechTense>,
        #[doc = "The grammatical voice."]
        #[serde(rename = "voice", default)]
        pub voice: Option<crate::schemas::PartOfSpeechVoice>,
    }
    impl ::field_selector::FieldSelector for PartOfSpeech {
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
    pub struct Sentence {
        #[doc = "For calls to AnalyzeSentiment or if\nAnnotateTextRequest.Features.extract_document_sentiment is set to\ntrue, this field will contain the sentiment for the sentence."]
        #[serde(rename = "sentiment", default)]
        pub sentiment: Option<crate::schemas::Sentiment>,
        #[doc = "The sentence text."]
        #[serde(rename = "text", default)]
        pub text: Option<crate::schemas::TextSpan>,
    }
    impl ::field_selector::FieldSelector for Sentence {
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
    pub struct Sentiment {
        #[doc = "A non-negative number in the [0, +inf) range, which represents\nthe absolute magnitude of sentiment regardless of score (positive or\nnegative)."]
        #[serde(rename = "magnitude", default)]
        pub magnitude: Option<f32>,
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0\n(positive sentiment)."]
        #[serde(rename = "score", default)]
        pub score: Option<f32>,
    }
    impl ::field_selector::FieldSelector for Sentiment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
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
    pub struct TextSpan {
        #[doc = "The API calculates the beginning offset of the content in the original\ndocument according to the EncodingType specified in the API request."]
        #[serde(rename = "beginOffset", default)]
        pub begin_offset: Option<i32>,
        #[doc = "The content of the output text."]
        #[serde(rename = "content", default)]
        pub content: Option<String>,
    }
    impl ::field_selector::FieldSelector for TextSpan {
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
    pub struct Token {
        #[doc = "Dependency tree parse for this token."]
        #[serde(rename = "dependencyEdge", default)]
        pub dependency_edge: Option<crate::schemas::DependencyEdge>,
        #[doc = "[Lemma](https://en.wikipedia.org/wiki/Lemma_%28morphology%29) of the token."]
        #[serde(rename = "lemma", default)]
        pub lemma: Option<String>,
        #[doc = "Parts of speech tag for this token."]
        #[serde(rename = "partOfSpeech", default)]
        pub part_of_speech: Option<crate::schemas::PartOfSpeech>,
        #[doc = "The token text."]
        #[serde(rename = "text", default)]
        pub text: Option<crate::schemas::TextSpan>,
    }
    impl ::field_selector::FieldSelector for Token {
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
    #[doc = "Actions that can be performed on the documents resource"]
    pub fn documents(&self) -> crate::documents::DocumentsActions<A> {
        crate::documents::DocumentsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod documents {
    pub mod params {}
    pub struct DocumentsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> DocumentsActions<'a, A> {
        #[doc = "Finds named entities (currently proper names and common nouns) in the text\nalong with entity types, salience, mentions for each entity, and\nother properties."]
        pub fn analyze_entities(
            &self,
            request: crate::schemas::AnalyzeEntitiesRequest,
        ) -> AnalyzeEntitiesRequestBuilder<A> {
            AnalyzeEntitiesRequestBuilder {
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
        #[doc = "Finds entities, similar to AnalyzeEntities in the text and analyzes\nsentiment associated with each entity and its mentions."]
        pub fn analyze_entity_sentiment(
            &self,
            request: crate::schemas::AnalyzeEntitySentimentRequest,
        ) -> AnalyzeEntitySentimentRequestBuilder<A> {
            AnalyzeEntitySentimentRequestBuilder {
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
        #[doc = "Analyzes the sentiment of the provided text."]
        pub fn analyze_sentiment(
            &self,
            request: crate::schemas::AnalyzeSentimentRequest,
        ) -> AnalyzeSentimentRequestBuilder<A> {
            AnalyzeSentimentRequestBuilder {
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
        #[doc = "Analyzes the syntax of the text and provides sentence boundaries and\ntokenization along with part of speech tags, dependency trees, and other\nproperties."]
        pub fn analyze_syntax(
            &self,
            request: crate::schemas::AnalyzeSyntaxRequest,
        ) -> AnalyzeSyntaxRequestBuilder<A> {
            AnalyzeSyntaxRequestBuilder {
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
        #[doc = "A convenience method that provides all syntax, sentiment, entity, and\nclassification features in one call."]
        pub fn annotate_text(
            &self,
            request: crate::schemas::AnnotateTextRequest,
        ) -> AnnotateTextRequestBuilder<A> {
            AnnotateTextRequestBuilder {
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
        #[doc = "Classifies a document into categories."]
        pub fn classify_text(
            &self,
            request: crate::schemas::ClassifyTextRequest,
        ) -> ClassifyTextRequestBuilder<A> {
            ClassifyTextRequestBuilder {
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
    pub struct AnalyzeEntitiesRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AnalyzeEntitiesRequest,
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
    impl<'a, A: yup_oauth2::GetToken> AnalyzeEntitiesRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AnalyzeEntitiesResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://language.googleapis.com/".to_owned();
            output.push_str("v1beta2/documents:analyzeEntities");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-language"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct AnalyzeEntitySentimentRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AnalyzeEntitySentimentRequest,
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
    impl<'a, A: yup_oauth2::GetToken> AnalyzeEntitySentimentRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AnalyzeEntitySentimentResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://language.googleapis.com/".to_owned();
            output.push_str("v1beta2/documents:analyzeEntitySentiment");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-language"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct AnalyzeSentimentRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AnalyzeSentimentRequest,
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
    impl<'a, A: yup_oauth2::GetToken> AnalyzeSentimentRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AnalyzeSentimentResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://language.googleapis.com/".to_owned();
            output.push_str("v1beta2/documents:analyzeSentiment");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-language"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct AnalyzeSyntaxRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AnalyzeSyntaxRequest,
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
    impl<'a, A: yup_oauth2::GetToken> AnalyzeSyntaxRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AnalyzeSyntaxResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://language.googleapis.com/".to_owned();
            output.push_str("v1beta2/documents:analyzeSyntax");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-language"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct AnnotateTextRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AnnotateTextRequest,
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
    impl<'a, A: yup_oauth2::GetToken> AnnotateTextRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AnnotateTextResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://language.googleapis.com/".to_owned();
            output.push_str("v1beta2/documents:annotateText");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-language"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ClassifyTextRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ClassifyTextRequest,
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
    impl<'a, A: yup_oauth2::GetToken> ClassifyTextRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::ClassifyTextResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://language.googleapis.com/".to_owned();
            output.push_str("v1beta2/documents:classifyText");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-language"])
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
