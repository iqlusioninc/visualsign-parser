//! Text safe to put in front of a signer.

const ELIDED: char = '?';

/// Mark what cannot render rather than dropping it: deleting would let two
/// different values render identically, and a signer comparing them has to be
/// able to tell them apart.
///
/// `visualsign-near`, `visualsign-solana` and `visualsign-ethereum` each carry
/// their own copy for their own rendering; this crate's copy can drift from
/// theirs since none of the four share a single implementation.
pub(crate) fn charset_safe(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c == ' ' || (c.is_ascii_graphic() && c != '\\') {
                c
            } else {
                ELIDED
            }
        })
        .collect()
}

/// Reversible display of signed freeform text. Escape UTF-8 bytes outside the
/// printable ASCII set, plus percent (the escape introducer) and backslash
/// (whose JSON encoding can violate the payload charset rules).
pub(crate) fn charset_escaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte == b' ' || (byte.is_ascii_graphic() && byte != b'%' && byte != b'\\') {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}
