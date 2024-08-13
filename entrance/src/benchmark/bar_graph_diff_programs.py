import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

file_name = '8s_128c_k32'
file_path = 'vault_csv/' + file_name + '.csv'
df = pd.read_csv(file_path)

# Get unique programs and drives
programs = df['Program'].unique()
drives = df['Drive'].unique()

# Define bar width and positions with additional spacing between bars
bar_width = 0.15  # Width of the bars (reduced width for spacing)
bar_spacing = 0.05  # Spacing between bars within a group
drive_spacing = 0.1  # Extra spacing between groups of bars for different drives

# Calculate bar positions
bar_positions = np.arange(len(programs)) * (len(drives) * (bar_width + bar_spacing) + drive_spacing)

colors = ['#93c47dff', '#a4c2f4ff', '#ea9999ff'] 

# Create a figure and axis
fig, ax = plt.subplots(figsize=(12, 6))

# Plot each drive's data with specified colors
for i, (drive, color) in enumerate(zip(drives, colors)):
    drive_df = df[df['Drive'] == drive]

    # Ensure that data aligns correctly with the unique programs
    throughput_values = []
    for program in programs:
        # Filter the data for the current program and drive
        throughput = drive_df[drive_df['Program'] == program]['Throughput']

        # If the throughput data exists, use it; otherwise, append 0 or NaN
        if not throughput.empty:
            throughput_values.append(throughput.mean())  # Average duplicates if any
        else:
            throughput_values.append(0)

    # Plot the bars for the current drive with a specific color
    bars = ax.bar(bar_positions + i * (bar_width + bar_spacing), throughput_values, 
                  width=bar_width, color=color, label=drive)

    # Add throughput labels with some spacing above the bars
    for bar in bars:
        height = bar.get_height()
        ax.text(
            bar.get_x() + bar.get_width() / 2.,  # X-coordinate
            height + 1,  # Y-coordinate with spacing
            f'{height:.1f}',  # Label with one decimal place
            ha='center', va='bottom', fontsize=10
        )

# Set the labels and title
ax.set_xlabel('Program')
ax.set_ylabel('Throughput (MB/s)')
ax.set_xticks(bar_positions + (len(drives) - 1) * (bar_width + bar_spacing) / 2)
ax.set_xticklabels(programs)
ax.set_title('8Socket, 128 threads, k=32', fontweight='bold')

# Add a legend
ax.legend(title='Drive')

# Show the plot
plt.tight_layout()
plt.savefig('vault_plot/' + file_name + '.svg')
