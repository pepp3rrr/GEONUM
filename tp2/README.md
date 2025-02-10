# TP2

## Exécution

```sh
# Afficher les options CLI
cargo run -p tp2-spline -- -h
# Génération de plot
cargo run -p tp2-spline -- -o plot.png tp2/data/simple.bcv
# Augmenter la résolution
cargo run -p tp2-spline -- -s 1000 -o plot.png tp2/data/simple.bcv
```
