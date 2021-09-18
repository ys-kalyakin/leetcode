use std::cmp::min;

/// https://leetcode.com/problems/01-matrix/
#[allow(dead_code)]
pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = mat.len();

    if rows == 0 {
        return mat;
    }

    let cols = mat[0].len();
   
    let mut dist = vec![vec![i32::MAX - 100000]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] == 0 {
                dist[i][j] = 0;
            } else {
                if i > 0 {
                    dist[i][j] = min(dist[i][j], dist[i - 1][j] + 1);
                }
                if j > 0 {
                    dist[i][j] = min(dist[i][j], dist[i][j - 1] + 1);
                }
            }
        }
    }

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            if i < rows - 1 {
                dist[i][j] = min(dist[i][j], dist[i + 1][j] + 1);
            }
            if j < cols - 1 {
                dist[i][j] = min(dist[i][j], dist[i][j + 1] + 1);
            }
        }
    }

    dist
}
