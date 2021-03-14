var board = [].fill.call({length: 32}, 0);
var pieces = ["_", "^", "*", "A"];

var render_images = ["", "pyramid_yellow.png", "pyramid_blue.png", "pyramid_red.png"];
var render_sizes = [0, 60, 70, 80];

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
            out += pieces[board[x + y * 4]] + " ";
        }

        out += "\n"
    }

    console.log(out);
}

function render_board() {
    var out = "";
    var width = 50;

    for (var y = 7; y >= 0; y--) {
        out += '<div class="row row' + y + '">';

        for (var x = 3; x >= 0; x--) {
            var piece = board[x + y * 4];

            out += '<div class="square'

            out += ' square' + (x + y * 4)

            if ((x + y) % 2 == 0) {
                out += ' black"'
            } else {
                out += ' white"'
            }

            out += ' style="width:' + width + 'px;height:'

            if (y == 3 || y == 4) {
                out += width - 2
            } else {
                out += width
            }

            out += 'px">';

            if (piece !== 0) {
                out +=
                    '<img src="assets/' + render_images[piece] +
                     '" width="'  + render_sizes[piece] +
                    '%" height="' + render_sizes[piece] +
                    '%">'
            }

            out += "</div>"
        }

        out += '</div>';
    }

    document.getElementById("board").innerHTML = out;
    set_square_click_events();
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

function do_engine_move(time) {
    engine.search(time);
    engine.do_move(engine.get_best_move());
    render_engine_board();
}

var clickedSquare = -1;

function click_square(square) {
    if (clickedSquare === -1) {
        var moves = engine.get_piece_moves(square);

        show_moves(moves);

        if (moves !== 0) {
            clickedSquare = square;
        }
    } else if (clickedSquare >= 0) {
        show_moves(0);

        engine.do_num_move(clickedSquare, square);
        render_engine_board();

        clickedSquare = -2;

        setTimeout(() => {
            do_engine_move(1000);
            clickedSquare = -1;
        }, 10);
    }
}

function set_square_click_events() {
    for (var i = 0; i < 32; i++) {
        document.getElementsByClassName('square' + i)[0].onclick =
            eval('() => click_square(' + i + ')')
    }
}

function run() {
    render_engine_board();
    set_square_click_events();
}

setTimeout(run, 100);
