use std::sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    Arc,
};

use super::analyze_source_file::analyze_source_file;
use super::pos::{BytePos, ByteToCharPosState, MultiByteChar, NonNarrowChar, Pos, Span};

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct SourceFile {
    pub src: Arc<String>,
    pub start_pos: BytePos,
    pub lines: Vec<BytePos>,
    pub multibyte_chars: Vec<MultiByteChar>,
    pub non_narrow_chars: Vec<NonNarrowChar>,
}

impl SourceFile {
    pub fn new(mut src: String, start_pos: BytePos) -> SourceFile {
        remove_bom(&mut src);

        Self::new_from(Arc::new(src), start_pos)
    }

    pub fn new_from(src: Arc<String>, start_pos: BytePos) -> SourceFile {
        let (lines, multibyte_chars, non_narrow_chars) = analyze_source_file(&src[..], start_pos);

        SourceFile {
            src,
            start_pos,
            lines,
            multibyte_chars,
            non_narrow_chars,
        }
    }
}

pub struct SourceMap {
    start_pos: AtomicUsize,
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap {
            start_pos: AtomicUsize::new(1),
        }
    }

    pub fn new_source_file(&self, mut src: String) -> Arc<SourceFile> {
        remove_bom(&mut src);

        self.new_source_file_from(Arc::new(src))
    }

    pub fn new_source_file_from(&self, src: Arc<String>) -> Arc<SourceFile> {
        let start_pos: usize = self.next_start_pos(src.len());

        let source_file = Arc::new(SourceFile::new_from(src, Pos::from_usize(start_pos)));

        source_file
    }

    fn next_start_pos(&self, len: usize) -> usize {
        self.start_pos.fetch_add(len + 1, SeqCst)
    }

    pub fn span_to_char_offset(&self, file: &SourceFile, span: Span) -> (u32, u32) {
        let start_offset = file.start_pos;

        let mut state = ByteToCharPosState::default();
        let start = span.lo.to_u32()
            - start_offset.to_u32()
            - self.calc_utf16_offset(file, span.lo, &mut state);
        let end = span.hi.to_u32()
            - start_offset.to_u32()
            - self.calc_utf16_offset(file, span.hi, &mut state);

        (start, end)
    }

    fn calc_utf16_offset(
        &self,
        file: &SourceFile,
        bpos: BytePos,
        state: &mut ByteToCharPosState,
    ) -> u32 {
        let mut total_extra_bytes = state.total_extra_bytes;
        let mut index = state.mbc_index;

        if bpos >= state.pos {
            let range = index..file.multibyte_chars.len();
            for i in range {
                let mbc = &file.multibyte_chars[i];
                if mbc.pos >= bpos {
                    break;
                }
                total_extra_bytes += mbc.byte_to_char_diff() as u32;
                index += 1;
            }
        } else {
            let range = 0..index;
            for i in range.rev() {
                let mbc = &file.multibyte_chars[i];
                if mbc.pos < bpos {
                    break;
                }
                total_extra_bytes -= mbc.byte_to_char_diff() as u32;
                index -= 1;
            }
        }

        state.pos = bpos;
        state.total_extra_bytes = total_extra_bytes;
        state.mbc_index = index;

        total_extra_bytes
    }
}

pub(super) fn remove_bom(src: &mut String) {
    if src.starts_with('\u{feff}') {
        src.drain(..3);
    }
}

pub struct Utf8ToUtf16 {
    source_map: Arc<SourceMap>,
    source_file: Arc<SourceFile>,
}

impl Utf8ToUtf16 {
    pub fn new(src: String) -> Self {
        let source_map = Arc::new(SourceMap::default());
        let source_file = source_map.new_source_file(src);
        Utf8ToUtf16 {
            source_map,
            source_file,
        }
    }

    pub fn utf8_to_utf16(&self, range: (u32, u32)) -> (u32, u32) {
        let utf8_range = range;
        let span = Span {
            lo: BytePos(utf8_range.0 + 1),
            hi: BytePos(utf8_range.1 + 1),
        };
        let utf16_range = self
            .source_map
            .span_to_char_offset(self.source_file.as_ref(), span);
        utf16_range
    }
}

#[test]
fn run() {
    let text = "中文测试cc是😅é是我是是";

    let ins = Utf8ToUtf16::new(text.into());

    assert_eq!(ins.utf8_to_utf16((12, 13)), (4, 5)); // c
    assert_eq!(ins.utf8_to_utf16((17, 21)), (7, 9)); // 😅
    assert_eq!(ins.utf8_to_utf16((21, 24)), (9, 11)); // é
}
