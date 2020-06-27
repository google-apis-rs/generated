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
        let mut app = App::new("oslogin1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200620")
            .about("You can use OS Login to manage access to your VM instances using IAM roles.")
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
            .about("methods: get_login_profile and import_ssh_public_key");
        {
            let mcmd = SubCommand::with_name("get_login_profile").about("Retrieves the profile information used for logging in to a virtual machine\non Google Compute Engine.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_ssh_public_key").about("Adds an SSH public key and returns the profile information. Default POSIX\naccount information is set when no username and UID exist as part of the\nlogin profile.");
            users0 = users0.subcommand(mcmd);
        }
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a POSIX account.");
            projects1 = projects1.subcommand(mcmd);
        }
        let mut ssh_public_keys1 = SubCommand::with_name("ssh_public_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an SSH public key.");
            ssh_public_keys1 = ssh_public_keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an SSH public key.");
            ssh_public_keys1 = ssh_public_keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an SSH public key and returns the profile information. This method\nsupports patch semantics.");
            ssh_public_keys1 = ssh_public_keys1.subcommand(mcmd);
        }
        users0 = users0.subcommand(ssh_public_keys1);
        users0 = users0.subcommand(projects1);
        app = app.subcommand(users0);

        Self { app }
    }
}
use google_oslogin1_beta as api;

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
