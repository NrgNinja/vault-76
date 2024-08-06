import matplotlib.pyplot as plt
import numpy as np

# Data
years =       [1956, 1962, 1970, 1979, 1991, 1996, 2000, 2005, 2007, 2011, 2012, 2019, 2020, 2026]
capacities = [0.005, 0.028, 0.020, 1.0, 9.0, 25.0, 1000, 500, 1000, 4000, 10000, 20000, 20000, 50000]

# Convert capacities from GB to TB for plotting
capacities_TB = [x / 1000 for x in capacities]

# Create the plot
plt.figure(figsize=(10, 5))
plt.plot(years, capacities_TB, marker='o', linestyle='-', color='b')

# Set the scale of the y-axis to logarithmic
plt.yscale('log')

# Customizing the plot
plt.title('Growth of HDD Storage Capacity (1956-Present)')
plt.xlabel('Year')
plt.ylabel('Storage Capacity (TB)')
plt.grid(True, which="both", ls="--")

# Adding annotations for specific points
for i, txt in enumerate(capacities_TB):
    plt.annotate(f"{capacities[i]} GB", (years[i], capacities_TB[i]))

# Show the plot
plt.tight_layout()
plt.savefig(f"stats/HDDGrowthGraph.svg")
