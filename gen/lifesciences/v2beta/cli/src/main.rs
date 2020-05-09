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
        let mut app = App::new("lifesciences2_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200507")
            .about("Cloud Life Sciences is a suite of services and tools for managing, processing, and transforming life sciences data.")
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
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.\nThe server makes a best effort to cancel the operation, but success is not\nguaranteed. Clients may use Operations.GetOperation\nor Operations.ListOperations\nto check whether the cancellation succeeded or the operation completed\ndespite cancellation.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `lifesciences.operations.cancel`");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.\nClients can use this method to poll the operation result at intervals as\nrecommended by the API service.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `lifesciences.operations.get`");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `lifesciences.operations.list`");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut pipelines2 = SubCommand::with_name("pipelines")
            .setting(AppSettings::ColoredHelp)
            .about("methods: run");
        {
            let mcmd = SubCommand::with_name("run").about("Runs a pipeline.  The returned Operation\'s metadata field will contain a\ngoogle.cloud.lifesciences.v2beta.Metadata object describing the status\nof the pipeline execution. The\nresponse field will contain a\ngoogle.cloud.lifesciences.v2beta.RunPipelineResponse object if the\npipeline completes successfully.\n\n**Note:** Before you can use this method, the *Life Sciences Service Agent*\nmust have access to your project. This is done automatically when the\nCloud Life Sciences API is first enabled, but if you delete this permission\nyou must disable and re-enable the API to grant the Life Sciences\nService Agent the required permissions.\nAuthorization requires the following [Google\nIAM](https://cloud.google.com/iam/) permission:\n\n* `lifesciences.workflows.run`");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(pipelines2);
        locations1 = locations1.subcommand(operations2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_lifesciences2_beta as api;

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
