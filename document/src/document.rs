use crate::buffer::Buffer;

// TODO: choose bounds for numerics later
struct NewlineCount(usize);
struct PieceIndex(usize);
struct BufferIndex(usize);
struct GraphemeIndex(u16);

struct Node {
    count: NewlineCount,
    left_count: NewlineCount,
    piece: PieceIndex,
}

struct Piece {
    buffer: BufferIndex,
    start: GraphemeIndex, // inclusive
    end: GraphemeIndex, // exclusive
}

pub struct Document {
    #[doc(hidden)]
    newline_tree: Vec<Node>,
    #[doc(hidden)]
    pieces: Vec<Piece>, // TODO: use other storage for pieces?
    #[doc(hidden)]
    buffers: Vec<Buffer>,
}

const fn left(parent: usize) -> usize {
    2*parent
}

const fn right(parent: usize) -> usize {
    2*parent + 1
}
