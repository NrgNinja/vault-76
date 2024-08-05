import matplotlib.pyplot as plt
import pandas as pd

def strip_last(time): 
    if time[-2:] == "ms":
        time = float(time[:-2]) / 1000
        return time
    elif time[-1:] == "s":
        time = float(time[:-1])
        return time
    
machine_name = "torus"

for k in range(25, 31):
    threads = [1, 2, 4, 8, 16]
    bar_width = 0.2

    plt.figure(figsize=(12,6))
    
    plt.xlabel("Number of threads")
    plt.ylabel("Time (seconds)")
    plt.title(f"Workload: $\mathregular{2^{25}}$records")
    plt.xticks(ticks=list(range(len(threads))), labels=threads) 

    # Set up legend labels
    plt.bar(0, 0, color="#cfe2f3ff", label="Generation")
    plt.bar(0, 0, color="#e01e20", label="Writing")
    
    for i, thread_num in enumerate(threads):
        csv_file = f"gen_sort_csv/vault_{k}_{thread_num}t.csv"
        df = pd.read_csv(csv_file)
        
        df["Gen&Sort"] = df["Gen&Sort"].apply(strip_last)
        df["Writing"] = df["Writing"].apply(strip_last)
                
        avg_gen_time = round(df["Gen&Sort"].mean(), 2)
        avg_write_time = round(df["Writing"].mean(), 2)
                 
        # Adjusted positions
        plt.bar(i - bar_width / 2, avg_gen_time, width=bar_width, color="#cfe2f3ff", edgecolor="grey")
        plt.bar(i + bar_width / 2, avg_write_time, width=bar_width, color="#e01e20", edgecolor="grey")

        # Annotating bars
        plt.text(i - bar_width / 2, avg_gen_time + 0.05, f'{avg_gen_time}', ha="center", va="bottom")
        plt.text(i + bar_width / 2, avg_write_time + 0.05, f'{avg_write_time}', ha="center", va="bottom")

    plt.legend()
    plt.savefig(f"vault_plot/vault_{machine_name}_{k}.svg")