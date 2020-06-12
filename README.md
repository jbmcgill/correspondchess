# Correspondence Chess Server

An anonymous correspondence chess server. 

Note that this is a work in progress and not ready to be used.


# Installation

    TODO


# Configuration

Configuration can be set via the environment or by a `.env` file in the working
directory.

|Name|Default|Notes|
|---|---|---|
|CORRESPONDCHESS_PHRASE|kujlturenbvjccna|Used to uniquely encode identifiers into shareable urls|
|CORRESPONDCHESS_BIND|127.0.0.1:8080|Server listening port|
|CORRESPONDCHESS_DB|correspondchess.db|SQLite database file|

# Usage

    TODO 

# Developer

## System Architecture

    +--------------+                +-----------------+
    |   browser    |                |    actix-web    |
    |  Javascript  |  <-- REST -->  | correspondchess |
    +--------------+                |         |       |
                                    |    ( SQLITE )   |
                                    +-----------------+

## Client Side

The correspondence web page uses Javascript to call the server's REST services.
It is built with:

- [LatexCSS](https://latex.now.sh/) for the styling
- [ChessboardJS](https://chessboardjs.com/) for the interactive chess board
- [ChessJS](https://github.com/jhlywa/chess.js) for legal move constraints

## Server side

Correspondchess server exposes a REST API for creating and playing games. It is
built with:

- Rust
- actix-web
- diesel
- shakmaty

Data is stored in an SQLite database.

## Schema

    +---------+      +---------+
    |  game   |      |   move  |
    +---------+      +---------+
    | id      |--+   | id      |
    | created |  +-->| game_id |
    +---------+      | move    |
                     +---------+


## REST Protocol

The REST protocol is used for correspondence games. It is also used for creating
new games.

When a game is created two game links are generated with identifiers encoded in a
harshid ([game_id, WHITE_OR_BLACK]). One URL is the link for the white player,
the other for the black player.

### PUT /game 

Create a new game.

#### CreateGameRequest

```json
{
	"white": "white nickname",
	"black": "black nickname"
}

```

#### CreateGameResponse

```json
{
	"white": "link for white player",
	"black": "link for black player"
}
```

### GET /game/{slug}

Parses the slug and returns the current game.

#### GetGameResponse

```json
{
	"created": "{timestamp}",
	"white": "{nick}",
	"black": "{nick}",
	"side" : "white|black",
	"moves": ["SAN","SAN","SAN","SAN", "..."]
}
```

### POST /game/{slug}/move

#### PlayerMoveRequest

```json
{
	"san": "SAN"
}
```

#### PlayerMoveResponse

```json
{ 
	"status": "true|false",
	"description": "description"
}
```

