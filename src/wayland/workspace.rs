use smithay::desktop::{Space, Window};

struct Workspace {
    windows: Vec<Window>,
    active_window: Option<usize>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window: None,
        }
    }
}

pub struct Workspaces {
    workspaces: Vec<Workspace>,
    active_workspace: usize,
    previous_workspace: usize,
}

impl Workspaces {
    pub fn new() -> Self {
        Self {
            workspaces: (0..=8).map(|_| Workspace::new()).collect(),
            active_workspace: 0,
            previous_workspace: 0,
        }
    }

    pub fn active(&self) -> usize {
        self.active_workspace
    }

    pub fn set_active(&mut self, workspace: usize, space: &mut Space<Window>) {
        self.previous_workspace = self.active_workspace;
        self.active_workspace = workspace;
        self.refresh_geometry(space);
    }

    pub fn set_active_window(&mut self, window: Window) {
        let workspace = &mut self.workspaces[self.active_workspace];
        workspace.active_window = workspace.windows.iter().position(|w| w == &window);
    }

    pub fn insert_window(&mut self, workspace: usize, window: Window) {
        self.workspaces[workspace].windows.push(window.clone());
    }

    pub fn move_window(&mut self, workspace: usize, space: &mut Space<Window>) {
        if let Some(active_window) = self.workspaces[self.active_workspace].active_window {
            let window = self.workspaces[self.active_workspace].windows[active_window].clone();

            self.workspaces[self.active_workspace]
                .windows
                .retain(|w| w != &window);
            self.insert_window(workspace, window);
            self.refresh_geometry(space);
        }
    }

    pub fn refresh_geometry(&mut self, space: &mut Space<Window>) {
        space.refresh();

        self.workspaces[self.previous_workspace]
            .windows
            .iter()
            .for_each(|window| space.unmap_elem(window));

        let output = space.outputs().next().cloned().unwrap();

        let output_geometry = space.output_geometry(&output).unwrap();
        let output_width = output_geometry.size.w;
        let output_height = output_geometry.size.h;

        let gap = 6;

        let windows = &mut self.workspaces[self.active_workspace].windows;

        let elements_count = windows.len() as i32;

        for (i, window) in windows.iter().enumerate() {
            let (mut x, mut y) = (gap, gap);
            let (mut width, mut height) = (output_width - gap * 2, output_height - gap * 2);

            if elements_count > 1 {
                width -= gap;
                width /= 2;
            }

            if i > 0 {
                height /= elements_count - 1;

                x += width + gap;
                y += height * (i as i32 - 1);
            }

            if i > 1 {
                height -= gap;
                y += gap;
            }

            window.toplevel().with_pending_state(|state| {
                state.size = Some((width, height).into());
            });
            window.toplevel().send_pending_configure();

            space.map_element(window.clone(), (x, y), false);
        }
    }
}
