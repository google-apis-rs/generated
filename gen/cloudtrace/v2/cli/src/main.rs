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
        let mut app = App::new("cloudtrace2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200622")
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
            .about("sub-resources: traces");
        let mut traces1 = SubCommand::with_name("traces")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_write");
        {
            let mcmd = SubCommand::with_name("batch_write").about("Sends new spans to new or existing traces. You cannot update\nexisting spans.\nIn this case, writing traces is not considered an active developer\nmethod since traces are machine generated.");
            traces1 = traces1.subcommand(mcmd);
        }
        let mut spans2 = SubCommand::with_name("spans")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create_span");
        {
            let mcmd = SubCommand::with_name("create_span").about("Creates a new span.\nIn this case, writing traces is not considered an active developer\nmethod since traces are machine generated.");
            spans2 = spans2.subcommand(mcmd);
        }
        traces1 = traces1.subcommand(spans2);
        projects0 = projects0.subcommand(traces1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_cloudtrace2 as api;

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
