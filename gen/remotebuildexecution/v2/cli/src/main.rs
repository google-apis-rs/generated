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
        let mut app = App::new("remotebuildexecution2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200623")
            .about("Supplies a Remote Execution API service for tools such as bazel.")
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
        let mut action_results0 = SubCommand::with_name("action_results")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and update");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a cached execution result.\n\nImplementations SHOULD ensure that any blobs referenced from the\nContentAddressableStorage\nare available at the time of returning the\nActionResult and will be\nfor some period of time afterwards. The TTLs of the referenced blobs SHOULD be increased\nif necessary and applicable.\n\nErrors:\n\n* `NOT_FOUND`: The requested `ActionResult` is not in the cache.");
            action_results0 = action_results0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Upload a new execution result.\n\nIn order to allow the server to perform access control based on the type of\naction, and to assist with client debugging, the client MUST first upload\nthe Action that produced the\nresult, along with its\nCommand, into the\n`ContentAddressableStorage`.\n\nErrors:\n\n* `INVALID_ARGUMENT`: One or more arguments are invalid.\n* `FAILED_PRECONDITION`: One or more errors occurred in updating the\n  action result, such as a missing command or action.\n* `RESOURCE_EXHAUSTED`: There is insufficient storage space to add the\n  entry to the cache.");
            action_results0 = action_results0.subcommand(mcmd);
        }
        let mut actions0 = SubCommand::with_name("actions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: execute");
        {
            let mcmd = SubCommand::with_name("execute").about("Execute an action remotely.\n\nIn order to execute an action, the client must first upload all of the\ninputs, the\nCommand to run, and the\nAction into the\nContentAddressableStorage.\nIt then calls `Execute` with an `action_digest` referring to them. The\nserver will run the action and eventually return the result.\n\nThe input `Action`\'s fields MUST meet the various canonicalization\nrequirements specified in the documentation for their types so that it has\nthe same digest as other logically equivalent `Action`s. The server MAY\nenforce the requirements and return errors if a non-canonical input is\nreceived. It MAY also proceed without verifying some or all of the\nrequirements, such as for performance reasons. If the server does not\nverify the requirement, then it will treat the `Action` as distinct from\nanother logically equivalent action if they hash differently.\n\nReturns a stream of\ngoogle.longrunning.Operation messages\ndescribing the resulting execution, with eventual `response`\nExecuteResponse. The\n`metadata` on the operation is of type\nExecuteOperationMetadata.\n\nIf the client remains connected after the first response is returned after\nthe server, then updates are streamed as if the client had called\nWaitExecution\nuntil the execution completes or the request reaches an error. The\noperation can also be queried using Operations\nAPI.\n\nThe server NEED NOT implement other methods or functionality of the\nOperations API.\n\nErrors discovered during creation of the `Operation` will be reported\nas gRPC Status errors, while errors that occurred while running the\naction will be reported in the `status` field of the `ExecuteResponse`. The\nserver MUST NOT set the `error` field of the `Operation` proto.\nThe possible errors include:\n\n* `INVALID_ARGUMENT`: One or more arguments are invalid.\n* `FAILED_PRECONDITION`: One or more errors occurred in setting up the\n  action requested, such as a missing input or command or no worker being\n  available. The client may be able to fix the errors and retry.\n* `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run\n  the action.\n* `UNAVAILABLE`: Due to a transient condition, such as all workers being\n  occupied (and the server does not support a queue), the action could not\n  be started. The client should retry.\n* `INTERNAL`: An internal error occurred in the execution engine or the\n  worker.\n* `DEADLINE_EXCEEDED`: The execution timed out.\n* `CANCELLED`: The operation was cancelled by the client. This status is\n  only possible if the server implements the Operations API CancelOperation\n  method, and it was called for the current execution.\n\nIn the case of a missing input or command, the server SHOULD additionally\nsend a PreconditionFailure error detail\nwhere, for each requested blob not present in the CAS, there is a\n`Violation` with a `type` of `MISSING` and a `subject` of\n`\"blobs/{hash}/{size}\"` indicating the digest of the missing blob.");
            actions0 = actions0.subcommand(mcmd);
        }
        let mut blobs0 = SubCommand::with_name("blobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_read, batch_update, find_missing and get_tree");
        {
            let mcmd = SubCommand::with_name("batch_read").about("Download many blobs at once.\n\nThe server may enforce a limit of the combined total size of blobs\nto be downloaded using this API. This limit may be obtained using the\nCapabilities API.\nRequests exceeding the limit should either be split into smaller\nchunks or downloaded using the\nByteStream API, as appropriate.\n\nThis request is equivalent to calling a Bytestream `Read` request\non each individual blob, in parallel. The requests may succeed or fail\nindependently.\n\nErrors:\n\n* `INVALID_ARGUMENT`: The client attempted to read more than the\n  server supported limit.\n\nEvery error on individual read will be returned in the corresponding digest\nstatus.");
            blobs0 = blobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Upload many blobs at once.\n\nThe server may enforce a limit of the combined total size of blobs\nto be uploaded using this API. This limit may be obtained using the\nCapabilities API.\nRequests exceeding the limit should either be split into smaller\nchunks or uploaded using the\nByteStream API, as appropriate.\n\nThis request is equivalent to calling a Bytestream `Write` request\non each individual blob, in parallel. The requests may succeed or fail\nindependently.\n\nErrors:\n\n* `INVALID_ARGUMENT`: The client attempted to upload more than the\n  server supported limit.\n\nIndividual requests may return the following errors, additionally:\n\n* `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob.\n* `INVALID_ARGUMENT`: The\nDigest does not match the\nprovided data.");
            blobs0 = blobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("find_missing").about("Determine if blobs are present in the CAS.\n\nClients can use this API before uploading blobs to determine which ones are\nalready present in the CAS and do not need to be uploaded again.\n\nServers SHOULD increase the TTLs of the referenced blobs if necessary and\napplicable.\n\nThere are no method-specific errors.");
            blobs0 = blobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_tree").about("Fetch the entire directory tree rooted at a node.\n\nThis request must be targeted at a\nDirectory stored in the\nContentAddressableStorage\n(CAS). The server will enumerate the `Directory` tree recursively and\nreturn every node descended from the root.\n\nThe GetTreeRequest.page_token parameter can be used to skip ahead in\nthe stream (e.g. when retrying a partially completed and aborted request),\nby setting it to a value taken from GetTreeResponse.next_page_token of the\nlast successfully processed GetTreeResponse).\n\nThe exact traversal order is unspecified and, unless retrieving subsequent\npages from an earlier request, is not guaranteed to be stable across\nmultiple invocations of `GetTree`.\n\nIf part of the tree is missing from the CAS, the server will return the\nportion present and omit the rest.\n\nErrors:\n\n* `NOT_FOUND`: The requested tree root is not present in the CAS.");
            blobs0 = blobs0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: wait_execution");
        {
            let mcmd = SubCommand::with_name("wait_execution").about("Wait for an execution operation to complete. When the client initially\nmakes the request, the server immediately responds with the current status\nof the execution. The server will leave the request stream open until the\noperation completes, and then respond with the completed operation. The\nserver MAY choose to stream additional updates as execution progresses,\nsuch as to provide an update as to the state of the execution.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut v_20 = SubCommand::with_name("v_2")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_capabilities");
        {
            let mcmd = SubCommand::with_name("get_capabilities").about("GetCapabilities returns the server capabilities configuration of the\nremote endpoint.\nOnly the capabilities of the services supported by the endpoint will\nbe returned:\n* Execution + CAS + Action Cache endpoints should return both\n  CacheCapabilities and ExecutionCapabilities.\n* Execution only endpoints should return ExecutionCapabilities.\n* CAS + Action Cache only endpoints should return CacheCapabilities.");
            v_20 = v_20.subcommand(mcmd);
        }
        app = app.subcommand(v_20);
        app = app.subcommand(operations0);
        app = app.subcommand(blobs0);
        app = app.subcommand(actions0);
        app = app.subcommand(action_results0);

        Self { app }
    }
}
use google_remotebuildexecution2 as api;

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
