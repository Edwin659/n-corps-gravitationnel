import pandas as pd
import matplotlib.pyplot as plt

# Lecture des deux fichiers générés par Rust
df_euler = pd.read_csv("csv/trajectoire_euler.csv")
df_euler.columns = df_euler.columns.str.strip()
df_lf = pd.read_csv("csv/trajectoire_leapfrog.csv")
df_lf.columns = df_lf.columns.str.strip()
plt.figure(figsize=(10, 6))

# Tracé des courbes de dérive d'énergie
plt.plot(df_euler['temps'], df_euler['derive_E'], label='Euler Explicite (Ordre 1)', color='#e74c3c', linewidth=1.5)
plt.plot(df_lf['temps'], df_lf['derive_E'], label='Leapfrog (Symplectique - Ordre 2)', color='#2ecc71', linewidth=1.5)

# Configuration de l'affichage
plt.yscale('log') # Indispensable pour comparer des ordres de grandeur très différents !
plt.xlabel('Temps (années / orbites)')
plt.ylabel(r"Dérive d'énergie relative $|E(t) - E(0)| / |E(0)|$")
plt.title('Évolution de la dérive d\'énergie relative sur 50 orbites')
plt.grid(True, which="both", linestyle="--", alpha=0.5)
plt.legend(loc="upper left")

# Sauvegarde et affichage
plt.savefig('image/derive_energie_comparaison.png', dpi=300)
plt.show()

# Trouver le maximum de la colonne 'derive_E'
max_euler = df_euler['derive_E'].max()
max_lf = df_lf['derive_E'].max()

print("-" * 50)
print(f"Dérive d'énergie relative maximale :")
print(f" -> Euler Explicite : {max_euler}")
print(f" -> Leapfrog        : {max_lf}")
print("-" * 50)