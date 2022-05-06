#[derive(Debug)]
pub enum Error {
    Octocrab(octocrab::Error),
    TokioIO(rocket::tokio::io::Error),
    DirectoryGivenToDownloadFile,
    DecodeContentFailed,
}

impl From<octocrab::Error> for Error {
    fn from(e: octocrab::Error) -> Self {
        Self::Octocrab(e)
    }
}

impl From<rocket::tokio::io::Error> for Error {
    fn from(e: rocket::tokio::io::Error) -> Self {
        Self::TokioIO(e)
    }
}
