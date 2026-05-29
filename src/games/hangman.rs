use std::time::Duration;
use rand::Rng;
use ratatui::{
    Frame,
    layout::{Rect, Layout, Constraint, Direction, Alignment},
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Clear},
};
use crossterm::event::KeyCode;
use crate::settings::ThemePalette;
use super::{Game, GameCommand};

const WORD_BANK: &[&str] = &[
    "TERMINAL", "ARCADE", "RUSTACEAN", "COMPILER", "DOUBLEBUFFER",
    "RETROWAVE", "KEYBOARD", "MONITOR", "DUNGEON", "SPACESHIP",
    "MINESWEEPER", "TETROMINO", "SHIELDS", "BATTLESHIP", "HANGMAN",
    "ABOUT", "SEARCH", "OTHER", "WHICH", "THEIR",
    "THERE", "CONTACT", "BUSINESS", "ONLINE", "FIRST",
    "WOULD", "SERVICES", "THESE", "CLICK", "SERVICE",
    "PRICE", "PEOPLE", "STATE", "EMAIL", "HEALTH",
    "WORLD", "PRODUCTS", "MUSIC", "SHOULD", "PRODUCT",
    "SYSTEM", "POLICY", "NUMBER", "PLEASE", "SUPPORT",
    "MESSAGE", "AFTER", "SOFTWARE", "VIDEO", "WHERE",
    "RIGHTS", "PUBLIC", "BOOKS", "SCHOOL", "THROUGH",
    "LINKS", "REVIEW", "YEARS", "ORDER", "PRIVACY",
    "ITEMS", "COMPANY", "GROUP", "UNDER", "GENERAL",
    "RESEARCH", "JANUARY", "REVIEWS", "PROGRAM", "GAMES",
    "COULD", "GREAT", "UNITED", "HOTEL", "CENTER",
    "STORE", "TRAVEL", "COMMENTS", "REPORT", "MEMBER",
    "DETAILS", "TERMS", "BEFORE", "HOTELS", "RIGHT",
    "BECAUSE", "LOCAL", "THOSE", "USING", "RESULTS",
    "OFFICE", "NATIONAL", "DESIGN", "POSTED", "INTERNET",
    "ADDRESS", "WITHIN", "STATES", "PHONE", "SHIPPING",
    "RESERVED", "SUBJECT", "BETWEEN", "FORUM", "FAMILY",
    "BASED", "BLACK", "CHECK", "SPECIAL", "PRICES",
    "WEBSITE", "INDEX", "BEING", "WOMEN", "TODAY",
    "SOUTH", "PROJECT", "PAGES", "VERSION", "SECTION",
    "FOUND", "SPORTS", "HOUSE", "RELATED", "SECURITY",
    "COUNTY", "AMERICAN", "PHOTO", "MEMBERS", "POWER",
    "WHILE", "NETWORK", "COMPUTER", "SYSTEMS", "THREE",
    "TOTAL", "PLACE", "DOWNLOAD", "WITHOUT", "ACCESS",
    "THINK", "NORTH", "CURRENT", "POSTS", "MEDIA",
    "CONTROL", "WATER", "HISTORY", "PICTURES", "PERSONAL",
    "SINCE", "GUIDE", "BOARD", "LOCATION", "CHANGE",
    "WHITE", "SMALL", "RATING", "CHILDREN", "DURING",
    "RETURN", "STUDENTS", "SHOPPING", "ACCOUNT", "TIMES",
    "SITES", "LEVEL", "DIGITAL", "PROFILE", "PREVIOUS",
    "EVENTS", "HOURS", "IMAGE", "TITLE", "ANOTHER",
    "SHALL", "PROPERTY", "CLASS", "STILL", "MONEY",
    "QUALITY", "EVERY", "LISTING", "CONTENT", "COUNTRY",
    "PRIVATE", "LITTLE", "VISIT", "TOOLS", "REPLY",
    "CUSTOMER", "DECEMBER", "COMPARE", "MOVIES", "INCLUDE",
    "COLLEGE", "VALUE", "ARTICLE", "PROVIDE", "SOURCE",
    "AUTHOR", "PRESS", "LEARN", "AROUND", "PRINT",
    "COURSE", "CANADA", "PROCESS", "STOCK", "TRAINING",
    "CREDIT", "POINT", "SCIENCE", "ADVANCED", "SALES",
    "ENGLISH", "ESTATE", "SELECT", "WINDOWS", "PHOTOS",
    "THREAD", "CATEGORY", "LARGE", "GALLERY", "TABLE",
    "REGISTER", "HOWEVER", "OCTOBER", "NOVEMBER", "MARKET",
    "LIBRARY", "REALLY", "ACTION", "START", "SERIES",
    "MODEL", "FEATURES", "INDUSTRY", "HUMAN", "PROVIDED",
    "REQUIRED", "SECOND", "MOVIE", "FORUMS", "MARCH",
    "BETTER", "YAHOO", "GOING", "MEDICAL", "FRIEND",
    "SERVER", "STUDY", "STAFF", "ARTICLES", "FEEDBACK",
    "AGAIN", "LOOKING", "ISSUES", "APRIL", "NEVER",
    "USERS", "COMPLETE", "STREET", "TOPIC", "COMMENT",
    "THINGS", "WORKING", "AGAINST", "STANDARD", "PERSON",
    "BELOW", "MOBILE", "PARTY", "PAYMENT", "LOGIN",
    "STUDENT", "PROGRAMS", "OFFERS", "LEGAL", "ABOVE",
    "RECENT", "STORES", "PROBLEM", "MEMORY", "SOCIAL",
    "AUGUST", "QUOTE", "LANGUAGE", "STORY", "OPTIONS",
    "RATES", "CREATE", "YOUNG", "AMERICA", "FIELD",
    "PAPER", "SINGLE", "EXAMPLE", "GIRLS", "PASSWORD",
    "LATEST", "QUESTION", "CHANGES", "NIGHT", "TEXAS",
    "POKER", "STATUS", "BROWSE", "ISSUE", "RANGE",
    "BUILDING", "SELLER", "COURT", "FEBRUARY", "ALWAYS",
    "RESULT", "AUDIO", "LIGHT", "WRITE", "OFFER",
    "GROUPS", "GIVEN", "FILES", "EVENT", "RELEASE",
    "ANALYSIS", "REQUEST", "CHINA", "MAKING", "PICTURE",
    "NEEDS", "POSSIBLE", "MIGHT", "MONTH", "MAJOR",
    "AREAS", "FUTURE", "SPACE", "CARDS", "PROBLEMS",
    "LONDON", "MEETING", "BECOME", "INTEREST", "CHILD",
    "ENTER", "SHARE", "SIMILAR", "GARDEN", "SCHOOLS",
    "MILLION", "ADDED", "LISTED", "LEARNING", "ENERGY",
    "DELIVERY", "POPULAR", "STORIES", "JOURNAL", "REPORTS",
    "WELCOME", "CENTRAL", "IMAGES", "NOTICE", "ORIGINAL",
    "RADIO", "UNTIL", "COLOR", "COUNCIL", "INCLUDES",
    "TRACK", "ARCHIVE", "OTHERS", "FORMAT", "LEAST",
    "SOCIETY", "MONTHS", "SAFETY", "FRIENDS", "TRADE",
    "EDITION", "MESSAGES", "FURTHER", "UPDATED", "HAVING",
    "PROVIDES", "DAVID", "ALREADY", "GREEN", "STUDIES",
    "CLOSE", "COMMON", "DRIVE", "SPECIFIC", "SEVERAL",
    "LIVING", "CALLED", "SHORT", "DISPLAY", "LIMITED",
    "POWERED", "MEANS", "DIRECTOR", "DAILY", "BEACH",
    "NATURAL", "WHETHER", "PERIOD", "PLANNING", "DATABASE",
    "OFFICIAL", "WEATHER", "AVERAGE", "WINDOW", "FRANCE",
    "REGION", "ISLAND", "RECORD", "DIRECT", "RECORDS",
    "DISTRICT", "CALENDAR", "COSTS", "STYLE", "FRONT",
    "UPDATE", "PARTS", "EARLY", "MILES", "SOUND",
    "RESOURCE", "PRESENT", "EITHER", "DOCUMENT", "WORKS",
    "MATERIAL", "WRITTEN", "FEDERAL", "HOSTING", "RULES",
    "FINAL", "ADULT", "TICKETS", "THING", "CENTRE",
    "CHEAP", "FINANCE", "MINUTES", "THIRD", "GIFTS",
    "EUROPE", "READING", "TOPICS", "COVER", "USUALLY",
    "TOGETHER", "VIDEOS", "PERCENT", "FUNCTION", "GETTING",
    "GLOBAL", "ECONOMIC", "PLAYER", "PROJECTS", "LYRICS",
    "OFTEN", "SUBMIT", "GERMANY", "AMOUNT", "WATCH",
    "INCLUDED", "THOUGH", "THANKS", "DEALS", "VARIOUS",
    "WORDS", "LINUX", "JAMES", "WEIGHT", "HEART",
    "RECEIVED", "CHOOSE", "ARCHIVES", "POINTS", "MAGAZINE",
    "ERROR", "CAMERA", "CLEAR", "RECEIVE", "DOMAIN",
    "METHODS", "CHAPTER", "MAKES", "POLICIES", "BEAUTY",
    "MANAGER", "INDIA", "POSITION", "TAKEN", "LISTINGS",
    "MODELS", "MICHAEL", "KNOWN", "CASES", "FLORIDA",
    "SIMPLE", "QUICK", "WIRELESS", "LICENSE", "FRIDAY",
    "WHOLE", "ANNUAL", "LATER", "BASIC", "SHOWS",
    "GOOGLE", "CHURCH", "METHOD", "PURCHASE", "ACTIVE",
    "RESPONSE", "PRACTICE", "HARDWARE", "FIGURE", "HOLIDAY",
    "ENOUGH", "DESIGNED", "ALONG", "AMONG", "DEATH",
    "WRITING", "SPEED", "BRAND", "DISCOUNT", "HIGHER",
    "EFFECTS", "CREATED", "REMEMBER", "YELLOW", "INCREASE",
    "KINGDOM", "THOUGHT", "STUFF", "FRENCH", "STORAGE",
    "JAPAN", "DOING", "LOANS", "SHOES", "ENTRY",
    "NATURE", "ORDERS", "AFRICA", "SUMMARY", "GROWTH",
    "NOTES", "AGENCY", "MONDAY", "EUROPEAN", "ACTIVITY",
    "ALTHOUGH", "WESTERN", "INCOME", "FORCE", "OVERALL",
    "RIVER", "PACKAGE", "CONTENTS", "PLAYERS", "ENGINE",
    "ALBUM", "REGIONAL", "SUPPLIES", "STARTED", "VIEWS",
    "PLANS", "DOUBLE", "BUILD", "SCREEN", "EXCHANGE",
    "TYPES", "LINES", "CONTINUE", "ACROSS", "BENEFITS",
    "NEEDED", "SEASON", "APPLY", "SOMEONE", "ANYTHING",
    "PRINTER", "BELIEVE", "EFFECT", "ASKED", "SUNDAY",
    "CASINO", "VOLUME", "CROSS", "ANYONE", "MORTGAGE",
    "SILVER", "INSIDE", "SOLUTION", "MATURE", "RATHER",
    "WEEKS", "ADDITION", "SUPPLY", "NOTHING", "CERTAIN",
    "RUNNING", "LOWER", "UNION", "JEWELRY", "CLOTHING",
    "NAMES", "ROBERT", "HOMEPAGE", "SKILLS", "ISLANDS",
    "ADVICE", "CAREER", "MILITARY", "RENTAL", "DECISION",
    "LEAVE", "BRITISH", "TEENS", "WOMAN", "SELLERS",
    "MIDDLE", "CABLE", "TAKING", "VALUES", "DIVISION",
    "COMING", "TUESDAY", "OBJECT", "LESBIAN", "MACHINE",
    "LENGTH", "ACTUALLY", "SCORE", "CLIENT", "RETURNS",
    "CAPITAL", "FOLLOW", "SAMPLE", "SHOWN", "SATURDAY",
    "ENGLAND", "CULTURE", "FLASH", "GEORGE", "CHOICE",
    "STARTING", "THURSDAY", "COURSES", "CONSUMER", "AIRPORT",
    "FOREIGN", "ARTIST", "OUTSIDE", "LEVELS", "CHANNEL",
    "LETTER", "PHONES", "IDEAS", "SUMMER", "ALLOW",
    "DEGREE", "CONTRACT", "BUTTON", "RELEASES", "HOMES",
    "SUPER", "MATTER", "CUSTOM", "VIRGINIA", "ALMOST",
    "LOCATED", "MULTIPLE", "ASIAN", "EDITOR", "CAUSE",
    "FOCUS", "FEATURED", "ROOMS", "FEMALE", "THOMAS",
    "PRIMARY", "CANCER", "NUMBERS", "REASON", "BROWSER",
    "SPRING", "ANSWER", "VOICE", "FRIENDLY", "SCHEDULE",
    "PURPOSE", "FEATURE", "COMES", "POLICE", "EVERYONE",
    "APPROACH", "CAMERAS", "BROWN", "PHYSICAL", "MEDICINE",
    "RATINGS", "CHICAGO", "FORMS", "GLASS", "HAPPY",
    "SMITH", "WANTED", "THANK", "UNIQUE", "SURVEY",
    "PRIOR", "SPORT", "READY", "ANIMAL", "SOURCES",
    "MEXICO", "REGULAR", "SECURE", "SIMPLY", "EVIDENCE",
    "STATION", "ROUND", "PAYPAL", "FAVORITE", "OPTION",
    "MASTER", "VALLEY", "RECENTLY", "PROBABLY", "RENTALS",
    "BUILT", "BLOOD", "IMPROVE", "LARGER", "NETWORKS",
    "EARTH", "PARENTS", "NOKIA", "IMPACT", "TRANSFER",
    "KITCHEN", "STRONG", "CAROLINA", "WEDDING", "HOSPITAL",
    "GROUND", "OVERVIEW", "OWNERS", "DISEASE", "ITALY",
    "PERFECT", "CLASSIC", "BASIS", "COMMAND", "CITIES",
    "WILLIAM", "EXPRESS", "AWARD", "DISTANCE", "PETER",
    "ENSURE", "INVOLVED", "EXTRA", "PARTNERS", "BUDGET",
    "RATED", "GUIDES", "SUCCESS", "MAXIMUM", "EXISTING",
    "QUITE", "SELECTED", "AMAZON", "PATIENTS", "WARNING",
    "HORSE", "FORWARD", "FLOWERS", "STARS", "LISTS",
    "OWNER", "RETAIL", "ANIMALS", "USEFUL", "DIRECTLY",
    "HOUSING", "TAKES", "BRING", "CATALOG", "SEARCHES",
    "TRYING", "MOTHER", "TRAFFIC", "JOINED", "INPUT",
    "STRATEGY", "AGENT", "VALID", "MODERN", "SENIOR",
    "IRELAND", "TEACHING", "GRAND", "TESTING", "TRIAL",
    "CHARGE", "UNITS", "INSTEAD", "CANADIAN", "NORMAL",
    "WROTE", "SHIPS", "ENTIRE", "LEADING", "METAL",
    "POSITIVE", "FITNESS", "CHINESE", "OPINION", "FOOTBALL",
    "ABSTRACT", "OUTPUT", "FUNDS", "GREATER", "LIKELY",
    "DEVELOP", "ARTISTS", "GUEST", "SEEMS", "TRUST",
    "CONTAINS", "SESSION", "MULTI", "REPUBLIC", "VACATION",
    "CENTURY", "ACADEMIC", "GRAPHICS", "INDIAN", "EXPECTED",
    "GRADE", "DATING", "PACIFIC", "MOUNTAIN", "FILTER",
    "MAILING", "VEHICLE", "LONGER", "CONSIDER", "NORTHERN",
    "BEHIND", "PANEL", "FLOOR", "GERMAN", "BUYING",
    "MATCH", "PROPOSED", "DEFAULT", "REQUIRE", "OUTDOOR",
    "MORNING", "ALLOWS", "PROTEIN", "PLANT", "REPORTED",
    "POLITICS", "PARTNER", "AUTHORS", "BOARDS", "FACULTY",
    "PARTIES", "MISSION", "STRING", "SENSE", "MODIFIED",
    "RELEASED", "STAGE", "INTERNAL", "GOODS", "UNLESS",
    "RICHARD", "DETAILED", "JAPANESE", "APPROVED", "TARGET",
    "EXCEPT", "ABILITY", "MAYBE", "MOVING", "BRANDS",
    "PLACES", "PRETTY", "SPAIN", "SOUTHERN", "YOURSELF",
    "WINTER", "BATTERY", "YOUTH", "PRESSURE", "BOSTON",
    "KEYWORDS", "MEDIUM", "BREAK", "PURPOSES", "DANCE",
    "ITSELF", "DEFINED", "PAPERS", "PLAYING", "AWARDS",
    "STUDIO", "READER", "VIRTUAL", "DEVICE", "ANSWERS",
    "REMOTE", "EXTERNAL", "APPLE", "OFFERED", "THEORY",
    "ENJOY", "REMOVE", "SURFACE", "MINIMUM", "VISUAL",
    "VARIETY", "TEACHERS", "MARTIN", "MANUAL", "BLOCK",
    "SUBJECTS", "AGENTS", "REPAIR", "CIVIL", "STEEL",
    "SONGS", "FIXED", "WRONG", "HANDS", "FINALLY",
    "UPDATES", "DESKTOP", "CLASSES", "PARIS", "SECTOR",
    "CAPACITY", "REQUIRES", "JERSEY", "FULLY", "FATHER",
    "ELECTRIC", "QUOTES", "OFFICER", "DRIVER", "RESPECT",
    "UNKNOWN", "WORTH", "TEACHER", "WORKERS", "GEORGIA",
    "PEACE", "CAMPUS", "SHOWING", "CREATIVE", "COAST",
    "BENEFIT", "PROGRESS", "FUNDING", "DEVICES", "GRANT",
    "AGREE", "FICTION", "WATCHES", "CAREERS", "BEYOND",
    "FAMILIES", "MUSEUM", "BLOGS", "ACCEPTED", "FORMER",
    "COMPLEX", "AGENCIES", "PARENT", "SPANISH", "MICHIGAN",
    "COLUMBIA", "SETTING", "SCALE", "STAND", "ECONOMY",
    "HIGHEST", "HELPFUL", "MONTHLY", "CRITICAL", "FRAME",
    "MUSICAL", "ANGELES", "EMPLOYEE", "CHIEF", "GIVES",
    "BOTTOM", "PACKAGES", "DETAIL", "CHANGED", "HEARD",
    "BEGIN", "COLORADO", "ROYAL", "CLEAN", "SWITCH",
    "RUSSIAN", "LARGEST", "AFRICAN", "TITLES", "RELEVANT",
    "JUSTICE", "CONNECT", "BIBLE", "BASKET", "APPLIED",
    "WEEKLY", "DEMAND", "SUITE", "VEGAS", "SQUARE",
    "CHRIS", "ADVANCE", "AUCTION", "ALLOWED", "CORRECT",
    "CHARLES", "NATION", "SELLING", "PIECE", "SHEET",
    "SEVEN", "OLDER", "ILLINOIS", "ELEMENTS", "SPECIES",
    "CELLS", "MODULE", "RESORT", "FACILITY", "RANDOM",
    "PRICING", "MINISTER", "MOTION", "LOOKS", "FASHION",
    "VISITORS", "MONITOR", "TRADING", "FOREST", "CALLS",
    "WHOSE", "COVERAGE", "COUPLE", "GIVING", "CHANCE",
    "VISION", "ENDING", "CLIENTS", "ACTIONS", "LISTEN",
    "DISCUSS", "ACCEPT", "NAKED", "CLINICAL", "SCIENCES",
    "MARKETS", "LOWEST", "HIGHLY", "APPEAR", "LIVES",
    "CURRENCY", "LEATHER", "PATIENT", "ACTUAL", "STONE",
    "COMMERCE", "PERHAPS", "PERSONS", "TESTS", "VILLAGE",
    "ACCOUNTS", "AMATEUR", "FACTORS", "COFFEE", "SETTINGS",
    "BUYER", "CULTURAL", "STEVE", "EASILY", "POSTER",
    "CLOSED", "HOLIDAYS", "ZEALAND", "BALANCE", "GRADUATE",
    "REPLIES", "INITIAL", "LABEL", "THINKING", "SCOTT",
];

