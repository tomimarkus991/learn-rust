fn main(){
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join("lines");
    let file = std::fs::read_to_string(file_path).unwrap();

    file.lines().for_each(|line| println!("{}", line))
}


// curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh