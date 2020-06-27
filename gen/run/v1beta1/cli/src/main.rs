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
        let mut app = App::new("run1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200622")
            .about("Deploy and manage user provided container images that scale automatically based on HTTP traffic.")
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
        let mut customresourcedefinitions0 = SubCommand::with_name("customresourcedefinitions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Rpc to list custom resource definitions.");
            customresourcedefinitions0 = customresourcedefinitions0.subcommand(mcmd);
        }
        let mut namespaces0 = SubCommand::with_name("namespaces")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: customresourcedefinitions");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut customresourcedefinitions1 = SubCommand::with_name("customresourcedefinitions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Rpc to get information about a CustomResourceDefinition.");
            customresourcedefinitions1 = customresourcedefinitions1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: customresourcedefinitions");
        let mut customresourcedefinitions2 = SubCommand::with_name("customresourcedefinitions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Rpc to get information about a CustomResourceDefinition.");
            customresourcedefinitions2 = customresourcedefinitions2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Rpc to list custom resource definitions.");
            customresourcedefinitions2 = customresourcedefinitions2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(customresourcedefinitions2);
        projects0 = projects0.subcommand(locations1);
        namespaces0 = namespaces0.subcommand(customresourcedefinitions1);
        app = app.subcommand(projects0);
        app = app.subcommand(namespaces0);
        app = app.subcommand(customresourcedefinitions0);

        Self { app }
    }
}
use google_run1_beta1 as api;

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
