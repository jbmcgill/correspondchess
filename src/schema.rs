table! {
    game (id) {
        id -> Integer,
        created -> BigInt,
    }
}

table! {
    moves (id) {
        id -> Integer,
        created -> BigInt,
        game_id -> Integer,
        player_move -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    game,
    moves,
);
