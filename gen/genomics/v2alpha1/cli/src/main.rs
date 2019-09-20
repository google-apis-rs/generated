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
        let mut app = App::new("genomics2_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190919")
            .about("Uploads, processes, queries, and searches Genomics data in the cloud.")
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
        let mut pipelines0 = SubCommand::with_name("pipelines")
            .setting(AppSettings::ColoredHelp)
            .about("methods: run");
        {
            let mcmd = SubCommand::with_name("run").about("Runs a pipeline.  The returned Operation\'s metadata field will contain a\ngoogle.genomics.v2alpha1.Metadata object describing the status of the\npipeline execution.  The [response] field will contain a\ngoogle.genomics.v2alpha1.RunPipelineResponse object if the pipeline\ncompletes successfully.\n\n**Note:** Before you can use this method, the Genomics Service Agent\nmust have access to your project. This is done automatically when the\nCloud Genomics API is first enabled, but if you delete this permission,\nor if you enabled the Cloud Genomics API before the v2alpha1 API\nlaunch, you must disable and re-enable the API to grant the Genomics\nService Agent the required permissions.\nAuthorization requires the following [Google\nIAM](https://cloud.google.com/iam/) permission:\n\n* `genomics.operations.create`\n\n[1]: /genomics/gsa");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut workers0 = SubCommand::with_name("workers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: check_in");
        {
            let mcmd = SubCommand::with_name("check_in").about("The worker uses this method to retrieve the assigned operation and\nprovide periodic status updates.");
            workers0 = workers0.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.\nThe server makes a best effort to cancel the operation, but success is not\nguaranteed. Clients may use Operations.GetOperation\nor Operations.ListOperations\nto check whether the cancellation succeeded or the operation completed\ndespite cancellation.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `genomics.operations.cancel`");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.\nClients can use this method to poll the operation result at intervals as\nrecommended by the API service.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `genomics.operations.get`");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `genomics.operations.list`");
            operations1 = operations1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(operations1);
        app = app.subcommand(workers0);
        app = app.subcommand(projects0);
        app = app.subcommand(pipelines0);

        Self { app }
    }
}
use google_genomics2_alpha1 as api;

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
