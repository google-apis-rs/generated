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
        let mut app = App::new("dialogflow2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190914")
            .about("Builds conversational interfaces (for example, chatbots, and voice-powered apps and devices).")
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
            .about("methods: agent_method, delete_agent and get_agent");
        {
            let mcmd =
                SubCommand::with_name("agent_method").about("Creates/updates the specified agent.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_agent").about("Deletes the specified agent.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_agent").about("Retrieves the specified agent.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut agent1 = SubCommand::with_name("agent")
            .setting(AppSettings::ColoredHelp)
            .about("methods: export, import, restore, search and train");
        {
            let mcmd = SubCommand::with_name("export").about("Exports the specified agent to a ZIP file.\n\nOperation <response: ExportAgentResponse>");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports the specified agent from a ZIP file.\n\nUploads new intents and entity types without deleting the existing ones.\nIntents and entity types with the same name are replaced with the new\nversions from ImportAgentRequest.\n\nOperation <response: google.protobuf.Empty>");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Restores the specified agent from a ZIP file.\n\nReplaces the current agent version with a new one. All the intents and\nentity types in the older version are deleted.\n\nOperation <response: google.protobuf.Empty>");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Returns the list of agents.\n\nSince there is at most one conversational agent per project, this method is\nuseful primarily for listing all agents across projects the caller has\naccess to. One can achieve that with a wildcard project collection id \"-\".\nRefer to [List\nSub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections).");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("train").about(
                "Trains the specified agent.\n\nOperation <response: google.protobuf.Empty>",
            );
            agent1 = agent1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut entity_types2 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes entity types in the specified agent.\n\nOperation <response: google.protobuf.Empty>");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates/Creates multiple entity types in the specified agent.\n\nOperation <response: BatchUpdateEntityTypesResponse>");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an entity type in the specified agent.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified entity type.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified entity type.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all entity types in the specified agent.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified entity type.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        let mut intents2 = SubCommand::with_name("intents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes intents in the specified agent.\n\nOperation <response: google.protobuf.Empty>");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates/Creates multiple intents in the specified agent.\n\nOperation <response: BatchUpdateIntentsResponse>");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates an intent in the specified agent.");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified intent and its direct or indirect followup intents.");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified intent.");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all intents in the specified agent.");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified intent.");
            intents2 = intents2.subcommand(mcmd);
        }
        let mut sessions2 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_contexts and detect_intent");
        {
            let mcmd = SubCommand::with_name("delete_contexts")
                .about("Deletes all active contexts in the specified session.");
            sessions2 = sessions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data\nas a result. This method is not idempotent, because it may cause contexts\nand session entity types to be updated, which in turn might affect\nresults of future queries.");
            sessions2 = sessions2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut entities3 = SubCommand::with_name("entities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_create, batch_delete and batch_update");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates multiple new entities in the specified entity type.\n\nOperation <response: google.protobuf.Empty>");
            entities3 = entities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes entities in the specified entity type.\n\nOperation <response: google.protobuf.Empty>");
            entities3 = entities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates or creates multiple entities in the specified entity type. This\nmethod does not affect entities in the entity type that aren\'t explicitly\nspecified in the request.\n\nOperation <response: google.protobuf.Empty>");
            entities3 = entities3.subcommand(mcmd);
        }
        let mut contexts3 = SubCommand::with_name("contexts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a context.\n\nIf the specified context already exists, overrides the context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all contexts in the specified session.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        let mut entity_types3 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type.\n\nIf the specified session entity type already exists, overrides the session\nentity type.\n\nThis method doesn\'t work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified session entity type.\n\nThis method doesn\'t work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified session entity type.\n\nThis method doesn\'t work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all session entity types in the specified session.\n\nThis method doesn\'t work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified session entity type.\n\nThis method doesn\'t work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        sessions2 = sessions2.subcommand(entity_types3);
        sessions2 = sessions2.subcommand(contexts3);
        entity_types2 = entity_types2.subcommand(entities3);
        locations1 = locations1.subcommand(operations2);
        agent1 = agent1.subcommand(sessions2);
        agent1 = agent1.subcommand(intents2);
        agent1 = agent1.subcommand(entity_types2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(agent1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dialogflow2 as api;

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
