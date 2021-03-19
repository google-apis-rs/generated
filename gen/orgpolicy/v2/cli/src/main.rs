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
        let mut app = App::new("orgpolicy2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210317")
            .about("The Org Policy API allows users to configure governance ruleson their GCP resources across the Cloud Resource Hierarchy.")
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
        let mut folders0 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: constraints and policies");
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: constraints and policies");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: constraints and policies");
        let mut constraints1 = SubCommand::with_name("constraints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists `Constraints` that could be applied on the specified resource.");
            constraints1 = constraints1.subcommand(mcmd);
        }
        let mut policies1 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_effective_policy, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the policy already exists on the given Cloud resource.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint or Org Policy does not exist.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a `Policy` on a resource. If no `Policy` is set on the resource, NOT_FOUND is returned. The `etag` value can be used with `UpdatePolicy()` to update a `Policy` during read-modify-write.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_effective_policy").about("Gets the effective `Policy` on a resource. This is the result of merging `Policies` in the resource hierarchy and evaluating conditions. The returned `Policy` will not have an `etag` or `condition` set because it is a computed `Policy` across multiple resources. Subtrees of Resource Manager resource hierarchy with \'under:\' prefix will not be expanded.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves all of the `Policies` that exist on a particular resource.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint or the policy do not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag supplied in the request does not match the persisted etag of the policy Note: the supplied policy will perform a full overwrite of all fields.");
            policies1 = policies1.subcommand(mcmd);
        }
        let mut constraints1 = SubCommand::with_name("constraints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists `Constraints` that could be applied on the specified resource.");
            constraints1 = constraints1.subcommand(mcmd);
        }
        let mut policies1 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_effective_policy, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the policy already exists on the given Cloud resource.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint or Org Policy does not exist.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a `Policy` on a resource. If no `Policy` is set on the resource, NOT_FOUND is returned. The `etag` value can be used with `UpdatePolicy()` to update a `Policy` during read-modify-write.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_effective_policy").about("Gets the effective `Policy` on a resource. This is the result of merging `Policies` in the resource hierarchy and evaluating conditions. The returned `Policy` will not have an `etag` or `condition` set because it is a computed `Policy` across multiple resources. Subtrees of Resource Manager resource hierarchy with \'under:\' prefix will not be expanded.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves all of the `Policies` that exist on a particular resource.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint or the policy do not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag supplied in the request does not match the persisted etag of the policy Note: the supplied policy will perform a full overwrite of all fields.");
            policies1 = policies1.subcommand(mcmd);
        }
        let mut constraints1 = SubCommand::with_name("constraints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists `Constraints` that could be applied on the specified resource.");
            constraints1 = constraints1.subcommand(mcmd);
        }
        let mut policies1 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_effective_policy, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the policy already exists on the given Cloud resource.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint or Org Policy does not exist.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a `Policy` on a resource. If no `Policy` is set on the resource, NOT_FOUND is returned. The `etag` value can be used with `UpdatePolicy()` to update a `Policy` during read-modify-write.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_effective_policy").about("Gets the effective `Policy` on a resource. This is the result of merging `Policies` in the resource hierarchy and evaluating conditions. The returned `Policy` will not have an `etag` or `condition` set because it is a computed `Policy` across multiple resources. Subtrees of Resource Manager resource hierarchy with \'under:\' prefix will not be expanded.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves all of the `Policies` that exist on a particular resource.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Policy. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the constraint or the policy do not exist. Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag supplied in the request does not match the persisted etag of the policy Note: the supplied policy will perform a full overwrite of all fields.");
            policies1 = policies1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(policies1);
        projects0 = projects0.subcommand(constraints1);
        organizations0 = organizations0.subcommand(policies1);
        organizations0 = organizations0.subcommand(constraints1);
        folders0 = folders0.subcommand(policies1);
        folders0 = folders0.subcommand(constraints1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_orgpolicy2 as api;

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
