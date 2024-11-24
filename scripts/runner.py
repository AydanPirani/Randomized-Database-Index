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
            name = f"{int(write_ratio*100)}write-{operation_ct}ops-randomized"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_workload(write_ratio)
            del runner;

            
            command = ["cargo", "run", sequence_file, output_file]
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)


def sequential_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
            name = f"{operation_ct}ops-sequential"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_sequential_workload()
            del runner;

            command = ["cargo", "run", sequence_file, output_file]
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)

def cyclic_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for write_ratio in write_ratios:
            for cycle_size in cycle_sizes:
                name = f"{int(write_ratio*100)}write-{cycle_size}cycle-{operation_ct}ops-cyclic"

                sequence_file = f"{sequence_dir}/{name}.seq"
                output_file = f"{output_dir}/{name}.csv"
                image_file = f"{figure_dir}/{name}.png"

                runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
                runner.generate_cyclic_workload(cycle_size=cycle_size, write_ratio=write_ratio)
                del runner;
                
                command = ["cargo", "run", sequence_file, output_file]
                subprocess.run(command)
                plt = postprocessing.get_plot(output_file, name)
                plt.savefig(image_file)

def repeated_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for write_ratio in write_ratios:
            for duplicate_ct in duplicate_counts:
                name = f"{int(write_ratio*100)}write-{duplicate_ct}dups-{operation_ct}ops-repeated"

                sequence_file = f"{sequence_dir}/{name}.seq"
                output_file = f"{output_dir}/{name}.csv"
                image_file = f"{figure_dir}/{name}.png"

                runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
                runner.generate_cyclic_workload(duplicate_ct=duplicate_ct, write_ratio=write_ratio)
                del runner;
                
                command = ["cargo", "run", sequence_file, output_file]
                subprocess.run(command)
                plt = postprocessing.get_plot(output_file, name)
                plt.savefig(image_file)


def run_workloads(sequence_dir, output_dir, figure_dir):
    random_workload(sequence_dir, output_dir, figure_dir)
    sequential_workload(sequence_dir, output_dir, figure_dir)
    cyclic_workload(sequence_dir, output_dir, figure_dir)
    repeated_workload(sequence_dir, output_dir, figure_dir)

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
