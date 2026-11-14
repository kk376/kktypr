/// Represents the pedagogical stage of a lesson, mirroring Typing Master Pro progression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExerciseType {
    WarmUp,
    WordDrill,
    SentenceDrill,
    LessonReview,
}

impl ExerciseType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::WarmUp => "1. Warm-Up Drill",
            Self::WordDrill => "2. Word Drill",
            Self::SentenceDrill => "3. Sentence Drill",
            Self::LessonReview => "4. Lesson Mastery Test",
        }
    }
}

/// A specific exercise step within a lesson.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Exercise {
    pub exercise_type: ExerciseType,
    pub title: String,
    pub prompt: String,
    pub target_text: String,
    pub minimum_accuracy: f64,
}

impl Exercise {
    /// Returns true if this exercise is an isolated key-preparation drill
    /// (e.g. repeated letter groups like "asdf jkl;" or "ded kik").
    /// Natural sentences, pangrams, word drills, code expressions, and paragraphs return false.
    pub fn is_key_drill(&self) -> bool {
        self.exercise_type == ExerciseType::WarmUp
            && !self.title.contains("Pangram")
            && !self.title.to_lowercase().contains("code")
            && !self.title.to_lowercase().contains("systems")
    }
}

/// A lesson introducing specific keys and containing progressive exercises.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lesson {
    pub number: usize,
    pub title: String,
    pub new_keys: Vec<char>,
    pub finger_guidance: String,
    pub exercises: Vec<Exercise>,
}

/// A complete multi-lesson course.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub lessons: Vec<Lesson>,
}

