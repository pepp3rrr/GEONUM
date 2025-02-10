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

## Rendus

![simple.bcv](https://github.com/user-attachments/assets/1561f9b0-aafd-4c18-abea-a229b273a017)

![semi.bcv](https://github.com/user-attachments/assets/ddb460a9-5a69-4342-9f9c-7b477f8088d8)

![infinity.bcv](https://github.com/user-attachments/assets/064f9458-a3ac-4c52-8bf7-6b012c247d02)

![spiral.bcv](https://github.com/user-attachments/assets/4b6e28fb-272e-49b7-9197-fc1bf440c674)

### Changement de position initiale

Si on change la position initiale de $b^0_1$ de $0.5(p_0+p_1)$ à $0.1(p_0+p_1)$, on observe que les angles entre les différentes courbes de bezier deviennent moins ronds :

![semi-tweaked.bcv](https://github.com/user-attachments/assets/60e1d39b-2ea7-447d-8d5c-a88ce1d1b74e)




