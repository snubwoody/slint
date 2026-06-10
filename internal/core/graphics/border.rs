

// TODO: add colors
#[repr(C)]
pub struct Border<T> {
    /// The top width.
    pub top_width: T,
    /// The right width.
    pub right_width: T,
    /// The left width.
    pub left_width: T,
    /// The bottom width.
    pub bottom_width: T,
}
