use crate::commands::{CursorMoveDown, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::fs::JoshutoDirEntry;
use crate::ui::TuiBackend;
use crate::unix;
use crate::util::textfield::TextField;

#[derive(Clone, Debug)]
pub struct SetMode;

const LIBC_PERMISSION_VALS: [(libc::mode_t, char); 9] = [
    (libc::S_IRUSR, 'r'),
    (libc::S_IWUSR, 'w'),
    (libc::S_IXUSR, 'x'),
    (libc::S_IRGRP, 'r'),
    (libc::S_IWGRP, 'w'),
    (libc::S_IXGRP, 'x'),
    (libc::S_IROTH, 'r'),
    (libc::S_IWOTH, 'w'),
    (libc::S_IXOTH, 'x'),
];

impl SetMode {
    pub fn new() -> Self {
        SetMode
    }
    pub const fn command() -> &'static str {
        "set_mode"
    }

    pub fn set_mode(&self, entry: &mut JoshutoDirEntry, initial: String) -> bool {
        use std::os::unix::fs::PermissionsExt;

        const PROMPT: &str = ":set_mode ";
        let user_input: Option<String> = None;

        match user_input {
            Some(s) => {
                let mut mode: u32 = 0;
                for (i, ch) in s.chars().enumerate() {
                    if ch == LIBC_PERMISSION_VALS[i].1 {
                        let val: u32 = LIBC_PERMISSION_VALS[i].0 as u32;
                        mode |= val;
                    }
                }
                unix::set_mode(entry.file_path().as_path(), mode);
                entry.metadata.permissions.set_mode(mode + (1 << 15));
                true
            }
            None => false,
        }
    }
}

impl JoshutoCommand for SetMode {}

impl std::fmt::Display for SetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for SetMode {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        use std::os::unix::fs::PermissionsExt;
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        if let Some(curr_list) = curr_tab.curr_list_mut() {
            if let Some(file) = curr_list.get_curr_mut() {
                let mode = file.metadata.permissions.mode();
                let mut mode_string = unix::stringify_mode(mode);
                mode_string.remove(0);

                self.set_mode(file, mode_string);
                CursorMoveDown::new(1).execute(context, backend)?;
            }
        }
        Ok(())
    }
}
