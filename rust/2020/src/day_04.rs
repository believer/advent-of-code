use itertools::join;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// Day 4 - Passport Processing

lazy_static! {
    static ref BYR_RE_LOOSE: Regex = Regex::new(r"byr:(\d+)\s?").unwrap();
    static ref IYR_RE_LOOSE: Regex = Regex::new(r"iyr:(\d+)\s?").unwrap();
    static ref EYR_RE_LOOSE: Regex = Regex::new(r"eyr:(\d+)\s?").unwrap();
    static ref HGT_RE_LOOSE: Regex = Regex::new(r"hgt:(\d{2,3})\s?").unwrap();
    static ref HCL_RE_LOOSE: Regex = Regex::new(r"hcl:(#?\w+)\s?").unwrap();
    static ref ECL_RE_LOOSE: Regex = Regex::new(r"ecl:(#?\w+)\s?").unwrap();
    static ref PID_RE_LOOSE: Regex = Regex::new(r"pid:(#?\w+)\s?").unwrap();
    static ref CID_RE_LOOSE: Regex = Regex::new(r"cid:(\w+)\s?").unwrap();
    static ref BYR_RE: Regex = Regex::new(r"byr:(\d{4})\s?").unwrap();
    static ref IYR_RE: Regex = Regex::new(r"iyr:(\d{4})\s?").unwrap();
    static ref EYR_RE: Regex = Regex::new(r"eyr:(\d{4})\s?").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"hgt:(\d{2,3})(cm|in)\s?").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"hcl:(#[0-9a-f]{6})\s?").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\s?").unwrap();
    static ref PID_RE: Regex = Regex::new(r"pid:(\d+)").unwrap();
    static ref CID_RE: Regex = Regex::new(r"cid:(\w+)\s?").unwrap();
}

#[derive(Debug)]
enum Height {
    Centimeters,
    Inches,
}

impl std::str::FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Height::Centimeters),
            "in" => Ok(Height::Inches),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    /// Birth year
    /// Valid 1920 - 2002
    byr: Option<u32>,
    /// Country ID
    cid: Option<String>,
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
        let mut byr = BYR_RE.captures(&line);
        let mut cid = CID_RE.captures(&line);
        let mut ecl = ECL_RE.captures(&line);
        let mut eyr = EYR_RE.captures(&line);
        let mut hcl = HCL_RE.captures(&line);
        let mut hgt = HGT_RE.captures(&line);
        let mut iyr = IYR_RE.captures(&line);
        let mut pid = PID_RE.captures(&line);

        if loose {
            byr = BYR_RE_LOOSE.captures(&line);
            cid = CID_RE_LOOSE.captures(&line);
            ecl = ECL_RE_LOOSE.captures(&line);
            eyr = EYR_RE_LOOSE.captures(&line);
            hcl = HCL_RE_LOOSE.captures(&line);
            hgt = HGT_RE_LOOSE.captures(&line);
            iyr = IYR_RE_LOOSE.captures(&line);
            pid = PID_RE_LOOSE.captures(&line);
        }

        let height: Option<Height> = match &hgt {
            Some(v) => match v.get(2) {
                Some(height) => height.as_str().parse().ok(),
                None => None,
            },
            None => None,
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
            cid: get_string_value(cid),
            ecl: get_string_value(ecl),
            eyr: get_int_value(eyr),
            hcl: get_string_value(hcl),
            hgt: (get_int_value(hgt), height),
            iyr: get_int_value(iyr),
            pid,
        })
    }
}

fn parse_lines(input: &str, loose: bool) -> Vec<Passport> {
    let data: Vec<&str> = input.lines().collect();

    data.iter()
        .group_by(|&&l| l.is_empty())
        .into_iter()
        .map(|(_, g)| join(g, " "))
        .filter(|l| !l.is_empty())
        .filter_map(|l| Passport::new(&l, loose))
        .collect()
}

#[aoc_generator(day4, part1)]
pub fn input_generator_part_01(input: &str) -> Vec<Passport> {
    parse_lines(input, true)
}

#[aoc_generator(day4, part2)]
pub fn input_generator_part_02(input: &str) -> Vec<Passport> {
    parse_lines(input, false)
}

/* Part One
 *
 * You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport.
 * While these documents are extremely similar, North Pole Credentials aren't issued by a
 * country and therefore aren't actually valid documentation for travel in most of the world.
 *
 * It seems like you're not the only one having problems, though; a very long line has formed for the
 * automatic passport scanners, and the delay could upset your travel itinerary.
 *
 * Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.
 *
 * The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields.
 * The expected fields are as follows:
 *
 * byr (Birth Year)
 * iyr (Issue Year)
 * eyr (Expiration Year)
 * hgt (Height)
 * hcl (Hair Color)
 * ecl (Eye Color)
 * pid (Passport ID)
 * cid (Country ID)
 * Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence
 * of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.
 *
 * Here is an example batch file containing four passports:
 *
 * ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
 * byr:1937 iyr:2017 cid:147 hgt:183cm
 *
 * iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
 * hcl:#cfa07d byr:1929
 *
 * hcl:#ae17e1 iyr:2013
 * eyr:2024
 * ecl:brn pid:760753108 byr:1931
 * hgt:179cm
 *
 * hcl:#cfa07d eyr:2025 pid:166559648
 * iyr:2011 ecl:brn hgt:59in
 * The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).
 *
 * The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials,
 * not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.
 *
 * The fourth passport is missing two fields, cid and byr. Missing cid is fine,
 * but missing any other field is not, so this passport is invalid.
 *
 * According to the above rules, your improved system would report 2 valid passports.
 *
 * Count the number of valid passports - those that have all required fields.
 * Treat cid as optional. In your batch file, how many passports are valid?
*/
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_04::*;
/// let input = include_str!("../input/2020/day4.txt");
/// assert_eq!(solve_part_01(&input_generator_part_01(input)), 200);
/// ```
#[aoc(day4, part1)]
pub fn solve_part_01(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|passport| passport.is_valid_without_validation())
        .count()
}

