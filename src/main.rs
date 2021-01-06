use structopt::StructOpt;
use std::fs;
use std::path::Path;

#[derive(Debug, StructOpt)]
struct Cli {
    pub dir_name: String,
}

fn main() -> std::io::Result<()>{
    let args = Cli::from_args();

    let index_content = fs::read_to_string("./bin/template/index.html")
                            .expect("Something went wrong reading the file");
    
    let style_content = fs::read_to_string("./bin/template/style.css")
                            .expect("Something went wrong reading the file");

    let index_path_string = format!("./{}/index.html", &args.dir_name);
    let style_path_string = format!("./{}/style.css", &args.dir_name);

    let index_path = Path::new(&index_path_string);
    let style_path = Path::new(&style_path_string);
    
    match fs::create_dir(&args.dir_name) {
        Ok(_) => print!("\nDiretório {} criado...", &args.dir_name),
        Err(_) => print!("\nNão foi possivel criar o diretorio: {}", &args.dir_name)
    };

    fs::File::create(&index_path)
              .expect("Não foi possivel criar o index.html");
    
    fs::File::create(&style_path)
              .expect("Não foi possivel criar o style.css");


    fs::write(index_path, index_content).expect("Unable to write data");
    fs::write(style_path, style_content).expect("Unable to write data");

    Ok(())
}
