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
        let mut app = App::new("firestore1_beta2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200405")
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
        let mut collection_groups2 = SubCommand::with_name("collection_groups")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: fields and indexes");
        let mut fields3 = SubCommand::with_name("fields")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and patch");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the metadata and configuration for a Field.");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the field configuration and metadata for this database.\n\nCurrently, FirestoreAdmin.ListFields only supports listing fields\nthat have been explicitly overridden. To issue this query, call\nFirestoreAdmin.ListFields with the filter set to\n`indexConfig.usesAncestorConfig:false`.");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a field configuration. Currently, field updates apply only to\nsingle field index configuration. However, calls to\nFirestoreAdmin.UpdateField should provide a field mask to avoid\nchanging any configuration that the caller isn\'t aware of. The field mask\nshould be specified as: `{ paths: \"index_config\" }`.\n\nThis call returns a google.longrunning.Operation which may be used to\ntrack the status of the field update. The metadata for\nthe operation will be the type FieldOperationMetadata.\n\nTo configure the default field settings for the database, use\nthe special `Field` with resource name:\n`projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`.");
            fields3 = fields3.subcommand(mcmd);
        }
        let mut indexes3 = SubCommand::with_name("indexes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a composite index. This returns a google.longrunning.Operation\nwhich may be used to track the status of the creation. The metadata for\nthe operation will be the type IndexOperationMetadata.");
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
        databases1 = databases1.subcommand(collection_groups2);
        projects0 = projects0.subcommand(databases1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_firestore1_beta2 as api;

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
