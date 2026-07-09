use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vecteur2D {
    pub x: f64,
    pub y: f64,
}

impl Vecteur2D{
    pub fn new(x:f64,y:f64)-> Vecteur2D{
        Vecteur2D { x, y }
    }

    pub fn produit_scalaire(self, other: Vecteur2D)-> f64{
        self.x * other.x + self.y * other.y
    }

    pub fn norme_carree(self)->f64{
        self.x *self.x +self.y * self.y
    }

    pub fn norme(self)->f64{
        self.norme_carree().sqrt()
    }
}

impl Add for Vecteur2D {
    type Output = Vecteur2D;
    fn add(self, other: Vecteur2D) -> Vecteur2D {
        Vecteur2D{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vecteur2D {
    type Output = Vecteur2D;
    fn sub(self, other: Vecteur2D) -> Vecteur2D {
        Vecteur2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1=Vecteur2D::new(4.0,5.0);
        let v2 = Vecteur2D::new(2.0,3.0);
        assert_eq!(v1+v2, Vecteur2D::new(6.0,8.0));
    }

    #[test]
    fn test_sub() {
        let v1=Vecteur2D::new(4.0,5.0);
        let v2 = Vecteur2D::new(2.0,3.0);
        assert_eq!(v1-v2, Vecteur2D::new(2.0,2.0));
    }

    #[test]
    fn test_norme() {
        let v1=Vecteur2D::new(3.0,4.0);
        assert_eq!(v1.norme(), 5.0);
    }
    
    #[test]
    fn test_produit_scalaire() {
        let v1=Vecteur2D::new(4.0,5.0);
        let v2 = Vecteur2D::new(2.0,3.0);
        assert_eq!(v1.produit_scalaire(v2), 23.0);
    }
}
