// error handling and io (incl. prep for ui)
use std::{
    error::Error, 
    io
};

// prep for ui (capturing every input and such)
use ratatui::{
    Terminal, 
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode
        }, 
        execute, 
        terminal::{
            EnterAlternateScreen, 
            LeaveAlternateScreen, 
            disable_raw_mode, 
            enable_raw_mode
        }
    }, 
    prelude::{
        Backend, 
        CrosstermBackend
    }
};

// app states and all of the ui rendering
use crate::{
    app::{
        App, 
        CurrentScreen, 
        CurrentlyEditing
    }, 
    ui::ui
};

mod user;
mod ui;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnableMouseCapture, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?
        }
    } else if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> 
where 
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            // ignore any event that isnt KeyEventKind::Press
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            // actions based on current screen
            match app.current_screen {

                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Username);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('r') => {
                        app.current_screen = CurrentScreen::LoadingFromFile;
                        app.file_path_input = String::new();
                    }

                    KeyCode::Backspace => {
                        app.current_screen = CurrentScreen::DeleteUser;
                        app.user_to_delete_str = String::new()
                    }
                    _ => {}
                },

                CurrentScreen::Editing => match key.code {
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }

                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Username => {
                                    app.currently_editing = Some(CurrentlyEditing::Email)
                                }
                                CurrentlyEditing::Email => {
                                    app.currently_editing = Some(CurrentlyEditing::Admin)
                                }
                                CurrentlyEditing::Admin => {
                                    app.toggle_admin();
                                }
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Username => {
                                    app.username.pop();
                                }
                                CurrentlyEditing::Email => {
                                    app.email.pop();
                                }
                                _ => {}
                            }
                        }
                    }

                    KeyCode::Char('s') => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Username => {
                                    app.username.push('s');
                                }
                                CurrentlyEditing::Email => {
                                    app.email.push('s');
                                }
                                CurrentlyEditing::Admin => {
                                    if &app.username != "" && &app.email != "" {
                                        app.save_user();
                                        app.currently_editing = None;
                                        app.current_screen = CurrentScreen::Main;
                                    } else {
                                        app.error = String::from("No field can be empty");
                                        app.current_screen = CurrentScreen::Error;
                                    }
                                }
                            }
                        }
                    }

                    KeyCode::Char(char) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Username => {
                                    app.username.push(char);
                                }
                                CurrentlyEditing::Email => {
                                    app.email.push(char);
                                }
                                CurrentlyEditing::Admin => {}
                            }
                        }
                    }

                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None
                    }

                    _ => {}
                },

                CurrentScreen::DeleteUser => {
                    match key.code {
                        KeyCode::Enter => {
                            match app.user_to_delete_str.trim().parse::<u32>() {
                                Ok(num) => {
                                    if num <= 1000 {
                                        app.error = String::from("Cannot delete user: should not exist or is user 1000. If so, remove manually from file");
                                        app.current_screen = CurrentScreen::Error;
                                    } else {
                                        app.user_to_delete = num;
                                        app.delete_user();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                },
                                Err(err) => {
                                    app.error = err.to_string();
                                    app.current_screen = CurrentScreen::Error;
                                }
                            }
                        },

                        KeyCode::Char(char) => {
                            app.user_to_delete_str.push(char);
                        }

                        KeyCode::Backspace => {
                            app.user_to_delete_str.pop();
                        }

                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    }
                },

                CurrentScreen::LoadingFromFile => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Backspace => {
                        app.file_path_input.pop();
                    }
                    KeyCode::Char(char) => {
                        app.file_path_input.push(char);
                    }
                    KeyCode::Enter => {
                        match app.json_to_hashmap() {
                            Ok(map) => {
                                app.user_list = map;
                                app.current_screen = CurrentScreen::Main;
                            },
                            Err(err) => {
                                app.error = err.to_string();
                                app.current_screen = CurrentScreen::Error;
                            }
                        }
                    }
                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Enter | KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        return Ok(false);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },

                CurrentScreen::Error => match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main
                    }
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    _ => {}
                },

            }
        }
    }
}
