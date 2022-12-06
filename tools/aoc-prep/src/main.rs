use clap::{Parser, command};


/// Program to generate aoc template projects
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// AOC puzzle year 
    #[arg(short,long,default_value_t=2021)]
    year: u32,
    /// AOC puzzle day
    #[arg(short,long)]
    day: u32,
}

struct 

fn find_workspace(args: &Args) {
    for path in  std::env::current_dir().unwrap().ancestors() {
        let cargo_toml = path.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml).unwrap();
            let mut doc : toml_edit::Document = content.parse().unwrap();

            if doc.contains_key("workspace") {
                println!("Found workspace at {}", cargo_toml.display());

                let workspace = doc.get_mut("workspace").unwrap();
                let members = workspace.get_mut("members").unwrap().as_array_mut().unwrap();

                let new_path = format!("challenges/{}/day{}", args.year, args.day); 
                members.push(&new_path);

                let default_members = workspace.get_mut("default-members").unwrap().as_array_mut().unwrap();

                assert_eq!(1, default_members.len());
                
                default_members.replace(0, &new_path);
                println!("{}", doc.to_string());
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    find_workspace(&args);

    println!("Hello, world!");
}
