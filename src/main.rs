
#[derive(Debug, Clone, Copy)]
struct Grid([[Cell; 9]; 9]);
type Cell = i32;

#[derive(Debug, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Grid {
    fn pick_empty_coordinate(&self) -> Result<Coordinate, &'static str> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let value = *cell;
                if value == 0 {
                    return Ok(Coordinate{
                        x: (x + 1) as i32,
                        y: (y + 1) as i32
                        });
                }
            }
        }

        Err("空のセルがありません。")
    }

    fn get_possible_numbers(&self, coordinate: Coordinate) -> Vec<i32> {
        let mut possible_numbers = vec![1,2,3,4,5,6,7,8,9];

        let row: Vec<i32> = self.get_row(coordinate.y as usize);
        let column: Vec<i32> = self.get_column(coordinate.x as usize);
        let block: Vec<i32> = self.get_block(coordinate);

        possible_numbers.retain(|x| !row.contains(x));
        possible_numbers.retain(|x| !column.contains(x));
        possible_numbers.retain(|x| !block.contains(x));

        possible_numbers
    }

    // y 行の0以外の数値を取得する
    fn get_row(&self, y: usize) -> Vec<i32> {
        let mut row = self.0[y - 1].to_vec();
        row.retain(|x| *x != 0);
        row
    }

    // x 列の0以外の数値を取得する
    fn get_column(&self, x: usize) -> Vec<i32> {
        let mut column: Vec<i32> = Vec::new();

        let row_arrs = self.0;
        for row in row_arrs.iter() {
            column.push(row[x - 1]);
        }
        column.retain(|x| *x != 0);
        column
    }

    // 引数の座標のいるブロック（3*3）で、0以外の数値を取得する
    fn get_block(&self, coordinate: Coordinate) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let block_x_num = Self::zahyo_to_block(coordinate.x);
        let block_y_num = Self::zahyo_to_block(coordinate.y);

        let block_x_num_indexes = Self::block_indexes(block_x_num);
        let block_y_num_indexes = Self::block_indexes(block_y_num);

        for y in block_y_num_indexes.iter() {
            for x in block_x_num_indexes.iter() {
              let z = self.0[(y - 1) as usize][(x - 1) as usize];
              result.push(z);
            }
        }

        result
    }

    // ブロック（3*3のまとまり）の番号を元に、その番号が所属する座標の3つを取得する
    // 例えば、 4 だったら、4,5,6
    fn block_indexes(block_num: i32) -> Vec<i32> {
        vec![(block_num * 3) - 2, (block_num * 3) - 1, (block_num * 3)]
    }

    fn is_complete(&self) -> bool {
        for y in self.0.iter() {
            for x in y.iter() {
                if *x == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn print(&self) {
        for y in self.0.iter() {
            println!("{:?}", y);
        }
    }

    // その座標のいるブロックの番号を取得する
    // 例えば、 4 だったら2。9だったら3
    fn zahyo_to_block(n: i32) -> i32 {
        (n + 2) / 3
    }
}

fn main(){
    let grid = Grid([
        [0,8,0,0,0,0,0,1,0],
        [1,0,0,2,0,0,9,0,0],
        [0,0,7,0,0,4,0,0,3],
        [3,0,0,0,1,0,0,9,0],
        [0,0,0,7,0,2,0,0,0],
        [0,6,0,0,8,0,0,0,4],
        [9,0,0,4,0,0,1,0,0],
        [0,0,4,0,0,3,0,0,5],
        [0,2,0,0,0,0,0,8,0]
    ]);

    println!("Problem");
    grid.print();

    solve(grid);
}

fn solve(mut grid: Grid) -> bool {
    if grid.is_complete() {
        println!("complete!");
        grid.print();

        true
    } else {
        match grid.pick_empty_coordinate() {
            Ok(empty_coordinate) => {
                let possible_numbers = grid.get_possible_numbers(empty_coordinate.clone());

                for number in possible_numbers.iter() {
                    grid.0[empty_coordinate.y as usize - 1][empty_coordinate.x as usize - 1] = *number;
                    if solve(grid){
                        return true;
                    }
                }
                false
            },
            Err(error) => panic!("{}", error),
        }
    }
}
