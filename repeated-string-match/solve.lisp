;; https://leetcode.com/problems/repeated-string-match/

(defpackage :repeated-string-match
  (:use :cl :iterate))
(in-package :repeated-string-match)

(defclass example ()
  ((a
    :initarg :a
    :accessor example-a)
   (b
    :initarg :b
    :accessor example-b)
   (output
    :initarg :output
    :accessor example-output)))

(defparameter *example-1*
  (make-instance 'example :a "abcd" :b "cdabcdab" :output 3))

(defparameter *example-2*
  (make-instance 'example :a "a" :b "aa" :output 2))


(defun repeat-string (str n)
  (apply #'concatenate 'string (iterate (repeat n) (collect str))))

(defun brute-force (a b)
  (iterate
    (for repetitions upfrom 0)
    (if (search b (repeat-string a repetitions)) (return repetitions))))

;; some limits
;; - the character that occurs the most in B, is at least the maximum amount of
;;   times A should be multiplied?

;; some examples
;; -------------
;; a: abc
;; b: abc
;;
;; a: abc / abcabc
;; b: bca
;;
;; a: abc
;; b: bcabca
;;
;; a: abc
;; b: bcafoobca
;;
;; a: abc
;; b: bcabbbca <- scary example, fails if we check b[0:len(a)] and b[(len(b)-len(a)):]
;;
;; can we check the set of letters?
;; - we can reject entirely if the set of all characters is not equivalent between A and B
;;
;; how do we brute-force?
;; - keep repeating A, check if A is a substring every time
;; - when would we stop?
;;   - if A is 2x the length of B?
;;
;;
;;
;; REPEATED-STRING-MATCH> (example-a *example-1*)
;; "abcd"
;; REPEATED-STRING-MATCH> (example-b *example-1*)
;; "cdabcdab"

;; (defun can-be-substring (a b)
;;   (let* ((start-b  (subseq b 0 (length)))
;;          ()
;;          (end-b    (subseq b))))

;;   (error "todo"))

(defun solve (a b)
  (error "todo"))

;; a = "abc"
;; b = "xyz"
;;
;; there is no way to repeat a to generate b
;;
;; i think that we can check every input that we get to see if this is possible
;;
;;
;; (a + a)
;; check whetehr len(a) characters at start of b contains a
;; check whether len(a) characters at end of b   contains a
;;
;;
;; validate whether we can do that
;;
;; CL-USER> (search "a" "abc")
;; 0 (0 bits, #x0, #o0, #b0)
;; CL-USER> (search "x" "xyz")
;; 0 (0 bits, #x0, #o0, #b0)
;; CL-USER> (search "a" "xyz")
;; NIL

;; CL-USER> (subseq "abcdefgh" 1 2)
;; "b"
;; CL-USER> (subseq "abcdefgh" 1 3)
;; "bc"

;; CL-USER> (length "string")
;; 6 (3 bits, #x6, #o6, #b110)

