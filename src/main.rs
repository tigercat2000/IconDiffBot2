#![allow(non_snake_case)]

mod github_types;

use std::{fs::File, io::Read, path::PathBuf};

use github_types::{Installation, ModifiedFile, PullRequest, PullRequestEventPayload};
use octocrab::OctocrabBuilder;
use once_cell::sync::OnceCell;
// use dmm_tools::dmi::IconFile;
use rocket::{
    figment::Figment,
    get,
    http::Status,
    launch, post,
    request::{self, FromRequest, Outcome},
    routes, Request,
};
use serde::Deserialize;

#[get("/")]
async fn index() -> &'static str {
    "IDB says hello!"
}

#[derive(Debug)]
struct GithubEvent(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GithubEvent {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("X-Github-Event") {
            Some(event) => Outcome::Success(GithubEvent(event.to_owned())),
            None => Outcome::Failure((Status::BadRequest, "Missing X-Github-Event header")),
        }
    }
}

pub async fn get_pull_files(
    installation: &Installation,
    pull: &PullRequest,
) -> Result<Vec<ModifiedFile>, String> {
    let res = octocrab::instance()
        .installation(installation.id.into())
        .get(
            &format!(
                "/repos/{repo}/pulls/{pull_number}/files",
                repo = pull.base.repo.full_name(),
                pull_number = pull.number
            ),
            None::<&()>,
        )
        .await
        .map_err(|e| format!("{e}"))?;

    Ok(res)
}

#[post("/payload", format = "json", data = "<payload>")]
async fn process_github_payload(
    event: GithubEvent,
    payload: String,
) -> Result<&'static str, String> {
    if event.0 != "pull_request" {
        return Ok("Not a pull request event");
    }

    let payload: PullRequestEventPayload =
        serde_json::from_str(&payload).map_err(|e| format!("{e}"))?;

    let files = get_pull_files(&payload.installation, &payload.pull_request).await?;

    let changed_dmis: Vec<&ModifiedFile> = files
        .iter()
        .filter(|e| e.filename.ends_with(".dmi"))
        .collect();

    if changed_dmis.is_empty() {
        return Ok("");
    }

    dbg!(changed_dmis);

    Ok("")
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub private_key_path: String,
    pub app_id: u64,
    // pub blacklist: Vec<u64>,
    // pub blacklist_contact: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

fn init_config(figment: &Figment) -> &Config {
    let config: Config = figment
        .extract()
        .expect("Missing config values in Rocket.toml");

    CONFIG.set(config).expect("Failed to set config");
    CONFIG.get().unwrap()
}

fn read_key(path: PathBuf) -> Vec<u8> {
    let mut key_file =
        File::open(&path).unwrap_or_else(|_| panic!("Unable to find file {}", path.display()));

    let mut key = Vec::new();
    let _ = key_file
        .read_to_end(&mut key)
        .unwrap_or_else(|_| panic!("Failed to read key {}", path.display()));

    key
}

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let config = init_config(rocket.figment());

    let key = read_key(PathBuf::from(&config.private_key_path));

    octocrab::initialise(OctocrabBuilder::new().app(
        config.app_id.into(),
        jsonwebtoken::EncodingKey::from_rsa_pem(&key).unwrap(),
    ))
    .expect("Octocrab failed to initialise");

    rocket.mount("/", routes![index, process_github_payload])
}
