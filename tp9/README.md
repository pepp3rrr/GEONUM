# TP9

## Exécution

```sh
# Afficher les options CLI
cargo run -p tp9-subdivision-triangle -- -h
# Rendu d'un .off en mode wireframe
cargo run -p tp9-subdivision-triangle -- -w tp9/data/horse.off
```

## Rendus

### Calcul de poids β

![armadillo.off](https://github.com/user-attachments/assets/564024b1-0e35-48b8-985b-b830fea11d28)
*2 subdivision steps*

![bumpycube](https://github.com/user-attachments/assets/5ab6034b-8738-404b-a891-864e8cac8ef0)
*2 subdivision steps*

![cow.off](https://github.com/user-attachments/assets/8a223db1-80ff-4ac3-bdfa-d4193297d7c9)
*2 subdivision steps*

![cube.off](https://github.com/user-attachments/assets/86e5df69-9ffc-4363-accf-2f446a08d556)
*3 subdivision steps*

![horse.off](https://github.com/user-attachments/assets/8ceff8a9-070c-4c99-b06e-4ef0be716d97)
*2 subdivision steps*

![sphere.off](https://github.com/user-attachments/assets/a5255d28-fdbf-49a2-88fc-96c28be7882e)
*4 subdivision steps*

![testsurf.off](https://github.com/user-attachments/assets/212d0817-dde2-447e-bf43-48e714528c79)
*2 subdivision steps*

![tetra.off](https://github.com/user-attachments/assets/ee9df492-e4b4-4047-a044-95c322f7e3c3)
*2 subdivision steps*

### Calcul de poids Warren

![armadillo.off](https://github.com/user-attachments/assets/b183e5d3-e071-4174-bdb9-d543a1d1cf0d)

![bumpycube.off](https://github.com/user-attachments/assets/92488768-c8db-442c-89f2-ae0149d48107)

![cow.off](https://github.com/user-attachments/assets/ff895b41-1d91-4bb4-a65b-b637ef50a4cf)

![cube.off](https://github.com/user-attachments/assets/09919273-4548-4f4c-9bcd-3abb08cd23b9)

![horse.off](https://github.com/user-attachments/assets/6d774429-e89c-42af-813e-b736123e22be)

![sphere.off](https://github.com/user-attachments/assets/19e8185d-b7af-4489-aeef-013fc703eb0b)

![testsurf.off](https://github.com/user-attachments/assets/5c2ef8b5-c106-4657-8221-e9430c5fd713)

![tetra.off](https://github.com/user-attachments/assets/0b0c60ea-fa56-4fea-8c9a-e7f736bbb2a4)

La méthode de Warren n'introduit aucune différence visuelle notable, elle permet donc de simplifier les calculs à moindre coût.

### Comparaison cube et sphere pour 5 subdivisions

#### Cube

![cube.off](https://github.com/user-attachments/assets/3f96d689-9dcc-4800-994b-83a7eb928735)

#### Sphere

![sphere.off](https://github.com/user-attachments/assets/c3d96944-ed16-4901-af88-a2290694996c)

La sphère conserve très bien sa forme de sphère après 5 subdivisions, tandis que le cube tend vers une forme de sphère, mais avec seulement 5 subdivisions on voit encore apparaître les coins du cube. Le moyennage des points  est bien mieux réparti dans la sphère uniforme que dans les coins du cube.