/* Part Two
 * The line is moving more quickly now, but you overhear airport security talking about how
 * passports with invalid data are getting through. Better add some data validation, quick!
 *
 * You can continue to ignore the cid field, but each other field has strict
 * rules about what values are valid for automatic validation:
 *
 * byr (Birth Year) - four digits; at least 1920 and at most 2002.
 * iyr (Issue Year) - four digits; at least 2010 and at most 2020.
 * eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
 * hgt (Height) - a number followed by either cm or in:
 * If cm, the number must be at least 150 and at most 193.
 * If in, the number must be at least 59 and at most 76.
 * hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
 * ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
 * pid (Passport ID) - a nine-digit number, including leading zeroes.
 * cid (Country ID) - ignored, missing or not.
 *
 * Your job is to count the passports where all required fields are both
 * present and valid according to the above rules. Here are some example values:
 *
 * byr valid:   2002
 * byr invalid: 2003
 *
 * hgt valid:   60in
 * hgt valid:   190cm
 * hgt invalid: 190in
 * hgt invalid: 190
 *
 * hcl valid:   #123abc
 * hcl invalid: #123abz
 * hcl invalid: 123abc
 *
 * ecl valid:   brn
 * ecl invalid: wat
 *
 * pid valid:   000000001
 * pid invalid: 0123456789
 * Here are some invalid passports:
 *
 * eyr:1972 cid:100
 * hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
 *
 * iyr:2019
 * hcl:#602927 eyr:1967 hgt:170cm
 * ecl:grn pid:012533040 byr:1946
 *
 * hcl:dab227 iyr:2012
 * ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
 *
 * hgt:59cm ecl:zzz
 * eyr:2038 hcl:74454a iyr:2023
 * pid:3556412378 byr:2007
 * Here are some valid passports:
 *
 * pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
 * hcl:#623a2f
 *
 * eyr:2029 ecl:blu cid:129 byr:1989
 * iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
 *
 * hcl:#888785
 * hgt:164cm byr:2001 iyr:2015 cid:88
 * pid:545766238 ecl:hzl
 * eyr:2022
 *
 * iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
 * Count the number of valid passports - those that have all required fields and valid values.
 * Continue to treat cid as optional. In your batch file, how many passports are valid?
*/
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_04::*;
/// let input = include_str!("../input/2020/day4.txt");
/// assert_eq!(solve_part_02(&input_generator_part_02(input)), 116);
/// ```
#[aoc(day4, part2)]
pub fn solve_part_02(input: &[Passport]) -> usize {
    input.iter().filter(|passport| passport.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

        assert_eq!(solve_part_01(&input_generator_part_01(data)), 2)
    }

    /// Test example data on part 2
    #[test]
    fn sample_data_invalid_02() {
        let data = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
    ";

        assert_eq!(solve_part_02(&input_generator_part_02(data)), 0)
    }

    #[test]
    fn sample_data_valid_02() {
        let data = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    ";

        assert_eq!(solve_part_02(&input_generator_part_02(data)), 4)
    }

    #[test]
    fn valid_passport_birth_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_birth_year() {
        let p = Passport::new(
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1919",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_issue_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_issue_year() {
        let p = Passport::new(
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2021 byr:1980",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_expiration_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_expiration_year() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2031 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_height_in() {
        let p = Passport::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_height_in() {
        let p = Passport::new(
            "pid:087499704 hgt:77in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_height_cm() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_height_cm() {
        let p = Passport::new(
            "pid:087499704 hgt:200cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn invalid_passport_height_without_unit() {
        let p = Passport::new(
            "pid:087499704 hgt:200 ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_hair_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_hair_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#62zz22",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn invalid_passport_hair_color_without_hash() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_eye_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_eye_color() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:brun iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_pid() {
        let p = Passport::new(
            "pid:087499704 hgt:177cm ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }

    #[test]
    fn invalid_passport_pid() {
        let p = Passport::new(
            "pid:1 hgt:177cm ecl:brn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn invalid_passport_pid_too_long() {
        let p = Passport::new(
            "pid:1234567891 hgt:177cm ecl:brn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            false,
        )
        .unwrap();

        assert!(!p.is_valid());
    }

    #[test]
    fn valid_passport_pid_last() {
        let p = Passport::new(
            "hgt:177cm ecl:brn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f pid:123456789",
            false,
        )
        .unwrap();

        assert!(p.is_valid());
    }
}
