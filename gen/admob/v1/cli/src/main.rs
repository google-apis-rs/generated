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
        let mut app = App::new("admob1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200513")
            .about("The Google AdMob API lets you programmatically get reports on earnings.\n")
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
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets information about the specified AdMob publisher account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the AdMob publisher account accessible with the client credential.\nCurrently, all credentials have access to at most one AdMob account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut mediation_report1 = SubCommand::with_name("mediation_report")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about(
                "Generates an AdMob Mediation report based on the provided report\nspecification.",
            );
            mediation_report1 = mediation_report1.subcommand(mcmd);
        }
        let mut network_report1 = SubCommand::with_name("network_report")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about(
                "Generates an AdMob Network report based on the provided report\nspecification.",
            );
            network_report1 = network_report1.subcommand(mcmd);
        }
        accounts0 = accounts0.subcommand(network_report1);
        accounts0 = accounts0.subcommand(mediation_report1);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_admob1 as api;

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
