import numpy as np
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import sys

def get_plot(filepath, title):
    plt.rcParams.update({
    'axes.titlesize': 16,     # Title font size
    'axes.labelsize': 14,     # Axes label font size
    'xtick.labelsize': 12,    # X-axis tick font size
    'ytick.labelsize': 12,    # Y-axis tick font size
    'legend.fontsize': 12,    # Legend font size
    'figure.titlesize': 18    # Figure title font size
    })
    
    # Load the CSV data into a DataFrame
    print(f"{filepath=}")
    df = pd.read_csv(filepath)
    df["log_time"] = np.log2(df["time"])

    grouped = df.groupby(["index", "op"])["log_time"].apply(list).unstack(fill_value=[])

    # Create a 2-row, 1-column subplot
    plt.figure(figsize=(8, 8), num=title)
    # plt.title(sys.argv[1])

    # Plot the first subplot for Write Times
    plt.subplot(2, 1, 1)
    for index, times in grouped["write"].items():
        sns.kdeplot(times, label=str(index))

    plt.title("KDE Plot for Write Times")
    plt.xlabel("Log2 Time (Nanoseconds)")
    plt.ylabel("Density")
    plt.legend()

    # Plot the second subplot for Read Times
    plt.subplot(2, 1, 2)
    try: 
        for index, times in grouped["read"].items():
            sns.kdeplot(times, label=str(index))
    except Exception:
        print("Read DNE")
    plt.title("KDE Plot for Read Times")
    plt.xlabel("Log2 Time (Nanoseconds)")
    plt.ylabel("Density")
    plt.legend()

    # Adjust layout to prevent overlap
    plt.tight_layout()
    return plt


if __name__ == "__main__":
    plt = get_plot(sys.argv[1], sys.argv[2])
    plt.show()