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
        let mut app = App::new("storage1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210306")
            .about("Stores and retrieves potentially large, immutable data objects.")
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
        let mut bucket_access_controls0 = SubCommand::with_name("bucket_access_controls")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes the ACL entry for the specified entity on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the ACL entry for the specified entity on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a new ACL entry on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves ACL entries on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patches an ACL entry on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an ACL entry on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        let mut buckets0 = SubCommand::with_name("buckets")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_iam_policy, insert, list, lock_retention_policy, patch, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Permanently deletes an empty bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns metadata for the specified bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Returns an IAM policy for the specified bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of buckets for a given project.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lock_retention_policy")
                .about("Locks retention policy on a bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Updates an IAM policy for the specified bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Tests a set of permissions on the given bucket to see which, if any, are held by the caller.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: stop");
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stop watching resources through this channel");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut default_object_access_controls0 =
            SubCommand::with_name("default_object_access_controls")
                .setting(AppSettings::ColoredHelp)
                .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes the default object ACL entry for the specified entity on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the default object ACL entry for the specified entity on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a new default object ACL entry on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves default object ACL entries on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patches a default object ACL entry on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a default object ACL entry on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        let mut notifications0 = SubCommand::with_name("notifications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Permanently deletes a notification subscription.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("View a notification configuration.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a notification subscription for a given bucket.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of notification subscriptions for a given bucket.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        let mut object_access_controls0 = SubCommand::with_name("object_access_controls")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes the ACL entry for the specified entity on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the ACL entry for the specified entity on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a new ACL entry on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves ACL entries on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patches an ACL entry on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an ACL entry on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        let mut objects0 = SubCommand::with_name("objects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: compose, copy, delete, get, get_iam_policy, insert, list, patch, rewrite, set_iam_policy, test_iam_permissions, update and watch_all");
        {
            let mcmd = SubCommand::with_name("compose").about(
                "Concatenates a list of existing objects into a new object in the same bucket.",
            );
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("copy").about(
                "Copies a source object to a destination object. Optionally overrides metadata.",
            );
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an object or its metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Returns an IAM policy for the specified object.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Stores a new object and metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of objects matching the criteria.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches an object\'s metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rewrite").about(
                "Rewrites a source object to a destination object. Optionally overrides metadata.",
            );
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Updates an IAM policy for the specified object.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Tests a set of permissions on the given object to see which, if any, are held by the caller.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an object\'s metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch_all")
                .about("Watch for changes on all objects in a bucket.");
            objects0 = objects0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: hmac_keys and service_account");
        let mut hmac_keys1 = SubCommand::with_name("hmac_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new HMAC key for the specified service account.");
            hmac_keys1 = hmac_keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an HMAC key.");
            hmac_keys1 = hmac_keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an HMAC key\'s metadata");
            hmac_keys1 = hmac_keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of HMAC keys matching the criteria.");
            hmac_keys1 = hmac_keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states.");
            hmac_keys1 = hmac_keys1.subcommand(mcmd);
        }
        let mut service_account1 = SubCommand::with_name("service_account")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Get the email address of this project\'s Google Cloud Storage service account.",
            );
            service_account1 = service_account1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(service_account1);
        projects0 = projects0.subcommand(hmac_keys1);
        app = app.subcommand(projects0);
        app = app.subcommand(objects0);
        app = app.subcommand(object_access_controls0);
        app = app.subcommand(notifications0);
        app = app.subcommand(default_object_access_controls0);
        app = app.subcommand(channels0);
        app = app.subcommand(buckets0);
        app = app.subcommand(bucket_access_controls0);

        Self { app }
    }
}
use google_storage1 as api;

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
