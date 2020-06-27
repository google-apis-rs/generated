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
        let mut app = App::new("firestore1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200522")
            .about("Accesses the NoSQL document database built for automatic scaling, high performance, and ease of application development.\n")
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
            .about("sub-resources: databases");
        let mut databases1 = SubCommand::with_name("databases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: export_documents and import_documents");
        {
            let mcmd = SubCommand::with_name("export_documents").about("Exports a copy of all or a subset of documents from Google Cloud Firestore\nto another storage system, such as Google Cloud Storage. Recent updates to\ndocuments may not be reflected in the export. The export occurs in the\nbackground and its progress can be monitored and managed via the\nOperation resource that is created. The output of an export may only be\nused once the associated operation is done. If an export operation is\ncancelled before completion it may leave partial data behind in Google\nCloud Storage.");
            databases1 = databases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_documents").about("Imports documents into Google Cloud Firestore. Existing documents with the\nsame name are overwritten. The import occurs in the background and its\nprogress can be monitored and managed via the Operation resource that is\ncreated. If an ImportDocuments operation is cancelled, it is possible\nthat a subset of the data has already been imported to Cloud Firestore.");
            databases1 = databases1.subcommand(mcmd);
        }
        let mut documents2 = SubCommand::with_name("documents")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_get, begin_transaction, commit, create_document, delete, get, list, list_collection_ids, listen, patch, rollback, run_query and write");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Gets multiple documents.\n\nDocuments returned by this method are not guaranteed to be returned in the\nsame order that they were requested.");
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
        let mut indexes2 = SubCommand::with_name("indexes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates the specified index.\nA newly created index\'s initial state is `CREATING`. On completion of the\nreturned google.longrunning.Operation, the state will be `READY`.\nIf the index already exists, the call will return an `ALREADY_EXISTS`\nstatus.\n\nDuring creation, the process could result in an error, in which case the\nindex will move to the `ERROR` state. The process can be recovered by\nfixing the data that caused the error, removing the index with\ndelete, then re-creating the index with\ncreate.\n\nIndexes with a single field cannot be created.");
            indexes2 = indexes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an index.");
            indexes2 = indexes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an index.");
            indexes2 = indexes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the indexes that match the specified filters.");
            indexes2 = indexes2.subcommand(mcmd);
        }
        databases1 = databases1.subcommand(indexes2);
        databases1 = databases1.subcommand(documents2);
        projects0 = projects0.subcommand(databases1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_firestore1_beta1 as api;

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
