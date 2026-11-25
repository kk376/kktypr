use crate::engine::metrics::TypingMetrics;
use rand::RngExt;

/// Accuracy difficulty modes enforced across lesson chapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum DifficultyMode {
    Easy,
    #[default]
    Medium,
    Hard,
    Impossible,
}

impl DifficultyMode {
    pub const ALL: [Self; 4] = [Self::Easy, Self::Medium, Self::Hard, Self::Impossible];

    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            Self::Impossible => "Impossible",
        }
    }

    pub fn min_accuracy(self) -> f64 {
        match self {
            Self::Easy => 90.0,
            Self::Medium => 94.0,
            Self::Hard => 98.0,
            Self::Impossible => 100.0,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Easy => Self::Medium,
            Self::Medium => Self::Hard,
            Self::Hard => Self::Impossible,
            Self::Impossible => Self::Easy,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Easy => Self::Impossible,
            Self::Medium => Self::Easy,
            Self::Hard => Self::Medium,
            Self::Impossible => Self::Hard,
        }
    }

    pub fn passed(self, metrics: &TypingMetrics) -> bool {
        match self {
            Self::Impossible => metrics.error_count == 0 && metrics.accuracy_pct >= 100.0,
            _ => metrics.accuracy_pct >= self.min_accuracy(),
        }
    }
}

/// Represents the pedagogical stage of a progressive touch typing lesson.
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
            Self::WarmUp => "1. Warm Up Drill",
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
    #[serde(default)]
    pub variants: Vec<String>,
    pub minimum_accuracy: f64,
}

impl Exercise {
    pub fn new(
        exercise_type: ExerciseType,
        title: &str,
        prompt: &str,
        target_text: &str,
        minimum_accuracy: f64,
    ) -> Self {
        Self {
            exercise_type,
            title: title.to_string(),
            prompt: prompt.to_string(),
            target_text: target_text.to_string(),
            variants: Vec::new(),
            minimum_accuracy,
        }
    }

