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
        let mut app = App::new("ml1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190913")
            .about("An API to enable creating and using machine learning models.")
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
            .about("methods: get_config and predict");
        {
            let mcmd = SubCommand::with_name("get_config").about("Get the service account information associated with your project. You need\nthis information in order to grant the service account permissions for\nthe Google Cloud Storage location where you put your model training code\nfor training the model with Google Cloud Machine Learning.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("predict").about("Performs prediction on the data in the request.\nAI Platform implements a custom `predict` verb on top of an HTTP POST\nmethod. <p>For details of the request and response format, see the **guide\nto the [predict request format](/ml-engine/docs/v1/predict-request)**.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut jobs1 = SubCommand::with_name("jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: cancel, create, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a running job.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a training or a batch prediction job.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Describes a job.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the jobs in the project.\n\nIf there are no jobs that match the request parameters, the list\nrequest returns an empty response body: {}.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a specific job resource.\n\nCurrently the only supported fields to update are `labels`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get the complete list of CMLE capabilities in a location, along with their\nlocation-specific properties.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all locations that provides at least one type of CMLE capability.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut models1 = SubCommand::with_name("models")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a model which will later contain one or more versions.\n\nYou must add at least one version before you can request predictions from\nthe model. Add versions by calling\n[projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create).");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model.\n\nYou can only delete a model if there are no versions in it. You can delete\nversions by calling\n[projects.models.versions.delete](/ml-engine/reference/rest/v1/projects.models.versions/delete).");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a model, including its name, the description (if\nset), and the default version (if at least one version of the model has\nbeen deployed).");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the models in a project.\n\nEach project can contain multiple models, and each model can have multiple\nversions.\n\nIf there are no models that match the request parameters, the list request\nreturns an empty response body: {}.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a specific model resource.\n\nCurrently the only supported fields to update are `description` and\n`default_version.name`.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            models1 = models1.subcommand(mcmd);
        }
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
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and set_default");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new version of a model from a trained TensorFlow model.\n\nIf the version created in the cloud by this call is the first deployed\nversion of the specified model, it will be made the default version of the\nmodel. When you add a version to a model that already has one or more\nversions, the default version does not automatically change. If you want a\nnew version to be the default, you must call\n[projects.models.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault).");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model version.\n\nEach model can have multiple versions deployed and in use at any given\ntime. Use this method to remove a single version.\n\nNote: You cannot delete the version that is set as the default version\nof the model unless it is the only remaining version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a model version.\n\nModels can have multiple versions. You can call\n[projects.models.versions.list](/ml-engine/reference/rest/v1/projects.models.versions/list)\nto get the same information that this method returns for all of the\nversions of a model.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets basic information about all the versions of a model.\n\nIf you expect that a model has many versions, or if you need to handle\nonly a limited number of results at a time, you can request that the list\nbe retrieved in batches (called pages).\n\nIf there are no versions that match the request parameters, the list\nrequest returns an empty response body: {}.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Version resource.\n\nCurrently the only update-able fields are `description` and\n`autoScaling.minNodes`.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_default").about("Designates a version to be the default for the model.\n\nThe default version is used for prediction requests made against the model\nthat don\'t specify a version.\n\nThe first version to be created for a model is automatically set as the\ndefault. You must make any subsequent changes to the default version\nsetting manually using this method.");
            versions2 = versions2.subcommand(mcmd);
        }
        models1 = models1.subcommand(versions2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(models1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(jobs1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_ml1 as api;

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
