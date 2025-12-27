pub mod commands {
use colored::Colorize;

pub const GITHUBLINK:&str = "https://github.com/mohamemd-v1/Shell-like-toolbox-.git";

use crate::backend::{safe::{Safe, SafeT}, standard::{input, tell}};
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
            println!("   *{} {} {} to use the built-in calculator" , "enter".green() , "calc".bright_blue() , "<Math>".purple());
            println!("   *{} {} to know the time" , "enter".green() , "time".bright_blue() );
            println!("   *{} {} {} {} {} {} to make/extract tar files" , "enter".green() , "ship".bright_blue() , "<Type>".bright_purple(), "<Flag>".bright_yellow() , "<File-Name>".bright_cyan() , "<File-Outpot-Name>".bright_magenta());
            println!("   *{} {} {} {} {} {} to encode/decode files" , "enter".green() , "transmute".bright_blue() , "<Type>".bright_purple(), "<Flag>".bright_yellow() , "<File-Name>".bright_cyan() , "<File-Outpot-Name>".bright_magenta());

        }
        "--about" => {
            println!("{}HyperKit is a modern, extensible, and lightweight command-line environment built to unify the tools you need into one powerful workspace." , "@".bright_green() )
        }
        "ship" => {
                println!("   *[{}: tar][{}: --load {} --Unload {}]" , "Types".bright_green().bold() , "flags".bright_blue().bold(), "to make an arcive".bright_purple().bold() , "to extract an arcive".bright_yellow().bold());
        }
        "transmute" => {
                println!("   *[{}: base64-PD<pedding> , base64-ST<standerd> , base64-URL<url> , hex<low-case hex> , Hex<uper-case hex> ][{}: --enc {} --dec {}]" , "Types".bright_green().bold() , "flags".bright_blue().bold(), "to encode a file".bright_purple().bold() , "to decode a file".bright_yellow().bold());
        }
        _ => {
            println!("   *{} {} {} to see all the commands , {} to list all the available built in apps , {} for about" , "Enter".green()  , "help".red() ,"--commands".bright_purple() , "--built-in-apps".bright_purple() , "--about".bright_purple() );
         } 
       }    
    }

   
    pub fn clean() -> std::io::Result<()> {
       print!("\x1B[2J\x1B[1;1H");
       stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());
       Ok(())
    }

    
    pub fn go(t:String) -> std::io::Result<()> { 
        let path = PathBuf::from(&t);
        env::set_current_dir(&path).safe_mas("Go" , "directory has been changed successfully", &t);
        Ok(())
    }
    
  
    pub fn  wh() -> std::io::Result<()> {
        let path = tell();

        let wh = env::current_dir().safe_w_res(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str())?;
        println!("[{path:?}]~>{}\x1b[34m{}\x1b[0m" ,"~".bright_green(), wh.display());
        Ok(())
    }

    
    pub fn see () -> std::io::Result<()> {
        let path = tell();

        let cur = env::current_dir().safe_w_res("Dir Not Found")?;
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

   
    pub fn peek(file:String) -> std::io::Result<()> {
        let path = tell();
        let fe = File::open(&file);

        if let Err(e) = &fe {
            if e.kind() == ErrorKind::NotFound {
                println!("[{path:?}]~>{}: couldn't open the file due to [{}]" , "Error".red().bold() , "NotFound error".red().bold());
                println!("[{path:?}]~>Do you want to make this file?");
                print!("[{path:?}]~>({}/{}):" , "Y".green() , "N".red());
                stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());

                let yesorno = input();

                if yesorno == "Y" {
                    fs::File::create(&file).safe(&file);
                }
            }
        };

        let fe = &mut fe?;

        let mut r = String::new();
        let _read =  fe.read_to_string(&mut r).safe(&file);

        println!("\x1b[34m{}\x1b[0m" , r);
        Ok(())
    }

    
    pub fn mk(path:String) -> std::io::Result<()> {
        fs::create_dir(&path).safe_mas("Mk", "Directory created successfully" , &path);
        Ok(())
    }

    
    pub fn burn(path:String) -> std::io::Result<()> {
        let tell = tell();

        let burn = fs::remove_file(&path);

        if burn.is_ok() == true {
                println!("[{path:?}]~>{}: [{}]" , "burn".bright_green().bold() , "file has been burned successfully".bright_green().bold());
            }

        if let Err(e) = burn {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    let burn_dir = fs::remove_dir(&path);

                    if let Err(e) = burn_dir {
                        if e.kind() == ErrorKind::DirectoryNotEmpty {

                            print!("[{tell:?}]~>[{}/{}]: the Directory is Not Empty do you stil want to delete it? >> " , "Y".bold().green() , "N".bold().red());
                            stdout().flush().safe(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str());

                            let yesorno = input();
                            if yesorno == "Y" {
                                fs::remove_dir_all(&path).safe_mas("burn", "Directory has been burned successfully", &path);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn rn(f:String , t:String) -> std::io::Result<()> {
        fs::rename(&f, &t).safe_mas("rn", "Renamed successfully",format!("{}+{}" , &f , &t).as_str());
        Ok(())
    }

    pub fn clone(f:String , t:String) -> std::io::Result<()> {
        fs::copy(&f, &t).safe_mas("clone", "Copied!", format!("{}+{}" , &f , &t).as_str());
        Ok(())
    }

    pub fn forge(file:String) -> std::io::Result<()> {
        fs::File::create(&file).safe_mas("Forge completed!", "File created" , &file);
        Ok(())
    }

    pub fn run(app:String) -> std::io::Result<()> {
        let path = tell();
        let run = process::Command::new(&app).output().safe_w_res(&app)?;

        println!("[{path:?}]~>\x1b[34m{}\x1b[0m" , String::from_utf8_lossy(&run.stdout));
        Ok(())
    }

   
    pub fn mv(name:String , path:String) -> std::io::Result<()> {
        fs::copy(&name, format!("{}/{}" , &path , &name)).safe(format!("{}/{}" , &path, &name).as_str());

        let delete_eveadnice = fs::remove_file(&name);

        if let Err(e) = delete_eveadnice {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    fs::remove_dir_all(&name).safe_mas("mv", "moving completed" , format!("{}/{}" , &path, &name).as_str());
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

    use crate::backend::safe::SafeT;

    pub fn input() -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).safe("Parsing Error");
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
    use std::{fs::{File, Metadata}, io::ErrorKind, ops::Add, path::PathBuf, process::Output};

    use colored::Colorize;

    use crate::backend::{standard::tell};

    pub trait Safe {
        fn safe(self , err_res:&str);
        fn safe_mas(self , mas1:&str , mas1:&str , err_res:&str);
        fn safe_w_res(self , err_res:&str) -> Self;
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:&str) -> Self;
    }

    pub trait SafeT<T> {
        fn safe(self , err_res:&str);
        fn safe_mas(self , mas1:&str , mas1:&str , err_res:&str);
    }

    pub fn errmes(e:&std::io::Error , err_res:&str) {
        let path = tell();

        match e.kind() {
            ErrorKind::NotFound => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the requested resource was not found".bright_red().bold() , err_res.bright_yellow().bold());
            }
            ErrorKind::IsADirectory => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "is a directory".bright_red().bold() , err_res.bright_yellow().bold());
            }
            ErrorKind::Interrupted => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "operation interrupted".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::InvalidInput => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "invalid input".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::DirectoryNotEmpty => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "directory not empty".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::InvalidFilename => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Invalid file name".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::FileTooLarge => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "The file is to large".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::NotADirectory => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "is not a directory".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::PermissionDenied => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Permission denied".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::ReadOnlyFilesystem => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Read only file".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::InvalidData => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Invalid data".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::StorageFull => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Storage is full try to free up some storage and try again".bright_red().bold() , err_res.bright_yellow());
            }
            ErrorKind::Unsupported => {
                println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Unsupported operation".bright_red().bold() , err_res.bright_yellow());
            }
            _ => {
                eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold())
            }
        }
    }


    impl Safe for std::io::Result<File> {
        fn safe(self , err_res:&str) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str) {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:&str) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:&str) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }
    
    impl<T: Add<Output = T> + Copy > SafeT<T> for std::io::Result<T> {
        fn safe(self , err_res:&str) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str )  {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
    }

    impl Safe for std::io::Result<PathBuf> {
        fn safe(self , err_res:&str)  {
             match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str) {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:&str) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:&str) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }

    impl Safe for std::io::Result<()> {
        fn safe(self , err_res:&str) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str) {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:&str) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:&str) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }
    
    impl Safe for std::io::Result<Metadata> {
        fn safe(self , err_res:&str ) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str) {
            let path = tell();

             match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:&str) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:&str) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }

    impl Safe for std::io::Result<Output> {
        fn safe(self , err_res:&str ) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str) {
            let path = tell();

             match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:&str) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:&str) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(&e, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }
}

pub mod clean {
    use std::{fs::{File }, io::Read};
    use crate::{backend::safe::{Safe, SafeT}};
    
    pub fn read_file_cont(path:&str) -> std::io::Result<String> {
        let mut txtf = File::open(&path).safe_w_res(&path)?;
        let mut readed = String::new();
        txtf.read_to_string(&mut readed).safe(&path);
        return Ok(readed);
    }
}
