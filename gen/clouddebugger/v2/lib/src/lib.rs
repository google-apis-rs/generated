#![doc = "# Resources and Methods\n    * [controller](resources/controller/struct.ControllerActions.html)\n      * [debuggees](resources/controller/debuggees/struct.DebuggeesActions.html)\n        * [*register*](resources/controller/debuggees/struct.RegisterRequestBuilder.html)\n        * [breakpoints](resources/controller/debuggees/breakpoints/struct.BreakpointsActions.html)\n          * [*list*](resources/controller/debuggees/breakpoints/struct.ListRequestBuilder.html), [*update*](resources/controller/debuggees/breakpoints/struct.UpdateRequestBuilder.html)\n    * [debugger](resources/debugger/struct.DebuggerActions.html)\n      * [debuggees](resources/debugger/debuggees/struct.DebuggeesActions.html)\n        * [*list*](resources/debugger/debuggees/struct.ListRequestBuilder.html)\n        * [breakpoints](resources/debugger/debuggees/breakpoints/struct.BreakpointsActions.html)\n          * [*delete*](resources/debugger/debuggees/breakpoints/struct.DeleteRequestBuilder.html), [*get*](resources/debugger/debuggees/breakpoints/struct.GetRequestBuilder.html), [*list*](resources/debugger/debuggees/breakpoints/struct.ListRequestBuilder.html), [*set*](resources/debugger/debuggees/breakpoints/struct.SetRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "Use Stackdriver Debugger\n\n`https://www.googleapis.com/auth/cloud_debugger`"]
    pub const CLOUD_DEBUGGER: &str = "https://www.googleapis.com/auth/cloud_debugger";
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
    pub struct AliasContext {
        #[doc = "The alias kind."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::AliasContextKind>,
        #[doc = "The alias name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AliasContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AliasContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AliasContextKind {
        #[doc = "Do not use."]
        Any,
        #[doc = "Git tag"]
        Fixed,
        #[doc = "Git branch"]
        Movable,
        #[doc = "OTHER is used to specify non-standard aliases, those not of the kinds\nabove. For example, if a Git repo has a ref named \"refs/foo/bar\", it\nis considered to be of kind OTHER."]
        Other,
    }
    impl AliasContextKind {
        pub fn as_str(self) -> &'static str {
            match self {
                AliasContextKind::Any => "ANY",
                AliasContextKind::Fixed => "FIXED",
                AliasContextKind::Movable => "MOVABLE",
                AliasContextKind::Other => "OTHER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AliasContextKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AliasContextKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AliasContextKind, ()> {
            Ok(match s {
                "ANY" => AliasContextKind::Any,
                "FIXED" => AliasContextKind::Fixed,
                "MOVABLE" => AliasContextKind::Movable,
                "OTHER" => AliasContextKind::Other,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AliasContextKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AliasContextKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AliasContextKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANY" => AliasContextKind::Any,
                "FIXED" => AliasContextKind::Fixed,
                "MOVABLE" => AliasContextKind::Movable,
                "OTHER" => AliasContextKind::Other,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AliasContextKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AliasContextKind {
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
    pub struct Breakpoint {
        #[doc = "Action that the agent should perform when the code at the\nbreakpoint location is hit."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<crate::schemas::BreakpointAction>,
        #[doc = "The deadline for the breakpoint to stay in CANARY_ACTIVE state. The value\nis meaningless when the breakpoint is not in CANARY_ACTIVE state."]
        #[serde(
            rename = "canaryExpireTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canary_expire_time: ::std::option::Option<String>,
        #[doc = "Condition that triggers the breakpoint.\nThe condition is a compound boolean expression composed using expressions\nin a programming language at the source location."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<String>,
        #[doc = "Time this breakpoint was created by the server in seconds resolution."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Values of evaluated expressions at breakpoint time.\nThe evaluated expressions appear in exactly the same order they\nare listed in the `expressions` field.\nThe `name` field holds the original expression text, the `value` or\n`members` field holds the result of the evaluated expression.\nIf the expression cannot be evaluated, the `status` inside the `Variable`\nwill indicate an error and contain the error text."]
        #[serde(
            rename = "evaluatedExpressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluated_expressions: ::std::option::Option<Vec<crate::schemas::Variable>>,
        #[doc = "List of read-only expressions to evaluate at the breakpoint location.\nThe expressions are composed using expressions in the programming language\nat the source location. If the breakpoint action is `LOG`, the evaluated\nexpressions are included in log statements."]
        #[serde(
            rename = "expressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expressions: ::std::option::Option<Vec<String>>,
        #[doc = "Time this breakpoint was finalized as seen by the server in seconds\nresolution."]
        #[serde(
            rename = "finalTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_time: ::std::option::Option<String>,
        #[doc = "Breakpoint identifier, unique in the scope of the debuggee."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "When true, indicates that this is a final result and the\nbreakpoint state will not change from here on."]
        #[serde(
            rename = "isFinalState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_final_state: ::std::option::Option<bool>,
        #[doc = "A set of custom breakpoint properties, populated by the agent, to be\ndisplayed to the user."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Breakpoint source location."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::SourceLocation>,
        #[doc = "Indicates the severity of the log. Only relevant when action is `LOG`."]
        #[serde(
            rename = "logLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_level: ::std::option::Option<crate::schemas::BreakpointLogLevel>,
        #[doc = "Only relevant when action is `LOG`. Defines the message to log when\nthe breakpoint hits. The message may include parameter placeholders `$0`,\n`$1`, etc. These placeholders are replaced with the evaluated value\nof the appropriate expression. Expressions not referenced in\n`log_message_format` are not logged.\n\nExample: `Message received, id = $0, count = $1` with\n`expressions` = `[ message.id, message.count ]`."]
        #[serde(
            rename = "logMessageFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_message_format: ::std::option::Option<String>,
        #[doc = "The stack at breakpoint time, where stack_frames[0] represents the most\nrecently entered function."]
        #[serde(
            rename = "stackFrames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_frames: ::std::option::Option<Vec<crate::schemas::StackFrame>>,
        #[doc = "The current state of the breakpoint."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::BreakpointState>,
        #[doc = "Breakpoint status.\n\nThe status includes an error flag and a human readable message.\nThis field is usually unset. The message can be either\ninformational or an error message. Regardless, clients should always\ndisplay the text message back to the user.\n\nError status indicates complete failure of the breakpoint.\n\nExample (non-final state): `Still loading symbols...`\n\nExamples (final state):\n\n* `Invalid line number` referring to location\n* `Field f not found in class C` referring to condition"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::StatusMessage>,
        #[doc = "E-mail address of the user that created this breakpoint"]
        #[serde(
            rename = "userEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
        #[doc = "The `variable_table` exists to aid with computation, memory and network\ntraffic optimization.  It enables storing a variable once and reference\nit from multiple variables, including variables stored in the\n`variable_table` itself.\nFor example, the same `this` object, which may appear at many levels of\nthe stack, can have all of its data stored once in this table.  The\nstack frame variables then would hold only a reference to it.\n\nThe variable `var_table_index` field is an index into this repeated field.\nThe stored objects are nameless and get their name from the referencing\nvariable. The effective variable is a merge of the referencing variable\nand the referenced variable."]
        #[serde(
            rename = "variableTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub variable_table: ::std::option::Option<Vec<crate::schemas::Variable>>,
    }
    impl ::google_field_selector::FieldSelector for Breakpoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Breakpoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BreakpointAction {
        #[doc = "Capture stack frame and variables and update the breakpoint.\nThe data is only captured once. After that the breakpoint is set\nin a final state."]
        Capture,
        #[doc = "Log each breakpoint hit. The breakpoint remains active until\ndeleted or expired."]
        Log,
    }
    impl BreakpointAction {
        pub fn as_str(self) -> &'static str {
            match self {
                BreakpointAction::Capture => "CAPTURE",
                BreakpointAction::Log => "LOG",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BreakpointAction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BreakpointAction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BreakpointAction, ()> {
            Ok(match s {
                "CAPTURE" => BreakpointAction::Capture,
                "LOG" => BreakpointAction::Log,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BreakpointAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BreakpointAction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BreakpointAction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CAPTURE" => BreakpointAction::Capture,
                "LOG" => BreakpointAction::Log,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BreakpointAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BreakpointAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BreakpointLogLevel {
        #[doc = "Error log message."]
        Error,
        #[doc = "Information log message."]
        Info,
        #[doc = "Warning log message."]
        Warning,
    }
    impl BreakpointLogLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                BreakpointLogLevel::Error => "ERROR",
                BreakpointLogLevel::Info => "INFO",
                BreakpointLogLevel::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BreakpointLogLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BreakpointLogLevel {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BreakpointLogLevel, ()> {
            Ok(match s {
                "ERROR" => BreakpointLogLevel::Error,
                "INFO" => BreakpointLogLevel::Info,
                "WARNING" => BreakpointLogLevel::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BreakpointLogLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BreakpointLogLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BreakpointLogLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => BreakpointLogLevel::Error,
                "INFO" => BreakpointLogLevel::Info,
                "WARNING" => BreakpointLogLevel::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BreakpointLogLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BreakpointLogLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BreakpointState {
        #[doc = "Enabling canary and successfully assigning canary agents."]
        StateCanaryActive,
        #[doc = "Enabling canary but no agents are available."]
        StateCanaryPendingAgents,
        #[doc = "Breakpoint is hit/complete/failed."]
        StateIsFinal,
        #[doc = "Breakpoint rolling out to all agents."]
        StateRollingToAll,
        #[doc = "Breakpoint state UNSPECIFIED."]
        StateUnspecified,
    }
    impl BreakpointState {
        pub fn as_str(self) -> &'static str {
            match self {
                BreakpointState::StateCanaryActive => "STATE_CANARY_ACTIVE",
                BreakpointState::StateCanaryPendingAgents => "STATE_CANARY_PENDING_AGENTS",
                BreakpointState::StateIsFinal => "STATE_IS_FINAL",
                BreakpointState::StateRollingToAll => "STATE_ROLLING_TO_ALL",
                BreakpointState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BreakpointState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BreakpointState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BreakpointState, ()> {
            Ok(match s {
                "STATE_CANARY_ACTIVE" => BreakpointState::StateCanaryActive,
                "STATE_CANARY_PENDING_AGENTS" => BreakpointState::StateCanaryPendingAgents,
                "STATE_IS_FINAL" => BreakpointState::StateIsFinal,
                "STATE_ROLLING_TO_ALL" => BreakpointState::StateRollingToAll,
                "STATE_UNSPECIFIED" => BreakpointState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BreakpointState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BreakpointState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BreakpointState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATE_CANARY_ACTIVE" => BreakpointState::StateCanaryActive,
                "STATE_CANARY_PENDING_AGENTS" => BreakpointState::StateCanaryPendingAgents,
                "STATE_IS_FINAL" => BreakpointState::StateIsFinal,
                "STATE_ROLLING_TO_ALL" => BreakpointState::StateRollingToAll,
                "STATE_UNSPECIFIED" => BreakpointState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BreakpointState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BreakpointState {
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
    pub struct CloudRepoSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<crate::schemas::AliasContext>,
        #[doc = "The name of an alias (branch, tag, etc.)."]
        #[serde(
            rename = "aliasName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_name: ::std::option::Option<String>,
        #[doc = "The ID of the repo."]
        #[serde(
            rename = "repoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_id: ::std::option::Option<crate::schemas::RepoId>,
        #[doc = "A revision ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CloudRepoSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudRepoSourceContext {
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
    pub struct CloudWorkspaceId {
        #[doc = "The unique name of the workspace within the repo.  This is the name\nchosen by the client in the Source API's CreateWorkspace method."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID of the repo containing the workspace."]
        #[serde(
            rename = "repoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_id: ::std::option::Option<crate::schemas::RepoId>,
    }
    impl ::google_field_selector::FieldSelector for CloudWorkspaceId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudWorkspaceId {
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
    pub struct CloudWorkspaceSourceContext {
        #[doc = "The ID of the snapshot.\nAn empty snapshot_id refers to the most recent snapshot."]
        #[serde(
            rename = "snapshotId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snapshot_id: ::std::option::Option<String>,
        #[doc = "The ID of the workspace."]
        #[serde(
            rename = "workspaceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub workspace_id: ::std::option::Option<crate::schemas::CloudWorkspaceId>,
    }
    impl ::google_field_selector::FieldSelector for CloudWorkspaceSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudWorkspaceSourceContext {
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
    pub struct Debuggee {
        #[doc = "Version ID of the agent.\nSchema: `domain/language-platform/vmajor.minor` (for example\n`google.com/java-gcp/v1.1`)."]
        #[serde(
            rename = "agentVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub agent_version: ::std::option::Option<String>,
        #[doc = "Used when setting breakpoint canary for this debuggee."]
        #[serde(
            rename = "canaryMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canary_mode: ::std::option::Option<crate::schemas::DebuggeeCanaryMode>,
        #[doc = "Human readable description of the debuggee.\nIncluding a human-readable project name, environment name and version\ninformation is recommended."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "References to the locations and revisions of the source code used in the\ndeployed application."]
        #[serde(
            rename = "extSourceContexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ext_source_contexts: ::std::option::Option<Vec<crate::schemas::ExtendedSourceContext>>,
        #[doc = "Unique identifier for the debuggee generated by the controller service."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "If set to `true`, indicates that the agent should disable itself and\ndetach from the debuggee."]
        #[serde(
            rename = "isDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_disabled: ::std::option::Option<bool>,
        #[doc = "If set to `true`, indicates that Controller service does not detect any\nactivity from the debuggee agents and the application is possibly stopped."]
        #[serde(
            rename = "isInactive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_inactive: ::std::option::Option<bool>,
        #[doc = "A set of custom debuggee properties, populated by the agent, to be\ndisplayed to the user."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Project the debuggee is associated with.\nUse project number or id when registering a Google Cloud Platform project."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "References to the locations and revisions of the source code used in the\ndeployed application."]
        #[serde(
            rename = "sourceContexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_contexts: ::std::option::Option<Vec<crate::schemas::SourceContext>>,
        #[doc = "Human readable message to be displayed to the user about this debuggee.\nAbsence of this field indicates no status. The message can be either\ninformational or an error status."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::StatusMessage>,
        #[doc = "Uniquifier to further distinguish the application.\nIt is possible that different applications might have identical values in\nthe debuggee message, thus, incorrectly identified as a single application\nby the Controller service. This field adds salt to further distinguish the\napplication. Agents should consider seeding this field with value that\nidentifies the code, binary, configuration and environment."]
        #[serde(
            rename = "uniquifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uniquifier: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Debuggee {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Debuggee {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DebuggeeCanaryMode {
        #[doc = "Always disable breakpoint canary regardless of the value of breakpoint's\ncanary option."]
        CanaryModeAlwaysDisabled,
        #[doc = "Always enable breakpoint canary regardless of the value of breakpoint's\ncanary option."]
        CanaryModeAlwaysEnabled,
        #[doc = "Depends on the breakpoint's canary option. Disable canary by default if\nthe breakpoint's canary option is not specified."]
        CanaryModeDefaultDisabled,
        #[doc = "Depends on the breakpoint's canary option. Enable canary by default if\nthe breakpoint's canary option is not specified."]
        CanaryModeDefaultEnabled,
        #[doc = "CANARY_MODE_UNSPECIFIED is equivalent to CANARY_MODE_ALWAYS_DISABLED so\nthat if the debuggee is not configured to use the canary feature, the\nfeature will be disabled."]
        CanaryModeUnspecified,
    }
    impl DebuggeeCanaryMode {
        pub fn as_str(self) -> &'static str {
            match self {
                DebuggeeCanaryMode::CanaryModeAlwaysDisabled => "CANARY_MODE_ALWAYS_DISABLED",
                DebuggeeCanaryMode::CanaryModeAlwaysEnabled => "CANARY_MODE_ALWAYS_ENABLED",
                DebuggeeCanaryMode::CanaryModeDefaultDisabled => "CANARY_MODE_DEFAULT_DISABLED",
                DebuggeeCanaryMode::CanaryModeDefaultEnabled => "CANARY_MODE_DEFAULT_ENABLED",
                DebuggeeCanaryMode::CanaryModeUnspecified => "CANARY_MODE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DebuggeeCanaryMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DebuggeeCanaryMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DebuggeeCanaryMode, ()> {
            Ok(match s {
                "CANARY_MODE_ALWAYS_DISABLED" => DebuggeeCanaryMode::CanaryModeAlwaysDisabled,
                "CANARY_MODE_ALWAYS_ENABLED" => DebuggeeCanaryMode::CanaryModeAlwaysEnabled,
                "CANARY_MODE_DEFAULT_DISABLED" => DebuggeeCanaryMode::CanaryModeDefaultDisabled,
                "CANARY_MODE_DEFAULT_ENABLED" => DebuggeeCanaryMode::CanaryModeDefaultEnabled,
                "CANARY_MODE_UNSPECIFIED" => DebuggeeCanaryMode::CanaryModeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DebuggeeCanaryMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DebuggeeCanaryMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DebuggeeCanaryMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANARY_MODE_ALWAYS_DISABLED" => DebuggeeCanaryMode::CanaryModeAlwaysDisabled,
                "CANARY_MODE_ALWAYS_ENABLED" => DebuggeeCanaryMode::CanaryModeAlwaysEnabled,
                "CANARY_MODE_DEFAULT_DISABLED" => DebuggeeCanaryMode::CanaryModeDefaultDisabled,
                "CANARY_MODE_DEFAULT_ENABLED" => DebuggeeCanaryMode::CanaryModeDefaultEnabled,
                "CANARY_MODE_UNSPECIFIED" => DebuggeeCanaryMode::CanaryModeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DebuggeeCanaryMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DebuggeeCanaryMode {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
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
    pub struct ExtendedSourceContext {
        #[doc = "Any source context."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "Labels with user defined metadata."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedSourceContext {
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
    pub struct FormatMessage {
        #[doc = "Format template for the message. The `format` uses placeholders `$0`,\n`$1`, etc. to reference parameters. `$$` can be used to denote the `$`\ncharacter.\n\nExamples:\n\n* `Failed to load '$0' which helps debug $1 the first time it is loaded.  Again, $0 is very important.`\n* `Please pay $$10 to use $0 instead of $1.`"]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
        #[doc = "Optional parameters to be embedded into the message."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for FormatMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FormatMessage {
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
    pub struct GerritSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<crate::schemas::AliasContext>,
        #[doc = "The name of an alias (branch, tag, etc.)."]
        #[serde(
            rename = "aliasName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_name: ::std::option::Option<String>,
        #[doc = "The full project name within the host. Projects may be nested, so\n\"project/subproject\" is a valid project name.\nThe \"repo name\" is hostURI/project."]
        #[serde(
            rename = "gerritProject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit_project: ::std::option::Option<String>,
        #[doc = "The URI of a running Gerrit instance."]
        #[serde(
            rename = "hostUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_uri: ::std::option::Option<String>,
        #[doc = "A revision (commit) ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GerritSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GerritSourceContext {
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
    pub struct GetBreakpointResponse {
        #[doc = "Complete breakpoint state.\nThe fields `id` and `location` are guaranteed to be set."]
        #[serde(
            rename = "breakpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub breakpoint: ::std::option::Option<crate::schemas::Breakpoint>,
    }
    impl ::google_field_selector::FieldSelector for GetBreakpointResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetBreakpointResponse {
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
    pub struct GitSourceContext {
        #[doc = "Git commit hash.\nrequired."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Git repository URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GitSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GitSourceContext {
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
    pub struct ListActiveBreakpointsResponse {
        #[doc = "List of all active breakpoints.\nThe fields `id` and `location` are guaranteed to be set on each breakpoint."]
        #[serde(
            rename = "breakpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub breakpoints: ::std::option::Option<Vec<crate::schemas::Breakpoint>>,
        #[doc = "A token that can be used in the next method call to block until\nthe list of breakpoints changes."]
        #[serde(
            rename = "nextWaitToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_wait_token: ::std::option::Option<String>,
        #[doc = "If set to `true`, indicates that there is no change to the\nlist of active breakpoints and the server-selected timeout has expired.\nThe `breakpoints` field would be empty and should be ignored."]
        #[serde(
            rename = "waitExpired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wait_expired: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ListActiveBreakpointsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListActiveBreakpointsResponse {
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
    pub struct ListBreakpointsResponse {
        #[doc = "List of breakpoints matching the request.\nThe fields `id` and `location` are guaranteed to be set on each breakpoint.\nThe fields: `stack_frames`, `evaluated_expressions` and `variable_table`\nare cleared on each breakpoint regardless of its status."]
        #[serde(
            rename = "breakpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub breakpoints: ::std::option::Option<Vec<crate::schemas::Breakpoint>>,
        #[doc = "A wait token that can be used in the next call to `list` (REST) or\n`ListBreakpoints` (RPC) to block until the list of breakpoints has changes."]
        #[serde(
            rename = "nextWaitToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_wait_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListBreakpointsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListBreakpointsResponse {
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
    pub struct ListDebuggeesResponse {
        #[doc = "List of debuggees accessible to the calling user.\nThe fields `debuggee.id` and `description` are guaranteed to be set.\nThe `description` field is a human readable field provided by agents and\ncan be displayed to users."]
        #[serde(
            rename = "debuggees",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debuggees: ::std::option::Option<Vec<crate::schemas::Debuggee>>,
    }
    impl ::google_field_selector::FieldSelector for ListDebuggeesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDebuggeesResponse {
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
    pub struct ProjectRepoId {
        #[doc = "The ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The name of the repo. Leave empty for the default repo."]
        #[serde(
            rename = "repoName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProjectRepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectRepoId {
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
    pub struct RegisterDebuggeeRequest {
        #[doc = "Required. Debuggee information to register.\nThe fields `project`, `uniquifier`, `description` and `agent_version`\nof the debuggee must be set."]
        #[serde(
            rename = "debuggee",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debuggee: ::std::option::Option<crate::schemas::Debuggee>,
    }
    impl ::google_field_selector::FieldSelector for RegisterDebuggeeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RegisterDebuggeeRequest {
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
    pub struct RegisterDebuggeeResponse {
        #[doc = "A unique ID generated for the agent.\nEach RegisterDebuggee request will generate a new agent ID."]
        #[serde(
            rename = "agentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub agent_id: ::std::option::Option<String>,
        #[doc = "Debuggee resource.\nThe field `id` is guaranteed to be set (in addition to the echoed fields).\nIf the field `is_disabled` is set to `true`, the agent should disable\nitself by removing all breakpoints and detaching from the application.\nIt should however continue to poll `RegisterDebuggee` until reenabled."]
        #[serde(
            rename = "debuggee",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debuggee: ::std::option::Option<crate::schemas::Debuggee>,
    }
    impl ::google_field_selector::FieldSelector for RegisterDebuggeeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RegisterDebuggeeResponse {
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
    pub struct RepoId {
        #[doc = "A combination of a project ID and a repo name."]
        #[serde(
            rename = "projectRepoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_repo_id: ::std::option::Option<crate::schemas::ProjectRepoId>,
        #[doc = "A server-assigned, globally unique identifier."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepoId {
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
    pub struct SetBreakpointResponse {
        #[doc = "Breakpoint resource.\nThe field `id` is guaranteed to be set (in addition to the echoed fields)."]
        #[serde(
            rename = "breakpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub breakpoint: ::std::option::Option<crate::schemas::Breakpoint>,
    }
    impl ::google_field_selector::FieldSelector for SetBreakpointResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetBreakpointResponse {
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
    pub struct SourceContext {
        #[doc = "A SourceContext referring to a revision in a cloud repo."]
        #[serde(
            rename = "cloudRepo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_repo: ::std::option::Option<crate::schemas::CloudRepoSourceContext>,
        #[doc = "A SourceContext referring to a snapshot in a cloud workspace."]
        #[serde(
            rename = "cloudWorkspace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_workspace: ::std::option::Option<crate::schemas::CloudWorkspaceSourceContext>,
        #[doc = "A SourceContext referring to a Gerrit project."]
        #[serde(
            rename = "gerrit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit: ::std::option::Option<crate::schemas::GerritSourceContext>,
        #[doc = "A SourceContext referring to any third party Git repo (e.g. GitHub)."]
        #[serde(
            rename = "git",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git: ::std::option::Option<crate::schemas::GitSourceContext>,
    }
    impl ::google_field_selector::FieldSelector for SourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceContext {
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
    pub struct SourceLocation {
        #[doc = "Column within a line. The first column in a line as the value `1`.\nAgents that do not support setting breakpoints on specific columns ignore\nthis field."]
        #[serde(
            rename = "column",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column: ::std::option::Option<i32>,
        #[doc = "Line inside the file. The first line in the file has the value `1`."]
        #[serde(
            rename = "line",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line: ::std::option::Option<i32>,
        #[doc = "Path to the source file within the source context of the target binary."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SourceLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceLocation {
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
    pub struct StackFrame {
        #[doc = "Set of arguments passed to this function.\nNote that this might not be populated for all stack frames."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<Vec<crate::schemas::Variable>>,
        #[doc = "Demangled function name at the call site."]
        #[serde(
            rename = "function",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function: ::std::option::Option<String>,
        #[doc = "Set of local variables at the stack frame location.\nNote that this might not be populated for all stack frames."]
        #[serde(
            rename = "locals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locals: ::std::option::Option<Vec<crate::schemas::Variable>>,
        #[doc = "Source location of the call site."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::SourceLocation>,
    }
    impl ::google_field_selector::FieldSelector for StackFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StackFrame {
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
    pub struct StatusMessage {
        #[doc = "Status message text."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<crate::schemas::FormatMessage>,
        #[doc = "Distinguishes errors from informational messages."]
        #[serde(
            rename = "isError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_error: ::std::option::Option<bool>,
        #[doc = "Reference to which the message applies."]
        #[serde(
            rename = "refersTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refers_to: ::std::option::Option<crate::schemas::StatusMessageRefersTo>,
    }
    impl ::google_field_selector::FieldSelector for StatusMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StatusMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StatusMessageRefersTo {
        #[doc = "Status applies to the breakpoint and is related to its age."]
        BreakpointAge,
        #[doc = "Status applies to the breakpoint when the breakpoint failed to exit the\ncanary state."]
        BreakpointCanaryFailed,
        #[doc = "Status applies to the breakpoint and is related to its condition."]
        BreakpointCondition,
        #[doc = "Status applies to the breakpoint and is related to its expressions."]
        BreakpointExpression,
        #[doc = "Status applies to the breakpoint and is related to its location."]
        BreakpointSourceLocation,
        #[doc = "Status doesn't refer to any particular input."]
        Unspecified,
        #[doc = "Status applies to the entire variable."]
        VariableName,
        #[doc = "Status applies to variable value (variable name is valid)."]
        VariableValue,
    }
    impl StatusMessageRefersTo {
        pub fn as_str(self) -> &'static str {
            match self {
                StatusMessageRefersTo::BreakpointAge => "BREAKPOINT_AGE",
                StatusMessageRefersTo::BreakpointCanaryFailed => "BREAKPOINT_CANARY_FAILED",
                StatusMessageRefersTo::BreakpointCondition => "BREAKPOINT_CONDITION",
                StatusMessageRefersTo::BreakpointExpression => "BREAKPOINT_EXPRESSION",
                StatusMessageRefersTo::BreakpointSourceLocation => "BREAKPOINT_SOURCE_LOCATION",
                StatusMessageRefersTo::Unspecified => "UNSPECIFIED",
                StatusMessageRefersTo::VariableName => "VARIABLE_NAME",
                StatusMessageRefersTo::VariableValue => "VARIABLE_VALUE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for StatusMessageRefersTo {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StatusMessageRefersTo {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StatusMessageRefersTo, ()> {
            Ok(match s {
                "BREAKPOINT_AGE" => StatusMessageRefersTo::BreakpointAge,
                "BREAKPOINT_CANARY_FAILED" => StatusMessageRefersTo::BreakpointCanaryFailed,
                "BREAKPOINT_CONDITION" => StatusMessageRefersTo::BreakpointCondition,
                "BREAKPOINT_EXPRESSION" => StatusMessageRefersTo::BreakpointExpression,
                "BREAKPOINT_SOURCE_LOCATION" => StatusMessageRefersTo::BreakpointSourceLocation,
                "UNSPECIFIED" => StatusMessageRefersTo::Unspecified,
                "VARIABLE_NAME" => StatusMessageRefersTo::VariableName,
                "VARIABLE_VALUE" => StatusMessageRefersTo::VariableValue,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StatusMessageRefersTo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StatusMessageRefersTo {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StatusMessageRefersTo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BREAKPOINT_AGE" => StatusMessageRefersTo::BreakpointAge,
                "BREAKPOINT_CANARY_FAILED" => StatusMessageRefersTo::BreakpointCanaryFailed,
                "BREAKPOINT_CONDITION" => StatusMessageRefersTo::BreakpointCondition,
                "BREAKPOINT_EXPRESSION" => StatusMessageRefersTo::BreakpointExpression,
                "BREAKPOINT_SOURCE_LOCATION" => StatusMessageRefersTo::BreakpointSourceLocation,
                "UNSPECIFIED" => StatusMessageRefersTo::Unspecified,
                "VARIABLE_NAME" => StatusMessageRefersTo::VariableName,
                "VARIABLE_VALUE" => StatusMessageRefersTo::VariableValue,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for StatusMessageRefersTo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StatusMessageRefersTo {
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
    pub struct UpdateActiveBreakpointRequest {
        #[doc = "Required. Updated breakpoint information.\nThe field `id` must be set.\nThe agent must echo all Breakpoint specification fields in the update."]
        #[serde(
            rename = "breakpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub breakpoint: ::std::option::Option<crate::schemas::Breakpoint>,
    }
    impl ::google_field_selector::FieldSelector for UpdateActiveBreakpointRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateActiveBreakpointRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateActiveBreakpointResponse {}
    impl ::google_field_selector::FieldSelector for UpdateActiveBreakpointResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateActiveBreakpointResponse {
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
    pub struct Variable {
        #[doc = "Members contained or pointed to by the variable."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<crate::schemas::Variable>>,
        #[doc = "Name of the variable, if any."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Variable type (e.g. `MyClass`). If the variable is split with\n`var_table_index`, `type` goes next to `value`. The interpretation of\na type is agent specific. It is recommended to include the dynamic type\nrather than a static type of an object."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Status associated with the variable. This field will usually stay\nunset. A status of a single variable only applies to that variable or\nexpression. The rest of breakpoint data still remains valid. Variables\nmight be reported in error state even when breakpoint is not in final\nstate.\n\nThe message may refer to variable name with `refers_to` set to\n`VARIABLE_NAME`. Alternatively `refers_to` will be set to `VARIABLE_VALUE`.\nIn either case variable value and members will be unset.\n\nExample of error message applied to name: `Invalid expression syntax`.\n\nExample of information message applied to value: `Not captured`.\n\nExamples of error message applied to value:\n\n* `Malformed string`,\n* `Field f not found in class C`\n* `Null pointer dereference`"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::StatusMessage>,
        #[doc = "Simple value of the variable."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
        #[doc = "Reference to a variable in the shared variable table. More than\none variable can reference the same variable in the table. The\n`var_table_index` field is an index into `variable_table` in Breakpoint."]
        #[serde(
            rename = "varTableIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub var_table_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Variable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Variable {
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
    #[doc = "Actions that can be performed on the controller resource"]
    pub fn controller(&self) -> crate::resources::controller::ControllerActions {
        crate::resources::controller::ControllerActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the debugger resource"]
    pub fn debugger(&self) -> crate::resources::debugger::DebuggerActions {
        crate::resources::debugger::DebuggerActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod controller {
        pub mod params {}
        pub struct ControllerActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ControllerActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the debuggees resource"]
            pub fn debuggees(&self) -> crate::resources::controller::debuggees::DebuggeesActions {
                crate::resources::controller::debuggees::DebuggeesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod debuggees {
            pub mod params {}
            pub struct DebuggeesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DebuggeesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Registers the debuggee with the controller service.\n\nAll agents attached to the same application must call this method with\nexactly the same request content to get back the same stable `debuggee_id`.\nAgents should call this method again whenever `google.rpc.Code.NOT_FOUND`\nis returned from any controller method.\n\nThis protocol allows the controller service to disable debuggees, recover\nfrom data loss, or change the `debuggee_id` format. Agents must handle\n`debuggee_id` value changing upon re-registration."]
                pub fn register(
                    &self,
                    request: crate::schemas::RegisterDebuggeeRequest,
                ) -> RegisterRequestBuilder {
                    RegisterRequestBuilder {
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
                #[doc = "Actions that can be performed on the breakpoints resource"]
                pub fn breakpoints(
                    &self,
                ) -> crate::resources::controller::debuggees::breakpoints::BreakpointsActions
                {
                    crate::resources::controller::debuggees::breakpoints::BreakpointsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [DebuggeesActions::register()](struct.DebuggeesActions.html#method.register)"]
            #[derive(Debug, Clone)]
            pub struct RegisterRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::RegisterDebuggeeRequest,
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
            impl<'a> RegisterRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::RegisterDebuggeeResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::RegisterDebuggeeResponse, crate::Error>
                {
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                    output.push_str("v2/controller/debuggees/register");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
            pub mod breakpoints {
                pub mod params {}
                pub struct BreakpointsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> BreakpointsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Returns the list of all active breakpoints for the debuggee.\n\nThe breakpoint specification (`location`, `condition`, and `expressions`\nfields) is semantically immutable, although the field values may\nchange. For example, an agent may update the location line number\nto reflect the actual line where the breakpoint was set, but this\ndoesn't change the breakpoint semantics.\n\nThis means that an agent does not need to check if a breakpoint has changed\nwhen it encounters the same breakpoint on a successive call.\nMoreover, an agent should remember the breakpoints that are completed\nuntil the controller removes them from the active list to avoid\nsetting those breakpoints again."]
                    pub fn list(&self, debuggee_id: impl Into<String>) -> ListRequestBuilder {
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
                            debuggee_id: debuggee_id.into(),
                            agent_id: None,
                            success_on_timeout: None,
                            wait_token: None,
                        }
                    }
                    #[doc = "Updates the breakpoint state or mutable fields.\nThe entire Breakpoint message must be sent back to the controller service.\n\nUpdates to active breakpoint fields are only allowed if the new value\ndoes not change the breakpoint specification. Updates to the `location`,\n`condition` and `expressions` fields should not alter the breakpoint\nsemantics. These may only make changes such as canonicalizing a value\nor snapping the location to the correct line of code."]
                    pub fn update(
                        &self,
                        request: crate::schemas::UpdateActiveBreakpointRequest,
                        debuggee_id: impl Into<String>,
                        id: impl Into<String>,
                    ) -> UpdateRequestBuilder {
                        UpdateRequestBuilder {
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
                            debuggee_id: debuggee_id.into(),
                            id: id.into(),
                        }
                    }
                }
                #[doc = "Created via [BreakpointsActions::list()](struct.BreakpointsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    debuggee_id: String,
                    agent_id: Option<String>,
                    success_on_timeout: Option<bool>,
                    wait_token: Option<String>,
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
                    #[doc = "Identifies the agent.\nThis is the ID returned in the RegisterDebuggee response."]
                    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
                        self.agent_id = Some(value.into());
                        self
                    }
                    #[doc = "If set to `true` (recommended), returns `google.rpc.Code.OK` status and\nsets the `wait_expired` response field to `true` when the server-selected\ntimeout has expired.\n\nIf set to `false` (deprecated), returns `google.rpc.Code.ABORTED` status\nwhen the server-selected timeout has expired."]
                    pub fn success_on_timeout(mut self, value: bool) -> Self {
                        self.success_on_timeout = Some(value);
                        self
                    }
                    #[doc = "A token that, if specified, blocks the method call until the list\nof active breakpoints has changed, or a server-selected timeout has\nexpired. The value should be set from the `next_wait_token` field in\nthe last response. The initial value should be set to `\"init\"`."]
                    pub fn wait_token(mut self, value: impl Into<String>) -> Self {
                        self.wait_token = Some(value.into());
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
                    ) -> Result<crate::schemas::ListActiveBreakpointsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListActiveBreakpointsResponse, crate::Error>
                    {
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
                        let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                        output.push_str("v2/controller/debuggees/");
                        {
                            let var_as_str = &self.debuggee_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/breakpoints");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("agentId", &self.agent_id)]);
                        let req = req.query(&[("successOnTimeout", &self.success_on_timeout)]);
                        let req = req.query(&[("waitToken", &self.wait_token)]);
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
                #[doc = "Created via [BreakpointsActions::update()](struct.BreakpointsActions.html#method.update)"]
                #[derive(Debug, Clone)]
                pub struct UpdateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::UpdateActiveBreakpointRequest,
                    debuggee_id: String,
                    id: String,
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
                impl<'a> UpdateRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::UpdateActiveBreakpointResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::UpdateActiveBreakpointResponse, crate::Error>
                    {
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                        output.push_str("v2/controller/debuggees/");
                        {
                            let var_as_str = &self.debuggee_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/breakpoints/");
                        {
                            let var_as_str = &self.id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
    pub mod debugger {
        pub mod params {}
        pub struct DebuggerActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DebuggerActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the debuggees resource"]
            pub fn debuggees(&self) -> crate::resources::debugger::debuggees::DebuggeesActions {
                crate::resources::debugger::debuggees::DebuggeesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod debuggees {
            pub mod params {}
            pub struct DebuggeesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DebuggeesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists all the debuggees that the user has access to."]
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
                        client_version: None,
                        include_inactive: None,
                        project: None,
                    }
                }
                #[doc = "Actions that can be performed on the breakpoints resource"]
                pub fn breakpoints(
                    &self,
                ) -> crate::resources::debugger::debuggees::breakpoints::BreakpointsActions
                {
                    crate::resources::debugger::debuggees::breakpoints::BreakpointsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [DebuggeesActions::list()](struct.DebuggeesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                client_version: Option<String>,
                include_inactive: Option<bool>,
                project: Option<String>,
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
                #[doc = "Required. The client version making the call.\nSchema: `domain/type/version` (e.g., `google.com/intellij/v1`)."]
                pub fn client_version(mut self, value: impl Into<String>) -> Self {
                    self.client_version = Some(value.into());
                    self
                }
                #[doc = "When set to `true`, the result includes all debuggees. Otherwise, the\nresult includes only debuggees that are active."]
                pub fn include_inactive(mut self, value: bool) -> Self {
                    self.include_inactive = Some(value);
                    self
                }
                #[doc = "Required. Project number of a Google Cloud project whose debuggees to list."]
                pub fn project(mut self, value: impl Into<String>) -> Self {
                    self.project = Some(value.into());
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
                ) -> Result<crate::schemas::ListDebuggeesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDebuggeesResponse, crate::Error> {
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
                    let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                    output.push_str("v2/debugger/debuggees");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("clientVersion", &self.client_version)]);
                    let req = req.query(&[("includeInactive", &self.include_inactive)]);
                    let req = req.query(&[("project", &self.project)]);
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
            pub mod breakpoints {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListActionValue {
                        Capture,
                        Log,
                    }
                    impl ListActionValue {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListActionValue::Capture => "CAPTURE",
                                ListActionValue::Log => "LOG",
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for ListActionValue {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for ListActionValue {
                        type Err = ();
                        fn from_str(s: &str) -> ::std::result::Result<ListActionValue, ()> {
                            Ok(match s {
                                "CAPTURE" => ListActionValue::Capture,
                                "LOG" => ListActionValue::Log,
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for ListActionValue {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListActionValue {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListActionValue {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "CAPTURE" => ListActionValue::Capture,
                                "LOG" => ListActionValue::Log,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for ListActionValue {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for ListActionValue {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum SetCanaryOption {
                        CanaryOptionTryDisable,
                        CanaryOptionTryEnable,
                        CanaryOptionUnspecified,
                    }
                    impl SetCanaryOption {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                SetCanaryOption::CanaryOptionTryDisable => {
                                    "CANARY_OPTION_TRY_DISABLE"
                                }
                                SetCanaryOption::CanaryOptionTryEnable => {
                                    "CANARY_OPTION_TRY_ENABLE"
                                }
                                SetCanaryOption::CanaryOptionUnspecified => {
                                    "CANARY_OPTION_UNSPECIFIED"
                                }
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for SetCanaryOption {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for SetCanaryOption {
                        type Err = ();
                        fn from_str(s: &str) -> ::std::result::Result<SetCanaryOption, ()> {
                            Ok(match s {
                                "CANARY_OPTION_TRY_DISABLE" => {
                                    SetCanaryOption::CanaryOptionTryDisable
                                }
                                "CANARY_OPTION_TRY_ENABLE" => {
                                    SetCanaryOption::CanaryOptionTryEnable
                                }
                                "CANARY_OPTION_UNSPECIFIED" => {
                                    SetCanaryOption::CanaryOptionUnspecified
                                }
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for SetCanaryOption {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for SetCanaryOption {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for SetCanaryOption {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "CANARY_OPTION_TRY_DISABLE" => {
                                    SetCanaryOption::CanaryOptionTryDisable
                                }
                                "CANARY_OPTION_TRY_ENABLE" => {
                                    SetCanaryOption::CanaryOptionTryEnable
                                }
                                "CANARY_OPTION_UNSPECIFIED" => {
                                    SetCanaryOption::CanaryOptionUnspecified
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
                    impl ::google_field_selector::FieldSelector for SetCanaryOption {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for SetCanaryOption {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct BreakpointsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> BreakpointsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Deletes the breakpoint from the debuggee."]
                    pub fn delete(
                        &self,
                        debuggee_id: impl Into<String>,
                        breakpoint_id: impl Into<String>,
                    ) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
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
                            debuggee_id: debuggee_id.into(),
                            breakpoint_id: breakpoint_id.into(),
                            client_version: None,
                        }
                    }
                    #[doc = "Gets breakpoint information."]
                    pub fn get(
                        &self,
                        debuggee_id: impl Into<String>,
                        breakpoint_id: impl Into<String>,
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
                            debuggee_id: debuggee_id.into(),
                            breakpoint_id: breakpoint_id.into(),
                            client_version: None,
                        }
                    }
                    #[doc = "Lists all breakpoints for the debuggee."]
                    pub fn list(&self, debuggee_id: impl Into<String>) -> ListRequestBuilder {
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
                            debuggee_id: debuggee_id.into(),
                            action_value: None,
                            client_version: None,
                            include_all_users: None,
                            include_inactive: None,
                            strip_results: None,
                            wait_token: None,
                        }
                    }
                    #[doc = "Sets the breakpoint to the debuggee."]
                    pub fn set(
                        &self,
                        request: crate::schemas::Breakpoint,
                        debuggee_id: impl Into<String>,
                    ) -> SetRequestBuilder {
                        SetRequestBuilder {
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
                            debuggee_id: debuggee_id.into(),
                            canary_option: None,
                            client_version: None,
                        }
                    }
                }
                #[doc = "Created via [BreakpointsActions::delete()](struct.BreakpointsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    debuggee_id: String,
                    breakpoint_id: String,
                    client_version: Option<String>,
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
                impl<'a> DeleteRequestBuilder<'a> {
                    #[doc = "Required. The client version making the call.\nSchema: `domain/type/version` (e.g., `google.com/intellij/v1`)."]
                    pub fn client_version(mut self, value: impl Into<String>) -> Self {
                        self.client_version = Some(value.into());
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
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
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
                        let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                        output.push_str("v2/debugger/debuggees/");
                        {
                            let var_as_str = &self.debuggee_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/breakpoints/");
                        {
                            let var_as_str = &self.breakpoint_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("clientVersion", &self.client_version)]);
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
                #[doc = "Created via [BreakpointsActions::get()](struct.BreakpointsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    debuggee_id: String,
                    breakpoint_id: String,
                    client_version: Option<String>,
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
                    #[doc = "Required. The client version making the call.\nSchema: `domain/type/version` (e.g., `google.com/intellij/v1`)."]
                    pub fn client_version(mut self, value: impl Into<String>) -> Self {
                        self.client_version = Some(value.into());
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
                    ) -> Result<crate::schemas::GetBreakpointResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GetBreakpointResponse, crate::Error>
                    {
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
                        let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                        output.push_str("v2/debugger/debuggees/");
                        {
                            let var_as_str = &self.debuggee_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/breakpoints/");
                        {
                            let var_as_str = &self.breakpoint_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("clientVersion", &self.client_version)]);
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
                #[doc = "Created via [BreakpointsActions::list()](struct.BreakpointsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    debuggee_id: String,
                    action_value: Option<
                        crate::resources::debugger::debuggees::breakpoints::params::ListActionValue,
                    >,
                    client_version: Option<String>,
                    include_all_users: Option<bool>,
                    include_inactive: Option<bool>,
                    strip_results: Option<bool>,
                    wait_token: Option<String>,
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
                    #[doc = "Only breakpoints with the specified action will pass the filter."]
                    pub fn action_value(
                        mut self,
                        value : crate :: resources :: debugger :: debuggees :: breakpoints :: params :: ListActionValue,
                    ) -> Self {
                        self.action_value = Some(value);
                        self
                    }
                    #[doc = "Required. The client version making the call.\nSchema: `domain/type/version` (e.g., `google.com/intellij/v1`)."]
                    pub fn client_version(mut self, value: impl Into<String>) -> Self {
                        self.client_version = Some(value.into());
                        self
                    }
                    #[doc = "When set to `true`, the response includes the list of breakpoints set by\nany user. Otherwise, it includes only breakpoints set by the caller."]
                    pub fn include_all_users(mut self, value: bool) -> Self {
                        self.include_all_users = Some(value);
                        self
                    }
                    #[doc = "When set to `true`, the response includes active and inactive\nbreakpoints. Otherwise, it includes only active breakpoints."]
                    pub fn include_inactive(mut self, value: bool) -> Self {
                        self.include_inactive = Some(value);
                        self
                    }
                    #[doc = "This field is deprecated. The following fields are always stripped out of\nthe result: `stack_frames`, `evaluated_expressions` and `variable_table`."]
                    pub fn strip_results(mut self, value: bool) -> Self {
                        self.strip_results = Some(value);
                        self
                    }
                    #[doc = "A wait token that, if specified, blocks the call until the breakpoints\nlist has changed, or a server selected timeout has expired.  The value\nshould be set from the last response. The error code\n`google.rpc.Code.ABORTED` (RPC) is returned on wait timeout, which\nshould be called again with the same `wait_token`."]
                    pub fn wait_token(mut self, value: impl Into<String>) -> Self {
                        self.wait_token = Some(value.into());
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
                    ) -> Result<crate::schemas::ListBreakpointsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListBreakpointsResponse, crate::Error>
                    {
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
                        let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                        output.push_str("v2/debugger/debuggees/");
                        {
                            let var_as_str = &self.debuggee_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/breakpoints");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("action.value", &self.action_value)]);
                        let req = req.query(&[("clientVersion", &self.client_version)]);
                        let req = req.query(&[("includeAllUsers", &self.include_all_users)]);
                        let req = req.query(&[("includeInactive", &self.include_inactive)]);
                        let req = req.query(&[("stripResults", &self.strip_results)]);
                        let req = req.query(&[("waitToken", &self.wait_token)]);
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
                #[doc = "Created via [BreakpointsActions::set()](struct.BreakpointsActions.html#method.set)"]
                #[derive(Debug, Clone)]
                pub struct SetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Breakpoint,
                    debuggee_id: String,
                    canary_option: Option<
                        crate::resources::debugger::debuggees::breakpoints::params::SetCanaryOption,
                    >,
                    client_version: Option<String>,
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
                impl<'a> SetRequestBuilder<'a> {
                    #[doc = "The canary option set by the user upon setting breakpoint."]
                    pub fn canary_option(
                        mut self,
                        value : crate :: resources :: debugger :: debuggees :: breakpoints :: params :: SetCanaryOption,
                    ) -> Self {
                        self.canary_option = Some(value);
                        self
                    }
                    #[doc = "Required. The client version making the call.\nSchema: `domain/type/version` (e.g., `google.com/intellij/v1`)."]
                    pub fn client_version(mut self, value: impl Into<String>) -> Self {
                        self.client_version = Some(value.into());
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
                    ) -> Result<crate::schemas::SetBreakpointResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::SetBreakpointResponse, crate::Error>
                    {
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://clouddebugger.googleapis.com/".to_owned();
                        output.push_str("v2/debugger/debuggees/");
                        {
                            let var_as_str = &self.debuggee_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/breakpoints/set");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("canaryOption", &self.canary_option)]);
                        let req = req.query(&[("clientVersion", &self.client_version)]);
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
