use rand::{StdRng, SeedableRng};
use veil_state::*;

const WIDTH: usize = 40;
const HEIGHT: usize = 40;

#[test]
fn veil_state() {
    let mut rng = StdRng::from_seed(&[0]);
    let mut veil = VeilState::new(WIDTH, HEIGHT, &mut rng);

    let to_char = |cell: VeilCell| {
        if cell.current {
            if cell.next {
                '#'
            } else {
                '+'
            }
        } else {
            if cell.next {
                '.'
            } else {
                ' '
            }
        }
    };

    for row in veil.rows() {
        for cell in row {
            print!("{}", to_char(cell));
        }
        print!("\n");
    }

    veil.step(&mut rng);

    println!("----");

    for row in veil.rows() {
        for cell in row {
            print!("{}", to_char(cell));
        }
        print!("\n");
    }

    veil.step(&mut rng);

    println!("----");

    for row in veil.rows() {
        for cell in row {
            print!("{}", to_char(cell));
        }
        print!("\n");
    }
}
