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
        let mut app = App::new("clouderrorreporting1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210225")
            .about("Groups and counts similar errors from cloud services and applications, reports new errors, and provides access to error groups and their associated errors. ")
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
            .about("methods: delete_events");
        {
            let mcmd = SubCommand::with_name("delete_events")
                .about("Deletes all error events of a given project.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut events1 = SubCommand::with_name("events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and report");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the specified events.");
            events1 = events1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report").about("Report an individual error event and record the event to a log. This endpoint accepts **either** an OAuth token, **or** an [API key](https://support.google.com/cloud/answer/6158862) for authentication. To use an API key, append it to the URL as the value of a `key` parameter. For example: `POST https://clouderrorreporting.googleapis.com/v1beta1/{projectName}/events:report?key=123ABC456` **Note:** [Error Reporting](/error-reporting) is a global service built on Cloud Logging and doesn\'t analyze logs stored in regional log buckets or logs routed to other Google Cloud projects. For more information, see [Using Error Reporting with regionalized logs](/error-reporting/docs/regionalization).");
            events1 = events1.subcommand(mcmd);
        }
        let mut group_stats1 = SubCommand::with_name("group_stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the specified groups.");
            group_stats1 = group_stats1.subcommand(mcmd);
        }
        let mut groups1 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and update");
        {
            let mcmd = SubCommand::with_name("get").about("Get the specified group.");
            groups1 = groups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Replace the data for the specified group. Fails if the group does not exist.",
            );
            groups1 = groups1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(groups1);
        projects0 = projects0.subcommand(group_stats1);
        projects0 = projects0.subcommand(events1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_clouderrorreporting1_beta1 as api;

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
