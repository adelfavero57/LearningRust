fn main() {
    let vowel = String::from("apple");
    let cons = String::from("hello");
    let cons2 = String::from("bingus");
    pig_latin(vowel);
    pig_latin(cons);
    pig_latin(cons2)
}

fn pig_latin(s: String) {
    let first_letter = &s[..1];
    if is_consonant(&first_letter) {
        let pig_latin = s[1..].to_string() + first_letter + &"ay";
        println!("Pig latin = {pig_latin}");
    }
    
    if is_vowel(&first_letter) {
        let pig_latin = s + &"hay";
        println!("Pig latin = {pig_latin}");
    }
}

fn is_consonant(c: &str) -> bool {
    let consonants = ["b","c","d","f","g","h","j","k","l","m","n","p","q","r","s","t","v","w","x","y","z"];
    if consonants.contains(&c) == true {
        return true;
    }
    return false;
}

fn is_vowel(c: &str) -> bool {
    let vowels = ["a","e","i","o","u"];
    if vowels.contains(&c) == true {
        return true;
    }
    return false;
}
