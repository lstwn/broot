use {
    super::*,
    crate::{
        app::SelectionType,
        keys::*,
    },
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

fn build_internal(
    internal: Internal,
    bang: bool,
) -> Verb {
    let invocation = internal.invocation_pattern();
    let execution = VerbExecution::Internal(
        InternalExecution::from_internal_bang(internal, bang)
    );
    let description = VerbDescription::from_text(internal.description().to_string());
    Verb::new(Some(invocation), execution, description).unwrap()
}

fn internal(
    internal: Internal,
) -> Verb {
    build_internal(internal, false)
}

fn internal_bang(
    internal: Internal,
) -> Verb {
    build_internal(internal, true)
}

fn external(
    invocation_str: &str,
    execution_str: &str,
    exec_mode: ExternalExecutionMode,
) -> Verb {
    let execution = VerbExecution::External(ExternalExecution::new(
        execution_str.to_string(),
        exec_mode,
    ));
    Verb::new(
        Some(invocation_str),
        execution,
        VerbDescription::from_code(execution_str.to_string()),
    ).unwrap()
}

/// declare the built_in verbs, the ones which are available
/// in standard (they still may be overriden by configuration)
pub fn builtin_verbs() -> Vec<Verb> {
    use super::{ExternalExecutionMode::*, Internal::*};
    vec![
        internal(back),

        // those two operations are mapped on ALT-ENTER, one
        // for directories and the other one for the other files
        external(
            "cd",
            "cd {directory}",
            FromParentShell,
        )
            .with_stype(SelectionType::Directory)
            .with_key(ALT_ENTER)
            .with_description("change directory and quit"),
        internal(open_leave) // calls the system open
            .with_key(ALT_ENTER)
            .with_shortcut("ol"),

        #[cfg(unix)]
        external(
            "chmod {args}",
            "chmod {args} {file}",
            StayInBroot,
        )
            .with_stype(SelectionType::File),
        #[cfg(unix)]
        external(
            "chmod {args}",
            "chmod -R {args} {file}",
            StayInBroot,
        )
            .with_stype(SelectionType::Directory),
        internal(open_preview),
        internal(close_preview),
        internal(toggle_preview),
        internal(preview_image),
        internal(preview_text),
        internal(preview_binary),
        internal(close_panel_ok),
        internal(close_panel_cancel)
            .with_key(BACK_TAB)
            .with_control_key('w'),
        external(
            "copy {newpath:path-from-parent}",
            "cp -r {file} {newpath:path-from-parent}",
            StayInBroot,
        )
            .with_shortcut("cp"),
        #[cfg(feature="clipboard")]
        internal(copy_path)
            .with_alt_key('c'),
        external(
            "copy_to_panel",
            "cp -r {file} {other-panel-directory}",
            StayInBroot,
        )
            .with_shortcut("cpp"),
        #[cfg(unix)]
        internal(filesystems)
            .with_shortcut("fs"),
        // :focus is also hardcoded on Enter on directories
        // but ctrl-f is useful for focusing on a file's parent
        // (and keep the filter)
        internal(focus)
            .with_control_key('f'),
        internal(help)
            .with_key(F1).with_shortcut("?"),
        #[cfg(feature="clipboard")]
        internal(input_paste)
            .with_control_key('v'),
        internal(line_down).with_key(DOWN),
        internal(line_up).with_key(UP),
        external(
            "mkdir {subpath}",
            "mkdir -p {subpath:path-from-directory}",
            StayInBroot,
        )
            .with_shortcut("md"),
        external(
            "move {newpath:path-from-parent}",
            "mv {file} {newpath:path-from-parent}",
            StayInBroot,
        )
            .with_shortcut("mv"),
        external(
            "move_to_panel",
            "mv {file} {other-panel-directory}",
            StayInBroot,
        )
            .with_shortcut("mvp"),
        internal_bang(start_end_panel)
            .with_control_key('p'),
        internal(next_match)
            .with_key(TAB),
        internal(no_sort)
            .with_shortcut("ns"),
        internal(open_stay)
            .with_key(ENTER)
            .with_shortcut("os"),
        internal(open_stay_filter)
            .with_shortcut("osf"),
        internal(parent).with_shortcut("p"),
        internal(page_down).with_key(PAGE_DOWN),
        internal(page_up).with_key(PAGE_UP),
        internal(panel_left)
            .with_key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::CONTROL,
            }),
        internal(panel_right)
            .with_key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CONTROL,
            }),
        internal(print_path).with_shortcut("pp"),
        internal(print_relative_path).with_shortcut("prp"),
        internal(print_tree).with_shortcut("pt"),
        internal(quit)
            .with_control_key('c')
            .with_control_key('q')
            .with_shortcut("q"),
        internal(refresh).with_key(F5),
        internal(sort_by_count).with_shortcut("sc"),
        internal(sort_by_date).with_shortcut("sd"),
        internal(sort_by_size).with_shortcut("ss"),
        external(
            "rm",
            "rm -rf {file}",
            StayInBroot,
        ),
        internal(toggle_counts).with_shortcut("counts"),
        internal(toggle_dates).with_shortcut("dates"),
        internal(toggle_files).with_shortcut("files"),
        internal(toggle_git_ignore).with_shortcut("gi"),
        internal(toggle_git_file_info).with_shortcut("gf"),
        internal(toggle_git_status).with_shortcut("gs"),
        internal(toggle_root_fs).with_shortcut("rfs"),
        internal(toggle_hidden).with_shortcut("h"),
        #[cfg(unix)]
        internal(toggle_perm).with_shortcut("perm"),
        internal(toggle_sizes).with_shortcut("sizes"),
        internal(toggle_trim_root),
        internal(total_search).with_control_key('s'),
        internal(up_tree).with_shortcut("up"),
    ]
}
