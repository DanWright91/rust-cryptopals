extern crate ascii;
extern crate data_encoding;
use self::ascii::{AsciiString, AsciiStr, AsciiChar};
use std::cmp::Ordering;
use self::data_encoding::HEXLOWER;
use std::ascii::AsciiExt;

pub fn repeating_key_xor<'a, 'b, T, U>(input_bytes: T, key_bytes: U) -> Vec<u8> 
where T: Iterator<Item=&'a u8>, U: Iterator<Item=&'b u8> + Clone
{
    input_bytes
        .zip(key_bytes.cycle())
        .map(|(input_byte, key_byte)| input_byte ^ key_byte)
        .collect::<Vec<_>>()
}

pub fn plaintext_scoring(plaintext: &Option<AsciiString>) -> isize {
    match *plaintext {
        Some(ref plaintext) => {
            plaintext
                .to_ascii_lowercase()
                .chars()
                .map(|x| 
                {
                    match *x
                    {
                        AsciiChar::e => 26,
                        AsciiChar::t => 25,
                        AsciiChar::a => 24,
                        AsciiChar::o => 23,
                        AsciiChar::i => 22,
                        AsciiChar::n => 21,
                        AsciiChar::s => 20,
                        AsciiChar::r => 19,
                        AsciiChar::h => 18,
                        AsciiChar::l => 17,
                        AsciiChar::d => 16,
                        AsciiChar::c => 15,
                        AsciiChar::u => 14,
                        AsciiChar::m => 13,
                        AsciiChar::f => 12,
                        AsciiChar::p => 11,
                        AsciiChar::g => 10,
                        AsciiChar::w => 9,
                        AsciiChar::y => 8,
                        AsciiChar::b => 7,
                        AsciiChar::v => 6,
                        AsciiChar::k => 5,
                        AsciiChar::x => 4,
                        AsciiChar::j => 3,
                        AsciiChar::q => 2,
                        AsciiChar::z => 1,
                        AsciiChar::Space => 1,
                        _ => -1
                    }
                })
            .sum()
        },
        None => 0
    }
}

pub fn ascii_printable_chars_bytes() -> Vec<Vec<u8>> {
    (32..126).map(|x| vec![x as u8]).collect::<Vec<Vec<_>>>()
}

#[derive(Debug)]
pub struct CandidateSolution
{
    pub score: isize,
    pub key: AsciiString,
    pub plaintext: Option<AsciiString>
}

impl Ord for CandidateSolution {
    fn cmp(&self, other: &CandidateSolution) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for CandidateSolution {
    fn partial_cmp(&self, other: &CandidateSolution) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CandidateSolution {
    fn eq(&self, other: &CandidateSolution) -> bool {
        self.score == other.score
    }
}

impl Eq for CandidateSolution {}