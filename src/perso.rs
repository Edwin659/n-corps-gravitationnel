use sha2::{Digest, Sha256};

// Le mot-clé 'pub' permet à main.rs d'accéder à cette fonction
pub fn calculer_parametres_perso(email: &str) -> (u32, f64, f64) {
    let identifiant = email.trim().to_lowercase();

    let mut hasher = Sha256::new();
    hasher.update(identifiant.as_bytes());
    let hash_result = hasher.finalize();

    let h_hex = format!("{:x}", hash_result);
    let seed_str = &h_hex[0..8];
    let seed = u32::from_str_radix(seed_str, 16).unwrap();

    let e_perso = 0.05 + 0.50 * ((seed % 11) as f64) / 10.0;
    let e_perso_arrondi = (e_perso * 100.0).round() / 100.0;

    let theta_perso = 0.2 + 0.1 * ((seed % 9) as f64);
    let theta_perso_arrondi = (theta_perso * 100.0).round() / 100.0;

    (seed, e_perso_arrondi, theta_perso_arrondi)
}

// Les tests restent ici, avec la fonction qu'ils testent !
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_regression_jean_dupont() {
        //let email_test = "gwennaelle.airo-farulla@isen.yncrea.fr";
        let email_test = "jean.dupont@isen.fr";
        let (seed, e_perso, theta_perso) = calculer_parametres_perso(email_test);

        assert_eq!(seed, 3188418341);
        assert_eq!(e_perso, 0.5);
        assert_eq!(theta_perso, 0.7);
    }
}