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
        let mut app = App::new("dataproc1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200409")
            .about("Manages Hadoop-based clusters and jobs on Google Cloud Platform.")
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
            .about("sub-resources: locations and regions");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: autoscaling_policies and workflow_templates");
        let mut regions1 = SubCommand::with_name("regions")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: autoscaling_policies, clusters, jobs, operations and workflow_templates");
        let mut autoscaling_policies2 = SubCommand::with_name("autoscaling_policies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates new autoscaling policy.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an autoscaling policy. It is an error to delete an autoscaling policy that is in use by one or more clusters.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves autoscaling policy.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists autoscaling policies in the project.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates (replaces) autoscaling policy.Disabled check for update_mask, because all updates will be full replacements.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        let mut workflow_templates2 = SubCommand::with_name("workflow_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, instantiate, instantiate_inline, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates new workflow template.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a workflow template. It does not cancel in-progress workflows.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the latest workflow template.Can retrieve previously instantiated template by specifying optional version parameter.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("instantiate").about("Instantiates a template and begins execution.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("instantiate_inline").about("Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists workflows that match the specified filter in the request.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates (replaces) workflow template. The updated template must contain version that matches the current server version.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        let mut autoscaling_policies2 = SubCommand::with_name("autoscaling_policies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates new autoscaling policy.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an autoscaling policy. It is an error to delete an autoscaling policy that is in use by one or more clusters.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves autoscaling policy.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists autoscaling policies in the project.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates (replaces) autoscaling policy.Disabled check for update_mask, because all updates will be full replacements.");
            autoscaling_policies2 = autoscaling_policies2.subcommand(mcmd);
        }
        let mut clusters2 = SubCommand::with_name("clusters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, diagnose, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("diagnose").about("Gets cluster diagnostic information. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata). After the operation completes, Operation.response contains DiagnoseClusterResults (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#diagnoseclusterresults).");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the resource representation for a cluster in a project.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all regions/{region}/clusters in a project alphabetically.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        let mut jobs2 = SubCommand::with_name("jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: cancel, delete, get, get_iam_policy, list, patch, set_iam_policy, submit, submit_as_operation and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts a job cancellation request. To access the job resource after cancellation, call regions/{region}/jobs.list (https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/list) or regions/{region}/jobs.get (https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/get).");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the job from the project. If the job is active, the delete fails, and the response returns FAILED_PRECONDITION.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the resource representation for a job in a project.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists regions/{region}/jobs in a project.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a job in a project.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("submit").about("Submits a job to a cluster.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("submit_as_operation").about("Submits job to a cluster.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: cancel, delete, get, get_iam_policy, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns google.rpc.Code.UNIMPLEMENTED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut workflow_templates2 = SubCommand::with_name("workflow_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, instantiate, instantiate_inline, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates new workflow template.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a workflow template. It does not cancel in-progress workflows.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the latest workflow template.Can retrieve previously instantiated template by specifying optional version parameter.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("instantiate").about("Instantiates a template and begins execution.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("instantiate_inline").about("Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists workflows that match the specified filter in the request.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates (replaces) workflow template. The updated template must contain version that matches the current server version.");
            workflow_templates2 = workflow_templates2.subcommand(mcmd);
        }
        regions1 = regions1.subcommand(workflow_templates2);
        regions1 = regions1.subcommand(operations2);
        regions1 = regions1.subcommand(jobs2);
        regions1 = regions1.subcommand(clusters2);
        regions1 = regions1.subcommand(autoscaling_policies2);
        locations1 = locations1.subcommand(workflow_templates2);
        locations1 = locations1.subcommand(autoscaling_policies2);
        projects0 = projects0.subcommand(regions1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dataproc1 as api;

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
