#![doc = "# Resources and Methods\n    * [backup_runs](resources/backup_runs/struct.BackupRunsActions.html)\n      * [*delete*](resources/backup_runs/struct.DeleteRequestBuilder.html), [*get*](resources/backup_runs/struct.GetRequestBuilder.html), [*insert*](resources/backup_runs/struct.InsertRequestBuilder.html), [*list*](resources/backup_runs/struct.ListRequestBuilder.html)\n    * [databases](resources/databases/struct.DatabasesActions.html)\n      * [*delete*](resources/databases/struct.DeleteRequestBuilder.html), [*get*](resources/databases/struct.GetRequestBuilder.html), [*insert*](resources/databases/struct.InsertRequestBuilder.html), [*list*](resources/databases/struct.ListRequestBuilder.html), [*patch*](resources/databases/struct.PatchRequestBuilder.html), [*update*](resources/databases/struct.UpdateRequestBuilder.html)\n    * [flags](resources/flags/struct.FlagsActions.html)\n      * [*list*](resources/flags/struct.ListRequestBuilder.html)\n    * [instances](resources/instances/struct.InstancesActions.html)\n      * [*addServerCa*](resources/instances/struct.AddServerCaRequestBuilder.html), [*clone*](resources/instances/struct.CloneRequestBuilder.html), [*delete*](resources/instances/struct.DeleteRequestBuilder.html), [*demoteMaster*](resources/instances/struct.DemoteMasterRequestBuilder.html), [*export*](resources/instances/struct.ExportRequestBuilder.html), [*failover*](resources/instances/struct.FailoverRequestBuilder.html), [*get*](resources/instances/struct.GetRequestBuilder.html), [*import*](resources/instances/struct.ImportRequestBuilder.html), [*insert*](resources/instances/struct.InsertRequestBuilder.html), [*list*](resources/instances/struct.ListRequestBuilder.html), [*listServerCas*](resources/instances/struct.ListServerCasRequestBuilder.html), [*patch*](resources/instances/struct.PatchRequestBuilder.html), [*promoteReplica*](resources/instances/struct.PromoteReplicaRequestBuilder.html), [*resetSslConfig*](resources/instances/struct.ResetSslConfigRequestBuilder.html), [*restart*](resources/instances/struct.RestartRequestBuilder.html), [*restoreBackup*](resources/instances/struct.RestoreBackupRequestBuilder.html), [*rotateServerCa*](resources/instances/struct.RotateServerCaRequestBuilder.html), [*startReplica*](resources/instances/struct.StartReplicaRequestBuilder.html), [*stopReplica*](resources/instances/struct.StopReplicaRequestBuilder.html), [*truncateLog*](resources/instances/struct.TruncateLogRequestBuilder.html), [*update*](resources/instances/struct.UpdateRequestBuilder.html)\n    * [operations](resources/operations/struct.OperationsActions.html)\n      * [*get*](resources/operations/struct.GetRequestBuilder.html), [*list*](resources/operations/struct.ListRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [instances](resources/projects/instances/struct.InstancesActions.html)\n        * [*rescheduleMaintenance*](resources/projects/instances/struct.RescheduleMaintenanceRequestBuilder.html), [*startExternalSync*](resources/projects/instances/struct.StartExternalSyncRequestBuilder.html), [*verifyExternalSyncSettings*](resources/projects/instances/struct.VerifyExternalSyncSettingsRequestBuilder.html)\n    * [ssl_certs](resources/ssl_certs/struct.SslCertsActions.html)\n      * [*createEphemeral*](resources/ssl_certs/struct.CreateEphemeralRequestBuilder.html), [*delete*](resources/ssl_certs/struct.DeleteRequestBuilder.html), [*get*](resources/ssl_certs/struct.GetRequestBuilder.html), [*insert*](resources/ssl_certs/struct.InsertRequestBuilder.html), [*list*](resources/ssl_certs/struct.ListRequestBuilder.html)\n    * [tiers](resources/tiers/struct.TiersActions.html)\n      * [*list*](resources/tiers/struct.ListRequestBuilder.html)\n    * [users](resources/users/struct.UsersActions.html)\n      * [*delete*](resources/users/struct.DeleteRequestBuilder.html), [*insert*](resources/users/struct.InsertRequestBuilder.html), [*list*](resources/users/struct.ListRequestBuilder.html), [*update*](resources/users/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "Manage your Google SQL Service instances\n\n`https://www.googleapis.com/auth/sqlservice.admin`"]
    pub const SQLSERVICE_ADMIN: &str = "https://www.googleapis.com/auth/sqlservice.admin";
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
    pub struct AclEntry {
        #[doc = "The time when this access control entry expires in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expiration_time: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#aclEntry</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Optional. A label to identify this entry."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The whitelisted value for the access control list."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AclEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AclEntry {
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
    pub struct ApiWarning {
        #[doc = "Code to uniquely identify the warning type."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ApiWarningCode>,
        #[doc = "The warning message."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ApiWarning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiWarning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiWarningCode {
        #[doc = "Warning when one or more regions are not reachable.  The returned result\nset may be incomplete."]
        RegionUnreachable,
        #[doc = "An unknown or unset warning type from Cloud SQL API."]
        SqlApiWarningCodeUnspecified,
    }
    impl ApiWarningCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiWarningCode::RegionUnreachable => "REGION_UNREACHABLE",
                ApiWarningCode::SqlApiWarningCodeUnspecified => "SQL_API_WARNING_CODE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApiWarningCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApiWarningCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApiWarningCode, ()> {
            Ok(match s {
                "REGION_UNREACHABLE" => ApiWarningCode::RegionUnreachable,
                "SQL_API_WARNING_CODE_UNSPECIFIED" => ApiWarningCode::SqlApiWarningCodeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApiWarningCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiWarningCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiWarningCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REGION_UNREACHABLE" => ApiWarningCode::RegionUnreachable,
                "SQL_API_WARNING_CODE_UNSPECIFIED" => ApiWarningCode::SqlApiWarningCodeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApiWarningCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiWarningCode {
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
    pub struct BackupConfiguration {
        #[doc = "(MySQL only) Whether binary log is enabled. If backup configuration is\ndisabled, binarylog must be disabled as well."]
        #[serde(
            rename = "binaryLogEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binary_log_enabled: ::std::option::Option<bool>,
        #[doc = "Whether this configuration is enabled."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "This is always <code>sql#backupConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Location of the backup"]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Reserved for future use."]
        #[serde(
            rename = "pointInTimeRecoveryEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point_in_time_recovery_enabled: ::std::option::Option<bool>,
        #[doc = "Reserved for future use."]
        #[serde(
            rename = "replicationLogArchivingEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replication_log_archiving_enabled: ::std::option::Option<bool>,
        #[doc = "Start time for the daily backup configuration in UTC timezone in the 24\nhour format - <code>HH:MM</code>."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BackupConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackupConfiguration {
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
    pub struct BackupRun {
        #[doc = "The description of this run, only applicable to on-demand backups."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Encryption configuration specific to a backup.\nApplies only to Second Generation instances."]
        #[serde(
            rename = "diskEncryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_encryption_configuration:
            ::std::option::Option<crate::schemas::DiskEncryptionConfiguration>,
        #[doc = "Encryption status specific to a backup.\nApplies only to Second Generation instances."]
        #[serde(
            rename = "diskEncryptionStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_encryption_status: ::std::option::Option<crate::schemas::DiskEncryptionStatus>,
        #[doc = "The time the backup operation completed in UTC timezone in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The time the run was enqueued in UTC timezone in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "enqueuedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enqueued_time: ::std::option::Option<String>,
        #[doc = "Information about why the backup operation failed. This is only present if\nthe run has the FAILED status."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::OperationError>,
        #[doc = "The identifier for this backup run. Unique only for a specific Cloud SQL\ninstance."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "Name of the database instance."]
        #[serde(
            rename = "instance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#backupRun</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Location of the backups."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The type of this run; can be either \"AUTOMATED\" or \"ON_DEMAND\"."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::BackupRunType>,
        #[doc = "The URI of this resource."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "The time the backup operation actually started in UTC timezone in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The status of this run."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::BackupRunStatus>,
        #[doc = "The start time of the backup window during which this the backup was\nattempted in <a href=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a>\nformat, for example <code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "windowStartTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub window_start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BackupRun {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackupRun {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BackupRunType {
        #[doc = "The backup schedule automatically triggers a backup."]
        Automated,
        #[doc = "The user manually triggers a backup."]
        OnDemand,
        #[doc = "This is an unknown BackupRun type."]
        SqlBackupRunTypeUnspecified,
    }
    impl BackupRunType {
        pub fn as_str(self) -> &'static str {
            match self {
                BackupRunType::Automated => "AUTOMATED",
                BackupRunType::OnDemand => "ON_DEMAND",
                BackupRunType::SqlBackupRunTypeUnspecified => "SQL_BACKUP_RUN_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BackupRunType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BackupRunType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BackupRunType, ()> {
            Ok(match s {
                "AUTOMATED" => BackupRunType::Automated,
                "ON_DEMAND" => BackupRunType::OnDemand,
                "SQL_BACKUP_RUN_TYPE_UNSPECIFIED" => BackupRunType::SqlBackupRunTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BackupRunType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BackupRunType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BackupRunType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTOMATED" => BackupRunType::Automated,
                "ON_DEMAND" => BackupRunType::OnDemand,
                "SQL_BACKUP_RUN_TYPE_UNSPECIFIED" => BackupRunType::SqlBackupRunTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BackupRunType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackupRunType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BackupRunStatus {
        #[doc = "The backup has been deleted."]
        Deleted,
        #[doc = "The backup deletion failed."]
        DeletionFailed,
        #[doc = "The backup is about to be deleted."]
        DeletionPending,
        #[doc = "The backup operation was enqueued."]
        Enqueued,
        #[doc = "The backup failed."]
        Failed,
        #[doc = "The backup is overdue across a given backup window. Indicates a\nproblem. Example: Long-running operation in progress during\nthe whole window."]
        Overdue,
        #[doc = "The backup is in progress."]
        Running,
        #[doc = "The backup was skipped (without problems) for a given backup\nwindow. Example: Instance was idle."]
        Skipped,
        #[doc = "The status of the run is unknown."]
        SqlBackupRunStatusUnspecified,
        #[doc = "The backup was successful."]
        Successful,
    }
    impl BackupRunStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                BackupRunStatus::Deleted => "DELETED",
                BackupRunStatus::DeletionFailed => "DELETION_FAILED",
                BackupRunStatus::DeletionPending => "DELETION_PENDING",
                BackupRunStatus::Enqueued => "ENQUEUED",
                BackupRunStatus::Failed => "FAILED",
                BackupRunStatus::Overdue => "OVERDUE",
                BackupRunStatus::Running => "RUNNING",
                BackupRunStatus::Skipped => "SKIPPED",
                BackupRunStatus::SqlBackupRunStatusUnspecified => {
                    "SQL_BACKUP_RUN_STATUS_UNSPECIFIED"
                }
                BackupRunStatus::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BackupRunStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BackupRunStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BackupRunStatus, ()> {
            Ok(match s {
                "DELETED" => BackupRunStatus::Deleted,
                "DELETION_FAILED" => BackupRunStatus::DeletionFailed,
                "DELETION_PENDING" => BackupRunStatus::DeletionPending,
                "ENQUEUED" => BackupRunStatus::Enqueued,
                "FAILED" => BackupRunStatus::Failed,
                "OVERDUE" => BackupRunStatus::Overdue,
                "RUNNING" => BackupRunStatus::Running,
                "SKIPPED" => BackupRunStatus::Skipped,
                "SQL_BACKUP_RUN_STATUS_UNSPECIFIED" => {
                    BackupRunStatus::SqlBackupRunStatusUnspecified
                }
                "SUCCESSFUL" => BackupRunStatus::Successful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BackupRunStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BackupRunStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BackupRunStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DELETED" => BackupRunStatus::Deleted,
                "DELETION_FAILED" => BackupRunStatus::DeletionFailed,
                "DELETION_PENDING" => BackupRunStatus::DeletionPending,
                "ENQUEUED" => BackupRunStatus::Enqueued,
                "FAILED" => BackupRunStatus::Failed,
                "OVERDUE" => BackupRunStatus::Overdue,
                "RUNNING" => BackupRunStatus::Running,
                "SKIPPED" => BackupRunStatus::Skipped,
                "SQL_BACKUP_RUN_STATUS_UNSPECIFIED" => {
                    BackupRunStatus::SqlBackupRunStatusUnspecified
                }
                "SUCCESSFUL" => BackupRunStatus::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BackupRunStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackupRunStatus {
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
    pub struct BackupRunsListResponse {
        #[doc = "A list of backup runs in reverse chronological order of the enqueued time."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::BackupRun>>,
        #[doc = "This is always <code>sql#backupRunsList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The continuation token, used to page through large result sets. Provide\nthis value in a subsequent request to return the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BackupRunsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackupRunsListResponse {
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
    pub struct BinLogCoordinates {
        #[doc = "Name of the binary log file for a Cloud SQL instance."]
        #[serde(
            rename = "binLogFileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bin_log_file_name: ::std::option::Option<String>,
        #[doc = "Position (offset) within the binary log file."]
        #[serde(
            rename = "binLogPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bin_log_position: ::std::option::Option<i64>,
        #[doc = "This is always <code>sql#binLogCoordinates</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BinLogCoordinates {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BinLogCoordinates {
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
    pub struct CloneContext {
        #[doc = "Binary log coordinates, if specified, identify the position up to which the\nsource instance should be cloned. If not specified, the source instance is\ncloned up to the most recent binary log coordinates."]
        #[serde(
            rename = "binLogCoordinates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bin_log_coordinates: ::std::option::Option<crate::schemas::BinLogCoordinates>,
        #[doc = "Name of the Cloud SQL instance to be created as a clone."]
        #[serde(
            rename = "destinationInstanceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_instance_name: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#cloneContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Reserved for future use."]
        #[serde(
            rename = "pitrTimestampMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pitr_timestamp_ms: ::std::option::Option<i64>,
        #[doc = "Reserved for future use."]
        #[serde(
            rename = "pointInTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point_in_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CloneContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloneContext {
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
    pub struct Database {
        #[doc = "The MySQL charset value."]
        #[serde(
            rename = "charset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charset: ::std::option::Option<String>,
        #[doc = "The MySQL collation value."]
        #[serde(
            rename = "collation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub collation: ::std::option::Option<String>,
        #[doc = "This field is deprecated and will be removed from a future version of the\nAPI."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The name of the Cloud SQL instance. This does not include the project ID."]
        #[serde(
            rename = "instance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#database</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the database in the Cloud SQL instance. This does not include\nthe project ID or instance name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The project ID of the project containing the Cloud SQL database. The Google\napps domain is prefixed if applicable."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "The URI of this resource."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[serde(
            rename = "sqlserverDatabaseDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sqlserver_database_details:
            ::std::option::Option<crate::schemas::SqlServerDatabaseDetails>,
    }
    impl ::google_field_selector::FieldSelector for Database {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Database {
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
    pub struct DatabaseFlags {
        #[doc = "The name of the flag. These flags are passed at instance startup, so\ninclude both server options and system variables for MySQL. Flags should be\nspecified with underscores, not hyphens. For more information, see <a\nhref=\"/sql/docs/mysql/flags\">Configuring Database Flags</a> in the Cloud\nSQL documentation."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The value of the flag. Booleans should be set to <code>on</code> for true\nand <code>off</code> for false. This field must be omitted if the flag\ndoesn't take a value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatabaseFlags {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseFlags {
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
    pub struct DatabaseInstance {
        #[doc = "<code>FIRST_GEN</code>: First Generation instance. MySQL only. <br\n/><code>SECOND_GEN</code>: Second Generation instance or PostgreSQL\ninstance. <br /><code>EXTERNAL</code>: A database server that is not\nmanaged by Google. <br>This property is read-only; use the\n<code>tier</code> property in the <code>settings</code> object to determine\nthe database type and Second or First Generation."]
        #[serde(
            rename = "backendType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backend_type: ::std::option::Option<crate::schemas::DatabaseInstanceBackendType>,
        #[doc = "Connection name of the Cloud SQL instance used in connection strings."]
        #[serde(
            rename = "connectionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_name: ::std::option::Option<String>,
        #[doc = "The current disk usage of the instance in bytes. This property has been\ndeprecated. Users should use the\n\"cloudsql.googleapis.com/database/disk/bytes_used\" metric in Cloud\nMonitoring API instead. Please see <a\nhref=\"https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ\">this\nannouncement</a> for details."]
        #[serde(
            rename = "currentDiskSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current_disk_size: ::std::option::Option<i64>,
        #[doc = "The database engine type and version. The <code>databaseVersion</code>\nfield can not be changed after instance creation.  MySQL Second Generation\ninstances: <code>MYSQL_5_7</code> (default) or <code>MYSQL_5_6</code>.\nPostgreSQL instances: <code>POSTGRES_9_6</code> (default) or\n<code>POSTGRES_11 Beta</code> MySQL First Generation\ninstances: <code>MYSQL_5_6</code> (default) or <code>MYSQL_5_5</code>"]
        #[serde(
            rename = "databaseVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub database_version:
            ::std::option::Option<crate::schemas::DatabaseInstanceDatabaseVersion>,
        #[doc = "Disk encryption configuration specific to an instance.\nApplies only to Second Generation instances."]
        #[serde(
            rename = "diskEncryptionConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_encryption_configuration:
            ::std::option::Option<crate::schemas::DiskEncryptionConfiguration>,
        #[doc = "Disk encryption status specific to an instance.\nApplies only to Second Generation instances."]
        #[serde(
            rename = "diskEncryptionStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_encryption_status: ::std::option::Option<crate::schemas::DiskEncryptionStatus>,
        #[doc = "This field is deprecated and will be removed from a future version of the\nAPI. Use the <code>settings.settingsVersion</code> field instead."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The name and status of the failover replica. This property is applicable\nonly to Second Generation instances."]
        #[serde(
            rename = "failoverReplica",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failover_replica:
            ::std::option::Option<crate::schemas::DatabaseInstanceFailoverReplica>,
        #[doc = "The Compute Engine zone that the instance is currently serving from. This\nvalue could be different from the zone that was specified when the instance\nwas created if the instance has failed over to its secondary zone."]
        #[serde(
            rename = "gceZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gce_zone: ::std::option::Option<String>,
        #[doc = "The instance type. This can be one of the following.\n<br><code>CLOUD_SQL_INSTANCE</code>: A Cloud SQL instance that is not\nreplicating from a master. <br><code>ON_PREMISES_INSTANCE</code>: An\ninstance running on the\ncustomer's premises. <br><code>READ_REPLICA_INSTANCE</code>: A Cloud SQL\ninstance configured as a read-replica."]
        #[serde(
            rename = "instanceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_type: ::std::option::Option<crate::schemas::DatabaseInstanceInstanceType>,
        #[doc = "The assigned IP addresses for the instance."]
        #[serde(
            rename = "ipAddresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_addresses: ::std::option::Option<Vec<crate::schemas::IpMapping>>,
        #[doc = "The IPv6 address assigned to the instance. This property is applicable only\nto First Generation instances."]
        #[serde(
            rename = "ipv6Address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ipv_6_address: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#instance</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the instance which will act as master in the replication setup."]
        #[serde(
            rename = "masterInstanceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_instance_name: ::std::option::Option<String>,
        #[doc = "The maximum disk size of the instance in bytes."]
        #[serde(
            rename = "maxDiskSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_disk_size: ::std::option::Option<i64>,
        #[doc = "Name of the Cloud SQL instance. This does not include the project ID."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Configuration specific to on-premises instances."]
        #[serde(
            rename = "onPremisesConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_premises_configuration:
            ::std::option::Option<crate::schemas::OnPremisesConfiguration>,
        #[doc = "The project ID of the project containing the Cloud SQL instance. The Google\napps domain is prefixed if applicable."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "The geographical region. Can be <code>us-central</code>\n(<code>FIRST_GEN</code> instances only), <code>us-central1</code>\n(<code>SECOND_GEN</code> instances only), <code>asia-east1</code> or\n<code>europe-west1</code>. Defaults to <code>us-central</code> or\n<code>us-central1</code> depending on the instance type (First Generation\nor Second Generation). The region can not be changed after instance\ncreation."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "Configuration specific to failover replicas and read replicas."]
        #[serde(
            rename = "replicaConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replica_configuration: ::std::option::Option<crate::schemas::ReplicaConfiguration>,
        #[doc = "The replicas of the instance."]
        #[serde(
            rename = "replicaNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replica_names: ::std::option::Option<Vec<String>>,
        #[doc = "Initial root password. Use only on creation."]
        #[serde(
            rename = "rootPassword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root_password: ::std::option::Option<String>,
        #[doc = "The start time of any upcoming scheduled maintenance for this instance."]
        #[serde(
            rename = "scheduledMaintenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scheduled_maintenance: ::std::option::Option<crate::schemas::SqlScheduledMaintenance>,
        #[doc = "The URI of this resource."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "SSL configuration."]
        #[serde(
            rename = "serverCaCert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub server_ca_cert: ::std::option::Option<crate::schemas::SslCert>,
        #[doc = "The service account email address assigned to the instance. This property\nis applicable only to Second Generation instances."]
        #[serde(
            rename = "serviceAccountEmailAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account_email_address: ::std::option::Option<String>,
        #[doc = "The user settings."]
        #[serde(
            rename = "settings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub settings: ::std::option::Option<crate::schemas::Settings>,
        #[doc = "The current serving state of the Cloud SQL instance. This can be one of the\nfollowing. <br><code>RUNNABLE</code>: The instance is running, or is ready\nto run when accessed. <br><code>SUSPENDED</code>: The instance is not\navailable, for example due to problems with billing.\n<br><code>PENDING_CREATE</code>: The instance is being created.\n<br><code>MAINTENANCE</code>: The instance is down for maintenance.\n<br><code>FAILED</code>: The instance creation failed.\n<br><code>UNKNOWN_STATE</code>: The state of the instance is unknown."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::DatabaseInstanceState>,
        #[doc = "If the instance state is SUSPENDED, the reason for the suspension."]
        #[serde(
            rename = "suspensionReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suspension_reason:
            ::std::option::Option<Vec<crate::schemas::DatabaseInstanceSuspensionReasonItems>>,
    }
    impl ::google_field_selector::FieldSelector for DatabaseInstance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DatabaseInstanceBackendType {
        #[doc = "On premises instance."]
        External,
        #[doc = "V1 speckle instance."]
        FirstGen,
        #[doc = "V2 speckle instance."]
        SecondGen,
        #[doc = "This is an unknown backend type for instance."]
        SqlBackendTypeUnspecified,
    }
    impl DatabaseInstanceBackendType {
        pub fn as_str(self) -> &'static str {
            match self {
                DatabaseInstanceBackendType::External => "EXTERNAL",
                DatabaseInstanceBackendType::FirstGen => "FIRST_GEN",
                DatabaseInstanceBackendType::SecondGen => "SECOND_GEN",
                DatabaseInstanceBackendType::SqlBackendTypeUnspecified => {
                    "SQL_BACKEND_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DatabaseInstanceBackendType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DatabaseInstanceBackendType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DatabaseInstanceBackendType, ()> {
            Ok(match s {
                "EXTERNAL" => DatabaseInstanceBackendType::External,
                "FIRST_GEN" => DatabaseInstanceBackendType::FirstGen,
                "SECOND_GEN" => DatabaseInstanceBackendType::SecondGen,
                "SQL_BACKEND_TYPE_UNSPECIFIED" => {
                    DatabaseInstanceBackendType::SqlBackendTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DatabaseInstanceBackendType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DatabaseInstanceBackendType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DatabaseInstanceBackendType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXTERNAL" => DatabaseInstanceBackendType::External,
                "FIRST_GEN" => DatabaseInstanceBackendType::FirstGen,
                "SECOND_GEN" => DatabaseInstanceBackendType::SecondGen,
                "SQL_BACKEND_TYPE_UNSPECIFIED" => {
                    DatabaseInstanceBackendType::SqlBackendTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for DatabaseInstanceBackendType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstanceBackendType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DatabaseInstanceDatabaseVersion {
        #[doc = "The database version is MySQL 5.1."]
        Mysql51,
        #[doc = "The database version is MySQL 5.5."]
        Mysql55,
        #[doc = "The database version is MySQL 5.6."]
        Mysql56,
        #[doc = "The database version is MySQL 5.7."]
        Mysql57,
        #[doc = "The database version is MySQL 8."]
        Mysql80,
        #[doc = "The database version is PostgreSQL 10."]
        Postgres10,
        #[doc = "The database version is PostgreSQL 11."]
        Postgres11,
        #[doc = "The database version is PostgreSQL 12."]
        Postgres12,
        #[doc = "The database version is PostgreSQL 9.6."]
        Postgres96,
        #[doc = "This is an unknown database version."]
        SqlDatabaseVersionUnspecified,
        #[doc = "The database version is SQL Server 2017 Enterprise."]
        Sqlserver2017Enterprise,
        #[doc = "The database version is SQL Server 2017 Express."]
        Sqlserver2017Express,
        #[doc = "The database version is SQL Server 2017 Standard."]
        Sqlserver2017Standard,
        #[doc = "The database version is SQL Server 2017 Web."]
        Sqlserver2017Web,
    }
    impl DatabaseInstanceDatabaseVersion {
        pub fn as_str(self) -> &'static str {
            match self {
                DatabaseInstanceDatabaseVersion::Mysql51 => "MYSQL_5_1",
                DatabaseInstanceDatabaseVersion::Mysql55 => "MYSQL_5_5",
                DatabaseInstanceDatabaseVersion::Mysql56 => "MYSQL_5_6",
                DatabaseInstanceDatabaseVersion::Mysql57 => "MYSQL_5_7",
                DatabaseInstanceDatabaseVersion::Mysql80 => "MYSQL_8_0",
                DatabaseInstanceDatabaseVersion::Postgres10 => "POSTGRES_10",
                DatabaseInstanceDatabaseVersion::Postgres11 => "POSTGRES_11",
                DatabaseInstanceDatabaseVersion::Postgres12 => "POSTGRES_12",
                DatabaseInstanceDatabaseVersion::Postgres96 => "POSTGRES_9_6",
                DatabaseInstanceDatabaseVersion::SqlDatabaseVersionUnspecified => {
                    "SQL_DATABASE_VERSION_UNSPECIFIED"
                }
                DatabaseInstanceDatabaseVersion::Sqlserver2017Enterprise => {
                    "SQLSERVER_2017_ENTERPRISE"
                }
                DatabaseInstanceDatabaseVersion::Sqlserver2017Express => "SQLSERVER_2017_EXPRESS",
                DatabaseInstanceDatabaseVersion::Sqlserver2017Standard => "SQLSERVER_2017_STANDARD",
                DatabaseInstanceDatabaseVersion::Sqlserver2017Web => "SQLSERVER_2017_WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DatabaseInstanceDatabaseVersion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DatabaseInstanceDatabaseVersion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DatabaseInstanceDatabaseVersion, ()> {
            Ok(match s {
                "MYSQL_5_1" => DatabaseInstanceDatabaseVersion::Mysql51,
                "MYSQL_5_5" => DatabaseInstanceDatabaseVersion::Mysql55,
                "MYSQL_5_6" => DatabaseInstanceDatabaseVersion::Mysql56,
                "MYSQL_5_7" => DatabaseInstanceDatabaseVersion::Mysql57,
                "MYSQL_8_0" => DatabaseInstanceDatabaseVersion::Mysql80,
                "POSTGRES_10" => DatabaseInstanceDatabaseVersion::Postgres10,
                "POSTGRES_11" => DatabaseInstanceDatabaseVersion::Postgres11,
                "POSTGRES_12" => DatabaseInstanceDatabaseVersion::Postgres12,
                "POSTGRES_9_6" => DatabaseInstanceDatabaseVersion::Postgres96,
                "SQL_DATABASE_VERSION_UNSPECIFIED" => {
                    DatabaseInstanceDatabaseVersion::SqlDatabaseVersionUnspecified
                }
                "SQLSERVER_2017_ENTERPRISE" => {
                    DatabaseInstanceDatabaseVersion::Sqlserver2017Enterprise
                }
                "SQLSERVER_2017_EXPRESS" => DatabaseInstanceDatabaseVersion::Sqlserver2017Express,
                "SQLSERVER_2017_STANDARD" => DatabaseInstanceDatabaseVersion::Sqlserver2017Standard,
                "SQLSERVER_2017_WEB" => DatabaseInstanceDatabaseVersion::Sqlserver2017Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DatabaseInstanceDatabaseVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DatabaseInstanceDatabaseVersion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DatabaseInstanceDatabaseVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MYSQL_5_1" => DatabaseInstanceDatabaseVersion::Mysql51,
                "MYSQL_5_5" => DatabaseInstanceDatabaseVersion::Mysql55,
                "MYSQL_5_6" => DatabaseInstanceDatabaseVersion::Mysql56,
                "MYSQL_5_7" => DatabaseInstanceDatabaseVersion::Mysql57,
                "MYSQL_8_0" => DatabaseInstanceDatabaseVersion::Mysql80,
                "POSTGRES_10" => DatabaseInstanceDatabaseVersion::Postgres10,
                "POSTGRES_11" => DatabaseInstanceDatabaseVersion::Postgres11,
                "POSTGRES_12" => DatabaseInstanceDatabaseVersion::Postgres12,
                "POSTGRES_9_6" => DatabaseInstanceDatabaseVersion::Postgres96,
                "SQL_DATABASE_VERSION_UNSPECIFIED" => {
                    DatabaseInstanceDatabaseVersion::SqlDatabaseVersionUnspecified
                }
                "SQLSERVER_2017_ENTERPRISE" => {
                    DatabaseInstanceDatabaseVersion::Sqlserver2017Enterprise
                }
                "SQLSERVER_2017_EXPRESS" => DatabaseInstanceDatabaseVersion::Sqlserver2017Express,
                "SQLSERVER_2017_STANDARD" => DatabaseInstanceDatabaseVersion::Sqlserver2017Standard,
                "SQLSERVER_2017_WEB" => DatabaseInstanceDatabaseVersion::Sqlserver2017Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DatabaseInstanceDatabaseVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstanceDatabaseVersion {
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
    pub struct DatabaseInstanceFailoverReplica {
        #[doc = "The availability status of the failover replica. A false status indicates\nthat the failover replica is out of sync. The master can only failover to\nthe failover replica when the status is true."]
        #[serde(
            rename = "available",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available: ::std::option::Option<bool>,
        #[doc = "The name of the failover replica. If specified at instance creation, a\nfailover replica is created for the instance. The name\ndoesn't include the project ID. This property is applicable only to\nSecond Generation instances."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatabaseInstanceFailoverReplica {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstanceFailoverReplica {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DatabaseInstanceInstanceType {
        #[doc = "A regular Cloud SQL instance."]
        CloudSqlInstance,
        #[doc = "An instance running on the customer's premises that is not managed by\nCloud SQL."]
        OnPremisesInstance,
        #[doc = "A Cloud SQL instance acting as a read-replica."]
        ReadReplicaInstance,
        #[doc = "This is an unknown Cloud SQL instance type."]
        SqlInstanceTypeUnspecified,
    }
    impl DatabaseInstanceInstanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                DatabaseInstanceInstanceType::CloudSqlInstance => "CLOUD_SQL_INSTANCE",
                DatabaseInstanceInstanceType::OnPremisesInstance => "ON_PREMISES_INSTANCE",
                DatabaseInstanceInstanceType::ReadReplicaInstance => "READ_REPLICA_INSTANCE",
                DatabaseInstanceInstanceType::SqlInstanceTypeUnspecified => {
                    "SQL_INSTANCE_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DatabaseInstanceInstanceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DatabaseInstanceInstanceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DatabaseInstanceInstanceType, ()> {
            Ok(match s {
                "CLOUD_SQL_INSTANCE" => DatabaseInstanceInstanceType::CloudSqlInstance,
                "ON_PREMISES_INSTANCE" => DatabaseInstanceInstanceType::OnPremisesInstance,
                "READ_REPLICA_INSTANCE" => DatabaseInstanceInstanceType::ReadReplicaInstance,
                "SQL_INSTANCE_TYPE_UNSPECIFIED" => {
                    DatabaseInstanceInstanceType::SqlInstanceTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DatabaseInstanceInstanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DatabaseInstanceInstanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DatabaseInstanceInstanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLOUD_SQL_INSTANCE" => DatabaseInstanceInstanceType::CloudSqlInstance,
                "ON_PREMISES_INSTANCE" => DatabaseInstanceInstanceType::OnPremisesInstance,
                "READ_REPLICA_INSTANCE" => DatabaseInstanceInstanceType::ReadReplicaInstance,
                "SQL_INSTANCE_TYPE_UNSPECIFIED" => {
                    DatabaseInstanceInstanceType::SqlInstanceTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for DatabaseInstanceInstanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstanceInstanceType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DatabaseInstanceState {
        #[doc = "The instance failed to be created."]
        Failed,
        #[doc = "The instance is down for maintenance."]
        Maintenance,
        #[doc = "The instance is being created."]
        PendingCreate,
        #[doc = "The instance is being deleted."]
        PendingDelete,
        #[doc = "The instance is running."]
        Runnable,
        #[doc = "The state of the instance is unknown."]
        SqlInstanceStateUnspecified,
        #[doc = "The instance is currently offline, but it may run again in the future."]
        Suspended,
    }
    impl DatabaseInstanceState {
        pub fn as_str(self) -> &'static str {
            match self {
                DatabaseInstanceState::Failed => "FAILED",
                DatabaseInstanceState::Maintenance => "MAINTENANCE",
                DatabaseInstanceState::PendingCreate => "PENDING_CREATE",
                DatabaseInstanceState::PendingDelete => "PENDING_DELETE",
                DatabaseInstanceState::Runnable => "RUNNABLE",
                DatabaseInstanceState::SqlInstanceStateUnspecified => {
                    "SQL_INSTANCE_STATE_UNSPECIFIED"
                }
                DatabaseInstanceState::Suspended => "SUSPENDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DatabaseInstanceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DatabaseInstanceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DatabaseInstanceState, ()> {
            Ok(match s {
                "FAILED" => DatabaseInstanceState::Failed,
                "MAINTENANCE" => DatabaseInstanceState::Maintenance,
                "PENDING_CREATE" => DatabaseInstanceState::PendingCreate,
                "PENDING_DELETE" => DatabaseInstanceState::PendingDelete,
                "RUNNABLE" => DatabaseInstanceState::Runnable,
                "SQL_INSTANCE_STATE_UNSPECIFIED" => {
                    DatabaseInstanceState::SqlInstanceStateUnspecified
                }
                "SUSPENDED" => DatabaseInstanceState::Suspended,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DatabaseInstanceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DatabaseInstanceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DatabaseInstanceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FAILED" => DatabaseInstanceState::Failed,
                "MAINTENANCE" => DatabaseInstanceState::Maintenance,
                "PENDING_CREATE" => DatabaseInstanceState::PendingCreate,
                "PENDING_DELETE" => DatabaseInstanceState::PendingDelete,
                "RUNNABLE" => DatabaseInstanceState::Runnable,
                "SQL_INSTANCE_STATE_UNSPECIFIED" => {
                    DatabaseInstanceState::SqlInstanceStateUnspecified
                }
                "SUSPENDED" => DatabaseInstanceState::Suspended,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DatabaseInstanceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstanceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DatabaseInstanceSuspensionReasonItems {
        BillingIssue,
        KmsKeyIssue,
        LegalIssue,
        OperationalIssue,
        SqlSuspensionReasonUnspecified,
    }
    impl DatabaseInstanceSuspensionReasonItems {
        pub fn as_str(self) -> &'static str {
            match self {
                DatabaseInstanceSuspensionReasonItems::BillingIssue => "BILLING_ISSUE",
                DatabaseInstanceSuspensionReasonItems::KmsKeyIssue => "KMS_KEY_ISSUE",
                DatabaseInstanceSuspensionReasonItems::LegalIssue => "LEGAL_ISSUE",
                DatabaseInstanceSuspensionReasonItems::OperationalIssue => "OPERATIONAL_ISSUE",
                DatabaseInstanceSuspensionReasonItems::SqlSuspensionReasonUnspecified => {
                    "SQL_SUSPENSION_REASON_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DatabaseInstanceSuspensionReasonItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DatabaseInstanceSuspensionReasonItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DatabaseInstanceSuspensionReasonItems, ()> {
            Ok(match s {
                "BILLING_ISSUE" => DatabaseInstanceSuspensionReasonItems::BillingIssue,
                "KMS_KEY_ISSUE" => DatabaseInstanceSuspensionReasonItems::KmsKeyIssue,
                "LEGAL_ISSUE" => DatabaseInstanceSuspensionReasonItems::LegalIssue,
                "OPERATIONAL_ISSUE" => DatabaseInstanceSuspensionReasonItems::OperationalIssue,
                "SQL_SUSPENSION_REASON_UNSPECIFIED" => {
                    DatabaseInstanceSuspensionReasonItems::SqlSuspensionReasonUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DatabaseInstanceSuspensionReasonItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DatabaseInstanceSuspensionReasonItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DatabaseInstanceSuspensionReasonItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BILLING_ISSUE" => DatabaseInstanceSuspensionReasonItems::BillingIssue,
                "KMS_KEY_ISSUE" => DatabaseInstanceSuspensionReasonItems::KmsKeyIssue,
                "LEGAL_ISSUE" => DatabaseInstanceSuspensionReasonItems::LegalIssue,
                "OPERATIONAL_ISSUE" => DatabaseInstanceSuspensionReasonItems::OperationalIssue,
                "SQL_SUSPENSION_REASON_UNSPECIFIED" => {
                    DatabaseInstanceSuspensionReasonItems::SqlSuspensionReasonUnspecified
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
    impl ::google_field_selector::FieldSelector for DatabaseInstanceSuspensionReasonItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseInstanceSuspensionReasonItems {
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
    pub struct DatabasesListResponse {
        #[doc = "List of database resources in the instance."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Database>>,
        #[doc = "This is always <code>sql#databasesList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatabasesListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabasesListResponse {
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
    pub struct DemoteMasterConfiguration {
        #[doc = "This is always <code>sql#demoteMasterConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "MySQL specific configuration when replicating from a MySQL on-premises\nmaster. Replication configuration information such as the username,\npassword, certificates, and keys are not stored in the instance metadata.\nThe configuration information is used only to set up the replication\nconnection and is stored by MySQL in a file named <code>master.info</code>\nin the data directory."]
        #[serde(
            rename = "mysqlReplicaConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mysql_replica_configuration:
            ::std::option::Option<crate::schemas::DemoteMasterMySqlReplicaConfiguration>,
    }
    impl ::google_field_selector::FieldSelector for DemoteMasterConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DemoteMasterConfiguration {
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
    pub struct DemoteMasterContext {
        #[doc = "This is always <code>sql#demoteMasterContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the instance which will act as on-premises master in the\nreplication setup."]
        #[serde(
            rename = "masterInstanceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_instance_name: ::std::option::Option<String>,
        #[doc = "Configuration specific to read-replicas replicating from the on-premises\nmaster."]
        #[serde(
            rename = "replicaConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replica_configuration: ::std::option::Option<crate::schemas::DemoteMasterConfiguration>,
        #[doc = "Verify GTID consistency for demote operation. Default value:\n<code>True</code>. Second Generation instances only.  Setting this flag to\nfalse enables you to bypass GTID consistency check between on-premises\nmaster and Cloud SQL instance during the demotion operation but also\nexposes you to the risk of future replication failures. Change the value\nonly if you know the reason for the GTID divergence and are confident that\ndoing so will not cause any replication issues."]
        #[serde(
            rename = "verifyGtidConsistency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verify_gtid_consistency: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DemoteMasterContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DemoteMasterContext {
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
    pub struct DemoteMasterMySqlReplicaConfiguration {
        #[doc = "PEM representation of the trusted CA's x509 certificate."]
        #[serde(
            rename = "caCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ca_certificate: ::std::option::Option<String>,
        #[doc = "PEM representation of the slave's x509 certificate."]
        #[serde(
            rename = "clientCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_certificate: ::std::option::Option<String>,
        #[doc = "PEM representation of the slave's private key. The corresponsing public key\nis encoded in the client's certificate. The format of the slave's private\nkey can be either PKCS #1 or PKCS #8."]
        #[serde(
            rename = "clientKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_key: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#demoteMasterMysqlReplicaConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The password for the replication connection."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "The username for the replication connection."]
        #[serde(
            rename = "username",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub username: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DemoteMasterMySqlReplicaConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DemoteMasterMySqlReplicaConfiguration {
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
    pub struct DiskEncryptionConfiguration {
        #[doc = "This is always <code>sql#diskEncryptionConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Resource name of KMS key for disk encryption"]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DiskEncryptionConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiskEncryptionConfiguration {
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
    pub struct DiskEncryptionStatus {
        #[doc = "This is always <code>sql#diskEncryptionStatus</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "KMS key version used to encrypt the Cloud SQL instance resource"]
        #[serde(
            rename = "kmsKeyVersionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_version_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DiskEncryptionStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiskEncryptionStatus {
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
    pub struct ExportContext {
        #[doc = "Options for exporting data as CSV."]
        #[serde(
            rename = "csvExportOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub csv_export_options:
            ::std::option::Option<crate::schemas::ExportContextCsvExportOptions>,
        #[doc = "Databases to be exported. <br /> <b>MySQL instances:</b> If\n<code>fileType</code> is <code>SQL</code> and no database is specified, all\ndatabases are exported, except for the <code>mysql</code> system database.\nIf <code>fileType</code> is <code>CSV</code>, you can specify one database,\neither by using this property or by using the\n<code>csvExportOptions.selectQuery</code> property, which takes precedence\nover this property. <br /> <b>PostgreSQL instances:</b> You must specify\none database to be exported. If <code>fileType</code> is <code>CSV</code>,\nthis database must match the one specified in the\n<code>csvExportOptions.selectQuery</code> property."]
        #[serde(
            rename = "databases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub databases: ::std::option::Option<Vec<String>>,
        #[doc = "The file type for the specified uri. <br><code>SQL</code>: The file\ncontains SQL statements. <br><code>CSV</code>: The file contains CSV data."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<crate::schemas::ExportContextFileType>,
        #[doc = "This is always <code>sql#exportContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Option for export offload."]
        #[serde(
            rename = "offload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offload: ::std::option::Option<bool>,
        #[doc = "Options for exporting data as SQL statements."]
        #[serde(
            rename = "sqlExportOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sql_export_options:
            ::std::option::Option<crate::schemas::ExportContextSqlExportOptions>,
        #[doc = "The path to the file in Google Cloud Storage where the export will be\nstored. The URI is in the form <code>gs:\n//bucketName/fileName</code>. If the file already exists, the requests\n// succeeds, but the operation fails. If <code>fileType</code> is\n// <code>SQL</code> and the filename ends with .gz, the contents are\n// compressed."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExportContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportContext {
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
    pub struct ExportContextCsvExportOptions {
        #[doc = "The select query used to extract the data."]
        #[serde(
            rename = "selectQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub select_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExportContextCsvExportOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportContextCsvExportOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExportContextFileType {
        Bak,
        #[doc = "File in CSV format."]
        Csv,
        #[doc = "File containing SQL statements."]
        Sql,
        #[doc = "Unknown file type."]
        SqlFileTypeUnspecified,
    }
    impl ExportContextFileType {
        pub fn as_str(self) -> &'static str {
            match self {
                ExportContextFileType::Bak => "BAK",
                ExportContextFileType::Csv => "CSV",
                ExportContextFileType::Sql => "SQL",
                ExportContextFileType::SqlFileTypeUnspecified => "SQL_FILE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ExportContextFileType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExportContextFileType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ExportContextFileType, ()> {
            Ok(match s {
                "BAK" => ExportContextFileType::Bak,
                "CSV" => ExportContextFileType::Csv,
                "SQL" => ExportContextFileType::Sql,
                "SQL_FILE_TYPE_UNSPECIFIED" => ExportContextFileType::SqlFileTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ExportContextFileType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExportContextFileType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExportContextFileType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BAK" => ExportContextFileType::Bak,
                "CSV" => ExportContextFileType::Csv,
                "SQL" => ExportContextFileType::Sql,
                "SQL_FILE_TYPE_UNSPECIFIED" => ExportContextFileType::SqlFileTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ExportContextFileType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportContextFileType {
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
    pub struct ExportContextSqlExportOptions {
        #[doc = "Options for exporting from MySQL."]
        #[serde(
            rename = "mysqlExportOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mysql_export_options:
            ::std::option::Option<crate::schemas::ExportContextSqlExportOptionsMysqlExportOptions>,
        #[doc = "Export only schemas."]
        #[serde(
            rename = "schemaOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_only: ::std::option::Option<bool>,
        #[doc = "Tables to export, or that were exported, from the specified database. If\nyou specify tables, specify one and only one database. For PostgreSQL\ninstances, you can specify only one table."]
        #[serde(
            rename = "tables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tables: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ExportContextSqlExportOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportContextSqlExportOptions {
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
    pub struct ExportContextSqlExportOptionsMysqlExportOptions {
        #[doc = "Option to include SQL statement required to set up replication.\nIf set to <code>1</code>, the dump file includes\na CHANGE MASTER TO statement with the binary log coordinates.\nIf set to <code>2</code>, the CHANGE MASTER TO statement is written as\na SQL comment, and has no effect.\nAll other values are ignored."]
        #[serde(
            rename = "masterData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_data: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ExportContextSqlExportOptionsMysqlExportOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportContextSqlExportOptionsMysqlExportOptions {
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
    pub struct FailoverContext {
        #[doc = "This is always <code>sql#failoverContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The current settings version of this instance. Request will be rejected if\nthis version doesn't match the current settings version."]
        #[serde(
            rename = "settingsVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub settings_version: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for FailoverContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FailoverContext {
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
    pub struct Flag {
        #[doc = "Use this field if only certain integers are accepted. Can be combined\nwith min_value and max_value to add additional values."]
        #[serde(
            rename = "allowedIntValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_int_values: ::std::option::Option<Vec<i64>>,
        #[doc = "For <code>STRING</code> flags, a list of strings that the value can be set\nto."]
        #[serde(
            rename = "allowedStringValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_string_values: ::std::option::Option<Vec<String>>,
        #[doc = "The database version this flag applies to. Can be <code>MYSQL_5_5</code>,\n<code>MYSQL_5_6</code>, or <code>MYSQL_5_7</code>. <code>MYSQL_5_7</code>\nis applicable only to Second Generation instances."]
        #[serde(
            rename = "appliesTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applies_to: ::std::option::Option<Vec<crate::schemas::FlagAppliesToItems>>,
        #[doc = "Whether or not the flag is considered in beta."]
        #[serde(
            rename = "inBeta",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub in_beta: ::std::option::Option<bool>,
        #[doc = "This is always <code>sql#flag</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "For <code>INTEGER</code> flags, the maximum allowed value."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_value: ::std::option::Option<i64>,
        #[doc = "For <code>INTEGER</code> flags, the minimum allowed value."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_value: ::std::option::Option<i64>,
        #[doc = "This is the name of the flag. Flag names always use underscores, not\nhyphens, e.g. <code>max_allowed_packet</code>"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the flag. Flags are typed to being <code>BOOLEAN</code>,\n<code>STRING</code>, <code>INTEGER</code> or <code>NONE</code>.\n<code>NONE</code> is used for flags which do not take a value, such as\n<code>skip_grant_tables</code>."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::FlagType>,
        #[doc = "Indicates whether changing this flag will trigger a database restart. Only\napplicable to Second Generation instances."]
        #[serde(
            rename = "requiresRestart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requires_restart: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Flag {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Flag {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FlagAppliesToItems {
        Mysql51,
        Mysql55,
        Mysql56,
        Mysql57,
        Mysql80,
        Postgres10,
        Postgres11,
        Postgres12,
        Postgres96,
        SqlDatabaseVersionUnspecified,
        Sqlserver2017Enterprise,
        Sqlserver2017Express,
        Sqlserver2017Standard,
        Sqlserver2017Web,
    }
    impl FlagAppliesToItems {
        pub fn as_str(self) -> &'static str {
            match self {
                FlagAppliesToItems::Mysql51 => "MYSQL_5_1",
                FlagAppliesToItems::Mysql55 => "MYSQL_5_5",
                FlagAppliesToItems::Mysql56 => "MYSQL_5_6",
                FlagAppliesToItems::Mysql57 => "MYSQL_5_7",
                FlagAppliesToItems::Mysql80 => "MYSQL_8_0",
                FlagAppliesToItems::Postgres10 => "POSTGRES_10",
                FlagAppliesToItems::Postgres11 => "POSTGRES_11",
                FlagAppliesToItems::Postgres12 => "POSTGRES_12",
                FlagAppliesToItems::Postgres96 => "POSTGRES_9_6",
                FlagAppliesToItems::SqlDatabaseVersionUnspecified => {
                    "SQL_DATABASE_VERSION_UNSPECIFIED"
                }
                FlagAppliesToItems::Sqlserver2017Enterprise => "SQLSERVER_2017_ENTERPRISE",
                FlagAppliesToItems::Sqlserver2017Express => "SQLSERVER_2017_EXPRESS",
                FlagAppliesToItems::Sqlserver2017Standard => "SQLSERVER_2017_STANDARD",
                FlagAppliesToItems::Sqlserver2017Web => "SQLSERVER_2017_WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FlagAppliesToItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FlagAppliesToItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FlagAppliesToItems, ()> {
            Ok(match s {
                "MYSQL_5_1" => FlagAppliesToItems::Mysql51,
                "MYSQL_5_5" => FlagAppliesToItems::Mysql55,
                "MYSQL_5_6" => FlagAppliesToItems::Mysql56,
                "MYSQL_5_7" => FlagAppliesToItems::Mysql57,
                "MYSQL_8_0" => FlagAppliesToItems::Mysql80,
                "POSTGRES_10" => FlagAppliesToItems::Postgres10,
                "POSTGRES_11" => FlagAppliesToItems::Postgres11,
                "POSTGRES_12" => FlagAppliesToItems::Postgres12,
                "POSTGRES_9_6" => FlagAppliesToItems::Postgres96,
                "SQL_DATABASE_VERSION_UNSPECIFIED" => {
                    FlagAppliesToItems::SqlDatabaseVersionUnspecified
                }
                "SQLSERVER_2017_ENTERPRISE" => FlagAppliesToItems::Sqlserver2017Enterprise,
                "SQLSERVER_2017_EXPRESS" => FlagAppliesToItems::Sqlserver2017Express,
                "SQLSERVER_2017_STANDARD" => FlagAppliesToItems::Sqlserver2017Standard,
                "SQLSERVER_2017_WEB" => FlagAppliesToItems::Sqlserver2017Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FlagAppliesToItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FlagAppliesToItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FlagAppliesToItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MYSQL_5_1" => FlagAppliesToItems::Mysql51,
                "MYSQL_5_5" => FlagAppliesToItems::Mysql55,
                "MYSQL_5_6" => FlagAppliesToItems::Mysql56,
                "MYSQL_5_7" => FlagAppliesToItems::Mysql57,
                "MYSQL_8_0" => FlagAppliesToItems::Mysql80,
                "POSTGRES_10" => FlagAppliesToItems::Postgres10,
                "POSTGRES_11" => FlagAppliesToItems::Postgres11,
                "POSTGRES_12" => FlagAppliesToItems::Postgres12,
                "POSTGRES_9_6" => FlagAppliesToItems::Postgres96,
                "SQL_DATABASE_VERSION_UNSPECIFIED" => {
                    FlagAppliesToItems::SqlDatabaseVersionUnspecified
                }
                "SQLSERVER_2017_ENTERPRISE" => FlagAppliesToItems::Sqlserver2017Enterprise,
                "SQLSERVER_2017_EXPRESS" => FlagAppliesToItems::Sqlserver2017Express,
                "SQLSERVER_2017_STANDARD" => FlagAppliesToItems::Sqlserver2017Standard,
                "SQLSERVER_2017_WEB" => FlagAppliesToItems::Sqlserver2017Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FlagAppliesToItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FlagAppliesToItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FlagType {
        #[doc = "Boolean type flag."]
        Boolean,
        #[doc = "Float type flag."]
        Float,
        #[doc = "Integer type flag."]
        Integer,
        #[doc = "Type introduced specically for MySQL TimeZone offset. Accept a string value\nwith the format [-12:59, 13:00]."]
        MysqlTimezoneOffset,
        #[doc = "Flag type used for a server startup option."]
        None,
        #[doc = "Comma-separated list of the strings in a SqlFlagType enum."]
        RepeatedString,
        #[doc = "This is an unknown flag type."]
        SqlFlagTypeUnspecified,
        #[doc = "String type flag."]
        String,
    }
    impl FlagType {
        pub fn as_str(self) -> &'static str {
            match self {
                FlagType::Boolean => "BOOLEAN",
                FlagType::Float => "FLOAT",
                FlagType::Integer => "INTEGER",
                FlagType::MysqlTimezoneOffset => "MYSQL_TIMEZONE_OFFSET",
                FlagType::None => "NONE",
                FlagType::RepeatedString => "REPEATED_STRING",
                FlagType::SqlFlagTypeUnspecified => "SQL_FLAG_TYPE_UNSPECIFIED",
                FlagType::String => "STRING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FlagType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FlagType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FlagType, ()> {
            Ok(match s {
                "BOOLEAN" => FlagType::Boolean,
                "FLOAT" => FlagType::Float,
                "INTEGER" => FlagType::Integer,
                "MYSQL_TIMEZONE_OFFSET" => FlagType::MysqlTimezoneOffset,
                "NONE" => FlagType::None,
                "REPEATED_STRING" => FlagType::RepeatedString,
                "SQL_FLAG_TYPE_UNSPECIFIED" => FlagType::SqlFlagTypeUnspecified,
                "STRING" => FlagType::String,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FlagType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FlagType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FlagType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOLEAN" => FlagType::Boolean,
                "FLOAT" => FlagType::Float,
                "INTEGER" => FlagType::Integer,
                "MYSQL_TIMEZONE_OFFSET" => FlagType::MysqlTimezoneOffset,
                "NONE" => FlagType::None,
                "REPEATED_STRING" => FlagType::RepeatedString,
                "SQL_FLAG_TYPE_UNSPECIFIED" => FlagType::SqlFlagTypeUnspecified,
                "STRING" => FlagType::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FlagType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FlagType {
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
    pub struct FlagsListResponse {
        #[doc = "List of flags."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Flag>>,
        #[doc = "This is always <code>sql#flagsList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FlagsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FlagsListResponse {
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
    pub struct ImportContext {
        #[doc = "Import parameters specific to SQL Server .BAK files"]
        #[serde(
            rename = "bakImportOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bak_import_options:
            ::std::option::Option<crate::schemas::ImportContextBakImportOptions>,
        #[doc = "Options for importing data as CSV."]
        #[serde(
            rename = "csvImportOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub csv_import_options:
            ::std::option::Option<crate::schemas::ImportContextCsvImportOptions>,
        #[doc = "The target database for the import. If <code>fileType</code> is\n<code>SQL</code>, this field is required only if the import file does not\nspecify a database, and is overridden by any database specification in the\nimport file. If <code>fileType</code> is <code>CSV</code>, one database\nmust be specified."]
        #[serde(
            rename = "database",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub database: ::std::option::Option<String>,
        #[doc = "The file type for the specified uri. <br><code>SQL</code>: The file\ncontains SQL statements. <br><code>CSV</code>: The file contains CSV data."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<crate::schemas::ImportContextFileType>,
        #[doc = "The PostgreSQL user for this import operation. PostgreSQL instances only."]
        #[serde(
            rename = "importUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_user: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#importContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Path to the import file in Cloud Storage, in the form\n<code>gs:\n//bucketName/fileName</code>. Compressed gzip files (.gz) are supported\n// when <code>fileType</code> is <code>SQL</code>. The instance must have\n// write permissions to the bucket and read access to the file."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ImportContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportContext {
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
    pub struct ImportContextBakImportOptions {
        #[serde(
            rename = "encryptionOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_options:
            ::std::option::Option<crate::schemas::ImportContextBakImportOptionsEncryptionOptions>,
    }
    impl ::google_field_selector::FieldSelector for ImportContextBakImportOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportContextBakImportOptions {
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
    pub struct ImportContextBakImportOptionsEncryptionOptions {
        #[doc = "Path to the Certificate (.cer) in Cloud Storage, in the form\n<code>gs://bucketName/fileName</code>. The instance must have\nwrite permissions to the bucket and read access to the file."]
        #[serde(
            rename = "certPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert_path: ::std::option::Option<String>,
        #[doc = "Password that encrypts the private key"]
        #[serde(
            rename = "pvkPassword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pvk_password: ::std::option::Option<String>,
        #[doc = "Path to the Certificate Private Key (.pvk)  in Cloud Storage, in the\nform <code>gs://bucketName/fileName</code>. The instance must have\nwrite permissions to the bucket and read access to the file."]
        #[serde(
            rename = "pvkPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pvk_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ImportContextBakImportOptionsEncryptionOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportContextBakImportOptionsEncryptionOptions {
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
    pub struct ImportContextCsvImportOptions {
        #[doc = "The columns to which CSV data is imported. If not specified, all columns\nof the database table are loaded with CSV data."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<Vec<String>>,
        #[doc = "The table to which CSV data is imported."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ImportContextCsvImportOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportContextCsvImportOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImportContextFileType {
        Bak,
        #[doc = "File in CSV format."]
        Csv,
        #[doc = "File containing SQL statements."]
        Sql,
        #[doc = "Unknown file type."]
        SqlFileTypeUnspecified,
    }
    impl ImportContextFileType {
        pub fn as_str(self) -> &'static str {
            match self {
                ImportContextFileType::Bak => "BAK",
                ImportContextFileType::Csv => "CSV",
                ImportContextFileType::Sql => "SQL",
                ImportContextFileType::SqlFileTypeUnspecified => "SQL_FILE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImportContextFileType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImportContextFileType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImportContextFileType, ()> {
            Ok(match s {
                "BAK" => ImportContextFileType::Bak,
                "CSV" => ImportContextFileType::Csv,
                "SQL" => ImportContextFileType::Sql,
                "SQL_FILE_TYPE_UNSPECIFIED" => ImportContextFileType::SqlFileTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImportContextFileType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImportContextFileType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImportContextFileType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BAK" => ImportContextFileType::Bak,
                "CSV" => ImportContextFileType::Csv,
                "SQL" => ImportContextFileType::Sql,
                "SQL_FILE_TYPE_UNSPECIFIED" => ImportContextFileType::SqlFileTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImportContextFileType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportContextFileType {
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
    pub struct InstancesCloneRequest {
        #[doc = "Contains details about the clone operation."]
        #[serde(
            rename = "cloneContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clone_context: ::std::option::Option<crate::schemas::CloneContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesCloneRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesCloneRequest {
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
    pub struct InstancesDemoteMasterRequest {
        #[doc = "Contains details about the demoteMaster operation."]
        #[serde(
            rename = "demoteMasterContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub demote_master_context: ::std::option::Option<crate::schemas::DemoteMasterContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesDemoteMasterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesDemoteMasterRequest {
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
    pub struct InstancesExportRequest {
        #[doc = "Contains details about the export operation."]
        #[serde(
            rename = "exportContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub export_context: ::std::option::Option<crate::schemas::ExportContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesExportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesExportRequest {
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
    pub struct InstancesFailoverRequest {
        #[doc = "Failover Context."]
        #[serde(
            rename = "failoverContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failover_context: ::std::option::Option<crate::schemas::FailoverContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesFailoverRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesFailoverRequest {
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
    pub struct InstancesImportRequest {
        #[doc = "Contains details about the import operation."]
        #[serde(
            rename = "importContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_context: ::std::option::Option<crate::schemas::ImportContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesImportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesImportRequest {
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
    pub struct InstancesListResponse {
        #[doc = "List of database instance resources."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::DatabaseInstance>>,
        #[doc = "This is always <code>sql#instancesList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The continuation token, used to page through large result sets. Provide\nthis value in a subsequent request to return the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of warnings that occurred while handling the request."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<Vec<crate::schemas::ApiWarning>>,
    }
    impl ::google_field_selector::FieldSelector for InstancesListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesListResponse {
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
    pub struct InstancesListServerCasResponse {
        #[serde(
            rename = "activeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub active_version: ::std::option::Option<String>,
        #[doc = "List of server CA certificates for the instance."]
        #[serde(
            rename = "certs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub certs: ::std::option::Option<Vec<crate::schemas::SslCert>>,
        #[doc = "This is always <code>sql#instancesListServerCas</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InstancesListServerCasResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesListServerCasResponse {
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
    pub struct InstancesRestoreBackupRequest {
        #[doc = "Parameters required to perform the restore backup operation."]
        #[serde(
            rename = "restoreBackupContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restore_backup_context: ::std::option::Option<crate::schemas::RestoreBackupContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesRestoreBackupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesRestoreBackupRequest {
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
    pub struct InstancesRotateServerCaRequest {
        #[doc = "Contains details about the rotate server CA operation."]
        #[serde(
            rename = "rotateServerCaContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotate_server_ca_context: ::std::option::Option<crate::schemas::RotateServerCaContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesRotateServerCaRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesRotateServerCaRequest {
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
    pub struct InstancesTruncateLogRequest {
        #[doc = "Contains details about the truncate log operation."]
        #[serde(
            rename = "truncateLogContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub truncate_log_context: ::std::option::Option<crate::schemas::TruncateLogContext>,
    }
    impl ::google_field_selector::FieldSelector for InstancesTruncateLogRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstancesTruncateLogRequest {
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
    pub struct IpConfiguration {
        #[doc = "The list of external networks that are allowed to connect to the instance\nusing the IP. In <a\nhref=\"http://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation\">CIDR\nnotation</a>, also known as 'slash' notation (e.g.\n<code>192.168.100.0/24</code>)."]
        #[serde(
            rename = "authorizedNetworks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorized_networks: ::std::option::Option<Vec<crate::schemas::AclEntry>>,
        #[doc = "Whether the instance should be assigned an IP address or not."]
        #[serde(
            rename = "ipv4Enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ipv_4_enabled: ::std::option::Option<bool>,
        #[doc = "The resource link for the VPC network from which the Cloud SQL instance is\naccessible for private IP. For example,\n<code>/projects/myProject/global/networks/default</code>. This setting can\nbe updated, but it cannot be removed after it is set."]
        #[serde(
            rename = "privateNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_network: ::std::option::Option<String>,
        #[doc = "Whether SSL connections over IP should be enforced or not."]
        #[serde(
            rename = "requireSsl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_ssl: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for IpConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IpConfiguration {
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
    pub struct IpMapping {
        #[doc = "The IP address assigned."]
        #[serde(
            rename = "ipAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_address: ::std::option::Option<String>,
        #[doc = "The type of this IP address. A <code>PRIMARY</code> address is a public\naddress that can accept incoming connections. A <code>PRIVATE</code>\naddress is a private address that can accept incoming connections. An\n<code>OUTGOING</code> address is the source address of connections\noriginating from the instance, if supported."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::IpMappingType>,
        #[doc = "The due time for this IP to be retired in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>. This field is only available when\nthe IP is scheduled to be retired."]
        #[serde(
            rename = "timeToRetire",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_to_retire: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IpMapping {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IpMapping {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IpMappingType {
        #[doc = "V1 IP of a migrated instance. We want the user to\ndecommission this IP as soon as the migration is complete.\nNote: V1 instances with V1 ip addresses will be counted as PRIMARY."]
        Migrated1StGen,
        #[doc = "Source IP address of the connection a read replica establishes to its\nexternal master. This IP address can be whitelisted by the customer\nin case it has a firewall that filters incoming connection to its\non premises master."]
        Outgoing,
        #[doc = "IP address the customer is supposed to connect to. Usually this is the\nload balancer's IP address"]
        Primary,
        #[doc = "Private IP used when using private IPs and network peering."]
        Private,
        #[doc = "This is an unknown IP address type."]
        SqlIpAddressTypeUnspecified,
    }
    impl IpMappingType {
        pub fn as_str(self) -> &'static str {
            match self {
                IpMappingType::Migrated1StGen => "MIGRATED_1ST_GEN",
                IpMappingType::Outgoing => "OUTGOING",
                IpMappingType::Primary => "PRIMARY",
                IpMappingType::Private => "PRIVATE",
                IpMappingType::SqlIpAddressTypeUnspecified => "SQL_IP_ADDRESS_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IpMappingType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IpMappingType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IpMappingType, ()> {
            Ok(match s {
                "MIGRATED_1ST_GEN" => IpMappingType::Migrated1StGen,
                "OUTGOING" => IpMappingType::Outgoing,
                "PRIMARY" => IpMappingType::Primary,
                "PRIVATE" => IpMappingType::Private,
                "SQL_IP_ADDRESS_TYPE_UNSPECIFIED" => IpMappingType::SqlIpAddressTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IpMappingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IpMappingType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IpMappingType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MIGRATED_1ST_GEN" => IpMappingType::Migrated1StGen,
                "OUTGOING" => IpMappingType::Outgoing,
                "PRIMARY" => IpMappingType::Primary,
                "PRIVATE" => IpMappingType::Private,
                "SQL_IP_ADDRESS_TYPE_UNSPECIFIED" => IpMappingType::SqlIpAddressTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IpMappingType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IpMappingType {
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
    pub struct LocationPreference {
        #[doc = "The AppEngine application to follow, it must be in the same region as the\nCloud SQL instance."]
        #[serde(
            rename = "followGaeApplication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub follow_gae_application: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#locationPreference</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The preferred Compute Engine zone (e.g. us-central1-a, us-central1-b,\netc.)."]
        #[serde(
            rename = "zone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LocationPreference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocationPreference {
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
    pub struct MaintenanceWindow {
        #[doc = "day of week (1-7), starting on Monday."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "hour of day - 0 to 23."]
        #[serde(
            rename = "hour",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hour: ::std::option::Option<i32>,
        #[doc = "This is always <code>sql#maintenanceWindow</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Maintenance timing setting: <code>canary</code> (Earlier) or\n<code>stable</code> (Later). <br /><a\nhref=\"/sql/docs/db_path/instance-settings#maintenance-timing-2ndgen\">\nLearn more</a>."]
        #[serde(
            rename = "updateTrack",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_track: ::std::option::Option<crate::schemas::MaintenanceWindowUpdateTrack>,
    }
    impl ::google_field_selector::FieldSelector for MaintenanceWindow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MaintenanceWindow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MaintenanceWindowUpdateTrack {
        #[doc = "For instance update that requires a restart, this update track indicates\nyour instance prefer to restart for new version early in maintenance\nwindow."]
        Canary,
        #[doc = "This is an unknown maintenance timing preference."]
        SqlUpdateTrackUnspecified,
        #[doc = "For instance update that requires a restart, this update track indicates\nyour instance prefer to let Cloud SQL choose the timing of restart (within\nits Maintenance window, if applicable)."]
        Stable,
    }
    impl MaintenanceWindowUpdateTrack {
        pub fn as_str(self) -> &'static str {
            match self {
                MaintenanceWindowUpdateTrack::Canary => "canary",
                MaintenanceWindowUpdateTrack::SqlUpdateTrackUnspecified => {
                    "SQL_UPDATE_TRACK_UNSPECIFIED"
                }
                MaintenanceWindowUpdateTrack::Stable => "stable",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MaintenanceWindowUpdateTrack {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MaintenanceWindowUpdateTrack {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MaintenanceWindowUpdateTrack, ()> {
            Ok(match s {
                "canary" => MaintenanceWindowUpdateTrack::Canary,
                "SQL_UPDATE_TRACK_UNSPECIFIED" => {
                    MaintenanceWindowUpdateTrack::SqlUpdateTrackUnspecified
                }
                "stable" => MaintenanceWindowUpdateTrack::Stable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MaintenanceWindowUpdateTrack {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MaintenanceWindowUpdateTrack {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MaintenanceWindowUpdateTrack {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "canary" => MaintenanceWindowUpdateTrack::Canary,
                "SQL_UPDATE_TRACK_UNSPECIFIED" => {
                    MaintenanceWindowUpdateTrack::SqlUpdateTrackUnspecified
                }
                "stable" => MaintenanceWindowUpdateTrack::Stable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MaintenanceWindowUpdateTrack {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MaintenanceWindowUpdateTrack {
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
    pub struct MySqlReplicaConfiguration {
        #[doc = "PEM representation of the trusted CA's x509 certificate."]
        #[serde(
            rename = "caCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ca_certificate: ::std::option::Option<String>,
        #[doc = "PEM representation of the slave's x509 certificate."]
        #[serde(
            rename = "clientCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_certificate: ::std::option::Option<String>,
        #[doc = "PEM representation of the slave's private key. The corresponsing public key\nis encoded in the client's certificate."]
        #[serde(
            rename = "clientKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_key: ::std::option::Option<String>,
        #[doc = "Seconds to wait between connect retries. MySQL's default is 60 seconds."]
        #[serde(
            rename = "connectRetryInterval",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connect_retry_interval: ::std::option::Option<i32>,
        #[doc = "Path to a SQL dump file in Google Cloud Storage from which the slave\ninstance is to be created. The URI is in the form gs:\n//bucketName/fileName. Compressed gzip files (.gz) are also supported.\n// Dumps should have the binlog co-ordinates from which replication should\n// begin. This can be accomplished by setting --master-data to 1 when using\n// mysqldump."]
        #[serde(
            rename = "dumpFilePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dump_file_path: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#mysqlReplicaConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Interval in milliseconds between replication heartbeats."]
        #[serde(
            rename = "masterHeartbeatPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub master_heartbeat_period: ::std::option::Option<i64>,
        #[doc = "The password for the replication connection."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "A list of permissible ciphers to use for SSL encryption."]
        #[serde(
            rename = "sslCipher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ssl_cipher: ::std::option::Option<String>,
        #[doc = "The username for the replication connection."]
        #[serde(
            rename = "username",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub username: ::std::option::Option<String>,
        #[doc = "Whether or not to check the master's Common Name value in the certificate\nthat it sends during the SSL handshake."]
        #[serde(
            rename = "verifyServerCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verify_server_certificate: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for MySqlReplicaConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MySqlReplicaConfiguration {
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
    pub struct OnPremisesConfiguration {
        #[doc = "PEM representation of the trusted CA's x509 certificate."]
        #[serde(
            rename = "caCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ca_certificate: ::std::option::Option<String>,
        #[doc = "PEM representation of the slave's x509 certificate."]
        #[serde(
            rename = "clientCertificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_certificate: ::std::option::Option<String>,
        #[doc = "PEM representation of the slave's private key. The corresponsing public key\nis encoded in the client's certificate."]
        #[serde(
            rename = "clientKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_key: ::std::option::Option<String>,
        #[doc = "The dump file to create the Cloud SQL replica."]
        #[serde(
            rename = "dumpFilePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dump_file_path: ::std::option::Option<String>,
        #[doc = "The host and port of the on-premises instance in host:port format"]
        #[serde(
            rename = "hostPort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_port: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#onPremisesConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The password for connecting to on-premises instance."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "The username for connecting to on-premises instance."]
        #[serde(
            rename = "username",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub username: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OnPremisesConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OnPremisesConfiguration {
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
    pub struct Operation {
        #[doc = "The time this operation finished in UTC timezone in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "If errors occurred during processing of this operation, this field will be\npopulated."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::OperationErrors>,
        #[doc = "The context for export operation, if applicable."]
        #[serde(
            rename = "exportContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub export_context: ::std::option::Option<crate::schemas::ExportContext>,
        #[doc = "The context for import operation, if applicable."]
        #[serde(
            rename = "importContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_context: ::std::option::Option<crate::schemas::ImportContext>,
        #[doc = "The time this operation was enqueued in UTC timezone in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "insertTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_time: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#operation</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "An identifier that uniquely identifies the operation. You can use this\nidentifier to retrieve the Operations resource that has information about\nthe operation."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the operation. Valid values are <code>CREATE</code>,\n<code>DELETE</code>, <code>UPDATE</code>, <code>RESTART</code>,\n<code>IMPORT</code>, <code>EXPORT</code>, <code>BACKUP_VOLUME</code>,\n<code>RESTORE_VOLUME</code>, <code>CREATE_USER</code>,\n<code>DELETE_USER</code>, <code>CREATE_DATABASE</code>,\n<code>DELETE_DATABASE</code> ."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<crate::schemas::OperationOperationType>,
        #[doc = "The URI of this resource."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "The time this operation actually started in UTC timezone in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The status of an operation. Valid values are <code>PENDING</code>,\n<code>RUNNING</code>, <code>DONE</code>,\n<code>SQL_OPERATION_STATUS_UNSPECIFIED</code>."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::OperationStatus>,
        #[doc = "Name of the database instance related to this operation."]
        #[serde(
            rename = "targetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_id: ::std::option::Option<String>,
        #[serde(
            rename = "targetLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_link: ::std::option::Option<String>,
        #[doc = "The project ID of the target instance related to this operation."]
        #[serde(
            rename = "targetProject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_project: ::std::option::Option<String>,
        #[doc = "The email address of the user who initiated this operation."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<String>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationOperationType {
        Backup,
        #[doc = "Performs instance backup."]
        BackupVolume,
        #[doc = "Clones a Cloud SQL instance."]
        Clone,
        #[doc = "Creates a new Cloud SQL instance."]
        Create,
        #[doc = "Creates clone instance."]
        CreateClone,
        #[doc = "Creates a database in the Cloud SQL instance."]
        CreateDatabase,
        #[doc = "Creates a Cloud SQL replica instance."]
        CreateReplica,
        #[doc = "Creates a new user in a Cloud SQL instance."]
        CreateUser,
        DeferMaintenance,
        #[doc = "Deletes a Cloud SQL instance."]
        Delete,
        #[doc = "Deletes the backup taken by a backup run."]
        DeleteBackup,
        #[doc = "Deletes a database in the Cloud SQL instance."]
        DeleteDatabase,
        #[doc = "Deletes a user from a Cloud SQL instance."]
        DeleteUser,
        #[doc = "Deletes an instance backup."]
        DeleteVolume,
        #[doc = "Demotes the stand-alone instance to be a Cloud SQL\nread replica for an external database server."]
        DemoteMaster,
        #[doc = "This field is deprecated, and will be removed in future version of API."]
        EnablePrivateIp,
        #[doc = "Exports data from a Cloud SQL instance to a Cloud Storage\nbucket."]
        Export,
        #[doc = "Performs failover of an HA-enabled Cloud SQL\nfailover replica."]
        Failover,
        #[doc = "Imports data into a Cloud SQL instance."]
        Import,
        #[doc = "Injects a privileged user in mysql for MOB instances."]
        InjectUser,
        #[doc = "Indicates that the instance is currently in maintenance. Maintenance\ntypically causes the instance to be unavailable for 1-3 minutes."]
        Maintenance,
        #[doc = "Promotes a Cloud SQL replica instance."]
        PromoteReplica,
        RecreateReplica,
        #[doc = "Reschedule maintenance to another time."]
        RescheduleMaintenance,
        #[doc = "Restarts the Cloud SQL instance."]
        Restart,
        #[doc = "Restores an instance backup."]
        RestoreVolume,
        Snapshot,
        #[doc = "Unknown operation type."]
        SqlOperationTypeUnspecified,
        #[doc = "Starts external sync of a Cloud SQL EM replica to an external master."]
        StartExternalSync,
        #[doc = "Starts replication on a Cloud SQL read replica instance."]
        StartReplica,
        #[doc = "Stops replication on a Cloud SQL read replica instance."]
        StopReplica,
        #[doc = "Truncates a general or slow log table in MySQL."]
        TruncateLog,
        #[doc = "Updates the settings of a Cloud SQL instance."]
        Update,
        #[doc = "Updates a database in the Cloud SQL instance."]
        UpdateDatabase,
        #[doc = "Updates an existing user in a Cloud SQL instance."]
        UpdateUser,
    }
    impl OperationOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationOperationType::Backup => "BACKUP",
                OperationOperationType::BackupVolume => "BACKUP_VOLUME",
                OperationOperationType::Clone => "CLONE",
                OperationOperationType::Create => "CREATE",
                OperationOperationType::CreateClone => "CREATE_CLONE",
                OperationOperationType::CreateDatabase => "CREATE_DATABASE",
                OperationOperationType::CreateReplica => "CREATE_REPLICA",
                OperationOperationType::CreateUser => "CREATE_USER",
                OperationOperationType::DeferMaintenance => "DEFER_MAINTENANCE",
                OperationOperationType::Delete => "DELETE",
                OperationOperationType::DeleteBackup => "DELETE_BACKUP",
                OperationOperationType::DeleteDatabase => "DELETE_DATABASE",
                OperationOperationType::DeleteUser => "DELETE_USER",
                OperationOperationType::DeleteVolume => "DELETE_VOLUME",
                OperationOperationType::DemoteMaster => "DEMOTE_MASTER",
                OperationOperationType::EnablePrivateIp => "ENABLE_PRIVATE_IP",
                OperationOperationType::Export => "EXPORT",
                OperationOperationType::Failover => "FAILOVER",
                OperationOperationType::Import => "IMPORT",
                OperationOperationType::InjectUser => "INJECT_USER",
                OperationOperationType::Maintenance => "MAINTENANCE",
                OperationOperationType::PromoteReplica => "PROMOTE_REPLICA",
                OperationOperationType::RecreateReplica => "RECREATE_REPLICA",
                OperationOperationType::RescheduleMaintenance => "RESCHEDULE_MAINTENANCE",
                OperationOperationType::Restart => "RESTART",
                OperationOperationType::RestoreVolume => "RESTORE_VOLUME",
                OperationOperationType::Snapshot => "SNAPSHOT",
                OperationOperationType::SqlOperationTypeUnspecified => {
                    "SQL_OPERATION_TYPE_UNSPECIFIED"
                }
                OperationOperationType::StartExternalSync => "START_EXTERNAL_SYNC",
                OperationOperationType::StartReplica => "START_REPLICA",
                OperationOperationType::StopReplica => "STOP_REPLICA",
                OperationOperationType::TruncateLog => "TRUNCATE_LOG",
                OperationOperationType::Update => "UPDATE",
                OperationOperationType::UpdateDatabase => "UPDATE_DATABASE",
                OperationOperationType::UpdateUser => "UPDATE_USER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OperationOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OperationOperationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OperationOperationType, ()> {
            Ok(match s {
                "BACKUP" => OperationOperationType::Backup,
                "BACKUP_VOLUME" => OperationOperationType::BackupVolume,
                "CLONE" => OperationOperationType::Clone,
                "CREATE" => OperationOperationType::Create,
                "CREATE_CLONE" => OperationOperationType::CreateClone,
                "CREATE_DATABASE" => OperationOperationType::CreateDatabase,
                "CREATE_REPLICA" => OperationOperationType::CreateReplica,
                "CREATE_USER" => OperationOperationType::CreateUser,
                "DEFER_MAINTENANCE" => OperationOperationType::DeferMaintenance,
                "DELETE" => OperationOperationType::Delete,
                "DELETE_BACKUP" => OperationOperationType::DeleteBackup,
                "DELETE_DATABASE" => OperationOperationType::DeleteDatabase,
                "DELETE_USER" => OperationOperationType::DeleteUser,
                "DELETE_VOLUME" => OperationOperationType::DeleteVolume,
                "DEMOTE_MASTER" => OperationOperationType::DemoteMaster,
                "ENABLE_PRIVATE_IP" => OperationOperationType::EnablePrivateIp,
                "EXPORT" => OperationOperationType::Export,
                "FAILOVER" => OperationOperationType::Failover,
                "IMPORT" => OperationOperationType::Import,
                "INJECT_USER" => OperationOperationType::InjectUser,
                "MAINTENANCE" => OperationOperationType::Maintenance,
                "PROMOTE_REPLICA" => OperationOperationType::PromoteReplica,
                "RECREATE_REPLICA" => OperationOperationType::RecreateReplica,
                "RESCHEDULE_MAINTENANCE" => OperationOperationType::RescheduleMaintenance,
                "RESTART" => OperationOperationType::Restart,
                "RESTORE_VOLUME" => OperationOperationType::RestoreVolume,
                "SNAPSHOT" => OperationOperationType::Snapshot,
                "SQL_OPERATION_TYPE_UNSPECIFIED" => {
                    OperationOperationType::SqlOperationTypeUnspecified
                }
                "START_EXTERNAL_SYNC" => OperationOperationType::StartExternalSync,
                "START_REPLICA" => OperationOperationType::StartReplica,
                "STOP_REPLICA" => OperationOperationType::StopReplica,
                "TRUNCATE_LOG" => OperationOperationType::TruncateLog,
                "UPDATE" => OperationOperationType::Update,
                "UPDATE_DATABASE" => OperationOperationType::UpdateDatabase,
                "UPDATE_USER" => OperationOperationType::UpdateUser,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OperationOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BACKUP" => OperationOperationType::Backup,
                "BACKUP_VOLUME" => OperationOperationType::BackupVolume,
                "CLONE" => OperationOperationType::Clone,
                "CREATE" => OperationOperationType::Create,
                "CREATE_CLONE" => OperationOperationType::CreateClone,
                "CREATE_DATABASE" => OperationOperationType::CreateDatabase,
                "CREATE_REPLICA" => OperationOperationType::CreateReplica,
                "CREATE_USER" => OperationOperationType::CreateUser,
                "DEFER_MAINTENANCE" => OperationOperationType::DeferMaintenance,
                "DELETE" => OperationOperationType::Delete,
                "DELETE_BACKUP" => OperationOperationType::DeleteBackup,
                "DELETE_DATABASE" => OperationOperationType::DeleteDatabase,
                "DELETE_USER" => OperationOperationType::DeleteUser,
                "DELETE_VOLUME" => OperationOperationType::DeleteVolume,
                "DEMOTE_MASTER" => OperationOperationType::DemoteMaster,
                "ENABLE_PRIVATE_IP" => OperationOperationType::EnablePrivateIp,
                "EXPORT" => OperationOperationType::Export,
                "FAILOVER" => OperationOperationType::Failover,
                "IMPORT" => OperationOperationType::Import,
                "INJECT_USER" => OperationOperationType::InjectUser,
                "MAINTENANCE" => OperationOperationType::Maintenance,
                "PROMOTE_REPLICA" => OperationOperationType::PromoteReplica,
                "RECREATE_REPLICA" => OperationOperationType::RecreateReplica,
                "RESCHEDULE_MAINTENANCE" => OperationOperationType::RescheduleMaintenance,
                "RESTART" => OperationOperationType::Restart,
                "RESTORE_VOLUME" => OperationOperationType::RestoreVolume,
                "SNAPSHOT" => OperationOperationType::Snapshot,
                "SQL_OPERATION_TYPE_UNSPECIFIED" => {
                    OperationOperationType::SqlOperationTypeUnspecified
                }
                "START_EXTERNAL_SYNC" => OperationOperationType::StartExternalSync,
                "START_REPLICA" => OperationOperationType::StartReplica,
                "STOP_REPLICA" => OperationOperationType::StopReplica,
                "TRUNCATE_LOG" => OperationOperationType::TruncateLog,
                "UPDATE" => OperationOperationType::Update,
                "UPDATE_DATABASE" => OperationOperationType::UpdateDatabase,
                "UPDATE_USER" => OperationOperationType::UpdateUser,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OperationOperationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationOperationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationStatus {
        #[doc = "The operation completed."]
        Done,
        #[doc = "The operation has been queued, but has not started yet."]
        Pending,
        #[doc = "The operation is running."]
        Running,
        #[doc = "The state of the operation is unknown."]
        SqlOperationStatusUnspecified,
    }
    impl OperationStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationStatus::Done => "DONE",
                OperationStatus::Pending => "PENDING",
                OperationStatus::Running => "RUNNING",
                OperationStatus::SqlOperationStatusUnspecified => {
                    "SQL_OPERATION_STATUS_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for OperationStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OperationStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OperationStatus, ()> {
            Ok(match s {
                "DONE" => OperationStatus::Done,
                "PENDING" => OperationStatus::Pending,
                "RUNNING" => OperationStatus::Running,
                "SQL_OPERATION_STATUS_UNSPECIFIED" => {
                    OperationStatus::SqlOperationStatusUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OperationStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DONE" => OperationStatus::Done,
                "PENDING" => OperationStatus::Pending,
                "RUNNING" => OperationStatus::Running,
                "SQL_OPERATION_STATUS_UNSPECIFIED" => {
                    OperationStatus::SqlOperationStatusUnspecified
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
    impl ::google_field_selector::FieldSelector for OperationStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationStatus {
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
    pub struct OperationError {
        #[doc = "Identifies the specific error that occurred."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#operationError</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Additional information about the error encountered."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OperationError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationError {
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
    pub struct OperationErrors {
        #[doc = "The list of errors encountered while processing this operation."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::OperationError>>,
        #[doc = "This is always <code>sql#operationErrors</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OperationErrors {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationErrors {
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
    pub struct OperationsListResponse {
        #[doc = "List of operation resources."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Operation>>,
        #[doc = "This is always <code>sql#operationsList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The continuation token, used to page through large result sets. Provide\nthis value in a subsequent request to return the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OperationsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationsListResponse {
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
    pub struct ReplicaConfiguration {
        #[doc = "Specifies if the replica is the failover target. If the field is set to\n<code>true</code> the replica will be designated as a failover replica. In\ncase the master instance fails, the replica instance will be promoted as\nthe new master instance.  <p>Only one replica can be specified as failover\ntarget, and the replica has to be in different zone with the master\ninstance."]
        #[serde(
            rename = "failoverTarget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failover_target: ::std::option::Option<bool>,
        #[doc = "This is always <code>sql#replicaConfiguration</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "MySQL specific configuration when replicating from a MySQL on-premises\nmaster. Replication configuration information such as the username,\npassword, certificates, and keys are not stored in the instance metadata.\nThe configuration information is used only to set up the replication\nconnection and is stored by MySQL in a file named <code>master.info</code>\nin the data directory."]
        #[serde(
            rename = "mysqlReplicaConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mysql_replica_configuration:
            ::std::option::Option<crate::schemas::MySqlReplicaConfiguration>,
    }
    impl ::google_field_selector::FieldSelector for ReplicaConfiguration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplicaConfiguration {
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
    pub struct Reschedule {
        #[doc = "Required. The type of the reschedule."]
        #[serde(
            rename = "rescheduleType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reschedule_type: ::std::option::Option<crate::schemas::RescheduleRescheduleType>,
        #[doc = "Optional. Timestamp when the maintenance shall be rescheduled to if\nreschedule_type=SPECIFIC_TIME, in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for\nexample <code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "scheduleTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schedule_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Reschedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Reschedule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RescheduleRescheduleType {
        #[doc = "If the user wants to schedule the maintenance to happen now."]
        Immediate,
        #[doc = "If the user wants to use the existing maintenance policy to find the\nnext available window."]
        NextAvailableWindow,
        RescheduleTypeUnspecified,
        #[doc = "If the user wants to reschedule the maintenance to a specific time."]
        SpecificTime,
    }
    impl RescheduleRescheduleType {
        pub fn as_str(self) -> &'static str {
            match self {
                RescheduleRescheduleType::Immediate => "IMMEDIATE",
                RescheduleRescheduleType::NextAvailableWindow => "NEXT_AVAILABLE_WINDOW",
                RescheduleRescheduleType::RescheduleTypeUnspecified => {
                    "RESCHEDULE_TYPE_UNSPECIFIED"
                }
                RescheduleRescheduleType::SpecificTime => "SPECIFIC_TIME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RescheduleRescheduleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RescheduleRescheduleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RescheduleRescheduleType, ()> {
            Ok(match s {
                "IMMEDIATE" => RescheduleRescheduleType::Immediate,
                "NEXT_AVAILABLE_WINDOW" => RescheduleRescheduleType::NextAvailableWindow,
                "RESCHEDULE_TYPE_UNSPECIFIED" => {
                    RescheduleRescheduleType::RescheduleTypeUnspecified
                }
                "SPECIFIC_TIME" => RescheduleRescheduleType::SpecificTime,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RescheduleRescheduleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RescheduleRescheduleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RescheduleRescheduleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMMEDIATE" => RescheduleRescheduleType::Immediate,
                "NEXT_AVAILABLE_WINDOW" => RescheduleRescheduleType::NextAvailableWindow,
                "RESCHEDULE_TYPE_UNSPECIFIED" => {
                    RescheduleRescheduleType::RescheduleTypeUnspecified
                }
                "SPECIFIC_TIME" => RescheduleRescheduleType::SpecificTime,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RescheduleRescheduleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RescheduleRescheduleType {
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
    pub struct RestoreBackupContext {
        #[doc = "The ID of the backup run to restore from."]
        #[serde(
            rename = "backupRunId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub backup_run_id: ::std::option::Option<i64>,
        #[doc = "The ID of the instance that the backup was taken from."]
        #[serde(
            rename = "instanceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#restoreBackupContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The full project ID of the source instance."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestoreBackupContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestoreBackupContext {
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
    pub struct RotateServerCaContext {
        #[doc = "This is always <code>sql#rotateServerCaContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The fingerprint of the next version to be rotated to. If left unspecified,\nwill be rotated to the most recently added server CA version."]
        #[serde(
            rename = "nextVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RotateServerCaContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RotateServerCaContext {
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
    pub struct Settings {
        #[doc = "The activation policy specifies when the instance is activated; it is\napplicable only when the instance state is <code>RUNNABLE</code>. Valid\nvalues: <br><code>ALWAYS</code>: The instance is on, and remains so even in\nthe absence of connection requests. <br><code>NEVER</code>: The instance is\noff; it is not activated, even if a connection request arrives.\n<br><code>ON_DEMAND</code>: First Generation instances only. The instance\nresponds to incoming requests, and turns itself off when not in use.\nInstances with <code>PER_USE</code> pricing turn off after 15 minutes of\ninactivity. Instances with <code>PER_PACKAGE</code> pricing turn off after\n12 hours of inactivity."]
        #[serde(
            rename = "activationPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activation_policy: ::std::option::Option<crate::schemas::SettingsActivationPolicy>,
        #[doc = "The App Engine app IDs that can access this instance. First Generation\ninstances only."]
        #[serde(
            rename = "authorizedGaeApplications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorized_gae_applications: ::std::option::Option<Vec<String>>,
        #[doc = "Availability type (PostgreSQL and MySQL instances only). Potential values:\n<br><code>ZONAL</code>: The instance serves data from only one zone.\nOutages in that zone affect data accessibility. <br><code>REGIONAL</code>:\nThe instance can serve data from more than one zone in a region (it is\nhighly available). <br>For more information, see <a\nhref=\"https://cloud.google.com/sql/docs/postgres/high-availability\">Overview\nof the High Availability Configuration</a>."]
        #[serde(
            rename = "availabilityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availability_type: ::std::option::Option<crate::schemas::SettingsAvailabilityType>,
        #[doc = "The daily backup configuration for the instance."]
        #[serde(
            rename = "backupConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backup_configuration: ::std::option::Option<crate::schemas::BackupConfiguration>,
        #[doc = "Configuration specific to read replica instances. Indicates whether\ndatabase flags for crash-safe replication are enabled. This property is\nonly applicable to First Generation instances."]
        #[serde(
            rename = "crashSafeReplicationEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crash_safe_replication_enabled: ::std::option::Option<bool>,
        #[doc = "The size of data disk, in GB. The data disk size minimum is 10GB. Not used\nfor First Generation instances."]
        #[serde(
            rename = "dataDiskSizeGb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub data_disk_size_gb: ::std::option::Option<i64>,
        #[doc = "The type of data disk: <code>PD_SSD</code> (default) or\n<code>PD_HDD</code>. Not used for First Generation instances."]
        #[serde(
            rename = "dataDiskType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_disk_type: ::std::option::Option<crate::schemas::SettingsDataDiskType>,
        #[doc = "The database flags passed to the instance at startup."]
        #[serde(
            rename = "databaseFlags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub database_flags: ::std::option::Option<Vec<crate::schemas::DatabaseFlags>>,
        #[doc = "Configuration specific to read replica instances. Indicates whether\nreplication is enabled or not."]
        #[serde(
            rename = "databaseReplicationEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub database_replication_enabled: ::std::option::Option<bool>,
        #[doc = "The settings for IP Management. This allows to enable or disable the\ninstance IP and manage which external networks can connect to the instance.\nThe IPv4 address cannot be disabled for Second Generation instances."]
        #[serde(
            rename = "ipConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_configuration: ::std::option::Option<crate::schemas::IpConfiguration>,
        #[doc = "This is always <code>sql#settings</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The location preference settings. This allows the instance to be located as\nnear as possible to either an App Engine app or Compute Engine zone for\nbetter performance. App Engine co-location is only applicable to First\nGeneration instances."]
        #[serde(
            rename = "locationPreference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_preference: ::std::option::Option<crate::schemas::LocationPreference>,
        #[doc = "The maintenance window for this instance. This specifies when the instance\ncan be restarted for maintenance purposes. Not used for First Generation\ninstances."]
        #[serde(
            rename = "maintenanceWindow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maintenance_window: ::std::option::Option<crate::schemas::MaintenanceWindow>,
        #[doc = "The pricing plan for this instance. This can be either <code>PER_USE</code>\nor <code>PACKAGE</code>. Only <code>PER_USE</code> is supported for Second\nGeneration instances."]
        #[serde(
            rename = "pricingPlan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pricing_plan: ::std::option::Option<crate::schemas::SettingsPricingPlan>,
        #[doc = "The type of replication this instance uses. This can be either\n<code>ASYNCHRONOUS</code> or <code>SYNCHRONOUS</code>. This property is\nonly applicable to First Generation instances."]
        #[serde(
            rename = "replicationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replication_type: ::std::option::Option<crate::schemas::SettingsReplicationType>,
        #[doc = "The version of instance settings. This is a required field for update\nmethod to make sure concurrent updates are handled properly. During update,\nuse the most recent settingsVersion value for this instance and do not try\nto update this value."]
        #[serde(
            rename = "settingsVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub settings_version: ::std::option::Option<i64>,
        #[doc = "Configuration to increase storage size automatically. The default value is\ntrue. Not used for First Generation instances."]
        #[serde(
            rename = "storageAutoResize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_auto_resize: ::std::option::Option<bool>,
        #[doc = "The maximum size to which storage capacity can be automatically increased.\nThe default value is 0, which specifies that there is no limit. Not used\nfor First Generation instances."]
        #[serde(
            rename = "storageAutoResizeLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub storage_auto_resize_limit: ::std::option::Option<i64>,
        #[doc = "The tier (or machine type) for this instance, for example\n<code>db-n1-standard-1</code> (MySQL instances) or\n<code>db-custom-1-3840</code> (PostgreSQL instances). For MySQL instances,\nthis property determines whether the instance is First or Second\nGeneration. For more information, see <a\nhref=\"/sql/docs/db_path/instance-settings\">Instance Settings</a>."]
        #[serde(
            rename = "tier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<String>,
        #[doc = "User-provided labels, represented as a dictionary where each label is a\nsingle key value pair."]
        #[serde(
            rename = "userLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for Settings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Settings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SettingsActivationPolicy {
        #[doc = "The instance is always up and running."]
        Always,
        #[doc = "The instance should never spin up."]
        Never,
        #[doc = "The instance spins up upon receiving requests."]
        OnDemand,
        #[doc = "Unknown activation plan."]
        SqlActivationPolicyUnspecified,
    }
    impl SettingsActivationPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                SettingsActivationPolicy::Always => "ALWAYS",
                SettingsActivationPolicy::Never => "NEVER",
                SettingsActivationPolicy::OnDemand => "ON_DEMAND",
                SettingsActivationPolicy::SqlActivationPolicyUnspecified => {
                    "SQL_ACTIVATION_POLICY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for SettingsActivationPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SettingsActivationPolicy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SettingsActivationPolicy, ()> {
            Ok(match s {
                "ALWAYS" => SettingsActivationPolicy::Always,
                "NEVER" => SettingsActivationPolicy::Never,
                "ON_DEMAND" => SettingsActivationPolicy::OnDemand,
                "SQL_ACTIVATION_POLICY_UNSPECIFIED" => {
                    SettingsActivationPolicy::SqlActivationPolicyUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SettingsActivationPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SettingsActivationPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SettingsActivationPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALWAYS" => SettingsActivationPolicy::Always,
                "NEVER" => SettingsActivationPolicy::Never,
                "ON_DEMAND" => SettingsActivationPolicy::OnDemand,
                "SQL_ACTIVATION_POLICY_UNSPECIFIED" => {
                    SettingsActivationPolicy::SqlActivationPolicyUnspecified
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
    impl ::google_field_selector::FieldSelector for SettingsActivationPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SettingsActivationPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SettingsAvailabilityType {
        #[doc = "Regional available instance."]
        Regional,
        #[doc = "This is an unknown Availability type."]
        SqlAvailabilityTypeUnspecified,
        #[doc = "Zonal available instance."]
        Zonal,
    }
    impl SettingsAvailabilityType {
        pub fn as_str(self) -> &'static str {
            match self {
                SettingsAvailabilityType::Regional => "REGIONAL",
                SettingsAvailabilityType::SqlAvailabilityTypeUnspecified => {
                    "SQL_AVAILABILITY_TYPE_UNSPECIFIED"
                }
                SettingsAvailabilityType::Zonal => "ZONAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SettingsAvailabilityType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SettingsAvailabilityType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SettingsAvailabilityType, ()> {
            Ok(match s {
                "REGIONAL" => SettingsAvailabilityType::Regional,
                "SQL_AVAILABILITY_TYPE_UNSPECIFIED" => {
                    SettingsAvailabilityType::SqlAvailabilityTypeUnspecified
                }
                "ZONAL" => SettingsAvailabilityType::Zonal,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SettingsAvailabilityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SettingsAvailabilityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SettingsAvailabilityType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REGIONAL" => SettingsAvailabilityType::Regional,
                "SQL_AVAILABILITY_TYPE_UNSPECIFIED" => {
                    SettingsAvailabilityType::SqlAvailabilityTypeUnspecified
                }
                "ZONAL" => SettingsAvailabilityType::Zonal,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SettingsAvailabilityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SettingsAvailabilityType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SettingsDataDiskType {
        #[doc = "This field is deprecated and will be removed from a future version of the\nAPI."]
        ObsoleteLocalSsd,
        #[doc = "An HDD data disk."]
        PdHdd,
        #[doc = "An SSD data disk."]
        PdSsd,
        #[doc = "This is an unknown data disk type."]
        SqlDataDiskTypeUnspecified,
    }
    impl SettingsDataDiskType {
        pub fn as_str(self) -> &'static str {
            match self {
                SettingsDataDiskType::ObsoleteLocalSsd => "OBSOLETE_LOCAL_SSD",
                SettingsDataDiskType::PdHdd => "PD_HDD",
                SettingsDataDiskType::PdSsd => "PD_SSD",
                SettingsDataDiskType::SqlDataDiskTypeUnspecified => {
                    "SQL_DATA_DISK_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for SettingsDataDiskType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SettingsDataDiskType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SettingsDataDiskType, ()> {
            Ok(match s {
                "OBSOLETE_LOCAL_SSD" => SettingsDataDiskType::ObsoleteLocalSsd,
                "PD_HDD" => SettingsDataDiskType::PdHdd,
                "PD_SSD" => SettingsDataDiskType::PdSsd,
                "SQL_DATA_DISK_TYPE_UNSPECIFIED" => {
                    SettingsDataDiskType::SqlDataDiskTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SettingsDataDiskType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SettingsDataDiskType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SettingsDataDiskType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OBSOLETE_LOCAL_SSD" => SettingsDataDiskType::ObsoleteLocalSsd,
                "PD_HDD" => SettingsDataDiskType::PdHdd,
                "PD_SSD" => SettingsDataDiskType::PdSsd,
                "SQL_DATA_DISK_TYPE_UNSPECIFIED" => {
                    SettingsDataDiskType::SqlDataDiskTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for SettingsDataDiskType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SettingsDataDiskType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SettingsPricingPlan {
        #[doc = "The instance is billed at a monthly flat rate."]
        Package,
        #[doc = "The instance is billed per usage."]
        PerUse,
        #[doc = "This is an unknown pricing plan for this instance."]
        SqlPricingPlanUnspecified,
    }
    impl SettingsPricingPlan {
        pub fn as_str(self) -> &'static str {
            match self {
                SettingsPricingPlan::Package => "PACKAGE",
                SettingsPricingPlan::PerUse => "PER_USE",
                SettingsPricingPlan::SqlPricingPlanUnspecified => "SQL_PRICING_PLAN_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SettingsPricingPlan {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SettingsPricingPlan {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SettingsPricingPlan, ()> {
            Ok(match s {
                "PACKAGE" => SettingsPricingPlan::Package,
                "PER_USE" => SettingsPricingPlan::PerUse,
                "SQL_PRICING_PLAN_UNSPECIFIED" => SettingsPricingPlan::SqlPricingPlanUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SettingsPricingPlan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SettingsPricingPlan {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SettingsPricingPlan {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PACKAGE" => SettingsPricingPlan::Package,
                "PER_USE" => SettingsPricingPlan::PerUse,
                "SQL_PRICING_PLAN_UNSPECIFIED" => SettingsPricingPlan::SqlPricingPlanUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SettingsPricingPlan {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SettingsPricingPlan {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SettingsReplicationType {
        #[doc = "The asynchronous replication mode for First Generation instances. It\nprovides a slight performance gain, but if an outage occurs while this\noption is set to asynchronous, you can lose up to a few seconds of updates\nto your data."]
        Asynchronous,
        #[doc = "This is an unknown replication type for a Cloud SQL instance."]
        SqlReplicationTypeUnspecified,
        #[doc = "The synchronous replication mode for First Generation instances. It is the\ndefault value."]
        Synchronous,
    }
    impl SettingsReplicationType {
        pub fn as_str(self) -> &'static str {
            match self {
                SettingsReplicationType::Asynchronous => "ASYNCHRONOUS",
                SettingsReplicationType::SqlReplicationTypeUnspecified => {
                    "SQL_REPLICATION_TYPE_UNSPECIFIED"
                }
                SettingsReplicationType::Synchronous => "SYNCHRONOUS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SettingsReplicationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SettingsReplicationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SettingsReplicationType, ()> {
            Ok(match s {
                "ASYNCHRONOUS" => SettingsReplicationType::Asynchronous,
                "SQL_REPLICATION_TYPE_UNSPECIFIED" => {
                    SettingsReplicationType::SqlReplicationTypeUnspecified
                }
                "SYNCHRONOUS" => SettingsReplicationType::Synchronous,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SettingsReplicationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SettingsReplicationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SettingsReplicationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASYNCHRONOUS" => SettingsReplicationType::Asynchronous,
                "SQL_REPLICATION_TYPE_UNSPECIFIED" => {
                    SettingsReplicationType::SqlReplicationTypeUnspecified
                }
                "SYNCHRONOUS" => SettingsReplicationType::Synchronous,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SettingsReplicationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SettingsReplicationType {
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
    pub struct SqlExternalSyncSettingError {
        #[doc = "Additional information about the error encountered."]
        #[serde(
            rename = "detail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#migrationSettingError</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Identifies the specific error that occurred."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SqlExternalSyncSettingErrorType>,
    }
    impl ::google_field_selector::FieldSelector for SqlExternalSyncSettingError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlExternalSyncSettingError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SqlExternalSyncSettingErrorType {
        BinlogNotEnabled,
        ConnectionFailure,
        IncompatibleDatabaseVersion,
        InsufficientPrivilege,
        #[doc = "No pglogical extension installed on databases, applicable for postgres."]
        NoPglogicalInstalled,
        #[doc = "pglogical node already exists on databases, applicable for postgres."]
        PglogicalNodeAlreadyExists,
        ReplicaAlreadySetup,
        SqlExternalSyncSettingErrorTypeUnspecified,
        #[doc = "Unsupported migration type."]
        UnsupportedMigrationType,
    }
    impl SqlExternalSyncSettingErrorType {
        pub fn as_str(self) -> &'static str {
            match self {
                SqlExternalSyncSettingErrorType::BinlogNotEnabled => "BINLOG_NOT_ENABLED",
                SqlExternalSyncSettingErrorType::ConnectionFailure => "CONNECTION_FAILURE",
                SqlExternalSyncSettingErrorType::IncompatibleDatabaseVersion => {
                    "INCOMPATIBLE_DATABASE_VERSION"
                }
                SqlExternalSyncSettingErrorType::InsufficientPrivilege => "INSUFFICIENT_PRIVILEGE",
                SqlExternalSyncSettingErrorType::NoPglogicalInstalled => "NO_PGLOGICAL_INSTALLED",
                SqlExternalSyncSettingErrorType::PglogicalNodeAlreadyExists => {
                    "PGLOGICAL_NODE_ALREADY_EXISTS"
                }
                SqlExternalSyncSettingErrorType::ReplicaAlreadySetup => "REPLICA_ALREADY_SETUP",
                SqlExternalSyncSettingErrorType::SqlExternalSyncSettingErrorTypeUnspecified => {
                    "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED"
                }
                SqlExternalSyncSettingErrorType::UnsupportedMigrationType => {
                    "UNSUPPORTED_MIGRATION_TYPE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for SqlExternalSyncSettingErrorType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SqlExternalSyncSettingErrorType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SqlExternalSyncSettingErrorType, ()> {
            Ok(match s {
                "BINLOG_NOT_ENABLED" => SqlExternalSyncSettingErrorType::BinlogNotEnabled,
                "CONNECTION_FAILURE" => SqlExternalSyncSettingErrorType::ConnectionFailure,
                "INCOMPATIBLE_DATABASE_VERSION" => {
                    SqlExternalSyncSettingErrorType::IncompatibleDatabaseVersion
                }
                "INSUFFICIENT_PRIVILEGE" => SqlExternalSyncSettingErrorType::InsufficientPrivilege,
                "NO_PGLOGICAL_INSTALLED" => SqlExternalSyncSettingErrorType::NoPglogicalInstalled,
                "PGLOGICAL_NODE_ALREADY_EXISTS" => {
                    SqlExternalSyncSettingErrorType::PglogicalNodeAlreadyExists
                }
                "REPLICA_ALREADY_SETUP" => SqlExternalSyncSettingErrorType::ReplicaAlreadySetup,
                "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED" => {
                    SqlExternalSyncSettingErrorType::SqlExternalSyncSettingErrorTypeUnspecified
                }
                "UNSUPPORTED_MIGRATION_TYPE" => {
                    SqlExternalSyncSettingErrorType::UnsupportedMigrationType
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SqlExternalSyncSettingErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SqlExternalSyncSettingErrorType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SqlExternalSyncSettingErrorType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BINLOG_NOT_ENABLED" => SqlExternalSyncSettingErrorType::BinlogNotEnabled,
                "CONNECTION_FAILURE" => SqlExternalSyncSettingErrorType::ConnectionFailure,
                "INCOMPATIBLE_DATABASE_VERSION" => {
                    SqlExternalSyncSettingErrorType::IncompatibleDatabaseVersion
                }
                "INSUFFICIENT_PRIVILEGE" => SqlExternalSyncSettingErrorType::InsufficientPrivilege,
                "NO_PGLOGICAL_INSTALLED" => SqlExternalSyncSettingErrorType::NoPglogicalInstalled,
                "PGLOGICAL_NODE_ALREADY_EXISTS" => {
                    SqlExternalSyncSettingErrorType::PglogicalNodeAlreadyExists
                }
                "REPLICA_ALREADY_SETUP" => SqlExternalSyncSettingErrorType::ReplicaAlreadySetup,
                "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED" => {
                    SqlExternalSyncSettingErrorType::SqlExternalSyncSettingErrorTypeUnspecified
                }
                "UNSUPPORTED_MIGRATION_TYPE" => {
                    SqlExternalSyncSettingErrorType::UnsupportedMigrationType
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
    impl ::google_field_selector::FieldSelector for SqlExternalSyncSettingErrorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlExternalSyncSettingErrorType {
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
    pub struct SqlInstancesRescheduleMaintenanceRequestBody {
        #[doc = "Required. The type of the reschedule the user wants."]
        #[serde(
            rename = "reschedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reschedule: ::std::option::Option<crate::schemas::Reschedule>,
    }
    impl ::google_field_selector::FieldSelector for SqlInstancesRescheduleMaintenanceRequestBody {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlInstancesRescheduleMaintenanceRequestBody {
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
    pub struct SqlInstancesVerifyExternalSyncSettingsResponse {
        #[doc = "List of migration violations."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::SqlExternalSyncSettingError>>,
        #[doc = "This is always <code>sql#migrationSettingErrorList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SqlInstancesVerifyExternalSyncSettingsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlInstancesVerifyExternalSyncSettingsResponse {
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
    pub struct SqlScheduledMaintenance {
        #[serde(
            rename = "canDefer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_defer: ::std::option::Option<bool>,
        #[doc = "If the scheduled maintenance can be rescheduled."]
        #[serde(
            rename = "canReschedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_reschedule: ::std::option::Option<bool>,
        #[doc = "The start time of any upcoming scheduled maintenance for this instance."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SqlScheduledMaintenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlScheduledMaintenance {
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
    pub struct SqlServerDatabaseDetails {
        #[doc = "The version of SQL Server with which the database is to be made compatible"]
        #[serde(
            rename = "compatibilityLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compatibility_level: ::std::option::Option<i32>,
        #[doc = "The recovery model of a SQL Server database"]
        #[serde(
            rename = "recoveryModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recovery_model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SqlServerDatabaseDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlServerDatabaseDetails {
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
    pub struct SqlServerUserDetails {
        #[doc = "If the user has been disabled"]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "The server roles for this user"]
        #[serde(
            rename = "serverRoles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub server_roles: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for SqlServerUserDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SqlServerUserDetails {
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
    pub struct SslCert {
        #[doc = "PEM representation."]
        #[serde(
            rename = "cert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert: ::std::option::Option<String>,
        #[doc = "Serial number, as extracted from the certificate."]
        #[serde(
            rename = "certSerialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert_serial_number: ::std::option::Option<String>,
        #[doc = "User supplied name.  Constrained to [a-zA-Z.-_ ]+."]
        #[serde(
            rename = "commonName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_name: ::std::option::Option<String>,
        #[doc = "The time when the certificate was created in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>"]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time when the certificate expires in <a\nhref=\"https://tools.ietf.org/html/rfc3339\">RFC 3339</a> format, for example\n<code>2012-11-15T16:19:00.094Z</code>."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expiration_time: ::std::option::Option<String>,
        #[doc = "Name of the database instance."]
        #[serde(
            rename = "instance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#sslCert</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The URI of this resource."]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Sha1 Fingerprint."]
        #[serde(
            rename = "sha1Fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_1_fingerprint: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SslCert {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SslCert {
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
    pub struct SslCertDetail {
        #[doc = "The public information about the cert."]
        #[serde(
            rename = "certInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert_info: ::std::option::Option<crate::schemas::SslCert>,
        #[doc = "The private key for the client cert, in pem format.  Keep private in order\nto protect your security."]
        #[serde(
            rename = "certPrivateKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cert_private_key: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SslCertDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SslCertDetail {
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
    pub struct SslCertsCreateEphemeralRequest {
        #[doc = "PEM encoded public key to include in the signed certificate."]
        #[serde(
            rename = "public_key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SslCertsCreateEphemeralRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SslCertsCreateEphemeralRequest {
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
    pub struct SslCertsInsertRequest {
        #[doc = "User supplied name.  Must be a distinct name from the other certificates\nfor this instance."]
        #[serde(
            rename = "commonName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SslCertsInsertRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SslCertsInsertRequest {
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
    pub struct SslCertsInsertResponse {
        #[doc = "The new client certificate and private key.  For First Generation\ninstances, the new certificate does not take effect until the instance is\nrestarted."]
        #[serde(
            rename = "clientCert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_cert: ::std::option::Option<crate::schemas::SslCertDetail>,
        #[doc = "This is always <code>sql#sslCertsInsert</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The operation to track the ssl certs insert request."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<crate::schemas::Operation>,
        #[doc = "The server Certificate Authority's certificate.  If this is missing you can\nforce a new one to be generated by calling resetSslConfig method on\ninstances resource."]
        #[serde(
            rename = "serverCaCert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub server_ca_cert: ::std::option::Option<crate::schemas::SslCert>,
    }
    impl ::google_field_selector::FieldSelector for SslCertsInsertResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SslCertsInsertResponse {
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
    pub struct SslCertsListResponse {
        #[doc = "List of client certificates for the instance."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::SslCert>>,
        #[doc = "This is always <code>sql#sslCertsList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SslCertsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SslCertsListResponse {
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
    pub struct Tier {
        #[doc = "The maximum disk size of this tier in bytes."]
        #[serde(
            rename = "DiskQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub disk_quota: ::std::option::Option<i64>,
        #[doc = "This is always <code>sql#tier</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The maximum RAM usage of this tier in bytes."]
        #[serde(
            rename = "RAM",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ram: ::std::option::Option<i64>,
        #[doc = "The applicable regions for this tier."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<Vec<String>>,
        #[doc = "An identifier for the machine type, for example, db-n1-standard-1. For\nrelated information, see <a href=\"/sql/pricing\">Pricing</a>."]
        #[serde(
            rename = "tier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Tier {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Tier {
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
    pub struct TiersListResponse {
        #[doc = "List of tiers."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Tier>>,
        #[doc = "This is always <code>sql#tiersList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TiersListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TiersListResponse {
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
    pub struct TruncateLogContext {
        #[doc = "This is always <code>sql#truncateLogContext</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The type of log to truncate. Valid values are\n<code>MYSQL_GENERAL_TABLE</code> and <code>MYSQL_SLOW_TABLE</code>."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TruncateLogContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TruncateLogContext {
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
    pub struct User {
        #[doc = "This field is deprecated and will be removed from a future version of the\nAPI."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The host name from which the user can connect. For <code>insert</code>\noperations, host defaults to an empty string. For <code>update</code>\noperations, host is specified as part of the request URL. The host name\ncannot be updated after insertion."]
        #[serde(
            rename = "host",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host: ::std::option::Option<String>,
        #[doc = "The name of the Cloud SQL instance. This does not include the project ID.\nCan be omitted for <code>update</code> since it is already specified on the\nURL."]
        #[serde(
            rename = "instance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance: ::std::option::Option<String>,
        #[doc = "This is always <code>sql#user</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the user in the Cloud SQL instance. Can be omitted for\n<code>update</code> since it is already specified in the URL."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The password for the user."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "The project ID of the project containing the Cloud SQL database. The Google\napps domain is prefixed if applicable. Can be omitted for\n<code>update</code> since it is already specified on the URL."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[serde(
            rename = "sqlserverUserDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sqlserver_user_details: ::std::option::Option<crate::schemas::SqlServerUserDetails>,
    }
    impl ::google_field_selector::FieldSelector for User {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for User {
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
    pub struct UsersListResponse {
        #[doc = "List of user resources in the instance."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::User>>,
        #[doc = "This is always <code>sql#usersList</code>."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "An identifier that uniquely identifies the operation. You can use this\nidentifier to retrieve the Operations resource that has information about\nthe operation."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsersListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersListResponse {
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
    #[doc = "Actions that can be performed on the backup_runs resource"]
    pub fn backup_runs(&self) -> crate::resources::backup_runs::BackupRunsActions {
        crate::resources::backup_runs::BackupRunsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the databases resource"]
    pub fn databases(&self) -> crate::resources::databases::DatabasesActions {
        crate::resources::databases::DatabasesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the flags resource"]
    pub fn flags(&self) -> crate::resources::flags::FlagsActions {
        crate::resources::flags::FlagsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the instances resource"]
    pub fn instances(&self) -> crate::resources::instances::InstancesActions {
        crate::resources::instances::InstancesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions {
        crate::resources::operations::OperationsActions {
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
    #[doc = "Actions that can be performed on the ssl_certs resource"]
    pub fn ssl_certs(&self) -> crate::resources::ssl_certs::SslCertsActions {
        crate::resources::ssl_certs::SslCertsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the tiers resource"]
    pub fn tiers(&self) -> crate::resources::tiers::TiersActions {
        crate::resources::tiers::TiersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the users resource"]
    pub fn users(&self) -> crate::resources::users::UsersActions {
        crate::resources::users::UsersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod backup_runs {
        pub mod params {}
        pub struct BackupRunsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BackupRunsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes the backup taken by a backup run."]
            pub fn delete(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
                id: i64,
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
                    project: project.into(),
                    instance: instance.into(),
                    id,
                }
            }
            #[doc = "Retrieves a resource containing information about a backup run."]
            pub fn get(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
                id: i64,
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
                    project: project.into(),
                    instance: instance.into(),
                    id,
                }
            }
            #[doc = "Creates a new backup run on demand. This method is applicable only to\nSecond Generation instances."]
            pub fn insert(
                &self,
                request: crate::schemas::BackupRun,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Lists all backup runs associated with a given instance and configuration in\nthe reverse chronological order of the backup initiation time."]
            pub fn list(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [BackupRunsActions::delete()](struct.BackupRunsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            id: i64,
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/backupRuns/");
                {
                    let var_as_string = self.id.to_string();
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
        #[doc = "Created via [BackupRunsActions::get()](struct.BackupRunsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            id: i64,
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
            ) -> Result<crate::schemas::BackupRun, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BackupRun, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/backupRuns/");
                {
                    let var_as_string = self.id.to_string();
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
        #[doc = "Created via [BackupRunsActions::insert()](struct.BackupRunsActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BackupRun,
            project: String,
            instance: String,
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
        impl<'a> InsertRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/backupRuns");
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
        #[doc = "Created via [BackupRunsActions::list()](struct.BackupRunsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
            #[doc = "Maximum number of backup runs per response."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A previously-returned page token representing part of the larger set of\nresults to view."]
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
            pub fn iter_items<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_items_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::BackupRun> {
                self.iter_items_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::BackupRun> {
                self.iter_items_with_fields(Some("*"))
            }
            pub fn iter_items_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "items").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "items")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::BackupRunsListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::BackupRunsListResponse> {
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
            ) -> Result<crate::schemas::BackupRunsListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BackupRunsListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/backupRuns");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
    pub mod databases {
        pub mod params {}
        pub struct DatabasesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DatabasesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes a database from a Cloud SQL instance."]
            pub fn delete(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
                database: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    database: database.into(),
                }
            }
            #[doc = "Retrieves a resource containing information about a database inside a Cloud\nSQL instance."]
            pub fn get(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
                database: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    database: database.into(),
                }
            }
            #[doc = "Inserts a resource containing information about a database inside a Cloud\nSQL instance."]
            pub fn insert(
                &self,
                request: crate::schemas::Database,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Lists databases in the specified Cloud SQL instance."]
            pub fn list(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Partially updates a resource containing information about a database inside\na Cloud SQL instance. This method supports patch semantics."]
            pub fn patch(
                &self,
                request: crate::schemas::Database,
                project: impl Into<String>,
                instance: impl Into<String>,
                database: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                    database: database.into(),
                }
            }
            #[doc = "Updates a resource containing information about a database inside a Cloud\nSQL instance."]
            pub fn update(
                &self,
                request: crate::schemas::Database,
                project: impl Into<String>,
                instance: impl Into<String>,
                database: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    database: database.into(),
                }
            }
        }
        #[doc = "Created via [DatabasesActions::delete()](struct.DatabasesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            database: String,
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/databases/");
                {
                    let var_as_str = &self.database;
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
        #[doc = "Created via [DatabasesActions::get()](struct.DatabasesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            database: String,
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
            ) -> Result<crate::schemas::Database, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Database, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/databases/");
                {
                    let var_as_str = &self.database;
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
        #[doc = "Created via [DatabasesActions::insert()](struct.DatabasesActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Database,
            project: String,
            instance: String,
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
        impl<'a> InsertRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/databases");
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
        #[doc = "Created via [DatabasesActions::list()](struct.DatabasesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
            ) -> Result<crate::schemas::DatabasesListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DatabasesListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/databases");
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
        #[doc = "Created via [DatabasesActions::patch()](struct.DatabasesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Database,
            project: String,
            instance: String,
            database: String,
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
        impl<'a> PatchRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/databases/");
                {
                    let var_as_str = &self.database;
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
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        #[doc = "Created via [DatabasesActions::update()](struct.DatabasesActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Database,
            project: String,
            instance: String,
            database: String,
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/databases/");
                {
                    let var_as_str = &self.database;
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
    pub mod flags {
        pub mod params {}
        pub struct FlagsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> FlagsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "List all available database flags for Cloud SQL instances."]
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
                    database_version: None,
                }
            }
        }
        #[doc = "Created via [FlagsActions::list()](struct.FlagsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            database_version: Option<String>,
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
            #[doc = "Database type and version you want to retrieve flags for. By default, this\nmethod returns flags for all database types and versions."]
            pub fn database_version(mut self, value: impl Into<String>) -> Self {
                self.database_version = Some(value.into());
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
            ) -> Result<crate::schemas::FlagsListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FlagsListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/flags");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("databaseVersion", &self.database_version)]);
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
    pub mod instances {
        pub mod params {}
        pub struct InstancesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> InstancesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Add a new trusted Certificate Authority (CA) version for the specified\ninstance. Required to prepare for a certificate rotation. If a CA version\nwas previously added but never used in a certificate rotation, this\noperation replaces that version. There cannot be more than one CA version\nwaiting to be rotated in."]
            pub fn add_server_ca(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> AddServerCaRequestBuilder {
                AddServerCaRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Creates a Cloud SQL instance as a clone of the source instance. Using this\noperation might cause your instance to restart."]
            pub fn clone(
                &self,
                request: crate::schemas::InstancesCloneRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> CloneRequestBuilder {
                CloneRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Deletes a Cloud SQL instance."]
            pub fn delete(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Demotes the stand-alone instance to be a Cloud SQL read replica for an\nexternal database server."]
            pub fn demote_master(
                &self,
                request: crate::schemas::InstancesDemoteMasterRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> DemoteMasterRequestBuilder {
                DemoteMasterRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL\ndump or CSV file."]
            pub fn export(
                &self,
                request: crate::schemas::InstancesExportRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> ExportRequestBuilder {
                ExportRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Failover the instance to its failover replica instance. Using this\noperation might cause your instance to restart."]
            pub fn failover(
                &self,
                request: crate::schemas::InstancesFailoverRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> FailoverRequestBuilder {
                FailoverRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Retrieves a resource containing information about a Cloud SQL instance."]
            pub fn get(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Imports data into a Cloud SQL instance from a SQL dump  or CSV file in\nCloud Storage."]
            pub fn import(
                &self,
                request: crate::schemas::InstancesImportRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> ImportRequestBuilder {
                ImportRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Creates a new Cloud SQL instance."]
            pub fn insert(
                &self,
                request: crate::schemas::DatabaseInstance,
                project: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    project: project.into(),
                }
            }
            #[doc = "Lists instances under a given project."]
            pub fn list(&self, project: impl Into<String>) -> ListRequestBuilder {
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
                    project: project.into(),
                    filter: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Lists all of the trusted Certificate Authorities (CAs) for the specified\ninstance. There can be up to three CAs listed: the CA that was used to sign\nthe certificate that is currently in use, a CA that has been added but not\nyet used to sign a certificate, and a CA used to sign a certificate that\nhas previously rotated out."]
            pub fn list_server_cas(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> ListServerCasRequestBuilder {
                ListServerCasRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Updates settings of a Cloud SQL instance.\nThis method supports patch semantics."]
            pub fn patch(
                &self,
                request: crate::schemas::DatabaseInstance,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Promotes the read replica instance to be a stand-alone Cloud SQL instance.\nUsing this operation might cause your instance to restart."]
            pub fn promote_replica(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> PromoteReplicaRequestBuilder {
                PromoteReplicaRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Deletes all client certificates and generates a new server SSL certificate\nfor the instance."]
            pub fn reset_ssl_config(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> ResetSslConfigRequestBuilder {
                ResetSslConfigRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Restarts a Cloud SQL instance."]
            pub fn restart(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> RestartRequestBuilder {
                RestartRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Restores a backup of a Cloud SQL instance. Using this operation might cause\nyour instance to restart."]
            pub fn restore_backup(
                &self,
                request: crate::schemas::InstancesRestoreBackupRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> RestoreBackupRequestBuilder {
                RestoreBackupRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Rotates the server certificate to one signed by the Certificate Authority\n(CA) version previously added with the addServerCA method."]
            pub fn rotate_server_ca(
                &self,
                request: crate::schemas::InstancesRotateServerCaRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> RotateServerCaRequestBuilder {
                RotateServerCaRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Starts the replication in the read replica instance."]
            pub fn start_replica(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> StartReplicaRequestBuilder {
                StartReplicaRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Stops the replication in the read replica instance."]
            pub fn stop_replica(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> StopReplicaRequestBuilder {
                StopReplicaRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Truncate MySQL general and slow query log tables"]
            pub fn truncate_log(
                &self,
                request: crate::schemas::InstancesTruncateLogRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> TruncateLogRequestBuilder {
                TruncateLogRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Updates settings of a Cloud SQL instance. Using this operation might cause\nyour instance to restart."]
            pub fn update(
                &self,
                request: crate::schemas::DatabaseInstance,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
        }
        #[doc = "Created via [InstancesActions::add_server_ca()](struct.InstancesActions.html#method.add_server_ca)"]
        #[derive(Debug, Clone)]
        pub struct AddServerCaRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> AddServerCaRequestBuilder<'a> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/addServerCa");
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
        #[doc = "Created via [InstancesActions::clone()](struct.InstancesActions.html#method.clone)"]
        #[derive(Debug, Clone)]
        pub struct CloneRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesCloneRequest,
            project: String,
            instance: String,
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
        impl<'a> CloneRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/clone");
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
        #[doc = "Created via [InstancesActions::delete()](struct.InstancesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
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
        #[doc = "Created via [InstancesActions::demote_master()](struct.InstancesActions.html#method.demote_master)"]
        #[derive(Debug, Clone)]
        pub struct DemoteMasterRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesDemoteMasterRequest,
            project: String,
            instance: String,
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
        impl<'a> DemoteMasterRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/demoteMaster");
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
        #[doc = "Created via [InstancesActions::export()](struct.InstancesActions.html#method.export)"]
        #[derive(Debug, Clone)]
        pub struct ExportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesExportRequest,
            project: String,
            instance: String,
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
        impl<'a> ExportRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/export");
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
        #[doc = "Created via [InstancesActions::failover()](struct.InstancesActions.html#method.failover)"]
        #[derive(Debug, Clone)]
        pub struct FailoverRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesFailoverRequest,
            project: String,
            instance: String,
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
        impl<'a> FailoverRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/failover");
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
        #[doc = "Created via [InstancesActions::get()](struct.InstancesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
            ) -> Result<crate::schemas::DatabaseInstance, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DatabaseInstance, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
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
        #[doc = "Created via [InstancesActions::import()](struct.InstancesActions.html#method.import)"]
        #[derive(Debug, Clone)]
        pub struct ImportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesImportRequest,
            project: String,
            instance: String,
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
        impl<'a> ImportRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/import");
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
        #[doc = "Created via [InstancesActions::insert()](struct.InstancesActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::DatabaseInstance,
            project: String,
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
        impl<'a> InsertRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances");
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
        #[doc = "Created via [InstancesActions::list()](struct.InstancesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            filter: Option<String>,
            max_results: Option<u32>,
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
            #[doc = "A filter expression that filters resources listed in the response.\nThe expression is in the form of field:value. For example,\n'instanceType:CLOUD_SQL_INSTANCE'. Fields can be nested as needed as per\ntheir JSON representation, such as 'settings.userLabels.auto_start:true'.\n\nMultiple filter queries are space-separated. For example.\n'state:RUNNABLE instanceType:CLOUD_SQL_INSTANCE'. By default, each\nexpression is an AND expression. However, you can include AND and OR\nexpressions explicitly."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The maximum number of results to return per response."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A previously-returned page token representing part of the larger set of\nresults to view."]
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
            pub fn iter_items<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_items_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DatabaseInstance> {
                self.iter_items_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DatabaseInstance> {
                self.iter_items_with_fields(Some("*"))
            }
            pub fn iter_items_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "items").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_warnings<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_warnings_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_warnings_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ApiWarning> {
                self.iter_warnings_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_warnings_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ApiWarning> {
                self.iter_warnings_with_fields(Some("*"))
            }
            pub fn iter_warnings_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "warnings").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "warnings")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::InstancesListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::InstancesListResponse> {
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
            ) -> Result<crate::schemas::InstancesListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::InstancesListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
        #[doc = "Created via [InstancesActions::list_server_cas()](struct.InstancesActions.html#method.list_server_cas)"]
        #[derive(Debug, Clone)]
        pub struct ListServerCasRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> ListServerCasRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::InstancesListServerCasResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::InstancesListServerCasResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/listServerCas");
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
        #[doc = "Created via [InstancesActions::patch()](struct.InstancesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::DatabaseInstance,
            project: String,
            instance: String,
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
        impl<'a> PatchRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
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
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        #[doc = "Created via [InstancesActions::promote_replica()](struct.InstancesActions.html#method.promote_replica)"]
        #[derive(Debug, Clone)]
        pub struct PromoteReplicaRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> PromoteReplicaRequestBuilder<'a> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/promoteReplica");
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
        #[doc = "Created via [InstancesActions::reset_ssl_config()](struct.InstancesActions.html#method.reset_ssl_config)"]
        #[derive(Debug, Clone)]
        pub struct ResetSslConfigRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> ResetSslConfigRequestBuilder<'a> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/resetSslConfig");
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
        #[doc = "Created via [InstancesActions::restart()](struct.InstancesActions.html#method.restart)"]
        #[derive(Debug, Clone)]
        pub struct RestartRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> RestartRequestBuilder<'a> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/restart");
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
        #[doc = "Created via [InstancesActions::restore_backup()](struct.InstancesActions.html#method.restore_backup)"]
        #[derive(Debug, Clone)]
        pub struct RestoreBackupRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesRestoreBackupRequest,
            project: String,
            instance: String,
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
        impl<'a> RestoreBackupRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/restoreBackup");
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
        #[doc = "Created via [InstancesActions::rotate_server_ca()](struct.InstancesActions.html#method.rotate_server_ca)"]
        #[derive(Debug, Clone)]
        pub struct RotateServerCaRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesRotateServerCaRequest,
            project: String,
            instance: String,
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
        impl<'a> RotateServerCaRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/rotateServerCa");
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
        #[doc = "Created via [InstancesActions::start_replica()](struct.InstancesActions.html#method.start_replica)"]
        #[derive(Debug, Clone)]
        pub struct StartReplicaRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> StartReplicaRequestBuilder<'a> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/startReplica");
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
        #[doc = "Created via [InstancesActions::stop_replica()](struct.InstancesActions.html#method.stop_replica)"]
        #[derive(Debug, Clone)]
        pub struct StopReplicaRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
        impl<'a> StopReplicaRequestBuilder<'a> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/stopReplica");
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
        #[doc = "Created via [InstancesActions::truncate_log()](struct.InstancesActions.html#method.truncate_log)"]
        #[derive(Debug, Clone)]
        pub struct TruncateLogRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InstancesTruncateLogRequest,
            project: String,
            instance: String,
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
        impl<'a> TruncateLogRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/truncateLog");
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
        #[doc = "Created via [InstancesActions::update()](struct.InstancesActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::DatabaseInstance,
            project: String,
            instance: String,
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
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
    pub mod operations {
        pub mod params {}
        pub struct OperationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OperationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves an instance operation that has been performed on an instance."]
            pub fn get(
                &self,
                project: impl Into<String>,
                operation: impl Into<String>,
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
                    project: project.into(),
                    operation: operation.into(),
                }
            }
            #[doc = "Lists all instance operations that have been performed on the given Cloud\nSQL instance in the reverse chronological order of the start time."]
            pub fn list(&self, project: impl Into<String>) -> ListRequestBuilder {
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
                    project: project.into(),
                    instance: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            operation: String,
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/operations/");
                {
                    let var_as_str = &self.operation;
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
        #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: Option<String>,
            max_results: Option<u32>,
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
            #[doc = "Cloud SQL instance ID. This does not include the project ID."]
            pub fn instance(mut self, value: impl Into<String>) -> Self {
                self.instance = Some(value.into());
                self
            }
            #[doc = "Maximum number of operations per response."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A previously-returned page token representing part of the larger set of\nresults to view."]
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
            pub fn iter_items<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_items_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
                self.iter_items_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
                self.iter_items_with_fields(Some("*"))
            }
            pub fn iter_items_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "items").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "items")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::OperationsListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::OperationsListResponse> {
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
            ) -> Result<crate::schemas::OperationsListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::OperationsListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/operations");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("instance", &self.instance)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the instances resource"]
            pub fn instances(&self) -> crate::resources::projects::instances::InstancesActions {
                crate::resources::projects::instances::InstancesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod instances {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum StartExternalSyncSyncMode {
                    ExternalSyncModeUnspecified,
                    Offline,
                    Online,
                }
                impl StartExternalSyncSyncMode {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            StartExternalSyncSyncMode::ExternalSyncModeUnspecified => {
                                "EXTERNAL_SYNC_MODE_UNSPECIFIED"
                            }
                            StartExternalSyncSyncMode::Offline => "OFFLINE",
                            StartExternalSyncSyncMode::Online => "ONLINE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for StartExternalSyncSyncMode {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for StartExternalSyncSyncMode {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<StartExternalSyncSyncMode, ()> {
                        Ok(match s {
                            "EXTERNAL_SYNC_MODE_UNSPECIFIED" => {
                                StartExternalSyncSyncMode::ExternalSyncModeUnspecified
                            }
                            "OFFLINE" => StartExternalSyncSyncMode::Offline,
                            "ONLINE" => StartExternalSyncSyncMode::Online,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for StartExternalSyncSyncMode {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for StartExternalSyncSyncMode {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for StartExternalSyncSyncMode {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "EXTERNAL_SYNC_MODE_UNSPECIFIED" => {
                                StartExternalSyncSyncMode::ExternalSyncModeUnspecified
                            }
                            "OFFLINE" => StartExternalSyncSyncMode::Offline,
                            "ONLINE" => StartExternalSyncSyncMode::Online,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for StartExternalSyncSyncMode {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for StartExternalSyncSyncMode {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum VerifyExternalSyncSettingsSyncMode {
                    ExternalSyncModeUnspecified,
                    Offline,
                    Online,
                }
                impl VerifyExternalSyncSettingsSyncMode {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            VerifyExternalSyncSettingsSyncMode::ExternalSyncModeUnspecified => {
                                "EXTERNAL_SYNC_MODE_UNSPECIFIED"
                            }
                            VerifyExternalSyncSettingsSyncMode::Offline => "OFFLINE",
                            VerifyExternalSyncSettingsSyncMode::Online => "ONLINE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for VerifyExternalSyncSettingsSyncMode {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for VerifyExternalSyncSettingsSyncMode {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<VerifyExternalSyncSettingsSyncMode, ()>
                    {
                        Ok(match s {
                            "EXTERNAL_SYNC_MODE_UNSPECIFIED" => {
                                VerifyExternalSyncSettingsSyncMode::ExternalSyncModeUnspecified
                            }
                            "OFFLINE" => VerifyExternalSyncSettingsSyncMode::Offline,
                            "ONLINE" => VerifyExternalSyncSettingsSyncMode::Online,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for VerifyExternalSyncSettingsSyncMode {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for VerifyExternalSyncSettingsSyncMode {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for VerifyExternalSyncSettingsSyncMode {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "EXTERNAL_SYNC_MODE_UNSPECIFIED" => {
                                VerifyExternalSyncSettingsSyncMode::ExternalSyncModeUnspecified
                            }
                            "OFFLINE" => VerifyExternalSyncSettingsSyncMode::Offline,
                            "ONLINE" => VerifyExternalSyncSettingsSyncMode::Online,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for VerifyExternalSyncSettingsSyncMode {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for VerifyExternalSyncSettingsSyncMode {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct InstancesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> InstancesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Reschedules the maintenance on the given instance."]
                pub fn reschedule_maintenance(
                    &self,
                    request: crate::schemas::SqlInstancesRescheduleMaintenanceRequestBody,
                    project: impl Into<String>,
                    instance: impl Into<String>,
                ) -> RescheduleMaintenanceRequestBuilder {
                    RescheduleMaintenanceRequestBuilder {
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
                        project: project.into(),
                        instance: instance.into(),
                    }
                }
                #[doc = "Start External master migration."]
                pub fn start_external_sync(
                    &self,
                    project: impl Into<String>,
                    instance: impl Into<String>,
                ) -> StartExternalSyncRequestBuilder {
                    StartExternalSyncRequestBuilder {
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
                        project: project.into(),
                        instance: instance.into(),
                        sync_mode: None,
                    }
                }
                #[doc = "Verify External master external sync settings."]
                pub fn verify_external_sync_settings(
                    &self,
                    project: impl Into<String>,
                    instance: impl Into<String>,
                ) -> VerifyExternalSyncSettingsRequestBuilder {
                    VerifyExternalSyncSettingsRequestBuilder {
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
                        project: project.into(),
                        instance: instance.into(),
                        sync_mode: None,
                        verify_connection_only: None,
                    }
                }
            }
            #[doc = "Created via [InstancesActions::reschedule_maintenance()](struct.InstancesActions.html#method.reschedule_maintenance)"]
            #[derive(Debug, Clone)]
            pub struct RescheduleMaintenanceRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SqlInstancesRescheduleMaintenanceRequestBody,
                project: String,
                instance: String,
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
            impl<'a> RescheduleMaintenanceRequestBuilder<'a> {
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
                    let mut output = "https://sqladmin.googleapis.com/".to_owned();
                    output.push_str("sql/v1beta4/projects/");
                    {
                        let var_as_str = &self.project;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/instances/");
                    {
                        let var_as_str = &self.instance;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/rescheduleMaintenance");
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
            #[doc = "Created via [InstancesActions::start_external_sync()](struct.InstancesActions.html#method.start_external_sync)"]
            #[derive(Debug, Clone)]
            pub struct StartExternalSyncRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project: String,
                instance: String,
                sync_mode: Option<
                    crate::resources::projects::instances::params::StartExternalSyncSyncMode,
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
            impl<'a> StartExternalSyncRequestBuilder<'a> {
                #[doc = "External sync mode"]
                pub fn sync_mode(
                    mut self,
                    value: crate::resources::projects::instances::params::StartExternalSyncSyncMode,
                ) -> Self {
                    self.sync_mode = Some(value);
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
                    let mut output = "https://sqladmin.googleapis.com/".to_owned();
                    output.push_str("sql/v1beta4/projects/");
                    {
                        let var_as_str = &self.project;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/instances/");
                    {
                        let var_as_str = &self.instance;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/startExternalSync");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("syncMode", &self.sync_mode)]);
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
            #[doc = "Created via [InstancesActions::verify_external_sync_settings()](struct.InstancesActions.html#method.verify_external_sync_settings)"]
            #[derive(Debug, Clone)]
            pub struct VerifyExternalSyncSettingsRequestBuilder < 'a > { pub ( crate ) reqwest : & 'a :: reqwest :: blocking :: Client , pub ( crate ) auth : & 'a dyn :: google_api_auth :: GetAccessToken , project : String , instance : String , sync_mode : Option < crate :: resources :: projects :: instances :: params :: VerifyExternalSyncSettingsSyncMode > , verify_connection_only : Option < bool > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
            impl<'a> VerifyExternalSyncSettingsRequestBuilder<'a> {
                #[doc = "External sync mode"]
                pub fn sync_mode(
                    mut self,
                    value : crate :: resources :: projects :: instances :: params :: VerifyExternalSyncSettingsSyncMode,
                ) -> Self {
                    self.sync_mode = Some(value);
                    self
                }
                #[doc = "Flag to enable verifying connection only"]
                pub fn verify_connection_only(mut self, value: bool) -> Self {
                    self.verify_connection_only = Some(value);
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
                ) -> Result<
                    crate::schemas::SqlInstancesVerifyExternalSyncSettingsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::SqlInstancesVerifyExternalSyncSettingsResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://sqladmin.googleapis.com/".to_owned();
                    output.push_str("sql/v1beta4/projects/");
                    {
                        let var_as_str = &self.project;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/instances/");
                    {
                        let var_as_str = &self.instance;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/verifyExternalSyncSettings");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("syncMode", &self.sync_mode)]);
                    let req = req.query(&[("verifyConnectionOnly", &self.verify_connection_only)]);
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
    pub mod ssl_certs {
        pub mod params {}
        pub struct SslCertsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SslCertsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Generates a short-lived X509 certificate containing the provided public key\nand signed by a private key specific to the target instance. Users may use\nthe certificate to authenticate as themselves when connecting to the\ndatabase."]
            pub fn create_ephemeral(
                &self,
                request: crate::schemas::SslCertsCreateEphemeralRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> CreateEphemeralRequestBuilder {
                CreateEphemeralRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Deletes the SSL certificate. For First Generation instances, the\ncertificate remains valid until the instance is restarted."]
            pub fn delete(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
                sha_1_fingerprint: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    sha_1_fingerprint: sha_1_fingerprint.into(),
                }
            }
            #[doc = "Retrieves a particular SSL certificate.  Does not include the private key\n(required for usage).  The private key must be saved from the response to\ninitial creation."]
            pub fn get(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
                sha_1_fingerprint: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    sha_1_fingerprint: sha_1_fingerprint.into(),
                }
            }
            #[doc = "Creates an SSL certificate and returns it along with the private key and\nserver certificate authority.  The new certificate will not be usable until\nthe instance is restarted."]
            pub fn insert(
                &self,
                request: crate::schemas::SslCertsInsertRequest,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Lists all of the current SSL certificates for the instance."]
            pub fn list(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
        }
        #[doc = "Created via [SslCertsActions::create_ephemeral()](struct.SslCertsActions.html#method.create_ephemeral)"]
        #[derive(Debug, Clone)]
        pub struct CreateEphemeralRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SslCertsCreateEphemeralRequest,
            project: String,
            instance: String,
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
        impl<'a> CreateEphemeralRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SslCert, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::SslCert, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/createEphemeral");
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
        #[doc = "Created via [SslCertsActions::delete()](struct.SslCertsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            sha_1_fingerprint: String,
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sslCerts/");
                {
                    let var_as_str = &self.sha_1_fingerprint;
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
        #[doc = "Created via [SslCertsActions::get()](struct.SslCertsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            sha_1_fingerprint: String,
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
            ) -> Result<crate::schemas::SslCert, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::SslCert, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sslCerts/");
                {
                    let var_as_str = &self.sha_1_fingerprint;
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
        #[doc = "Created via [SslCertsActions::insert()](struct.SslCertsActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SslCertsInsertRequest,
            project: String,
            instance: String,
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
        impl<'a> InsertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SslCertsInsertResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SslCertsInsertResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sslCerts");
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
        #[doc = "Created via [SslCertsActions::list()](struct.SslCertsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
            ) -> Result<crate::schemas::SslCertsListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SslCertsListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/sslCerts");
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
    }
    pub mod tiers {
        pub mod params {}
        pub struct TiersActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> TiersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Lists all available machine types (tiers) for Cloud SQL, for example,\ndb-n1-standard-1. For related information, see <a\nhref=\"/sql/pricing\">Pricing</a>."]
            pub fn list(&self, project: impl Into<String>) -> ListRequestBuilder {
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
                    project: project.into(),
                }
            }
        }
        #[doc = "Created via [TiersActions::list()](struct.TiersActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
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
            ) -> Result<crate::schemas::TiersListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TiersListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/tiers");
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
    }
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> UsersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes a user from a Cloud SQL instance."]
            pub fn delete(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    host: None,
                    name: None,
                }
            }
            #[doc = "Creates a new user in a Cloud SQL instance."]
            pub fn insert(
                &self,
                request: crate::schemas::User,
                project: impl Into<String>,
                instance: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Lists users in the specified Cloud SQL instance."]
            pub fn list(
                &self,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                }
            }
            #[doc = "Updates an existing user in a Cloud SQL instance."]
            pub fn update(
                &self,
                request: crate::schemas::User,
                project: impl Into<String>,
                instance: impl Into<String>,
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
                    project: project.into(),
                    instance: instance.into(),
                    host: None,
                    name: None,
                }
            }
        }
        #[doc = "Created via [UsersActions::delete()](struct.UsersActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
            host: Option<String>,
            name: Option<String>,
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
            #[doc = "Host of the user in the instance."]
            pub fn host(mut self, value: impl Into<String>) -> Self {
                self.host = Some(value.into());
                self
            }
            #[doc = "Name of the user in the instance."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/users");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("host", &self.host)]);
                let req = req.query(&[("name", &self.name)]);
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
        #[doc = "Created via [UsersActions::insert()](struct.UsersActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::User,
            project: String,
            instance: String,
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
        impl<'a> InsertRequestBuilder<'a> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/users");
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
        #[doc = "Created via [UsersActions::list()](struct.UsersActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            instance: String,
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
            ) -> Result<crate::schemas::UsersListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UsersListResponse, crate::Error> {
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/users");
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
        #[doc = "Created via [UsersActions::update()](struct.UsersActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::User,
            project: String,
            instance: String,
            host: Option<String>,
            name: Option<String>,
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
            #[doc = "Optional. Host of the user in the instance."]
            pub fn host(mut self, value: impl Into<String>) -> Self {
                self.host = Some(value.into());
                self
            }
            #[doc = "Name of the user in the instance."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
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
                let mut output = "https://sqladmin.googleapis.com/".to_owned();
                output.push_str("sql/v1beta4/projects/");
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/instances/");
                {
                    let var_as_str = &self.instance;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/users");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("host", &self.host)]);
                let req = req.query(&[("name", &self.name)]);
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
