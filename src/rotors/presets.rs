//(A)(B)(C)(D)(E)(F)(G)(H)(I)(J)(K)(L)(M)(N)(O)(P)(Q)(R)(S)(T)(U)(V)(W)(X)(Y)(Z)

pub(in crate) mod commercial {
    pub const IC: &'static str   = "(AD)(BM)(CT)(DW)(ES)(FI)(GL)(HR)(IU)(JY)(KQ)(LN)(MK)(NF)(OE)(PJ)(QC)(RA)(SZ)(TB)(UP)(VG)(WX)(XO)(YH)(ZV)";
    pub const IIC: &'static str  = "(AH)(BQ)(CZ)(DG)(EP)(FJ)(GT)(HM)(IO)(JB)(KL)(LN)(MC)(NI)(OF)(PD)(QY)(RA)(SW)(TV)(UE)(VU)(WS)(XR)(YK)(ZX)";
    pub const IIIC: &'static str = "(AU)(BQ)(CN)(DT)(EL)(F)(G)(H)(I)(J)(K)(L)(M)(N)(O)(P)(Q)(R)(S)(T)(U)(V)(W)(XW)(YO)(ZA)";
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

    pub const COMMERCIAL: [(&'static str, &'static str); 3] = [
        ("IC", commercial::IC),
        ("IIC", commercial::IIC),
        ("IIIC", commercial::IIIC),
    ];

    pub const ROCKET: [(&'static str, &'static str); 5] = [
        ("I", rocket::I),
        ("II", rocket::II),
        ("III", rocket::III),
        ("UKW", rocket::UKW),
        ("ETW", rocket::ETW),
    ];

    pub const SWISS_K: [(&'static str, &'static str); 5] = [
        ("I_K", swiss_k::I_K),
        ("II_K", swiss_k::II_K),
        ("III_K", swiss_k::III_K),
        ("UWK_K", swiss_k::UWK_K),
        ("ETW_K", swiss_k::ETW_K),
    ];

    pub const TECHNICAL: [(&'static str, &'static str); 10] = [
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
