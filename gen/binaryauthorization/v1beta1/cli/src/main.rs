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
        let mut app = App::new("binaryauthorization1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190916")
            .about("The management interface for Binary Authorization, a system providing policy control for images deployed to Kubernetes Engine clusters.\n")
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
            .about("methods: get_policy and update_policy");
        {
            let mcmd = SubCommand::with_name("get_policy").about("A policy specifies the attestors that must attest to\na container image, before the project is allowed to deploy that\nimage. There is at most one policy per project. All image admission\nrequests are permitted if a project has no policy.\n\nGets the policy for this project. Returns a default\npolicy if the project does not have one.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_policy").about("Creates or updates a project\'s policy, and returns a copy of the\nnew policy. A policy is always updated as a whole, to avoid race\nconditions with concurrent policy enforcement (or management!)\nrequests. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT\nif the request is malformed.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut attestors1 = SubCommand::with_name("attestors")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an attestor, and returns a copy of the new\nattestor. Returns NOT_FOUND if the project does not exist,\nINVALID_ARGUMENT if the request is malformed, ALREADY_EXISTS if the\nattestor already exists.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an attestor. Returns NOT_FOUND if the\nattestor does not exist.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an attestor.\nReturns NOT_FOUND if the attestor does not exist.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists attestors.\nReturns INVALID_ARGUMENT if the project does not exist.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an attestor.\nReturns NOT_FOUND if the attestor does not exist.");
            attestors1 = attestors1.subcommand(mcmd);
        }
        let mut policy1 = SubCommand::with_name("policy")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            policy1 = policy1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.");
            policy1 = policy1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            policy1 = policy1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(policy1);
        projects0 = projects0.subcommand(attestors1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_binaryauthorization1_beta1 as api;

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
