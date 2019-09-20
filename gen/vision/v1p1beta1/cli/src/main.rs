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
        let mut app = App::new("vision1_p1beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190913")
            .about("Integrates Google Vision features, including image labeling, face, logo, and landmark detection, optical character recognition (OCR), and detection of explicit content, into applications.")
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
        let mut files0 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
            files0 = files0.subcommand(mcmd);
        }
        let mut images0 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate")
                .about("Run image detection and annotation for a batch of images.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images0 = images0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: files, images and locations");
        let mut files1 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted.");
            files1 = files1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
            files1 = files1.subcommand(mcmd);
        }
        let mut images1 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate")
                .about("Run image detection and annotation for a batch of images.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images1 = images1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: files and images");
        let mut files2 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted.");
            files2 = files2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
            files2 = files2.subcommand(mcmd);
        }
        let mut images2 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate")
                .about("Run image detection and annotation for a batch of images.");
            images2 = images2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images2 = images2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(images2);
        locations1 = locations1.subcommand(files2);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(images1);
        projects0 = projects0.subcommand(files1);
        app = app.subcommand(projects0);
        app = app.subcommand(images0);
        app = app.subcommand(files0);

        Self { app }
    }
}
use google_vision1_p1beta1 as api;

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
