type Trigram = [char; 3];

pub fn get_similarity(string_1: String, string_2: String) -> f32
{
    let string_1_vec: Vec<Trigram> = make_trigrams(string_1);
    let string_2_vec: Vec<Trigram> = make_trigrams(string_2);

    let large_num: f32 = 
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

    matches / large_num * 100.
}

fn make_trigrams(string: String) -> Vec<Trigram>
{
    if string.len() < 3
    {
        return Vec::new();
    }
    
    let string = "  ".to_owned() + &string + " ";

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

    trigram_vec // Return trigram_vec
}

fn safe_char(c: char) -> char
{
    if let Some(c1) = c.to_lowercase().collect::<Vec<char>>().first()
    {
        return *c1
    }
    c
}
