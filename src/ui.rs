// sorting the user list
use itertools::Itertools;

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
        Block, Borders, List, ListItem, Paragraph, Wrap
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

pub fn ui(frame: &mut Frame, app: &App) {
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
        format!("Simple User System v{}", &app.version), 
        Style::default().fg(Color::Red)
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

// -- middle block layout and content --
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(chunks[1]);

    // layout for user input fields
    let user_enter_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
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

        let username_value = Paragraph::new(app.username.clone()).block(user_field).wrap(Wrap { trim: false });
        frame.render_widget(username_value, user_enter_layout[0]);

        let email_value = Paragraph::new(app.email.clone()).block(email_field).wrap(Wrap { trim: false });
        frame.render_widget(email_value, user_enter_layout[1]);

        let admin_value = Paragraph::new(app.admin.clone().to_string()).block(admin_field).wrap(Wrap { trim: false });
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

    // list of users sorted by uuid number
    let mut list_users = Vec::<ListItem>::new();

    let list_users_clone = app.user_list.iter().clone();

    for pair in list_users_clone.sorted() {
        list_users.push(ListItem::new(Line::from(Span::styled(
            format!("{: <6} : {}", pair.0, pair.1), 
            Style::default().fg(Color::Yellow)
        ))));
    }

    let list = List::new(list_users);

    frame.render_widget(list, list_chunks[1]);

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
        match &app.current_screen {
            CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
            CurrentScreen::LoadingFromFile => Span::styled("Loading From File", Style::default().fg(Color::Magenta)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
            CurrentScreen::Error => Span::styled("Error", Style::default().bg(Color::Red).fg(Color::Black).bold())
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
                "[q] to quit / [e] to make new user / [r] to load from file", 
                key_hint_style
            ),
            CurrentScreen::Editing => Span::styled(
                "[Tab] to switch fields / [Esc] to cancel / [Enter] to complete / [s] to save (when on admin)", 
                key_hint_style
            ),
            CurrentScreen::LoadingFromFile => Span::styled(
                "WARNING: THIS WILL DELETE ALL ADDED USERS. DO NOT USE IN MIDDLE OF RUNTIME",
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Exiting => Span::styled(
                "[Y/Enter] to quit / [N] to quit and discard / [Esc] to cancel", 
                key_hint_style
            ),
            CurrentScreen::Error => Span::styled(
                "[Enter] to dismiss", 
                key_hint_style
            )
        }
    };

    let key_note_footer = Paragraph::new(Line::from(current_key_hints))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(mode_footer, footer_block[0]);
    frame.render_widget(key_note_footer, footer_block[1]);

    // popup for entering file path to read from
    if let CurrentScreen::LoadingFromFile = &app.current_screen {
        let area = centered_rect(40, 10, frame.area());

        let popup_block = Block::default()
            .title("Enter a file path to read from / [Enter to confirm]")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Gray).fg(Color::Black));

        let file_path_value = Paragraph::new(app.file_path_input.clone()).block(popup_block).wrap(Wrap { trim: false });
        frame.render_widget(file_path_value, area);
    }

    // popup for showing any errors (very reusable)
    if let CurrentScreen::Error = &app.current_screen {
        let area = centered_rect(40, 10, frame.area());

        let popup_block = Block::default()
            .title("An error has occurred")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Gray).fg(Color::Black));

        let file_path_value = Paragraph::new(app.error.clone()).block(popup_block).wrap(Wrap { trim: false });
        frame.render_widget(file_path_value, area);
    }

}

// helper for making popups
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut given rectangle into three pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2)
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

