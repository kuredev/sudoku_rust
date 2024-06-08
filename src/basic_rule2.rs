use core::num;

use array_concat::*;

#[derive(Debug, Clone)]
struct Grid([[Cell; 9]; 9]);

impl Grid {
    fn is_complete(&self) -> bool {
        for y in self.0.iter() {
            for x in y.iter() {
                if x.number == 0 {
                    return false;
                }
            }
        }
        true
    }

    // ルールを1つ取り出して適用(propagate)する
    // true -> 適用した
    // false -> 何もなかった
    fn apply_some_rule(&mut self) -> bool {
        if let Some(mut fixed_cell) = self.find_only_number_in_cell() {
            // println!("find_only_number_in_cell: #{:?}, #{:?}, #{:?}", fixed_cell.cell.coodinate.x, fixed_cell.cell.coodinate.y, fixed_cell.number);

            fixed_cell.cell.number = fixed_cell.number.clone();
            fixed_cell.cell.possible_numbers = vec![];

            self.propagate(fixed_cell.clone().cell);
            return true;
        }

        if let Some(mut fixed_cell) = self.find_only_cell_in_block() {
           // println!("find_only_cell_in_block: #{:?}, #{:?}, #{:?}", fixed_cell.cell.coodinate.x, fixed_cell.cell.coodinate.y, fixed_cell.number);

            fixed_cell.cell.number = fixed_cell.number.clone();
            fixed_cell.cell.possible_numbers = vec![];

            self.propagate(fixed_cell.clone().cell);
            return true;
        }

        //println!("===false");
        false
    }

    // 27個のブロックを返す
    fn get_blocks(&self) -> [Block; 27] {
        let mut blocks: [Block; 27] = Default::default();
        blocks = concat_arrays!(
            self.get_square_blocks(),
            self.get_column_blocks(),
            self.get_row_blocks()
        );

        blocks
    }

    // 四角ブロックを返す
    // 1,2,3
    // 4,5,6
    // 7,8,9
    // の順番
    // 要リファクタリング
    fn get_square_blocks(&self) -> [Block; 9] {
        let mut blocks: [Block; 9] = Default::default();
        let cells1: [Cell; 9] = [
            self.0[0][0].clone(),
            self.0[0][1].clone(),
            self.0[0][2].clone(),
            self.0[1][0].clone(),
            self.0[1][1].clone(),
            self.0[1][2].clone(),
            self.0[2][0].clone(),
            self.0[2][1].clone(),
            self.0[2][2].clone(),
        ];
        let cells2: [Cell; 9] = [
            self.0[0][3].clone(),
            self.0[0][4].clone(),
            self.0[0][5].clone(),
            self.0[1][3].clone(),
            self.0[1][4].clone(),
            self.0[1][5].clone(),
            self.0[2][3].clone(),
            self.0[2][4].clone(),
            self.0[2][5].clone(),
        ];
        let cells3: [Cell; 9] = [
            self.0[0][6].clone(),
            self.0[0][7].clone(),
            self.0[0][8].clone(),
            self.0[1][6].clone(),
            self.0[1][7].clone(),
            self.0[1][8].clone(),
            self.0[2][6].clone(),
            self.0[2][7].clone(),
            self.0[2][8].clone(),
        ];
        let cells4: [Cell; 9] = [
            self.0[3][0].clone(),
            self.0[3][1].clone(),
            self.0[3][2].clone(),
            self.0[4][0].clone(),
            self.0[4][1].clone(),
            self.0[4][2].clone(),
            self.0[5][0].clone(),
            self.0[5][1].clone(),
            self.0[5][2].clone(),
        ];
        let cells5: [Cell; 9] = [
            self.0[3][3].clone(),
            self.0[3][4].clone(),
            self.0[3][5].clone(),
            self.0[4][3].clone(),
            self.0[4][4].clone(),
            self.0[4][5].clone(),
            self.0[5][3].clone(),
            self.0[5][4].clone(),
            self.0[5][5].clone(),
        ];
        let cells6: [Cell; 9] = [
            self.0[3][6].clone(),
            self.0[3][7].clone(),
            self.0[3][8].clone(),
            self.0[4][6].clone(),
            self.0[4][7].clone(),
            self.0[4][8].clone(),
            self.0[5][6].clone(),
            self.0[5][7].clone(),
            self.0[5][8].clone(),
        ];
        let cells7: [Cell; 9] = [
            self.0[6][0].clone(),
            self.0[6][1].clone(),
            self.0[6][2].clone(),
            self.0[7][0].clone(),
            self.0[7][1].clone(),
            self.0[7][2].clone(),
            self.0[8][0].clone(),
            self.0[8][1].clone(),
            self.0[8][2].clone(),
        ];
        let cells8: [Cell; 9] = [
            self.0[6][3].clone(),
            self.0[6][4].clone(),
            self.0[6][5].clone(),
            self.0[7][3].clone(),
            self.0[7][4].clone(),
            self.0[7][5].clone(),
            self.0[8][3].clone(),
            self.0[8][4].clone(),
            self.0[8][5].clone(),
        ];
        let cells9: [Cell; 9] = [
            self.0[6][6].clone(),
            self.0[6][7].clone(),
            self.0[6][8].clone(),
            self.0[7][6].clone(),
            self.0[7][7].clone(),
            self.0[7][8].clone(),
            self.0[8][6].clone(),
            self.0[8][7].clone(),
            self.0[8][8].clone(),
        ];

        blocks[0] = Block(cells1);
        blocks[1] = Block(cells2);
        blocks[2] = Block(cells3);
        blocks[3] = Block(cells4);
        blocks[4] = Block(cells5);
        blocks[5] = Block(cells6);
        blocks[6] = Block(cells7);
        blocks[7] = Block(cells8);
        blocks[8] = Block(cells9);

        blocks
    }

