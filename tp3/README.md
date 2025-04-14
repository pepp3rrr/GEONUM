# TP3

## Exécution

```sh
# Afficher les options CLI
cargo run -p tp3-bspline -- -h
# Génération de plot
cargo run -p tp3-bspline -- -o plot.svg tp3/data/simple.bcv
```

## Rendus

### BSpline

![simple.bspline](https://github.com/user-attachments/assets/dc5c7b08-1010-4655-947e-569979844419)

Après changement du dernier knot de `2` à `4` :

![simple.bspline](https://github.com/user-attachments/assets/9506e7ee-c829-4660-b2f2-221a9f404dd0)

Rien ne change...

![spiral.bspline](https://github.com/user-attachments/assets/172fda62-b716-4e9f-a527-ee7208647ca7)

Après changement des knots à `0 0 0 0 1 1 1 1 2 2 2 2 3 3 3 3 4 4 4 4 5 5 5 5` :

![spiral.bspline](https://github.com/user-attachments/assets/ab31453d-6e94-446e-9dd6-6e06698b912d)

Je ne sais pas trop quoi en dire...

![circle.bspline](https://github.com/user-attachments/assets/088d8739-5c61-4b00-bed2-897dec129110)

![camel.bspline](https://github.com/user-attachments/assets/6e8921f3-ed91-44ef-be89-25e83965b15d)

On bouge la patte

![camel.bspline](https://github.com/user-attachments/assets/804b2a40-9dbe-434b-8ae0-746d613b9188)


### NURBS

Notre implémentation est incomplète, mais nous permet quand même de faire les rendus de fichier NURBS.

![circle7.nurbs](https://github.com/user-attachments/assets/f7308b2e-9f22-491a-94ac-c7e46d6fd131)

![circle9.nurbs](https://github.com/user-attachments/assets/656bcddc-0519-4ba8-8ed4-172910d7246f)
