# GEONUM TPs

## Prérequis

Le SDK Rust est nécéssaire à l'exéctution du projet. Pour son installation je recommande [Rustup](https://rustup.rs/).

Si vous avez des erreurs durant la compilation, essayez d'installer les librairies `pkg-config`, `libfreetype6-dev` et `libfontconfig1-dev`.

## TPS

Chaque TP est développé dans son propre module Rust, et peut dépendre de TPs précédents. Le module [common](./common) contient les types de base (Point, Vecteur) et quelques utilités communes.

Pour exécuter le code d'un TP, on a besoin de son nom de module :

 - [TP1](./tp1) : `tp1-bezier`
 - [TP2](./tp2) : `tp2-spline`

Que l'on utilise dans la commande

```sh
cargo run -p nom-de-module -- flags_dexecution
```

## Contributeurs

 - Gaspard Culis
 - Maxim Frolov
