// actual ui
use ratatui::{
    Frame, 
    layout::{
        Constraint, Direction, Layout, Rect
    }, 
    style::{
        Color, Style
    }, 
    text::{
        Line, Span, Text
    }, 
    widgets::{
        Block, Borders, List, Paragraph, Wrap
    }
};
// app states
use crate::{
    App, 
    app::{
        CurrentScreen, 
        CurrentlyEditing
    }
};

pub fn ui(frame: &mut Frame, app: &mut App) {
// -- initial division of screen --
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(5)
        ])
        .split(frame.area());

// -- title layout and content --
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        format!("Simple User System v{} -- selected user: {} ({})", &app.version, match &app.user_list.list_state.selected() {
            Some(val) => val.to_string(),
            None => "Nothing selected".to_string()
        },
        &app.user_list.user_vec.len()
    ), 
        Style::default().fg(Color::Red)
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

// -- middle block layout and content --
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(chunks[1]);

    // layout for user input fields
    let user_enter_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1)
        ])
        .split(middle_chunks[0]);

    let mut user_field = Block::default()
        .title("Username")
        .borders(Borders::ALL)
        .style(Style::default());

    let mut email_field = Block::default()
        .title("Email")
        .borders(Borders::ALL)
        .style(Style::default());

    let mut admin_field = Block::default()
        .title("Admin?")
        .borders(Borders::ALL)
        .style(Style::default());

    let active_style = Style::default().bg(Color::Gray).fg(Color::Black);

    // show active field when editing
    if let Some(editing) = &app.currently_editing {
        match editing {
            CurrentlyEditing::Username => user_field = user_field.style(active_style),
            CurrentlyEditing::Email => email_field = email_field.style(active_style),
            CurrentlyEditing::Admin => admin_field = admin_field.style(active_style) 
        }

        let username_value = Paragraph::new(app.user_info.username.clone()).block(user_field).wrap(Wrap { trim: false });
        frame.render_widget(username_value, user_enter_layout[0]);

        let email_value = Paragraph::new(app.user_info.email.clone()).block(email_field).wrap(Wrap { trim: false });
        frame.render_widget(email_value, user_enter_layout[1]);

        let admin_value = Paragraph::new(app.user_info.admin.clone().to_string()).block(admin_field).wrap(Wrap { trim: false });
        frame.render_widget(admin_value, user_enter_layout[2]);
    } else {
        frame.render_widget(user_field, user_enter_layout[0]);
        frame.render_widget(email_field, user_enter_layout[1]);
        frame.render_widget(admin_field, user_enter_layout[2]);
    }

    // layout for user list
    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(5), // extra whitespace so it doesnt look cramped
            Constraint::Percentage(95), // actual list space
        ])
        .split(middle_chunks[1]);

    let list_users = app.user_list.user_vec.clone()
        .iter()
        .map(|i| {format!("{i}")})
        .collect::<List>()
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow))
        .highlight_symbol("> ");

    // // list highlight
    // let list = List::new(list_users)
    //     .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow))
    //     .highlight_symbol("> ");

    frame.render_stateful_widget(list_users, list_chunks[1], &mut app.user_list.list_state);

// -- bottom footer block --
    let footer_block = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(chunks[2]);

    // information on which screen the user is on and what they are doing
    let current_nav_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
            CurrentScreen::LoadingFromFile => Span::styled("Loading From File", Style::default().fg(Color::Magenta)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
            CurrentScreen::Error => Span::styled("Error", Style::default().bg(Color::Red).fg(Color::Black).bold()),
            CurrentScreen::DeleteUser => Span::styled("Deleting User", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),

        Span::styled(" | ", Style::default().fg(Color::White)),

        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Username => Span::styled("Editing Username", Style::default().fg(Color::LightGreen)),
                    CurrentlyEditing::Email => Span::styled("Editing Email", Style::default().fg(Color::LightGreen)),
                    CurrentlyEditing::Admin => Span::styled("Editing Admin Status", Style::default().fg(Color::LightGreen)),
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },        
    ];

    let mode_footer = Paragraph::new(Line::from(current_nav_text))
        .block(Block::default().borders(Borders::ALL));

    // instructions for current screen
    let key_hint_style = Style::default().fg(Color::Blue);

    let current_key_hints = {
        match &app.current_screen {
            CurrentScreen::Main => Span::styled(
                "[q] quit | [e] make new user | [r] load from file | [Backspace] delete user", 
                key_hint_style
            ),
            CurrentScreen::Editing => Span::styled(
                "[Tab] switch fields | [Esc] cancel | [Enter] complete | [s] save (when on admin)", 
                key_hint_style
            ),
            CurrentScreen::LoadingFromFile => Span::styled(
                "WARNING: THIS WILL DELETE ALL ADDED USERS. DO NOT USE IN MIDDLE OF RUNTIME | [Esc] cancel",
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Exiting => Span::styled(
                "[Y/Enter] to quit | [N] to quit and discard | [Esc] to cancel", 
                key_hint_style
            ),
            CurrentScreen::Error => Span::styled(
                "[Enter] return to Main | [e] return to user creation", 
                key_hint_style
            ),
            CurrentScreen::DeleteUser => Span::styled(
                "[Enter] to confirm | [Esc] to cancel", 
                key_hint_style
            ),
        }
    };

    let key_note_footer = Paragraph::new(Line::from(current_key_hints))
        .block(Block::default().borders(Borders::ALL)).wrap(Wrap { trim: false });

    frame.render_widget(mode_footer, footer_block[0]);
    frame.render_widget(key_note_footer, footer_block[1]);

    let popup_area_std: Rect = centered_rect(40, frame.area());

    // popup for entering file path to read from
    if let CurrentScreen::LoadingFromFile = &app.current_screen {
        let title = "Enter a file path to read from / [Enter to confirm]";
        popup_render(app.file_path_input.clone(), popup_area_std, title, frame);
    }

    // popup for showing any errors (very reusable)
    if let CurrentScreen::Error = &app.current_screen {
        let title = "An error has occurred";
        popup_render(app.error.clone(), popup_area_std, title, frame);
    }
    
    if let CurrentScreen::DeleteUser = &app.current_screen {
        let title = String::from("Enter the uuid for the user you want to delete");
        popup_render(app.user_to_delete_str.clone(), user_enter_layout[3], title, frame);
    }
}

// helper for making popups themselves
fn popup_render<'a, T, A>(text: T, location: Rect, title: A, frame: &mut Frame) 
where 
    T: Into<Text<'a>>,
    A: Into<Line<'a>>
{
    let popup_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Gray).fg(Color::Black));

    let text = Paragraph::new(text).block(popup_block).wrap(Wrap { trim: false });
    frame.render_widget(text, location);
}

// helper for making popups area
fn centered_rect(percent_x: u16, r: Rect) -> Rect {
    // Cut given rectangle into three pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1)
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2)
        ])
        .split(popup_layout[1])[1]
}
