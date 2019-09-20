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
        let mut app = App::new("surveys2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20180508")
            .about("Creates and conducts surveys, lists the surveys that an authenticated user owns, and retrieves survey results and information about specified surveys.")
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
        let mut results0 = SubCommand::with_name("results")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add \"?alt=media\" to the URL as an argument to get results.");
            results0 = results0.subcommand(mcmd);
        }
        let mut surveys0 = SubCommand::with_name("surveys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, start, stop and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a survey from view in all user GET requests.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves information about the specified survey.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a survey.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the surveys owned by the authenticated user.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Begins running a survey.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops a running survey.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a survey. Currently the only property that can be updated is the owners property.");
            surveys0 = surveys0.subcommand(mcmd);
        }
        app = app.subcommand(surveys0);
        app = app.subcommand(results0);

        Self { app }
    }
}
use google_surveys2 as api;

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
