;; https://leetcode.com/problems/find-pivot-index/

(ql:quickload "iterate")

(defpackage :find-pivot-index
  (:use :cl :iterate))
(in-package :find-pivot-index)


(defparameter *example-1*
  '((nums . #(1 7 3 6 5 6))
    (output . 3)))

(defparameter *example-2*
  '((nums . #(1 2 3))
    (output . -1)))

(defparameter *example-3*
  '((nums . #(2 1 -1))
    (output . 0)))

(defun pivot-index? (index nums)
  "Check if index is a pivot index."
  (let ((left-sum
          (if (zerop index) 0 ;; if leftmost, sum is 0
              (reduce #'+ (subseq nums 0 index))))
        (right-sum
          (if (= (length nums) (+ 1 index)) 0  ;; if rightmost, sum is 0
              (reduce #'+ (subseq nums (+ 1 index))))))
    (= left-sum right-sum)))

(defun find-leftmost-pivot-index (nums)
  (iterate
    (for index from 0 below (length nums))
    (if (pivot-index? index nums) (leave index))
    (finally (return -1))))

;; ITER> (iterate (for i in-sequence #(1 2)) (if (> i 2) (leave i)) (finally (return -1)))
;; -1 (0 bits)
;; ITER> (iterate (for i in-sequence #(1 2 3)) (if (> i 2) (leave i)) (finally (return -1)))
;; 3 (2 bits, #x3, #o3, #b11)
