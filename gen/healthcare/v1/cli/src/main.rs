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
            .version("0.1.0-20200612")
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
            .about("sub-resources: datasets");
        let mut datasets2 = SubCommand::with_name("datasets")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, deidentify, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new health dataset. Results are returned through the\nOperation interface which returns either an\n`Operation.response` which contains a Dataset or\n`Operation.error`. The metadata\nfield type is OperationMetadata.\nA Google Cloud Platform project can contain up to 500 datasets across all\nregions.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deidentify").about("Creates a new dataset containing de-identified data from the source\ndataset. The metadata field type\nis OperationMetadata.\nIf the request is successful, the\nresponse field type is\nDeidentifySummary.\nIf errors occur, error is set.\nThe LRO result may still be successful if de-identification fails for some\nDICOM instances. The new de-identified dataset will not contain these\nfailed resources. Failed resource totals are tracked in\nOperation.metadata.\nError details are also logged to Cloud Logging. For more information,\nsee [Viewing logs](/healthcare/docs/how-tos/logging).");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified health dataset and all data contained in the dataset.\nDeleting a dataset does not affect the sources from which the dataset was\nimported (if any).");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets any metadata associated with a dataset.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
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
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            datasets2 = datasets2.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies data from the source store and writes it to the destination\nstore. The metadata field type\nis OperationMetadata.\nIf the request is successful, the\nresponse field type is\nDeidentifyDicomStoreSummary. If errors occur,\nerror is set.\nThe LRO result may still be successful if de-identification fails for some\nDICOM instances. The output DICOM store will not contain\nthese failed resources. Failed resource totals are tracked in\nOperation.metadata.\nError details are also logged to Cloud Logging\n(see [Viewing logs](/healthcare/docs/how-tos/logging)).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified DICOM store and removes all images that are contained\nwithin it.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports data to the specified destination by copying it from the DICOM\nstore.\nErrors are also logged to Cloud Logging. For more information,\nsee [Viewing logs](/healthcare/docs/how-tos/logging).\nThe metadata field type is\nOperationMetadata.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified DICOM store.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports data into the DICOM store by copying it from the specified source.\nErrors are logged to Cloud Logging. For more information, see\n[Viewing logs](/healthcare/docs/how-tos/logging). The\nmetadata field type is\nOperationMetadata.");
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
            let mcmd = SubCommand::with_name("search_for_instances").about("SearchForInstances returns a list of matching instances. See\n[Search Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_series").about("SearchForSeries returns a list of matching series. See\n[Search Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_studies").about("SearchForStudies returns a list of matching studies. See\n[Search Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("store_instances").about("StoreInstances stores DICOM instances associated with study instance unique\nidentifiers (SUID). See\n[Store Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5).");
            dicom_stores3 = dicom_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
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
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies data from the source store and writes it to the destination\nstore. The metadata field type\nis OperationMetadata.\nIf the request is successful, the\nresponse field type is\nDeidentifyFhirStoreSummary. If errors occur,\nerror is set.\nError details are also logged to Cloud Logging\n(see [Viewing logs](/healthcare/docs/how-tos/logging)).");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified FHIR store and removes all resources within it.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Export resources from the FHIR store to the specified destination.\n\nThis method returns an Operation that can\nbe used to track the status of the export by calling\nGetOperation.\n\nImmediate fatal errors appear in the\nerror field, errors are also logged\nto Cloud Logging (see [Viewing\nlogs](/healthcare/docs/how-tos/logging)).\nOtherwise, when the operation finishes, a detailed response of type\nExportResourcesResponse is returned in the\nresponse field.\nThe metadata field type for this\noperation is OperationMetadata.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the configuration of the specified FHIR store.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports resources to the FHIR store by loading data from the specified\nsources. This method is optimized to load large quantities of data using\nimport semantics that ignore some FHIR store configuration options and are\nnot suitable for all use cases. It is primarily intended to load data into\nan empty FHIR store that is not being used by other clients. In cases\nwhere this method is not appropriate, consider using ExecuteBundle to\nload data.\n\nEvery resource in the input must contain a client-supplied ID. Each\nresource is stored using the supplied ID regardless of the\nenable_update_create setting on the FHIR\nstore.\n\nThe import process does not enforce referential integrity, regardless of\nthe\ndisable_referential_integrity\nsetting on the FHIR store. This allows the import of resources with\narbitrary interdependencies without considering grouping or ordering, but\nif the input data contains invalid references or if some resources fail to\nbe imported, the FHIR store might be left in a state that violates\nreferential integrity.\n\nThe import process does not trigger Pub/Sub notification or BigQuery\nstreaming update, regardless of how those are configured on the FHIR store.\n\nIf a resource with the specified ID already exists, the most recent\nversion of the resource is overwritten without creating a new historical\nversion, regardless of the\ndisable_resource_versioning\nsetting on the FHIR store. If transient failures occur during the import,\nit\'s possible that successfully imported resources will be overwritten\nmore than once.\n\nThe import operation is idempotent unless the input data contains multiple\nvalid resources with the same ID but different contents. In that case,\nafter the import completes, the store contains exactly one resource\nwith that ID but there is no ordering guarantee on which version of the\ncontents it will have. The operation result counters do not count\nduplicate IDs as an error and count one success for each resource in\nthe input, which might result in a success count larger than the number\nof resources in the FHIR store. This often occurs when importing data\norganized in bundles produced by Patient-everything\nwhere each bundle contains its own copy of a resource such as Practitioner\nthat might be referred to by many patients.\n\nIf some resources fail to import, for example due to parsing errors,\nsuccessfully imported resources are not rolled back.\n\nThe location and format of the input data is specified by the parameters\nbelow. Note that if no format is specified, this method assumes the\n`BUNDLE` format. When using the `BUNDLE` format this method ignores the\n`Bundle.type` field, except that `history` bundles are rejected, and does\nnot apply any of the bundle processing semantics for batch or transaction\nbundles. Unlike in ExecuteBundle, transaction bundles are not executed\nas a single transaction and bundle-internal references are not rewritten.\nThe bundle is treated as a collection of resources to be written as\nprovided in `Bundle.entry.resource`, ignoring `Bundle.entry.request`. As\nan example, this allows the import of `searchset` bundles produced by a\nFHIR search or\nPatient-everything operation.\n\nThis method returns an Operation that can\nbe used to track the status of the import by calling\nGetOperation.\n\nImmediate fatal errors appear in the\nerror field, errors are also logged\nto Cloud Logging (see [Viewing\nlogs](/healthcare/docs/how-tos/logging)). Otherwise, when the\noperation finishes, a detailed response of type ImportResourcesResponse\nis returned in the response field.\nThe metadata field type for this\noperation is OperationMetadata.");
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
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            fhir_stores3 = fhir_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
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
                "Deletes the specified HL7v2 store and removes all messages that it\ncontains.",
            );
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified HL7v2 store.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
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
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            hl_7v2_stores3 = hl_7v2_stores3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut studies4 = SubCommand::with_name("studies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, retrieve_metadata, retrieve_study, search_for_instances, search_for_series and store_instances");
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteStudy deletes all instances within the given study. Delete requests\nare equivalent to the GET requests specified in the Retrieve transaction.\nThe method returns an Operation which\nwill be marked successful when the deletion is complete.");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_metadata").about("RetrieveStudyMetadata returns instance associated with the given study\npresented as metadata with the bulk data removed. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_study").about("RetrieveStudy returns all instances within the given study. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_instances").about("SearchForInstances returns a list of matching instances. See\n[Search Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_series").about("SearchForSeries returns a list of matching series. See\n[Search Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6).");
            studies4 = studies4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("store_instances").about("StoreInstances stores DICOM instances associated with study instance unique\nidentifiers (SUID). See\n[Store Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5).");
            studies4 = studies4.subcommand(mcmd);
        }
        let mut fhir4 = SubCommand::with_name("fhir")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: patient_everything, resource_purge, capabilities, create, delete, execute_bundle, history, patch, read, search, update and vread");
        {
            let mcmd = SubCommand::with_name("patient_everything").about("Retrieves a Patient resource and resources related to that patient.\n\nImplements the FHIR extended operation Patient-everything\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/patient-operations.html#everything),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/patient-operations.html#everything),\n[R4](http://hl7.org/implement/standards/fhir/R4/patient-operations.html#everything)).\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `searchset`, containing the results of the\noperation.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.\n\nThe resources in scope for the response are:\n\n* The patient resource itself.\n* All the resources directly referenced by the patient resource.\n* Resources directly referencing the patient resource that meet the\n  inclusion criteria. The inclusion criteria are based on the membership\n  rules in the patient compartment definition\n  ([DSTU2](http://hl7.org/fhir/DSTU2/compartment-patient.html),\n  [STU3](http://www.hl7.org/fhir/stu3/compartmentdefinition-patient.html),\n  [R4](http://hl7.org/fhir/R4/compartmentdefinition-patient.html)), which\n  details the eligible resource types and referencing search parameters.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resource_purge").about("Deletes all the historical versions of a resource (excluding the current\nversion) from the FHIR store. To remove all versions of a resource, first\ndelete the current version and then call this method.\n\nThis is not a FHIR standard operation.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("capabilities").about("Gets the FHIR capability statement\n([STU3](http://hl7.org/implement/standards/fhir/STU3/capabilitystatement.html),\n[R4](http://hl7.org/implement/standards/fhir/R4/capabilitystatement.html)),\nor the [conformance\nstatement](http://hl7.org/implement/standards/fhir/DSTU2/conformance.html)\nin the DSTU2 case for the store, which contains a description of\nfunctionality supported by the server.\n\nImplements the FHIR standard capabilities interaction\n([STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#capabilities),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#capabilities)),\nor the [conformance\ninteraction](http://hl7.org/implement/standards/fhir/DSTU2/http.html#conformance)\nin the DSTU2 case.\n\nOn success, the response body will contain a JSON-encoded representation\nof a `CapabilityStatement` resource.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a FHIR resource.\n\nImplements the FHIR standard create interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#create),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#create),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#create)),\nwhich creates a new resource with a server-assigned resource ID.\n\nThe request body must contain a JSON-encoded FHIR resource, and the request\nheaders must contain `Content-Type: application/fhir+json`.\n\nOn success, the response body will contain a JSON-encoded representation\nof the resource as it was created on the server, including the\nserver-assigned resource ID and version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a FHIR resource.\n\nImplements the FHIR standard delete interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#delete),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#delete),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#delete)).\n\nNote: Unless resource versioning is disabled by setting the\ndisable_resource_versioning flag\non the FHIR store, the deleted resources will be moved to a history\nrepository that can still be retrieved through vread\nand related methods, unless they are removed by the\npurge method.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute_bundle").about("Executes all the requests in the given Bundle.\n\nImplements the FHIR standard batch/transaction interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#transaction),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#transaction),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#transaction)).\n\nSupports all interactions within a bundle, except search. This method\naccepts Bundles of type `batch` and `transaction`, processing them\naccording to the batch processing rules\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#2.1.0.16.1),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.1),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#brules))\nand transaction processing rules\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#2.1.0.16.2),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.2),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#trules)).\n\nThe request body must contain a JSON-encoded FHIR `Bundle` resource, and\nthe request headers must contain `Content-Type: application/fhir+json`.\n\nFor a batch bundle or a successful transaction the response body will\ncontain a JSON-encoded representation of a `Bundle` resource of type\n`batch-response` or `transaction-response` containing one entry for each\nentry in the request, with the outcome of processing the entry. In the\ncase of an error for a transaction bundle, the response body will contain\na JSON-encoded `OperationOutcome` resource describing the reason for the\nerror. If the request cannot be mapped to a valid API method on a FHIR\nstore, a generic GCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("history").about("Lists all the versions of a resource (including the current version and\ndeleted versions) from the FHIR store.\n\nImplements the per-resource form of the FHIR standard history interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#history),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#history),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#history)).\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `history`, containing the version history\nsorted from most recent to oldest versions.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates part of an existing resource by applying the operations specified\nin a [JSON Patch](http://jsonpatch.com/) document.\n\nImplements the FHIR standard patch interaction\n([STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#patch),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#patch)).\n\nDSTU2 doesn\'t define a patch method, but the server supports it in the same\nway it supports STU3.\n\nThe request body must contain a JSON Patch document, and the request\nheaders must contain `Content-Type: application/json-patch+json`.\n\nOn success, the response body will contain a JSON-encoded representation\nof the updated resource, including the server-assigned version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("read").about("Gets the contents of a FHIR resource.\n\nImplements the FHIR standard read interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#read),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#read),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#read)).\n\nAlso supports the FHIR standard conditional read interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#cread),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#cread),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#cread))\nspecified by supplying an `If-Modified-Since` header with a date/time value\nor an `If-None-Match` header with an ETag value.\n\nOn success, the response body will contain a JSON-encoded representation\nof the resource.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches for resources in the given FHIR store according to criteria\nspecified as query parameters.\n\nImplements the FHIR standard search interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#search),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#search),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#search))\nusing the search semantics described in the FHIR Search specification\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/search.html),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/search.html),\n[R4](http://hl7.org/implement/standards/fhir/R4/search.html)).\n\nSupports three methods of search defined by the specification:\n\n*  `GET [base]?[parameters]` to search across all resources.\n*  `GET [base]/[type]?[parameters]` to search resources of a specified\ntype.\n*  `POST [base]/[type]/_search?[parameters]` as an alternate form having\nthe same semantics as the `GET` method.\n\nThe `GET` methods do not support compartment searches. The `POST` method\ndoes not support `application/x-www-form-urlencoded` search parameters.\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `searchset`, containing the results of the\nsearch.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.\n\nThe server\'s capability statement, retrieved through\ncapabilities, indicates what search parameters\nare supported on each FHIR resource. A list of all search parameters\ndefined by the specification can be found in the FHIR Search Parameter\nRegistry\n([STU3](http://hl7.org/implement/standards/fhir/STU3/searchparameter-registry.html),\n[R4](http://hl7.org/implement/standards/fhir/R4/searchparameter-registry.html)).\nFHIR search parameters for DSTU2 can be found on each resource\'s definition\npage.\n\nSupported search modifiers: `:missing`, `:exact`, `:contains`, `:text`,\n`:in`, `:not-in`, `:above`, `:below`, `:[type]`, `:not`, and `:recurse`.\n\nSupported search result parameters: `_sort`, `_count`, `_include`,\n`_revinclude`, `_summary=text`, `_summary=data`, and `_elements`.\n\nThe maximum number of search results returned defaults to 100, which can\nbe overridden by the `_count` parameter up to a maximum limit of 1000. If\nthere are additional results, the returned `Bundle` will contain\npagination links.\n\nResources with a total size larger than 5MB or a field count larger than\n50,000 might not be fully searchable as the server might trim its generated\nsearch index in those cases.\n\nNote: FHIR resources are indexed asynchronously, so there might be a slight\ndelay between the time a resource is created or changes and when the change\nis reflected in search results.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the entire contents of a resource.\n\nImplements the FHIR standard update interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#update),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#update),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#update)).\n\nIf the specified resource does\nnot exist and the FHIR store has\nenable_update_create set, creates the\nresource with the client-specified ID.\n\nThe request body must contain a JSON-encoded FHIR resource, and the request\nheaders must contain `Content-Type: application/fhir+json`. The resource\nmust contain an `id` element having an identical value to the ID in the\nREST path of the request.\n\nOn success, the response body will contain a JSON-encoded representation\nof the updated resource, including the server-assigned version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("vread").about("Gets the contents of a version (current or historical) of a FHIR resource\nby version ID.\n\nImplements the FHIR standard vread interaction\n([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#vread),\n[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#vread),\n[R4](http://hl7.org/implement/standards/fhir/R4/http.html#vread)).\n\nOn success, the response body will contain a JSON-encoded representation\nof the resource.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.");
            fhir4 = fhir4.subcommand(mcmd);
        }
        let mut messages4 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, ingest, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a message and sends a notification to the Cloud Pub/Sub topic. If\nconfigured, the MLLP adapter listens to messages created by this method and\nsends those back to the hospital. A successful response indicates the\nmessage has been persisted to storage and a Cloud Pub/Sub notification has\nbeen sent. Sending to the hospital by the MLLP adapter happens\nasynchronously.");
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
            let mcmd = SubCommand::with_name("ingest").about("Ingests a new HL7v2 message from the hospital and sends a notification to\nthe Cloud Pub/Sub topic. Return is an HL7v2 ACK message if the message was\nsuccessfully stored. Otherwise an error is returned.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the messages in the given HL7v2 store with support for filtering.\n\nNote: HL7v2 messages are indexed asynchronously, so there might be a slight\ndelay between the time a message is created and when it can be found\nthrough a filter.");
            messages4 = messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update the message.\n\nThe contents of the message in Message.data and data extracted from\nthe contents such as Message.create_time cannot be altered. Only the\nMessage.labels field is allowed to be updated. The labels in the\nrequest are merged with the existing set of labels. Existing labels with\nthe same keys are updated.");
            messages4 = messages4.subcommand(mcmd);
        }
        let mut series5 = SubCommand::with_name("series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, retrieve_metadata, retrieve_series and search_for_instances");
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteSeries deletes all instances within the given study and series.\nDelete requests are equivalent to the GET requests specified in the\nRetrieve transaction.\nThe method returns an Operation which\nwill be marked successful when the deletion is complete.");
            series5 = series5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_metadata").about("RetrieveSeriesMetadata returns instance associated with the given study and\nseries, presented as metadata with the bulk data removed. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            series5 = series5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_series").about("RetrieveSeries returns all instances within the given study and series. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            series5 = series5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_instances").about("SearchForInstances returns a list of matching instances. See\n[Search Transaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6).");
            series5 = series5.subcommand(mcmd);
        }
        let mut instances6 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, retrieve_instance, retrieve_metadata and retrieve_rendered");
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteInstance deletes an instance associated with the given study, series,\nand SOP Instance UID. Delete requests are equivalent to the GET requests\nspecified in the Retrieve transaction.\nStudy and series search results can take a few seconds to be updated after\nan instance is deleted using DeleteInstance.");
            instances6 = instances6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_instance").about("RetrieveInstance returns instance associated with the given study, series,\nand SOP Instance UID. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            instances6 = instances6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_metadata").about("RetrieveInstanceMetadata returns instance associated with the given study,\nseries, and SOP Instance UID presented as metadata with the bulk data\nremoved. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            instances6 = instances6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_rendered").about("RetrieveRenderedInstance returns instance associated with the given study,\nseries, and SOP Instance UID in an acceptable Rendered Media Type. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            instances6 = instances6.subcommand(mcmd);
        }
        let mut frames7 = SubCommand::with_name("frames")
            .setting(AppSettings::ColoredHelp)
            .about("methods: retrieve_frames and retrieve_rendered");
        {
            let mcmd = SubCommand::with_name("retrieve_frames").about("RetrieveFrames returns instances associated with the given study, series,\nSOP Instance UID and frame numbers. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4}.");
            frames7 = frames7.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retrieve_rendered").about("RetrieveRenderedFrames returns instances associated with the given study,\nseries, SOP Instance UID and frame numbers in an acceptable Rendered Media\nType. See\n[RetrieveTransaction]\n(http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4).");
            frames7 = frames7.subcommand(mcmd);
        }
        instances6 = instances6.subcommand(frames7);
        series5 = series5.subcommand(instances6);
        studies4 = studies4.subcommand(series5);
        hl_7v2_stores3 = hl_7v2_stores3.subcommand(messages4);
        fhir_stores3 = fhir_stores3.subcommand(fhir4);
        dicom_stores3 = dicom_stores3.subcommand(studies4);
        datasets2 = datasets2.subcommand(operations3);
        datasets2 = datasets2.subcommand(hl_7v2_stores3);
        datasets2 = datasets2.subcommand(fhir_stores3);
        datasets2 = datasets2.subcommand(dicom_stores3);
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
