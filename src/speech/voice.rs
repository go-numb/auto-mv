// 日本語（日本）	標準	ja-JP	ja-JP-Standard-A	女性
// 日本語（日本）	標準	ja-JP	ja-JP-Standard-B	女性
// 日本語（日本）	標準	ja-JP	ja-JP-Standard-C	男性
// 日本語（日本）	標準	ja-JP	ja-JP-Standard-D	男性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Neural2-B	女性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Neural2-C	男性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Neural2-D	男性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Wavenet-A	女性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Wavenet-B	女性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Wavenet-C	男性
// 日本語（日本）	プレミアム	ja-JP	ja-JP-Wavenet-D 男性

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Code {
    JaJP,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Code::JaJP => "ja-JP",
        };
        write!(f, "{}", name)
    }
}

impl Code {
    pub fn new() -> Self {
        Code::JaJP
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub enum Name {
    JaJPStandardA,
    JaJPStandardB,
    JaJPStandardC,
    JaJPStandardD,
    JaJPNeural2B,
    JaJPNeural2C,
    JaJPNeural2D,
    JaJPWavenetA,
    JaJPWavenetB,
    JaJPWavenetC,
    JaJPWavenetD,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Name::JaJPStandardA => "ja-JP-Standard-A",
            Name::JaJPStandardB => "ja-JP-Standard-B",
            Name::JaJPStandardC => "ja-JP-Standard-C",
            Name::JaJPStandardD => "ja-JP-Standard-D",

            Name::JaJPNeural2B => "ja-JP-Neural2-B",
            Name::JaJPNeural2C => "ja-JP-Neural2-C",
            Name::JaJPNeural2D => "ja-JP-Neural2-D",

            Name::JaJPWavenetA => "ja-JP-Wavenet-A",
            Name::JaJPWavenetB => "ja-JP-Wavenet-B",
            Name::JaJPWavenetC => "ja-JP-Wavenet-C",
            Name::JaJPWavenetD => "ja-JP-Wavenet-D",
        };
        write!(f, "{}", name)
    }
}

impl Name {
    fn new() -> Self {
        Name::JaJPStandardA
    }
    pub fn from(code: Code, n: u32) -> Option<Self> {
        match code {
            Code::JaJP => match n {
                0 => Some(Name::JaJPStandardA),
                1 => Some(Name::JaJPStandardB),
                2 => Some(Name::JaJPStandardC),
                3 => Some(Name::JaJPStandardD),
                4 => Some(Name::JaJPNeural2B),
                5 => Some(Name::JaJPNeural2C),
                6 => Some(Name::JaJPNeural2D),
                7 => Some(Name::JaJPWavenetA),
                8 => Some(Name::JaJPWavenetB),
                9 => Some(Name::JaJPWavenetC),
                10 => Some(Name::JaJPWavenetD),
                _ => Some(Name::new()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_display() {
        assert_eq!(Code::JaJP.to_string(), "ja-JP");
    }

    #[test]
    fn test_name_display() {
        assert_eq!(Name::JaJPStandardA.to_string(), "ja-JP-Standard-A");
        assert_eq!(Name::JaJPStandardB.to_string(), "ja-JP-Standard-B");
        assert_eq!(Name::JaJPStandardC.to_string(), "ja-JP-Standard-C");
        assert_eq!(Name::JaJPStandardD.to_string(), "ja-JP-Standard-D");

        assert_eq!(Name::JaJPNeural2B.to_string(), "ja-JP-Neural2-B");
        assert_eq!(Name::JaJPNeural2C.to_string(), "ja-JP-Neural2-C");
        assert_eq!(Name::JaJPNeural2D.to_string(), "ja-JP-Neural2-D");

        assert_eq!(Name::JaJPWavenetA.to_string(), "ja-JP-Wavenet-A");
        assert_eq!(Name::JaJPWavenetB.to_string(), "ja-JP-Wavenet-B");
        assert_eq!(Name::JaJPWavenetC.to_string(), "ja-JP-Wavenet-C");
        assert_eq!(Name::JaJPWavenetD.to_string(), "ja-JP-Wavenet-D");
    }

    #[test]
    fn test_name_from() {
        let count = 15;
        let code = Code::JaJP;

        for i in 0..count {
            let name = Name::from(code, i).unwrap();
            println!("name: {}", name);
        }
    }
}
