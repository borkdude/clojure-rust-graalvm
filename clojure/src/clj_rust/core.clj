(ns clj-rust.core
  (:import [borkdude.clojure.rust ClojureRust])
  (:gen-class))

(defn -main
  [& _args]
  (println "Hello from Clojure")
  (println (ClojureRust/helloFromRust)))
