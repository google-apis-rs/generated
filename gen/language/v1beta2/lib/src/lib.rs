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
    pub struct AnalyzeEntitiesRequest {
        #[doc = "Required. Input document."]
        #[serde(rename = "document", default)]
        pub document: ::std::option::Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type:
            ::std::option::Option<crate::schemas::AnalyzeEntitiesRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnalyzeEntitiesRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
    }
    impl AnalyzeEntitiesRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeEntitiesRequestEncodingType::None => "NONE",
                AnalyzeEntitiesRequestEncodingType::Utf16 => "UTF16",
                AnalyzeEntitiesRequestEncodingType::Utf32 => "UTF32",
                AnalyzeEntitiesRequestEncodingType::Utf8 => "UTF8",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeEntitiesRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeEntitiesRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeEntitiesRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeEntitiesRequestEncodingType::None,
                "UTF16" => AnalyzeEntitiesRequestEncodingType::Utf16,
                "UTF32" => AnalyzeEntitiesRequestEncodingType::Utf32,
                "UTF8" => AnalyzeEntitiesRequestEncodingType::Utf8,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitiesRequestEncodingType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnalyzeEntitiesResponse {
        #[doc = "The recognized entities in the input document."]
        #[serde(rename = "entities", default)]
        pub entities: ::std::option::Option<Vec<crate::schemas::Entity>>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitiesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub document: ::std::option::Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type:
            ::std::option::Option<crate::schemas::AnalyzeEntitySentimentRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitySentimentRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnalyzeEntitySentimentRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
    }
    impl AnalyzeEntitySentimentRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeEntitySentimentRequestEncodingType::None => "NONE",
                AnalyzeEntitySentimentRequestEncodingType::Utf16 => "UTF16",
                AnalyzeEntitySentimentRequestEncodingType::Utf32 => "UTF32",
                AnalyzeEntitySentimentRequestEncodingType::Utf8 => "UTF8",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeEntitySentimentRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeEntitySentimentRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeEntitySentimentRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeEntitySentimentRequestEncodingType::None,
                "UTF16" => AnalyzeEntitySentimentRequestEncodingType::Utf16,
                "UTF32" => AnalyzeEntitySentimentRequestEncodingType::Utf32,
                "UTF8" => AnalyzeEntitySentimentRequestEncodingType::Utf8,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitySentimentRequestEncodingType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnalyzeEntitySentimentResponse {
        #[doc = "The recognized entities in the input document with associated sentiments."]
        #[serde(rename = "entities", default)]
        pub entities: ::std::option::Option<Vec<crate::schemas::Entity>>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AnalyzeEntitySentimentResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub document: ::std::option::Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate sentence offsets for the\nsentence sentiment."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type:
            ::std::option::Option<crate::schemas::AnalyzeSentimentRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSentimentRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnalyzeSentimentRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
    }
    impl AnalyzeSentimentRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeSentimentRequestEncodingType::None => "NONE",
                AnalyzeSentimentRequestEncodingType::Utf16 => "UTF16",
                AnalyzeSentimentRequestEncodingType::Utf32 => "UTF32",
                AnalyzeSentimentRequestEncodingType::Utf8 => "UTF8",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeSentimentRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeSentimentRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeSentimentRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeSentimentRequestEncodingType::None,
                "UTF16" => AnalyzeSentimentRequestEncodingType::Utf16,
                "UTF32" => AnalyzeSentimentRequestEncodingType::Utf32,
                "UTF8" => AnalyzeSentimentRequestEncodingType::Utf8,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AnalyzeSentimentRequestEncodingType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnalyzeSentimentResponse {
        #[doc = "The overall sentiment of the input document."]
        #[serde(rename = "documentSentiment", default)]
        pub document_sentiment: ::std::option::Option<crate::schemas::Sentiment>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "The sentiment for all the sentences in the document."]
        #[serde(rename = "sentences", default)]
        pub sentences: ::std::option::Option<Vec<crate::schemas::Sentence>>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSentimentResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub document: ::std::option::Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: ::std::option::Option<crate::schemas::AnalyzeSyntaxRequestEncodingType>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSyntaxRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnalyzeSyntaxRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
    }
    impl AnalyzeSyntaxRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnalyzeSyntaxRequestEncodingType::None => "NONE",
                AnalyzeSyntaxRequestEncodingType::Utf16 => "UTF16",
                AnalyzeSyntaxRequestEncodingType::Utf32 => "UTF32",
                AnalyzeSyntaxRequestEncodingType::Utf8 => "UTF8",
            }
        }
    }
    impl ::std::fmt::Display for AnalyzeSyntaxRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnalyzeSyntaxRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnalyzeSyntaxRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnalyzeSyntaxRequestEncodingType::None,
                "UTF16" => AnalyzeSyntaxRequestEncodingType::Utf16,
                "UTF32" => AnalyzeSyntaxRequestEncodingType::Utf32,
                "UTF8" => AnalyzeSyntaxRequestEncodingType::Utf8,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AnalyzeSyntaxRequestEncodingType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnalyzeSyntaxResponse {
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "Sentences in the input document."]
        #[serde(rename = "sentences", default)]
        pub sentences: ::std::option::Option<Vec<crate::schemas::Sentence>>,
        #[doc = "Tokens, along with their syntactic information, in the input document."]
        #[serde(rename = "tokens", default)]
        pub tokens: ::std::option::Option<Vec<crate::schemas::Token>>,
    }
    impl ::field_selector::FieldSelector for AnalyzeSyntaxResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub document: ::std::option::Option<crate::schemas::Document>,
        #[doc = "The encoding type used by the API to calculate offsets."]
        #[serde(rename = "encodingType", default)]
        pub encoding_type: ::std::option::Option<crate::schemas::AnnotateTextRequestEncodingType>,
        #[doc = "Required. The enabled features."]
        #[serde(rename = "features", default)]
        pub features: ::std::option::Option<crate::schemas::Features>,
    }
    impl ::field_selector::FieldSelector for AnnotateTextRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnnotateTextRequestEncodingType {
        #[doc = "If `EncodingType` is not specified, encoding-dependent information (such as\n`begin_offset`) will be set at `-1`."]
        None,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-16 encoding of the input. Java and JavaScript are examples of\nlanguages that use this encoding natively."]
        Utf16,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-32 encoding of the input. Python is an example of a language\nthat uses this encoding natively."]
        Utf32,
        #[doc = "Encoding-dependent information (such as `begin_offset`) is calculated based\non the UTF-8 encoding of the input. C++ and Go are examples of languages\nthat use this encoding natively."]
        Utf8,
    }
    impl AnnotateTextRequestEncodingType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnnotateTextRequestEncodingType::None => "NONE",
                AnnotateTextRequestEncodingType::Utf16 => "UTF16",
                AnnotateTextRequestEncodingType::Utf32 => "UTF32",
                AnnotateTextRequestEncodingType::Utf8 => "UTF8",
            }
        }
    }
    impl ::std::fmt::Display for AnnotateTextRequestEncodingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnnotateTextRequestEncodingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnnotateTextRequestEncodingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => AnnotateTextRequestEncodingType::None,
                "UTF16" => AnnotateTextRequestEncodingType::Utf16,
                "UTF32" => AnnotateTextRequestEncodingType::Utf32,
                "UTF8" => AnnotateTextRequestEncodingType::Utf8,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AnnotateTextRequestEncodingType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnnotateTextResponse {
        #[doc = "Categories identified in the input document."]
        #[serde(rename = "categories", default)]
        pub categories: ::std::option::Option<Vec<crate::schemas::ClassificationCategory>>,
        #[doc = "The overall sentiment for the document. Populated if the user enables\nAnnotateTextRequest.Features.extract_document_sentiment."]
        #[serde(rename = "documentSentiment", default)]
        pub document_sentiment: ::std::option::Option<crate::schemas::Sentiment>,
        #[doc = "Entities, along with their semantic information, in the input document.\nPopulated if the user enables\nAnnotateTextRequest.Features.extract_entities."]
        #[serde(rename = "entities", default)]
        pub entities: ::std::option::Option<Vec<crate::schemas::Entity>>,
        #[doc = "The language of the text, which will be the same as the language specified\nin the request or, if not specified, the automatically-detected language.\nSee Document.language field for more details."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "Sentences in the input document. Populated if the user enables\nAnnotateTextRequest.Features.extract_syntax."]
        #[serde(rename = "sentences", default)]
        pub sentences: ::std::option::Option<Vec<crate::schemas::Sentence>>,
        #[doc = "Tokens, along with their syntactic information, in the input document.\nPopulated if the user enables\nAnnotateTextRequest.Features.extract_syntax."]
        #[serde(rename = "tokens", default)]
        pub tokens: ::std::option::Option<Vec<crate::schemas::Token>>,
    }
    impl ::field_selector::FieldSelector for AnnotateTextResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClassificationCategory {
        #[doc = "The classifier's confidence of the category. Number represents how certain\nthe classifier is that this category represents the given text."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of the category representing the document, from the [predefined\ntaxonomy](/natural-language/docs/categories)."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ClassificationCategory {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub document: ::std::option::Option<crate::schemas::Document>,
    }
    impl ::field_selector::FieldSelector for ClassifyTextRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClassifyTextResponse {
        #[doc = "Categories representing the input document."]
        #[serde(rename = "categories", default)]
        pub categories: ::std::option::Option<Vec<crate::schemas::ClassificationCategory>>,
    }
    impl ::field_selector::FieldSelector for ClassifyTextResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub head_token_index: ::std::option::Option<i32>,
        #[doc = "The parse label for the token."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<crate::schemas::DependencyEdgeLabel>,
    }
    impl ::field_selector::FieldSelector for DependencyEdge {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DependencyEdgeLabel {
        #[doc = "Abbreviation modifier"]
        Abbrev,
        #[doc = "Adjectival complement"]
        Acomp,
        #[doc = "Adverbial clause modifier"]
        Advcl,
        #[doc = "Adverbial modifier"]
        Advmod,
        #[doc = "Adverbial phrase modifier"]
        Advphmod,
        #[doc = "Adjectival modifier of an NP"]
        Amod,
        #[doc = "Appositional modifier of an NP"]
        Appos,
        #[doc = "Aspect marker"]
        Asp,
        #[doc = "Attribute dependent of a copular verb"]
        Attr,
        #[doc = "Auxiliary (non-main) verb"]
        Aux,
        #[doc = "Causative auxiliary"]
        Auxcaus,
        #[doc = "Passive auxiliary"]
        Auxpass,
        #[doc = "Helper auxiliary"]
        Auxvv,
        #[doc = "Coordinating conjunction"]
        Cc,
        #[doc = "Clausal complement of a verb or adjective"]
        Ccomp,
        #[doc = "Conjunct"]
        Conj,
        #[doc = "Copula"]
        Cop,
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
        #[doc = "Dislocated relation (for fronted/topicalized elements)"]
        Dislocated,
        #[doc = "Direct object"]
        Dobj,
        #[doc = "Rentaishi (Prenominal modifier)"]
        Dtmod,
        #[doc = "Expletive"]
        Expl,
        #[doc = "Foreign words"]
        Foreign,
        #[doc = "Genitive modifier"]
        Gmod,
        #[doc = "Genitive object"]
        Gobj,
        #[doc = "Goes with (part of a word in a text not well edited)"]
        Goeswith,
        #[doc = "Infinitival modifier"]
        Infmod,
        #[doc = "Indirect object"]
        Iobj,
        #[doc = "Keyword"]
        Kw,
        #[doc = "List for chains of comparable items"]
        List,
        #[doc = "Marker (word introducing a subordinate clause)"]
        Mark,
        #[doc = "Measure"]
        Mes,
        #[doc = "Multi-word expression"]
        Mwe,
        #[doc = "Multi-word verbal expression"]
        Mwv,
        #[doc = "Nominal complement of a noun"]
        Ncomp,
        #[doc = "Negation modifier"]
        Neg,
        #[doc = "Noun compound modifier"]
        Nn,
        #[doc = "Nominalized clause"]
        Nomc,
        #[doc = "Nominalized clausal subject"]
        Nomcsubj,
        #[doc = "Nominalized clausal passive"]
        Nomcsubjpass,
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
        #[doc = "Compound of numeric modifier"]
        Numc,
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
        #[doc = "Name suffix"]
        Suffix,
        #[doc = "Name title"]
        Title,
        #[doc = "Temporal modifier"]
        Tmod,
        #[doc = "Topic marker"]
        Topic,
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Clause headed by an infinite form of the verb that modifies a noun"]
        Vmod,
        #[doc = "Vocative"]
        Vocative,
        #[doc = "Open clausal complement"]
        Xcomp,
    }
    impl DependencyEdgeLabel {
        pub fn as_str(self) -> &'static str {
            match self {
                DependencyEdgeLabel::Abbrev => "ABBREV",
                DependencyEdgeLabel::Acomp => "ACOMP",
                DependencyEdgeLabel::Advcl => "ADVCL",
                DependencyEdgeLabel::Advmod => "ADVMOD",
                DependencyEdgeLabel::Advphmod => "ADVPHMOD",
                DependencyEdgeLabel::Amod => "AMOD",
                DependencyEdgeLabel::Appos => "APPOS",
                DependencyEdgeLabel::Asp => "ASP",
                DependencyEdgeLabel::Attr => "ATTR",
                DependencyEdgeLabel::Aux => "AUX",
                DependencyEdgeLabel::Auxcaus => "AUXCAUS",
                DependencyEdgeLabel::Auxpass => "AUXPASS",
                DependencyEdgeLabel::Auxvv => "AUXVV",
                DependencyEdgeLabel::Cc => "CC",
                DependencyEdgeLabel::Ccomp => "CCOMP",
                DependencyEdgeLabel::Conj => "CONJ",
                DependencyEdgeLabel::Cop => "COP",
                DependencyEdgeLabel::Csubj => "CSUBJ",
                DependencyEdgeLabel::Csubjpass => "CSUBJPASS",
                DependencyEdgeLabel::Dep => "DEP",
                DependencyEdgeLabel::Det => "DET",
                DependencyEdgeLabel::Discourse => "DISCOURSE",
                DependencyEdgeLabel::Dislocated => "DISLOCATED",
                DependencyEdgeLabel::Dobj => "DOBJ",
                DependencyEdgeLabel::Dtmod => "DTMOD",
                DependencyEdgeLabel::Expl => "EXPL",
                DependencyEdgeLabel::Foreign => "FOREIGN",
                DependencyEdgeLabel::Gmod => "GMOD",
                DependencyEdgeLabel::Gobj => "GOBJ",
                DependencyEdgeLabel::Goeswith => "GOESWITH",
                DependencyEdgeLabel::Infmod => "INFMOD",
                DependencyEdgeLabel::Iobj => "IOBJ",
                DependencyEdgeLabel::Kw => "KW",
                DependencyEdgeLabel::List => "LIST",
                DependencyEdgeLabel::Mark => "MARK",
                DependencyEdgeLabel::Mes => "MES",
                DependencyEdgeLabel::Mwe => "MWE",
                DependencyEdgeLabel::Mwv => "MWV",
                DependencyEdgeLabel::Ncomp => "NCOMP",
                DependencyEdgeLabel::Neg => "NEG",
                DependencyEdgeLabel::Nn => "NN",
                DependencyEdgeLabel::Nomc => "NOMC",
                DependencyEdgeLabel::Nomcsubj => "NOMCSUBJ",
                DependencyEdgeLabel::Nomcsubjpass => "NOMCSUBJPASS",
                DependencyEdgeLabel::Npadvmod => "NPADVMOD",
                DependencyEdgeLabel::Nsubj => "NSUBJ",
                DependencyEdgeLabel::Nsubjpass => "NSUBJPASS",
                DependencyEdgeLabel::Num => "NUM",
                DependencyEdgeLabel::Number => "NUMBER",
                DependencyEdgeLabel::Numc => "NUMC",
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
                DependencyEdgeLabel::Suffix => "SUFFIX",
                DependencyEdgeLabel::Title => "TITLE",
                DependencyEdgeLabel::Tmod => "TMOD",
                DependencyEdgeLabel::Topic => "TOPIC",
                DependencyEdgeLabel::Unknown => "UNKNOWN",
                DependencyEdgeLabel::Vmod => "VMOD",
                DependencyEdgeLabel::Vocative => "VOCATIVE",
                DependencyEdgeLabel::Xcomp => "XCOMP",
            }
        }
    }
    impl ::std::fmt::Display for DependencyEdgeLabel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DependencyEdgeLabel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DependencyEdgeLabel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABBREV" => DependencyEdgeLabel::Abbrev,
                "ACOMP" => DependencyEdgeLabel::Acomp,
                "ADVCL" => DependencyEdgeLabel::Advcl,
                "ADVMOD" => DependencyEdgeLabel::Advmod,
                "ADVPHMOD" => DependencyEdgeLabel::Advphmod,
                "AMOD" => DependencyEdgeLabel::Amod,
                "APPOS" => DependencyEdgeLabel::Appos,
                "ASP" => DependencyEdgeLabel::Asp,
                "ATTR" => DependencyEdgeLabel::Attr,
                "AUX" => DependencyEdgeLabel::Aux,
                "AUXCAUS" => DependencyEdgeLabel::Auxcaus,
                "AUXPASS" => DependencyEdgeLabel::Auxpass,
                "AUXVV" => DependencyEdgeLabel::Auxvv,
                "CC" => DependencyEdgeLabel::Cc,
                "CCOMP" => DependencyEdgeLabel::Ccomp,
                "CONJ" => DependencyEdgeLabel::Conj,
                "COP" => DependencyEdgeLabel::Cop,
                "CSUBJ" => DependencyEdgeLabel::Csubj,
                "CSUBJPASS" => DependencyEdgeLabel::Csubjpass,
                "DEP" => DependencyEdgeLabel::Dep,
                "DET" => DependencyEdgeLabel::Det,
                "DISCOURSE" => DependencyEdgeLabel::Discourse,
                "DISLOCATED" => DependencyEdgeLabel::Dislocated,
                "DOBJ" => DependencyEdgeLabel::Dobj,
                "DTMOD" => DependencyEdgeLabel::Dtmod,
                "EXPL" => DependencyEdgeLabel::Expl,
                "FOREIGN" => DependencyEdgeLabel::Foreign,
                "GMOD" => DependencyEdgeLabel::Gmod,
                "GOBJ" => DependencyEdgeLabel::Gobj,
                "GOESWITH" => DependencyEdgeLabel::Goeswith,
                "INFMOD" => DependencyEdgeLabel::Infmod,
                "IOBJ" => DependencyEdgeLabel::Iobj,
                "KW" => DependencyEdgeLabel::Kw,
                "LIST" => DependencyEdgeLabel::List,
                "MARK" => DependencyEdgeLabel::Mark,
                "MES" => DependencyEdgeLabel::Mes,
                "MWE" => DependencyEdgeLabel::Mwe,
                "MWV" => DependencyEdgeLabel::Mwv,
                "NCOMP" => DependencyEdgeLabel::Ncomp,
                "NEG" => DependencyEdgeLabel::Neg,
                "NN" => DependencyEdgeLabel::Nn,
                "NOMC" => DependencyEdgeLabel::Nomc,
                "NOMCSUBJ" => DependencyEdgeLabel::Nomcsubj,
                "NOMCSUBJPASS" => DependencyEdgeLabel::Nomcsubjpass,
                "NPADVMOD" => DependencyEdgeLabel::Npadvmod,
                "NSUBJ" => DependencyEdgeLabel::Nsubj,
                "NSUBJPASS" => DependencyEdgeLabel::Nsubjpass,
                "NUM" => DependencyEdgeLabel::Num,
                "NUMBER" => DependencyEdgeLabel::Number,
                "NUMC" => DependencyEdgeLabel::Numc,
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
                "SUFFIX" => DependencyEdgeLabel::Suffix,
                "TITLE" => DependencyEdgeLabel::Title,
                "TMOD" => DependencyEdgeLabel::Tmod,
                "TOPIC" => DependencyEdgeLabel::Topic,
                "UNKNOWN" => DependencyEdgeLabel::Unknown,
                "VMOD" => DependencyEdgeLabel::Vmod,
                "VOCATIVE" => DependencyEdgeLabel::Vocative,
                "XCOMP" => DependencyEdgeLabel::Xcomp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DependencyEdgeLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub content: ::std::option::Option<String>,
        #[doc = "The Google Cloud Storage URI where the file content is located.\nThis URI must be of the form: gs://bucket_name/object_name. For more\ndetails, see https://cloud.google.com/storage/docs/reference-uris.\nNOTE: Cloud Storage object versioning is not supported."]
        #[serde(rename = "gcsContentUri", default)]
        pub gcs_content_uri: ::std::option::Option<String>,
        #[doc = "The language of the document (if not specified, the language is\nautomatically detected). Both ISO and BCP-47 language codes are\naccepted.<br>\n[Language Support](/natural-language/docs/languages)\nlists currently supported languages for each API method.\nIf the language (either specified by the caller or automatically detected)\nis not supported by the called API method, an `INVALID_ARGUMENT` error\nis returned."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "Required. If the type is not set or is `TYPE_UNSPECIFIED`,\nreturns an `INVALID_ARGUMENT` error."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DocumentType>,
    }
    impl ::field_selector::FieldSelector for Document {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DocumentType {
        #[doc = "HTML"]
        Html,
        #[doc = "Plain text"]
        PlainText,
        #[doc = "The content type is not specified."]
        TypeUnspecified,
    }
    impl DocumentType {
        pub fn as_str(self) -> &'static str {
            match self {
                DocumentType::Html => "HTML",
                DocumentType::PlainText => "PLAIN_TEXT",
                DocumentType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for DocumentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DocumentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DocumentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HTML" => DocumentType::Html,
                "PLAIN_TEXT" => DocumentType::PlainText,
                "TYPE_UNSPECIFIED" => DocumentType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DocumentType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Entity {
        #[doc = "The mentions of this entity in the input document. The API currently\nsupports proper noun mentions."]
        #[serde(rename = "mentions", default)]
        pub mentions: ::std::option::Option<Vec<crate::schemas::EntityMention>>,
        #[doc = "Metadata associated with the entity.\n\nFor most entity types, the metadata is a Wikipedia URL (`wikipedia_url`)\nand Knowledge Graph MID (`mid`), if they are available. For the metadata\nassociated with other entity types, see the Type table below."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The representative name for the entity."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The entity type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::EntityType>,
        #[doc = "The salience score associated with the entity in the [0, 1.0] range.\n\nThe salience score for an entity provides information about the\nimportance or centrality of that entity to the entire document text.\nScores closer to 0 are less salient, while scores closer to 1.0 are highly\nsalient."]
        #[serde(rename = "salience", default)]
        pub salience: ::std::option::Option<f32>,
        #[doc = "For calls to AnalyzeEntitySentiment or if\nAnnotateTextRequest.Features.extract_entity_sentiment is set to\ntrue, this field will contain the aggregate sentiment expressed for this\nentity in the provided document."]
        #[serde(rename = "sentiment", default)]
        pub sentiment: ::std::option::Option<crate::schemas::Sentiment>,
    }
    impl ::field_selector::FieldSelector for Entity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EntityType {
        #[doc = "Address\n\nThe metadata identifies the street number and locality plus whichever\nadditional elements appear in the text:\n\n* `street_number` - street number\n* `locality` - city or town\n* `street_name` - street/route name, if detected\n* `postal_code` - postal code, if detected\n* `country` - country, if detected<\n* `broad_region` - administrative area, such as the state, if detected\n* `narrow_region` - smaller administrative area, such as county, if\n  detected\n* `sublocality` - used in Asian addresses to demark a district within a\n  city, if detected"]
        Address,
        #[doc = "Consumer product"]
        ConsumerGood,
        #[doc = "Date\n\nThe metadata identifies the components of the date:\n\n* `year` - four digit year, if detected\n* `month` - two digit month number, if detected\n* `day` - two digit day number, if detected"]
        Date,
        #[doc = "Event"]
        Event,
        #[doc = "Location"]
        Location,
        #[doc = "Number\n\nThe metadata is the number itself."]
        Number,
        #[doc = "Organization"]
        Organization,
        #[doc = "Other types of entities"]
        Other,
        #[doc = "Person"]
        Person,
        #[doc = "Phone number\n\nThe metadata lists the phone number, formatted according to local\nconvention, plus whichever additional elements appear in the text:\n\n* `number` - the actual number, broken down into sections as per local\n  convention\n* `national_prefix` - country code, if detected\n* `area_code` - region or area code, if detected\n* `extension` - phone extension (to be dialed after connection), if\n  detected"]
        PhoneNumber,
        #[doc = "Price\n\nThe metadata identifies the `value` and `currency`."]
        Price,
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Artwork"]
        WorkOfArt,
    }
    impl EntityType {
        pub fn as_str(self) -> &'static str {
            match self {
                EntityType::Address => "ADDRESS",
                EntityType::ConsumerGood => "CONSUMER_GOOD",
                EntityType::Date => "DATE",
                EntityType::Event => "EVENT",
                EntityType::Location => "LOCATION",
                EntityType::Number => "NUMBER",
                EntityType::Organization => "ORGANIZATION",
                EntityType::Other => "OTHER",
                EntityType::Person => "PERSON",
                EntityType::PhoneNumber => "PHONE_NUMBER",
                EntityType::Price => "PRICE",
                EntityType::Unknown => "UNKNOWN",
                EntityType::WorkOfArt => "WORK_OF_ART",
            }
        }
    }
    impl ::std::fmt::Display for EntityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EntityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EntityType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDRESS" => EntityType::Address,
                "CONSUMER_GOOD" => EntityType::ConsumerGood,
                "DATE" => EntityType::Date,
                "EVENT" => EntityType::Event,
                "LOCATION" => EntityType::Location,
                "NUMBER" => EntityType::Number,
                "ORGANIZATION" => EntityType::Organization,
                "OTHER" => EntityType::Other,
                "PERSON" => EntityType::Person,
                "PHONE_NUMBER" => EntityType::PhoneNumber,
                "PRICE" => EntityType::Price,
                "UNKNOWN" => EntityType::Unknown,
                "WORK_OF_ART" => EntityType::WorkOfArt,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for EntityType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EntityMention {
        #[doc = "The type of the entity mention."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::EntityMentionType>,
        #[doc = "For calls to AnalyzeEntitySentiment or if\nAnnotateTextRequest.Features.extract_entity_sentiment is set to\ntrue, this field will contain the sentiment expressed for this mention of\nthe entity in the provided document."]
        #[serde(rename = "sentiment", default)]
        pub sentiment: ::std::option::Option<crate::schemas::Sentiment>,
        #[doc = "The mention text."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<crate::schemas::TextSpan>,
    }
    impl ::field_selector::FieldSelector for EntityMention {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EntityMentionType {
        #[doc = "Common noun (or noun compound)"]
        Common,
        #[doc = "Proper name"]
        Proper,
        #[doc = "Unknown"]
        TypeUnknown,
    }
    impl EntityMentionType {
        pub fn as_str(self) -> &'static str {
            match self {
                EntityMentionType::Common => "COMMON",
                EntityMentionType::Proper => "PROPER",
                EntityMentionType::TypeUnknown => "TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for EntityMentionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EntityMentionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EntityMentionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMON" => EntityMentionType::Common,
                "PROPER" => EntityMentionType::Proper,
                "TYPE_UNKNOWN" => EntityMentionType::TypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for EntityMentionType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub classify_text: ::std::option::Option<bool>,
        #[doc = "Extract document-level sentiment."]
        #[serde(rename = "extractDocumentSentiment", default)]
        pub extract_document_sentiment: ::std::option::Option<bool>,
        #[doc = "Extract entities."]
        #[serde(rename = "extractEntities", default)]
        pub extract_entities: ::std::option::Option<bool>,
        #[doc = "Extract entities and their associated sentiment."]
        #[serde(rename = "extractEntitySentiment", default)]
        pub extract_entity_sentiment: ::std::option::Option<bool>,
        #[doc = "Extract syntax information."]
        #[serde(rename = "extractSyntax", default)]
        pub extract_syntax: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for Features {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub aspect: ::std::option::Option<crate::schemas::PartOfSpeechAspect>,
        #[doc = "The grammatical case."]
        #[serde(rename = "case", default)]
        pub case: ::std::option::Option<crate::schemas::PartOfSpeechCase>,
        #[doc = "The grammatical form."]
        #[serde(rename = "form", default)]
        pub form: ::std::option::Option<crate::schemas::PartOfSpeechForm>,
        #[doc = "The grammatical gender."]
        #[serde(rename = "gender", default)]
        pub gender: ::std::option::Option<crate::schemas::PartOfSpeechGender>,
        #[doc = "The grammatical mood."]
        #[serde(rename = "mood", default)]
        pub mood: ::std::option::Option<crate::schemas::PartOfSpeechMood>,
        #[doc = "The grammatical number."]
        #[serde(rename = "number", default)]
        pub number: ::std::option::Option<crate::schemas::PartOfSpeechNumber>,
        #[doc = "The grammatical person."]
        #[serde(rename = "person", default)]
        pub person: ::std::option::Option<crate::schemas::PartOfSpeechPerson>,
        #[doc = "The grammatical properness."]
        #[serde(rename = "proper", default)]
        pub proper: ::std::option::Option<crate::schemas::PartOfSpeechProper>,
        #[doc = "The grammatical reciprocity."]
        #[serde(rename = "reciprocity", default)]
        pub reciprocity: ::std::option::Option<crate::schemas::PartOfSpeechReciprocity>,
        #[doc = "The part of speech tag."]
        #[serde(rename = "tag", default)]
        pub tag: ::std::option::Option<crate::schemas::PartOfSpeechTag>,
        #[doc = "The grammatical tense."]
        #[serde(rename = "tense", default)]
        pub tense: ::std::option::Option<crate::schemas::PartOfSpeechTense>,
        #[doc = "The grammatical voice."]
        #[serde(rename = "voice", default)]
        pub voice: ::std::option::Option<crate::schemas::PartOfSpeechVoice>,
    }
    impl ::field_selector::FieldSelector for PartOfSpeech {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechAspect {
        #[doc = "Aspect is not applicable in the analyzed language or is not predicted."]
        AspectUnknown,
        #[doc = "Imperfective"]
        Imperfective,
        #[doc = "Perfective"]
        Perfective,
        #[doc = "Progressive"]
        Progressive,
    }
    impl PartOfSpeechAspect {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechAspect::AspectUnknown => "ASPECT_UNKNOWN",
                PartOfSpeechAspect::Imperfective => "IMPERFECTIVE",
                PartOfSpeechAspect::Perfective => "PERFECTIVE",
                PartOfSpeechAspect::Progressive => "PROGRESSIVE",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechAspect {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechAspect {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechAspect {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASPECT_UNKNOWN" => PartOfSpeechAspect::AspectUnknown,
                "IMPERFECTIVE" => PartOfSpeechAspect::Imperfective,
                "PERFECTIVE" => PartOfSpeechAspect::Perfective,
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
    impl ::field_selector::FieldSelector for PartOfSpeechAspect {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechCase {
        #[doc = "Accusative"]
        Accusative,
        #[doc = "Adverbial"]
        Adverbial,
        #[doc = "Case is not applicable in the analyzed language or is not predicted."]
        CaseUnknown,
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
                PartOfSpeechCase::Accusative => "ACCUSATIVE",
                PartOfSpeechCase::Adverbial => "ADVERBIAL",
                PartOfSpeechCase::CaseUnknown => "CASE_UNKNOWN",
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechCase {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechCase {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCUSATIVE" => PartOfSpeechCase::Accusative,
                "ADVERBIAL" => PartOfSpeechCase::Adverbial,
                "CASE_UNKNOWN" => PartOfSpeechCase::CaseUnknown,
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
    impl ::field_selector::FieldSelector for PartOfSpeechCase {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechForm {
        #[doc = "Adnomial"]
        Adnomial,
        #[doc = "Auxiliary"]
        Auxiliary,
        #[doc = "Complementizer"]
        Complementizer,
        #[doc = "Final ending"]
        FinalEnding,
        #[doc = "Form is not applicable in the analyzed language or is not predicted."]
        FormUnknown,
        #[doc = "Gerund"]
        Gerund,
        #[doc = "Irrealis"]
        Irrealis,
        #[doc = "Long form"]
        Long,
        #[doc = "Order form"]
        Order,
        #[doc = "Realis"]
        Realis,
        #[doc = "Short form"]
        Short,
        #[doc = "Specific form"]
        Specific,
    }
    impl PartOfSpeechForm {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechForm::Adnomial => "ADNOMIAL",
                PartOfSpeechForm::Auxiliary => "AUXILIARY",
                PartOfSpeechForm::Complementizer => "COMPLEMENTIZER",
                PartOfSpeechForm::FinalEnding => "FINAL_ENDING",
                PartOfSpeechForm::FormUnknown => "FORM_UNKNOWN",
                PartOfSpeechForm::Gerund => "GERUND",
                PartOfSpeechForm::Irrealis => "IRREALIS",
                PartOfSpeechForm::Long => "LONG",
                PartOfSpeechForm::Order => "ORDER",
                PartOfSpeechForm::Realis => "REALIS",
                PartOfSpeechForm::Short => "SHORT",
                PartOfSpeechForm::Specific => "SPECIFIC",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechForm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechForm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechForm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADNOMIAL" => PartOfSpeechForm::Adnomial,
                "AUXILIARY" => PartOfSpeechForm::Auxiliary,
                "COMPLEMENTIZER" => PartOfSpeechForm::Complementizer,
                "FINAL_ENDING" => PartOfSpeechForm::FinalEnding,
                "FORM_UNKNOWN" => PartOfSpeechForm::FormUnknown,
                "GERUND" => PartOfSpeechForm::Gerund,
                "IRREALIS" => PartOfSpeechForm::Irrealis,
                "LONG" => PartOfSpeechForm::Long,
                "ORDER" => PartOfSpeechForm::Order,
                "REALIS" => PartOfSpeechForm::Realis,
                "SHORT" => PartOfSpeechForm::Short,
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
    impl ::field_selector::FieldSelector for PartOfSpeechForm {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechGender {
        #[doc = "Feminine"]
        Feminine,
        #[doc = "Gender is not applicable in the analyzed language or is not predicted."]
        GenderUnknown,
        #[doc = "Masculine"]
        Masculine,
        #[doc = "Neuter"]
        Neuter,
    }
    impl PartOfSpeechGender {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechGender::Feminine => "FEMININE",
                PartOfSpeechGender::GenderUnknown => "GENDER_UNKNOWN",
                PartOfSpeechGender::Masculine => "MASCULINE",
                PartOfSpeechGender::Neuter => "NEUTER",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechGender {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechGender {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechGender {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FEMININE" => PartOfSpeechGender::Feminine,
                "GENDER_UNKNOWN" => PartOfSpeechGender::GenderUnknown,
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
    impl ::field_selector::FieldSelector for PartOfSpeechGender {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechMood {
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
        #[doc = "Mood is not applicable in the analyzed language or is not predicted."]
        MoodUnknown,
        #[doc = "Subjunctive"]
        Subjunctive,
    }
    impl PartOfSpeechMood {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechMood::ConditionalMood => "CONDITIONAL_MOOD",
                PartOfSpeechMood::Imperative => "IMPERATIVE",
                PartOfSpeechMood::Indicative => "INDICATIVE",
                PartOfSpeechMood::Interrogative => "INTERROGATIVE",
                PartOfSpeechMood::Jussive => "JUSSIVE",
                PartOfSpeechMood::MoodUnknown => "MOOD_UNKNOWN",
                PartOfSpeechMood::Subjunctive => "SUBJUNCTIVE",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechMood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechMood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechMood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONDITIONAL_MOOD" => PartOfSpeechMood::ConditionalMood,
                "IMPERATIVE" => PartOfSpeechMood::Imperative,
                "INDICATIVE" => PartOfSpeechMood::Indicative,
                "INTERROGATIVE" => PartOfSpeechMood::Interrogative,
                "JUSSIVE" => PartOfSpeechMood::Jussive,
                "MOOD_UNKNOWN" => PartOfSpeechMood::MoodUnknown,
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
    impl ::field_selector::FieldSelector for PartOfSpeechMood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechNumber {
        #[doc = "Dual"]
        Dual,
        #[doc = "Number is not applicable in the analyzed language or is not predicted."]
        NumberUnknown,
        #[doc = "Plural"]
        Plural,
        #[doc = "Singular"]
        Singular,
    }
    impl PartOfSpeechNumber {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechNumber::Dual => "DUAL",
                PartOfSpeechNumber::NumberUnknown => "NUMBER_UNKNOWN",
                PartOfSpeechNumber::Plural => "PLURAL",
                PartOfSpeechNumber::Singular => "SINGULAR",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechNumber {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechNumber {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DUAL" => PartOfSpeechNumber::Dual,
                "NUMBER_UNKNOWN" => PartOfSpeechNumber::NumberUnknown,
                "PLURAL" => PartOfSpeechNumber::Plural,
                "SINGULAR" => PartOfSpeechNumber::Singular,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechNumber {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechPerson {
        #[doc = "First"]
        First,
        #[doc = "Person is not applicable in the analyzed language or is not predicted."]
        PersonUnknown,
        #[doc = "Reflexive"]
        ReflexivePerson,
        #[doc = "Second"]
        Second,
        #[doc = "Third"]
        Third,
    }
    impl PartOfSpeechPerson {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechPerson::First => "FIRST",
                PartOfSpeechPerson::PersonUnknown => "PERSON_UNKNOWN",
                PartOfSpeechPerson::ReflexivePerson => "REFLEXIVE_PERSON",
                PartOfSpeechPerson::Second => "SECOND",
                PartOfSpeechPerson::Third => "THIRD",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechPerson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechPerson {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechPerson {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FIRST" => PartOfSpeechPerson::First,
                "PERSON_UNKNOWN" => PartOfSpeechPerson::PersonUnknown,
                "REFLEXIVE_PERSON" => PartOfSpeechPerson::ReflexivePerson,
                "SECOND" => PartOfSpeechPerson::Second,
                "THIRD" => PartOfSpeechPerson::Third,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechPerson {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechProper {
        #[doc = "Not proper"]
        NotProper,
        #[doc = "Proper"]
        Proper,
        #[doc = "Proper is not applicable in the analyzed language or is not predicted."]
        ProperUnknown,
    }
    impl PartOfSpeechProper {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechProper::NotProper => "NOT_PROPER",
                PartOfSpeechProper::Proper => "PROPER",
                PartOfSpeechProper::ProperUnknown => "PROPER_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechProper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechProper {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechProper {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_PROPER" => PartOfSpeechProper::NotProper,
                "PROPER" => PartOfSpeechProper::Proper,
                "PROPER_UNKNOWN" => PartOfSpeechProper::ProperUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechProper {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechReciprocity {
        #[doc = "Non-reciprocal"]
        NonReciprocal,
        #[doc = "Reciprocal"]
        Reciprocal,
        #[doc = "Reciprocity is not applicable in the analyzed language or is not\npredicted."]
        ReciprocityUnknown,
    }
    impl PartOfSpeechReciprocity {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechReciprocity::NonReciprocal => "NON_RECIPROCAL",
                PartOfSpeechReciprocity::Reciprocal => "RECIPROCAL",
                PartOfSpeechReciprocity::ReciprocityUnknown => "RECIPROCITY_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechReciprocity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechReciprocity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechReciprocity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NON_RECIPROCAL" => PartOfSpeechReciprocity::NonReciprocal,
                "RECIPROCAL" => PartOfSpeechReciprocity::Reciprocal,
                "RECIPROCITY_UNKNOWN" => PartOfSpeechReciprocity::ReciprocityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechReciprocity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechTag {
        #[doc = "Adjective"]
        Adj,
        #[doc = "Adposition (preposition and postposition)"]
        Adp,
        #[doc = "Adverb"]
        Adv,
        #[doc = "Affix"]
        Affix,
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
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Verb (all tenses and modes)"]
        Verb,
        #[doc = "Other: foreign words, typos, abbreviations"]
        X,
    }
    impl PartOfSpeechTag {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechTag::Adj => "ADJ",
                PartOfSpeechTag::Adp => "ADP",
                PartOfSpeechTag::Adv => "ADV",
                PartOfSpeechTag::Affix => "AFFIX",
                PartOfSpeechTag::Conj => "CONJ",
                PartOfSpeechTag::Det => "DET",
                PartOfSpeechTag::Noun => "NOUN",
                PartOfSpeechTag::Num => "NUM",
                PartOfSpeechTag::Pron => "PRON",
                PartOfSpeechTag::Prt => "PRT",
                PartOfSpeechTag::Punct => "PUNCT",
                PartOfSpeechTag::Unknown => "UNKNOWN",
                PartOfSpeechTag::Verb => "VERB",
                PartOfSpeechTag::X => "X",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechTag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechTag {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechTag {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADJ" => PartOfSpeechTag::Adj,
                "ADP" => PartOfSpeechTag::Adp,
                "ADV" => PartOfSpeechTag::Adv,
                "AFFIX" => PartOfSpeechTag::Affix,
                "CONJ" => PartOfSpeechTag::Conj,
                "DET" => PartOfSpeechTag::Det,
                "NOUN" => PartOfSpeechTag::Noun,
                "NUM" => PartOfSpeechTag::Num,
                "PRON" => PartOfSpeechTag::Pron,
                "PRT" => PartOfSpeechTag::Prt,
                "PUNCT" => PartOfSpeechTag::Punct,
                "UNKNOWN" => PartOfSpeechTag::Unknown,
                "VERB" => PartOfSpeechTag::Verb,
                "X" => PartOfSpeechTag::X,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechTag {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechTense {
        #[doc = "Conditional"]
        ConditionalTense,
        #[doc = "Future"]
        Future,
        #[doc = "Imperfect"]
        Imperfect,
        #[doc = "Past"]
        Past,
        #[doc = "Pluperfect"]
        Pluperfect,
        #[doc = "Present"]
        Present,
        #[doc = "Tense is not applicable in the analyzed language or is not predicted."]
        TenseUnknown,
    }
    impl PartOfSpeechTense {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechTense::ConditionalTense => "CONDITIONAL_TENSE",
                PartOfSpeechTense::Future => "FUTURE",
                PartOfSpeechTense::Imperfect => "IMPERFECT",
                PartOfSpeechTense::Past => "PAST",
                PartOfSpeechTense::Pluperfect => "PLUPERFECT",
                PartOfSpeechTense::Present => "PRESENT",
                PartOfSpeechTense::TenseUnknown => "TENSE_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechTense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechTense {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechTense {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONDITIONAL_TENSE" => PartOfSpeechTense::ConditionalTense,
                "FUTURE" => PartOfSpeechTense::Future,
                "IMPERFECT" => PartOfSpeechTense::Imperfect,
                "PAST" => PartOfSpeechTense::Past,
                "PLUPERFECT" => PartOfSpeechTense::Pluperfect,
                "PRESENT" => PartOfSpeechTense::Present,
                "TENSE_UNKNOWN" => PartOfSpeechTense::TenseUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechTense {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartOfSpeechVoice {
        #[doc = "Active"]
        Active,
        #[doc = "Causative"]
        Causative,
        #[doc = "Passive"]
        Passive,
        #[doc = "Voice is not applicable in the analyzed language or is not predicted."]
        VoiceUnknown,
    }
    impl PartOfSpeechVoice {
        pub fn as_str(self) -> &'static str {
            match self {
                PartOfSpeechVoice::Active => "ACTIVE",
                PartOfSpeechVoice::Causative => "CAUSATIVE",
                PartOfSpeechVoice::Passive => "PASSIVE",
                PartOfSpeechVoice::VoiceUnknown => "VOICE_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for PartOfSpeechVoice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartOfSpeechVoice {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartOfSpeechVoice {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => PartOfSpeechVoice::Active,
                "CAUSATIVE" => PartOfSpeechVoice::Causative,
                "PASSIVE" => PartOfSpeechVoice::Passive,
                "VOICE_UNKNOWN" => PartOfSpeechVoice::VoiceUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PartOfSpeechVoice {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Sentence {
        #[doc = "For calls to AnalyzeSentiment or if\nAnnotateTextRequest.Features.extract_document_sentiment is set to\ntrue, this field will contain the sentiment for the sentence."]
        #[serde(rename = "sentiment", default)]
        pub sentiment: ::std::option::Option<crate::schemas::Sentiment>,
        #[doc = "The sentence text."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<crate::schemas::TextSpan>,
    }
    impl ::field_selector::FieldSelector for Sentence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Sentiment {
        #[doc = "A non-negative number in the [0, +inf) range, which represents\nthe absolute magnitude of sentiment regardless of score (positive or\nnegative)."]
        #[serde(rename = "magnitude", default)]
        pub magnitude: ::std::option::Option<f32>,
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0\n(positive sentiment)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for Sentiment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub begin_offset: ::std::option::Option<i32>,
        #[doc = "The content of the output text."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TextSpan {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        pub dependency_edge: ::std::option::Option<crate::schemas::DependencyEdge>,
        #[doc = "[Lemma](https://en.wikipedia.org/wiki/Lemma_%28morphology%29) of the token."]
        #[serde(rename = "lemma", default)]
        pub lemma: ::std::option::Option<String>,
        #[doc = "Parts of speech tag for this token."]
        #[serde(rename = "partOfSpeech", default)]
        pub part_of_speech: ::std::option::Option<crate::schemas::PartOfSpeech>,
        #[doc = "The token text."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<crate::schemas::TextSpan>,
    }
    impl ::field_selector::FieldSelector for Token {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Xgafv {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub fn documents(&self) -> crate::resources::documents::DocumentsActions<A> {
        crate::resources::documents::DocumentsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod documents {
        pub mod params {}
        pub struct DocumentsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::AnalyzeEntitiesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AnalyzeEntitiesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-language"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::AnalyzeEntitySentimentResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AnalyzeEntitySentimentResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-language"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::AnalyzeSentimentResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AnalyzeSentimentResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-language"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::AnalyzeSyntaxResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AnalyzeSyntaxResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-language"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::AnnotateTextResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AnnotateTextResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-language"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::ClassifyTextResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ClassifyTextResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-language"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
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
