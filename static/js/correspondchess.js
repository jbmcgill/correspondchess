
var board = null
var game = new Chess()
var whiteSquareGrey = '#a9a9a9'
var blackSquareGrey = '#696969'

var hash = $(location).attr('hash');
var game_slug = hash.replace("#", "");
var savedGames = loadSavedGames();
var curAction;
var socket;
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
  $("#chat-input").on("keypress", function(e){
    if(e.which == 13)
    {
	$("#chat-input").attr("disabled", "disabled")
        clickChatSubmit()
	$("#chat-input").removeAttr("disabled")
	$("#chat-input").focus()
    }
  })
  if( game_slug == "" ){
      showBox("#welcome-box")
  }else{
      showBox("#board-box")
      $.ajax({
        url: "/game/" + game_slug,
        type: "GET",
        contentType: "application/json; charset=utf-8",
        error: function(e){ alert(e)},
        success: function (result) {
          if( result.side == "black" ) board.flip()
          result.moves.forEach(mv => game.move(mv))
          board.position(game.fen())
          updateUI()
          socket = new WebSocket('ws://10.0.0.238:8080/ws/'+ game_slug)
          socket.addEventListener('message', function (event) {
            var o = JSON.parse(event.data)
            if ("OpponentMove" in o ){
            	game.move(o.OpponentMove.san)
            	board.position(game.fen())
            	updateUI()
            }else if ( "ChatMessage" in o ){
            	//alert(JSON.stringify(o.ChatMessage))
            	console.log("chatmessage received")
            	$("chat-history").append("<p><b>"+ o.ChatMessage.handle +"</b> "+ o.ChatMessage.msg +"</p>" )
      		$("chat-history").scrollTop($("chat-history")[0].scrollHeight)
            	if( curAction != "chat-box" ){
            	  $("#chat-button").html("Chat (!)")
            	}
            }else{
            	alert(event.data)
            }
          });
          // TODO: add event listener to disconnect attempt to reconnect

        },
        error: function (error) { console.log(error) },
      })
  }
})

function updateUI() {

  $('#fenDiv').html(game.fen())
  $('#pgn-box').html(game.pgn({ max_width: 5, newline_char: '<br />' }))
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
  $('#pgn-div').scrollTop($('#pgn-div').scrollHeight);

}

function loadSavedGames(){
  var results = []
  storage = window.localStorage
  for ( var i=0; i< storage.length; i++){
    var item = JSON.parse(storage.getItem(storage.key(i)))
    results.push(item)
  }
  return results
}

function handleNewGameCreated(x) {

  var id, side, your_link, opponent_link
  if ($("#i-will-play-white").is(":checked")) {
    id = x.white
    side = "white"
    your_link = x.white
    opponent_link = x.black
  } else {
    id = x.black
    your_link = x.black
    opponent_link = x.white
    side = "black"
  }

  var obj = { white: x.white, 
              black: x.black, 
              side: side, 
              state: "in-play", 
              turn: "white", 
              notes: ""
             }
  localStorage.setItem(id, JSON.stringify(obj))
  $("#opponent-link").val(opponent_link)
  $("#your-link").val(your_link)
  showBox("#game-created-box")
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
      handleNewGameCreated(result);
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

var tabs_map = new Map()
tabs_map.set("chat-box","#chat-button")
tabs_map.set("#pgn-box","#history-button")
function showAction(id){
  $("#pgn-box").hide()
  $("chat-box").hide()
  $("#history-button").removeClass("toolbar-button-selected")
  $("#chat-button").removeClass("toolbar-button-selected")
  var tabId = tabs_map.get(id) 
  $(tabId).addClass("toolbar-button-selected")
  $(id).animate({ "opacity": "show" }, 200)
  curAction = id
  if( id == "chat-box" ){
    $("#chat-button").html("Chat")
  }
}

function clickChatSubmit(){
  var txt = $("#chat-input").val()
  if( txt != "" ){
	socket.send(JSON.stringify({"ChatMessage": {"handle": "NewMessage", "msg": txt}}))
	$("#chat-input").val("")
	$("#chat-input").focus()
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

function onDragStart(source, piece, position, orientation) {
  // do not pick up pieces if the game is over
  if (game.game_over()) return false

  // or if it's not that side's turn
  if ((game.turn() === 'w' && (piece.search(/^b/) !== -1 || orientation === 'black')) ||
    (game.turn() === 'b' && (piece.search(/^w/) !== -1 || orientation === 'white')) ) {
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
