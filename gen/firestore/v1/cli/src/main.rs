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
        let mut app = App::new("firestore1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210220")
            .about("Accesses the NoSQL document database built for automatic scaling, high performance, and ease of application development. ")
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
            .about("sub-resources: databases and locations");
        let mut databases1 = SubCommand::with_name("databases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: export_documents and import_documents");
        {
            let mcmd = SubCommand::with_name("export_documents").about("Exports a copy of all or a subset of documents from Google Cloud Firestore to another storage system, such as Google Cloud Storage. Recent updates to documents may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage.");
            databases1 = databases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_documents").about("Imports documents into Google Cloud Firestore. Existing documents with the same name are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportDocuments operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Firestore.");
            databases1 = databases1.subcommand(mcmd);
        }
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
        let mut collection_groups2 = SubCommand::with_name("collection_groups")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: fields and indexes");
        let mut documents2 = SubCommand::with_name("documents")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_get, batch_write, begin_transaction, commit, create_document, delete, get, list, list_collection_ids, listen, partition_query, patch, rollback, run_query and write");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Gets multiple documents. Documents returned by this method are not guaranteed to be returned in the same order that they were requested.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_write").about("Applies a batch of write operations. The BatchWrite method does not apply the write operations atomically and can apply them out of order. Method does not allow more than one write per document. Each write succeeds or fails independently. See the BatchWriteResponse for the success status of each write. If you require an atomically applied set of writes, use Commit instead.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("begin_transaction").about("Starts a new transaction.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("commit")
                .about("Commits a transaction, while optionally updating documents.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_document").about("Creates a new document.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a document.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single document.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists documents.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_collection_ids")
                .about("Lists all the collection IDs underneath a document.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("listen").about("Listens to changes.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("partition_query").about("Partitions a query by returning partition cursors that can be used to run the query in parallel. The returned partition cursors are split points that can be used by RunQuery as starting/end points for the query results.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates or inserts a document.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Rolls back a transaction.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run_query").about("Runs a query.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("write")
                .about("Streams batches of document updates and deletes, in order.");
            documents2 = documents2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut fields3 = SubCommand::with_name("fields")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and patch");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the metadata and configuration for a Field.");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the field configuration and metadata for this database. Currently, FirestoreAdmin.ListFields only supports listing fields that have been explicitly overridden. To issue this query, call FirestoreAdmin.ListFields with the filter set to `indexConfig.usesAncestorConfig:false`.");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a field configuration. Currently, field updates apply only to single field index configuration. However, calls to FirestoreAdmin.UpdateField should provide a field mask to avoid changing any configuration that the caller isn\'t aware of. The field mask should be specified as: `{ paths: \"index_config\" }`. This call returns a google.longrunning.Operation which may be used to track the status of the field update. The metadata for the operation will be the type FieldOperationMetadata. To configure the default field settings for the database, use the special `Field` with resource name: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`.");
            fields3 = fields3.subcommand(mcmd);
        }
        let mut indexes3 = SubCommand::with_name("indexes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a composite index. This returns a google.longrunning.Operation which may be used to track the status of the creation. The metadata for the operation will be the type IndexOperationMetadata.");
            indexes3 = indexes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a composite index.");
            indexes3 = indexes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a composite index.");
            indexes3 = indexes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists composite indexes.");
            indexes3 = indexes3.subcommand(mcmd);
        }
        collection_groups2 = collection_groups2.subcommand(indexes3);
        collection_groups2 = collection_groups2.subcommand(fields3);
        databases1 = databases1.subcommand(operations2);
        databases1 = databases1.subcommand(documents2);
        databases1 = databases1.subcommand(collection_groups2);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(databases1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_firestore1 as api;

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
