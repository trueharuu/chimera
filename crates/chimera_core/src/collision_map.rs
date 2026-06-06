use crate::{
    board::Board,
    data::{PIECE_CELLS, x_range},
    header::{COLS, ROWS},
    piece::Piece,
    rotation::Rotation,
};

/// Mapping from `(rotation, column)` to a bitmask of rows that collide with the piece in that rotation when placed at that column.
pub struct CollisionMap {
    pub data: [[u8; COLS]; Rotation::NB],
}

impl CollisionMap {
    /// Build the collision map for a given piece on a given board.
    pub const fn new(board: Board, piece: Piece) -> Self {
        let mut data = [[0; COLS]; Rotation::NB];
        // rows mask (low `ROWS` bits)
        let rows_mask: u8 = if ROWS >= 8 {
            u8::MAX
        } else {
            (1u8 << ROWS) - 1
        };

        // for each rotation and pivot column, compute a mask of pivot-`y`
        // positions that collide with the board or are out-of-bounds vertically
        // bit `r` of the stored u8 means: placing the piece pivot at row `r`
        // (and at the given column) would collide / be invalid
        let mut rot_idx = 0;
        while rot_idx < Rotation::NB {
            let rot = Rotation::from(rot_idx as u8);
            let cells = PIECE_CELLS[piece as usize][rot_idx];

            let (min_px, max_px) = x_range(piece, rot);

            let mut x = 0;
            while x < COLS {
                let i = &mut data[rot_idx][x];

                let px = x as i8;

                // if pivot x is out of horizontal bounds for this rotation,
                // mark all pivot rows invalid.
                if px < min_px || px > max_px {
                    *i = rows_mask;
                    continue;
                }

                // compute which pivot-y values are vertically valid (all piece
                // cells stay inside [0..ROWS)).
                let mut valid_y_mask: u8 = 0;
                let mut y = 0;
                while y < ROWS {
                    let mut ok = true;
                    let mut t = 0;
                    while t < cells.len() {
                        let (_dx, dy) = cells[t];
                        let cy = (y as i8) + dy;
                        if cy < 0 || cy >= ROWS as i8 {
                            ok = false;
                            break;
                        }

                        t += 1;
                    }

                    if ok {
                        valid_y_mask |= 1 << y;
                    }

                    y += 1;
                }

                // aggregate collisions from each mino by shifting the column
                // bitboards to align board bits with pivot-y coordinates.
                let mut coll: u8 = 0;

                let mut t = 0;
                while t < cells.len() {
                    let (dx, dy) = cells[t];
                    let cx = (px + dx) as usize;
                    let colbits = (board.col(cx) as u8) & rows_mask;
                    if dy >= 0 {
                        coll |= colbits >> (dy as usize);
                    } else {
                        coll |= (colbits << ((-dy) as usize)) & rows_mask;
                    }

                    t += 1;
                }

                // final mask: invalid if out-of-bounds (`!valid_y_mask`) or
                // if any mino collides with the board (`coll`).
                *i = (!valid_y_mask) | coll;

                x += 1;
            }
            rot_idx += 1;
        }

        Self { data }
    }

    #[inline]
    pub const fn get(&self, rot: Rotation, x: usize, y: usize) -> bool {
        self.data[rot as usize][x] & (1 << y) != 0
    }
}
