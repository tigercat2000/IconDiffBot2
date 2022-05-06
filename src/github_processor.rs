use crate::github_types::{ModifiedFile, PullRequestEventPayload};
// use dmm_tools::dmi::IconFile;
use rocket::{
    http::Status,
    post,
    request::{FromRequest, Outcome},
    Request,
};

use crate::github_api::get_pull_files;

#[derive(Debug)]
pub struct GithubEvent(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GithubEvent {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-Github-Event") {
            Some(event) => Outcome::Success(GithubEvent(event.to_owned())),
            None => Outcome::Failure((Status::BadRequest, "Missing X-Github-Event header")),
        }
    }
}

#[post("/payload", format = "json", data = "<payload>")]
pub async fn process_github_payload(
    event: GithubEvent,
    payload: String,
) -> Result<&'static str, String> {
    if event.0 != "pull_request" {
        return Ok("Not a pull request event");
    }

    let payload: PullRequestEventPayload =
        serde_json::from_str(&payload).map_err(|e| format!("{e}"))?;

    let files = get_pull_files(&payload.installation, &payload.pull_request).await?;

    let changed_dmis: Vec<ModifiedFile> = files
        .into_iter()
        .filter(|e| e.filename.ends_with(".dmi"))
        .collect();

    if changed_dmis.is_empty() {
        return Ok("");
    }

    Ok("")
}
