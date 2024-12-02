import workloads
import os
import subprocess
import postprocessing
from pathlib import Path
from multiprocessing import Process

operation_cts = [100, 1000, 10000]
write_ratios = [0.1, 0.5, 0.9]
cycle_sizes = [5, 50, 500]
duplicate_counts = [5, 20, 50]
keys = [1, 50, 5000]


def random_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for write_ratio in write_ratios:
            name = f"randomized-{int(write_ratio*100)}write-{operation_ct}ops"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_workload(write_ratio)
            del runner;
            
            command = ["target/debug/randomized-database-indexes", sequence_file, output_file]
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)
            plt.close()


def sequential_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
            name = f"sequential-{operation_ct}ops"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_sequential_workload()
            del runner;

            command = ["target/debug/randomized-database-indexes", sequence_file, output_file]
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)
            plt.close()

def cyclic_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for write_ratio in write_ratios:
            for cycle_size in cycle_sizes:
                name = f"cyclic-{int(write_ratio*100)}write-{cycle_size}cycle-{operation_ct}ops"

                sequence_file = f"{sequence_dir}/{name}.seq"
                output_file = f"{output_dir}/{name}.csv"
                image_file = f"{figure_dir}/{name}.png"

                runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
                runner.generate_cyclic_workload(cycle_size=cycle_size, write_ratio=write_ratio)
                del runner;
                
                command = ["target/debug/randomized-database-indexes", sequence_file, output_file]
                subprocess.run(command)
                plt = postprocessing.get_plot(output_file, name)
                plt.savefig(image_file)
                plt.close()

def reverse_random_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for duplicate_ct in duplicate_counts:
            name = f"reverse-random-{duplicate_ct}dups-{operation_ct}ops"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_reverse_random_workload(duplicates=duplicate_ct)
            del runner;
            
            command = ["target/debug/randomized-database-indexes", sequence_file, output_file]
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)
            plt.close()

def reverse_repeated_workload(sequence_dir, output_dir, figure_dir):
    for operation_ct in operation_cts:
        for duplicate_ct in duplicate_counts:
            name = f"reverse-repeated-{duplicate_ct}dups-{operation_ct}ops"

            sequence_file = f"{sequence_dir}/{name}.seq"
            output_file = f"{output_dir}/{name}.csv"
            image_file = f"{figure_dir}/{name}.png"

            runner = workloads.WorkloadGenerator(output_file=sequence_file, total_operations=operation_ct)
            runner.generate_reverse_repeated_workload(duplicates=duplicate_ct)
            del runner;
            
            command = ["target/debug/randomized-database-indexes", sequence_file, output_file]
            subprocess.run(command)
            plt = postprocessing.get_plot(output_file, name)
            plt.savefig(image_file)
            plt.close()



def run_workloads(sequence_dir, output_dir, figure_dir):
    random_wl = Process(target=random_workload, args=(sequence_dir, output_dir, figure_dir))
    sequential_wl = Process(target=sequential_workload, args=(sequence_dir, output_dir, figure_dir))
    cyclic_wl = Process(target=cyclic_workload, args=(sequence_dir, output_dir, figure_dir))
    reverse_random_wl = Process(target=reverse_random_workload, args=(sequence_dir, output_dir, figure_dir))
    reverse_repeated_wl = Process(target=reverse_repeated_workload, args=(sequence_dir, output_dir, figure_dir))

    random_wl.start()
    sequential_wl.start()
    cyclic_wl.start()
    reverse_repeated_wl.start()
    reverse_random_wl.start()

    random_wl.join()
    sequential_wl.join()
    cyclic_wl.join()
    reverse_repeated_wl.join()
    reverse_random_wl.join()


if __name__ == "__main__":
    root = os.getenv('RDI_ROOT')
    if root is None:
        print("Must set RDI_ROOT env val to point to root directory!")
        exit(1)

    os.chdir(root)
    subprocess.run(["cargo", "build"])
    sequence_dir = f"{root}/sequences/generated"
    output_dir = f"{root}/out/generated"
    figure_dir = f"{root}/figures/generated"

    print(sequence_dir)
    Path(sequence_dir).mkdir(parents=True, exist_ok=True)
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    Path(figure_dir).mkdir(parents=True, exist_ok=True)
    
    run_workloads(sequence_dir, output_dir, figure_dir)
