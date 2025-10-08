//! Example: The Heist - An Interactive Fiction
//! 
//! This example demonstrates a complete interactive fiction game with:
//! - Quality-based narrative system
//! - Storylets that unlock based on conditions
//! - Character stats and inventory
//! - Timed choices and consequences
//! - Multiple paths and endings
//! - Save/load functionality

use plotscript::{Engine, EngineConfig, GameMode, init};
use std::io::{self, Write};

const GAME_SCRIPT: &str = r#"
InteractiveFiction((
    title: "The Heist",
    author: "PlotScript Examples",
    description: Some("Pull off the perfect heist... or die trying"),
    version: Some("1.0.0"),
    
    settings: (
        show_stats: true,
        timed_choices: true,
        checkpoint_saves: true,
        hidden_qualities: false,
        storylet_mode: true,
    ),
    
    starting_storylet: "the_offer",
    starting_qualities: {
        "health": 10,
        "stealth": 3,
        "tech": 2,
        "charm": 2,
        "notoriety": 0,
        "money": 100,
    },
    
    storylets: {
        "the_offer": (
            title: "An Offer You Can't Refuse",
            description: "A mysterious figure approaches you in the dive bar.",
            conditions: [],
            content: (
                text: "The woman slides a photo across the stained bar top. It shows a gleaming diamond, easily worth millions. 'The Midnight Star,' she says. 'Currently residing in the Blackwood Gallery's vault. I need someone with your... particular skills.'",
                choices: [
                    (
                        text: "I'm listening.",
                        effects: [
                            (Quality, "heist_knowledge", Set, 1),
                        ],
                        next: Some("mission_details"),
                        time_limit: None,
                    ),
                    (
                        text: "Not interested. I'm retired.",
                        effects: [
                            (Quality, "cautious", Add, 1),
                        ],
                        next: Some("persistence"),
                        time_limit: None,
                    ),
                    (
                        text: "[Charm] Name your price first.",
                        conditions: [(Quality, "charm", GreaterThan, 2)],
                        effects: [
                            (Quality, "money", Add, 500),
                            (Quality, "negotiator", Set, 1),
                        ],
                        next: Some("mission_details"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 100,
            once_only: true,
        ),
        
        "persistence": (
            title: "She Persists",
            description: "The woman isn't taking no for an answer.",
            conditions: [],
            content: (
                text: "'Retirement?' She laughs, a sound like breaking glass. 'With your debts? Your enemies? This job could set you free. One last score.' She taps the photo. 'Unless you'd prefer I share your location with certain... interested parties?'",
                choices: [
                    (
                        text: "Fine. But this is the last time.",
                        effects: [
                            (Quality, "reluctant", Set, 1),
                            (Quality, "heist_knowledge", Set, 1),
                        ],
                        next: Some("mission_details"),
                        time_limit: None,
                    ),
                    (
                        text: "Are you threatening me?",
                        effects: [
                            (Quality, "aggressive", Add, 1),
                            (Quality, "notoriety", Add, 1),
                        ],
                        next: Some("standoff"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 90,
            once_only: true,
        ),
        
        "standoff": (
            title: "Mexican Standoff",
            description: "Things are getting tense.",
            conditions: [],
            content: (
                text: "Your hand moves to your concealed weapon. Hers mirrors the gesture. The bar goes silent. 'Let's not make this messy,' she says quietly. 'You need this job. I need this thief. We can help each other, or we can paint these walls red. Your choice.'",
                choices: [
                    (
                        text: "[TIMED: 5s] Draw your weapon!",
                        effects: [
                            (Quality, "health", Subtract, 5),
                            (Quality, "notoriety", Add, 3),
                            (Quality, "violent_reputation", Set, 1),
                        ],
                        next: Some("bar_fight"),
                        time_limit: Some(5.0),
                    ),
                    (
                        text: "Stand down. Let's talk terms.",
                        effects: [
                            (Quality, "practical", Add, 1),
                            (Quality, "heist_knowledge", Set, 1),
                        ],
                        next: Some("mission_details"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 100,
            once_only: true,
        ),
        
        "bar_fight": (
            title: "Bar Brawl",
            description: "Violence erupts!",
            conditions: [],
            content: (
                text: "Gunfire erupts! You dive behind the bar as bottles explode above you. The woman is faster than she looks. A bullet grazes your shoulder. In the chaos, she vanishes, leaving only the photo and a business card: 'When you're done being stupid - V.'",
                choices: [
                    (
                        text: "Patch yourself up and reconsider.",
                        effects: [
                            (Quality, "wounded", Set, 1),
                            (Quality, "heist_knowledge", Set, 1),
                        ],
                        next: Some("preparation_wounded"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 100,
            once_only: true,
        ),
        
        "mission_details": (
            title: "The Briefing",
            description: "Learning about the job.",
            conditions: [(Quality, "heist_knowledge", GreaterThan, 0)],
            content: (
                text: "'The Blackwood Gallery. High-tech security, guards, the works. But there's a gala next week - lots of wealthy marks, lots of distractions. You'll need a team: someone for security systems, someone for the safe, maybe some muscle. I can provide contacts... for a price.'",
                choices: [
                    (
                        text: "I work alone.",
                        effects: [
                            (Quality, "lone_wolf", Set, 1),
                            (Quality, "difficulty", Add, 2),
                        ],
                        next: Some("preparation_solo"),
                        time_limit: None,
                    ),
                    (
                        text: "Give me the contacts. (-$200)",
                        conditions: [(Quality, "money", GreaterThan, 199)],
                        effects: [
                            (Quality, "money", Subtract, 200),
                            (Quality, "has_contacts", Set, 1),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "I'll find my own team.",
                        effects: [
                            (Quality, "independent", Add, 1),
                        ],
                        next: Some("preparation_team_search"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 80,
            once_only: true,
        ),
        
        "preparation_solo": (
            title: "Going Solo",
            description: "Preparing for a one-person job.",
            conditions: [
                (Quality, "lone_wolf", Equals, 1),
                (Quality, "heist_ready", Equals, 0),
            ],
            content: (
                text: "Working alone means you'll need to handle everything yourself. The gallery's security is no joke. You'll need to improve your skills or acquire the right equipment. What's your focus?",
                choices: [
                    (
                        text: "Study the building plans. [+Tech]",
                        effects: [
                            (Quality, "tech", Add, 2),
                            (Quality, "blueprints", Set, 1),
                            (Quality, "prep_days", Add, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                    (
                        text: "Practice lockpicking and stealth. [+Stealth]",
                        effects: [
                            (Quality, "stealth", Add, 2),
                            (Quality, "infiltration_ready", Set, 1),
                            (Quality, "prep_days", Add, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                    (
                        text: "Buy high-tech equipment. (-$500)",
                        conditions: [(Quality, "money", GreaterThan, 499)],
                        effects: [
                            (Quality, "money", Subtract, 500),
                            (Quality, "high_tech_gear", Set, 1),
                            (Quality, "prep_days", Add, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                    (
                        text: "I'm ready. Let's do this.",
                        conditions: [(Quality, "prep_days", GreaterThan, 1)],
                        effects: [
                            (Quality, "heist_ready", Set, 1),
                        ],
                        next: Some("heist_night"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 70,
            once_only: false,
        ),
        
        "team_building": (
            title: "Assembling the Crew",
            description: "Meeting potential team members.",
            conditions: [
                (Quality, "has_contacts", Equals, 1),
                (Quality, "team_complete", Equals, 0),
            ],
            content: (
                text: "You have contacts for three specialists: Maya (master hacker), Boris (explosives expert), and Shadow (cat burglar extraordinaire). You can afford to hire two. Who do you approach first?",
                choices: [
                    (
                        text: "Meet with Maya the hacker.",
                        conditions: [(Quality, "maya_hired", Equals, 0)],
                        effects: [],
                        next: Some("meet_maya"),
                        time_limit: None,
                    ),
                    (
                        text: "Meet with Boris the demo expert.",
                        conditions: [(Quality, "boris_hired", Equals, 0)],
                        effects: [],
                        next: Some("meet_boris"),
                        time_limit: None,
                    ),
                    (
                        text: "Meet with Shadow the burglar.",
                        conditions: [(Quality, "shadow_hired", Equals, 0)],
                        effects: [],
                        next: Some("meet_shadow"),
                        time_limit: None,
                    ),
                    (
                        text: "The team is ready. Time to plan.",
                        conditions: [(Quality, "team_size", GreaterThan, 1)],
                        effects: [
                            (Quality, "team_complete", Set, 1),
                        ],
                        next: Some("heist_planning"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 70,
            once_only: false,
        ),
        
        "meet_maya": (
            title: "The Hacker",
            description: "Meeting Maya.",
            conditions: [],
            content: (
                text: "Maya's apartment is a cave of glowing monitors. 'The Blackwood job? Their security system is military-grade. I can crack it, but I want 30% of the take.' Her fingers dance across a keyboard, already pulling up the gallery's digital footprint.",
                choices: [
                    (
                        text: "Deal. You're in.",
                        effects: [
                            (Quality, "maya_hired", Set, 1),
                            (Quality, "team_size", Add, 1),
                            (Quality, "tech_support", Set, 1),
                            (Quality, "cut_percentage", Add, 30),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "[Charm] How about 20%?",
                        conditions: [(Quality, "charm", GreaterThan, 2)],
                        effects: [
                            (Quality, "maya_hired", Set, 1),
                            (Quality, "team_size", Add, 1),
                            (Quality, "tech_support", Set, 1),
                            (Quality, "cut_percentage", Add, 20),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "Too rich for my blood. Pass.",
                        effects: [
                            (Quality, "maya_declined", Set, 1),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 60,
            once_only: true,
        ),
        
        "meet_boris": (
            title: "The Demolition Expert",
            description: "Meeting Boris.",
            conditions: [],
            content: (
                text: "Boris grins, revealing gold teeth. 'Ah, the Blackwood vault! Reinforced titanium, pressure sensors, the works. But nothing a little C4 can't handle, да?' He hefts a suspiciously heavy duffel bag. 'Quiet job or loud job, I do both. 25% cut.'",
                choices: [
                    (
                        text: "Welcome aboard, Boris.",
                        effects: [
                            (Quality, "boris_hired", Set, 1),
                            (Quality, "team_size", Add, 1),
                            (Quality, "explosives_ready", Set, 1),
                            (Quality, "cut_percentage", Add, 25),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "Can you do it without waking the whole block?",
                        effects: [
                            (Quality, "boris_hired", Set, 1),
                            (Quality, "team_size", Add, 1),
                            (Quality, "silent_explosives", Set, 1),
                            (Quality, "cut_percentage", Add, 25),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "Too risky. I'll pass.",
                        effects: [
                            (Quality, "boris_declined", Set, 1),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 60,
            once_only: true,
        ),
        
        "meet_shadow": (
            title: "The Cat Burglar",
            description: "Meeting Shadow.",
            conditions: [],
            content: (
                text: "You almost don't see Shadow until they speak from the rafters above. 'The Midnight Star? I've had my eye on that for years.' They drop silently beside you. 'I know three ways in already. 25% and I'll get you to the vault undetected.'",
                choices: [
                    (
                        text: "Perfect. You're hired.",
                        effects: [
                            (Quality, "shadow_hired", Set, 1),
                            (Quality, "team_size", Add, 1),
                            (Quality, "stealth_expert", Set, 1),
                            (Quality, "cut_percentage", Add, 25),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "What are these three ways in?",
                        effects: [
                            (Quality, "shadow_hired", Set, 1),
                            (Quality, "team_size", Add, 1),
                            (Quality, "multiple_routes", Set, 1),
                            (Quality, "cut_percentage", Add, 25),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                    (
                        text: "I need someone more... conventional.",
                        effects: [
                            (Quality, "shadow_declined", Set, 1),
                        ],
                        next: Some("team_building"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 60,
            once_only: true,
        ),
        
        "heist_planning": (
            title: "The Plan",
            description: "Finalizing the heist strategy.",
            conditions: [(Quality, "team_complete", Equals, 1)],
            content: (
                text: "Your team gathers around the blueprints. The gala is tomorrow night. You'll have a narrow window while the guests are distracted by the auction. How do you want to approach this?",
                choices: [
                    (
                        text: "Go in through the roof. [Requires Shadow or high Stealth]",
                        conditions: [(Any, [
                            (Quality, "shadow_hired", Equals, 1),
                            (Quality, "stealth", GreaterThan, 4),
                        ])],
                        effects: [
                            (Quality, "entry_method", Set, 1),
                            (Quality, "heist_ready", Set, 1),
                        ],
                        next: Some("heist_night"),
                        time_limit: None,
                    ),
                    (
                        text: "Hack the security and walk in the front. [Requires Maya or high Tech]",
                        conditions: [(Any, [
                            (Quality, "maya_hired", Equals, 1),
                            (Quality, "tech", GreaterThan, 4),
                        ])],
                        effects: [
                            (Quality, "entry_method", Set, 2),
                            (Quality, "heist_ready", Set, 1),
                        ],
                        next: Some("heist_night"),
                        time_limit: None,
                    ),
                    (
                        text: "Blow a hole in the wall. [Requires Boris]",
                        conditions: [(Quality, "boris_hired", Equals, 1)],
                        effects: [
                            (Quality, "entry_method", Set, 3),
                            (Quality, "heist_ready", Set, 1),
                            (Quality, "notoriety", Add, 2),
                        ],
                        next: Some("heist_night"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 50,
            once_only: true,
        ),
        
        "heist_night": (
            title: "The Night of the Heist",
            description: "It's showtime.",
            conditions: [(Quality, "heist_ready", Equals, 1)],
            content: (
                text: "The Blackwood Gallery glitters with lights and laughter. Wealthy patrons mill about, champagne in hand, oblivious to your presence. Your earpiece crackles to life. It's time. The Midnight Star awaits.",
                choices: [
                    (
                        text: "Execute the plan.",
                        effects: [],
                        next: Some("heist_execution"),
                        time_limit: None,
                    ),
                    (
                        text: "[TIMED: 10s] Wait... something's wrong. Abort!",
                        conditions: [(Quality, "paranoid", GreaterThan, 0)],
                        effects: [
                            (Quality, "aborted_heist", Set, 1),
                        ],
                        next: Some("heist_abort"),
                        time_limit: Some(10.0),
                    ),
                ],
            ),
            priority: 40,
            once_only: true,
        ),
        
        "heist_execution": (
            title: "Inside the Gallery",
            description: "The heist is underway.",
            conditions: [],
            content: (
                text: "You're in. The gallery's main floor stretches before you, marble gleaming under crystal chandeliers. Guards patrol in predictable patterns. The vault is three floors down. Every second counts.",
                choices: [
                    (
                        text: "[Stealth] Sneak past the guards.",
                        conditions: [(Quality, "stealth", GreaterThan, 3)],
                        effects: [
                            (Quality, "alert_level", Add, 0),
                            (Quality, "time_remaining", Subtract, 5),
                        ],
                        next: Some("vault_approach"),
                        time_limit: None,
                    ),
                    (
                        text: "[Charm] Blend in with the guests.",
                        conditions: [(Quality, "charm", GreaterThan, 2)],
                        effects: [
                            (Quality, "alert_level", Add, 0),
                            (Quality, "time_remaining", Subtract, 10),
                        ],
                        next: Some("vault_approach"),
                        time_limit: None,
                    ),
                    (
                        text: "Take out the guards quietly.",
                        effects: [
                            (Quality, "alert_level", Add, 1),
                            (Quality, "violent_approach", Set, 1),
                            (Quality, "time_remaining", Subtract, 3),
                        ],
                        next: Some("vault_approach"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 30,
            once_only: true,
        ),
        
        "vault_approach": (
            title: "The Vault",
            description: "The final obstacle.",
            conditions: [],
            content: (
                text: "The vault door looms before you - three tons of reinforced steel with a complex locking mechanism. Your tools are ready, but time is running out. How do you crack it?",
                choices: [
                    (
                        text: "Use the thermal lance. [Loud but fast]",
                        conditions: [(Quality, "high_tech_gear", Equals, 1)],
                        effects: [
                            (Quality, "alert_level", Add, 2),
                            (Quality, "time_remaining", Subtract, 5),
                            (Quality, "vault_open", Set, 1),
                        ],
                        next: Some("the_prize"),
                        time_limit: None,
                    ),
                    (
                        text: "Let Maya hack the electronic lock. [Requires Maya]",
                        conditions: [(Quality, "maya_hired", Equals, 1)],
                        effects: [
                            (Quality, "time_remaining", Subtract, 10),
                            (Quality, "vault_open", Set, 1),
                        ],
                        next: Some("the_prize"),
                        time_limit: None,
                    ),
                    (
                        text: "Boris, blow it. [Requires Boris]",
                        conditions: [(Quality, "boris_hired", Equals, 1)],
                        effects: [
                            (Quality, "alert_level", Add, 5),
                            (Quality, "time_remaining", Subtract, 2),
                            (Quality, "vault_open", Set, 1),
                            (Quality, "explosion_exit", Set, 1),
                        ],
                        next: Some("the_prize"),
                        time_limit: None,
                    ),
                    (
                        text: "[TIMED: 15s] Crack the combination manually.",
                        effects: [
                            (Quality, "time_remaining", Subtract, 15),
                            (Quality, "vault_open", Set, 1),
                        ],
                        next: Some("the_prize"),
                        time_limit: Some(15.0),
                    ),
                ],
            ),
            priority: 20,
            once_only: true,
        ),
        
        "the_prize": (
            title: "The Midnight Star",
            description: "Success... or is it?",
            conditions: [(Quality, "vault_open", Equals, 1)],
            content: (
                text: "The vault swings open. There it is - the Midnight Star, even more magnificent than the photos. But as you reach for it, you notice something else: documents, USB drives, evidence of massive money laundering. This is bigger than just a heist.",
                choices: [
                    (
                        text: "Take only the diamond. Stick to the plan.",
                        effects: [
                            (Quality, "has_diamond", Set, 1),
                            (Quality, "clean_heist", Set, 1),
                        ],
                        next: Some("escape_route"),
                        time_limit: None,
                    ),
                    (
                        text: "Take everything. Knowledge is power.",
                        effects: [
                            (Quality, "has_diamond", Set, 1),
                            (Quality, "has_evidence", Set, 1),
                            (Quality, "time_remaining", Subtract, 5),
                        ],
                        next: Some("escape_route"),
                        time_limit: None,
                    ),
                    (
                        text: "[TIMED: 5s] It's a trap! Get out!",
                        effects: [
                            (Quality, "trap_detected", Set, 1),
                        ],
                        next: Some("ambush"),
                        time_limit: Some(5.0),
                    ),
                ],
            ),
            priority: 10,
            once_only: true,
        ),
        
        "escape_route": (
            title: "The Getaway",
            description: "Time to leave.",
            conditions: [],
            content: (
                text: "Alarms blare! The heist is blown. Security forces are converging on your position. You need to get out NOW. Your escape routes are limited and every second counts.",
                choices: [
                    (
                        text: "Fight your way out the main entrance.",
                        conditions: [(Quality, "health", GreaterThan, 5)],
                        effects: [
                            (Quality, "health", Subtract, 3),
                            (Quality, "notoriety", Add, 5),
                            (Quality, "violent_escape", Set, 1),
                        ],
                        next: Some("aftermath"),
                        time_limit: None,
                    ),
                    (
                        text: "Use the maintenance tunnels. [Requires planning]",
                        conditions: [(Quality, "blueprints", Equals, 1)],
                        effects: [
                            (Quality, "clean_escape", Set, 1),
                        ],
                        next: Some("aftermath"),
                        time_limit: None,
                    ),
                    (
                        text: "Shadow, get us out! [Requires Shadow]",
                        conditions: [(Quality, "shadow_hired", Equals, 1)],
                        effects: [
                            (Quality, "stealth_escape", Set, 1),
                        ],
                        next: Some("aftermath"),
                        time_limit: None,
                    ),
                    (
                        text: "[TIMED: 10s] Improvise! Through the skylight!",
                        effects: [
                            (Quality, "health", Subtract, 2),
                            (Quality, "dramatic_escape", Set, 1),
                        ],
                        next: Some("aftermath"),
                        time_limit: Some(10.0),
                    ),
                ],
            ),
            priority: 5,
            once_only: true,
        ),
        
        "ambush": (
            title: "Betrayal!",
            description: "It was all a setup.",
            conditions: [],
            content: (
                text: "The mysterious woman steps out from behind the vault door, flanked by armed guards. 'Did you really think it would be that easy?' She smiles coldly. 'You've been played. But I'm feeling generous - work for me permanently, or die here.'",
                choices: [
                    (
                        text: "Never! I'll fight my way out!",
                        effects: [
                            (Quality, "health", Subtract, 8),
                            (Quality, "escaped_betrayal", Set, 1),
                        ],
                        next: Some("desperate_escape"),
                        time_limit: None,
                    ),
                    (
                        text: "...What kind of work?",
                        effects: [
                            (Quality, "turned_asset", Set, 1),
                        ],
                        next: Some("dark_ending"),
                        time_limit: None,
                    ),
                    (
                        text: "[With evidence] I know about your operation. Back off.",
                        conditions: [(Quality, "has_evidence", Equals, 1)],
                        effects: [
                            (Quality, "blackmail_victory", Set, 1),
                        ],
                        next: Some("leverage_ending"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 10,
            once_only: true,
        ),
        
        "aftermath": (
            title: "Counting the Cost",
            description: "The dust settles.",
            conditions: [(Quality, "has_diamond", Equals, 1)],
            content: (
                text: "Back at the safehouse, you assess the night's work. The Midnight Star glimmers on the table, your prize after all that chaos. But at what cost? And what comes next?",
                choices: [
                    (
                        text: "Sell the diamond and disappear.",
                        effects: [
                            (Quality, "money", Add, 1000000),
                            (Quality, "retired", Set, 1),
                        ],
                        next: Some("retirement_ending"),
                        time_limit: None,
                    ),
                    (
                        text: "Use the evidence to take down the conspiracy.",
                        conditions: [(Quality, "has_evidence", Equals, 1)],
                        effects: [
                            (Quality, "hero_path", Set, 1),
                        ],
                        next: Some("hero_ending"),
                        time_limit: None,
                    ),
                    (
                        text: "Keep the diamond. Some things are worth more than money.",
                        effects: [
                            (Quality, "kept_star", Set, 1),
                        ],
                        next: Some("thief_ending"),
                        time_limit: None,
                    ),
                ],
            ),
            priority: 1,
            once_only: true,
        ),
        
        // Endings
        "desperate_escape": (
            title: "Against All Odds",
            description: "A bloody escape.",
            conditions: [],
            content: (
                text: "You fight like a demon, using every trick you know. Bullets fly, glass shatters, and somehow - miraculously - you make it out alive. Bloodied, beaten, but not broken. The Midnight Star is lost, but you have your life. Sometimes that's enough.",
                choices: [
                    (
                        text: "THE END - Survivor's Ending",
                        effects: [(GameEnd, "survivor_ending", Set, 1)],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 0,
            once_only: true,
        ),
        
        "dark_ending": (
            title: "The Devil's Bargain",
            description: "A new master.",
            conditions: [],
            content: (
                text: "You become her personal thief, pulling impossible jobs for shadowy employers. The pay is excellent, the work is thrilling, but you'll never be free. Every heist deepens your chains. Welcome to your gilded cage.",
                choices: [
                    (
                        text: "THE END - Indentured Ending",
                        effects: [(GameEnd, "dark_ending", Set, 1)],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 0,
            once_only: true,
        ),
        
        "leverage_ending": (
            title: "Mutual Destruction",
            description: "Checkmate.",
            conditions: [],
            content: (
                text: "The evidence gives you leverage. She backs down, knowing exposure would destroy her entire operation. You walk out with the Midnight Star and a powerful enemy. But in this business, that's just another Tuesday. You've won this round.",
                choices: [
                    (
                        text: "THE END - Mastermind Ending",
                        effects: [(GameEnd, "mastermind_ending", Set, 1)],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 0,
            once_only: true,
        ),
        
        "retirement_ending": (
            title: "Fade to Black",
            description: "One last sunset.",
            conditions: [],
            content: (
                text: "The money from the Midnight Star buys you a new identity, a beach house in a country with no extradition treaty, and blessed anonymity. You watch sunsets and try to forget the thrill of the heist. Some days, you almost succeed.",
                choices: [
                    (
                        text: "THE END - Retirement Ending",
                        effects: [(GameEnd, "retirement_ending", Set, 1)],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 0,
            once_only: true,
        ),
        
        "hero_ending": (
            title: "The Whistleblower",
            description: "Justice served.",
            conditions: [],
            content: (
                text: "The evidence brings down a massive criminal conspiracy. You testify in secret, your identity protected. The Midnight Star is returned to its rightful owners. You're no hero - just a thief with a conscience. But sometimes that's exactly what the world needs.",
                choices: [
                    (
                        text: "THE END - Hero Ending",
                        effects: [(GameEnd, "hero_ending", Set, 1)],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 0,
            once_only: true,
        ),
        
        "thief_ending": (
            title: "The Perfect Thief",
            description: "Legend in the making.",
            conditions: [],
            content: (
                text: "You keep the Midnight Star, not for its value but as a reminder of the perfect heist. Your reputation spreads through the underworld. Jobs come calling, each more impossible than the last. You've become exactly what you always dreamed - the greatest thief alive.",
                choices: [
                    (
                        text: "THE END - Legend Ending",
                        effects: [(GameEnd, "legend_ending", Set, 1)],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 0,
            once_only: true,
        ),
        
        // Random encounters based on qualities
        "opportunity_knocks": (
            title: "A Side Job",
            description: "Quick money opportunity.",
            conditions: [
                (Quality, "money", LessThan, 500),
                (Quality, "heist_ready", Equals, 0),
                (Random, 30), // 30% chance when conditions are met
            ],
            content: (
                text: "A shady character approaches you. 'Hey, you look like someone who can handle themselves. I need a package delivered, no questions asked. $300 for an hour's work. Interested?'",
                choices: [
                    (
                        text: "Easy money. I'll do it.",
                        effects: [
                            (Quality, "money", Add, 300),
                            (Quality, "notoriety", Add, 1),
                            (Quality, "prep_days", Add, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                    (
                        text: "Too risky. I'll pass.",
                        effects: [
                            (Quality, "cautious", Add, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 30,
            once_only: false,
        ),
        
        "old_friend": (
            title: "Blast from the Past",
            description: "An old acquaintance appears.",
            conditions: [
                (Quality, "prep_days", GreaterThan, 2),
                (Quality, "heist_ready", Equals, 0),
                (Quality, "old_friend_met", Equals, 0),
            ],
            content: (
                text: "'Well, well. If it isn't my old partner.' Jackie grins at you from across the bar. 'Heard you're back in the game. The Blackwood job, right? I might have some info that could help... for old time's sake.'",
                choices: [
                    (
                        text: "What do you know about it?",
                        effects: [
                            (Quality, "gallery_intel", Add, 1),
                            (Quality, "old_friend_met", Set, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                    (
                        text: "We're not partners anymore, Jackie.",
                        effects: [
                            (Quality, "paranoid", Add, 1),
                            (Quality, "old_friend_met", Set, 1),
                        ],
                        next: None,
                        time_limit: None,
                    ),
                ],
            ),
            priority: 40,
            once_only: true,
        ),
    },
    
    qualities: {
        "health": (
            name: "Health",
            description: "Your physical condition",
            min: 0,
            max: 10,
            visible: true,
        ),
        "stealth": (
            name: "Stealth",
            description: "Ability to move unseen",
            min: 0,
            max: 10,
            visible: true,
        ),
        "tech": (
            name: "Tech",
            description: "Hacking and technical skills",
            min: 0,
            max: 10,
            visible: true,
        ),
        "charm": (
            name: "Charm",
            description: "Social manipulation skills",
            min: 0,
            max: 10,
            visible: true,
        ),
        "notoriety": (
            name: "Notoriety",
            description: "How well-known you are to law enforcement",
            min: 0,
            max: 10,
            visible: true,
        ),
        "money": (
            name: "Money",
            description: "Available funds",
            min: 0,
            max: None,
            visible: true,
        ),
        "prep_days": (
            name: "Days of Preparation",
            description: "Time spent preparing for the heist",
            min: 0,
            max: None,
            visible: false,
        ),
        "team_size": (
            name: "Team Members",
            description: "Number of specialists hired",
            min: 0,
            max: 3,
            visible: false,
        ),
        "alert_level": (
            name: "Alert Level",
            description: "Security awareness during heist",
            min: 0,
            max: 10,
            visible: false,
        ),
        "time_remaining": (
            name: "Time Window",
            description: "Minutes until security sweep",
            min: 0,
            max: 60,
            visible: false,
        ),
    },
))
"#;

fn main() {
    // Initialize the engine
    init();
    
    println!("=== The Heist ===");
    println!("A PlotScript Interactive Fiction Example");
    println!();
    println!("Your choices matter. Time limits are real. Good luck.");
    println!();
    
    // Create engine with interactive fiction configuration
    let config = EngineConfig {
        mode: GameMode::InteractiveFiction,
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
            display_story(&response);
        }
        Err(e) => {
            eprintln!("Failed to start game: {}", e);
            return;
        }
    }
    
    // Game loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // Check for quit
        if input == "quit" || input == "exit" {
            println!("Thanks for playing!");
            break;
        }
        
        // Process input
        match engine.process_input(input) {
            Ok(response) => {
                display_story(&response);
                
                // Check game over
                if response.ended {
                    println!("\n=== GAME OVER ===");
                    display_final_stats(&engine);
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}\n", e);
            }
        }
    }
}

fn display_story(response: &plotscript::Response) {
    if let Some(location) = &response.location {
        println!("\n=== {} ===", location);
    }
    
    println!("\n{}", response.text);
    
    // Display choices if available
    if !response.choices.is_empty() {
        println!("\n[What do you do?]");
        for (i, choice) in response.choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.text);
        }
    }
}


fn display_stats(engine: &Engine) {
    println!("\n=== Character Stats ===");
    let state = &engine.state;
    println!("Health: {:?}", state.get_variable("health"));
    println!("Stealth: {:?}", state.get_variable("stealth"));
    println!("Tech: {:?}", state.get_variable("tech"));
    println!("Charm: {:?}", state.get_variable("charm"));
    println!("Notoriety: {:?}", state.get_variable("notoriety"));
    println!("Money: {:?}", state.get_variable("money"));
}

fn display_final_stats(engine: &Engine) {
    println!("\n=== Final Stats ===");
    display_stats(engine);
    
    println!("\nThank you for playing The Heist!");
    println!("Try different choices to discover all {} endings!", 7);
}