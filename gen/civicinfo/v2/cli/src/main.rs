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
        let mut app = App::new("civicinfo2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190402")
            .about("Provides polling places, early vote locations, contest data, election officials, and government representatives for U.S. residential addresses.")
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
        let mut divisions0 = SubCommand::with_name("divisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search")
                .about("Searches for political divisions by their natural name or OCD ID.");
            divisions0 = divisions0.subcommand(mcmd);
        }
        let mut elections0 = SubCommand::with_name("elections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: election_query and voter_info_query");
        {
            let mcmd = SubCommand::with_name("election_query")
                .about("List of available elections to query.");
            elections0 = elections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("voter_info_query").about("Looks up information relevant to a voter based on the voter\'s registered address.");
            elections0 = elections0.subcommand(mcmd);
        }
        let mut representatives0 = SubCommand::with_name("representatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: representative_info_by_address and representative_info_by_division");
        {
            let mcmd = SubCommand::with_name("representative_info_by_address").about(
                "Looks up political geography and representative information for a single address.",
            );
            representatives0 = representatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("representative_info_by_division")
                .about("Looks up representative information for a single geographic division.");
            representatives0 = representatives0.subcommand(mcmd);
        }
        app = app.subcommand(representatives0);
        app = app.subcommand(elections0);
        app = app.subcommand(divisions0);

        Self { app }
    }
}
use google_civicinfo2 as api;

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
