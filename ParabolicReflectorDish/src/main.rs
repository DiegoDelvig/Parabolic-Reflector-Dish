use std::fs::read_to_string;



fn main()
{
    let mut rows: Vec<Vec<char>> = Vec::new();

    for line in read_to_string("src/input.txt").unwrap().lines() {
        let mut cols: Vec<char> = Vec::new();
        println!("{}", line);
        for c in line.chars() {
            cols.push(c);
        }
        rows.push(cols);
    }

    for _ in 0..10 {
        loop 
        {
            let mut n = 0;


            for i in 0..rows.len() {
                for j in 0..rows[i].len() {
                    if i == 0 { break; }
                    
                    if rows[i-1][j] == '.' && rows[i][j] == 'O' {
                        rows[i-1][j] = 'O';
                        rows[i][j] = '.';
                        n += 1;
                    }
                }
            }
            

            if n == 0 { 
                println!("\n \n");
                let mut score = 0;
                for i in 0..rows.len() {
                    for j in 0..rows[i].len() {
                        print!("{}", rows[i][j]);
                        if rows[i][j] == 'O' {
                            score += rows.len() - i;
                        }
                    }
                    println!("");
                }

        
                // println!("{}", score);
                break;
            }
        }
    }
}
