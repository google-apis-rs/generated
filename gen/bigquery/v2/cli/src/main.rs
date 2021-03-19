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
        let mut app = App::new("bigquery2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210303")
            .about("A data platform for customers to create, manage, share and query data.")
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
        let mut datasets0 = SubCommand::with_name("datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name.");
            datasets0 = datasets0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the dataset specified by datasetID.");
            datasets0 = datasets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new empty dataset.");
            datasets0 = datasets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all datasets in the specified project to which you have been granted the READER dataset role.");
            datasets0 = datasets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics.");
            datasets0 = datasets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.");
            datasets0 = datasets0.subcommand(mcmd);
        }
        let mut jobs0 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get, get_query_results, insert, list and query");
        {
            let mcmd = SubCommand::with_name("cancel").about("Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a specific job. Job information is available for a six month period after creation. Requires that you\'re the person who ran the job, or have the Is Owner project role.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_query_results")
                .about("Retrieves the results of a query job.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Starts a new asynchronous job. Requires the Can View project role.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query").about("Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        let mut models0 = SubCommand::with_name("models")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the model specified by modelId from the dataset.");
            models0 = models0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the specified model resource by model ID.");
            models0 = models0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all models in the specified dataset. Requires the READER dataset role.",
            );
            models0 = models0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patch specific fields in the specified model.");
            models0 = models0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_service_account and list");
        {
            let mcmd = SubCommand::with_name("get_service_account").about("Returns the email address of the service account for your project used for interactions with Google Cloud KMS.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all projects to which you have been granted any project role.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut routines0 = SubCommand::with_name("routines")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the routine specified by routineId from the dataset.");
            routines0 = routines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the specified routine resource by routine ID.");
            routines0 = routines0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new routine in the dataset.");
            routines0 = routines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all routines in the specified dataset. Requires the READER dataset role.",
            );
            routines0 = routines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates information in an existing routine. The update method replaces the entire Routine resource.");
            routines0 = routines0.subcommand(mcmd);
        }
        let mut row_access_policies0 = SubCommand::with_name("row_access_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            row_access_policies0 = row_access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all row access policies on the specified table.");
            row_access_policies0 = row_access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            row_access_policies0 = row_access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            row_access_policies0 = row_access_policies0.subcommand(mcmd);
        }
        let mut tabledata0 = SubCommand::with_name("tabledata")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert_all and list");
        {
            let mcmd = SubCommand::with_name("insert_all").about("Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role.");
            tabledata0 = tabledata0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves table data from a specified set of rows. Requires the READER dataset role.");
            tabledata0 = tabledata0.subcommand(mcmd);
        }
        let mut tables0 = SubCommand::with_name("tables")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_iam_policy, insert, list, patch, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new, empty table in the dataset.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all tables in the specified dataset. Requires the READER dataset role.",
            );
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource.");
            tables0 = tables0.subcommand(mcmd);
        }
        app = app.subcommand(tables0);
        app = app.subcommand(tabledata0);
        app = app.subcommand(row_access_policies0);
        app = app.subcommand(routines0);
        app = app.subcommand(projects0);
        app = app.subcommand(models0);
        app = app.subcommand(jobs0);
        app = app.subcommand(datasets0);

        Self { app }
    }
}
use google_bigquery2 as api;

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
