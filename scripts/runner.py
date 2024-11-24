import workloads
import os
import subprocess
import postprocessing
import matplotlib.pyplot as plt
from pathlib import Path


operation_cts = [100, 1000, 10000]
write_ratios = [0.1, 0.5, 0.9]
cycle_sizes = [5, 50, 500]
duplicate_counts = [5, 20, 50]
keys = [1, 50, 5000]



def random_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for write_ratio in write_ratios:
            name = f"{int(write_ratio*100)}-{operation_ct}-randomized"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_workload(write_ratio)
            del runner;
            print(os.getcwd())
            command = ["cargo", "run", sequence_file, output_file]
            print(" ".join(command))
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)

                
def run_workloads(sequence_dir, output_dir, figure_dir):
    random_workload(sequence_dir, output_dir, figure_dir)

if __name__ == "__main__":
    root = os.getenv('RDI_ROOT')
    if root is None:
        print("Must set RDI root!")
        exit(1)

    sequence_dir = f"{root}/sequences/generated"
    output_dir = f"{root}/out/generated"
    figure_dir = f"{root}/figures/generated"

    Path(f"{root}{sequence_dir}").mkdir(parents=True, exist_ok=True)
    Path(f"{root}{output_dir}").mkdir(parents=True, exist_ok=True)
    Path(f"{root}{figure_dir}").mkdir(parents=True, exist_ok=True)
    
    run_workloads(sequence_dir, output_dir, figure_dir)
