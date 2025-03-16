# GEONUM TPs

## Prérequis

Le SDK Rust est nécéssaire à l'exéctution du projet. Pour son installation je recommande [Rustup](https://rustup.rs/).

Si vous avez des erreurs durant la compilation, essayez d'installer les librairies `build-essentials`, `pkg-config`, `libfreetype6-dev` et `libfontconfig1-dev` (sur un système basé sur Debian).

## TPS

Chaque TP est développé dans son propre module Rust, et peut dépendre de TPs précédents. Le module [common](./common) contient les types de base (Point, Vecteur) et quelques utilités communes.

Pour exécuter le code d'un TP, on a besoin de son nom de module :

 - [TP1](./tp1) : `tp1-bezier`
 - [TP2](./tp2) : `tp2-spline`
 - [TP4](./tp4) : `tp4-subdivision`
 - [TP5](./tp5) : `tp5-uniform-spline`
 - [TP6](./tp6) : `tp6-bezier-surface`

Que l'on utilise dans la commande

```sh
cargo run -p nom-de-module -- flags_dexecution
```

## Contributeurs

 - Gaspard Culis
 - Maxim Frolov
