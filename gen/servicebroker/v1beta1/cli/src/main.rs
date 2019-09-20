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
        let mut app = App::new("servicebroker1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190918")
            .about("The Google Cloud Platform Service Broker API provides Google hosted\nimplementation of the Open Service Broker API\n(https://www.openservicebrokerapi.org/).\n")
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
            .about("sub-resources: brokers");
        let mut v_1beta_10 = SubCommand::with_name("v_1beta_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            v_1beta_10 = v_1beta_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.");
            v_1beta_10 = v_1beta_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            v_1beta_10 = v_1beta_10.subcommand(mcmd);
        }
        let mut brokers1 = SubCommand::with_name("brokers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("CreateBroker creates a Broker.");
            brokers1 = brokers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("DeleteBroker deletes a Broker.");
            brokers1 = brokers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("ListBrokers lists brokers.");
            brokers1 = brokers1.subcommand(mcmd);
        }
        let mut instances2 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_last_operation and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the given service instance from the system.\nThe API call accepts both OSB style API and standard google style API\nresource path.\ni.e. both `projects/*/brokers/*/instances/*`\n and `projects/*/brokers/*/v2/service_instances/*` are acceptable paths.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_last_operation").about("Returns the state of the last operation for the service instance.\nOnly last (or current) operation can be polled.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the instances in the brokers\nThis API is an extension and not part of the OSB spec.\nHence the path is a standard Google API URL.");
            instances2 = instances2.subcommand(mcmd);
        }
        let mut v_22 = SubCommand::with_name("v_2")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: catalog and service_instances");
        let mut bindings3 = SubCommand::with_name("bindings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_last_operation and list");
        {
            let mcmd = SubCommand::with_name("get_last_operation").about("Returns the state of the last operation for the binding.\nOnly last (or current) operation can be polled.");
            bindings3 = bindings3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all the bindings in the instance.");
            bindings3 = bindings3.subcommand(mcmd);
        }
        let mut catalog3 = SubCommand::with_name("catalog")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the Services registered with this broker for consumption for\ngiven service registry broker, which contains an set of services.\nNote, that Service producer API is separate from Broker API.");
            catalog3 = catalog3.subcommand(mcmd);
        }
        let mut service_instances3 = SubCommand::with_name("service_instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_last_operation and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Provisions a service instance.\nIf `request.accepts_incomplete` is false and Broker cannot execute request\nsynchronously HTTP 422 error will be returned along with\nFAILED_PRECONDITION status.\nIf `request.accepts_incomplete` is true and the Broker decides to execute\nresource asynchronously then HTTP 202 response code will be returned and a\nvalid polling operation in the response will be included.\nIf Broker executes the request synchronously and it succeeds HTTP 201\nresponse will be furnished.\nIf identical instance exists, then HTTP 200 response will be returned.\nIf an instance with identical ID but mismatching parameters exists, then\nHTTP 409 status code will be returned.");
            service_instances3 = service_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deprovisions a service instance.\nFor synchronous/asynchronous request details see CreateServiceInstance\nmethod.\nIf service instance does not exist HTTP 410 status will be returned.");
            service_instances3 = service_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the given service instance from the system.\nThe API call accepts both OSB style API and standard google style API\nresource path.\ni.e. both `projects/*/brokers/*/instances/*`\n and `projects/*/brokers/*/v2/service_instances/*` are acceptable paths.");
            service_instances3 = service_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_last_operation").about("Returns the state of the last operation for the service instance.\nOnly last (or current) operation can be polled.");
            service_instances3 = service_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing service instance.\nSee CreateServiceInstance for possible response codes.");
            service_instances3 = service_instances3.subcommand(mcmd);
        }
        let mut service_bindings4 = SubCommand::with_name("service_bindings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and get_last_operation");
        {
            let mcmd = SubCommand::with_name("create").about("CreateBinding generates a service binding to an existing service instance.\nSee ProviServiceInstance for async operation details.");
            service_bindings4 = service_bindings4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Unbinds from a service instance.\nFor synchronous/asynchronous request details see CreateServiceInstance\nmethod.\nIf binding does not exist HTTP 410 status will be returned.");
            service_bindings4 = service_bindings4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("GetBinding returns the binding information.");
            service_bindings4 = service_bindings4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_last_operation").about("Returns the state of the last operation for the binding.\nOnly last (or current) operation can be polled.");
            service_bindings4 = service_bindings4.subcommand(mcmd);
        }
        service_instances3 = service_instances3.subcommand(service_bindings4);
        v_22 = v_22.subcommand(service_instances3);
        v_22 = v_22.subcommand(catalog3);
        instances2 = instances2.subcommand(bindings3);
        brokers1 = brokers1.subcommand(v_22);
        brokers1 = brokers1.subcommand(instances2);
        projects0 = projects0.subcommand(brokers1);
        app = app.subcommand(v_1beta_10);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_servicebroker1_beta1 as api;

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
