use crossterm::{
    cursor,
    event::{self},
    execute,
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
};
use std::io;
use std::io::Write;

struct SelectableItem {
    label: String,
    selected: bool,
}

impl SelectableItem {
    fn new(label: &str) -> Self {
        SelectableItem {
            label: label.to_string(),
            selected: false,
        }
    }
}

struct SelectableList {
    items: Vec<SelectableItem>,
    selected_index: usize,
}

impl SelectableList {
    fn new(items: Vec<&str>) -> Self {
        let items = items
            .into_iter()
            .map(|label| SelectableItem::new(label))
            .collect();

        SelectableList {
            items,
            selected_index: 0,
        }
    }

    fn draw(&self) {
        for (index, item) in self.items.iter().enumerate() {
            let line = if index == self.selected_index {
                format!(
                    "{}{}{}{}",
                    style::Attribute::Bold,
                    if item.selected { "* " } else { "> " },
                    style::style(&item.label)
                        .with(if item.selected { Color::Green } else { Color::White }),
                    style::Attribute::Reset,
                )
            } else {
                format!(
                    "{}{}",
                    { "  " },
                    if item.selected {
                        style::style(&item.label)
                            .with(Color::Green)
                            .to_string()
                    } else {
                        style::style(&item.label).to_string()
                    }
                )
            };

            execute!(
                io::stdout(),
                cursor::MoveToColumn(0)
            )
            .unwrap();
        
            println!("{line}");
        }
    }

    fn toggle_selected(&mut self) {
        if let Some(item) = self.items.get_mut(self.selected_index) {
            item.selected = !item.selected;
        }
    }

    fn move_selection(&mut self, offset: i32) {
        let len = self.items.len() as i32;
        self.selected_index = ((self.selected_index as i32 + offset + len) % len) as usize;
    }
}

fn main() {
    // Set up terminal
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    let mut selectable_list = SelectableList::new(items);

    loop {

        execute!(
            io::stdout(),
            cursor::Hide,
            style::PrintStyledContent(
                style::style("\n\nWelcome to the Selectable List Example!\n\n").bold()
            )
        )
        .unwrap();

        selectable_list.draw();
        io::stdout().flush().unwrap();

        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                if key_event.modifiers == event::KeyModifiers::CONTROL
                    && key_event.code == event::KeyCode::Char('c')
                {
                    break;
                }

                match key_event.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Esc => break,
                    event::KeyCode::Enter => selectable_list.toggle_selected(),
                    event::KeyCode::Char(' ') => selectable_list.toggle_selected(),
                    event::KeyCode::Up => selectable_list.move_selection(-1),
                    event::KeyCode::Down => selectable_list.move_selection(1),
                    event::KeyCode::Tab => selectable_list.move_selection(1),
                    _ => {}
                }
            }
        }

        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::FromCursorDown)
        )
        .unwrap();
    }

    // Clean up terminal
    execute!(
        io::stdout(),
        cursor::Show,
    )
    .unwrap();
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
