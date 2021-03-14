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
        let mut app = App::new("gameservices1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210304")
            .about("Deploy and manage infrastructure for global multiplayer gaming experiences.")
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
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut game_server_deployments2 = SubCommand::with_name("game_server_deployments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, fetch_deployment_state, get, get_iam_policy, get_rollout, list, patch, preview_rollout, set_iam_policy, test_iam_permissions and update_rollout");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new game server deployment in a given project and location.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a single game server deployment.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_deployment_state").about("Retrieves information about the current state of the game server deployment. Gathers all the Agones fleets and Agones autoscalers, including fleets running an older version of the game server deployment.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets details of a single game server deployment.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_rollout")
                .about("Gets details a single game server deployment rollout.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists game server deployments in a given project and location.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a game server deployment.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("preview_rollout").about("Previews the game server deployment rollout. This API does not mutate the rollout resource.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_rollout").about("Patches a single game server deployment rollout. The method will not return an error if the update does not affect any existing realms. For example - if the default_game_server_config is changed but all existing realms use the override, that is valid. Similarly, if a non existing realm is explicitly called out in game_server_config_overrides field, that will also not result in an error.");
            game_server_deployments2 = game_server_deployments2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut realms2 = SubCommand::with_name("realms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and preview_update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new realm in a given project and location.");
            realms2 = realms2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single realm.");
            realms2 = realms2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single realm.");
            realms2 = realms2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists realms in a given project and location.");
            realms2 = realms2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a single realm.");
            realms2 = realms2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("preview_update")
                .about("Previews patches to a single realm.");
            realms2 = realms2.subcommand(mcmd);
        }
        let mut configs3 = SubCommand::with_name("configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new game server config in a given project, location, and game server deployment. Game server configs are immutable, and are not applied until referenced in the game server deployment rollout resource.");
            configs3 = configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single game server config. The deletion will fail if the game server config is referenced in a game server deployment rollout.");
            configs3 = configs3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets details of a single game server config.");
            configs3 = configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists game server configs in a given project, location, and game server deployment.");
            configs3 = configs3.subcommand(mcmd);
        }
        let mut game_server_clusters3 = SubCommand::with_name("game_server_clusters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, list, patch, preview_create, preview_delete and preview_update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new game server cluster in a given project and location.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a single game server cluster.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets details of a single game server cluster.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists game server clusters in a given project and location.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Patches a single game server cluster.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("preview_create").about(
                "Previews creation of a new game server cluster in a given project and location.",
            );
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("preview_delete")
                .about("Previews deletion of a single game server cluster.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("preview_update")
                .about("Previews updating a GameServerCluster.");
            game_server_clusters3 = game_server_clusters3.subcommand(mcmd);
        }
        realms2 = realms2.subcommand(game_server_clusters3);
        game_server_deployments2 = game_server_deployments2.subcommand(configs3);
        locations1 = locations1.subcommand(realms2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(game_server_deployments2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_gameservices1_beta as api;

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
