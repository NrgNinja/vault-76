import matplotlib.pyplot as plt
import numpy as np

# Example data
phases = ["Generation", "Sorting", "Writing to disk"]
threads = ["8 Threads", "16 Threads", "32 Threads"]

# Execution times for each phase with different number of threads
execution_times = {
    "Generation": [0.57491, 0.35645, 0.25022],
    "Sorting": [1.1789, 0.86441, 0.89731],
    "Writing to disk": [5.8331, 5.9697, 5.8114],
}

# Bar width
bar_width = 0.2

# X positions for the bars
r1 = np.arange(len(threads))
r2 = [x + bar_width for x in r1]
r3 = [x + bar_width for x in r2]

# Create the bar chart
plt.figure(figsize=(12, 6))

plt.bar(
    r1,
    execution_times["Generation"],
    color="#cfe2f3ff",
    width=bar_width,
    edgecolor="grey",
    label="Generation",
)
plt.bar(
    r2,
    execution_times["Sorting"],
    color="#ffe599ff",
    width=bar_width,
    edgecolor="grey",
    label="Sorting",
)
plt.bar(
    r3,
    execution_times["Writing to disk"],
    color="#e01e20",
    width=bar_width,
    edgecolor="grey",
    label="Writing to disk",
)

# Add labels and title
plt.xlabel("Number of Threads")
plt.ylabel("Execution Time (seconds)")
plt.title("Benchmark of Algorithms by Number of Threads")
plt.xticks([r + bar_width for r in range(len(threads))], threads)

# Add data labels on top of bars
for i in range(len(threads)):
    plt.text(
        r1[i],
        execution_times["Generation"][i] + 0.01,
        str(execution_times["Generation"][i]),
        ha="center",
        va="bottom",
    )
    plt.text(
        r2[i],
        execution_times["Sorting"][i] + 0.01,
        str(execution_times["Sorting"][i]),
        ha="center",
        va="bottom",
    )
    plt.text(
        r3[i],
        execution_times["Writing to disk"][i] + 0.01,
        str(execution_times["Writing to disk"][i]),
        ha="center",
        va="bottom",
    )

# Add legend
plt.legend()

# Display the plot
plt.show()
