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

![bone.data](https://github.com/user-attachments/assets/908bc8c5-60cc-44da-88a2-bd64baa9d879)

![hepta.data](https://github.com/user-attachments/assets/4eb73ee3-c83a-4639-8a01-3754a3f67403)

![infinity.data](https://github.com/user-attachments/assets/07d29c9c-7db3-46a0-937a-76ed35ee54f1)

![sumsign.data](https://github.com/user-attachments/assets/b0796f13-423a-4542-8672-4cae3e85cd78)

#### Rendus de degré 2

![sumsign.data](https://github.com/user-attachments/assets/57667087-f3a0-4329-bc7c-7de89c7c0c4e)

#### Rendus de degré 1

![sumsign.data](https://github.com/user-attachments/assets/8892af6e-7056-416a-b126-1d9ed0acb91e)

#### Rendus de degré 0

![sumsign.data](https://github.com/user-attachments/assets/4ce80884-ff20-4c9e-b31b-e5d0bd617f71)

#### Rendus de degré 5

![sumsign.data](https://github.com/user-attachments/assets/7f24bdb8-8f56-4f3c-bdde-328796f3ada2)

## Conclusion

La méthode qui (selon nous) donne le meilleur résultat est la méthode two-point avec un degré de deux, qui ajoute des arrondis simples et élégants, contrairement aux méthodes four-point et six-point qui ajoutent trop de complexité/détail, du fait qu'ils font la moyenne entre plus de points.
