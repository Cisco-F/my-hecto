use std::ops::{Add, Range};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
enum GraphemeWidth {
    Half, 
    Full
}

impl Add<usize> for GraphemeWidth {
    type Output = usize;

    fn add(self, other: usize) -> usize {
        match self {
            Self::Half => other + 1,
            Self::Full => other + 2,
        }
    }
}

struct GraphemeFragment {
    grapheme: String, 
    grapheme_width: GraphemeWidth, 
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<GraphemeFragment>,
}

impl Line {
    pub fn from(str: &str) -> Self {
        let fragments = str
            .graphemes(true)
            .map(|grapheme| {
                let u_width = grapheme.width();
                let grapheme_width = match u_width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };
                let replacement = match u_width {
                    0 => Some('.'),
                    _ => None,
                };

                GraphemeFragment {
                    grapheme: grapheme.to_string(),
                    grapheme_width,
                    replacement,
                }
            })
            .collect();
        Self { fragments }
    }
    pub fn get_graphems(&self, range: Range<usize>) -> String {
        let mut ret = String::new();
        if range.start >= range.end {
            return ret;
        }
        let mut pos = 0;
        for fragment in &self.fragments {
            let end = fragment.grapheme_width + pos;
            if pos >= range.end {
                break;
            }
            if end > range.start {
                if end > range.end || pos < range.start {
                    // Clip on the right or left
                    ret.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    ret.push(char);
                } else {
                    ret.push_str(&fragment.grapheme);
                }
            }
            pos = end;
        }
        ret
    }
    pub fn len(&self) -> usize {
        self.fragments.len()
    }
}