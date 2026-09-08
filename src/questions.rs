pub struct Question {
    pub world: usize,
    pub prompt: &'static str,
    pub answers: [&'static str; 4],
    pub explanation: &'static str,
}

// The first answer is correct here; the game shuffles its position each mission.
macro_rules! q {
    ($w:expr, $p:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        Question {
            world: $w,
            prompt: $p,
            answers: [$a, $b, $c, $d],
            explanation: $e,
        }
    };
}

pub const QUESTIONS: &[Question] = &[
    q!(0, "What does an NPC mean in a video game?", "Non-player character", "New power crystal", "Next player challenge", "Network power cable", "An NPC is a character controlled by the game, like a shopkeeper or a quest guide."),
    q!(0, "What is a pixel?", "A tiny dot of an image", "A game controller", "A kind of battery", "A secret level", "Screen images are made of tiny colored dots called pixels. Many dots together make a picture!"),
    q!(0, "In Minecraft, which tool is best for mining stone?", "A pickaxe", "A fishing rod", "A shovel", "Shears", "Pickaxes mine stone and ores. Different tools are useful for different jobs."),
    q!(0, "What does a checkpoint usually do?", "Saves a place to restart", "Turns off the screen", "Deletes your character", "Changes your name", "Checkpoints let you try again from a nearby spot instead of starting the whole level over."),
    q!(0, "What is a cooperative game?", "Players work together", "Nobody can win", "Only robots can play", "Every player plays alone", "Co-op players help each other reach a shared goal. Teamwork is a superpower!"),
    q!(0, "What does a game designer plan?", "Rules and challenges", "The weather outside", "A computer's electricity", "The size of your desk", "Game designers plan how a game works, including its rules, goals, and challenges."),
    q!(0, "What is a game bug?", "A mistake in the software", "An insect in every console", "A special controller", "A perfect high score", "A software bug makes a game behave in an unexpected way. Testing helps developers find bugs."),
    q!(0, "Which habit helps you improve at a tricky level?", "Practice and try new ideas", "Blame your teammates", "Never take a break", "Repeat without thinking", "Trying different strategies helps your brain learn. A break can help you return with fresh ideas."),
    q!(0, "What does a mini-map usually show?", "Your nearby surroundings", "Your real home address", "Your controller battery", "The game's source code", "A mini-map is a small map that helps you find paths, teammates, or important places."),
    q!(0, "What is an avatar?", "Your character or image", "A type of screen cable", "A game's price", "A computer fan", "An avatar represents you in a game or online space. You can often customize its look."),
    q!(0, "What is the job of a game playtester?", "Try the game and find issues", "Sell real spaceships", "Build all the monitors", "Choose your passwords", "Playtesters try games and share feedback so developers can improve the experience."),
    q!(0, "What makes a game a puzzle game?", "Solving problems is central", "It must have racing cars", "It can never have a story", "It only works on phones", "Puzzle games challenge you to spot patterns, use logic, and find solutions."),
    q!(0, "What is a good response when a teammate makes a mistake?", "Encourage them to try again", "Send an unkind message", "Share their private details", "Say they can never improve", "Kind teammates make games more fun. Everyone makes mistakes while learning."),
    q!(0, "What does '3D' stand for?", "Three dimensions", "Three downloads", "Three difficulty levels", "Three digital dragons", "Three-dimensional objects have width, height, and depth, like a cube."),
    q!(0, "Which game feature helps people read dialogue?", "Subtitles", "Motion blur", "A higher score", "A loading spinner", "Subtitles show speech as text. They help many players understand what's happening."),
    q!(0, "Why do games use a tutorial?", "To teach you how to play", "To erase your progress", "To hide all the buttons", "To stop you exploring", "Tutorials introduce the controls and rules so you can learn a game step by step."),
    q!(1, "Which part is often called the brain of a computer?", "CPU", "Keyboard", "Speaker", "Mouse pad", "The central processing unit, or CPU, follows instructions and does calculations."),
    q!(1, "What is an algorithm?", "Steps for solving a problem", "A computer's color", "A kind of screen", "A robot's shoe", "An algorithm is a set of steps. A recipe is a great everyday example!"),
    q!(1, "Which two digits does binary use?", "0 and 1", "2 and 3", "5 and 9", "1 and 9", "Computers represent data using bits. Each bit has one of two values: 0 or 1."),
    q!(1, "What does a keyboard let you do?", "Enter letters and commands", "Print on paper", "Cool the computer", "Store solar energy", "A keyboard is an input device: it sends information into a computer."),
    q!(1, "What should you do if a stranger asks for your password?", "Tell a trusted adult", "Send it right away", "Post it in game chat", "Trade it for game coins", "Keep passwords private. A trusted adult can help when an online message feels strange."),
    q!(1, "What does a loop do in a program?", "Repeats instructions", "Breaks the screen", "Makes the computer round", "Deletes every file", "Loops repeat a set of instructions, such as moving every enemy one step each frame."),
    q!(1, "Which device makes a physical object from a digital design?", "A 3D printer", "A microphone", "A webcam", "A router", "Many 3D printers build objects one thin layer at a time from a digital model."),
    q!(1, "What does Wi-Fi help devices do?", "Connect without a cable", "Create drinking water", "Work without any power", "Print without a printer", "Wi-Fi uses radio waves to connect devices to a network without wires."),
    q!(1, "What is debugging?", "Finding and fixing code mistakes", "Cleaning insects off a desk", "Painting a computer", "Adding more passwords", "Programmers debug by investigating what went wrong and testing a fix."),
    q!(1, "Which part stores files even when a computer is off?", "An SSD", "A cooling fan", "A microphone", "A power button", "A solid-state drive stores files long-term. RAM is working memory that normally needs power."),
    q!(1, "What does a robot's sensor do?", "Detects things around it", "Writes every story", "Makes unlimited energy", "Always gives it feelings", "Sensors can detect light, distance, temperature, and more. They help robots respond."),
    q!(1, "What is a variable in a program?", "A named place for a value", "A broken cable", "A kind of chair", "A screen's frame", "A variable stores a value, such as a player's score, that a program can use or change."),
    q!(1, "Which is an output device?", "A speaker", "A mouse", "A keyboard", "A microphone", "Output devices send information out of a computer. Speakers turn audio data into sound."),
    q!(1, "What is the internet?", "A worldwide network of networks", "One giant keyboard", "A single video game", "A battery inside a phone", "The internet connects computer networks around the world so they can exchange information."),
    q!(1, "What does a solar panel turn into electricity?", "Sunlight", "Plastic", "Sound from headphones", "Wi-Fi passwords", "Solar cells convert energy from light into electrical energy."),
    q!(1, "Why should you check a surprising claim online?", "Online information can be wrong", "All websites are always right", "Pictures can never be changed", "Popular posts are always facts", "Compare reliable sources and ask a trusted adult. Even convincing posts can contain mistakes."),
    q!(2, "Which planet is known as the Red Planet?", "Mars", "Venus", "Neptune", "Saturn", "Rust-like iron minerals in its soil give Mars its reddish color."),
    q!(2, "What force keeps us on the ground?", "Gravity", "Magnetism", "Friction alone", "Sound", "Gravity pulls objects with mass toward each other. Earth's gravity pulls us toward its center."),
    q!(2, "What do plants use sunlight to help make?", "Their own food", "Plastic", "Rocks", "Metal", "Plants use light, water, and carbon dioxide to make sugars through photosynthesis."),
    q!(2, "What is water called in its solid form?", "Ice", "Steam", "Fog", "Rain", "When liquid water freezes, it becomes solid ice."),
    q!(2, "Which animal is a mammal?", "Dolphin", "Shark", "Octopus", "Trout", "Dolphins breathe air and feed their babies milk. Those are mammal traits!"),
    q!(2, "What is at the center of our solar system?", "The Sun", "Earth", "The Moon", "Mars", "The Sun is our star. The planets travel around it in paths called orbits."),
    q!(2, "What tool helps us see tiny cells?", "A microscope", "A telescope", "A compass", "A thermometer", "Microscopes magnify tiny things. Telescopes help us see objects that are far away."),
    q!(2, "Which gas do we need from the air to stay alive?", "Oxygen", "Helium", "Neon", "Hydrogen", "Our bodies use oxygen to release energy from food. Our lungs take it from the air."),
    q!(2, "Why do we see lightning before hearing thunder?", "Light travels faster than sound", "Thunder happens a day later", "Our ears turn off in storms", "Lightning makes no sound", "Light travels much faster than sound, so the flash reaches you before the rumble."),
    q!(2, "How many legs does an insect have?", "Six", "Four", "Eight", "Ten", "Insects have six legs and three main body sections. Spiders have eight legs and are arachnids."),
    q!(2, "What causes day and night on Earth?", "Earth spinning", "The Sun switching off", "The Moon covering us daily", "Clouds covering all the land", "As Earth rotates, your part faces toward the Sun for day and away from it for night."),
    q!(2, "Which material is usually attracted to a magnet?", "Iron", "Wood", "Glass", "Paper", "Magnets attract iron and some other materials. They do not attract every kind of metal."),
    q!(2, "What is evaporation?", "Liquid changing into gas", "Ice turning into rock", "Gas changing into metal", "Light changing into water", "When water evaporates, it becomes water vapor in the air. This helps puddles dry up."),
    q!(2, "What should you change in a fair experiment?", "One thing you are testing", "Everything at once", "Only the answer afterward", "Nothing, ever", "Changing one factor while keeping others the same helps you understand what caused a result."),
    q!(2, "Which body part pumps blood?", "Heart", "Lungs", "Stomach", "Brain", "Your heart is a muscle that pumps blood, carrying oxygen and nutrients around your body."),
    q!(2, "What is the Moon's light mostly?", "Reflected sunlight", "Light from a giant battery", "Its own fire", "Light from Earth's lamps", "The Moon reflects sunlight. It does not make its own visible light like the Sun does."),
];
