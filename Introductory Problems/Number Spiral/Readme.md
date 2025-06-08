1  2  9  10 25 26
4  3  8  11 24 27
5  6  7  12 23 28
16 15 14 13 22 29
17 18 19 20 21 30
36 35 34 33 32 31


2,3 => 2nd row, 3rd column = 8
1,1 => 1st row 1st column = 1
4,2 => 4th row 2nd column = 15

if row >= col:
    - at the xth row, 
    - if row is odd, 1st elemnt is (x-1)^2 + 1, and goes increasing answer is (row-1)^2 + col
    - else 1st element is x^2 goes decreasing, ans is x^2 - col + 1

if row < col:
    - at the xth col, 
    - if col is odd, 1st elemnt is (x-1)^2 + 1, and goes increasing answer is (col-1)^2 + row
    - else 1st element is x^2 goes decreasing, ans is x^2 - row + 1