    // 縦9Cell
    fn get_column_blocks(&self) -> [Block; 9] {
        let mut blocks: [Block; 9] = Default::default();
        let mut row_cellses: [[Cell; 9]; 9] = Default::default();

        // 縦横を転置する
        for (y_index, row_cells) in self.0.iter().enumerate() {
            for (x_index, cell) in row_cells.iter().enumerate() {
                row_cellses[x_index][y_index] = cell.clone();
            }
        }
        // println!("{:?}", row_cellses[0]);

        for (index, cells) in row_cellses.iter().enumerate() {
            blocks[index] = Block(cells.clone());
        }
        blocks
    }

    // 横一列のCell
    // あってるぽい！
    fn get_row_blocks(&self) -> [Block; 9] {
        let mut blocks: [Block; 9] = Default::default();

        for (index, cells) in self.0.iter().enumerate() {
            blocks[index] = Block(cells.clone());
        }
        blocks
    }

    // あるセルではその数字しか入る可能性が無い
    // Ruby の中の find_single_number_rule_for_cell
    // セルと、Numberの組み合わせの回答がいるか
    fn find_only_number_in_cell(&self) -> Option<FixedCell> {
        for y in self.0.iter() {
            for x in y.iter() {
                if x.possible_numbers.len() == 1 {
                    // println!("find! find_only_number_in_cell: x: #{:?}, y: #{:?}, num: #{:?}", x.coodinate.x, x.coodinate.y, x.possible_numbers);
                    return Some(FixedCell {
                        cell: x.clone(),
                        number: x.possible_numbers[0],
                    });
                }
            }
        }
        None
    }

    // あるブロックでは、その数字が入る可能性があるのは、そのセルだけ
    fn find_only_cell_in_block(&self) -> Option<FixedCell> {
        let blocks = self.get_blocks();
        for block in blocks.iter() {
            let not_assigned_numbers = block.get_not_assigned_numbers();
            for not_assigned_number in not_assigned_numbers {
                let cells = block.get_cells_can_assign_number(not_assigned_number);
                if cells.len() == 1 {
                    return Some(FixedCell{
                        cell: cells[0].clone(),
                        number: not_assigned_number
                    });
                }
            }
        }
        None
    }

    // 指定のセルの所属する行の０以外のセルを抜き出す
    fn get_row_empty_cells(&self, cell: Cell) -> Vec<Cell> {
        let mut row: Vec<Cell> = self.0[(cell.coodinate.y.0 - 1) as usize].to_vec();
        row.retain(|cell| cell.number == 0);
        row
    }

    fn get_column_empty_cells(&self, cell: Cell) -> Vec<Cell> {
        let mut column: Vec<Cell> = Vec::new();

        let row_arrs = self.clone().0;
        for row in row_arrs.iter() {
            column.push(row.clone()[(cell.coodinate.x.0 - 1) as usize].clone());
        }
        column.retain(|cell| cell.number == 0);
        column
    }

