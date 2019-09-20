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
        let mut app = App::new("cloudprivatecatalog1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190913")
            .about("Enable cloud users to discover enterprise catalogs and products in their organizations.")
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
        let mut folders0 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: catalogs, products and versions");
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: catalogs, products and versions");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: catalogs, products and versions");
        let mut catalogs1 = SubCommand::with_name("catalogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Catalog resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            catalogs1 = catalogs1.subcommand(mcmd);
        }
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Product resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            products1 = products1.subcommand(mcmd);
        }
        let mut versions1 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Version resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            versions1 = versions1.subcommand(mcmd);
        }
        let mut catalogs1 = SubCommand::with_name("catalogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Catalog resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            catalogs1 = catalogs1.subcommand(mcmd);
        }
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Product resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            products1 = products1.subcommand(mcmd);
        }
        let mut versions1 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Version resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            versions1 = versions1.subcommand(mcmd);
        }
        let mut catalogs1 = SubCommand::with_name("catalogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Catalog resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            catalogs1 = catalogs1.subcommand(mcmd);
        }
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Product resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            products1 = products1.subcommand(mcmd);
        }
        let mut versions1 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Search Version resources that consumers have access to, within the\nscope of the consumer cloud resource hierarchy context.");
            versions1 = versions1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(versions1);
        projects0 = projects0.subcommand(products1);
        projects0 = projects0.subcommand(catalogs1);
        organizations0 = organizations0.subcommand(versions1);
        organizations0 = organizations0.subcommand(products1);
        organizations0 = organizations0.subcommand(catalogs1);
        folders0 = folders0.subcommand(versions1);
        folders0 = folders0.subcommand(products1);
        folders0 = folders0.subcommand(catalogs1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_cloudprivatecatalog1_beta1 as api;

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
