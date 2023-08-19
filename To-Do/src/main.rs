use std::{rc::Rc, cell::RefCell};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use fltk::{app::*, button::*, enums::*, frame::*, group::Pack, group::{PackType, Scroll}, prelude::*, text::TextEditor, text::TextBuffer, window::*};
use fltk_theme::{WidgetTheme, ThemeType};

fn load_tasks() -> Vec<(String, bool)> {
    let mut tasks = Vec::new(); 
    let mut task_text = String::new(); 
    let mut is_important = false; 

    if let Ok(file) = File::open("Tasks.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap_or_default(); 
            if line == "<#####%%%#####>" {
                if !task_text.is_empty() {
                    tasks.push((task_text.clone(), is_important));
                    task_text.clear();
                }
            } else if line.starts_with("Дата:") && line.contains("Время:") {
                task_text.push_str(&line);
                task_text.push('\n');
            } else if line.starts_with('|') {
                is_important = line.trim() == "|важное";
            } else {
                task_text.push_str(&line);
                task_text.push('\n');
            }
        }
        if !task_text.is_empty() {
            tasks.push((task_text, is_important));
        }
    }

    tasks 
}

fn create_task_frame(
    text: &str,
    is_important: bool,
    delete_button: &Button,
    del_frame: &Rc<RefCell<Frame>>,
) -> Frame {
    let num_newlines = text.chars().filter(|&c| c == '\n').count() as i32;
    let frame_height = 40 + (num_newlines * 16);

    let mut frame = Frame::new(0, 0, 370, frame_height, "");
    frame.set_frame(FrameType::EmbossedBox);
    frame.set_color(if is_important {
        Color::from_rgb(255, 255, 0) 
    } else {
        Color::from_rgb(100, 100, 100) 
    });
    frame.set_label(text.trim()); 

    let mut delete_button = delete_button.clone();
    let del_frame = del_frame.clone();

    frame.handle(move |this, ev| {
        if ev == Event::Push {
            delete_button.activate();
            this.set_color(Color::from_rgb(82, 85, 89)); 
            *del_frame.borrow_mut() = this.clone(); 
            return true;
        }
        false
    });

    frame 
}

fn main() {
    let app = App::default();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    let mut wind = Window::new(100, 100, 400, 600, "Программа TODO").center_screen();

    let mut pack = Pack::new(10, 10, 380, 200, "");
    pack.set_spacing(10);

    let mut text_editor = TextEditor::new(0, 0, 380, 100, "");
    text_editor.set_buffer(TextBuffer::default());
    text_editor.set_scrollbar_size(16);

    let mut button = Button::new(310, 0, 70, 30, "Добавить");

    let mut delete_button = Button::new(330, 0, 70, 30, "Удалить");
    delete_button.deactivate();

    let important_checkbox = CheckButton::new(350, 20, 100, 30, "Важное");

    let scroll = Scroll::new(0, 0, 380, 350, "");

    let del_frame = Rc::new(RefCell::new(Frame::default()));

    let mut frame_group = Pack::new(0, 0, 365, 160, "");
    frame_group.set_spacing(5);
    frame_group.set_type(PackType::Vertical);
    frame_group.set_align(Align::Left);

    scroll.end();

    let mut window = wind.clone();

    let mut frame_group_clone = frame_group.clone();
    let del_frame_clone = del_frame.clone();
    delete_button.set_callback(move |this| {
        frame_group_clone.remove(&*del_frame_clone.borrow());
        this.deactivate(); 
        window.redraw(); 
    });

    let mut window = wind.clone();
    let mut frame_group_clone = frame_group.clone();

    let tasks = load_tasks();
    for (text, is_important) in tasks {
        frame_group_clone.add(&create_task_frame(&text, is_important, &delete_button, &del_frame));
        window.redraw();
    }

    button.set_callback(move |_| {
        let task_text = text_editor.buffer().unwrap().text();
        if !task_text.is_empty() {
            frame_group_clone.add(&create_task_frame(&format!(
                "Дата: {}\tВремя: {}\n{}",
                chrono::Local::now().format("%Y-%m-%d"),
                chrono::Local::now().format("%H:%M:%S"),
                task_text,
            ), important_checkbox.value(), &delete_button, &del_frame));
            window.redraw();
        }
    });

    wind.end();
    wind.show();

    app.run().unwrap();

    save_tasks(&frame_group);
}

fn save_tasks(frame_group: &Pack) {
    match File::create("Tasks.txt") {
        Ok(mut file) => {
            for i in 0..frame_group.children() {
                if let Some(frame) = frame_group.child(i) {
                    let task_text = frame.label();
                    let is_important = if frame.color() == Color::from_rgb(255, 255, 0) {
                        "важное"
                    } else {
                        "обычное"
                    };
        
                    file.write_all(&format!("{}\n|{}\n<#####%%%#####>\n", task_text.trim(), is_important).as_bytes()).unwrap();
                }
            }
        },
        Err(_) => {
            return;
        }
    };
}
