use std::process::Command;
pub fn lancer_graphique(){
    println!("Lancement du script Python...");

    // Note : Selon ton système, remplace "python" par "python3" si nécessaire
    let statut = Command::new("python")
        .arg("src/graphique.py")
        .status() // Exécute le script et attend qu'il se termine
        .expect("Échec du lancement du processus Python");

    // On vérifie si le script Python s'est terminé sans erreur
    if statut.success() {
        println!("Le script Python s'est exécuté avec succès !");
    } else {
        println!("Le script Python a rencontré une erreur.");
    }
}