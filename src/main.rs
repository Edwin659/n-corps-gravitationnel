// On déclare le fichier perso.rs comme étant un module de notre projet
mod perso;
mod vect;

fn main() {
    let mon_email = "gwennaelle.airo-farulla@isen.yncrea.fr"; 
    
    // On appelle la fonction en précisant qu'elle vient du module 'perso'
    let (seed, e_perso, theta_perso) = perso::calculer_parametres_perso(mon_email);

    println!("==================================================");
    println!("          PARAMÈTRES DE PERSONNALISATION          ");
    println!("==================================================");
    println!("Email utilisé : {}", mon_email);
    println!("Graine (seed) : {}", seed);
    println!("e_perso       : {:.2}", e_perso);
    println!("theta_perso   : {:.2}", theta_perso);
    println!("==================================================");
}