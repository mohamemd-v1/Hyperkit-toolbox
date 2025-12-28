use std::{ fs, io::* ,path};
use colored::*;
use evalexpr::*;
use chrono::*;
use crossterm::{terminal::* , event::* , execute , };

use tar::{Archive};
use crate::backend::{clean::read_file_cont, commands::{self, GITHUBLINK}, safe::Safe, standard::tell};
use base64::{prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE}, *};

pub fn calc (math:String) {
    let path = tell();

    let e = match eval(&math) {
        Ok(t) => t,
        
        Err(error) => {
            println!("[{path:?}]>{}: due to {error:?}" , "Error".red() );
            return;
        }
    };

    println!("[{path:?}]~>[ \x1b[34m{e}\x1b[0m ]");
}


pub fn time() {
    let path= tell();

    let time = Local::now();
    println!("[{path:?}]~>[{time}]");
}


pub fn ship( ttype:String, flag:String , the_name_of_the_file:String , output_name:String) -> std::io::Result<()> {
  use tar::{Builder};
  let path = tell();
    match ttype.trim() {
        "tar" => { 
            match flag.trim() {
                "--load" => {
                    let mut open = fs::File::open(&the_name_of_the_file).safe_w_res("The file is not found")?;
                    let make =  fs::File::create(format!("{}.tar", output_name)).safe_w_res("Couldn`t make the file")?;
                    let ifdir = open.metadata().safe_w_res("The file is not found")?;
                    if ifdir.is_dir() == true {
                        let mut builder1 = Builder::new(make);
                        let _apd = builder1.append_dir_all(&output_name, path::Path::new(&the_name_of_the_file)).safe("Couldn1t build the arcive");
                        let _finsh = builder1.into_inner().safe_mas("Ship completed" , "loaded successfully", "Couldn`t build the archive");
                    }

                    else {
                        let mut builder2 = Builder::new(make);
                        let _ap = builder2.append_file(&output_name, &mut open).safe("Couldn`t build the arcive");
                        let _finsh = builder2.into_inner().safe_mas("Ship completed", "loaded successfully", "Couldn`t build the arcive");
                    }
                }
                "--Unload" => {
                    let open = fs::File::open(the_name_of_the_file).safe_w_res("the file is not found")?;

                    let mut arc = Archive::new(open);
                    arc.unpack(output_name).safe_mas("Ship completed", "Unloaded successfully", "Couldn`t unload the arcive");
                }   
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        _ => {
            println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No type were suplied".red().bold());
        }
    }
    Ok(())
}

pub fn transmute (ttype:String, flag:String , the_name_of_the_file:String , output_name:String) -> std::io::Result<()> {
    let path = tell();
    let readed = read_file_cont(&the_name_of_the_file)?;
    match ttype.trim() {
        "base64-ST" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD.encode(&readed);

                    fs::write(&output_name, enc).safe_mas("transmute","encoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                "--dec" => {
                    let dec = BASE64_STANDARD.decode(&readed.trim()).unwrap_or_default();

                    fs::write(&output_name, dec).safe_mas("transmute", "decoded successfully", "couldn`t write the encoded codic to the file Consider trying abother type");
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        "base64-PD" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD_NO_PAD.encode(&readed);

                    fs::write(&output_name, enc).safe_mas("transmute","encoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                "--dec" => {
                    let dec = BASE64_STANDARD_NO_PAD.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).safe_mas("transmute","decoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        "base64-URL" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_URL_SAFE.encode(&readed);

                    fs::write(&output_name, enc).safe_mas("transmute","encoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                "--dec" => {
                    let dec = BASE64_URL_SAFE.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).safe_mas("transmute","decoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }            
        }
        "hex" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode(&readed);

                    fs::write(&output_name, enc).safe_mas("transmute","encoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).safe_mas("transmute","decoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        "HEX" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode_upper(&readed);

                    fs::write(&output_name, enc).safe_mas("transmute","encoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).safe_mas("transmute","decoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        "url" => {
            match flag.trim() {
                "--enc" => {
                    let enc = urlencoding::encode(&readed).into_owned();

                    fs::write(&output_name, enc).safe_mas("transmute","encoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                "--dec" => {
                    let dec = urlencoding::decode(&readed).unwrap_or_default().into_owned();
            
                    fs::write(&output_name, dec).safe_mas("transmute","decoded successfully","couldn`t write the encoded codic to the file Consider trying abother type");
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        _ => {
            println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No type were suplied".red().bold());
        }
    }
    Ok(())
}



fn get_cursor_row_col(text: &str, cursor_pos: usize) -> (usize, usize) {
    let before_cursor = &text[..cursor_pos];
    let row = before_cursor.matches('\n').count();
    let col = before_cursor.split('\n').last().unwrap_or("").len();
    (row, col)
}

fn get_pos_from_row_col(text: &str, target_row: usize, target_col: usize) -> usize {
    let lines: Vec<&str> = text.split('\n').collect();
    
    let mut pos = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == target_row {
            return pos + target_col.min(line.len());
        }
        pos += line.len() + 1;
    }
    
    text.len()
}

fn redraw_from_cursor(text: &str, cursor_pos: usize) -> std::io::Result<()> {
    let remaining = &text[cursor_pos..];
    let show = remaining.replace('\n', "\n\r");

    execute!(stdout(), crossterm::cursor::SavePosition)?;

    print!("{}", show);
    

    execute!(stdout(), Clear(ClearType::FromCursorDown))?;
   
    execute!(stdout(), crossterm::cursor::RestorePosition)?;
    
    stdout().flush()?;
    Ok(())
}

pub fn vortex(file_p: &str) -> std::io::Result<()> {

    let open = fs::File::open(&file_p).safe_w_res(&file_p)?;
    let de = open.metadata().safe_w_res(&file_p)?;

    if de.is_file() == true {

    execute!(stdout(), Clear(ClearType::All)).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
    commands::clean()?;

    execute!(stdout(), EnterAlternateScreen).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
    println!("{}{file_p}", "~".bright_magenta().bold());

    let cont = read_file_cont(&file_p)?;
    
    enable_raw_mode().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());    
    
    let mut text = String::from(&cont);
    let mut cursor_pos: usize = text.len();
    
    if !text.is_empty() {
        let show = text.replace('\n', "\n\r");
        print!("{}", &show);
        stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
    }

    loop {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE
            }) => {
                break;
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Char('s'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                fs::write(&file_p, &text).safe("Couldn't write to the file");
                break;
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Char(c), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                text.insert(cursor_pos, c);
                
                redraw_from_cursor(&text, cursor_pos).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                cursor_pos += 1;
                execute!(stdout(), crossterm::cursor::MoveRight(1))
                    .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Enter, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                text.insert(cursor_pos, '\n');
                cursor_pos += 1;
                
                execute!(stdout(), Clear(ClearType::FromCursorDown))
                    .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                let remaining = &text[cursor_pos..];
                let show = remaining.replace('\n', "\n\r");
                
                print!("\r\n{}", show);
                
                let lines_below = remaining.matches('\n').count();
                if lines_below > 0 {
                    execute!(stdout(), crossterm::cursor::MoveUp(lines_below as u16))
                        .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                }
                
                execute!(stdout(), crossterm::cursor::MoveToColumn(0))
                    .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Backspace, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    let deleted_char = text.remove(cursor_pos);
                    
                    if deleted_char == '\n' {
                        
                        execute!(stdout(), crossterm::cursor::MoveUp(1))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        
                        let lines: Vec<&str> = text[..cursor_pos].split('\n').collect();
                        let col = if let Some(last_line) = lines.last() {
                            last_line.len()
                        } else {
                            0
                        };
                        
                        execute!(stdout(), crossterm::cursor::MoveToColumn(col as u16))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        
                        execute!(stdout(), Clear(ClearType::FromCursorDown))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        
                        let remaining = &text[cursor_pos..];
                        let show = remaining.replace('\n', "\n\r");
                        print!("{}", show);

                        let lines_printed = remaining.matches('\n').count();
                        
                        if lines_printed > 0 {
                            execute!(stdout(), crossterm::cursor::MoveUp(lines_printed as u16))
                                .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        }
                        execute!(stdout(), crossterm::cursor::MoveToColumn(col as u16))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        
                    } else {
                        execute!(stdout(), crossterm::cursor::MoveLeft(1))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        
                        let remaining = &text[cursor_pos..];
                        let current_line_end = remaining.find('\n').unwrap_or(remaining.len());
                        let rest_of_line = &remaining[..current_line_end];
                        
                        print!("{} ", rest_of_line);
                        
                        let move_back = rest_of_line.len() + 1;
                        execute!(stdout(), crossterm::cursor::MoveLeft(move_back as u16))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    }
                    
                    stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                }
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Left, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                if cursor_pos > 0 {

                    if text.chars().nth(cursor_pos - 1) == Some('\n') {
                        cursor_pos -= 1;
                        
                        execute!(stdout(), crossterm::cursor::MoveUp(1))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        
                        let lines: Vec<&str> = text[..cursor_pos].split('\n').collect();
                        if let Some(last_line) = lines.last() {
                            execute!(stdout(), crossterm::cursor::MoveToColumn(last_line.len() as u16))
                                .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                        }
                    } else {
                        cursor_pos -= 1;
                        execute!(stdout(), crossterm::cursor::MoveLeft(1))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    }
                    stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                }
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Right, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                if cursor_pos < text.len() {
                    if text.chars().nth(cursor_pos) == Some('\n') {
                        cursor_pos += 1;
                        
                        print!("\r\n");
                    } else {
                        cursor_pos += 1;
                        execute!(stdout(), crossterm::cursor::MoveRight(1))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    }
                    stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                }
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Up, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                let (row, col) = get_cursor_row_col(&text, cursor_pos);
                if row > 0 {
                    let new_pos = get_pos_from_row_col(&text, row - 1, col);
                    let (_, new_col) = get_cursor_row_col(&text, new_pos);
                    
                    cursor_pos = new_pos;
                    execute!(stdout(), crossterm::cursor::MoveUp(1))
                        .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    
                    if new_col != col {
                        execute!(stdout(), crossterm::cursor::MoveToColumn(new_col as u16))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    }
                    
                    stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                }
            }
            
            Event::Key(KeyEvent { 
                code: KeyCode::Down, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }) => {
                let (row, col) = get_cursor_row_col(&text, cursor_pos);
                let total_rows = text.matches('\n').count();
                
                if row < total_rows {
                    let new_pos = get_pos_from_row_col(&text, row + 1, col);
                    let (_, new_col) = get_cursor_row_col(&text, new_pos);
                    
                    cursor_pos = new_pos;
                    execute!(stdout(), crossterm::cursor::MoveDown(1))
                        .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    
                    if new_col != col {
                        execute!(stdout(), crossterm::cursor::MoveToColumn(new_col as u16))
                            .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                    }
                    
                    stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                }
            }
            Event::Key(KeyEvent { code:KeyCode::Char(cap), modifiers:KeyModifiers::SHIFT, kind:KeyEventKind::Press, state:KeyEventState::NONE }) => {
                text.insert(cursor_pos, cap);
                
                redraw_from_cursor(&text, cursor_pos).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                cursor_pos += 1;
                execute!(stdout(), crossterm::cursor::MoveRight(1))
                    .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
            }
            Event::Key(KeyEvent { code:KeyCode::Char(caps_lock), modifiers:KeyModifiers::NONE, kind:KeyEventKind::Press, state:KeyEventState::CAPS_LOCK }) => {
                text.insert(cursor_pos, caps_lock);
                
                redraw_from_cursor(&text, cursor_pos).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                cursor_pos += 1;
                execute!(stdout(), crossterm::cursor::MoveRight(1))
                    .safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
                
                stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
            }
            
            _ => {}
        }
    }
}
    execute!(stdout(), LeaveAlternateScreen).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
    disable_raw_mode().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
    Ok(())
}