    /// Selects a practice string from `target_text` and any extra `variants`,
    /// avoiding an immediate repeat of `previous` when multiple options exist.
    pub fn pick_text(&self, previous: Option<&str>) -> &str {
        if self.variants.is_empty() {
            return &self.target_text;
        }
        let mut rng = rand::rng();
        if let Some(prev) = previous {
            let candidates: Vec<&str> = std::iter::once(self.target_text.as_str())
                .chain(self.variants.iter().map(String::as_str))
                .filter(|&s| s != prev)
                .collect();
            if !candidates.is_empty() {
                return candidates[rng.random_range(0..candidates.len())];
            }
        }
        let idx = rng.random_range(0..(1 + self.variants.len()));
        if idx == 0 {
            &self.target_text
        } else {
            &self.variants[idx - 1]
        }
    }

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

pub(super) fn single_drill(
    exercise_type: ExerciseType,
    title: &str,
    prompt: &str,
    target_text: &str,
    minimum_accuracy: f64,
) -> Exercise {
    Exercise::new(exercise_type, title, prompt, target_text, minimum_accuracy)
}

pub(super) fn multi_drill(
    exercise_type: ExerciseType,
    title: &str,
    prompt: &str,
    minimum_accuracy: f64,
    texts: &[&str],
) -> Exercise {
    let primary = texts.first().copied().unwrap_or_default();
    let variants = texts.iter().skip(1).map(|s| (*s).to_string()).collect();
    Exercise {
        exercise_type,
        title: title.to_string(),
        prompt: prompt.to_string(),
        target_text: primary.to_string(),
        variants,
        minimum_accuracy,
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
                        single_drill(
                            ExerciseType::WarmUp,
                            "Home Row Warm Up",
                            "Keep your fingers resting gently on ASDF and JKL;. Strike each key without looking down.",
                            "asdf jkl; asdf jkl; fads jkal fads jkal aass ddff jjkk ll;;",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Home Row Words",
                            "Practice typing simple combinations formed with the home row keys.",
                            94.0,
                            &[
                                "all fall salad ask flask glad sad dad lad falls lads asks",
                                "ask dad; a sad lad falls; all salads fall; dad adds alfalfa;",
                                "a lass asks dad; dad asks all lads; alas a flask falls;",
                                "add salsa; ask all dads; a lad falls as a flask falls;",
                                "alas a dad falls; all lads ask a lass; alfalfa salads;",
                                "salsa falls; a flask asks dad; dad adds all salads; alas;",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Home Row Sentences",
                            "Type complete home row phrases at a steady, rhythmic pace.",
                            94.0,
                            &[
                                "a sad lad asks all dads; a glad lad falls; a flask asks a salad;",
                                "a lad falls; dad asks all lads; a lass adds salsa as a salad falls;",
                                "alas dad falls; all dads ask a sad lad; add alfalfa as a flask falls;",
                                "dad asks a lass; a sad dad adds a salad; all flasks fall as lads ask;",
                                "a lass asks all dads; a fad falls; dad adds salsa; a lad asks as a flask falls;",
                                "all salads fall; alas a sad lass asks dad; add a flask; dad falls;",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Home Row Mastery Test",
                            "Complete this review meeting your accuracy target to complete Lesson 1.",
                            94.0,
                            &[
                                "all lads ask dad; glad dads add salad; a sad lad falls as dad asks all lads;",
                                "all dads ask a sad lad; alas a lass adds alfalfa salsa; all flasks fall;",
                                "dad adds a salad; a lass asks all lads as a flask falls; dad falls alas;",
                                "sad lads ask dad; all dads add salsa salads; a fad falls as a lass asks;",
                                "ask all lads; add a flask; dad falls as a salad falls; alas all dads ask;",
                                "a lass adds alfalfa salad; all sad dads ask lads; a flask falls as all fall;",
                            ],
                        ),
                    ],
                },
                // Lesson 2: Keys E and I
                Lesson {
                    number: 2,
                    title: "Keys E and I".to_string(),
                    new_keys: vec!['e', 'i'],
                    finger_guidance: "Left middle finger reaches up for E. Right middle finger reaches up for I. Return immediately to home row.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "E and I Warm Up",
                            "Practice the upward reach of both middle fingers.",
                            "ded ded kik kik ded kik ded kik de ki ed ik dede kiki",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with E and I",
                            "Type real words incorporating E and I.",
                            94.0,
                            &[
                                "feed file desk side safe leaf like self field life leaf slide",
                                "aisle aside deal deed deli desk dial disk ease fade fail fake",
                                "jade jail kid kill kiss laid lake lead leaf leak lied life",
                                "safe said sail seal seed seek sell side silk skill sled slide",
                                "fields files lakes slides ideas disks seals deeds leaves lids",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with E and I",
                            "Maintain cadence across natural phrasing.",
                            94.0,
                            &[
                                "she feels safe inside the field; side files hide a leak; see if life feels like a fresh leaf;",
                                "a safe desk feels idle; i see a jade seal beside a silk sail; dad filed all dead ideas;",
                                "kids slid beside a field; i said a fake deed failed; see a lake feed life as ice fades;",
                                "a skilled kid sells jade dials; i liked a safe aisle; a leaf slid beside a sleek desk;",
                                "dead leaves filled a side aisle; dad feeds six seals; i seek idle fields;",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 2 Review",
                            "Aim for clean rhythm over raw speed.",
                            94.0,
                            &[
                                "a wise idea hides inside; like a leaf in a field, she sailed safe beside the lake;",
                                "a skilled kid laid a silk sail beside a jade lake; i filed safe leads as deals faded;",
                                "dad sees a sleek sled slide beside a field; i feel safe if dead ideas fade like leaves;",
                                "fill a side desk; seal all safe files; skilled kids seek ideal fields;",
                                "aside a quiet lake i laid a faded file; dad sells jade seals as sales slide;",
                            ],
                        ),
                    ],
                },
                // Lesson 3: Keys R and U
                Lesson {
                    number: 3,
                    title: "Keys R and U".to_string(),
                    new_keys: vec!['r', 'u'],
                    finger_guidance: "Left index finger reaches up to R. Right index finger reaches up to U. Always snap back to F and J.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "R and U Warm Up",
                            "Reach with your index fingers without drifting your whole hand.",
                            "frf frf juj juj frf juj fr ju rf uj fur ruf jur fujr",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with R and U",
                            "Form words using R, U, and previously learned letters.",
                            94.0,
                            &[
                                "rule rust user sure cure fire rude pure rider fruit flare",
                                "read real fear fair rail raise lurk fluid dusk dust rudder",
                                "radius reuse relief issuer druid dual lure ruler jars fur",
                                "rural surfer area air ear rules fuels users cures riders",
                                "fears fuels fairs flares fluids ruins rulers fires lids",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with R and U",
                            "Keep wrist elevated and light.",
                            94.0,
                            &[
                                "the true rule is fair and sure; pure fire fuels the rustic user ride;",
                                "a fair ruler reads real rules; rude rust ruined a dull rudder beside us;",
                                "users find relief as pure fluid fuels a safe fire; a surfer rides a fair rail;",
                                "dark dusk fell as a rude druid lurked beside a rural lake; i feel sure relief;",
                                "real riders raise fair dues; a pure aura fuels dull airs as dusk arrives;",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 3 Review",
                            "Combine lessons 1, 2, and 3 into unified muscle memory.",
                            94.0,
                            &[
                                "rare fruits are useful in dire fuels; true riders rule our fair rustic fields;",
                                "a fair ruler refused rude rules; pure users share useful ideas under a clear dusk;",
                                "rusted rails laid beside rural fields; a true surfer rides real swells as fires flare;",
                                "read fair rules daily; useful readers raise sure ideas under safe leaders;",
                                "dual sliders ease real risk; a rural field feeds deer as pure air feels full;",
                            ],
                        ),
                    ],
                },
                // Lesson 4: Period and Comma
                Lesson {
                    number: 4,
                    title: "Punctuation: Period (.) and Comma (,)".to_string(),
                    new_keys: vec!['.', ','],
                    finger_guidance: "Right ring finger reaches down for period. Right middle finger reaches down for comma.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Punctuation Warm Up",
                            "Drop down smoothly from K to comma and L to period.",
                            "k,k k,k l.l l.l k,k l.l ,,, ... k, l. k, l. ,. ,.",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Punctuation Rhythm Drill",
                            "Type words followed by punctuation and standard space.",
                            94.0,
                            &[
                                "red, blue, safe. fire, dust, dark. sure, fair, real. fuel, seed, leaf.",
                                "silk, jade, lake. ruler, rider, user. file, disk, desk. rule, risk, deal.",
                                "dusk, air, fire. rail, sail, field. idea, lead, deed. jar, lid, seal.",
                                "real, pure, clear. dull, loud, full. fast, free, safe. fair, rare, sure.",
                                "read, file, sell. ask, add, feed. feel, fear, find. fall, rise, slide.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Punctuation Sentences",
                            "Build natural pause rhythm after periods and commas.",
                            94.0,
                            &[
                                "if fire rises, find fuel. as life is fair, real users rule, and rustic roads guide us.",
                                "if dusk falls, seal a side file. dad reads, kids laugh, and a safe fire fades.",
                                "real leaders read, listen, and decide. idle fears fade, as fair ideas rise.",
                                "slide a disk, file a lead, and seal a deal. fair readers find sure relief.",
                                "silk, jade, and slate feel sleek. as dusk arrives, rural fields feel quiet, safe, and still.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 4 Review",
                            "Test fluent punctuation integration.",
                            94.0,
                            &[
                                "sure, a fair user shares a seed, a leaf, and a pure fuel. see, life rises safe.",
                                "ask, listen, and read. a skilled user feels sure, even if real rules look austere.",
                                "beside a clear lake, a rider rests, reads, and fuels a small fire. all feels safe.",
                                "file all leads, seal each jar, and add a fair rule. steady riders ease risk.",
                                "ideas rise, fade, and return. read well, feel sure, and slide past dull fears.",
                            ],
                        ),
                    ],
                },
                // Lesson 5: Keys C and N
                Lesson {
                    number: 5,
                    title: "Keys C and N".to_string(),
                    new_keys: vec!['c', 'n'],
                    finger_guidance: "Left middle finger reaches down to C. Right index finger reaches down left to N.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "C and N Warm Up",
                            "Practice downward reaches while keeping home anchor fingers in place.",
                            "dcd dcd jnj jnj dcd jnj dc jn cd nj dcdc jnjn",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with C and N",
                            "Type common core words containing C and N.",
                            94.0,
                            &[
                                "clean scan clan face nice coin fence iron chain dance cone",
                                "cabin canal cancel candy crane fence glance lance lunch niece",
                                "ocean ounce panic pencil prince science screen silence since scene",
                                "accent anchor ancient balance bounce cancel central chance clinical",
                                "concern concise connect corner council crunch dancer decline design",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with C and N",
                            "Type fluent English sentences containing C and N.",
                            94.0,
                            &[
                                "clean code can run nice; find an iron coin inside a clean fence; scan each corner carefully;",
                                "since noon, nice cranes circled near an ancient stone canal in quiet silence.",
                                "a concise science note can enhance clear confidence and calm keen concerns.",
                                "clean linen and fresh cinnamon scented a sunny corner near a calm ocean cliff.",
                                "ninety keen dancers balanced nicely on an inclined cedar scene under neon lights.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 5 Review",
                            "Master smooth bottom row reach discipline.",
                            94.0,
                            &[
                                "a clean mind can craft an iron fence; coins and chains dance in fine clean sand;",
                                "keen mechanics scan every corner, align clean connectors, and silence minor concerns.",
                                "calm confidence enters in silence; connect each clean cadence and never rush a turn.",
                                "across scenic canyons, cool clouds announce rain while crisp winds dance around pine cones.",
                                "clear science connects evidence, reason, and honest inquiry into concise human knowledge.",
                            ],
                        ),
                    ],
                },
                // Lesson 6: Keys T and O
                Lesson {
                    number: 6,
                    title: "Keys T and O".to_string(),
                    new_keys: vec!['t', 'o'],
                    finger_guidance: "Left index finger reaches up right to T. Right ring finger reaches up to O.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "T and O Warm Up",
                            "Alternate between index stretch (T) and ring stretch (O).",
                            "ftf ftf lol lol ftf lol to ot toto lolo toot root",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with T and O",
                            "Expand vocabulary with highly frequent T and O combinations.",
                            94.0,
                            &[
                                "root door fort look stone note cold forest train torch toast",
                                "actor doctor editor factor filter folder format foster option",
                                "potato protocol rotor short sort story stout strict target",
                                "throat ticket toaster toilet tomato tonight topic total tractor",
                                "control contour cotton custom effort fortune motion notion smooth",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with T and O",
                            "Focus on relaxed shoulders and even rhythm.",
                            94.0,
                            &[
                                "take note of the cold stone door; train often to foster cool, calm, and solid focus;",
                                "onto the old stone fort stood two stout torches lit to comfort lost travelers.",
                                "total control of motion comes not from brute force, but from soft, steady tone.",
                                "out on the autumn road, octaves of frost coated tall oaks near our cottage.",
                                "soon those short notes turned into solid stories told to attentive students.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 6 Review",
                            "Review all letters learned across lessons 1 through 6.",
                            94.0,
                            &[
                                "to look into the future, take clean action; forest roots drink rain to stand tall and cool;",
                                "cool tones echo across smooth corridors as thoughtful editors polish every short footnote.",
                                "sort out cluttered thoughts, focus on true priorities, and set realistic daily targets.",
                                "two scouts crossed the wooden footbridge toward the distant lookout station before noon.",
                                "consistent routines transform awkward reaches into automatic motions that last a lifetime.",
                            ],
                        ),
                    ],
                },
                // Lesson 7: Keys W and M
                Lesson {
                    number: 7,
                    title: "Keys W and M".to_string(),
                    new_keys: vec!['w', 'm'],
                    finger_guidance: "Left ring finger reaches up to W. Right index finger reaches down to M.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "W and M Warm Up",
                            "Coordinate left ring reach with right index descent.",
                            "sws sws jmj jmj sws jmj sw jm ws mj swim warm",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with W and M",
                            "Type words featuring W and M.",
                            94.0,
                            &[
                                "warm water storm dream flame modern world wisdom meadow swarm",
                                "amber camera frame grammar hammer mammal memory mirror moment",
                                "meadow mower narrow network owner shadow shower swim swimmer",
                                "welcome western wheat wheel whisper widow willow window winter",
                                "windmill summit drummer shimmer wonder warmth swimmer welcome",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with W and M",
                            "Type continuous expressive sentences.",
                            94.0,
                            &[
                                "warm summer water calms the weary mind; wisdom comes when we dream of a modern world;",
                                "we watched warm mist drift west over wildflower meadows while morning birds murmured.",
                                "many swimmers welcome wide waves, warm wind, and smooth rhythms along calm waters.",
                                "tomorrow morning we must mend the wooden window frame where winter storms blew.",
                                "wise mentors model calm momentum even when looming deadlines seem overwhelming.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 7 Review",
                            "Achieve consistent speed with W and M.",
                            94.0,
                            &[
                                "we walk into the modern world with warm dreams; wise minds make calm moves when storms loom;",
                                "when winter winds sweep wide valleys, warm timber rooms welcome weary wanderers home.",
                                "steady rhythm turns awkward motions into effortless momentum across every work week.",
                                "memory grows stronger when we write meaningful words with calm, mindful awareness.",
                                "swimmers move smoothly through mountain waters while summer clouds drift calmly westward.",
                            ],
                        ),
                    ],
                },
                // Lesson 8: Keys V and B
                Lesson {
                    number: 8,
                    title: "Keys V and B".to_string(),
                    new_keys: vec!['v', 'b'],
                    finger_guidance: "Left index finger reaches down to V and down right to B.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "V and B Warm Up",
                            "Train the dual reach mobility of the left index finger.",
                            "fvf fvf fbf fbf fvf fbf vb bv fv fb brave valve",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with V and B",
                            "Practice words using V and B.",
                            94.0,
                            &[
                                "brave valve vibrant beam balance novel orbit harbor velvet",
                                "above absorb advise arrive banish banner barrel basket beaver",
                                "behave believe beverage binary blanket blossom boulder branch",
                                "bubble bucket buffer bundle cable canvas cavern clever cobalt",
                                "combine comfort verbose verify vertical vessel vibration vibrant",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with V and B",
                            "Execute agile index maneuvers without moving the wrist.",
                            94.0,
                            &[
                                "brave travelers observe vibrant views from the boat; balance every move before diving deep;",
                                "vibrant birds hovered above timber cabins beside the broad river valley below.",
                                "a clever builder verified every brace, bolt, and beam before raising the heavy bridge.",
                                "velvet banners wavered briefly above busy harbor vessels bounded for visible coves.",
                                "keep brief notes visible beside your workbench so subtle bugs become obvious quickly.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 8 Review",
                            "Synthesize V and B with earlier keys.",
                            94.0,
                            &[
                                "vibrant books provide bold views; brave builders balance beams above the vast blue bay;",
                                "broad harbor breakwaters absorb heavy waves, giving small sailboats safe shelter nearby.",
                                "verbs bring vivid movement to prose while balanced habits prevent avoidable burnouts.",
                                "bold innovators observe subtle behaviors before breaking reliable baseline systems.",
                                "beneath silver birch branches, vivid berries ripened evenly above soft mossy banks.",
                            ],
                        ),
                    ],
                },
                // Lesson 9: Keys Q and P
                Lesson {
                    number: 9,
                    title: "Keys Q and P".to_string(),
                    new_keys: vec!['q', 'p'],
                    finger_guidance: "Left pinky reaches up left to Q. Right pinky reaches up to P.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Q and P Warm Up",
                            "Strengthen pinky independence and extension.",
                            "aqa aqa ;p; ;p; aqa ;p; qa ;p qu pl quick pride",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with Q and P",
                            "Type words utilizing the perimeter pinky keys.",
                            94.0,
                            &[
                                "quiet paper quest point quote planet spark pulse proud",
                                "aqua equal equip opaque plaque quarry quart quarter quartz",
                                "queen quench query queue quick quiet quill quilt quiver quota",
                                "package paddle palace parcel parent parser pattern pencil pepper",
                                "permit pillar pixel planet pocket polish popular proper purpose",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with Q and P",
                            "Keep light touch on the outer keys.",
                            94.0,
                            &[
                                "a quiet planet prepares for a quick quest; place proper points upon every paper prompt;",
                                "practical pupils quickly grasp crisp punctuation principles through patient practice.",
                                "pale quartz sparkled softly beside quiet pools in the upper pine quarry.",
                                "prompt replies help teammates pinpoint opaque problems and ship crisp patches.",
                                "a proud poet penned four compact quatrains upon crisp parchment paper.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 9 Review",
                            "Master pinky reach accuracy.",
                            94.0,
                            &[
                                "quick paths require proper pace; proud people pursue quiet quests with pure purpose;",
                                "quality proofs require patient preparation, precise punctuation, and plain prose.",
                                "packed cargo ships slipped quietly past pier pilings under pale twilight skies.",
                                "equip your practice routine with quiet focus rather than rapid sloppy key presses.",
                                "powerful principles appear simple once perplexing parts are peeled apart properly.",
                            ],
                        ),
                    ],
                },
                // Lesson 10: Keys X and Z
                Lesson {
                    number: 10,
                    title: "Keys X and Z".to_string(),
                    new_keys: vec!['x', 'z'],
                    finger_guidance: "Left ring finger reaches down to X. Left pinky reaches down to Z.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "X and Z Warm Up",
                            "Train the delicate bottom left reach of ring and pinky fingers.",
                            "sxs sxs aza aza sxs aza sx az zx xz zero exact",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Words with X and Z",
                            "Practice rare but essential letter combinations.",
                            94.0,
                            &[
                                "exact zero pixel puzzle freeze apex luxury bronze matrix",
                                "boxcar complex duplex exam excel excess exhaust exist exotic",
                                "expand expert export extend extra galaxy oxygen syntax vertex",
                                "azalea blizzard breeze citizen crazy dizzy dozen drizzle fizz",
                                "hazard jazz lizard nozzle ozone plaza quartz razor seize zone",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Sentences with X and Z",
                            "Smoothly transition through bottom left reaches.",
                            94.0,
                            &[
                                "solve the complex puzzle with zero panic; freeze the matrix to inspect every exact pixel;",
                                "six zealous experts examined exotic quartz crystals frozen beneath an alpine zone.",
                                "dizzying jazz saxophones echoed across the bronze plaza while cool breezes swept past.",
                                "clean syntax relaxes complexity and helps future authors fix exact edge cases fast.",
                                "a nimble fox zigzagged across glazed tundra zones toward an exposed rocky apex.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 10 Review",
                            "Complete all 26 lowercase alphabet letters.",
                            94.0,
                            &[
                                "the expert gazed upon a bronze puzzle; analyze zero defects with maximum zealous focus;",
                                "complex systems demand exact diagnostics so hidden hazards cannot freeze critical zones.",
                                "six dozen wizards mixed bizarre elixirs inside oxidized bronze vessels until midnight.",
                                "examine every vertex carefully, extract excess noise, and maximize clarity at zero cost.",
                                "cool winter breezes glazed the quiet plaza as exact chimes marked zero hour.",
                            ],
                        ),
                    ],
                },
                // Lesson 11: Capital Letters & Shift
                Lesson {
                    number: 11,
                    title: "Capital Letters & Shift Keys".to_string(),
                    new_keys: vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
                    finger_guidance: "Use opposite pinky for Shift! Right Shift for left hand letters, Left Shift for right hand letters.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Shift Synchronization Warm Up",
                            "Press and hold the opposite shift before striking the key, then release both.",
                            "Aa Bb Cc Dd Ee Ff Gg Hh Ii Jj Kk Ll Mm Nn Oo Pp Qq Rr Ss Tt Uu Vv Ww Xx Yy Zz",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Proper Nouns & Capitalized Words",
                            "Type capitalized names, places, and terms.",
                            94.0,
                            &[
                                "London Tokyo Paris Berlin Rome Austin Denver Sydney Oslo Madrid Dublin Zurich",
                                "Linux Fedora Debian Alpine Ubuntu Cargo Rust Python Go TypeScript Java SQLite",
                                "Atlantic Pacific Indian Arctic Nile Amazon Danube Rhine Alps Andes Himalayas",
                                "Mercury Venus Earth Mars Jupiter Saturn Uranus Neptune Apollo Voyager Hubble",
                                "Monday Tuesday Wednesday Thursday Friday Saturday Sunday January October December",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Capitalized Prose",
                            "Practice standard capitalization at the beginning of sentences and titles.",
                            94.0,
                            &[
                                "Grace Hopper built compilers. Alan Turing imagined universal machines. Ada Lovelace wrote the first program.",
                                "Margaret Hamilton led flight software for Apollo. Ken Thompson and Dennis Ritchie shaped Unix.",
                                "Barbara Liskov defined behavioral subtyping. Donald Knuth authored The Art of Computer Programming.",
                                "Trains departed Kyoto for Osaka before sunrise, while ferries crossed Tokyo Bay under clear skies.",
                                "Fedora Workstation ships GNOME cleanly on Wayland with PipeWire handling audio routing across devices.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Lesson 11 Review",
                            "Test fluid alternating shift mechanics.",
                            94.0,
                            &[
                                "Great Engineers Design Deep Systems. Focus, Discipline, and Rigor Yield Unstoppable Velocity.",
                                "Every Tuesday morning, Elena reviews pull requests from Berlin, Toronto, Seattle, and Bangalore.",
                                "The Atlantic Express crosses five borders before reaching Zurich Central Station by sunset.",
                                "Calm Shift coordination stops awkward capital-letter stumbles in Names, Acronyms, and Headers.",
                                "Remember that HTTP, JSON, TLS, and SQL conventions stay readable when CamelCase and Title Case flow naturally.",
                            ],
                        ),
                    ],
                },
                // Lesson 12: Alphabet Mastery Certification (ALL 4 chapters expanded!)
                Lesson {
                    number: 12,
                    title: "Alphabet Touch Typing Mastery Certification".to_string(),
                    new_keys: vec![' '],
                    finger_guidance: "Bring all skills together. Sit tall, breathe steadily, and maintain effortless cadence.".to_string(),
                    exercises: vec![
                        multi_drill(
                            ExerciseType::WarmUp,
                            "Pangram Cadence Warm Up",
                            "Type comprehensive pangrams hitting every letter of the alphabet.",
                            94.0,
                            &[
                                "the quick brown fox jumps over the lazy dog. pack my box with five dozen liquor jugs.",
                                "sphinx of black quartz, judge my vow. how vexingly quick daft zebras jump across wet moss.",
                                "waltz, bad nymph, for quick jigs vex. bright vixens jump; dozy fowl quack near tall oaks.",
                                "jived fox nymph grabs quick waltz. cozy lummox gives smart squid who asks for job pen.",
                                "glistening waves quiz bright fjord nymphs before dusk. five quacking zephyrs jolt my wax bed.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Polysyllabic Word Flow",
                            "Type longer words without pausing mid word.",
                            95.0,
                            &[
                                "architecture engineering craftsmanship philosophy telemetry infrastructure resilience",
                                "synchronization maintainability deterministic observability throughput ergonomics",
                                "configuration authentication verification instrumentation modularity parallelism",
                                "compatibility accessibility troubleshooting benchmarking serialization optimization",
                                "collaboration comprehension specification visualization persistence calibration",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "The Craft of Touch Typing",
                            "Type rhythmic paragraphs reflecting modern systems craft.",
                            95.0,
                            &[
                                "Touch typing transforms conscious keystrokes into fluid subconscious expression. When your hands move freely across the keys without hesitation, your thoughts flow directly into software architecture with zero cognitive drag.",
                                "Fast typing is never about hammering keys frantically; true velocity emerges when every finger travels the shortest path back to the home row with relaxed posture and steady timing.",
                                "Clear thinking deserves an effortless medium. Once finger placement stops stealing mental bandwidth, you can hold entire module boundaries and invariants in mind while composing clean prose or code.",
                                "Great craftspeople sharpen their fundamentals long after novices move on. Checking posture, easing tension in the shoulders, and valuing precision over haste keeps fatigue away during long sessions.",
                                "Notice how difficult letter combinations become effortless once rhythm connects them into whole word gestures rather than twenty isolated single-character hunts.",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Final Touch Typing Certification",
                            "Pass this comprehensive test under your selected accuracy mode to graduate from Touch Typing Basics.",
                            95.0,
                            &[
                                "Mastery is not the absence of errors, but the quiet rhythm of continuous precision. Build clean modules, embrace strict disciplines, and let your keyboard become a transparent extension of your intellect.",
                                "Complex software survives decades only when curious engineers refuse shortcuts, write crisp tests, keep interfaces honest, and document every subtle boundary clearly for whoever maintains it next.",
                                "Every marathon of learning is won through daily patience. Keep accuracy sacred first, breathe evenly through tricky passages, and watch speed climb naturally behind unhurried control.",
                                "Across sunlit mountain passes and busy harbor terminals alike, clear communication connects distant teams, turns rough sketches into durable tools, and rewards deliberate craft.",
                                "Zero wasted motion, zero guesswork, and steady focus turn mechanical keystrokes into fluid creative thought from the very first morning warm-up to your final keystroke.",
                            ],
                        ),
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
        let warm_up = Exercise::new(
            ExerciseType::WarmUp,
            "Home Row Warm Up",
            "Prompt",
            "asdf jkl;",
            94.0,
        );
        assert!(warm_up.is_key_drill());

        let pangram_warm_up = Exercise::new(
            ExerciseType::WarmUp,
            "Pangram Cadence Warm Up",
            "Prompt",
            "the quick brown fox jumps over the lazy dog.",
            95.0,
        );
        assert!(!pangram_warm_up.is_key_drill());

        let sentence_drill = Exercise::new(
            ExerciseType::SentenceDrill,
            "Sentences with E and I",
            "Prompt",
            "she feels safe inside the field;",
            94.0,
        );
        assert!(!sentence_drill.is_key_drill());

        let word_drill = Exercise::new(
            ExerciseType::WordDrill,
            "Home Row Words",
            "Prompt",
            "all fall salad ask flask",
            94.0,
        );
        assert!(!word_drill.is_key_drill());
    }

    #[test]
    fn test_difficulty_modes_criteria_and_pass_evaluation() {
        assert_eq!(DifficultyMode::ALL.len(), 4);
        assert_eq!(DifficultyMode::Easy.min_accuracy(), 90.0);
        assert_eq!(DifficultyMode::Medium.min_accuracy(), 94.0);
        assert_eq!(DifficultyMode::Hard.min_accuracy(), 98.0);
        assert_eq!(DifficultyMode::Impossible.min_accuracy(), 100.0);

        let acc_93 = crate::engine::metrics::TypingMetrics {
            accuracy_pct: 93.0,
            ..Default::default()
        };
        assert!(DifficultyMode::Easy.passed(&acc_93));
        assert!(!DifficultyMode::Medium.passed(&acc_93));
        assert!(!DifficultyMode::Hard.passed(&acc_93));
        assert!(!DifficultyMode::Impossible.passed(&acc_93));

        let acc_99_one_err = crate::engine::metrics::TypingMetrics {
            accuracy_pct: 99.0,
            error_count: 1,
            ..Default::default()
        };
        assert!(DifficultyMode::Hard.passed(&acc_99_one_err));
        assert!(!DifficultyMode::Impossible.passed(&acc_99_one_err));
    }

    #[test]
    fn test_extra_practice_variants_exclude_single_key_drills_only() {
        let basics = Course::touch_typing_basics();
        for lesson in &basics.lessons {
            for (idx, ex) in lesson.exercises.iter().enumerate() {
                if lesson.number <= 11 && idx == 0 {
                    assert!(ex.is_key_drill());
                    assert!(
                        ex.variants.is_empty(),
                        "Lesson {} chapter 1 is a single-key drill and must not have extra random strings",
                        lesson.number
                    );
                } else {
                    assert!(
                        ex.variants.len() >= 4,
                        "Course 1 Lesson {} Chapter {} should provide extra practice string variants",
                        lesson.number,
                        idx + 1
                    );
                    if lesson.number == 1 {
                        for variant in &ex.variants {
                            assert!(
                                variant.chars().all(|c| "asdfjkl; ".contains(c)),
                                "Lesson 1 variant introduced unlearned key: {}",
                                variant
                            );
                        }
                    }
                }
            }
        }

        let dev = Course::developer_special();
        for lesson in &dev.lessons {
            for (idx, ex) in lesson.exercises.iter().enumerate() {
                if lesson.number <= 7 && idx == 0 {
                    assert!(ex.is_key_drill());
                    assert!(
                        ex.variants.is_empty(),
                        "Developer Lesson {} chapter 1 is an isolated symbol drill",
                        lesson.number
                    );
                } else {
                    assert!(
                        ex.variants.len() >= 4,
                        "Course 2 Lesson {} Chapter {} should provide extra developer practice strings",
                        lesson.number,
                        idx + 1
                    );
                }
            }
        }
    }
}
