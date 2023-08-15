#[derive(PartialEq)]
pub enum Action {
    WorkspaceSetActive(usize),
    WindowSetWorkspace(usize),
    Spawn(String),
}
