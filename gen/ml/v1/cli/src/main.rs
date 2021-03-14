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
            .version("0.1.0-20210303")
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
            .about("methods: explain, get_config and predict");
        {
            let mcmd = SubCommand::with_name("explain").about("Performs explanation on the data in the request. {% dynamic include \"/ai-platform/includes/___explain-request\" %} ");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_config").about("Get the service account information associated with your project. You need this information in order to grant the service account permissions for the Google Cloud Storage location where you put your model training code for training the model with Google Cloud Machine Learning.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("predict").about("Performs online prediction on the data in the request. {% dynamic include \"/ai-platform/includes/___predict-request\" %} ");
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
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the jobs in the project. If there are no jobs that match the request parameters, the list request returns an empty response body: {}.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a specific job resource. Currently the only supported fields to update are `labels`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get the complete list of CMLE capabilities in a location, along with their location-specific properties.");
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
            let mcmd = SubCommand::with_name("create").about("Creates a model which will later contain one or more versions. You must add at least one version before you can request predictions from the model. Add versions by calling projects.models.versions.create.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model. You can only delete a model if there are no versions in it. You can delete versions by calling projects.models.versions.delete.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a model, including its name, the description (if set), and the default version (if at least one version of the model has been deployed).");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the models in a project. Each project can contain multiple models, and each model can have multiple versions. If there are no models that match the request parameters, the list request returns an empty response body: {}.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a specific model resource. Currently the only supported fields to update are `description` and `default_version.name`.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            models1 = models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            models1 = models1.subcommand(mcmd);
        }
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
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut studies2 = SubCommand::with_name("studies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a study.");
            studies2 = studies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a study.");
            studies2 = studies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a study.");
            studies2 = studies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the studies in a region for an associated project.");
            studies2 = studies2.subcommand(mcmd);
        }
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and set_default");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new version of a model from a trained TensorFlow model. If the version created in the cloud by this call is the first deployed version of the specified model, it will be made the default version of the model. When you add a version to a model that already has one or more versions, the default version does not automatically change. If you want a new version to be the default, you must call projects.models.versions.setDefault.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model version. Each model can have multiple versions deployed and in use at any given time. Use this method to remove a single version. Note: You cannot delete the version that is set as the default version of the model unless it is the only remaining version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a model version. Models can have multiple versions. You can call projects.models.versions.list to get the same information that this method returns for all of the versions of a model.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets basic information about all the versions of a model. If you expect that a model has many versions, or if you need to handle only a limited number of results at a time, you can request that the list be retrieved in batches (called pages). If there are no versions that match the request parameters, the list request returns an empty response body: {}.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Version resource. Currently the only update-able fields are `description`, `requestLoggingConfig`, `autoScaling.minNodes`, and `manualScaling.nodes`.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_default").about("Designates a version to be the default for the model. The default version is used for prediction requests made against the model that don\'t specify a version. The first version to be created for a model is automatically set as the default. You must make any subsequent changes to the default version setting manually using this method.");
            versions2 = versions2.subcommand(mcmd);
        }
        let mut trials3 = SubCommand::with_name("trials")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_measurement, check_early_stopping_state, complete, create, delete, get, list, list_optimal_trials, stop and suggest");
        {
            let mcmd = SubCommand::with_name("add_measurement").about("Adds a measurement of the objective metrics to a trial. This measurement is assumed to have been taken before the trial is complete.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("check_early_stopping_state").about("Checks whether a trial should stop or not. Returns a long-running operation. When the operation is successful, it will contain a CheckTrialEarlyStoppingStateResponse.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("complete").about("Marks a trial as complete.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("create").about("Adds a user provided trial to a study.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a trial.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a trial.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the trials associated with a study.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_optimal_trials").about("Lists the pareto-optimal trials for multi-objective study or the optimal trials for single-objective study. The definition of pareto-optimal can be checked in wiki page. https://en.wikipedia.org/wiki/Pareto_efficiency");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops a trial.");
            trials3 = trials3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest").about("Adds one or more trials to a study, with parameter values suggested by AI Platform Vizier. Returns a long-running operation associated with the generation of trial suggestions. When this long-running operation succeeds, it will contain a SuggestTrialsResponse.");
            trials3 = trials3.subcommand(mcmd);
        }
        studies2 = studies2.subcommand(trials3);
        models1 = models1.subcommand(versions2);
        locations1 = locations1.subcommand(studies2);
        locations1 = locations1.subcommand(operations2);
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
