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
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationProgress", default)]
        pub annotation_progress:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoAnnotationProgress>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1AnnotateVideoProgress {
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
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationResults", default)]
        pub annotation_results:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoAnnotationResults>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1AnnotateVideoResponse {
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
    pub struct GoogleCloudVideointelligenceV1Beta2AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationProgress", default)]
        pub annotation_progress:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2AnnotateVideoProgress {
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
    pub struct GoogleCloudVideointelligenceV1Beta2AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationResults", default)]
        pub annotation_results:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2AnnotateVideoResponse {
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
    pub struct GoogleCloudVideointelligenceV1Beta2Entity {
        #[doc = "Textual description, e.g. `Fixed-gear bicycle`."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2Entity {
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
    pub struct GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame { # [ doc = "Likelihood of the pornography content.." ] # [ serde ( rename = "pornographyLikelihood" , default ) ] pub pornography_likelihood : Option < crate :: schemas :: GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood > , # [ doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location." ] # [ serde ( rename = "timeOffset" , default ) ] pub time_offset : Option < String > , }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Likely."]
        Likely,
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryUnlikely , "UNLIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Unlikely , "POSSIBLE" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Possible , "LIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Likely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryLikely , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2LabelAnnotation {
        #[doc = "Common categories for the detected entity.\nE.g. when the label is `Terrier` the category is likely `dog`. And in some\ncases there might be more than one categories e.g. `Terrier` could also be\na `pet`."]
        #[serde(rename = "categoryEntities", default)]
        pub category_entities:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>>,
        #[doc = "Detected entity."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelFrame>>,
        #[doc = "All video segments where a label was detected."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelSegment>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2LabelAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1Beta2LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2LabelFrame {
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
    pub struct GoogleCloudVideointelligenceV1Beta2LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2LabelSegment {
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
    pub struct GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(rename = "bottom", default)]
        pub bottom: Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(rename = "left", default)]
        pub left: Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(rename = "right", default)]
        pub right: Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(rename = "top", default)]
        pub top: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox {
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
    pub struct GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(rename = "vertices", default)]
        pub vertices:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedVertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly {
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
    pub struct GoogleCloudVideointelligenceV1Beta2NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2NormalizedVertex {
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
    pub struct GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation {
        #[doc = "Object category's labeling confidence of this track."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>,
        #[doc = "Information corresponding to all frames where this object track appears.\nNon-streaming batch mode: it may be one or multiple ObjectTrackingFrame\nmessages in frames.\nStreaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame>>,
        #[doc = "Non-streaming batch mode ONLY.\nEach object track corresponds to one video segment where it appears."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "Streaming mode ONLY.\nIn streaming mode, we do not know the end time of a tracked object\nbefore it is completed. Hence, there is no VideoSegment info returned.\nInstead, we provide a unique identifiable integer track_id so that\nthe customers can correlate the results of the ongoing\nObjectTrackAnnotation of the same track_id over time."]
        #[serde(rename = "trackId", default)]
        #[serde(with = "crate::parsed_string")]
        pub track_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation
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
    pub struct GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(rename = "normalizedBoundingBox", default)]
        pub normalized_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox>,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame {
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
    pub struct GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(rename = "transcript", default)]
        pub transcript: Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word.\nNote: When `enable_speaker_diarization` is true, you will see all the words\nfrom the beginning of the audio."]
        #[serde(rename = "words", default)]
        pub words: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2WordInfo>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative
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
    pub struct GoogleCloudVideointelligenceV1Beta2SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified\nin `max_alternatives`).  These alternatives are ordered in terms of\naccuracy, with the top (first) alternative being the most probable, as\nranked by the recognizer."]
        #[serde(rename = "alternatives", default)]
        pub alternatives: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the\nlanguage in this result. This language code was detected to have the most\nlikelihood of being spoken in the audio."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2SpeechTranscription {
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
    pub struct GoogleCloudVideointelligenceV1Beta2TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TextSegment>>,
        #[doc = "The detected text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2TextAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1Beta2TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(rename = "rotatedBoundingBox", default)]
        pub rotated_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly>,
        #[doc = "Timestamp of this frame."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2TextFrame {
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
    pub struct GoogleCloudVideointelligenceV1Beta2TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest\nover all frames where OCR detected text appears."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TextFrame>>,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2TextSegment {
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
    pub struct GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than\none features."]
        #[serde(rename = "feature", default)]
        pub feature: Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be\n100 when fully processed."]
        #[serde(rename = "progressPercent", default)]
        pub progress_percent: Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than\none segments."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
    }
    impl GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LabelDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ShotChangeDetection , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ExplicitContentDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: TextDetection , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ObjectTracking , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest`\nsome videos may succeed and some may fail."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(rename = "explicitAnnotation", default)]
        pub explicit_annotation:
            Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation>,
        #[doc = "Label annotations on frame level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "frameLabelAnnotations", default)]
        pub frame_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>>,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(rename = "objectAnnotations", default)]
        pub object_annotations: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "Topical label annotations on video level or user specified segment level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "segmentLabelAnnotations", default)]
        pub segment_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>>,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(rename = "shotAnnotations", default)]
        pub shot_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>>,
        #[doc = "Topical label annotations on shot level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "shotLabelAnnotations", default)]
        pub shot_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>>,
        #[doc = "Speech transcription."]
        #[serde(rename = "speechTranscriptions", default)]
        pub speech_transcriptions:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2SpeechTranscription>>,
        #[doc = "OCR text detection and tracking.\nAnnotations for list of detected text snippets. Each will have list of\nframe information associated with it."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TextAnnotation>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults {
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
    pub struct GoogleCloudVideointelligenceV1Beta2VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the end of the segment (inclusive)."]
        #[serde(rename = "endTimeOffset", default)]
        pub end_time_offset: Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the start of the segment (inclusive)."]
        #[serde(rename = "startTimeOffset", default)]
        pub start_time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2VideoSegment {
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
    pub struct GoogleCloudVideointelligenceV1Beta2WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the end of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within\nthe audio. This field specifies which one of those speakers was detected to\nhave spoken this word. Value ranges from 1 up to diarization_speaker_count,\nand is only set if speaker diarization is enabled."]
        #[serde(rename = "speakerTag", default)]
        pub speaker_tag: Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the start of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(rename = "word", default)]
        pub word: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2WordInfo {
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
    pub struct GoogleCloudVideointelligenceV1Entity {
        #[doc = "Textual description, e.g. `Fixed-gear bicycle`."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1Entity {
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
    pub struct GoogleCloudVideointelligenceV1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1ExplicitContentFrame>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1ExplicitContentAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1ExplicitContentFrame {
        #[doc = "Likelihood of the pornography content.."]
        #[serde(rename = "pornographyLikelihood", default)]
        pub pornography_likelihood: Option<
            crate::schemas::GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1ExplicitContentFrame {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Likely."]
        Likely,
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , "UNLIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Unlikely , "POSSIBLE" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Possible , "LIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Likely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryLikely , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1LabelAnnotation {
        #[doc = "Common categories for the detected entity.\nE.g. when the label is `Terrier` the category is likely `dog`. And in some\ncases there might be more than one categories e.g. `Terrier` could also be\na `pet`."]
        #[serde(rename = "categoryEntities", default)]
        pub category_entities: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Entity>>,
        #[doc = "Detected entity."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelFrame>>,
        #[doc = "All video segments where a label was detected."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelSegment>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1LabelAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1LabelFrame {
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
    pub struct GoogleCloudVideointelligenceV1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1LabelSegment {
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
    pub struct GoogleCloudVideointelligenceV1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(rename = "bottom", default)]
        pub bottom: Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(rename = "left", default)]
        pub left: Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(rename = "right", default)]
        pub right: Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(rename = "top", default)]
        pub top: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1NormalizedBoundingBox {
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
    pub struct GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(rename = "vertices", default)]
        pub vertices: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1NormalizedVertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
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
    pub struct GoogleCloudVideointelligenceV1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1NormalizedVertex {
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
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingAnnotation {
        #[doc = "Object category's labeling confidence of this track."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears.\nNon-streaming batch mode: it may be one or multiple ObjectTrackingFrame\nmessages in frames.\nStreaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1ObjectTrackingFrame>>,
        #[doc = "Non-streaming batch mode ONLY.\nEach object track corresponds to one video segment where it appears."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "Streaming mode ONLY.\nIn streaming mode, we do not know the end time of a tracked object\nbefore it is completed. Hence, there is no VideoSegment info returned.\nInstead, we provide a unique identifiable integer track_id so that\nthe customers can correlate the results of the ongoing\nObjectTrackAnnotation of the same track_id over time."]
        #[serde(rename = "trackId", default)]
        #[serde(with = "crate::parsed_string")]
        pub track_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1ObjectTrackingAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(rename = "normalizedBoundingBox", default)]
        pub normalized_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1NormalizedBoundingBox>,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1ObjectTrackingFrame {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationProgress", default)]
        pub annotation_progress: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoProgress
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationResults", default)]
        pub annotation_results: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoResponse
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1Entity {
        #[doc = "Textual description, e.g. `Fixed-gear bicycle`."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1Entity {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame { # [ doc = "Likelihood of the pornography content.." ] # [ serde ( rename = "pornographyLikelihood" , default ) ] pub pornography_likelihood : Option < crate :: schemas :: GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood > , # [ doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location." ] # [ serde ( rename = "timeOffset" , default ) ] pub time_offset : Option < String > , }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Likely."]
        Likely,
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , "UNLIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "POSSIBLE" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Possible , "LIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Likely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation {
        #[doc = "Common categories for the detected entity.\nE.g. when the label is `Terrier` the category is likely `dog`. And in some\ncases there might be more than one categories e.g. `Terrier` could also be\na `pet`."]
        #[serde(rename = "categoryEntities", default)]
        pub category_entities:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>>,
        #[doc = "Detected entity."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelFrame>>,
        #[doc = "All video segments where a label was detected."]
        #[serde(rename = "segments", default)]
        pub segments:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelSegment>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1LabelFrame {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1LabelSegment {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(rename = "bottom", default)]
        pub bottom: Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(rename = "left", default)]
        pub left: Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(rename = "right", default)]
        pub right: Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(rename = "top", default)]
        pub top: Option<f32>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(rename = "vertices", default)]
        pub vertices:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation {
        #[doc = "Object category's labeling confidence of this track."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears.\nNon-streaming batch mode: it may be one or multiple ObjectTrackingFrame\nmessages in frames.\nStreaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame>>,
        #[doc = "Non-streaming batch mode ONLY.\nEach object track corresponds to one video segment where it appears."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
        #[doc = "Streaming mode ONLY.\nIn streaming mode, we do not know the end time of a tracked object\nbefore it is completed. Hence, there is no VideoSegment info returned.\nInstead, we provide a unique identifiable integer track_id so that\nthe customers can correlate the results of the ongoing\nObjectTrackAnnotation of the same track_id over time."]
        #[serde(rename = "trackId", default)]
        #[serde(with = "crate::parsed_string")]
        pub track_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(rename = "normalizedBoundingBox", default)]
        pub normalized_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox>,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(rename = "transcript", default)]
        pub transcript: Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word.\nNote: When `enable_speaker_diarization` is true, you will see all the words\nfrom the beginning of the audio."]
        #[serde(rename = "words", default)]
        pub words: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1WordInfo>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified\nin `max_alternatives`).  These alternatives are ordered in terms of\naccuracy, with the top (first) alternative being the most probable, as\nranked by the recognizer."]
        #[serde(rename = "alternatives", default)]
        pub alternatives: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the\nlanguage in this result. This language code was detected to have the most\nlikelihood of being spoken in the audio."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TextSegment>>,
        #[doc = "The detected text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1TextAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(rename = "rotatedBoundingBox", default)]
        pub rotated_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly>,
        #[doc = "Timestamp of this frame."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1TextFrame {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest\nover all frames where OCR detected text appears."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TextFrame>>,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1TextSegment {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than\none features."]
        #[serde(rename = "feature", default)]
        pub feature: Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be\n100 when fully processed."]
        #[serde(rename = "progressPercent", default)]
        pub progress_percent: Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than\none segments."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
    }
    impl GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LabelDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: TextDetection , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ObjectTracking , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest`\nsome videos may succeed and some may fail."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(rename = "explicitAnnotation", default)]
        pub explicit_annotation:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation>,
        #[doc = "Label annotations on frame level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "frameLabelAnnotations", default)]
        pub frame_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>>,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(rename = "objectAnnotations", default)]
        pub object_annotations: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
        #[doc = "Topical label annotations on video level or user specified segment level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "segmentLabelAnnotations", default)]
        pub segment_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>>,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(rename = "shotAnnotations", default)]
        pub shot_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>>,
        #[doc = "Topical label annotations on shot level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "shotLabelAnnotations", default)]
        pub shot_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>>,
        #[doc = "Speech transcription."]
        #[serde(rename = "speechTranscriptions", default)]
        pub speech_transcriptions:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription>>,
        #[doc = "OCR text detection and tracking.\nAnnotations for list of detected text snippets. Each will have list of\nframe information associated with it."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TextAnnotation>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the end of the segment (inclusive)."]
        #[serde(rename = "endTimeOffset", default)]
        pub end_time_offset: Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the start of the segment (inclusive)."]
        #[serde(rename = "startTimeOffset", default)]
        pub start_time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1VideoSegment {
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the end of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within\nthe audio. This field specifies which one of those speakers was detected to\nhave spoken this word. Value ranges from 1 up to diarization_speaker_count,\nand is only set if speaker diarization is enabled."]
        #[serde(rename = "speakerTag", default)]
        pub speaker_tag: Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the start of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(rename = "word", default)]
        pub word: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1WordInfo {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationProgress", default)]
        pub annotation_progress: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoProgress
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest { # [ doc = "Requested video annotation features." ] # [ serde ( rename = "features" , default ) ] pub features : Option < Vec < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems > > , # [ doc = "The video data bytes.\nIf unset, the input video(s) should be specified via `input_uri`.\nIf set, `input_uri` should be unset." ] # [ serde ( rename = "inputContent" , default ) ] pub input_content : Option < Vec < u8 > > , # [ doc = "Input video location. Currently, only\n[Google Cloud Storage](https://cloud.google.com/storage/) URIs are\nsupported, which must be specified in the following format:\n`gs://bucket-id/object-id` (other URI formats return\ngoogle.rpc.Code.INVALID_ARGUMENT). For more information, see\n[Request URIs](/storage/docs/reference-uris).\nA video URI may include wildcards in `object-id`, and thus identify\nmultiple videos. Supported wildcards: '*' to match 0 or more characters;\n'?' to match 1 character. If unset, the input video should be embedded\nin the request as `input_content`. If set, `input_content` should be unset." ] # [ serde ( rename = "inputUri" , default ) ] pub input_uri : Option < String > , # [ doc = "Optional cloud region where annotation should take place. Supported cloud\nregions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region\nis specified, a region will be determined based on video file location." ] # [ serde ( rename = "locationId" , default ) ] pub location_id : Option < String > , # [ doc = "Optional location where the output (in JSON format) should be stored.\nCurrently, only [Google Cloud Storage](https://cloud.google.com/storage/)\nURIs are supported, which must be specified in the following format:\n`gs://bucket-id/object-id` (other URI formats return\ngoogle.rpc.Code.INVALID_ARGUMENT). For more information, see\n[Request URIs](/storage/docs/reference-uris)." ] # [ serde ( rename = "outputUri" , default ) ] pub output_uri : Option < String > , # [ doc = "Additional video context and/or feature-specific parameters." ] # [ serde ( rename = "videoContext" , default ) ] pub video_context : Option < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1VideoContext > , }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems {}
    impl GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationResults", default)]
        pub annotation_results: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoResponse
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1Entity {
        #[doc = "Textual description, e.g. `Fixed-gear bicycle`."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1Entity {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig {
        #[doc = "Model to use for explicit content detection.\nSupported values: \"builtin/stable\" (the default if unset) and\n\"builtin/latest\"."]
        #[serde(rename = "model", default)]
        pub model: Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame { # [ doc = "Likelihood of the pornography content.." ] # [ serde ( rename = "pornographyLikelihood" , default ) ] pub pornography_likelihood : Option < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood > , # [ doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location." ] # [ serde ( rename = "timeOffset" , default ) ] pub time_offset : Option < String > , }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Likely."]
        Likely,
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , "UNLIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "POSSIBLE" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Possible , "LIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Likely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation {
        #[doc = "Common categories for the detected entity.\nE.g. when the label is `Terrier` the category is likely `dog`. And in some\ncases there might be more than one categories e.g. `Terrier` could also be\na `pet`."]
        #[serde(rename = "categoryEntities", default)]
        pub category_entities:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>>,
        #[doc = "Detected entity."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelFrame>>,
        #[doc = "All video segments where a label was detected."]
        #[serde(rename = "segments", default)]
        pub segments:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelSegment>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig { # [ doc = "The confidence threshold we perform filtering on the labels from\nframe-level detection. If not set, it is set to 0.4 by default. The valid\nrange for this threshold is [0.1, 0.9]. Any value set outside of this\nrange will be clipped.\nNote: for best results please follow the default threshold. We will update\nthe default threshold everytime when we release a new model." ] # [ serde ( rename = "frameConfidenceThreshold" , default ) ] pub frame_confidence_threshold : Option < f32 > , # [ doc = "What labels should be detected with LABEL_DETECTION, in addition to\nvideo-level labels or segment-level labels.\nIf unspecified, defaults to `SHOT_MODE`." ] # [ serde ( rename = "labelDetectionMode" , default ) ] pub label_detection_mode : Option < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode > , # [ doc = "Model to use for label detection.\nSupported values: \"builtin/stable\" (the default if unset) and\n\"builtin/latest\"." ] # [ serde ( rename = "model" , default ) ] pub model : Option < String > , # [ doc = "Whether the video has been shot from a stationary (i.e. non-moving) camera.\nWhen set to true, might improve detection accuracy for moving objects.\nShould be used with `SHOT_AND_FRAME_MODE` enabled." ] # [ serde ( rename = "stationaryCamera" , default ) ] pub stationary_camera : Option < bool > , # [ doc = "The confidence threshold we perform filtering on the labels from\nvideo-level and shot-level detections. If not set, it is set to 0.3 by\ndefault. The valid range for this threshold is [0.1, 0.9]. Any value set\noutside of this range will be clipped.\nNote: for best results please follow the default threshold. We will update\nthe default threshold everytime when we release a new model." ] # [ serde ( rename = "videoConfidenceThreshold" , default ) ] pub video_confidence_threshold : Option < f32 > , }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode {
        #[doc = "Unspecified."]
        LabelDetectionModeUnspecified,
        #[doc = "Detect shot-level labels."]
        ShotMode,
        #[doc = "Detect frame-level labels."]
        FrameMode,
        #[doc = "Detect both shot-level and frame-level labels."]
        ShotAndFrameMode,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: LabelDetectionModeUnspecified => "LABEL_DETECTION_MODE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotMode => "SHOT_MODE" , GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: FrameMode => "FRAME_MODE" , GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotAndFrameMode => "SHOT_AND_FRAME_MODE" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LABEL_DETECTION_MODE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: LabelDetectionModeUnspecified , "SHOT_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotMode , "FRAME_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: FrameMode , "SHOT_AND_FRAME_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotAndFrameMode , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1LabelFrame {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1LabelSegment {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(rename = "bottom", default)]
        pub bottom: Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(rename = "left", default)]
        pub left: Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(rename = "right", default)]
        pub right: Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(rename = "top", default)]
        pub top: Option<f32>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(rename = "vertices", default)]
        pub vertices:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation {
        #[doc = "Object category's labeling confidence of this track."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears.\nNon-streaming batch mode: it may be one or multiple ObjectTrackingFrame\nmessages in frames.\nStreaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame>>,
        #[doc = "Non-streaming batch mode ONLY.\nEach object track corresponds to one video segment where it appears."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
        #[doc = "Streaming mode ONLY.\nIn streaming mode, we do not know the end time of a tracked object\nbefore it is completed. Hence, there is no VideoSegment info returned.\nInstead, we provide a unique identifiable integer track_id so that\nthe customers can correlate the results of the ongoing\nObjectTrackAnnotation of the same track_id over time."]
        #[serde(rename = "trackId", default)]
        #[serde(with = "crate::parsed_string")]
        pub track_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig {
        #[doc = "Model to use for object tracking.\nSupported values: \"builtin/stable\" (the default if unset) and\n\"builtin/latest\"."]
        #[serde(rename = "model", default)]
        pub model: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(rename = "normalizedBoundingBox", default)]
        pub normalized_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox>,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig {
        #[doc = "Model to use for shot change detection.\nSupported values: \"builtin/stable\" (the default if unset) and\n\"builtin/latest\"."]
        #[serde(rename = "model", default)]
        pub model: Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechContext {
        #[doc = "*Optional* A list of strings containing words and phrases \"hints\" so that\nthe speech recognition is more likely to recognize them. This can be used\nto improve the accuracy for specific words and phrases, for example, if\nspecific commands are typically spoken by the user. This can also be used\nto add additional words to the vocabulary of the recognizer. See\n[usage limits](https://cloud.google.com/speech/limits#content)."]
        #[serde(rename = "phrases", default)]
        pub phrases: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1SpeechContext {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(rename = "transcript", default)]
        pub transcript: Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word.\nNote: When `enable_speaker_diarization` is true, you will see all the words\nfrom the beginning of the audio."]
        #[serde(rename = "words", default)]
        pub words: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1WordInfo>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified\nin `max_alternatives`).  These alternatives are ordered in terms of\naccuracy, with the top (first) alternative being the most probable, as\nranked by the recognizer."]
        #[serde(rename = "alternatives", default)]
        pub alternatives: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the\nlanguage in this result. This language code was detected to have the most\nlikelihood of being spoken in the audio."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig {
        #[doc = "*Optional* For file formats, such as MXF or MKV, supporting multiple audio\ntracks, specify up to two tracks. Default: track 0."]
        #[serde(rename = "audioTracks", default)]
        pub audio_tracks: Option<Vec<i32>>,
        #[doc = "*Optional*\nIf set, specifies the estimated number of speakers in the conversation.\nIf not set, defaults to '2'.\nIgnored unless enable_speaker_diarization is set to true."]
        #[serde(rename = "diarizationSpeakerCount", default)]
        pub diarization_speaker_count: Option<i32>,
        #[doc = "*Optional* If 'true', adds punctuation to recognition result hypotheses.\nThis feature is only available in select languages. Setting this for\nrequests in other languages has no effect at all. The default 'false' value\ndoes not add punctuation to result hypotheses. NOTE: \"This is currently\noffered as an experimental service, complimentary to all users. In the\nfuture this may be exclusively available as a premium feature.\""]
        #[serde(rename = "enableAutomaticPunctuation", default)]
        pub enable_automatic_punctuation: Option<bool>,
        #[doc = "*Optional* If 'true', enables speaker detection for each recognized word in\nthe top alternative of the recognition result using a speaker_tag provided\nin the WordInfo.\nNote: When this is true, we send all the words from the beginning of the\naudio for the top alternative in every consecutive responses.\nThis is done in order to improve our speaker tags as our models learn to\nidentify the speakers in the conversation over time."]
        #[serde(rename = "enableSpeakerDiarization", default)]
        pub enable_speaker_diarization: Option<bool>,
        #[doc = "*Optional* If `true`, the top result includes a list of words and the\nconfidence for those words. If `false`, no word-level confidence\ninformation is returned. The default is `false`."]
        #[serde(rename = "enableWordConfidence", default)]
        pub enable_word_confidence: Option<bool>,
        #[doc = "*Optional* If set to `true`, the server will attempt to filter out\nprofanities, replacing all but the initial character in each filtered word\nwith asterisks, e.g. \"f***\". If set to `false` or omitted, profanities\nwon't be filtered out."]
        #[serde(rename = "filterProfanity", default)]
        pub filter_profanity: Option<bool>,
        #[doc = "*Required* The language of the supplied audio as a\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag.\nExample: \"en-US\".\nSee [Language Support](https://cloud.google.com/speech/docs/languages)\nfor a list of the currently supported language codes."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
        #[doc = "*Optional* Maximum number of recognition hypotheses to be returned.\nSpecifically, the maximum number of `SpeechRecognitionAlternative` messages\nwithin each `SpeechTranscription`. The server may return fewer than\n`max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will\nreturn a maximum of one. If omitted, will return a maximum of one."]
        #[serde(rename = "maxAlternatives", default)]
        pub max_alternatives: Option<i32>,
        #[doc = "*Optional* A means to provide context to assist the speech recognition."]
        #[serde(rename = "speechContexts", default)]
        pub speech_contexts:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechContext>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextSegment>>,
        #[doc = "The detected text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1TextAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig {
        #[doc = "Language hint can be specified if the language to be detected is known a\npriori. It can increase the accuracy of the detection. Language hint must\nbe language code in BCP-47 format.\n\nAutomatic language detection is performed if no hint is provided."]
        #[serde(rename = "languageHints", default)]
        pub language_hints: Option<Vec<String>>,
        #[doc = "Model to use for text detection.\nSupported values: \"builtin/stable\" (the default if unset) and\n\"builtin/latest\"."]
        #[serde(rename = "model", default)]
        pub model: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(rename = "rotatedBoundingBox", default)]
        pub rotated_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly>,
        #[doc = "Timestamp of this frame."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1TextFrame {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest\nover all frames where OCR detected text appears."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextFrame>>,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1TextSegment {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than\none features."]
        #[serde(rename = "feature", default)]
        pub feature: Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be\n100 when fully processed."]
        #[serde(rename = "progressPercent", default)]
        pub progress_percent: Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than\none segments."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LabelDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: TextDetection , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ObjectTracking , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest`\nsome videos may succeed and some may fail."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(rename = "explicitAnnotation", default)]
        pub explicit_annotation:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation>,
        #[doc = "Label annotations on frame level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "frameLabelAnnotations", default)]
        pub frame_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>>,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(rename = "objectAnnotations", default)]
        pub object_annotations: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
        #[doc = "Topical label annotations on video level or user specified segment level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "segmentLabelAnnotations", default)]
        pub segment_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>>,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(rename = "shotAnnotations", default)]
        pub shot_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>>,
        #[doc = "Topical label annotations on shot level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "shotLabelAnnotations", default)]
        pub shot_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>>,
        #[doc = "Speech transcription."]
        #[serde(rename = "speechTranscriptions", default)]
        pub speech_transcriptions:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription>>,
        #[doc = "OCR text detection and tracking.\nAnnotations for list of detected text snippets. Each will have list of\nframe information associated with it."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextAnnotation>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoContext {
        #[doc = "Config for EXPLICIT_CONTENT_DETECTION."]
        #[serde(rename = "explicitContentDetectionConfig", default)]
        pub explicit_content_detection_config: Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig,
        >,
        #[doc = "Config for LABEL_DETECTION."]
        #[serde(rename = "labelDetectionConfig", default)]
        pub label_detection_config:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig>,
        #[doc = "Config for OBJECT_TRACKING."]
        #[serde(rename = "objectTrackingConfig", default)]
        pub object_tracking_config:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig>,
        #[doc = "Video segments to annotate. The segments may overlap and are not required\nto be contiguous or span the whole video. If unspecified, each video is\ntreated as a single segment."]
        #[serde(rename = "segments", default)]
        pub segments:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>>,
        #[doc = "Config for SHOT_CHANGE_DETECTION."]
        #[serde(rename = "shotChangeDetectionConfig", default)]
        pub shot_change_detection_config:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig>,
        #[doc = "Config for SPEECH_TRANSCRIPTION."]
        #[serde(rename = "speechTranscriptionConfig", default)]
        pub speech_transcription_config:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig>,
        #[doc = "Config for TEXT_DETECTION."]
        #[serde(rename = "textDetectionConfig", default)]
        pub text_detection_config:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1VideoContext {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the end of the segment (inclusive)."]
        #[serde(rename = "endTimeOffset", default)]
        pub end_time_offset: Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the start of the segment (inclusive)."]
        #[serde(rename = "startTimeOffset", default)]
        pub start_time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1VideoSegment {
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the end of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within\nthe audio. This field specifies which one of those speakers was detected to\nhave spoken this word. Value ranges from 1 up to diarization_speaker_count,\nand is only set if speaker diarization is enabled."]
        #[serde(rename = "speakerTag", default)]
        pub speaker_tag: Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the start of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(rename = "word", default)]
        pub word: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1WordInfo {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationProgress", default)]
        pub annotation_progress: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoProgress
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(rename = "annotationResults", default)]
        pub annotation_results: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoResponse
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute {
        #[doc = "Detected attribute confidence. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "The name of the attribute, i.e. glasses, dark_glasses, mouth_open etc.\nA full list of supported type names will be provided in the document."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Text value of the detection result. For example, the value for \"HairColor\"\ncan be \"black\", \"blonde\", etc."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1Entity {
        #[doc = "Textual description, e.g. `Fixed-gear bicycle`."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1Entity {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame { # [ doc = "Likelihood of the pornography content.." ] # [ serde ( rename = "pornographyLikelihood" , default ) ] pub pornography_likelihood : Option < crate :: schemas :: GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood > , # [ doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location." ] # [ serde ( rename = "timeOffset" , default ) ] pub time_offset : Option < String > , }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Likely."]
        Likely,
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , "UNLIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "POSSIBLE" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Possible , "LIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Likely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation {
        #[doc = "Common categories for the detected entity.\nE.g. when the label is `Terrier` the category is likely `dog`. And in some\ncases there might be more than one categories e.g. `Terrier` could also be\na `pet`."]
        #[serde(rename = "categoryEntities", default)]
        pub category_entities:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>>,
        #[doc = "Detected entity."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelFrame>>,
        #[doc = "All video segments where a label was detected."]
        #[serde(rename = "segments", default)]
        pub segments:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelSegment>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the\nvideo frame for this location."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1LabelFrame {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1LabelSegment {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation {
        #[doc = "Entity category information to specify the logo class that all the logo\ntracks within this LogoRecognitionAnnotation are recognized as."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>,
        #[doc = "All video segments where the recognized logo appears. There might be\nmultiple instances of the same logo class appearing in one VideoSegment."]
        #[serde(rename = "segments", default)]
        pub segments:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>>,
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds\nto one logo instance appearing in consecutive frames."]
        #[serde(rename = "tracks", default)]
        pub tracks: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Track>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(rename = "bottom", default)]
        pub bottom: Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(rename = "left", default)]
        pub left: Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(rename = "right", default)]
        pub right: Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(rename = "top", default)]
        pub top: Option<f32>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(rename = "vertices", default)]
        pub vertices:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation {
        #[doc = "Object category's labeling confidence of this track."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(rename = "entity", default)]
        pub entity: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears.\nNon-streaming batch mode: it may be one or multiple ObjectTrackingFrame\nmessages in frames.\nStreaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(rename = "frames", default)]
        pub frames:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame>>,
        #[doc = "Non-streaming batch mode ONLY.\nEach object track corresponds to one video segment where it appears."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        #[doc = "Streaming mode ONLY.\nIn streaming mode, we do not know the end time of a tracked object\nbefore it is completed. Hence, there is no VideoSegment info returned.\nInstead, we provide a unique identifiable integer track_id so that\nthe customers can correlate the results of the ongoing\nObjectTrackAnnotation of the same track_id over time."]
        #[serde(rename = "trackId", default)]
        #[serde(with = "crate::parsed_string")]
        pub track_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(rename = "normalizedBoundingBox", default)]
        pub normalized_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox>,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(rename = "transcript", default)]
        pub transcript: Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word.\nNote: When `enable_speaker_diarization` is true, you will see all the words\nfrom the beginning of the audio."]
        #[serde(rename = "words", default)]
        pub words: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1WordInfo>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified\nin `max_alternatives`).  These alternatives are ordered in terms of\naccuracy, with the top (first) alternative being the most probable, as\nranked by the recognizer."]
        #[serde(rename = "alternatives", default)]
        pub alternatives: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the\nlanguage in this result. This language code was detected to have the most\nlikelihood of being spoken in the audio."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1StreamingAnnotateVideoResponse {
        #[doc = "Streaming annotation results."]
        #[serde(rename = "annotationResults", default)]
        pub annotation_results: Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults,
        >,
        #[doc = "GCS URI that stores annotation results of one streaming session.\nIt is a directory that can hold multiple files in JSON format.\nExample uri format:\ngs://bucket_id/object_id/cloud_project_name-session_id"]
        #[serde(rename = "annotationResultsUri", default)]
        pub annotation_results_uri: Option<String>,
        #[doc = "If set, returns a google.rpc.Status message that\nspecifies the error for the operation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1StreamingAnnotateVideoResponse
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults {
        #[doc = "Explicit content annotation results."]
        #[serde(rename = "explicitAnnotation", default)]
        pub explicit_annotation:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation>,
        #[doc = "Label annotation results."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>>,
        #[doc = "Object tracking results."]
        #[serde(rename = "objectAnnotations", default)]
        pub object_annotations: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Shot annotation results. Each shot is represented as a video segment."]
        #[serde(rename = "shotAnnotations", default)]
        pub shot_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TextSegment>>,
        #[doc = "The detected text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1TextAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(rename = "rotatedBoundingBox", default)]
        pub rotated_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly>,
        #[doc = "Timestamp of this frame."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1TextFrame {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest\nover all frames where OCR detected text appears."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TextFrame>>,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1TextSegment {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1TimestampedObject {
        #[doc = "Optional. The attributes of the object in the bounding box."]
        #[serde(rename = "attributes", default)]
        pub attributes:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute>>,
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        #[serde(rename = "normalizedBoundingBox", default)]
        pub normalized_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox>,
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the video frame for this object."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1TimestampedObject {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1Track {
        #[doc = "Optional. Attributes in the track level."]
        #[serde(rename = "attributes", default)]
        pub attributes:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute>>,
        #[doc = "Optional. The confidence score of the tracked object."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Video segment of a track."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        #[doc = "The object with timestamp and attributes per frame in the track."]
        #[serde(rename = "timestampedObjects", default)]
        pub timestamped_objects:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TimestampedObject>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1Track {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than\none features."]
        #[serde(rename = "feature", default)]
        pub feature: Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be\n100 when fully processed."]
        #[serde(rename = "progressPercent", default)]
        pub progress_percent: Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than\none segments."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
    }
    impl GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LogoRecognition => "LOGO_RECOGNITION" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LabelDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: TextDetection , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ObjectTracking , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LogoRecognition , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest`\nsome videos may succeed and some may fail."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(rename = "explicitAnnotation", default)]
        pub explicit_annotation:
            Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation>,
        #[doc = "Label annotations on frame level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "frameLabelAnnotations", default)]
        pub frame_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>>,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        #[serde(rename = "logoRecognitionAnnotations", default)]
        pub logo_recognition_annotations: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation>,
        >,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(rename = "objectAnnotations", default)]
        pub object_annotations: Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        #[doc = "Topical label annotations on video level or user specified segment level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "segmentLabelAnnotations", default)]
        pub segment_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>>,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(rename = "shotAnnotations", default)]
        pub shot_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>>,
        #[doc = "Topical label annotations on shot level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "shotLabelAnnotations", default)]
        pub shot_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>>,
        #[doc = "Speech transcription."]
        #[serde(rename = "speechTranscriptions", default)]
        pub speech_transcriptions:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription>>,
        #[doc = "OCR text detection and tracking.\nAnnotations for list of detected text snippets. Each will have list of\nframe information associated with it."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TextAnnotation>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the end of the segment (inclusive)."]
        #[serde(rename = "endTimeOffset", default)]
        pub end_time_offset: Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the start of the segment (inclusive)."]
        #[serde(rename = "startTimeOffset", default)]
        pub start_time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1VideoSegment {
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the end of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within\nthe audio. This field specifies which one of those speakers was detected to\nhave spoken this word. Value ranges from 1 up to diarization_speaker_count,\nand is only set if speaker diarization is enabled."]
        #[serde(rename = "speakerTag", default)]
        pub speaker_tag: Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the start of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(rename = "word", default)]
        pub word: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1WordInfo {
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
    pub struct GoogleCloudVideointelligenceV1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(rename = "transcript", default)]
        pub transcript: Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word.\nNote: When `enable_speaker_diarization` is true, you will see all the words\nfrom the beginning of the audio."]
        #[serde(rename = "words", default)]
        pub words: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1WordInfo>>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1SpeechRecognitionAlternative
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
    pub struct GoogleCloudVideointelligenceV1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified\nin `max_alternatives`).  These alternatives are ordered in terms of\naccuracy, with the top (first) alternative being the most probable, as\nranked by the recognizer."]
        #[serde(rename = "alternatives", default)]
        pub alternatives:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1SpeechRecognitionAlternative>>,
        #[doc = "Output only. The\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the\nlanguage in this result. This language code was detected to have the most\nlikelihood of being spoken in the audio."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1SpeechTranscription {
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
    pub struct GoogleCloudVideointelligenceV1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1TextSegment>>,
        #[doc = "The detected text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1TextAnnotation {
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
    pub struct GoogleCloudVideointelligenceV1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(rename = "rotatedBoundingBox", default)]
        pub rotated_bounding_box:
            Option<crate::schemas::GoogleCloudVideointelligenceV1NormalizedBoundingPoly>,
        #[doc = "Timestamp of this frame."]
        #[serde(rename = "timeOffset", default)]
        pub time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1TextFrame {
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
    pub struct GoogleCloudVideointelligenceV1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest\nover all frames where OCR detected text appears."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(rename = "frames", default)]
        pub frames: Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1TextFrame>>,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1TextSegment {
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
    pub struct GoogleCloudVideointelligenceV1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than\none features."]
        #[serde(rename = "feature", default)]
        pub feature:
            Option<crate::schemas::GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature>,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be\n100 when fully processed."]
        #[serde(rename = "progressPercent", default)]
        pub progress_percent: Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than\none segments."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1VideoAnnotationProgress {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
    }
    impl GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LabelDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ShotChangeDetection , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ExplicitContentDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: TextDetection , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ObjectTracking , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest`\nsome videos may succeed and some may fail."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(rename = "explicitAnnotation", default)]
        pub explicit_annotation:
            Option<crate::schemas::GoogleCloudVideointelligenceV1ExplicitContentAnnotation>,
        #[doc = "Label annotations on frame level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "frameLabelAnnotations", default)]
        pub frame_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>>,
        #[doc = "Video file location in\n[Google Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(rename = "inputUri", default)]
        pub input_uri: Option<String>,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(rename = "objectAnnotations", default)]
        pub object_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1ObjectTrackingAnnotation>>,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(rename = "segment", default)]
        pub segment: Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "Topical label annotations on video level or user specified segment level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "segmentLabelAnnotations", default)]
        pub segment_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>>,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(rename = "shotAnnotations", default)]
        pub shot_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>>,
        #[doc = "Topical label annotations on shot level.\nThere is exactly one element for each unique label."]
        #[serde(rename = "shotLabelAnnotations", default)]
        pub shot_label_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>>,
        #[doc = "Speech transcription."]
        #[serde(rename = "speechTranscriptions", default)]
        pub speech_transcriptions:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1SpeechTranscription>>,
        #[doc = "OCR text detection and tracking.\nAnnotations for list of detected text snippets. Each will have list of\nframe information associated with it."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1TextAnnotation>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1VideoAnnotationResults {
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
    pub struct GoogleCloudVideointelligenceV1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the end of the segment (inclusive)."]
        #[serde(rename = "endTimeOffset", default)]
        pub end_time_offset: Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video,\ncorresponding to the start of the segment (inclusive)."]
        #[serde(rename = "startTimeOffset", default)]
        pub start_time_offset: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1VideoSegment {
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
    pub struct GoogleCloudVideointelligenceV1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. This field is set only for the top alternative.\nThis field is not guaranteed to be accurate and users should not rely on it\nto be always provided.\nThe default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(rename = "confidence", default)]
        pub confidence: Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the end of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within\nthe audio. This field specifies which one of those speakers was detected to\nhave spoken this word. Value ranges from 1 up to diarization_speaker_count,\nand is only set if speaker diarization is enabled."]
        #[serde(rename = "speakerTag", default)]
        pub speaker_tag: Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and\ncorresponding to the start of the spoken word. This field is only set if\n`enable_word_time_offsets=true` and only in the top hypothesis. This is an\nexperimental feature and the accuracy of the time offset can vary."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(rename = "word", default)]
        pub word: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVideointelligenceV1WordInfo {
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
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for GoogleLongrunningOperation {
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
    pub struct GoogleRpcStatus {
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
    impl ::field_selector::FieldSelector for GoogleRpcStatus {
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
    #[doc = "Actions that can be performed on the videos resource"]
    pub fn videos(&self) -> crate::videos::VideosActions<A> {
        crate::videos::VideosActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod videos {
    pub mod params {}
    pub struct VideosActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> VideosActions<'a, A> {
        #[doc = "Performs asynchronous video annotation. Progress and results can be\nretrieved through the `google.longrunning.Operations` interface.\n`Operation.metadata` contains `AnnotateVideoProgress` (progress).\n`Operation.response` contains `AnnotateVideoResponse` (results)."]
        pub fn annotate(
            &self,
            request: crate::schemas::GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest,
        ) -> AnnotateRequestBuilder<A> {
            AnnotateRequestBuilder {
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
    pub struct AnnotateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest,
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
    impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
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
            let mut output = "https://videointelligence.googleapis.com/".to_owned();
            output.push_str("v1p2beta1/videos:annotate");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
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
