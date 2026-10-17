use std::{collections::HashMap, io::Read};

use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use flate2::read::GzDecoder;
use rust_embed::RustEmbed;
use tracing::{debug, error};
use typst::{
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime, Duration},
    text::{Font, FontBook},
    utils::LazyHash,
    Features, Library, LibraryExt, World,
    syntax::{FileId, Source, VirtualRoot},
};
use typst_kit::{
    downloader::SystemDownloader,
    files::{FileLoader, FileStore, FsRoot},
    packages::SystemPackages,
};

#[derive(RustEmbed)]
#[folder = "templates/typst/"]
pub(crate) struct TypstTemplates;

pub struct ReportGenerator {
    pub(crate) library: LazyHash<Library>,
    pub(crate) fontbook: FontBook,
    pub(crate) sources: Vec<Source>,
    pub(crate) fonts: Vec<typst::text::Font>,
    packages: FileStore<EmbeddedTypstPackages>,
}

#[derive(Default)]
struct EmbeddedTypstPackages {
    packages: HashMap<String, HashMap<String, Bytes>>,
}

impl EmbeddedTypstPackages {
    fn new() -> Self {
        let mut packages = HashMap::new();

        for file in TypstTemplates::iter().filter(|path| path.ends_with(".tar.gz")) {
            let Some(template) = TypstTemplates::get(&file) else {
                continue;
            };

            let archive_name = file.rsplit('/').next().unwrap_or(&file);
            let Some((namespace, package_name)) = archive_name
                .strip_prefix('@')
                .and_then(|name| name.split_once('-'))
            else {
                continue;
            };
            let package_name = package_name.strip_suffix(".tar.gz").unwrap_or(package_name);
            let Some((name, version)) = package_name.rsplit_once('-') else {
                continue;
            };

            let mut package_files = HashMap::new();
            let mut archive = tar::Archive::new(GzDecoder::new(std::io::Cursor::new(
                template.data.to_vec(),
            )));

            let Ok(entries) = archive.entries() else {
                continue;
            };

            for entry in entries {
                let Ok(mut entry) = entry else {
                    continue;
                };

                let Ok(path) = entry.path() else {
                    continue;
                };
                let path = path.to_string_lossy().replace('\\', "/");

                if path.is_empty() || path.ends_with('/') || path == "." {
                    continue;
                }

                let mut bytes = Vec::new();
                if entry.read_to_end(&mut bytes).is_err() {
                    continue;
                }

                debug!("Adding {}", path);
                package_files.insert(
                    path.trim_start_matches("./").to_string(),
                    Bytes::new(bytes),
                );
            }
            packages.insert(format!("@{namespace}/{name}:{version}"), package_files);
        }

        Self { packages }
    }
}

impl FileLoader for EmbeddedTypstPackages {
    fn load(&self, id: FileId) -> FileResult<Bytes> {
        match id.root() {
            VirtualRoot::Project => FsRoot::new(std::env::current_dir().unwrap_or_default())
                .load(id.vpath()),
            VirtualRoot::Package(spec) => {
                let key = format!("@{}/{}:{}", spec.namespace, spec.name, spec.version);
                if let Some(file) = self
                    .packages
                    .get(&key)
                    .and_then(|files| files.get(id.vpath().get_without_slash()))
                    .cloned()
                {
                    return Ok(file);
                }

                let packages = SystemPackages::new(SystemDownloader::new("cucumber-rs-reporter"));
                packages
                    .obtain(spec)
                    .map(|root| root.load(id.vpath()))
                    .map_err(|_| {
                        FileError::NotFound(id.vpath().get_without_slash().to_string().into())
                    })
                    .and_then(|result| {
                        result.map_err(|_| {
                            FileError::NotFound(id.vpath().get_without_slash().to_string().into())
                        })
                    })
            }
        }
    }
}

impl ReportGenerator {
    pub(crate) fn typst_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<Datetime> {
        Datetime::from_ymd_hms(
            datetime.year(),
            datetime.month() as u8,
            datetime.day() as u8,
            datetime.hour() as u8,
            datetime.minute() as u8,
            datetime.second() as u8,
        )
    }

    pub fn new(inputs: typst::foundations::Dict) -> Self {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        typst_kit::fonts::embedded().for_each(|(font, info)| {
            book.push(info);
            fonts.push(font);
        });

        let features = Features::all();

        ReportGenerator {
            library: LazyHash::new(
                typst::Library::builder()
                    .with_features(features)
                    .with_inputs(inputs)
                    .build(),
            ),
            fontbook: book,
            sources: Vec::new(),
            fonts,
            packages: FileStore::new(EmbeddedTypstPackages::new()),
        }
    }
}

impl World for ReportGenerator {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        static LAZY_BOOK: std::sync::OnceLock<LazyHash<FontBook>> = std::sync::OnceLock::new();
        LAZY_BOOK.get_or_init(|| LazyHash::new(self.fontbook.clone()))
    }

    fn main(&self) -> FileId {
        self.sources.first().map(|source| source.id()).unwrap()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let source = self.sources.iter().find(|source| source.id() == id).cloned();
        debug!("Accessing source file with id {:?}: {:?}", id, source);
        match source {
            Some(source) => {
                debug!("Accessed source file with id {:?}: {:?}", id, source);
                FileResult::Ok(source)
            }
            None if matches!(id.vpath().extension(), Some("typ" | "toml")) => {
                self.packages.source(id)
            }
            None => FileResult::Err(FileError::NotFound(
                "Source file not found".to_string().into(),
            )),
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        match self.source(id) {
            FileResult::Ok(source) => FileResult::Ok(Bytes::from_string(source.text().to_string())),
            FileResult::Err(FileError::NotFound(error)) => {
                debug!("not found: {:?} for {:?}", error, id);
                self.packages.file(id).inspect_err(|error| error!("{:?}", error))
            }
            FileResult::Err(error) => FileResult::Err(error),
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        match offset {
            None => Self::typst_datetime(Local::now()),
            Some(offset) => {
                let seconds = offset.seconds();
                let utc = Utc::now()
                    + chrono::Duration::microseconds((seconds * 1_000_000.0) as i64);
                Self::typst_datetime(utc)
            }
        }
    }
}
