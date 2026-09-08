mod questions;

use macroquad::prelude::*;
use questions::QUESTIONS;
use std::path::PathBuf;

const W: f32 = 1200.;
const H: f32 = 800.;
const BG: Color = Color::new(0.035, 0.055, 0.12, 1.);
const PANEL: Color = Color::new(0.075, 0.10, 0.19, 1.);
const INK: Color = Color::new(0.91, 0.95, 1., 1.);
const MUTED: Color = Color::new(0.52, 0.61, 0.75, 1.);
const MINT: Color = Color::new(0.36, 0.94, 0.75, 1.);
const GOLD: Color = Color::new(1., 0.79, 0.36, 1.);
const ACCENTS: [Color; 3] = [
    Color::new(0.64, 0.52, 1., 1.),
    Color::new(0.28, 0.77, 1., 1.),
    MINT,
];
const NAMES: [&str; 3] = ["Pixel Planet", "Tech Station", "Science Galaxy"];
const BADGES: [(&str, &str); 8] = [
    ("First Launch", "Finish your first mission"),
    ("Pixel Pilot", "Get 8 game answers right"),
    ("Tech Inventor", "Get 8 tech answers right"),
    ("Science Scout", "Get 8 science answers right"),
    ("On a Roll", "Get 3 right in a row"),
    ("Perfect Orbit", "Get 8/8 in one mission"),
    ("World Explorer", "Finish a mission in every world"),
    ("Knowledge Hero", "Earn 1,000 XP"),
];

