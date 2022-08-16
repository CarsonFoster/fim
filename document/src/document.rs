use crate::buffer::Buffer;
use crate::scapegoat_tree::ScapegoatTree;

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
    newline_tree: ScapegoatTree<Node>,
    #[doc(hidden)]
    pieces: Vec<Piece>, // TODO: use other storage for pieces?
    #[doc(hidden)]
    buffers: Vec<Buffer>,
}

