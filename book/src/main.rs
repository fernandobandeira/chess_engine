use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use shakmaty::Color;

fn main() {
    parse_pgns();
}

fn parse_pgns() {
    // Load all files from the pgns directory
    let paths = fs::read_dir("./book/pgns").unwrap();

    let mut white_wins_moves: Vec<String> = Vec::new();
    let mut black_wins_moves: Vec<String> = Vec::new();
    let mut draw_moves: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();

        // Read the file line by line with the lines() method from std::io::BufRead
        let file = fs::File::open(path).unwrap();
        let reader: BufReader<fs::File> = BufReader::new(file);

        let mut winner: Option<Color> = None;
        let mut parse_moves: bool = false;
        let mut current_moves: String = String::new();

        for line in reader.lines() {
            let line = line.unwrap();

            if line.starts_with("[") {
                if line.starts_with("[Result") {
                    if line == "[Result \"1-0\"]" {
                        winner = Some(Color::White);
                    } else if line == "[Result \"0-1\"]" {
                        winner = Some(Color::Black);
                    } else {
                        winner = None;
                    }
                }
                continue;
            }

            if line.starts_with("1.") {
                parse_moves = true;
            }

            if parse_moves {
                // Parse the move into a string
                // Example line 1.c4 c5 2.Nf3 Nf6 3.Nc3 Nc6 4.g3 d5 5.cxd5 Nxd5 6.Bg2 g6 7.Nxd5 Qxd5 8.O-O Bg7
                // Moves are c4c5 Nf3Nf6 Nc3Nc6 g3d5 cxd5Nxd5 Bg2g6 Nxd5Qxd5 O-OBg7
                let moves: Vec<&str> = line.split(" ").collect();
                for m in moves {
                    let mut m = m.trim();

                    // Check if the game is over
                    if m == "1-0" || m == "0-1" || m == "1/2-1/2" {
                        // Replace double spaces with single spaces
                        current_moves = current_moves.replace("  ", " ");
                        current_moves = current_moves.trim().to_string();

                        if winner == Some(Color::White) {
                            white_wins_moves.push(current_moves);
                        } else if winner == Some(Color::Black) {
                            black_wins_moves.push(current_moves);
                        } else {
                            draw_moves.push(current_moves);
                        }

                        parse_moves = false;
                        current_moves = String::new();
                        break;
                    }

                    if m.contains(".") {
                        // Remove the move number
                        m = m.split(".").collect::<Vec<&str>>()[1];
                    }
                    current_moves.push_str(&m.trim());
                    current_moves.push_str(" ");
                }
            }
        }
    }

    // Write the moves to a file
    let mut file = fs::File::create("./book/output/white_wins_moves.txt").unwrap();
    for m in white_wins_moves {
        file.write_all(m.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }

    let mut file = fs::File::create("./book/output/black_wins_moves.txt").unwrap();
    for m in black_wins_moves {
        file.write_all(m.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }

    let mut file = fs::File::create("./book/output/draw_moves.txt").unwrap();
    for m in draw_moves {
        file.write_all(m.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}