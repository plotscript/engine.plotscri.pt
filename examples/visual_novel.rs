//! Example: Love & Mystery - A Visual Novel
//! 
//! This example demonstrates a complete visual novel game with:
//! - Multiple scenes with backgrounds and character sprites
//! - Branching dialogue with choices
//! - Character relationship tracking
//! - Multiple endings based on choices
//! - Save/load functionality

use plotscript::{Engine, EngineConfig, GameMode, init};
use std::io::{self, Write};

const GAME_SCRIPT: &str = r#"
VisualNovel((
    title: "Love & Mystery",
    author: "PlotScript Examples",
    description: Some("A tale of romance and intrigue at Sakura Academy"),
    version: Some("1.0.0"),
    
    settings: (
        auto_mode_available: true,
        skip_mode_available: true,
        text_speed: Medium,
        character_portraits: true,
        voice_acting: false,
    ),
    
    starting_scene: "intro",
    
    scenes: {
        "intro": (
            name: "Introduction",
            background: Some("school_gate.jpg"),
            music: Some("theme.mp3"),
            characters: [],
            dialogue: [
                (
                    speaker: None,
                    text: "Spring has arrived at Sakura Academy...",
                    sprite: None,
                    position: None,
                    effect: Some(FadeIn),
                ),
                (
                    speaker: None,
                    text: "You've just transferred here, hoping for a fresh start.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: Some("???"),
                    text: "Hey! You must be the new student!",
                    sprite: Some("yuki_happy.png"),
                    position: Some(Center),
                    effect: Some(SlideIn),
                ),
            ],
            choices: Some([
                (
                    text: "Yes, I just arrived today.",
                    next_scene: "meet_yuki",
                    conditions: None,
                    effects: Some([(Variable, "yuki_points", Add, 1)]),
                ),
                (
                    text: "Who are you?",
                    next_scene: "meet_yuki",
                    conditions: None,
                    effects: Some([(Variable, "yuki_points", Add, 0)]),
                ),
            ]),
            next_scene: None,
        ),
        "meet_yuki": (
            name: "Meeting Yuki",
            background: Some("school_gate.jpg"),
            music: Some("theme.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "I'm Yuki Tanaka! I'm in class 2-B. Nice to meet you!",
                    sprite: Some("yuki_smile.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "I can show you around if you'd like. The school can be confusing at first.",
                    sprite: Some("yuki_happy.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "That would be great, thank you!",
                    next_scene: "school_tour",
                    conditions: None,
                    effects: Some([(Variable, "yuki_points", Add, 2)]),
                ),
                (
                    text: "I think I can manage on my own.",
                    next_scene: "explore_alone",
                    conditions: None,
                    effects: Some([(Variable, "independent", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "school_tour": (
            name: "School Tour",
            background: Some("hallway.jpg"),
            music: Some("cheerful.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "This is the main hallway. Your classroom is down this way.",
                    sprite: Some("yuki_happy.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "Oh! And over there is the library. It's my favorite place.",
                    sprite: Some("yuki_smile.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "As Yuki shows you around, you notice a mysterious figure watching from the shadows...",
                    sprite: Some("shadow_figure.png"),
                    position: Some(Left),
                    effect: Some(FadeIn),
                ),
            ],
            choices: Some([
                (
                    text: "Who is that person?",
                    next_scene: "mysterious_encounter",
                    conditions: None,
                    effects: Some([(Variable, "noticed_shadow", Set, 1)]),
                ),
                (
                    text: "Tell me more about the library.",
                    next_scene: "library_talk",
                    conditions: None,
                    effects: Some([(Variable, "yuki_points", Add, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "explore_alone": (
            name: "Solo Exploration",
            background: Some("hallway.jpg"),
            music: Some("mysterious.mp3"),
            characters: [],
            dialogue: [
                (
                    speaker: None,
                    text: "You decide to explore the school on your own.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "The hallways seem endless, and you quickly realize you're lost.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: Some("???"),
                    text: "Lost already? How predictable.",
                    sprite: Some("ryo_smirk.png"),
                    position: Some(Left),
                    effect: Some(SlideIn),
                ),
            ],
            choices: Some([
                (
                    text: "Who are you?",
                    next_scene: "meet_ryo",
                    conditions: None,
                    effects: Some([(Variable, "ryo_points", Add, 1)]),
                ),
                (
                    text: "I'm not lost!",
                    next_scene: "meet_ryo",
                    conditions: None,
                    effects: Some([(Variable, "defiant", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "mysterious_encounter": (
            name: "The Mysterious Student",
            background: Some("hallway.jpg"),
            music: Some("tension.mp3"),
            characters: ["yuki", "ryo"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "Oh... that's Ryo Kuroda. He's... complicated.",
                    sprite: Some("yuki_worried.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Talking about me, Tanaka? How bold of you.",
                    sprite: Some("ryo_serious.png"),
                    position: Some(Left),
                    effect: Some(SlideIn),
                ),
                (
                    speaker: Some("Ryo"),
                    text: "And you must be the new student everyone's talking about.",
                    sprite: Some("ryo_smirk.png"),
                    position: Some(Left),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "Nice to meet you, Ryo.",
                    next_scene: "tension_rises",
                    conditions: None,
                    effects: Some([(Variable, "ryo_points", Add, 2)]),
                ),
                (
                    text: "What do you want?",
                    next_scene: "tension_rises",
                    conditions: None,
                    effects: Some([(Variable, "confrontational", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "library_talk": (
            name: "Library Discussion",
            background: Some("library.jpg"),
            music: Some("peaceful.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "The library has books on everything! History, science, even old legends about the school.",
                    sprite: Some("yuki_excited.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "There's a rumor about a hidden room somewhere in the school...",
                    sprite: Some("yuki_whisper.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "Tell me more about this hidden room!",
                    next_scene: "hidden_room_story",
                    conditions: None,
                    effects: Some([(Variable, "mystery_interest", Set, 1)]),
                ),
                (
                    text: "Do you come here often?",
                    next_scene: "yuki_backstory",
                    conditions: None,
                    effects: Some([(Variable, "yuki_points", Add, 2)]),
                ),
            ]),
            next_scene: None,
        ),
        "meet_ryo": (
            name: "Meeting Ryo",
            background: Some("hallway.jpg"),
            music: Some("mysterious.mp3"),
            characters: ["ryo"],
            dialogue: [
                (
                    speaker: Some("Ryo"),
                    text: "I'm Ryo Kuroda. Student council president and keeper of this school's secrets.",
                    sprite: Some("ryo_serious.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "You'd do well to be careful here. Not everything is as it seems.",
                    sprite: Some("ryo_mysterious.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "What do you mean by that?",
                    next_scene: "ryo_warning",
                    conditions: None,
                    effects: Some([(Variable, "ryo_points", Add, 1)]),
                ),
                (
                    text: "Are you threatening me?",
                    next_scene: "ryo_warning",
                    conditions: None,
                    effects: Some([(Variable, "suspicious", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "tension_rises": (
            name: "Rising Tensions",
            background: Some("hallway.jpg"),
            music: Some("tension.mp3"),
            characters: ["yuki", "ryo"],
            dialogue: [
                (
                    speaker: Some("Ryo"),
                    text: "Interesting. You're not like the other students.",
                    sprite: Some("ryo_interested.png"),
                    position: Some(Left),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "Leave them alone, Ryo. They just got here.",
                    sprite: Some("yuki_angry.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Always so protective, Tanaka. But can you protect them from everything?",
                    sprite: Some("ryo_smirk.png"),
                    position: Some(Left),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "What's going on between you two?",
                    next_scene: "revelation",
                    conditions: None,
                    effects: Some([(Variable, "curious", Set, 1)]),
                ),
                (
                    text: "I don't need protection.",
                    next_scene: "show_strength",
                    conditions: None,
                    effects: Some([
                        (Variable, "ryo_points", Add, 2),
                        (Variable, "independent", Set, 1),
                    ]),
                ),
            ]),
            next_scene: None,
        ),
        "hidden_room_story": (
            name: "The Legend",
            background: Some("library.jpg"),
            music: Some("mysterious.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "They say the founder of the school hid something precious here.",
                    sprite: Some("yuki_serious.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "Some believe it's a treasure, others think it's something more... supernatural.",
                    sprite: Some("yuki_whisper.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "Want to help me look for it?",
                    sprite: Some("yuki_excited.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "Absolutely! Let's solve this mystery together!",
                    next_scene: "mystery_route",
                    conditions: None,
                    effects: Some([
                        (Variable, "yuki_points", Add, 3),
                        (Variable, "route", Set, 1), // Yuki route
                    ]),
                ),
                (
                    text: "Maybe we should focus on studying instead...",
                    next_scene: "normal_day",
                    conditions: None,
                    effects: Some([(Variable, "practical", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "yuki_backstory": (
            name: "Getting to Know Yuki",
            background: Some("library.jpg"),
            music: Some("emotional.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "I... I love books. They're like windows to other worlds.",
                    sprite: Some("yuki_shy.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "I used to be very shy. Books were my only friends for a long time.",
                    sprite: Some("yuki_sad.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "But now... I'm glad I met you.",
                    sprite: Some("yuki_blush.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "I'm glad I met you too, Yuki.",
                    next_scene: "romantic_moment",
                    conditions: None,
                    effects: Some([
                        (Variable, "yuki_points", Add, 5),
                        (Variable, "route", Set, 1), // Yuki route
                    ]),
                ),
                (
                    text: "Books are great companions.",
                    next_scene: "normal_day",
                    conditions: None,
                    effects: Some([(Variable, "yuki_points", Add, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "ryo_warning": (
            name: "Ryo's Warning",
            background: Some("hallway.jpg"),
            music: Some("dark.mp3"),
            characters: ["ryo"],
            dialogue: [
                (
                    speaker: Some("Ryo"),
                    text: "This school has a dark history. Things happen to curious students.",
                    sprite: Some("ryo_dark.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "But perhaps... you're different. Perhaps you're exactly what this place needs.",
                    sprite: Some("ryo_mysterious.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "Tell me everything.",
                    next_scene: "ryo_route",
                    conditions: None,
                    effects: Some([
                        (Variable, "ryo_points", Add, 3),
                        (Variable, "route", Set, 2), // Ryo route
                    ]),
                ),
                (
                    text: "I should go to class.",
                    next_scene: "normal_day",
                    conditions: None,
                    effects: Some([(Variable, "cautious", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "revelation": (
            name: "The Truth Emerges",
            background: Some("rooftop.jpg"),
            music: Some("revelation.mp3"),
            characters: ["yuki", "ryo"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "We used to be friends... before everything changed.",
                    sprite: Some("yuki_sad.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Friends? We were investigating the school's mysteries together.",
                    sprite: Some("ryo_serious.png"),
                    position: Some(Left),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Until you got scared and abandoned me.",
                    sprite: Some("ryo_angry.png"),
                    position: Some(Left),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "I was trying to protect you!",
                    sprite: Some("yuki_crying.png"),
                    position: Some(Right),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "We should work together to solve this.",
                    next_scene: "true_ending_path",
                    conditions: Some([
                        (Variable, "yuki_points", GreaterThan, 5),
                        (Variable, "ryo_points", GreaterThan, 3),
                    ]),
                    effects: Some([(Variable, "united", Set, 1)]),
                ),
                (
                    text: "You both need to move on.",
                    next_scene: "neutral_ending_path",
                    conditions: None,
                    effects: Some([(Variable, "mediator", Set, 1)]),
                ),
            ]),
            next_scene: None,
        ),
        "show_strength": (
            name: "Standing Strong",
            background: Some("hallway.jpg"),
            music: Some("determination.mp3"),
            characters: ["ryo"],
            dialogue: [
                (
                    speaker: Some("Ryo"),
                    text: "Impressive. Most students cower at the first sign of conflict.",
                    sprite: Some("ryo_impressed.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Perhaps you're worthy of knowing the truth after all.",
                    sprite: Some("ryo_smile.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: Some([
                (
                    text: "I'm listening.",
                    next_scene: "ryo_route",
                    conditions: None,
                    effects: Some([
                        (Variable, "ryo_points", Add, 4),
                        (Variable, "route", Set, 2), // Ryo route
                    ]),
                ),
            ]),
            next_scene: None,
        ),
        "mystery_route": (
            name: "The Mystery Deepens",
            background: Some("secret_room.jpg"),
            music: Some("discovery.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "I can't believe we found it! The hidden room!",
                    sprite: Some("yuki_amazed.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "Inside, ancient symbols cover the walls, and a mysterious artifact glows softly...",
                    sprite: None,
                    position: None,
                    effect: Some(Flash),
                ),
            ],
            choices: None,
            next_scene: Some("yuki_ending"),
        ),
        "romantic_moment": (
            name: "A Moment of Connection",
            background: Some("sunset_library.jpg"),
            music: Some("romantic.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "You know... you're the first person who really understands me.",
                    sprite: Some("yuki_love.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "Would you... would you like to go to the festival with me?",
                    sprite: Some("yuki_blush.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: None,
            next_scene: Some("yuki_ending"),
        ),
        "ryo_route": (
            name: "Into the Darkness",
            background: Some("underground.jpg"),
            music: Some("ominous.mp3"),
            characters: ["ryo"],
            dialogue: [
                (
                    speaker: Some("Ryo"),
                    text: "The school was built on an ancient site. There are tunnels beneath...",
                    sprite: Some("ryo_serious.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "I've been investigating alone, but with you... we might finally uncover the truth.",
                    sprite: Some("ryo_determined.png"),
                    position: Some(Center),
                    effect: None,
                ),
            ],
            choices: None,
            next_scene: Some("ryo_ending"),
        ),
        "normal_day": (
            name: "School Life",
            background: Some("classroom.jpg"),
            music: Some("daily.mp3"),
            characters: [],
            dialogue: [
                (
                    speaker: None,
                    text: "You settle into your new school life, making friends and focusing on your studies.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "The mysteries of the school remain unsolved, but perhaps that's for the best...",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
            ],
            choices: None,
            next_scene: Some("normal_ending"),
        ),
        "true_ending_path": (
            name: "Unity",
            background: Some("shrine.jpg"),
            music: Some("epic.mp3"),
            characters: ["yuki", "ryo"],
            dialogue: [
                (
                    speaker: None,
                    text: "Working together, the three of you uncover the school's greatest secret...",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "We did it! Together!",
                    sprite: Some("yuki_happy.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Perhaps friendship isn't weakness after all.",
                    sprite: Some("ryo_smile.png"),
                    position: Some(Left),
                    effect: None,
                ),
            ],
            choices: None,
            next_scene: Some("true_ending"),
        ),
        "neutral_ending_path": (
            name: "Moving Forward",
            background: Some("school_gate.jpg"),
            music: Some("bittersweet.mp3"),
            characters: [],
            dialogue: [
                (
                    speaker: None,
                    text: "You help Yuki and Ryo find closure, though they go their separate ways.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "Life at Sakura Academy continues, quieter but perhaps more peaceful.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
            ],
            choices: None,
            next_scene: Some("neutral_ending"),
        ),
        
        // Endings
        "yuki_ending": (
            name: "Yuki's Ending - Love in the Library",
            background: Some("library_night.jpg"),
            music: Some("ending_romantic.mp3"),
            characters: ["yuki"],
            dialogue: [
                (
                    speaker: Some("Yuki"),
                    text: "Thank you for believing in me, and for sharing this adventure.",
                    sprite: Some("yuki_love.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "I love you.",
                    sprite: Some("yuki_kiss.png"),
                    position: Some(Center),
                    effect: Some(HeartEffect),
                ),
                (
                    speaker: None,
                    text: "THE END - Yuki's Route Complete",
                    sprite: None,
                    position: None,
                    effect: Some(FadeOut),
                ),
            ],
            choices: None,
            next_scene: None,
        ),
        "ryo_ending": (
            name: "Ryo's Ending - Master of Mysteries",
            background: Some("moonlit_rooftop.jpg"),
            music: Some("ending_mysterious.mp3"),
            characters: ["ryo"],
            dialogue: [
                (
                    speaker: Some("Ryo"),
                    text: "You've proven yourself worthy. Together, we'll protect this school's secrets.",
                    sprite: Some("ryo_gentle.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "And perhaps... we'll create some new mysteries of our own.",
                    sprite: Some("ryo_love.png"),
                    position: Some(Center),
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "THE END - Ryo's Route Complete",
                    sprite: None,
                    position: None,
                    effect: Some(FadeOut),
                ),
            ],
            choices: None,
            next_scene: None,
        ),
        "true_ending": (
            name: "True Ending - Bonds of Friendship",
            background: Some("festival.jpg"),
            music: Some("ending_true.mp3"),
            characters: ["yuki", "ryo"],
            dialogue: [
                (
                    speaker: None,
                    text: "The school festival arrives, and the three of you stand together.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: Some("Yuki"),
                    text: "We make a great team!",
                    sprite: Some("yuki_happy.png"),
                    position: Some(Right),
                    effect: None,
                ),
                (
                    speaker: Some("Ryo"),
                    text: "Indeed. The future looks bright.",
                    sprite: Some("ryo_happy.png"),
                    position: Some(Left),
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "THE END - True Route Complete",
                    sprite: None,
                    position: None,
                    effect: Some(Fireworks),
                ),
            ],
            choices: None,
            next_scene: None,
        ),
        "normal_ending": (
            name: "Normal Ending - Peaceful Days",
            background: Some("graduation.jpg"),
            music: Some("ending_normal.mp3"),
            characters: [],
            dialogue: [
                (
                    speaker: None,
                    text: "Time passes peacefully at Sakura Academy.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "You graduate with good memories and lifelong friendships.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "THE END - Normal Route Complete",
                    sprite: None,
                    position: None,
                    effect: Some(FadeOut),
                ),
            ],
            choices: None,
            next_scene: None,
        ),
        "neutral_ending": (
            name: "Neutral Ending - New Beginnings",
            background: Some("spring_school.jpg"),
            music: Some("ending_neutral.mp3"),
            characters: [],
            dialogue: [
                (
                    speaker: None,
                    text: "Spring comes again to Sakura Academy.",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "New students arrive, and the cycle continues...",
                    sprite: None,
                    position: None,
                    effect: None,
                ),
                (
                    speaker: None,
                    text: "THE END - Neutral Route Complete",
                    sprite: None,
                    position: None,
                    effect: Some(FadeOut),
                ),
            ],
            choices: None,
            next_scene: None,
        ),
    },
    
    characters: {
        "yuki": (
            name: "Yuki Tanaka",
            description: "A cheerful and kind student who loves books and mysteries.",
            sprites: {
                "happy": "yuki_happy.png",
                "smile": "yuki_smile.png",
                "sad": "yuki_sad.png",
                "angry": "yuki_angry.png",
                "worried": "yuki_worried.png",
                "excited": "yuki_excited.png",
                "shy": "yuki_shy.png",
                "blush": "yuki_blush.png",
                "love": "yuki_love.png",
                "crying": "yuki_crying.png",
                "whisper": "yuki_whisper.png",
                "amazed": "yuki_amazed.png",
                "kiss": "yuki_kiss.png",
            },
            voice: None,
        ),
        "ryo": (
            name: "Ryo Kuroda",
            description: "The mysterious student council president with a dark secret.",
            sprites: {
                "serious": "ryo_serious.png",
                "smirk": "ryo_smirk.png",
                "angry": "ryo_angry.png",
                "mysterious": "ryo_mysterious.png",
                "interested": "ryo_interested.png",
                "dark": "ryo_dark.png",
                "impressed": "ryo_impressed.png",
                "smile": "ryo_smile.png",
                "determined": "ryo_determined.png",
                "gentle": "ryo_gentle.png",
                "love": "ryo_love.png",
                "happy": "ryo_happy.png",
            },
            voice: None,
        ),
    },
    
    variables: {
        "yuki_points": 0,
        "ryo_points": 0,
        "route": 0, // 0 = undecided, 1 = yuki, 2 = ryo
        "independent": 0,
        "noticed_shadow": 0,
        "defiant": 0,
        "confrontational": 0,
        "mystery_interest": 0,
        "suspicious": 0,
        "curious": 0,
        "practical": 0,
        "cautious": 0,
        "united": 0,
        "mediator": 0,
    },
    
    achievements: None,
    gallery: None,
))
"#;

fn main() {
    // Initialize the engine
    init();
    
    println!("=== Love & Mystery ===");
    println!("A PlotScript Visual Novel Example");
    println!();
    println!("Commands: save, load, skip, auto, quit");
    println!();
    
    // Create engine with visual novel configuration
    let config = EngineConfig {
        mode: GameMode::VisualNovel,
        auto_save: true,
        ..Default::default()
    };
    
    let mut engine = Engine::with_config(config);
    
    // Load the game script
    match engine.load_script(GAME_SCRIPT) {
        Ok(_) => println!("Game loaded successfully!\n"),
        Err(e) => {
            eprintln!("Failed to load game: {}", e);
            return;
        }
    }
    
    // Start the game
    match engine.start() {
        Ok(response) => {
            display_scene(&response);
        }
        Err(e) => {
            eprintln!("Failed to start game: {}", e);
            return;
        }
    }
    
    let mut auto_mode = false;
    let mut skip_mode = false;
    
    // Game loop
    loop {
        // Auto mode handling
        if auto_mode {
            std::thread::sleep(std::time::Duration::from_secs(3));
            match engine.process_input("1") {
                Ok(response) => {
                    display_scene(&response);
                    if response.ended {
                        break;
                    }
                }
                Err(_) => auto_mode = false,
            }
            continue;
        }
        
        // Skip mode handling
        if skip_mode {
            match engine.process_input("1") {
                Ok(response) => {
                    if response.ended {
                        display_scene(&response);
                        break;
                    }
                }
                Err(_) => {
                    skip_mode = false;
                    // Display current state after loading
                    match engine.process_input("") {
                        Ok(response) => display_scene(&response),
                        Err(e) => println!("Failed to get current state: {}\n", e),
                    }
                }
            }
            continue;
        }
        
        // Prompt for input
        print!("> ");
        io::stdout().flush().unwrap();
        
        // Read player input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // Handle meta commands
        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Thanks for playing!");
                break;
            }
            "save" => {
                match engine.save_game(Some(1)) {
                    Ok(_) => println!("Game saved.\n"),
                    Err(e) => println!("Failed to save: {}\n", e),
                }
                continue;
            }
            "load" => {
                match engine.load_game(Some(1)) {
                    Ok(response) => display_scene(&response),
                    Err(e) => println!("Failed to load: {}\n", e),
                }
                continue;
            }
            "auto" => {
                auto_mode = !auto_mode;
                println!("Auto mode: {}\n", if auto_mode { "ON" } else { "OFF" });
                continue;
            }
            "skip" => {
                skip_mode = !skip_mode;
                println!("Skip mode: {}\n", if skip_mode { "ON" } else { "OFF" });
                continue;
            }
            _ => {}
        }
        
        // Process game command (choice selection)
        match engine.process_input(input) {
            Ok(response) => {
                display_scene(&response);
                
                // Check if game ended
                if response.ended {
                    println!("\n=== Thank you for playing Love & Mystery! ===");
                    println!("Try different choices to unlock all endings!");
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}\n", e);
            }
        }
    }
}

fn display_scene(response: &plotscript::Response) {
    // Clear screen (simplified for example)
    println!("\n--- {} ---\n", response.location.as_deref().unwrap_or("Scene"));
    
    // Display the scene text
    println!("{}", response.text);
    
    // Display choices if available
    if !response.choices.is_empty() {
        println!("\n[Choices]");
        for (i, choice) in response.choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.text);
        }
    }
    
    println!();
}