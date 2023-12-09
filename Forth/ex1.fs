256 Constant max-line
Create line-buffer max-line 2 + allot \ line-buffer[max-line + 1]

s" ./ex1_in.txt" r/o open-file throw Constant in-file


: next-line ( wfileid -- u flag ) \ u: size of line read to line-buffer, flag: has data 
    >r
    line-buffer max-line r> read-line throw ;

: char-at-line ( u -- c )
    chars line-buffer + c@ ; \ char at index u

: is-in-range ( u u1 u2 -- f ) \ u > u1 & u <= u2
    >r over swap
    u>= swap
    r> u< and ;

: is-numeric ( c -- f )
    48 58 is-in-range ;

: char-to-int ( c -- u )
    48 - ;

: parse-line-forward ( u1 -- u2 ) \ string of length u1 and parse l to r, u: result digit;
    \ s" parse-line-forward" type .s cr
    0 0 begin
        \ TODO abort at line end

        drop dup char-at-line
        swap 1 + swap
    dup is-numeric until

    nip nip char-to-int ;

: parse-line-backward ( u1 -- u2 ) \ take line string and parse l to r, u: result digit;
    \ s" parse-line-backward" type .s cr
    0 begin
        drop 1 -
        dup char-at-line
    dup is-numeric until

    nip char-to-int ;

: parse-line ( wfileid -- u flag )
    next-line if
        dup parse-line-forward 10 *
        swap parse-line-backward +
        true
    else
        0 false
    endif 

    s" res: " type over . cr ;

: parse ( wfileid -- sum )
    0 begin
        over parse-line while
            +
    repeat 
    2drop nip ;
