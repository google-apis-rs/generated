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
        let mut app = App::new("firebaserules1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210311")
            .about("Creates and manages rules that determine when a Firebase Rules-enabled service should permit a request. ")
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
            .about("methods: test");
        {
            let mcmd = SubCommand::with_name("test").about("Test `Source` for syntactic and semantic correctness. Issues present, if any, will be returned to the caller with a description, severity, and source location. The test method may be executed with `Source` or a `Ruleset` name. Passing `Source` is useful for unit testing new rules. Passing a `Ruleset` name is useful for regression testing an existing rule. The following is an example of `Source` that permits users to upload images to a bucket bearing their user id and matching the correct metadata: _*Example*_ // Users are allowed to subscribe and unsubscribe to the blog. service firebase.storage { match /users/{userId}/images/{imageName} { allow write: if userId == request.auth.uid && (imageName.matches(\'*.png$\') || imageName.matches(\'*.jpg$\')) && resource.mimeType.matches(\'^image/\') } }");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut releases1 = SubCommand::with_name("releases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_executable, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a `Release`. Release names should reflect the developer\'s deployment practices. For example, the release name may include the environment name, application name, application version, or any other name meaningful to the developer. Once a `Release` refers to a `Ruleset`, the rules can be enforced by Firebase Rules-enabled services. More than one `Release` may be \'live\' concurrently. Consider the following three `Release` names for `projects/foo` and the `Ruleset` to which they refer. Release Name | Ruleset Name --------------------------------|------------- projects/foo/releases/prod | projects/foo/rulesets/uuid123 projects/foo/releases/prod/beta | projects/foo/rulesets/uuid123 projects/foo/releases/prod/v23 | projects/foo/rulesets/uuid456 The table reflects the `Ruleset` rollout in progress. The `prod` and `prod/beta` releases refer to the same `Ruleset`. However, `prod/v23` refers to a new `Ruleset`. The `Ruleset` reference for a `Release` may be updated using the UpdateRelease method.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete a `Release` by resource name.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a `Release` by name.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_executable")
                .about("Get the `Release` executable to use when enforcing rules.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the `Release` values for a project. This list may optionally be filtered by `Release` name, `Ruleset` name, `TestSuite` name, or any combination thereof.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a `Release` via PATCH. Only updates to the `ruleset_name` and `test_suite_name` fields will be honored. `Release` rename is not supported. To create a `Release` use the CreateRelease method.");
            releases1 = releases1.subcommand(mcmd);
        }
        let mut rulesets1 = SubCommand::with_name("rulesets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Create a `Ruleset` from `Source`. The `Ruleset` is given a unique generated name which is returned to the caller. `Source` containing syntactic or semantics errors will result in an error response indicating the first error encountered. For a detailed view of `Source` issues, use TestRuleset.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a `Ruleset` by resource name. If the `Ruleset` is referenced by a `Release` the operation will fail.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a `Ruleset` by name including the full `Source` contents.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List `Ruleset` metadata only and optionally filter the results by `Ruleset` name. The full `Source` contents of a `Ruleset` may be retrieved with GetRuleset.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(rulesets1);
        projects0 = projects0.subcommand(releases1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_firebaserules1 as api;

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
