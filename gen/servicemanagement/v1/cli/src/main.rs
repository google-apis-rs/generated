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
        let mut app = App::new("servicemanagement1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200508")
            .about("Google Service Management allows service producers to publish their services on Google Cloud Platform so that they can be discovered and used by service consumers.")
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
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists service operations that match the specified filter in the request.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut services0 = SubCommand::with_name("services")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, disable, enable, generate_config_report, get, get_config, get_iam_policy, list, set_iam_policy, test_iam_permissions and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new managed service.\n\nA managed service is immutable, and is subject to mandatory 30-day\ndata retention. You cannot move a service or recreate it within 30 days\nafter deletion.\n\nOne producer project can own no more than 500 services. For security and\nreliability purposes, a production service should be hosted in a\ndedicated producer project.\n\nOperation<response: ManagedService>");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a managed service. This method will change the service to the\n`Soft-Delete` state for 30 days. Within this period, service producers may\ncall UndeleteService to restore the service.\nAfter 30 days, the service will be permanently deleted.\n\nOperation<response: google.protobuf.Empty>");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about("Disables a service for a project, so it can no longer be\nbe used for the project. It prevents accidental usage that may cause\nunexpected billing charges or security leaks.\n\nOperation<response: DisableServiceResponse>");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable").about("Enables a service for a project, so it can be used\nfor the project. See\n[Cloud Auth Guide](https://cloud.google.com/docs/authentication) for\nmore information.\n\nOperation<response: EnableServiceResponse>");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_config_report").about("Generates and returns a report (errors, warnings and changes from\nexisting configurations) associated with\nGenerateConfigReportRequest.new_value\n\nIf GenerateConfigReportRequest.old_value is specified,\nGenerateConfigReportRequest will contain a single ChangeReport based on the\ncomparison between GenerateConfigReportRequest.new_value and\nGenerateConfigReportRequest.old_value.\nIf GenerateConfigReportRequest.old_value is not specified, this method\nwill compare GenerateConfigReportRequest.new_value with the last pushed\nservice configuration.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets a managed service. Authentication is required unless the service is\npublic.",
            );
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Gets a service configuration (version) for a managed service.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists managed services.\n\nReturns all public services. For authenticated users, also returns all\nservices the calling user has \"servicemanagement.services.get\" permission\nfor.\n\n**BETA:** If the caller specifies the `consumer_id`, it returns only the\nservices enabled on the consumer. The `consumer_id` must have the format\nof \"project:{PROJECT-ID}\".");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Revives a previously deleted managed service. The method restores the\nservice using the configuration at the time the service was deleted.\nThe target service must exist and must have been deleted within the\nlast 30 days.\n\nOperation<response: UndeleteServiceResponse>");
            services0 = services0.subcommand(mcmd);
        }
        let mut configs1 = SubCommand::with_name("configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and submit");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new service configuration (version) for a managed service.\nThis method only stores the service configuration. To roll out the service\nconfiguration to backend systems please call\nCreateServiceRollout.\n\nOnly the 100 most recent service configurations and ones referenced by\nexisting rollouts are kept for each service. The rest will be deleted\neventually.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a service configuration (version) for a managed service.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the history of the service configuration for a managed service,\nfrom the newest to the oldest.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("submit").about("Creates a new service configuration (version) for a managed service based\non\nuser-supplied configuration source files (for example: OpenAPI\nSpecification). This method stores the source configurations as well as the\ngenerated service configuration. To rollout the service configuration to\nother services,\nplease call CreateServiceRollout.\n\nOnly the 100 most recent configuration sources and ones referenced by\nexisting service configurtions are kept for each service. The rest will be\ndeleted eventually.\n\nOperation<response: SubmitConfigSourceResponse>");
            configs1 = configs1.subcommand(mcmd);
        }
        let mut consumers1 = SubCommand::with_name("consumers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            consumers1 = consumers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            consumers1 = consumers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            consumers1 = consumers1.subcommand(mcmd);
        }
        let mut rollouts1 = SubCommand::with_name("rollouts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new service configuration rollout. Based on rollout, the\nGoogle Service Management will roll out the service configurations to\ndifferent backend services. For example, the logging configuration will be\npushed to Google Cloud Logging.\n\nPlease note that any previous pending and running Rollouts and associated\nOperations will be automatically cancelled so that the latest Rollout will\nnot be blocked by previous Rollouts.\n\nOnly the 100 most recent (in any state) and the last 10 successful (if not\nalready part of the set of 100 most recent) rollouts are kept for each\nservice. The rest will be deleted eventually.\n\nOperation<response: Rollout>");
            rollouts1 = rollouts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a service configuration rollout.");
            rollouts1 = rollouts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the history of the service configuration rollouts for a managed\nservice, from the newest to the oldest.");
            rollouts1 = rollouts1.subcommand(mcmd);
        }
        services0 = services0.subcommand(rollouts1);
        services0 = services0.subcommand(consumers1);
        services0 = services0.subcommand(configs1);
        app = app.subcommand(services0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_servicemanagement1 as api;

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
