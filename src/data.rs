use chrono::NaiveDate;

pub struct Me {
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub _location: &'static str,
    pub _birth_date: NaiveDate,
    pub contact_email: &'static str,
    pub links: Vec<PersonLink>,
    pub scholary_experiences: Vec<ScholaryExperience>,
    pub work_experiences: Vec<WorkExperience>,
    pub languages: Vec<Language>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
    pub side_projects: Vec<SideProject>,
    pub open_source_contributions: Vec<OpenSourceContribution>,
}

impl Default for Me {
    fn default() -> Self {
        let mut me = Me {
            first_name: "Midas",
            last_name: "Lambrichts",
            _location: "Turnhout",
            _birth_date: NaiveDate::from_ymd(1996, 02, 06),
            contact_email: "midaslamb@gmail.com",
            links: PersonLink::get_all(),
            scholary_experiences: ScholaryExperience::get_all(),
            work_experiences: WorkExperience::get_all(),
            languages: Language::get_all(),
            skills: Skill::get_all(),
            projects: Project::get_all(),
            side_projects: SideProject::get_all(),
            open_source_contributions: OpenSourceContribution::get_all(),
        };

        me.scholary_experiences.sort_by_key(|k| k.start_date);
        me.scholary_experiences.reverse();

        me.work_experiences.sort_by_key(|k| k.start_date);
        me.work_experiences.reverse();

        me.projects.sort_by_key(|k| k.priority);
        me.projects.reverse();

        me.side_projects.sort_by_key(|k| k.priority);
        me.side_projects.reverse();

        me.open_source_contributions.sort_by_key(|k| k.priority);
        me.open_source_contributions.reverse();

        me
    }
}

