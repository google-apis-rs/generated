use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("healthcare1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210301")
            .about("Manage, store, and access healthcare data in Google Cloud Platform.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut datasets2 = SubCommand::with_name("datasets")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, deidentify, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new health dataset. Results are returned through the Operation interface which returns either an `Operation.response` which contains a Dataset or `Operation.error`. The metadata field type is OperationMetadata.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deidentify").about("Creates a new dataset containing de-identified data from the source dataset. The metadata field type is OperationMetadata. If the request is successful, the response field type is DeidentifySummary. If errors occur, error is set. The LRO result may still be successful if de-identification fails for some DICOM instances. The new de-identified dataset will not contain these failed resources. Failed resource totals are tracked in Operation.metadata. Error details are also logged to Cloud Logging. For more information, see [Viewing logs](/healthcare/docs/how-tos/logging).");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified health dataset and all data contained in the dataset. Deleting a dataset does not affect the sources from which the dataset was imported (if any).");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets any metadata associated with a dataset.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the health datasets in the current project.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates dataset metadata.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        let mut consent_stores3 = SubCommand::with_name("consent_stores")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: check_data_access, create, delete, evaluate_user_consents, get, get_iam_policy, list, patch, query_accessible_data, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("check_data_access").about("Checks if a particular data_id of a User data mapping in the specified consent store is consented for the specified use.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new consent store in the parent dataset. Attempting to create a consent store with the same ID as an existing store fails with an ALREADY_EXISTS error.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes the specified consent store and removes all the consent store\'s data.",
            );
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("evaluate_user_consents").about("Evaluates the user\'s Consents for all matching User data mappings. Note: User data mappings are indexed asynchronously, which can cause a slight delay between the time mappings are created or updated and when they are included in EvaluateUserConsents results.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified consent store.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the consent stores in the specified dataset.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified consent store.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query_accessible_data").about("Queries all data_ids that are consented for a specified use in the given consent store and writes them to a specified destination. The returned Operation includes a progress counter for the number of User data mappings processed. Errors are logged to Cloud Logging (see [Viewing logs] (cloud.google.com/healthcare/docs/how-tos/logging)). For example, the following sample log entry shows a `failed to evaluate consent policy` error that occurred during a QueryAccessibleData call to consent store `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. ```json jsonPayload: { @type: \"type.googleapis.com/google.cloud.healthcare.logging.QueryAccessibleDataLogEntry\" error: { code: 9 message: \"failed to evaluate consent policy\" } resourceName: \"projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}\" } logName: \"projects/{project_id}/logs/healthcare.googleapis.com%2Fquery_accessible_data\" operation: { id: \"projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/operations/{operation_id}\" producer: \"healthcare.googleapis.com/QueryAccessibleData\" } receiveTimestamp: \"TIMESTAMP\" resource: { labels: { consent_store_id: \"{consent_store_id}\" dataset_id: \"{dataset_id}\" location: \"{location_id}\" project_id: \"{project_id}\" } type: \"healthcare_consent_store\" } severity: \"ERROR\" timestamp: \"TIMESTAMP\" ```");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            consent_stores3 = consent_stores3.subcommand(mcmd);
        }
        let mut dicom_stores3 = SubCommand::with_name("dicom_stores")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, deidentify, delete, export, get, get_iam_policy, import, list, patch, search_for_instances, search_for_series, search_for_studies, set_iam_policy, store_instances and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new DICOM store within the parent dataset.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies data from the source store and writes it to the destination store. The metadata field type is OperationMetadata. If the request is successful, the response field type is DeidentifyDicomStoreSummary. If errors occur, error is set. The LRO result may still be successful if de-identification fails for some DICOM instances. The output DICOM store will not contain these failed resources. Failed resource totals are tracked in Operation.metadata. Error details are also logged to Cloud Logging (see [Viewing logs](/healthcare/docs/how-tos/logging)).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified DICOM store and removes all images that are contained within it.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports data to the specified destination by copying it from the DICOM store. Errors are also logged to Cloud Logging. For more information, see [Viewing logs](/healthcare/docs/how-tos/logging). The metadata field type is OperationMetadata.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified DICOM store.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports data into the DICOM store by copying it from the specified source. Errors are logged to Cloud Logging. For more information, see [Viewing logs](/healthcare/docs/how-tos/logging). The metadata field type is OperationMetadata.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the DICOM stores in the given dataset.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified DICOM store.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_instances").about("SearchForInstances returns a list of matching instances. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForInstances, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForInstances, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_series").about("SearchForSeries returns a list of matching series. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForSeries, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForSeries, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_studies").about("SearchForStudies returns a list of matching studies. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForStudies, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForStudies, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("store_instances").about("StoreInstances stores DICOM instances associated with study instance unique identifiers (SUID). See [Store Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5). For details on the implementation of StoreInstances, see [Store transaction](https://cloud.google.com/healthcare/docs/dicom#store_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call StoreInstances, see [Storing DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#storing_dicom_data).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        let mut fhir_stores3 = SubCommand::with_name("fhir_stores")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, deidentify, delete, export, get, get_iam_policy, import, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new FHIR store within the parent dataset.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies data from the source store and writes it to the destination store. The metadata field type is OperationMetadata. If the request is successful, the response field type is DeidentifyFhirStoreSummary. If errors occur, error is set. Error details are also logged to Cloud Logging (see [Viewing logs](/healthcare/docs/how-tos/logging)).");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified FHIR store and removes all resources within it.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Export resources from the FHIR store to the specified destination. This method returns an Operation that can be used to track the status of the export by calling GetOperation. Immediate fatal errors appear in the error field, errors are also logged to Cloud Logging (see [Viewing logs](/healthcare/docs/how-tos/logging)). Otherwise, when the operation finishes, a detailed response of type ExportResourcesResponse is returned in the response field. The metadata field type for this operation is OperationMetadata.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the configuration of the specified FHIR store.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports resources to the FHIR store by loading data from the specified sources. This method is optimized to load large quantities of data using import semantics that ignore some FHIR store configuration options and are not suitable for all use cases. It is primarily intended to load data into an empty FHIR store that is not being used by other clients. In cases where this method is not appropriate, consider using ExecuteBundle to load data. Every resource in the input must contain a client-supplied ID. Each resource is stored using the supplied ID regardless of the enable_update_create setting on the FHIR store. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. The import process does not enforce referential integrity, regardless of the disable_referential_integrity setting on the FHIR store. This allows the import of resources with arbitrary interdependencies without considering grouping or ordering, but if the input data contains invalid references or if some resources fail to be imported, the FHIR store might be left in a state that violates referential integrity. The import process does not trigger Pub/Sub notification or BigQuery streaming update, regardless of how those are configured on the FHIR store. If a resource with the specified ID already exists, the most recent version of the resource is overwritten without creating a new historical version, regardless of the disable_resource_versioning setting on the FHIR store. If transient failures occur during the import, it\'s possible that successfully imported resources will be overwritten more than once. The import operation is idempotent unless the input data contains multiple valid resources with the same ID but different contents. In that case, after the import completes, the store contains exactly one resource with that ID but there is no ordering guarantee on which version of the contents it will have. The operation result counters do not count duplicate IDs as an error and count one success for each resource in the input, which might result in a success count larger than the number of resources in the FHIR store. This often occurs when importing data organized in bundles produced by Patient-everything where each bundle contains its own copy of a resource such as Practitioner that might be referred to by many patients. If some resources fail to import, for example due to parsing errors, successfully imported resources are not rolled back. The location and format of the input data is specified by the parameters in ImportResourcesRequest. Note that if no format is specified, this method assumes the `BUNDLE` format. When using the `BUNDLE` format this method ignores the `Bundle.type` field, except that `history` bundles are rejected, and does not apply any of the bundle processing semantics for batch or transaction bundles. Unlike in ExecuteBundle, transaction bundles are not executed as a single transaction and bundle-internal references are not rewritten. The bundle is treated as a collection of resources to be written as provided in `Bundle.entry.resource`, ignoring `Bundle.entry.request`. As an example, this allows the import of `searchset` bundles produced by a FHIR search or Patient-everything operation. This method returns an Operation that can be used to track the status of the import by calling GetOperation. Immediate fatal errors appear in the error field, errors are also logged to Cloud Logging (see [Viewing logs](/healthcare/docs/how-tos/logging)). Otherwise, when the operation finishes, a detailed response of type ImportResourcesResponse is returned in the response field. The metadata field type for this operation is OperationMetadata.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the FHIR stores in the given dataset.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the configuration of the specified FHIR store.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        let mut hl_7v2_stores3 = SubCommand::with_name("hl_7v2_stores")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new HL7v2 store within the parent dataset.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes the specified HL7v2 store and removes all messages that it contains.",
            );
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified HL7v2 store.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the HL7v2 stores in the given dataset.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the HL7v2 store.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut attribute_definitions4 = SubCommand::with_name("attribute_definitions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Attribute definition in the parent consent store.");
            attribute_definitions4 = attribute_definitions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Attribute definition. Fails if the Attribute definition is referenced by any User data mapping, or the latest revision of any Consent.");
            attribute_definitions4 = attribute_definitions4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the specified Attribute definition.");
            attribute_definitions4 = attribute_definitions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the Attribute definitions in the specified consent store.");
            attribute_definitions4 = attribute_definitions4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified Attribute definition.");
            attribute_definitions4 = attribute_definitions4.subcommand(mcmd);
        }
        let mut consent_artifacts4 = SubCommand::with_name("consent_artifacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Consent artifact in the parent consent store.");
            consent_artifacts4 = consent_artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Consent artifact. Fails if the artifact is referenced by the latest revision of any Consent.");
            consent_artifacts4 = consent_artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified Consent artifact.");
            consent_artifacts4 = consent_artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the Consent artifacts in the specified consent store.");
            consent_artifacts4 = consent_artifacts4.subcommand(mcmd);
        }
        let mut consents4 = SubCommand::with_name("consents")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: activate, create, delete, delete_revision, get, list, list_revisions, patch, reject and revoke");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates the latest revision of the specified Consent by committing a new revision with `state` updated to `ACTIVE`. If the latest revision of the specified Consent is in the `ACTIVE` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the specified Consent is in the `REJECTED` or `REVOKED` state.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Consent in the parent consent store.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the Consent and its revisions. To keep a record of the Consent but mark it inactive, see [RevokeConsent]. To delete a revision of a Consent, see [DeleteConsentRevision]. This operation does not delete the related Consent artifact.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_revision").about("Deletes the specified revision of a Consent. An INVALID_ARGUMENT error occurs if the specified revision is the latest revision.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified revision of a Consent, or the latest revision if `revision_id` is not specified in the resource name.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Consent in the given consent store, returning each Consent\'s latest revision.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_revisions").about(
                "Lists the revisions of the specified Consent in reverse chronological order.",
            );
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the latest revision of the specified Consent by committing a new revision with the changes. A FAILED_PRECONDITION error occurs if the latest revision of the specified Consent is in the `REJECTED` or `REVOKED` state.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reject").about("Rejects the latest revision of the specified Consent by committing a new revision with `state` updated to `REJECTED`. If the latest revision of the specified Consent is in the `REJECTED` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the specified Consent is in the `ACTIVE` or `REVOKED` state.");
            consents4 = consents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revoke").about("Revokes the latest revision of the specified Consent by committing a new revision with `state` updated to `REVOKED`. If the latest revision of the specified Consent is in the `REVOKED` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the given consent is in `DRAFT` or `REJECTED` state.");
            consents4 = consents4.subcommand(mcmd);
        }
        let mut user_data_mappings4 = SubCommand::with_name("user_data_mappings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: archive, create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("archive").about("Archives the specified User data mapping.");
            user_data_mappings4 = user_data_mappings4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new User data mapping in the parent consent store.");
            user_data_mappings4 = user_data_mappings4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified User data mapping.");
            user_data_mappings4 = user_data_mappings4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified User data mapping.");
            user_data_mappings4 = user_data_mappings4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the User data mappings in the specified consent store.");
            user_data_mappings4 = user_data_mappings4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified User data mapping.");
            user_data_mappings4 = user_data_mappings4.subcommand(mcmd);
        }
        let mut studies4 = SubCommand::with_name("studies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, retrieve_metadata, retrieve_study, search_for_instances, search_for_series and store_instances");
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteStudy deletes all instances within the given study. Delete requests are equivalent to the GET requests specified in the Retrieve transaction. The method returns an Operation which will be marked successful when the deletion is complete. Warning: Inserting instances into a study while a delete operation is running for that study could result in the new instances not appearing in search results until the deletion operation finishes. For samples that show how to call DeleteStudy, see [Deleting a study, series, or instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#deleting_a_study_series_or_instance).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_metadata").about("RetrieveStudyMetadata returns instance associated with the given study presented as metadata with the bulk data removed. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveStudyMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveStudyMetadata, see [Retrieving metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_metadata).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_study").about("RetrieveStudy returns all instances within the given study. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveStudy, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveStudy, see [Retrieving DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_dicom_data).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_instances").about("SearchForInstances returns a list of matching instances. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForInstances, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForInstances, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_series").about("SearchForSeries returns a list of matching series. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForSeries, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForSeries, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("store_instances").about("StoreInstances stores DICOM instances associated with study instance unique identifiers (SUID). See [Store Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5). For details on the implementation of StoreInstances, see [Store transaction](https://cloud.google.com/healthcare/docs/dicom#store_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call StoreInstances, see [Storing DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#storing_dicom_data).");
            studies4 = studies4.subcommand(mcmd);
        }
        let mut fhir4 = SubCommand::with_name("fhir")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: patient_everything, resource_purge, capabilities, create, delete, execute_bundle, history, patch, read, search, search_type, update and vread");
        {
            let mcmd = SubCommand::with_name("patient_everything").about("Retrieves a Patient resource and resources related to that patient. Implements the FHIR extended operation Patient-everything ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/patient-operations.html#everything), [STU3](http://hl7.org/implement/standards/fhir/STU3/patient-operations.html#everything), [R4](http://hl7.org/implement/standards/fhir/R4/patient-operations.html#everything)). On success, the response body will contain a JSON-encoded representation of a `Bundle` resource of type `searchset`, containing the results of the operation. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. The resources in scope for the response are: * The patient resource itself. * All the resources directly referenced by the patient resource. * Resources directly referencing the patient resource that meet the inclusion criteria. The inclusion criteria are based on the membership rules in the patient compartment definition ([DSTU2](http://hl7.org/fhir/DSTU2/compartment-patient.html), [STU3](http://www.hl7.org/fhir/stu3/compartmentdefinition-patient.html), [R4](http://hl7.org/fhir/R4/compartmentdefinition-patient.html)), which details the eligible resource types and referencing search parameters. For samples that show how to call `Patient-everything`, see [Getting all patient compartment resources](/healthcare/docs/how-tos/fhir-resources#getting_all_patient_compartment_resources).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resource_purge").about("Deletes all the historical versions of a resource (excluding the current version) from the FHIR store. To remove all versions of a resource, first delete the current version and then call this method. This is not a FHIR standard operation. For samples that show how to call `Resource-purge`, see [Deleting historical versions of a FHIR resource](/healthcare/docs/how-tos/fhir-resources#deleting_historical_versions_of_a_fhir_resource).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("capabilities").about("Gets the FHIR capability statement ([STU3](http://hl7.org/implement/standards/fhir/STU3/capabilitystatement.html), [R4](http://hl7.org/implement/standards/fhir/R4/capabilitystatement.html)), or the [conformance statement](http://hl7.org/implement/standards/fhir/DSTU2/conformance.html) in the DSTU2 case for the store, which contains a description of functionality supported by the server. Implements the FHIR standard capabilities interaction ([STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#capabilities), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#capabilities)), or the [conformance interaction](http://hl7.org/implement/standards/fhir/DSTU2/http.html#conformance) in the DSTU2 case. On success, the response body will contain a JSON-encoded representation of a `CapabilityStatement` resource.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a FHIR resource. Implements the FHIR standard create interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#create), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#create), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#create)), which creates a new resource with a server-assigned resource ID. The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. On success, the response body will contain a JSON-encoded representation of the resource as it was created on the server, including the server-assigned resource ID and version ID. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `create`, see [Creating a FHIR resource](/healthcare/docs/how-tos/fhir-resources#creating_a_fhir_resource).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a FHIR resource. Implements the FHIR standard delete interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#delete), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#delete), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#delete)). Note: Unless resource versioning is disabled by setting the disable_resource_versioning flag on the FHIR store, the deleted resources will be moved to a history repository that can still be retrieved through vread and related methods, unless they are removed by the purge method. For samples that show how to call `delete`, see [Deleting a FHIR resource](/healthcare/docs/how-tos/fhir-resources#deleting_a_fhir_resource).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute_bundle").about("Executes all the requests in the given Bundle. Implements the FHIR standard batch/transaction interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#transaction), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#transaction), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#transaction)). Supports all interactions within a bundle, except search. This method accepts Bundles of type `batch` and `transaction`, processing them according to the batch processing rules ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#2.1.0.16.1), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.1), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#brules)) and transaction processing rules ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#2.1.0.16.2), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.2), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#trules)). The request body must contain a JSON-encoded FHIR `Bundle` resource, and the request headers must contain `Content-Type: application/fhir+json`. For a batch bundle or a successful transaction the response body will contain a JSON-encoded representation of a `Bundle` resource of type `batch-response` or `transaction-response` containing one entry for each entry in the request, with the outcome of processing the entry. In the case of an error for a transaction bundle, the response body will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `executeBundle`, see [Managing FHIR resources using FHIR bundles](/healthcare/docs/how-tos/fhir-bundles).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("history").about("Lists all the versions of a resource (including the current version and deleted versions) from the FHIR store. Implements the per-resource form of the FHIR standard history interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#history), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#history), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#history)). On success, the response body will contain a JSON-encoded representation of a `Bundle` resource of type `history`, containing the version history sorted from most recent to oldest versions. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `history`, see [Listing FHIR resource versions](/healthcare/docs/how-tos/fhir-resources#listing_fhir_resource_versions).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates part of an existing resource by applying the operations specified in a [JSON Patch](http://jsonpatch.com/) document. Implements the FHIR standard patch interaction ([STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#patch), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#patch)). DSTU2 doesn\'t define a patch method, but the server supports it in the same way it supports STU3. The request body must contain a JSON Patch document, and the request headers must contain `Content-Type: application/json-patch+json`. On success, the response body will contain a JSON-encoded representation of the updated resource, including the server-assigned version ID. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `patch`, see [Patching a FHIR resource](/healthcare/docs/how-tos/fhir-resources#patching_a_fhir_resource).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("read").about("Gets the contents of a FHIR resource. Implements the FHIR standard read interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#read), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#read), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#read)). Also supports the FHIR standard conditional read interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#cread), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#cread), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#cread)) specified by supplying an `If-Modified-Since` header with a date/time value or an `If-None-Match` header with an ETag value. On success, the response body will contain a JSON-encoded representation of the resource. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `read`, see [Getting a FHIR resource](/healthcare/docs/how-tos/fhir-resources#getting_a_fhir_resource).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches for resources in the given FHIR store according to criteria specified as query parameters. Implements the FHIR standard search interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#search), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#search), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#search)) using the search semantics described in the FHIR Search specification ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/search.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/search.html), [R4](http://hl7.org/implement/standards/fhir/R4/search.html)). Supports four methods of search defined by the specification: * `GET [base]?[parameters]` to search across all resources. * `GET [base]/[type]?[parameters]` to search resources of a specified type. * `POST [base]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method across all resources. * `POST [base]/[type]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method for the specified type. The `GET` and `POST` methods do not support compartment searches. The `POST` method does not support `application/x-www-form-urlencoded` search parameters. On success, the response body will contain a JSON-encoded representation of a `Bundle` resource of type `searchset`, containing the results of the search. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. The server\'s capability statement, retrieved through capabilities, indicates what search parameters are supported on each FHIR resource. A list of all search parameters defined by the specification can be found in the FHIR Search Parameter Registry ([STU3](http://hl7.org/implement/standards/fhir/STU3/searchparameter-registry.html), [R4](http://hl7.org/implement/standards/fhir/R4/searchparameter-registry.html)). FHIR search parameters for DSTU2 can be found on each resource\'s definition page. Supported search modifiers: `:missing`, `:exact`, `:contains`, `:text`, `:in`, `:not-in`, `:above`, `:below`, `:[type]`, `:not`, and `:recurse`. Supported search result parameters: `_sort`, `_count`, `_include`, `_revinclude`, `_summary=text`, `_summary=data`, and `_elements`. The maximum number of search results returned defaults to 100, which can be overridden by the `_count` parameter up to a maximum limit of 1000. If there are additional results, the returned `Bundle` will contain pagination links. Resources with a total size larger than 5MB or a field count larger than 50,000 might not be fully searchable as the server might trim its generated search index in those cases. Note: FHIR resources are indexed asynchronously, so there might be a slight delay between the time a resource is created or changes and when the change is reflected in search results. For samples and detailed information, see [Searching for FHIR resources](/healthcare/docs/how-tos/fhir-search) and [Advanced FHIR search features](/healthcare/docs/how-tos/fhir-advanced-search).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_type").about("Searches for resources in the given FHIR store according to criteria specified as query parameters. Implements the FHIR standard search interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#search), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#search), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#search)) using the search semantics described in the FHIR Search specification ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/search.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/search.html), [R4](http://hl7.org/implement/standards/fhir/R4/search.html)). Supports four methods of search defined by the specification: * `GET [base]?[parameters]` to search across all resources. * `GET [base]/[type]?[parameters]` to search resources of a specified type. * `POST [base]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method across all resources. * `POST [base]/[type]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method for the specified type. The `GET` and `POST` methods do not support compartment searches. The `POST` method does not support `application/x-www-form-urlencoded` search parameters. On success, the response body will contain a JSON-encoded representation of a `Bundle` resource of type `searchset`, containing the results of the search. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. The server\'s capability statement, retrieved through capabilities, indicates what search parameters are supported on each FHIR resource. A list of all search parameters defined by the specification can be found in the FHIR Search Parameter Registry ([STU3](http://hl7.org/implement/standards/fhir/STU3/searchparameter-registry.html), [R4](http://hl7.org/implement/standards/fhir/R4/searchparameter-registry.html)). FHIR search parameters for DSTU2 can be found on each resource\'s definition page. Supported search modifiers: `:missing`, `:exact`, `:contains`, `:text`, `:in`, `:not-in`, `:above`, `:below`, `:[type]`, `:not`, and `:recurse`. Supported search result parameters: `_sort`, `_count`, `_include`, `_revinclude`, `_summary=text`, `_summary=data`, and `_elements`. The maximum number of search results returned defaults to 100, which can be overridden by the `_count` parameter up to a maximum limit of 1000. If there are additional results, the returned `Bundle` will contain pagination links. Resources with a total size larger than 5MB or a field count larger than 50,000 might not be fully searchable as the server might trim its generated search index in those cases. Note: FHIR resources are indexed asynchronously, so there might be a slight delay between the time a resource is created or changes and when the change is reflected in search results. For samples and detailed information, see [Searching for FHIR resources](/healthcare/docs/how-tos/fhir-search) and [Advanced FHIR search features](/healthcare/docs/how-tos/fhir-advanced-search).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the entire contents of a resource. Implements the FHIR standard update interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#update), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#update), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#update)). If the specified resource does not exist and the FHIR store has enable_update_create set, creates the resource with the client-specified ID. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. The resource must contain an `id` element having an identical value to the ID in the REST path of the request. On success, the response body will contain a JSON-encoded representation of the updated resource, including the server-assigned version ID. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `update`, see [Updating a FHIR resource](/healthcare/docs/how-tos/fhir-resources#updating_a_fhir_resource).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("vread").about("Gets the contents of a version (current or historical) of a FHIR resource by version ID. Implements the FHIR standard vread interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#vread), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#vread), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#vread)). On success, the response body will contain a JSON-encoded representation of the resource. Errors generated by the FHIR store will contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `vread`, see [Retrieving a FHIR resource version](/healthcare/docs/how-tos/fhir-resources#retrieving_a_fhir_resource_version).");
            fhir4 = fhir4.subcommand(mcmd);
        }
        let mut messages4 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, ingest, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Parses and stores an HL7v2 message. This method triggers an asynchronous notification to any Cloud Pub/Sub topic configured in projects.locations.datasets.hl7V2Stores.Hl7V2NotificationConfig, if the filtering matches the message. If an MLLP adapter is configured to listen to a Cloud Pub/Sub topic, the adapter transmits the message when a notification is received.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an HL7v2 message.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an HL7v2 message.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("ingest").about("Parses and stores an HL7v2 message. This method triggers an asynchronous notification to any Cloud Pub/Sub topic configured in projects.locations.datasets.hl7V2Stores.Hl7V2NotificationConfig, if the filtering matches the message. If an MLLP adapter is configured to listen to a Cloud Pub/Sub topic, the adapter transmits the message when a notification is received. This method also generates a response containing an HL7v2 acknowledgement (`ACK`) message when successful or a negative acknowledgement (`NACK`) message in case of error, suitable for replying to HL7v2 interface systems that expect these acknowledgements.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the messages in the given HL7v2 store with support for filtering. Note: HL7v2 messages are indexed asynchronously, so there might be a slight delay between the time a message is created and when it can be found through a filter.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update the message. The contents of the message in Message.data and data extracted from the contents such as Message.create_time cannot be altered. Only the Message.labels field is allowed to be updated. The labels in the request are merged with the existing set of labels. Existing labels with the same keys are updated.");
            messages4 = messages4.subcommand(mcmd);
        }
        let mut series5 = SubCommand::with_name("series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, retrieve_metadata, retrieve_series and search_for_instances");
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteSeries deletes all instances within the given study and series. Delete requests are equivalent to the GET requests specified in the Retrieve transaction. The method returns an Operation which will be marked successful when the deletion is complete. Warning: Inserting instances into a series while a delete operation is running for that series could result in the new instances not appearing in search results until the deletion operation finishes. For samples that show how to call DeleteSeries, see [Deleting a study, series, or instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#deleting_a_study_series_or_instance).");
            series5 = series5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_metadata").about("RetrieveSeriesMetadata returns instance associated with the given study and series, presented as metadata with the bulk data removed. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveSeriesMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveSeriesMetadata, see [Retrieving metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_metadata).");
            series5 = series5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_series").about("RetrieveSeries returns all instances within the given study and series. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveSeries, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveSeries, see [Retrieving DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_dicom_data).");
            series5 = series5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_instances").about("SearchForInstances returns a list of matching instances. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForInstances, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForInstances, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).");
            series5 = series5.subcommand(mcmd);
        }
        let mut instances6 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, retrieve_instance, retrieve_metadata and retrieve_rendered");
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteInstance deletes an instance associated with the given study, series, and SOP Instance UID. Delete requests are equivalent to the GET requests specified in the Retrieve transaction. Study and series search results can take a few seconds to be updated after an instance is deleted using DeleteInstance. For samples that show how to call DeleteInstance, see [Deleting a study, series, or instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#deleting_a_study_series_or_instance).");
            instances6 = instances6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_instance").about("RetrieveInstance returns instance associated with the given study, series, and SOP Instance UID. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveInstance, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) and [DICOM instances](https://cloud.google.com/healthcare/docs/dicom#dicom_instances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveInstance, see [Retrieving an instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_an_instance).");
            instances6 = instances6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_metadata").about("RetrieveInstanceMetadata returns instance associated with the given study, series, and SOP Instance UID presented as metadata with the bulk data removed. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveInstanceMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveInstanceMetadata, see [Retrieving metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_metadata).");
            instances6 = instances6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_rendered").about("RetrieveRenderedInstance returns instance associated with the given study, series, and SOP Instance UID in an acceptable Rendered Media Type. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveRenderedInstance, see [Rendered resources](https://cloud.google.com/healthcare/docs/dicom#rendered_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveRenderedInstance, see [Retrieving consumer image formats](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_consumer_image_formats).");
            instances6 = instances6.subcommand(mcmd);
        }
        let mut frames7 = SubCommand::with_name("frames")
            .setting(AppSettings::ColoredHelp)
            .about("methods: retrieve_frames and retrieve_rendered");
        {
            let mcmd = SubCommand::with_name("retrieve_frames").about("RetrieveFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4}. For details on the implementation of RetrieveFrames, see [DICOM frames](https://cloud.google.com/healthcare/docs/dicom#dicom_frames) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveFrames, see [Retrieving DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_dicom_data).");
            frames7 = frames7.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_rendered").about("RetrieveRenderedFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers in an acceptable Rendered Media Type. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveRenderedFrames, see [Rendered resources](https://cloud.google.com/healthcare/docs/dicom#rendered_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveRenderedFrames, see [Retrieving consumer image formats](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_consumer_image_formats).");
            frames7 = frames7.subcommand(mcmd);
        }
        instances6 = instances6.subcommand(frames7);
        series5 = series5.subcommand(instances6);
        studies4 = studies4.subcommand(series5);
        hl_7v2_stores3 = hl_7v2_stores3.subcommand(messages4);
        fhir_stores3 = fhir_stores3.subcommand(fhir4);
        dicom_stores3 = dicom_stores3.subcommand(studies4);
        consent_stores3 = consent_stores3.subcommand(user_data_mappings4);
        consent_stores3 = consent_stores3.subcommand(consents4);
        consent_stores3 = consent_stores3.subcommand(consent_artifacts4);
        consent_stores3 = consent_stores3.subcommand(attribute_definitions4);
        datasets2 = datasets2.subcommand(operations3);
        datasets2 = datasets2.subcommand(hl_7v2_stores3);
        datasets2 = datasets2.subcommand(fhir_stores3);
        datasets2 = datasets2.subcommand(dicom_stores3);
        datasets2 = datasets2.subcommand(consent_stores3);
        locations1 = locations1.subcommand(datasets2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_healthcare1 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
