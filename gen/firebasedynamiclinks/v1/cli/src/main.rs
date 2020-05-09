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
        let mut app = App::new("firebasedynamiclinks1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200507")
            .about("Programmatically creates and manages Firebase Dynamic Links.")
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
        let mut managed_short_links0 = SubCommand::with_name("managed_short_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a managed short Dynamic Link given either a valid long Dynamic Link\nor details such as Dynamic Link domain, Android and iOS app information.\nThe created short Dynamic Link will not expire.\n\nThis differs from CreateShortDynamicLink in the following ways:\n  - The request will also contain a name for the link (non unique name\n    for the front end).\n  - The response must be authenticated with an auth token (generated with\n    the admin service account).\n  - The link will appear in the FDL list of links in the console front end.\n\nThe Dynamic Link domain in the request must be owned by requester\'s\nFirebase project.");
            managed_short_links0 = managed_short_links0.subcommand(mcmd);
        }
        let mut short_links0 = SubCommand::with_name("short_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a short Dynamic Link given either a valid long Dynamic Link or\ndetails such as Dynamic Link domain, Android and iOS app information.\nThe created short Dynamic Link will not expire.\n\nRepeated calls with the same long Dynamic Link or Dynamic Link information\nwill produce the same short Dynamic Link.\n\nThe Dynamic Link domain in the request must be owned by requester\'s\nFirebase project.");
            short_links0 = short_links0.subcommand(mcmd);
        }
        let mut v_10 = SubCommand::with_name("v_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_link_stats, install_attribution and reopen_attribution");
        {
            let mcmd = SubCommand::with_name("get_link_stats").about("Fetches analytics stats of a short Dynamic Link for a given\nduration. Metrics include number of clicks, redirects, installs,\napp first opens, and app reopens.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("install_attribution")
                .about("Get iOS strong/weak-match info for post-install attribution.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reopen_attribution")
                .about("Get iOS reopen attribution for app universal link open deeplinking.");
            v_10 = v_10.subcommand(mcmd);
        }
        app = app.subcommand(v_10);
        app = app.subcommand(short_links0);
        app = app.subcommand(managed_short_links0);

        Self { app }
    }
}
use google_firebasedynamiclinks1 as api;

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
