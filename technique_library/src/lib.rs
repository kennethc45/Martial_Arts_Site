pub mod technique_catalog {
    use::std::collections::HashMap;

    #[derive(Debug)]
    pub enum TechniqueCategory {
        Strike(StrikeCategory),
        Submission(SubmissionCategory),
        Evasion(EvasionCategory),
        Takedown(TakedownCategory),
        Defense
    }

    #[derive(Debug)]
    pub enum StrikeCategory {
        Punch,
        Kick,
        Knee,
        Elbow
    }

    #[derive(Debug)]
    pub enum SubmissionCategory {
        JointLock,
        ChokeOrStrangle,
        Pin
    }

    #[derive(Debug)]
    pub enum EvasionCategory {
        Headmovement,
        Evasion 
    }

    #[derive(Debug)]
    pub enum TakedownCategory {
        Takedown,
        Sweep,
        Thow
    }



    pub struct Technique {
        name: String,
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
        pub fn add_technique(&mut self, name: String, technique: Technique) {
            self.techniques.insert(name, technique);
        }
    }
}
