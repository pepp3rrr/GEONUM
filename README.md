# GEONUM TPs

## Prérequis

Le SDK Rust est nécéssaire à l'exéctution du projet. Pour son installation je recommande [Rustup](https://rustup.rs/).

## TPS

Chaque TP est développé dans son propre module Rust, et peut dépendre de TPs précédents. Le module [common](./common) contient les types de base (Point, Vecteur) et quelques utilités communes.

Pour exécuter le code d'un TP, on a besoin de son nom de module :

 - [TP1](./tp1) : `tp1-bezier`

Que l'on utilise dans la commande

```sh
cargo run -p nom-de-module -- flags_dexecution
```

### TP2

```sh
# Afficher les options CLI
cargo run -p tp2-spline -- -h
# Génération de plot
cargo run -p tp2-spline -- -o plot.png tp2/data/simple.bcv
# Augmenter la résolution
cargo run -p tp2-spline -- -s 1000 -o plot.png tp2/data/simple.bcv
```

## Contributeurs

 - Gaspard Culis
 - Maxim Frolov
