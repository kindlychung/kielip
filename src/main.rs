use instant::Instant;
use rand::distributions::Alphanumeric;
use rand::Rng;
use regex::Regex;
use xshell::cmd;

use kielip::config::{get_config, get_config_path, get_editor, setup_config, Action, Config};
use kielip::ringbuffer::RingBuffer;
use std::{collections::HashMap, path::PathBuf, process::Output, sync::Arc, thread};
use std::{time::Duration, vec};
use x11_clipboard::Clipboard;

use druid::{
    im::get_in,
    widget::{prelude::*, Button, Flex, Label, List, Painter, Scroll},
    AppDelegate, Command, DelegateCtx, Handled, Insets, UnitPoint,
};
use druid::{AppLauncher, Color, Data, Lens, Selector, Target, WidgetExt, WindowDesc};

const ADD_ENTRY: Selector<String> = Selector::new("kielip.add_entry");

const MAX_STR_LEN: f64 = 50.0;
const MAX_LIGHTNESS: f64 = 0.5;
const CORNER_RADIUS: f64 = 3.0;
const STROKE_COLOR: Color = Color::from_rgba32_u32(0x000000ff);
const HEADER_COLOR: Color = Color::from_rgba32_u32(0x111111ff);
const STROKE_WIDTH: f64 = 1.0;

pub fn main() {
    setup_config();
    let window = WindowDesc::new(make_ui).title("Kielip");
    let data = ClipboardData {
        history: RingBuffer::new(get_config().max_history()),
    };
    let launcher = AppLauncher::with_window(window);
    let event_sink = launcher.get_external_handle();
    thread::spawn(move || watch_clipboard(event_sink));
    launcher
        .use_simple_logger()
        .delegate(Delegate)
        .launch(data)
        .expect("launch failed");
}

fn watch_clipboard(event_sink: druid::ExtEventSink) {
    let clipboard: Clipboard = Clipboard::new().unwrap();
    let mut last: String = String::new();
    // for (s, action) in get_config().actions() {}
    let actions: Vec<(Regex, &(bool, Action))> = get_config()
        .actions()
        .iter()
        .map(|(k, v)| (Regex::new(k).unwrap(), v))
        .collect();
    loop {
        if let Ok(curr) = clipboard.load_wait(
            clipboard.getter.atoms.clipboard,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property,
        ) {
            let curr = String::from_utf8_lossy(&curr);
            let mut curr = curr.trim_matches('\u{0}').trim().to_owned();
            if !curr.is_empty() && last != curr {
                    dbg!(&actions);
                for (regex, action) in actions.iter() {
                    if let Some(_) = regex.find(&curr) {
                        match action {
                            (true, Action::Remove) => {
                                curr = regex.replace_all(&curr, "").to_string();
                            }
                            (true, Action::Scramble) => {
                                let s: String = rand::thread_rng()
                                    .sample_iter(&Alphanumeric)
                                    .take(7)
                                    .map(char::from)
                                    .collect();
                                curr = regex.replace_all(&curr, &s[..]).to_string();
                                clipboard.store(
                                    clipboard.setter.atoms.clipboard,
                                    clipboard.setter.atoms.utf8_string,
                                    curr.clone().as_bytes(),
                                );
                            }
                            (true, Action::Replace { replacement }) => {
                                curr = regex.replace_all(&curr, &replacement[..]).to_string();
                                dbg!("Replacing...");
                                dbg!(&curr);
                                clipboard.store(
                                    clipboard.setter.atoms.clipboard,
                                    clipboard.setter.atoms.utf8_string,
                                    curr.clone().as_bytes(),
                                );
                            }
                            (true, Action::Exec { command_pattern }) => {
                                let cmd = Regex::new(r#"\{\}"#)
                                    .unwrap()
                                    .replace_all(&command_pattern[..], &curr[..]);
                                run_cmd(&cmd);
                            }
                            _ => {}
                        }
                    }
                }
                last = curr.clone();
                if event_sink
                    .submit_command(ADD_ENTRY, format!("{}", &curr), Target::Auto)
                    .is_err()
                {
                    break;
                }
            }
        }
    }
}

#[derive(Clone, Data, PartialEq, Lens)]
struct ClipboardData {
    history: RingBuffer<String>,
}

impl ClipboardData {
    pub fn clear(self: &mut ClipboardData) {
        self.history.clear();
    }
}

fn make_ui() -> impl Widget<ClipboardData> {
    let mut root = Flex::column();
    let mut buttons = Flex::row();
    buttons.add_flex_child(
        Button::new("Clear")
            .on_click(|_, data: &mut ClipboardData, _| {
                data.clear();
            })
            .fix_height(35.0)
            .padding(Insets::new(0.0, 0.0, 8.0, 0.0)),
        1.0,
    );
    buttons.add_flex_child(
        Button::new("Config")
            .on_click(|_, data: &mut ClipboardData, _| {
                let editor = get_editor();
                let path = get_config_path().as_os_str().to_str().unwrap();
                let cmd = format!("gvim {}", path);
                run_cmd(&cmd);
                let cmd = r#"zenity --height 100 --width 300 --info --title="Restart" --text="You have changed configuration, please restart kielip to make your changes take effect.""#;
                run_cmd(&cmd);
            })
            .fix_height(35.0),
        1.0,
    );
    buttons.add_flex_spacer(1.0);
    root.add_child(buttons.padding(10.0).background(HEADER_COLOR));
    root.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|item: &String, _env: &_| {
                let item = item.trim().replace("\n", " ");
                if item.is_empty() {
                    return item;
                }
                let index = (MAX_STR_LEN as usize - 1).min(item.len()).max(0);
                if index < item.len() {
                    format!("{}...", &item[0..index])
                } else {
                    format!("{}", &item[0..index])
                }
            })
            .align_vertical(UnitPoint::LEFT)
            .padding(8.0)
            .expand()
            .height(35.0)
            // .background(Color::rgb(0.2, 0.2, 0.2))
            .background(Painter::new(
                |ctx: &mut PaintCtx, data: &String, env: &Env| {
                    let c = (data.len() as f64 / MAX_STR_LEN).min(1.0) * MAX_LIGHTNESS;
                    let color = Color::rgb(c, c, c);
                    let bounds = ctx.size().to_rect();
                    let rounded = bounds.to_rounded_rect(CORNER_RADIUS);
                    ctx.fill(rounded, &color);
                    ctx.stroke(rounded, &STROKE_COLOR, STROKE_WIDTH);
                },
            ))
            .on_click(|_ctx, s: &mut String, _env| {
                let clipboard: Clipboard = Clipboard::new().unwrap();
                clipboard.store(
                    clipboard.setter.atoms.clipboard,
                    clipboard.setter.atoms.utf8_string,
                    s.as_bytes(),
                );
            })
            .padding(Insets::new(10.0, 3.0, 10.0, 0.0))
        }))
        .vertical()
        .lens(ClipboardData::history),
        1.0,
    );
    root
}

struct Delegate;

impl AppDelegate<ClipboardData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut ClipboardData,
        _env: &Env,
    ) -> Handled {
        if let Some(s) = cmd.get(ADD_ENTRY) {
            data.history.push(s.clone());
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

fn run_cmd(cmd: &str) -> Output {
    std::process::Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute process")
}
