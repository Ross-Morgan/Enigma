pub(in crate) mod commercial {
    pub const IC: &'static str   = "DMTWSILRUYQNKFEJCAZBPGXHOV";
    pub const IIC: &'static str  = "HQZGPJTMOBLNCIFDYAWVEUSRKX";
    pub const IIIC: &'static str = "UQNTLSZFMREHDPXKIBVYGJCWOA";
    }

pub(in crate) mod rocket {
    pub const I: &'static str   = "JGDQOXUSCAMIFRVTPNEWKBLZYH";
    pub const II: &'static str  = "NTZPSFBOKMWRCJDIVLAEYUXHGQ";
    pub const III: &'static str = "JVIUBHTCDYAKEQZPOSGXNRMWFL";
    pub const UKW: &'static str = "QYHOGNECVPUZTFDJAXWMKISRBL";
    pub const ETW: &'static str = "QWERTZUIOASDFGHJKPYXCVBNML";
}

pub(in crate) mod swiss_k {
    pub const I_K: &'static str   = "PEZUOHXSCVFMTBGLRINQJWAYDK";
    pub const II_K: &'static str  = "ZOUESYDKFWPCIQXHMVBLGNJRAT";
    pub const III_K: &'static str = "EHRVXGAOBQUSIMZFLYNWKTPDJC";
    pub const UWK_K: &'static str = "IMETCGFRAYSQBZXWLHKDVUPOJN";
    pub const ETW_K: &'static str = "QWERTZUIOASDFGHJKPYXCVBNML";
}

// https://www.codesandciphers.org.uk/enigma/rotorspec.htm
pub(in crate) mod technical {
    pub const I: &'static str     = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
    pub const II: &'static str    = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
    pub const III: &'static str   = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
    pub const IV: &'static str    = "ESOVPZJAYQUIRHXLNFTGKDCMWB";
    pub const V: &'static str     = "VZBRGITYUPSDNHLXAWMJQOFECK";
    pub const VI: &'static str    = "JPGVOUMFYQBENHZRDKASXLICTW";
    pub const VII: &'static str   = "NZJHGRCXMYSWBOUFAIVLPEKQDT";
    pub const VIII: &'static str  = "FKQHTLXOCBJSPDZRAMEWNIUYGV";
    pub const BETA: &'static str  = "LEYJVCNIXWPBQMDRTAKZGFUHOS";
    pub const GAMMA: &'static str = "FSOKANUERHMBTIYCWLQPZXVGJD";
}


pub mod groups {
    use super::{
        commercial,
        rocket,
        swiss_k,
        technical
    };

    pub const COMMERCIAL: [&'static str; 3] = [
        commercial::IC,
        commercial::IIC,
        commercial::IIIC,
    ];

    pub const ROCKET: [&'static str; 5] = [
        rocket::I,
        rocket::II,
        rocket::III,
        rocket::UKW,
        rocket::ETW,
    ];

    pub const SWISS_K: [&'static str; 5] = [
        swiss_k::I_K,
        swiss_k::II_K,
        swiss_k::III_K,
        swiss_k::UWK_K,
        swiss_k::ETW_K,
    ];

    pub const TECHNICAL: [&'static str; 10] = [
        technical::I,
        technical::II,
        technical::III,
        technical::IV,
        technical::V,
        technical::VI,
        technical::VII,
        technical::VIII,
        technical::BETA,
        technical::GAMMA,
    ];
}
