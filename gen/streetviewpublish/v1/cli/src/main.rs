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
        let mut app = App::new("streetviewpublish1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200512")
            .about("Publishes 360 photos to Google Maps, along with position, orientation, and connectivity metadata. Apps can offer an interface for positioning, connecting, and uploading user-generated Street View images.\n")
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
        let mut photo0 = SubCommand::with_name("photo")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, start_upload and update");
        {
            let mcmd = SubCommand::with_name("create").about("After the client finishes uploading the photo with the returned\nUploadRef,\nCreatePhoto\npublishes the uploaded Photo to\nStreet View on Google Maps.\n\nCurrently, the only way to set heading, pitch, and roll in CreatePhoto is\nthrough the [Photo Sphere XMP\nmetadata](https://developers.google.com/streetview/spherical-metadata) in\nthe photo bytes. CreatePhoto ignores the  `pose.heading`, `pose.pitch`,\n`pose.roll`, `pose.altitude`, and `pose.level` fields in Pose.\n\nThis method returns the following error codes:\n\n* google.rpc.Code.INVALID_ARGUMENT if the request is malformed or if\nthe uploaded photo is not a 360 photo.\n* google.rpc.Code.NOT_FOUND if the upload reference does not exist.\n* google.rpc.Code.RESOURCE_EXHAUSTED if the account has reached the\nstorage limit.");
            photo0 = photo0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Photo and its metadata.\n\nThis method returns the following error codes:\n\n* google.rpc.Code.PERMISSION_DENIED if the requesting user did not\ncreate the requested photo.\n* google.rpc.Code.NOT_FOUND if the photo ID does not exist.");
            photo0 = photo0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the metadata of the specified\nPhoto.\n\nThis method returns the following error codes:\n\n* google.rpc.Code.PERMISSION_DENIED if the requesting user did not\ncreate the requested Photo.\n* google.rpc.Code.NOT_FOUND if the requested\nPhoto does not exist.\n* google.rpc.Code.UNAVAILABLE if the requested\nPhoto is still being indexed.");
            photo0 = photo0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_upload").about("Creates an upload session to start uploading photo bytes.  The method uses\nthe upload URL of the returned\nUploadRef to upload the bytes for\nthe Photo.\n\nIn addition to the photo requirements shown in\nhttps://support.google.com/maps/answer/7012050?hl=en&ref_topic=6275604,\nthe photo must meet the following requirements:\n\n* Photo Sphere XMP metadata must be included in the photo metadata. See\nhttps://developers.google.com/streetview/spherical-metadata for the\nrequired fields.\n* The pixel size of the photo must meet the size requirements listed in\nhttps://support.google.com/maps/answer/7012050?hl=en&ref_topic=6275604, and\nthe photo must be a full 360 horizontally.\n\nAfter the upload completes, the method uses\nUploadRef with\nCreatePhoto\nto create the Photo object entry.");
            photo0 = photo0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the metadata of a Photo, such\nas pose, place association, connections, etc. Changing the pixels of a\nphoto is not supported.\n\nOnly the fields specified in the\nupdateMask\nfield are used. If `updateMask` is not present, the update applies to all\nfields.\n\nThis method returns the following error codes:\n\n* google.rpc.Code.PERMISSION_DENIED if the requesting user did not\ncreate the requested photo.\n* google.rpc.Code.INVALID_ARGUMENT if the request is malformed.\n* google.rpc.Code.NOT_FOUND if the requested photo does not exist.\n* google.rpc.Code.UNAVAILABLE if the requested\nPhoto is still being indexed.");
            photo0 = photo0.subcommand(mcmd);
        }
        let mut photos0 = SubCommand::with_name("photos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_get, batch_update and list");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes a list of Photos and their\nmetadata.\n\nNote that if\nBatchDeletePhotos\nfails, either critical fields are missing or there is an authentication\nerror. Even if\nBatchDeletePhotos\nsucceeds, individual photos in the batch may have failures.\nThese failures are specified in each\nPhotoResponse.status\nin\nBatchDeletePhotosResponse.results.\nSee\nDeletePhoto\nfor specific failures that can occur per photo.");
            photos0 = photos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_get").about("Gets the metadata of the specified\nPhoto batch.\n\nNote that if\nBatchGetPhotos\nfails, either critical fields are missing or there is an authentication\nerror. Even if\nBatchGetPhotos\nsucceeds, individual photos in the batch may have failures.\nThese failures are specified in each\nPhotoResponse.status\nin\nBatchGetPhotosResponse.results.\nSee\nGetPhoto\nfor specific failures that can occur per photo.");
            photos0 = photos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates the metadata of Photos, such\nas pose, place association, connections, etc. Changing the pixels of photos\nis not supported.\n\nNote that if\nBatchUpdatePhotos\nfails, either critical fields are missing or there is an authentication\nerror. Even if\nBatchUpdatePhotos\nsucceeds, individual photos in the batch may have failures.\nThese failures are specified in each\nPhotoResponse.status\nin\nBatchUpdatePhotosResponse.results.\nSee\nUpdatePhoto\nfor specific failures that can occur per photo.\n\nOnly the fields specified in\nupdateMask\nfield are used. If `updateMask` is not present, the update applies to all\nfields.\n\nThe number of\nUpdatePhotoRequest\nmessages in a\nBatchUpdatePhotosRequest\nmust not exceed 20.\n\n<aside class=\"note\"><b>Note:</b> To update\nPose.altitude,\nPose.latLngPair has to be\nfilled as well. Otherwise, the request will fail.</aside>");
            photos0 = photos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the Photos that belong to\nthe user.\n\n<aside class=\"note\"><b>Note:</b> Recently created photos that are still\nbeing indexed are not returned in the response.</aside>");
            photos0 = photos0.subcommand(mcmd);
        }
        app = app.subcommand(photos0);
        app = app.subcommand(photo0);

        Self { app }
    }
}
use google_streetviewpublish1 as api;

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
