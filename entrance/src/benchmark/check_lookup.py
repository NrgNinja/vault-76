import sys
import pandas as pd

# Check if a filename was provided
if len(sys.argv) < 2:
    print("Usage: python check_lookup.py <filename>")
    sys.exit(1)

filename = sys.argv[1]

# Load the CSV file
data = pd.read_csv(filename)

# Normalize 'LookupTime' to milliseconds
def normalize_time(time_str):
    if 'µs' in time_str:
        # Convert microseconds to milliseconds
        return float(time_str.replace('µs', '')) / 1000
    else:
        # Remove 'ms' and convert
        return float(time_str.replace('ms', ''))

data['LookupTime(ms)'] = data['LookupTime(ms)'].apply(normalize_time)

# Filter the data into found and not found
found_data = data[data['IsExist'] == True]
not_found_data = data[data['IsExist'] == False]

# Compute min, max, and average for found
found_min = found_data['LookupTime(ms)'].min()
found_avg = found_data['LookupTime(ms)'].mean()
found_max = found_data['LookupTime(ms)'].max()

# Compute min, max, and average for not found
not_found_min = not_found_data['LookupTime(ms)'].min()
not_found_avg = not_found_data['LookupTime(ms)'].mean()
not_found_max = not_found_data['LookupTime(ms)'].max()

# Print results
print(f'Found: min={found_min:.2f}ms, average={found_avg:.2f}ms, max={found_max:.2f}ms')
print(f'Not Found: min={not_found_min:.2f}ms, average={not_found_avg:.2f}ms, max={not_found_max:.2f}ms')
