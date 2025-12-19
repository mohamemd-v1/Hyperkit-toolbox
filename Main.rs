mod backend;
mod apps;
use crate::backend::safe::Safe;
use crate::backend::{commands, standard::tell, tokenization::* };
use std::{env::*};
use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

const GITHUBLINK:&str = "https://github.com/mohamemd-v1/Shell-like-toolbox-.git";

fn main() {
    println!("*{} {} {} to see all the commands , {} to list all the available built in apps , {} for about" , "Enter".green()  , "help".red() ,"--commands".bright_purple() , "--built-in-apps".bright_purple() , "--about".bright_purple() );
    let home = match home_dir() {
        Some(h) => h,
        None => {
            println!(">{}: home dir does not exsit?" , "Error".red());
            return;
        }
    };

    set_current_dir(home).safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());

    let mut def = match DefaultEditor::new() {
        Ok(o) => o,
        Err(e) => {
            let tell = tell();
            eprintln!("[{tell:?}]~>{}: due to {e}" , "Error".red());
            return;
        }
    };

  loop {
    let path = tell();
    
    let f  = format!("[{path:?}]~>");
    
    let void = match def.readline(&f) {
        Ok(o) => {
            def.add_history_entry(o.as_str()).unwrap();
            o
        },

        Err(e) => match e {
            ReadlineError::Interrupted => {
                break;
            }
            _ => {
                eprintln!("[{path:?}]~>{}: due to {e}" , "Error".red());
                return;
            }
        }
    };

    let data = proc(void);
    let tok1 = match data.get(0) {
        Some(t) => t.to_owned(),
            None => {
                continue;
           }
    };

    match tok1.trim() {
        "help" => {
            let tok1 = token(&data, 1);
            commands::help(tok1);
        }
        "clean" => {
            commands::clean().unwrap_or_default();
        }
        "go" => {
            let tok2  = token(&data, 1);
            if &tok2 == "back" {
                commands::go("..".to_string()).unwrap_or_default();
            }
            else {
                let tok2 = token(&data, 1);
                commands::go(tok2).unwrap_or_default();
            }
        }
        "wh" => {
            commands::wh().unwrap_or_default();
        }
        "see" => {
            commands::see().unwrap_or_default();
        }
        "peek" => {
            let tok2 = token(&data, 1);
            commands::peek(tok2).unwrap_or_default();
        }
        "mk" => {
            let tok2 = token(&data, 1);
            commands::mk(tok2).unwrap_or_default();
        }
        "burn" => {
            let tok2 = token(&data, 1);
            commands::burn(tok2).unwrap_or_default();
        }
        "rn" => {
            let tok1 = token(&data, 1);
            let tok2 = token(&data, 2);
            commands::rn(tok1 , tok2).unwrap_or_default();
        }
        "clone" => {
            let tok1 = token(&data, 1);
            let tok2 = token(&data, 2);
            commands::clone(tok1 , tok2).unwrap_or_default();
        }
        "forge" => {
            let tok1 = token(&data, 1);
            let _ = commands::forge(tok1);
        }
        "run" => {
            let tok1 = token(&data , 1);
            commands::run(tok1).unwrap_or_default();
        }
        "cal" => {
            let tok1 = token(&data , 1);
            apps::cul(tok1);
        }
        "time" => {
            apps::time();
        }
        "mv" => {
            let tok1 = token(&data, 1);
            let tok2 = token(&data, 2);
            commands::mv(tok1, tok2).unwrap_or_default();
        }
        "ship" => {
            let ttype = token(&data, 1);
            let flag  = token(&data, 2);
            let fname = token(&data, 3);
            let outname = token(&data, 4);
            apps::ship(ttype, flag , fname , outname).unwrap_or_default();
        }
        "end" => {
            break;
        }
        _ => {
            if !tok1.is_empty() {
                println!("[{path:?}]~>{}: [\x1b[36m{}\x1b[0m] please try again" , "unknown command".bright_red() ,&tok1 );
                continue;
            }
          }
      }
   }
}
