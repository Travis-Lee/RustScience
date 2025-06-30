fn main() {
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6]
    ];
    
    let matrix_b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12]
    ];
    
    match multiply_matrices(&matrix_a, &matrix_b) {
        Ok(result) => {
            println!("Matrix A:");
            print_matrix(&matrix_a);
            
            println!("\nMatrix B:");
            print_matrix(&matrix_b);
            
            println!("\nMulti Matrix Result (A × B):");
            print_matrix(&result);
        },
        Err(e) => println!("ERROR: {}", e)
    }
}

fn multiply_matrices(a: &[Vec<i32>], b: &[Vec<i32>]) -> Result<Vec<Vec<i32>>, String> {
    if a[0].len() != b.len() {
        return Err(format!("matrix dimension not matching : A is {}×{} Matrix，B is {}×{} Matrix", 
                          a.len(), a[0].len(), b.len(), b[0].len()));
    }
    
    let mut result = vec![vec![0; b[0].len()]; a.len()];
    
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    Ok(result)
}

fn print_matrix(matrix: &[Vec<i32>]) {
    for row in matrix {
        for &element in row {
            print!("{:4}", element);
        }
        println!();
    }
}

