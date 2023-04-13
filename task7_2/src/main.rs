use std::collections::HashSet;

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut state = matrix.clone();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            state[j][i] = matrix[i][j]
        }
    }

    state
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..matrix.len() {
        let joined_string = matrix[i].iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
        println!("|{joined_string}|");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
