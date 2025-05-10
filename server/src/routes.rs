use crate::constants::PAGE_SIZE;

pub fn get_projects_api_route(domain: String) -> String {
    return format!("https://{}/rest/api/3/project", domain);
}

pub fn search_issues_for_project(domain: &String, project_id: &str, start_at: usize) -> String {
    return format!(
        "https://{}/rest/api/3/search?jql=project={}&startAt={}&maxResults={}",
        domain, project_id, start_at, PAGE_SIZE
    );
}

pub fn get_issue_object(domain: &String, issue_id: &String) -> String {
    // extra params -> ?fields=*all&expand=renderedFields,names,schema,editmeta,changelog,versionedRepresentations
    return format!("https://{}/rest/api/3/issue/{}", domain, issue_id)
}
