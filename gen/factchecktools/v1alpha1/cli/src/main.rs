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
        let mut app = App::new("factchecktools1_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
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
        let mut claims0 = SubCommand::with_name("claims")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search through fact-checked claims.");
            claims0 = claims0.subcommand(mcmd);
        }
        let mut pages0 = SubCommand::with_name("pages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd =
                SubCommand::with_name("create").about("Create `ClaimReview` markup on a page.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete all `ClaimReview` markup on a page.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get all `ClaimReview` markup on a page.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List the `ClaimReview` markup pages for a specific URL or for an\norganization.",
            );
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update for all `ClaimReview` markup on a page\n\nNote that this is a full update. To retain the existing `ClaimReview`\nmarkup on a page, first perform a Get operation, then modify the returned\nmarkup, and finally call Update with the entire `ClaimReview` markup as the\nbody.");
            pages0 = pages0.subcommand(mcmd);
        }
        app = app.subcommand(pages0);
        app = app.subcommand(claims0);

        Self { app }
    }
}
use google_factchecktools1_alpha1 as api;

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
