// Импортируем необходимые библиотеки и модули из стандартной библиотеки Rust и Fltk
use std::{rc::Rc, cell::RefCell};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use fltk::{app::*, button::*, enums::*, frame::*, group::Pack, group::{PackType, Scroll}, prelude::*, text::TextEditor, text::TextBuffer, window::*};
use fltk_theme::{WidgetTheme, ThemeType};

/// Функция для загрузки задач из файла
fn load_tasks() -> Vec<(String, bool)> {
    let mut tasks = Vec::new(); // Вектор для хранения задач
    let mut task_text = String::new(); // Переменная для хранения текста текущей задачи
    let mut is_important = false; // Флаг, указывающий, является ли текущая задача важной

    // Открываем файл и читаем его построчно
    if let Ok(file) = File::open("Tasks.txt") {
        let reader = BufReader::new(file);

        // Читаем каждую строку файла
        for line in reader.lines() {
            let line = line.unwrap_or_default(); // Читаем строку и обрабатываем возможные ошибки

            if line == "<#####%%%#####>" {
                // Если встретили разделитель, добавляем задачу в список задач
                if !task_text.is_empty() {
                    tasks.push((task_text.clone(), is_important));
                    task_text.clear();
                }
            } else if line.starts_with("Дата:") && line.contains("Время:") {
                // Если строка содержит дату и время, добавляем их в текст задачи
                task_text.push_str(&line);
                task_text.push('\n');
            } else if line.starts_with('|') {
                // Если строка начинается с "|", это означает важность задачи
                is_important = line.trim() == "|важное";
            } else {
                // Иначе добавляем строку в текст задачи
                task_text.push_str(&line);
                task_text.push('\n');
            }
        }

        // Добавляем последнюю задачу в список задач
        if !task_text.is_empty() {
            tasks.push((task_text, is_important));
        }
    }

    tasks // Возвращаем список задач
}

/// Функция для создания фрейма задачи
fn create_task_frame(
    text: &str,
    is_important: bool,
    delete_button: &Button,
    del_frame: &Rc<RefCell<Frame>>,
) -> Frame {
    // Вычисляем высоту фрейма на основе количества строк текста
    let num_newlines = text.chars().filter(|&c| c == '\n').count() as i32;
    let frame_height = 40 + (num_newlines * 16);

    // Создаем фрейм и настраиваем его свойства
    let mut frame = Frame::new(0, 0, 370, frame_height, "");
    frame.set_frame(FrameType::EmbossedBox);
    frame.set_color(if is_important {
        Color::from_rgb(255, 255, 0) // Цвет для важных задач
    } else {
        Color::from_rgb(100, 100, 100) // Цвет для обычных задач
    });
    frame.set_label(text.trim()); // Устанавливаем текст задачи в фрейм

    // Клонируем кнопку "Удалить" и фрейм "del_frame" для использования в замыкании
    let mut delete_button = delete_button.clone();
    let del_frame = del_frame.clone();

    // Обработчик события для фрейма
    frame.handle(move |this, ev| {
        if ev == Event::Push {
            delete_button.activate();
            this.set_color(Color::from_rgb(82, 85, 89)); // Изменяем цвет фрейма при нажатии
            *del_frame.borrow_mut() = this.clone(); // Обновляем ссылку на текущий фрейм для удаления
            return true;
        }
        false
    });

    frame // Возвращаем созданный фрейм
}

fn main() {
    // Создаём приложение и настраиваем тему оформления
    let app = App::default();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    // Создаём главное окно
    let mut wind = Window::new(100, 100, 400, 600, "Программа TODO").center_screen();

    // Создаём виджет для группировки элементов
    let mut pack = Pack::new(10, 10, 380, 200, "");
    pack.set_spacing(10);

    // Создаём многострочное текстовое поле
    let mut text_editor = TextEditor::new(0, 0, 380, 100, "");
    text_editor.set_buffer(TextBuffer::default());
    text_editor.set_scrollbar_size(16);

    // Создаём кнопку для создания фреймов
    let mut button = Button::new(310, 0, 70, 30, "Добавить");

    // Создаём кнопку для удаления фреймов
    let mut delete_button = Button::new(330, 0, 70, 30, "Удалить");
    delete_button.deactivate();

    // Создаём чекбокс для важных задач
    let important_checkbox = CheckButton::new(350, 20, 100, 30, "Важное");

    // Создаём скролл для навигации по фреймам
    let scroll = Scroll::new(0, 0, 380, 350, "");

    // Создаём переменную, в которой будем хранить фрейм для удаления
    let del_frame = Rc::new(RefCell::new(Frame::default()));

    // Создаём виджет для группировки фреймов
    let mut frame_group = Pack::new(0, 0, 365, 160, "");
    frame_group.set_spacing(5);
    frame_group.set_type(PackType::Vertical);
    frame_group.set_align(Align::Left);

    scroll.end();

    let mut window = wind.clone();

    let mut frame_group_clone = frame_group.clone();
    let del_frame_clone = del_frame.clone();
    // Создаём колбэк для кнопки
    delete_button.set_callback(move |this| {
        frame_group_clone.remove(&*del_frame_clone.borrow());
        this.deactivate(); // Деактивируем кнопку
        window.redraw(); // Перерисовываем окно
    });

    let mut window = wind.clone();
    let mut frame_group_clone = frame_group.clone();

    // Загружаем задачи из файла и создаем соответствующие фреймы
    let tasks = load_tasks();
    for (text, is_important) in tasks {
        frame_group_clone.add(&create_task_frame(&text, is_important, &delete_button, &del_frame));
        window.redraw();
    }

    // Создаём колбэк для добавления фрейма
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

    // Сохраняем задачи
    save_tasks(&frame_group);
}

/// Функция для сохранения задач в файл
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
