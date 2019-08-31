pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnalyzeCommentRequest {
        #[doc = "Opaque token that is echoed from the request to the response."]
        #[serde(rename = "clientToken", default)]
        pub client_token: ::std::option::Option<String>,
        #[doc = "The comment to analyze."]
        #[serde(rename = "comment", default)]
        pub comment: ::std::option::Option<crate::schemas::TextEntry>,
        #[doc = "Optional identifier associating this AnalyzeCommentRequest with a\nparticular client's community. Different communities may have different\nnorms and rules. Specifying this value enables us to explore building\ncommunity-specific models for clients."]
        #[serde(rename = "communityId", default)]
        pub community_id: ::std::option::Option<String>,
        #[doc = "The context of the comment."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<crate::schemas::Context>,
        #[doc = "Do not store the comment or context sent in this request. By default, the\nservice may store comments/context for debugging purposes."]
        #[serde(rename = "doNotStore", default)]
        pub do_not_store: ::std::option::Option<bool>,
        #[doc = "The language(s) of the comment and context (if none are specified, the\nlanguage is automatically detected). If multiple languages are specified,\nthe text is checked in all of them that are supported. Both ISO and BCP-47\nlanguage codes are accepted.\nCurrent Language Restrictions:\n\n* Only English text (\"en\") is supported.\n  If none of the languages specified by the caller are supported, an\n  `UNIMPLEMENTED` error is returned."]
        #[serde(rename = "languages", default)]
        pub languages: ::std::option::Option<Vec<String>>,
        #[doc = "Specification of requested attributes. The AttributeParameters serve as\nconfiguration for each associated attribute. The map keys are attribute\nnames. The following attributes are available:\n\"ATTACK_ON_AUTHOR\" - Attack on author of original article or post.\n\"ATTACK_ON_COMMENTER\" - Attack on fellow commenter.\n\"ATTACK_ON_PUBLISHER\" - Attack on publisher of article/post.\n\"INCOHERENT\" - Difficult to understand, nonsensical.\n\"INFLAMMATORY\" - Intending to provoke or inflame.\n\"OBSCENE\" - Obscene, such as cursing.\n\"OFF_TOPIC\" - Not related to the original topic.\n\"SPAM\" - Commercial/advertising spam content.\n\"UNSUBSTANTIAL\" - Trivial."]
        #[serde(rename = "requestedAttributes", default)]
        pub requested_attributes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::AttributeParameters>,
        >,
        #[doc = "Session ID. Used to join related RPCs into a single session. For example,\nan interactive tool that calls both the AnalyzeComment and\nSuggestCommentScore RPCs should set all invocations of both RPCs to the\nsame Session ID, typically a random 64-bit integer."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
        #[doc = "An advisory parameter that will return span annotations if the model\nis capable of providing scores with sub-comment resolution. This will\nlikely increase the size of the returned message."]
        #[serde(rename = "spanAnnotations", default)]
        pub span_annotations: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeCommentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeCommentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnalyzeCommentResponse {
        #[doc = "Scores for the requested attributes. The map keys are attribute names (same\nas the requested_attribute field in AnalyzeCommentRequest, for example\n\"ATTACK_ON_AUTHOR\", \"INFLAMMATORY\", etc)."]
        #[serde(rename = "attributeScores", default)]
        pub attribute_scores: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::AttributeScores>,
        >,
        #[doc = "Same token from the original AnalyzeCommentRequest."]
        #[serde(rename = "clientToken", default)]
        pub client_token: ::std::option::Option<String>,
        #[doc = "Contains the languages detected from the text content, sorted in order of\nlikelihood."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<Vec<String>>,
        #[doc = "The language(s) used by CommentAnalyzer service to choose which Model to\nuse when analyzing the comment. Might better be called\n\"effective_languages\". The logic used to make the choice is as follows:\nif Request.languages.empty()\neffective_languages = detected_languages\nelse\neffective_languages = Request.languages"]
        #[serde(rename = "languages", default)]
        pub languages: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeCommentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeCommentResponse {
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
    pub struct ArticleAndParentComment {
        #[doc = "The source content about which the comment was made (article text, article\nsummary, video transcript, etc)."]
        #[serde(rename = "article", default)]
        pub article: ::std::option::Option<crate::schemas::TextEntry>,
        #[doc = "Refers to text that is a direct parent of the source comment, such as in a\none-deep threaded message board. This field will only be present for\ncomments that are replies to other comments and will not be populated for\ndirect comments on the article_text."]
        #[serde(rename = "parentComment", default)]
        pub parent_comment: ::std::option::Option<crate::schemas::TextEntry>,
    }
    impl ::google_field_selector::FieldSelector for ArticleAndParentComment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArticleAndParentComment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AttributeParameters {
        #[doc = "Don't return scores for this attribute that are below this threshold. If\nunset, a default threshold will be applied. A FloatValue wrapper is used to\ndistinguish between 0 vs. default/unset."]
        #[serde(rename = "scoreThreshold", default)]
        pub score_threshold: ::std::option::Option<f32>,
        #[doc = "What type of scores to return. If unset, defaults to probability scores."]
        #[serde(rename = "scoreType", default)]
        pub score_type: ::std::option::Option<crate::schemas::AttributeParametersScoreType>,
    }
    impl ::google_field_selector::FieldSelector for AttributeParameters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttributeParameters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AttributeParametersScoreType {
        #[doc = "Percentile scores are in the range [0, 1] and indicate the percentile of\nthe raw score, normalized with a test dataset. This is not generally\nrecommended, as the normalization is dependent on the dataset used, which\nmay not match other usecases."]
        Percentile,
        #[doc = "Probability scores are in the range [0, 1] and indicate level of confidence\nin the attribute label."]
        Probability,
        #[doc = "Raw scores are the raw values from the model, and may take any value. This\nis primarily for debugging/testing, and not generally recommended."]
        Raw,
        #[doc = "Unspecified. Defaults to PROBABILITY scores if available, and otherwise\nRAW. Every model has a RAW score."]
        ScoreTypeUnspecified,
        #[doc = "Standard deviation scores are in the range (-inf, +inf)."]
        StdDevScore,
    }
    impl AttributeParametersScoreType {
        pub fn as_str(self) -> &'static str {
            match self {
                AttributeParametersScoreType::Percentile => "PERCENTILE",
                AttributeParametersScoreType::Probability => "PROBABILITY",
                AttributeParametersScoreType::Raw => "RAW",
                AttributeParametersScoreType::ScoreTypeUnspecified => "SCORE_TYPE_UNSPECIFIED",
                AttributeParametersScoreType::StdDevScore => "STD_DEV_SCORE",
            }
        }
    }
    impl ::std::fmt::Display for AttributeParametersScoreType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AttributeParametersScoreType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AttributeParametersScoreType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERCENTILE" => AttributeParametersScoreType::Percentile,
                "PROBABILITY" => AttributeParametersScoreType::Probability,
                "RAW" => AttributeParametersScoreType::Raw,
                "SCORE_TYPE_UNSPECIFIED" => AttributeParametersScoreType::ScoreTypeUnspecified,
                "STD_DEV_SCORE" => AttributeParametersScoreType::StdDevScore,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AttributeParametersScoreType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttributeParametersScoreType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AttributeScores {
        #[doc = "Per-span scores."]
        #[serde(rename = "spanScores", default)]
        pub span_scores: ::std::option::Option<Vec<crate::schemas::SpanScore>>,
        #[doc = "Overall score for comment as a whole."]
        #[serde(rename = "summaryScore", default)]
        pub summary_score: ::std::option::Option<crate::schemas::Score>,
    }
    impl ::google_field_selector::FieldSelector for AttributeScores {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttributeScores {
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
    pub struct Context {
        #[doc = "Information about the source for which the original comment was made, and\nany parent comment info."]
        #[serde(rename = "articleAndParentComment", default)]
        pub article_and_parent_comment:
            ::std::option::Option<crate::schemas::ArticleAndParentComment>,
        #[doc = "A list of messages. For example, a linear comments section or forum thread."]
        #[serde(rename = "entries", default)]
        pub entries: ::std::option::Option<Vec<crate::schemas::TextEntry>>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Score {
        #[doc = "The type of the above value."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::ScoreType>,
        #[doc = "Score value. Semantics described by type below."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for Score {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Score {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScoreType {
        #[doc = "Percentile scores are in the range [0, 1] and indicate the percentile of\nthe raw score, normalized with a test dataset. This is not generally\nrecommended, as the normalization is dependent on the dataset used, which\nmay not match other usecases."]
        Percentile,
        #[doc = "Probability scores are in the range [0, 1] and indicate level of confidence\nin the attribute label."]
        Probability,
        #[doc = "Raw scores are the raw values from the model, and may take any value. This\nis primarily for debugging/testing, and not generally recommended."]
        Raw,
        #[doc = "Unspecified. Defaults to PROBABILITY scores if available, and otherwise\nRAW. Every model has a RAW score."]
        ScoreTypeUnspecified,
        #[doc = "Standard deviation scores are in the range (-inf, +inf)."]
        StdDevScore,
    }
    impl ScoreType {
        pub fn as_str(self) -> &'static str {
            match self {
                ScoreType::Percentile => "PERCENTILE",
                ScoreType::Probability => "PROBABILITY",
                ScoreType::Raw => "RAW",
                ScoreType::ScoreTypeUnspecified => "SCORE_TYPE_UNSPECIFIED",
                ScoreType::StdDevScore => "STD_DEV_SCORE",
            }
        }
    }
    impl ::std::fmt::Display for ScoreType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScoreType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScoreType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERCENTILE" => ScoreType::Percentile,
                "PROBABILITY" => ScoreType::Probability,
                "RAW" => ScoreType::Raw,
                "SCORE_TYPE_UNSPECIFIED" => ScoreType::ScoreTypeUnspecified,
                "STD_DEV_SCORE" => ScoreType::StdDevScore,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScoreType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScoreType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SpanScore {
        #[doc = "\"begin\" and \"end\" describe the span of the original text that the attribute\nscore applies to. The values are the UTF-16 codepoint range. \"end\" is\nexclusive. For example, with the text \"Hi there\", the begin/end pair (0,2)\ndescribes the text \"Hi\".\n\nIf \"begin\" and \"end\" are unset, the score applies to the full text."]
        #[serde(rename = "begin", default)]
        pub begin: ::std::option::Option<i32>,
        #[serde(rename = "end", default)]
        pub end: ::std::option::Option<i32>,
        #[doc = "The score value."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<crate::schemas::Score>,
    }
    impl ::google_field_selector::FieldSelector for SpanScore {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpanScore {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestCommentScoreRequest {
        #[doc = "Attribute scores for the comment. The map keys are attribute names, same as\nthe requested_attribute field in AnalyzeCommentRequest (for example\n\"ATTACK_ON_AUTHOR\", \"INFLAMMATORY\", etc.). This field has the same type as\nthe `attribute_scores` field in AnalyzeCommentResponse.\n\nTo specify an overall attribute score for the entire comment as a whole,\nuse the `summary_score` field of the mapped AttributeScores object. To\nspecify scores on specific subparts of the comment, use the `span_scores`\nfield. All SpanScore objects must have begin and end fields set.\n\nAll Score objects must be explicitly set (for binary classification, use\nthe score values 0 and 1). If Score objects don't include a ScoreType,\n`PROBABILITY` is assumed.\n\n`attribute_scores` must not be empty. The mapped AttributeScores objects\nalso must not be empty. An `INVALID_ARGUMENT` error is returned for all\nmalformed requests."]
        #[serde(rename = "attributeScores", default)]
        pub attribute_scores: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::AttributeScores>,
        >,
        #[doc = "Opaque token that is echoed from the request to the response."]
        #[serde(rename = "clientToken", default)]
        pub client_token: ::std::option::Option<String>,
        #[doc = "The comment being scored."]
        #[serde(rename = "comment", default)]
        pub comment: ::std::option::Option<crate::schemas::TextEntry>,
        #[doc = "Optional identifier associating this comment score suggestion with a\nparticular sub-community. Different communities may have different norms\nand rules. Specifying this value enables training community-specific\nmodels."]
        #[serde(rename = "communityId", default)]
        pub community_id: ::std::option::Option<String>,
        #[doc = "The context of the comment."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<crate::schemas::Context>,
        #[doc = "The language(s) of the comment and context (if none are specified, the\nlanguage is automatically detected). If multiple languages are specified,\nthe text is checked in all of them that are supported. Both ISO and BCP-47\nlanguage codes are accepted.\nCurrent Language Restrictions:\n\n* Only English text (\"en\") is supported.\n  If none of the languages specified by the caller are supported, an\n  `UNIMPLEMENTED` error is returned."]
        #[serde(rename = "languages", default)]
        pub languages: ::std::option::Option<Vec<String>>,
        #[doc = "Session ID. Used to join related RPCs into a single session. For example,\nan interactive tool that calls both the AnalyzeComment and\nSuggestCommentScore RPCs should set all invocations of both RPCs to the\nsame Session ID, typically a random 64-bit integer."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SuggestCommentScoreRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestCommentScoreRequest {
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
    pub struct SuggestCommentScoreResponse {
        #[doc = "Same token from the original SuggestCommentScoreRequest."]
        #[serde(rename = "clientToken", default)]
        pub client_token: ::std::option::Option<String>,
        #[doc = "The list of languages detected from the comment text."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<Vec<String>>,
        #[doc = "The list of languages provided in the request."]
        #[serde(rename = "requestedLanguages", default)]
        pub requested_languages: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for SuggestCommentScoreResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestCommentScoreResponse {
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
    pub struct TextEntry {
        #[doc = "Type of the text field."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::TextEntryType>,
        #[doc = "UTF-8 encoded text."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TextEntryType {
        #[doc = "HTML."]
        Html,
        #[doc = "Plain text."]
        PlainText,
        #[doc = "The content type is not specified. Text will be interpreted as plain text\nby default."]
        TextTypeUnspecified,
    }
    impl TextEntryType {
        pub fn as_str(self) -> &'static str {
            match self {
                TextEntryType::Html => "HTML",
                TextEntryType::PlainText => "PLAIN_TEXT",
                TextEntryType::TextTypeUnspecified => "TEXT_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for TextEntryType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TextEntryType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TextEntryType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HTML" => TextEntryType::Html,
                "PLAIN_TEXT" => TextEntryType::PlainText,
                "TEXT_TYPE_UNSPECIFIED" => TextEntryType::TextTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TextEntryType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextEntryType {
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
    #[doc = "Actions that can be performed on the comments resource"]
    pub fn comments(&self) -> crate::resources::comments::CommentsActions {
        crate::resources::comments::CommentsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod comments {
        pub mod params {}
        pub struct CommentsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CommentsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Analyzes the provided text and returns scores for requested attributes."]
            pub fn analyze(
                &self,
                request: crate::schemas::AnalyzeCommentRequest,
            ) -> AnalyzeRequestBuilder {
                AnalyzeRequestBuilder {
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
                }
            }
            #[doc = "Suggest comment scores as training data."]
            pub fn suggestscore(
                &self,
                request: crate::schemas::SuggestCommentScoreRequest,
            ) -> SuggestscoreRequestBuilder {
                SuggestscoreRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AnalyzeRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::AnalyzeCommentRequest,
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
        impl<'a> AnalyzeRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::AnalyzeCommentResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeCommentResponse, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://commentanalyzer.googleapis.com/".to_owned();
                output.push_str("v1alpha1/comments:analyze");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SuggestscoreRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SuggestCommentScoreRequest,
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
        impl<'a> SuggestscoreRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SuggestCommentScoreResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SuggestCommentScoreResponse, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://commentanalyzer.googleapis.com/".to_owned();
                output.push_str("v1alpha1/comments:suggestscore");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error>),
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
