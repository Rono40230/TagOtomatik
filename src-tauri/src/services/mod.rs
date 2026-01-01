pub mod audio;
pub mod cleaner;
pub mod converter;
pub mod cover;
pub mod dictionaries;
pub mod exception;
pub mod io;
pub mod player;
pub mod playlist;
pub mod processor;
pub mod renamer;
pub mod scanner;

#[cfg(test)]
mod processor_tests;

pub use audio::AudioService;
pub use cleaner::CleanerService;
pub use exception::ExceptionService;
pub use io::IOService;
pub use processor::MetadataProcessorService;
pub use renamer::RenamerService;
pub use scanner::ScannerService;
