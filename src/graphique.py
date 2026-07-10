import pandas as pd
import matplotlib.pyplot as plt

# Charger les données générées par Rust
df = pd.read_csv("trajectoire.csv")

# Tracer l'orbite
plt.figure(figsize=(7, 7), dpi=300)
plt.plot(df['x'], df['y'], label='Trajectoire Terre', color='#1f77b4', linewidth=0.5, antialiased=True)
plt.scatter(0, 0, color='orange', s=120, edgecolors='black', zorder=5, label='Soleil')
plt.axis('equal')
plt.grid(True, which='both', linestyle='--', linewidth=0.5, color='gray', alpha=0.7)
plt.legend(loc='upper left', frameon=True, facecolor='white', edgecolor='none')
plt.xlabel("X (UA)", fontsize=10)
plt.ylabel("Y (UA)", fontsize=10)
plt.title("Simulation de l'orbite avec Euler Explicite")
plt.show()