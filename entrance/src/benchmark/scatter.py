import pandas as pd
import matplotlib.pyplot as plt

def strip_last(lookup_time):
    if lookup_time[-2:] == "ms":
        return lookup_time[:-2]
    elif lookup_time[-2:] == "µs":
        return float(lookup_time[:-2]) / 1000

for k in [25, 30]:
    for hash_len in [4, 8, 12]:
        csv_file = f"lookup_csv/lookup_{k}_{hash_len}.csv"
        
        df = pd.read_csv(csv_file)
        
        df["LookupTime(ms)"] = df["LookupTime(ms)"].apply(strip_last)
        df["LookupTime(ms)"] = df["LookupTime(ms)"].astype(float)
        
        found_true = df[df["IsExist"] == True]
        found_false = df[df["IsExist"] == False]
        
        fig, ax = plt.subplots()
        plt.scatter(found_true.index, found_true["LookupTime(ms)"], label='Found', s=2, color="green")
        plt.scatter(found_false.index, found_false["LookupTime(ms)"], label='Not Found', s=2, color="red")
        
        avg_lookup_time = df["LookupTime(ms)"].mean()
        plt.axhline(y=avg_lookup_time, color="blue", linestyle="-", linewidth=1, label="Average")
        plt.text(0.5, avg_lookup_time, f'{avg_lookup_time:.2f} ms', color='blue', weight='bold',
                 verticalalignment='bottom', horizontalalignment='center', transform=ax.get_yaxis_transform())
        
        plt.legend()
        
        plt.title(f'Lookup Times in 2^{k} records w/ a prefix length of {hash_len}')
        plt.xlabel('Lookup #')
        plt.ylabel('Lookup Time (ms)')
        
        plt.savefig(f"lookup_plot/lookup_{k}_{hash_len}_plot.svg")