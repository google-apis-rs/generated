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
    pub struct BucketBilling {
        #[doc = "When set to true, Requester Pays is enabled for this bucket."]
        #[serde(rename = "requesterPays", default)]
        pub requester_pays: Option<bool>,
    }
    impl ::field_selector::FieldSelector for BucketBilling {
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
    pub struct BucketCorsItems {
        #[doc = "The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses."]
        #[serde(rename = "maxAgeSeconds", default)]
        pub max_age_seconds: Option<i32>,
        #[doc = "The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
        #[serde(rename = "method", default)]
        pub method: Option<Vec<String>>,
        #[doc = "The list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
        #[serde(rename = "origin", default)]
        pub origin: Option<Vec<String>>,
        #[doc = "The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
        #[serde(rename = "responseHeader", default)]
        pub response_header: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for BucketCorsItems {
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
    pub struct BucketEncryption {
        #[doc = "A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified."]
        #[serde(rename = "defaultKmsKeyName", default)]
        pub default_kms_key_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketEncryption {
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
    pub struct BucketIamConfigurationBucketPolicyOnly {
        #[doc = "If set, access is controlled only by bucket-level or above IAM policies."]
        #[serde(rename = "enabled", default)]
        pub enabled: Option<bool>,
        #[doc = "The deadline for changing iamConfiguration.bucketPolicyOnly.enabled from true to false in RFC 3339 format. iamConfiguration.bucketPolicyOnly.enabled may be changed from true to false until the locked time, after which the field is immutable."]
        #[serde(rename = "lockedTime", default)]
        pub locked_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for BucketIamConfigurationBucketPolicyOnly {
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
    pub struct BucketIamConfigurationUniformBucketLevelAccess {
        #[doc = "If set, access is controlled only by bucket-level or above IAM policies."]
        #[serde(rename = "enabled", default)]
        pub enabled: Option<bool>,
        #[doc = "The deadline for changing iamConfiguration.uniformBucketLevelAccess.enabled from true to false in RFC 3339  format. iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until the locked time, after which the field is immutable."]
        #[serde(rename = "lockedTime", default)]
        pub locked_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for BucketIamConfigurationUniformBucketLevelAccess {
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
    pub struct BucketIamConfiguration {
        #[doc = "The bucket's Bucket Policy Only configuration."]
        #[serde(rename = "bucketPolicyOnly", default)]
        pub bucket_policy_only: Option<crate::schemas::BucketIamConfigurationBucketPolicyOnly>,
        #[doc = "The bucket's uniform bucket-level access configuration."]
        #[serde(rename = "uniformBucketLevelAccess", default)]
        pub uniform_bucket_level_access:
            Option<crate::schemas::BucketIamConfigurationUniformBucketLevelAccess>,
    }
    impl ::field_selector::FieldSelector for BucketIamConfiguration {
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
    pub struct BucketLifecycleRuleItemsAction {
        #[doc = "Type of the action. Currently, only Delete and SetStorageClass are supported."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "Target storage class. Required iff the type of the action is SetStorageClass."]
        #[serde(rename = "storageClass", default)]
        pub storage_class: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketLifecycleRuleItemsAction {
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
    pub struct BucketLifecycleRuleItemsCondition {
        #[doc = "Age of an object (in days). This condition is satisfied when an object reaches the specified age."]
        #[serde(rename = "age", default)]
        pub age: Option<i32>,
        #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when an object is created before midnight of the specified date in UTC."]
        #[serde(rename = "createdBefore", default)]
        pub created_before: Option<::chrono::Date<chrono::offset::Utc>>,
        #[doc = "Relevant only for versioned objects. If the value is true, this condition matches live objects; if the value is false, it matches archived objects."]
        #[serde(rename = "isLive", default)]
        pub is_live: Option<bool>,
        #[doc = "A regular expression that satisfies the RE2 syntax. This condition is satisfied when the name of the object matches the RE2 pattern. Note: This feature is currently in the \"Early Access\" launch stage and is only available to a whitelisted set of users; that means that this feature may be changed in backward-incompatible ways and that it is not guaranteed to be released."]
        #[serde(rename = "matchesPattern", default)]
        pub matches_pattern: Option<String>,
        #[doc = "Objects having any of the storage classes specified by this condition will be matched. Values include MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, STANDARD, and DURABLE_REDUCED_AVAILABILITY."]
        #[serde(rename = "matchesStorageClass", default)]
        pub matches_storage_class: Option<Vec<String>>,
        #[doc = "Relevant only for versioned objects. If the value is N, this condition is satisfied when there are at least N versions (including the live version) newer than this version of the object."]
        #[serde(rename = "numNewerVersions", default)]
        pub num_newer_versions: Option<i32>,
    }
    impl ::field_selector::FieldSelector for BucketLifecycleRuleItemsCondition {
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
    pub struct BucketLifecycleRuleItems {
        #[doc = "The action to take."]
        #[serde(rename = "action", default)]
        pub action: Option<crate::schemas::BucketLifecycleRuleItemsAction>,
        #[doc = "The condition(s) under which the action will be taken."]
        #[serde(rename = "condition", default)]
        pub condition: Option<crate::schemas::BucketLifecycleRuleItemsCondition>,
    }
    impl ::field_selector::FieldSelector for BucketLifecycleRuleItems {
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
    pub struct BucketLifecycle {
        #[doc = "A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken."]
        #[serde(rename = "rule", default)]
        pub rule: Option<Vec<crate::schemas::BucketLifecycleRuleItems>>,
    }
    impl ::field_selector::FieldSelector for BucketLifecycle {
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
    pub struct BucketLogging {
        #[doc = "The destination bucket where the current bucket's logs should be placed."]
        #[serde(rename = "logBucket", default)]
        pub log_bucket: Option<String>,
        #[doc = "A prefix for log object names."]
        #[serde(rename = "logObjectPrefix", default)]
        pub log_object_prefix: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketLogging {
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
    pub struct BucketOwner {
        #[doc = "The entity, in the form project-owner-projectId."]
        #[serde(rename = "entity", default)]
        pub entity: Option<String>,
        #[doc = "The ID for the entity."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketOwner {
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
    pub struct BucketRetentionPolicy {
        #[doc = "Server-determined value that indicates the time from which policy was enforced and effective. This value is in RFC 3339 format."]
        #[serde(rename = "effectiveTime", default)]
        pub effective_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Once locked, an object retention policy cannot be modified."]
        #[serde(rename = "isLocked", default)]
        pub is_locked: Option<bool>,
        #[doc = "The duration in seconds that objects need to be retained. Retention duration must be greater than zero and less than 100 years. Note that enforcement of retention periods less than a day is not guaranteed. Such periods should only be used for testing purposes."]
        #[serde(rename = "retentionPeriod", default)]
        #[serde(with = "crate::parsed_string")]
        pub retention_period: Option<i64>,
    }
    impl ::field_selector::FieldSelector for BucketRetentionPolicy {
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
    pub struct BucketVersioning {
        #[doc = "While set to true, versioning is fully enabled for this bucket."]
        #[serde(rename = "enabled", default)]
        pub enabled: Option<bool>,
    }
    impl ::field_selector::FieldSelector for BucketVersioning {
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
    pub struct BucketWebsite {
        #[doc = "If the requested object path is missing, the service will ensure the path has a trailing '/', append this suffix, and attempt to retrieve the resulting object. This allows the creation of index.html objects to represent directory pages."]
        #[serde(rename = "mainPageSuffix", default)]
        pub main_page_suffix: Option<String>,
        #[doc = "If the requested object path is missing, and any mainPageSuffix object is missing, if applicable, the service will return the named object from this bucket as the content for a 404 Not Found result."]
        #[serde(rename = "notFoundPage", default)]
        pub not_found_page: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketWebsite {
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
    pub struct Bucket {
        #[doc = "Access controls on the bucket."]
        #[serde(rename = "acl", default)]
        pub acl: Option<Vec<crate::schemas::BucketAccessControl>>,
        #[doc = "The bucket's billing configuration."]
        #[serde(rename = "billing", default)]
        pub billing: Option<crate::schemas::BucketBilling>,
        #[doc = "The bucket's Cross-Origin Resource Sharing (CORS) configuration."]
        #[serde(rename = "cors", default)]
        pub cors: Option<Vec<crate::schemas::BucketCorsItems>>,
        #[doc = "The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed."]
        #[serde(rename = "defaultEventBasedHold", default)]
        pub default_event_based_hold: Option<bool>,
        #[doc = "Default access controls to apply to new objects when no ACL is provided."]
        #[serde(rename = "defaultObjectAcl", default)]
        pub default_object_acl: Option<Vec<crate::schemas::ObjectAccessControl>>,
        #[doc = "Encryption configuration for a bucket."]
        #[serde(rename = "encryption", default)]
        pub encryption: Option<crate::schemas::BucketEncryption>,
        #[doc = "HTTP 1.1 Entity tag for the bucket."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The bucket's IAM configuration."]
        #[serde(rename = "iamConfiguration", default)]
        pub iam_configuration: Option<crate::schemas::BucketIamConfiguration>,
        #[doc = "The ID of the bucket. For buckets, the id and name properties are the same."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of item this is. For buckets, this is always storage#bucket."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "User-provided labels, in key/value pairs."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The bucket's lifecycle configuration. See lifecycle management for more information."]
        #[serde(rename = "lifecycle", default)]
        pub lifecycle: Option<crate::schemas::BucketLifecycle>,
        #[doc = "The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer's guide for the authoritative list."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The type of the bucket location."]
        #[serde(rename = "locationType", default)]
        pub location_type: Option<String>,
        #[doc = "The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs."]
        #[serde(rename = "logging", default)]
        pub logging: Option<crate::schemas::BucketLogging>,
        #[doc = "The metadata generation of this bucket."]
        #[serde(rename = "metageneration", default)]
        #[serde(with = "crate::parsed_string")]
        pub metageneration: Option<i64>,
        #[doc = "The name of the bucket."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The owner of the bucket. This is always the project team's owner group."]
        #[serde(rename = "owner", default)]
        pub owner: Option<crate::schemas::BucketOwner>,
        #[doc = "The project number of the project the bucket belongs to."]
        #[serde(rename = "projectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub project_number: Option<u64>,
        #[doc = "The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error."]
        #[serde(rename = "retentionPolicy", default)]
        pub retention_policy: Option<crate::schemas::BucketRetentionPolicy>,
        #[doc = "The URI of this bucket."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
        #[doc = "The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see storage classes."]
        #[serde(rename = "storageClass", default)]
        pub storage_class: Option<String>,
        #[doc = "The creation time of the bucket in RFC 3339 format."]
        #[serde(rename = "timeCreated", default)]
        pub time_created: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The modification time of the bucket in RFC 3339 format."]
        #[serde(rename = "updated", default)]
        pub updated: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The bucket's versioning configuration."]
        #[serde(rename = "versioning", default)]
        pub versioning: Option<crate::schemas::BucketVersioning>,
        #[doc = "The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information."]
        #[serde(rename = "website", default)]
        pub website: Option<crate::schemas::BucketWebsite>,
    }
    impl ::field_selector::FieldSelector for Bucket {
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
    pub struct BucketAccessControlProjectTeam {
        #[doc = "The project number."]
        #[serde(rename = "projectNumber", default)]
        pub project_number: Option<String>,
        #[doc = "The team."]
        #[serde(rename = "team", default)]
        pub team: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketAccessControlProjectTeam {
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
    pub struct BucketAccessControl {
        #[doc = "The name of the bucket."]
        #[serde(rename = "bucket", default)]
        pub bucket: Option<String>,
        #[doc = "The domain associated with the entity, if any."]
        #[serde(rename = "domain", default)]
        pub domain: Option<String>,
        #[doc = "The email address associated with the entity, if any."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The entity holding the permission, in one of the following forms: \n- user-userId \n- user-email \n- group-groupId \n- group-email \n- domain-domain \n- project-team-projectId \n- allUsers \n- allAuthenticatedUsers Examples: \n- The user liz@example.com would be user-liz@example.com. \n- The group example@googlegroups.com would be group-example@googlegroups.com. \n- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com."]
        #[serde(rename = "entity", default)]
        pub entity: Option<String>,
        #[doc = "The ID for the entity, if any."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "HTTP 1.1 Entity tag for the access-control entry."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The ID of the access-control entry."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The project team associated with the entity, if any."]
        #[serde(rename = "projectTeam", default)]
        pub project_team: Option<crate::schemas::BucketAccessControlProjectTeam>,
        #[doc = "The access permission for the entity."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
        #[doc = "The link to this access-control entry."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketAccessControl {
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
    pub struct BucketAccessControls {
        #[doc = "The list of items."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::BucketAccessControl>>,
        #[doc = "The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for BucketAccessControls {
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
    pub struct Buckets {
        #[doc = "The list of items."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Bucket>>,
        #[doc = "The kind of item this is. For lists of buckets, this is always storage#buckets."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for Buckets {
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
    pub struct Channel {
        #[doc = "The address where notifications are delivered for this channel."]
        #[serde(rename = "address", default)]
        pub address: Option<String>,
        #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
        #[serde(rename = "expiration", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration: Option<i64>,
        #[doc = "A UUID or similar unique string that identifies this channel."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
        #[serde(rename = "params", default)]
        pub params: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
        #[serde(rename = "payload", default)]
        pub payload: Option<bool>,
        #[doc = "The type of delivery mechanism used for this channel."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
        #[serde(rename = "resourceId", default)]
        pub resource_id: Option<String>,
        #[doc = "A version-specific identifier for the watched resource."]
        #[serde(rename = "resourceUri", default)]
        pub resource_uri: Option<String>,
        #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for Channel {
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
    pub struct ComposeRequestSourceObjectsItemsObjectPreconditions {
        #[doc = "Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail."]
        #[serde(rename = "ifGenerationMatch", default)]
        #[serde(with = "crate::parsed_string")]
        pub if_generation_match: Option<i64>,
    }
    impl ::field_selector::FieldSelector for ComposeRequestSourceObjectsItemsObjectPreconditions {
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
    pub struct ComposeRequestSourceObjectsItems {
        #[doc = "The generation of this object to use as the source."]
        #[serde(rename = "generation", default)]
        #[serde(with = "crate::parsed_string")]
        pub generation: Option<i64>,
        #[doc = "The source object's name. All source objects must reside in the same bucket."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Conditions that must be met for this operation to execute."]
        #[serde(rename = "objectPreconditions", default)]
        pub object_preconditions:
            Option<crate::schemas::ComposeRequestSourceObjectsItemsObjectPreconditions>,
    }
    impl ::field_selector::FieldSelector for ComposeRequestSourceObjectsItems {
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
    pub struct ComposeRequest {
        #[doc = "Properties of the resulting object."]
        #[serde(rename = "destination", default)]
        pub destination: Option<crate::schemas::Object>,
        #[doc = "The kind of item this is."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The list of source objects that will be concatenated into a single object."]
        #[serde(rename = "sourceObjects", default)]
        pub source_objects: Option<Vec<crate::schemas::ComposeRequestSourceObjectsItems>>,
    }
    impl ::field_selector::FieldSelector for ComposeRequest {
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
    pub struct Expr {
        #[doc = "An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
        #[serde(rename = "expression", default)]
        pub expression: Option<String>,
        #[doc = "An optional string indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "An optional title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Expr {
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
    pub struct HmacKey {
        #[doc = "The kind of item this is. For HMAC keys, this is always storage#hmacKey."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Key metadata."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::HmacKeyMetadata>,
        #[doc = "HMAC secret key material."]
        #[serde(rename = "secret", default)]
        pub secret: Option<String>,
    }
    impl ::field_selector::FieldSelector for HmacKey {
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
    pub struct HmacKeyMetadata {
        #[doc = "The ID of the HMAC Key."]
        #[serde(rename = "accessId", default)]
        pub access_id: Option<String>,
        #[doc = "HTTP 1.1 Entity tag for the HMAC key."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The ID of the HMAC key, including the Project ID and the Access ID."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of item this is. For HMAC Key metadata, this is always storage#hmacKeyMetadata."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Project ID owning the service account to which the key authenticates."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "The link to this resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
        #[doc = "The email address of the key's associated service account."]
        #[serde(rename = "serviceAccountEmail", default)]
        pub service_account_email: Option<String>,
        #[doc = "The state of the key. Can be one of ACTIVE, INACTIVE, or DELETED."]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[doc = "The creation time of the HMAC key in RFC 3339 format."]
        #[serde(rename = "timeCreated", default)]
        pub time_created: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The last modification time of the HMAC key metadata in RFC 3339 format."]
        #[serde(rename = "updated", default)]
        pub updated: Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for HmacKeyMetadata {
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
    pub struct HmacKeysMetadata {
        #[doc = "The list of items."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::HmacKeyMetadata>>,
        #[doc = "The kind of item this is. For lists of hmacKeys, this is always storage#hmacKeysMetadata."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for HmacKeysMetadata {
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
    pub struct Notification {
        #[doc = "An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription."]
        #[serde(rename = "custom_attributes", default)]
        pub custom_attributes: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "HTTP 1.1 Entity tag for this subscription notification."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "If present, only send notifications about listed event types. If empty, sent notifications for all event types."]
        #[serde(rename = "event_types", default)]
        pub event_types: Option<Vec<String>>,
        #[doc = "The ID of the notification."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of item this is. For notifications, this is always storage#notification."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "If present, only apply this notification configuration to object names that begin with this prefix."]
        #[serde(rename = "object_name_prefix", default)]
        pub object_name_prefix: Option<String>,
        #[doc = "The desired content of the Payload."]
        #[serde(rename = "payload_format", default)]
        pub payload_format: Option<String>,
        #[doc = "The canonical URL of this notification."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
        #[doc = "The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'"]
        #[serde(rename = "topic", default)]
        pub topic: Option<String>,
    }
    impl ::field_selector::FieldSelector for Notification {
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
    pub struct Notifications {
        #[doc = "The list of items."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Notification>>,
        #[doc = "The kind of item this is. For lists of notifications, this is always storage#notifications."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for Notifications {
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
    pub struct ObjectCustomerEncryption {
        #[doc = "The encryption algorithm."]
        #[serde(rename = "encryptionAlgorithm", default)]
        pub encryption_algorithm: Option<String>,
        #[doc = "SHA256 hash value of the encryption key."]
        #[serde(rename = "keySha256", default)]
        pub key_sha_256: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObjectCustomerEncryption {
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
    pub struct ObjectOwner {
        #[doc = "The entity, in the form user-userId."]
        #[serde(rename = "entity", default)]
        pub entity: Option<String>,
        #[doc = "The ID for the entity."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObjectOwner {
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
    pub struct Object {
        #[doc = "Access controls on the object."]
        #[serde(rename = "acl", default)]
        pub acl: Option<Vec<crate::schemas::ObjectAccessControl>>,
        #[doc = "The name of the bucket containing this object."]
        #[serde(rename = "bucket", default)]
        pub bucket: Option<String>,
        #[doc = "Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600."]
        #[serde(rename = "cacheControl", default)]
        pub cache_control: Option<String>,
        #[doc = "Number of underlying components that make up this object. Components are accumulated by compose operations."]
        #[serde(rename = "componentCount", default)]
        pub component_count: Option<i32>,
        #[doc = "Content-Disposition of the object data."]
        #[serde(rename = "contentDisposition", default)]
        pub content_disposition: Option<String>,
        #[doc = "Content-Encoding of the object data."]
        #[serde(rename = "contentEncoding", default)]
        pub content_encoding: Option<String>,
        #[doc = "Content-Language of the object data."]
        #[serde(rename = "contentLanguage", default)]
        pub content_language: Option<String>,
        #[doc = "Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream."]
        #[serde(rename = "contentType", default)]
        pub content_type: Option<String>,
        #[doc = "CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best Practices."]
        #[serde(rename = "crc32c", default)]
        pub crc_3_2c: Option<String>,
        #[doc = "Metadata of customer-supplied encryption key, if the object is encrypted by such a key."]
        #[serde(rename = "customerEncryption", default)]
        pub customer_encryption: Option<crate::schemas::ObjectCustomerEncryption>,
        #[doc = "HTTP 1.1 Entity tag for the object."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false."]
        #[serde(rename = "eventBasedHold", default)]
        pub event_based_hold: Option<bool>,
        #[doc = "The content generation of this object. Used for object versioning."]
        #[serde(rename = "generation", default)]
        #[serde(with = "crate::parsed_string")]
        pub generation: Option<i64>,
        #[doc = "The ID of the object, including the bucket name, object name, and generation number."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of item this is. For objects, this is always storage#object."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Cloud KMS Key used to encrypt this object, if the object is encrypted by such a key."]
        #[serde(rename = "kmsKeyName", default)]
        pub kms_key_name: Option<String>,
        #[doc = "MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see Hashes and ETags: Best Practices."]
        #[serde(rename = "md5Hash", default)]
        pub md_5_hash: Option<String>,
        #[doc = "Media download link."]
        #[serde(rename = "mediaLink", default)]
        pub media_link: Option<String>,
        #[doc = "User-provided metadata, in key/value pairs."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object."]
        #[serde(rename = "metageneration", default)]
        #[serde(with = "crate::parsed_string")]
        pub metageneration: Option<i64>,
        #[doc = "The name of the object. Required if not specified by URL parameter."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The owner of the object. This will always be the uploader of the object."]
        #[serde(rename = "owner", default)]
        pub owner: Option<crate::schemas::ObjectOwner>,
        #[doc = "A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold)."]
        #[serde(rename = "retentionExpirationTime", default)]
        pub retention_expiration_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The link to this object."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
        #[doc = "Content-Length of the data in bytes."]
        #[serde(rename = "size", default)]
        #[serde(with = "crate::parsed_string")]
        pub size: Option<u64>,
        #[doc = "Storage class of the object."]
        #[serde(rename = "storageClass", default)]
        pub storage_class: Option<String>,
        #[doc = "Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object."]
        #[serde(rename = "temporaryHold", default)]
        pub temporary_hold: Option<bool>,
        #[doc = "The creation time of the object in RFC 3339 format."]
        #[serde(rename = "timeCreated", default)]
        pub time_created: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted."]
        #[serde(rename = "timeDeleted", default)]
        pub time_deleted: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated."]
        #[serde(rename = "timeStorageClassUpdated", default)]
        pub time_storage_class_updated: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The modification time of the object metadata in RFC 3339 format."]
        #[serde(rename = "updated", default)]
        pub updated: Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for Object {
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
    pub struct ObjectAccessControlProjectTeam {
        #[doc = "The project number."]
        #[serde(rename = "projectNumber", default)]
        pub project_number: Option<String>,
        #[doc = "The team."]
        #[serde(rename = "team", default)]
        pub team: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObjectAccessControlProjectTeam {
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
    pub struct ObjectAccessControl {
        #[doc = "The name of the bucket."]
        #[serde(rename = "bucket", default)]
        pub bucket: Option<String>,
        #[doc = "The domain associated with the entity, if any."]
        #[serde(rename = "domain", default)]
        pub domain: Option<String>,
        #[doc = "The email address associated with the entity, if any."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The entity holding the permission, in one of the following forms: \n- user-userId \n- user-email \n- group-groupId \n- group-email \n- domain-domain \n- project-team-projectId \n- allUsers \n- allAuthenticatedUsers Examples: \n- The user liz@example.com would be user-liz@example.com. \n- The group example@googlegroups.com would be group-example@googlegroups.com. \n- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com."]
        #[serde(rename = "entity", default)]
        pub entity: Option<String>,
        #[doc = "The ID for the entity, if any."]
        #[serde(rename = "entityId", default)]
        pub entity_id: Option<String>,
        #[doc = "HTTP 1.1 Entity tag for the access-control entry."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The content generation of the object, if applied to an object."]
        #[serde(rename = "generation", default)]
        #[serde(with = "crate::parsed_string")]
        pub generation: Option<i64>,
        #[doc = "The ID of the access-control entry."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of item this is. For object access control entries, this is always storage#objectAccessControl."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The name of the object, if applied to an object."]
        #[serde(rename = "object", default)]
        pub object: Option<String>,
        #[doc = "The project team associated with the entity, if any."]
        #[serde(rename = "projectTeam", default)]
        pub project_team: Option<crate::schemas::ObjectAccessControlProjectTeam>,
        #[doc = "The access permission for the entity."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
        #[doc = "The link to this access-control entry."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObjectAccessControl {
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
    pub struct ObjectAccessControls {
        #[doc = "The list of items."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::ObjectAccessControl>>,
        #[doc = "The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObjectAccessControls {
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
    pub struct Objects {
        #[doc = "The list of items."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Object>>,
        #[doc = "The kind of item this is. For lists of objects, this is always storage#objects."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter."]
        #[serde(rename = "prefixes", default)]
        pub prefixes: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Objects {
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
    pub struct PolicyBindingsItems {
        #[doc = "The condition that is associated with this binding. NOTE: an unsatisfied condition will not allow user access via current binding. Different bindings, including their conditions, are examined independently."]
        #[serde(rename = "condition", default)]
        pub condition: Option<crate::schemas::Expr>,
        #[doc = "A collection of identifiers for members who may assume the provided role. Recognized identifiers are as follows:  \n- allUsers \u{2014} A special identifier that represents anyone on the internet; with or without a Google account.  \n- allAuthenticatedUsers \u{2014} A special identifier that represents anyone who is authenticated with a Google account or a service account.  \n- user:emailid \u{2014} An email address that represents a specific account. For example, user:alice@gmail.com or user:joe@example.com.  \n- serviceAccount:emailid \u{2014} An email address that represents a service account. For example,  serviceAccount:my-other-app@appspot.gserviceaccount.com .  \n- group:emailid \u{2014} An email address that represents a Google group. For example, group:admins@example.com.  \n- domain:domain \u{2014} A Google Apps domain name that represents all the users of that domain. For example, domain:google.com or domain:example.com.  \n- projectOwner:projectid \u{2014} Owners of the given project. For example, projectOwner:my-example-project  \n- projectEditor:projectid \u{2014} Editors of the given project. For example, projectEditor:my-example-project  \n- projectViewer:projectid \u{2014} Viewers of the given project. For example, projectViewer:my-example-project"]
        #[serde(rename = "members", default)]
        pub members: Option<Vec<String>>,
        #[doc = "The role to which members belong. Two types of roles are supported: new IAM roles, which grant permissions that do not map directly to those provided by ACLs, and legacy IAM roles, which do map directly to ACL permissions. All roles are of the format roles/storage.specificRole.\nThe new IAM roles are:  \n- roles/storage.admin \u{2014} Full control of Google Cloud Storage resources.  \n- roles/storage.objectViewer \u{2014} Read-Only access to Google Cloud Storage objects.  \n- roles/storage.objectCreator \u{2014} Access to create objects in Google Cloud Storage.  \n- roles/storage.objectAdmin \u{2014} Full control of Google Cloud Storage objects.   The legacy IAM roles are:  \n- roles/storage.legacyObjectReader \u{2014} Read-only access to objects without listing. Equivalent to an ACL entry on an object with the READER role.  \n- roles/storage.legacyObjectOwner \u{2014} Read/write access to existing objects without listing. Equivalent to an ACL entry on an object with the OWNER role.  \n- roles/storage.legacyBucketReader \u{2014} Read access to buckets with object listing. Equivalent to an ACL entry on a bucket with the READER role.  \n- roles/storage.legacyBucketWriter \u{2014} Read access to buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the WRITER role.  \n- roles/storage.legacyBucketOwner \u{2014} Read and write access to existing buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the OWNER role."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
    }
    impl ::field_selector::FieldSelector for PolicyBindingsItems {
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
    pub struct Policy {
        #[doc = "An association between a role, which comes with a set of permissions, and members who may assume that role."]
        #[serde(rename = "bindings", default)]
        pub bindings: Option<Vec<crate::schemas::PolicyBindingsItems>>,
        #[doc = "HTTP 1.1  Entity tag for the policy."]
        #[serde(rename = "etag", default)]
        pub etag: Option<Vec<u8>>,
        #[doc = "The kind of item this is. For policies, this is always storage#policy. This field is ignored on input."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The ID of the resource to which this policy belongs. Will be of the form projects/_/buckets/bucket for buckets, and projects/_/buckets/bucket/objects/object for objects. A specific generation may be specified by appending #generationNumber to the end of the object name, e.g. projects/_/buckets/my-bucket/objects/data.txt#17. The current generation can be denoted with #0. This field is ignored on input."]
        #[serde(rename = "resourceId", default)]
        pub resource_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Policy {
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
    pub struct RewriteResponse {
        #[doc = "true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The kind of item this is."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The total size of the object being copied in bytes. This property is always present in the response."]
        #[serde(rename = "objectSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub object_size: Option<i64>,
        #[doc = "A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes."]
        #[serde(rename = "resource", default)]
        pub resource: Option<crate::schemas::Object>,
        #[doc = "A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy."]
        #[serde(rename = "rewriteToken", default)]
        pub rewrite_token: Option<String>,
        #[doc = "The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response."]
        #[serde(rename = "totalBytesRewritten", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_rewritten: Option<i64>,
    }
    impl ::field_selector::FieldSelector for RewriteResponse {
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
    pub struct ServiceAccount {
        #[doc = "The ID of the notification."]
        #[serde(rename = "email_address", default)]
        pub email_address: Option<String>,
        #[doc = "The kind of item this is. For notifications, this is always storage#notification."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for ServiceAccount {
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
    pub struct TestIamPermissionsResponse {
        #[doc = "The kind of item this is."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The permissions held by the caller. Permissions are always of the format storage.resource.capability, where resource is one of buckets or objects. The supported permissions are as follows:  \n- storage.buckets.delete \u{2014} Delete bucket.  \n- storage.buckets.get \u{2014} Read bucket metadata.  \n- storage.buckets.getIamPolicy \u{2014} Read bucket IAM policy.  \n- storage.buckets.create \u{2014} Create bucket.  \n- storage.buckets.list \u{2014} List buckets.  \n- storage.buckets.setIamPolicy \u{2014} Update bucket IAM policy.  \n- storage.buckets.update \u{2014} Update bucket metadata.  \n- storage.objects.delete \u{2014} Delete object.  \n- storage.objects.get \u{2014} Read object data and metadata.  \n- storage.objects.getIamPolicy \u{2014} Read object IAM policy.  \n- storage.objects.create \u{2014} Create object.  \n- storage.objects.list \u{2014} List objects.  \n- storage.objects.setIamPolicy \u{2014} Update object IAM policy.  \n- storage.objects.update \u{2014} Update object metadata."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TestIamPermissionsResponse {
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
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
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
    #[doc = "Actions that can be performed on the bucket_access_controls resource"]
    pub fn bucket_access_controls(
        &self,
    ) -> crate::bucket_access_controls::BucketAccessControlsActions<A> {
        crate::bucket_access_controls::BucketAccessControlsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the buckets resource"]
    pub fn buckets(&self) -> crate::buckets::BucketsActions<A> {
        crate::buckets::BucketsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the channels resource"]
    pub fn channels(&self) -> crate::channels::ChannelsActions<A> {
        crate::channels::ChannelsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the default_object_access_controls resource"]
    pub fn default_object_access_controls(
        &self,
    ) -> crate::default_object_access_controls::DefaultObjectAccessControlsActions<A> {
        crate::default_object_access_controls::DefaultObjectAccessControlsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the notifications resource"]
    pub fn notifications(&self) -> crate::notifications::NotificationsActions<A> {
        crate::notifications::NotificationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the object_access_controls resource"]
    pub fn object_access_controls(
        &self,
    ) -> crate::object_access_controls::ObjectAccessControlsActions<A> {
        crate::object_access_controls::ObjectAccessControlsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the objects resource"]
    pub fn objects(&self) -> crate::objects::ObjectsActions<A> {
        crate::objects::ObjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::projects::ProjectsActions<A> {
        crate::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod bucket_access_controls {
    pub mod params {}
    pub struct BucketAccessControlsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> BucketAccessControlsActions<'a, A> {
        #[doc = "Permanently deletes the ACL entry for the specified entity on the specified bucket."]
        pub fn delete(
            &self,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Returns the ACL entry for the specified entity on the specified bucket."]
        pub fn get(
            &self,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Creates a new ACL entry on the specified bucket."]
        pub fn insert(
            &self,
            request: crate::schemas::BucketAccessControl,
            bucket: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves ACL entries on the specified bucket."]
        pub fn list(&self, bucket: impl Into<String>) -> ListRequestBuilder<A> {
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
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Patches an ACL entry on the specified bucket."]
        pub fn patch(
            &self,
            request: crate::schemas::BucketAccessControl,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Updates an ACL entry on the specified bucket."]
        pub fn update(
            &self,
            request: crate::schemas::BucketAccessControl,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::BucketAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::BucketAccessControl,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::BucketAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/acl");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::BucketAccessControls, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/acl");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::BucketAccessControl,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::BucketAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::BucketAccessControl,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::BucketAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod buckets {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum GetProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit owner, acl and defaultObjectAcl properties."]
            NoAcl,
        }
        impl GetProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    GetProjection::Full => "full",
                    GetProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for GetProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for GetProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for GetProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => GetProjection::Full,
                    "noAcl" => GetProjection::NoAcl,
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
        pub enum InsertPredefinedAcl {
            #[doc = "Project team owners get OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Project team owners get OWNER access."]
            Private,
            #[doc = "Project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Project team owners get OWNER access, and allUsers get READER access."]
            PublicRead,
            #[doc = "Project team owners get OWNER access, and allUsers get WRITER access."]
            PublicReadWrite,
        }
        impl InsertPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    InsertPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    InsertPredefinedAcl::Private => "private",
                    InsertPredefinedAcl::ProjectPrivate => "projectPrivate",
                    InsertPredefinedAcl::PublicRead => "publicRead",
                    InsertPredefinedAcl::PublicReadWrite => "publicReadWrite",
                }
            }
        }
        impl ::std::fmt::Display for InsertPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for InsertPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for InsertPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => InsertPredefinedAcl::AuthenticatedRead,
                    "private" => InsertPredefinedAcl::Private,
                    "projectPrivate" => InsertPredefinedAcl::ProjectPrivate,
                    "publicRead" => InsertPredefinedAcl::PublicRead,
                    "publicReadWrite" => InsertPredefinedAcl::PublicReadWrite,
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
        pub enum InsertPredefinedDefaultObjectAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl InsertPredefinedDefaultObjectAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    InsertPredefinedDefaultObjectAcl::AuthenticatedRead => "authenticatedRead",
                    InsertPredefinedDefaultObjectAcl::BucketOwnerFullControl => {
                        "bucketOwnerFullControl"
                    }
                    InsertPredefinedDefaultObjectAcl::BucketOwnerRead => "bucketOwnerRead",
                    InsertPredefinedDefaultObjectAcl::Private => "private",
                    InsertPredefinedDefaultObjectAcl::ProjectPrivate => "projectPrivate",
                    InsertPredefinedDefaultObjectAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for InsertPredefinedDefaultObjectAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for InsertPredefinedDefaultObjectAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for InsertPredefinedDefaultObjectAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => InsertPredefinedDefaultObjectAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => {
                        InsertPredefinedDefaultObjectAcl::BucketOwnerFullControl
                    }
                    "bucketOwnerRead" => InsertPredefinedDefaultObjectAcl::BucketOwnerRead,
                    "private" => InsertPredefinedDefaultObjectAcl::Private,
                    "projectPrivate" => InsertPredefinedDefaultObjectAcl::ProjectPrivate,
                    "publicRead" => InsertPredefinedDefaultObjectAcl::PublicRead,
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
        pub enum InsertProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit owner, acl and defaultObjectAcl properties."]
            NoAcl,
        }
        impl InsertProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    InsertProjection::Full => "full",
                    InsertProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for InsertProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for InsertProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for InsertProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => InsertProjection::Full,
                    "noAcl" => InsertProjection::NoAcl,
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
        pub enum ListProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit owner, acl and defaultObjectAcl properties."]
            NoAcl,
        }
        impl ListProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListProjection::Full => "full",
                    ListProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for ListProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => ListProjection::Full,
                    "noAcl" => ListProjection::NoAcl,
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
        pub enum PatchPredefinedAcl {
            #[doc = "Project team owners get OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Project team owners get OWNER access."]
            Private,
            #[doc = "Project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Project team owners get OWNER access, and allUsers get READER access."]
            PublicRead,
            #[doc = "Project team owners get OWNER access, and allUsers get WRITER access."]
            PublicReadWrite,
        }
        impl PatchPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    PatchPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    PatchPredefinedAcl::Private => "private",
                    PatchPredefinedAcl::ProjectPrivate => "projectPrivate",
                    PatchPredefinedAcl::PublicRead => "publicRead",
                    PatchPredefinedAcl::PublicReadWrite => "publicReadWrite",
                }
            }
        }
        impl ::std::fmt::Display for PatchPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for PatchPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for PatchPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => PatchPredefinedAcl::AuthenticatedRead,
                    "private" => PatchPredefinedAcl::Private,
                    "projectPrivate" => PatchPredefinedAcl::ProjectPrivate,
                    "publicRead" => PatchPredefinedAcl::PublicRead,
                    "publicReadWrite" => PatchPredefinedAcl::PublicReadWrite,
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
        pub enum PatchPredefinedDefaultObjectAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl PatchPredefinedDefaultObjectAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    PatchPredefinedDefaultObjectAcl::AuthenticatedRead => "authenticatedRead",
                    PatchPredefinedDefaultObjectAcl::BucketOwnerFullControl => {
                        "bucketOwnerFullControl"
                    }
                    PatchPredefinedDefaultObjectAcl::BucketOwnerRead => "bucketOwnerRead",
                    PatchPredefinedDefaultObjectAcl::Private => "private",
                    PatchPredefinedDefaultObjectAcl::ProjectPrivate => "projectPrivate",
                    PatchPredefinedDefaultObjectAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for PatchPredefinedDefaultObjectAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for PatchPredefinedDefaultObjectAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for PatchPredefinedDefaultObjectAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => PatchPredefinedDefaultObjectAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => {
                        PatchPredefinedDefaultObjectAcl::BucketOwnerFullControl
                    }
                    "bucketOwnerRead" => PatchPredefinedDefaultObjectAcl::BucketOwnerRead,
                    "private" => PatchPredefinedDefaultObjectAcl::Private,
                    "projectPrivate" => PatchPredefinedDefaultObjectAcl::ProjectPrivate,
                    "publicRead" => PatchPredefinedDefaultObjectAcl::PublicRead,
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
        pub enum PatchProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit owner, acl and defaultObjectAcl properties."]
            NoAcl,
        }
        impl PatchProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    PatchProjection::Full => "full",
                    PatchProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for PatchProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for PatchProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for PatchProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => PatchProjection::Full,
                    "noAcl" => PatchProjection::NoAcl,
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
        pub enum UpdatePredefinedAcl {
            #[doc = "Project team owners get OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Project team owners get OWNER access."]
            Private,
            #[doc = "Project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Project team owners get OWNER access, and allUsers get READER access."]
            PublicRead,
            #[doc = "Project team owners get OWNER access, and allUsers get WRITER access."]
            PublicReadWrite,
        }
        impl UpdatePredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    UpdatePredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    UpdatePredefinedAcl::Private => "private",
                    UpdatePredefinedAcl::ProjectPrivate => "projectPrivate",
                    UpdatePredefinedAcl::PublicRead => "publicRead",
                    UpdatePredefinedAcl::PublicReadWrite => "publicReadWrite",
                }
            }
        }
        impl ::std::fmt::Display for UpdatePredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UpdatePredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UpdatePredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => UpdatePredefinedAcl::AuthenticatedRead,
                    "private" => UpdatePredefinedAcl::Private,
                    "projectPrivate" => UpdatePredefinedAcl::ProjectPrivate,
                    "publicRead" => UpdatePredefinedAcl::PublicRead,
                    "publicReadWrite" => UpdatePredefinedAcl::PublicReadWrite,
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
        pub enum UpdatePredefinedDefaultObjectAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl UpdatePredefinedDefaultObjectAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    UpdatePredefinedDefaultObjectAcl::AuthenticatedRead => "authenticatedRead",
                    UpdatePredefinedDefaultObjectAcl::BucketOwnerFullControl => {
                        "bucketOwnerFullControl"
                    }
                    UpdatePredefinedDefaultObjectAcl::BucketOwnerRead => "bucketOwnerRead",
                    UpdatePredefinedDefaultObjectAcl::Private => "private",
                    UpdatePredefinedDefaultObjectAcl::ProjectPrivate => "projectPrivate",
                    UpdatePredefinedDefaultObjectAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for UpdatePredefinedDefaultObjectAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UpdatePredefinedDefaultObjectAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UpdatePredefinedDefaultObjectAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => UpdatePredefinedDefaultObjectAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => {
                        UpdatePredefinedDefaultObjectAcl::BucketOwnerFullControl
                    }
                    "bucketOwnerRead" => UpdatePredefinedDefaultObjectAcl::BucketOwnerRead,
                    "private" => UpdatePredefinedDefaultObjectAcl::Private,
                    "projectPrivate" => UpdatePredefinedDefaultObjectAcl::ProjectPrivate,
                    "publicRead" => UpdatePredefinedDefaultObjectAcl::PublicRead,
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
        pub enum UpdateProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit owner, acl and defaultObjectAcl properties."]
            NoAcl,
        }
        impl UpdateProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    UpdateProjection::Full => "full",
                    UpdateProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for UpdateProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UpdateProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UpdateProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => UpdateProjection::Full,
                    "noAcl" => UpdateProjection::NoAcl,
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
    pub struct BucketsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> BucketsActions<'a, A> {
        #[doc = "Permanently deletes an empty bucket."]
        pub fn delete(&self, bucket: impl Into<String>) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Returns metadata for the specified bucket."]
        pub fn get(&self, bucket: impl Into<String>) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Returns an IAM policy for the specified bucket."]
        pub fn get_iam_policy(&self, bucket: impl Into<String>) -> GetIamPolicyRequestBuilder<A> {
            GetIamPolicyRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Creates a new bucket."]
        pub fn insert(
            &self,
            request: crate::schemas::Bucket,
            project: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project: project.into(),
                predefined_acl: None,
                predefined_default_object_acl: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves a list of buckets for a given project."]
        pub fn list(&self, project: impl Into<String>) -> ListRequestBuilder<A> {
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
                project: project.into(),
                max_results: None,
                page_token: None,
                prefix: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Locks retention policy on a bucket."]
        pub fn lock_retention_policy(
            &self,
            bucket: impl Into<String>,
            if_metageneration_match: i64,
        ) -> LockRetentionPolicyRequestBuilder<A> {
            LockRetentionPolicyRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                if_metageneration_match,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Patches a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate."]
        pub fn patch(
            &self,
            request: crate::schemas::Bucket,
            bucket: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                predefined_acl: None,
                predefined_default_object_acl: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Updates an IAM policy for the specified bucket."]
        pub fn set_iam_policy(
            &self,
            request: crate::schemas::Policy,
            bucket: impl Into<String>,
        ) -> SetIamPolicyRequestBuilder<A> {
            SetIamPolicyRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Tests a set of permissions on the given bucket to see which, if any, are held by the caller."]
        pub fn test_iam_permissions(
            &self,
            bucket: impl Into<String>,
            permissions: impl Into<String>,
        ) -> TestIamPermissionsRequestBuilder<A> {
            TestIamPermissionsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                permissions: permissions.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate."]
        pub fn update(
            &self,
            request: crate::schemas::Bucket,
            bucket: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                predefined_acl: None,
                predefined_default_object_acl: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "If set, only deletes the bucket if its metageneration matches this value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "If set, only deletes the bucket if its metageneration does not match this value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        projection: Option<crate::buckets::params::GetProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl."]
        pub fn projection(&mut self, value: crate::buckets::params::GetProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute_debug(self) -> Result<crate::schemas::Bucket, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetIamPolicyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetIamPolicyRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute_debug(self) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/iam");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Bucket,
        project: String,
        predefined_acl: Option<crate::buckets::params::InsertPredefinedAcl>,
        predefined_default_object_acl:
            Option<crate::buckets::params::InsertPredefinedDefaultObjectAcl>,
        projection: Option<crate::buckets::params::InsertProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "Apply a predefined set of access controls to this bucket."]
        pub fn predefined_acl(
            &mut self,
            value: crate::buckets::params::InsertPredefinedAcl,
        ) -> &mut Self {
            self.predefined_acl = Some(value);
            self
        }
        #[doc = "Apply a predefined set of default object access controls to this bucket."]
        pub fn predefined_default_object_acl(
            &mut self,
            value: crate::buckets::params::InsertPredefinedDefaultObjectAcl,
        ) -> &mut Self {
            self.predefined_default_object_acl = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties, when it defaults to full."]
        pub fn projection(&mut self, value: crate::buckets::params::InsertProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Bucket, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("project", &self.project)]);
            let req = req.query(&[("predefinedAcl", &self.predefined_acl)]);
            let req = req.query(&[(
                "predefinedDefaultObjectAcl",
                &self.predefined_default_object_acl,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        prefix: Option<String>,
        projection: Option<crate::buckets::params::ListProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Maximum number of buckets to return in a single response. The service will use this parameter or 1,000 items, whichever is smaller."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A previously-returned page token representing part of the larger set of results to view."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Filter results to buckets whose names begin with this prefix."]
        pub fn prefix(&mut self, value: impl Into<String>) -> &mut Self {
            self.prefix = Some(value.into());
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl."]
        pub fn projection(&mut self, value: crate::buckets::params::ListProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn iter_items<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "items")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
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
        ) -> Result<crate::schemas::Buckets, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("project", &self.project)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("prefix", &self.prefix)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct LockRetentionPolicyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        if_metageneration_match: i64,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> LockRetentionPolicyRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute_debug(self) -> Result<crate::schemas::Bucket, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/lockRetentionPolicy");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Bucket,
        bucket: String,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        predefined_acl: Option<crate::buckets::params::PatchPredefinedAcl>,
        predefined_default_object_acl:
            Option<crate::buckets::params::PatchPredefinedDefaultObjectAcl>,
        projection: Option<crate::buckets::params::PatchProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Apply a predefined set of access controls to this bucket."]
        pub fn predefined_acl(
            &mut self,
            value: crate::buckets::params::PatchPredefinedAcl,
        ) -> &mut Self {
            self.predefined_acl = Some(value);
            self
        }
        #[doc = "Apply a predefined set of default object access controls to this bucket."]
        pub fn predefined_default_object_acl(
            &mut self,
            value: crate::buckets::params::PatchPredefinedDefaultObjectAcl,
        ) -> &mut Self {
            self.predefined_default_object_acl = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to full."]
        pub fn projection(&mut self, value: crate::buckets::params::PatchProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Bucket, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("predefinedAcl", &self.predefined_acl)]);
            let req = req.query(&[(
                "predefinedDefaultObjectAcl",
                &self.predefined_default_object_acl,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SetIamPolicyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Policy,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SetIamPolicyRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/iam");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct TestIamPermissionsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        permissions: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> TestIamPermissionsRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::TestIamPermissionsResponse, Box<dyn ::std::error::Error>>
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
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/iam/testPermissions");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("permissions", &self.permissions)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Bucket,
        bucket: String,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        predefined_acl: Option<crate::buckets::params::UpdatePredefinedAcl>,
        predefined_default_object_acl:
            Option<crate::buckets::params::UpdatePredefinedDefaultObjectAcl>,
        projection: Option<crate::buckets::params::UpdateProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Apply a predefined set of access controls to this bucket."]
        pub fn predefined_acl(
            &mut self,
            value: crate::buckets::params::UpdatePredefinedAcl,
        ) -> &mut Self {
            self.predefined_acl = Some(value);
            self
        }
        #[doc = "Apply a predefined set of default object access controls to this bucket."]
        pub fn predefined_default_object_acl(
            &mut self,
            value: crate::buckets::params::UpdatePredefinedDefaultObjectAcl,
        ) -> &mut Self {
            self.predefined_default_object_acl = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to full."]
        pub fn projection(&mut self, value: crate::buckets::params::UpdateProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Bucket, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("predefinedAcl", &self.predefined_acl)]);
            let req = req.query(&[(
                "predefinedDefaultObjectAcl",
                &self.predefined_default_object_acl,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod channels {
    pub mod params {}
    pub struct ChannelsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ChannelsActions<'a, A> {
        #[doc = "Stop watching resources through this channel"]
        pub fn stop(&self, request: crate::schemas::Channel) -> StopRequestBuilder<A> {
            StopRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct StopRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Channel,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> StopRequestBuilder<'a, A> {
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("channels/stop");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod default_object_access_controls {
    pub mod params {}
    pub struct DefaultObjectAccessControlsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> DefaultObjectAccessControlsActions<'a, A> {
        #[doc = "Permanently deletes the default object ACL entry for the specified entity on the specified bucket."]
        pub fn delete(
            &self,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Returns the default object ACL entry for the specified entity on the specified bucket."]
        pub fn get(
            &self,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Creates a new default object ACL entry on the specified bucket."]
        pub fn insert(
            &self,
            request: crate::schemas::ObjectAccessControl,
            bucket: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves default object ACL entries on the specified bucket."]
        pub fn list(&self, bucket: impl Into<String>) -> ListRequestBuilder<A> {
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
                bucket: bucket.into(),
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Patches a default object ACL entry on the specified bucket."]
        pub fn patch(
            &self,
            request: crate::schemas::ObjectAccessControl,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Updates a default object ACL entry on the specified bucket."]
        pub fn update(
            &self,
            request: crate::schemas::ObjectAccessControl,
            bucket: impl Into<String>,
            entity: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                entity: entity.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/defaultObjectAcl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/defaultObjectAcl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ObjectAccessControl,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/defaultObjectAcl");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "If present, only return default ACL listing if the bucket's current metageneration matches this value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "If present, only return default ACL listing if the bucket's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::ObjectAccessControls, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/defaultObjectAcl");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ObjectAccessControl,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/defaultObjectAcl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ObjectAccessControl,
        bucket: String,
        entity: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/defaultObjectAcl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod notifications {
    pub mod params {}
    pub struct NotificationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> NotificationsActions<'a, A> {
        #[doc = "Permanently deletes a notification subscription."]
        pub fn delete(
            &self,
            bucket: impl Into<String>,
            notification: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                notification: notification.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "View a notification configuration."]
        pub fn get(
            &self,
            bucket: impl Into<String>,
            notification: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                notification: notification.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Creates a notification subscription for a given bucket."]
        pub fn insert(
            &self,
            request: crate::schemas::Notification,
            bucket: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves a list of notification subscriptions for a given bucket."]
        pub fn list(&self, bucket: impl Into<String>) -> ListRequestBuilder<A> {
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
                bucket: bucket.into(),
                provisional_user_project: None,
                user_project: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        notification: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/notificationConfigs/");
            output.push_str(&self.notification);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        notification: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/notificationConfigs/");
            output.push_str(&self.notification);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Notification,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/notificationConfigs");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::Notifications, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/notificationConfigs");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod object_access_controls {
    pub mod params {}
    pub struct ObjectAccessControlsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ObjectAccessControlsActions<'a, A> {
        #[doc = "Permanently deletes the ACL entry for the specified entity on the specified object."]
        pub fn delete(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
            entity: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                entity: entity.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Returns the ACL entry for the specified entity on the specified object."]
        pub fn get(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
            entity: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                entity: entity.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Creates a new ACL entry on the specified object."]
        pub fn insert(
            &self,
            request: crate::schemas::ObjectAccessControl,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves ACL entries on the specified object."]
        pub fn list(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> ListRequestBuilder<A> {
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
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Patches an ACL entry on the specified object."]
        pub fn patch(
            &self,
            request: crate::schemas::ObjectAccessControl,
            bucket: impl Into<String>,
            object: impl Into<String>,
            entity: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                entity: entity.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Updates an ACL entry on the specified object."]
        pub fn update(
            &self,
            request: crate::schemas::ObjectAccessControl,
            bucket: impl Into<String>,
            object: impl Into<String>,
            entity: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                entity: entity.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        entity: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        entity: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ObjectAccessControl,
        bucket: String,
        object: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/acl");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::ObjectAccessControls, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/acl");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ObjectAccessControl,
        bucket: String,
        object: String,
        entity: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ObjectAccessControl,
        bucket: String,
        object: String,
        entity: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ObjectAccessControl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/acl/");
            output.push_str(&self.entity);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod objects {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum ComposeDestinationPredefinedAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl ComposeDestinationPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    ComposeDestinationPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    ComposeDestinationPredefinedAcl::BucketOwnerFullControl => {
                        "bucketOwnerFullControl"
                    }
                    ComposeDestinationPredefinedAcl::BucketOwnerRead => "bucketOwnerRead",
                    ComposeDestinationPredefinedAcl::Private => "private",
                    ComposeDestinationPredefinedAcl::ProjectPrivate => "projectPrivate",
                    ComposeDestinationPredefinedAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for ComposeDestinationPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ComposeDestinationPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ComposeDestinationPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => ComposeDestinationPredefinedAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => {
                        ComposeDestinationPredefinedAcl::BucketOwnerFullControl
                    }
                    "bucketOwnerRead" => ComposeDestinationPredefinedAcl::BucketOwnerRead,
                    "private" => ComposeDestinationPredefinedAcl::Private,
                    "projectPrivate" => ComposeDestinationPredefinedAcl::ProjectPrivate,
                    "publicRead" => ComposeDestinationPredefinedAcl::PublicRead,
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
        pub enum CopyDestinationPredefinedAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl CopyDestinationPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    CopyDestinationPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    CopyDestinationPredefinedAcl::BucketOwnerFullControl => {
                        "bucketOwnerFullControl"
                    }
                    CopyDestinationPredefinedAcl::BucketOwnerRead => "bucketOwnerRead",
                    CopyDestinationPredefinedAcl::Private => "private",
                    CopyDestinationPredefinedAcl::ProjectPrivate => "projectPrivate",
                    CopyDestinationPredefinedAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for CopyDestinationPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for CopyDestinationPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for CopyDestinationPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => CopyDestinationPredefinedAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => {
                        CopyDestinationPredefinedAcl::BucketOwnerFullControl
                    }
                    "bucketOwnerRead" => CopyDestinationPredefinedAcl::BucketOwnerRead,
                    "private" => CopyDestinationPredefinedAcl::Private,
                    "projectPrivate" => CopyDestinationPredefinedAcl::ProjectPrivate,
                    "publicRead" => CopyDestinationPredefinedAcl::PublicRead,
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
        pub enum CopyProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl CopyProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    CopyProjection::Full => "full",
                    CopyProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for CopyProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for CopyProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for CopyProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => CopyProjection::Full,
                    "noAcl" => CopyProjection::NoAcl,
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
        pub enum GetProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl GetProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    GetProjection::Full => "full",
                    GetProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for GetProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for GetProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for GetProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => GetProjection::Full,
                    "noAcl" => GetProjection::NoAcl,
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
        pub enum InsertPredefinedAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl InsertPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    InsertPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    InsertPredefinedAcl::BucketOwnerFullControl => "bucketOwnerFullControl",
                    InsertPredefinedAcl::BucketOwnerRead => "bucketOwnerRead",
                    InsertPredefinedAcl::Private => "private",
                    InsertPredefinedAcl::ProjectPrivate => "projectPrivate",
                    InsertPredefinedAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for InsertPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for InsertPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for InsertPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => InsertPredefinedAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => InsertPredefinedAcl::BucketOwnerFullControl,
                    "bucketOwnerRead" => InsertPredefinedAcl::BucketOwnerRead,
                    "private" => InsertPredefinedAcl::Private,
                    "projectPrivate" => InsertPredefinedAcl::ProjectPrivate,
                    "publicRead" => InsertPredefinedAcl::PublicRead,
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
        pub enum InsertProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl InsertProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    InsertProjection::Full => "full",
                    InsertProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for InsertProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for InsertProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for InsertProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => InsertProjection::Full,
                    "noAcl" => InsertProjection::NoAcl,
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
        pub enum ListProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl ListProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListProjection::Full => "full",
                    ListProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for ListProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => ListProjection::Full,
                    "noAcl" => ListProjection::NoAcl,
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
        pub enum PatchPredefinedAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl PatchPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    PatchPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    PatchPredefinedAcl::BucketOwnerFullControl => "bucketOwnerFullControl",
                    PatchPredefinedAcl::BucketOwnerRead => "bucketOwnerRead",
                    PatchPredefinedAcl::Private => "private",
                    PatchPredefinedAcl::ProjectPrivate => "projectPrivate",
                    PatchPredefinedAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for PatchPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for PatchPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for PatchPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => PatchPredefinedAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => PatchPredefinedAcl::BucketOwnerFullControl,
                    "bucketOwnerRead" => PatchPredefinedAcl::BucketOwnerRead,
                    "private" => PatchPredefinedAcl::Private,
                    "projectPrivate" => PatchPredefinedAcl::ProjectPrivate,
                    "publicRead" => PatchPredefinedAcl::PublicRead,
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
        pub enum PatchProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl PatchProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    PatchProjection::Full => "full",
                    PatchProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for PatchProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for PatchProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for PatchProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => PatchProjection::Full,
                    "noAcl" => PatchProjection::NoAcl,
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
        pub enum RewriteDestinationPredefinedAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl RewriteDestinationPredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    RewriteDestinationPredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    RewriteDestinationPredefinedAcl::BucketOwnerFullControl => {
                        "bucketOwnerFullControl"
                    }
                    RewriteDestinationPredefinedAcl::BucketOwnerRead => "bucketOwnerRead",
                    RewriteDestinationPredefinedAcl::Private => "private",
                    RewriteDestinationPredefinedAcl::ProjectPrivate => "projectPrivate",
                    RewriteDestinationPredefinedAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for RewriteDestinationPredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RewriteDestinationPredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RewriteDestinationPredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => RewriteDestinationPredefinedAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => {
                        RewriteDestinationPredefinedAcl::BucketOwnerFullControl
                    }
                    "bucketOwnerRead" => RewriteDestinationPredefinedAcl::BucketOwnerRead,
                    "private" => RewriteDestinationPredefinedAcl::Private,
                    "projectPrivate" => RewriteDestinationPredefinedAcl::ProjectPrivate,
                    "publicRead" => RewriteDestinationPredefinedAcl::PublicRead,
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
        pub enum RewriteProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl RewriteProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    RewriteProjection::Full => "full",
                    RewriteProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for RewriteProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RewriteProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RewriteProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => RewriteProjection::Full,
                    "noAcl" => RewriteProjection::NoAcl,
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
        pub enum UpdatePredefinedAcl {
            #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
            AuthenticatedRead,
            #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
            BucketOwnerFullControl,
            #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
            BucketOwnerRead,
            #[doc = "Object owner gets OWNER access."]
            Private,
            #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
            ProjectPrivate,
            #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
            PublicRead,
        }
        impl UpdatePredefinedAcl {
            pub fn as_str(self) -> &'static str {
                match self {
                    UpdatePredefinedAcl::AuthenticatedRead => "authenticatedRead",
                    UpdatePredefinedAcl::BucketOwnerFullControl => "bucketOwnerFullControl",
                    UpdatePredefinedAcl::BucketOwnerRead => "bucketOwnerRead",
                    UpdatePredefinedAcl::Private => "private",
                    UpdatePredefinedAcl::ProjectPrivate => "projectPrivate",
                    UpdatePredefinedAcl::PublicRead => "publicRead",
                }
            }
        }
        impl ::std::fmt::Display for UpdatePredefinedAcl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UpdatePredefinedAcl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UpdatePredefinedAcl {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "authenticatedRead" => UpdatePredefinedAcl::AuthenticatedRead,
                    "bucketOwnerFullControl" => UpdatePredefinedAcl::BucketOwnerFullControl,
                    "bucketOwnerRead" => UpdatePredefinedAcl::BucketOwnerRead,
                    "private" => UpdatePredefinedAcl::Private,
                    "projectPrivate" => UpdatePredefinedAcl::ProjectPrivate,
                    "publicRead" => UpdatePredefinedAcl::PublicRead,
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
        pub enum UpdateProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl UpdateProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    UpdateProjection::Full => "full",
                    UpdateProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for UpdateProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UpdateProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UpdateProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => UpdateProjection::Full,
                    "noAcl" => UpdateProjection::NoAcl,
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
        pub enum WatchAllProjection {
            #[doc = "Include all properties."]
            Full,
            #[doc = "Omit the owner, acl property."]
            NoAcl,
        }
        impl WatchAllProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    WatchAllProjection::Full => "full",
                    WatchAllProjection::NoAcl => "noAcl",
                }
            }
        }
        impl ::std::fmt::Display for WatchAllProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for WatchAllProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for WatchAllProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => WatchAllProjection::Full,
                    "noAcl" => WatchAllProjection::NoAcl,
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
    pub struct ObjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ObjectsActions<'a, A> {
        #[doc = "Concatenates a list of existing objects into a new object in the same bucket."]
        pub fn compose(
            &self,
            request: crate::schemas::ComposeRequest,
            destination_bucket: impl Into<String>,
            destination_object: impl Into<String>,
        ) -> ComposeRequestBuilder<A> {
            ComposeRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                destination_bucket: destination_bucket.into(),
                destination_object: destination_object.into(),
                destination_predefined_acl: None,
                if_generation_match: None,
                if_metageneration_match: None,
                kms_key_name: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Copies a source object to a destination object. Optionally overrides metadata."]
        pub fn copy(
            &self,
            request: crate::schemas::Object,
            source_bucket: impl Into<String>,
            source_object: impl Into<String>,
            destination_bucket: impl Into<String>,
            destination_object: impl Into<String>,
        ) -> CopyRequestBuilder<A> {
            CopyRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                source_bucket: source_bucket.into(),
                source_object: source_object.into(),
                destination_bucket: destination_bucket.into(),
                destination_object: destination_object.into(),
                destination_predefined_acl: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                if_source_generation_match: None,
                if_source_generation_not_match: None,
                if_source_metageneration_match: None,
                if_source_metageneration_not_match: None,
                projection: None,
                provisional_user_project: None,
                source_generation: None,
                user_project: None,
            }
        }
        #[doc = "Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used."]
        pub fn delete(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves an object or its metadata."]
        pub fn get(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Returns an IAM policy for the specified object."]
        pub fn get_iam_policy(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> GetIamPolicyRequestBuilder<A> {
            GetIamPolicyRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Stores a new object and metadata."]
        pub fn insert(
            &self,
            request: crate::schemas::Object,
            bucket: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                content_encoding: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                kms_key_name: None,
                name: None,
                predefined_acl: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Retrieves a list of objects matching the criteria."]
        pub fn list(&self, bucket: impl Into<String>) -> ListRequestBuilder<A> {
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
                bucket: bucket.into(),
                delimiter: None,
                include_trailing_delimiter: None,
                max_results: None,
                page_token: None,
                prefix: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
                versions: None,
            }
        }
        #[doc = "Patches an object's metadata."]
        pub fn patch(
            &self,
            request: crate::schemas::Object,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                predefined_acl: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Rewrites a source object to a destination object. Optionally overrides metadata."]
        pub fn rewrite(
            &self,
            request: crate::schemas::Object,
            source_bucket: impl Into<String>,
            source_object: impl Into<String>,
            destination_bucket: impl Into<String>,
            destination_object: impl Into<String>,
        ) -> RewriteRequestBuilder<A> {
            RewriteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                source_bucket: source_bucket.into(),
                source_object: source_object.into(),
                destination_bucket: destination_bucket.into(),
                destination_object: destination_object.into(),
                destination_kms_key_name: None,
                destination_predefined_acl: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                if_source_generation_match: None,
                if_source_generation_not_match: None,
                if_source_metageneration_match: None,
                if_source_metageneration_not_match: None,
                max_bytes_rewritten_per_call: None,
                projection: None,
                provisional_user_project: None,
                rewrite_token: None,
                source_generation: None,
                user_project: None,
            }
        }
        #[doc = "Updates an IAM policy for the specified object."]
        pub fn set_iam_policy(
            &self,
            request: crate::schemas::Policy,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> SetIamPolicyRequestBuilder<A> {
            SetIamPolicyRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Tests a set of permissions on the given object to see which, if any, are held by the caller."]
        pub fn test_iam_permissions(
            &self,
            bucket: impl Into<String>,
            object: impl Into<String>,
            permissions: impl Into<String>,
        ) -> TestIamPermissionsRequestBuilder<A> {
            TestIamPermissionsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                permissions: permissions.into(),
                generation: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Updates an object's metadata."]
        pub fn update(
            &self,
            request: crate::schemas::Object,
            bucket: impl Into<String>,
            object: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                object: object.into(),
                generation: None,
                if_generation_match: None,
                if_generation_not_match: None,
                if_metageneration_match: None,
                if_metageneration_not_match: None,
                predefined_acl: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
            }
        }
        #[doc = "Watch for changes on all objects in a bucket."]
        pub fn watch_all(
            &self,
            request: crate::schemas::Channel,
            bucket: impl Into<String>,
        ) -> WatchAllRequestBuilder<A> {
            WatchAllRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                bucket: bucket.into(),
                delimiter: None,
                include_trailing_delimiter: None,
                max_results: None,
                page_token: None,
                prefix: None,
                projection: None,
                provisional_user_project: None,
                user_project: None,
                versions: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ComposeRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ComposeRequest,
        destination_bucket: String,
        destination_object: String,
        destination_predefined_acl: Option<crate::objects::params::ComposeDestinationPredefinedAcl>,
        if_generation_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        kms_key_name: Option<String>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ComposeRequestBuilder<'a, A> {
        #[doc = "Apply a predefined set of access controls to the destination object."]
        pub fn destination_predefined_acl(
            &mut self,
            value: crate::objects::params::ComposeDestinationPredefinedAcl,
        ) -> &mut Self {
            self.destination_predefined_acl = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
        pub fn kms_key_name(&mut self, value: impl Into<String>) -> &mut Self {
            self.kms_key_name = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Object, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.destination_bucket);
            output.push_str("/o/");
            output.push_str(&self.destination_object);
            output.push_str("/compose");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("destinationPredefinedAcl", &self.destination_predefined_acl)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[("kmsKeyName", &self.kms_key_name)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct CopyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Object,
        source_bucket: String,
        source_object: String,
        destination_bucket: String,
        destination_object: String,
        destination_predefined_acl: Option<crate::objects::params::CopyDestinationPredefinedAcl>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        if_source_generation_match: Option<i64>,
        if_source_generation_not_match: Option<i64>,
        if_source_metageneration_match: Option<i64>,
        if_source_metageneration_not_match: Option<i64>,
        projection: Option<crate::objects::params::CopyProjection>,
        provisional_user_project: Option<String>,
        source_generation: Option<i64>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CopyRequestBuilder<'a, A> {
        #[doc = "Apply a predefined set of access controls to the destination object."]
        pub fn destination_predefined_acl(
            &mut self,
            value: crate::objects::params::CopyDestinationPredefinedAcl,
        ) -> &mut Self {
            self.destination_predefined_acl = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the destination object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the destination object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the destination object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the destination object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current generation matches the given value."]
        pub fn if_source_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_source_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current generation does not match the given value."]
        pub fn if_source_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_source_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current metageneration matches the given value."]
        pub fn if_source_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_source_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current metageneration does not match the given value."]
        pub fn if_source_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_source_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
        pub fn projection(&mut self, value: crate::objects::params::CopyProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "If present, selects a specific revision of the source object (as opposed to the latest version, the default)."]
        pub fn source_generation(&mut self, value: i64) -> &mut Self {
            self.source_generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Object, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.source_bucket);
            output.push_str("/o/");
            output.push_str(&self.source_object);
            output.push_str("/copyTo/b/");
            output.push_str(&self.destination_bucket);
            output.push_str("/o/");
            output.push_str(&self.destination_object);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("destinationPredefinedAcl", &self.destination_predefined_acl)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("ifSourceGenerationMatch", &self.if_source_generation_match)]);
            let req = req.query(&[(
                "ifSourceGenerationNotMatch",
                &self.if_source_generation_not_match,
            )]);
            let req = req.query(&[(
                "ifSourceMetagenerationMatch",
                &self.if_source_metageneration_match,
            )]);
            let req = req.query(&[(
                "ifSourceMetagenerationNotMatch",
                &self.if_source_metageneration_not_match,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("sourceGeneration", &self.source_generation)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        generation: Option<i64>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        generation: Option<i64>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        projection: Option<crate::objects::params::GetProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl."]
        pub fn projection(&mut self, value: crate::objects::params::GetProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        fn _download_path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/download/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output
        }
        pub fn download<W>(mut self, output: &mut W) -> Result<u64, Box<dyn ::std::error::Error>>
        where
            W: ::std::io::Write + ?Sized,
        {
            self.alt(crate::params::Alt::Media);
            Ok(self
                ._request(&self._path())
                .send()?
                .error_for_status()?
                .copy_to(output)?)
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
        pub fn execute_debug(self) -> Result<crate::schemas::Object, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetIamPolicyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetIamPolicyRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        pub fn execute_debug(self) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/iam");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Object,
        bucket: String,
        content_encoding: Option<String>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        kms_key_name: Option<String>,
        name: Option<String>,
        predefined_acl: Option<crate::objects::params::InsertPredefinedAcl>,
        projection: Option<crate::objects::params::InsertProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "If set, sets the contentEncoding property of the final object to this value. Setting this parameter is equivalent to setting the contentEncoding metadata property. This can be useful when uploading an object with uploadType=media to indicate the encoding of the content being uploaded."]
        pub fn content_encoding(&mut self, value: impl Into<String>) -> &mut Self {
            self.content_encoding = Some(value.into());
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
        pub fn kms_key_name(&mut self, value: impl Into<String>) -> &mut Self {
            self.kms_key_name = Some(value.into());
            self
        }
        #[doc = "Name of the object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."]
        pub fn name(&mut self, value: impl Into<String>) -> &mut Self {
            self.name = Some(value.into());
            self
        }
        #[doc = "Apply a predefined set of access controls to this object."]
        pub fn predefined_acl(
            &mut self,
            value: crate::objects::params::InsertPredefinedAcl,
        ) -> &mut Self {
            self.predefined_acl = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
        pub fn projection(&mut self, value: crate::objects::params::InsertProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        fn _simple_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("upload/storage/v1/b/");
            output.push_str(&self.bucket);
            output.push_str("/o");
            output
        }
        pub fn upload<T, R>(
            mut self,
            content: R,
            mime_type: ::mime::Mime,
        ) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            R: ::std::io::Read + ::std::io::Seek + Send + 'static,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._simple_upload_path());
            let req = req.query(&[("uploadType", "multipart")]);
            use crate::multipart::{Part, RelatedMultiPart};
            let mut multipart = RelatedMultiPart::new();
            let request_json = ::serde_json::to_vec(&self.request)?;
            multipart.new_part(Part::new(
                ::mime::APPLICATION_JSON,
                Box::new(::std::io::Cursor::new(request_json)),
            ));
            multipart.new_part(Part::new(mime_type, Box::new(content)));
            let req = req.header(
                ::reqwest::header::CONTENT_TYPE,
                format!("multipart/related; boundary={}", multipart.boundary()),
            );
            let req = req.body(reqwest::Body::new(multipart.into_reader()));
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _resumable_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("resumable/upload/storage/v1/b/");
            output.push_str(&self.bucket);
            output.push_str("/o");
            output
        }
        pub fn start_resumable_upload(
            self,
            mime_type: ::mime::Mime,
        ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._resumable_upload_path());
            let req = req.query(&[("uploadType", "resumable")]);
            let req = req.header(
                ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                mime_type.to_string(),
            );
            let req = req.json(&self.request);
            let resp = req.send()?.error_for_status()?;
            let location_header =
                resp.headers()
                    .get(::reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        format!("No LOCATION header returned when initiating resumable upload")
                    })?;
            let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
            Ok(crate::ResumableUpload::new(
                self.reqwest.clone(),
                upload_url,
            ))
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
        pub fn execute_debug(self) -> Result<crate::schemas::Object, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("contentEncoding", &self.content_encoding)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("kmsKeyName", &self.kms_key_name)]);
            let req = req.query(&[("name", &self.name)]);
            let req = req.query(&[("predefinedAcl", &self.predefined_acl)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        delimiter: Option<String>,
        include_trailing_delimiter: Option<bool>,
        max_results: Option<u32>,
        page_token: Option<String>,
        prefix: Option<String>,
        projection: Option<crate::objects::params::ListProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        versions: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted."]
        pub fn delimiter(&mut self, value: impl Into<String>) -> &mut Self {
            self.delimiter = Some(value.into());
            self
        }
        #[doc = "If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes."]
        pub fn include_trailing_delimiter(&mut self, value: bool) -> &mut Self {
            self.include_trailing_delimiter = Some(value);
            self
        }
        #[doc = "Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A previously-returned page token representing part of the larger set of results to view."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Filter results to objects whose names begin with this prefix."]
        pub fn prefix(&mut self, value: impl Into<String>) -> &mut Self {
            self.prefix = Some(value.into());
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl."]
        pub fn projection(&mut self, value: crate::objects::params::ListProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
            self
        }
        #[doc = "If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning."]
        pub fn versions(&mut self, value: bool) -> &mut Self {
            self.versions = Some(value);
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
        pub fn iter_items<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "items")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter_prefixes<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "prefixes")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
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
        ) -> Result<crate::schemas::Objects, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("delimiter", &self.delimiter)]);
            let req = req.query(&[("includeTrailingDelimiter", &self.include_trailing_delimiter)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("prefix", &self.prefix)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("versions", &self.versions)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Object,
        bucket: String,
        object: String,
        generation: Option<i64>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        predefined_acl: Option<crate::objects::params::PatchPredefinedAcl>,
        projection: Option<crate::objects::params::PatchProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Apply a predefined set of access controls to this object."]
        pub fn predefined_acl(
            &mut self,
            value: crate::objects::params::PatchPredefinedAcl,
        ) -> &mut Self {
            self.predefined_acl = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to full."]
        pub fn projection(&mut self, value: crate::objects::params::PatchProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request, for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Object, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("predefinedAcl", &self.predefined_acl)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct RewriteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Object,
        source_bucket: String,
        source_object: String,
        destination_bucket: String,
        destination_object: String,
        destination_kms_key_name: Option<String>,
        destination_predefined_acl: Option<crate::objects::params::RewriteDestinationPredefinedAcl>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        if_source_generation_match: Option<i64>,
        if_source_generation_not_match: Option<i64>,
        if_source_metageneration_match: Option<i64>,
        if_source_metageneration_not_match: Option<i64>,
        max_bytes_rewritten_per_call: Option<i64>,
        projection: Option<crate::objects::params::RewriteProjection>,
        provisional_user_project: Option<String>,
        rewrite_token: Option<String>,
        source_generation: Option<i64>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RewriteRequestBuilder<'a, A> {
        #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
        pub fn destination_kms_key_name(&mut self, value: impl Into<String>) -> &mut Self {
            self.destination_kms_key_name = Some(value.into());
            self
        }
        #[doc = "Apply a predefined set of access controls to the destination object."]
        pub fn destination_predefined_acl(
            &mut self,
            value: crate::objects::params::RewriteDestinationPredefinedAcl,
        ) -> &mut Self {
            self.destination_predefined_acl = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the destination object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the destination object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current generation matches the given value."]
        pub fn if_source_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_source_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current generation does not match the given value."]
        pub fn if_source_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_source_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current metageneration matches the given value."]
        pub fn if_source_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_source_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the source object's current metageneration does not match the given value."]
        pub fn if_source_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_source_metageneration_not_match = Some(value);
            self
        }
        #[doc = "The maximum number of bytes that will be rewritten per rewrite request. Most callers shouldn't need to specify this parameter - it is primarily in place to support testing. If specified the value must be an integral multiple of 1 MiB (1048576). Also, this only applies to requests where the source and destination span locations and/or storage classes. Finally, this value must not change across rewrite calls else you'll get an error that the rewriteToken is invalid."]
        pub fn max_bytes_rewritten_per_call(&mut self, value: i64) -> &mut Self {
            self.max_bytes_rewritten_per_call = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
        pub fn projection(
            &mut self,
            value: crate::objects::params::RewriteProjection,
        ) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "Include this field (from the previous rewrite response) on each rewrite request after the first one, until the rewrite response 'done' flag is true. Calls that provide a rewriteToken can omit all other request fields, but if included those fields must match the values provided in the first rewrite request."]
        pub fn rewrite_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.rewrite_token = Some(value.into());
            self
        }
        #[doc = "If present, selects a specific revision of the source object (as opposed to the latest version, the default)."]
        pub fn source_generation(&mut self, value: i64) -> &mut Self {
            self.source_generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::RewriteResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.source_bucket);
            output.push_str("/o/");
            output.push_str(&self.source_object);
            output.push_str("/rewriteTo/b/");
            output.push_str(&self.destination_bucket);
            output.push_str("/o/");
            output.push_str(&self.destination_object);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("destinationKmsKeyName", &self.destination_kms_key_name)]);
            let req = req.query(&[("destinationPredefinedAcl", &self.destination_predefined_acl)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("ifSourceGenerationMatch", &self.if_source_generation_match)]);
            let req = req.query(&[(
                "ifSourceGenerationNotMatch",
                &self.if_source_generation_not_match,
            )]);
            let req = req.query(&[(
                "ifSourceMetagenerationMatch",
                &self.if_source_metageneration_match,
            )]);
            let req = req.query(&[(
                "ifSourceMetagenerationNotMatch",
                &self.if_source_metageneration_not_match,
            )]);
            let req = req.query(&[(
                "maxBytesRewrittenPerCall",
                &self.max_bytes_rewritten_per_call,
            )]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("rewriteToken", &self.rewrite_token)]);
            let req = req.query(&[("sourceGeneration", &self.source_generation)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SetIamPolicyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Policy,
        bucket: String,
        object: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SetIamPolicyRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/iam");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct TestIamPermissionsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        bucket: String,
        object: String,
        permissions: String,
        generation: Option<i64>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> TestIamPermissionsRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
        ) -> Result<crate::schemas::TestIamPermissionsResponse, Box<dyn ::std::error::Error>>
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
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output.push_str("/iam/testPermissions");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("permissions", &self.permissions)]);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Object,
        bucket: String,
        object: String,
        generation: Option<i64>,
        if_generation_match: Option<i64>,
        if_generation_not_match: Option<i64>,
        if_metageneration_match: Option<i64>,
        if_metageneration_not_match: Option<i64>,
        predefined_acl: Option<crate::objects::params::UpdatePredefinedAcl>,
        projection: Option<crate::objects::params::UpdateProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
        pub fn generation(&mut self, value: i64) -> &mut Self {
            self.generation = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
        pub fn if_generation_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
        pub fn if_generation_not_match(&mut self, value: i64) -> &mut Self {
            self.if_generation_not_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
        pub fn if_metageneration_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_match = Some(value);
            self
        }
        #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
        pub fn if_metageneration_not_match(&mut self, value: i64) -> &mut Self {
            self.if_metageneration_not_match = Some(value);
            self
        }
        #[doc = "Apply a predefined set of access controls to this object."]
        pub fn predefined_acl(
            &mut self,
            value: crate::objects::params::UpdatePredefinedAcl,
        ) -> &mut Self {
            self.predefined_acl = Some(value);
            self
        }
        #[doc = "Set of properties to return. Defaults to full."]
        pub fn projection(&mut self, value: crate::objects::params::UpdateProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Object, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/");
            output.push_str(&self.object);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("generation", &self.generation)]);
            let req = req.query(&[("ifGenerationMatch", &self.if_generation_match)]);
            let req = req.query(&[("ifGenerationNotMatch", &self.if_generation_not_match)]);
            let req = req.query(&[("ifMetagenerationMatch", &self.if_metageneration_match)]);
            let req = req.query(&[(
                "ifMetagenerationNotMatch",
                &self.if_metageneration_not_match,
            )]);
            let req = req.query(&[("predefinedAcl", &self.predefined_acl)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct WatchAllRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Channel,
        bucket: String,
        delimiter: Option<String>,
        include_trailing_delimiter: Option<bool>,
        max_results: Option<u32>,
        page_token: Option<String>,
        prefix: Option<String>,
        projection: Option<crate::objects::params::WatchAllProjection>,
        provisional_user_project: Option<String>,
        user_project: Option<String>,
        versions: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> WatchAllRequestBuilder<'a, A> {
        #[doc = "Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted."]
        pub fn delimiter(&mut self, value: impl Into<String>) -> &mut Self {
            self.delimiter = Some(value.into());
            self
        }
        #[doc = "If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes."]
        pub fn include_trailing_delimiter(&mut self, value: bool) -> &mut Self {
            self.include_trailing_delimiter = Some(value);
            self
        }
        #[doc = "Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A previously-returned page token representing part of the larger set of results to view."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Filter results to objects whose names begin with this prefix."]
        pub fn prefix(&mut self, value: impl Into<String>) -> &mut Self {
            self.prefix = Some(value.into());
            self
        }
        #[doc = "Set of properties to return. Defaults to noAcl."]
        pub fn projection(
            &mut self,
            value: crate::objects::params::WatchAllProjection,
        ) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
        pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.provisional_user_project = Some(value.into());
            self
        }
        #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
        pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_project = Some(value.into());
            self
        }
        #[doc = "If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning."]
        pub fn versions(&mut self, value: bool) -> &mut Self {
            self.versions = Some(value);
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Channel, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
            output.push_str("b/");
            output.push_str(&self.bucket);
            output.push_str("/o/watch");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("delimiter", &self.delimiter)]);
            let req = req.query(&[("includeTrailingDelimiter", &self.include_trailing_delimiter)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("prefix", &self.prefix)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
            let req = req.query(&[("userProject", &self.user_project)]);
            let req = req.query(&[("versions", &self.versions)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
pub mod projects {
    pub mod params {}
    pub struct ProjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
        #[doc = "Actions that can be performed on the hmac_keys resource"]
        pub fn hmac_keys(&self) -> hmac_keys::HmacKeysActions<A> {
            hmac_keys::HmacKeysActions
        }
        #[doc = "Actions that can be performed on the service_account resource"]
        pub fn service_account(&self) -> service_account::ServiceAccountActions<A> {
            service_account::ServiceAccountActions
        }
    }
    pub mod hmac_keys {
        pub mod params {}
        pub struct HmacKeysActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> HmacKeysActions<'a, A> {
            #[doc = "Creates a new HMAC key for the specified service account."]
            pub fn create(
                &self,
                project_id: impl Into<String>,
                service_account_email: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    service_account_email: service_account_email.into(),
                    user_project: None,
                }
            }
            #[doc = "Deletes an HMAC key."]
            pub fn delete(
                &self,
                project_id: impl Into<String>,
                access_id: impl Into<String>,
            ) -> DeleteRequestBuilder<A> {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    access_id: access_id.into(),
                    user_project: None,
                }
            }
            #[doc = "Retrieves an HMAC key's metadata"]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                access_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    access_id: access_id.into(),
                    user_project: None,
                }
            }
            #[doc = "Retrieves a list of HMAC keys matching the criteria."]
            pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    project_id: project_id.into(),
                    max_results: None,
                    page_token: None,
                    service_account_email: None,
                    show_deleted_keys: None,
                    user_project: None,
                }
            }
            #[doc = "Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states."]
            pub fn update(
                &self,
                request: crate::schemas::HmacKeyMetadata,
                project_id: impl Into<String>,
                access_id: impl Into<String>,
            ) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    access_id: access_id.into(),
                    user_project: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            service_account_email: String,
            user_project: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "The project to be billed for this request."]
            pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.user_project = Some(value.into());
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
            ) -> Result<crate::schemas::HmacKey, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
                output.push_str("projects/");
                output.push_str(&self.project_id);
                output.push_str("/hmacKeys");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("serviceAccountEmail", &self.service_account_email)]);
                let req = req.query(&[("userProject", &self.user_project)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            access_id: String,
            user_project: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = "The project to be billed for this request."]
            pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.user_project = Some(value.into());
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
                output.push_str("projects/");
                output.push_str(&self.project_id);
                output.push_str("/hmacKeys/");
                output.push_str(&self.access_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("userProject", &self.user_project)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            access_id: String,
            user_project: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "The project to be billed for this request."]
            pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.user_project = Some(value.into());
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
            ) -> Result<crate::schemas::HmacKeyMetadata, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
                output.push_str("projects/");
                output.push_str(&self.project_id);
                output.push_str("/hmacKeys/");
                output.push_str(&self.access_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("userProject", &self.user_project)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            max_results: Option<u32>,
            page_token: Option<String>,
            service_account_email: Option<String>,
            show_deleted_keys: Option<bool>,
            user_project: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Maximum number of items to return in a single page of responses. The service uses this parameter or 250 items, whichever is smaller. The max number of items per page will also be limited by the number of distinct service accounts in the response. If the number of service accounts in a single response is too high, the page will truncated and a next page token will be returned."]
            pub fn max_results(&mut self, value: u32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A previously-returned page token representing part of the larger set of results to view."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "If present, only keys for the given service account are returned."]
            pub fn service_account_email(&mut self, value: impl Into<String>) -> &mut Self {
                self.service_account_email = Some(value.into());
                self
            }
            #[doc = "Whether or not to show keys in the DELETED state."]
            pub fn show_deleted_keys(&mut self, value: bool) -> &mut Self {
                self.show_deleted_keys = Some(value);
                self
            }
            #[doc = "The project to be billed for this request."]
            pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.user_project = Some(value.into());
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
            pub fn iter_items<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "items")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
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
            ) -> Result<crate::schemas::HmacKeysMetadata, Box<dyn ::std::error::Error>>
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
                output.push_str("projects/");
                output.push_str(&self.project_id);
                output.push_str("/hmacKeys");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("serviceAccountEmail", &self.service_account_email)]);
                let req = req.query(&[("showDeletedKeys", &self.show_deleted_keys)]);
                let req = req.query(&[("userProject", &self.user_project)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::HmacKeyMetadata,
            project_id: String,
            access_id: String,
            user_project: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = "The project to be billed for this request."]
            pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.user_project = Some(value.into());
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::HmacKeyMetadata, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
                output.push_str("projects/");
                output.push_str(&self.project_id);
                output.push_str("/hmacKeys/");
                output.push_str(&self.access_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("userProject", &self.user_project)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
    pub mod service_account {
        pub mod params {}
        pub struct ServiceAccountActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ServiceAccountActions<'a, A> {
            #[doc = "Get the email address of this project's Google Cloud Storage service account."]
            pub fn get(&self, project_id: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                    provisional_user_project: None,
                    user_project: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            provisional_user_project: Option<String>,
            user_project: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
            pub fn provisional_user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.provisional_user_project = Some(value.into());
                self
            }
            #[doc = "The project to be billed for this request."]
            pub fn user_project(&mut self, value: impl Into<String>) -> &mut Self {
                self.user_project = Some(value.into());
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
            ) -> Result<crate::schemas::ServiceAccount, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/storage/v1/".to_owned();
                output.push_str("projects/");
                output.push_str(&self.project_id);
                output.push_str("/serviceAccount");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("provisionalUserProject", &self.provisional_user_project)]);
                let req = req.query(&[("userProject", &self.user_project)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
