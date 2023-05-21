
pub(crate) fn is_namechar(ch: &char) -> bool {
    if is_namestartchar(ch) {
        true
    } else {
        matches!(ch,
            '.'
            | '-'
            | '0'..='9'
            | '\u{B7}'
            | '\u{0300}'..='\u{036F}'
            | '\u{203F}'..='\u{2040}'
        )
    }
}

pub(crate) fn is_ncnamechar(ch: &char) -> bool {
    if is_ncnamestartchar(ch) {
        true
    } else {
        matches!(ch,
            '.'
            | '-'
            | '0'..='9'
            | '\u{B7}'
            | '\u{0300}'..='\u{036F}'
            | '\u{203F}'..='\u{2040}'
        )
    }
}

pub(crate) fn is_namestartchar(ch: &char) -> bool {
    match ch {
        ':' => true,
        _ => is_ncnamestartchar(ch),
    }
}
pub(crate) fn is_ncnamestartchar(ch: &char) -> bool {
    matches!(ch,
        '\u{0041}'..='\u{005A}' // A-Z
        | '\u{005F}' // _
        | '\u{0061}'..='\u{007A}' // a-z
        | '\u{00C0}'..='\u{00D6}' //  [#xC0-#xD6]
        | '\u{00D8}'..='\u{00F6}' //  [#xD8-#xF6]
        | '\u{00F8}'..='\u{02FF}' //  [#xF8-#x2FF]
        | '\u{0370}'..='\u{037D}' //  [#x370-#x37D]
        | '\u{037F}'..='\u{1FFF}' //  [#x37F-#x1FFF]
        | '\u{200C}'..='\u{200D}' //  [#x200C-#x200D]
        | '\u{2070}'..='\u{218F}' //  [#x2070-#x218F]
        | '\u{2C00}'..='\u{2FEF}' //  [#x2C00-#x2FEF]
        | '\u{3001}'..='\u{D7FF}' //  [#x3001-#xD7FF]
        | '\u{F900}'..='\u{FDCF}' //  [#xF900-#xFDCF]
        | '\u{FDF0}'..='\u{FFFD}' //  [#xFDF0-#xFFFD]
        | '\u{10000}'..='\u{EFFFF}' //  [#x10000-#xEFFFF]
    )
}

pub fn is_char(ch: &char) -> bool {
    matches!(ch,
        '\u{0009}' // #x9
        | '\u{000A}' // #xA
        | '\u{000D}' // #xD
        | '\u{0020}'..='\u{D7FF}' //  [#x0020-#xD7FF]
        | '\u{E000}'..='\u{FFFD}' //  [#xE000-#xFFFD]
        | '\u{10000}'..='\u{10FFFF}' //  [#x10000-#10FFFF]
    )
}

pub(crate) fn is_pubid_charwithapos(ch: &char) -> bool {
    match ch {
        '\'' => true,
        _ => is_pubid_char(ch),
    }
}

pub(crate) fn is_pubid_char(ch: &char) -> bool {
    matches!(ch,
        '\u{0020}' // #x0020
        | '\u{000A}' // #xA
        | '\u{000D}' // #xD
        | 'a'..='z'
        | 'A'..='Z'
        | '0'..='9'
        | '-'
        | '('
        | ')'
        | '+'
        | ','
        | '.'
        | '/'
        | ':'
        | '='
        | '?'
        | ';'
        | '!'
        | '*'
        | '#'
        | '@'
        | '$'
        | '_'
        | '%'
    )
}
