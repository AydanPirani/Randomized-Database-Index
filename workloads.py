import random
import sys
from generator import SequenceGenerator 

class WorkloadGenerator:
    def __init__(self, output_file, total_operations):
        self.total_operations = total_operations
        self.generator = SequenceGenerator(output_file)
        self.generator.setSerialize(True)

    def generate_read_heavy_workload(self, read_ratio=0.8):
        num_reads = int(self.total_operations * read_ratio)
        num_writes = self.total_operations - num_reads

        for _ in range(num_reads):
            key = random.randint(1, 100)  # Random key
            self.generator.createRead(key)

        for _ in range(num_writes):
            key = random.randint(1, 100)
            value = random.randint(1, 1000)
            self.generator.createWrite(key, value)

    def generate_write_heavy_workload(self, write_ratio=0.8):
        num_writes = int(self.total_operations * write_ratio)
        num_reads = self.total_operations - num_writes

        for _ in range(num_writes):
            key = random.randint(1, 100)
            value = random.randint(1, 1000)
            self.generator.createWrite(key, value)

        for _ in range(num_reads):
            key = random.randint(1, 100)
            self.generator.createRead(key)

    def generate_sequential_workload(self):
        for i in range(1, self.total_operations + 1):
            if i % 2 == 0:  # Alternate between reads and writes
                self.generator.createRead(i)
            else:
                self.generator.createWrite(i, i * 10)

    def generate_random_workload(self):
        for _ in range(self.total_operations):
            if random.random() < 0.5: 
                key = random.randint(1, 100)
                self.generator.createRead(key)
            else:
                key = random.randint(1, 100)
                value = random.randint(1, 1000)
                self.generator.createWrite(key, value)

    def generate_repeated_key_workload(self, key, duplicates):
        for _ in range(duplicates):
            self.generator.createRead(key)

        remaining_ops = self.total_operations - duplicates
        for _ in range(remaining_ops):
            key = random.randint(1, 100)
            value = random.randint(1, 1000)
            self.generator.createWrite(key, value)

if __name__ == "__main__":
    if len(sys.argv) < 3:
        raise Exception("Usage: python workloads.py <output_file> <total_operations>")

    output_file = sys.argv[1]
    total_operations = int(sys.argv[2])

    workload = WorkloadGenerator(output_file, total_operations)

    # Example usage:
    workload.generate_read_heavy_workload(read_ratio=0.9)  # Generate 90% reads, 10% writes
    # workload.generate_write_heavy_workload(write_ratio=0.7)  # Generate 70% writes, 30% reads
    # workload.generate_sequential_workload()  # Generate alternating sequential reads and writes
    # workload.generate_random_workload()  # Generate random reads and writes
    # workload.generate_repeated_key_workload(key=42, duplicates=50)  # 50 duplicate reads for key 42
