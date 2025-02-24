# geonum-common

Tous les TPs partagent use base de code commune contenue dans cette *crate*.

## Opérations arithmétiques

Les primitives `Point` et `Vec2` sont définies dans cette *crate*. Grâce au language *Rust*, nous avons pu implémenter les opérateurs standards +, -, *, etc (*operator overloading*) de manière à respecter les règles de calcul sur les points et vecteurs. Cela nous permet de garantir que nos opérations sont valides. Par exemple :

#### Soustraction de points

```rust
let a = Point::new(1., 3.);
let b = Point::new(2., 5.);

let result = a - b;
assert_eq!(result, Vec2::new(-1., -2.));
```

Donne un vecteur.

#### Combinaison point vecteur

```rust
let point = Point::new(1., 2.);
let vec = Vec2::new(7., 3.);

let result = point + vec;
assert_eq!(result, Point::new(8., 5.));
```

L'ajout d'un vecteur à un point retourne un nouveau point.

#### Combinaison barycentrique

```rust
let a = Point::new(1., 3.);
let b = Point::new(2., 5.);
let c = Point::new(-1., 2.);

let comb = (1. / 4.) * a + (2. / 4.) * b + (1. / 4.) * c;

assert_eq!(comb.into_point(), Point::new(1., 3.75));
```

Somme des facteurs égale à 1, la conversion `into_point` fonctionne.

```rust
let a = Point::new(1., 3.);
let b = Point::new(2., 5.);
let c = Point::new(-1., 2.);

let comb = (1. / 4.) * a + (3. / 4.) * b + (1. / 4.) * c;

let result = comb.into_point(); // Error: assertion failed
```

Somme des facteurs n'est pas égale à 1, la conversion `into_point` fait planter le programme.

## Chargement de fichier CSV

Notre crate déclare aussi une interface simple pour le chargement de données depuis un fichier CSV, et l'implémente pour certaines primitives (`impl FromCSV for Vec<Point>`).

## *Bounding box*

Pour effectuer le rendu graphique des courbes, nous devons savoir ses dimentions/la zone qu'elle occupe, cette crate définit donc l'interface `BoundingBox` qui fournint une méthode retournant un couple de points (rectangle) définissant la zone occupée par notre forme. Cette interface est aussi implémentée pour certaines primitives (`impl BoundingBox for Vec<Point>`).
