// On déclare les fichiers comme étant des modules de notre projet
mod calcul;
mod lancement_py;
mod perso;
mod vect;

use std::f64::consts::PI;

use crate::calcul::Methode::EulerFixe;
// use crate::calcul::Astre;
// use crate::calcul::calculer_energie_totale;
// use crate::calcul::etape_leapfrog;
use crate::calcul::simulation;
use crate::lancement_py::lancer_graphique;
//use crate::vect::Vecteur2D;
use crate::calcul::Methode::Euler;
use crate::calcul::Methode::Leapfrog;

fn main() {
    let mon_email = "gwennaelle.airo-farulla@isen.yncrea.fr";
    const G: f64 = 4.0 * PI * PI; // G = 4π²

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

    // Paramètres temporels
    let dt = 0.0001;
    let temps_tot = 1.1;

    //Étape A1 - Un corps en orbite, Euler explicite
    //Mettre dt dans simualtion à 2.0
    simulation("csv/trajectoire_euler_fixe.csv", e_perso, G, dt, temps_tot, EulerFixe);
    println!("Simulation terminée ! Les données ont été sauvegardées dans leurs fichiers respectives'.");
    //Graphique Euler
    lancer_graphique("src/graphique.py", Some("euler"));

    // // Étape A2 - Deux corps mobiles, leapfrog, orbite personnalisée
    // simulation("csv/trajectoire_euler.csv", e_perso, G, dt, temps_tot, Euler);
    // println!("-> Euler explicite terminé.");
    // simulation("csv/trajectoire_leapfrog.csv", e_perso, G, dt, temps_tot, Leapfrog);
    // println!("-> Leapfrog terminé.");
    // //Graphique Leapfrog
    // lancer_graphique("src/graphique.py", Some("leapfrog"));
    // // Auto-diagnostique A2 dérive d'énergie relative 
    // // Mettre temps_tot dans simualtion à 50.0
    // lancer_graphique("src/graphique_derive_energie.py", None);

}
//     const MME: f64 = 1.660e-7; // Masse de Mercure
//     const MV: f64 = 2.447e-6; // Masse de Vénus
//     const MT: f64 = 3.003e-6; // Masse de Terre
//     const MMA: f64 = 3.213e-7; // Masse de Mars
//     const MJ: f64 = 9.543e-4; // Masse de Jupiter
//     const MSA: f64 = 2.857e-4; // Masse de Saturne
//     const MU: f64 = 4.365e-5; // Masse de Uranus
//     const MN: f64 = 5.150e-5; // Masse de Neptune
