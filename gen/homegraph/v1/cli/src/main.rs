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
            .version("0.1.0-20210310")
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
            let mcmd = SubCommand::with_name("delete").about("Unlinks the given third-party user from your smart home Action. All data related to this user will be deleted. For more details on how users link their accounts, see [fulfillment and authentication](https://developers.google.com/assistant/smarthome/concepts/fulfillment-authentication). The third-party user\'s identity is passed in via the `agent_user_id` (see DeleteAgentUserRequest). This request must be authorized using service account credentials from your Actions console project.");
            agent_users0 = agent_users0.subcommand(mcmd);
        }
        let mut devices0 = SubCommand::with_name("devices")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: query, report_state_and_notification, request_link, request_sync and sync");
        {
            let mcmd = SubCommand::with_name("query").about("Gets the current states in Home Graph for the given set of the third-party user\'s devices. The third-party user\'s identity is passed in via the `agent_user_id` (see QueryRequest). This request must be authorized using service account credentials from your Actions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_state_and_notification").about("Reports device state and optionally sends device notifications. Called by your smart home Action when the state of a third-party device changes or you need to send a notification about the device. See [Implement Report State](https://developers.google.com/assistant/smarthome/develop/report-state) for more information. This method updates the device state according to its declared [traits](https://developers.google.com/assistant/smarthome/concepts/devices-traits). Publishing a new state value outside of these traits will result in an `INVALID_ARGUMENT` error response. The third-party user\'s identity is passed in via the `agent_user_id` (see ReportStateAndNotificationRequest). This request must be authorized using service account credentials from your Actions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("request_link").about("Sends an account linking suggestion to users associated with any potential Cast devices detected by third-party devices. This request must be authorized using service account credentials from your Actions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("request_sync").about("Requests Google to send an `action.devices.SYNC` [intent](https://developers.google.com/assistant/smarthome/reference/intent/sync) to your smart home Action to update device metadata for the given user. The third-party user\'s identity is passed via the `agent_user_id` (see RequestSyncDevicesRequest). This request must be authorized using service account credentials from your Actions console project.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sync").about("Gets all the devices associated with the given third-party user. The third-party user\'s identity is passed in via the `agent_user_id` (see SyncRequest). This request must be authorized using service account credentials from your Actions console project.");
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
