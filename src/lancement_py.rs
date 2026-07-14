use std::process::Command;
pub fn lancer_graphique(nom_script: &str, argument: Option<&str>) {
    println!("Lancement du script Python...");

    let mut commande = Command::new("python");
    commande.arg(nom_script);

    if let Some(arg) = argument {
        commande.arg(arg);
    }

    let statut = commande
        .status() // Exécute le script et attend qu'il se termine
        .expect("Échec du lancement du processus Python");

    // On vérifie si le script Python s'est terminé sans erreur
    if statut.success() {
        println!("Le script Python {} s'est exécuté avec succès !", nom_script);
    } else {
        println!("Le script Python {} a rencontré une erreur.", nom_script);
    }
}
