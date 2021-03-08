pub struct BitStack(pub u32);

impl Iterator for BitStack {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let out = self.0 & !(self.0 - 1);
            self.0 ^= out;
            Some(out)
        }
    }
}

pub struct LocStack(pub u32);

impl Iterator for LocStack {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let out = self.0.trailing_zeros();
            self.0 ^= 1 << out;
            Some(out as usize)
        }
    }
}

pub struct LocStack64(pub u64);

impl Iterator for LocStack64 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let out = self.0.trailing_zeros();
            self.0 ^= 1 << out;
            Some(out as usize)
        }
    }
}

pub fn stringify_move(mov: (usize, usize)) -> String {
    let letters = "dcba".chars().collect::<Vec<_>>();
    let digits  = "12345678".chars().collect::<Vec<_>>();

    let mut out = String::new();

    if mov.0 > 31 {
        out.push('?');
    } else {
        out.push(letters[mov.0 % 4]);
        out.push(digits [mov.0 / 4]);
    }

    if mov.1 > 31 {
        out.push('?');
    } else {
        out.push(letters[mov.1 % 4]);
        out.push(digits [mov.1 / 4]);
    }

    out
}

pub fn print_move(mov: (usize, usize)) {
    println!("{}", stringify_move(mov));
}

#[allow(dead_code)]
pub fn print_u32(board: u32) {
    println!("{:#08x}", board);

    for y in (0..8).rev() {
        for x in (0..4).rev() {
            if board & (1 << (x + y * 4)) != 0 {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn print_u64(board: u64) {
    println!("{:#016x}", board);

    for y in (0..8).rev() {
        for x in (0..8).rev() {
            if board & (1 << (x + y * 8)) != 0 {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn num_to_mask(num: u32, mask: u32) -> u32 {
    let mut num_bit = 1;
    let mut out = 0;

    for mask_bit in BitStack(mask) {
        if num & num_bit != 0 {
            out |= mask_bit;
        }
        num_bit <<= 1;
    }

    out
}

fn within_board(x: isize, y: isize) -> bool {
    x >= 0 && x < 4 && y >= 0 && y < 8
}

fn gen_att(sq: usize, dist: usize, deltas: &Vec<(isize, isize)>, board: u32, field: bool)
    -> u32
{
    let startx = (sq % 4) as isize;
    let starty = (sq / 4) as isize;
    let mut x;
    let mut y;

    let mut i;
    let mut out = 0;

    for (dx, dy) in deltas {
        i = 0;
        x = startx + dx;
        y = starty + dy;

        while within_board(x, y) &&
              i < dist &&
              board & (1 << (x + y * 4)) == 0
        {
            out |= 1 << (x + y * 4);
            x += dx;
            y += dy;
            i += 1;
        }

        if within_board(x, y) && i < dist && (field || ((y >= 4) != (starty >= 4))) {
            out |= 1 << (x + y * 4);
        }
    }

    out
}

fn piece_properties() -> Vec<(usize, Vec<(isize, isize)>)> {
    vec![
        // (1, vec![(-1, -1), (1, -1), (-1,  1), (1,  1), (-1,  0), (0, -1), ( 1,  0), (0,  1)]),
        (1, vec![(-1, -1), (1, -1), (-1,  1), (1,  1)]),
        (2, vec![(-1,  0), (0, -1), ( 1,  0), (0,  1)]),
        (2, vec![(-1,  0), (0, -1), ( 1,  0), (0,  1)]),
        (9, vec![(-1,  0), (1,  0), (-1, -1), (1, -1), (-1, 1), (1, 1)]),
        (9, vec![( 0, -1), (0,  1)])
    ]
}

fn gen_masks() -> Vec<Vec<u32>> {
    let props = piece_properties();
    let mut out = Vec::new();

    for (dist, deltas) in props {
        let mut o = Vec::new();

        for sq in 0..32 {
            o.push(gen_att(sq, dist, &deltas, 0, false));
        }

        out.push(o);
    }

    out
}

fn gen_occ_att() -> Vec<Vec<Vec<(u32, u32)>>> {
    let masks = gen_masks();
    let props = piece_properties();

    let mut out = Vec::new();

    for piece in 1..5 {
        let (dist, deltas) = &props[piece];
        let mut pout = Vec::new();

        for sq in 0..32 {
            let mask = masks[piece][sq];
            let size = 1 << mask.count_ones();
            let mut sout = vec![(0, 0); size];

            for i in 0..size {
                let board = num_to_mask(i as u32, mask);
                sout[i] = (board, gen_att(sq, *dist, deltas, board, piece == 2));
            }
            pout.push(sout)
        }

        out.push(pout);
    }

    out
}

fn test_magic(table: &mut Vec<u32>,
              changed: &mut Vec<usize>,
              occ_att: &Vec<(u32, u32)>,
              bits: usize,
              magic: u32)
    -> bool
{
    let mut nchanged = 0;

    for (occ, att) in occ_att {
        let ind = (occ.overflowing_mul(magic).0 >> (32 - bits)) as usize;

        changed[nchanged] = ind;
        nchanged += 1;

        if table[ind] == u32::MAX {
            table[ind] = *att;
        } else if table[ind] != *att {

            for i in 0..nchanged {
                table[changed[i]] = u32::MAX;
            }

            return false;
        }
    }

    true
}

fn gen_magic_table(occ_att: &Vec<(u32, u32)>, bits: usize, magic: u32)
    -> Vec<u32>
{
    let size = 1 << bits;
    let mut table = vec![0; size];

    for (occ, att) in occ_att {
        let ind = (occ.overflowing_mul(magic).0 >> (32 - bits)) as usize;

        table[ind] = *att;
    }

    table
}

fn gen_magic(occ_att: &Vec<(u32, u32)>, bits: usize) -> u32 {
    let size = 1 << bits;
    let mut table = vec![u32::MAX; size];
    let mut changed = vec![0; size];

    loop {
        let magic =
            rand::random::<u32>() & rand::random::<u32>() &
            rand::random::<u32>() & rand::random::<u32>();

        if test_magic(&mut table, &mut changed, occ_att, bits, magic) {
            return magic;
        }
    }
}

fn gen_sliding_table() -> Vec<Vec<(u32, u32, usize, Vec<u32>)>> {
    let masks = gen_masks();
    let occ_atts = gen_occ_att();
    let mut out = Vec::new();

    for piece in 1..5 {
        let mut pout = Vec::new();

        for sq in 0..32 {
            let mask = masks[piece][sq];
            let occ_att = &occ_atts[piece - 1][sq];
            let bits = mask.count_ones() as usize + 1;

            let magic = gen_magic(occ_att, bits);
            let table = gen_magic_table(occ_att, bits, magic);
            pout.push((mask, magic, 32 - bits, table));
        }

        out.push(pout);
    }

    out
}

pub struct Tables {
    pub pawn: Vec<u32>,
    pub drone: Vec<(u32, u32, usize, Vec<u32>)>,
    pub field_drone: Vec<(u32, u32, usize, Vec<u32>)>,
    pub queen1: Vec<(u32, u32, usize, Vec<u32>)>,
    pub queen2: Vec<(u32, u32, usize, Vec<u32>)>,
}

impl Tables {
    pub fn new() -> Self {
        let mut masks = gen_masks().into_iter();
        let mut sliding = gen_sliding_table().into_iter();

        Self {
            pawn: masks.next().unwrap(),
            drone: sliding.next().unwrap(),
            field_drone: sliding.next().unwrap(),
            queen1: sliding.next().unwrap(),
            queen2: sliding.next().unwrap(),
        }
    }
}


#[test]
fn t_gen_tables() {
    let tables = Tables::new();

    assert_eq!(tables.queen2.len(), 32);
}
