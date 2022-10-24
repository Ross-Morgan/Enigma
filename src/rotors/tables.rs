pub mod tables{
    pub mod commerical {
        pub const IC: &'static str = "DMTWSILRUYQNKFEJCAZBPGXHOV";
        pub const IIC: &'static str = "HQZGPJTMOBLNCIFDYAWVEUSRKX";
        pub const IIIC: &'static str = "UQNTLSZFMREHDPXKIBVYGJCWOA";
    }

    pub mod rocket {
        pub const I: &'static str = "JGDQOXUSCAMIFRVTPNEWKBLZYH";
        pub const II: &'static str = "NTZPSFBOKMWRCJDIVLAEYUXHGQ";
        pub const III: &'static str = "JVIUBHTCDYAKEQZPOSGXNRMWFL";
        pub const UKW: &'static str = "QYHOGNECVPUZTFDJAXWMKISRBL";
        pub const ETW: &'static str = "QWERTZUIOASDFGHJKPYXCVBNML";
    }

    pub mod swiss_k {
        pub const I_K: &'static str = "PEZUOHXSCVFMTBGLRINQJWAYDK";
        pub const II_K: &'static str = "ZOUESYDKFWPCIQXHMVBLGNJRAT";
        pub const III_K: &'static str = "EHRVXGAOBQUSIMZFLYNWKTPDJC";
        pub const UWK_K: &'static str = "IMETCGFRAYSQBZXWLHKDVUPOJN";
        pub const ETW_K: &'static str = "QWERTZUIOASDFGHJKPYXCVBNML";
    }
}

pub mod groups {
    use super::tables::{
        commerical::*,
        rocket::*,
        swiss_k::*
    };

    pub const COMMERCIAL: [&'static str; 3] = [IC, IIC, IIIC];
    pub const ROCKET: [&'static str; 5] = [I, II, III, UKW, ETW];
    pub const SWISS_K: [&'static str; 5] = [I_K, II_K, III_K, UWK_K, ETW_K];

}
