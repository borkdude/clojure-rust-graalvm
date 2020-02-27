(ns clj-rust.core
  (:import [borkdude.clojure.rust ClojureRust])
  (:gen-class))

(defn -main
  [& [unit]]
  (if-not (contains? #{"byte" "megabyte" "gigabyte"} unit)
    (binding [*out* *err*]
      (println "Expected unit argument: byte, megabyte or gigabyte.")
      (when unit (println "Got:" unit)))
    (prn {:memory/free [(keyword unit) (ClojureRust/getFreeMemory unit)]})))
