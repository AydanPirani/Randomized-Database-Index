import random
import sys
from generator import SequenceGenerator 

class WorkloadGenerator:
    def __init__(self, output_file, total_operations):
        self.total_operations = total_operations
        self.generator = SequenceGenerator(output_file)
        self.generator.setSerialize(True)

    def generate_workload(self, write_ratio=0.8):
        for _ in range(self.total_operations):
            if random.random() < write_ratio: 
                key = random.randint(1, 100)
                value = random.randint(1, 1000)
                self.generator.createWrite(key, value)
            else:
                key = random.randint(1, 100)
                self.generator.createRead(key)
        
    def generate_sequential_workload(self):
        for i in range(1, self.total_operations + 1):
            if i % 2 == 0:  # Alternate between reads and writes
                self.generator.createRead(i)
            else:
                self.generator.createWrite(i, i * 10)

    def generate_cyclic_workload(self, cycle_size=10, write_ratio=0.8):
        for key in range(self.total_operations):
            key = key % cycle_size
            if random.random() < write_ratio: 
                value = random.randint(1, 1000)
                self.generator.createWrite(key, value)
            else:
                self.generator.createRead(key)
        
    def generate_repeated_key_workload(self, duplicates):
        remaining_ops = self.total_operations - duplicates
        key = None
        for _ in range(remaining_ops):
            key = random.randint(1, 100)
            value = random.randint(1, 1000)
            self.generator.createWrite(key, value)

        for _ in range(duplicates):
            self.generator.createRead(key, key * 10)

if __name__ == "__main__":
    if len(sys.argv) < 3:
        raise Exception("Usage: python workloads.py <output_file> <total_operations>")

    output_file = sys.argv[1]
    total_operations = int(sys.argv[2])

    workload = WorkloadGenerator(output_file, total_operations)

    # Example usage:
    # workload.generate_workload(write_ratio=0.7)  # Generate 70% writes, 30% reads
    # workload.generate_repeated_key_workload(key=42, duplicates=50)  # 50 duplicate reads for key 42
    workload.generate_cyclic_workload(write_ratio=0.95)