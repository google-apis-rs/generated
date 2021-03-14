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
        let mut app = App::new("cloudprofiler2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210308")
            .about("Manages continuous profiling information.")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: profiles");
        let mut profiles1 = SubCommand::with_name("profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, create_offline and patch");
        {
            let mcmd = SubCommand::with_name("create").about("CreateProfile creates a new profile resource in the online mode. The server ensures that the new profiles are created at a constant rate per deployment, so the creation request may hang for some time until the next profile session is available. The request may fail with ABORTED error if the creation is not available within ~1m, the response will indicate the duration of the backoff the client should take before attempting creating a profile again. The backoff duration is returned in google.rpc.RetryInfo extension on the response status. To a gRPC client, the extension will be return as a binary-serialized proto in the trailing metadata item named \"google.rpc.retryinfo-bin\".");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_offline").about("CreateOfflineProfile creates a new profile resource in the offline mode. The client provides the profile to create along with the profile bytes, the server records it.");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("UpdateProfile updates the profile bytes and labels on the profile resource created in the online mode. Updating the bytes for profiles created in the offline mode is currently not supported: the profile content must be provided at the time of the profile creation.");
            profiles1 = profiles1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(profiles1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_cloudprofiler2 as api;

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
