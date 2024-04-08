pub mod technique_catalog {
    use::std::collections::HashMap;

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum TechniqueCategory {
        Strike(StrikeCategory),
        Submission(SubmissionCategory),
        Evasion(EvasionCategory),
        Takedown(TakedownCategory),
        Defense
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum StrikeCategory {
        Punch,
        Kick,
        Knee,
        Elbow
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum SubmissionCategory {
        JointLock,
        ChokeOrStrangle,
        Pin
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum EvasionCategory {
        Headmovement,
        Evasion 
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum TakedownCategory {
        Takedown,
        Sweep,
        Thow
    }


    #[derive(Clone)]
    pub struct Technique {
        pub name: String,
        category: TechniqueCategory,
        description: String 
    }

    impl Technique {
        pub fn new(name: String, category: TechniqueCategory, description: String) -> Self {
            Technique {
                name,
                category,
                description 
            }
        }

        pub fn display(&self) {
            println!("Name: {}", self.name);
            println!("Category: {:?}", self.category);
            println!("Description: {}", self.description);
        }
    }

    pub struct TechniqueLibrary {
        techniques:HashMap<String, Technique>,
    }

    impl TechniqueLibrary {
        pub fn new() -> Self {
            TechniqueLibrary {
                techniques: HashMap::new(),
            }
        }

        pub fn add_technique(&mut self, name: String, technique: Technique) {
            self.techniques.insert(name, technique);
        }

        pub fn get_technique(&self, name: &str) -> Result<&Technique, String> {
            match self.techniques.get(name) {
                Some(technique) => Ok(technique),
                None => Err(format!("Technique with name '{}' not found", name)),
            }
        }

        pub fn update_technqique(&mut self, name: &str, updated_technique: Technique) -> bool {
            if self.techniques.contains_key(name) {
                self.techniques.insert(name.to_string(), updated_technique);
                return true;
            } else {
                return false;
            }
        }

        pub fn delete_technique(&mut self, name: &str) -> bool {
            if self.techniques.contains_key(name) {
                self.techniques.remove(name);
                return true;
            } else {
                return false;
            }
        }
    }
}
