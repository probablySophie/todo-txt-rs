use std::io;

type Trigram = [char; 3];

/// Gets the similarity between two `&str` inputs using Trigrams!
/// The level of similarity depends on the length of the strings and the number of differences
/// *The similarity can get very low, very quickly*
///
/// # Errors
///
/// - Returns -1. if unable to make a 
pub fn get_similarity(string_1: &str, string_2: &str) -> i32
{
    // Either make the Trigram vectors or return -1
    let string_1_vec = make_trigrams(string_1);
    let string_2_vec = make_trigrams(string_2);

    if string_1_vec.is_err()
    || string_2_vec.is_err()
    {
        return -1
    }

    let string_1_vec: Vec<Trigram> = string_1_vec.unwrap();
    let string_2_vec: Vec<Trigram> = string_2_vec.unwrap();

    
    #[allow(clippy::cast_precision_loss)]
    let small_num: f32 = 
        if string_1_vec.len() > string_2_vec.len() 
             {string_1_vec.len() as f32}
        else {string_2_vec.len() as f32};
   
    let mut matches: f32 = 0.;

    for trigram_1 in string_1_vec
    {
        for trigram_2 in string_2_vec.clone()
        {
            if trigram_1 == trigram_2
            {
                matches += 1.;
                continue;
            }
        }
    }

    #[cfg(test)]
    println!("Similarity: {}", matches / small_num * 100.);

    #[allow(clippy::cast_possible_truncation)]
    return (matches / small_num * 100.).round() as i32    
}

fn make_trigrams(string: &str) -> io::Result<Vec<Trigram>>
{
    if string.len() < 3
    {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidData, 
                "Input &str was not long enough, must be at least 3 chars"
            )
        )
    }
    
    let string = "  ".to_owned() + string + " ";

    let mut trigram_vec: Vec<Trigram> = Vec::new();

    let mut string_chars = string.chars();

    // We made sure the string len was at least 3, so this SHOULDN'T be able to panic
    let mut working_trigram: Trigram = [
        safe_char(string_chars.next().unwrap()), 
        safe_char(string_chars.next().unwrap()), 
        safe_char(string_chars.next().unwrap()), 
    ];

    for c in string_chars
    {
        trigram_vec.push( working_trigram );
        working_trigram[0] = working_trigram[1];
        working_trigram[1] = working_trigram[2];
        working_trigram[2] = safe_char(c);
    }
    trigram_vec.push( working_trigram );

    Ok(trigram_vec) // Return trigram_vec
}

fn safe_char(c: char) -> char
{
    if let Some(c1) = c.to_lowercase().collect::<Vec<char>>().first()
    {
        return *c1
    }
    c
}


#[cfg(test)]
mod test
{    
    #[test] fn safe_char_lower () { assert_eq!(super::safe_char('a'), 'a') }
    #[test] fn safe_char_upper () { assert_eq!(super::safe_char('A'), 'a') }
    #[test] fn safe_char_number() { assert_eq!(super::safe_char('1'), '1') }
    #[test] fn safe_char_symbol() { assert_eq!(super::safe_char('@'), '@') }

    // Floats are my enemy
    // https://rust-lang.github.io/rust-clippy/master/index.html#/float_cmp
    fn assert_eq_f(a: f32, b: f32)
    {
        println!("{a} < {b}");
        assert!((a - b).abs() < 0.01);
    }

    #[test] fn similarity() { assert_eq!(super::get_similarity(
        "Far from the old watch-tower", 
        "Far from the old watch-tower"), 100); }

    #[test] fn similarity_caps() { assert_eq!(super::get_similarity(
        "Far from the old watch-tower", 
        "Far FROM THE old watcH-tOwEr"), 100); }
    
    #[test] fn similarity_very() { assert!(super::get_similarity(
        "Far from the old watch-tower", 
        "Far from th3 old watch-tower") > 85); }

    #[test] fn simularity_invalid() {assert_eq!(super::get_similarity("", ""), -1)}

}
