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
            .version("0.1.0-20190918")
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
            let mcmd = SubCommand::with_name("delete").about("Unlinks an agent user from Google. As a result, all data related to this\nuser will be deleted.\n\nHere is how the agent user is created in Google:\n\n1.  When a user opens their Google Home App, they can begin linking a 3p\n    partner.\n2.  User is guided through the OAuth process.\n3.  After entering the 3p credentials, Google gets the 3p OAuth token and\n    uses it to make a Sync call to the 3p partner and gets back all of the\n    user\'s data, including `agent_user_id` and devices.\n4.  Google creates the agent user and stores a mapping from the\n    `agent_user_id` -> Google ID mapping. Google also\n    stores all of the user\'s devices under that Google ID.\n\nThe mapping from `agent_user_id` to Google ID is many to many, since one\nGoogle user can have multiple 3p accounts, and multiple Google users can\nmap to one `agent_user_id` (e.g., a husband and wife share one Nest account\nusername/password).\n\nThe third-party user\'s identity is passed in as `agent_user_id`.\nThe agent is identified by the JWT signed by the partner\'s service account.\n\nNote: Special characters (except \"/\") in `agent_user_id` must be\nURL-encoded.");
            agent_users0 = agent_users0.subcommand(mcmd);
        }
        let mut devices0 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query, report_state_and_notification, request_sync and sync");
        {
            let mcmd = SubCommand::with_name("query").about("Gets the device states for the devices in QueryRequest.\nThe third-party user\'s identity is passed in as `agent_user_id`. The agent\nis identified by the JWT signed by the third-party partner\'s service\naccount.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_state_and_notification").about("Reports device state and optionally sends device notifications. Called by\nan agent when the device state of a third-party changes or the agent wants\nto send a notification about the device. See\n[Implement Report State](/actions/smarthome/report-state) for more\ninformation.\nThis method updates a predefined set of states for a device, which all\ndevices have according to their prescribed traits (for example, a light\nwill have the [OnOff](/actions/smarthome/traits/onoff) trait that reports\nthe state `on` as a boolean value).\nA new state may not be created and an INVALID_ARGUMENT code will be thrown\nif so. It also optionally takes in a list of Notifications that may be\ncreated, which are associated to this state change.\n\nThe third-party user\'s identity is passed in as `agent_user_id`.\nThe agent is identified by the JWT signed by the partner\'s service account.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("request_sync").about("Requests a `SYNC` call from Google to a 3p partner\'s home control agent for\na user.\n\n\nThe third-party user\'s identity is passed in as `agent_user_id`\n(see RequestSyncDevicesRequest) and forwarded back to the agent.\nThe agent is identified by the API key or JWT signed by the partner\'s\nservice account.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sync").about("Gets all the devices associated with the given third-party user.\nThe third-party user\'s identity is passed in as `agent_user_id`. The agent\nis identified by the JWT signed by the third-party partner\'s service\naccount.");
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
