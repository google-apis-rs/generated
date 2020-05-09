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
        let mut app = App::new("spanner1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200409")
            .about("Cloud Spanner is a managed, mission-critical, globally consistent and scalable relational database service.")
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
            .about("sub-resources: instance_configs and instances");
        let mut instance_configs1 = SubCommand::with_name("instance_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets information about a particular instance configuration.");
            instance_configs1 = instance_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the supported instance configurations for a given project.");
            instance_configs1 = instance_configs1.subcommand(mcmd);
        }
        let mut instances1 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an instance and begins preparing it to begin serving. The\nreturned long-running operation\ncan be used to track the progress of preparing the new\ninstance. The instance name is assigned by the caller. If the\nnamed instance already exists, `CreateInstance` returns\n`ALREADY_EXISTS`.\n\nImmediately upon completion of this request:\n\n  * The instance is readable via the API, with all requested attributes\n    but no allocated resources. Its state is `CREATING`.\n\nUntil completion of the returned operation:\n\n  * Cancelling the operation renders the instance immediately unreadable\n    via the API.\n  * The instance can be deleted.\n  * All other attempts to modify the instance are rejected.\n\nUpon completion of the returned operation:\n\n  * Billing for all successfully-allocated resources begins (some types\n    may have lower than the requested levels).\n  * Databases can be created in the instance.\n  * The instance\'s allocated resource levels are readable via the API.\n  * The instance\'s state becomes `READY`.\n\nThe returned long-running operation will\nhave a name of the format `<instance_name>/operations/<operation_id>` and\ncan be used to track creation of the instance.  The\nmetadata field type is\nCreateInstanceMetadata.\nThe response field type is\nInstance, if successful.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an instance.\n\nImmediately upon completion of the request:\n\n  * Billing ceases for all of the instance\'s reserved resources.\n\nSoon afterward:\n\n  * The instance and *all of its databases* immediately and\n    irrevocably disappear from the API. All data in the databases\n    is permanently deleted.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a particular instance.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for an instance resource. Returns an empty\npolicy if an instance exists but does not have a policy set.\n\nAuthorization requires `spanner.instances.getIamPolicy` on\nresource.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all instances in the given project.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an instance, and begins allocating or releasing resources\nas requested. The returned long-running\noperation can be used to track the\nprogress of updating the instance. If the named instance does not\nexist, returns `NOT_FOUND`.\n\nImmediately upon completion of this request:\n\n  * For resource types for which a decrease in the instance\'s allocation\n    has been requested, billing is based on the newly-requested level.\n\nUntil completion of the returned operation:\n\n  * Cancelling the operation sets its metadata\'s\n    cancel_time, and begins\n    restoring resources to their pre-request values. The operation\n    is guaranteed to succeed at undoing all resource changes,\n    after which point it terminates with a `CANCELLED` status.\n  * All other attempts to modify the instance are rejected.\n  * Reading the instance via the API continues to give the pre-request\n    resource levels.\n\nUpon completion of the returned operation:\n\n  * Billing begins for all successfully-allocated resources (some types\n    may have lower than the requested levels).\n  * All newly-reserved resources are available for serving the instance\'s\n    tables.\n  * The instance\'s new resource levels are readable via the API.\n\nThe returned long-running operation will\nhave a name of the format `<instance_name>/operations/<operation_id>` and\ncan be used to track the instance modification.  The\nmetadata field type is\nUpdateInstanceMetadata.\nThe response field type is\nInstance, if successful.\n\nAuthorization requires `spanner.instances.update` permission on\nresource name.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on an instance resource. Replaces any\nexisting policy.\n\nAuthorization requires `spanner.instances.setIamPolicy` on\nresource.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that the caller has on the specified instance resource.\n\nAttempting this RPC on a non-existent Cloud Spanner instance resource will\nresult in a NOT_FOUND error if the user has `spanner.instances.list`\npermission on the containing Google Cloud Project. Otherwise returns an\nempty set of permissions.");
            instances1 = instances1.subcommand(mcmd);
        }
        let mut backup_operations2 = SubCommand::with_name("backup_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the backup long-running operations in\nthe given instance. A backup operation has a name of the form\n`projects/<project>/instances/<instance>/backups/<backup>/operations/<operation>`.\nThe long-running operation\nmetadata field type\n`metadata.type_url` describes the type of the metadata. Operations returned\ninclude those that have completed/failed/canceled within the last 7 days,\nand pending operations. Operations returned are ordered by\n`operation.metadata.value.progress.start_time` in descending order starting\nfrom the most recently started operation.");
            backup_operations2 = backup_operations2.subcommand(mcmd);
        }
        let mut backups2 = SubCommand::with_name("backups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Starts creating a new Cloud Spanner Backup.\nThe returned backup long-running operation\nwill have a name of the format\n`projects/<project>/instances/<instance>/backups/<backup>/operations/<operation_id>`\nand can be used to track creation of the backup. The\nmetadata field type is\nCreateBackupMetadata. The\nresponse field type is\nBackup, if successful. Cancelling the returned operation will stop the\ncreation and delete the backup.\nThere can be only one pending backup creation per database. Backup creation\nof different databases can run concurrently.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a pending or completed Backup.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets metadata on a pending or completed Backup.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a database or backup resource.\nReturns an empty policy if a database or backup exists but does not have a\npolicy set.\n\nAuthorization requires `spanner.databases.getIamPolicy` permission on\nresource.\nFor backups, authorization requires `spanner.backups.getIamPolicy`\npermission on resource.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists completed and pending backups.\nBackups returned are ordered by `create_time` in descending order,\nstarting from the most recent `create_time`.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a pending or completed Backup.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on a database or backup resource.\nReplaces any existing policy.\n\nAuthorization requires `spanner.databases.setIamPolicy`\npermission on resource.\nFor backups, authorization requires `spanner.backups.setIamPolicy`\npermission on resource.");
            backups2 = backups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that the caller has on the specified database or backup\nresource.\n\nAttempting this RPC on a non-existent Cloud Spanner database will\nresult in a NOT_FOUND error if the user has\n`spanner.databases.list` permission on the containing Cloud\nSpanner instance. Otherwise returns an empty set of permissions.\nCalling this method on a backup that does not exist will\nresult in a NOT_FOUND error if the user has\n`spanner.backups.list` permission on the containing instance.");
            backups2 = backups2.subcommand(mcmd);
        }
        let mut database_operations2 = SubCommand::with_name("database_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists database longrunning-operations.\nA database operation has a name of the form\n`projects/<project>/instances/<instance>/databases/<database>/operations/<operation>`.\nThe long-running operation\nmetadata field type\n`metadata.type_url` describes the type of the metadata. Operations returned\ninclude those that have completed/failed/canceled within the last 7 days,\nand pending operations.");
            database_operations2 = database_operations2.subcommand(mcmd);
        }
        let mut databases2 = SubCommand::with_name("databases")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, drop_database, get, get_ddl, get_iam_policy, list, restore, set_iam_policy, test_iam_permissions and update_ddl");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Cloud Spanner database and starts to prepare it for serving.\nThe returned long-running operation will\nhave a name of the format `<database_name>/operations/<operation_id>` and\ncan be used to track preparation of the database. The\nmetadata field type is\nCreateDatabaseMetadata. The\nresponse field type is\nDatabase, if successful.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("drop_database").about("Drops (aka deletes) a Cloud Spanner database.\nCompleted backups for the database will be retained according to their\n`expire_time`.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the state of a Cloud Spanner database.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ddl").about("Returns the schema of a Cloud Spanner database as a list of formatted\nDDL statements. This method does not show pending schema updates, those may\nbe queried using the Operations API.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a database or backup resource.\nReturns an empty policy if a database or backup exists but does not have a\npolicy set.\n\nAuthorization requires `spanner.databases.getIamPolicy` permission on\nresource.\nFor backups, authorization requires `spanner.backups.getIamPolicy`\npermission on resource.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Cloud Spanner databases.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Create a new database by restoring from a completed backup. The new\ndatabase must be in the same project and in an instance with the same\ninstance configuration as the instance containing\nthe backup. The returned database long-running\noperation has a name of the format\n`projects/<project>/instances/<instance>/databases/<database>/operations/<operation_id>`,\nand can be used to track the progress of the operation, and to cancel it.\nThe metadata field type is\nRestoreDatabaseMetadata.\nThe response type\nis Database, if\nsuccessful. Cancelling the returned operation will stop the restore and\ndelete the database.\nThere can be only one database being restored into an instance at a time.\nOnce the restore operation completes, a new restore operation can be\ninitiated, without waiting for the optimize operation associated with the\nfirst restore to complete.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on a database or backup resource.\nReplaces any existing policy.\n\nAuthorization requires `spanner.databases.setIamPolicy`\npermission on resource.\nFor backups, authorization requires `spanner.backups.setIamPolicy`\npermission on resource.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that the caller has on the specified database or backup\nresource.\n\nAttempting this RPC on a non-existent Cloud Spanner database will\nresult in a NOT_FOUND error if the user has\n`spanner.databases.list` permission on the containing Cloud\nSpanner instance. Otherwise returns an empty set of permissions.\nCalling this method on a backup that does not exist will\nresult in a NOT_FOUND error if the user has\n`spanner.backups.list` permission on the containing instance.");
            databases2 = databases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_ddl").about("Updates the schema of a Cloud Spanner database by\ncreating/altering/dropping tables, columns, indexes, etc. The returned\nlong-running operation will have a name of\nthe format `<database_name>/operations/<operation_id>` and can be used to\ntrack execution of the schema change(s). The\nmetadata field type is\nUpdateDatabaseDdlMetadata.  The operation has no response.");
            databases2 = databases2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
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
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut sessions3 = SubCommand::with_name("sessions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create, begin_transaction, commit, create, delete, execute_batch_dml, execute_sql, execute_streaming_sql, get, list, partition_query, partition_read, read, rollback and streaming_read");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates multiple new sessions.\n\nThis API can be used to initialize a session cache on the clients.\nSee https://goo.gl/TgSFN2 for best practices on session cache management.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("begin_transaction").about("Begins a new transaction. This step can often be skipped:\nRead, ExecuteSql and\nCommit can begin a new transaction as a\nside-effect.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("commit").about("Commits a transaction. The request includes the mutations to be\napplied to rows in the database.\n\n`Commit` might return an `ABORTED` error. This can occur at any time;\ncommonly, the cause is conflicts with concurrent\ntransactions. However, it can also happen for a variety of other\nreasons. If `Commit` returns `ABORTED`, the caller should re-attempt\nthe transaction from the beginning, re-using the same session.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new session. A session can be used to perform\ntransactions that read and/or modify data in a Cloud Spanner database.\nSessions are meant to be reused for many consecutive\ntransactions.\n\nSessions can only execute one transaction at a time. To execute\nmultiple concurrent read-write/write-only transactions, create\nmultiple sessions. Note that standalone reads and queries use a\ntransaction internally, and count toward the one transaction\nlimit.\n\nActive sessions use additional server resources, so it is a good idea to\ndelete idle and unneeded sessions.\nAside from explicit deletes, Cloud Spanner may delete sessions for which no\noperations are sent for more than an hour. If a session is deleted,\nrequests to it return `NOT_FOUND`.\n\nIdle sessions can be kept alive by sending a trivial SQL query\nperiodically, e.g., `\"SELECT 1\"`.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Ends a session, releasing server resources associated with it. This will\nasynchronously trigger cancellation of any operations that are running with\nthis session.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute_batch_dml").about("Executes a batch of SQL DML statements. This method allows many statements\nto be run with lower latency than submitting them sequentially with\nExecuteSql.\n\nStatements are executed in sequential order. A request can succeed even if\na statement fails. The ExecuteBatchDmlResponse.status field in the\nresponse provides information about the statement that failed. Clients must\ninspect this field to determine whether an error occurred.\n\nExecution stops after the first failed statement; the remaining statements\nare not executed.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute_sql").about("Executes an SQL statement, returning all results in a single reply. This\nmethod cannot be used to return a result set larger than 10 MiB;\nif the query yields more data than that, the query fails with\na `FAILED_PRECONDITION` error.\n\nOperations inside read-write transactions might return `ABORTED`. If\nthis occurs, the application should restart the transaction from\nthe beginning. See Transaction for more details.\n\nLarger result sets can be fetched in streaming fashion by calling\nExecuteStreamingSql instead.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute_streaming_sql").about("Like ExecuteSql, except returns the result\nset as a stream. Unlike ExecuteSql, there\nis no limit on the size of the returned result set. However, no\nindividual row in the result set can exceed 100 MiB, and no\ncolumn value can exceed 10 MiB.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a session. Returns `NOT_FOUND` if the session does not exist.\nThis is mainly useful for determining whether a session is still\nalive.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all sessions in a given database.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("partition_query").about("Creates a set of partition tokens that can be used to execute a query\noperation in parallel.  Each of the returned partition tokens can be used\nby ExecuteStreamingSql to specify a subset\nof the query result to read.  The same session and read-only transaction\nmust be used by the PartitionQueryRequest used to create the\npartition tokens and the ExecuteSqlRequests that use the partition tokens.\n\nPartition tokens become invalid when the session used to create them\nis deleted, is idle for too long, begins a new transaction, or becomes too\nold.  When any of these happen, it is not possible to resume the query, and\nthe whole operation must be restarted from the beginning.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("partition_read").about("Creates a set of partition tokens that can be used to execute a read\noperation in parallel.  Each of the returned partition tokens can be used\nby StreamingRead to specify a subset of the read\nresult to read.  The same session and read-only transaction must be used by\nthe PartitionReadRequest used to create the partition tokens and the\nReadRequests that use the partition tokens.  There are no ordering\nguarantees on rows returned among the returned partition tokens, or even\nwithin each individual StreamingRead call issued with a partition_token.\n\nPartition tokens become invalid when the session used to create them\nis deleted, is idle for too long, begins a new transaction, or becomes too\nold.  When any of these happen, it is not possible to resume the read, and\nthe whole operation must be restarted from the beginning.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("read").about("Reads rows from the database using key lookups and scans, as a\nsimple key/value style alternative to\nExecuteSql.  This method cannot be used to\nreturn a result set larger than 10 MiB; if the read matches more\ndata than that, the read fails with a `FAILED_PRECONDITION`\nerror.\n\nReads inside read-write transactions might return `ABORTED`. If\nthis occurs, the application should restart the transaction from\nthe beginning. See Transaction for more details.\n\nLarger result sets can be yielded in streaming fashion by calling\nStreamingRead instead.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Rolls back a transaction, releasing any locks it holds. It is a good\nidea to call this for any transaction that includes one or more\nRead or ExecuteSql requests and\nultimately decides not to commit.\n\n`Rollback` returns `OK` if it successfully aborts the transaction, the\ntransaction was already aborted, or the transaction is not\nfound. `Rollback` never returns `ABORTED`.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("streaming_read").about("Like Read, except returns the result set as a\nstream. Unlike Read, there is no limit on the\nsize of the returned result set. However, no individual row in\nthe result set can exceed 100 MiB, and no column value can exceed\n10 MiB.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        databases2 = databases2.subcommand(sessions3);
        databases2 = databases2.subcommand(operations3);
        backups2 = backups2.subcommand(operations3);
        instances1 = instances1.subcommand(operations2);
        instances1 = instances1.subcommand(databases2);
        instances1 = instances1.subcommand(database_operations2);
        instances1 = instances1.subcommand(backups2);
        instances1 = instances1.subcommand(backup_operations2);
        projects0 = projects0.subcommand(instances1);
        projects0 = projects0.subcommand(instance_configs1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_spanner1 as api;

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