impl Course {
    pub fn touch_typing_basics() -> Self {
        Self {
            id: "basics".to_string(),
            title: "Course 1: Touch Typing Basics".to_string(),
            description: "12 progressive lessons mastering the full QWERTY keyboard from home row to numbers."
                .to_string(),
            lessons: vec![
                // Lesson 1: Home Row
                Lesson {
                    number: 1,
                    title: "Home Row Keys (A S D F  J K L ;)".to_string(),
                    new_keys: vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';'],
                    finger_guidance: "Left hand: A(pinky) S(ring) D(mid) F(index). Right hand: J(index) K(mid) L(ring) ;(pinky). Thumbs on Space.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Home Row Warm-Up".to_string(),
                            prompt: "Keep your fingers resting gently on ASDF and JKL;. Strike each key without looking down.".to_string(),
                            target_text: "asdf jkl; asdf jkl; fads jkal fads jkal aass ddff jjkk ll;;".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Home Row Words".to_string(),
                            prompt: "Practice typing simple combinations formed with the home row keys.".to_string(),
                            target_text: "all fall salad ask flask glad sad dad lad falls lads asks".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Home Row Sentences".to_string(),
                            prompt: "Type complete home-row phrases at a steady, rhythmic pace.".to_string(),
                            target_text: "a sad lad asks all dads; a glad lad falls; a flask asks a salad;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Home Row Mastery Test".to_string(),
                            prompt: "Complete this review with at least 94% accuracy to complete Lesson 1.".to_string(),
                            target_text: "all lads ask dad; glad dads add salad; a sad lad falls as dad asks all lads;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 2: Keys E and I
                Lesson {
                    number: 2,
                    title: "Keys E and I".to_string(),
                    new_keys: vec!['e', 'i'],
                    finger_guidance: "Left middle finger reaches up for E. Right middle finger reaches up for I. Return immediately to home row.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "E and I Warm-Up".to_string(),
                            prompt: "Practice the upward reach of both middle fingers.".to_string(),
                            target_text: "ded ded kik kik ded kik ded kik de ki ed ik dede kiki".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with E and I".to_string(),
                            prompt: "Type real words incorporating E and I.".to_string(),
                            target_text: "feed file desk side safe leaf like self field life leaf slide".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with E and I".to_string(),
                            prompt: "Maintain cadence across natural phrasing.".to_string(),
                            target_text: "she feels safe inside the field; side files hide a leak; see if life feels like a fresh leaf;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 2 Review".to_string(),
                            prompt: "Aim for clean rhythm over raw speed.".to_string(),
                            target_text: "a wise idea hides inside; like a leaf in a field, she sailed safe beside the lake;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 3: Keys R and U
                Lesson {
                    number: 3,
                    title: "Keys R and U".to_string(),
                    new_keys: vec!['r', 'u'],
                    finger_guidance: "Left index finger reaches up to R. Right index finger reaches up to U. Always snap back to F and J.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "R and U Warm-Up".to_string(),
                            prompt: "Reach with your index fingers without drifting your whole hand.".to_string(),
                            target_text: "frf frf juj juj frf juj fr ju rf uj fur ruf jur fujr".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with R and U".to_string(),
                            prompt: "Form words using R, U, and previously learned letters.".to_string(),
                            target_text: "rule rust user sure cure fire rude pure rider fruit flare".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with R and U".to_string(),
                            prompt: "Keep wrist elevated and light.".to_string(),
                            target_text: "the true rule is fair and sure; pure fire fuels the rustic user ride;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 3 Review".to_string(),
                            prompt: "Combine lessons 1, 2, and 3 into unified muscle memory.".to_string(),
                            target_text: "rare fruits are useful in dire fuels; true riders rule our fair rustic fields;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 4: Period and Comma
                Lesson {
                    number: 4,
                    title: "Punctuation: Period (.) and Comma (,)".to_string(),
                    new_keys: vec!['.', ','],
                    finger_guidance: "Right ring finger reaches down for period. Right middle finger reaches down for comma.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Punctuation Warm-Up".to_string(),
                            prompt: "Drop down smoothly from K to comma and L to period.".to_string(),
                            target_text: "k,k k,k l.l l.l k,k l.l ,,, ... k, l. k, l. ,. ,.".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Punctuation Rhythm Drill".to_string(),
                            prompt: "Type words followed by punctuation and standard space.".to_string(),
                            target_text: "red, blue, safe. fire, dust, dark. sure, fair, real. fuel, seed, leaf.".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Punctuation Sentences".to_string(),
                            prompt: "Build natural pause rhythm after periods and commas.".to_string(),
                            target_text: "if fire rises, find fuel. as life is fair, real users rule, and rustic roads guide us.".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 4 Review".to_string(),
                            prompt: "Test fluent punctuation integration.".to_string(),
                            target_text: "sure, a fair user shares a seed, a leaf, and a pure fuel. see, life rises safe.".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 5: Keys C and N
                Lesson {
                    number: 5,
                    title: "Keys C and N".to_string(),
                    new_keys: vec!['c', 'n'],
                    finger_guidance: "Left middle finger reaches down to C. Right index finger reaches down-left to N.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "C and N Warm-Up".to_string(),
                            prompt: "Practice downward reaches while keeping home anchor fingers in place.".to_string(),
                            target_text: "dcd dcd jnj jnj dcd jnj dc jn cd nj dcdc jnjn".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with C and N".to_string(),
                            prompt: "Type common core words containing C and N.".to_string(),
                            target_text: "clean scan clan face nice coin fence iron chain dance cone".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with C and N".to_string(),
                            prompt: "Type fluent English sentences containing C and N.".to_string(),
                            target_text: "clean code can run nice; find an iron coin inside a clean fence; scan each corner carefully;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 5 Review".to_string(),
                            prompt: "Master smooth bottom-row reach discipline.".to_string(),
                            target_text: "a clean mind can craft an iron fence; coins and chains dance in fine clean sand;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 6: Keys T and O
                Lesson {
                    number: 6,
                    title: "Keys T and O".to_string(),
                    new_keys: vec!['t', 'o'],
                    finger_guidance: "Left index finger reaches up-right to T. Right ring finger reaches up to O.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "T and O Warm-Up".to_string(),
                            prompt: "Alternate between index stretch (T) and ring stretch (O).".to_string(),
                            target_text: "ftf ftf lol lol ftf lol to ot toto lolo toot root".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with T and O".to_string(),
                            prompt: "Expand vocabulary with highly frequent T and O combinations.".to_string(),
                            target_text: "root door fort look stone note cold forest train torch toast".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with T and O".to_string(),
                            prompt: "Focus on relaxed shoulders and even rhythm.".to_string(),
                            target_text: "take note of the cold stone door; train often to foster cool, calm, and solid focus;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 6 Review".to_string(),
                            prompt: "Review all letters learned across lessons 1 through 6.".to_string(),
                            target_text: "to look into the future, take clean action; forest roots drink rain to stand tall and cool;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 7: Keys W and M
                Lesson {
                    number: 7,
                    title: "Keys W and M".to_string(),
                    new_keys: vec!['w', 'm'],
                    finger_guidance: "Left ring finger reaches up to W. Right index finger reaches down to M.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "W and M Warm-Up".to_string(),
                            prompt: "Coordinate left ring reach with right index descent.".to_string(),
                            target_text: "sws sws jmj jmj sws jmj sw jm ws mj swim warm".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with W and M".to_string(),
                            prompt: "Type words featuring W and M.".to_string(),
                            target_text: "warm water storm dream flame modern world wisdom meadow swarm".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with W and M".to_string(),
                            prompt: "Type continuous expressive sentences.".to_string(),
                            target_text: "warm summer water calms the weary mind; wisdom comes when we dream of a modern world;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 7 Review".to_string(),
                            prompt: "Achieve consistent speed with W and M.".to_string(),
                            target_text: "we walk into the modern world with warm dreams; wise minds make calm moves when storms loom;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 8: Keys V and B
                Lesson {
                    number: 8,
                    title: "Keys V and B".to_string(),
                    new_keys: vec!['v', 'b'],
                    finger_guidance: "Left index finger reaches down to V and down-right to B.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "V and B Warm-Up".to_string(),
                            prompt: "Train the dual-reach mobility of the left index finger.".to_string(),
                            target_text: "fvf fvf fbf fbf fvf fbf vb bv fv fb brave valve".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with V and B".to_string(),
                            prompt: "Practice words using V and B.".to_string(),
                            target_text: "brave valve vibrant beam balance novel orbit harbor velvet".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with V and B".to_string(),
                            prompt: "Execute agile index maneuvers without moving the wrist.".to_string(),
                            target_text: "brave travelers observe vibrant views from the boat; balance every move before diving deep;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 8 Review".to_string(),
                            prompt: "Synthesize V and B with earlier keys.".to_string(),
                            target_text: "vibrant books provide bold views; brave builders balance beams above the vast blue bay;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 9: Keys Q and P
                Lesson {
                    number: 9,
                    title: "Keys Q and P".to_string(),
                    new_keys: vec!['q', 'p'],
                    finger_guidance: "Left pinky reaches up-left to Q. Right pinky reaches up to P.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Q and P Warm-Up".to_string(),
                            prompt: "Strengthen pinky independence and extension.".to_string(),
                            target_text: "aqa aqa ;p; ;p; aqa ;p; qa ;p qu pl quick pride".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with Q and P".to_string(),
                            prompt: "Type words utilizing the perimeter pinky keys.".to_string(),
                            target_text: "quiet paper quest point quote planet spark pulse proud".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with Q and P".to_string(),
                            prompt: "Keep light touch on the outer keys.".to_string(),
                            target_text: "a quiet planet prepares for a quick quest; place proper points upon every paper prompt;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 9 Review".to_string(),
                            prompt: "Master pinky reach accuracy.".to_string(),
                            target_text: "quick paths require proper pace; proud people pursue quiet quests with pure purpose;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 10: Keys X and Z
                Lesson {
                    number: 10,
                    title: "Keys X and Z".to_string(),
                    new_keys: vec!['x', 'z'],
                    finger_guidance: "Left ring finger reaches down to X. Left pinky reaches down to Z.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "X and Z Warm-Up".to_string(),
                            prompt: "Train the delicate bottom-left reach of ring and pinky fingers.".to_string(),
                            target_text: "sxs sxs aza aza sxs aza sx az zx xz zero exact".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Words with X and Z".to_string(),
                            prompt: "Practice rare but essential letter combinations.".to_string(),
                            target_text: "exact zero pixel puzzle freeze apex luxury bronze matrix".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Sentences with X and Z".to_string(),
                            prompt: "Smoothly transition through bottom-left reaches.".to_string(),
                            target_text: "solve the complex puzzle with zero panic; freeze the matrix to inspect every exact pixel;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 10 Review".to_string(),
                            prompt: "Complete all 26 lowercase alphabet letters.".to_string(),
                            target_text: "the expert gazed upon a bronze puzzle; analyze zero defects with maximum zealous focus;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 11: Capital Letters & Shift
                Lesson {
                    number: 11,
                    title: "Capital Letters & Shift Keys".to_string(),
                    new_keys: vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
                    finger_guidance: "Use opposite pinky for Shift! Right Shift for left-hand letters, Left Shift for right-hand letters.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Shift Synchronization Warm-Up".to_string(),
                            prompt: "Press and hold the opposite shift before striking the key, then release both.".to_string(),
                            target_text: "Aa Bb Cc Dd Ee Ff Gg Hh Ii Jj Kk Ll Mm Nn Oo Pp Qq Rr Ss Tt Uu Vv Ww Xx Yy Zz".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Proper Nouns & Capitalized Words".to_string(),
                            prompt: "Type capitalized names, places, and terms.".to_string(),
                            target_text: "London Tokyo Paris Berlin Rome Austin Denver Sydney Oslo Madrid Dublin Zurich".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Capitalized Prose".to_string(),
                            prompt: "Practice standard capitalization at the beginning of sentences and titles.".to_string(),
                            target_text: "Grace Hopper built compilers. Alan Turing imagined universal machines. Ada Lovelace wrote the first program.".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Lesson 11 Review".to_string(),
                            prompt: "Test fluid alternating-shift mechanics.".to_string(),
                            target_text: "Great Engineers Design Deep Systems. Focus, Discipline, and Rigor Yield Unstoppable Velocity.".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                // Lesson 12: Alphabet Mastery Certification
                Lesson {
                    number: 12,
                    title: "Alphabet Touch Typing Mastery Certification".to_string(),
                    new_keys: vec![' '],
                    finger_guidance: "Bring all skills together. Sit tall, breathe steadily, and maintain effortless cadence.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Pangram Cadence Warm-Up".to_string(),
                            prompt: "Type comprehensive pangrams hitting every letter of the alphabet.".to_string(),
                            target_text: "the quick brown fox jumps over the lazy dog. pack my box with five dozen liquor jugs.".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Polysyllabic Word Flow".to_string(),
                            prompt: "Type longer words without pausing mid-word.".to_string(),
                            target_text: "architecture engineering craftsmanship philosophy telemetry infrastructure resilience".to_string(),
                            minimum_accuracy: 95.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "The Craft of Touch Typing".to_string(),
                            prompt: "Type rhythmic paragraphs reflecting modern systems craft.".to_string(),
                            target_text: "Touch typing transforms conscious keystrokes into fluid subconscious expression. When your hands move freely across the keys without hesitation, your thoughts flow directly into software architecture with zero cognitive drag.".to_string(),
                            minimum_accuracy: 95.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Final Touch-Typing Certification".to_string(),
                            prompt: "Pass this comprehensive test with 95%+ accuracy to graduate from Touch Typing Basics.".to_string(),
                            target_text: "Mastery is not the absence of errors, but the quiet rhythm of continuous precision. Build clean modules, embrace strict disciplines, and let your keyboard become a transparent extension of your intellect.".to_string(),
                            minimum_accuracy: 95.0,
                        },
                    ],
                },
            ],
        }
    }

    pub fn developer_special() -> Self {
        Self {
            id: "developer".to_string(),
            title: "Course 2: Developer Syntax & Code Symbols".to_string(),
            description: "Targeted drills for brackets, curly braces, arrows, and punctuation used in real programming."
                .to_string(),
            lessons: vec![
                Lesson {
                    number: 1,
                    title: "Brackets and Parens: () [] {}".to_string(),
                    new_keys: vec!['(', ')', '[', ']', '{', '}'],
                    finger_guidance: "Right pinky reaches top row for (), and bracket keys [ ] { }.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Bracket Warm-Up".to_string(),
                            prompt: "Type matching delimiter pairs.".to_string(),
                            target_text: "() [] {} (()) [[]] {{}} ([{}]) {([()])} () [] {}".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Code Expression Drill".to_string(),
                            prompt: "Type common programming function calls and index expressions.".to_string(),
                            target_text: "fn main() -> Result<(), Box<dyn Error>> { let items = vec![1, 2, 3]; }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Rust Idioms".to_string(),
                            prompt: "Type real idiomatic Rust statements.".to_string(),
                            target_text: "pub fn calculate(&self, count: usize) -> Option<Duration> { match self.timer { Some(t) => Some(t.elapsed()), None => None } }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Syntax Certification".to_string(),
                            prompt: "Attain high accuracy on symbol-dense code.".to_string(),
                            target_text: "impl<T: Clone + Send> DataPipe<T> { pub async fn forward(&mut self, payload: T) -> Result<usize, ChannelError> { Ok(1) } }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 2,
                    title: "Arrows & Comparison: -> => <= >= == !=".to_string(),
                    new_keys: vec!['-', '>', '=', '<', '!', '+'],
                    finger_guidance: "Left pinky reaches ! and +, right pinky/ring reaches = < >.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Arrow & Operator Warm-Up".to_string(),
                            prompt: "Build muscle memory for compound operators and arrows.".to_string(),
                            target_text: "-> => == != <= >= += -= *= /= -> => == != <= >=".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Comparison & Assignment Drill".to_string(),
                            prompt: "Type comparison statements and assignment operators.".to_string(),
                            target_text: "x => x * 2; if a == b && c != d || e <= f { return Ok(1); }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Match Arms & Lambda Expressions".to_string(),
                            prompt: "Type pattern matching arms with fat arrows and guards.".to_string(),
                            target_text: "match signal { Ok(val) => val.process(), Err(e) if e.is_timeout() => retry(), Err(_) => abort() }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Control Flow Certification".to_string(),
                            prompt: "Demonstrate accuracy on compound logical conditions.".to_string(),
                            target_text: "let delta = (next_val - prev_val) >= threshold && max_limit != 0 || offset < baseline;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 3,
                    title: "Semicolons, Colons & Path Operators: ; : :: , .".to_string(),
                    new_keys: vec![';', ':', ',', '.'],
                    finger_guidance: "Right pinky strikes ; and Shift+; for :. Right middle and ring reach , and . on bottom row.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Scope & Path Warm-Up".to_string(),
                            prompt: "Type colons, semicolons, and path qualification symbols.".to_string(),
                            target_text: ":: : ; :: : ; std::sync::Arc; std::collections::HashMap; std::path::PathBuf;".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Module Imports & Declarations".to_string(),
                            prompt: "Type nested use declarations and imports.".to_string(),
                            target_text: "use std::sync::atomic::{AtomicBool, Ordering}; use std::time::{Duration, Instant};".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Struct Definition & Fields".to_string(),
                            prompt: "Type typed struct fields and visibility specifiers.".to_string(),
                            target_text: "pub struct Config { pub host: String, pub port: u16, pub timeout_ms: u64, pub enabled: bool }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Trait Bounds & Scopes Certification".to_string(),
                            prompt: "Type generic struct implementations with trait bounds.".to_string(),
                            target_text: "impl<K: std::hash::Hash + Eq, V: Clone> Cache<K, V> { pub fn new() -> Self { Self { map: HashMap::new() } } }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 4,
                    title: "Pipes, Ampersands & Bitwise Operators: | || & && ! ~ ^".to_string(),
                    new_keys: vec!['|', '&', '!', '~', '^'],
                    finger_guidance: "Right pinky reaches | with Shift+\\. Left pinky and index reach ! ~ ^ &.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Bitwise & Logic Warm-Up".to_string(),
                            prompt: "Type boolean and bitwise operator pairs.".to_string(),
                            target_text: "& && | || ! ~ ^ & && | || ! ~ ^ & && | || ! ~ ^".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Boolean Condition Drill".to_string(),
                            prompt: "Type compound conditional statements.".to_string(),
                            target_text: "if (is_valid && !is_empty) || (has_token && has_access) { proceed(); }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Bitmasking & Flags".to_string(),
                            prompt: "Type low-level bit shifting and masking expressions.".to_string(),
                            target_text: "let mask: u32 = (1 << 4) | (1 << 2) | (1 << 0); let flags = header.flags() & !mask;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Pattern Matching with Pipes".to_string(),
                            prompt: "Type multi-pattern match arms and error handling.".to_string(),
                            target_text: "match token { Token::Ident(id) | Token::Keyword(id) => emit(id), Token::Eof | Token::Whitespace => skip() }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 5,
                    title: "Quotes, Strings & Regex Tokens: \" ' ` \\ / $".to_string(),
                    new_keys: vec!['"', '\'', '`', '\\', '/', '$'],
                    finger_guidance: "Right pinky strikes ' and \", left pinky reaches ` and ~, right index reaches / and pinky \\.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Quote & Slash Warm-Up".to_string(),
                            prompt: "Type quotation marks, backslashes, and path delimiters.".to_string(),
                            target_text: "\"str\" 'c' `lit` \\n \\t \\r /path/to/file \"$var\" 'a' \"b\"".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "String Interpolation & Regex".to_string(),
                            prompt: "Type formatted string templates and regular expressions.".to_string(),
                            target_text: "let path = format!(\"{}/{}.json\", base_dir, filename); let re = Regex::new(r\"^\\d{4}-\\d{2}-\\d{2}$\");".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Shell Automation & Path Builders".to_string(),
                            prompt: "Type shell command strings and quoted path parameters.".to_string(),
                            target_text: "git commit -m \"feat(core): sanitize path inputs\" && printf \"%s\\n\" \"$output\" > build.log".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Escape Sequences Certification".to_string(),
                            prompt: "Type complex escaped logging statements and URLs.".to_string(),
                            target_text: "println!(\"Response [status={}]: \\\"{}\\\" from endpoint /api/v1/auth\", code, body);".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 6,
                    title: "Generics, Turbofish & Lifetimes: < > ::< > 'a &mut".to_string(),
                    new_keys: vec!['<', '>', ':', '\'', '&'],
                    finger_guidance: "Shift comma and dot for < >, right pinky for : and '.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Generic Syntax Warm-Up".to_string(),
                            prompt: "Type generic angle brackets and lifetime annotations.".to_string(),
                            target_text: "<T> <K, V> ::<T> ::<Vec<String>> &'a str &'a mut [u8] <T: Clone>".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Turbofish & Collection Transforms".to_string(),
                            prompt: "Type turbofish parsing and collection collectors.".to_string(),
                            target_text: "let nums = raw.split(',').map(str::trim).filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Generic Lifetime Bounds".to_string(),
                            prompt: "Type generic function signatures with explicit lifetime bounds.".to_string(),
                            target_text: "pub fn filter_active<'a, T: Identifiable + 'a>(items: &'a [T]) -> Vec<&'a T> { items.iter().filter(|i| i.is_active()).collect() }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Zero-Cost Generic Pipeline".to_string(),
                            prompt: "Type deserialization and typed container declarations.".to_string(),
                            target_text: "let parsed: Result<HashMap<String, Vec<u8>>, Error> = serde_json::from_str(&payload);".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 7,
                    title: "Async/Await & Functional Pipelines: ? .map .filter .await".to_string(),
                    new_keys: vec!['?', '.', '_', '-'],
                    finger_guidance: "Right pinky reaches ? with Shift+/, right ring for dot, left pinky for _.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Pipeline Operator Warm-Up".to_string(),
                            prompt: "Type method chaining calls and error try operators.".to_string(),
                            target_text: ".map() .filter() .unwrap() .unwrap_or_default() .ok()? .await? .as_ref()".to_string(),
                            minimum_accuracy: 92.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Async File Operations".to_string(),
                            prompt: "Type async I/O function calls with error propagation.".to_string(),
                            target_text: "let mut file = File::open(&path).await?; file.read_to_end(&mut buffer).await?; Ok(buffer)".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Functional Stream Aggregation".to_string(),
                            prompt: "Type functional filter and mapping pipeline expressions.".to_string(),
                            target_text: "let total = entries.into_iter().filter(|e| e.is_valid).map(|e| e.score * 2).sum::<u64>();".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Async Join & Concurrency".to_string(),
                            prompt: "Type concurrent future join operations with error unwraps.".to_string(),
                            target_text: "let (res1, res2) = tokio::join!(fetch_node(&addr1), fetch_node(&addr2)); let combined = res1? + res2?;".to_string(),
                            minimum_accuracy: 94.0,
                        },
                    ],
                },
                Lesson {
                    number: 8,
                    title: "Developer Certification: Idiomatic Systems Code".to_string(),
                    new_keys: vec!['*', '&', '[', ']', '{', '}', '!', '?'],
                    finger_guidance: "Comprehensive integration of all developer syntax, symbols, and operators.".to_string(),
                    exercises: vec![
                        Exercise {
                            exercise_type: ExerciseType::WarmUp,
                            title: "Systems Code Warm-Up".to_string(),
                            prompt: "Type systems network connection and result return.".to_string(),
                            target_text: "fn connect(addr: &str) -> io::Result<TcpStream> { TcpStream::connect(addr) }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::WordDrill,
                            title: "Macro & Unsafe Pointer Ops".to_string(),
                            prompt: "Type attribute macros and low-level memory operations.".to_string(),
                            target_text: "#[inline(always)] pub unsafe fn fast_copy<T: Copy>(src: *const T, dst: *mut T, count: usize) { std::ptr::copy_nonoverlapping(src, dst, count); }".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::SentenceDrill,
                            title: "Concurrent Channel Dispatch".to_string(),
                            prompt: "Type async task spawn and channel message processing loops.".to_string(),
                            target_text: "tokio::spawn(async move { while let Some(msg) = rx.recv().await { if let Err(e) = tx.send(msg).await { eprintln!(\"Err: {}\", e); break; } } });".to_string(),
                            minimum_accuracy: 94.0,
                        },
                        Exercise {
                            exercise_type: ExerciseType::LessonReview,
                            title: "Production Systems Certification".to_string(),
                            prompt: "Prove mastery of complex systems syntax under strict accuracy criteria.".to_string(),
                            target_text: "impl<T: Send + Sync + 'static> Drop for ThreadWorkerPool<T> { fn drop(&mut self) { for w in &mut self.workers { if let Some(t) = w.handle.take() { let _ = t.join(); } } } }".to_string(),
                            minimum_accuracy: 95.0,
                        },
                    ],
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_loading_and_lesson_count() {
        let course = Course::touch_typing_basics();
        assert_eq!(course.lessons.len(), 12);
        for lesson in &course.lessons {
            assert_eq!(lesson.exercises.len(), 4);
            for ex in &lesson.exercises {
                assert!(!ex.target_text.is_empty());
                assert!(ex.minimum_accuracy >= 90.0);
            }
        }
    }

    #[test]
    fn test_developer_course() {
        let dev_course = Course::developer_special();
        assert_eq!(dev_course.lessons.len(), 8);
        for lesson in &dev_course.lessons {
            assert_eq!(lesson.exercises.len(), 4);
            for ex in &lesson.exercises {
                assert!(!ex.target_text.is_empty());
                assert!(ex.minimum_accuracy >= 90.0);
            }
        }
    }

    #[test]
    fn test_exercise_key_drill_classification() {
        let warm_up = Exercise {
            exercise_type: ExerciseType::WarmUp,
            title: "Home Row Warm-Up".to_string(),
            prompt: "Prompt".to_string(),
            target_text: "asdf jkl;".to_string(),
            minimum_accuracy: 94.0,
        };
        assert!(warm_up.is_key_drill());

        let pangram_warm_up = Exercise {
            exercise_type: ExerciseType::WarmUp,
            title: "Pangram Cadence Warm-Up".to_string(),
            prompt: "Prompt".to_string(),
            target_text: "the quick brown fox jumps over the lazy dog.".to_string(),
            minimum_accuracy: 95.0,
        };
        assert!(!pangram_warm_up.is_key_drill());

        let sentence_drill = Exercise {
            exercise_type: ExerciseType::SentenceDrill,
            title: "Sentences with E and I".to_string(),
            prompt: "Prompt".to_string(),
            target_text: "she feels safe inside the field;".to_string(),
            minimum_accuracy: 94.0,
        };
        assert!(!sentence_drill.is_key_drill());

        let word_drill = Exercise {
            exercise_type: ExerciseType::WordDrill,
            title: "Home Row Words".to_string(),
            prompt: "Prompt".to_string(),
            target_text: "all fall salad ask flask".to_string(),
            minimum_accuracy: 94.0,
        };
        assert!(!word_drill.is_key_drill());
    }
}
