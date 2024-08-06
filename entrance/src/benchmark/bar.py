import matplotlib.pyplot as plt
import pandas as pd

# def strip_last(time): 
#     if time[-2:] == "ms":
#         time = float(time[:-2]) / 1000
#         return time
#     elif time[-1:] == "s":
#         time = float(time[:-1])
#         return time
    
machine_name = "torus"

# for k in range(25, 31):
threads = [1, 2, 4, 8, 16]
bar_width = 0.2

plt.figure(figsize=(12,6))

plt.xlabel("Number of threads")
plt.ylabel("Time (seconds)")
plt.title(f"Vault 76 time on Torus, memory limit 16384 MB with k=32")
plt.xticks(ticks=list(range(len(threads))), labels=threads) 

# Set up legend labels
plt.bar(0, 0, color="#cfe2f3ff", label="Generation & Writing")
plt.bar(0, 0, color="#e01e20", label="Sorting File")
plt.bar(0, 0, color="#00FF00", label="Syncing File")

for i, thread_num in enumerate(threads):
    csv_file = f"gen_sort_csv/vault76_{machine_name}_32_{thread_num}t.csv"
    df = pd.read_csv(csv_file)
    
    # df["Gen&Flush"] = df["Gen&Flush"].apply(strip_last)
    # df["Sort"] = df["Sort"].apply(strip_last)
    # df["Sync"] = df["Sync"].apply(strip_last)
            
    avg_gen_time = round(df["Gen&Flush"].mean(), 2)
    avg_sort_time = round(df["Sort"].mean(), 2)
    avg_sync_time = round(df["Sync"].mean(), 2)
                
    # Adjusted positions
    plt.bar(i - bar_width / 2, avg_gen_time, width=bar_width, color="#cfe2f3ff", edgecolor="grey")
    plt.bar(i + bar_width / 2, avg_sort_time, width=bar_width, color="#e01e20", edgecolor="grey")
    plt.bar(i + (bar_width * 3) / 2, avg_sync_time, width=bar_width, color="#00FF00", edgecolor="grey")
    
    # Annotating bars
    plt.text(i - bar_width / 2, avg_gen_time + 0.05, f'{avg_gen_time}', ha="center", va="bottom")
    plt.text(i + bar_width / 2, avg_sort_time + 0.05, f'{avg_sort_time}', ha="center", va="bottom")
    plt.text(i + (bar_width * 3) / 2, avg_sync_time + 0.05, f'{avg_sync_time}', ha="center", va="bottom")

    plt.legend()
    plt.savefig(f"vault76_plot/vault76_{machine_name}_32.svg")
