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
        let mut app = App::new("cloudtrace2_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200504")
            .about("Sends application trace data to Cloud Trace for viewing. Trace data is collected for all App Engine applications by default. Trace data from other applications can be provided using this API. This library is used to interact with the Cloud Trace API directly. If you are looking to instrument your application for Cloud Trace, we recommend using OpenCensus.\n")
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
            .about("sub-resources: trace_sinks");
        let mut trace_sinks1 = SubCommand::with_name("trace_sinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a sink that exports trace spans to a destination.  The\nexport of newly-ingested traces begins immediately, unless the sink\'s\n`writer_identity` is not permitted to write to the destination.  A sink can\nexport traces only from the resource owning the sink (the \'parent\').");
            trace_sinks1 = trace_sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sink.");
            trace_sinks1 = trace_sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a trace sink by name under the parent resource (GCP project).");
            trace_sinks1 = trace_sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all sinks for the parent resource (GCP project).");
            trace_sinks1 = trace_sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a sink.  This method updates fields in the existing sink according\nto the provided update mask. The sink\'s name cannot be changed nor any\noutput-only fields (e.g. the writer_identity).");
            trace_sinks1 = trace_sinks1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(trace_sinks1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_cloudtrace2_beta1 as api;

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
