pub mod commands {
use colored::Colorize;

use crate::backend::{safe::{SafeVoid, SafePath, SafeOutpot, SafeNum, SafeFile}, standard::{input, tell}};
use std::{env, fs::{self,File}, io::*,  path::PathBuf  , process};
    pub fn help(helpt:String) {
       match helpt.trim() {
        "--commands" => {
            println!("   *{} {} to end the session" , "enter".green() , "end".bright_blue() );
            println!("   *{} {} to clear the screen" , "enter".green() , "clean".bright_blue() );
            println!("   *{} {} {} to change the dir" , "enter".green() , "go".bright_blue(), "<Dir>".bright_purple() );
            println!("   *{} {} to see the current dir" , "enter".green() , "wh".bright_blue() );
            println!("   *{} {} to show all the files in the current dir" , "enter".green() , "see".bright_blue() );
            println!("   *{} {} {} to see what is inside the file" , "enter".green() , "peek ".bright_blue() ,  "<Path>".bright_purple() );
            println!("   *{} {} {} to delete anything" , "enter".green() , "burn".bright_blue() , "<Path/File>".bright_purple());
            println!("   *{} {} {} {} to copy a file" , "enter".green() , "clone".bright_blue() , "<Name/File>".bright_purple() , "<Nname/File>".bright_yellow());
            println!("   *{} {} {} to create a file" , "enter".green() , "forge".bright_blue() , "<Name>".bright_purple());
            println!("   *{} {} {} to make a dir" , "enter".green() , "mk".bright_blue() , "<Name>".bright_purple());
            println!("   *{} {} {} to run a program" , "enter".green() , "run".bright_blue() , "<App>".bright_purple() );
            println!("   *{} {} {} to move a file from place to another" , "enter".green() , "mv".bright_blue() , "<Name>".bright_purple());
        }
        "--built-in-apps" => {
            println!("   *{} {} {} to use the built-in calculator" , "enter".green() , "<cal>".bright_blue() , "<Math>".purple());
            println!("   *{} {} to know the time" , "enter".green() , "<time>".bright_blue() );
            println!("   *{} {} {} {} {} {} to make/extract tar files" , "enter".green() , "ship".bright_blue() , "<Type>".bright_purple(), "<Flag>".bright_yellow() , "<File-Name>".bright_cyan() , "<File-Outpot-Name>".bright_magenta());
        }
        "--about" => {
            println!("{}HyperKit is a modern, extensible, and lightweight command-line environment built to unify the tools you need into one powerful workspace." , "@".bright_green() )
        }
        _ => {
            println!("   *{} {} {} to see all the commands , {} to list all the available built in apps , {} for about" , "Enter".green()  , "help".red() ,"--commands".bright_purple() , "--built-in-apps".bright_purple() , "--about".bright_purple() );
         } 
       }
    }

    //code:1
    pub fn clean() -> std::io::Result<()> {
       print!("\x1B[2J\x1B[1;1H");
       stdout().flush().safe()?;
       Ok(())
    }

    //code:1
    pub fn go(t:String) -> std::io::Result<()> { 
        let path = PathBuf::from(t);
        env::set_current_dir(&path).safe_mas("Go".to_string() , "directory has been changed successfully".to_string())?;
        Ok(())
    }
    
    //code:1
    pub fn  wh() -> std::io::Result<()> {
        let path = tell();

        let wh = env::current_dir().safe()?;
        println!("[{path:?}]~>{}\x1b[34m{}\x1b[0m" ,"~".bright_green(), wh.display());
        Ok(())
    }

    //code:1
    pub fn see () -> std::io::Result<()> {
        let path = tell();

        let cur = env::current_dir().safe()?;
        let dir = fs::read_dir(cur);

        match dir {
            Ok(w) => {
                for i in w {
                    let dir = match i {Ok(t) => t, Err(e) => {println!("[{path:?}]>Error: due to {e:?}"); return Ok(());}};
                    println!("   {}\x1B[94m{}\x1b[0m" ,"~".bright_green() , dir.file_name().to_string_lossy());
                } 
            }
            Err(error) => {
                println!("[{path:?}]~>{}: due to \x1b[33m{error:?}\x1b[0m" , "Error".red());
                return Ok(());
            }
        }
        Ok(())
    }

