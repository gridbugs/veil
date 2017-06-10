use rand::{StdRng, SeedableRng};
use veil_state::*;
use content::VeilStepInfo;

const WIDTH: usize = 40;
const HEIGHT: usize = 40;

#[test]
fn veil_state() {
    let mut rng = StdRng::from_seed(&[0]);
    let info = VeilStepInfo {
        x: 0.02,
        y: 0.01,
        z: 0.02,
        min: 0.2,
        max: -0.2,
    };
    let mut veil = VeilState::new(WIDTH, HEIGHT, &mut rng, &info);

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

    veil.step(&mut rng, &info);

    println!("----");

    for row in veil.rows() {
        for cell in row {
            print!("{}", to_char(cell));
        }
        print!("\n");
    }

    veil.step(&mut rng, &info);

    println!("----");

    for row in veil.rows() {
        for cell in row {
            print!("{}", to_char(cell));
        }
        print!("\n");
    }
}