pub enum PersonLink {
    GitHub { username: &'static str },
    LinkedIn { link_name: &'static str },
}

impl PersonLink {
    fn get_all() -> Vec<Self> {
        use PersonLink::*;
        vec![
            GitHub {
                username: "MidasLamb",
            },
            LinkedIn {
                link_name: "midas-lambrichts",
            },
        ]
    }

    pub fn linkify(&self) -> String {
        match self {
            PersonLink::GitHub { username: u } => format!("https://github.com/{}/", u),
            PersonLink::LinkedIn { link_name: l } => format!("https://www.linkedin.com/in/{}/", l),
        }
    }

    pub fn short_view(&self) -> String {
        match self {
            PersonLink::GitHub { username: u } => format!("/{}/", u),
            PersonLink::LinkedIn { link_name: l } => format!("/in/{}/", l),
        }
    }
}

pub struct ScholaryExperience {
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub institution: &'static str,    // TODO: Turn into enum?
    pub degree: &'static str,         // TODO: Turn into enum?
    pub honors: Option<&'static str>, // TODO: Turn into enum?
    pub thesis_title: Option<&'static str>,
}

impl ScholaryExperience {
    fn get_all() -> Vec<Self> {
        vec![ScholaryExperience {
            start_date: NaiveDate::from_ymd(2014, 9, 25),
            end_date: Some(NaiveDate::from_ymd(2017, 6, 30)),
            institution: "KU Leuven",
            degree: "Bachelor of informatics",
            honors: Some("Cum Laude"),
            thesis_title: None,
        }, ScholaryExperience {
            start_date: NaiveDate::from_ymd(2017, 9, 25),
            end_date: Some(NaiveDate::from_ymd(2019, 6, 30)),
            institution: "KU Leuven",
            degree: "Master Computer Science",
            honors: Some("Cum Laude"),
            thesis_title: Some(
                "The Effects of the position of the Tutorial Persona throughout the Rose of Leary.",
            ),
        },
        ]
    }
}

pub struct WorkExperience {
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub title: &'static str,
    pub company: &'static str,
    pub summary: &'static str,
}

impl WorkExperience {
    fn get_all() -> Vec<Self> {
        vec![
            WorkExperience {
                start_date: NaiveDate::from_ymd(2015, 01, 01),
                end_date: Some(NaiveDate::from_ymd(2016, 12, 31)),
                title: "Software Tester",
                company: "Bringme",
                summary: r"Responsible for manually testing (including regression tests) of mobile and web applications.
Found a lot of edge-cases, which made me the least favorite tester of the developers.
Bringme introduced me to agile development.",
            },
            WorkExperience {
                start_date: NaiveDate::from_ymd(2017, 07, 20),
                end_date: Some(NaiveDate::from_ymd(2018, 06, 30)),
                title: "Member of Marketing Committee",
                company: "Ekonomika VZW",
                summary: r"Responsible for the entire IT-infrastructure of Ekonomika VZW, which included, but was not
limited to: creating new sites, maintaining sites, domain administration, transferring email setup
to GSuite, introducing Git (and GitHub) for version control,...",
            },
            WorkExperience {
                start_date: NaiveDate::from_ymd(2019, 07, 15),
                end_date: Some(NaiveDate::from_ymd(2021, 07, 31)),
                title: "Software Engineer",
                company: "Dover Fueling Solutions",
                summary: r"Responsible for migrating data management from C++ to C# in a Point of Sale (POS) system,
and ensuring that in less-than-ideal network conditions, the data is synchronised with the Cloud. Other responsibilities include 
maintaining a Web Server, investigating improvements, and working on the frontend (Angular). My team worked in an Agile fashion (Scrum), with two-week sprints.",
            },
            WorkExperience {
                start_date: NaiveDate::from_ymd(2021, 08, 1),
                end_date: None,
                title: "Senior Software Engineer",
                company: "Dover Fueling Solutions",
                summary: r"Responsible for migrating data management from C++ to C# in a POS system. Ensure data-synchronicity between the POS and the Cloud.
Also training/guiding colleagues in general development practices (PR etiquette, Agile, usage of git) as well as more project-specific trainings in backend and frontend.",
            },
        ]
    }
}

pub struct Language {
    pub language: &'static str,
    pub proficiency: LanguageProficiency,
}

impl Language {
    pub fn get_all() -> Vec<Self> {
        vec![
            Language {
                language: "English",
                proficiency: LanguageProficiency::Proficient,
            },
            Language {
                language: "Dutch",
                proficiency: LanguageProficiency::NativeSpeaker,
            },
            Language {
                language: "French",
                proficiency: LanguageProficiency::Basic,
            },
            Language {
                language: "German",
                proficiency: LanguageProficiency::Basic,
            },
        ]
    }
}

pub enum LanguageProficiency {
    NativeSpeaker,
    Proficient,
    Basic,
}

impl std::string::ToString for LanguageProficiency {
    fn to_string(&self) -> String {
        let v = match self {
            LanguageProficiency::NativeSpeaker => "Native Speaker",
            LanguageProficiency::Proficient => "Proficient",
            LanguageProficiency::Basic => "Basic knowledge",
        };
        v.to_owned()
    }
}

pub struct Skill {
    pub name: &'static str,
    pub ability: u8,
    pub note: Option<SkillNote>,
}

impl Skill {
    pub fn get_all() -> Vec<Skill> {
        vec![
            Skill {
                name: "C#",
                ability: 8,
                note: Some(SkillNote::AutoDidact),
            },
            Skill {
                name: "Rust",
                ability: 6,
                note: Some(SkillNote::AutoDidact),
            },
            Skill {
                name: "Git",
                ability: 7,
                note: None,
            },
            Skill {
                name: "SQL",
                ability: 7,
                note: None,
            },
            Skill {
                name: "Typescript",
                ability: 7,
                note: Some(SkillNote::AutoDidact),
            },
            Skill {
                name: "Software Architecture",
                ability: 6,
                note: None,
            },
            Skill {
                name: "Java",
                ability: 6,
                note: None,
            },
            Skill {
                name: "Information Visualisation",
                ability: 6,
                note: None,
            },
        ]
    }
}

pub enum SkillNote {
    AutoDidact,
}

impl std::string::ToString for SkillNote {
    fn to_string(&self) -> String {
        let v = match self {
            SkillNote::AutoDidact => "Autodidact",
        };
        v.to_owned()
    }
}

pub struct Project {
    pub name: &'static str,
    pub used_technologies: Vec<Technology>,
    pub link: Option<&'static str>,
    pub summary: &'static str,
    pub priority: usize,
}

impl Project {
    pub fn get_all() -> Vec<Self> {
        use Technology::*;
        vec![
            Project {
                name: "Speaker at RustConf 2021",
                used_technologies: vec![Rust, Fuzzing],
                link: Some("https://rustconf.com/speakers/midas-lambrichts"),
                summary: "Give a talk at RustConf 2021 about how one can use Fuzzing to drive development for a domain while having limited domain knowledge.",
                priority: 5
            },
            Project {
                name: "KBBC T&T Turnhout",
                used_technologies: vec![PHP, Laravel, Bootstrap, JavaScript, CSS3, HTML5, VueJs],
                link: Some("https://github.com/MidasLamb/TTTurnhout"),
                summary: "A Laravel Application which reads data from Basketbal Vlaanderen, and displays it in a userfriendly manner.
This is currently being rebuild in a complete Rust stack (async-graphql, actix, yew, graphql-client)",
                priority: 0
            },
            Project {
                name: "Ekonomika KD",
                used_technologies: vec![PHP, Laravel, Bootstrap, JavaScript, CSS3, HTML5],
                link: None,
                summary: "A Site to enable FEB students of the KU Leuven to reserve books.
The application handles the picking up of said books, expiration in case students take too long to
pick up their books, adding stock when a delivery is made, and preventing reservations in case
stock is too low. Hooks into a legacy systems which handles the monetary part of the
transactions. Most of the site required a KU Leuven SSO Login.",
                priority: 1
            },
            Project {
                name: "CS Schedule",
                used_technologies: vec![PHP, JavaScript, CSS3, HTML5],
                link: Some("https://cs-schedule.herokuapp.com/"),
                summary: "A small site which allows students computer science to get the time and place of courses for any
course of the master they want, which is necessary because some dutch courses require you to
take the english lectures, but they don't appear in the students their timetable provided by the KU
Leuven. It parses the courses from a webpage provided by the KU Leuven.",
                priority: 4,
            },
            Project {
                name: "Curriculum Vitae",
                used_technologies: vec![Rust, Yew, Wasm, CSS3, HTML5, Sass],
                link: Some("./#"),
                summary: r#"A CV is made of up two parts: Data & Representation. "Traditional" ways of making a CV (e.g. Word) do not account
for these. Data and representation are mixed, which I didn't like. That's why I decided to create my CV as a "webpage", which is then printed to pdf.
This lets me vary data (e.g. adding a new project) and representation (e.g. CSS), separately, without much hassle.
This CV is created through with Rust (Yew for building the wasm/html, grass for turning the Sass into CSS)."#,
                priority: 3
            },
            Project {
                name: "Rs Wiki Quest Checker",
                used_technologies: vec![JavaScript, HTML5, CSS3],
                link: Some("https://github.com/MidasLamb/RS-Wiki-Quest-Checker"),
                summary: r#"A Chrome and Firefox extension which will visualize the users progress relative to the quest (w.r.t. skills, quests completed,...)
and whether or not the user is able to start the quest on the Wiki for the MMORPG Runescape (specifically Runescape 3). Currently still has over 300 weekly users.
Further growth is stunted because the Wiki has been moved to a new platform which integrates what this extension does directly into the Wiki itself."#,
                priority: 2
            }
        ]
    }
}

pub enum Technology {
    PHP,
    Laravel,
    Bootstrap,
    JavaScript,
    HTML5,
    CSS3,
    VueJs,
    Rust,
    Yew,
    Wasm,
    Sass,
    Fuzzing,
}

impl std::string::ToString for Technology {
    fn to_string(&self) -> String {
        let v = match self {
            Technology::PHP => "PHP",
            Technology::Laravel => "Laravel",
            Technology::Bootstrap => "Bootstrap",
            Technology::JavaScript => "JavaScript",
            Technology::HTML5 => "HTML5",
            Technology::CSS3 => "CSS3",
            Technology::VueJs => "Vue.js",
            Technology::Rust => "Rust",
            Technology::Yew => "Yew",
            Technology::Wasm => "Wasm",
            Technology::Sass => "Sass",
            Technology::Fuzzing => "Fuzzing",
        };
        v.to_owned()
    }
}

pub struct SideProject {
    pub name: &'static str,
    pub used_technologies: Vec<Technology>,
    pub summary: &'static str,
    pub priority: usize,
}
impl SideProject {
    pub fn get_all() -> Vec<Self> {
        use Technology::*;
        vec![
            SideProject {
                name: "SQLite clone in Rust",
                used_technologies: vec![Rust],
                summary: "Created a small POC that could read and update an SQLite file, as long as it didn't need to update the B-Tree. A remnant of this side project can be found in my crate sqlite_varint.",
                priority: 1
            },
            SideProject {
                name: "Re-implement company internal RPC in Rust",
                used_technologies: vec![Rust],
                summary: "Re-implemented an internal RPC protocol in Rust, making use of bindgen to port message definitions. And using tokio_codec as a basis to implement communication.",
                priority: 2
            }
        ]
    }
}

pub struct OpenSourceContribution {
    pub repo_name: &'static str,
    pub link: &'static str,
    pub summary: &'static str,
    pub priority: usize,
}

impl OpenSourceContribution {
    pub fn get_all() -> Vec<Self> {
        use Technology::*;
        vec![
            OpenSourceContribution {
                repo_name: "rust",
                link: "https://github.com/rust-lang/rust/pull/83535/files",
                summary: "Fixed a small ICE.",
                priority: 1
            },
            OpenSourceContribution {
                repo_name: "azure-sdk-for-rust",
                link: "https://github.com/Azure/azure-sdk-for-rust/pulls?q=is%3Apr+author%3AMidasLamb",
                summary: "Fixed misalignment between REST api & implementation in the SDK (also fixed small other issues in other PRs)",
                priority: 2
            },
            OpenSourceContribution {
                repo_name: "wasmer",
                link: "https://github.com/wasmerio/wasmer/pulls?q=is%3Apr+author%3AMidasLamb",
                summary: "Made modifications towards getting the wasmer runtime up & running for 32bit Windows machines.",
                priority: 3
            },
            OpenSourceContribution {
                repo_name: "grass",
                link: "https://github.com/connorskees/grass/pulls?q=is%3Apr+author%3AMidasLamb+",
                summary: "Fix small issues found by fuzzing this implementation agains the official Sass implementation (dart-sass)",
                priority: 4
            },
            OpenSourceContribution {
                repo_name: "async-graphql",
                link: "https://github.com/async-graphql/async-graphql/pull/232",
                summary: "Small performance improvement",
                priority: 5
            },
        ]
    }
}
