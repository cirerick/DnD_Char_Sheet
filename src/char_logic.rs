use crate::user_input_logic;

//Character Profile Start
pub struct CharacterProfile  {
    pub player_name: String,
    pub character_name: String,
    pub class: String,
    pub level: u8,
    pub background: String,
    pub race: String,
    pub alignment: String,
    pub exp: u16,
}

impl CharacterProfile {
    
    pub fn new() -> Self {
        return
        CharacterProfile {
            player_name: "Player Name".to_string(),
            character_name: "Character Name".to_string(),
            class: "Class".to_string(),
            level: 0,
            background: "Background".to_string(),
            race: "Race".to_string(),
            alignment: "Alignment".to_string(),
            exp: 0,
        }
    }
    

    pub fn set_attributes(&mut self) {
        println!("\nChoose which PROFILE change?");
        match user_input_logic::cin_char(){
            'n' => self.player_name = user_input_logic::cin_string(),
            'c' => self.character_name = user_input_logic::cin_string(),
            's' => self.class = user_input_logic::cin_string(),
            'l' => self.level = user_input_logic::cin_u8(),
            'b' => self.background = user_input_logic::cin_string(),
            'r' => self.race = user_input_logic::cin_string(),
            'a' => self.alignment = user_input_logic::cin_string(),
            'e' => self.exp = user_input_logic::cin_u16(), 
            _ => println!("Nothing was changed."),
        }
    }


    //print current character profile struct
    pub fn print_character_profile(&self){
        println!("Player Name: {}", self.player_name);
        println!("Character Name: {}", self.character_name);
        println!("Class: {}", self.class);
        println!("Level: {}", self.level);
        println!("Background: {}", self.background);
        println!("Race: {}", self.race);
        println!("Alignment: {}", self.alignment);
        println!("Exp: {}", self.exp); 
    }

}
//Character Profile End

//Characterists Start
pub struct Characteristics {
    pub personality_traits: String,
    pub ideals: String,
    pub bonds: String,
    pub flaws: String,  
}

impl Characteristics {

    pub fn new() -> Self {
        return
        Characteristics{
            personality_traits: "Personality Traits".to_string(),
            ideals: "Ideals".to_string(),
            bonds: "Bonds".to_string(),
            flaws: "Flaws".to_string(),
        }
    }

    pub fn set_attributes(&mut self) {
        println!("\nChoose which CHARACTERISTIC to change?");
        match user_input_logic::cin_char() {
            'p' => self.personality_traits = user_input_logic::cin_string(),
            'i' => self.ideals = user_input_logic::cin_string(),
            'b' => self.bonds = user_input_logic::cin_string(),
            'f' => self.flaws = user_input_logic::cin_string(),
            _ => println!("Nothing was changed."),
        }
    }


    pub fn print_characteristics(&self){
        println!("Personality Traits: {}", self.personality_traits);
        println!("Ideals: {}", self.ideals);
        println!("Bonds: {}", self.bonds);
        println!("Flaws: {}", self.flaws);
    }    

}
//End of Characteristics

//Character Skills Start
pub struct CharacterSkills {
    //Attributes and their saving throws modifiers
    pub strength_and_save: (u16, i8),
    pub dexterity_and_save: (u16, i8),
    pub constitution_and_save: (u16, i8),
    pub intelligence_and_save: (u16, i8),
    pub charisma_and_save: (u16, i8),

    //Skills
    pub acrobat: i8,
    pub animal_handling: i8,
    pub athletics: i8,
    pub deception: i8,
    pub history: i8,
    pub insight: i8,
    pub intimidation: i8,
    pub medicine: i8,
    pub nature: i8,
    pub preception: i8,
    pub performance: i8,
    pub religion: i8,
    pub sleight_of_hand: i8, 

}

impl CharacterSkills {

