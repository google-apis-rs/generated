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
        let mut app = App::new("testing1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210308")
            .about("Allows developers to run automated tests for their mobile applications on Google infrastructure.")
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
        let mut application_detail_service0 = SubCommand::with_name("application_detail_service")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_apk_details");
        {
            let mcmd = SubCommand::with_name("get_apk_details")
                .about("Gets the details of an Android application APK.");
            application_detail_service0 = application_detail_service0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: test_matrices");
        let mut test_environment_catalog0 = SubCommand::with_name("test_environment_catalog")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred");
            test_environment_catalog0 = test_environment_catalog0.subcommand(mcmd);
        }
        let mut test_matrices1 = SubCommand::with_name("test_matrices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist");
            test_matrices1 = test_matrices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.");
            test_matrices1 = test_matrices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Checks the status of a test matrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist");
            test_matrices1 = test_matrices1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(test_matrices1);
        app = app.subcommand(test_environment_catalog0);
        app = app.subcommand(projects0);
        app = app.subcommand(application_detail_service0);

        Self { app }
    }
}
use google_testing1 as api;

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
