import matplotlib.pyplot as plt
import numpy as np

def plot_performance(threads, gen_times, write_times):
    # Set the width of the bars
    bar_width = 0.35

    # Set the positions of the bars
    index = np.arange(len(threads))

    fig, ax = plt.subplots()
    gen_bars = ax.bar(index, gen_times, bar_width, label='Hash Generation Time', color='red')
    write_bars = ax.bar(index + bar_width, write_times, bar_width, label='Write-to-Disk Time', color='blue')

    # Add some text for labels, title and custom x-axis tick labels, etc.
    ax.set_xlabel('Number of Threads')
    ax.set_ylabel('Time (seconds)')
    ax.set_title('33 Million Records on Macbook Air (M2)')
    ax.set_xticks(index + bar_width / 2)
    ax.set_xticklabels(threads)
    ax.legend()

    fig.tight_layout()
    plt.show()

# Sample data - replace these with your actual data
threads = ['1', '2', '4', '8']
gen_times = [7.0, 3.8, 2.1, 1.9]  # Example generation times
write_times = [0.6, 0.8, 0.9, 0.7]  # Example write-to-disk times

plot_performance(threads, gen_times, write_times)