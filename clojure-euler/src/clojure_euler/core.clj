(ns clojure-euler.core
  (:gen-class))

(defn euler-1
  "Find the sum of all the multiples of 3 or 5 below n."
  [n]
  (reduce + (filter #(or (= 0 (mod % 3)) (= 0 (mod % 5))) (range n))))

;; via tail-recursion
(defn euler-2
  "Find the sum of all the even Fibonacci numbers below n."
  [n]
  (loop [a 1
         b 2
         sum 0]
    (if (>= b n)
      sum
      ;; tail recursive call, send a->b, a+b->a, ...->sum
      (recur b (+ a b) (if (even? b) (+ sum b) sum)))))

;; via lazy-seq
(defn euler-2a
  "Find the sum of all the even Fibonacci numbers below n."
  [n]
  ; define a closure that generates the Fibonacci sequence
  (letfn [(fib [a b]
            (lazy-seq (cons a (fib b (+ a b)))))]
    (reduce + (filter even? (take-while #(< % n) (fib 1 2))))))

(defn euler-3 "Find the largest prime factor of n." [n]
  ;; first remove any factors of 2
  (loop [n n largest 1]
    (if (zero? (mod n 2))
      (recur (/ n 2) 2)
      ;; then remove odd factors
      (loop [n n f 3 largest largest]
        (if (>= f n)
          (max largest n)
          (if (zero? (mod n f))
            (recur (/ n f) f f)
            (recur n (+ f 2) largest)))))))

(defn euler-3-refactor "Find the largest prime factor of n." [n]
  (letfn [(remove-factors [n f]
            (loop [n n]
              (if (zero? (mod n f))
                (recur (/ n f))
                n)))]
    (let [largest (if (zero? (mod n 2)) 2 1)
          n (remove-factors n 2)]
      (loop [n n f 3 largest largest]
        (if (>= f n)
          (max largest n)
          (if (zero? (mod n f))
            (recur (/ n f) f f)
            (recur n (+ f 2) largest)))))))
(map euler-3-refactor (range 2 20))
;; (map euler-3 (range 2 20))

;; (defn euler-4 "Find the largest palindrome made from the product of two 3-digit numbers." []
(defn euler-4
  "Find the largest palindrome made from the product of two 3-digit numbers."
  []
  (letfn [(palindrome? [n] (= (seq (str n)) (reverse (str n))))]
    (apply max (filter palindrome?
                       (for [x (range 100 1000) y (range 100 1000)] (* x y))))))


(defn euler-5 "Find the smallest number that is evenly divisible by all numbers from 1 to n." [n]
  ;; todo
  )

(defn -main [& _args]
  (println (euler-1 1000))
  (println (euler-2 4000000))
  (println (euler-2a 4000000))
  (println (euler-3 600851475143))
  (println (euler-4)))
