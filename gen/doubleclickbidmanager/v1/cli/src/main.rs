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
        let mut app = App::new("doubleclickbidmanager1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200316")
            .about("API for viewing and managing your reports in DoubleClick Bid Manager.")
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
        let mut lineitems0 = SubCommand::with_name("lineitems")
            .setting(AppSettings::ColoredHelp)
            .about("methods: downloadlineitems and uploadlineitems");
        {
            let mcmd = SubCommand::with_name("downloadlineitems").about(
                "Retrieves line items in CSV format. TrueView line items are not supported.",
            );
            lineitems0 = lineitems0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("uploadlineitems")
                .about("Uploads line items in CSV format. TrueView line items are not supported.");
            lineitems0 = lineitems0.subcommand(mcmd);
        }
        let mut queries0 = SubCommand::with_name("queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: createquery, deletequery, getquery, listqueries and runquery");
        {
            let mcmd = SubCommand::with_name("createquery").about("Creates a query.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deletequery")
                .about("Deletes a stored query as well as the associated stored reports.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("getquery").about("Retrieves a stored query.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("listqueries").about("Retrieves stored queries.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("runquery")
                .about("Runs a stored query to generate a report.");
            queries0 = queries0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: listreports");
        {
            let mcmd = SubCommand::with_name("listreports").about("Retrieves stored reports.");
            reports0 = reports0.subcommand(mcmd);
        }
        let mut sdf0 = SubCommand::with_name("sdf")
            .setting(AppSettings::ColoredHelp)
            .about("methods: download");
        {
            let mcmd = SubCommand::with_name("download").about("Retrieves entities in SDF format.");
            sdf0 = sdf0.subcommand(mcmd);
        }
        app = app.subcommand(sdf0);
        app = app.subcommand(reports0);
        app = app.subcommand(queries0);
        app = app.subcommand(lineitems0);

        Self { app }
    }
}
use google_doubleclickbidmanager1 as api;

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
