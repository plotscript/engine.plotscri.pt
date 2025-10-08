game "Simple Adventure" {
    author: "Test Author"
    mode: text_adventure
    description: "A simple test adventure"
}

room entrance "Entrance Hall" {
    description: "You stand in a grand entrance hall with marble floors."
    exits: {
        north: hallway
    }
    items: [brass_key]
}

room hallway "Long Hallway" {
    description: "A long hallway stretches before you."
    exits: {
        south: entrance
        east: library
    }
}

room library "Dusty Library" {
    description: "Shelves of ancient books surround you."
    exits: {
        west: hallway
    }
}

item brass_key "Brass Key" {
    description: "A small brass key with intricate engravings."
    takeable: true
}