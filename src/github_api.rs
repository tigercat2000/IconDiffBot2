use crate::github_types::{Installation, ModifiedFile, PullRequest};

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
