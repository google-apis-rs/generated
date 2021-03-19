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
        let mut app = App::new("dialogflow3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210318")
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
            .about("sub-resources: locations and operations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: agents, operations and security_settings");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut agents2 = SubCommand::with_name("agents")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, export, get, get_validation_result, list, patch, restore and validate");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an agent in the specified location.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified agent.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export")
                .about("Exports the specified agent to a binary file.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified agent.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_validation_result").about("Gets the latest agent validation result. Agent validation is performed when ValidateAgent is called.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all agents in the specified location.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified agent.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Restores the specified agent from a binary file. Replaces the current agent with a new one. Note that all existing resources in agent (e.g. intents, entity types, flows) will be removed.");
            agents2 = agents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Validates the specified agent and creates or updates validation results. The agent in draft version is validated. Please call this API after the training is completed to get the complete validation results.");
            agents2 = agents2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
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
        let mut security_settings2 = SubCommand::with_name("security_settings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create security settings in the specified location.");
            security_settings2 = security_settings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified SecuritySettings.");
            security_settings2 = security_settings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified SecuritySettings. The returned settings may be stale by up to 1 minute.");
            security_settings2 = security_settings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all security settings in the specified location.");
            security_settings2 = security_settings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified SecuritySettings.");
            security_settings2 = security_settings2.subcommand(mcmd);
        }
        let mut entity_types3 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an entity type in the specified agent.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified entity type.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified entity type.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all entity types in the specified agent.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified entity type.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        let mut environments3 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, lookup_environment_history and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an Environment in the specified Agent.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified Environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all environments in the specified Agent.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup_environment_history")
                .about("Looks up the history of the specified Environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        let mut flows3 = SubCommand::with_name("flows")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_validation_result, list, patch, train and validate");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a flow in the specified agent.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a specified flow.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified flow.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_validation_result").about("Gets the latest flow validation result. Flow validation is performed when ValidateFlow is called.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all flows in the specified agent.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified flow.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("train").about("Trains the specified flow. Note that only the flow in \'draft\' environment is trained.");
            flows3 = flows3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Validates the specified flow and creates or updates validation results. Please call this API after the training is completed to get the complete validation results.");
            flows3 = flows3.subcommand(mcmd);
        }
        let mut intents3 = SubCommand::with_name("intents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates an intent in the specified agent.");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified intent.");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified intent.");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all intents in the specified agent.");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified intent.");
            intents3 = intents3.subcommand(mcmd);
        }
        let mut sessions3 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: detect_intent, fulfill_intent and match_intent");
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fulfill_intent").about("Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("match_intent").about(
                "Returns preliminary intent match results, doesn\'t change the session status.",
            );
            sessions3 = sessions3.subcommand(mcmd);
        }
        let mut test_cases3 = SubCommand::with_name("test_cases")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_delete, batch_run, calculate_coverage, create, export, get, import, list, patch and run");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Batch deletes test cases.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("batch_run").about("Kicks off a batch run of test cases.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("calculate_coverage")
                .about("Calculates the test coverage for an agent.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a test case for the given agent.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports the test cases under the agent to a Cloud Storage bucket or a local file. Filter can be applied to export a subset of test cases.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a test case.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports the test cases from a Cloud Storage bucket or a local file. It always creates new test cases and won\'t overwite any existing ones. The provided ID in the imported test case is neglected.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Fetches a list of test cases for a given agent.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified test case.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Kicks off a test case run.");
            test_cases3 = test_cases3.subcommand(mcmd);
        }
        let mut webhooks3 = SubCommand::with_name("webhooks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a webhook in the specified agent.");
            webhooks3 = webhooks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified webhook.");
            webhooks3 = webhooks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified webhook.");
            webhooks3 = webhooks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all webhooks in the specified agent.");
            webhooks3 = webhooks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified webhook.");
            webhooks3 = webhooks3.subcommand(mcmd);
        }
        let mut experiments4 = SubCommand::with_name("experiments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, start and stop");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an Experiment in the specified Environment.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Experiment.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified Experiment.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all experiments in the specified Environment.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Experiment.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Starts the specified Experiment. This rpc only changes the state of experiment from PENDING to RUNNING.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops the specified Experiment. This rpc only changes the state of experiment from RUNNING to DONE.");
            experiments4 = experiments4.subcommand(mcmd);
        }
        let mut sessions4 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: detect_intent, fulfill_intent and match_intent");
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).");
            sessions4 = sessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fulfill_intent").about("Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined.");
            sessions4 = sessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("match_intent").about(
                "Returns preliminary intent match results, doesn\'t change the session status.",
            );
            sessions4 = sessions4.subcommand(mcmd);
        }
        let mut pages4 = SubCommand::with_name("pages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a page in the specified flow.");
            pages4 = pages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified page.");
            pages4 = pages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified page.");
            pages4 = pages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all pages in the specified flow.");
            pages4 = pages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified page.");
            pages4 = pages4.subcommand(mcmd);
        }
        let mut transition_route_groups4 = SubCommand::with_name("transition_route_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an TransitionRouteGroup in the specified flow.");
            transition_route_groups4 = transition_route_groups4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TransitionRouteGroup.");
            transition_route_groups4 = transition_route_groups4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified TransitionRouteGroup.");
            transition_route_groups4 = transition_route_groups4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all transition route groups in the specified flow.");
            transition_route_groups4 = transition_route_groups4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified TransitionRouteGroup.");
            transition_route_groups4 = transition_route_groups4.subcommand(mcmd);
        }
        let mut versions4 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, load and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a Version in the specified Flow.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Version.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified Version.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all versions in the specified Flow.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("load").about("Loads a specified version to draft version.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Version.");
            versions4 = versions4.subcommand(mcmd);
        }
        let mut entity_types4 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified session entity type.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified session entity type.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all session entity types in the specified session.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified session entity type.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        let mut results4 = SubCommand::with_name("results")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a test case result.");
            results4 = results4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Fetches a list of results for a given test case.");
            results4 = results4.subcommand(mcmd);
        }
        let mut entity_types5 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified session entity type.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified session entity type.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all session entity types in the specified session.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified session entity type.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        sessions4 = sessions4.subcommand(entity_types5);
        test_cases3 = test_cases3.subcommand(results4);
        sessions3 = sessions3.subcommand(entity_types4);
        flows3 = flows3.subcommand(versions4);
        flows3 = flows3.subcommand(transition_route_groups4);
        flows3 = flows3.subcommand(pages4);
        environments3 = environments3.subcommand(sessions4);
        environments3 = environments3.subcommand(experiments4);
        agents2 = agents2.subcommand(webhooks3);
        agents2 = agents2.subcommand(test_cases3);
        agents2 = agents2.subcommand(sessions3);
        agents2 = agents2.subcommand(intents3);
        agents2 = agents2.subcommand(flows3);
        agents2 = agents2.subcommand(environments3);
        agents2 = agents2.subcommand(entity_types3);
        locations1 = locations1.subcommand(security_settings2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(agents2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dialogflow3 as api;

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
