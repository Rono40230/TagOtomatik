use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref PARTICULES: HashSet<&'static str> = {
        let mut s = HashSet::new();
        let list = vec![
            // English
            "a", "an", "the", "and", "but", "or", "nor", "at", "by", "for", "from", "in", "into", "of", "off", "on", "onto", "out", "over", "up", "with", "to", "as", "via", "under",
            // French
            "le", "la", "les", "un", "une", "des", "du", "de", "et", "ou", "mais", "ni", "car", "dans", "par", "pour", "en", "vers", "avec", "sans", "sous", "sur", "chez"
        ];
        for p in list { s.insert(p); }
        s
    };

    pub static ref ROMAN_NUMERALS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        let list = vec![
            "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X",
            "XI", "XII", "XIII", "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX",
            "XXI", "XXII", "XXIII", "XXIV", "XXV", "XXX", "XL", "L", "LX", "LXX",
            "LXXX", "XC", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M"
        ];
        for p in list { s.insert(p); }
        s
    };

    pub static ref ABBREVIATIONS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        let list = vec![
            "USA", "UK", "US", "DJ", "MC", "NYC", "LA", "SF", "DC", "CD", "DVD",
            "TV", "FM", "PM", "BC", "AD", "CEO", "FBI", "CIA", "NASA",
            "BBC", "CNN", "ESPN", "MTV", "VHS", "GPS", "WWW", "HTTP", "FTP"
        ];
        for p in list { s.insert(p); }
        s
    };
}
