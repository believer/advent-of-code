use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref BYR_RE_LOOSE: Regex = Regex::new(r"byr:(\d+)\s?").unwrap();
    static ref IYR_RE_LOOSE: Regex = Regex::new(r"iyr:(\d+)\s?").unwrap();
    static ref EYR_RE_LOOSE: Regex = Regex::new(r"eyr:(\d+)\s?").unwrap();
    static ref HGT_RE_LOOSE: Regex = Regex::new(r"hgt:(\d{2,3})\s?").unwrap();
    static ref HCL_RE_LOOSE: Regex = Regex::new(r"hcl:(#?\w+)\s?").unwrap();
    static ref ECL_RE_LOOSE: Regex = Regex::new(r"ecl:(#?\w+)\s?").unwrap();
    static ref PID_RE_LOOSE: Regex = Regex::new(r"pid:(#?\w+)\s?").unwrap();
    static ref BYR_RE: Regex = Regex::new(r"byr:(\d{4})\s?").unwrap();
    static ref IYR_RE: Regex = Regex::new(r"iyr:(\d{4})\s?").unwrap();
    static ref EYR_RE: Regex = Regex::new(r"eyr:(\d{4})\s?").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"hgt:(\d{2,3})(cm|in)\s?").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"hcl:(#[0-9a-f]{6})\s?").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\s?").unwrap();
    static ref PID_RE: Regex = Regex::new(r"pid:(\d+)").unwrap();
}

#[derive(Debug)]
enum Height {
    Centimeters,
    Inches,
}

impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Height::Centimeters),
            "in" => Ok(Height::Inches),
            _ => unreachable!("Invalid Height value"),
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    /// Birth year
    /// Valid 1920 - 2002
    byr: Option<u32>,
    /// Eye color
    /// (amb|blu|brn|gry|grn|hzl|oth)
    ecl: Option<String>,
    /// Expiration year
    /// Valid 2020 - 2030
    eyr: Option<u32>,
    /// Hair color
    /// Hex color
    hcl: Option<String>,
    /// Height
    hgt: (Option<u32>, Option<Height>),
    /// Issue year
    /// Valid 2010 - 2020
    iyr: Option<u32>,
    /// Passport ID
    pid: Option<String>,
}

fn get_string_value(cap: Option<regex::Captures>) -> Option<String> {
    match cap {
        Some(v) => Some(v.get(1)?.as_str().to_string()),
        None => None,
    }
}

fn get_int_value(cap: Option<regex::Captures>) -> Option<u32> {
    match cap {
        Some(v) => v.get(1)?.as_str().parse().ok(),
        None => None,
    }
}

impl Passport {
    pub fn is_valid_without_validation(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.0.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_valid_without_validation() {
            return false;
        }

        if self.byr.map_or(true, |v| !(1920..=2002).contains(&v)) {
            return false;
        }

        if self.iyr.map_or(true, |v| !(2010..=2020).contains(&v)) {
            return false;
        }

        if self.eyr.map_or(true, |v| !(2020..=2030).contains(&v)) {
            return false;
        }

        match self.hgt {
            (Some(h), Some(Height::Inches)) if (59..=76).contains(&h) => (),
            (Some(h), Some(Height::Centimeters)) if (150..=193).contains(&h) => (),
            _ => return false,
        }

        true
    }

    pub fn new(line: &str, loose: bool) -> Option<Passport> {
        let mut byr = BYR_RE.captures(line);
        let mut ecl = ECL_RE.captures(line);
        let mut eyr = EYR_RE.captures(line);
        let mut hcl = HCL_RE.captures(line);
        let mut hgt = HGT_RE.captures(line);
        let mut iyr = IYR_RE.captures(line);
        let mut pid = PID_RE.captures(line);

        if loose {
            byr = BYR_RE_LOOSE.captures(line);
            ecl = ECL_RE_LOOSE.captures(line);
            eyr = EYR_RE_LOOSE.captures(line);
            hcl = HCL_RE_LOOSE.captures(line);
            hgt = HGT_RE_LOOSE.captures(line);
            iyr = IYR_RE_LOOSE.captures(line);
            pid = PID_RE_LOOSE.captures(line);
        }

        let height: Option<Height> = match (&hgt, loose) {
            (_, true) => None,
            (Some(v), false) => v.get(2)?.as_str().parse().ok(),
            _ => None,
        };

        let pid: Option<String> = match (&pid, loose) {
            (_, true) => get_string_value(pid),
            (Some(v), false) => match v.get(1) {
                Some(pid) if pid.as_str().len() == 9 => Some(pid.as_str().to_string()),
                _ => None,
            },
            _ => None,
        };

        Some(Passport {
            byr: get_int_value(byr),
            ecl: get_string_value(ecl),
            eyr: get_int_value(eyr),
            hcl: get_string_value(hcl),
            hgt: (get_int_value(hgt), height),
            iyr: get_int_value(iyr),
            pid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_birth_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_birth_year() {
        let p = Passport::new(
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1919",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_issue_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_issue_year() {
        let p = Passport::new(
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2021 byr:1980",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_expiration_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_expiration_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2031 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_height_in() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_height_in() {
        let p = Passport::new(
            "pid:087499704 hgt:77in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_height_cm() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_height_cm() {
        let p = Passport::new(
            "pid:087499704 hgt:200cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn invalid_height_without_unit() {
        let p = Passport::new(
            "pid:087499704 hgt:200 ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_hair_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_hair_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#62zz22",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn invalid_hair_color_without_hash() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_eye_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_eye_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:brun iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_pid() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_pid() {
        let p = Passport::new(
            "pid:1 hgt:177cm ecl:brn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn invalid_pid_too_long() {
        let p = Passport::new(
            "pid:1234567891 hgt:177cm ecl:brn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_pid_last() {
        let p = Passport::new(
            "hgt:177cm ecl:brn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f pid:123456789",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }
}
