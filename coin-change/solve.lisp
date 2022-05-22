;; https://leetcode.com/problems/coin-change/

(defpackage :find-change
  (:use :cl :iterate))
(in-package :find-change)

(defun next-iterations (coins current-change)
  (iter (for coin in coins) (collect (cons coin current-change))))

(defun find-change (amount-wanted coins)
  (if (zerop amount-wanted)
      nil
      (find-change-loop 0 amount-wanted coins (next-iterations coins nil))))

;; breadth-first search
(defun find-change-loop (iterations amount-wanted coins queue)
  (if (null queue)
      -1 ;; ran out of combinations
      (let* ((coin-combo (first queue))
             (valuation (iter (for coin in coin-combo) (sum coin))))
        (cond
          ((> iterations 100) ;; recursion limit
           (error "too far"))
          ((= valuation amount-wanted) ;; found it!
           (length coin-combo))
          ((> valuation amount-wanted) ;; too high
           (find-change-loop (+ 1 iterations) amount-wanted coins
                        (rest queue)))
          ((< valuation amount-wanted) ;; too low
           (find-change-loop (+ 1 iterations) amount-wanted coins
                        (append (rest queue) (next-iterations coins coin-combo))))))))
