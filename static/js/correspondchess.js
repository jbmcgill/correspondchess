
var board = null
var game = new Chess()
var whiteSquareGrey = '#a9a9a9'
var blackSquareGrey = '#696969'

var hash = $(location).attr('hash');
var game_slug = hash.replace("#", "");
$(document).ready(function () {
  var config = {
    draggable: true,
    position: 'start',
    pieceTheme: 'img/chesspieces/wikipedia/{piece}.png',
    onDragStart: onDragStart,
    onDrop: onDrop,
    onMouseoutSquare: onMouseoutSquare,
    onMouseoverSquare: onMouseoverSquare,
    onSnapEnd: onSnapEnd
  }
  board = Chessboard('myBoard', config)
  jQuery('#myBoard').on('scroll touchmove touchend touchstart contextmenu', function (e) {
    e.preventDefault();
  });
  $.ajax({
    url: "/game/" + game_slug,
    type: "GET",
    contentType: "application/json; charset=utf-8",
    success: function (result) {
      result.moves.forEach(mv => game.move(mv))
      board.position(game.fen())
      updateUI()
    },
    error: function (error) { console.log(error) },
  })
})

function updateUI() {

  $('#fenDiv').html(game.fen())
  $('#pgnDiv').html(game.pgn({ max_width: 5, newline_char: '<br />' }))
  var l = (game.history().length % 2) == 0 ? "White" : "Black"
  var status = "In-Play - " + l + " to move"
  if (game.in_stalemate()) {
    status = "Stalemate - Game Over"
  } else if (game.in_draw()) {
    status = "Draw - Game Over"
  } else if (game.in_threefold_repetition()) {
    status = "Threefold repetition"
  } else if (game.in_checkmate()) {
    status = "Checkmate - " + ((game.history().length % 2) == 0 ? "Black" : "White") + " Wins"
  } else if (game.in_check()) {
    status = "Check - " + l + " to move"
  }
  $('#statusDiv').html("Status: " + status)
  $('#pgnDiv').scrollTop($('#pgnDiv')[0].scrollHeight);

}


function handleGenerateClick() {
  data = {
    white: $("#white-handle").val(),
    black: $("#black-handle").val()
  }

  $.ajax({
    url: "/game",
    type: "PUT",
    contentType: "application/json; charset=utf-8",
    data: JSON.stringify(data),
    success: function (result) {
      alert(JSON.stringify(result))
    },
    error: function (error) { console.log(error) },
  });
}


function showBox(id) {
  $("#welcome-box").hide()
  $("#game-created-box").hide()
  $("#create-game-box").hide()
  $("#error-box").hide()
  $("#board-box").hide()
  $(id).animate({ "opacity": "show" }, 500)
  if (id == '#create-game-box') {
    $('#white-handle').val(generateName())
    $('#black-handle').val(generateName())
  }
}

function removeGreySquares() {
  $('#myBoard .square-55d63').css('background', '')
}

function greySquare(square) {
  var $square = $('#myBoard .square-' + square)

  var background = whiteSquareGrey
  if ($square.hasClass('black-3c85d')) {
    background = blackSquareGrey
  }

  $square.css('background', background)
}

function onDragStart(source, piece) {
  // do not pick up pieces if the game is over
  if (game.game_over()) return false

  // or if it's not that side's turn
  if ((game.turn() === 'w' && piece.search(/^b/) !== -1) ||
    (game.turn() === 'b' && piece.search(/^w/) !== -1)) {
    return false
  }
}

function onDrop(source, target) {
  removeGreySquares()

  // see if the move is legal
  var move = game.move({
    from: source,
    to: target,
    promotion: 'q' // NOTE: always promote to a queen for example simplicity
  })

  // illegal move
  if (move === null) return 'snapback'

  console.log("move san: " + move.san);
  console.log("move: " + JSON.stringify(move));
  console.log("game_slug: " + game_slug);
  $.ajax({
    url: "/game/" + game_slug + "/move",
    type: "POST",
    contentType: "application/json; charset=utf-8",
    data: JSON.stringify({ san: move.san }),
    success: function (result) { updateUI() },
    error: function (error) { console.log(error) },
  })
}

function onMouseoverSquare(square, piece) {
  // get list of possible moves for this square
  var moves = game.moves({
    square: square,
    verbose: true
  })

  // exit if there are no moves available for this square
  if (moves.length === 0) return

  // highlight the square they moused over
  greySquare(square)

  // highlight the possible squares for this piece
  for (var i = 0; i < moves.length; i++) {
    greySquare(moves[i].to)
  }
}

function onMouseoutSquare(square, piece) {
  removeGreySquares()
}

function onSnapEnd() {
  board.position(game.fen())
}