pub struct HangmanGame {
    secret_word: String,
    guessed_chars: Vec<char>,
    wrong_guesses: usize,
    game_over: bool,
    won: bool,
    paused: bool,
    score: u32,
    warning_msg: Option<String>,
}

impl Default for HangmanGame {
    fn default() -> Self {
        Self::new()
    }
}

impl HangmanGame {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..WORD_BANK.len());
        let secret = WORD_BANK[idx].to_string();

        Self {
            secret_word: secret,
            guessed_chars: Vec::new(),
            wrong_guesses: 0,
            game_over: false,
            won: false,
            paused: false,
            score: 0,
            warning_msg: None,
        }
    }

    fn make_guess(&mut self, c: char) {
        self.warning_msg = None;

        if self.guessed_chars.contains(&c) {
            self.warning_msg = Some(format!("Already guessed letter '{}'!", c));
            return;
        }

        self.guessed_chars.push(c);

        if !self.secret_word.contains(c) {
            self.wrong_guesses += 1;
            if self.wrong_guesses >= 6 {
                self.game_over = true;
                self.won = false;
            }
        } else {
            // Check victory
            let mut completed = true;
            for sc in self.secret_word.chars() {
                if !self.guessed_chars.contains(&sc) {
                    completed = false;
                    break;
                }
            }
            if completed {
                self.won = true;
                self.game_over = true;
                self.score = 600 - (self.wrong_guesses as u32 * 100);
            }
        }
    }

    fn get_gallows_ascii(&self) -> Vec<&'static str> {
        match self.wrong_guesses {
            0 => vec![
                "   +---+  ",
                "   |   |  ",
                "       |  ",
                "       |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            1 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "       |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            2 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "   |   |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            3 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|   |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            4 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|\\  |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            5 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|\\  |  ",
                "  /    |  ",
                "       |  ",
                "  ========="
            ],
            _ => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|\\  |  ",
                "  / \\  |  ",
                "       |  ",
                "  ========="
            ],
        }
    }
}

