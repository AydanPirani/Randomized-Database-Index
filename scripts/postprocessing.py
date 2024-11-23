import numpy as np
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import sys

# Load the CSV data into a DataFrame
df = pd.read_csv(sys.argv[1])
df["log_time"] = np.log2(df["time"])

grouped = df.groupby(["index", "op"])["log_time"].apply(list).unstack(fill_value=[])

# Create a 2-row, 1-column subplot
plt.figure(figsize=(8, 8))

# Plot the first subplot for Write Times
plt.subplot(2, 1, 1)
# for column in write_data:
#     sns.kdeplot(write_data[column], label=str(column))

for index, times in grouped["write"].items():
    sns.kdeplot(times, label=str(index))
plt.title("KDE Plot for Write Times")
plt.xlabel("Time")
plt.ylabel("Density")
plt.legend()

# Plot the second subplot for Read Times
plt.subplot(2, 1, 2)
# for column in read_data:
#     sns.kdeplot(read_data[column], label=str(column))

for index, times in grouped["read"].items():
    sns.kdeplot(times, label=str(index))
plt.title("KDE Plot for Read Times")
plt.xlabel("Time")
plt.ylabel("Density")

plt.legend()

# Adjust layout to prevent overlap
plt.tight_layout()

# Show the plot
plt.show()
