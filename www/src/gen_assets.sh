#!/bin/sh

convert -size 250x250 xc:white -alpha transparent \
    -fill black -draw "polygon 0,250 250,250 125,0" \
    -fill white -draw "polygon 34,229 216,229, 125,40" \
    pyramid_white1.png

convert -size 250x250 xc:white -alpha transparent \
    -fill black -draw "polygon 0,250 250,250 125,0" \
    -fill white -draw "polygon 39,226 211,226, 125,45" \
    pyramid_white2.png

convert -size 250x250 xc:white -alpha transparent \
    -fill black -draw "polygon 0,250 250,250 125,0" \
    -fill white -draw "polygon 45,222 205,222, 125,53" \
    pyramid_white3.png

convert -size 250x250 xc:white -alpha transparent \
    -fill black     -draw "polygon 0,250 250,250 125,0" \
    -fill '#ff391a' -draw "polygon 34,229 216,229, 125,40" \
    pyramid_red.png

convert -size 250x250 xc:white -alpha transparent \
    -fill black     -draw "polygon 0,250 250,250 125,0" \
    -fill '#352fee' -draw "polygon 39,226 211,226, 125,45" \
    pyramid_blue.png

convert -size 250x250 xc:white -alpha transparent \
    -fill black     -draw "polygon 0,250 250,250 125,0" \
    -fill '#ece430' -draw "polygon 45,222 205,222, 125,53" \
    pyramid_yellow.png
