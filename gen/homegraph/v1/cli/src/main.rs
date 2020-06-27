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
        let mut app = App::new("homegraph1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200619")
            .about("")
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
        let mut agent_users0 = SubCommand::with_name("agent_users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete");
        {
            let mcmd = SubCommand::with_name("delete").about("Unlinks the given third-party user from your smart home Action.\nAll data related to this user will be deleted.\n\nFor more details on how users link their accounts, see\n[fulfillment and\nauthentication](https://developers.google.com/assistant/smarthome/concepts/fulfillment-authentication).\n\nThe third-party user\'s identity is passed in via the `agent_user_id`\n(see DeleteAgentUserRequest).\nThis request must be authorized using service account credentials from your\nActions console project.");
            agent_users0 = agent_users0.subcommand(mcmd);
        }
        let mut devices0 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query, report_state_and_notification, request_sync and sync");
        {
            let mcmd = SubCommand::with_name("query").about("Gets the current states in Home Graph for the given set of the third-party\nuser\'s devices.\n\nThe third-party user\'s identity is passed in via the `agent_user_id`\n(see QueryRequest).\nThis request must be authorized using service account credentials from your\nActions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_state_and_notification").about("Reports device state and optionally sends device notifications.\nCalled by your smart home Action when the state of a third-party device\nchanges or you need to send a notification about the device.\nSee [Implement Report\nState](https://developers.google.com/assistant/smarthome/develop/report-state)\nfor more information.\n\nThis method updates the device state according to its declared\n[traits](https://developers.google.com/assistant/smarthome/concepts/devices-traits).\nPublishing a new state value outside of these traits will result in an\n`INVALID_ARGUMENT` error response.\n\nThe third-party user\'s identity is passed in via the `agent_user_id`\n(see ReportStateAndNotificationRequest).\nThis request must be authorized using service account credentials from your\nActions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("request_sync").about("Requests Google to send an `action.devices.SYNC`\n[intent](https://developers.google.com/assistant/smarthome/reference/intent/sync)\nto your smart home Action to update device metadata for the given user.\n\n\nThe third-party user\'s identity is passed via the `agent_user_id`\n(see RequestSyncDevicesRequest).\nThis request must be authorized using service account credentials from your\nActions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sync").about("Gets all the devices associated with the given third-party user.\n\nThe third-party user\'s identity is passed in via the `agent_user_id`\n(see SyncRequest).\nThis request must be authorized using service account credentials from your\nActions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        app = app.subcommand(devices0);
        app = app.subcommand(agent_users0);

        Self { app }
    }
}
use google_homegraph1 as api;

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
