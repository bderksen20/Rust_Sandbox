use std::io::stdout;
use std::io::Write;
use ncurses::*;

static WIN_H: i32 = 1;
static WIN_W: i32 = 1;

fn main() {
    
    initscr();
    raw();
    start_color();

    init_pair(1, COLOR_BLUE, COLOR_RED);

    //-- extended keyboard
    keypad(stdscr(), true);
    noecho();

    //-- invis cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    //-- screen bounds
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    //-- start 
    let mut start_y = (max_y - WIN_H) / 2;
    let mut start_x = (max_x - WIN_W) / 2;
    let mut win = create_win(start_y, start_x);
    
    //-- init circle
    let c1 = Circle{center: Point{x:start_x,y:start_y,z:0} , radius: 10};
    
    //-- loop until p pressed
    let mut ch = getch();
    while ch != KEY_F(1){
        match ch {
            KEY_LEFT => {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
                if c1.contains(Point{x: start_x, y: start_y, z: 0}) {
                    wbkgd(win, COLOR_PAIR(1));   
                }
            },
            KEY_RIGHT => {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
                if c1.contains(Point{x: start_x, y: start_y, z: 0}) {
                    wbkgd(win, COLOR_PAIR(1));   
                }
            },
            KEY_DOWN => {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
                if c1.contains(Point{x: start_x, y: start_y, z: 0}) {
                    wbkgd(win, COLOR_PAIR(1));   
                }
            },
            KEY_UP => {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
                if c1.contains(Point{x: start_x, y: start_y, z: 0}) {
                    wbkgd(win, COLOR_PAIR(1));   
                }
            },
            ENTER => {
               clear();
               wrefresh(win);
            },
            _ => { }
        }
        ch = getch();
    }
    
    endwin();

    /*

    //static ORIGIN: Point = Point{x: 0, y: 0, z:0};

    println!("Welcome to the sandbox....\n--------------------");
    
    let p1 = Point{x: 2, y: 1, z: 0};
    //p1.printer(); 

    let c1 = Circle{center: Point{x:0,y:0,z:0} , radius: 10};

    //c1.printer();
    
    let ct = c1.contains(p1);
    
    //for a in 0..100000{
    //    print!("\r{}", a);
    //    stdout().flush();
    //}

    loop {
        for y in -32..32{
            for x in -64..64{
                if c1.contains(Point{x: x, y: y, z: 0}) {
                    print!("x");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
        //stdout().flush();
        //print!("{esc}c", esc = 27 as char);
        break;
    }

    */
   
}

fn create_win(start_y: i32, start_x: i32) -> WINDOW {
    let win = newwin(WIN_H, WIN_W, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW){
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}
struct Point{
    x: i32,
    y: i32,
    z: i32
}

struct Circle{
    center: Point,
    radius: i32
}

trait Contains{
    fn contains(&self, p: Point) -> bool;
}

impl Contains for Circle{
    fn contains(&self, p: Point) -> bool{
        // Eq for circle: (x-h)^2 + (y-k)^2 = r^2
        let x =(p.x - self.center.x).pow(2) + (p.y - self.center.y).pow(2);
        let rs = self.radius.pow(2);
        if x == rs{
            //println!("Point is on circle!");
            return true;
        } else if x < rs {
            //println!("Point is in circle!");
            return true;
        } else {
            //println!("Point is outside of circle!");
            return false;
        }

    }
}

trait Printer {
    fn printer(&self);
}

impl Printer for Point{
    fn printer(&self){
        print!("({}, {}, {})", self.x, self.y, self.z);
    }
}

impl Printer for Circle{
    fn printer(&self){
        print!("\nradius = {}\ncenter = ", self.radius);
        self.center.printer();
    }
}

//struct c{
//    p: point,
//}


