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
            .version("0.1.0-20210304")
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
            let mcmd = SubCommand::with_name("get").about("Returns the Google service account that is used by Storage Transfer Service to access buckets in the project where transfers run or in other projects. Each Google service account is associated with one Google Cloud Platform Console project. Users should add this service account to the Google Cloud Storage bucket ACLs to grant access to Storage Transfer Service. This service account is created and owned by Storage Transfer Service and can only be used by Storage Transfer Service.");
            google_service_accounts0 = google_service_accounts0.subcommand(mcmd);
        }
        let mut transfer_jobs0 = SubCommand::with_name("transfer_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list, patch and run");
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
            let mcmd = SubCommand::with_name("patch").about("Updates a transfer job. Updating a job\'s transfer spec does not affect transfer operations that are running already. **Note:** The job\'s status field can be modified using this RPC (for example, to set a job\'s status to DELETED, DISABLED, or ENABLED).");
            transfer_jobs0 = transfer_jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Attempts to start a new TransferOperation for the current TransferJob. A TransferJob has a maximum of one active TransferOperation. If this method is called while a TransferOperation is active, an error wil be returned.");
            transfer_jobs0 = transfer_jobs0.subcommand(mcmd);
        }
        let mut transfer_operations0 = SubCommand::with_name("transfer_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get, list, pause and resume");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a transfer. Use the transferOperations.get method to check if the cancellation succeeded or if the operation completed despite the `cancel` request. When you cancel an operation, the currently running transfer is interrupted. For recurring transfer jobs, the next instance of the transfer job will still run. For example, if your job is configured to run every day at 1pm and you cancel Monday\'s operation at 1:05pm, Monday\'s transfer will stop. However, a transfer job will still be attempted on Tuesday. This applies only to currently running operations. If an operation is not currently running, `cancel` does nothing. *Caution:* Canceling a transfer job can leave your data in an unknown state. We recommend that you restore the state at both the destination and the source after the `cancel` request completes so that your data is in a consistent state. When you cancel a job, the next job computes a delta of files and may repair any inconsistent state. For instance, if you run a job every day, and today\'s job found 10 new files and transferred five files before you canceled the job, tomorrow\'s transfer operation will compute a new delta with the five files that were not copied today plus any new files discovered tomorrow.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            transfer_operations0 = transfer_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists transfer operations. Operations are ordered by their creation time in reverse chronological order.");
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
