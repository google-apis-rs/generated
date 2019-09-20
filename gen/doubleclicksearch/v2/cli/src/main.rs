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
        let mut app = App::new("doubleclicksearch2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190917")
            .about("Reports and modifies your advertising data in DoubleClick Search (for example, campaigns, ad groups, keywords, and conversions).")
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
        let mut conversion0 = SubCommand::with_name("conversion")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, patch, update and update_availability");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a list of conversions from a DoubleClick Search engine account.");
            conversion0 = conversion0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a batch of new conversions into DoubleClick Search.");
            conversion0 = conversion0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a batch of conversions in DoubleClick Search. This method supports patch semantics.");
            conversion0 = conversion0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a batch of conversions in DoubleClick Search.");
            conversion0 = conversion0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_availability").about("Updates the availabilities of a batch of floodlight activities in DoubleClick Search.");
            conversion0 = conversion0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate, get, get_file and request");
        {
            let mcmd = SubCommand::with_name("generate")
                .about("Generates and returns a report immediately.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Polls for the status of a report request.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_file")
                .about("Downloads a report file encoded in UTF-8.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("request")
                .about("Inserts a report request into the reporting system.");
            reports0 = reports0.subcommand(mcmd);
        }
        let mut saved_columns0 = SubCommand::with_name("saved_columns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve the list of saved columns for a specified advertiser.");
            saved_columns0 = saved_columns0.subcommand(mcmd);
        }
        app = app.subcommand(saved_columns0);
        app = app.subcommand(reports0);
        app = app.subcommand(conversion0);

        Self { app }
    }
}
use google_doubleclicksearch2 as api;

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
