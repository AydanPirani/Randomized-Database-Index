import os
import pandas as pd
from dataclasses import dataclass
from collections import defaultdict
import numpy as np
import tempfile
import postprocessing

@dataclass
class ConfigItem:
    filename: str
    op_count: int | None
    write_ratio: float | None    
    cycle_size: int | None
    lookback: float | None
    filter: list | None

MAPPING = {
    "splaytree": "Splay Tree",
    "btree": "B-Tree",
    "skiplist": "Skip List",
    "treap_random": "Randomized Treap",
    "treap": "Treap",
    "scapegoat": "Scapegoat Tree"
}

CONFIG = {
    "randomized": ConfigItem("randomized-90write-10000ops",10000, 0.9, None, None, None),
    "cyclic": ConfigItem("cyclic-90write-500cycle-10000ops",10000, 0.9, 500, None, None),
    "sequential": ConfigItem("sequential-10000ops", 10000, None, None, None, None),
    "reverse-random": ConfigItem("reverse-random-50dups-10000ops", 10000, None, 50, None, None),
    "reverse-repeat": ConfigItem("reverse-repeated-50dups-10000ops", 10000, None, 50, None, None),
}

CONFIG["randomized"].filter = ["splaytree", "btree", "skiplist", "treap_random"]
CONFIG["cyclic"].filter = ["splaytree", "btree", "skiplist", "treap_random"]
CONFIG["sequential"].filter = ["splaytree", "btree", "skiplist", "treap_random"]
CONFIG["reverse-random"].filter = ["splaytree", "btree", "skiplist", "treap_random"]
CONFIG["reverse-repeat"].filter = ["splaytree", "btree", "skiplist", "treap_random"]

def main():
    root = os.getenv('RDI_ROOT')
    if root is None:
        print("Must set RDI_ROOT env val to point to root directory!")
        exit(1)

    csv_in_dir = f"{root}/out/generated"
    csv_out_dir = f"{root}/out/filtered"    
    
    fig_in_dir = f"{root}/figures/generated"
    fig_out_dir = f"{root}/figures/filtered"
    
    for workload, config_item in CONFIG.items():
        in_csv = f"{csv_in_dir}/{config_item.filename}.csv"
        in_df = pd.read_csv(in_csv)
        grouped = in_df.groupby(["index", "op"])["time"].apply(list).unstack(fill_value=[])
        data = []
        idx_mapping = {}
        
        for index, times in grouped["write"].items():
            median_write = np.median(times)
            idx_mapping[index] = len(data)
            data.append([index, median_write, 0])
            
        for index, times in grouped["read"].items():
            median_read = np.median(times)
            data[idx_mapping[index]][2] = median_read
        
        
        # with open(f"{csv_out_dir}/{workload}.tbl", "w+") as out_file:
        #     for line in data:
        #         index, read, write = line        
        #         out_file.write(f"{MAPPING[index]}&{read}&{write}\\\\\n\\hline\n")

        with tempfile.NamedTemporaryFile() as tmpfile:
            in_df = in_df[in_df["index"].isin(config_item.filter)]
            in_df["index"] = in_df["index"].map(MAPPING.get)
            in_df.to_csv(tmpfile, index=False)

            plt = postprocessing.get_plot(tmpfile.name, config_item.filename)
            plt.savefig(f"{fig_out_dir}/{workload}.png")

        

        

if __name__ == "__main__":
    main()