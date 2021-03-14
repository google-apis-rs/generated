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
        let mut app = App::new("androidpublisher3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210311")
            .about("Lets Android application developers access their Google Play accounts.")
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
        let mut edits0 = SubCommand::with_name("edits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: commit, delete, get, insert and validate");
        {
            let mcmd = SubCommand::with_name("commit").about("Commits an app edit.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an app edit.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an app edit.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new edit for an app.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Validates an app edit.");
            edits0 = edits0.subcommand(mcmd);
        }
        let mut inappproducts0 = SubCommand::with_name("inappproducts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an in-app product (i.e. a managed product or a subscriptions).");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an in-app product, which can be a managed product or a subscription.");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates an in-app product (i.e. a managed product or a subscriptions).");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all in-app products - both managed products and subscriptions.");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patches an in-app product (i.e. a managed product or a subscriptions).");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an in-app product (i.e. a managed product or a subscriptions).");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        let mut internalappsharingartifacts0 = SubCommand::with_name("internalappsharingartifacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: uploadapk and uploadbundle");
        {
            let mcmd = SubCommand::with_name("uploadapk").about("Uploads an APK to internal app sharing. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.");
            internalappsharingartifacts0 = internalappsharingartifacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("uploadbundle").about("Uploads an app bundle to internal app sharing. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.");
            internalappsharingartifacts0 = internalappsharingartifacts0.subcommand(mcmd);
        }
        let mut orders0 = SubCommand::with_name("orders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: refund");
        {
            let mcmd = SubCommand::with_name("refund")
                .about("Refund a user\'s subscription or in-app purchase order.");
            orders0 = orders0.subcommand(mcmd);
        }
        let mut purchases0 = SubCommand::with_name("purchases")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: products, subscriptions and voidedpurchases");
        let mut reviews0 = SubCommand::with_name("reviews")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and reply");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single review.");
            reviews0 = reviews0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all reviews.");
            reviews0 = reviews0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reply")
                .about("Replies to a single review, or updates an existing reply.");
            reviews0 = reviews0.subcommand(mcmd);
        }
        let mut systemapks0 = SubCommand::with_name("systemapks")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: variants");
        let mut apks1 = SubCommand::with_name("apks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: addexternallyhosted, list and upload");
        {
            let mcmd = SubCommand::with_name("addexternallyhosted").about("Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to organizations using Managed Play whose application is configured to restrict distribution to the organizations.");
            apks1 = apks1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all current APKs of the app and edit.");
            apks1 = apks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload")
                .about("Uploads an APK and adds to the current edit.");
            apks1 = apks1.subcommand(mcmd);
        }
        let mut bundles1 = SubCommand::with_name("bundles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and upload");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all current Android App Bundles of the app and edit.");
            bundles1 = bundles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.");
            bundles1 = bundles1.subcommand(mcmd);
        }
        let mut deobfuscationfiles1 = SubCommand::with_name("deobfuscationfiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: upload");
        {
            let mcmd = SubCommand::with_name("upload")
                .about("Uploads a new deobfuscation file and attaches to the specified APK.");
            deobfuscationfiles1 = deobfuscationfiles1.subcommand(mcmd);
        }
        let mut details1 = SubCommand::with_name("details")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of an app.");
            details1 = details1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches details of an app.");
            details1 = details1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates details of an app.");
            details1 = details1.subcommand(mcmd);
        }
        let mut expansionfiles1 = SubCommand::with_name("expansionfiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch, update and upload");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetches the expansion file configuration for the specified APK.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the APK\'s expansion file configuration to reference another APK\'s expansion file. To add a new expansion file use the Upload method.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the APK\'s expansion file configuration to reference another APK\'s expansion file. To add a new expansion file use the Upload method.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload")
                .about("Uploads a new expansion file and attaches to the specified APK.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        let mut images1 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, deleteall, list and upload");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the image (specified by id) from the edit.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deleteall").about("Deletes all images for the specified language and image type. Returns an empty response if no images are found.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all images. The response may be empty.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about(
                "Uploads an image of the specified language and image type, and adds to the edit.",
            );
            images1 = images1.subcommand(mcmd);
        }
        let mut listings1 = SubCommand::with_name("listings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, deleteall, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a localized store listing.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deleteall").about("Deletes all store listings.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a localized store listing.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all localized store listings.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a localized store listing.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Creates or updates a localized store listing.");
            listings1 = listings1.subcommand(mcmd);
        }
        let mut testers1 = SubCommand::with_name("testers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets testers.");
            testers1 = testers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches testers.");
            testers1 = testers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates testers.");
            testers1 = testers1.subcommand(mcmd);
        }
        let mut tracks1 = SubCommand::with_name("tracks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a track.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all tracks.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a track.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a track.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: acknowledge and get");
        {
            let mcmd = SubCommand::with_name("acknowledge")
                .about("Acknowledges a purchase of an inapp item.");
            products1 = products1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Checks the purchase and consumption status of an inapp item.");
            products1 = products1.subcommand(mcmd);
        }
        let mut subscriptions1 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: acknowledge, cancel, defer, get, refund and revoke");
        {
            let mcmd =
                SubCommand::with_name("acknowledge").about("Acknowledges a subscription purchase.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a user\'s subscription purchase. The subscription remains valid until its expiration time.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("defer").about(
                "Defers a user\'s subscription purchase until a specified future expiration time.",
            );
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Checks whether a user\'s subscription purchase is valid and returns its expiry time.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("refund").about("Refunds a user\'s subscription purchase, but the subscription remains valid until its expiration time and it will continue to recur.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revoke").about("Refunds and immediately revokes a user\'s subscription purchase. Access to the subscription will be terminated immediately and it will stop recurring.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        let mut voidedpurchases1 = SubCommand::with_name("voidedpurchases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the purchases that were canceled, refunded or charged-back.");
            voidedpurchases1 = voidedpurchases1.subcommand(mcmd);
        }
        let mut variants1 = SubCommand::with_name("variants")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, download, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an APK which is suitable for inclusion in a system image from an already uploaded Android App Bundle.");
            variants1 = variants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("download").about("Downloads a previously created system APK which is suitable for inclusion in a system image.");
            variants1 = variants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a previously created system APK variant.");
            variants1 = variants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of previously created system APK variants.");
            variants1 = variants1.subcommand(mcmd);
        }
        systemapks0 = systemapks0.subcommand(variants1);
        purchases0 = purchases0.subcommand(voidedpurchases1);
        purchases0 = purchases0.subcommand(subscriptions1);
        purchases0 = purchases0.subcommand(products1);
        edits0 = edits0.subcommand(tracks1);
        edits0 = edits0.subcommand(testers1);
        edits0 = edits0.subcommand(listings1);
        edits0 = edits0.subcommand(images1);
        edits0 = edits0.subcommand(expansionfiles1);
        edits0 = edits0.subcommand(details1);
        edits0 = edits0.subcommand(deobfuscationfiles1);
        edits0 = edits0.subcommand(bundles1);
        edits0 = edits0.subcommand(apks1);
        app = app.subcommand(systemapks0);
        app = app.subcommand(reviews0);
        app = app.subcommand(purchases0);
        app = app.subcommand(orders0);
        app = app.subcommand(internalappsharingartifacts0);
        app = app.subcommand(inappproducts0);
        app = app.subcommand(edits0);

        Self { app }
    }
}
use google_androidpublisher3 as api;

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
