use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// On garde le main lisible, en déléguant les responsabilités (chargement, saisie, correction) à des fonctions dédiées.
fn main() {

    // - Chargement d'un dictionnaire de mots.
    let dictionary = load_dictionary("dictionnaire.txt");

    // - Sélection aléatoire d'un mot secret.
    let secret_word = match dictionary.choose(&mut rand::thread_rng()) {
        Some(word) => word.clone(),
        None => {
            return;
        }
    };

    let max_attempts = 6;
    let mut attempts_left = max_attempts;

    println!("Bienvenue dans Wordle ! Devinez le mot, Vous avez {} tentatives.", max_attempts);

    // La boucle principale du jeu - avec chaque itération = une tentative.

    while attempts_left > 0 {
        println!("Il vous reste {} tentatives.", attempts_left);

        // On impose que le joueur entre un mot de longueur strictement égale à la longueur du mot secret.

        let guess = user_input(secret_word.len(), &dictionary);

        // Si le joueur trouve le mot.

        if guess == secret_word {
            println!("\x1b[32mBravo ! Vous avez trouvé le mot : {}\x1b[0m", secret_word);
            return;
        } else {

            // Sinon on affiche la correction.
            correction(&secret_word, &guess);
        }

        attempts_left -= 1;
    }

    // Sortie de la boucle , fin de partie.

    println!("\x1b[31mVous avez perdu ! Le mot était : {}\x1b[0m", secret_word);
}

/// Fonction chargée du chargement et du filtrage des mots du dictionnaire.
/// On choisit un mot de longueur comprise entre 4 et 6 pour que ce ne soit pas trop dur mais pas trop facile non plus.
fn load_dictionary(filename: &str) -> Vec<String> {
    if let Ok(lines) = read_lines(filename) {
        return lines
            .filter_map(Result::ok)
            .filter(|word| (4..=6).contains(&word.len()))
            .collect();
    }
    vec![]
}

/// Petite fonction utilitaire.
/// Elle permet de rendre la fonction `load_dictionary` plus lisible.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Fonction responsable de la saisie utilisateur.
/// Le choix ici est de forcer la bonne longueur du mot saisi pour garantir la coherence avec le mot secret.
/// Tant que l'utilisateur ne respecte pas la consigne on ne sort pas de la boucle.
fn user_input(length: usize, dictionary: &Vec<String>) -> String {
    loop {
        println!("Entrez un mot de {} lettres :", length);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("erreur");

        let input = input.trim().to_string();

        if input.len() != length {
            println!("Le mot doit contenir exactement {} lettres.", length);
            continue;
        }
	// Verifie que le mot entree par le joueur est un mot de la langue francaise
        if !dictionary.contains(&input) {
            println!("Ce mot n'existe pas dans le dictionnaire.");
            continue;
        }

        return input;
    }
}

/// Fonction d'affichage de la correction.
/// Compare le mot secret et le mot deviné pour afficher la couleur de chaque lettre.
/// On utilise les couleurs ANSI pour que ce soit plus visuel.
fn correction(secret: &str, guess: &str) {
    let mut correct = String::new();
    // La comparaison lettre à lettre repose sur le zip , on suppose que les deux mots ont même longueur (garantie en amont).
    for (s, g) in secret.chars().zip(guess.chars()) {
        if s == g {
            correct.push_str(&format!("\x1b[32m{}\x1b[0m", g)); // Vert 
        } else if secret.contains(g) {
            correct.push_str(&format!("\x1b[33m{}\x1b[0m", g)); // Jaune 
        } else {
            correct.push_str(&format!("\x1b[31m{}\x1b[0m", g)); // Rouge 
        }
    }
    println!("Mot saisi : {}", correct);
}
