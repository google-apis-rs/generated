#![doc = "# Resources and Methods\n    * [achievement_definitions](resources/achievement_definitions/struct.AchievementDefinitionsActions.html)\n      * [*list*](resources/achievement_definitions/struct.ListRequestBuilder.html)\n    * [achievements](resources/achievements/struct.AchievementsActions.html)\n      * [*increment*](resources/achievements/struct.IncrementRequestBuilder.html), [*list*](resources/achievements/struct.ListRequestBuilder.html), [*reveal*](resources/achievements/struct.RevealRequestBuilder.html), [*setStepsAtLeast*](resources/achievements/struct.SetStepsAtLeastRequestBuilder.html), [*unlock*](resources/achievements/struct.UnlockRequestBuilder.html), [*updateMultiple*](resources/achievements/struct.UpdateMultipleRequestBuilder.html)\n    * [applications](resources/applications/struct.ApplicationsActions.html)\n      * [*get*](resources/applications/struct.GetRequestBuilder.html), [*getEndPoint*](resources/applications/struct.GetEndPointRequestBuilder.html), [*played*](resources/applications/struct.PlayedRequestBuilder.html), [*verify*](resources/applications/struct.VerifyRequestBuilder.html)\n    * [events](resources/events/struct.EventsActions.html)\n      * [*listByPlayer*](resources/events/struct.ListByPlayerRequestBuilder.html), [*listDefinitions*](resources/events/struct.ListDefinitionsRequestBuilder.html), [*record*](resources/events/struct.RecordRequestBuilder.html)\n    * [leaderboards](resources/leaderboards/struct.LeaderboardsActions.html)\n      * [*get*](resources/leaderboards/struct.GetRequestBuilder.html), [*list*](resources/leaderboards/struct.ListRequestBuilder.html)\n    * [metagame](resources/metagame/struct.MetagameActions.html)\n      * [*getMetagameConfig*](resources/metagame/struct.GetMetagameConfigRequestBuilder.html), [*listCategoriesByPlayer*](resources/metagame/struct.ListCategoriesByPlayerRequestBuilder.html)\n    * [players](resources/players/struct.PlayersActions.html)\n      * [*get*](resources/players/struct.GetRequestBuilder.html), [*list*](resources/players/struct.ListRequestBuilder.html)\n    * [revisions](resources/revisions/struct.RevisionsActions.html)\n      * [*check*](resources/revisions/struct.CheckRequestBuilder.html)\n    * [scores](resources/scores/struct.ScoresActions.html)\n      * [*get*](resources/scores/struct.GetRequestBuilder.html), [*list*](resources/scores/struct.ListRequestBuilder.html), [*listWindow*](resources/scores/struct.ListWindowRequestBuilder.html), [*submit*](resources/scores/struct.SubmitRequestBuilder.html), [*submitMultiple*](resources/scores/struct.SubmitMultipleRequestBuilder.html)\n    * [snapshots](resources/snapshots/struct.SnapshotsActions.html)\n      * [*get*](resources/snapshots/struct.GetRequestBuilder.html), [*list*](resources/snapshots/struct.ListRequestBuilder.html)\n    * [snapshots_extended](resources/snapshots_extended/struct.SnapshotsExtendedActions.html)\n      * [*resolveSnapshotHead*](resources/snapshots_extended/struct.ResolveSnapshotHeadRequestBuilder.html)\n    * [stats](resources/stats/struct.StatsActions.html)\n      * [*get*](resources/stats/struct.GetRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, create, and delete its own configuration data in your Google Drive\n\n`https://www.googleapis.com/auth/drive.appdata`"]
    pub const DRIVE_APPDATA: &str = "https://www.googleapis.com/auth/drive.appdata";
    #[doc = "Create, edit, and delete your Google Play Games activity\n\n`https://www.googleapis.com/auth/games`"]
    pub const GAMES: &str = "https://www.googleapis.com/auth/games";
}
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
    pub struct AchievementDefinition {
        #[doc = "The type of the achievement."]
        #[serde(
            rename = "achievementType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub achievement_type:
            ::std::option::Option<crate::schemas::AchievementDefinitionAchievementType>,
        #[doc = "The description of the achievement."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Experience points which will be earned when unlocking this achievement."]
        #[serde(
            rename = "experiencePoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub experience_points: ::std::option::Option<i64>,
        #[doc = "The total steps for an incremental achievement as a string."]
        #[serde(
            rename = "formattedTotalSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_total_steps: ::std::option::Option<String>,
        #[doc = "The ID of the achievement."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The initial state of the achievement."]
        #[serde(
            rename = "initialState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_state: ::std::option::Option<crate::schemas::AchievementDefinitionInitialState>,
        #[doc = "Indicates whether the revealed icon image being returned is a default image, or is provided by the game."]
        #[serde(
            rename = "isRevealedIconUrlDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_revealed_icon_url_default: ::std::option::Option<bool>,
        #[doc = "Indicates whether the unlocked icon image being returned is a default image, or is game-provided."]
        #[serde(
            rename = "isUnlockedIconUrlDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_unlocked_icon_url_default: ::std::option::Option<bool>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinition`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the achievement."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The image URL for the revealed achievement icon."]
        #[serde(
            rename = "revealedIconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revealed_icon_url: ::std::option::Option<String>,
        #[doc = "The total steps for an incremental achievement."]
        #[serde(
            rename = "totalSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_steps: ::std::option::Option<i32>,
        #[doc = "The image URL for the unlocked achievement icon."]
        #[serde(
            rename = "unlockedIconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unlocked_icon_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AchievementDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementDefinition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AchievementDefinitionAchievementType {
        #[doc = "Safe default, don't use."]
        AchievementTypeUnspecified,
        #[doc = "Achievement is incremental."]
        Incremental,
        #[doc = "Achievement is either locked or unlocked."]
        Standard,
    }
    impl AchievementDefinitionAchievementType {
        pub fn as_str(self) -> &'static str {
            match self {
                AchievementDefinitionAchievementType::AchievementTypeUnspecified => {
                    "ACHIEVEMENT_TYPE_UNSPECIFIED"
                }
                AchievementDefinitionAchievementType::Incremental => "INCREMENTAL",
                AchievementDefinitionAchievementType::Standard => "STANDARD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AchievementDefinitionAchievementType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AchievementDefinitionAchievementType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AchievementDefinitionAchievementType, ()> {
            Ok(match s {
                "ACHIEVEMENT_TYPE_UNSPECIFIED" => {
                    AchievementDefinitionAchievementType::AchievementTypeUnspecified
                }
                "INCREMENTAL" => AchievementDefinitionAchievementType::Incremental,
                "STANDARD" => AchievementDefinitionAchievementType::Standard,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AchievementDefinitionAchievementType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AchievementDefinitionAchievementType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AchievementDefinitionAchievementType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACHIEVEMENT_TYPE_UNSPECIFIED" => {
                    AchievementDefinitionAchievementType::AchievementTypeUnspecified
                }
                "INCREMENTAL" => AchievementDefinitionAchievementType::Incremental,
                "STANDARD" => AchievementDefinitionAchievementType::Standard,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AchievementDefinitionAchievementType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementDefinitionAchievementType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AchievementDefinitionInitialState {
        #[doc = "Achievement is hidden."]
        Hidden,
        #[doc = "Safe default, don't use."]
        InitialAchievementStateUnspecified,
        #[doc = "Achievement is revealed."]
        Revealed,
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl AchievementDefinitionInitialState {
        pub fn as_str(self) -> &'static str {
            match self {
                AchievementDefinitionInitialState::Hidden => "HIDDEN",
                AchievementDefinitionInitialState::InitialAchievementStateUnspecified => {
                    "INITIAL_ACHIEVEMENT_STATE_UNSPECIFIED"
                }
                AchievementDefinitionInitialState::Revealed => "REVEALED",
                AchievementDefinitionInitialState::Unlocked => "UNLOCKED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AchievementDefinitionInitialState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AchievementDefinitionInitialState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AchievementDefinitionInitialState, ()> {
            Ok(match s {
                "HIDDEN" => AchievementDefinitionInitialState::Hidden,
                "INITIAL_ACHIEVEMENT_STATE_UNSPECIFIED" => {
                    AchievementDefinitionInitialState::InitialAchievementStateUnspecified
                }
                "REVEALED" => AchievementDefinitionInitialState::Revealed,
                "UNLOCKED" => AchievementDefinitionInitialState::Unlocked,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AchievementDefinitionInitialState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AchievementDefinitionInitialState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AchievementDefinitionInitialState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HIDDEN" => AchievementDefinitionInitialState::Hidden,
                "INITIAL_ACHIEVEMENT_STATE_UNSPECIFIED" => {
                    AchievementDefinitionInitialState::InitialAchievementStateUnspecified
                }
                "REVEALED" => AchievementDefinitionInitialState::Revealed,
                "UNLOCKED" => AchievementDefinitionInitialState::Unlocked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AchievementDefinitionInitialState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementDefinitionInitialState {
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
    pub struct AchievementDefinitionsListResponse {
        #[doc = "The achievement definitions."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::AchievementDefinition>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinitionsListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token corresponding to the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AchievementDefinitionsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementDefinitionsListResponse {
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
    pub struct AchievementIncrementResponse {
        #[doc = "The current steps recorded for this incremental achievement."]
        #[serde(
            rename = "currentSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_steps: ::std::option::Option<i32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementIncrementResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Whether the current steps for the achievement has reached the number of steps required to unlock."]
        #[serde(
            rename = "newlyUnlocked",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub newly_unlocked: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AchievementIncrementResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementIncrementResponse {
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
    pub struct AchievementRevealResponse {
        #[doc = "The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked."]
        #[serde(
            rename = "currentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_state:
            ::std::option::Option<crate::schemas::AchievementRevealResponseCurrentState>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementRevealResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AchievementRevealResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementRevealResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AchievementRevealResponseCurrentState {
        #[doc = "Safe default, don't use."]
        RevealAchievementStateUnspecified,
        #[doc = "Achievement is revealed."]
        Revealed,
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl AchievementRevealResponseCurrentState {
        pub fn as_str(self) -> &'static str {
            match self {
                AchievementRevealResponseCurrentState::RevealAchievementStateUnspecified => {
                    "REVEAL_ACHIEVEMENT_STATE_UNSPECIFIED"
                }
                AchievementRevealResponseCurrentState::Revealed => "REVEALED",
                AchievementRevealResponseCurrentState::Unlocked => "UNLOCKED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AchievementRevealResponseCurrentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AchievementRevealResponseCurrentState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AchievementRevealResponseCurrentState, ()> {
            Ok(match s {
                "REVEAL_ACHIEVEMENT_STATE_UNSPECIFIED" => {
                    AchievementRevealResponseCurrentState::RevealAchievementStateUnspecified
                }
                "REVEALED" => AchievementRevealResponseCurrentState::Revealed,
                "UNLOCKED" => AchievementRevealResponseCurrentState::Unlocked,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AchievementRevealResponseCurrentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AchievementRevealResponseCurrentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AchievementRevealResponseCurrentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REVEAL_ACHIEVEMENT_STATE_UNSPECIFIED" => {
                    AchievementRevealResponseCurrentState::RevealAchievementStateUnspecified
                }
                "REVEALED" => AchievementRevealResponseCurrentState::Revealed,
                "UNLOCKED" => AchievementRevealResponseCurrentState::Unlocked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AchievementRevealResponseCurrentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementRevealResponseCurrentState {
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
    pub struct AchievementSetStepsAtLeastResponse {
        #[doc = "The current steps recorded for this incremental achievement."]
        #[serde(
            rename = "currentSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_steps: ::std::option::Option<i32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementSetStepsAtLeastResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Whether the current steps for the achievement has reached the number of steps required to unlock."]
        #[serde(
            rename = "newlyUnlocked",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub newly_unlocked: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AchievementSetStepsAtLeastResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementSetStepsAtLeastResponse {
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
    pub struct AchievementUnlockResponse {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUnlockResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player)."]
        #[serde(
            rename = "newlyUnlocked",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub newly_unlocked: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AchievementUnlockResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUnlockResponse {
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
    pub struct AchievementUpdateMultipleRequest {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleRequest`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The individual achievement update requests."]
        #[serde(
            rename = "updates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updates: ::std::option::Option<Vec<crate::schemas::AchievementUpdateRequest>>,
    }
    impl ::google_field_selector::FieldSelector for AchievementUpdateMultipleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUpdateMultipleRequest {
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
    pub struct AchievementUpdateMultipleResponse {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The updated state of the achievements."]
        #[serde(
            rename = "updatedAchievements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_achievements:
            ::std::option::Option<Vec<crate::schemas::AchievementUpdateResponse>>,
    }
    impl ::google_field_selector::FieldSelector for AchievementUpdateMultipleResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUpdateMultipleResponse {
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
    pub struct AchievementUpdateRequest {
        #[doc = "The achievement this update is being applied to."]
        #[serde(
            rename = "achievementId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub achievement_id: ::std::option::Option<String>,
        #[doc = "The payload if an update of type `INCREMENT` was requested for the achievement."]
        #[serde(
            rename = "incrementPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub increment_payload: ::std::option::Option<crate::schemas::GamesAchievementIncrement>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateRequest`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The payload if an update of type `SET_STEPS_AT_LEAST` was requested for the achievement."]
        #[serde(
            rename = "setStepsAtLeastPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub set_steps_at_least_payload:
            ::std::option::Option<crate::schemas::GamesAchievementSetStepsAtLeast>,
        #[doc = "The type of update being applied."]
        #[serde(
            rename = "updateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_type: ::std::option::Option<crate::schemas::AchievementUpdateRequestUpdateType>,
    }
    impl ::google_field_selector::FieldSelector for AchievementUpdateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUpdateRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AchievementUpdateRequestUpdateType {
        #[doc = "Safe default, don't use."]
        AchievementUpdateTypeUnspecified,
        #[doc = "Achievement is incremented."]
        Increment,
        #[doc = "Achievement is revealed."]
        Reveal,
        #[doc = "Achievement progress is set to at least the passed value."]
        SetStepsAtLeast,
        #[doc = "Achievement is unlocked."]
        Unlock,
    }
    impl AchievementUpdateRequestUpdateType {
        pub fn as_str(self) -> &'static str {
            match self {
                AchievementUpdateRequestUpdateType::AchievementUpdateTypeUnspecified => {
                    "ACHIEVEMENT_UPDATE_TYPE_UNSPECIFIED"
                }
                AchievementUpdateRequestUpdateType::Increment => "INCREMENT",
                AchievementUpdateRequestUpdateType::Reveal => "REVEAL",
                AchievementUpdateRequestUpdateType::SetStepsAtLeast => "SET_STEPS_AT_LEAST",
                AchievementUpdateRequestUpdateType::Unlock => "UNLOCK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AchievementUpdateRequestUpdateType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AchievementUpdateRequestUpdateType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AchievementUpdateRequestUpdateType, ()> {
            Ok(match s {
                "ACHIEVEMENT_UPDATE_TYPE_UNSPECIFIED" => {
                    AchievementUpdateRequestUpdateType::AchievementUpdateTypeUnspecified
                }
                "INCREMENT" => AchievementUpdateRequestUpdateType::Increment,
                "REVEAL" => AchievementUpdateRequestUpdateType::Reveal,
                "SET_STEPS_AT_LEAST" => AchievementUpdateRequestUpdateType::SetStepsAtLeast,
                "UNLOCK" => AchievementUpdateRequestUpdateType::Unlock,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AchievementUpdateRequestUpdateType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AchievementUpdateRequestUpdateType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AchievementUpdateRequestUpdateType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACHIEVEMENT_UPDATE_TYPE_UNSPECIFIED" => {
                    AchievementUpdateRequestUpdateType::AchievementUpdateTypeUnspecified
                }
                "INCREMENT" => AchievementUpdateRequestUpdateType::Increment,
                "REVEAL" => AchievementUpdateRequestUpdateType::Reveal,
                "SET_STEPS_AT_LEAST" => AchievementUpdateRequestUpdateType::SetStepsAtLeast,
                "UNLOCK" => AchievementUpdateRequestUpdateType::Unlock,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AchievementUpdateRequestUpdateType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUpdateRequestUpdateType {
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
    pub struct AchievementUpdateResponse {
        #[doc = "The achievement this update is was applied to."]
        #[serde(
            rename = "achievementId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub achievement_id: ::std::option::Option<String>,
        #[doc = "The current state of the achievement."]
        #[serde(
            rename = "currentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_state:
            ::std::option::Option<crate::schemas::AchievementUpdateResponseCurrentState>,
        #[doc = "The current steps recorded for this achievement if it is incremental."]
        #[serde(
            rename = "currentSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_steps: ::std::option::Option<i32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player)."]
        #[serde(
            rename = "newlyUnlocked",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub newly_unlocked: ::std::option::Option<bool>,
        #[doc = "Whether the requested updates actually affected the achievement."]
        #[serde(
            rename = "updateOccurred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_occurred: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AchievementUpdateResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUpdateResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AchievementUpdateResponseCurrentState {
        #[doc = "Achievement is hidden."]
        Hidden,
        #[doc = "Achievement is revealed."]
        Revealed,
        #[doc = "Achievement is unlocked."]
        Unlocked,
        #[doc = "Safe default, don't use."]
        UpdatedAchievementStateUnspecified,
    }
    impl AchievementUpdateResponseCurrentState {
        pub fn as_str(self) -> &'static str {
            match self {
                AchievementUpdateResponseCurrentState::Hidden => "HIDDEN",
                AchievementUpdateResponseCurrentState::Revealed => "REVEALED",
                AchievementUpdateResponseCurrentState::Unlocked => "UNLOCKED",
                AchievementUpdateResponseCurrentState::UpdatedAchievementStateUnspecified => {
                    "UPDATED_ACHIEVEMENT_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AchievementUpdateResponseCurrentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AchievementUpdateResponseCurrentState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AchievementUpdateResponseCurrentState, ()> {
            Ok(match s {
                "HIDDEN" => AchievementUpdateResponseCurrentState::Hidden,
                "REVEALED" => AchievementUpdateResponseCurrentState::Revealed,
                "UNLOCKED" => AchievementUpdateResponseCurrentState::Unlocked,
                "UPDATED_ACHIEVEMENT_STATE_UNSPECIFIED" => {
                    AchievementUpdateResponseCurrentState::UpdatedAchievementStateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AchievementUpdateResponseCurrentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AchievementUpdateResponseCurrentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AchievementUpdateResponseCurrentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HIDDEN" => AchievementUpdateResponseCurrentState::Hidden,
                "REVEALED" => AchievementUpdateResponseCurrentState::Revealed,
                "UNLOCKED" => AchievementUpdateResponseCurrentState::Unlocked,
                "UPDATED_ACHIEVEMENT_STATE_UNSPECIFIED" => {
                    AchievementUpdateResponseCurrentState::UpdatedAchievementStateUnspecified
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
    impl ::google_field_selector::FieldSelector for AchievementUpdateResponseCurrentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AchievementUpdateResponseCurrentState {
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
    pub struct Application {
        #[doc = "The number of achievements visible to the currently authenticated player."]
        #[serde(
            rename = "achievement_count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub achievement_count: ::std::option::Option<i32>,
        #[doc = "The assets of the application."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets: ::std::option::Option<Vec<crate::schemas::ImageAsset>>,
        #[doc = "The author of the application."]
        #[serde(
            rename = "author",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub author: ::std::option::Option<String>,
        #[doc = "The category of the application."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<crate::schemas::ApplicationCategory>,
        #[doc = "The description of the application."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A list of features that have been enabled for the application."]
        #[serde(
            rename = "enabledFeatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled_features:
            ::std::option::Option<Vec<crate::schemas::ApplicationEnabledFeaturesItems>>,
        #[doc = "The ID of the application."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The instances of the application."]
        #[serde(
            rename = "instances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instances: ::std::option::Option<Vec<crate::schemas::Instance>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#application`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The last updated timestamp of the application."]
        #[serde(
            rename = "lastUpdatedTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_updated_timestamp: ::std::option::Option<i64>,
        #[doc = "The number of leaderboards visible to the currently authenticated player."]
        #[serde(
            rename = "leaderboard_count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub leaderboard_count: ::std::option::Option<i32>,
        #[doc = "The name of the application."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A hint to the client UI for what color to use as an app-themed color. The color is given as an RGB triplet (e.g. \"E0E0E0\")."]
        #[serde(
            rename = "themeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub theme_color: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Application {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Application {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationEnabledFeaturesItems {
        #[doc = "Safe default, don't use."]
        ApplicationFeatureUnspecified,
        #[doc = "Saved Games (snapshots)."]
        Snapshots,
    }
    impl ApplicationEnabledFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationEnabledFeaturesItems::ApplicationFeatureUnspecified => {
                    "APPLICATION_FEATURE_UNSPECIFIED"
                }
                ApplicationEnabledFeaturesItems::Snapshots => "SNAPSHOTS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationEnabledFeaturesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationEnabledFeaturesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationEnabledFeaturesItems, ()> {
            Ok(match s {
                "APPLICATION_FEATURE_UNSPECIFIED" => {
                    ApplicationEnabledFeaturesItems::ApplicationFeatureUnspecified
                }
                "SNAPSHOTS" => ApplicationEnabledFeaturesItems::Snapshots,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationEnabledFeaturesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationEnabledFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationEnabledFeaturesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_FEATURE_UNSPECIFIED" => {
                    ApplicationEnabledFeaturesItems::ApplicationFeatureUnspecified
                }
                "SNAPSHOTS" => ApplicationEnabledFeaturesItems::Snapshots,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationEnabledFeaturesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationEnabledFeaturesItems {
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
    pub struct ApplicationCategory {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationCategory`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The primary category."]
        #[serde(
            rename = "primary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary: ::std::option::Option<String>,
        #[doc = "The secondary category."]
        #[serde(
            rename = "secondary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secondary: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationCategory {
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
    pub struct ApplicationVerifyResponse {
        #[doc = "An alternate ID that was once used for the player that was issued the auth token used in this request. (This field is not normally populated.)"]
        #[serde(
            rename = "alternate_player_id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternate_player_id: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationVerifyResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The ID of the player that was issued the auth token used in this request."]
        #[serde(
            rename = "player_id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationVerifyResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationVerifyResponse {
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
    pub struct Category {
        #[doc = "The category name."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "Experience points earned in this category."]
        #[serde(
            rename = "experiencePoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub experience_points: ::std::option::Option<i64>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#category`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Category {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Category {
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
    pub struct CategoryListResponse {
        #[doc = "The list of categories with usage data."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Category>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#categoryListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token corresponding to the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CategoryListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CategoryListResponse {
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
    pub struct EndPoint {
        #[doc = "A URL suitable for loading in a web browser for the requested endpoint."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EndPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EndPoint {
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
    pub struct EventBatchRecordFailure {
        #[doc = "The cause for the update failure."]
        #[serde(
            rename = "failureCause",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_cause:
            ::std::option::Option<crate::schemas::EventBatchRecordFailureFailureCause>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventBatchRecordFailure`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The time range which was rejected; empty for a request-wide failure."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::EventPeriodRange>,
    }
    impl ::google_field_selector::FieldSelector for EventBatchRecordFailure {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventBatchRecordFailure {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EventBatchRecordFailureFailureCause {
        #[doc = "An attempt was made to record a batch of data which was already seen."]
        AlreadyUpdated,
        #[doc = "Default value. Should not be used."]
        EventFailureCauseUnspecified,
        #[doc = "An attempt was made to record data faster than the server will apply updates."]
        RecordRateHigh,
        #[doc = "A batch was sent with data too far in the past to record."]
        TimePeriodExpired,
        #[doc = "A batch was sent with a time range that was too long."]
        TimePeriodLong,
        #[doc = "A batch was sent with a time range that was too short."]
        TimePeriodShort,
        #[doc = "A batch request was issued with more events than are allowed in a single batch."]
        TooLarge,
    }
    impl EventBatchRecordFailureFailureCause {
        pub fn as_str(self) -> &'static str {
            match self {
                EventBatchRecordFailureFailureCause::AlreadyUpdated => "ALREADY_UPDATED",
                EventBatchRecordFailureFailureCause::EventFailureCauseUnspecified => {
                    "EVENT_FAILURE_CAUSE_UNSPECIFIED"
                }
                EventBatchRecordFailureFailureCause::RecordRateHigh => "RECORD_RATE_HIGH",
                EventBatchRecordFailureFailureCause::TimePeriodExpired => "TIME_PERIOD_EXPIRED",
                EventBatchRecordFailureFailureCause::TimePeriodLong => "TIME_PERIOD_LONG",
                EventBatchRecordFailureFailureCause::TimePeriodShort => "TIME_PERIOD_SHORT",
                EventBatchRecordFailureFailureCause::TooLarge => "TOO_LARGE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EventBatchRecordFailureFailureCause {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EventBatchRecordFailureFailureCause {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EventBatchRecordFailureFailureCause, ()> {
            Ok(match s {
                "ALREADY_UPDATED" => EventBatchRecordFailureFailureCause::AlreadyUpdated,
                "EVENT_FAILURE_CAUSE_UNSPECIFIED" => {
                    EventBatchRecordFailureFailureCause::EventFailureCauseUnspecified
                }
                "RECORD_RATE_HIGH" => EventBatchRecordFailureFailureCause::RecordRateHigh,
                "TIME_PERIOD_EXPIRED" => EventBatchRecordFailureFailureCause::TimePeriodExpired,
                "TIME_PERIOD_LONG" => EventBatchRecordFailureFailureCause::TimePeriodLong,
                "TIME_PERIOD_SHORT" => EventBatchRecordFailureFailureCause::TimePeriodShort,
                "TOO_LARGE" => EventBatchRecordFailureFailureCause::TooLarge,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EventBatchRecordFailureFailureCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EventBatchRecordFailureFailureCause {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EventBatchRecordFailureFailureCause {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALREADY_UPDATED" => EventBatchRecordFailureFailureCause::AlreadyUpdated,
                "EVENT_FAILURE_CAUSE_UNSPECIFIED" => {
                    EventBatchRecordFailureFailureCause::EventFailureCauseUnspecified
                }
                "RECORD_RATE_HIGH" => EventBatchRecordFailureFailureCause::RecordRateHigh,
                "TIME_PERIOD_EXPIRED" => EventBatchRecordFailureFailureCause::TimePeriodExpired,
                "TIME_PERIOD_LONG" => EventBatchRecordFailureFailureCause::TimePeriodLong,
                "TIME_PERIOD_SHORT" => EventBatchRecordFailureFailureCause::TimePeriodShort,
                "TOO_LARGE" => EventBatchRecordFailureFailureCause::TooLarge,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EventBatchRecordFailureFailureCause {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventBatchRecordFailureFailureCause {
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
    pub struct EventChild {
        #[doc = "The ID of the child event."]
        #[serde(
            rename = "childId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub child_id: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventChild`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EventChild {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventChild {
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
    pub struct EventDefinition {
        #[doc = "A list of events that are a child of this event."]
        #[serde(
            rename = "childEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub child_events: ::std::option::Option<Vec<crate::schemas::EventChild>>,
        #[doc = "Description of what this event represents."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The name to display for the event."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The ID of the event."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The base URL for the image that represents the event."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "Indicates whether the icon image being returned is a default image, or is game-provided."]
        #[serde(
            rename = "isDefaultImageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_default_image_url: ::std::option::Option<bool>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinition`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The visibility of event being tracked in this definition."]
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::EventDefinitionVisibility>,
    }
    impl ::google_field_selector::FieldSelector for EventDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventDefinition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EventDefinitionVisibility {
        #[doc = "Default value. Should not be used."]
        EventVisibilityUnspecified,
        #[doc = "This event should only be shown to users that have recorded this event at least once."]
        Hidden,
        #[doc = "This event should be visible to all users."]
        Revealed,
    }
    impl EventDefinitionVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                EventDefinitionVisibility::EventVisibilityUnspecified => {
                    "EVENT_VISIBILITY_UNSPECIFIED"
                }
                EventDefinitionVisibility::Hidden => "HIDDEN",
                EventDefinitionVisibility::Revealed => "REVEALED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EventDefinitionVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EventDefinitionVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EventDefinitionVisibility, ()> {
            Ok(match s {
                "EVENT_VISIBILITY_UNSPECIFIED" => {
                    EventDefinitionVisibility::EventVisibilityUnspecified
                }
                "HIDDEN" => EventDefinitionVisibility::Hidden,
                "REVEALED" => EventDefinitionVisibility::Revealed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EventDefinitionVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EventDefinitionVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EventDefinitionVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EVENT_VISIBILITY_UNSPECIFIED" => {
                    EventDefinitionVisibility::EventVisibilityUnspecified
                }
                "HIDDEN" => EventDefinitionVisibility::Hidden,
                "REVEALED" => EventDefinitionVisibility::Revealed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EventDefinitionVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventDefinitionVisibility {
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
    pub struct EventDefinitionListResponse {
        #[doc = "The event definitions."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::EventDefinition>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinitionListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The pagination token for the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EventDefinitionListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventDefinitionListResponse {
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
    pub struct EventPeriodRange {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodRange`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The time when this update period ends, in millis, since 1970 UTC (Unix Epoch)."]
        #[serde(
            rename = "periodEndMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub period_end_millis: ::std::option::Option<i64>,
        #[doc = "The time when this update period begins, in millis, since 1970 UTC (Unix Epoch)."]
        #[serde(
            rename = "periodStartMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub period_start_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for EventPeriodRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventPeriodRange {
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
    pub struct EventPeriodUpdate {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodUpdate`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The time period being covered by this update."]
        #[serde(
            rename = "timePeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_period: ::std::option::Option<crate::schemas::EventPeriodRange>,
        #[doc = "The updates being made for this time period."]
        #[serde(
            rename = "updates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updates: ::std::option::Option<Vec<crate::schemas::EventUpdateRequest>>,
    }
    impl ::google_field_selector::FieldSelector for EventPeriodUpdate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventPeriodUpdate {
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
    pub struct EventRecordFailure {
        #[doc = "The ID of the event that was not updated."]
        #[serde(
            rename = "eventId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_id: ::std::option::Option<String>,
        #[doc = "The cause for the update failure."]
        #[serde(
            rename = "failureCause",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_cause: ::std::option::Option<crate::schemas::EventRecordFailureFailureCause>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordFailure`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EventRecordFailure {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventRecordFailure {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EventRecordFailureFailureCause {
        #[doc = "Default value. Should not use."]
        EventUpdateFailureCauseUnspecified,
        #[doc = "An attempt was made to increment an event by a non-positive value."]
        InvalidUpdateValue,
        #[doc = "An attempt was made to set an event that was not defined."]
        NotFound,
    }
    impl EventRecordFailureFailureCause {
        pub fn as_str(self) -> &'static str {
            match self {
                EventRecordFailureFailureCause::EventUpdateFailureCauseUnspecified => {
                    "EVENT_UPDATE_FAILURE_CAUSE_UNSPECIFIED"
                }
                EventRecordFailureFailureCause::InvalidUpdateValue => "INVALID_UPDATE_VALUE",
                EventRecordFailureFailureCause::NotFound => "NOT_FOUND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EventRecordFailureFailureCause {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EventRecordFailureFailureCause {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EventRecordFailureFailureCause, ()> {
            Ok(match s {
                "EVENT_UPDATE_FAILURE_CAUSE_UNSPECIFIED" => {
                    EventRecordFailureFailureCause::EventUpdateFailureCauseUnspecified
                }
                "INVALID_UPDATE_VALUE" => EventRecordFailureFailureCause::InvalidUpdateValue,
                "NOT_FOUND" => EventRecordFailureFailureCause::NotFound,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EventRecordFailureFailureCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EventRecordFailureFailureCause {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EventRecordFailureFailureCause {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EVENT_UPDATE_FAILURE_CAUSE_UNSPECIFIED" => {
                    EventRecordFailureFailureCause::EventUpdateFailureCauseUnspecified
                }
                "INVALID_UPDATE_VALUE" => EventRecordFailureFailureCause::InvalidUpdateValue,
                "NOT_FOUND" => EventRecordFailureFailureCause::NotFound,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EventRecordFailureFailureCause {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventRecordFailureFailureCause {
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
    pub struct EventRecordRequest {
        #[doc = "The current time when this update was sent, in milliseconds, since 1970 UTC (Unix Epoch)."]
        #[serde(
            rename = "currentTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current_time_millis: ::std::option::Option<i64>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordRequest`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The request ID used to identify this attempt to record events."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub request_id: ::std::option::Option<i64>,
        #[doc = "A list of the time period updates being made in this request."]
        #[serde(
            rename = "timePeriods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_periods: ::std::option::Option<Vec<crate::schemas::EventPeriodUpdate>>,
    }
    impl ::google_field_selector::FieldSelector for EventRecordRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventRecordRequest {
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
    pub struct EventUpdateRequest {
        #[doc = "The ID of the event being modified in this update."]
        #[serde(
            rename = "definitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub definition_id: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateRequest`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The number of times this event occurred in this time period."]
        #[serde(
            rename = "updateCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub update_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for EventUpdateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventUpdateRequest {
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
    pub struct EventUpdateResponse {
        #[doc = "Any batch-wide failures which occurred applying updates."]
        #[serde(
            rename = "batchFailures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub batch_failures: ::std::option::Option<Vec<crate::schemas::EventBatchRecordFailure>>,
        #[doc = "Any failures updating a particular event."]
        #[serde(
            rename = "eventFailures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_failures: ::std::option::Option<Vec<crate::schemas::EventRecordFailure>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The current status of any updated events"]
        #[serde(
            rename = "playerEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_events: ::std::option::Option<Vec<crate::schemas::PlayerEvent>>,
    }
    impl ::google_field_selector::FieldSelector for EventUpdateResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventUpdateResponse {
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
    pub struct GamesAchievementIncrement {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementIncrement`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The requestId associated with an increment to an achievement."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub request_id: ::std::option::Option<i64>,
        #[doc = "The number of steps to be incremented."]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GamesAchievementIncrement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GamesAchievementIncrement {
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
    pub struct GamesAchievementSetStepsAtLeast {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementSetStepsAtLeast`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The minimum number of steps for the achievement to be set to."]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GamesAchievementSetStepsAtLeast {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GamesAchievementSetStepsAtLeast {
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
    pub struct ImageAsset {
        #[doc = "The height of the asset."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#imageAsset`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the asset."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The URL of the asset."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[doc = "The width of the asset."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ImageAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageAsset {
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
    pub struct Instance {
        #[doc = "URI which shows where a user can acquire this instance."]
        #[serde(
            rename = "acquisitionUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acquisition_uri: ::std::option::Option<String>,
        #[doc = "Platform dependent details for Android."]
        #[serde(
            rename = "androidInstance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_instance: ::std::option::Option<crate::schemas::InstanceAndroidDetails>,
        #[doc = "Platform dependent details for iOS."]
        #[serde(
            rename = "iosInstance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_instance: ::std::option::Option<crate::schemas::InstanceIosDetails>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instance`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Localized display name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The platform type."]
        #[serde(
            rename = "platformType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_type: ::std::option::Option<crate::schemas::InstancePlatformType>,
        #[doc = "Flag to show if this game instance supports realtime play."]
        #[serde(
            rename = "realtimePlay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub realtime_play: ::std::option::Option<bool>,
        #[doc = "Flag to show if this game instance supports turn based play."]
        #[serde(
            rename = "turnBasedPlay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub turn_based_play: ::std::option::Option<bool>,
        #[doc = "Platform dependent details for Web."]
        #[serde(
            rename = "webInstance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_instance: ::std::option::Option<crate::schemas::InstanceWebDetails>,
    }
    impl ::google_field_selector::FieldSelector for Instance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Instance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InstancePlatformType {
        #[doc = "Instance is for Android."]
        Android,
        #[doc = "Instance is for iOS."]
        Ios,
        #[doc = "Default value. Should be unused."]
        PlatformTypeUnspecified,
        #[doc = "Instance is for Web App."]
        WebApp,
    }
    impl InstancePlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                InstancePlatformType::Android => "ANDROID",
                InstancePlatformType::Ios => "IOS",
                InstancePlatformType::PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED",
                InstancePlatformType::WebApp => "WEB_APP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InstancePlatformType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InstancePlatformType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InstancePlatformType, ()> {
            Ok(match s {
                "ANDROID" => InstancePlatformType::Android,
                "IOS" => InstancePlatformType::Ios,
                "PLATFORM_TYPE_UNSPECIFIED" => InstancePlatformType::PlatformTypeUnspecified,
                "WEB_APP" => InstancePlatformType::WebApp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InstancePlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InstancePlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InstancePlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => InstancePlatformType::Android,
                "IOS" => InstancePlatformType::Ios,
                "PLATFORM_TYPE_UNSPECIFIED" => InstancePlatformType::PlatformTypeUnspecified,
                "WEB_APP" => InstancePlatformType::WebApp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InstancePlatformType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancePlatformType {
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
    pub struct InstanceAndroidDetails {
        #[doc = "Flag indicating whether the anti-piracy check is enabled."]
        #[serde(
            rename = "enablePiracyCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_piracy_check: ::std::option::Option<bool>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceAndroidDetails`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Android package name which maps to Google Play URL."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Indicates that this instance is the default for new installations."]
        #[serde(
            rename = "preferred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preferred: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for InstanceAndroidDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceAndroidDetails {
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
    pub struct InstanceIosDetails {
        #[doc = "Bundle identifier."]
        #[serde(
            rename = "bundleIdentifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_identifier: ::std::option::Option<String>,
        #[doc = "iTunes App ID."]
        #[serde(
            rename = "itunesAppId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub itunes_app_id: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceIosDetails`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Indicates that this instance is the default for new installations on iPad devices."]
        #[serde(
            rename = "preferredForIpad",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preferred_for_ipad: ::std::option::Option<bool>,
        #[doc = "Indicates that this instance is the default for new installations on iPhone devices."]
        #[serde(
            rename = "preferredForIphone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preferred_for_iphone: ::std::option::Option<bool>,
        #[doc = "Flag to indicate if this instance supports iPad."]
        #[serde(
            rename = "supportIpad",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_ipad: ::std::option::Option<bool>,
        #[doc = "Flag to indicate if this instance supports iPhone."]
        #[serde(
            rename = "supportIphone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_iphone: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for InstanceIosDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceIosDetails {
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
    pub struct InstanceWebDetails {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceWebDetails`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Launch URL for the game."]
        #[serde(
            rename = "launchUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_url: ::std::option::Option<String>,
        #[doc = "Indicates that this instance is the default for new installations."]
        #[serde(
            rename = "preferred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preferred: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for InstanceWebDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceWebDetails {
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
    pub struct Leaderboard {
        #[doc = "The icon for the leaderboard."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The leaderboard ID."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Indicates whether the icon image being returned is a default image, or is game-provided."]
        #[serde(
            rename = "isIconUrlDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_icon_url_default: ::std::option::Option<bool>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboard`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the leaderboard."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "How scores are ordered."]
        #[serde(
            rename = "order",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order: ::std::option::Option<crate::schemas::LeaderboardOrder>,
    }
    impl ::google_field_selector::FieldSelector for Leaderboard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Leaderboard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LeaderboardOrder {
        #[doc = "Larger values are better; scores are sorted in descending order"]
        LargerIsBetter,
        #[doc = "Default value. This value is unused."]
        ScoreOrderUnspecified,
        #[doc = "Smaller values are better; scores are sorted in ascending order"]
        SmallerIsBetter,
    }
    impl LeaderboardOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                LeaderboardOrder::LargerIsBetter => "LARGER_IS_BETTER",
                LeaderboardOrder::ScoreOrderUnspecified => "SCORE_ORDER_UNSPECIFIED",
                LeaderboardOrder::SmallerIsBetter => "SMALLER_IS_BETTER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LeaderboardOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LeaderboardOrder {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LeaderboardOrder, ()> {
            Ok(match s {
                "LARGER_IS_BETTER" => LeaderboardOrder::LargerIsBetter,
                "SCORE_ORDER_UNSPECIFIED" => LeaderboardOrder::ScoreOrderUnspecified,
                "SMALLER_IS_BETTER" => LeaderboardOrder::SmallerIsBetter,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LeaderboardOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LeaderboardOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LeaderboardOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LARGER_IS_BETTER" => LeaderboardOrder::LargerIsBetter,
                "SCORE_ORDER_UNSPECIFIED" => LeaderboardOrder::ScoreOrderUnspecified,
                "SMALLER_IS_BETTER" => LeaderboardOrder::SmallerIsBetter,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LeaderboardOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LeaderboardOrder {
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
    pub struct LeaderboardEntry {
        #[doc = "The localized string for the numerical value of this score."]
        #[serde(
            rename = "formattedScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_score: ::std::option::Option<String>,
        #[doc = "The localized string for the rank of this score for this leaderboard."]
        #[serde(
            rename = "formattedScoreRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_score_rank: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardEntry`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The player who holds this score."]
        #[serde(
            rename = "player",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player: ::std::option::Option<crate::schemas::Player>,
        #[doc = "The rank of this score for this leaderboard."]
        #[serde(
            rename = "scoreRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub score_rank: ::std::option::Option<i64>,
        #[doc = "Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        #[serde(
            rename = "scoreTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_tag: ::std::option::Option<String>,
        #[doc = "The numerical value of this score."]
        #[serde(
            rename = "scoreValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub score_value: ::std::option::Option<i64>,
        #[doc = "The time span of this high score."]
        #[serde(
            rename = "timeSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_span: ::std::option::Option<crate::schemas::LeaderboardEntryTimeSpan>,
        #[doc = "The timestamp at which this score was recorded, in milliseconds since the epoch in UTC."]
        #[serde(
            rename = "writeTimestampMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub write_timestamp_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for LeaderboardEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LeaderboardEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LeaderboardEntryTimeSpan {
        #[doc = "The score is an all-time score."]
        AllTime,
        #[doc = "The score is a daily score."]
        Daily,
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[doc = "The score is a weekly score."]
        Weekly,
    }
    impl LeaderboardEntryTimeSpan {
        pub fn as_str(self) -> &'static str {
            match self {
                LeaderboardEntryTimeSpan::AllTime => "ALL_TIME",
                LeaderboardEntryTimeSpan::Daily => "DAILY",
                LeaderboardEntryTimeSpan::ScoreTimeSpanUnspecified => "SCORE_TIME_SPAN_UNSPECIFIED",
                LeaderboardEntryTimeSpan::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LeaderboardEntryTimeSpan {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LeaderboardEntryTimeSpan {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LeaderboardEntryTimeSpan, ()> {
            Ok(match s {
                "ALL_TIME" => LeaderboardEntryTimeSpan::AllTime,
                "DAILY" => LeaderboardEntryTimeSpan::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => LeaderboardEntryTimeSpan::ScoreTimeSpanUnspecified,
                "WEEKLY" => LeaderboardEntryTimeSpan::Weekly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LeaderboardEntryTimeSpan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LeaderboardEntryTimeSpan {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LeaderboardEntryTimeSpan {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => LeaderboardEntryTimeSpan::AllTime,
                "DAILY" => LeaderboardEntryTimeSpan::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => LeaderboardEntryTimeSpan::ScoreTimeSpanUnspecified,
                "WEEKLY" => LeaderboardEntryTimeSpan::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LeaderboardEntryTimeSpan {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LeaderboardEntryTimeSpan {
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
    pub struct LeaderboardListResponse {
        #[doc = "The leaderboards."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Leaderboard>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token corresponding to the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LeaderboardListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LeaderboardListResponse {
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
    pub struct LeaderboardScoreRank {
        #[doc = "The number of scores in the leaderboard as a string."]
        #[serde(
            rename = "formattedNumScores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_num_scores: ::std::option::Option<String>,
        #[doc = "The rank in the leaderboard as a string."]
        #[serde(
            rename = "formattedRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_rank: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScoreRank`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The number of scores in the leaderboard."]
        #[serde(
            rename = "numScores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_scores: ::std::option::Option<i64>,
        #[doc = "The rank in the leaderboard."]
        #[serde(
            rename = "rank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub rank: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for LeaderboardScoreRank {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LeaderboardScoreRank {
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
    pub struct LeaderboardScores {
        #[doc = "The scores in the leaderboard."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::LeaderboardEntry>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScores`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The pagination token for the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of scores in the leaderboard."]
        #[serde(
            rename = "numScores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_scores: ::std::option::Option<i64>,
        #[doc = "The score of the requesting player on the leaderboard. The player's score may appear both here and in the list of scores above. If you are viewing a public leaderboard and the player is not sharing their gameplay information publicly, the `scoreRank`and `formattedScoreRank` values will not be present."]
        #[serde(
            rename = "playerScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_score: ::std::option::Option<crate::schemas::LeaderboardEntry>,
        #[doc = "The pagination token for the previous page of results."]
        #[serde(
            rename = "prevPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prev_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LeaderboardScores {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LeaderboardScores {
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
    pub struct MetagameConfig {
        #[doc = "Current version of the metagame configuration data. When this data is updated, the version number will be increased by one."]
        #[serde(
            rename = "currentVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_version: ::std::option::Option<i32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#metagameConfig`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The list of player levels."]
        #[serde(
            rename = "playerLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_levels: ::std::option::Option<Vec<crate::schemas::PlayerLevel>>,
    }
    impl ::google_field_selector::FieldSelector for MetagameConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetagameConfig {
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
    pub struct Player {
        #[doc = "The base URL for the image that represents the player."]
        #[serde(
            rename = "avatarImageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub avatar_image_url: ::std::option::Option<String>,
        #[doc = "The url to the landscape mode player banner image."]
        #[serde(
            rename = "bannerUrlLandscape",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banner_url_landscape: ::std::option::Option<String>,
        #[doc = "The url to the portrait mode player banner image."]
        #[serde(
            rename = "bannerUrlPortrait",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banner_url_portrait: ::std::option::Option<String>,
        #[doc = "The name to display for the player."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "An object to represent Play Game experience information for the player."]
        #[serde(
            rename = "experienceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub experience_info: ::std::option::Option<crate::schemas::PlayerExperienceInfo>,
        #[doc = "The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game."]
        #[serde(
            rename = "friendStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friend_status: ::std::option::Option<crate::schemas::PlayerFriendStatus>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#player`"]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A representation of the individual components of the name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<crate::schemas::PlayerName>,
        #[doc = "The player ID that was used for this player the first time they signed into the game in question. This is only populated for calls to player.get for the requesting player, only if the player ID has subsequently changed, and only to clients that support remapping player IDs."]
        #[serde(
            rename = "originalPlayerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_player_id: ::std::option::Option<String>,
        #[doc = "The ID of the player."]
        #[serde(
            rename = "playerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_id: ::std::option::Option<String>,
        #[doc = "The player's profile settings. Controls whether or not the player's profile is visible to other players."]
        #[serde(
            rename = "profileSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_settings: ::std::option::Option<crate::schemas::ProfileSettings>,
        #[doc = "The player's title rewarded for their game activities."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Player {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Player {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlayerFriendStatus {
        #[doc = "The player and requester are friends."]
        Friend,
        #[doc = "Default value. This value is unused."]
        FriendStatusUnspecified,
        #[doc = "There is no relationship between the players."]
        NoRelationship,
    }
    impl PlayerFriendStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                PlayerFriendStatus::Friend => "FRIEND",
                PlayerFriendStatus::FriendStatusUnspecified => "FRIEND_STATUS_UNSPECIFIED",
                PlayerFriendStatus::NoRelationship => "NO_RELATIONSHIP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PlayerFriendStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PlayerFriendStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PlayerFriendStatus, ()> {
            Ok(match s {
                "FRIEND" => PlayerFriendStatus::Friend,
                "FRIEND_STATUS_UNSPECIFIED" => PlayerFriendStatus::FriendStatusUnspecified,
                "NO_RELATIONSHIP" => PlayerFriendStatus::NoRelationship,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PlayerFriendStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlayerFriendStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlayerFriendStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FRIEND" => PlayerFriendStatus::Friend,
                "FRIEND_STATUS_UNSPECIFIED" => PlayerFriendStatus::FriendStatusUnspecified,
                "NO_RELATIONSHIP" => PlayerFriendStatus::NoRelationship,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PlayerFriendStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerFriendStatus {
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
    pub struct PlayerName {
        #[doc = "The family name of this player. In some places, this is known as the last name."]
        #[serde(
            rename = "familyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub family_name: ::std::option::Option<String>,
        #[doc = "The given name of this player. In some places, this is known as the first name."]
        #[serde(
            rename = "givenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub given_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PlayerName {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerName {
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
    pub struct PlayerAchievement {
        #[doc = "The state of the achievement."]
        #[serde(
            rename = "achievementState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub achievement_state:
            ::std::option::Option<crate::schemas::PlayerAchievementAchievementState>,
        #[doc = "The current steps for an incremental achievement."]
        #[serde(
            rename = "currentSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_steps: ::std::option::Option<i32>,
        #[doc = "Experience points earned for the achievement. This field is absent for achievements that have not yet been unlocked and 0 for achievements that have been unlocked by testers but that are unpublished."]
        #[serde(
            rename = "experiencePoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub experience_points: ::std::option::Option<i64>,
        #[doc = "The current steps for an incremental achievement as a string."]
        #[serde(
            rename = "formattedCurrentStepsString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_current_steps_string: ::std::option::Option<String>,
        #[doc = "The ID of the achievement."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievement`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The timestamp of the last modification to this achievement's state."]
        #[serde(
            rename = "lastUpdatedTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_updated_timestamp: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PlayerAchievement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerAchievement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlayerAchievementAchievementState {
        #[doc = "Achievement is hidden."]
        Hidden,
        #[doc = "Achievement is revealed."]
        Revealed,
        #[doc = "Default value. This value is unused."]
        StateUnspecified,
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl PlayerAchievementAchievementState {
        pub fn as_str(self) -> &'static str {
            match self {
                PlayerAchievementAchievementState::Hidden => "HIDDEN",
                PlayerAchievementAchievementState::Revealed => "REVEALED",
                PlayerAchievementAchievementState::StateUnspecified => "STATE_UNSPECIFIED",
                PlayerAchievementAchievementState::Unlocked => "UNLOCKED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PlayerAchievementAchievementState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PlayerAchievementAchievementState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PlayerAchievementAchievementState, ()> {
            Ok(match s {
                "HIDDEN" => PlayerAchievementAchievementState::Hidden,
                "REVEALED" => PlayerAchievementAchievementState::Revealed,
                "STATE_UNSPECIFIED" => PlayerAchievementAchievementState::StateUnspecified,
                "UNLOCKED" => PlayerAchievementAchievementState::Unlocked,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PlayerAchievementAchievementState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlayerAchievementAchievementState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlayerAchievementAchievementState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HIDDEN" => PlayerAchievementAchievementState::Hidden,
                "REVEALED" => PlayerAchievementAchievementState::Revealed,
                "STATE_UNSPECIFIED" => PlayerAchievementAchievementState::StateUnspecified,
                "UNLOCKED" => PlayerAchievementAchievementState::Unlocked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PlayerAchievementAchievementState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerAchievementAchievementState {
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
    pub struct PlayerAchievementListResponse {
        #[doc = "The achievements."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::PlayerAchievement>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievementListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token corresponding to the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PlayerAchievementListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerAchievementListResponse {
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
    pub struct PlayerEvent {
        #[doc = "The ID of the event definition."]
        #[serde(
            rename = "definitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub definition_id: ::std::option::Option<String>,
        #[doc = "The current number of times this event has occurred, as a string. The formatting of this string depends on the configuration of your event in the Play Games Developer Console."]
        #[serde(
            rename = "formattedNumEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_num_events: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEvent`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The current number of times this event has occurred."]
        #[serde(
            rename = "numEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_events: ::std::option::Option<i64>,
        #[doc = "The ID of the player."]
        #[serde(
            rename = "playerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PlayerEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerEvent {
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
    pub struct PlayerEventListResponse {
        #[doc = "The player events."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::PlayerEvent>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEventListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The pagination token for the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PlayerEventListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerEventListResponse {
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
    pub struct PlayerExperienceInfo {
        #[doc = "The current number of experience points for the player."]
        #[serde(
            rename = "currentExperiencePoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current_experience_points: ::std::option::Option<i64>,
        #[doc = "The current level of the player."]
        #[serde(
            rename = "currentLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_level: ::std::option::Option<crate::schemas::PlayerLevel>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerExperienceInfo`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The timestamp when the player was leveled up, in millis since Unix epoch UTC."]
        #[serde(
            rename = "lastLevelUpTimestampMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_level_up_timestamp_millis: ::std::option::Option<i64>,
        #[doc = "The next level of the player. If the current level is the maximum level, this should be same as the current level."]
        #[serde(
            rename = "nextLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_level: ::std::option::Option<crate::schemas::PlayerLevel>,
    }
    impl ::google_field_selector::FieldSelector for PlayerExperienceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerExperienceInfo {
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
    pub struct PlayerLeaderboardScore {
        #[doc = "The rank of the score in the friends collection for this leaderboard."]
        #[serde(
            rename = "friendsRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friends_rank: ::std::option::Option<crate::schemas::LeaderboardScoreRank>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScore`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The ID of the leaderboard this score is in."]
        #[serde(
            rename = "leaderboard_id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub leaderboard_id: ::std::option::Option<String>,
        #[doc = "The public rank of the score in this leaderboard. This object will not be present if the user is not sharing their scores publicly."]
        #[serde(
            rename = "publicRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_rank: ::std::option::Option<crate::schemas::LeaderboardScoreRank>,
        #[doc = "The formatted value of this score."]
        #[serde(
            rename = "scoreString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_string: ::std::option::Option<String>,
        #[doc = "Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        #[serde(
            rename = "scoreTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_tag: ::std::option::Option<String>,
        #[doc = "The numerical value of this score."]
        #[serde(
            rename = "scoreValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub score_value: ::std::option::Option<i64>,
        #[doc = "The social rank of the score in this leaderboard."]
        #[serde(
            rename = "socialRank",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub social_rank: ::std::option::Option<crate::schemas::LeaderboardScoreRank>,
        #[doc = "The time span of this score."]
        #[serde(
            rename = "timeSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_span: ::std::option::Option<crate::schemas::PlayerLeaderboardScoreTimeSpan>,
        #[doc = "The timestamp at which this score was recorded, in milliseconds since the epoch in UTC."]
        #[serde(
            rename = "writeTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub write_timestamp: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PlayerLeaderboardScore {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerLeaderboardScore {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlayerLeaderboardScoreTimeSpan {
        #[doc = "The score is an all-time score."]
        AllTime,
        #[doc = "The score is a daily score."]
        Daily,
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[doc = "The score is a weekly score."]
        Weekly,
    }
    impl PlayerLeaderboardScoreTimeSpan {
        pub fn as_str(self) -> &'static str {
            match self {
                PlayerLeaderboardScoreTimeSpan::AllTime => "ALL_TIME",
                PlayerLeaderboardScoreTimeSpan::Daily => "DAILY",
                PlayerLeaderboardScoreTimeSpan::ScoreTimeSpanUnspecified => {
                    "SCORE_TIME_SPAN_UNSPECIFIED"
                }
                PlayerLeaderboardScoreTimeSpan::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PlayerLeaderboardScoreTimeSpan {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PlayerLeaderboardScoreTimeSpan {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PlayerLeaderboardScoreTimeSpan, ()> {
            Ok(match s {
                "ALL_TIME" => PlayerLeaderboardScoreTimeSpan::AllTime,
                "DAILY" => PlayerLeaderboardScoreTimeSpan::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => {
                    PlayerLeaderboardScoreTimeSpan::ScoreTimeSpanUnspecified
                }
                "WEEKLY" => PlayerLeaderboardScoreTimeSpan::Weekly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PlayerLeaderboardScoreTimeSpan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlayerLeaderboardScoreTimeSpan {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlayerLeaderboardScoreTimeSpan {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => PlayerLeaderboardScoreTimeSpan::AllTime,
                "DAILY" => PlayerLeaderboardScoreTimeSpan::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => {
                    PlayerLeaderboardScoreTimeSpan::ScoreTimeSpanUnspecified
                }
                "WEEKLY" => PlayerLeaderboardScoreTimeSpan::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PlayerLeaderboardScoreTimeSpan {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerLeaderboardScoreTimeSpan {
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
    pub struct PlayerLeaderboardScoreListResponse {
        #[doc = "The leaderboard scores."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::PlayerLeaderboardScore>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScoreListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The pagination token for the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The Player resources for the owner of this score."]
        #[serde(
            rename = "player",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player: ::std::option::Option<crate::schemas::Player>,
    }
    impl ::google_field_selector::FieldSelector for PlayerLeaderboardScoreListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerLeaderboardScoreListResponse {
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
    pub struct PlayerLevel {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLevel`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The level for the user."]
        #[serde(
            rename = "level",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub level: ::std::option::Option<i32>,
        #[doc = "The maximum experience points for this level."]
        #[serde(
            rename = "maxExperiencePoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_experience_points: ::std::option::Option<i64>,
        #[doc = "The minimum experience points for this level."]
        #[serde(
            rename = "minExperiencePoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_experience_points: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PlayerLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerLevel {
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
    pub struct PlayerListResponse {
        #[doc = "The players."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Player>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token corresponding to the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PlayerListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerListResponse {
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
    pub struct PlayerScore {
        #[doc = "The formatted score for this player score."]
        #[serde(
            rename = "formattedScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_score: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScore`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The numerical value for this player score."]
        #[serde(
            rename = "score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub score: ::std::option::Option<i64>,
        #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        #[serde(
            rename = "scoreTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_tag: ::std::option::Option<String>,
        #[doc = "The time span for this player score."]
        #[serde(
            rename = "timeSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_span: ::std::option::Option<crate::schemas::PlayerScoreTimeSpan>,
    }
    impl ::google_field_selector::FieldSelector for PlayerScore {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerScore {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlayerScoreTimeSpan {
        #[doc = "The score is an all-time score."]
        AllTime,
        #[doc = "The score is a daily score."]
        Daily,
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[doc = "The score is a weekly score."]
        Weekly,
    }
    impl PlayerScoreTimeSpan {
        pub fn as_str(self) -> &'static str {
            match self {
                PlayerScoreTimeSpan::AllTime => "ALL_TIME",
                PlayerScoreTimeSpan::Daily => "DAILY",
                PlayerScoreTimeSpan::ScoreTimeSpanUnspecified => "SCORE_TIME_SPAN_UNSPECIFIED",
                PlayerScoreTimeSpan::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PlayerScoreTimeSpan {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PlayerScoreTimeSpan {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PlayerScoreTimeSpan, ()> {
            Ok(match s {
                "ALL_TIME" => PlayerScoreTimeSpan::AllTime,
                "DAILY" => PlayerScoreTimeSpan::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => PlayerScoreTimeSpan::ScoreTimeSpanUnspecified,
                "WEEKLY" => PlayerScoreTimeSpan::Weekly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PlayerScoreTimeSpan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlayerScoreTimeSpan {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlayerScoreTimeSpan {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => PlayerScoreTimeSpan::AllTime,
                "DAILY" => PlayerScoreTimeSpan::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => PlayerScoreTimeSpan::ScoreTimeSpanUnspecified,
                "WEEKLY" => PlayerScoreTimeSpan::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PlayerScoreTimeSpan {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerScoreTimeSpan {
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
    pub struct PlayerScoreListResponse {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The score submissions statuses."]
        #[serde(
            rename = "submittedScores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub submitted_scores: ::std::option::Option<Vec<crate::schemas::PlayerScoreResponse>>,
    }
    impl ::google_field_selector::FieldSelector for PlayerScoreListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerScoreListResponse {
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
    pub struct PlayerScoreResponse {
        #[doc = "The time spans where the submitted score is better than the existing score for that time span."]
        #[serde(
            rename = "beatenScoreTimeSpans",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub beaten_score_time_spans: ::std::option::Option<
            Vec<crate::schemas::PlayerScoreResponseBeatenScoreTimeSpansItems>,
        >,
        #[doc = "The formatted value of the submitted score."]
        #[serde(
            rename = "formattedScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_score: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The leaderboard ID that this score was submitted to."]
        #[serde(
            rename = "leaderboardId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub leaderboard_id: ::std::option::Option<String>,
        #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        #[serde(
            rename = "scoreTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_tag: ::std::option::Option<String>,
        #[doc = "The scores in time spans that have not been beaten. As an example, the submitted score may be better than the player's `DAILY` score, but not better than the player's scores for the `WEEKLY` or `ALL_TIME` time spans."]
        #[serde(
            rename = "unbeatenScores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unbeaten_scores: ::std::option::Option<Vec<crate::schemas::PlayerScore>>,
    }
    impl ::google_field_selector::FieldSelector for PlayerScoreResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerScoreResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlayerScoreResponseBeatenScoreTimeSpansItems {
        #[doc = "The score is an all-time score."]
        AllTime,
        #[doc = "The score is a daily score."]
        Daily,
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[doc = "The score is a weekly score."]
        Weekly,
    }
    impl PlayerScoreResponseBeatenScoreTimeSpansItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PlayerScoreResponseBeatenScoreTimeSpansItems::AllTime => "ALL_TIME",
                PlayerScoreResponseBeatenScoreTimeSpansItems::Daily => "DAILY",
                PlayerScoreResponseBeatenScoreTimeSpansItems::ScoreTimeSpanUnspecified => {
                    "SCORE_TIME_SPAN_UNSPECIFIED"
                }
                PlayerScoreResponseBeatenScoreTimeSpansItems::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PlayerScoreResponseBeatenScoreTimeSpansItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PlayerScoreResponseBeatenScoreTimeSpansItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PlayerScoreResponseBeatenScoreTimeSpansItems, ()> {
            Ok(match s {
                "ALL_TIME" => PlayerScoreResponseBeatenScoreTimeSpansItems::AllTime,
                "DAILY" => PlayerScoreResponseBeatenScoreTimeSpansItems::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => {
                    PlayerScoreResponseBeatenScoreTimeSpansItems::ScoreTimeSpanUnspecified
                }
                "WEEKLY" => PlayerScoreResponseBeatenScoreTimeSpansItems::Weekly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PlayerScoreResponseBeatenScoreTimeSpansItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlayerScoreResponseBeatenScoreTimeSpansItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlayerScoreResponseBeatenScoreTimeSpansItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => PlayerScoreResponseBeatenScoreTimeSpansItems::AllTime,
                "DAILY" => PlayerScoreResponseBeatenScoreTimeSpansItems::Daily,
                "SCORE_TIME_SPAN_UNSPECIFIED" => {
                    PlayerScoreResponseBeatenScoreTimeSpansItems::ScoreTimeSpanUnspecified
                }
                "WEEKLY" => PlayerScoreResponseBeatenScoreTimeSpansItems::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PlayerScoreResponseBeatenScoreTimeSpansItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerScoreResponseBeatenScoreTimeSpansItems {
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
    pub struct PlayerScoreSubmissionList {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreSubmissionList`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The score submissions."]
        #[serde(
            rename = "scores",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scores: ::std::option::Option<Vec<crate::schemas::ScoreSubmission>>,
    }
    impl ::google_field_selector::FieldSelector for PlayerScoreSubmissionList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlayerScoreSubmissionList {
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
    pub struct ProfileSettings {
        #[serde(
            rename = "friendsListVisibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub friends_list_visibility:
            ::std::option::Option<crate::schemas::ProfileSettingsFriendsListVisibility>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#profileSettings`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Whether the player's profile is visible to the currently signed in player."]
        #[serde(
            rename = "profileVisible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_visible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ProfileSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProfileSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileSettingsFriendsListVisibility {
        #[doc = "Unused."]
        FriendsListVisibilityUnspecified,
        #[doc = "The developer does not have access to the friends list, but can call the Android API to show a consent dialog."]
        RequestRequired,
        #[doc = "The friends list is currently unavailable for this user, and it is not possible to request access at this time, either because the user has permanently declined or the friends feature is not available to them. In this state, any attempts to request access to the friends list will be unsuccessful."]
        Unavailable,
        #[doc = "The friends list is currently visible to the game."]
        Visible,
    }
    impl ProfileSettingsFriendsListVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileSettingsFriendsListVisibility::FriendsListVisibilityUnspecified => {
                    "FRIENDS_LIST_VISIBILITY_UNSPECIFIED"
                }
                ProfileSettingsFriendsListVisibility::RequestRequired => "REQUEST_REQUIRED",
                ProfileSettingsFriendsListVisibility::Unavailable => "UNAVAILABLE",
                ProfileSettingsFriendsListVisibility::Visible => "VISIBLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ProfileSettingsFriendsListVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ProfileSettingsFriendsListVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ProfileSettingsFriendsListVisibility, ()> {
            Ok(match s {
                "FRIENDS_LIST_VISIBILITY_UNSPECIFIED" => {
                    ProfileSettingsFriendsListVisibility::FriendsListVisibilityUnspecified
                }
                "REQUEST_REQUIRED" => ProfileSettingsFriendsListVisibility::RequestRequired,
                "UNAVAILABLE" => ProfileSettingsFriendsListVisibility::Unavailable,
                "VISIBLE" => ProfileSettingsFriendsListVisibility::Visible,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ProfileSettingsFriendsListVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileSettingsFriendsListVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileSettingsFriendsListVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FRIENDS_LIST_VISIBILITY_UNSPECIFIED" => {
                    ProfileSettingsFriendsListVisibility::FriendsListVisibilityUnspecified
                }
                "REQUEST_REQUIRED" => ProfileSettingsFriendsListVisibility::RequestRequired,
                "UNAVAILABLE" => ProfileSettingsFriendsListVisibility::Unavailable,
                "VISIBLE" => ProfileSettingsFriendsListVisibility::Visible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ProfileSettingsFriendsListVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProfileSettingsFriendsListVisibility {
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
    pub struct ResolveSnapshotHeadRequest {
        #[doc = "The maximum number of SnapshotRevision resources for `conflictingRevisions` to return per SnapshotExtended resource in the response. For any response, the actual number of resources returned may be less than specified by `maxConflictsPerSnapshot`. The value provided should be greater or equal to 0. If no value is provided, the server will use a sensible default."]
        #[serde(
            rename = "maxConflictsPerSnapshot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_conflicts_per_snapshot: ::std::option::Option<i32>,
        #[doc = "Required. The automatic resolution policy. All conflicts are resolved in chronological order, starting from the/ least recent. If the comparison metric is equal for the tentative head and the conflict, the head wins."]
        #[serde(
            rename = "resolutionPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_policy:
            ::std::option::Option<crate::schemas::ResolveSnapshotHeadRequestResolutionPolicy>,
    }
    impl ::google_field_selector::FieldSelector for ResolveSnapshotHeadRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResolveSnapshotHeadRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ResolveSnapshotHeadRequestResolutionPolicy {
        #[doc = "Use the snapshot with the highest progress value."]
        HighestProgress,
        #[doc = "Use the snapshot with the longest played time."]
        LongestPlaytime,
        #[doc = "Use the snapshot that was most recently modified."]
        MostRecentlyModified,
        #[doc = "Don't resolve conflicts at all. Effectively only returns the current head revision of the snapshot. Corresponds to a game opening the snapshot with manual resolution policy."]
        NoAutomaticResolution,
        #[doc = "Safe default, don't use explicitly."]
        ResolutionPolicyUnspecified,
        #[doc = "Drops all conflicts and keeps the current head only."]
        UseHead,
    }
    impl ResolveSnapshotHeadRequestResolutionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                ResolveSnapshotHeadRequestResolutionPolicy::HighestProgress => "HIGHEST_PROGRESS",
                ResolveSnapshotHeadRequestResolutionPolicy::LongestPlaytime => "LONGEST_PLAYTIME",
                ResolveSnapshotHeadRequestResolutionPolicy::MostRecentlyModified => {
                    "MOST_RECENTLY_MODIFIED"
                }
                ResolveSnapshotHeadRequestResolutionPolicy::NoAutomaticResolution => {
                    "NO_AUTOMATIC_RESOLUTION"
                }
                ResolveSnapshotHeadRequestResolutionPolicy::ResolutionPolicyUnspecified => {
                    "RESOLUTION_POLICY_UNSPECIFIED"
                }
                ResolveSnapshotHeadRequestResolutionPolicy::UseHead => "USE_HEAD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ResolveSnapshotHeadRequestResolutionPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ResolveSnapshotHeadRequestResolutionPolicy {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ResolveSnapshotHeadRequestResolutionPolicy, ()> {
            Ok(match s {
                "HIGHEST_PROGRESS" => ResolveSnapshotHeadRequestResolutionPolicy::HighestProgress,
                "LONGEST_PLAYTIME" => ResolveSnapshotHeadRequestResolutionPolicy::LongestPlaytime,
                "MOST_RECENTLY_MODIFIED" => {
                    ResolveSnapshotHeadRequestResolutionPolicy::MostRecentlyModified
                }
                "NO_AUTOMATIC_RESOLUTION" => {
                    ResolveSnapshotHeadRequestResolutionPolicy::NoAutomaticResolution
                }
                "RESOLUTION_POLICY_UNSPECIFIED" => {
                    ResolveSnapshotHeadRequestResolutionPolicy::ResolutionPolicyUnspecified
                }
                "USE_HEAD" => ResolveSnapshotHeadRequestResolutionPolicy::UseHead,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ResolveSnapshotHeadRequestResolutionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ResolveSnapshotHeadRequestResolutionPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ResolveSnapshotHeadRequestResolutionPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HIGHEST_PROGRESS" => ResolveSnapshotHeadRequestResolutionPolicy::HighestProgress,
                "LONGEST_PLAYTIME" => ResolveSnapshotHeadRequestResolutionPolicy::LongestPlaytime,
                "MOST_RECENTLY_MODIFIED" => {
                    ResolveSnapshotHeadRequestResolutionPolicy::MostRecentlyModified
                }
                "NO_AUTOMATIC_RESOLUTION" => {
                    ResolveSnapshotHeadRequestResolutionPolicy::NoAutomaticResolution
                }
                "RESOLUTION_POLICY_UNSPECIFIED" => {
                    ResolveSnapshotHeadRequestResolutionPolicy::ResolutionPolicyUnspecified
                }
                "USE_HEAD" => ResolveSnapshotHeadRequestResolutionPolicy::UseHead,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ResolveSnapshotHeadRequestResolutionPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResolveSnapshotHeadRequestResolutionPolicy {
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
    pub struct ResolveSnapshotHeadResponse {
        #[doc = "The state of the snapshot."]
        #[serde(
            rename = "snapshot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapshot: ::std::option::Option<crate::schemas::SnapshotExtended>,
    }
    impl ::google_field_selector::FieldSelector for ResolveSnapshotHeadResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResolveSnapshotHeadResponse {
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
    pub struct RevisionCheckResponse {
        #[doc = "The version of the API this client revision should use when calling API methods."]
        #[serde(
            rename = "apiVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_version: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#revisionCheckResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The result of the revision check."]
        #[serde(
            rename = "revisionStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_status:
            ::std::option::Option<crate::schemas::RevisionCheckResponseRevisionStatus>,
    }
    impl ::google_field_selector::FieldSelector for RevisionCheckResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RevisionCheckResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RevisionCheckResponseRevisionStatus {
        #[doc = "There is currently a newer version available, but the revision being used still works."]
        Deprecated,
        #[doc = "The revision being used is not supported in any released version."]
        Invalid,
        #[doc = "The revision being used is current."]
        Ok,
        #[doc = "Default value. This value is unused."]
        RevisionStatusUnspecified,
    }
    impl RevisionCheckResponseRevisionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                RevisionCheckResponseRevisionStatus::Deprecated => "DEPRECATED",
                RevisionCheckResponseRevisionStatus::Invalid => "INVALID",
                RevisionCheckResponseRevisionStatus::Ok => "OK",
                RevisionCheckResponseRevisionStatus::RevisionStatusUnspecified => {
                    "REVISION_STATUS_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for RevisionCheckResponseRevisionStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RevisionCheckResponseRevisionStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RevisionCheckResponseRevisionStatus, ()> {
            Ok(match s {
                "DEPRECATED" => RevisionCheckResponseRevisionStatus::Deprecated,
                "INVALID" => RevisionCheckResponseRevisionStatus::Invalid,
                "OK" => RevisionCheckResponseRevisionStatus::Ok,
                "REVISION_STATUS_UNSPECIFIED" => {
                    RevisionCheckResponseRevisionStatus::RevisionStatusUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RevisionCheckResponseRevisionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RevisionCheckResponseRevisionStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RevisionCheckResponseRevisionStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPRECATED" => RevisionCheckResponseRevisionStatus::Deprecated,
                "INVALID" => RevisionCheckResponseRevisionStatus::Invalid,
                "OK" => RevisionCheckResponseRevisionStatus::Ok,
                "REVISION_STATUS_UNSPECIFIED" => {
                    RevisionCheckResponseRevisionStatus::RevisionStatusUnspecified
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
    impl ::google_field_selector::FieldSelector for RevisionCheckResponseRevisionStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RevisionCheckResponseRevisionStatus {
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
    pub struct ScoreSubmission {
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#scoreSubmission`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The leaderboard this score is being submitted to."]
        #[serde(
            rename = "leaderboardId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub leaderboard_id: ::std::option::Option<String>,
        #[doc = "The new score being submitted."]
        #[serde(
            rename = "score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub score: ::std::option::Option<i64>,
        #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        #[serde(
            rename = "scoreTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_tag: ::std::option::Option<String>,
        #[doc = "Signature Values will contain URI-safe characters as defined by section 2.3 of RFC 3986."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ScoreSubmission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScoreSubmission {
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
    pub struct Snapshot {
        #[doc = "The cover image of this snapshot. May be absent if there is no image."]
        #[serde(
            rename = "coverImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cover_image: ::std::option::Option<crate::schemas::SnapshotImage>,
        #[doc = "The description of this snapshot."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The ID of the file underlying this snapshot in the Drive API. Only present if the snapshot is a view on a Drive file and the file is owned by the caller."]
        #[serde(
            rename = "driveId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_id: ::std::option::Option<String>,
        #[doc = "The duration associated with this snapshot, in millis."]
        #[serde(
            rename = "durationMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub duration_millis: ::std::option::Option<i64>,
        #[doc = "The ID of the snapshot."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshot`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The timestamp (in millis since Unix epoch) of the last modification to this snapshot."]
        #[serde(
            rename = "lastModifiedMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_millis: ::std::option::Option<i64>,
        #[doc = "The progress value (64-bit integer set by developer) associated with this snapshot."]
        #[serde(
            rename = "progressValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub progress_value: ::std::option::Option<i64>,
        #[doc = "The type of this snapshot."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SnapshotType>,
        #[doc = "The title of this snapshot."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The unique name provided when the snapshot was created."]
        #[serde(
            rename = "uniqueName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Snapshot {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Snapshot {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SnapshotType {
        #[doc = "A snapshot representing a save game."]
        SaveGame,
        #[doc = "Default value. This value is unused."]
        SnapshotTypeUnspecified,
    }
    impl SnapshotType {
        pub fn as_str(self) -> &'static str {
            match self {
                SnapshotType::SaveGame => "SAVE_GAME",
                SnapshotType::SnapshotTypeUnspecified => "SNAPSHOT_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SnapshotType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SnapshotType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SnapshotType, ()> {
            Ok(match s {
                "SAVE_GAME" => SnapshotType::SaveGame,
                "SNAPSHOT_TYPE_UNSPECIFIED" => SnapshotType::SnapshotTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SnapshotType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SnapshotType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SnapshotType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SAVE_GAME" => SnapshotType::SaveGame,
                "SNAPSHOT_TYPE_UNSPECIFIED" => SnapshotType::SnapshotTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SnapshotType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotType {
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
    pub struct SnapshotCoverImageResource {
        #[doc = "Output only. Hash-like weak identifier of the uploaded image bytes, consistent per player per application. The content hash for a given resource will not change if the binary data hasn't changed. Except in very rare circumstances, the content_hash for matching binary data will be the same within a given player and application."]
        #[serde(
            rename = "contentHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_hash: ::std::option::Option<String>,
        #[doc = "Output only. A URL the client can use to download the image. May vary across requests, and only guaranteed to be valid for a short time after it is returned."]
        #[serde(
            rename = "downloadUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_url: ::std::option::Option<String>,
        #[doc = "The height of the image in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Output only. The MIME type of the image."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The ID of the image resource. It's guaranteed that if two IDs are equal then the contents are equal as well. It's not guaranteed that two identical blobs coming from separate uploads have the same ID. The resource ID can only be used within the application, user and resource type it was originally returned for. For example, it's not possible to use SnapshotDataResource's resource ID as the resource_id of a SnapshotCoverImageResource, even if the blob is a valid image file."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "The width of the image in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotCoverImageResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotCoverImageResource {
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
    pub struct SnapshotDataResource {
        #[doc = "Output only. Hash-like weak identifier of the uploaded blob bytes, consistent per player per application. The content hash for a given resource will not change if the binary data hasn't changed. Except in very rare circumstances, the content_hash for matching binary data will be the same within a given player and application."]
        #[serde(
            rename = "contentHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_hash: ::std::option::Option<String>,
        #[doc = "Output only. A URL that the client can use to download the blob. May vary across requests, and only guaranteed to be valid for a short time after it is returned."]
        #[serde(
            rename = "downloadUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_url: ::std::option::Option<String>,
        #[doc = "The ID of the blob resource. It's guaranteed that if two IDs are equal then the contents are equal as well. It's not guaranteed that two identical blobs coming from separate uploads have the same resource ID. The resource ID can only be used within the application, user and resource type it was originally returned for. For example, it's not possible to use SnapshotDataResource's resource ID as the resource_id of a SnapshotCoverImageResource, even if the blob is a valid image file."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "Output only. Size of the saved game blob in bytes."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub size: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotDataResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotDataResource {
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
    pub struct SnapshotExtended {
        #[doc = "A list of conflicting revisions. Only set if explicitly requested (e.g. using a field mask or a request flag), or if the RPC guarantees that this field is set. The conflicting revisions are sorted chronologically by their server creation time (oldest first). If there are too many conflicting revisions to return all of them in a single request this will only contain the first batch. In such case, the presented conflicting revisions must be resolved first in order to fetch the next batch."]
        #[serde(
            rename = "conflictingRevisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conflicting_revisions: ::std::option::Option<Vec<crate::schemas::SnapshotRevision>>,
        #[doc = "An indicator whether the snapshot has any conflicting revisions or not. Always set."]
        #[serde(
            rename = "hasConflictingRevisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_conflicting_revisions: ::std::option::Option<bool>,
        #[doc = "The current head revision (the canonical revision as understood by the server)."]
        #[serde(
            rename = "headRevision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub head_revision: ::std::option::Option<crate::schemas::SnapshotRevision>,
        #[doc = "An identifier of the snapshot, developer-specified. It must match the pattern [0-9a-zA-Z-._~]{1,100}."]
        #[serde(
            rename = "snapshotName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapshot_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotExtended {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotExtended {
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
    pub struct SnapshotImage {
        #[doc = "The height of the image."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotImage`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The MIME type of the image."]
        #[serde(
            rename = "mime_type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The URL of the image. This URL may be invalidated at any time and should not be cached."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[doc = "The width of the image."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotImage {
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
    pub struct SnapshotListResponse {
        #[doc = "The snapshots."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Snapshot>>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotListResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token corresponding to the next page of results. If there are no more results, the token is omitted."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotListResponse {
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
    pub struct SnapshotMetadata {
        #[doc = "The description of this snapshot."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The device that created the current revision."]
        #[serde(
            rename = "deviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_name: ::std::option::Option<String>,
        #[doc = "The duration associated with this snapshot. Values with sub-millisecond precision can be rounded or trimmed to the closest millisecond."]
        #[serde(
            rename = "gameplayDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gameplay_duration: ::std::option::Option<String>,
        #[doc = "The timestamp of the last modification to this snapshot as provided by the client. Values with sub-millisecond precision can be rounded or trimmed to the closest millisecond."]
        #[serde(
            rename = "lastModifyTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_modify_time: ::std::option::Option<String>,
        #[doc = "The progress value (64-bit integer set by developer) associated with this snapshot."]
        #[serde(
            rename = "progressValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub progress_value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotMetadata {
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
    pub struct SnapshotRevision {
        #[doc = "Reference to the game provided blob for this revision."]
        #[serde(
            rename = "blob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blob: ::std::option::Option<crate::schemas::SnapshotDataResource>,
        #[doc = "Reference to the cover image for this revision."]
        #[serde(
            rename = "coverImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cover_image: ::std::option::Option<crate::schemas::SnapshotCoverImageResource>,
        #[doc = "Output only. A server generated identifier of the snapshot revision."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Metadata for this snapshot revision."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::SnapshotMetadata>,
    }
    impl ::google_field_selector::FieldSelector for SnapshotRevision {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SnapshotRevision {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StatsResponse {
        #[doc = "Average session length in minutes of the player. E.g., 1, 30, 60, ... . Not populated if there is not enough information."]
        #[serde(
            rename = "avg_session_length_minutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub avg_session_length_minutes: ::std::option::Option<f32>,
        #[doc = "The probability of the player not returning to play the game in the next day. E.g., 0, 0.1, 0.5, ..., 1.0. Not populated if there is not enough information."]
        #[serde(
            rename = "churn_probability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub churn_probability: ::std::option::Option<f32>,
        #[doc = "Number of days since the player last played this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
        #[serde(
            rename = "days_since_last_played",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub days_since_last_played: ::std::option::Option<i32>,
        #[doc = "The probability of the player going to spend beyond a threshold amount of money. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information."]
        #[serde(
            rename = "high_spender_probability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_spender_probability: ::std::option::Option<f32>,
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#statsResponse`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Number of in-app purchases made by the player in this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
        #[serde(
            rename = "num_purchases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_purchases: ::std::option::Option<i32>,
        #[doc = "The approximate number of sessions of the player within the last 28 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
        #[serde(
            rename = "num_sessions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_sessions: ::std::option::Option<i32>,
        #[doc = "The approximation of the sessions percentile of the player within the last 30 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information."]
        #[serde(
            rename = "num_sessions_percentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_sessions_percentile: ::std::option::Option<f32>,
        #[doc = "The approximate spend percentile of the player in this game. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information."]
        #[serde(
            rename = "spend_percentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spend_percentile: ::std::option::Option<f32>,
        #[doc = "The probability of the player going to spend the game in the next seven days. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information."]
        #[serde(
            rename = "spend_probability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spend_probability: ::std::option::Option<f32>,
        #[doc = "The predicted amount of money that the player going to spend in the next 28 days. E.g., 1, 30, 60, ... . Not populated if there is not enough information."]
        #[serde(
            rename = "total_spend_next_28_days",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_spend_next_28_days: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for StatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StatsResponse {
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
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
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
    #[doc = "Actions that can be performed on the achievement_definitions resource"]
    pub fn achievement_definitions(
        &self,
    ) -> crate::resources::achievement_definitions::AchievementDefinitionsActions {
        crate::resources::achievement_definitions::AchievementDefinitionsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the achievements resource"]
    pub fn achievements(&self) -> crate::resources::achievements::AchievementsActions {
        crate::resources::achievements::AchievementsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the applications resource"]
    pub fn applications(&self) -> crate::resources::applications::ApplicationsActions {
        crate::resources::applications::ApplicationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the events resource"]
    pub fn events(&self) -> crate::resources::events::EventsActions {
        crate::resources::events::EventsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the leaderboards resource"]
    pub fn leaderboards(&self) -> crate::resources::leaderboards::LeaderboardsActions {
        crate::resources::leaderboards::LeaderboardsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the metagame resource"]
    pub fn metagame(&self) -> crate::resources::metagame::MetagameActions {
        crate::resources::metagame::MetagameActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the players resource"]
    pub fn players(&self) -> crate::resources::players::PlayersActions {
        crate::resources::players::PlayersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the revisions resource"]
    pub fn revisions(&self) -> crate::resources::revisions::RevisionsActions {
        crate::resources::revisions::RevisionsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the scores resource"]
    pub fn scores(&self) -> crate::resources::scores::ScoresActions {
        crate::resources::scores::ScoresActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the snapshots resource"]
    pub fn snapshots(&self) -> crate::resources::snapshots::SnapshotsActions {
        crate::resources::snapshots::SnapshotsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the snapshots_extended resource"]
    pub fn snapshots_extended(
        &self,
    ) -> crate::resources::snapshots_extended::SnapshotsExtendedActions {
        crate::resources::snapshots_extended::SnapshotsExtendedActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the stats resource"]
    pub fn stats(&self) -> crate::resources::stats::StatsActions {
        crate::resources::stats::StatsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod achievement_definitions {
        pub mod params {}
        pub struct AchievementDefinitionsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AchievementDefinitionsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Lists all the achievement definitions for your application."]
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
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [AchievementDefinitionsActions::list()](struct.AchievementDefinitionsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of achievement resources to return in the response, used for paging. For any response, the actual number of achievement resources returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AchievementDefinitionsListResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AchievementDefinitionsListResponse, crate::Error>
            {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/achievements");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod achievements {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListState {
                #[doc = "List all achievements. This is the default."]
                All,
                #[doc = "List only hidden achievements."]
                Hidden,
                #[doc = "List only revealed achievements."]
                Revealed,
                #[doc = "List only unlocked achievements."]
                Unlocked,
            }
            impl ListState {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListState::All => "ALL",
                        ListState::Hidden => "HIDDEN",
                        ListState::Revealed => "REVEALED",
                        ListState::Unlocked => "UNLOCKED",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListState {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListState {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListState, ()> {
                    Ok(match s {
                        "ALL" => ListState::All,
                        "HIDDEN" => ListState::Hidden,
                        "REVEALED" => ListState::Revealed,
                        "UNLOCKED" => ListState::Unlocked,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListState {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListState {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListState {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ALL" => ListState::All,
                        "HIDDEN" => ListState::Hidden,
                        "REVEALED" => ListState::Revealed,
                        "UNLOCKED" => ListState::Unlocked,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListState {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListState {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct AchievementsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AchievementsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Increments the steps of the achievement with the given ID for the currently authenticated player."]
            pub fn increment(
                &self,
                achievement_id: impl Into<String>,
                steps_to_increment: i32,
            ) -> IncrementRequestBuilder {
                IncrementRequestBuilder {
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
                    achievement_id: achievement_id.into(),
                    steps_to_increment,
                    request_id: None,
                }
            }
            #[doc = "Lists the progress for all your application's achievements for the currently authenticated player."]
            pub fn list(&self, player_id: impl Into<String>) -> ListRequestBuilder {
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
                    player_id: player_id.into(),
                    language: None,
                    max_results: None,
                    page_token: None,
                    state: None,
                }
            }
            #[doc = "Sets the state of the achievement with the given ID to `REVEALED` for the currently authenticated player."]
            pub fn reveal(&self, achievement_id: impl Into<String>) -> RevealRequestBuilder {
                RevealRequestBuilder {
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
                    achievement_id: achievement_id.into(),
                }
            }
            #[doc = "Sets the steps for the currently authenticated player towards unlocking an achievement. If the steps parameter is less than the current number of steps that the player already gained for the achievement, the achievement is not modified."]
            pub fn set_steps_at_least(
                &self,
                achievement_id: impl Into<String>,
                steps: i32,
            ) -> SetStepsAtLeastRequestBuilder {
                SetStepsAtLeastRequestBuilder {
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
                    achievement_id: achievement_id.into(),
                    steps,
                }
            }
            #[doc = "Unlocks this achievement for the currently authenticated player."]
            pub fn unlock(&self, achievement_id: impl Into<String>) -> UnlockRequestBuilder {
                UnlockRequestBuilder {
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
                    achievement_id: achievement_id.into(),
                }
            }
            #[doc = "Updates multiple achievements for the currently authenticated player."]
            pub fn update_multiple(
                &self,
                request: crate::schemas::AchievementUpdateMultipleRequest,
            ) -> UpdateMultipleRequestBuilder {
                UpdateMultipleRequestBuilder {
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
        #[doc = "Created via [AchievementsActions::increment()](struct.AchievementsActions.html#method.increment)"]
        #[derive(Debug, Clone)]
        pub struct IncrementRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            achievement_id: String,
            steps_to_increment: i32,
            request_id: Option<i64>,
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
        impl<'a> IncrementRequestBuilder<'a> {
            #[doc = "A randomly generated numeric ID for each request specified by the caller. This number is used at the server to ensure that the request is handled correctly across retries."]
            pub fn request_id(mut self, value: i64) -> Self {
                self.request_id = Some(value);
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AchievementIncrementResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AchievementIncrementResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/achievements/");
                {
                    let var_as_str = &self.achievement_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/increment");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("stepsToIncrement", &self.steps_to_increment)]);
                req = req.query(&[("requestId", &self.request_id)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [AchievementsActions::list()](struct.AchievementsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            player_id: String,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
            state: Option<crate::resources::achievements::params::ListState>,
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
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of achievement resources to return in the response, used for paging. For any response, the actual number of achievement resources returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Tells the server to return only achievements with the specified state. If this parameter isn't specified, all achievements are returned."]
            pub fn state(
                mut self,
                value: crate::resources::achievements::params::ListState,
            ) -> Self {
                self.state = Some(value);
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::PlayerAchievementListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PlayerAchievementListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/players/");
                {
                    let var_as_str = &self.player_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/achievements");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("state", &self.state)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [AchievementsActions::reveal()](struct.AchievementsActions.html#method.reveal)"]
        #[derive(Debug, Clone)]
        pub struct RevealRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            achievement_id: String,
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
        impl<'a> RevealRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AchievementRevealResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AchievementRevealResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/achievements/");
                {
                    let var_as_str = &self.achievement_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/reveal");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [AchievementsActions::set_steps_at_least()](struct.AchievementsActions.html#method.set_steps_at_least)"]
        #[derive(Debug, Clone)]
        pub struct SetStepsAtLeastRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            achievement_id: String,
            steps: i32,
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
        impl<'a> SetStepsAtLeastRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AchievementSetStepsAtLeastResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AchievementSetStepsAtLeastResponse, crate::Error>
            {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/achievements/");
                {
                    let var_as_str = &self.achievement_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/setStepsAtLeast");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("steps", &self.steps)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [AchievementsActions::unlock()](struct.AchievementsActions.html#method.unlock)"]
        #[derive(Debug, Clone)]
        pub struct UnlockRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            achievement_id: String,
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
        impl<'a> UnlockRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AchievementUnlockResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AchievementUnlockResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/achievements/");
                {
                    let var_as_str = &self.achievement_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/unlock");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [AchievementsActions::update_multiple()](struct.AchievementsActions.html#method.update_multiple)"]
        #[derive(Debug, Clone)]
        pub struct UpdateMultipleRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::AchievementUpdateMultipleRequest,
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
        impl<'a> UpdateMultipleRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AchievementUpdateMultipleResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AchievementUpdateMultipleResponse, crate::Error>
            {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/achievements/updateMultiple");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod applications {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetPlatformType {
                #[doc = "Retrieve applications that can be played on Android."]
                Android,
                #[doc = "Retrieve applications that can be played on iOS."]
                Ios,
                #[doc = "Default value, don't use."]
                PlatformTypeUnspecified,
                #[doc = "Retrieve applications that can be played on desktop web."]
                WebApp,
            }
            impl GetPlatformType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetPlatformType::Android => "ANDROID",
                        GetPlatformType::Ios => "IOS",
                        GetPlatformType::PlatformTypeUnspecified => "PLATFORM_TYPE_UNSPECIFIED",
                        GetPlatformType::WebApp => "WEB_APP",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetPlatformType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetPlatformType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetPlatformType, ()> {
                    Ok(match s {
                        "ANDROID" => GetPlatformType::Android,
                        "IOS" => GetPlatformType::Ios,
                        "PLATFORM_TYPE_UNSPECIFIED" => GetPlatformType::PlatformTypeUnspecified,
                        "WEB_APP" => GetPlatformType::WebApp,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetPlatformType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetPlatformType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetPlatformType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ANDROID" => GetPlatformType::Android,
                        "IOS" => GetPlatformType::Ios,
                        "PLATFORM_TYPE_UNSPECIFIED" => GetPlatformType::PlatformTypeUnspecified,
                        "WEB_APP" => GetPlatformType::WebApp,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetPlatformType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetPlatformType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetEndPointEndPointType {
                #[doc = "Default value. This value is unused."]
                EndPointTypeUnspecified,
                #[doc = "Request a URL to create a new profile."]
                ProfileCreation,
                #[doc = "Request a URL for the Settings view."]
                ProfileSettings,
            }
            impl GetEndPointEndPointType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetEndPointEndPointType::EndPointTypeUnspecified => {
                            "END_POINT_TYPE_UNSPECIFIED"
                        }
                        GetEndPointEndPointType::ProfileCreation => "PROFILE_CREATION",
                        GetEndPointEndPointType::ProfileSettings => "PROFILE_SETTINGS",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetEndPointEndPointType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetEndPointEndPointType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetEndPointEndPointType, ()> {
                    Ok(match s {
                        "END_POINT_TYPE_UNSPECIFIED" => {
                            GetEndPointEndPointType::EndPointTypeUnspecified
                        }
                        "PROFILE_CREATION" => GetEndPointEndPointType::ProfileCreation,
                        "PROFILE_SETTINGS" => GetEndPointEndPointType::ProfileSettings,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetEndPointEndPointType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetEndPointEndPointType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetEndPointEndPointType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "END_POINT_TYPE_UNSPECIFIED" => {
                            GetEndPointEndPointType::EndPointTypeUnspecified
                        }
                        "PROFILE_CREATION" => GetEndPointEndPointType::ProfileCreation,
                        "PROFILE_SETTINGS" => GetEndPointEndPointType::ProfileSettings,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetEndPointEndPointType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetEndPointEndPointType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ApplicationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ApplicationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves the metadata of the application with the given ID. If the requested application is not available for the specified `platformType`, the returned response will not include any instance data."]
            pub fn get(&self, application_id: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
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
                    application_id: application_id.into(),
                    language: None,
                    platform_type: None,
                }
            }
            #[doc = "Returns a URL for the requested end point type."]
            pub fn get_end_point(&self) -> GetEndPointRequestBuilder {
                GetEndPointRequestBuilder {
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
                    application_id: None,
                    end_point_type: None,
                }
            }
            #[doc = "Indicate that the currently authenticated user is playing your application."]
            pub fn played(&self) -> PlayedRequestBuilder {
                PlayedRequestBuilder {
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
                }
            }
            #[doc = "Verifies the auth token provided with this request is for the application with the specified ID, and returns the ID of the player it was granted for."]
            pub fn verify(&self, application_id: impl Into<String>) -> VerifyRequestBuilder {
                VerifyRequestBuilder {
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
                    application_id: application_id.into(),
                }
            }
        }
        #[doc = "Created via [ApplicationsActions::get()](struct.ApplicationsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            application_id: String,
            language: Option<String>,
            platform_type: Option<crate::resources::applications::params::GetPlatformType>,
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
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "Restrict application details returned to the specific platform."]
            pub fn platform_type(
                mut self,
                value: crate::resources::applications::params::GetPlatformType,
            ) -> Self {
                self.platform_type = Some(value);
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Application, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Application, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/applications/");
                {
                    let var_as_str = &self.application_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("platformType", &self.platform_type)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ApplicationsActions::get_end_point()](struct.ApplicationsActions.html#method.get_end_point)"]
        #[derive(Debug, Clone)]
        pub struct GetEndPointRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            application_id: Option<String>,
            end_point_type: Option<crate::resources::applications::params::GetEndPointEndPointType>,
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
        impl<'a> GetEndPointRequestBuilder<'a> {
            #[doc = "The application ID from the Google Play developer console."]
            pub fn application_id(mut self, value: impl Into<String>) -> Self {
                self.application_id = Some(value.into());
                self
            }
            #[doc = "Type of endpoint being requested."]
            pub fn end_point_type(
                mut self,
                value: crate::resources::applications::params::GetEndPointEndPointType,
            ) -> Self {
                self.end_point_type = Some(value);
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::EndPoint, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EndPoint, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/applications/getEndPoint");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("applicationId", &self.application_id)]);
                req = req.query(&[("endPointType", &self.end_point_type)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ApplicationsActions::played()](struct.ApplicationsActions.html#method.played)"]
        #[derive(Debug, Clone)]
        pub struct PlayedRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> PlayedRequestBuilder<'a> {
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
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/applications/played");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ApplicationsActions::verify()](struct.ApplicationsActions.html#method.verify)"]
        #[derive(Debug, Clone)]
        pub struct VerifyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            application_id: String,
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
        impl<'a> VerifyRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ApplicationVerifyResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ApplicationVerifyResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/applications/");
                {
                    let var_as_str = &self.application_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/verify");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod events {
        pub mod params {}
        pub struct EventsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> EventsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a list showing the current progress on events in this application for the currently authenticated user."]
            pub fn list_by_player(&self) -> ListByPlayerRequestBuilder {
                ListByPlayerRequestBuilder {
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
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Returns a list of the event definitions in this application."]
            pub fn list_definitions(&self) -> ListDefinitionsRequestBuilder {
                ListDefinitionsRequestBuilder {
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
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Records a batch of changes to the number of times events have occurred for the currently authenticated user of this application."]
            pub fn record(
                &self,
                request: crate::schemas::EventRecordRequest,
            ) -> RecordRequestBuilder {
                RecordRequestBuilder {
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
                    language: None,
                }
            }
        }
        #[doc = "Created via [EventsActions::list_by_player()](struct.EventsActions.html#method.list_by_player)"]
        #[derive(Debug, Clone)]
        pub struct ListByPlayerRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
        impl<'a> ListByPlayerRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of events to return in the response, used for paging. For any response, the actual number of events to return may be less than the specified maxResults."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::PlayerEventListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PlayerEventListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/events");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [EventsActions::list_definitions()](struct.EventsActions.html#method.list_definitions)"]
        #[derive(Debug, Clone)]
        pub struct ListDefinitionsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
        impl<'a> ListDefinitionsRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of event definitions to return in the response, used for paging. For any response, the actual number of event definitions to return may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::EventDefinitionListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EventDefinitionListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/eventDefinitions");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [EventsActions::record()](struct.EventsActions.html#method.record)"]
        #[derive(Debug, Clone)]
        pub struct RecordRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::EventRecordRequest,
            language: Option<String>,
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
        impl<'a> RecordRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::EventUpdateResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EventUpdateResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/events");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod leaderboards {
        pub mod params {}
        pub struct LeaderboardsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> LeaderboardsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves the metadata of the leaderboard with the given ID."]
            pub fn get(&self, leaderboard_id: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
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
                    leaderboard_id: leaderboard_id.into(),
                    language: None,
                }
            }
            #[doc = "Lists all the leaderboard metadata for your application."]
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
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [LeaderboardsActions::get()](struct.LeaderboardsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            leaderboard_id: String,
            language: Option<String>,
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
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Leaderboard, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Leaderboard, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/leaderboards/");
                {
                    let var_as_str = &self.leaderboard_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [LeaderboardsActions::list()](struct.LeaderboardsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of leaderboards to return in the response. For any response, the actual number of leaderboards returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::LeaderboardListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::LeaderboardListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/leaderboards");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod metagame {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListCategoriesByPlayerCollection {
                #[doc = "Retrieve data for all categories. This is the default."]
                All,
                #[doc = "Default value. This value is unused."]
                CollectionUnspecified,
            }
            impl ListCategoriesByPlayerCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListCategoriesByPlayerCollection::All => "ALL",
                        ListCategoriesByPlayerCollection::CollectionUnspecified => {
                            "COLLECTION_UNSPECIFIED"
                        }
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListCategoriesByPlayerCollection {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListCategoriesByPlayerCollection {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<ListCategoriesByPlayerCollection, ()> {
                    Ok(match s {
                        "ALL" => ListCategoriesByPlayerCollection::All,
                        "COLLECTION_UNSPECIFIED" => {
                            ListCategoriesByPlayerCollection::CollectionUnspecified
                        }
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListCategoriesByPlayerCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListCategoriesByPlayerCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListCategoriesByPlayerCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ALL" => ListCategoriesByPlayerCollection::All,
                        "COLLECTION_UNSPECIFIED" => {
                            ListCategoriesByPlayerCollection::CollectionUnspecified
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
            impl ::google_field_selector::FieldSelector for ListCategoriesByPlayerCollection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListCategoriesByPlayerCollection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct MetagameActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MetagameActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Return the metagame configuration data for the calling application."]
            pub fn get_metagame_config(&self) -> GetMetagameConfigRequestBuilder {
                GetMetagameConfigRequestBuilder {
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
                }
            }
            #[doc = "List play data aggregated per category for the player corresponding to `playerId`."]
            pub fn list_categories_by_player(
                &self,
                player_id: impl Into<String>,
                collection: crate::resources::metagame::params::ListCategoriesByPlayerCollection,
            ) -> ListCategoriesByPlayerRequestBuilder {
                ListCategoriesByPlayerRequestBuilder {
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
                    player_id: player_id.into(),
                    collection,
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [MetagameActions::get_metagame_config()](struct.MetagameActions.html#method.get_metagame_config)"]
        #[derive(Debug, Clone)]
        pub struct GetMetagameConfigRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> GetMetagameConfigRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::MetagameConfig, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::MetagameConfig, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/metagameConfig");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [MetagameActions::list_categories_by_player()](struct.MetagameActions.html#method.list_categories_by_player)"]
        #[derive(Debug, Clone)]
        pub struct ListCategoriesByPlayerRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            player_id: String,
            collection: crate::resources::metagame::params::ListCategoriesByPlayerCollection,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
        impl<'a> ListCategoriesByPlayerRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of category resources to return in the response, used for paging. For any response, the actual number of category resources returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::CategoryListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CategoryListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/players/");
                {
                    let var_as_str = &self.player_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/categories/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod players {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListCollection {
                #[doc = "Retrieve a list of players that are also playing this game in reverse chronological order."]
                Connected,
                #[doc = "Retrieve a list of players who are friends of the user in alphabetical order."]
                FriendsAll,
                #[doc = "Retrieve a list of players in the user's social graph that are visible to this game."]
                Visible,
            }
            impl ListCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListCollection::Connected => "CONNECTED",
                        ListCollection::FriendsAll => "FRIENDS_ALL",
                        ListCollection::Visible => "VISIBLE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListCollection {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListCollection {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListCollection, ()> {
                    Ok(match s {
                        "CONNECTED" => ListCollection::Connected,
                        "FRIENDS_ALL" => ListCollection::FriendsAll,
                        "VISIBLE" => ListCollection::Visible,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "CONNECTED" => ListCollection::Connected,
                        "FRIENDS_ALL" => ListCollection::FriendsAll,
                        "VISIBLE" => ListCollection::Visible,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListCollection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListCollection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct PlayersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PlayersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves the Player resource with the given ID. To retrieve the player for the currently authenticated user, set `playerId` to `me`."]
            pub fn get(&self, player_id: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
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
                    player_id: player_id.into(),
                    language: None,
                }
            }
            #[doc = "Get the collection of players for the currently authenticated user."]
            pub fn list(
                &self,
                collection: crate::resources::players::params::ListCollection,
            ) -> ListRequestBuilder {
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
                    collection,
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [PlayersActions::get()](struct.PlayersActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            player_id: String,
            language: Option<String>,
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
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Player, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Player, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/players/");
                {
                    let var_as_str = &self.player_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [PlayersActions::list()](struct.PlayersActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            collection: crate::resources::players::params::ListCollection,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of player resources to return in the response, used for paging. For any response, the actual number of player resources returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::PlayerListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PlayerListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/players/me/players/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod revisions {
        pub mod params {}
        pub struct RevisionsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> RevisionsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Checks whether the games client is out of date."]
            pub fn check(&self, client_revision: impl Into<String>) -> CheckRequestBuilder {
                CheckRequestBuilder {
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
                    client_revision: client_revision.into(),
                }
            }
        }
        #[doc = "Created via [RevisionsActions::check()](struct.RevisionsActions.html#method.check)"]
        #[derive(Debug, Clone)]
        pub struct CheckRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            client_revision: String,
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
        impl<'a> CheckRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::RevisionCheckResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RevisionCheckResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/revisions/check");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("clientRevision", &self.client_revision)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod scores {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetTimeSpan {
                #[doc = "Get the high scores for all time spans. If this is used, maxResults values will be ignored."]
                All,
                #[doc = "Get the all time high score."]
                AllTime,
                #[doc = "List the top scores for the current week."]
                Daily,
                #[doc = "Default value. This value is unused."]
                ScoreTimeSpanUnspecified,
                #[doc = "List the top scores for the current day."]
                Weekly,
            }
            impl GetTimeSpan {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetTimeSpan::All => "ALL",
                        GetTimeSpan::AllTime => "ALL_TIME",
                        GetTimeSpan::Daily => "DAILY",
                        GetTimeSpan::ScoreTimeSpanUnspecified => "SCORE_TIME_SPAN_UNSPECIFIED",
                        GetTimeSpan::Weekly => "WEEKLY",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetTimeSpan {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetTimeSpan {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetTimeSpan, ()> {
                    Ok(match s {
                        "ALL" => GetTimeSpan::All,
                        "ALL_TIME" => GetTimeSpan::AllTime,
                        "DAILY" => GetTimeSpan::Daily,
                        "SCORE_TIME_SPAN_UNSPECIFIED" => GetTimeSpan::ScoreTimeSpanUnspecified,
                        "WEEKLY" => GetTimeSpan::Weekly,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetTimeSpan {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetTimeSpan {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetTimeSpan {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ALL" => GetTimeSpan::All,
                        "ALL_TIME" => GetTimeSpan::AllTime,
                        "DAILY" => GetTimeSpan::Daily,
                        "SCORE_TIME_SPAN_UNSPECIFIED" => GetTimeSpan::ScoreTimeSpanUnspecified,
                        "WEEKLY" => GetTimeSpan::Weekly,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetTimeSpan {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetTimeSpan {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetIncludeRankType {
                #[doc = "Retrieve all supported ranks. In HTTP, this parameter value can also be specified as `ALL`."]
                All,
                #[doc = "Retrieve the rank on the friends collection."]
                Friends,
                #[doc = "Default value. Should be unused."]
                IncludeRankTypeUnspecified,
                #[doc = "Retrieve public ranks, if the player is sharing their gameplay activity publicly."]
                Public,
                #[doc = "(Obsolete) Retrieve the social rank."]
                Social,
            }
            impl GetIncludeRankType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetIncludeRankType::All => "ALL",
                        GetIncludeRankType::Friends => "FRIENDS",
                        GetIncludeRankType::IncludeRankTypeUnspecified => {
                            "INCLUDE_RANK_TYPE_UNSPECIFIED"
                        }
                        GetIncludeRankType::Public => "PUBLIC",
                        GetIncludeRankType::Social => "SOCIAL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetIncludeRankType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetIncludeRankType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetIncludeRankType, ()> {
                    Ok(match s {
                        "ALL" => GetIncludeRankType::All,
                        "FRIENDS" => GetIncludeRankType::Friends,
                        "INCLUDE_RANK_TYPE_UNSPECIFIED" => {
                            GetIncludeRankType::IncludeRankTypeUnspecified
                        }
                        "PUBLIC" => GetIncludeRankType::Public,
                        "SOCIAL" => GetIncludeRankType::Social,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetIncludeRankType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetIncludeRankType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetIncludeRankType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ALL" => GetIncludeRankType::All,
                        "FRIENDS" => GetIncludeRankType::Friends,
                        "INCLUDE_RANK_TYPE_UNSPECIFIED" => {
                            GetIncludeRankType::IncludeRankTypeUnspecified
                        }
                        "PUBLIC" => GetIncludeRankType::Public,
                        "SOCIAL" => GetIncludeRankType::Social,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetIncludeRankType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetIncludeRankType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListCollection {
                #[doc = "List only scores of friends."]
                Friends,
                #[doc = "List all scores in the public leaderboard."]
                Public,
                #[doc = "Default value. This value is unused."]
                ScoreCollectionUnspecified,
                #[doc = "(Obsolete) Legacy G+ social scores."]
                Social,
            }
            impl ListCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListCollection::Friends => "FRIENDS",
                        ListCollection::Public => "PUBLIC",
                        ListCollection::ScoreCollectionUnspecified => {
                            "SCORE_COLLECTION_UNSPECIFIED"
                        }
                        ListCollection::Social => "SOCIAL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListCollection {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListCollection {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListCollection, ()> {
                    Ok(match s {
                        "FRIENDS" => ListCollection::Friends,
                        "PUBLIC" => ListCollection::Public,
                        "SCORE_COLLECTION_UNSPECIFIED" => {
                            ListCollection::ScoreCollectionUnspecified
                        }
                        "SOCIAL" => ListCollection::Social,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "FRIENDS" => ListCollection::Friends,
                        "PUBLIC" => ListCollection::Public,
                        "SCORE_COLLECTION_UNSPECIFIED" => {
                            ListCollection::ScoreCollectionUnspecified
                        }
                        "SOCIAL" => ListCollection::Social,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListCollection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListCollection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListTimeSpan {
                #[doc = "The score is an all-time score."]
                AllTime,
                #[doc = "The score is a daily score."]
                Daily,
                #[doc = "Default value. This value is unused."]
                ScoreTimeSpanUnspecified,
                #[doc = "The score is a weekly score."]
                Weekly,
            }
            impl ListTimeSpan {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListTimeSpan::AllTime => "ALL_TIME",
                        ListTimeSpan::Daily => "DAILY",
                        ListTimeSpan::ScoreTimeSpanUnspecified => "SCORE_TIME_SPAN_UNSPECIFIED",
                        ListTimeSpan::Weekly => "WEEKLY",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListTimeSpan {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListTimeSpan {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListTimeSpan, ()> {
                    Ok(match s {
                        "ALL_TIME" => ListTimeSpan::AllTime,
                        "DAILY" => ListTimeSpan::Daily,
                        "SCORE_TIME_SPAN_UNSPECIFIED" => ListTimeSpan::ScoreTimeSpanUnspecified,
                        "WEEKLY" => ListTimeSpan::Weekly,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListTimeSpan {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListTimeSpan {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListTimeSpan {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ALL_TIME" => ListTimeSpan::AllTime,
                        "DAILY" => ListTimeSpan::Daily,
                        "SCORE_TIME_SPAN_UNSPECIFIED" => ListTimeSpan::ScoreTimeSpanUnspecified,
                        "WEEKLY" => ListTimeSpan::Weekly,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListTimeSpan {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListTimeSpan {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListWindowCollection {
                #[doc = "List only scores of friends."]
                Friends,
                #[doc = "List all scores in the public leaderboard."]
                Public,
                #[doc = "Default value. This value is unused."]
                ScoreCollectionUnspecified,
                #[doc = "(Obsolete) Legacy G+ social scores."]
                Social,
            }
            impl ListWindowCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListWindowCollection::Friends => "FRIENDS",
                        ListWindowCollection::Public => "PUBLIC",
                        ListWindowCollection::ScoreCollectionUnspecified => {
                            "SCORE_COLLECTION_UNSPECIFIED"
                        }
                        ListWindowCollection::Social => "SOCIAL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListWindowCollection {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListWindowCollection {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListWindowCollection, ()> {
                    Ok(match s {
                        "FRIENDS" => ListWindowCollection::Friends,
                        "PUBLIC" => ListWindowCollection::Public,
                        "SCORE_COLLECTION_UNSPECIFIED" => {
                            ListWindowCollection::ScoreCollectionUnspecified
                        }
                        "SOCIAL" => ListWindowCollection::Social,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListWindowCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListWindowCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListWindowCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "FRIENDS" => ListWindowCollection::Friends,
                        "PUBLIC" => ListWindowCollection::Public,
                        "SCORE_COLLECTION_UNSPECIFIED" => {
                            ListWindowCollection::ScoreCollectionUnspecified
                        }
                        "SOCIAL" => ListWindowCollection::Social,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListWindowCollection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListWindowCollection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListWindowTimeSpan {
                #[doc = "The score is an all-time score."]
                AllTime,
                #[doc = "The score is a daily score."]
                Daily,
                #[doc = "Default value. This value is unused."]
                ScoreTimeSpanUnspecified,
                #[doc = "The score is a weekly score."]
                Weekly,
            }
            impl ListWindowTimeSpan {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListWindowTimeSpan::AllTime => "ALL_TIME",
                        ListWindowTimeSpan::Daily => "DAILY",
                        ListWindowTimeSpan::ScoreTimeSpanUnspecified => {
                            "SCORE_TIME_SPAN_UNSPECIFIED"
                        }
                        ListWindowTimeSpan::Weekly => "WEEKLY",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListWindowTimeSpan {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListWindowTimeSpan {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListWindowTimeSpan, ()> {
                    Ok(match s {
                        "ALL_TIME" => ListWindowTimeSpan::AllTime,
                        "DAILY" => ListWindowTimeSpan::Daily,
                        "SCORE_TIME_SPAN_UNSPECIFIED" => {
                            ListWindowTimeSpan::ScoreTimeSpanUnspecified
                        }
                        "WEEKLY" => ListWindowTimeSpan::Weekly,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListWindowTimeSpan {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListWindowTimeSpan {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListWindowTimeSpan {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ALL_TIME" => ListWindowTimeSpan::AllTime,
                        "DAILY" => ListWindowTimeSpan::Daily,
                        "SCORE_TIME_SPAN_UNSPECIFIED" => {
                            ListWindowTimeSpan::ScoreTimeSpanUnspecified
                        }
                        "WEEKLY" => ListWindowTimeSpan::Weekly,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListWindowTimeSpan {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListWindowTimeSpan {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ScoresActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ScoresActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get high scores, and optionally ranks, in leaderboards for the currently authenticated player. For a specific time span, `leaderboardId` can be set to `ALL` to retrieve data for all leaderboards in a given time span. `NOTE: You cannot ask for 'ALL' leaderboards and 'ALL' timeSpans in the same request; only one parameter may be set to 'ALL'."]
            pub fn get(
                &self,
                player_id: impl Into<String>,
                leaderboard_id: impl Into<String>,
                time_span: crate::resources::scores::params::GetTimeSpan,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
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
                    player_id: player_id.into(),
                    leaderboard_id: leaderboard_id.into(),
                    time_span,
                    include_rank_type: None,
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Lists the scores in a leaderboard, starting from the top."]
            pub fn list(
                &self,
                leaderboard_id: impl Into<String>,
                collection: crate::resources::scores::params::ListCollection,
                time_span: crate::resources::scores::params::ListTimeSpan,
            ) -> ListRequestBuilder {
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
                    leaderboard_id: leaderboard_id.into(),
                    collection,
                    time_span,
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Lists the scores in a leaderboard around (and including) a player's score."]
            pub fn list_window(
                &self,
                leaderboard_id: impl Into<String>,
                collection: crate::resources::scores::params::ListWindowCollection,
                time_span: crate::resources::scores::params::ListWindowTimeSpan,
            ) -> ListWindowRequestBuilder {
                ListWindowRequestBuilder {
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
                    leaderboard_id: leaderboard_id.into(),
                    collection,
                    time_span,
                    language: None,
                    max_results: None,
                    page_token: None,
                    results_above: None,
                    return_top_if_absent: None,
                }
            }
            #[doc = "Submits a score to the specified leaderboard."]
            pub fn submit(
                &self,
                leaderboard_id: impl Into<String>,
                score: i64,
            ) -> SubmitRequestBuilder {
                SubmitRequestBuilder {
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
                    leaderboard_id: leaderboard_id.into(),
                    score,
                    language: None,
                    score_tag: None,
                }
            }
            #[doc = "Submits multiple scores to leaderboards."]
            pub fn submit_multiple(
                &self,
                request: crate::schemas::PlayerScoreSubmissionList,
            ) -> SubmitMultipleRequestBuilder {
                SubmitMultipleRequestBuilder {
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
                    language: None,
                }
            }
        }
        #[doc = "Created via [ScoresActions::get()](struct.ScoresActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            player_id: String,
            leaderboard_id: String,
            time_span: crate::resources::scores::params::GetTimeSpan,
            include_rank_type: Option<crate::resources::scores::params::GetIncludeRankType>,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The types of ranks to return. If the parameter is omitted, no ranks will be returned."]
            pub fn include_rank_type(
                mut self,
                value: crate::resources::scores::params::GetIncludeRankType,
            ) -> Self {
                self.include_rank_type = Some(value);
                self
            }
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::PlayerLeaderboardScoreListResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PlayerLeaderboardScoreListResponse, crate::Error>
            {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/players/");
                {
                    let var_as_str = &self.player_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/leaderboards/");
                {
                    let var_as_str = &self.leaderboard_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/scores/");
                {
                    let var_as_string = self.time_span.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("includeRankType", &self.include_rank_type)]);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ScoresActions::list()](struct.ScoresActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            leaderboard_id: String,
            collection: crate::resources::scores::params::ListCollection,
            time_span: crate::resources::scores::params::ListTimeSpan,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::LeaderboardScores, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::LeaderboardScores, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/leaderboards/");
                {
                    let var_as_str = &self.leaderboard_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/scores/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("timeSpan", &self.time_span)]);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ScoresActions::list_window()](struct.ScoresActions.html#method.list_window)"]
        #[derive(Debug, Clone)]
        pub struct ListWindowRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            leaderboard_id: String,
            collection: crate::resources::scores::params::ListWindowCollection,
            time_span: crate::resources::scores::params::ListWindowTimeSpan,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
            results_above: Option<i32>,
            return_top_if_absent: Option<bool>,
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
        impl<'a> ListWindowRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The preferred number of scores to return above the player's score. More scores may be returned if the player is at the bottom of the leaderboard; fewer may be returned if the player is at the top. Must be less than or equal to maxResults."]
            pub fn results_above(mut self, value: i32) -> Self {
                self.results_above = Some(value);
                self
            }
            #[doc = "True if the top scores should be returned when the player is not in the leaderboard. Defaults to true."]
            pub fn return_top_if_absent(mut self, value: bool) -> Self {
                self.return_top_if_absent = Some(value);
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::LeaderboardScores, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::LeaderboardScores, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/leaderboards/");
                {
                    let var_as_str = &self.leaderboard_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/window/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("timeSpan", &self.time_span)]);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("resultsAbove", &self.results_above)]);
                req = req.query(&[("returnTopIfAbsent", &self.return_top_if_absent)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ScoresActions::submit()](struct.ScoresActions.html#method.submit)"]
        #[derive(Debug, Clone)]
        pub struct SubmitRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            leaderboard_id: String,
            score: i64,
            language: Option<String>,
            score_tag: Option<String>,
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
        impl<'a> SubmitRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "Additional information about the score you're submitting. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
            pub fn score_tag(mut self, value: impl Into<String>) -> Self {
                self.score_tag = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::PlayerScoreResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PlayerScoreResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/leaderboards/");
                {
                    let var_as_str = &self.leaderboard_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/scores");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("score", &self.score)]);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("scoreTag", &self.score_tag)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ScoresActions::submit_multiple()](struct.ScoresActions.html#method.submit_multiple)"]
        #[derive(Debug, Clone)]
        pub struct SubmitMultipleRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::PlayerScoreSubmissionList,
            language: Option<String>,
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
        impl<'a> SubmitMultipleRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::PlayerScoreListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PlayerScoreListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/leaderboards/scores");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod snapshots {
        pub mod params {}
        pub struct SnapshotsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SnapshotsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves the metadata for a given snapshot ID."]
            pub fn get(&self, snapshot_id: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
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
                    snapshot_id: snapshot_id.into(),
                    language: None,
                }
            }
            #[doc = "Retrieves a list of snapshots created by your application for the player corresponding to the player ID."]
            pub fn list(&self, player_id: impl Into<String>) -> ListRequestBuilder {
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
                    player_id: player_id.into(),
                    language: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [SnapshotsActions::get()](struct.SnapshotsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            snapshot_id: String,
            language: Option<String>,
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
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Snapshot, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Snapshot, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/snapshots/");
                {
                    let var_as_str = &self.snapshot_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [SnapshotsActions::list()](struct.SnapshotsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            player_id: String,
            language: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
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
            #[doc = "The preferred language to use for strings returned by this method."]
            pub fn language(mut self, value: impl Into<String>) -> Self {
                self.language = Some(value.into());
                self
            }
            #[doc = "The maximum number of snapshot resources to return in the response, used for paging. For any response, the actual number of snapshot resources returned may be less than the specified `maxResults`."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The token returned by the previous request."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::SnapshotListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SnapshotListResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/players/");
                {
                    let var_as_str = &self.player_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/snapshots");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("language", &self.language)]);
                req = req.query(&[("maxResults", &self.max_results)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod snapshots_extended {
        pub mod params {}
        pub struct SnapshotsExtendedActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SnapshotsExtendedActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Resolves any potential conflicts according to the resolution policy specified in the request and returns the snapshot head after the resolution."]
            pub fn resolve_snapshot_head(
                &self,
                request: crate::schemas::ResolveSnapshotHeadRequest,
                snapshot_name: impl Into<String>,
            ) -> ResolveSnapshotHeadRequestBuilder {
                ResolveSnapshotHeadRequestBuilder {
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
                    snapshot_name: snapshot_name.into(),
                }
            }
        }
        #[doc = "Created via [SnapshotsExtendedActions::resolve_snapshot_head()](struct.SnapshotsExtendedActions.html#method.resolve_snapshot_head)"]
        #[derive(Debug, Clone)]
        pub struct ResolveSnapshotHeadRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ResolveSnapshotHeadRequest,
            snapshot_name: String,
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
        impl<'a> ResolveSnapshotHeadRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ResolveSnapshotHeadResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ResolveSnapshotHeadResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/snapshotsExtended/");
                {
                    let var_as_str = &self.snapshot_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":resolveHead");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
    pub mod stats {
        pub mod params {}
        pub struct StatsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> StatsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns engagement and spend statistics in this application for the currently authenticated user."]
            pub fn get(&self) -> GetRequestBuilder {
                GetRequestBuilder {
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
                }
            }
        }
        #[doc = "Created via [StatsActions::get()](struct.StatsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> GetRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::StatsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::StatsResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://games.googleapis.com/".to_owned();
                output.push_str("games/v1/stats");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
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
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
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
            Error::IO(err) => write!(f, "IO Error: {}", err),
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
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
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
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
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
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
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
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

            futures::task::Poll::Ready(Ok(bytes_written))
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
