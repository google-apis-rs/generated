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
        let mut app = App::new("siteVerification1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20191119")
            .about("Verifies ownership of websites or domains with Google.")
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
        let mut web_resource0 = SubCommand::with_name("web_resource")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, get_token, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Relinquish ownership of a website or domain.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get the most current data for a website or domain.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_token")
                .about("Get a verification token for placing on a website or domain.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Attempt verification of a website or domain.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Get the list of your verified websites and domains.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Modify the list of owners for your website or domain. This method supports patch semantics.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Modify the list of owners for your website or domain.");
            web_resource0 = web_resource0.subcommand(mcmd);
        }
        app = app.subcommand(web_resource0);

        Self { app }
    }
}
use google_siteVerification1 as api;

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
