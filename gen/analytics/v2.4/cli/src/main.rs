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
        let mut app = App::new("analytics2d4")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190917")
            .about("Views and manages your Google Analytics data.")
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
        let mut data0 = SubCommand::with_name("data")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns Analytics report data for a view (profile).");
            data0 = data0.subcommand(mcmd);
        }
        let mut management0 = SubCommand::with_name("management")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: accounts, goals, profiles, segments and webproperties");
        let mut accounts1 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all accounts to which the user has access.");
            accounts1 = accounts1.subcommand(mcmd);
        }
        let mut goals1 = SubCommand::with_name("goals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists goals to which the user has access.");
            goals1 = goals1.subcommand(mcmd);
        }
        let mut profiles1 = SubCommand::with_name("profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists views (profiles) to which the user has access.");
            profiles1 = profiles1.subcommand(mcmd);
        }
        let mut segments1 = SubCommand::with_name("segments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists advanced segments to which the user has access.");
            segments1 = segments1.subcommand(mcmd);
        }
        let mut webproperties1 = SubCommand::with_name("webproperties")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists web properties to which the user has access.");
            webproperties1 = webproperties1.subcommand(mcmd);
        }
        management0 = management0.subcommand(webproperties1);
        management0 = management0.subcommand(segments1);
        management0 = management0.subcommand(profiles1);
        management0 = management0.subcommand(goals1);
        management0 = management0.subcommand(accounts1);
        app = app.subcommand(management0);
        app = app.subcommand(data0);

        Self { app }
    }
}
use google_analytics2d4 as api;

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
