use std::ops::{Add, Range};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone, Debug)]
pub enum GraphemeWidth {
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
    // 看不懂
    pub fn from(str: &str) -> Self {
        let fragments = str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, grapheme_width) = 
                Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let grapheme_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, grapheme_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                GraphemeFragment {
                    grapheme: grapheme.to_string(),
                    grapheme_width,
                    replacement,
                }
            })
            .collect();
        Self { fragments }
    }
    /// fill in GraphemeFragment's replacement field
    fn replacement_character(grapheme_str: &str) -> Option<char> {
        let width = grapheme_str.width();
        match grapheme_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && grapheme_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = grapheme_str.chars();
                if let Some(c) = chars.next() {
                    if c.is_control() && chars.next().is_none() {
                        return Some('▯')
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }
    /// get graphemes within given range
    /// mind that one grapheme may takes 2 bits
    pub fn get_graphems(&self, range: Range<usize>) -> String {
        let mut ret = String::new();
        if range.start >= range.end {
            return ret;
        }
        let mut pos = 0;
        for fragment in &self.fragments {
            if pos >= range.end {
                break;
            }
            let end = fragment.grapheme_width + pos;
            if end > range.start {
                if end > range.end || pos < range.start {
                    // Clip on the right or left
                    ret.push('·');
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
    /// total graphemes in a single line
    pub fn grapheme_len(&self) -> usize {
        self.fragments.len()
    }
    /// returns how many spaces the graphemes before given index take on the screen
    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.grapheme_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }
    pub fn insert_at(&mut self, grapheme_index: usize, c: char) {
        let new_fragment = Self::from(&c.to_string()).fragments;
        self.fragments.splice(grapheme_index..grapheme_index, new_fragment);
    }
}