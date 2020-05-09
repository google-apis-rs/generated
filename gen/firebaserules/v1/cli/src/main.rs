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
            .version("0.1.0-20200430")
            .about("Creates and manages rules that determine when a Firebase Rules-enabled service should permit a request.\n")
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
            let mcmd = SubCommand::with_name("test").about("Test `Source` for syntactic and semantic correctness. Issues present, if\nany, will be returned to the caller with a description, severity, and\nsource location.\n\nThe test method may be executed with `Source` or a `Ruleset` name.\nPassing `Source` is useful for unit testing new rules. Passing a `Ruleset`\nname is useful for regression testing an existing rule.\n\nThe following is an example of `Source` that permits users to upload images\nto a bucket bearing their user id and matching the correct metadata:\n\n_*Example*_\n\n    // Users are allowed to subscribe and unsubscribe to the blog.\n    service firebase.storage {\n      match /users/{userId}/images/{imageName} {\n          allow write: if userId == request.auth.uid\n              && (imageName.matches(\'*.png$\')\n              || imageName.matches(\'*.jpg$\'))\n              && resource.mimeType.matches(\'^image/\')\n      }\n    }");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut releases1 = SubCommand::with_name("releases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_executable, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a `Release`.\n\nRelease names should reflect the developer\'s deployment practices. For\nexample, the release name may include the environment name, application\nname, application version, or any other name meaningful to the developer.\nOnce a `Release` refers to a `Ruleset`, the rules can be enforced by\nFirebase Rules-enabled services.\n\nMore than one `Release` may be \'live\' concurrently. Consider the following\nthree `Release` names for `projects/foo` and the `Ruleset` to which they\nrefer.\n\nRelease Name                    | Ruleset Name\n--------------------------------|-------------\nprojects/foo/releases/prod      | projects/foo/rulesets/uuid123\nprojects/foo/releases/prod/beta | projects/foo/rulesets/uuid123\nprojects/foo/releases/prod/v23  | projects/foo/rulesets/uuid456\n\nThe table reflects the `Ruleset` rollout in progress. The `prod` and\n`prod/beta` releases refer to the same `Ruleset`. However, `prod/v23`\nrefers to a new `Ruleset`. The `Ruleset` reference for a `Release` may be\nupdated using the UpdateRelease method.");
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
            let mcmd = SubCommand::with_name("list").about("List the `Release` values for a project. This list may optionally be\nfiltered by `Release` name, `Ruleset` name, `TestSuite` name, or any\ncombination thereof.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a `Release` via PATCH.\n\nOnly updates to the `ruleset_name` and `test_suite_name` fields will be\nhonored. `Release` rename is not supported. To create a `Release` use the\nCreateRelease method.");
            releases1 = releases1.subcommand(mcmd);
        }
        let mut rulesets1 = SubCommand::with_name("rulesets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Create a `Ruleset` from `Source`.\n\nThe `Ruleset` is given a unique generated name which is returned to the\ncaller. `Source` containing syntactic or semantics errors will result in an\nerror response indicating the first error encountered. For a detailed view\nof `Source` issues, use TestRuleset.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a `Ruleset` by resource name.\n\nIf the `Ruleset` is referenced by a `Release` the operation will fail.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a `Ruleset` by name including the full `Source` contents.");
            rulesets1 = rulesets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List `Ruleset` metadata only and optionally filter the results by `Ruleset`\nname.\n\nThe full `Source` contents of a `Ruleset` may be retrieved with\nGetRuleset.");
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
