import numpy as np
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

# Load the CSV data into a DataFrame
df = pd.read_csv("out.csv")

indexes = df["index"].unique()

write_data = {i: [] for i in indexes}
read_data = {i: [] for i in indexes}

for _, row in df.iterrows():
    index = row["index"]
    op = row["op"]
    key = row["key"]
    time = np.log2(row["time"])

    if op == "write":
        write_data[index].append(time)
    else:
        read_data[index].append(time)

# plt.figure(figsize=(14, 7))

# for column in write_data:
#     sns.kdeplot(write_data[column], label=str(column))

# plt.figure(figsize=(14, 7))
# for column in read_data:
#     sns.kdeplot(read_data[column], label=str(column))


# plt.title('KDE Plot for Write Times')
# plt.xlabel('Time')
# plt.ylabel('Density')
# plt.xlim([0, 10000])
# # plt.savefig('kde_plot.png')  # You can change the file extension to '.pdf', '.svg', etc.

# plt.legend()
# # Show the plot
# plt.show()

# Create a 2-row, 1-column subplot
plt.figure(figsize=(8, 8))

# Plot the first subplot for Write Times
plt.subplot(2, 1, 1)
for column in write_data:
    sns.kdeplot(write_data[column], label=str(column))
plt.title("KDE Plot for Write Times")
plt.xlabel("Time")
plt.ylabel("Density")
# plt.xlim([0, 10])
plt.legend()

# Plot the second subplot for Read Times
plt.subplot(2, 1, 2)
for column in read_data:
    sns.kdeplot(read_data[column], label=str(column))
plt.title("KDE Plot for Read Times")
plt.xlabel("Time")
plt.ylabel("Density")

plt.legend()

# Adjust layout to prevent overlap
plt.tight_layout()

# Show the plot
plt.show()
