import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

file_name = 'pi_4c'
file_path = 'vault_csv/' + file_name + '.csv'

df = pd.read_csv(file_path)

# Get unique drives and programs
drives = df['Drive'].unique()
programs = df['Program'].unique()

# Define bar width and positions
bar_width = 1.9 / len(programs)  # Adjust the bar width so bars fit within the x-axis ticks
bar_spacing = 0.25  # Define spacing between bars for different programs
group_spacing = 0.5  # Define spacing between k-value groups

# Create a figure and set of subplots
fig, axes = plt.subplots(1, len(drives), figsize=(14, 6), sharey=True)

if len(drives) == 1:
    axes = [axes]  # Ensure axes is iterable if there's only one subplot

colors = ['#93c47dff', '#a4c2f4ff', '#ea9999ff']  # Define custom colors

for ax, drive in zip(axes, drives):
    drive_df = df[df['Drive'] == drive]
    k_values = drive_df['k'].unique()
    
    # Increase spacing between k-value groups
    bar_positions = np.arange(len(k_values)) * (len(programs) * (bar_width + bar_spacing) + group_spacing)

    # Plot each program's bars
    for i, program in enumerate(programs):
        program_df = drive_df[drive_df['Program'] == program]
        bars = ax.bar(bar_positions + i * (bar_width + bar_spacing), program_df['Throughput'], 
                      width=bar_width, color=colors[i], label=program)

        # Add throughput labels on each bar
        for bar in bars:
            height = bar.get_height()
            ax.text(bar.get_x() + bar.get_width() / 2., height, f'{height:.1f}', 
                    ha='center', va='bottom', fontsize=9, rotation=30)

    ax.set_title(f'{drive}')
    ax.set_xlabel('k')
    ax.set_ylabel('Throughput, MB/s')
    ax.set_xticks(bar_positions + (len(programs) - 1) * (bar_width + bar_spacing) / 2)
    ax.set_xticklabels(k_values)
    ax.legend(title='Program', loc='upper right')
    
fig.text(0.5, 0.02, 'Raspberry Pi, 4 threads', ha='center', va='center', fontsize=12, fontweight='bold')

plt.tight_layout(rect=[0, 0, 1, 1])

plt.savefig('vault_plot/' + file_name + '.svg')
