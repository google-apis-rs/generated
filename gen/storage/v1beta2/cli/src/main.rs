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
        let mut app = App::new("storage1_beta2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200430")
            .about("Lets you store and retrieve potentially-large, immutable data objects.")
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
            let mcmd = SubCommand::with_name("patch").about("Updates an ACL entry on the specified bucket. This method supports patch semantics.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an ACL entry on the specified bucket.");
            bucket_access_controls0 = bucket_access_controls0.subcommand(mcmd);
        }
        let mut buckets0 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
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
            let mcmd = SubCommand::with_name("insert").about("Creates a new bucket.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of buckets for a given project.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a bucket. This method supports patch semantics.");
            buckets0 = buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a bucket.");
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
            let mcmd = SubCommand::with_name("patch").about("Updates a default object ACL entry on the specified bucket. This method supports patch semantics.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a default object ACL entry on the specified bucket.");
            default_object_access_controls0 = default_object_access_controls0.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("patch").about("Updates an ACL entry on the specified object. This method supports patch semantics.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an ACL entry on the specified object.");
            object_access_controls0 = object_access_controls0.subcommand(mcmd);
        }
        let mut objects0 = SubCommand::with_name("objects")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: compose, copy, delete, get, insert, list, patch, update and watch_all",
            );
        {
            let mcmd = SubCommand::with_name("compose").about(
                "Concatenates a list of existing objects into a new object in the same bucket.",
            );
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("copy").about("Copies an object to a destination in the same location. Optionally overrides metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes data blobs and associated metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves objects or their associated metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Stores new data blobs and associated metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of objects matching the criteria.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates a data blob\'s associated metadata. This method supports patch semantics.",
            );
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a data blob\'s associated metadata.");
            objects0 = objects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch_all")
                .about("Watch for changes on all objects in a bucket.");
            objects0 = objects0.subcommand(mcmd);
        }
        app = app.subcommand(objects0);
        app = app.subcommand(object_access_controls0);
        app = app.subcommand(default_object_access_controls0);
        app = app.subcommand(channels0);
        app = app.subcommand(buckets0);
        app = app.subcommand(bucket_access_controls0);

        Self { app }
    }
}
use google_storage1_beta2 as api;

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
