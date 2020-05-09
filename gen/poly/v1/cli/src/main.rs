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
        let mut app = App::new("poly1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200424")
            .about("The Poly API provides read access to assets hosted on <a href=\"https://poly.google.com\">poly.google.com</a> to all, and upload access to <a href=\"https://poly.google.com\">poly.google.com</a> for whitelisted accounts.\n")
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
        let mut assets0 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns detailed information about an asset given its name.\nPRIVATE assets are returned only if\n the currently authenticated user (via OAuth token) is the author of the\n asset.");
            assets0 = assets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all public, remixable assets. These are assets with an access level\nof PUBLIC and published under the\nCC-By license.");
            assets0 = assets0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assets and likedassets");
        let mut assets1 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists assets authored by the given user. Only the value \'me\', representing\nthe currently-authenticated user, is supported. May include assets with an\naccess level of PRIVATE or\nUNLISTED and assets which are\nAll Rights Reserved for the\ncurrently-authenticated user.");
            assets1 = assets1.subcommand(mcmd);
        }
        let mut likedassets1 = SubCommand::with_name("likedassets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists assets that the user has liked. Only the value \'me\', representing\nthe currently-authenticated user, is supported. May include assets with an\naccess level of UNLISTED.");
            likedassets1 = likedassets1.subcommand(mcmd);
        }
        users0 = users0.subcommand(likedassets1);
        users0 = users0.subcommand(assets1);
        app = app.subcommand(users0);
        app = app.subcommand(assets0);

        Self { app }
    }
}
use google_poly1 as api;

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
