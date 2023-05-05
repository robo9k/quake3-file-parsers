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

impl From<RawSpan> for std::ops::Range<usize> {
    fn from(span: RawSpan) -> Self {
        span.start() as usize..span.end() as usize
    }
}

impl std::ops::Index<RawSpan> for str {
    type Output = str;

    #[inline]
    fn index(&self, span: RawSpan) -> &Self::Output {
        let range: std::ops::Range<usize> = span.into();
        &self[range]
    }
}
