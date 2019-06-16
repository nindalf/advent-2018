use std::collections::HashMap;

#[allow(dead_code)]
fn checksum(ids: &[&str]) -> usize {
    let twos = ids.iter().filter(|s| character_counts(s, 2)).count();

    let threes = ids.iter().filter(|s| character_counts(s, 3)).count();

    twos * threes
}

fn character_counts(s: &str, n: usize) -> bool {
    let matching_chars = counts(s).values().filter(|count| **count == n).count();
    matching_chars > 0
}

fn counts(s: &str) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        result.entry(c)
            .and_modify(|cur| *cur = *cur + 1)
            .or_insert(1);
    }
    result
}

#[allow(dead_code)]
fn correct_id(ids: &[&str]) -> String {
    for (i, id) in ids.iter().enumerate() {
        for id2 in ids.iter().skip(i + 1) {
            let (differences, common) = string_diff(id, id2);
            if differences == 1 {
                return common;
            }
        }
    }
    String::from("no matches found")
}

fn string_diff(s1: &str, s2: &str) -> (usize, String) {
    let differences = s1.chars().zip(s2.chars()).filter(|(x, y)| x != y).count();
    let commons: String = s1
        .chars()
        .zip(s2.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect();
    (differences, commons)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count() {
        let test_input = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(12, super::checksum(&test_input));
        assert_eq!(8610, super::checksum(&input));
    }

    #[test]
    fn test_correct_id() {
        let test_input = vec![
            "abcde", "fghij", "fguij", "klmno", "pqrst", "axcye", "wvxyz",
        ];
        assert_eq!(String::from("fgij"), super::correct_id(&test_input));
        assert_eq!(
            String::from("iosnxmfkpabcjpdywvrtahluy"),
            super::correct_id(&input)
        );
    }

    #[test]
    fn test_string_diff() {
        assert_eq!(
            (2, String::from("ace")),
            super::string_diff("abcde", "axcye")
        );
        assert_eq!(
            (0, String::from("abcde")),
            super::string_diff("abcde", "abcde")
        );
        assert_eq!(
            (1, String::from("fgij")),
            super::string_diff("fghij", "fguij")
        );
        assert_eq!((5, String::from("")), super::string_diff("abcde", "fghij"));
    }

    lazy_static! {
        static ref input: Vec<&'static str> = vec![
            "uosnxmfkezbojfdgwvrtaqhluy",
            "iosnxmfkazbcopdgnvrtaqhluy",
            "ioanxmfkezbcjpdgwvrjaohluy",
            "uosnxmfkezbcjpjgwvrtaqhlut",
            "imsnxmfkezbcjpugwvataqhluy",
            "ioenxmfkezbcjpdgwvrraqhluz",
            "iosnxmfkezbcjpdgevitnqhluy",
            "iosnxmfkezcccpdgcvrtaqhluy",
            "loinxmfkezbcjpdgwvrtaqhluu",
            "iosnlmfkezbczndgwvrtaqhluy",
            "iosnxmfkezbcjpdgwvrifghluy",
            "iosnuhfkezbcjpugwvrtaqhluy",
            "iosnxmfkezbcwpdgwvrtaihlgy",
            "iosnxzfwuzbcjpdgwvrtaqhluy",
            "hosnxmfjizbcjpdgwvrtaqhluy",
            "iornxmfktzbcjpdgwvrtaqhluo",
            "nosnxmfkdzbcjpdgwvrtaqhlwy",
            "iosnxmfkezbcjpdgwvrtaktluq",
            "ioszxmlkezbcjvdgwvrtaqhluy",
            "ionnxmfkezbcfpdgwvbtaqhluy",
            "iosnxmfkezrcjedgwvrtaqhluq",
            "irsnxmfkezbcjpdqwvrtafhluy",
            "ioshxmzkezbccpdgwvrtaqhluy",
            "iosnxmfkezbrjpdgwvothqhluy",
            "bosnxmfkezbcbpdgwvrtnqhluy",
            "iosnomfkszbcjpcgwvrtaqhluy",
            "iosnxmflezbcjpdgwvrtaqmuuy",
            "iobnxmfkezbcjpdgxvrtaqfluy",
            "ioenxmfvezbcjgdgwvrtaqhluy",
            "iosnxmfkekbcjprgwvrtaqhlty",
            "iosnumfkezbcjpmgwvrtaqhlmy",
            "ionnxufkezbcjpdgwvrqaqhluy",
            "tosnxmfbezbcjpdghvrtaqhluy",
            "iosnxmfktzbcjpogwmrtaqhluy",
            "iosnamfkezbjjpdgtvrtaqhluy",
            "iosnemfkezmcjpdgwvrtaqhlry",
            "losnxmfkezbcjpdxwvrtaqsluy",
            "fomnxmekezbcjpdgwvrtaqhluy",
            "rosnxmfkezbcjzdcwvrtaqhluy",
            "iosngmrkezbcjpdgwvrtaqhduy",
            "iosnxmaaebbcjpdgwvrtaqhluy",
            "xosnxmfkezbcjpdgwvrmrqhluy",
            "iosnxmfkgzbujpdgwhrtaqhluy",
            "iosnxmekecbcjpmgwvrtaqhluy",
            "mesnxmfdezbcjpdgwvrtaqhluy",
            "insnxmbkezbcjpdgwvrtgqhluy",
            "iosyxmfkezbcjpdgwirtavhluy",
            "iosnxmfkezbcjpdgwlvtjqhluy",
            "iosnxmtkezbcjpdgwvjtaqhlut",
            "iosnxmfkezbsjpdhwvrtaqaluy",
            "iosnumfkezbcjpfgwvrtaqhlfy",
            "iosnxmekezbcdpxgwvrtaqhluy",
            "iosnfmfkezbcjpdgavctaqhluy",
            "iosnxmfkezvcjpdgfvrtamhluy",
            "iovnxmfkezbcjpdgzvrtaqhzuy",
            "iosnbmfkuzbcjpdgwvrtaqhlux",
            "iosnxmfkezbcjpdgwvftauhluc",
            "iosmbmfkezbcmpdgwvrtaqhluy",
            "ifsnxmfvezbcjpdgwvrwaqhluy",
            "iosnxmfkezfcjpdgwvrmaqhyuy",
            "iospxmfkezbcjpdkwvytaqhluy",
            "issnxmfkyzbcjpdgwyrtaqhluy",
            "iosnxmfkezbcjpdbwvrtjqhluz",
            "iosnxmfkwzbcjpdgfvrtajhluy",
            "iosnxmfkezbcjndgwvrnaqxluy",
            "iosnxmfkezbcjpdgwvltawwluy",
            "iosnxmfkezbcjpdguvrtwqhtuy",
            "iornxmfkezbcjpdgwertaihluy",
            "iofdxmokezbcjpdgwvrtaqhluy",
            "iosnxmfkezbcjpdgwgrtiqdluy",
            "iosnxmfkenbcjpdgwqrtiqhluy",
            "iosnxmfkezbcjpugwvotcqhluy",
            "iksnxmfkezbcjfdgqvrtaqhluy",
            "iasnxmfkezbcjpdgwvrtaqheuo",
            "iosnxmfkehbcipdgwvrtaqtluy",
            "iosnxifkezbajpdgwvrtaahluy",
            "iosnxmpkezbccpdguvrtaqhluy",
            "ioinxnfkezbcjpdgwvgtaqhluy",
            "gosnxmfkezbcjpdgwvrtvqkluy",
            "iolnxmfcezbcjpdgwvrtaqhlgy",
            "iosnxmfkezbcppdgwortjqhluy",
            "iesnxafkezbcjpdgwvrtayhluy",
            "iqsnxmfxazbcjpdgwvrtaqhluy",
            "cosnxmfkezbcjpdgwvrtkahluy",
            "ioenxmfkezbcjpdgwvrtzqyluy",
            "iosnxmhkwzbcjpdgwvrtabhluy",
            "iosnxmfkezbcjpdtwvrhaqiluy",
            "iisnxmfkezbcvpdwwvrtaqhluy",
            "iosnsmfkeobcjpdgfvrtaqhluy",
            "iwsnxmfkfzbcjpugwvrtaqhluy",
            "iosnxmflezbcjpdgwvrtaspluy",
            "gosnimfkezbcjpdgwvrtjqhluy",
            "iosnxmfkfibcjmdgwvrtaqhluy",
            "iosnxmfkpzbcjpdgwvitaqhwuy",
            "ionnxmfkerbcjpjgwvrtaqhluy",
            "iosnxmfkezecjgdgwvrtaqhljy",
            "iosnxufkezbcjpdguvltaqhluy",
            "vosnzmfkezbcjpdvwvrtaqhluy",
            "iolnxmfkecbcjpdgwvrtaqpluy",
            "iosnxmfkezbcjpdgwortaqhouw",
            "iomnxmfkezbckpdgwvrtaqhluu",
            "iopnymfkezbchpdgwvrtaqhluy",
            "iosnxmfkezhcjpdguvrtaqhnuy",
            "iosfxmfkezecjpdgyvrtaqhluy",
            "iopnxmfkgzbcjpdgwvbtaqhluy",
            "tosnxmffezbcjpdgwvttaqhluy",
            "iosnxmfkpabcjpdywvrtaqhluy",
            "iosrxmfkekbcjpdgwvrtaqrluy",
            "iosnxmokezbcjpdjwvrtaxhluy",
            "iolnxmfkezbccpdgwvetaqhluy",
            "iosnxmfketecjpdgwvrtaqnluy",
            "iosnxmfkxzbtjpdgwvroaqhluy",
            "ioinxmfkezbcjpdqwvrtjqhluy",
            "iosnxmfkqzbcjpdgwvrtaqzluz",
            "iosnxmfklzbcjpdgwwrtaqhluh",
            "iosnxmfkezbcjpdtwvrtmqhlpy",
            "iosnomfqezgcjpdgwvrtaqhluy",
            "iosnxmfkezbcjodgwvutaqhduy",
            "iosnxmfkezbcjppgwertaqhluu",
            "iosnxmfkezbcjqdggvrtaqhluw",
            "iosnxmvkezbcjpdgwvrtlqfluy",
            "icsnwmfkezbcjpdiwvrtaqhluy",
            "iosnxxbkezbhjpdgwvrtaqhluy",
            "ioknxmfkezacjpdgwvrtaqhliy",
            "iosgxmfkezbcjpdgevrtpqhluy",
            "iosnxmfkezbejpdgwlrtaqhldy",
            "iosnxyfkezbcjpdowvrtaqhlur",
            "iosnxmfkezbcjpnjwvrtaqhlvy",
            "iosnxglkezbcjpdvwvrtaqhluy",
            "iosnxmpkezbcjpdgwvrtxqhlub",
            "iosnxsfwezbcjpdgwmrtaqhluy",
            "aosnxmfkezbcjpdgwvrtaqhpwy",
            "iopnxmqkkzbcjpdgwvrtaqhluy",
            "iosnxmfkewbcfpdgwvrtaqmluy",
            "iosnxmfkekbcjpdgwvltawhluy",
            "iosnxmfmezbcjpdgwvitaqtluy",
            "iosnomfkezbcjpggwvrtaqhlly",
            "iobnkmfkezbcjpdywvrtaqhluy",
            "yosnxmfkezbcjydgwvrtarhluy",
            "iosnxifkezbckpdgyvrtaqhluy",
            "iornxmfkezbcjpduwvreaqhluy",
            "ivsfxmfjezbcjpdgwvrtaqhluy",
            "iosnxmfkezbcspdgwartaqhlui",
            "iosnxmfkezbcjpdgasstaqhluy",
            "iosnxmfkezbajpdgwvrtaqmlux",
            "gzsnxmfkezxcjpdgwvrtaqhluy",
            "iosnxmikczbcjpdgwvrtyqhluy",
            "iosnxmgkezbcjvdgwdrtaqhluy",
            "iosnxpfkezbcjpdgwvrbachluy",
            "igsnxmfkezbcjpdgwkrtaqtluy",
            "posnxmfkfzbcjpdgwvrpaqhluy",
            "iosnxmfkezbhjtdgwvrtaqhsuy",
            "iosfxmfkezbcjpdwwvrtaqvluy",
            "iosnxmfkehecjpdgwvrtaqoluy",
            "iasnxmfkezbckpdgfvrtaqhluy",
            "iosnxmfkezbwjpdggvrtaqhlmy",
            "iosnxmfkezbcjpdgwvrkaqhbun",
            "iosnxmikezbcjpdgwvrtaqhlnt",
            "iosnxmfiazbcjpdgwvetaqhluy",
            "iosnxmfkczbcjpfgwvrnaqhluy",
            "iosnxmfkezkcjpdgsvrqaqhluy",
            "iosnxmfkezbcspdgwvrtaqhxuc",
            "iosnxmfdezbcjpdgwzrteqhluy",
            "qosnxmrkezbcjpdgwvrtaqhlpy",
            "iosnxmfkpabcjpdywvrtawhluy",
            "ojsnxmfkezbcjpdgwvrtiqhluy",
            "iosrxmfkezbcjpdgdvrtaqhlmy",
            "iosnxmfkezbcnqdgwvrtayhluy",
            "ionnxmfkezbcjpdgwvrsaehluy",
            "iosnxmfkezbcjpdgwvrtmqhpuk",
            "ifsnxmfkezbcjpdpwvrtaqhluf",
            "insnxmfkezbcjpdgwrrtaqhmuy",
            "iosnxmfxezbcjpdjwvrfaqhluy",
            "iojnxmbkezccjpdgwvrtaqhluy",
            "iosnomlkezbcjpdgwvotaqhluy",
            "iosnamfkezbcjpdgwvrhqqhluy",
            "iksnxmfkezbbjrdgwvrtaqhluy",
            "iosnfmfkezbcjpdgwvrtaqhyay",
            "iosnxmzkezbcjpdayvrtaqhluy",
            "iosnxmfkezbcwpdgwbrtaqhlut",
            "iosnxmfkezccjpdgivrtaqhbuy",
            "iosuxmfkezbcjgdgwvrtaqhvuy",
            "ipsnxmfkezbcjpaiwvrtaqhluy",
            "iisnxmfkezbcjpdgpvrtaqqluy",
            "ihsnxmfkezbcspdgwvrtahhluy",
            "imsnxmfkezbcjpdgwvrtaqhkly",
            "josnxmfkezbpjpdgwvttaqhluy",
            "bosnxyfkezmcjpdgwvrtaqhluy",
            "iosnxmfkezbcjpkgwvrtkqhjuy",
            "iosnxmfkezbcjpdgwfrgaqfluy",
            "rosnxmfkqzbcjpdgwvxtaqhluy",
            "iosnxmfkezbcjpdgwlrwaqhluu",
            "yysnxmfkezbcjpdgwvrtaxhluy",
            "iosnxmpkezbcjldgwvrtaqoluy",
            "gosnxmfkezrcjpdgwvrtarhluy",
            "iosnxmfrezbcjrdmwvrtaqhluy",
            "iosnxmfkekbcjpdgpvrtaqhyuy",
            "iosbemfkezbcjpdgwdrtaqhluy",
            "iosnxmfkezucjpdgwvatamhluy",
            "ioanfmfkwzbcjpdgwvrtaqhluy",
            "iosnxphkezbcjpdgwvrtaqhlly",
            "ioynxmfkezbcjvbgwvrtaqhluy",
            "iosnnmfkwzbcjpdgwvrtaqbluy",
            "iosnxmfjezbcjpkgwtrtaqhluy",
            "iosexmfkezbcjpdgwvrtmshluy",
            "irsnxmwkezbcjpdgwvotaqhluy",
            "iosnxmfkezpcjpdgwvrlaqkluy",
            "iosnxmfkezbcjpwgwvroaqkluy",
            "iosnxmfkizbcjpdgwvrtaqxlay",
            "ioszxmfkezbcjpdgwertrqhluy",
            "iosnxmfkczscjpdgwvrtcqhluy",
            "iosnxmfkedbcjpdgwirtaqhliy",
            "iosgxmfpezbcjpdgwvvtaqhluy",
            "iownxmfiezbcjpdgwvrtajhluy",
            "iosnxmfkezbejudgwvrqaqhluy",
            "iomnpmfkezbcjpdgwvwtaqhluy",
            "ioshxmfkecbcjpdgwfrtaqhluy",
            "iosnxmfkezmcjpdgwzrtaqkluy",
            "iownxdfkezdcjpdgwvrtaqhluy",
            "iosnxmfjezbcjpdgwrotaqhluy",
            "roknxmfkezbcjpdgwxrtaqhluy",
            "iosnxmfkeibcjpdgovrtaqhloy",
            "ifsnxmfkelbcjpdgwvrcaqhluy",
            "iosnamfuezbcjpdwwvrtaqhluy",
            "rssnxmfkeebcjpdgwvrtaqhluy",
            "iosnomfkjzbcjpdgwvrtaqhlun",
            "iosnxmfuezbcjpdgwfjtaqhluy",
            "iosnxzfkezbcjpdewvrtaqhlfy",
            "iosnxmfkezbcjpdgwvrtzqhlgr",
            "iosixmfkezbcjpdgwvrkaqhlut",
            "issnxmfkezbdjpdpwvrtaqhluy",
            "iosnxmfrezbcjpdgwkrtaghluy",
            "iysnxmfkezbcjpdgwrrtmqhluy",
            "iosoxmfkezbcjpdgwjrtaqhlua",
            "eosnxmfkezvcjpdgwvztaqhluy",
            "iosmxmckezbcjpdgwvrtaqhlay",
            "iosnxmfkezbcjodgwvrtaqhlma",
            "josnxwftezbcjpdgwvrtaqhluy",
            "iosnxjfkepbcjpdgwvrtaqhlsy",
            "iosnnmfkezbcjpdgwvriaqhnuy",
            "iosnxofkezbcupdgwvrtayhluy",
            "iosnxmfkezbcjpddwvroaqhluz",
            "iosnomfkezbcapdhwvrtaqhluy",
            "iosixmfkezycjpdgwvrtaqhruy",
            "iosnwefkezbcjpdgwvrtaqcluy",
            "iosnxmfkvzbcbpdgwvrhaqhluy",
            "insnxmfkezbczpdgwvrtajhluy",
            "iosnxrfkelbcjpdgwvrtaqhluf",
            "iosnxmfkezbcjpdgwsrtaqhzud",
            "iosnxmfyvzbcjpdgwyrtaqhluy"
        ];
    }
}
