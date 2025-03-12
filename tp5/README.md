# TP5

## Exécution

```sh
# Afficher les options CLI
cargo run -p tp5-uniform-spline -- -h
# Génération de plot
cargo run -p tp5-uniform-spline -- -m two-point -d 5 -o /tmp/plot.svg tp5/data/sumsign.data
```

## Rendus

### Two-point

#### Rendus de degré 3

![bone.data](https://github.com/user-attachments/assets/709e9c3e-8e9d-47b4-85ec-f3116952317c)

![hepta.data](https://github.com/user-attachments/assets/c25bf74b-95f2-4013-9408-e734d59c69df)

![infinity.data](https://github.com/user-attachments/assets/469d97b2-cabb-4b3f-8cbd-13903a298682)

![sumsign.data](https://github.com/user-attachments/assets/a71ec301-5da7-4e44-9429-634c39ffed24)

#### Rendus de degré 2

![sumsign.data](https://github.com/user-attachments/assets/e06a0572-4914-4c8e-9250-3ce25f390858)

#### Rendus de degré 1

![sumsign.data](https://github.com/user-attachments/assets/7951313a-6ac3-45eb-9648-73ea43b80618)

#### Rendus de degré 5

![sumsign.data](https://github.com/user-attachments/assets/23f9a5f4-3998-4dab-9239-9bbf3762830f)


### Four-point

#### Rendus de degré 3

![bone.data](https://github.com/user-attachments/assets/4aa1846f-bcab-4c49-be15-d943c98c455d)

![hepta.data](https://github.com/user-attachments/assets/ed772440-595d-4d66-b6ed-b9aca9d88ed7)

![infinity.data](https://github.com/user-attachments/assets/17340392-a1fb-4766-ace1-24138e0f0bbf)

![sumsign.data](https://github.com/user-attachments/assets/e9e823ac-4eca-4ad1-8bc8-b42a7c86a798)

#### Rendus de degré 2

![sumsign.data](https://github.com/user-attachments/assets/ba8423e2-61e0-4a80-b30f-13114cad970a)

#### Rendus de degré 1

![sumsign.data](https://github.com/user-attachments/assets/8a6d1b12-d48c-4770-9e18-dad6dcc9f111)

#### Rendus de degré 0

![sumsign.data](https://github.com/user-attachments/assets/4512389a-e39e-4760-bd1d-20c872146e40)

#### Rendus de degré 5

![sumsign.data](https://github.com/user-attachments/assets/305722ec-ed9d-46c7-a176-a700f3067390)

### Six-point

#### Rendus de degré 3

![bone.data](https://github.com/user-attachments/assets/f7082ae7-25d0-4f3e-a177-84c7059fb41a)

![hepta.data](https://github.com/user-attachments/assets/810d47b6-1e10-40c0-a500-2feeb77e5b5f)

![infinity.data](https://github.com/user-attachments/assets/a9f07110-9c84-43ce-b200-59670b61b1e1)

![sumsign.data](https://github.com/user-attachments/assets/4d3e79aa-1468-47e5-8ba9-f29d6454cd65)

Ici nous pensons que l'on a un bug, car nous n'avons pas les mêmes résultats que dans [le papier](https://onlinelibrary.wiley.com/doi/10.1155/2014/628285)

## Conclusion

La méthode qui (selon nous) donne le meilleur résultat est la méthode des quatre points avec un degré de trois, qui permet de conserver une forme nette et pas trop ronde tout en ayant de beaux arrondis. Nous pensons que c'est parce que l'utilisation de quatre points permet d'incorporer plus de détails dans les courbes. 
Si la méthode à six points n'était pas buguée, elle aurait pu être la méthode préférée.
