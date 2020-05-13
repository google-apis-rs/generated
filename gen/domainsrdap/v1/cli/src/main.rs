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
        let mut app = App::new("domainsrdap1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200509")
            .about("Read-only public API that lets users search for information about domain names.")
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
        let mut autnum0 = SubCommand::with_name("autnum")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            autnum0 = autnum0.subcommand(mcmd);
        }
        let mut domain0 = SubCommand::with_name("domain")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Look up RDAP information for a domain by name.");
            domain0 = domain0.subcommand(mcmd);
        }
        let mut entity0 = SubCommand::with_name("entity")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            entity0 = entity0.subcommand(mcmd);
        }
        let mut ip0 = SubCommand::with_name("ip")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            ip0 = ip0.subcommand(mcmd);
        }
        let mut nameserver0 = SubCommand::with_name("nameserver")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            nameserver0 = nameserver0.subcommand(mcmd);
        }
        let mut v_10 = SubCommand::with_name("v_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_domains, get_entities, get_help, get_ip and get_nameservers");
        {
            let mcmd = SubCommand::with_name("get_domains").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_entities").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_help")
                .about("Get help information for the RDAP API, including links to documentation.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ip").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_nameservers").about("The RDAP API recognizes this command from the RDAP specification but\ndoes not support it. The response is a formatted 501 error.");
            v_10 = v_10.subcommand(mcmd);
        }
        app = app.subcommand(v_10);
        app = app.subcommand(nameserver0);
        app = app.subcommand(ip0);
        app = app.subcommand(entity0);
        app = app.subcommand(domain0);
        app = app.subcommand(autnum0);

        Self { app }
    }
}
use google_domainsrdap1 as api;

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
