# Correspondence Chess Server

A real-time and correspondence chess server.


# Installation

    # deb / ubuntu
    sudo apt-get install correspondchess.deb

    # centos / redhat / fedora
    sudo rpm -i correspondchess.rpm

# Configuration

Configuration can be set via the environment or by a `.env` file in the working
directory.

|Name|Default|Notes|
|---|---|---|
|CORRESPONDCHESS_PHRASE|`kujlturenbvjccna`|Used to uniquely encode identifiers into shareable urls|
|CORRESPONDCHESS_PORT|8080|Server listening port|

# Usage

    service start correspondchess

# Developer

## System Architecture

    +--------------+                +-----------------+
    |   browser    |                |    actix-web    |
    | JS WebSocket |  <-- JSON -->  | correspondchess |
    +--------------+                |         |       |
                                    |    ( SQLITE )   |
                                    +-----------------+

## Client Side

Correspondchess client side creates a WebSocket connection to the server. The client sends
a message to the server when the user acts and it updates the board when the
servers sends a message.

- [LatexCSS](https://latex.now.sh/) for the styling
- [ChessboardJS](https://chessboardjs.com/) for the interactive chess board
- [ChessJS](https://github.com/jhlywa/chess.js) for legal move constraints

## Server side

Correspondchess server side listens for WebSocket connections form the client.
It sends board and game information to the client. It also receives player
moves. Data is stored in an SQLite database.

## Schema

    +---------+      +---------+
    |  game   |      |   move  |
    +---------+      +---------+
    | id      |--+   | id      |
    | created |  +-->| game_id |
    +---------+      | created |
                     | move    |
                     +---------+


## REST Protocol

The REST protocol is used for correspondence games. It is also used for creating
new games.

When a game is created two game links are generated with identifiers encoded in a
harshid ([game_id, WHITE_OR_BLACK]). One URL is the link for the white player,
the other for the black player.

### PUT /game 

Create a new game.

#### Request

```json
{
	"white": "white nickname",
	"black": "black nickname"
}

```

#### Response

```json
{
	"white": "link for white player",
	"black": "link for black player"
}
```

### GET /game/{game link}

Parses the game link and returns the current game.

```json
{
	"created": "{created}",
	"white": "{nick}",
	"black": "{nick}",
	"side" : "white|black",
	"moves": ["SAN","SAN","SAN","SAN", "..."]
}
```

### POST /game/{game link}/move

#### Request

```json
{
  "PlayerMoveRequest": {
                         "san": "SAN"
                       }
}
```

#### Response

```json
{
  "PlayerMoveResponse": {
                          "status": "OK|ERROR",
			  "description": "description"
                        }
}
```

## WebSocket Protocol

When a game is created two game links are generated with identifiers encoded in a
harshid ([game_id, WHITE_OR_BLACK]). One URL is the link for the white player,
the other for the black player.

When a user arrives to the site via a game link the client javascript creates a
WebSocket connection to the server. On successful connection the protocol
begins.

### ClientSubscribeRequest

Sent from client. Has the effect of subscribing to events related to the game link.

```json
{
  "ClientSubscribeRequest": {"gameid": "{game link slug}"}
}
```

### ClientSubscribeResponse 

Sent from the server. Tells the client is the subscription is successful and if
not explains why.

```json
{
  "ClientSubscribeResponse": {
                              "gameid": "{game link slug}", 
			      "status": "OK|INVALID",
			      "descript": "{description}"
                             } 
}
```

### GamePgnNotification

Sent from the server. A PGN representation of the chess game move history.

```json
{
  "GamePgnNotification": {"gameid": "{game link slug"}", "pgn": "{PGN string}"}
}
```

### ClientMoveRequest

Sent from the client. Tell the server about a move.

```json
{
  "ClientMoveRequest": {"gameid": "{game link slug}", "move": "{move}"}
}
```

### ClientMoveResponse

Sent from the server. Tell the client if the last move was successful.

```json
{
  "ClientMoveResponse": {
                         "gameid": "{game link slug}", 
			 "move": "{move}",
			 "status": "{OK|INVALID|NOT_YOUR_TURN}",
                         "description": "{description}"
			 }
}
```

### GameOverNotification

Sent from the server. Tell the client that the game is over.

```json
{
  "GameOverNotification": {
                       "gameid": "{game link slug"},
		       "status": "WIN|STALE|LOSE"
                      }
}
```

### OpponentMoveNotification

Sent from the server. Tell the client that the opponent has moved.

```json
{
  "OpponentMoveNotification": {"gameid": "{game link slug}", "move": "{move}"}
}
```