    pub fn new() -> Self {
        return
        CharacterSkills{
            strength_and_save: (0, 0),
            dexterity_and_save: (0, 0),
            constitution_and_save: (0, 0),
            intelligence_and_save: (0, 0),
            charisma_and_save: (0,0), 

            acrobat: 0,
            animal_handling: 0,
            athletics: 0,
            deception: 0,
            history: 0,
            insight: 0,
            intimidation: 0,
            medicine: 0,
            nature: 0,
            preception: 0,
            performance: 0,
            religion: 0,
            sleight_of_hand: 0,
        }
    }

    pub fn set_skills(&mut self){
        println!("\nSelect which SKILL to change?");
        match user_input_logic::cin_char(){
            'S' => self.strength_and_save.0 = user_input_logic::cin_u16(),
            's' => self.strength_and_save.1 = user_input_logic::cin_i8(), 
            'D' => self.dexterity_and_save.0 = user_input_logic::cin_u16(),
            'd' => self.dexterity_and_save.1 = user_input_logic::cin_i8(),
            'C' => self.constitution_and_save.0 = user_input_logic::cin_u16(),
            'c' => self.constitution_and_save.1 = user_input_logic::cin_i8(),
            'I' => self.intelligence_and_save.0 = user_input_logic::cin_u16(), 
            'i' => self.intelligence_and_save.1 = user_input_logic::cin_i8(), 
            'R' => self.charisma_and_save.0 = user_input_logic::cin_u16(),
            'r' => self.charisma_and_save.1 = user_input_logic::cin_i8(),
            
            'a' => self.acrobat = user_input_logic::cin_i8(),
            'b' => self.animal_handling = user_input_logic::cin_i8(),
            'e' => self.athletics = user_input_logic::cin_i8(),
            'f' => self.deception = user_input_logic::cin_i8(),
            'g' => self.history = user_input_logic::cin_i8(),
            'h' => self.insight = user_input_logic::cin_i8(),
            'j' => self.intimidation = user_input_logic::cin_i8(),
            'k' => self.medicine = user_input_logic::cin_i8(),
            'm' => self.nature = user_input_logic::cin_i8(),
            'n' => self.preception = user_input_logic::cin_i8(),
            'o' => self.performance = user_input_logic::cin_i8(),
            'p' => self.religion = user_input_logic::cin_i8(),
            'q' => self.sleight_of_hand = user_input_logic::cin_i8(),
            _ => println!("Nothing happened"),

            
        }
    }
    
    pub fn print_skills(&self){
        println!("\n");
    }
}
//End Character Skills


//Start Character Status
pub struct CharacterStatus {
    inspiration: u8,
    proficiency_bonus: u8,

    armor_class: u8,
    initiative: u8,
    speed: u16,
    max_and_current_hp: (u16, u16),
    temp_hp: u16,
    
    total_and_used_hitdice: (String, u8), 
    success_and_failed_death_saves: (u8, u8),
}
 impl CharacterStatus {

    pub fn new() -> Self {
        return 
        CharacterStatus{
            inspiration: 0, 
            proficiency_bonus: 0,
            
            armor_class: 0,
            initiative: 0, 
            speed: 0,
            max_and_current_hp: (0,0), 
            temp_hp: 0,

            total_and_used_hitdice: ("0d0".to_string(), 0),
            success_and_failed_death_saves: (3, 3),
        }
    }

    pub fn set_stats(&mut self) {
        println!("\nSelect which stat to mutate?");
    }

    pub fn print_stats(&self){
        
    }
}
//End Character Status


//Debugging purposes
pub fn print_character_string(str: String){
    println!("This is what it is set to: {}", str);
}

pub fn struct_pick(profile: &mut CharacterProfile, 
    characteristics: &mut Characteristics, 
    skills: &mut CharacterSkills,
    stats: &mut CharacterStatus){
    println!("\nSelect Character Profile attribute");
    match user_input_logic::cin_char(){
        'p' => CharacterProfile::set_attributes(profile),
        'c' => Characteristics::set_attributes(characteristics),
        'z' => CharacterSkills::set_skills(skills),
        's' => CharacterStatus::set_stats(stats),
        _ => println!("Nothing happened."),
    }
}