    //code:1
    pub fn peek(file:String) -> std::io::Result<()> {
        let path = tell();
        let fe = File::open(&file).safe();

        if let Err(e) = &fe {
            if e.kind() == ErrorKind::NotFound {
                println!("[{path:?}]~>{}: couldn't open the file due to [{}]" , "Error".red().bold() , "NotFound error".red().bold());
                println!("[{path:?}]~>Do you want to make this file?");
                print!("[{path:?}]~>({}/{}):" , "Y".green() , "N".red());
                stdout().flush().safe()?;

                let yesorno = input();

                if yesorno == "Y" {
                    fs::File::create(&file).safe()?;
                }
            }
        };

        let fe = &mut fe?;

        let mut r = String::new();
        let _read =  fe.read_to_string(&mut r)._safe()?;

        println!("\x1b[34m{}\x1b[0m" , r);
        Ok(())
    }

    //code:1
    pub fn mk(path:String) -> std::io::Result<()> {
        fs::create_dir(&path).safe_mas("Mk".to_string(), "Directory created successfully".to_string())?;
        Ok(())
    }

    //code:1
    pub fn burn(path:String) -> std::io::Result<()> {
        let tell = tell();

        let burn = fs::remove_file(&path).safe_mas("burn".to_string(), "File has been burned successfully".to_string());
        if let Err(e) = burn {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    let burn_dir = fs::remove_dir(&path).safe_mas("burn".to_string(), "Directory has been burned successfully".to_string());
                    if let Err(e) = burn_dir {
                        if e.kind() == ErrorKind::DirectoryNotEmpty {

                            print!("[{tell:?}]~>[{}/{}]: the Directory is Not Empty do you stil want to delete it? >> " , "Y".bold().green() , "N".bold().red());
                            stdout().flush().safe()?;

                            let yesorno = input();
                            if yesorno == "Y" {
                                fs::remove_dir_all(&path).safe_mas("burn".to_string(), "Directory has been burned successfully".to_string())?;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    //code:1
    pub fn rn(f:String , t:String) -> std::io::Result<()> {
        fs::rename(f, t).safe_mas("rn".to_string(), "Renamed successfully".to_string())?;
        Ok(())
    }

    //code:1
    pub fn clone(f:String , t:String) -> std::io::Result<()> {
        fs::copy(f, t).safe_mas("clone".to_string(), "Copied!".to_string())?;
        Ok(())
    }

    //code:1
    pub fn forge(file:String) -> std::io::Result<()> {
        fs::File::create(file).safe_mas("Forge completed!".to_string(), "File created".to_string())?;
        Ok(())
    }

    //code:1
    pub fn run(app:String) -> std::io::Result<()> {
        let path = tell();
        let run = process::Command::new(app).output().safe()?;
        
        println!("[{path:?}]~>\x1b[34m{}\x1b[0m" , String::from_utf8_lossy(&run.stdout));
        Ok(())
    }

    //code:1
    pub fn mv(name:String , path:String) -> std::io::Result<()> {
        fs::copy(&name, format!("{}/{}" , &path , &name))._safe()?;

        let delete_eveadnice = fs::remove_file(&name).safe_mas("mv".to_string(), "moving completed".to_string());
        if let Err(e) = delete_eveadnice {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    fs::remove_dir_all(&name).safe_mas("mv".to_string(), "moving completed".to_string())?;
                }
                _ => {}
            }
        }

        Ok(())
    }
} 

pub mod standard {
    use std::{ env::*, io::{stdin}, path::PathBuf};
    use colored::Colorize;

    use crate::backend::safe::SafeNum;

    pub fn input() -> String {
        let mut input = String::new();
        stdin().read_line(&mut input)._safe().unwrap_or_default();
        let input = input.trim().to_string();

        input
    }

    pub fn tell() -> PathBuf {
        let path = match current_dir() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("([Error]~>{}: due to {}" , "Error".red() , e);
                PathBuf::new() }};
        path
    }
}

pub mod tokenization {
    use colored::Colorize;

