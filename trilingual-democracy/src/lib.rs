/*
Switzerland has four official languages: German, French, Italian, and Romansh.

When native speakers of one or more of these languages meet, they follow certain regulations to choose a language for the group.

Here are the rules for groups of exactly three people:

When all three are native speakers of the same language, it also becomes their group's language.a

When two are native speakers of the same language, but the third person speaks a different language,

all three will converse in the minority language.

When native speakers of three different languages meet, they eschew these three languages and instead use the remaining

of the four official languages.

The languages are encoded by the letters D for Deutsch, F for Français, I for Italiano, and K for Rumantsch.

Your task is to write a function that takes a list of three languages and returns the language the group should use.

Examples:

The language for a group of three native French speakers is French: FFF → F

A group of two native Italian speakers and a Romansh speaker converses in Romansh: IIK → K

When three people meet whose native languages are German, French, and Romansh, the group language is Italian: DFK → I
*/

use std::collections::HashSet;

pub fn _trilingual_democracy(group: &[u8; 3]) -> u8 {
    let languages: HashSet<char> = group.iter().map(|&f| f as char).collect();

    if languages.len() == 1 {
        return group[0];
    } else if languages.len() == 2 {
        for lang in languages {
            if group.iter().filter(|&l| *l as char == lang).count() == 1 {
                return lang as u8;
            }
        }
    } else {
        let all_languages: HashSet<char> = HashSet::from(['D', 'F', 'I', 'K']);
        let remaining: Vec<_> = all_languages.difference(&languages).cloned().collect();
        return remaining[0] as u8;
    }

    0
}

// clever soluction ->
pub fn trilingual_democracy(group: &[u8; 3]) -> u8 {
    group[0] ^ group[1] ^ group[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(group: &[u8; 3], lang: u8) {
        let want = lang as char;
        let got = trilingual_democracy(group) as char;
        assert_eq!(
            want,
            got,
            "for group {}: expected {}, got {}",
            std::str::from_utf8(group).unwrap(),
            want,
            got
        );
    }

    #[test]
    fn test_examples() {
        test(b"FFF", b'F');
        test(b"IIK", b'K');
        test(b"DFK", b'I');
    }
}
