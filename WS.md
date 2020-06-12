# Future work to make real-time chess service

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

