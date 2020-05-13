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
        let mut app = App::new("storagetransfer1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200507")
            .about("Transfers data from external data sources to a Google Cloud Storage bucket or between Google Cloud Storage buckets.")
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
        let mut google_service_accounts0 = SubCommand::with_name("google_service_accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the Google service account that is used by Storage Transfer\nService to access buckets in the project where transfers\nrun or in other projects. Each Google service account is associated\nwith one Google Cloud Platform Console project. Users\nshould add this service account to the Google Cloud Storage bucket\nACLs to grant access to Storage Transfer Service. This service\naccount is created and owned by Storage Transfer Service and can\nonly be used by Storage Transfer Service.");
            google_service_accounts0 = google_service_accounts0.subcommand(mcmd);
        }
        let mut transfer_jobs0 = SubCommand::with_name("transfer_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a transfer job that runs periodically.");
            transfer_jobs0 = transfer_jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a transfer job.");
            transfer_jobs0 = transfer_jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists transfer jobs.");
            transfer_jobs0 = transfer_jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a transfer job. Updating a job\'s transfer spec does not affect\ntransfer operations that are running already. Updating a job\'s schedule\nis not allowed.\n\n**Note:** The job\'s status field can be modified\nusing this RPC (for example, to set a job\'s status to\nDELETED,\nDISABLED, or\nENABLED).");
            transfer_jobs0 = transfer_jobs0.subcommand(mcmd);
        }
        let mut transfer_operations0 = SubCommand::with_name("transfer_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get, list, pause and resume");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a transfer. Use the get method to check whether the cancellation succeeded or whether the operation completed despite cancellation.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists transfer operations.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Pauses a transfer operation.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume")
                .about("Resumes a transfer operation that is paused.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        app = app.subcommand(transfer_operations0);
        app = app.subcommand(transfer_jobs0);
        app = app.subcommand(google_service_accounts0);

        Self { app }
    }
}
use google_storagetransfer1 as api;

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
