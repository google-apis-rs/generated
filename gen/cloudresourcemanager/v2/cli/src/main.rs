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
        let mut app = App::new("cloudresourcemanager2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200624")
            .about("Creates, reads, and updates metadata for Google Cloud Platform resource containers.")
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
        let mut folders0 = SubCommand::with_name("folders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, r#move, patch, search, set_iam_policy, test_iam_permissions and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Folder in the resource hierarchy.\nReturns an Operation which can be used to track the progress of the\nfolder creation workflow.\nUpon success the Operation.response field will be populated with the\ncreated Folder.\n\nIn order to succeed, the addition of this new Folder must not violate\nthe Folder naming, height or fanout constraints.\n\n+ The Folder\'s display_name must be distinct from all other Folder\'s that\nshare its parent.\n+ The addition of the Folder must not cause the active Folder hierarchy\nto exceed a height of 4. Note, the full active + deleted Folder hierarchy\nis allowed to reach a height of 8; this provides additional headroom when\nmoving folders that contain deleted folders.\n+ The addition of the Folder must not cause the total number of Folders\nunder its parent to exceed 100.\n\nIf the operation fails due to a folder constraint violation, some errors\nmay be returned by the CreateFolder request, with status code\nFAILED_PRECONDITION and an error description. Other folder constraint\nviolations will be communicated in the Operation, with the specific\nPreconditionFailure returned via the details list in the Operation.error\nfield.\n\nThe caller must have `resourcemanager.folders.create` permission on the\nidentified parent.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Requests deletion of a Folder. The Folder is moved into the\nDELETE_REQUESTED state\nimmediately, and is deleted approximately 30 days later. This method may\nonly be called on an empty Folder in the\nACTIVE state, where a Folder is empty if\nit doesn\'t contain any Folders or Projects in the\nACTIVE state.\nThe caller must have `resourcemanager.folders.delete` permission on the\nidentified folder.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a Folder identified by the supplied resource name.\nValid Folder resource names have the format `folders/{folder_id}`\n(for example, `folders/1234`).\nThe caller must have `resourcemanager.folders.get` permission on the\nidentified folder.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a Folder. The returned policy may be\nempty if no such policy or resource exists. The `resource` field should\nbe the Folder\'s resource name, e.g. \"folders/1234\".\nThe caller must have `resourcemanager.folders.getIamPolicy` permission\non the identified folder.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Folders that are direct descendants of supplied parent resource.\nList provides a strongly consistent view of the Folders underneath\nthe specified parent resource.\nList returns Folders sorted based upon the (ascending) lexical ordering\nof their display_name.\nThe caller must have `resourcemanager.folders.list` permission on the\nidentified parent.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move").about("Moves a Folder under a new resource parent.\nReturns an Operation which can be used to track the progress of the\nfolder move workflow.\nUpon success the Operation.response field will be populated with the\nmoved Folder.\nUpon failure, a FolderOperationError categorizing the failure cause will\nbe returned - if the failure occurs synchronously then the\nFolderOperationError will be returned via the Status.details field\nand if it occurs asynchronously then the FolderOperation will be returned\nvia the Operation.error field.\nIn addition, the Operation.metadata field will be populated with a\nFolderOperation message as an aid to stateless clients.\nFolder moves will be rejected if they violate either the naming, height\nor fanout constraints described in the\nCreateFolder documentation.\nThe caller must have `resourcemanager.folders.move` permission on the\nfolder\'s current and proposed new parent.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Folder, changing its display_name.\nChanges to the folder display_name will be rejected if they violate either\nthe display_name formatting rules or naming constraints described in\nthe CreateFolder documentation.\n\nThe Folder\'s display name must start and end with a letter or digit,\nmay contain letters, digits, spaces, hyphens and underscores and can be\nno longer than 30 characters. This is captured by the regular expression:\n[\\p{L}\\p{N}]([\\p{L}\\p{N}_- ]{0,28}[\\p{L}\\p{N}])?.\nThe caller must have `resourcemanager.folders.update` permission on the\nidentified folder.\n\nIf the update fails due to the unique name constraint then a\nPreconditionFailure explaining this violation will be returned\nin the Status.details field.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Search for folders that match specific filter criteria.\nSearch provides an eventually consistent view of the folders a user has\naccess to which meet the specified filter criteria.\n\nThis will only return folders on which the caller has the\npermission `resourcemanager.folders.get`.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on a Folder, replacing any existing policy.\nThe `resource` field should be the Folder\'s resource name, e.g.\n\"folders/1234\".\nThe caller must have `resourcemanager.folders.setIamPolicy` permission\non the identified folder.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified Folder.\nThe `resource` field should be the Folder\'s resource name,\ne.g. \"folders/1234\".\n\nThere are no permissions required for making this API call.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Cancels the deletion request for a Folder. This method may only be\ncalled on a Folder in the\nDELETE_REQUESTED state.\nIn order to succeed, the Folder\'s parent must be in the\nACTIVE state.\nIn addition, reintroducing the folder into the tree must not violate\nfolder naming, height and fanout constraints described in the\nCreateFolder documentation.\nThe caller must have `resourcemanager.folders.undelete` permission on the\nidentified folder.");
            folders0 = folders0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        app = app.subcommand(operations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_cloudresourcemanager2 as api;

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
