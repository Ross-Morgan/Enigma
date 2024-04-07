//(A)(B)(C)(D)(E)(F)(G)(H)(I)(J)(K)(L)(M)(N)(O)(P)(Q)(R)(S)(T)(U)(V)(W)(X)(Y)(Z)

pub(in crate) mod commercial {
    pub const IC: &str   = "(AD)(BM)(CT)(DW)(ES)(FI)(GL)(HR)(IU)(JY)(KQ)(LN)(MK)(NF)(OE)(PJ)(QC)(RA)(SZ)(TB)(UP)(VG)(WX)(XO)(YH)(ZV)";
    pub const IIC: &str  = "(AH)(BQ)(CZ)(DG)(EP)(FJ)(GT)(HM)(IO)(JB)(KL)(LN)(MC)(NI)(OF)(PD)(QY)(RA)(SW)(TV)(UE)(VU)(WS)(XR)(YK)(ZX)";
    pub const IIIC: &str = "(AU)(BQ)(CN)(DT)(EL)(F)(G)(H)(I)(J)(K)(L)(M)(N)(O)(P)(Q)(R)(S)(T)(U)(V)(W)(XW)(YO)(ZA)";
    }

pub(in crate) mod rocket {
    pub const I: &str   = "JGDQOXUSCAMIFRVTPNEWKBLZYH";
    pub const II: &str  = "NTZPSFBOKMWRCJDIVLAEYUXHGQ";
    pub const III: &str = "JVIUBHTCDYAKEQZPOSGXNRMWFL";
    pub const UKW: &str = "QYHOGNECVPUZTFDJAXWMKISRBL";
    pub const ETW: &str = "QWERTZUIOASDFGHJKPYXCVBNML";
}

pub(in crate) mod swiss_k {
    pub const I_K: &str   = "PEZUOHXSCVFMTBGLRINQJWAYDK";
    pub const II_K: &str  = "ZOUESYDKFWPCIQXHMVBLGNJRAT";
    pub const III_K: &str = "EHRVXGAOBQUSIMZFLYNWKTPDJC";
    pub const UWK_K: &str = "IMETCGFRAYSQBZXWLHKDVUPOJN";
    pub const ETW_K: &str = "QWERTZUIOASDFGHJKPYXCVBNML";
}

// https://www.codesandciphers.org.uk/enigma/rotorspec.htm
pub(in crate) mod technical {
    pub const I: &str     = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
    pub const II: &str    = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
    pub const III: &str   = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
    pub const IV: &str    = "ESOVPZJAYQUIRHXLNFTGKDCMWB";
    pub const V: &str     = "VZBRGITYUPSDNHLXAWMJQOFECK";
    pub const VI: &str    = "JPGVOUMFYQBENHZRDKASXLICTW";
    pub const VII: &str   = "NZJHGRCXMYSWBOUFAIVLPEKQDT";
    pub const VIII: &str  = "FKQHTLXOCBJSPDZRAMEWNIUYGV";
    pub const BETA: &str  = "LEYJVCNIXWPBQMDRTAKZGFUHOS";
    pub const GAMMA: &str = "FSOKANUERHMBTIYCWLQPZXVGJD";
}


pub mod groups {
    use super::{
        commercial,
        rocket,
        swiss_k,
        technical
    };

    pub const COMMERCIAL: [(&str, &str); 3] = [
        ("IC", commercial::IC),
        ("IIC", commercial::IIC),
        ("IIIC", commercial::IIIC),
    ];

    pub const ROCKET: [(&str, &str); 5] = [
        ("I", rocket::I),
        ("II", rocket::II),
        ("III", rocket::III),
        ("UKW", rocket::UKW),
        ("ETW", rocket::ETW),
    ];

    pub const SWISS_K: [(&str, &str); 5] = [
        ("I_K", swiss_k::I_K),
        ("II_K", swiss_k::II_K),
        ("III_K", swiss_k::III_K),
        ("UWK_K", swiss_k::UWK_K),
        ("ETW_K", swiss_k::ETW_K),
    ];

    pub const TECHNICAL: [(&str, &str); 10] = [
        ("I", technical::I),
        ("II", technical::II),
        ("III", technical::III),
        ("IV", technical::IV),
        ("V", technical::V),
        ("VI", technical::VI),
        ("VII", technical::VII),
        ("VIII", technical::VIII),
        ("BETA", technical::BETA),
        ("GAMMA", technical::GAMMA),
    ];
}
