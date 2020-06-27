#![doc = "# Resources and Methods\n    * [processes](resources/processes/struct.ProcessesActions.html)\n      * [*list*](resources/processes/struct.ListRequestBuilder.html), [*listScriptProcesses*](resources/processes/struct.ListScriptProcessesRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*create*](resources/projects/struct.CreateRequestBuilder.html), [*get*](resources/projects/struct.GetRequestBuilder.html), [*getContent*](resources/projects/struct.GetContentRequestBuilder.html), [*getMetrics*](resources/projects/struct.GetMetricsRequestBuilder.html), [*updateContent*](resources/projects/struct.UpdateContentRequestBuilder.html)\n      * [deployments](resources/projects/deployments/struct.DeploymentsActions.html)\n        * [*create*](resources/projects/deployments/struct.CreateRequestBuilder.html), [*delete*](resources/projects/deployments/struct.DeleteRequestBuilder.html), [*get*](resources/projects/deployments/struct.GetRequestBuilder.html), [*list*](resources/projects/deployments/struct.ListRequestBuilder.html), [*update*](resources/projects/deployments/struct.UpdateRequestBuilder.html)\n      * [versions](resources/projects/versions/struct.VersionsActions.html)\n        * [*create*](resources/projects/versions/struct.CreateRequestBuilder.html), [*get*](resources/projects/versions/struct.GetRequestBuilder.html), [*list*](resources/projects/versions/struct.ListRequestBuilder.html)\n    * [scripts](resources/scripts/struct.ScriptsActions.html)\n      * [*run*](resources/scripts/struct.RunRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Read, compose, send, and permanently delete all your email from Gmail\n\n`https://mail.google.com/`"]
    pub const MAIL_GOOGLE_COM: &str = "https://mail.google.com/";
    #[doc = "See, edit, share, and permanently delete all the calendars you can access using Google Calendar\n\n`https://www.google.com/calendar/feeds`"]
    pub const WWW_GOOGLE_COM_CALENDAR_FEEDS: &str = "https://www.google.com/calendar/feeds";
    #[doc = "See, edit, download, and permanently delete your contacts\n\n`https://www.google.com/m8/feeds`"]
    pub const WWW_GOOGLE_COM_M8_FEEDS: &str = "https://www.google.com/m8/feeds";
    #[doc = "View and manage the provisioning of groups on your domain\n\n`https://www.googleapis.com/auth/admin.directory.group`"]
    pub const ADMIN_DIRECTORY_GROUP: &str = "https://www.googleapis.com/auth/admin.directory.group";
    #[doc = "View and manage the provisioning of users on your domain\n\n`https://www.googleapis.com/auth/admin.directory.user`"]
    pub const ADMIN_DIRECTORY_USER: &str = "https://www.googleapis.com/auth/admin.directory.user";
    #[doc = "View and manage your Google Docs documents\n\n`https://www.googleapis.com/auth/documents`"]
    pub const DOCUMENTS: &str = "https://www.googleapis.com/auth/documents";
    #[doc = "See, edit, create, and delete all of your Google Drive files\n\n`https://www.googleapis.com/auth/drive`"]
    pub const DRIVE: &str = "https://www.googleapis.com/auth/drive";
    #[doc = "View and manage your forms in Google Drive\n\n`https://www.googleapis.com/auth/forms`"]
    pub const FORMS: &str = "https://www.googleapis.com/auth/forms";
    #[doc = "View and manage forms that this application has been installed in\n\n`https://www.googleapis.com/auth/forms.currentonly`"]
    pub const FORMS_CURRENTONLY: &str = "https://www.googleapis.com/auth/forms.currentonly";
    #[doc = "View and manage your Google Groups\n\n`https://www.googleapis.com/auth/groups`"]
    pub const GROUPS: &str = "https://www.googleapis.com/auth/groups";
    #[doc = "Create and update Google Apps Script deployments\n\n`https://www.googleapis.com/auth/script.deployments`"]
    pub const SCRIPT_DEPLOYMENTS: &str = "https://www.googleapis.com/auth/script.deployments";
    #[doc = "View Google Apps Script deployments\n\n`https://www.googleapis.com/auth/script.deployments.readonly`"]
    pub const SCRIPT_DEPLOYMENTS_READONLY: &str =
        "https://www.googleapis.com/auth/script.deployments.readonly";
    #[doc = "View Google Apps Script project's metrics\n\n`https://www.googleapis.com/auth/script.metrics`"]
    pub const SCRIPT_METRICS: &str = "https://www.googleapis.com/auth/script.metrics";
    #[doc = "View Google Apps Script processes\n\n`https://www.googleapis.com/auth/script.processes`"]
    pub const SCRIPT_PROCESSES: &str = "https://www.googleapis.com/auth/script.processes";
    #[doc = "Create and update Google Apps Script projects\n\n`https://www.googleapis.com/auth/script.projects`"]
    pub const SCRIPT_PROJECTS: &str = "https://www.googleapis.com/auth/script.projects";
    #[doc = "View Google Apps Script projects\n\n`https://www.googleapis.com/auth/script.projects.readonly`"]
    pub const SCRIPT_PROJECTS_READONLY: &str =
        "https://www.googleapis.com/auth/script.projects.readonly";
    #[doc = "See, edit, create, and delete your spreadsheets in Google Drive\n\n`https://www.googleapis.com/auth/spreadsheets`"]
    pub const SPREADSHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
    #[doc = "View your email address\n\n`https://www.googleapis.com/auth/userinfo.email`"]
    pub const USERINFO_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";
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
    pub struct Content {
        #[doc = "The list of script project files.\nOne of the files is a script manifest; it must be named \"appsscript\",\nmust have type of JSON, and include the manifest configurations for the\nproject."]
        #[serde(
            rename = "files",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files: ::std::option::Option<Vec<crate::schemas::File>>,
        #[doc = "The script project's Drive ID."]
        #[serde(
            rename = "scriptId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Content {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Content {
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
    pub struct CreateProjectRequest {
        #[doc = "The Drive ID of a parent file that the created script project is bound to.\nThis is usually the ID of a Google Doc, Google Sheet, Google Form, or\nGoogle Slides file. If not set, a standalone script project is created."]
        #[serde(
            rename = "parentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_id: ::std::option::Option<String>,
        #[doc = "The title for the project."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateProjectRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateProjectRequest {
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
    pub struct Deployment {
        #[doc = "The deployment configuration."]
        #[serde(
            rename = "deploymentConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_config: ::std::option::Option<crate::schemas::DeploymentConfig>,
        #[doc = "The deployment ID for this deployment."]
        #[serde(
            rename = "deploymentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_id: ::std::option::Option<String>,
        #[doc = "The deployment's entry points."]
        #[serde(
            rename = "entryPoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_points: ::std::option::Option<Vec<crate::schemas::EntryPoint>>,
        #[doc = "Last modified date time stamp."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Deployment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Deployment {
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
    pub struct DeploymentConfig {
        #[doc = "The description for this deployment."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The manifest file name for this deployment."]
        #[serde(
            rename = "manifestFileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manifest_file_name: ::std::option::Option<String>,
        #[doc = "The script project's Drive ID."]
        #[serde(
            rename = "scriptId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_id: ::std::option::Option<String>,
        #[doc = "The version number on which this deployment is based."]
        #[serde(
            rename = "versionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_number: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeploymentConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentConfig {
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
    pub struct EntryPoint {
        #[doc = "Add-on properties."]
        #[serde(
            rename = "addOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_on: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeAddOnEntryPoint>,
        #[doc = "The type of the entry point."]
        #[serde(
            rename = "entryPointType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point_type: ::std::option::Option<crate::schemas::EntryPointEntryPointType>,
        #[doc = "An entry point specification for Apps Script API execution calls."]
        #[serde(
            rename = "executionApi",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execution_api:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeExecutionApiEntryPoint>,
        #[doc = "An entry point specification for web apps."]
        #[serde(
            rename = "webApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_app: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeWebAppEntryPoint>,
    }
    impl ::google_field_selector::FieldSelector for EntryPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EntryPoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EntryPointEntryPointType {
        #[doc = "An Add-On entry point."]
        AddOn,
        #[doc = "An unspecified entry point."]
        EntryPointTypeUnspecified,
        #[doc = "An API executable entry point."]
        ExecutionApi,
        #[doc = "A web application entry point."]
        WebApp,
    }
    impl EntryPointEntryPointType {
        pub fn as_str(self) -> &'static str {
            match self {
                EntryPointEntryPointType::AddOn => "ADD_ON",
                EntryPointEntryPointType::EntryPointTypeUnspecified => {
                    "ENTRY_POINT_TYPE_UNSPECIFIED"
                }
                EntryPointEntryPointType::ExecutionApi => "EXECUTION_API",
                EntryPointEntryPointType::WebApp => "WEB_APP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EntryPointEntryPointType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EntryPointEntryPointType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EntryPointEntryPointType, ()> {
            Ok(match s {
                "ADD_ON" => EntryPointEntryPointType::AddOn,
                "ENTRY_POINT_TYPE_UNSPECIFIED" => {
                    EntryPointEntryPointType::EntryPointTypeUnspecified
                }
                "EXECUTION_API" => EntryPointEntryPointType::ExecutionApi,
                "WEB_APP" => EntryPointEntryPointType::WebApp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EntryPointEntryPointType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EntryPointEntryPointType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EntryPointEntryPointType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD_ON" => EntryPointEntryPointType::AddOn,
                "ENTRY_POINT_TYPE_UNSPECIFIED" => {
                    EntryPointEntryPointType::EntryPointTypeUnspecified
                }
                "EXECUTION_API" => EntryPointEntryPointType::ExecutionApi,
                "WEB_APP" => EntryPointEntryPointType::WebApp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EntryPointEntryPointType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EntryPointEntryPointType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ExecuteStreamResponse {
        #[doc = "The result of the execution."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<crate::schemas::ScriptExecutionResult>,
    }
    impl ::google_field_selector::FieldSelector for ExecuteStreamResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecuteStreamResponse {
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
    pub struct ExecutionError {
        #[doc = "The error message thrown by Apps Script, usually localized into the user's\nlanguage."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "The error type, for example `TypeError` or `ReferenceError`. If the error\ntype is unavailable, this field is not included."]
        #[serde(
            rename = "errorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_type: ::std::option::Option<String>,
        #[doc = "An array of objects that provide a stack trace through the script to show\nwhere the execution failed, with the deepest call first."]
        #[serde(
            rename = "scriptStackTraceElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_stack_trace_elements:
            ::std::option::Option<Vec<crate::schemas::ScriptStackTraceElement>>,
    }
    impl ::google_field_selector::FieldSelector for ExecutionError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecutionError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ExecutionRequest {
        #[doc = "If `true` and the user is an owner of the script, the script runs at the\nmost recently saved version rather than the version deployed for use with\nthe Apps Script API. Optional; default is `false`."]
        #[serde(
            rename = "devMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dev_mode: ::std::option::Option<bool>,
        #[doc = "The name of the function to execute in the given script. The name does not\ninclude parentheses or parameters. It can reference a function in an\nincluded library such as `Library.libFunction1`."]
        #[serde(
            rename = "function",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function: ::std::option::Option<String>,
        #[doc = "The parameters to be passed to the function being executed. The object type\nfor each parameter should match the expected type in Apps Script.\nParameters cannot be Apps Script-specific object types (such as a\n`Document` or a `Calendar`); they can only be primitive types such as\n`string`, `number`, `array`, `object`, or `boolean`. Optional."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<::serde_json::Value>>,
        #[doc = "<b>Deprecated</b>. For use with Android add-ons only. An ID that represents\nthe user's current session in the Android app for Google Docs or Sheets,\nincluded as extra data in the\n[Intent](https://developer.android.com/guide/components/intents-filters.html)\nthat launches the add-on. When an Android add-on is run with a session\nstate, it gains the privileges of a\n[bound](https://developers.google.com/apps-script/guides/bound)\nscript—that is, it can access information like the user's current\ncursor position (in Docs) or selected cell (in Sheets). To retrieve the\nstate, call\n`Intent.getStringExtra(\"com.google.android.apps.docs.addons.SessionState\")`.\nOptional."]
        #[serde(
            rename = "sessionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExecutionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecutionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ExecutionResponse {
        #[doc = "The return value of the script function. The type matches the object type\nreturned in Apps Script. Functions called using the Apps Script API cannot\nreturn Apps Script-specific objects (such as a `Document` or a `Calendar`);\nthey can only return primitive types such as a `string`, `number`, `array`,\n`object`, or `boolean`."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<::serde_json::Value>,
    }
    impl ::google_field_selector::FieldSelector for ExecutionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecutionResponse {
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
    pub struct File {
        #[doc = "Creation date timestamp.\nThis read-only field is only visible to users who have WRITER\npermission for the script project."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The defined set of functions in the script file, if any."]
        #[serde(
            rename = "functionSet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function_set: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeFunctionSet>,
        #[doc = "The user who modified the file most recently.\nThis read-only field is only visible to users who have WRITER\npermission for the script project."]
        #[serde(
            rename = "lastModifyUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_modify_user: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeUser>,
        #[doc = "The name of the file. The file extension is not part of the file\nname, which can be identified from the type field."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the file."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::FileType>,
        #[doc = "The file content."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "Last modified date timestamp.\nThis read-only field is only visible to users who have WRITER\npermission for the script project."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for File {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for File {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FileType {
        #[doc = "Undetermined file type; never actually used."]
        EnumTypeUnspecified,
        #[doc = "A file containing client-side HTML."]
        Html,
        #[doc = "A file in JSON format. This type is only used for the script\nproject's manifest. The manifest file content must match the\nstructure of a valid\n[ScriptManifest](/apps-script/concepts/manifests)"]
        Json,
        #[doc = "An Apps Script server-side code file."]
        ServerJs,
    }
    impl FileType {
        pub fn as_str(self) -> &'static str {
            match self {
                FileType::EnumTypeUnspecified => "ENUM_TYPE_UNSPECIFIED",
                FileType::Html => "HTML",
                FileType::Json => "JSON",
                FileType::ServerJs => "SERVER_JS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FileType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FileType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FileType, ()> {
            Ok(match s {
                "ENUM_TYPE_UNSPECIFIED" => FileType::EnumTypeUnspecified,
                "HTML" => FileType::Html,
                "JSON" => FileType::Json,
                "SERVER_JS" => FileType::ServerJs,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FileType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FileType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FileType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENUM_TYPE_UNSPECIFIED" => FileType::EnumTypeUnspecified,
                "HTML" => FileType::Html,
                "JSON" => FileType::Json,
                "SERVER_JS" => FileType::ServerJs,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FileType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileType {
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
    pub struct GoogleAppsScriptTypeAddOnEntryPoint {
        #[doc = "The add-on's required list of supported container types."]
        #[serde(
            rename = "addOnType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_on_type:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeAddOnEntryPointAddOnType>,
        #[doc = "The add-on's optional description."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The add-on's optional help URL."]
        #[serde(
            rename = "helpUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub help_url: ::std::option::Option<String>,
        #[doc = "The add-on's required post install tip URL."]
        #[serde(
            rename = "postInstallTipUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub post_install_tip_url: ::std::option::Option<String>,
        #[doc = "The add-on's optional report issue URL."]
        #[serde(
            rename = "reportIssueUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_issue_url: ::std::option::Option<String>,
        #[doc = "The add-on's required title."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeAddOnEntryPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeAddOnEntryPoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        #[doc = "Add-on type for Data Studio."]
        DataStudio,
        #[doc = "Add-on type for Gmail."]
        Gmail,
        #[doc = "Default value, unknown add-on type."]
        UnknownAddonType,
    }
    impl GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeAddOnEntryPointAddOnType::DataStudio => "DATA_STUDIO",
                GoogleAppsScriptTypeAddOnEntryPointAddOnType::Gmail => "GMAIL",
                GoogleAppsScriptTypeAddOnEntryPointAddOnType::UnknownAddonType => {
                    "UNKNOWN_ADDON_TYPE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsScriptTypeAddOnEntryPointAddOnType, ()> {
            Ok(match s {
                "DATA_STUDIO" => GoogleAppsScriptTypeAddOnEntryPointAddOnType::DataStudio,
                "GMAIL" => GoogleAppsScriptTypeAddOnEntryPointAddOnType::Gmail,
                "UNKNOWN_ADDON_TYPE" => {
                    GoogleAppsScriptTypeAddOnEntryPointAddOnType::UnknownAddonType
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATA_STUDIO" => GoogleAppsScriptTypeAddOnEntryPointAddOnType::DataStudio,
                "GMAIL" => GoogleAppsScriptTypeAddOnEntryPointAddOnType::Gmail,
                "UNKNOWN_ADDON_TYPE" => {
                    GoogleAppsScriptTypeAddOnEntryPointAddOnType::UnknownAddonType
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
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeAddOnEntryPointAddOnType {
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
    pub struct GoogleAppsScriptTypeExecutionApiConfig {
        #[doc = "Who has permission to run the API executable."]
        #[serde(
            rename = "access",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeExecutionApiConfigAccess>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeExecutionApiConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeExecutionApiConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeExecutionApiConfigAccess {
        #[doc = "Any logged in user can access the web app or executable."]
        Anyone,
        #[doc = "Any user, logged in or not, can access the web app or executable."]
        AnyoneAnonymous,
        #[doc = "Only users in the same domain as the user who deployed the web app or\nexecutable can access it."]
        Domain,
        #[doc = "Only the user who deployed the web app or executable can access it.\nNote that this is not necessarily the owner of the script project."]
        Myself,
        #[doc = "Default value, should not be used."]
        UnknownAccess,
    }
    impl GoogleAppsScriptTypeExecutionApiConfigAccess {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeExecutionApiConfigAccess::Anyone => "ANYONE",
                GoogleAppsScriptTypeExecutionApiConfigAccess::AnyoneAnonymous => "ANYONE_ANONYMOUS",
                GoogleAppsScriptTypeExecutionApiConfigAccess::Domain => "DOMAIN",
                GoogleAppsScriptTypeExecutionApiConfigAccess::Myself => "MYSELF",
                GoogleAppsScriptTypeExecutionApiConfigAccess::UnknownAccess => "UNKNOWN_ACCESS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeExecutionApiConfigAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeExecutionApiConfigAccess {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsScriptTypeExecutionApiConfigAccess, ()> {
            Ok(match s {
                "ANYONE" => GoogleAppsScriptTypeExecutionApiConfigAccess::Anyone,
                "ANYONE_ANONYMOUS" => GoogleAppsScriptTypeExecutionApiConfigAccess::AnyoneAnonymous,
                "DOMAIN" => GoogleAppsScriptTypeExecutionApiConfigAccess::Domain,
                "MYSELF" => GoogleAppsScriptTypeExecutionApiConfigAccess::Myself,
                "UNKNOWN_ACCESS" => GoogleAppsScriptTypeExecutionApiConfigAccess::UnknownAccess,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeExecutionApiConfigAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeExecutionApiConfigAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeExecutionApiConfigAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANYONE" => GoogleAppsScriptTypeExecutionApiConfigAccess::Anyone,
                "ANYONE_ANONYMOUS" => GoogleAppsScriptTypeExecutionApiConfigAccess::AnyoneAnonymous,
                "DOMAIN" => GoogleAppsScriptTypeExecutionApiConfigAccess::Domain,
                "MYSELF" => GoogleAppsScriptTypeExecutionApiConfigAccess::Myself,
                "UNKNOWN_ACCESS" => GoogleAppsScriptTypeExecutionApiConfigAccess::UnknownAccess,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeExecutionApiConfigAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeExecutionApiConfigAccess {
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
    pub struct GoogleAppsScriptTypeExecutionApiEntryPoint {
        #[doc = "The entry point's configuration."]
        #[serde(
            rename = "entryPointConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point_config:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeExecutionApiConfig>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeExecutionApiEntryPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeExecutionApiEntryPoint {
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
    pub struct GoogleAppsScriptTypeFunction {
        #[doc = "The function name in the script project."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeFunction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeFunction {
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
    pub struct GoogleAppsScriptTypeFunctionSet {
        #[doc = "A list of functions composing the set."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::GoogleAppsScriptTypeFunction>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeFunctionSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeFunctionSet {
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
    pub struct GoogleAppsScriptTypeProcess {
        #[doc = "Duration the execution spent executing."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "Name of the function the started the execution."]
        #[serde(
            rename = "functionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function_name: ::std::option::Option<String>,
        #[doc = "The executions status."]
        #[serde(
            rename = "processStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub process_status:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeProcessProcessStatus>,
        #[doc = "The executions type."]
        #[serde(
            rename = "processType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub process_type:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeProcessProcessType>,
        #[doc = "Name of the script being executed."]
        #[serde(
            rename = "projectName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_name: ::std::option::Option<String>,
        #[doc = "Time the execution started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The executing users access level to the script."]
        #[serde(
            rename = "userAccessLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_access_level:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeProcessUserAccessLevel>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeProcess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeProcess {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeProcessProcessStatus {
        #[doc = "The process was cancelled."]
        Canceled,
        #[doc = "The process has completed."]
        Completed,
        #[doc = "The process is delayed, waiting for quota."]
        Delayed,
        #[doc = "The process failed."]
        Failed,
        #[doc = "The process has paused."]
        Paused,
        #[doc = "Unspecified status."]
        ProcessStatusUnspecified,
        #[doc = "The process is currently running."]
        Running,
        #[doc = "The process timed out."]
        TimedOut,
        #[doc = "Process status unknown."]
        Unknown,
    }
    impl GoogleAppsScriptTypeProcessProcessStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeProcessProcessStatus::Canceled => "CANCELED",
                GoogleAppsScriptTypeProcessProcessStatus::Completed => "COMPLETED",
                GoogleAppsScriptTypeProcessProcessStatus::Delayed => "DELAYED",
                GoogleAppsScriptTypeProcessProcessStatus::Failed => "FAILED",
                GoogleAppsScriptTypeProcessProcessStatus::Paused => "PAUSED",
                GoogleAppsScriptTypeProcessProcessStatus::ProcessStatusUnspecified => {
                    "PROCESS_STATUS_UNSPECIFIED"
                }
                GoogleAppsScriptTypeProcessProcessStatus::Running => "RUNNING",
                GoogleAppsScriptTypeProcessProcessStatus::TimedOut => "TIMED_OUT",
                GoogleAppsScriptTypeProcessProcessStatus::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeProcessProcessStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeProcessProcessStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsScriptTypeProcessProcessStatus, ()> {
            Ok(match s {
                "CANCELED" => GoogleAppsScriptTypeProcessProcessStatus::Canceled,
                "COMPLETED" => GoogleAppsScriptTypeProcessProcessStatus::Completed,
                "DELAYED" => GoogleAppsScriptTypeProcessProcessStatus::Delayed,
                "FAILED" => GoogleAppsScriptTypeProcessProcessStatus::Failed,
                "PAUSED" => GoogleAppsScriptTypeProcessProcessStatus::Paused,
                "PROCESS_STATUS_UNSPECIFIED" => {
                    GoogleAppsScriptTypeProcessProcessStatus::ProcessStatusUnspecified
                }
                "RUNNING" => GoogleAppsScriptTypeProcessProcessStatus::Running,
                "TIMED_OUT" => GoogleAppsScriptTypeProcessProcessStatus::TimedOut,
                "UNKNOWN" => GoogleAppsScriptTypeProcessProcessStatus::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeProcessProcessStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeProcessProcessStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeProcessProcessStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELED" => GoogleAppsScriptTypeProcessProcessStatus::Canceled,
                "COMPLETED" => GoogleAppsScriptTypeProcessProcessStatus::Completed,
                "DELAYED" => GoogleAppsScriptTypeProcessProcessStatus::Delayed,
                "FAILED" => GoogleAppsScriptTypeProcessProcessStatus::Failed,
                "PAUSED" => GoogleAppsScriptTypeProcessProcessStatus::Paused,
                "PROCESS_STATUS_UNSPECIFIED" => {
                    GoogleAppsScriptTypeProcessProcessStatus::ProcessStatusUnspecified
                }
                "RUNNING" => GoogleAppsScriptTypeProcessProcessStatus::Running,
                "TIMED_OUT" => GoogleAppsScriptTypeProcessProcessStatus::TimedOut,
                "UNKNOWN" => GoogleAppsScriptTypeProcessProcessStatus::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeProcessProcessStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeProcessProcessStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeProcessProcessType {
        #[doc = "The process was started from an add-on entry point."]
        AddOn,
        #[doc = "The process was started as a task in a batch job."]
        BatchTask,
        #[doc = "The process was started using the Apps Script IDE."]
        Editor,
        #[doc = "The process was started using the Apps Script API."]
        ExecutionApi,
        #[doc = "The process was started from a G Suite menu item."]
        Menu,
        #[doc = "Unspecified type."]
        ProcessTypeUnspecified,
        #[doc = "The process was started from a G Suite simple trigger."]
        SimpleTrigger,
        #[doc = "The process was started from a time-based trigger."]
        TimeDriven,
        #[doc = "The process was started from an event-based trigger."]
        Trigger,
        #[doc = "The process was started from a web app entry point."]
        Webapp,
    }
    impl GoogleAppsScriptTypeProcessProcessType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeProcessProcessType::AddOn => "ADD_ON",
                GoogleAppsScriptTypeProcessProcessType::BatchTask => "BATCH_TASK",
                GoogleAppsScriptTypeProcessProcessType::Editor => "EDITOR",
                GoogleAppsScriptTypeProcessProcessType::ExecutionApi => "EXECUTION_API",
                GoogleAppsScriptTypeProcessProcessType::Menu => "MENU",
                GoogleAppsScriptTypeProcessProcessType::ProcessTypeUnspecified => {
                    "PROCESS_TYPE_UNSPECIFIED"
                }
                GoogleAppsScriptTypeProcessProcessType::SimpleTrigger => "SIMPLE_TRIGGER",
                GoogleAppsScriptTypeProcessProcessType::TimeDriven => "TIME_DRIVEN",
                GoogleAppsScriptTypeProcessProcessType::Trigger => "TRIGGER",
                GoogleAppsScriptTypeProcessProcessType::Webapp => "WEBAPP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeProcessProcessType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeProcessProcessType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsScriptTypeProcessProcessType, ()> {
            Ok(match s {
                "ADD_ON" => GoogleAppsScriptTypeProcessProcessType::AddOn,
                "BATCH_TASK" => GoogleAppsScriptTypeProcessProcessType::BatchTask,
                "EDITOR" => GoogleAppsScriptTypeProcessProcessType::Editor,
                "EXECUTION_API" => GoogleAppsScriptTypeProcessProcessType::ExecutionApi,
                "MENU" => GoogleAppsScriptTypeProcessProcessType::Menu,
                "PROCESS_TYPE_UNSPECIFIED" => {
                    GoogleAppsScriptTypeProcessProcessType::ProcessTypeUnspecified
                }
                "SIMPLE_TRIGGER" => GoogleAppsScriptTypeProcessProcessType::SimpleTrigger,
                "TIME_DRIVEN" => GoogleAppsScriptTypeProcessProcessType::TimeDriven,
                "TRIGGER" => GoogleAppsScriptTypeProcessProcessType::Trigger,
                "WEBAPP" => GoogleAppsScriptTypeProcessProcessType::Webapp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeProcessProcessType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeProcessProcessType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeProcessProcessType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD_ON" => GoogleAppsScriptTypeProcessProcessType::AddOn,
                "BATCH_TASK" => GoogleAppsScriptTypeProcessProcessType::BatchTask,
                "EDITOR" => GoogleAppsScriptTypeProcessProcessType::Editor,
                "EXECUTION_API" => GoogleAppsScriptTypeProcessProcessType::ExecutionApi,
                "MENU" => GoogleAppsScriptTypeProcessProcessType::Menu,
                "PROCESS_TYPE_UNSPECIFIED" => {
                    GoogleAppsScriptTypeProcessProcessType::ProcessTypeUnspecified
                }
                "SIMPLE_TRIGGER" => GoogleAppsScriptTypeProcessProcessType::SimpleTrigger,
                "TIME_DRIVEN" => GoogleAppsScriptTypeProcessProcessType::TimeDriven,
                "TRIGGER" => GoogleAppsScriptTypeProcessProcessType::Trigger,
                "WEBAPP" => GoogleAppsScriptTypeProcessProcessType::Webapp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeProcessProcessType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeProcessProcessType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeProcessUserAccessLevel {
        #[doc = "The user has no access."]
        None,
        #[doc = "The user is an owner."]
        Owner,
        #[doc = "The user has read-only access."]
        Read,
        #[doc = "User access level unspecified"]
        UserAccessLevelUnspecified,
        #[doc = "The user has write access."]
        Write,
    }
    impl GoogleAppsScriptTypeProcessUserAccessLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeProcessUserAccessLevel::None => "NONE",
                GoogleAppsScriptTypeProcessUserAccessLevel::Owner => "OWNER",
                GoogleAppsScriptTypeProcessUserAccessLevel::Read => "READ",
                GoogleAppsScriptTypeProcessUserAccessLevel::UserAccessLevelUnspecified => {
                    "USER_ACCESS_LEVEL_UNSPECIFIED"
                }
                GoogleAppsScriptTypeProcessUserAccessLevel::Write => "WRITE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeProcessUserAccessLevel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeProcessUserAccessLevel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsScriptTypeProcessUserAccessLevel, ()> {
            Ok(match s {
                "NONE" => GoogleAppsScriptTypeProcessUserAccessLevel::None,
                "OWNER" => GoogleAppsScriptTypeProcessUserAccessLevel::Owner,
                "READ" => GoogleAppsScriptTypeProcessUserAccessLevel::Read,
                "USER_ACCESS_LEVEL_UNSPECIFIED" => {
                    GoogleAppsScriptTypeProcessUserAccessLevel::UserAccessLevelUnspecified
                }
                "WRITE" => GoogleAppsScriptTypeProcessUserAccessLevel::Write,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeProcessUserAccessLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeProcessUserAccessLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeProcessUserAccessLevel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => GoogleAppsScriptTypeProcessUserAccessLevel::None,
                "OWNER" => GoogleAppsScriptTypeProcessUserAccessLevel::Owner,
                "READ" => GoogleAppsScriptTypeProcessUserAccessLevel::Read,
                "USER_ACCESS_LEVEL_UNSPECIFIED" => {
                    GoogleAppsScriptTypeProcessUserAccessLevel::UserAccessLevelUnspecified
                }
                "WRITE" => GoogleAppsScriptTypeProcessUserAccessLevel::Write,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeProcessUserAccessLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeProcessUserAccessLevel {
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
    pub struct GoogleAppsScriptTypeUser {
        #[doc = "The user's domain."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "The user's identifying email address."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The user's display name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The user's photo."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeUser {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeUser {
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
    pub struct GoogleAppsScriptTypeWebAppConfig {
        #[doc = "Who has permission to run the web app."]
        #[serde(
            rename = "access",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeWebAppConfigAccess>,
        #[doc = "Who to execute the web app as."]
        #[serde(
            rename = "executeAs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execute_as:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeWebAppConfigExecuteAs>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeWebAppConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeWebAppConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeWebAppConfigAccess {
        #[doc = "Any logged in user can access the web app or executable."]
        Anyone,
        #[doc = "Any user, logged in or not, can access the web app or executable."]
        AnyoneAnonymous,
        #[doc = "Only users in the same domain as the user who deployed the web app or\nexecutable can access it."]
        Domain,
        #[doc = "Only the user who deployed the web app or executable can access it.\nNote that this is not necessarily the owner of the script project."]
        Myself,
        #[doc = "Default value, should not be used."]
        UnknownAccess,
    }
    impl GoogleAppsScriptTypeWebAppConfigAccess {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeWebAppConfigAccess::Anyone => "ANYONE",
                GoogleAppsScriptTypeWebAppConfigAccess::AnyoneAnonymous => "ANYONE_ANONYMOUS",
                GoogleAppsScriptTypeWebAppConfigAccess::Domain => "DOMAIN",
                GoogleAppsScriptTypeWebAppConfigAccess::Myself => "MYSELF",
                GoogleAppsScriptTypeWebAppConfigAccess::UnknownAccess => "UNKNOWN_ACCESS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeWebAppConfigAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeWebAppConfigAccess {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsScriptTypeWebAppConfigAccess, ()> {
            Ok(match s {
                "ANYONE" => GoogleAppsScriptTypeWebAppConfigAccess::Anyone,
                "ANYONE_ANONYMOUS" => GoogleAppsScriptTypeWebAppConfigAccess::AnyoneAnonymous,
                "DOMAIN" => GoogleAppsScriptTypeWebAppConfigAccess::Domain,
                "MYSELF" => GoogleAppsScriptTypeWebAppConfigAccess::Myself,
                "UNKNOWN_ACCESS" => GoogleAppsScriptTypeWebAppConfigAccess::UnknownAccess,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeWebAppConfigAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeWebAppConfigAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeWebAppConfigAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANYONE" => GoogleAppsScriptTypeWebAppConfigAccess::Anyone,
                "ANYONE_ANONYMOUS" => GoogleAppsScriptTypeWebAppConfigAccess::AnyoneAnonymous,
                "DOMAIN" => GoogleAppsScriptTypeWebAppConfigAccess::Domain,
                "MYSELF" => GoogleAppsScriptTypeWebAppConfigAccess::Myself,
                "UNKNOWN_ACCESS" => GoogleAppsScriptTypeWebAppConfigAccess::UnknownAccess,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeWebAppConfigAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeWebAppConfigAccess {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsScriptTypeWebAppConfigExecuteAs {
        #[doc = "Default value, should not be used."]
        UnknownExecuteAs,
        #[doc = "The script runs as the user accessing the web app."]
        UserAccessing,
        #[doc = "The script runs as the user who deployed the web app. Note that this is\nnot necessarily the owner of the script project."]
        UserDeploying,
    }
    impl GoogleAppsScriptTypeWebAppConfigExecuteAs {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsScriptTypeWebAppConfigExecuteAs::UnknownExecuteAs => "UNKNOWN_EXECUTE_AS",
                GoogleAppsScriptTypeWebAppConfigExecuteAs::UserAccessing => "USER_ACCESSING",
                GoogleAppsScriptTypeWebAppConfigExecuteAs::UserDeploying => "USER_DEPLOYING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsScriptTypeWebAppConfigExecuteAs {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsScriptTypeWebAppConfigExecuteAs {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsScriptTypeWebAppConfigExecuteAs, ()> {
            Ok(match s {
                "UNKNOWN_EXECUTE_AS" => GoogleAppsScriptTypeWebAppConfigExecuteAs::UnknownExecuteAs,
                "USER_ACCESSING" => GoogleAppsScriptTypeWebAppConfigExecuteAs::UserAccessing,
                "USER_DEPLOYING" => GoogleAppsScriptTypeWebAppConfigExecuteAs::UserDeploying,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsScriptTypeWebAppConfigExecuteAs {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsScriptTypeWebAppConfigExecuteAs {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsScriptTypeWebAppConfigExecuteAs {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_EXECUTE_AS" => GoogleAppsScriptTypeWebAppConfigExecuteAs::UnknownExecuteAs,
                "USER_ACCESSING" => GoogleAppsScriptTypeWebAppConfigExecuteAs::UserAccessing,
                "USER_DEPLOYING" => GoogleAppsScriptTypeWebAppConfigExecuteAs::UserDeploying,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeWebAppConfigExecuteAs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeWebAppConfigExecuteAs {
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
    pub struct GoogleAppsScriptTypeWebAppEntryPoint {
        #[doc = "The entry point's configuration."]
        #[serde(
            rename = "entryPointConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point_config:
            ::std::option::Option<crate::schemas::GoogleAppsScriptTypeWebAppConfig>,
        #[doc = "The URL for the web application."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsScriptTypeWebAppEntryPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsScriptTypeWebAppEntryPoint {
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
    pub struct ListDeploymentsResponse {
        #[doc = "The list of deployments."]
        #[serde(
            rename = "deployments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployments: ::std::option::Option<Vec<crate::schemas::Deployment>>,
        #[doc = "The token that can be used in the next call to get the next page of\nresults."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListDeploymentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDeploymentsResponse {
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
    pub struct ListScriptProcessesResponse {
        #[doc = "Token for the next page of results. If empty, there are no more pages\nremaining."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of processes matching request parameters."]
        #[serde(
            rename = "processes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processes: ::std::option::Option<Vec<crate::schemas::GoogleAppsScriptTypeProcess>>,
    }
    impl ::google_field_selector::FieldSelector for ListScriptProcessesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListScriptProcessesResponse {
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
    pub struct ListUserProcessesResponse {
        #[doc = "Token for the next page of results. If empty, there are no more pages\nremaining."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of processes matching request parameters."]
        #[serde(
            rename = "processes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processes: ::std::option::Option<Vec<crate::schemas::GoogleAppsScriptTypeProcess>>,
    }
    impl ::google_field_selector::FieldSelector for ListUserProcessesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUserProcessesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListValue {
        #[doc = "Repeated field of dynamically typed values."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::Value>>,
    }
    impl ::google_field_selector::FieldSelector for ListValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListValue {
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
    pub struct ListVersionsResponse {
        #[doc = "The token use to fetch the next page of records. if not exist in the\nresponse, that means no more versions to list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of versions."]
        #[serde(
            rename = "versions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versions: ::std::option::Option<Vec<crate::schemas::Version>>,
    }
    impl ::google_field_selector::FieldSelector for ListVersionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListVersionsResponse {
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
    pub struct Metrics {
        #[doc = "Number of active users."]
        #[serde(
            rename = "activeUsers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub active_users: ::std::option::Option<Vec<crate::schemas::MetricsValue>>,
        #[doc = "Number of failed executions."]
        #[serde(
            rename = "failedExecutions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failed_executions: ::std::option::Option<Vec<crate::schemas::MetricsValue>>,
        #[doc = "Number of total executions."]
        #[serde(
            rename = "totalExecutions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_executions: ::std::option::Option<Vec<crate::schemas::MetricsValue>>,
    }
    impl ::google_field_selector::FieldSelector for Metrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metrics {
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
    pub struct MetricsValue {
        #[doc = "Required field indicating the end time of the interval."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Required field indicating the start time of the interval."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Indicates the number of executions counted."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for MetricsValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricsValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "This field indicates whether the script execution has completed. A completed execution has a populated `response` field containing the ExecutionResponse from function that was executed."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, this field contains a Status object. The `Status` object's `details` field contains an array with a single ExecutionError object that provides information about the nature of the error."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If the script function returns successfully, this field contains an ExecutionResponse object with the function's return value."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
    pub struct Project {
        #[doc = "When the script was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "User who originally created the script."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeUser>,
        #[doc = "User who last modified the script."]
        #[serde(
            rename = "lastModifyUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_modify_user: ::std::option::Option<crate::schemas::GoogleAppsScriptTypeUser>,
        #[doc = "The parent's Drive ID that the script will be attached to. This is usually\nthe ID of a Google Document or Google Sheet. This filed is optional, and\nif not set, a stand-alone script will be created."]
        #[serde(
            rename = "parentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_id: ::std::option::Option<String>,
        #[doc = "The script project's Drive ID."]
        #[serde(
            rename = "scriptId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_id: ::std::option::Option<String>,
        #[doc = "The title for the project."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "When the script was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Project {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Project {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ScriptExecutionResult {
        #[doc = "The returned value of the execution."]
        #[serde(
            rename = "returnValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for ScriptExecutionResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScriptExecutionResult {
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
    pub struct ScriptStackTraceElement {
        #[doc = "The name of the function that failed."]
        #[serde(
            rename = "function",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function: ::std::option::Option<String>,
        #[doc = "The line number where the script failed."]
        #[serde(
            rename = "lineNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_number: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ScriptStackTraceElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScriptStackTraceElement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code. For this API, this value either: <ul> <li> 10, indicating a `SCRIPT_TIMEOUT` error,</li> <li> 3, indicating an `INVALID_ARGUMENT` error, or</li> <li> 1, indicating a `CANCELLED` execution.</li> </ul>"]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "An array that contains a single ExecutionError object that provides information about the nature of the error."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which is in English. Any user-facing error message is localized and sent in the details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Struct {
        #[doc = "Unordered map of dynamically typed values."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Struct {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Struct {
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
    pub struct UpdateDeploymentRequest {
        #[doc = "The deployment configuration."]
        #[serde(
            rename = "deploymentConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_config: ::std::option::Option<crate::schemas::DeploymentConfig>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDeploymentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDeploymentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Value {
        #[doc = "Represents a boolean value."]
        #[serde(
            rename = "boolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "Represents raw byte values."]
        #[serde(
            rename = "bytesValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bytes_value: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Represents a date in ms since the epoch."]
        #[serde(
            rename = "dateValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub date_value: ::std::option::Option<i64>,
        #[doc = "Represents a repeated `Value`."]
        #[serde(
            rename = "listValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_value: ::std::option::Option<crate::schemas::ListValue>,
        #[doc = "Represents a null value."]
        #[serde(
            rename = "nullValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub null_value: ::std::option::Option<crate::schemas::ValueNullValue>,
        #[doc = "Represents a double value."]
        #[serde(
            rename = "numberValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_value: ::std::option::Option<f64>,
        #[doc = "Represents a structured proto value."]
        #[serde(
            rename = "protoValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proto_value:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Represents a string value."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
        #[doc = "Represents a structured value."]
        #[serde(
            rename = "structValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub struct_value: ::std::option::Option<crate::schemas::Struct>,
    }
    impl ::google_field_selector::FieldSelector for Value {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Value {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ValueNullValue {
        #[doc = "Null value."]
        NullValue,
    }
    impl ValueNullValue {
        pub fn as_str(self) -> &'static str {
            match self {
                ValueNullValue::NullValue => "NULL_VALUE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ValueNullValue {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ValueNullValue {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ValueNullValue, ()> {
            Ok(match s {
                "NULL_VALUE" => ValueNullValue::NullValue,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ValueNullValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ValueNullValue {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ValueNullValue {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NULL_VALUE" => ValueNullValue::NullValue,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ValueNullValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueNullValue {
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
    pub struct Version {
        #[doc = "When the version was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The description for this version."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The script project's Drive ID."]
        #[serde(
            rename = "scriptId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_id: ::std::option::Option<String>,
        #[doc = "The incremental ID that is created by Apps Script when a version is\ncreated. This is system assigned number and is immutable once created."]
        #[serde(
            rename = "versionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_number: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Version {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Version {
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
        A: ::google_api_auth::GetAccessToken + 'static,
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
    #[doc = "Actions that can be performed on the processes resource"]
    pub fn processes(&self) -> crate::resources::processes::ProcessesActions {
        crate::resources::processes::ProcessesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the scripts resource"]
    pub fn scripts(&self) -> crate::resources::scripts::ScriptsActions {
        crate::resources::scripts::ScriptsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod processes {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListUserProcessFilterStatusesItems {
                Canceled,
                Completed,
                Delayed,
                Failed,
                Paused,
                ProcessStatusUnspecified,
                Running,
                TimedOut,
                Unknown,
            }
            impl ListUserProcessFilterStatusesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListUserProcessFilterStatusesItems::Canceled => "CANCELED",
                        ListUserProcessFilterStatusesItems::Completed => "COMPLETED",
                        ListUserProcessFilterStatusesItems::Delayed => "DELAYED",
                        ListUserProcessFilterStatusesItems::Failed => "FAILED",
                        ListUserProcessFilterStatusesItems::Paused => "PAUSED",
                        ListUserProcessFilterStatusesItems::ProcessStatusUnspecified => {
                            "PROCESS_STATUS_UNSPECIFIED"
                        }
                        ListUserProcessFilterStatusesItems::Running => "RUNNING",
                        ListUserProcessFilterStatusesItems::TimedOut => "TIMED_OUT",
                        ListUserProcessFilterStatusesItems::Unknown => "UNKNOWN",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListUserProcessFilterStatusesItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListUserProcessFilterStatusesItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<ListUserProcessFilterStatusesItems, ()> {
                    Ok(match s {
                        "CANCELED" => ListUserProcessFilterStatusesItems::Canceled,
                        "COMPLETED" => ListUserProcessFilterStatusesItems::Completed,
                        "DELAYED" => ListUserProcessFilterStatusesItems::Delayed,
                        "FAILED" => ListUserProcessFilterStatusesItems::Failed,
                        "PAUSED" => ListUserProcessFilterStatusesItems::Paused,
                        "PROCESS_STATUS_UNSPECIFIED" => {
                            ListUserProcessFilterStatusesItems::ProcessStatusUnspecified
                        }
                        "RUNNING" => ListUserProcessFilterStatusesItems::Running,
                        "TIMED_OUT" => ListUserProcessFilterStatusesItems::TimedOut,
                        "UNKNOWN" => ListUserProcessFilterStatusesItems::Unknown,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListUserProcessFilterStatusesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListUserProcessFilterStatusesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListUserProcessFilterStatusesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "CANCELED" => ListUserProcessFilterStatusesItems::Canceled,
                        "COMPLETED" => ListUserProcessFilterStatusesItems::Completed,
                        "DELAYED" => ListUserProcessFilterStatusesItems::Delayed,
                        "FAILED" => ListUserProcessFilterStatusesItems::Failed,
                        "PAUSED" => ListUserProcessFilterStatusesItems::Paused,
                        "PROCESS_STATUS_UNSPECIFIED" => {
                            ListUserProcessFilterStatusesItems::ProcessStatusUnspecified
                        }
                        "RUNNING" => ListUserProcessFilterStatusesItems::Running,
                        "TIMED_OUT" => ListUserProcessFilterStatusesItems::TimedOut,
                        "UNKNOWN" => ListUserProcessFilterStatusesItems::Unknown,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListUserProcessFilterStatusesItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListUserProcessFilterStatusesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListUserProcessFilterTypesItems {
                AddOn,
                BatchTask,
                Editor,
                ExecutionApi,
                Menu,
                ProcessTypeUnspecified,
                SimpleTrigger,
                TimeDriven,
                Trigger,
                Webapp,
            }
            impl ListUserProcessFilterTypesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListUserProcessFilterTypesItems::AddOn => "ADD_ON",
                        ListUserProcessFilterTypesItems::BatchTask => "BATCH_TASK",
                        ListUserProcessFilterTypesItems::Editor => "EDITOR",
                        ListUserProcessFilterTypesItems::ExecutionApi => "EXECUTION_API",
                        ListUserProcessFilterTypesItems::Menu => "MENU",
                        ListUserProcessFilterTypesItems::ProcessTypeUnspecified => {
                            "PROCESS_TYPE_UNSPECIFIED"
                        }
                        ListUserProcessFilterTypesItems::SimpleTrigger => "SIMPLE_TRIGGER",
                        ListUserProcessFilterTypesItems::TimeDriven => "TIME_DRIVEN",
                        ListUserProcessFilterTypesItems::Trigger => "TRIGGER",
                        ListUserProcessFilterTypesItems::Webapp => "WEBAPP",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListUserProcessFilterTypesItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListUserProcessFilterTypesItems {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListUserProcessFilterTypesItems, ()> {
                    Ok(match s {
                        "ADD_ON" => ListUserProcessFilterTypesItems::AddOn,
                        "BATCH_TASK" => ListUserProcessFilterTypesItems::BatchTask,
                        "EDITOR" => ListUserProcessFilterTypesItems::Editor,
                        "EXECUTION_API" => ListUserProcessFilterTypesItems::ExecutionApi,
                        "MENU" => ListUserProcessFilterTypesItems::Menu,
                        "PROCESS_TYPE_UNSPECIFIED" => {
                            ListUserProcessFilterTypesItems::ProcessTypeUnspecified
                        }
                        "SIMPLE_TRIGGER" => ListUserProcessFilterTypesItems::SimpleTrigger,
                        "TIME_DRIVEN" => ListUserProcessFilterTypesItems::TimeDriven,
                        "TRIGGER" => ListUserProcessFilterTypesItems::Trigger,
                        "WEBAPP" => ListUserProcessFilterTypesItems::Webapp,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListUserProcessFilterTypesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListUserProcessFilterTypesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListUserProcessFilterTypesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ADD_ON" => ListUserProcessFilterTypesItems::AddOn,
                        "BATCH_TASK" => ListUserProcessFilterTypesItems::BatchTask,
                        "EDITOR" => ListUserProcessFilterTypesItems::Editor,
                        "EXECUTION_API" => ListUserProcessFilterTypesItems::ExecutionApi,
                        "MENU" => ListUserProcessFilterTypesItems::Menu,
                        "PROCESS_TYPE_UNSPECIFIED" => {
                            ListUserProcessFilterTypesItems::ProcessTypeUnspecified
                        }
                        "SIMPLE_TRIGGER" => ListUserProcessFilterTypesItems::SimpleTrigger,
                        "TIME_DRIVEN" => ListUserProcessFilterTypesItems::TimeDriven,
                        "TRIGGER" => ListUserProcessFilterTypesItems::Trigger,
                        "WEBAPP" => ListUserProcessFilterTypesItems::Webapp,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListUserProcessFilterTypesItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListUserProcessFilterTypesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListUserProcessFilterUserAccessLevelsItems {
                None,
                Owner,
                Read,
                UserAccessLevelUnspecified,
                Write,
            }
            impl ListUserProcessFilterUserAccessLevelsItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListUserProcessFilterUserAccessLevelsItems::None => "NONE",
                        ListUserProcessFilterUserAccessLevelsItems::Owner => "OWNER",
                        ListUserProcessFilterUserAccessLevelsItems::Read => "READ",
                        ListUserProcessFilterUserAccessLevelsItems::UserAccessLevelUnspecified => {
                            "USER_ACCESS_LEVEL_UNSPECIFIED"
                        }
                        ListUserProcessFilterUserAccessLevelsItems::Write => "WRITE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListUserProcessFilterUserAccessLevelsItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListUserProcessFilterUserAccessLevelsItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<ListUserProcessFilterUserAccessLevelsItems, ()>
                {
                    Ok(match s {
                        "NONE" => ListUserProcessFilterUserAccessLevelsItems::None,
                        "OWNER" => ListUserProcessFilterUserAccessLevelsItems::Owner,
                        "READ" => ListUserProcessFilterUserAccessLevelsItems::Read,
                        "USER_ACCESS_LEVEL_UNSPECIFIED" => {
                            ListUserProcessFilterUserAccessLevelsItems::UserAccessLevelUnspecified
                        }
                        "WRITE" => ListUserProcessFilterUserAccessLevelsItems::Write,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListUserProcessFilterUserAccessLevelsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListUserProcessFilterUserAccessLevelsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListUserProcessFilterUserAccessLevelsItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "NONE" => ListUserProcessFilterUserAccessLevelsItems::None,
                        "OWNER" => ListUserProcessFilterUserAccessLevelsItems::Owner,
                        "READ" => ListUserProcessFilterUserAccessLevelsItems::Read,
                        "USER_ACCESS_LEVEL_UNSPECIFIED" => {
                            ListUserProcessFilterUserAccessLevelsItems::UserAccessLevelUnspecified
                        }
                        "WRITE" => ListUserProcessFilterUserAccessLevelsItems::Write,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListUserProcessFilterUserAccessLevelsItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListUserProcessFilterUserAccessLevelsItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListScriptProcessesScriptProcessFilterStatusesItems {
                Canceled,
                Completed,
                Delayed,
                Failed,
                Paused,
                ProcessStatusUnspecified,
                Running,
                TimedOut,
                Unknown,
            }
            impl ListScriptProcessesScriptProcessFilterStatusesItems {
                pub fn as_str(self) -> &'static str {
                    match self { ListScriptProcessesScriptProcessFilterStatusesItems :: Canceled => "CANCELED" , ListScriptProcessesScriptProcessFilterStatusesItems :: Completed => "COMPLETED" , ListScriptProcessesScriptProcessFilterStatusesItems :: Delayed => "DELAYED" , ListScriptProcessesScriptProcessFilterStatusesItems :: Failed => "FAILED" , ListScriptProcessesScriptProcessFilterStatusesItems :: Paused => "PAUSED" , ListScriptProcessesScriptProcessFilterStatusesItems :: ProcessStatusUnspecified => "PROCESS_STATUS_UNSPECIFIED" , ListScriptProcessesScriptProcessFilterStatusesItems :: Running => "RUNNING" , ListScriptProcessesScriptProcessFilterStatusesItems :: TimedOut => "TIMED_OUT" , ListScriptProcessesScriptProcessFilterStatusesItems :: Unknown => "UNKNOWN" , }
                }
            }
            impl ::std::convert::AsRef<str> for ListScriptProcessesScriptProcessFilterStatusesItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListScriptProcessesScriptProcessFilterStatusesItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<ListScriptProcessesScriptProcessFilterStatusesItems, ()>
                {
                    Ok ( match s { "CANCELED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Canceled , "COMPLETED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Completed , "DELAYED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Delayed , "FAILED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Failed , "PAUSED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Paused , "PROCESS_STATUS_UNSPECIFIED" => ListScriptProcessesScriptProcessFilterStatusesItems :: ProcessStatusUnspecified , "RUNNING" => ListScriptProcessesScriptProcessFilterStatusesItems :: Running , "TIMED_OUT" => ListScriptProcessesScriptProcessFilterStatusesItems :: TimedOut , "UNKNOWN" => ListScriptProcessesScriptProcessFilterStatusesItems :: Unknown , _ => return Err ( ( ) ) , } )
                }
            }
            impl ::std::fmt::Display for ListScriptProcessesScriptProcessFilterStatusesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListScriptProcessesScriptProcessFilterStatusesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListScriptProcessesScriptProcessFilterStatusesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok ( match value { "CANCELED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Canceled , "COMPLETED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Completed , "DELAYED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Delayed , "FAILED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Failed , "PAUSED" => ListScriptProcessesScriptProcessFilterStatusesItems :: Paused , "PROCESS_STATUS_UNSPECIFIED" => ListScriptProcessesScriptProcessFilterStatusesItems :: ProcessStatusUnspecified , "RUNNING" => ListScriptProcessesScriptProcessFilterStatusesItems :: Running , "TIMED_OUT" => ListScriptProcessesScriptProcessFilterStatusesItems :: TimedOut , "UNKNOWN" => ListScriptProcessesScriptProcessFilterStatusesItems :: Unknown , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
                }
            }
            impl ::google_field_selector::FieldSelector
                for ListScriptProcessesScriptProcessFilterStatusesItems
            {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListScriptProcessesScriptProcessFilterStatusesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListScriptProcessesScriptProcessFilterTypesItems {
                AddOn,
                BatchTask,
                Editor,
                ExecutionApi,
                Menu,
                ProcessTypeUnspecified,
                SimpleTrigger,
                TimeDriven,
                Trigger,
                Webapp,
            }
            impl ListScriptProcessesScriptProcessFilterTypesItems {
                pub fn as_str(self) -> &'static str {
                    match self { ListScriptProcessesScriptProcessFilterTypesItems :: AddOn => "ADD_ON" , ListScriptProcessesScriptProcessFilterTypesItems :: BatchTask => "BATCH_TASK" , ListScriptProcessesScriptProcessFilterTypesItems :: Editor => "EDITOR" , ListScriptProcessesScriptProcessFilterTypesItems :: ExecutionApi => "EXECUTION_API" , ListScriptProcessesScriptProcessFilterTypesItems :: Menu => "MENU" , ListScriptProcessesScriptProcessFilterTypesItems :: ProcessTypeUnspecified => "PROCESS_TYPE_UNSPECIFIED" , ListScriptProcessesScriptProcessFilterTypesItems :: SimpleTrigger => "SIMPLE_TRIGGER" , ListScriptProcessesScriptProcessFilterTypesItems :: TimeDriven => "TIME_DRIVEN" , ListScriptProcessesScriptProcessFilterTypesItems :: Trigger => "TRIGGER" , ListScriptProcessesScriptProcessFilterTypesItems :: Webapp => "WEBAPP" , }
                }
            }
            impl ::std::convert::AsRef<str> for ListScriptProcessesScriptProcessFilterTypesItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListScriptProcessesScriptProcessFilterTypesItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<ListScriptProcessesScriptProcessFilterTypesItems, ()>
                {
                    Ok(match s {
                        "ADD_ON" => ListScriptProcessesScriptProcessFilterTypesItems::AddOn,
                        "BATCH_TASK" => ListScriptProcessesScriptProcessFilterTypesItems::BatchTask,
                        "EDITOR" => ListScriptProcessesScriptProcessFilterTypesItems::Editor,
                        "EXECUTION_API" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::ExecutionApi
                        }
                        "MENU" => ListScriptProcessesScriptProcessFilterTypesItems::Menu,
                        "PROCESS_TYPE_UNSPECIFIED" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::ProcessTypeUnspecified
                        }
                        "SIMPLE_TRIGGER" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::SimpleTrigger
                        }
                        "TIME_DRIVEN" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::TimeDriven
                        }
                        "TRIGGER" => ListScriptProcessesScriptProcessFilterTypesItems::Trigger,
                        "WEBAPP" => ListScriptProcessesScriptProcessFilterTypesItems::Webapp,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListScriptProcessesScriptProcessFilterTypesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListScriptProcessesScriptProcessFilterTypesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListScriptProcessesScriptProcessFilterTypesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ADD_ON" => ListScriptProcessesScriptProcessFilterTypesItems::AddOn,
                        "BATCH_TASK" => ListScriptProcessesScriptProcessFilterTypesItems::BatchTask,
                        "EDITOR" => ListScriptProcessesScriptProcessFilterTypesItems::Editor,
                        "EXECUTION_API" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::ExecutionApi
                        }
                        "MENU" => ListScriptProcessesScriptProcessFilterTypesItems::Menu,
                        "PROCESS_TYPE_UNSPECIFIED" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::ProcessTypeUnspecified
                        }
                        "SIMPLE_TRIGGER" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::SimpleTrigger
                        }
                        "TIME_DRIVEN" => {
                            ListScriptProcessesScriptProcessFilterTypesItems::TimeDriven
                        }
                        "TRIGGER" => ListScriptProcessesScriptProcessFilterTypesItems::Trigger,
                        "WEBAPP" => ListScriptProcessesScriptProcessFilterTypesItems::Webapp,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListScriptProcessesScriptProcessFilterTypesItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListScriptProcessesScriptProcessFilterTypesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListScriptProcessesScriptProcessFilterUserAccessLevelsItems {
                None,
                Owner,
                Read,
                UserAccessLevelUnspecified,
                Write,
            }
            impl ListScriptProcessesScriptProcessFilterUserAccessLevelsItems {
                pub fn as_str(self) -> &'static str {
                    match self { ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: None => "NONE" , ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Owner => "OWNER" , ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Read => "READ" , ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: UserAccessLevelUnspecified => "USER_ACCESS_LEVEL_UNSPECIFIED" , ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Write => "WRITE" , }
                }
            }
            impl ::std::convert::AsRef<str> for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<
                    ListScriptProcessesScriptProcessFilterUserAccessLevelsItems,
                    (),
                > {
                    Ok ( match s { "NONE" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: None , "OWNER" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Owner , "READ" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Read , "USER_ACCESS_LEVEL_UNSPECIFIED" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: UserAccessLevelUnspecified , "WRITE" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Write , _ => return Err ( ( ) ) , } )
                }
            }
            impl ::std::fmt::Display for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de>
                for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems
            {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok ( match value { "NONE" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: None , "OWNER" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Owner , "READ" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Read , "USER_ACCESS_LEVEL_UNSPECIFIED" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: UserAccessLevelUnspecified , "WRITE" => ListScriptProcessesScriptProcessFilterUserAccessLevelsItems :: Write , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
                }
            }
            impl ::google_field_selector::FieldSelector
                for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems
            {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType
                for ListScriptProcessesScriptProcessFilterUserAccessLevelsItems
            {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ProcessesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProcessesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "List information about processes made by or on behalf of a user,\nsuch as process type and current status."]
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
                    page_size: None,
                    page_token: None,
                    user_process_filter_deployment_id: None,
                    user_process_filter_end_time: None,
                    user_process_filter_function_name: None,
                    user_process_filter_project_name: None,
                    user_process_filter_script_id: None,
                    user_process_filter_start_time: None,
                    user_process_filter_statuses: None,
                    user_process_filter_types: None,
                    user_process_filter_user_access_levels: None,
                }
            }
            #[doc = "List information about a script's executed processes, such as process type\nand current status."]
            pub fn list_script_processes(&self) -> ListScriptProcessesRequestBuilder {
                ListScriptProcessesRequestBuilder {
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
                    page_size: None,
                    page_token: None,
                    script_id: None,
                    script_process_filter_deployment_id: None,
                    script_process_filter_end_time: None,
                    script_process_filter_function_name: None,
                    script_process_filter_start_time: None,
                    script_process_filter_statuses: None,
                    script_process_filter_types: None,
                    script_process_filter_user_access_levels: None,
                }
            }
        }
        #[doc = "Created via [ProcessesActions::list()](struct.ProcessesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            page_size: Option<i32>,
            page_token: Option<String>,
            user_process_filter_deployment_id: Option<String>,
            user_process_filter_end_time: Option<String>,
            user_process_filter_function_name: Option<String>,
            user_process_filter_project_name: Option<String>,
            user_process_filter_script_id: Option<String>,
            user_process_filter_start_time: Option<String>,
            user_process_filter_statuses: Option<
                Vec<crate::resources::processes::params::ListUserProcessFilterStatusesItems>,
            >,
            user_process_filter_types:
                Option<Vec<crate::resources::processes::params::ListUserProcessFilterTypesItems>>,
            user_process_filter_user_access_levels: Option<
                Vec<
                    crate::resources::processes::params::ListUserProcessFilterUserAccessLevelsItems,
                >,
            >,
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
            #[doc = "The maximum number of returned processes per page of results. Defaults to\n50."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The token for continuing a previous list request on the next page. This\nshould be set to the value of `nextPageToken` from a previous response."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those originating from\nprojects with a specific deployment ID."]
            pub fn user_process_filter_deployment_id(mut self, value: impl Into<String>) -> Self {
                self.user_process_filter_deployment_id = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those that completed\non or before the given timestamp."]
            pub fn user_process_filter_end_time(mut self, value: impl Into<String>) -> Self {
                self.user_process_filter_end_time = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those originating from\na script function with the given function name."]
            pub fn user_process_filter_function_name(mut self, value: impl Into<String>) -> Self {
                self.user_process_filter_function_name = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those originating from\nprojects with project names containing a specific string."]
            pub fn user_process_filter_project_name(mut self, value: impl Into<String>) -> Self {
                self.user_process_filter_project_name = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those originating from\nprojects with a specific script ID."]
            pub fn user_process_filter_script_id(mut self, value: impl Into<String>) -> Self {
                self.user_process_filter_script_id = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those that were\nstarted on or after the given timestamp."]
            pub fn user_process_filter_start_time(mut self, value: impl Into<String>) -> Self {
                self.user_process_filter_start_time = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those having one of\nthe specified process statuses."]
            pub fn user_process_filter_statuses(
                mut self,
                value: impl Into<
                    Vec<crate::resources::processes::params::ListUserProcessFilterStatusesItems>,
                >,
            ) -> Self {
                self.user_process_filter_statuses = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those having one of\nthe specified process types."]
            pub fn user_process_filter_types(
                mut self,
                value: impl Into<
                    Vec<crate::resources::processes::params::ListUserProcessFilterTypesItems>,
                >,
            ) -> Self {
                self.user_process_filter_types = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those having one of\nthe specified user access levels."]
            pub fn user_process_filter_user_access_levels(
                mut self,
                value : impl Into < Vec < crate :: resources :: processes :: params :: ListUserProcessFilterUserAccessLevelsItems > >,
            ) -> Self {
                self.user_process_filter_user_access_levels = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_processes<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_processes_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_processes_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleAppsScriptTypeProcess>
            {
                self.iter_processes_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_processes_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleAppsScriptTypeProcess>
            {
                self.iter_processes_with_fields(Some("*"))
            }
            pub fn iter_processes_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "processes").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "processes")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListUserProcessesResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListUserProcessesResponse>
            {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
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
            ) -> Result<crate::schemas::ListUserProcessesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListUserProcessesResponse, crate::Error> {
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
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/processes");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[(
                    "userProcessFilter.deploymentId",
                    &self.user_process_filter_deployment_id,
                )]);
                let req = req.query(&[(
                    "userProcessFilter.endTime",
                    &self.user_process_filter_end_time,
                )]);
                let req = req.query(&[(
                    "userProcessFilter.functionName",
                    &self.user_process_filter_function_name,
                )]);
                let req = req.query(&[(
                    "userProcessFilter.projectName",
                    &self.user_process_filter_project_name,
                )]);
                let req = req.query(&[(
                    "userProcessFilter.scriptId",
                    &self.user_process_filter_script_id,
                )]);
                let req = req.query(&[(
                    "userProcessFilter.startTime",
                    &self.user_process_filter_start_time,
                )]);
                let req = req.query(&[(
                    "userProcessFilter.statuses",
                    &self.user_process_filter_statuses,
                )]);
                let req =
                    req.query(&[("userProcessFilter.types", &self.user_process_filter_types)]);
                let req = req.query(&[(
                    "userProcessFilter.userAccessLevels",
                    &self.user_process_filter_user_access_levels,
                )]);
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
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        #[doc = "Created via [ProcessesActions::list_script_processes()](struct.ProcessesActions.html#method.list_script_processes)"]
        #[derive(Debug, Clone)]
        pub struct ListScriptProcessesRequestBuilder < 'a > { pub ( crate ) reqwest : & 'a :: reqwest :: blocking :: Client , pub ( crate ) auth : & 'a dyn :: google_api_auth :: GetAccessToken , page_size : Option < i32 > , page_token : Option < String > , script_id : Option < String > , script_process_filter_deployment_id : Option < String > , script_process_filter_end_time : Option < String > , script_process_filter_function_name : Option < String > , script_process_filter_start_time : Option < String > , script_process_filter_statuses : Option < Vec < crate :: resources :: processes :: params :: ListScriptProcessesScriptProcessFilterStatusesItems > > , script_process_filter_types : Option < Vec < crate :: resources :: processes :: params :: ListScriptProcessesScriptProcessFilterTypesItems > > , script_process_filter_user_access_levels : Option < Vec < crate :: resources :: processes :: params :: ListScriptProcessesScriptProcessFilterUserAccessLevelsItems > > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
        impl<'a> ListScriptProcessesRequestBuilder<'a> {
            #[doc = "The maximum number of returned processes per page of results. Defaults to\n50."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The token for continuing a previous list request on the next page. This\nshould be set to the value of `nextPageToken` from a previous response."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The script ID of the project whose processes are listed."]
            pub fn script_id(mut self, value: impl Into<String>) -> Self {
                self.script_id = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those originating from\nprojects with a specific deployment ID."]
            pub fn script_process_filter_deployment_id(mut self, value: impl Into<String>) -> Self {
                self.script_process_filter_deployment_id = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those that completed\non or before the given timestamp."]
            pub fn script_process_filter_end_time(mut self, value: impl Into<String>) -> Self {
                self.script_process_filter_end_time = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those originating from\na script function with the given function name."]
            pub fn script_process_filter_function_name(mut self, value: impl Into<String>) -> Self {
                self.script_process_filter_function_name = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those that were\nstarted on or after the given timestamp."]
            pub fn script_process_filter_start_time(mut self, value: impl Into<String>) -> Self {
                self.script_process_filter_start_time = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those having one of\nthe specified process statuses."]
            pub fn script_process_filter_statuses(
                mut self,
                value : impl Into < Vec < crate :: resources :: processes :: params :: ListScriptProcessesScriptProcessFilterStatusesItems > >,
            ) -> Self {
                self.script_process_filter_statuses = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those having one of\nthe specified process types."]
            pub fn script_process_filter_types(
                mut self,
                value : impl Into < Vec < crate :: resources :: processes :: params :: ListScriptProcessesScriptProcessFilterTypesItems > >,
            ) -> Self {
                self.script_process_filter_types = Some(value.into());
                self
            }
            #[doc = "Optional field used to limit returned processes to those having one of\nthe specified user access levels."]
            pub fn script_process_filter_user_access_levels(
                mut self,
                value : impl Into < Vec < crate :: resources :: processes :: params :: ListScriptProcessesScriptProcessFilterUserAccessLevelsItems > >,
            ) -> Self {
                self.script_process_filter_user_access_levels = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_processes<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_processes_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_processes_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleAppsScriptTypeProcess>
            {
                self.iter_processes_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_processes_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleAppsScriptTypeProcess>
            {
                self.iter_processes_with_fields(Some("*"))
            }
            pub fn iter_processes_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "processes").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "processes")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListScriptProcessesResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListScriptProcessesResponse>
            {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
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
            ) -> Result<crate::schemas::ListScriptProcessesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListScriptProcessesResponse, crate::Error> {
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
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/processes:listScriptProcesses");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("scriptId", &self.script_id)]);
                let req = req.query(&[(
                    "scriptProcessFilter.deploymentId",
                    &self.script_process_filter_deployment_id,
                )]);
                let req = req.query(&[(
                    "scriptProcessFilter.endTime",
                    &self.script_process_filter_end_time,
                )]);
                let req = req.query(&[(
                    "scriptProcessFilter.functionName",
                    &self.script_process_filter_function_name,
                )]);
                let req = req.query(&[(
                    "scriptProcessFilter.startTime",
                    &self.script_process_filter_start_time,
                )]);
                let req = req.query(&[(
                    "scriptProcessFilter.statuses",
                    &self.script_process_filter_statuses,
                )]);
                let req = req.query(&[(
                    "scriptProcessFilter.types",
                    &self.script_process_filter_types,
                )]);
                let req = req.query(&[(
                    "scriptProcessFilter.userAccessLevels",
                    &self.script_process_filter_user_access_levels,
                )]);
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
        impl<'a> crate::iter::IterableMethod for ListScriptProcessesRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
    }
    pub mod projects {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetMetricsMetricsGranularity {
                Daily,
                UnspecifiedGranularity,
                Weekly,
            }
            impl GetMetricsMetricsGranularity {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetMetricsMetricsGranularity::Daily => "DAILY",
                        GetMetricsMetricsGranularity::UnspecifiedGranularity => {
                            "UNSPECIFIED_GRANULARITY"
                        }
                        GetMetricsMetricsGranularity::Weekly => "WEEKLY",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetMetricsMetricsGranularity {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetMetricsMetricsGranularity {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetMetricsMetricsGranularity, ()> {
                    Ok(match s {
                        "DAILY" => GetMetricsMetricsGranularity::Daily,
                        "UNSPECIFIED_GRANULARITY" => {
                            GetMetricsMetricsGranularity::UnspecifiedGranularity
                        }
                        "WEEKLY" => GetMetricsMetricsGranularity::Weekly,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetMetricsMetricsGranularity {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetMetricsMetricsGranularity {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetMetricsMetricsGranularity {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "DAILY" => GetMetricsMetricsGranularity::Daily,
                        "UNSPECIFIED_GRANULARITY" => {
                            GetMetricsMetricsGranularity::UnspecifiedGranularity
                        }
                        "WEEKLY" => GetMetricsMetricsGranularity::Weekly,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetMetricsMetricsGranularity {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetMetricsMetricsGranularity {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a new, empty script project with no script files and a base\nmanifest file."]
            pub fn create(
                &self,
                request: crate::schemas::CreateProjectRequest,
            ) -> CreateRequestBuilder {
                CreateRequestBuilder {
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
            #[doc = "Gets a script project's metadata."]
            pub fn get(&self, script_id: impl Into<String>) -> GetRequestBuilder {
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
                    script_id: script_id.into(),
                }
            }
            #[doc = "Gets the content of the script project, including the code source and\nmetadata for each script file."]
            pub fn get_content(&self, script_id: impl Into<String>) -> GetContentRequestBuilder {
                GetContentRequestBuilder {
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
                    script_id: script_id.into(),
                    version_number: None,
                }
            }
            #[doc = "Get metrics data for scripts, such as number of executions and\nactive users."]
            pub fn get_metrics(&self, script_id: impl Into<String>) -> GetMetricsRequestBuilder {
                GetMetricsRequestBuilder {
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
                    script_id: script_id.into(),
                    metrics_filter_deployment_id: None,
                    metrics_granularity: None,
                }
            }
            #[doc = "Updates the content of the specified script project.\nThis content is stored as the HEAD version, and is used when the script is\nexecuted as a trigger, in the script editor, in add-on preview mode, or as\na web app or Apps Script API in development mode. This clears all the\nexisting files in the project."]
            pub fn update_content(
                &self,
                request: crate::schemas::Content,
                script_id: impl Into<String>,
            ) -> UpdateContentRequestBuilder {
                UpdateContentRequestBuilder {
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
                    script_id: script_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the deployments resource"]
            pub fn deployments(
                &self,
            ) -> crate::resources::projects::deployments::DeploymentsActions {
                crate::resources::projects::deployments::DeploymentsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the versions resource"]
            pub fn versions(&self) -> crate::resources::projects::versions::VersionsActions {
                crate::resources::projects::versions::VersionsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::create()](struct.ProjectsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CreateProjectRequest,
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
        impl<'a> CreateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Project, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Project, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/projects");
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
        #[doc = "Created via [ProjectsActions::get()](struct.ProjectsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            script_id: String,
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
            ) -> Result<crate::schemas::Project, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Project, crate::Error> {
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
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.script_id;
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
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [ProjectsActions::get_content()](struct.ProjectsActions.html#method.get_content)"]
        #[derive(Debug, Clone)]
        pub struct GetContentRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            script_id: String,
            version_number: Option<i32>,
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
        impl<'a> GetContentRequestBuilder<'a> {
            #[doc = "The version number of the project to retrieve. If not provided, the\nproject's HEAD version is returned."]
            pub fn version_number(mut self, value: i32) -> Self {
                self.version_number = Some(value);
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
            ) -> Result<crate::schemas::Content, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Content, crate::Error> {
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
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.script_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/content");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("versionNumber", &self.version_number)]);
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
        #[doc = "Created via [ProjectsActions::get_metrics()](struct.ProjectsActions.html#method.get_metrics)"]
        #[derive(Debug, Clone)]
        pub struct GetMetricsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            script_id: String,
            metrics_filter_deployment_id: Option<String>,
            metrics_granularity:
                Option<crate::resources::projects::params::GetMetricsMetricsGranularity>,
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
        impl<'a> GetMetricsRequestBuilder<'a> {
            #[doc = "Optional field indicating a specific deployment to retrieve metrics from."]
            pub fn metrics_filter_deployment_id(mut self, value: impl Into<String>) -> Self {
                self.metrics_filter_deployment_id = Some(value.into());
                self
            }
            #[doc = "Required field indicating what granularity of metrics are returned."]
            pub fn metrics_granularity(
                mut self,
                value: crate::resources::projects::params::GetMetricsMetricsGranularity,
            ) -> Self {
                self.metrics_granularity = Some(value);
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
            ) -> Result<crate::schemas::Metrics, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Metrics, crate::Error> {
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
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.script_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/metrics");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[(
                    "metricsFilter.deploymentId",
                    &self.metrics_filter_deployment_id,
                )]);
                let req = req.query(&[("metricsGranularity", &self.metrics_granularity)]);
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
        #[doc = "Created via [ProjectsActions::update_content()](struct.ProjectsActions.html#method.update_content)"]
        #[derive(Debug, Clone)]
        pub struct UpdateContentRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Content,
            script_id: String,
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
        impl<'a> UpdateContentRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Content, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Content, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.script_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/content");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
        pub mod deployments {
            pub mod params {}
            pub struct DeploymentsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DeploymentsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a deployment of an Apps Script project."]
                pub fn create(
                    &self,
                    request: crate::schemas::DeploymentConfig,
                    script_id: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
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
                        script_id: script_id.into(),
                    }
                }
                #[doc = "Deletes a deployment of an Apps Script project."]
                pub fn delete(
                    &self,
                    script_id: impl Into<String>,
                    deployment_id: impl Into<String>,
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
                        script_id: script_id.into(),
                        deployment_id: deployment_id.into(),
                    }
                }
                #[doc = "Gets a deployment of an Apps Script project."]
                pub fn get(
                    &self,
                    script_id: impl Into<String>,
                    deployment_id: impl Into<String>,
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
                        script_id: script_id.into(),
                        deployment_id: deployment_id.into(),
                    }
                }
                #[doc = "Lists the deployments of an Apps Script project."]
                pub fn list(&self, script_id: impl Into<String>) -> ListRequestBuilder {
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
                        script_id: script_id.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a deployment of an Apps Script project."]
                pub fn update(
                    &self,
                    request: crate::schemas::UpdateDeploymentRequest,
                    script_id: impl Into<String>,
                    deployment_id: impl Into<String>,
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
                        script_id: script_id.into(),
                        deployment_id: deployment_id.into(),
                    }
                }
            }
            #[doc = "Created via [DeploymentsActions::create()](struct.DeploymentsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::DeploymentConfig,
                script_id: String,
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
            impl<'a> CreateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Deployment, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Deployment, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/deployments");
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
            #[doc = "Created via [DeploymentsActions::delete()](struct.DeploymentsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                script_id: String,
                deployment_id: String,
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/deployments/");
                    {
                        let var_as_str = &self.deployment_id;
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
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
            #[doc = "Created via [DeploymentsActions::get()](struct.DeploymentsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                script_id: String,
                deployment_id: String,
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
                ) -> Result<crate::schemas::Deployment, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Deployment, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/deployments/");
                    {
                        let var_as_str = &self.deployment_id;
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
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [DeploymentsActions::list()](struct.DeploymentsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                script_id: String,
                page_size: Option<i32>,
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
                #[doc = "The maximum number of deployments on each returned page. Defaults to 50."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The token for continuing a previous list request on the next page. This\nshould be set to the value of `nextPageToken` from a previous response."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_deployments<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_deployments_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_deployments_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Deployment> {
                    self.iter_deployments_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_deployments_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Deployment> {
                    self.iter_deployments_with_fields(Some("*"))
                }
                pub fn iter_deployments_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "deployments").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "deployments")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDeploymentsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDeploymentsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
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
                ) -> Result<crate::schemas::ListDeploymentsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDeploymentsResponse, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/deployments");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
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
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [DeploymentsActions::update()](struct.DeploymentsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UpdateDeploymentRequest,
                script_id: String,
                deployment_id: String,
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
                ) -> Result<crate::schemas::Deployment, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Deployment, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/deployments/");
                    {
                        let var_as_str = &self.deployment_id;
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
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
        pub mod versions {
            pub mod params {}
            pub struct VersionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> VersionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new immutable version using the current code, with a unique\nversion number."]
                pub fn create(
                    &self,
                    request: crate::schemas::Version,
                    script_id: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
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
                        script_id: script_id.into(),
                    }
                }
                #[doc = "Gets a version of a script project."]
                pub fn get(
                    &self,
                    script_id: impl Into<String>,
                    version_number: i32,
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
                        script_id: script_id.into(),
                        version_number,
                    }
                }
                #[doc = "List the versions of a script project."]
                pub fn list(&self, script_id: impl Into<String>) -> ListRequestBuilder {
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
                        script_id: script_id.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [VersionsActions::create()](struct.VersionsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Version,
                script_id: String,
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
            impl<'a> CreateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Version, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Version, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/versions");
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
            #[doc = "Created via [VersionsActions::get()](struct.VersionsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                script_id: String,
                version_number: i32,
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
                ) -> Result<crate::schemas::Version, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Version, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/versions/");
                    {
                        let var_as_string = self.version_number.to_string();
                        let var_as_str = &var_as_string;
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
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [VersionsActions::list()](struct.VersionsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                script_id: String,
                page_size: Option<i32>,
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
                #[doc = "The maximum number of versions on each returned page. Defaults to 50."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The token for continuing a previous list request on the next page. This\nshould be set to the value of `nextPageToken` from a previous response."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_versions<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_versions_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_versions_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Version> {
                    self.iter_versions_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_versions_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Version> {
                    self.iter_versions_with_fields(Some("*"))
                }
                pub fn iter_versions_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "versions").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "versions")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListVersionsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListVersionsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
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
                ) -> Result<crate::schemas::ListVersionsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListVersionsResponse, crate::Error> {
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
                    let mut output = "https://script.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.script_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/versions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
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
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
        }
    }
    pub mod scripts {
        pub mod params {}
        pub struct ScriptsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ScriptsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Runs a function in an Apps Script project. The script project must be\ndeployed for use with the Apps Script API and the calling application must\nshare the same Cloud Platform project.\n\nThis method requires authorization with an OAuth 2.0 token that includes at\nleast one of the scopes listed in the\n[Authorization](#authorization-scopes) section; script projects that do not\nrequire authorization cannot be executed through this API. To find the\ncorrect scopes to include in the authentication token, open the project in\nthe script editor, then select **File > Project properties** and click the\n**Scopes** tab.\n\nThe error `403, PERMISSION_DENIED: The caller does not have permission`\nindicates that the Cloud Platform project used to authorize the request is\nnot the same as the one used by the script."]
            pub fn run(
                &self,
                request: crate::schemas::ExecutionRequest,
                script_id: impl Into<String>,
            ) -> RunRequestBuilder {
                RunRequestBuilder {
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
                    script_id: script_id.into(),
                }
            }
        }
        #[doc = "Created via [ScriptsActions::run()](struct.ScriptsActions.html#method.run)"]
        #[derive(Debug, Clone)]
        pub struct RunRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ExecutionRequest,
            script_id: String,
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
        impl<'a> RunRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://script.googleapis.com/".to_owned();
                output.push_str("v1/scripts/");
                {
                    let var_as_str = &self.script_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":run");
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
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
}
