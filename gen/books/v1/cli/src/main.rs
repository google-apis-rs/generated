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
        let mut app = App::new("books1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200506")
            .about("The Google Books API allows clients to access the Google Books repository.")
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
        let mut bookshelves0 = SubCommand::with_name("bookshelves")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves metadata for a specific bookshelf for the specified user.");
            bookshelves0 = bookshelves0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of public bookshelves for the specified user.");
            bookshelves0 = bookshelves0.subcommand(mcmd);
        }
        let mut cloudloading0 = SubCommand::with_name("cloudloading")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add_book, delete_book and update_book");
        {
            let mcmd = SubCommand::with_name("add_book")
                .about("Add a user-upload volume and triggers processing.");
            cloudloading0 = cloudloading0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete_book").about("Remove the book and its contents");
            cloudloading0 = cloudloading0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_book").about("Updates a user-upload volume.");
            cloudloading0 = cloudloading0.subcommand(mcmd);
        }
        let mut dictionary0 = SubCommand::with_name("dictionary")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list_offline_metadata");
        {
            let mcmd = SubCommand::with_name("list_offline_metadata")
                .about("Returns a list of offline dictionary metadata available");
            dictionary0 = dictionary0.subcommand(mcmd);
        }
        let mut familysharing0 = SubCommand::with_name("familysharing")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_family_info, share and unshare");
        {
            let mcmd = SubCommand::with_name("get_family_info")
                .about("Gets information regarding the family that the user is part of.");
            familysharing0 = familysharing0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("share").about("Initiates sharing of the content with the user\'s family. Empty response\nindicates success.");
            familysharing0 = familysharing0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unshare").about("Initiates revoking content that has already been shared with the user\'s\nfamily. Empty response indicates success.");
            familysharing0 = familysharing0.subcommand(mcmd);
        }
        let mut layers0 = SubCommand::with_name("layers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the layer summary for a volume.");
            layers0 = layers0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List the layer summaries for a volume.");
            layers0 = layers0.subcommand(mcmd);
        }
        let mut myconfig0 = SubCommand::with_name("myconfig")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_user_settings, release_download_access, request_access, sync_volume_licenses and update_user_settings");
        {
            let mcmd = SubCommand::with_name("get_user_settings")
                .about("Gets the current settings for the user.");
            myconfig0 = myconfig0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("release_download_access")
                .about("Release downloaded content access restriction.");
            myconfig0 = myconfig0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("request_access")
                .about("Request concurrent and download access restrictions.");
            myconfig0 = myconfig0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sync_volume_licenses").about(
                "Request downloaded content access for specified volumes on the My eBooks\nshelf.",
            );
            myconfig0 = myconfig0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_user_settings").about("Sets the settings for the user. If a sub-object is specified, it will\noverwrite the existing sub-object stored in the server. Unspecified\nsub-objects will retain the existing value.");
            myconfig0 = myconfig0.subcommand(mcmd);
        }
        let mut mylibrary0 = SubCommand::with_name("mylibrary")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: annotations, bookshelves and readingpositions");
        let mut notification0 = SubCommand::with_name("notification")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns notification details for a given notification id.");
            notification0 = notification0.subcommand(mcmd);
        }
        let mut onboarding0 = SubCommand::with_name("onboarding")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list_categories and list_category_volumes");
        {
            let mcmd = SubCommand::with_name("list_categories")
                .about("List categories for onboarding experience.");
            onboarding0 = onboarding0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_category_volumes")
                .about("List available volumes under categories for onboarding experience.");
            onboarding0 = onboarding0.subcommand(mcmd);
        }
        let mut personalizedstream0 = SubCommand::with_name("personalizedstream")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a stream of personalized book clusters");
            personalizedstream0 = personalizedstream0.subcommand(mcmd);
        }
        let mut promooffer0 = SubCommand::with_name("promooffer")
            .setting(AppSettings::ColoredHelp)
            .about("methods: accept, dismiss and get");
        {
            let mcmd = SubCommand::with_name("accept").about("Accepts the promo offer.");
            promooffer0 = promooffer0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("dismiss").about("Marks the promo offer as dismissed.");
            promooffer0 = promooffer0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a list of promo offers available to the user");
            promooffer0 = promooffer0.subcommand(mcmd);
        }
        let mut series0 = SubCommand::with_name("series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns Series metadata for the given series ids.");
            series0 = series0.subcommand(mcmd);
        }
        let mut volumes0 = SubCommand::with_name("volumes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets volume information for a single volume.");
            volumes0 = volumes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Performs a book search.");
            volumes0 = volumes0.subcommand(mcmd);
        }
        let mut volumes1 = SubCommand::with_name("volumes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves volumes in a specific bookshelf for the specified user.");
            volumes1 = volumes1.subcommand(mcmd);
        }
        let mut annotation_data1 = SubCommand::with_name("annotation_data")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the annotation data.");
            annotation_data1 = annotation_data1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Gets the annotation data for a volume and layer.");
            annotation_data1 = annotation_data1.subcommand(mcmd);
        }
        let mut volume_annotations1 = SubCommand::with_name("volume_annotations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the volume annotation.");
            volume_annotations1 = volume_annotations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Gets the volume annotations for a volume and layer.");
            volume_annotations1 = volume_annotations1.subcommand(mcmd);
        }
        let mut annotations1 = SubCommand::with_name("annotations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list, summary and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an annotation.");
            annotations1 = annotations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new annotation.");
            annotations1 = annotations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of annotations, possibly filtered.");
            annotations1 = annotations1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("summary").about("Gets the summary of specified layers.");
            annotations1 = annotations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing annotation.");
            annotations1 = annotations1.subcommand(mcmd);
        }
        let mut bookshelves1 = SubCommand::with_name("bookshelves")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add_volume, clear_volumes, get, list, move_volume and remove_volume");
        {
            let mcmd = SubCommand::with_name("add_volume").about("Adds a volume to a bookshelf.");
            bookshelves1 = bookshelves1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("clear_volumes")
                .about("Clears all volumes from a bookshelf.");
            bookshelves1 = bookshelves1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieves metadata for a specific bookshelf belonging to the authenticated\nuser.",
            );
            bookshelves1 = bookshelves1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of bookshelves belonging to the authenticated user.");
            bookshelves1 = bookshelves1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("move_volume").about("Moves a volume within a bookshelf.");
            bookshelves1 = bookshelves1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("remove_volume").about("Removes a volume from a bookshelf.");
            bookshelves1 = bookshelves1.subcommand(mcmd);
        }
        let mut readingpositions1 = SubCommand::with_name("readingpositions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and set_position");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves my reading position information for a volume.");
            readingpositions1 = readingpositions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_position")
                .about("Sets my reading position information for a volume.");
            readingpositions1 = readingpositions1.subcommand(mcmd);
        }
        let mut membership1 = SubCommand::with_name("membership")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns Series membership data given the series id.");
            membership1 = membership1.subcommand(mcmd);
        }
        let mut associated1 = SubCommand::with_name("associated")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Return a list of associated books.");
            associated1 = associated1.subcommand(mcmd);
        }
        let mut mybooks1 = SubCommand::with_name("mybooks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Return a list of books in My Library.");
            mybooks1 = mybooks1.subcommand(mcmd);
        }
        let mut recommended1 = SubCommand::with_name("recommended")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and rate");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Return a list of recommended books for the current user.");
            recommended1 = recommended1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rate")
                .about("Rate a recommended book for the current user.");
            recommended1 = recommended1.subcommand(mcmd);
        }
        let mut useruploaded1 = SubCommand::with_name("useruploaded")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Return a list of books uploaded by the current user.");
            useruploaded1 = useruploaded1.subcommand(mcmd);
        }
        let mut volumes2 = SubCommand::with_name("volumes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Gets volume information for volumes on a bookshelf.");
            volumes2 = volumes2.subcommand(mcmd);
        }
        bookshelves1 = bookshelves1.subcommand(volumes2);
        volumes0 = volumes0.subcommand(useruploaded1);
        volumes0 = volumes0.subcommand(recommended1);
        volumes0 = volumes0.subcommand(mybooks1);
        volumes0 = volumes0.subcommand(associated1);
        series0 = series0.subcommand(membership1);
        mylibrary0 = mylibrary0.subcommand(readingpositions1);
        mylibrary0 = mylibrary0.subcommand(bookshelves1);
        mylibrary0 = mylibrary0.subcommand(annotations1);
        layers0 = layers0.subcommand(volume_annotations1);
        layers0 = layers0.subcommand(annotation_data1);
        bookshelves0 = bookshelves0.subcommand(volumes1);
        app = app.subcommand(volumes0);
        app = app.subcommand(series0);
        app = app.subcommand(promooffer0);
        app = app.subcommand(personalizedstream0);
        app = app.subcommand(onboarding0);
        app = app.subcommand(notification0);
        app = app.subcommand(mylibrary0);
        app = app.subcommand(myconfig0);
        app = app.subcommand(layers0);
        app = app.subcommand(familysharing0);
        app = app.subcommand(dictionary0);
        app = app.subcommand(cloudloading0);
        app = app.subcommand(bookshelves0);

        Self { app }
    }
}
use google_books1 as api;

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
