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
    pub struct Action {
        #[doc = "The actor responsible for this action (or empty if all actors are\nresponsible)."]
        #[serde(rename = "actor", default)]
        pub actor: ::std::option::Option<crate::schemas::Actor>,
        #[doc = "The type and detailed information about the action."]
        #[serde(rename = "detail", default)]
        pub detail: ::std::option::Option<crate::schemas::ActionDetail>,
        #[doc = "The target this action affects (or empty if affecting all targets). This\nrepresents the state of the target immediately after this action occurred."]
        #[serde(rename = "target", default)]
        pub target: ::std::option::Option<crate::schemas::Target>,
        #[doc = "The action occurred over this time range."]
        #[serde(rename = "timeRange", default)]
        pub time_range: ::std::option::Option<crate::schemas::TimeRange>,
        #[doc = "The action occurred at this specific time."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Action {
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
    pub struct ActionDetail {
        #[doc = "A change about comments was made."]
        #[serde(rename = "comment", default)]
        pub comment: ::std::option::Option<crate::schemas::Comment>,
        #[doc = "An object was created."]
        #[serde(rename = "create", default)]
        pub create: ::std::option::Option<crate::schemas::Create>,
        #[doc = "An object was deleted."]
        #[serde(rename = "delete", default)]
        pub delete: ::std::option::Option<crate::schemas::Delete>,
        #[doc = "A change happened in data leak prevention status."]
        #[serde(rename = "dlpChange", default)]
        pub dlp_change: ::std::option::Option<crate::schemas::DataLeakPreventionChange>,
        #[doc = "An object was edited."]
        #[serde(rename = "edit", default)]
        pub edit: ::std::option::Option<crate::schemas::Edit>,
        #[doc = "The permission on an object was changed."]
        #[serde(rename = "permissionChange", default)]
        pub permission_change: ::std::option::Option<crate::schemas::PermissionChange>,
        #[doc = "An object was moved."]
        #[serde(rename = "move", default)]
        pub r#move: ::std::option::Option<crate::schemas::Move>,
        #[doc = "An object was referenced in an application outside of Drive/Docs."]
        #[serde(rename = "reference", default)]
        pub reference: ::std::option::Option<crate::schemas::ApplicationReference>,
        #[doc = "An object was renamed."]
        #[serde(rename = "rename", default)]
        pub rename: ::std::option::Option<crate::schemas::Rename>,
        #[doc = "A deleted object was restored."]
        #[serde(rename = "restore", default)]
        pub restore: ::std::option::Option<crate::schemas::Restore>,
        #[doc = "Settings were changed."]
        #[serde(rename = "settingsChange", default)]
        pub settings_change: ::std::option::Option<crate::schemas::SettingsChange>,
    }
    impl ::field_selector::FieldSelector for ActionDetail {
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
    pub struct Actor {
        #[doc = "An administrator."]
        #[serde(rename = "administrator", default)]
        pub administrator: ::std::option::Option<crate::schemas::Administrator>,
        #[doc = "An anonymous user."]
        #[serde(rename = "anonymous", default)]
        pub anonymous: ::std::option::Option<crate::schemas::AnonymousUser>,
        #[doc = "An account acting on behalf of another."]
        #[serde(rename = "impersonation", default)]
        pub impersonation: ::std::option::Option<crate::schemas::Impersonation>,
        #[doc = "A non-user actor (i.e. system triggered)."]
        #[serde(rename = "system", default)]
        pub system: ::std::option::Option<crate::schemas::SystemEvent>,
        #[doc = "An end user."]
        #[serde(rename = "user", default)]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::field_selector::FieldSelector for Actor {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Administrator;
    impl ::field_selector::FieldSelector for Administrator {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AnonymousUser;
    impl ::field_selector::FieldSelector for AnonymousUser {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Anyone;
    impl ::field_selector::FieldSelector for Anyone {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationReferenceType {
        #[doc = "Comments were made regarding a Drive item."]
        Discuss,
        #[doc = "The links of one or more Drive items were posted."]
        Link,
        #[doc = "The type is not available."]
        UnspecifiedReferenceType,
    }
    impl ApplicationReferenceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationReferenceType::Discuss => "DISCUSS",
                ApplicationReferenceType::Link => "LINK",
                ApplicationReferenceType::UnspecifiedReferenceType => "UNSPECIFIED_REFERENCE_TYPE",
            }
        }
    }
    impl ::std::fmt::Display for ApplicationReferenceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationReferenceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationReferenceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISCUSS" => ApplicationReferenceType::Discuss,
                "LINK" => ApplicationReferenceType::Link,
                "UNSPECIFIED_REFERENCE_TYPE" => ApplicationReferenceType::UnspecifiedReferenceType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ApplicationReferenceType {
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
    pub struct ApplicationReference {
        #[doc = "The reference type corresponding to this event."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::ApplicationReferenceType>,
    }
    impl ::field_selector::FieldSelector for ApplicationReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssignmentSubtype {
        #[doc = "An assignment was added."]
        Added,
        #[doc = "An assignment was deleted."]
        Deleted,
        #[doc = "An assignment was reassigned."]
        Reassigned,
        #[doc = "A resolved assignment was reopened."]
        Reopened,
        #[doc = "An assignment reply was added."]
        ReplyAdded,
        #[doc = "An assignment reply was deleted."]
        ReplyDeleted,
        #[doc = "An assignment was resolved."]
        Resolved,
        #[doc = "Subtype not available."]
        SubtypeUnspecified,
    }
    impl AssignmentSubtype {
        pub fn as_str(self) -> &'static str {
            match self {
                AssignmentSubtype::Added => "ADDED",
                AssignmentSubtype::Deleted => "DELETED",
                AssignmentSubtype::Reassigned => "REASSIGNED",
                AssignmentSubtype::Reopened => "REOPENED",
                AssignmentSubtype::ReplyAdded => "REPLY_ADDED",
                AssignmentSubtype::ReplyDeleted => "REPLY_DELETED",
                AssignmentSubtype::Resolved => "RESOLVED",
                AssignmentSubtype::SubtypeUnspecified => "SUBTYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for AssignmentSubtype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssignmentSubtype {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssignmentSubtype {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED" => AssignmentSubtype::Added,
                "DELETED" => AssignmentSubtype::Deleted,
                "REASSIGNED" => AssignmentSubtype::Reassigned,
                "REOPENED" => AssignmentSubtype::Reopened,
                "REPLY_ADDED" => AssignmentSubtype::ReplyAdded,
                "REPLY_DELETED" => AssignmentSubtype::ReplyDeleted,
                "RESOLVED" => AssignmentSubtype::Resolved,
                "SUBTYPE_UNSPECIFIED" => AssignmentSubtype::SubtypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssignmentSubtype {
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
    pub struct Assignment {
        #[doc = "The sub-type of this event."]
        #[serde(rename = "subtype", default)]
        pub subtype: ::std::option::Option<crate::schemas::AssignmentSubtype>,
    }
    impl ::field_selector::FieldSelector for Assignment {
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
    pub struct Comment {
        #[doc = "A change on an assignment."]
        #[serde(rename = "assignment", default)]
        pub assignment: ::std::option::Option<crate::schemas::Assignment>,
        #[doc = "Users who are mentioned in this comment."]
        #[serde(rename = "mentionedUsers", default)]
        pub mentioned_users: ::std::option::Option<Vec<crate::schemas::User>>,
        #[doc = "A change on a regular posted comment."]
        #[serde(rename = "post", default)]
        pub post: ::std::option::Option<crate::schemas::Post>,
        #[doc = "A change on a suggestion."]
        #[serde(rename = "suggestion", default)]
        pub suggestion: ::std::option::Option<crate::schemas::Suggestion>,
    }
    impl ::field_selector::FieldSelector for Comment {
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
    pub struct ConsolidationStrategy {
        #[doc = "The individual activities are consolidated using the legacy strategy."]
        #[serde(rename = "legacy", default)]
        pub legacy: ::std::option::Option<crate::schemas::Legacy>,
        #[doc = "The individual activities are not consolidated."]
        #[serde(rename = "none", default)]
        pub none: ::std::option::Option<crate::schemas::NoConsolidation>,
    }
    impl ::field_selector::FieldSelector for ConsolidationStrategy {
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
    pub struct Copy {
        #[doc = "The the original object."]
        #[serde(rename = "originalObject", default)]
        pub original_object: ::std::option::Option<crate::schemas::TargetReference>,
    }
    impl ::field_selector::FieldSelector for Copy {
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
    pub struct Create {
        #[doc = "If present, indicates the object was created by copying an existing Drive\nobject."]
        #[serde(rename = "copy", default)]
        pub copy: ::std::option::Option<crate::schemas::Copy>,
        #[doc = "If present, indicates the object was newly created (e.g. as a blank\ndocument), not derived from a Drive object or external object."]
        #[serde(rename = "new", default)]
        pub new: ::std::option::Option<crate::schemas::New>,
        #[doc = "If present, indicates the object originated externally and was uploaded\nto Drive."]
        #[serde(rename = "upload", default)]
        pub upload: ::std::option::Option<crate::schemas::Upload>,
    }
    impl ::field_selector::FieldSelector for Create {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataLeakPreventionChangeType {
        #[doc = "Document is no longer flagged as containing sensitive content."]
        Cleared,
        #[doc = "Document has been flagged as containing sensitive content."]
        Flagged,
        #[doc = "An update to the DLP state that is neither FLAGGED or CLEARED."]
        TypeUnspecified,
    }
    impl DataLeakPreventionChangeType {
        pub fn as_str(self) -> &'static str {
            match self {
                DataLeakPreventionChangeType::Cleared => "CLEARED",
                DataLeakPreventionChangeType::Flagged => "FLAGGED",
                DataLeakPreventionChangeType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for DataLeakPreventionChangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataLeakPreventionChangeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataLeakPreventionChangeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLEARED" => DataLeakPreventionChangeType::Cleared,
                "FLAGGED" => DataLeakPreventionChangeType::Flagged,
                "TYPE_UNSPECIFIED" => DataLeakPreventionChangeType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DataLeakPreventionChangeType {
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
    pub struct DataLeakPreventionChange {
        #[doc = "The type of Data Leak Prevention (DLP) change."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DataLeakPreventionChangeType>,
    }
    impl ::field_selector::FieldSelector for DataLeakPreventionChange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeleteType {
        #[doc = "An object was deleted permanently."]
        PermanentDelete,
        #[doc = "An object was put into the trash."]
        Trash,
        #[doc = "Deletion type is not available."]
        TypeUnspecified,
    }
    impl DeleteType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeleteType::PermanentDelete => "PERMANENT_DELETE",
                DeleteType::Trash => "TRASH",
                DeleteType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for DeleteType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeleteType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeleteType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERMANENT_DELETE" => DeleteType::PermanentDelete,
                "TRASH" => DeleteType::Trash,
                "TYPE_UNSPECIFIED" => DeleteType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DeleteType {
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
    pub struct Delete {
        #[doc = "The type of delete action taken."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DeleteType>,
    }
    impl ::field_selector::FieldSelector for Delete {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeletedUser;
    impl ::field_selector::FieldSelector for DeletedUser {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct Domain {
        #[doc = "An opaque string used to identify this domain."]
        #[serde(rename = "legacyId", default)]
        pub legacy_id: ::std::option::Option<String>,
        #[doc = "The name of the domain, e.g. \"google.com\"."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Domain {
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
    pub struct Drive {
        #[doc = "The resource name of the shared drive. The format is\n\"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection\nID for this resource name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The root of this shared drive."]
        #[serde(rename = "root", default)]
        pub root: ::std::option::Option<crate::schemas::DriveItem>,
        #[doc = "The title of the shared drive."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Drive {
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
    pub struct DriveActivity {
        #[doc = "Details on all actions in this activity."]
        #[serde(rename = "actions", default)]
        pub actions: ::std::option::Option<Vec<crate::schemas::Action>>,
        #[doc = "All actor(s) responsible for the activity."]
        #[serde(rename = "actors", default)]
        pub actors: ::std::option::Option<Vec<crate::schemas::Actor>>,
        #[doc = "Key information about the primary action for this activity. This is either\nrepresentative, or the most important, of all actions in the activity,\naccording to the ConsolidationStrategy in the request."]
        #[serde(rename = "primaryActionDetail", default)]
        pub primary_action_detail: ::std::option::Option<crate::schemas::ActionDetail>,
        #[doc = "All Google Drive objects this activity is about (e.g. file, folder, drive).\nThis represents the state of the target immediately after the actions\noccurred."]
        #[serde(rename = "targets", default)]
        pub targets: ::std::option::Option<Vec<crate::schemas::Target>>,
        #[doc = "The activity occurred over this time range."]
        #[serde(rename = "timeRange", default)]
        pub time_range: ::std::option::Option<crate::schemas::TimeRange>,
        #[doc = "The activity occurred at this specific time."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DriveActivity {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveFile;
    impl ::field_selector::FieldSelector for DriveFile {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveFolderType {
        #[doc = "The folder is the root of a user's MyDrive."]
        MyDriveRoot,
        #[doc = "The folder is the root of a shared drive."]
        SharedDriveRoot,
        #[doc = "The folder is a standard, non-root, folder."]
        StandardFolder,
        #[doc = "The folder type is unknown."]
        TypeUnspecified,
    }
    impl DriveFolderType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveFolderType::MyDriveRoot => "MY_DRIVE_ROOT",
                DriveFolderType::SharedDriveRoot => "SHARED_DRIVE_ROOT",
                DriveFolderType::StandardFolder => "STANDARD_FOLDER",
                DriveFolderType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for DriveFolderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveFolderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveFolderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MY_DRIVE_ROOT" => DriveFolderType::MyDriveRoot,
                "SHARED_DRIVE_ROOT" => DriveFolderType::SharedDriveRoot,
                "STANDARD_FOLDER" => DriveFolderType::StandardFolder,
                "TYPE_UNSPECIFIED" => DriveFolderType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DriveFolderType {
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
    pub struct DriveFolder {
        #[doc = "The type of Drive folder."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DriveFolderType>,
    }
    impl ::field_selector::FieldSelector for DriveFolder {
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
    pub struct DriveItem {
        #[doc = "The Drive item is a file."]
        #[serde(rename = "driveFile", default)]
        pub drive_file: ::std::option::Option<crate::schemas::DriveFile>,
        #[doc = "The Drive item is a folder."]
        #[serde(rename = "driveFolder", default)]
        pub drive_folder: ::std::option::Option<crate::schemas::DriveFolder>,
        #[doc = "This field is deprecated; please use the `driveFile` field instead."]
        #[serde(rename = "file", default)]
        pub file: ::std::option::Option<crate::schemas::File>,
        #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
        #[serde(rename = "folder", default)]
        pub folder: ::std::option::Option<crate::schemas::Folder>,
        #[doc = "The MIME type of the Drive item.  See\nhttps://developers.google.com/drive/v3/web/mime-types."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Information about the owner of this Drive item."]
        #[serde(rename = "owner", default)]
        pub owner: ::std::option::Option<crate::schemas::Owner>,
        #[doc = "The title of the Drive item."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DriveItem {
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
    pub struct DriveItemReference {
        #[doc = "The Drive item is a file."]
        #[serde(rename = "driveFile", default)]
        pub drive_file: ::std::option::Option<crate::schemas::DriveFile>,
        #[doc = "The Drive item is a folder."]
        #[serde(rename = "driveFolder", default)]
        pub drive_folder: ::std::option::Option<crate::schemas::DriveFolder>,
        #[doc = "This field is deprecated; please use the `driveFile` field instead."]
        #[serde(rename = "file", default)]
        pub file: ::std::option::Option<crate::schemas::File>,
        #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
        #[serde(rename = "folder", default)]
        pub folder: ::std::option::Option<crate::schemas::Folder>,
        #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The title of the Drive item."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DriveItemReference {
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
    pub struct DriveReference {
        #[doc = "The resource name of the shared drive. The format is\n\"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection\nID for this resource name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The title of the shared drive."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DriveReference {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Edit;
    impl ::field_selector::FieldSelector for Edit {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct File;
    impl ::field_selector::FieldSelector for File {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct FileComment {
        #[doc = "The comment in the discussion thread. This identifier is an opaque string\ncompatible with the Drive API; see\nhttps://developers.google.com/drive/v3/reference/comments/get"]
        #[serde(rename = "legacyCommentId", default)]
        pub legacy_comment_id: ::std::option::Option<String>,
        #[doc = "The discussion thread to which the comment was added. This identifier is an\nopaque string compatible with the Drive API and references the first\ncomment in a discussion; see\nhttps://developers.google.com/drive/v3/reference/comments/get"]
        #[serde(rename = "legacyDiscussionId", default)]
        pub legacy_discussion_id: ::std::option::Option<String>,
        #[doc = "The link to the discussion thread containing this comment, for example,\n\"https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID\"."]
        #[serde(rename = "linkToDiscussion", default)]
        pub link_to_discussion: ::std::option::Option<String>,
        #[doc = "The Drive item containing this comment."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<crate::schemas::DriveItem>,
    }
    impl ::field_selector::FieldSelector for FileComment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FolderType {
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        MyDriveRoot,
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        StandardFolder,
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        TeamDriveRoot,
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        TypeUnspecified,
    }
    impl FolderType {
        pub fn as_str(self) -> &'static str {
            match self {
                FolderType::MyDriveRoot => "MY_DRIVE_ROOT",
                FolderType::StandardFolder => "STANDARD_FOLDER",
                FolderType::TeamDriveRoot => "TEAM_DRIVE_ROOT",
                FolderType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for FolderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FolderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FolderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MY_DRIVE_ROOT" => FolderType::MyDriveRoot,
                "STANDARD_FOLDER" => FolderType::StandardFolder,
                "TEAM_DRIVE_ROOT" => FolderType::TeamDriveRoot,
                "TYPE_UNSPECIFIED" => FolderType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FolderType {
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
    pub struct Folder {
        #[doc = "This field is deprecated; please see `DriveFolder.type` instead."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::FolderType>,
    }
    impl ::field_selector::FieldSelector for Folder {
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
    pub struct Group {
        #[doc = "The email address of the group."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The title of the group."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Group {
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
    pub struct Impersonation {
        #[doc = "The impersonated user."]
        #[serde(rename = "impersonatedUser", default)]
        pub impersonated_user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::field_selector::FieldSelector for Impersonation {
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
    pub struct KnownUser {
        #[doc = "True if this is the user making the request."]
        #[serde(rename = "isCurrentUser", default)]
        pub is_current_user: ::std::option::Option<bool>,
        #[doc = "The identifier for this user that can be used with the People API to get\nmore information. The format is \"people/ACCOUNT_ID\". See\nhttps://developers.google.com/people/."]
        #[serde(rename = "personName", default)]
        pub person_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for KnownUser {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Legacy;
    impl ::field_selector::FieldSelector for Legacy {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct Move {
        #[doc = "The added parent object(s)."]
        #[serde(rename = "addedParents", default)]
        pub added_parents: ::std::option::Option<Vec<crate::schemas::TargetReference>>,
        #[doc = "The removed parent object(s)."]
        #[serde(rename = "removedParents", default)]
        pub removed_parents: ::std::option::Option<Vec<crate::schemas::TargetReference>>,
    }
    impl ::field_selector::FieldSelector for Move {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct New;
    impl ::field_selector::FieldSelector for New {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NoConsolidation;
    impl ::field_selector::FieldSelector for NoConsolidation {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct Owner {
        #[doc = "The domain of the Drive item owner."]
        #[serde(rename = "domain", default)]
        pub domain: ::std::option::Option<crate::schemas::Domain>,
        #[doc = "The drive that owns the item."]
        #[serde(rename = "drive", default)]
        pub drive: ::std::option::Option<crate::schemas::DriveReference>,
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        #[serde(rename = "teamDrive", default)]
        pub team_drive: ::std::option::Option<crate::schemas::TeamDriveReference>,
        #[doc = "The user that owns the Drive item."]
        #[serde(rename = "user", default)]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::field_selector::FieldSelector for Owner {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PermissionRole {
        #[doc = "A role granting the ability to view and comment on content."]
        Commenter,
        #[doc = "A role granting the ability to contribute content. This role is sometimes\nalso known as \"writer\"."]
        Editor,
        #[doc = "A role granting the ability to contribute and manage content."]
        FileOrganizer,
        #[doc = "A role granting the ability to manage people and settings."]
        Organizer,
        #[doc = "A role granting full access."]
        Owner,
        #[doc = "A role granting the ability to view content only after it has been\npublished to the web. This role is sometimes also known as \"published\nreader\". See https://support.google.com/sites/answer/6372880 for more\ninformation."]
        PublishedViewer,
        #[doc = "The role is not available."]
        RoleUnspecified,
        #[doc = "A role granting the ability to view content. This role is sometimes also\nknown as \"reader\"."]
        Viewer,
    }
    impl PermissionRole {
        pub fn as_str(self) -> &'static str {
            match self {
                PermissionRole::Commenter => "COMMENTER",
                PermissionRole::Editor => "EDITOR",
                PermissionRole::FileOrganizer => "FILE_ORGANIZER",
                PermissionRole::Organizer => "ORGANIZER",
                PermissionRole::Owner => "OWNER",
                PermissionRole::PublishedViewer => "PUBLISHED_VIEWER",
                PermissionRole::RoleUnspecified => "ROLE_UNSPECIFIED",
                PermissionRole::Viewer => "VIEWER",
            }
        }
    }
    impl ::std::fmt::Display for PermissionRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PermissionRole {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PermissionRole {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMENTER" => PermissionRole::Commenter,
                "EDITOR" => PermissionRole::Editor,
                "FILE_ORGANIZER" => PermissionRole::FileOrganizer,
                "ORGANIZER" => PermissionRole::Organizer,
                "OWNER" => PermissionRole::Owner,
                "PUBLISHED_VIEWER" => PermissionRole::PublishedViewer,
                "ROLE_UNSPECIFIED" => PermissionRole::RoleUnspecified,
                "VIEWER" => PermissionRole::Viewer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PermissionRole {
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
    pub struct Permission {
        #[doc = "If true, the item can be discovered (e.g. in the user's \"Shared with me\"\ncollection) without needing a link to the item."]
        #[serde(rename = "allowDiscovery", default)]
        pub allow_discovery: ::std::option::Option<bool>,
        #[doc = "If set, this permission applies to anyone, even logged out users."]
        #[serde(rename = "anyone", default)]
        pub anyone: ::std::option::Option<crate::schemas::Anyone>,
        #[doc = "The domain to whom this permission applies."]
        #[serde(rename = "domain", default)]
        pub domain: ::std::option::Option<crate::schemas::Domain>,
        #[doc = "The group to whom this permission applies."]
        #[serde(rename = "group", default)]
        pub group: ::std::option::Option<crate::schemas::Group>,
        #[doc = "Indicates the\n<a href=\"/drive/web/manage-sharing#roles\">Google Drive permissions\nrole</a>. The role determines a user's ability to read, write, and\ncomment on items."]
        #[serde(rename = "role", default)]
        pub role: ::std::option::Option<crate::schemas::PermissionRole>,
        #[doc = "The user to whom this permission applies."]
        #[serde(rename = "user", default)]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::field_selector::FieldSelector for Permission {
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
    pub struct PermissionChange {
        #[doc = "The set of permissions added by this change."]
        #[serde(rename = "addedPermissions", default)]
        pub added_permissions: ::std::option::Option<Vec<crate::schemas::Permission>>,
        #[doc = "The set of permissions removed by this change."]
        #[serde(rename = "removedPermissions", default)]
        pub removed_permissions: ::std::option::Option<Vec<crate::schemas::Permission>>,
    }
    impl ::field_selector::FieldSelector for PermissionChange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PostSubtype {
        #[doc = "A post was added."]
        Added,
        #[doc = "A post was deleted."]
        Deleted,
        #[doc = "A posted comment was reopened."]
        Reopened,
        #[doc = "A reply was added."]
        ReplyAdded,
        #[doc = "A reply was deleted."]
        ReplyDeleted,
        #[doc = "A posted comment was resolved."]
        Resolved,
        #[doc = "Subtype not available."]
        SubtypeUnspecified,
    }
    impl PostSubtype {
        pub fn as_str(self) -> &'static str {
            match self {
                PostSubtype::Added => "ADDED",
                PostSubtype::Deleted => "DELETED",
                PostSubtype::Reopened => "REOPENED",
                PostSubtype::ReplyAdded => "REPLY_ADDED",
                PostSubtype::ReplyDeleted => "REPLY_DELETED",
                PostSubtype::Resolved => "RESOLVED",
                PostSubtype::SubtypeUnspecified => "SUBTYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for PostSubtype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PostSubtype {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PostSubtype {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED" => PostSubtype::Added,
                "DELETED" => PostSubtype::Deleted,
                "REOPENED" => PostSubtype::Reopened,
                "REPLY_ADDED" => PostSubtype::ReplyAdded,
                "REPLY_DELETED" => PostSubtype::ReplyDeleted,
                "RESOLVED" => PostSubtype::Resolved,
                "SUBTYPE_UNSPECIFIED" => PostSubtype::SubtypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PostSubtype {
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
    pub struct Post {
        #[doc = "The sub-type of this event."]
        #[serde(rename = "subtype", default)]
        pub subtype: ::std::option::Option<crate::schemas::PostSubtype>,
    }
    impl ::field_selector::FieldSelector for Post {
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
    pub struct QueryDriveActivityRequest {
        #[doc = "Return activities for this Drive folder and all children and descendants.\nThe format is \"items/ITEM_ID\"."]
        #[serde(rename = "ancestorName", default)]
        pub ancestor_name: ::std::option::Option<String>,
        #[doc = "Details on how to consolidate related actions that make up the activity. If\nnot set, then related actions will not be consolidated."]
        #[serde(rename = "consolidationStrategy", default)]
        pub consolidation_strategy: ::std::option::Option<crate::schemas::ConsolidationStrategy>,
        #[doc = "The filtering for items returned from this query request. The format of the\nfilter string is a sequence of expressions, joined by an optional \"AND\",\nwhere each expression is of the form \"field operator value\".\n\nSupported fields:\n\n* <tt>time</tt>: Uses numerical operators on date values either in\n  terms of milliseconds since Jan 1, 1970 or in RFC 3339 format.\n  Examples:\n  \n  * <tt>time > 1452409200000 AND time <= 1492812924310</tt>\n  * <tt>time >= \"2016-01-10T01:02:03-05:00\"</tt>\n* <tt>detail.action_detail_case</tt>: Uses the \"has\" operator (:) and\n  either a singular value or a list of allowed action types enclosed in\n  parentheses.\n  Examples:\n  \n  * <tt>detail.action_detail_case: RENAME</tt>\n  * <tt>detail.action_detail_case:(CREATE UPLOAD)</tt>\n  * <tt>-detail.action_detail_case:MOVE</tt>"]
        #[serde(rename = "filter", default)]
        pub filter: ::std::option::Option<String>,
        #[doc = "Return activities for this Drive item. The format is\n\"items/ITEM_ID\"."]
        #[serde(rename = "itemName", default)]
        pub item_name: ::std::option::Option<String>,
        #[doc = "The requested number of activity to return. If not set, a default value\nwill be used."]
        #[serde(rename = "pageSize", default)]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "The next_page_token value returned from a previous QueryDriveActivity\nrequest, if any."]
        #[serde(rename = "pageToken", default)]
        pub page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for QueryDriveActivityRequest {
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
    pub struct QueryDriveActivityResponse {
        #[doc = "List of activity requested."]
        #[serde(rename = "activities", default)]
        pub activities: ::std::option::Option<Vec<crate::schemas::DriveActivity>>,
        #[doc = "Token to retrieve the next page of results, or\nempty if there are no more results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for QueryDriveActivityResponse {
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
    pub struct Rename {
        #[doc = "The new title of the drive object."]
        #[serde(rename = "newTitle", default)]
        pub new_title: ::std::option::Option<String>,
        #[doc = "The previous title of the drive object."]
        #[serde(rename = "oldTitle", default)]
        pub old_title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Rename {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RestoreType {
        #[doc = "The type is not available."]
        TypeUnspecified,
        #[doc = "An object was restored from the trash."]
        Untrash,
    }
    impl RestoreType {
        pub fn as_str(self) -> &'static str {
            match self {
                RestoreType::TypeUnspecified => "TYPE_UNSPECIFIED",
                RestoreType::Untrash => "UNTRASH",
            }
        }
    }
    impl ::std::fmt::Display for RestoreType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RestoreType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RestoreType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => RestoreType::TypeUnspecified,
                "UNTRASH" => RestoreType::Untrash,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for RestoreType {
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
    pub struct Restore {
        #[doc = "The type of restore action taken."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::RestoreType>,
    }
    impl ::field_selector::FieldSelector for Restore {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RestrictionChangeFeature {
        #[doc = "When restricted, this prevents direct sharing of individual items."]
        DirectSharing,
        #[doc = "When restricted, this prevents use of Drive File Stream."]
        DriveFileStream,
        #[doc = "The feature which changed restriction settings was not available."]
        FeatureUnspecified,
        #[doc = "When restricted, this prevents actions like copy, download, and print\nthat might result in uncontrolled duplicates of items."]
        ItemDuplication,
        #[doc = "When restricted, this prevents items from being shared outside the\ndomain."]
        SharingOutsideDomain,
    }
    impl RestrictionChangeFeature {
        pub fn as_str(self) -> &'static str {
            match self {
                RestrictionChangeFeature::DirectSharing => "DIRECT_SHARING",
                RestrictionChangeFeature::DriveFileStream => "DRIVE_FILE_STREAM",
                RestrictionChangeFeature::FeatureUnspecified => "FEATURE_UNSPECIFIED",
                RestrictionChangeFeature::ItemDuplication => "ITEM_DUPLICATION",
                RestrictionChangeFeature::SharingOutsideDomain => "SHARING_OUTSIDE_DOMAIN",
            }
        }
    }
    impl ::std::fmt::Display for RestrictionChangeFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RestrictionChangeFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RestrictionChangeFeature {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIRECT_SHARING" => RestrictionChangeFeature::DirectSharing,
                "DRIVE_FILE_STREAM" => RestrictionChangeFeature::DriveFileStream,
                "FEATURE_UNSPECIFIED" => RestrictionChangeFeature::FeatureUnspecified,
                "ITEM_DUPLICATION" => RestrictionChangeFeature::ItemDuplication,
                "SHARING_OUTSIDE_DOMAIN" => RestrictionChangeFeature::SharingOutsideDomain,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for RestrictionChangeFeature {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RestrictionChangeNewRestriction {
        #[doc = "The use of this feature is fully restricted."]
        FullyRestricted,
        #[doc = "The type of restriction is not available."]
        RestrictionUnspecified,
        #[doc = "The feature is available without restriction."]
        Unrestricted,
    }
    impl RestrictionChangeNewRestriction {
        pub fn as_str(self) -> &'static str {
            match self {
                RestrictionChangeNewRestriction::FullyRestricted => "FULLY_RESTRICTED",
                RestrictionChangeNewRestriction::RestrictionUnspecified => {
                    "RESTRICTION_UNSPECIFIED"
                }
                RestrictionChangeNewRestriction::Unrestricted => "UNRESTRICTED",
            }
        }
    }
    impl ::std::fmt::Display for RestrictionChangeNewRestriction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RestrictionChangeNewRestriction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RestrictionChangeNewRestriction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULLY_RESTRICTED" => RestrictionChangeNewRestriction::FullyRestricted,
                "RESTRICTION_UNSPECIFIED" => {
                    RestrictionChangeNewRestriction::RestrictionUnspecified
                }
                "UNRESTRICTED" => RestrictionChangeNewRestriction::Unrestricted,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for RestrictionChangeNewRestriction {
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
    pub struct RestrictionChange {
        #[doc = "The feature which had a change in restriction policy."]
        #[serde(rename = "feature", default)]
        pub feature: ::std::option::Option<crate::schemas::RestrictionChangeFeature>,
        #[doc = "The restriction in place after the change."]
        #[serde(rename = "newRestriction", default)]
        pub new_restriction: ::std::option::Option<crate::schemas::RestrictionChangeNewRestriction>,
    }
    impl ::field_selector::FieldSelector for RestrictionChange {
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
    pub struct SettingsChange {
        #[doc = "The set of changes made to restrictions."]
        #[serde(rename = "restrictionChanges", default)]
        pub restriction_changes: ::std::option::Option<Vec<crate::schemas::RestrictionChange>>,
    }
    impl ::field_selector::FieldSelector for SettingsChange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SuggestionSubtype {
        #[doc = "An accepted suggestion was deleted."]
        AcceptDeleted,
        #[doc = "A suggestion was accepted."]
        Accepted,
        #[doc = "A suggestion was added."]
        Added,
        #[doc = "A suggestion was deleted."]
        Deleted,
        #[doc = "A rejected suggestion was deleted."]
        RejectDeleted,
        #[doc = "A suggestion was rejected."]
        Rejected,
        #[doc = "A suggestion reply was added."]
        ReplyAdded,
        #[doc = "A suggestion reply was deleted."]
        ReplyDeleted,
        #[doc = "Subtype not available."]
        SubtypeUnspecified,
    }
    impl SuggestionSubtype {
        pub fn as_str(self) -> &'static str {
            match self {
                SuggestionSubtype::AcceptDeleted => "ACCEPT_DELETED",
                SuggestionSubtype::Accepted => "ACCEPTED",
                SuggestionSubtype::Added => "ADDED",
                SuggestionSubtype::Deleted => "DELETED",
                SuggestionSubtype::RejectDeleted => "REJECT_DELETED",
                SuggestionSubtype::Rejected => "REJECTED",
                SuggestionSubtype::ReplyAdded => "REPLY_ADDED",
                SuggestionSubtype::ReplyDeleted => "REPLY_DELETED",
                SuggestionSubtype::SubtypeUnspecified => "SUBTYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for SuggestionSubtype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SuggestionSubtype {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuggestionSubtype {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPT_DELETED" => SuggestionSubtype::AcceptDeleted,
                "ACCEPTED" => SuggestionSubtype::Accepted,
                "ADDED" => SuggestionSubtype::Added,
                "DELETED" => SuggestionSubtype::Deleted,
                "REJECT_DELETED" => SuggestionSubtype::RejectDeleted,
                "REJECTED" => SuggestionSubtype::Rejected,
                "REPLY_ADDED" => SuggestionSubtype::ReplyAdded,
                "REPLY_DELETED" => SuggestionSubtype::ReplyDeleted,
                "SUBTYPE_UNSPECIFIED" => SuggestionSubtype::SubtypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SuggestionSubtype {
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
    pub struct Suggestion {
        #[doc = "The sub-type of this event."]
        #[serde(rename = "subtype", default)]
        pub subtype: ::std::option::Option<crate::schemas::SuggestionSubtype>,
    }
    impl ::field_selector::FieldSelector for Suggestion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SystemEventType {
        #[doc = "The event is due to the system automatically purging trash."]
        TrashAutoPurge,
        #[doc = "The event type is unspecified."]
        TypeUnspecified,
        #[doc = "The event is a consequence of a user account being deleted."]
        UserDeletion,
    }
    impl SystemEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                SystemEventType::TrashAutoPurge => "TRASH_AUTO_PURGE",
                SystemEventType::TypeUnspecified => "TYPE_UNSPECIFIED",
                SystemEventType::UserDeletion => "USER_DELETION",
            }
        }
    }
    impl ::std::fmt::Display for SystemEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SystemEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SystemEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TRASH_AUTO_PURGE" => SystemEventType::TrashAutoPurge,
                "TYPE_UNSPECIFIED" => SystemEventType::TypeUnspecified,
                "USER_DELETION" => SystemEventType::UserDeletion,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SystemEventType {
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
    pub struct SystemEvent {
        #[doc = "The type of the system event that may triggered activity."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::SystemEventType>,
    }
    impl ::field_selector::FieldSelector for SystemEvent {
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
    pub struct Target {
        #[doc = "The target is a shared drive."]
        #[serde(rename = "drive", default)]
        pub drive: ::std::option::Option<crate::schemas::Drive>,
        #[doc = "The target is a Drive item."]
        #[serde(rename = "driveItem", default)]
        pub drive_item: ::std::option::Option<crate::schemas::DriveItem>,
        #[doc = "The target is a comment on a Drive file."]
        #[serde(rename = "fileComment", default)]
        pub file_comment: ::std::option::Option<crate::schemas::FileComment>,
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        #[serde(rename = "teamDrive", default)]
        pub team_drive: ::std::option::Option<crate::schemas::TeamDrive>,
    }
    impl ::field_selector::FieldSelector for Target {
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
    pub struct TargetReference {
        #[doc = "The target is a shared drive."]
        #[serde(rename = "drive", default)]
        pub drive: ::std::option::Option<crate::schemas::DriveReference>,
        #[doc = "The target is a Drive item."]
        #[serde(rename = "driveItem", default)]
        pub drive_item: ::std::option::Option<crate::schemas::DriveItemReference>,
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        #[serde(rename = "teamDrive", default)]
        pub team_drive: ::std::option::Option<crate::schemas::TeamDriveReference>,
    }
    impl ::field_selector::FieldSelector for TargetReference {
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
    pub struct TeamDrive {
        #[doc = "This field is deprecated; please see `Drive.name` instead."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "This field is deprecated; please see `Drive.root` instead."]
        #[serde(rename = "root", default)]
        pub root: ::std::option::Option<crate::schemas::DriveItem>,
        #[doc = "This field is deprecated; please see `Drive.title` instead."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TeamDrive {
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
    pub struct TeamDriveReference {
        #[doc = "This field is deprecated; please see `DriveReference.name` instead."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "This field is deprecated; please see `DriveReference.title` instead."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TeamDriveReference {
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
    pub struct TimeRange {
        #[doc = "The end of the time range."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The start of the time range."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TimeRange {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UnknownUser;
    impl ::field_selector::FieldSelector for UnknownUser {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Upload;
    impl ::field_selector::FieldSelector for Upload {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct User {
        #[doc = "A user whose account has since been deleted."]
        #[serde(rename = "deletedUser", default)]
        pub deleted_user: ::std::option::Option<crate::schemas::DeletedUser>,
        #[doc = "A known user."]
        #[serde(rename = "knownUser", default)]
        pub known_user: ::std::option::Option<crate::schemas::KnownUser>,
        #[doc = "A user about whom nothing is currently known."]
        #[serde(rename = "unknownUser", default)]
        pub unknown_user: ::std::option::Option<crate::schemas::UnknownUser>,
    }
    impl ::field_selector::FieldSelector for User {
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
    #[doc = "Actions that can be performed on the activity resource"]
    pub fn activity(&self) -> crate::resources::activity::ActivityActions<A> {
        crate::resources::activity::ActivityActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod activity {
        pub mod params {}
        pub struct ActivityActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ActivityActions<'a, A> {
            #[doc = "Query past activity in Google Drive."]
            pub fn query(
                &self,
                request: crate::schemas::QueryDriveActivityRequest,
            ) -> QueryRequestBuilder<A> {
                QueryRequestBuilder {
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
        pub struct QueryRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::QueryDriveActivityRequest,
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
        impl<'a, A: yup_oauth2::GetToken> QueryRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::QueryDriveActivityResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::QueryDriveActivityResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://driveactivity.googleapis.com/".to_owned();
                output.push_str("v2/activity:query");
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/drive.activity"]);
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
#[allow(dead_code)]
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(format!(
                                    "no {} field found in iter response",
                                    self.items_field
                                )
                                .into()))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
} // Bytes in google apis are represented as urlsafe base64 encoded strings.
  // This defines a Bytes type that is a simple wrapper around a Vec<u8> used
  // internally to handle byte fields in google apis.
#[allow(dead_code)]
mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
