# TP1

## Exécution

```sh
# Afficher les options CLI
cargo run -p tp4-subdivision -- -h
# Génération de plot
cargo run -p tp4-subdivision -- -m four-point -o /tmp/plot.svg tp4/data/bunny.data
```

## Rendus

### Méthode Chaikin 

![simple.data](https://github.com/user-attachments/assets/9ae35fa4-7cbe-4719-809c-2e2ea83e0b7d)

![infinity.data](https://github.com/user-attachments/assets/7e669602-ac7d-4328-9d83-e35f1718dd66)

![bone.data](https://github.com/user-attachments/assets/e7059c46-e059-4fd6-92bb-1bb120236762)

![bunny.data](https://github.com/user-attachments/assets/f9df4f63-6a27-4202-b273-93dd179420a6)

### Méthode corner-cutting
($\alpha = 0.1$ ,  $\beta = 0.6$)

![simple.data](https://github.com/user-attachments/assets/82d573cd-9064-4144-8893-69c45b3e6d4f)

![infinity.data](https://github.com/user-attachments/assets/e116f3d5-d8fd-4099-ab7a-3157ee5c6f6d)

![bone.data](https://github.com/user-attachments/assets/4632e138-7772-4582-9c70-af4c9a4417fc)

![bunny.data](https://github.com/user-attachments/assets/67f73b80-c7d6-48a4-8078-6acac862e9bc)

### Méthode four-point

![simple.data](https://github.com/user-attachments/assets/0029f231-8a8e-4221-b9ba-1210f96e1748)

![infinity.data](https://github.com/user-attachments/assets/0941c69c-a0e1-49f5-8246-22bd41b442c3)

![bone.data](https://github.com/user-attachments/assets/102ce770-94ab-4b13-8fe3-7bf7a4a54e97)

![bunny.data](https://github.com/user-attachments/assets/48557a26-aa51-41d2-a52e-32e1bd41b86c)

### Méthode four-point (variation)
($\omega = 0.154$)

![bunny.data](https://github.com/user-attachments/assets/6d0ca104-e532-48b3-b13a-b8b4726c06eb)

