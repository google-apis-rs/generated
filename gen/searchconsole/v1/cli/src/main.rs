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
        let mut app = App::new("searchconsole1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
            .about("Provides tools for running validation tests against single URLs")
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
        let mut url_testing_tools0 = SubCommand::with_name("url_testing_tools")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: mobile_friendly_test");
        let mut mobile_friendly_test1 = SubCommand::with_name("mobile_friendly_test")
            .setting(AppSettings::ColoredHelp)
            .about("methods: run");
        {
            let mcmd =
                SubCommand::with_name("run").about("Runs Mobile-Friendly Test for a given URL.");
            mobile_friendly_test1 = mobile_friendly_test1.subcommand(mcmd);
        }
        url_testing_tools0 = url_testing_tools0.subcommand(mobile_friendly_test1);
        app = app.subcommand(url_testing_tools0);

        Self { app }
    }
}
use google_searchconsole1 as api;

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
