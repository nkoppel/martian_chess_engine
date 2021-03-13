var board = [].fill.call({length: 32}, 0);
var log_pieces = ["_", "^", "*", "A"];
var render_pieces = [" ", "^", "*", "A"];

function decode_board(rsboard) {
    board = [].fill.call({length: 32}, 0);

    var i = 0;
    var x = rsboard.lower;

    while (x !== 0) {
        board[i] += x & 1;
        x >>>= 1;
        i++;
    }

    x = rsboard.upper;
    i = 0;

    while (x !== 0) {
        board[i] += (x & 1) * 2;
        x >>>= 1;
        i++;
    }
}

function log_board() {
    var out = "";

    for (var y = 7; y >= 0; y--) {
        for (var x = 3; x >= 0; x--) {
            out += log_pieces[board[x + y * 4]] + " ";
        }

        out += "\n"
    }

    console.log(out);
}

function render_board() {
    var out = "";
    var width = 50;

    for (var y = 7; y >= 0; y--) {
        out += '<div class="row">';

        for (var x = 3; x >= 0; x--) {
            out += '<div class="square'

            out += ' square' + (x + y * 4)

            if ((x + y) % 2 == 0) {
                out += ' black"'
            } else {
                out += ' white"'
            }

            out +=
                ' style="width:' + width + 'px;' + 
                'height:'        + width + 'px;' +
                'line-height:'   + width + 'px">'

            out += render_pieces[board[x + y * 4]] + "</div>"
        }

        out += '</div>';
    }

    document.getElementById("board").innerHTML = out;
}

function render_engine_board() {
    decode_board(engine.get_position().board);
    render_board()
}

function show_moves(moves) {
    for (var i = 0; i < 32; i++) {
        if (moves & 1 !== 0) {
            document.getElementsByClassName('square' + i)[0].style.backgroundColor = 'red';
        } else {
            document.getElementsByClassName('square' + i)[0].style.backgroundColor = '';
        }

        moves >>>= 1;
    }
}

var clickedSquare = -1;

function click_square(square) {
    if (clickedSquare < 0) {
        var moves = engine.get_piece_moves(square);

        show_moves(moves);

        if (moves !== 0) {
            clickedSquare = square;
        }
    } else {
        show_moves(0);
        engine.do_num_move(clickedSquare, square);
        render_engine_board();
        clickedSquare = -1;
    }
}

function set_square_click_events() {
    for (var i = 0; i < 32; i++) {
        document.getElementsByClassName('square' + i)[0].onclick =
            eval('() => click_square(' + i + ')')
    }
}

function init() {
    engine.search(100);
    decode_board(engine.get_best_move());
    render_board()
}

setTimeout(init, 100);
