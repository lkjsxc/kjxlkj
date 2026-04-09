use super::index::{admin_create_actions, list_rail};

#[test]
fn guest_list_rail_places_github_above_sign_in() {
    let html = list_rail(
        "home",
        "",
        r#"<a href="/login" class="btn">Admin sign in</a>"#,
        false,
    );
    let github = html.find("Open GitHub").unwrap();
    let sign_in = html.find("Admin sign in").unwrap();
    assert!(github < sign_in);
}

#[test]
fn admin_list_rail_places_new_note_then_github_then_logout() {
    let html = list_rail("search", &admin_create_actions(), r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#, true);
    let new_note = html.find("New note").unwrap();
    let new_media = html.find("New media").unwrap();
    let github = html.find("Open GitHub").unwrap();
    let logout = html.find("Logout").unwrap();
    assert!(new_note < new_media && new_media < github && github < logout);
}
