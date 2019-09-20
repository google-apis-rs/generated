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
        let mut app = App::new("androidpublisher2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190917")
            .about("Accesses Android application developers\' Google Play accounts.")
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
            let mcmd = SubCommand::with_name("commit")
                .about("Commits/applies the changes made in this edit back to the app.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an edit for an app. Creating a new edit will automatically delete any of your previous edits so this method need only be called if you want to preemptively abandon an edit.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about the edit specified. Calls will fail if the edit is no long active (e.g. has been deleted, superseded or expired).");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a new edit for an app, populated with the app\'s current state.");
            edits0 = edits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Checks that the edit can be successfully committed. The edit\'s changes are not applied to the live app.");
            edits0 = edits0.subcommand(mcmd);
        }
        let mut inappproducts0 = SubCommand::with_name("inappproducts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete an in-app product for an app.");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about the in-app product specified.");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new in-app product for an app.");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all the in-app products for an Android app, both subscriptions and managed in-app products..");
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates the details of an in-app product. This method supports patch semantics.",
            );
            inappproducts0 = inappproducts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates the details of an in-app product.");
            inappproducts0 = inappproducts0.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("get").about("Returns a single review.");
            reviews0 = reviews0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of reviews. Only reviews from last week will be returned.");
            reviews0 = reviews0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reply")
                .about("Reply to a single review, or update an existing reply.");
            reviews0 = reviews0.subcommand(mcmd);
        }
        let mut apklistings1 = SubCommand::with_name("apklistings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, deleteall, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes the APK-specific localized listing for a specified APK and language code.",
            );
            apklistings1 = apklistings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deleteall")
                .about("Deletes all the APK-specific localized listings for a specified APK.");
            apklistings1 = apklistings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Fetches the APK-specific localized listing for a specified APK and language code.",
            );
            apklistings1 = apklistings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the APK-specific localized listings for a specified APK.");
            apklistings1 = apklistings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates or creates the APK-specific localized listing for a specified APK and language code. This method supports patch semantics.");
            apklistings1 = apklistings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates or creates the APK-specific localized listing for a specified APK and language code.");
            apklistings1 = apklistings1.subcommand(mcmd);
        }
        let mut apks1 = SubCommand::with_name("apks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: addexternallyhosted, list and upload");
        {
            let mcmd = SubCommand::with_name("addexternallyhosted").about("Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to enterprises using Google Play for Work whose application is configured to restrict distribution to the enterprise domain.");
            apks1 = apks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list");
            apks1 = apks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload");
            apks1 = apks1.subcommand(mcmd);
        }
        let mut bundles1 = SubCommand::with_name("bundles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and upload");
        {
            let mcmd = SubCommand::with_name("list");
            bundles1 = bundles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See: https://developers.google.com/api-client-library/java/google-api-java-client/errors for an example in java.");
            bundles1 = bundles1.subcommand(mcmd);
        }
        let mut deobfuscationfiles1 = SubCommand::with_name("deobfuscationfiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: upload");
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads the deobfuscation file of the specified APK. If a deobfuscation file already exists, it will be replaced.");
            deobfuscationfiles1 = deobfuscationfiles1.subcommand(mcmd);
        }
        let mut details1 = SubCommand::with_name("details")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Fetches app details for this edit. This includes the default language and developer support contact information.");
            details1 = details1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates app details for this edit. This method supports patch semantics.");
            details1 = details1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates app details for this edit.");
            details1 = details1.subcommand(mcmd);
        }
        let mut expansionfiles1 = SubCommand::with_name("expansionfiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch, update and upload");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetches the Expansion File configuration for the APK specified.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the APK\'s Expansion File configuration to reference another APK\'s Expansion Files. To add a new Expansion File use the Upload method. This method supports patch semantics.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the APK\'s Expansion File configuration to reference another APK\'s Expansion Files. To add a new Expansion File use the Upload method.");
            expansionfiles1 = expansionfiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload")
                .about("Uploads and attaches a new Expansion File to the APK specified.");
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
            let mcmd = SubCommand::with_name("deleteall")
                .about("Deletes all images for the specified language and image type.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all images for the specified language and image type.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads a new image and adds it to the list of images for the specified language and image type.");
            images1 = images1.subcommand(mcmd);
        }
        let mut listings1 = SubCommand::with_name("listings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, deleteall, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified localized store listing from an edit.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deleteall")
                .about("Deletes all localized listings from an edit.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetches information about a localized store listing.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all of the localized store listings attached to this edit.");
            listings1 = listings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Creates or updates a localized store listing. This method supports patch semantics.");
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
            let mcmd = SubCommand::with_name("get");
            testers1 = testers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch");
            testers1 = testers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update");
            testers1 = testers1.subcommand(mcmd);
        }
        let mut tracks1 = SubCommand::with_name("tracks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Fetches the track configuration for the specified track type. Includes the APK version codes that are in this track.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the track configurations for this edit.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the track configuration for the specified track type. This method supports patch semantics.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the track configuration for the specified track type.");
            tracks1 = tracks1.subcommand(mcmd);
        }
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Checks the purchase and consumption status of an inapp item.");
            products1 = products1.subcommand(mcmd);
        }
        let mut subscriptions1 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, defer, get, refund and revoke");
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
        edits0 = edits0.subcommand(apklistings1);
        app = app.subcommand(reviews0);
        app = app.subcommand(purchases0);
        app = app.subcommand(orders0);
        app = app.subcommand(inappproducts0);
        app = app.subcommand(edits0);

        Self { app }
    }
}
use google_androidpublisher2 as api;

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
