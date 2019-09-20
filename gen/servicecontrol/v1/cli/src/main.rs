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
        let mut app = App::new("servicecontrol1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190912")
            .about("Provides control plane functionality to managed services, such as logging, monitoring, and status checks.")
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
        let mut services0 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: allocate_quota, check and report");
        {
            let mcmd = SubCommand::with_name("allocate_quota").about("Attempts to allocate quota for the specified consumer. It should be called\nbefore the operation is executed.\n\nThis method requires the `servicemanagement.services.quota`\npermission on the specified service. For more information, see\n[Cloud IAM](https://cloud.google.com/iam).\n\n**NOTE:** The client **must** fail-open on server errors `INTERNAL`,\n`UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system\nreliability, the server may inject these errors to prohibit any hard\ndependency on the quota functionality.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("check").about("Checks whether an operation on a service should be allowed to proceed\nbased on the configuration of the service and related policies. It must be\ncalled before the operation is executed.\n\nIf feasible, the client should cache the check results and reuse them for\n60 seconds. In case of any server errors, the client should rely on the\ncached results for much longer time to avoid outage.\nWARNING: There is general 60s delay for the configuration and policy\npropagation, therefore callers MUST NOT depend on the `Check` method having\nthe latest policy information.\n\nNOTE: the CheckRequest has the size limit of 64KB.\n\nThis method requires the `servicemanagement.services.check` permission\non the specified service. For more information, see\n[Cloud IAM](https://cloud.google.com/iam).");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report").about("Reports operation results to Google Service Control, such as logs and\nmetrics. It should be called after an operation is completed.\n\nIf feasible, the client should aggregate reporting data for up to 5\nseconds to reduce API traffic. Limiting aggregation to 5 seconds is to\nreduce data loss during client crashes. Clients should carefully choose\nthe aggregation time window to avoid data loss risk more than 0.01%\nfor business and compliance reasons.\n\nNOTE: the ReportRequest has the size limit of 1MB.\n\nThis method requires the `servicemanagement.services.report` permission\non the specified service. For more information, see\n[Google Cloud IAM](https://cloud.google.com/iam).");
            services0 = services0.subcommand(mcmd);
        }
        app = app.subcommand(services0);

        Self { app }
    }
}
use google_servicecontrol1 as api;

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
