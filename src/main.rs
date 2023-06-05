// enum Color {
//   RED,
//   GREEN,
//   BLUE,
//   YELLOW,
// }
// impl Color {
//   fn is_green(&self) -> bool {
//     if let Color::GREEN = self {
//       return true;
//     }
//     return false;
//   }

//   fn is_green_parts(&self) -> bool {
//     match self {
//       Color::RED => false,
//       Color::GREEN => false,
//       Color::BLUE => true,
//       Color::YELLOW => true,
//     }
//   }
// }

// fn print_colors(color: Color) {
//   match color {
//     Color::RED => println!("red"),
//     Color::GREEN => println!("green"),
//     Color::BLUE => println!("blue"),
//     Color::YELLOW => println!("yellow"),
//   }
// }

// fn practice(num: Vec<usize>, index: usize) -> usize {
//   return num.get(index).unwrap_or(&index) * 5;
// }

fn main() {
  let file_name = std::env::args().nth(1).expect("where is filename");
  let file = std::fs::read_to_string(file_name).expect("no file found");

  file.lines().for_each(|line| {
    if let Ok(value) = line.parse::<usize>() {
      println!("{}", value)
    } else {
      println!("Line is not a number")
    }
  })
}

// curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
