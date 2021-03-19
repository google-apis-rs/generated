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
        let mut app = App::new("smartdevicemanagement1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210311")
            .about("Allow select enterprise partners to access, control, and manage Google and Nest devices programmatically.")
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
        let mut enterprises0 = SubCommand::with_name("enterprises")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: devices and structures");
        let mut devices1 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: execute_command, get and list");
        {
            let mcmd = SubCommand::with_name("execute_command")
                .about("Executes a command to device managed by the enterprise.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a device managed by the enterprise.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices managed by the enterprise.");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut structures1 = SubCommand::with_name("structures")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a structure managed by the enterprise.");
            structures1 = structures1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists structures managed by the enterprise.");
            structures1 = structures1.subcommand(mcmd);
        }
        let mut rooms2 = SubCommand::with_name("rooms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a room managed by the enterprise.");
            rooms2 = rooms2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists rooms managed by the enterprise.");
            rooms2 = rooms2.subcommand(mcmd);
        }
        structures1 = structures1.subcommand(rooms2);
        enterprises0 = enterprises0.subcommand(structures1);
        enterprises0 = enterprises0.subcommand(devices1);
        app = app.subcommand(enterprises0);

        Self { app }
    }
}
use google_smartdevicemanagement1 as api;

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
