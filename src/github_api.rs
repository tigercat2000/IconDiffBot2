use crate::github_types::{Installation, ModifiedFile, PullRequest};

pub async fn get_pull_files(
    installation: &Installation,
    pull: &PullRequest,
) -> Result<Vec<ModifiedFile>, String> {
    let (user, repo) = pull.base.repo.full_name();
    let res = octocrab::instance()
        .installation(installation.id.into())
        .get(
            &format!(
                "/repos/{user}/{repo}/pulls/{pull_number}/files",
                user = user,
                repo = repo,
                pull_number = pull.number
            ),
            None::<&()>,
        )
        .await
        .map_err(|e| format!("{e}"))?;

    Ok(res)
}
