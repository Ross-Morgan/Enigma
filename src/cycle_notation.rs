pub struct CycleNotation {
    pub mappings: Vec<Vec<char>>
}

impl CycleNotation {
    pub fn from_string(s: &'static str) -> Self {
        let mut mappings: Vec<Vec<char>> = vec![];

        let mut chars = s.chars();

        chars.next();
        chars.next_back();

        let s = chars.as_str().replace(" ", "");
        let split = s.split(")(");

        for group in split {
            mappings.push(group.chars().collect::<Vec<char>>());

        }

        Self {
            mappings: vec![]
        }
    }
}


impl std::fmt::Debug for CycleNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for CycleNotation {
    fn to_string(&self) -> String {
        let mut buf: String = String::new();

        for v in self.mappings.iter() {
            buf.push('(');
            buf.push_str(v.into_iter().collect::<String>().to_uppercase().as_str());
            buf.push(')');
            buf.push(' ');
        }

        buf.pop();

        buf
    }
}
