import sys
import pandas as pd
import matplotlib.pyplot as plt

# Lecture de l'argument (par défaut 'leapfrog' si lancé à la main)
METHODE = sys.argv[1].lower() if len(sys.argv) > 1 else 'leapfrog'

print(f"[Trajectoire] Mode sélectionné : {METHODE}")

# Configuration dynamique
if METHODE == 'leapfrog':
    fichier_csv = "csv/trajectoire_leapfrog.csv"
    fichier_image = "image/trajectoire_leapfrog.png"
    titre = "Simulation de l'orbite avec n corps (Leapfrog)"
    afficher_points_finaux = False

elif METHODE == 'euler':
    fichier_csv = "csv/trajectoire_euler_fixe.csv"
    fichier_image = "image/trajectoire_euler_fixe.png"
    titre = "Simulation de l'orbite avec Soleil fixe (Euler Explicite)"
    afficher_points_finaux = True
else:
    raise ValueError(f"Méthode inconnue : {METHODE}")

# Lecture et nettoyage
df = pd.read_csv(fichier_csv)
df.columns = df.columns.str.strip()

# Tracer l'orbite
plt.figure(figsize=(6, 6), dpi=300)
plt.plot(df['x_terre'], df['y_terre'], label='Trajectoire Terre', color='#1f77b4', linewidth=0.8, antialiased=True)
plt.plot(df['x_soleil'], df['y_soleil'], label='Trajectoire Soleil (Barycentre)', color='orange', linewidth=1.5)

if afficher_points_finaux :
    plt.scatter(df['x_soleil'].iloc[-1], df['y_soleil'].iloc[-1], color='orange', s=100, edgecolors='black', zorder=5, label='Soleil (Fin)')
    plt.scatter(df['x_terre'].iloc[-1], df['y_terre'].iloc[-1], color='blue', s=50, edgecolors='black', zorder=2, label='Terre (Fin)')


plt.axis('equal')
plt.grid(True, which='both', linestyle='--', linewidth=0.5, color='gray', alpha=0.7)
plt.legend(loc='upper left', frameon=True, facecolor='white', edgecolor='none')
plt.xlabel("X (UA)", fontsize=10)
plt.ylabel("Y (UA)", fontsize=10)

plt.title(titre)
plt.savefig(fichier_image, dpi=300)
plt.show()