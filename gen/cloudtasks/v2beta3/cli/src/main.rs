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
        let mut app = App::new("cloudtasks2_beta3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190916")
            .about("Manages the execution of large numbers of distributed requests.")
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
        let mut queues2 = SubCommand::with_name("queues")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, pause, purge, resume, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a queue.\n\nQueues created with this method allow tasks to live for a maximum of 31\ndays. After a task is 31 days old, the task will be deleted regardless of whether\nit was dispatched or not.\n\nWARNING: Using this method may have unintended side effects if you are\nusing an App Engine `queue.yaml` or `queue.xml` file to manage your queues.\nRead\n[Overview of Queue Management and\nqueue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using\nthis method.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a queue.\n\nThis command will delete the queue even if it has tasks in it.\n\nNote: If you delete a queue, a queue with the same name can\'t be created\nfor 7 days.\n\nWARNING: Using this method may have unintended side effects if you are\nusing an App Engine `queue.yaml` or `queue.xml` file to manage your queues.\nRead\n[Overview of Queue Management and\nqueue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using\nthis method.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a queue.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a Queue.\nReturns an empty policy if the resource exists and does not have a policy\nset.\n\nAuthorization requires the following\n[Google IAM](https://cloud.google.com/iam) permission on the specified\nresource parent:\n\n* `cloudtasks.queues.getIamPolicy`");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists queues.\n\nQueues are returned in lexicographical order.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a queue.\n\nThis method creates the queue if it does not exist and updates\nthe queue if it does exist.\n\nQueues created with this method allow tasks to live for a maximum of 31\ndays. After a task is 31 days old, the task will be deleted regardless of whether\nit was dispatched or not.\n\nWARNING: Using this method may have unintended side effects if you are\nusing an App Engine `queue.yaml` or `queue.xml` file to manage your queues.\nRead\n[Overview of Queue Management and\nqueue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using\nthis method.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Pauses the queue.\n\nIf a queue is paused then the system will stop dispatching tasks\nuntil the queue is resumed via\nResumeQueue. Tasks can still be added\nwhen the queue is paused. A queue is paused if its\nstate is PAUSED.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Purges a queue by deleting all of its tasks.\n\nAll tasks created before this method is called are permanently deleted.\n\nPurge operations can take up to one minute to take effect. Tasks\nmight be dispatched before the purge takes effect. A purge is irreversible.");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Resume a queue.\n\nThis method resumes a queue after it has been\nPAUSED or\nDISABLED. The state of a queue is stored\nin the queue\'s state; after calling this method it\nwill be set to RUNNING.\n\nWARNING: Resuming many high-QPS queues at the same time can\nlead to target overloading. If you are resuming high-QPS\nqueues, follow the 500/50/5 pattern described in\n[Managing Cloud Tasks Scaling\nRisks](https://cloud.google.com/tasks/docs/manage-cloud-task-scaling).");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a Queue. Replaces any existing\npolicy.\n\nNote: The Cloud Console does not check queue-level IAM permissions yet.\nProject-level permissions are required to use the Cloud Console.\n\nAuthorization requires the following\n[Google IAM](https://cloud.google.com/iam) permission on the specified\nresource parent:\n\n* `cloudtasks.queues.setIamPolicy`");
            queues2 = queues2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on a Queue.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            queues2 = queues2.subcommand(mcmd);
        }
        let mut tasks3 = SubCommand::with_name("tasks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and run");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a task and adds it to a queue.\n\nTasks cannot be updated after creation; there is no UpdateTask command.\n\n* The maximum task size is 100KB.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a task.\n\nA task can be deleted if it is scheduled or dispatched. A task\ncannot be deleted if it has executed successfully or permanently\nfailed.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a task.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the tasks in a queue.\n\nBy default, only the BASIC view is retrieved\ndue to performance considerations;\nresponse_view controls the\nsubset of information which is returned.\n\nThe tasks may be returned in any order. The ordering may change at any\ntime.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Forces a task to run now.\n\nWhen this method is called, Cloud Tasks will dispatch the task, even if\nthe task is already running, the queue has reached its RateLimits or\nis PAUSED.\n\nThis command is meant to be used for manual debugging. For\nexample, RunTask can be used to retry a failed\ntask after a fix has been made or to manually force a task to be\ndispatched now.\n\nThe dispatched task is returned. That is, the task that is returned\ncontains the status after the task is dispatched but\nbefore the task is received by its target.\n\nIf Cloud Tasks receives a successful response from the task\'s\ntarget, then the task will be deleted; otherwise the task\'s\nschedule_time will be reset to the time that\nRunTask was called plus the retry delay specified\nin the queue\'s RetryConfig.\n\nRunTask returns\nNOT_FOUND when it is called on a\ntask that has already succeeded or permanently failed.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        queues2 = queues2.subcommand(tasks3);
        locations1 = locations1.subcommand(queues2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_cloudtasks2_beta3 as api;

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
