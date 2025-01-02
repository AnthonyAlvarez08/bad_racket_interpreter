#lang racket

(if #t 1 3)
(if (< 5 3) 5 (* 3 2))
(or #t #t)
(xor #t #f)
(and #t #t)
(nand #t #t)
(< 1 2 3 4)
(> 5 6 4 3 2)
(= 4 4 4 4 4)
(= 5 5 3 4 5)

(if (or #t #t #f #f #f #t)  (nand #t #t (= 4 3) #f #t (< (* 2 3) 9)) (xor #t #t))