pub struct Range(pub usize, pub usize);

impl Range {
    pub fn parse(s: &str) -> Self {
        let mut vs = s.split("-")
            .map(|v| v.parse::<usize>().unwrap());
        
        Self(vs.next().unwrap(), vs.next().unwrap())
    }
    pub fn contains(&self, other: &Self) -> bool {
        (other.0 >= self.0) && (other.1 <= self.1)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        ((self.0 <= other.0) && (self.1 >= other.0)) ||
            ((self.0 <= other.1) && (self.1 >= other.0))
    }
}

pub struct RangePair(pub Range, pub Range);

impl RangePair {
    pub fn parse(line: &str) -> Self {
        let mut rs = line.split(",")
            .map(|r| Range::parse(r));
        Self(rs.next().unwrap(), rs.next().unwrap())
    }

    pub fn has_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

impl std::fmt::Display for RangePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}, {}-{}", self.0.0, self.0.1, self.1.0, self.1.1)
    }
}