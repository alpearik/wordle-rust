# Wordle

Ce projet est une implémentation du célèbre jeu **Wordle**, codée en Rust. 
L'objectif est de deviner un mot secret choisi aléatoirement parmi un dictionnaire de la langue francaise, avec un système de correction coloré.

## Fonctionnalités

- Sélection aléatoire d'un mot secret à partir d’un fichier dictionnaire.
- Saisie utilisateur
- Affichage coloré des lettres :
  
  - 🟩 **Vert** : bonne lettre à la bonne place.
  - 🟨 **Jaune** : bonne lettre à la mauvaise place.
  - 🟥 **Rouge** : lettre absente du mot.

## Règles du jeu

- Le mot secret a entre 4 et 6 lettres.
- Vous avez **6 tentatives** pour le deviner.
- À chaque essai, une correction vous indique quelles lettres sont bien placées, mal placées ou absentes.
- Le mot proposé **doit exister dans le dictionnaire**.

## Lancer le jeu

1. Assurez-vous d’avoir Rust installé sur votre machine.
2. Clonez ce dépôt.
3. Compilez le projet
```bash
cargo run
