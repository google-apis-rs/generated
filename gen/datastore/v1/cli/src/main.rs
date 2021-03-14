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
        let mut app = App::new("datastore1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210220")
            .about("Accesses the schemaless NoSQL database to provide fully managed, robust, scalable storage for your application. ")
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
                        .about("methods: allocate_ids, begin_transaction, commit, export, import, lookup, reserve_ids, rollback and run_query");
        {
            let mcmd = SubCommand::with_name("allocate_ids").about("Allocates IDs for the given keys, which is useful for referencing an entity before it is inserted.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("begin_transaction").about("Begins a new transaction.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("commit").about(
                "Commits a transaction, optionally creating, deleting or modifying some entities.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports a copy of all or a subset of entities from Google Cloud Datastore to another storage system, such as Google Cloud Storage. Recent updates to entities may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports entities into Google Cloud Datastore. Existing entities with the same key are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportEntities operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Datastore.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up entities by key.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reserve_ids").about(
                "Prevents the supplied keys\' IDs from being auto-allocated by Cloud Datastore.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Rolls back a transaction.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run_query").about("Queries for entities.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut indexes1 = SubCommand::with_name("indexes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates the specified index. A newly created index\'s initial state is `CREATING`. On completion of the returned google.longrunning.Operation, the state will be `READY`. If the index already exists, the call will return an `ALREADY_EXISTS` status. During index creation, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, removing the index with delete, then re-creating the index with create. Indexes with a single property cannot be created.");
            indexes1 = indexes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing index. An index can only be deleted if it is in a `READY` or `ERROR` state. On successful execution of the request, the index will be in a `DELETING` state. And on completion of the returned google.longrunning.Operation, the index will be removed. During index deletion, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, followed by calling delete again.");
            indexes1 = indexes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an index.");
            indexes1 = indexes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the indexes that match the specified filters. Datastore uses an eventually consistent query to fetch the list of indexes and may occasionally return stale results.");
            indexes1 = indexes1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(indexes1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_datastore1 as api;

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
