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
        let mut app = App::new("safebrowsing4")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200502")
            .about("Enables client applications to check web resources (most commonly URLs) against Google-generated lists of unsafe web resources. The Safe Browsing APIs are for non-commercial use only. If you need to use APIs to detect malicious URLs for commercial purposes – meaning “for sale or revenue-generating purposes” – please refer to the Web Risk API.")
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
        let mut encoded_full_hashes0 = SubCommand::with_name("encoded_full_hashes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("");
            encoded_full_hashes0 = encoded_full_hashes0.subcommand(mcmd);
        }
        let mut encoded_updates0 = SubCommand::with_name("encoded_updates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("");
            encoded_updates0 = encoded_updates0.subcommand(mcmd);
        }
        let mut full_hashes0 = SubCommand::with_name("full_hashes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: find");
        {
            let mcmd = SubCommand::with_name("find")
                .about("Finds the full hashes that match the requested hash prefixes.");
            full_hashes0 = full_hashes0.subcommand(mcmd);
        }
        let mut threat_hits0 = SubCommand::with_name("threat_hits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Reports a Safe Browsing threat list hit to Google. Only projects with\nTRUSTED_REPORTER visibility can use this method.");
            threat_hits0 = threat_hits0.subcommand(mcmd);
        }
        let mut threat_list_updates0 = SubCommand::with_name("threat_list_updates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: fetch");
        {
            let mcmd = SubCommand::with_name("fetch").about("Fetches the most recent threat list updates. A client can request updates\nfor multiple lists at once.");
            threat_list_updates0 = threat_list_updates0.subcommand(mcmd);
        }
        let mut threat_lists0 = SubCommand::with_name("threat_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the Safe Browsing threat lists available for download.");
            threat_lists0 = threat_lists0.subcommand(mcmd);
        }
        let mut threat_matches0 = SubCommand::with_name("threat_matches")
            .setting(AppSettings::ColoredHelp)
            .about("methods: find");
        {
            let mcmd = SubCommand::with_name("find")
                .about("Finds the threat entries that match the Safe Browsing lists.");
            threat_matches0 = threat_matches0.subcommand(mcmd);
        }
        app = app.subcommand(threat_matches0);
        app = app.subcommand(threat_lists0);
        app = app.subcommand(threat_list_updates0);
        app = app.subcommand(threat_hits0);
        app = app.subcommand(full_hashes0);
        app = app.subcommand(encoded_updates0);
        app = app.subcommand(encoded_full_hashes0);

        Self { app }
    }
}
use google_safebrowsing4 as api;

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
