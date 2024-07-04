use std::fs;
use std::io;
use std::env::args;
use zip::ZipArchive;

fn main(){
    
    std::process::exit(realmain());
}

fn realmain() -> i32{
    let collectedargs: Vec<_> = args().collect();
    
    if collectedargs.len() < 2{
        eprintln!("Usage: {} <filname>", collectedargs[0]);
        return 1;
    }

    let fname = std::path::Path::new(&*collectedargs[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        
        let output = match file.enclosed_name(){
            Some(path) => path.to_owned(),
            None => continue
        };

        {
            let comment = file.comment();

            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to: \"{}\"", i, output.display());
            fs::create_dir_all(&output).unwrap();
        }
        else {
            println!("File {} extracted to :\"{}\" ({} bytes)", i, output.display(), file.size());
            if let Some(p) = output.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&output).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&output, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    return 0;
}