    fn get_squre_empty_cells(&self, cell: Cell) -> Vec<Cell> {
        let mut result: Vec<Cell> = Vec::new();

        let block_x_num = Self::zahyo_to_block(cell.coodinate.x.0);
        let block_y_num = Self::zahyo_to_block(cell.coodinate.y.0);

        let block_x_num_indexes = Self::block_indexes(block_x_num);
        let block_y_num_indexes = Self::block_indexes(block_y_num);

        for y in block_y_num_indexes.iter() {
            for x in block_x_num_indexes.iter() {
                //let z = &mut self.0[(y - 1) as usize];
                let z = self.0[(y - 1) as usize][(x - 1) as usize].clone(); // as Cell;
                                                                            // result.push(z);
                result.push(z);
            }
        }

        result.retain(|cell| cell.number == 0);
        result
    }

    fn zahyo_to_block(n: i32) -> i32 {
        (n + 2) / 3
    }

    fn block_indexes(block_num: i32) -> Vec<i32> {
        vec![(block_num * 3) - 2, (block_num * 3) - 1, (block_num * 3)]
    }

    fn get_connected_empty_cells(&self, cell: Cell) -> Vec<Cell> {
        let mut results: Vec<Cell> = Vec::new();

        let row_empty_cells = self.get_row_empty_cells(cell.clone());
        let column_empty_cells = self.get_column_empty_cells(cell.clone());
        let squre_empty_cells = self.get_squre_empty_cells(cell.clone());

        results.extend(row_empty_cells);
        results.extend(column_empty_cells);
        results.extend(squre_empty_cells);
        results
    }

    // 引数の座標のセルの情報を他のセルに広報します
    fn propagate(&mut self, cell: Cell) {
        let fixed_my_cell = self.get_my_cell(cell.clone());
        fixed_my_cell.number = cell.number;
        fixed_my_cell.possible_numbers = vec![];

        let empty_cells = self.get_connected_empty_cells(cell.clone());

        for empty_cell in empty_cells.iter() {
            let my_cell = self.get_my_cell(empty_cell.clone());
            my_cell.cannot_assign(cell.number)
        }
    }

    fn initial_propagate(&mut self) {
        for y in self.clone().0.iter() {
            for cell in y.iter() {
                if cell.number != 0 {
                    self.propagate(cell.clone());
                }
            }
        }
    }

    fn get_my_cell(&mut self, cell: Cell) -> &mut Cell {
        let x = cell.coodinate.x;
        let y = cell.coodinate.y;

        &mut self.0[(y.value() - 1) as usize][(x.value() - 1) as usize]
    }

    fn print(&self) {
        for y in self.0.iter() {
            for cell in y.iter() {
                if cell.number == 0 {
                    print!(".");
                }else{
                    print!("{:?}", cell.number);
                }
            }
            print!(" ");
        }
        println!("");
    }
}

// Cellがその番号に決まった、ということを表現する構造体
#[derive(Debug, Clone, Default)]
struct FixedCell {
    cell: Cell,
    number: i32,
}

// 縦または横または 3*3 のブロックを表現する構造体
#[derive(Debug, Clone, Default)]
struct Block([Cell; 9]);

impl Block {
    // そのブロックでまだ使われていない数字を返す
    fn get_not_assigned_numbers(&self) -> Vec<i32> {
        let mut possible_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for cell in self.0.iter() {
            let cell_number = cell.number;
            possible_numbers.retain(|&num| num != cell_number);
        }
        possible_numbers
    }

    fn print_numbers(&self) {
        let cells = self.0.clone();
        for cell in cells {
            print!("{:?} ", cell.number);
        }
    }

    fn print_cells(&self) {
        for cell in self.0.clone() {
            println!("{:?}", cell);
        }
    }

    // ブロックの中で number が入る可能性のあるセルがあれば返す
    fn get_cells_can_assign_number(&self, number: i32) -> Vec<Cell>{
        let mut cells: Vec<Cell> = Default::default();
        for cell in self.0.clone() {
            let possible_numbers = cell.clone().possible_numbers;
            if possible_numbers.contains(&number) {
                cells.push(cell.clone());
            }
        }

        cells
    }
}

#[derive(Debug, Clone, Default)]
struct Cell {
    coodinate: Coordinate, // これないと、Blockに入れて渡したあとの返り値とかで、ハンドルしにくいか？
    number: i32,           // fix してない場合は 0
    possible_numbers: Vec<i32>,
}

impl Cell {
    fn cannot_assign(&mut self, number: i32) {
        self.possible_numbers.retain(|&x| x != number);
    }
}

// x, y の範囲を 1~ 9 に規定したい
#[derive(Debug, Clone, Default, Copy)]
struct Coordinate {
    x: CoordinateInt,
    y: CoordinateInt,
}

