import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from scipy.interpolate import griddata

# Load the data
data = pd.read_csv('vault_csv/vault_k25.csv')

# Clean the data: Remove rows with missing or non-numeric values in 'hash_time' and 'sort_time'
data = data.dropna()
data = data[pd.to_numeric(data['hash_time'], errors='coerce').notnull()]
data = data[pd.to_numeric(data['sort_time'], errors='coerce').notnull()]

# Convert columns to numeric
data['threads'] = data['threads'].astype(int)
data['memory'] = data['memory'].astype(int)
data['hash_time'] = data['hash_time'].astype(float)
data['sort_time'] = data['sort_time'].astype(float)

# Calculate the sum of hash_time and sort_time
data['total_time'] = data['hash_time'] + data['sort_time']

# Get unique ticks from data
# threads_ticks = np.sort(data['threads'].unique())
# memory_ticks = np.sort(data['memory'].unique())

# Prepare the grid for interpolation
threads_range = np.linspace(data['threads'].min(), data['threads'].max(), 100)
memory_range = np.linspace(data['memory'].min(), data['memory'].max(), 100)
threads_grid, memory_grid = np.meshgrid(threads_range, memory_range)

# Interpolate for total_time
total_time_grid = griddata(
    (data['threads'], data['memory']),
    data['total_time'],
    (threads_grid, memory_grid),
    method='cubic'
)

# Plotting
fig = plt.figure(figsize=(10, 7))

# Total Time Surface Plot
ax = fig.add_subplot(111, projection='3d')
surf = ax.plot_surface(threads_grid, memory_grid, total_time_grid, cmap='viridis')
ax.view_init(elev=20, azim=-50)

# Set the ticks and labels
# ax.set_xticks(threads_ticks)
# ax.set_xticklabels([str(t) for t in threads_ticks])
# ax.set_yticks(memory_ticks)
# ax.set_yticklabels([str(m) for m in memory_ticks])

ax.set_xlabel('Threads')
ax.set_ylabel('Memory (MB)')
ax.set_zlabel('Total Time (s)')
ax.set_title('Total Time Surface Plot')
fig.colorbar(surf, ax=ax, shrink=0.5, aspect=5)

plt.tight_layout()
# plt.subplots_adjust(left=0.1, right=0.9, top=0.9, bottom=0.1)
plt.show()

plt.savefig(f"vault_plot/vault_k25.svg")