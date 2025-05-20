pub struct Rays {
    n_rays: Vec<u64>,
    e_rays: Vec<u64>,
    nw_rays: Vec<u64>,
    ne_rays: Vec<u64>,
    w_rays: Vec<u64>,
    s_rays: Vec<u64>,
    sw_rays: Vec<u64>,
    se_rays: Vec<u64>,
}

macro_rules! make_rays {
    ($ray_fn:ident) => {{
        let mut rays = vec![];
        for row in 1..=8 {
            for col in 1..=8 {
                rays.push($ray_fn(row, col));
            }
        }

        rays
    }};

}

impl Rays {
    fn initialize() -> Self {
        let n_rays = make_rays!(n_ray);
        let e_rays = make_rays!(e_ray);
        let nw_rays = make_rays!(nw_ray);
        let ne_rays = make_rays!(ne_ray);
        let w_rays = make_rays!(w_ray);
        let s_rays = make_rays!(s_ray);
        let sw_rays = make_rays!(sw_ray);
        let se_rays = make_rays!(se_ray);

        Self {
            n_rays: n_rays,
            e_rays: e_rays,
            nw_rays: nw_rays,
            ne_rays: ne_rays,
            w_rays: w_rays,
            s_rays: s_rays,
            sw_rays: sw_rays,
            se_rays: se_rays,
        }

    }
}

macro_rules! define_ray {
    ($name:ident, $offset_fn:expr) => {
        fn $name(row: i8, col: i8) -> u64 {
            let mut bitboard = 0;

            for offset in 1..=8 {
                set_bit(&mut bitboard, $offset_fn(row, col, offset));
            }

            bitboard
        }
    };
}
define_ray!(n_ray, |row, col, offset| (row + offset, col));
define_ray!(e_ray, |row, col, offset| (row, col + offset));
define_ray!(nw_ray, |row, col, offset| (row + offset, col - offset));
define_ray!(ne_ray, |row, col, offset| (row + offset, col + offset));
define_ray!(w_ray, |row, col, offset| (row, col - offset));
define_ray!(s_ray, |row, col, offset| (row - offset, col));
define_ray!(sw_ray, |row, col, offset| (row - offset, col - offset));
define_ray!(se_ray, |row, col, offset| (row - offset, col + offset));



fn set_bit(bitboard: &mut u64, row_col: (i8, i8)) {
    let row = row_col.0;
    let col = row_col.1;
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return;
    }

    *bitboard = *bitboard | (1 << ((col - 1) + (row - 1) * 8))
}


fn bitboard_to_string(bitboard: u64, mark: Option<usize>) -> String {
    let mut board = String::new();

    for r in (0..8).rev() {
        for col in 0..8 {
            let idx = (r*8) + col;
            let c = if mark == Some(idx) {
                'x'
            } else if bitboard & (1u64 << idx) != 0 {
                '1'
            } else {
                '.'
            };
            board.push(c);
        }
        if r != 0 {
            board.push('\n');
        }
    }

    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_n_ray() {
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row - 1) * 8 + col - 1;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.n_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_se_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.se_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_sw_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.sw_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_nw_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.nw_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_ne_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.ne_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_e_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.e_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_w_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.w_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_s_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.s_rays[idx], Some(idx))
        );
    }
}