    pub fn proc(input:String) -> Vec<String> {
        let split = match shlex::split(&input) {
            Some(o) => o,
            None => {
                eprintln!("([Error]~>{}: due to Tokenizer is not working!!" , "Error".red());
                return Vec::new();
            }
        };
        split
    }
    pub fn token(data:&[String] , index:usize ) -> String {
        let token = match data.get(index).map(|s| s.as_str()) {
            Some(t) => t,
            None => {
                return "".to_string();
            }
        };
        token.to_string()
    }
}

pub mod safe {
    use std::{fs::{File, Metadata}, ops::Add, path::PathBuf, process::{Output}};

    use colored::Colorize;

    use crate::backend::standard::tell;

    pub trait SafeVoid {
        fn safe(self) -> std::io::Result<()>;
        fn safe_mas(self , mas1:String , mas1:String) -> std::io::Result<()>;
    }

    pub trait SafeFile {
        fn safe(self) -> std::io::Result<File>;
        fn safe_mas(self , mas1:String , mas2:String) -> std::io::Result<File>;
    }

    pub trait SafeNum<T> {
        fn _safe(self) -> std::io::Result<T>;
        fn safe_mas(self , mas1:String , mas2:String) -> std::io::Result<T>;
    }

    pub trait SafeOutpot {
        fn safe(self) -> std::io::Result<Output>;
        fn _safe_mas(self , mas1:String , mas2:String) -> std::io::Result<Output>;
    }

    pub trait SafePath {
        fn safe(self) -> std::io::Result<PathBuf>;
        fn _safe_mas(self , mas1:String , mas2:String) -> std::io::Result<PathBuf>;
    }

    pub trait SafeMeta {
        fn safe(self) -> std::io::Result<Metadata>;
        fn _safe_mas(self , mas1:String , mas2:String) -> std::io::Result<Metadata>;
    }

    impl SafeFile for std::io::Result<File> {
        fn safe(self) -> std::io::Result<File> {
            let path = tell();

            match self {
                Ok(o) => return Ok(o),
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
        fn safe_mas(self , mas1:String , mas2:String) -> std::io::Result<File> {
            let path = tell();

            match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
    }
    
    impl<T: Clone + Add<Output = T> + Copy> SafeNum<T> for std::io::Result<T> {
        fn _safe(self) -> std::io::Result<T> {
            let path = tell();

            match self {
                Ok(o) => {
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
        fn safe_mas(self , mas1:String , mas2:String) -> std::io::Result<T> {
            let path = tell();

            match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
    }

    impl SafeOutpot for std::io::Result<Output> {
        fn safe(self) -> std::io::Result<Output> {
            let path = tell();

            match self {
                Ok(o) => {
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
        fn _safe_mas(self , mas1:String , mas2:String) -> std::io::Result<Output> {
            let path = tell();

            match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
    }
    impl SafePath for std::io::Result<PathBuf> {
        fn safe(self) -> std::io::Result<PathBuf> {
            let path = tell();

             match self {
                Ok(o) => {
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
        fn _safe_mas(self , mas1:String , mas2:String) -> std::io::Result<PathBuf> {
            let path = tell();

            match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
    }

    impl SafeVoid for std::io::Result<()> {
        fn safe(self) -> std::io::Result<()> {
            let path = tell();

            match self {
                Ok(o) => {
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
        fn safe_mas(self , mas1:String , mas2:String) -> std::io::Result<()> {
            let path = tell();

            match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
    }

    impl SafeMeta for std::io::Result<Metadata> {
        fn safe(self) -> std::io::Result<Metadata> {
            let path = tell();

            match self {
                Ok(o) => {
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
        fn _safe_mas(self , mas1:String , mas2:String) -> std::io::Result<Metadata> {
            let path = tell();

             match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold() );
                    return Err(e);
                }
            }
        }
    }
  }
