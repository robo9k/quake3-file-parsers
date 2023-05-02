#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawSpan {
    start: u32,
    end: u32,
}

impl RawSpan {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub const fn start(&self) -> u32 {
        self.start
    }

    pub const fn end(&self) -> u32 {
        self.end
    }
}
