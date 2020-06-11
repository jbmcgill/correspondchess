table! {
    game (id) {
        id -> Integer,
        created -> BigInt,
        white -> Text,
        black -> Text,
    }
}

table! {
    moves (id) {
        id -> Integer,
        game_id -> Integer,
        player_move -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    game,
    moves,
);
