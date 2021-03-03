use std::{thread, time, fmt, convert::TryInto};
use rand::Rng;

const map_width : usize = 80;
const map_height: usize = 40;
const map_len   : usize = map_width*map_height;

/*
Display Tech
*/
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn display_map(map:Maze){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for x in 0..map.map.len(){
        let disp:char = match map.map[x].sides {
            //up    down   left   right
            [false, false, false, false] => ' ',
            [false, false, false, true ] => '╶',
            [false, false, true , false] => '╴',
            [false, false, true , true ] => '─',
            [false, true , false, false] => '╷',
            [false, true , false, true ] => '┌',
            [false, true , true , false] => '┐',
            [false, true , true , true ] => '┬',
            [true , false, false, false] => '╵',
            [true , false, false, true ] => '└',
            [true , false, true , false] => '┘',
            [true , false, true , true ] => '┴',
            [true , true , false, false] => '│',
            [true , true , false, true ] => '├',
            [true , true , true , false] => '┤',
            [true , true , true , true ] => '┼',
            _ => unreachable!(),
        };
        stdout.set_color(ColorSpec::new().set_fg(Some(map.map[x].color)));
        write!(stdout, "{}",disp);
        if x % map_width == map_width-1 {writeln!(stdout);}
    };
}

fn temp() -> Result<(), Box<dyn std::error::Error>>{
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,0,30))))?;
    writeln!(stdout, "{}", "This is temporary!")?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,255))))?;
    Ok(())
}

fn translate (x:usize, y:usize) -> [usize; 2]{
    [x/2, y/2]
}

/*
Struct you stuff
*/
// #[derive(Debug)]
struct Tile {
    sides: [bool; 4],
    color: termcolor::Color,
}

impl Tile {
    fn new () -> Tile {
        Tile {
            sides: [true; 4],
            color: termcolor::Color::Rgb(82,34,227),
        }
    }

    fn set_side (&mut self, side:usize, val:bool){
        assert!(side < self.sides.len());
        self.sides[side] = val;
    }

    fn set_color (&mut self, color:termcolor::Color){
        self.color = color;
    }
}

impl Copy for Tile {}

impl Clone for Tile {
    fn clone(&self) -> Tile {
        Tile{sides:self.sides, color:self.color}
    }
}

struct Maze {
    map: [Tile; map_len],
}

fn get_index (x:usize , y:usize, width:usize) -> usize {
    return x + width*y;
}

fn get_coords (index:usize, width:usize) -> (usize, usize) {
    return (index % map_width, ((index as i32)/(map_width as i32)) as usize);
}

impl Maze {
    fn new () -> Maze {
        Maze {
            map: [Tile::new(); map_len],
        }
    }

    fn clear (&mut self){
        for x in 0..map_width {
            for y in 0..map_height {
                self.set_cell_data(x, y, [true; 4]);
                self.set_cell_color(x, y, Color::Black)
            }
        }
    }

    fn get_cell (&self, x:usize, y:usize) -> &Tile{
        assert!(x<=map_width && y<=map_height);
        &self.map[get_index(x, y, map_width)]
    }

    fn set_cell_data (&mut self, x:usize, y:usize, data:[bool; 4]){
        assert!(x<=map_width && y<=map_height);
        self.map[get_index(x, y, map_width)].sides = data;
    }

    fn set_cell_color (&mut self, x:usize, y:usize, color:Color){
        assert!(x<=map_width && y<=map_height);
        self.map[get_index(x, y, map_width)].color = color;
    }


    fn gen_maze (&mut self, iters:u64) -> u64{
        self.clear();
        //let mut rng = rand::thread_rng();


        let goal_x:usize = map_width-2;//(rng.gen::<u32>() % (map_width-2) as u32) as usize;
        let goal_y:usize = map_height-1;//(rng.gen::<u32>() % (map_height-2) as u32) as usize;
        &self.set_cell_color(goal_x,goal_y,Color::Rgb(201,255,38));


        let mut cur_pos = [2usize, 0usize];
        let map_cur_pos = translate(cur_pos[0], cur_pos[1]);
        &self.set_cell_color(map_cur_pos[0], map_cur_pos[1],Color::Rgb(3,252,48));

        let maze_width = map_width*2-1;
        let maze_height = map_height*2-1;
        //              up down left right
        let mut sub_maze = [[false; 4]; ((map_width-3)*(map_height-3))];
        let mut walls: Vec<bool>;
        let mut visited = vec!(cur_pos);

        sub_maze[get_index(cur_pos[0], cur_pos[1], maze_width)] = [false, false, false, false];

        let mut iter_count = 0;
        let mut completed = false;

        if cur_pos[0] > 1 {walls.append(true)}

        while !completed && walls.len() > 0 {
            println!("{:?}", walls.pop());
            completed = true
        }

        //println!("{:?}", curpos);
        //setup corners
        self.map[0].sides                        = [false, true , false, true ];
        self.map[map_width-1].sides              = [false, true , true , false];
        self.map[(map_height-1)*map_width].sides = [true , false, false, true ];
        self.map[(map_height*map_width)-1].sides = [true , false, true , false];

        self.map[0].color = Color::Blue;
        self.map[map_width-1].color = Color::Blue;
        self.map[(map_height-1)*map_width].color = Color::Blue;
        self.map[(map_height*map_width)-1].color = Color::Blue;
        //temp setup top
        for x in 1..map_width-1 {
            self.map[x].sides = [false, true , true , true ];
        }
        //temp setup left
        for x in 1..map_height-1 {
            self.map[x*map_width].sides = [true , true , false, true ];
        }
        //temp setup right
        for x in 1..map_height-1 {
            self.map[(x*map_width)+map_width-1].sides = [true, true , true , false];
        }
        //temp setup bottom
        for x in 1..map_width-1 {
            self.map[x+(map_width*(map_height-1))].sides = [true, false, true, true];
        }

        iter_count
    }

    fn randomize (&mut self) {
        let mut rng = rand::thread_rng();

        //let mut curpos = vec![0usize, 0usize];
        self.clear();

        for x in 0..map_width {
            for y in 0..map_height {
                let mut data = [false; 4];
                for d in data.iter_mut() {
                    *d = rand::random();
                }
                self.set_cell_data(x, y, data);
            }
        }
    }
}

impl Clone for Maze {
    fn clone(&self) -> Maze {
        Maze{map:self.map}
    }
}

// impl fmt::Display for Maze {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         Ok(for x in 0..self.map.len(){
//             for side in self.map[x].sides.into_iter() {
//                 write!(f, "{},", if side==&true {"t"} else {"f"})?;
//             }
//             if x % map_width == map_width-1 {writeln!(f);}
//         })
//     }
// }

#[test]
fn test_display() {
    let mut map = Maze::new();
    map.clear();
    map.randomize();
    display_map(map.clone());
    println!();
}

//print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

fn main() {
    let mut map = Maze::new();
    map.clear();
    map.gen_maze(1);
    display_map(map.clone());
}
