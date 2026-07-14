use crate::vect::Vecteur2D;
use std::fs::File;
use std::io::Write;

use std::f64::consts::PI;


#[derive(Clone, Copy)]
pub struct Astre {
    pub pos: Vecteur2D,
    pub vit: Vecteur2D,
    pub masse: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Methode {
    EulerFixe,
    Euler,
    Leapfrog,
}

// Fonction utilitaire pour calculer les accélérations mutuelles de deux astres
fn calculer_accelerations(terre: &Astre, soleil: &Astre, G: f64) -> (Vecteur2D, Vecteur2D) {
    let vecteur_r = terre.pos - soleil.pos;
    let r = vecteur_r.norme();

    // Accélération subie par la Terre, puis par le Soleil
    let acc_terre = vecteur_r * (-(G * soleil.masse) / r.powi(3));
    let acc_soleil = vecteur_r * ((G * terre.masse) / r.powi(3));

    (acc_terre, acc_soleil)

}

// Fonction pour calculer l'énergie totale du système à un instant T
pub fn calculer_energie_totale(terre: &Astre, soleil: &Astre, G: f64) -> f64 {
    let r = (terre.pos - soleil.pos).norme();
    let ec_terre = 0.5 * terre.masse * terre.vit.norme_carree();
    let ec_soleil = 0.5 * soleil.masse * soleil.vit.norme_carree();
    let ep = -(G * terre.masse * soleil.masse) / r;

    ec_terre + ec_soleil + ep
}

// --- LES MÉTHODES D'INTÉGRATION ---

pub fn etape_euler_explicite_fixe(terre: &mut Astre, soleil: &mut Astre, G: f64, dt: f64){

    //Calcul de la distance r
    let r = terre.pos.norme();

    //Calcul de l'accélération
    let intensite_acc = - (G * soleil.masse) / (r.powi(3));
    let acceleration = terre.pos * intensite_acc;

    terre.pos = terre.pos + (terre.vit*dt);
    terre.vit = terre.vit + (acceleration*dt);
}

pub fn etape_euler_explicite(terre: &mut Astre, soleil: &mut Astre, G: f64, dt: f64) {
    let (acc_terre, acc_soleil) = calculer_accelerations(terre, soleil, G);

    // Mise à jour des positions (vitesse courante)
    terre.pos = terre.pos + terre.vit * dt;
    soleil.pos = soleil.pos + soleil.vit * dt;

    // Mise à jour des vitesses
    terre.vit = terre.vit + acc_terre * dt;
    soleil.vit = soleil.vit + acc_soleil * dt;
}

pub fn etape_leapfrog(terre: &mut Astre, soleil: &mut Astre, G: f64, dt: f64) {
    let (acc_terre, acc_soleil) = calculer_accelerations(terre, soleil, G);

    // Demi-pas de vitesse
    terre.vit = terre.vit + acc_terre * (dt / 2.0);
    soleil.vit = soleil.vit + acc_soleil * (dt / 2.0);

    // Pas complet de position
    terre.pos = terre.pos + terre.vit * dt;
    soleil.pos = soleil.pos + soleil.vit * dt;

    // Recalcul des accélérations avec les nouvelles positions
    let (acc_terre_suiv, acc_soleil_suiv) = calculer_accelerations(terre, soleil, G);

    // Second demi-pas de vitesse
    terre.vit = terre.vit + acc_terre_suiv * (dt / 2.0);
    soleil.vit = soleil.vit + acc_soleil_suiv * (dt / 2.0);
}

// --- SIMULATION ---

pub fn simulation(
    nom_fichier: &str,
    e_perso: f64,
    G: f64,
    dt: f64,
    temps_tot: f64,
    methode: Methode,
) {
    
    // Initialisation selon la methode
    let (mut terre, mut soleil) = match methode {
        Methode::EulerFixe => {
            // Cas Soleil fixe : la Terre est à 1.0 UA et le Soleil est pile au centre (0,0)
            let t = Astre {
                pos: Vecteur2D::new(1.0, 0.0),
                vit: Vecteur2D::new(0.0, (G * 1.0 / 1.0).sqrt()),
                masse: 3.003e-6,
            };
            let s = Astre {
                pos: Vecteur2D::new(0.0, 0.0), // Fixé à l'origine
                vit: Vecteur2D::new(0.0, 0.0),
                masse: 1.0,
            };
            (t, s)
        }
        _ => {
            // Cas n-corps (Euler ou Leapfrog) : le Soleil bouge autour du barycentre
            let a = 1.0;
            let v_p = (G * 1.0 * (1.0 + e_perso) / (a * (1.0 - e_perso))).sqrt();
            
            let t = Astre {
                pos: Vecteur2D::new(1.0 * (1.0 - e_perso), 0.0),
                vit: Vecteur2D::new(0.0, v_p),
                masse: 3.003e-6,
            };
            let s = Astre {
                pos: t.pos * (-t.masse / 1.0),
                vit: t.vit * (-t.masse / 1.0),
                masse: 1.0,
            };
            (t, s)
        }
    };
    // Paramètres temporels
    let mut t = 0.0;

    // Calcul de l'énergie initiale
    let e_0 = calculer_energie_totale(&terre, &soleil, G);
    let mut derive_e = 0.0;

    // On crée un fichier csv à la racine du projet
    let mut fichier = File::create(nom_fichier).expect("Impossible de créer le fichier CSV");
    // Écriture de l'en-tête du CSV
    writeln!(fichier, "temps,x_soleil,y_soleil,x_terre,y_terre,derive_E ").unwrap();

    while t < temps_tot {
        writeln!(
            fichier,
            "{:.4},{},{},{},{},{}",
            t, soleil.pos.x, soleil.pos.y, terre.pos.x, terre.pos.y, derive_e
        )
        .unwrap();

        // --- CHOIX DE LA MÉTHODE D'INTÉGRATION ---
        match methode {
            Methode::EulerFixe => etape_euler_explicite_fixe(&mut terre, &mut soleil, G, dt),
            Methode::Euler => etape_euler_explicite(&mut terre, &mut soleil, G, dt),
            Methode::Leapfrog => etape_leapfrog(&mut terre, &mut soleil, G, dt),
        }
        // Mise à jour du temps et de la dérive d'énergie
        t += dt;
        let e_t = calculer_energie_totale(&terre, &soleil, G);
        derive_e = (e_t - e_0).abs() / e_0.abs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vitesse_initiale_soleil_fixe() {
        // --- Constantes de simulation ---
        let g = 4.0 * PI.powi(2); 
        let e_perso = 0.0; // Orbite circulaire
        let a = 1.0;       // 1 UA

        // --- Calcul de la vitesse initiale ---
        let v_p = (g * 1.0 * (1.0 + e_perso) / (a * (1.0 - e_perso))).sqrt();

        // --- Valeur théorique attendue ---
        let v_theorique = 2.0 * PI; // 2π ≈ 6.283185307

        // --- Vérification avec tolérance stricte ---
        let tolerance = 1e-12; 
        let ecart = (v_p - v_theorique).abs();

        println!("--- Analyse Vitesse Initiale ---");
        println!("Vitesse calculée (v_p)   : {:.12} UA/an", v_p);
        println!("Vitesse théorique (2π)   : {:.12} UA/an", v_theorique);
        println!("Écart absolu             : {:.2e}", ecart);

        assert!(
            ecart < tolerance,
            "Alerte ! La vitesse calculée ({}) s'écarte de 2π ({}). Vérifie ta formule de v_p ou ta constante G !",
            v_p,
            v_theorique
        );
    }

    #[test]
    fn test_orbite_leapfrog_extremums() {
        // --- Paramètres physiques et numériques ---
        let G:f64 = 39.47841760435743; // Constante G adaptée à ton échelle (UA, années, masses solaires)
        let e_perso:f64 = 0.0167;      // Excentricité de test
        let dt:f64 = 0.001;            // Pas de temps fin pour la précision
        let temps_tot:f64 = 1.0;       // On simule exactement 1 orbite (1 an)

        // --- Initialisation des astres (comme dans ta simulation) ---
        let a:f64 = 1.0;
        let v_p = (G * 1.0 * (1.0 + e_perso) / (a * (1.0 - e_perso))).sqrt();

        let mut terre = Astre {
            pos: Vecteur2D::new(1.0 * (1.0 - e_perso), 0.0),
            vit: Vecteur2D::new(0.0, v_p),
            masse: 3.003e-6,
        };

        let mut soleil = Astre {
            pos: terre.pos * (-terre.masse / 1.0),
            vit: terre.vit * (-terre.masse / 1.0),
            masse: 1.0,
        };

        // --- Variables pour suivre la distance minimale et maximale ---
        let mut dist_min = f64::INFINITY;
        let mut dist_max = f64::NEG_INFINITY;
        let mut t = 0.0;

        // --- Boucle de simulation (en mémoire, pas d'écriture de fichier !) ---
        while t < temps_tot {
            let distance = (terre.pos - soleil.pos).norme();
            
            if distance < dist_min {
                dist_min = distance;
            }
            if distance > dist_max {
                dist_max = distance;
            }

            etape_leapfrog(&mut terre, &mut soleil, G, dt);
            t += dt;
        }

        // --- Valeurs théoriques ---
        let periastre_theorique = a * (1.0 - e_perso);
        let apoastre_theorique = a * (1.0 + e_perso);

        // --- Vérifications (Seuil de tolérance de 0.5%) ---
        let tolerance = 0.005; // 0.5% d'erreur maximum acceptée

        let erreur_relative_peri = (dist_min - periastre_theorique).abs() / periastre_theorique;
        let erreur_relative_apo = (dist_max - apoastre_theorique).abs() / apoastre_theorique;

        println!("Périastre mesuré: {}, Théorique: {}, Erreur: {:.4}%", dist_min, periastre_theorique, erreur_relative_peri * 100.0);
        println!("Apoastre mesuré: {}, Théorique: {}, Erreur: {:.4}%", dist_max, apoastre_theorique, erreur_relative_apo * 100.0);

        // Les assertions qui font réussir ou échouer le test :
        assert!(
            erreur_relative_peri < tolerance,
            "Le périastre mesuré ({}) est trop éloigné de la théorie ({})", 
            dist_min, periastre_theorique
        );
        
        assert!(
            erreur_relative_apo < tolerance,
            "L'apoastre mesuré ({}) est trop éloigné de la théorie ({})", 
            dist_max, apoastre_theorique
        );
    }
}