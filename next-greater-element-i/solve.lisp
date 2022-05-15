(defparameter *example-1*
  '((nums1 . (4 1 2))
    (nums2 . (1 3 4 2))
    (answer . (-1 3 -1))))

(defparameter *example-2*
  '((nums1 . (2 4))
    (nums2 . (1 2 3 4))
    (answer . (3 -1))))

(defun next-greater-element (element sequence)
  (let* ((element-position (position element sequence))
         (right-elements (nthcdr (+ 1 element-position) sequence))
         (greater-elements (remove-if (lambda (n) (<= n element)) right-elements)))
    (cond ((null greater-elements) -1)
          (t (first greater-elements)))))

(defun solve (nums1 nums2)
  (mapcar (lambda (element) (next-greater-element element nums2)) nums1))

(defun solve-example (example)
  (let ((nums1 (rest (assoc 'nums1 example)))
        (nums2 (rest (assoc 'nums2 example)))
        (answer (rest (assoc 'answer example))))
    (solve nums1 nums2)))

(defun solve-examples ()
  `((example-1 . ,(solve-example *example-1*))
    (example-2 . ,(solve-example *example-2*))))

(defun check-example-solutions (solutions)
  (let ((answer-1 (rest (assoc 'answer *example-1*)))
        (answer-2 (rest (assoc 'answer *example-2*)))
        (solution-1 (rest (assoc 'example-1 solutions)))
        (solution-2 (rest (assoc 'example-2 solutions))))
    (assert (equal answer-1 solution-1))
    (assert (equal answer-2 solution-2))))
