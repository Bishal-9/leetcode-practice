
/*

5. Longest Palindromic Substring

https://leetcode.com/problems/longest-palindromic-substring/

*/

use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut _input = String ::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    let _result = _longest_palindrome(_input);
    println!("{}", _result);
}

fn _longest_palindrome(word: String) -> String {

    let mut _result: &str = "";
    let length = word.len();

    for _index in 0..length {

        for _index_ in 1..=length {

            if _index > (length - _index_) {
                continue;
            }

            let word_to_check = word.get(_index..=(length - _index_)).unwrap();
            if _check_palindrome(word_to_check) && word_to_check.len() > _result.len() {
                _result = word_to_check;
            }
        }
    }

    _result.to_string()
}

fn _check_palindrome(_word: &str) -> bool {
    let mut _result = true;
    let length = _word.len();
    let characters: Vec<char> = _word.chars().collect();

    if length < 1 {
        return false;
    }
    
    for _index in 0..=(length / 2) {
        if characters[_index] != characters[length - (_index + 1)] {
            _result = false;
        }
    }

    _result
}

#[cfg(test)]
mod tests {
    use super::_longest_palindrome;

    #[test]
    fn test_case_1() {
        let _result = _longest_palindrome(String::from("babad"));
        assert_eq!(_result, String::from("bab"));
    }

    #[test]
    fn test_case_2() {
        let _result = _longest_palindrome(String::from("cbbd"));
        assert_eq!(_result, String::from("bb"));
    }

    #[test]
    fn test_case_3() {
        let _result = _longest_palindrome(String::from("a"));
        assert_eq!(_result, String::from("a"));
    }

    #[test]
    fn test_case_4() {
        let _result = _longest_palindrome(String::from("mwwfjysbkebpdjyabcfkgprtxpwvhglddhmvaprcvrnuxifcrjpdgnktvmggmguiiquibmtviwjsqwtchkqgxqwljouunurcdtoeygdqmijdympcamawnlzsxucbpqtuwkjfqnzvvvigifyvymfhtppqamlgjozvebygkxawcbwtouaankxsjrteeijpuzbsfsjwxejtfrancoekxgfyangvzjkdskhssdjvkvdskjtiybqgsmpxmghvvicmjxqtxdowkjhmlnfcpbtwvtmjhnzntxyfxyinmqzivxkwigkondghzmbioelmepgfttczskvqfejfiibxjcuyevvpawybcvvxtxycrfbcnpvkzryrqujqaqhoagdmofgdcbhvlwgwmsmhomknbanvntspvvhvccedzzngdywuccxrnzbtchisdwsrfdqpcwknwqvalczznilujdrlevncdsyuhnpmheukottewtkuzhookcsvctsqwwdvfjxifpfsqxpmpwospndozcdbfhselfdltmpujlnhfzjcgnbgprvopxklmlgrlbldzpnkhvhkybpgtzipzotrgzkdrqntnuaqyaplcybqyvidwcfcuxinchretgvfaepmgilbrtxgqoddzyjmmupkjqcypdpfhpkhitfegickfszermqhkwmffdizeoprmnlzbjcwfnqyvmhtdekmfhqwaftlyydirjnojbrieutjhymfpflsfemkqsoewbojwluqdckmzixwxufrdpqnwvwpbavosnvjqxqbosctttxvsbmqpnolfmapywtpfaotzmyjwnd"));
        assert_eq!(_result, String::from("khvhk"));
    }

    #[test]
    fn test_case_5() {
        let _result = _longest_palindrome(String::from("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone"));
        assert_eq!(_result, String::from("wfdfw"));
    }

    #[test]
    fn test_case_6() {
        let _result = _longest_palindrome(String::from("nmngaowrbsssvihklwmuqshcddwlxrywrlwtennwfvrevgvhsvgeccfulmuvrcksdmgeqrblnlwoepefhcwhmgyvgcoyyygrmttyfycxwbqktpurlcfhzlakhmrddsydgygganpmaglaxyhfwjusukzcnakznygqplngnkhcowavxoiwrfycxwdkxqfcjqwyqutcpyedbnuogedwobsktgioqdczxhikjrbkmqspnxcpngfdwdaboscqbkwforihzqdcppxjksiujfvlpdjryewaxgmdgigvxdlstxwngtbdrrkfudjinzyxbdmkautclvvyguekuzwwetmsxittgtxbnvvrgasvnlogdiepltweaehubwelznidltzlbzdsrxmhjpkmylnwkdsxnpkplkdzywioluaqguowtbaoqzqgjfewphqcvlnwlojbxgomvxxkhwwykawegxubjiobizicuxzeafgautefsurgjlbhcfevqzsbhwxycrcaibdsgluczcuewzqupakbzmcvzsfodbmgtugnihyhqkvyeboqhqldifbxuaxqzxtyejoswikbzpsvzkxcndgeyvfnyrfbkhlalzpqjueibnodamgpnxlkvwvliouvejcpnakllfxepldfmdzszagkyhdgqqbkb"));
        assert_eq!(_result, String::from("uczcu"));
    }
}
