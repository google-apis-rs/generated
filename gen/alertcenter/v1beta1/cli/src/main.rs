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
        let mut app = App::new("alertcenter1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210309")
            .about("Manages alerts on issues affecting your domain.")
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
        let mut alerts0 = SubCommand::with_name("alerts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_delete, batch_undelete, delete, get, get_metadata, list and undelete");
        {
            let mcmd = SubCommand::with_name("batch_delete")
                .about("Performs batch delete operation on alerts.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_undelete")
                .about("Performs batch undelete operation on alerts.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Marks the specified alert for deletion. An alert that has been marked for deletion is removed from Alert Center after 30 days. Marking an alert for deletion has no effect on an alert which has already been marked for deletion. Attempting to mark a nonexistent alert for deletion results in a `NOT_FOUND` error.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified alert. Attempting to get a nonexistent alert returns `NOT_FOUND` error.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_metadata").about("Returns the metadata of an alert. Attempting to get metadata for a non-existent alert returns `NOT_FOUND` error.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the alerts.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Restores, or \"undeletes\", an alert that was marked for deletion within the past 30 days. Attempting to undelete an alert which was marked for deletion over 30 days ago (which has been removed from the Alert Center database) or a nonexistent alert returns a `NOT_FOUND` error. Attempting to undelete an alert which has not been marked for deletion has no effect.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        let mut v_1beta_10 = SubCommand::with_name("v_1beta_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_settings and update_settings");
        {
            let mcmd =
                SubCommand::with_name("get_settings").about("Returns customer-level settings.");
            v_1beta_10 = v_1beta_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_settings")
                .about("Updates the customer-level settings.");
            v_1beta_10 = v_1beta_10.subcommand(mcmd);
        }
        let mut feedback1 = SubCommand::with_name("feedback")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates new feedback for an alert. Attempting to create a feedback for a non-existent alert returns `NOT_FOUND` error. Attempting to create a feedback for an alert that is marked for deletion returns `FAILED_PRECONDITION\' error.");
            feedback1 = feedback1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the feedback for an alert. Attempting to list feedbacks for a non-existent alert returns `NOT_FOUND` error.");
            feedback1 = feedback1.subcommand(mcmd);
        }
        alerts0 = alerts0.subcommand(feedback1);
        app = app.subcommand(v_1beta_10);
        app = app.subcommand(alerts0);

        Self { app }
    }
}
use google_alertcenter1_beta1 as api;

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
