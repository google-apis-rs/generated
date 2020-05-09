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
        let mut app = App::new("cloudshell1_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200507")
            .about("Allows users to start, configure, and connect to interactive shell sessions running in the cloud.\n")
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
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: environments");
        let mut environments1 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: authorize, get, patch and start");
        {
            let mcmd = SubCommand::with_name("authorize").about("Sends OAuth credentials to a running environment on behalf of a user. When\nthis completes, the environment will be authorized to run various Google\nCloud command line tools without requiring the user to manually\nauthenticate.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an environment. Returns NOT_FOUND if the environment does not exist.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing environment.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Starts an existing environment, allowing clients to connect to it. The\nreturned operation will contain an instance of StartEnvironmentMetadata in\nits metadata field. Users can wait for the environment to start by polling\nthis operation via GetOperation. Once the environment has finished starting\nand is ready to accept connections, the operation will contain a\nStartEnvironmentResponse in its response field.");
            environments1 = environments1.subcommand(mcmd);
        }
        let mut public_keys2 = SubCommand::with_name("public_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and delete");
        {
            let mcmd = SubCommand::with_name("create").about("Adds a public SSH key to an environment, allowing clients with the\ncorresponding private key to connect to that environment via SSH. If a key\nwith the same format and content already exists, this will return the\nexisting key.");
            public_keys2 = public_keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a public SSH key from an environment. Clients will no longer be\nable to connect to the environment using the corresponding private key.");
            public_keys2 = public_keys2.subcommand(mcmd);
        }
        environments1 = environments1.subcommand(public_keys2);
        users0 = users0.subcommand(environments1);
        app = app.subcommand(users0);

        Self { app }
    }
}
use google_cloudshell1_alpha1 as api;

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
