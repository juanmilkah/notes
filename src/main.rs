use std::io::{Read, Write};


fn main(){
    let args:Vec<String> = std::env::args().skip(1).collect(); 
    let file_path = home_dir().join("notes.txt");

    if args.len() > 1{
        match args[0].as_str() {
            "-w" =>{
                if args.len() > 1{
                match write_note(&args[1], &file_path) {
                    Ok(val) => println!("{}", val),
                    Err(e) => eprintln!("{}", e)
                    }
                }else{
                    eprintln!("Missing note argument for -w");
                }
            },
            "-u" =>{
                if args.len() > 2{

                    let index:usize = args[1].parse().expect("Invalid Index!");
                    let new_note = &args[2];
                    match update_note(index -1, &new_note, &file_path){
                        Ok(val) => println!("{}", val),
                        Err(e) => eprintln!("{}", e)
                    }
                }else{
                    eprintln!("Missing index argument for -u");
                }
            },
            "-d" => {
                if args.len() > 1{
                let index: usize = args[1].parse().expect("Invalid Index!");
                match delete_note(index -1, &file_path){
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("{}", e)
                }
                }else{
                    eprintln!("Missing index argument for -d");
                }
        },
            _ => eprintln!("Invalid Command!")
        }
    }
    else if args.len() == 1 && args[0] == "-r"{
        match read_notes(&file_path){
            Ok(val) => println!("{}", val),
            Err(err) => eprintln!("{}", err)
        }
    }else{
        eprintln!("No Arguments provided!");
    }

}

fn home_dir() -> std::path::PathBuf{
    dirs::home_dir().expect("Could not find Home Directory!")
}

fn write_note(note: &str, file_path: &std::path::Path)->Result<String, std::io::Error>{
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(file_path)?;
    file.write_all(&note.as_bytes())?;
    file.write_all(b"\n")?;
    
    Ok(String::from("Note Added!"))
}

fn read_notes(file_path: &std::path::Path)->Result<String, std::io::Error>{
    let mut contents = String::new();
    let mut file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn update_note(index: usize, note: &str, file_path: &std::path::Path) -> Result<String, std::io::Error>{
    let mut notes = read_notes(file_path)?.lines().map(|s| String::from(s)).collect::<Vec<_>>();

    if index >= notes.len(){
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Index out of Range!"));
    }
    notes[index] = String::from(note);
    write_all_notes(&notes, &file_path)?;
    Ok(String::from("Updated Note Successfully!"))

}

fn write_all_notes(notes: &[String], file_path: &std::path::Path)->Result<String, std::io::Error>{
    let mut file = std::fs::File::create(file_path)?;

    for note in notes{
        file.write_all(note.as_bytes())?;
        file.write_all(b"\n")?;

    }
    Ok(String::from("Written all notes!"))
}

fn delete_note(index:usize, file_path: &std::path::Path)->Result<String, std::io::Error>{
    let mut notes = read_notes(file_path)?.lines().map(|s| String::from(s)).collect::<Vec<_>>();

    if index >= notes.len(){
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Index out of Range!"));
    }
        notes.remove(index);
        write_all_notes(&notes, &file_path)?;
        Ok(String::from("Deleted Note!"))
    
}