impl Game for HangmanGame {
    fn update(&mut self, _delta: Duration) {}

    fn handle_input(&mut self, key: KeyCode) -> GameCommand {
        if self.game_over {
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') => return GameCommand::Restart,
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return GameCommand::Exit,
                _ => return GameCommand::None,
            }
        }

        if key == KeyCode::Tab {
            self.paused = !self.paused;
            return GameCommand::None;
        }

        if self.paused {
            if key == KeyCode::Esc || key == KeyCode::Char('q') || key == KeyCode::Char('Q') {
                return GameCommand::Exit;
            }
            return GameCommand::None;
        }

        match key {
            KeyCode::Char(c) => {
                if c.is_ascii_alphabetic() {
                    let uppercase_c = c.to_ascii_uppercase();
                    self.make_guess(uppercase_c);
                }
            }
            KeyCode::Esc => {
                return GameCommand::Exit;
            }
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect, palette: &ThemePalette) {
        let outer_block = Block::default()
            .title(" HANGMAN CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(palette.danger).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });

        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(28), // Gallows area
                Constraint::Min(12),    // Secret Word & Letters
            ])
            .split(inner_area);

        let gallow_area = layouts[0];
        let side_area = layouts[1];

        // 1. Draw Gallows
        let gallow_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(palette.muted));
        let gallow_inner = gallow_block.inner(gallow_area);
        frame.render_widget(gallow_block, gallow_area);

        let ascii = self.get_gallows_ascii();
        let mut gallow_lines = Vec::new();
        gallow_lines.push(Line::from(""));
        for row in ascii {
            gallow_lines.push(Line::from(Span::styled(
                format!("  {}", row),
                Style::default().fg(palette.muted).add_modifier(Modifier::BOLD)
            )));
        }
        let gallow_paragraph = Paragraph::new(gallow_lines);
        frame.render_widget(gallow_paragraph, gallow_inner);

        // 2. Draw Side Board Panels
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Secret Word Display
                Constraint::Min(6),    // Letters & Warnings
            ])
            .split(side_area);

        // Secret Word Display
        let mut secret_spans = Vec::new();
        secret_spans.push(Span::raw("   ")); // spacing
        for sc in self.secret_word.chars() {
            if self.guessed_chars.contains(&sc) || self.game_over {
                secret_spans.push(Span::styled(
                    format!("{} ", sc),
                    Style::default().fg(if self.guessed_chars.contains(&sc) { palette.accent } else { palette.danger }).add_modifier(Modifier::BOLD)
                ));
            } else {
                secret_spans.push(Span::styled("_ ", Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD)));
            }
        }
        let word_content = vec![
            Line::from(""),
            Line::from(Span::styled("   GUESS SECRET WORD:", Style::default().fg(palette.muted))),
            Line::from(""),
            Line::from(secret_spans),
        ];
        let word_paragraph = Paragraph::new(word_content)
            .block(Block::default().borders(Borders::ALL).title("WORD"));
        frame.render_widget(word_paragraph, side_layout[0]);

        // Letters Guessing Tracker
        let mut guess_content = Vec::new();
        guess_content.push(Line::from(""));
        
        let mut guessed_line_spans = vec![Span::styled("   Guessed: ", Style::default().fg(palette.muted))];
        if self.guessed_chars.is_empty() {
            guessed_line_spans.push(Span::styled("None", Style::default().fg(palette.muted)));
        } else {
            for &c in &self.guessed_chars {
                let correct = self.secret_word.contains(c);
                guessed_line_spans.push(Span::styled(
                    format!("{} ", c),
                    Style::default().fg(if correct { palette.accent } else { palette.danger }).add_modifier(Modifier::BOLD)
                ));
            }
        }
        guess_content.push(Line::from(guessed_line_spans));
        guess_content.push(Line::from(""));

        // Warning or Action instruction
        if let Some(ref warn) = self.warning_msg {
            guess_content.push(Line::from(Span::styled(format!("   ⚠ {}", warn), Style::default().fg(palette.accent_alt))));
        } else {
            guess_content.push(Line::from(Span::styled("   Type any letter [A-Z] to guess!", Style::default().fg(palette.muted))));
        }
        guess_content.push(Line::from(""));

        let lives_left = 6u32.saturating_sub(self.wrong_guesses as u32);
        let mut heart_spans = vec![Span::styled("   LIVES: ", Style::default().fg(palette.muted))];
        for _ in 0..lives_left {
            heart_spans.push(Span::styled("♥ ", Style::default().fg(palette.danger)));
        }
        for _ in 0..(6 - lives_left) {
            heart_spans.push(Span::styled(". ", Style::default().fg(palette.muted)));
        }
        guess_content.push(Line::from(heart_spans));

        let guess_paragraph = Paragraph::new(guess_content)
            .block(Block::default().borders(Borders::ALL).title("TACTICAL DATA"));
        frame.render_widget(guess_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: gallow_inner.x + 3,
                y: gallow_inner.y + 1,
                width: 18,
                height: 5,
            };
            frame.render_widget(Clear, pause_area);
            let pause_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" PAUSED ", Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled("Press [Tab] to resume", Style::default().fg(palette.muted))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(palette.accent_alt)));
            frame.render_widget(pause_widget, pause_area);
        } else if self.game_over {
            let go_area = Rect {
                x: gallow_inner.x + 2,
                y: gallow_inner.y + 1,
                width: 22,
                height: 8,
            };
            frame.render_widget(Clear, go_area);
            
            let message = if self.won {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" FREEDOM! WINNER ", Style::default().fg(palette.accent).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Score: {}", self.score)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to retry", Style::default().fg(palette.accent))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(palette.muted))),
                ]
            } else {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" EXECUTED... DEFEAT ", Style::default().fg(palette.danger).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Word: {}", self.secret_word)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to retry", Style::default().fg(palette.accent))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(palette.muted))),
                ]
            };

            let go_widget = Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(if self.won { palette.accent } else { palette.danger })));
            frame.render_widget(go_widget, go_area);
        }
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }
}
