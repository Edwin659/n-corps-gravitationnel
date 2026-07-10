// On déclare le fichier perso.rs comme étant un module de notre projet
mod perso;
mod vect;
mod lancement_py;

use vect::Vecteur2D;
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;

use crate::lancement_py::lancer_graphique;

fn main() {
    let mon_email = "gwennaelle.airo-farulla@isen.yncrea.fr"; 
    const G:f64 = 4.0 * PI * PI; // G = 4π²
    let mut r:f64 =1.0; // Distance Terre-Soleil
    const MS:f64=1.0; //Masse du Soleil = 1

    // Conditions initiales de la Terre
    let mut pos_terre = Vecteur2D::new(1.0, 0.0); // À 1 UA sur l'axe X

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

    //On calcule la vitesse initiale v 
    let v = (G * MS/r).sqrt();
    println!("Vitesse initiale : {:.3}", v);
    let mut vit_terre = Vecteur2D::new(0.0, v); // Vitesse perpendiculaire (axe Y)

    //Temps
    let dt =0.0001; //moins de marge d'erreur
    let temps_tot =1.5;
    //let mut t=0.0;
    // On calcule le nombre total d'étapes nécessaires
    let nb_etapes = (temps_tot / dt) as usize;

    // On crée un fichier csv à la racine du projet
    let mut fichier = File::create("trajectoire.csv").expect("Impossible de créer le fichier CSV");
    // Écriture de l'en-tête du CSV
    writeln!(fichier, "temps,x,y").unwrap();

    for step in 0..nb_etapes{
        let t = (step as f64) *dt;
writeln!(fichier, "{:.4},{},{}", t, pos_terre.x, pos_terre.y).unwrap();

        //On calcule la distance entre la Terre et le Soleil
        r= pos_terre.norme();

        //On calcule l'accélération et son vecteur subis par la Terre
        let acceleration = -(G*MS)/r.powi(3);
        let vect_acceleration = pos_terre * acceleration;

        //Euler explicite
        vit_terre = vit_terre + (vect_acceleration *dt);
        pos_terre = pos_terre + (vit_terre *dt);


        //On ittére le temps
        //t+= dt;
    } 
    println!("Simulation terminée ! Les données ont été sauvegardées dans 'trajectoire.csv'.");
    lancer_graphique();
}