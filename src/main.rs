//import file
mod char_logic;
mod user_input_logic;

//use the struct and their implementations
use char_logic::{CharacterProfile, Characteristics, CharacterSkills, CharacterStatus};

fn main() { 
    let mut user_input: char = '?';

    //Create and fill with placeholder data - 9 placeholder
    let mut profile: CharacterProfile = CharacterProfile::new();
    let mut characteristics: Characteristics = Characteristics::new();
    let mut skills: CharacterSkills = CharacterSkills::new();
    let mut stats: CharacterStatus = CharacterStatus::new();
    //Debug - print entire character profile
    char_logic::CharacterProfile::print_character_profile(&profile);

    

    while user_input != 'q' {
        //take user input to select page <-> then to select struct from page <-> select field
        println!("\nSelect page to change.");
        user_input = user_input_logic::cin_char();
        match user_input {
            'c' => char_logic::struct_pick(&mut profile, &mut characteristics, &mut skills, &mut stats),
            'e' => continue, //for now
            's' => continue, //for now
            _ => continue,
        }
    }
   

    
    char_logic::CharacterProfile::print_character_profile(&profile);

}


