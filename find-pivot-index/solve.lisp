;; https://leetcode.com/problems/find-pivot-index/

(defpackage :find-pivot-index
  (:use :cl :iterate))
(in-package :find-pivot-index)

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

(defclass example ()
  ((nums
    :initarg :nums
    :accessor example-nums)
   (output
    :initarg :output
    :accessor example-output)))

(defparameter *example-1*
  (make-instance 'example :nums #(1 7 3 6 5 6) :output 3))

(defparameter *example-2*
  (make-instance 'example :nums #(1 2 3) :output -1))

(defparameter *example-3*
  (make-instance 'example :nums #(2 1 -1) :output 0))

(defparameter *examples* (list *example-1* *example-2* *example-3*))

(defmethod check-example ((ex example))
  (=
   (find-leftmost-pivot-index (example-nums ex))
   (example-output ex)))

(defun check-all-examples ()
  (iterate
    (for example in *examples*)
    (for n upfrom 0)
    (if (check-example example)
        (format t "checked example ~d~%" n)
        (error "failed on example ~d~%~a~%" n example)))
  (format t "success!"))

(check-all-examples)