#[derive(Debug, Clone, Default, Copy)]
struct CoordinateInt(i32);

impl CoordinateInt {
    fn new(value: i32) -> Result<Self, String> {
        if value >= 1 && value <= 9 {
            Ok(CoordinateInt(value))
        } else {
            Err(format!(
                "Value {} is out of range. Must be between 1 and 9.",
                value
            ))
        }
    }

    fn value(&self) -> i32 {
        self.0
    }
}

fn build_cells_by_number(numbers: [[i32; 9]; 9]) -> [[Cell; 9]; 9] {
    let mut result: [[Cell; 9]; 9] = Default::default();
    for (y, row) in numbers.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            let mut possible_numbers: Vec<i32>;
            if *num == 0 {
                possible_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
                possible_numbers.retain(|&x| x != *num);
            } else {
                possible_numbers = vec![];
            }

            result[y][x] = Cell {
                // coodinate: Coordinate{x: x as i32, y: y as i32},
                coodinate: Coordinate {
                    x: CoordinateInt::new((x + 1) as i32).unwrap(),
                    y: CoordinateInt::new((y + 1) as i32).unwrap(),
                },
                number: *num,
                possible_numbers: possible_numbers,
            };
        }
    }

    result
}

fn print_cells(cells: Vec<Cell>) {
    for cell in cells.iter() {
        //println!("{:?}", cell.coodinate);
        println!(
            "{:?}, {:?}: {:?}",
            cell.coodinate.x, cell.coodinate.y, cell.number
        );
    }
}

fn main() {
    let numbers = [
        [0, 8, 0, 0, 0, 0, 0, 1, 0],
        [1, 0, 0, 2, 0, 0, 9, 0, 0],
        [0, 0, 7, 0, 0, 4, 0, 0, 3],
        [3, 0, 0, 0, 1, 0, 0, 9, 0],
        [0, 0, 0, 7, 0, 2, 0, 0, 0],
        [0, 6, 0, 0, 8, 0, 0, 0, 4],
        [9, 0, 0, 4, 0, 0, 1, 0, 0],
        [0, 0, 4, 0, 0, 3, 0, 0, 5],
        [0, 2, 0, 0, 0, 0, 0, 8, 0],
    ];

    let numbers2 = [
        [4, 8, 0, 0, 0, 0, 0, 1, 0],
        [1, 0, 0, 2, 0, 0, 9, 4, 0],
        [0, 0, 7, 1, 0, 4, 0, 0, 3],
        [3, 4, 0, 0, 1, 0, 0, 9, 0],
        [0, 0, 0, 7, 4, 2, 0, 0, 1],
        [0, 6, 1, 0, 8, 0, 0, 0, 4],
        [9, 0, 0, 4, 0, 0, 1, 0, 0],
        [0, 1, 4, 0, 0, 3, 0, 0, 5],
        [0, 2, 0, 0, 0, 1, 4, 8, 9],
    ];

    // 参照が必要なところをマークする
    let cells = build_cells_by_number(numbers);
    // 参照を取ってこないとだめ。
    // https://chatgpt.com/c/8b340753-a08b-4bfc-bbae-18f183c64e82
    // let n = cells[0][0]; // 構造体の配列をインデックス参照はできない
    let mut grid = Grid(cells.clone());
    let cell1s = grid.get_row_empty_cells(cells[0][0].clone()); // Copy の Cell を渡す
                                                                // print_cells(cell1s);

    let cell2s = grid.get_column_empty_cells(cells[0][0].clone());
    //print_cells(cell2s);

    let cell3s = grid.get_squre_empty_cells(cells[0][0].clone());

    grid.initial_propagate();

    //let blocks = grid.get_square_blocks();
    // println!("{:?}", blocks);
    /*for block in blocks {
        block.print_numbers();
        println!("");
    }*/
   // println!("{:?}", grid.find_only_cell_in_block());

    solve(grid); // ここでどこかにいっちゃったので、表示するには参照を渡さないとだめ
}

// 🌟ループさすところから。できたかも。
fn solve(mut grid: Grid) -> bool {
    grid.print();
    if grid.is_complete() {
        true
    } else {
        // 何か一つルールを取り出して適用
        // 適用できるルールがもう無かったら false を返す
        let appliable:bool = grid.apply_some_rule();
        if appliable {
            if solve(grid){
                return true;
            }
        } else {
            println!("失敗！");
            std::process::exit(0);
        }
        false
    }
}
