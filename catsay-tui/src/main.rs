extern crate cursive;

use cursive::traits::Identifiable;
use cursive::views::{Dialog, TextView, ListView, EditView, Checkbox, SelectView};
use cursive::event::Key;
use cursive::align::HAlign;

const TEMPLATE_EXT: &str = "catto";
const ALIVE_EYE: &str = "o";
const DEAD_EYE: &str = "x";
const DEFAULT_CAT_TEMPLATE: &str = "\
\\
 \\
  /\\_/\\
 ( {eye} {eye} )
 =( I )=\
";
const MESSAGE_INPUT_NAME: &str = "message_input";
const DEAD_INPUT_NAME: &str = "dead_input";
const TEMPLATE_INPUT_NAME: &str = "template_input";

struct CatsayOptions {
    message: String,
    dead: bool,
    template: Option<String>,
}

fn main() {
    let mut cursive_root = cursive::default();

    let options_dialog = options_dialog();
    cursive_root.add_layer(options_dialog);

    cursive_root.add_global_callback(Key::Esc, exit_cb);
    cursive_root.run();
}

fn options_dialog() -> Dialog {
    let message_input = EditView::new().with_name(MESSAGE_INPUT_NAME);
    let dead_input = Checkbox::new().with_name(DEAD_INPUT_NAME);
    let template_input = template_select().with_name(TEMPLATE_INPUT_NAME);

    let options_list = ListView::new()
        .child("message", message_input)
        .child("dead?", dead_input)
        .child("template", template_input);

    return Dialog::new()
        .title("catsay options:")
        .content(options_list)
        .button("OK", options_dialog_ok_cb)
        .button("Exit", exit_cb);
}

fn template_select() -> SelectView {
    let mut select = SelectView::new()
        .popup()
        .h_align(HAlign::Center);

    let current_dir_path: std::path::PathBuf;
    match std::env::current_dir() {
        Ok(cwd) => {
            current_dir_path = cwd
        },
        Err(_msg) => {
            select.disable();
            return select;
        }
    }

    let read_dir_iter: std::fs::ReadDir;
    match std::fs::read_dir(current_dir_path) {
        Ok(read_dir) => {
            read_dir_iter = read_dir; 
        },
        Err(_msg) => {
            select.disable();
            return select;
        }
    }

    let mut count = 0;
    for dir in read_dir_iter {
        match dir {
            Ok(entry) => {
                let path = entry.path();
                let ext = path.extension();
                let filepath = path.to_str();
                
                if ext.is_none() || filepath.is_none() || ext.unwrap() != TEMPLATE_EXT { continue }

                count+=1;
                select.add_item(path.file_stem().unwrap().to_str().unwrap(), filepath.unwrap().to_string());
            },
            Err(_msg) => {
                break;
            }
        }
    }

    if count == 0 { 
        select.disable();
    }

    return select;
}

fn options_dialog_ok_cb(siv: &mut cursive::Cursive) {
    let mut options = CatsayOptions{
        message: String::new(),
        dead: false,
        template: None,
    };

    let message_input_content = siv.call_on_name(MESSAGE_INPUT_NAME, |input: &mut EditView| { 
        return String::from(input.get_content().as_ref());
    });
    match message_input_content {
        Some(message) => {
            options.message.push_str(&message);
        },
        None => {
            options.message.push_str("sth went iffy");
        }
    }

    let dead_input_content = siv.call_on_name(DEAD_INPUT_NAME, |input: &mut Checkbox| { input.is_checked() });
    match dead_input_content {
        Some(dead) => {
            options.dead = dead;
        },
        None => {}
    }

    options.template = siv.call_on_name(TEMPLATE_INPUT_NAME, |input: &mut SelectView| { 
        match std::fs::read_to_string(input.selection().unwrap().as_ref()) {
            Ok(template) => {
                return template;
            },
            Err(_) => {
                return DEFAULT_CAT_TEMPLATE.to_owned();
            }
        };
    });

    siv.pop_layer();
    show_cat(siv, options);
    return
}

fn show_cat(siv: &mut cursive::Cursive, opts: CatsayOptions) {
    let eyes = if opts.dead { DEAD_EYE } else { ALIVE_EYE };
    let mut cat_render;
    match opts.template {
        Some(template) => {
            cat_render = template;
        },
        None => {
            cat_render = DEFAULT_CAT_TEMPLATE.to_owned()
        }
    }
    cat_render = cat_render.replace("{eye}", &format!("{}", eyes) );

    let mut render = String::new();
    render.push_str(&opts.message);
    render.push_str(&cat_render);

    let render_layer = TextView::new(render);
    let cat_dialog = Dialog::around(render_layer);
    let cat_dialog = cat_dialog.button("exit", exit_cb);

    siv.add_layer(cat_dialog);
}

fn exit_cb(siv: &mut cursive::Cursive) {
    siv.quit();
}