#[derive(Default, Clone, Debug, PartialEq)]
struct Progress {
    xp: u32,
    correct: [u32; 3],
    missions: [u32; 3],
    best_streak: u32,
    perfect: u32,
}
impl Progress {
    fn badges(&self) -> [bool; 8] {
        [
            self.missions.iter().sum::<u32>() > 0,
            self.correct[0] >= 8,
            self.correct[1] >= 8,
            self.correct[2] >= 8,
            self.best_streak >= 3,
            self.perfect > 0,
            self.missions.iter().all(|&n| n > 0),
            self.xp >= 1000,
        ]
    }
    fn encode(&self) -> String {
        format!(
            "QUESTLAB1 {} {} {} {} {} {} {} {} {}",
            self.xp,
            self.correct[0],
            self.correct[1],
            self.correct[2],
            self.missions[0],
            self.missions[1],
            self.missions[2],
            self.best_streak,
            self.perfect
        )
    }
    fn decode(s: &str) -> Option<Self> {
        let mut words = s.split_whitespace();
        if words.next()? != "QUESTLAB1" {
            return None;
        }
        let n: Vec<u32> = words.map(str::parse).collect::<Result<_, _>>().ok()?;
        if n.len() != 9 || n.iter().any(|&x| x > 100_000_000) {
            return None;
        }
        Some(Self {
            xp: n[0],
            correct: [n[1], n[2], n[3]],
            missions: [n[4], n[5], n[6]],
            best_streak: n[7],
            perfect: n[8],
        })
    }
}
fn save_path() -> PathBuf {
    let root = std::env::var_os("QUEST_LAB_SAVE_DIR")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("LOCALAPPDATA").map(PathBuf::from))
        .or_else(|| std::env::var_os("XDG_DATA_HOME").map(PathBuf::from))
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from("."));
    root.join("quest-lab/progress.txt")
}
fn save(p: &Progress) -> std::io::Result<()> {
    let path = save_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, p.encode())
}
fn shuffle<T>(items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let j = rand::gen_range(0, i + 1);
        items.swap(i, j);
    }
}
struct Mission {
    world: usize,
    deck: Vec<usize>,
    choices: [usize; 4],
    index: usize,
    selected: Option<usize>,
    correct: u32,
    streak: u32,
    earned: u32,
    new_badges: Vec<usize>,
}
impl Mission {
    fn new(world: usize) -> Self {
        let mut deck: Vec<usize> = QUESTIONS
            .iter()
            .enumerate()
            .filter(|(_, q)| q.world == world)
            .map(|(i, _)| i)
            .collect();
        shuffle(&mut deck);
        deck.truncate(8);
        let mut choices = [0, 1, 2, 3];
        shuffle(&mut choices);
        Self {
            world,
            deck,
            choices,
            index: 0,
            selected: None,
            correct: 0,
            streak: 0,
            earned: 0,
            new_badges: vec![],
        }
    }
    fn answer(&mut self, choice: usize, progress: &mut Progress) -> bool {
        if self.selected.is_some() {
            return false;
        }
        self.selected = Some(choice);
        let right = self.choices[choice] == 0;
        let xp = if right { 25 } else { 5 };
        self.earned += xp;
        progress.xp += xp;
        if right {
            self.correct += 1;
            self.streak += 1;
            progress.correct[self.world] += 1;
        } else {
            self.streak = 0;
        }
        progress.best_streak = progress.best_streak.max(self.streak);
        right
    }
}
#[derive(PartialEq)]
enum Screen {
    Home,
    Quiz,
    Result,
    Badges,
}
struct Spark {
    p: Vec2,
    v: Vec2,
    life: f32,
    color: Color,
}
fn burst(sparks: &mut Vec<Spark>, center: Vec2) {
    for _ in 0..65 {
        sparks.push(Spark {
            p: center,
            v: vec2(rand::gen_range(-230., 230.), rand::gen_range(-330., 30.)),
            life: rand::gen_range(0.6, 1.5),
            color: ACCENTS[rand::gen_range(0, 3)],
        });
    }
}
fn panel(r: Rect, color: Color) {
    let rad = 16_f32.min(r.w / 2.).min(r.h / 2.);
    draw_rectangle(r.x + rad, r.y, r.w - rad * 2., r.h, color);
    draw_rectangle(r.x, r.y + rad, r.w, r.h - rad * 2., color);
    for (x, y) in [
        (r.x + rad, r.y + rad),
        (r.x + r.w - rad, r.y + rad),
        (r.x + rad, r.y + r.h - rad),
        (r.x + r.w - rad, r.y + r.h - rad),
    ] {
        draw_circle(x, y, rad, color);
    }
}
fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(s, x, y, size, color);
}
fn wrap(s: &str, x: f32, y: f32, width: f32, size: f32, color: Color) {
    let mut line = String::new();
    let mut baseline = y;
    for word in s.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if measure_text(&candidate, None, size as u16, 1.).width > width && !line.is_empty() {
            text(&line, x, baseline, size, color);
            baseline += size * 1.3;
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    text(&line, x, baseline, size, color);
}
fn button(label: &str, r: Rect, color: Color, mouse: Vec2) -> bool {
    let hover = r.contains(mouse);
    panel(
        r,
        if hover {
            Color::new(
                (color.r + 0.09).min(1.),
                (color.g + 0.09).min(1.),
                (color.b + 0.09).min(1.),
                1.,
            )
        } else {
            color
        },
    );
    let dark = color.r + color.g + color.b > 1.5;
    let dims = measure_text(label, None, 24, 1.);
    text(
        label,
        r.x + (r.w - dims.width) / 2.,
        r.y + r.h / 2. + 8.,
        24.,
        if dark { BG } else { INK },
    );
    hover && is_mouse_button_pressed(MouseButton::Left)
}
fn icon(kind: usize, x: f32, y: f32, scale: f32, color: Color) {
    match kind % 3 {
        0 => {
            panel(
                Rect::new(x - 38. * scale, y - 23. * scale, 76. * scale, 46. * scale),
                color,
            );
            draw_rectangle(x - 25. * scale, y - 4. * scale, 22. * scale, 7. * scale, BG);
            draw_rectangle(
                x - 17. * scale,
                y - 12. * scale,
                7. * scale,
                23. * scale,
                BG,
            );
            draw_circle(x + 17. * scale, y - 6. * scale, 4. * scale, BG);
            draw_circle(x + 25. * scale, y + 7. * scale, 4. * scale, BG);
        }
        1 => {
            draw_rectangle(
                x - 25. * scale,
                y - 25. * scale,
                50. * scale,
                50. * scale,
                color,
            );
            draw_rectangle(
                x - 15. * scale,
                y - 15. * scale,
                30. * scale,
                30. * scale,
                BG,
            );
            for n in -2..=2 {
                let o = n as f32 * 9. * scale;
                draw_line(
                    x + o,
                    y - 34. * scale,
                    x + o,
                    y - 25. * scale,
                    3. * scale,
                    color,
                );
                draw_line(
                    x + o,
                    y + 25. * scale,
                    x + o,
                    y + 34. * scale,
                    3. * scale,
                    color,
                );
                draw_line(
                    x - 34. * scale,
                    y + o,
                    x - 25. * scale,
                    y + o,
                    3. * scale,
                    color,
                );
                draw_line(
                    x + 25. * scale,
                    y + o,
                    x + 34. * scale,
                    y + o,
                    3. * scale,
                    color,
                );
            }
        }
        _ => {
            for a in [0., std::f32::consts::PI / 3., -std::f32::consts::PI / 3.] {
                let mut prev = vec2(x + 36. * scale * a.cos(), y + 36. * scale * a.sin());
                for n in 1..=60 {
                    let t = n as f32 / 60. * std::f32::consts::TAU;
                    let p = vec2(
                        x + scale * (36. * t.cos() * a.cos() - 13. * t.sin() * a.sin()),
                        y + scale * (36. * t.cos() * a.sin() + 13. * t.sin() * a.cos()),
                    );
                    draw_line(prev.x, prev.y, p.x, p.y, 2. * scale, color);
                    prev = p;
                }
            }
            draw_circle(x, y, 6. * scale, color);
        }
    }
}
fn badge(i: usize, x: f32, y: f32, unlocked: bool) {
    let c = if unlocked {
        GOLD
    } else {
        Color::new(0.19, 0.24, 0.34, 1.)
    };
    draw_poly(x, y, 6, 39., 30., c);
    draw_poly(x, y, 6, 33., 30., BG);
    if unlocked {
        icon(i, x, y, 0.58, c);
    } else {
        text("?", x - 9., y + 11., 34., MUTED);
    }
}
fn robot(x: f32, y: f32, time: f32) {
    let y = y + (time * 2.).sin() * 7.;
    draw_ellipse(x, y + 112., 76., 12., 0., Color::new(0.02, 0.03, 0.08, 0.6));
    draw_line(x, y - 67., x, y - 91., 5., MINT);
    draw_circle(x, y - 95., 7., GOLD);
    panel(
        Rect::new(x - 64., y - 66., 128., 98.),
        Color::new(0.7, 0.82, 0.94, 1.),
    );
    panel(Rect::new(x - 51., y - 51., 102., 60.), BG);
    draw_circle(x - 24., y - 23., 8., MINT);
    draw_circle(x + 24., y - 23., 8., MINT);
    draw_line(x - 12., y - 4., x + 12., y - 4., 3., MINT);
    panel(Rect::new(x - 40., y + 40., 80., 50.), ACCENTS[1]);
    draw_circle(x, y + 60., 8., GOLD);
    draw_line(x - 52., y + 49., x - 78., y + 72., 10., ACCENTS[1]);
    draw_line(x + 52., y + 49., x + 83., y + 25., 10., ACCENTS[1]);
}

fn conf() -> Conf {
    Conf {
        window_title: "Quest Lab | A trivia adventure".into(),
        window_width: 1200,
        window_height: 800,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    rand::srand(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64,
    );
    let (mut progress, mut save_error) = match std::fs::read_to_string(save_path()) {
        Ok(s) => match Progress::decode(&s) {
            Some(p) => (p, false),
            None => (Progress::default(), true),
        },
        Err(e) => (
            Progress::default(),
            e.kind() != std::io::ErrorKind::NotFound,
        ),
    };
    let mut screen = Screen::Home;
    let mut mission = Mission::new(0);
    let mut before_badges = progress.badges();
    let mut sparks: Vec<Spark> = vec![];
    let mut confirm_leave = false;
    let mut help = false;
    loop {
        let scale = (screen_width() / W).min(screen_height() / H);
        let offset = vec2(
            (screen_width() - W * scale) / 2.,
            (screen_height() - H * scale) / 2.,
        );
        let (mx, my) = mouse_position();
        let mouse = (vec2(mx, my) - offset) / scale;
        clear_background(BG);
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., W, H));
        // Screen rendering inverts Y, so use positive zoom for top-left UI coordinates.
        camera.zoom.y = 2. / H;
        camera.viewport = Some((
            offset.x as i32,
            offset.y as i32,
            (W * scale) as i32,
            (H * scale) as i32,
        ));
        set_camera(&camera);
        let time = get_time() as f32;
        for i in 0..75 {
            let x = ((i * 173 + 47) % 1200) as f32;
            let y = ((i * 113 + 31) % 800) as f32;
            draw_circle(
                x,
                y,
                if i % 4 == 0 { 1.6 } else { 0.8 },
                Color::new(
                    0.42,
                    0.57,
                    0.8,
                    0.15 + 0.15 * (time * 0.6 + i as f32).sin().abs(),
                ),
            );
        }
        draw_circle_lines(1120., 170., 230., 1., Color::new(0.17, 0.23, 0.36, 0.5));
        draw_circle_lines(1120., 170., 265., 1., Color::new(0.17, 0.23, 0.36, 0.3));
        icon(2, 53., 40., 0.45, MINT);
        text("QUEST LAB", 80., 48., 27., INK);
        text("CURIOUS MINDS. BIG ADVENTURES.", 260., 46., 16., MUTED);
        let level = progress.xp / 200 + 1;
        text(&format!("LEVEL {level}"), 858., 35., 18., MINT);
        text(&format!("{} XP", progress.xp), 1040., 35., 18., INK);
        draw_rectangle(858., 47., 270., 5., PANEL);
        draw_rectangle(
            858.,
            47.,
            270. * (progress.xp % 200) as f32 / 200.,
            5.,
            MINT,
        );
        let blocked = help || confirm_leave;
        let pointer = if blocked { vec2(-1000., -1000.) } else { mouse };
        match screen {
            Screen::Home => {
                text("YOUR NEXT GREAT DISCOVERY", 64., 112., 18., MINT);
                text("Small questions.", 64., 177., 58., INK);
                text("Galaxy-sized adventures.", 64., 238., 58., INK);
                text(
                    "Play, learn, and collect badges. Your mission starts here.",
                    66.,
                    280.,
                    24.,
                    MUTED,
                );
                robot(1000., 197., time);
                if button("My badges", Rect::new(66., 306., 170., 48.), PANEL, pointer) {
                    screen = Screen::Badges;
                }
                text(
                    &format!(
                        "{} / 8 unlocked",
                        progress.badges().iter().filter(|&&b| b).count()
                    ),
                    255.,
                    337.,
                    21.,
                    GOLD,
                );
                text("CHOOSE YOUR WORLD", 64., 411., 18., MUTED);
                for world in 0..3 {
                    let x = 64. + world as f32 * 365.;
                    let r = Rect::new(x, 434., 342., 250.);
                    panel(r, PANEL);
                    draw_rectangle(x + 20., 451., 40., 3., ACCENTS[world]);
                    icon(world, x + 49., 500., 0.65, ACCENTS[world]);
                    text(&format!("0{}", world + 1), x + 280., 505., 24., MUTED);
                    text(NAMES[world], x + 22., 553., 30., INK);
                    text(
                        [
                            "Games, pixels & clever strategies",
                            "Code, robots & bright ideas",
                            "Space, nature & experiments",
                        ][world],
                        x + 22.,
                        584.,
                        19.,
                        MUTED,
                    );
                    if button(
                        "Start mission  >",
                        Rect::new(x + 20., 609., 302., 50.),
                        ACCENTS[world],
                        pointer,
                    ) {
                        mission = Mission::new(world);
                        before_badges = progress.badges();
                        screen = Screen::Quiz;
                    }
                }
                text(
                    "8 questions per mission   /   No timer   /   Every try earns XP",
                    64.,
                    724.,
                    21.,
                    MUTED,
                );
                if button(
                    "How to play",
                    Rect::new(949., 703., 185., 42.),
                    PANEL,
                    pointer,
                ) {
                    help = true;
                }
            }
            Screen::Quiz => {
                if button("< Home", Rect::new(64., 88., 122., 40.), PANEL, pointer)
                    || (!blocked && is_key_pressed(KeyCode::Escape))
                {
                    confirm_leave = true;
                }
                text(
                    NAMES[mission.world],
                    209.,
                    116.,
                    24.,
                    ACCENTS[mission.world],
                );
                text(
                    &format!("QUESTION {} / 8", mission.index + 1),
                    951.,
                    115.,
                    20.,
                    MUTED,
                );
                for i in 0..8 {
                    panel(
                        Rect::new(64. + i as f32 * 136., 147., 124., 6.),
                        if i < mission.index {
                            MINT
                        } else if i == mission.index {
                            ACCENTS[mission.world]
                        } else {
                            PANEL
                        },
                    );
                }
                let q = &QUESTIONS[mission.deck[mission.index]];
                panel(Rect::new(64., 185., 1072., 155.), PANEL);
                text(
                    "THINK IT THROUGH. YOU'VE GOT THIS.",
                    89.,
                    219.,
                    16.,
                    ACCENTS[mission.world],
                );
                wrap(q.prompt, 89., 266., 1000., 34., INK);
                let keys = [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
                for (i, key) in keys.iter().enumerate() {
                    let x = 64. + (i % 2) as f32 * 548.;
                    let y = 364. + (i / 2) as f32 * 92.;
                    let r = Rect::new(x, y, 524., 76.);
                    let revealed = mission.selected.is_some();
                    let right = mission.choices[i] == 0;
                    let c = if revealed && right {
                        Color::new(0.10, 0.29, 0.25, 1.)
                    } else if mission.selected == Some(i) {
                        Color::new(0.28, 0.16, 0.24, 1.)
                    } else if r.contains(pointer) && !revealed {
                        Color::new(0.13, 0.18, 0.29, 1.)
                    } else {
                        PANEL
                    };
                    panel(r, c);
                    draw_circle(
                        x + 32.,
                        y + 38.,
                        17.,
                        if revealed && right { MINT } else { BG },
                    );
                    text(
                        &(i + 1).to_string(),
                        x + 26.,
                        y + 45.,
                        22.,
                        if revealed && right { BG } else { MUTED },
                    );
                    text(q.answers[mission.choices[i]], x + 60., y + 46., 24., INK);
                    if !blocked
                        && !revealed
                        && ((r.contains(pointer) && is_mouse_button_pressed(MouseButton::Left))
                            || is_key_pressed(*key))
                    {
                        if mission.answer(i, &mut progress) {
                            burst(&mut sparks, vec2(x + 260., y + 30.));
                        }
                        save_error = save(&progress).is_err();
                    }
                }
                if let Some(selected) = mission.selected {
                    let right = mission.choices[selected] == 0;
                    text(
                        if right {
                            "Nice discovery!  +25 XP"
                        } else {
                            "Keep exploring!  +5 XP for trying"
                        },
                        88.,
                        580.,
                        27.,
                        if right { MINT } else { GOLD },
                    );
                    wrap(q.explanation, 88., 617., 990., 23., INK);
                    if button(
                        if mission.index == 7 {
                            "Mission complete  >"
                        } else {
                            "Next question  >"
                        },
                        Rect::new(844., 696., 292., 56.),
                        MINT,
                        pointer,
                    ) || (!blocked && is_key_pressed(KeyCode::Enter))
                    {
                        if mission.index == 7 {
                            progress.missions[mission.world] += 1;
                            progress.xp += 50;
                            mission.earned += 50;
                            if mission.correct == 8 {
                                progress.perfect += 1;
                            }
                            mission.new_badges = progress
                                .badges()
                                .iter()
                                .enumerate()
                                .filter(|(i, b)| **b && !before_badges[*i])
                                .map(|(i, _)| i)
                                .collect();
                            save_error = save(&progress).is_err();
                            screen = Screen::Result;
                            burst(&mut sparks, vec2(600., 260.));
                        } else {
                            mission.index += 1;
                            mission.selected = None;
                            shuffle(&mut mission.choices);
                        }
                    }
                    text("Enter to continue", 650., 731., 18., MUTED);
                } else {
                    text(
                        "Click an answer or press 1, 2, 3, 4. Take all the time you need.",
                        64.,
                        583.,
                        22.,
                        MUTED,
                    );
                }
                text(
                    &format!(
                        "{} correct  /  {} in a row",
                        mission.correct, mission.streak
                    ),
                    64.,
                    731.,
                    20.,
                    MUTED,
                );
            }
            Screen::Result => {
                text("MISSION COMPLETE", 64., 117., 19., MINT);
                text(
                    if mission.correct >= 6 {
                        "Out-of-this-world thinking!"
                    } else {
                        "Every discovery counts!"
                    },
                    64.,
                    182.,
                    50.,
                    INK,
                );
                text(
                    "New knowledge unlocked. Where will your curiosity take you next?",
                    66.,
                    225.,
                    24.,
                    MUTED,
                );
                for (i, (value, label)) in [
                    (format!("{} / 8", mission.correct), "CORRECT ANSWERS"),
                    (format!("+{}", mission.earned), "XP EARNED"),
                    (level.to_string(), "EXPLORER LEVEL"),
                ]
                .iter()
                .enumerate()
                {
                    let x = 64. + i as f32 * 365.;
                    panel(Rect::new(x, 263., 342., 132.), PANEL);
                    text(value, x + 25., 327., 40. + 6., ACCENTS[i]);
                    text(label, x + 25., 366., 18., MUTED);
                }
                text(
                    "Includes +50 XP for finishing your mission!",
                    66.,
                    432.,
                    22.,
                    MINT,
                );
                panel(Rect::new(64., 462., 1072., 174.), PANEL);
                if mission.new_badges.is_empty() {
                    icon(2, 126., 543., 0.9, GOLD);
                    text("Your next badge is waiting", 191., 522., 29., INK);
                    wrap("Try another world, build a streak of 3, or collect 8 correct answers in a topic. Every mission helps you grow.",191.,560.,870.,23.,MUTED);
                } else {
                    text("NEW BADGES EARNED", 89., 496., 18., GOLD);
                    for (slot, &i) in mission.new_badges.iter().enumerate() {
                        let x = 124. + slot as f32 * 205.;
                        badge(i, x, 548., true);
                        text(BADGES[i].0, x - 40., 612., 19., INK);
                    }
                }
                if button(
                    "Choose a world",
                    Rect::new(64., 674., 250., 58.),
                    MINT,
                    pointer,
                ) {
                    screen = Screen::Home;
                }
                if button(
                    "Play again",
                    Rect::new(336., 674., 230., 58.),
                    PANEL,
                    pointer,
                ) {
                    mission = Mission::new(mission.world);
                    before_badges = progress.badges();
                    screen = Screen::Quiz;
                }
                if button(
                    "View badges",
                    Rect::new(588., 674., 230., 58.),
                    PANEL,
                    pointer,
                ) {
                    screen = Screen::Badges;
                }
            }
            Screen::Badges => {
                text("THE DISCOVERY COLLECTION", 64., 115., 18., GOLD);
                text("Big ideas. Brilliant badges.", 64., 176., 50., INK);
                text(
                    "Your hard-earned discoveries, saved for your next adventure.",
                    64.,
                    217.,
                    23.,
                    MUTED,
                );
                let unlocked = progress.badges();
                for i in 0..8 {
                    let x = 64. + (i % 4) as f32 * 274.;
                    let y = 252. + (i / 4) as f32 * 205.;
                    panel(Rect::new(x, y, 250., 187.), PANEL);
                    badge(i, x + 125., y + 50. + 5., unlocked[i]);
                    text(
                        BADGES[i].0,
                        x + 18.,
                        y + 115.,
                        25.,
                        if unlocked[i] { INK } else { MUTED },
                    );
                    wrap(BADGES[i].1, x + 18., y + 146., 218., 18., MUTED);
                    if unlocked[i] {
                        draw_circle(x + 225., y + 21., 4., MINT);
                    }
                }
                if button(
                    "< Back to worlds",
                    Rect::new(64., 695., 250., 50.),
                    MINT,
                    pointer,
                ) || (!blocked && is_key_pressed(KeyCode::Escape))
                {
                    screen = Screen::Home;
                }
                text(
                    &format!(
                        "{} of 8 badges unlocked",
                        unlocked.iter().filter(|&&b| b).count()
                    ),
                    823.,
                    727.,
                    24.,
                    GOLD,
                );
            }
        }
        let dt = get_frame_time().min(0.05);
        for s in &mut sparks {
            s.life -= dt;
            s.p += s.v * dt;
            s.v.y += 400. * dt;
            draw_rectangle(
                s.p.x,
                s.p.y,
                5.,
                8.,
                Color::new(s.color.r, s.color.g, s.color.b, s.life.min(1.)),
            );
        }
        sparks.retain(|s| s.life > 0.);
        text(
            if save_error {
                "Progress could not be loaded or saved. Check your save-folder permissions."
            } else {
                "LOCAL PROGRESS SAVED  /  MADE FOR CURIOUS EXPLORERS"
            },
            64.,
            783.,
            15.,
            if save_error { GOLD } else { MUTED },
        );
        if help || confirm_leave {
            draw_rectangle(0., 0., W, H, Color::new(0., 0., 0., 0.8));
            panel(Rect::new(300., 230., 600., 330.), PANEL);
            text(
                if help {
                    "Welcome, explorer!"
                } else {
                    "Head back to your worlds?"
                },
                332.,
                283.,
                30. + 4.,
                INK,
            );
            wrap(
                if help {
                    "Choose a world and answer 8 questions. Click an answer or use keys 1-4. Press Enter after reading the explanation. Earn 25 XP for a correct answer, 5 XP for trying, and 50 XP for finishing. Each 200 XP gains a level. Collect all 8 badges!"
                } else {
                    "The XP and answers you have earned are saved. This mission will end early, so you will miss its completion bonus."
                },
                332.,
                327.,
                530.,
                24.,
                MUTED,
            );
            if button(
                if help {
                    "Let's explore!"
                } else {
                    "Keep playing"
                },
                Rect::new(332., 477., 250., 50.),
                MINT,
                mouse,
            ) || is_key_pressed(KeyCode::Escape)
            {
                help = false;
                confirm_leave = false;
            }
            if confirm_leave
                && button(
                    "Leave mission",
                    Rect::new(602., 477., 266., 50.),
                    PANEL,
                    mouse,
                )
            {
                confirm_leave = false;
                screen = Screen::Home;
            }
        }
        set_default_camera();
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn progress_round_trip_and_invalid_data() {
        let p = Progress {
            xp: 500,
            correct: [8, 3, 6],
            missions: [1, 2, 0],
            best_streak: 4,
            perfect: 1,
        };
        assert_eq!(Progress::decode(&p.encode()), Some(p));
        for bad in [
            "",
            "QUESTLAB1 1 2",
            "QUESTLAB1 -1 0 0 0 0 0 0 0 0",
            "UNKNOWN 0 0 0 0 0 0 0 0 0",
        ] {
            assert!(Progress::decode(bad).is_none());
        }
    }
    #[test]
    fn badge_thresholds() {
        assert_eq!(Progress::default().badges(), [false; 8]);
        let p = Progress {
            xp: 1000,
            correct: [8; 3],
            missions: [1; 3],
            best_streak: 3,
            perfect: 1,
        };
        assert_eq!(p.badges(), [true; 8]);
    }
    #[test]
    fn mission_has_unique_questions_and_awards_once() {
        for world in 0..3 {
            let mut m = Mission::new(world);
            let mut p = Progress::default();
            let mut ids = m.deck.clone();
            ids.sort();
            ids.dedup();
            assert_eq!(ids.len(), 8);
            assert!(m.deck.iter().all(|&i| QUESTIONS[i].world == world));
            let choice = m.choices.iter().position(|&c| c == 0).unwrap();
            assert!(m.answer(choice, &mut p));
            assert!(!m.answer(choice, &mut p));
            assert_eq!(p.xp, 25);
            assert_eq!(p.correct[world], 1);
        }
    }
    #[test]
    fn question_bank_has_unique_answers_and_balanced_worlds() {
        for world in 0..3 {
            assert_eq!(QUESTIONS.iter().filter(|q| q.world == world).count(), 16);
        }
        for q in QUESTIONS {
            let mut a = q.answers;
            a.sort();
            assert!(a.windows(2).all(|w| w[0] != w[1]));
        }
    }
}
