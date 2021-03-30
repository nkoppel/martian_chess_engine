var board = [].fill.call({length: 32}, 0);
var pieces = ["_", "^", "*", "A"];

var render_images = ["", "pyramid_yellow.png", "pyramid_blue.png", "pyramid_red.png"];
// var render_images = ["", "pyramid_white3.png", "pyramid_white2.png", "pyramid_white1.png"];
var render_sizes = [0, 60, 70, 80];

var square_width = 60

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

    $('#board')
        .css('width' , (square_width * 4) + 'px')
        .css('height', (square_width * 8) + 'px');

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

            out += ' style="width:' + square_width + 'px;height:'

            if (y == 3 || y == 4) {
                out += square_width - 2
            } else {
                out += square_width
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

    $('#board').html(out);
    set_square_click_events();

    $('#p1_score').html('Player 1 points: ' + engine.get_p1_score());
    $('#p2_score').html('Player 2 points: ' + engine.get_p2_score());
}

function render_engine_board() {
    decode_board(engine.get_position().board);
    render_board()
}

function show_moves(moves) {
    for (var i = 0; i < 32; i++) {
        var square = $('.square' + i);

        if (moves & 1 !== 0) {
            if ((i % 4 + Math.floor(i / 4)) % 2 == 0) {
                square.removeClass('black').addClass('black-highlight')
            } else {
                square.removeClass('white').addClass('white-highlight')
            }
        } else {
            if ((i % 4 + Math.floor(i / 4)) % 2 == 0) {
                square.removeClass('black-highlight').addClass('black')
            } else {
                square.removeClass('white-highlight').addClass('white')
            }
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

        if (engine.do_num_move(clickedSquare, square)) {
            render_engine_board();

            clickedSquare = -2;

            setTimeout(() => {
                do_engine_move(1000);
                clickedSquare = -1;
            }, 10);
        } else {
            clickedSquare = -1;
            click_square(square);
        }
    }
}

function set_square_click_events() {
    for (var i = 0; i < 32; i++) {
        $('.square' + i).click(eval('() => click_square(' + i + ')'))
    }
}

function run() {
    render_engine_board();
    set_square_click_events();
}

setTimeout(run, 100);
