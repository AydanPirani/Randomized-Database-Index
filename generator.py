import subprocess
import sys
import tempfile
import struct
import random

PROTO_FILE = 'proto/operation.proto'

class SequenceGenerator:
    def __init__(self, output_file):
        self.tmp_dir = tempfile.TemporaryDirectory(delete=False)
        self.output_file = open(output_file, "wb")
        self.default_serialize = False

        result = subprocess.run(
        ["protoc", PROTO_FILE, f"--python_out={self.tmp_dir.name}"],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )

        if result.returncode != 0:
            raise Exception(f"protoc error: {result.stderr.decode()}")

        sys.path.insert(0, f"{self.tmp_dir.name}/proto")

    def __del__(self):
        self.output_file.close()
        self.output_file = None

    def setSerialize(self, default_serialize):
        self.default_serialize = default_serialize

    def createRead(self, key, serialize=None):
        if serialize is None:
            serialize = self.default_serialize

        import operation_pb2
        op = operation_pb2.Op()
        op.read.key = key

        if serialize:
            self.serializeOp(op)
        return op

    def createWrite(self, key, val, serialize=None):
        if serialize is None:
            serialize = self.default_serialize

        import operation_pb2

        op = operation_pb2.Op()
        op.write.key = key
        op.write.value = val

        if serialize:
            self.serializeOp(op)
        return op    

    def serializeOp(self, op):
        serialized_message = op.SerializeToString()
        size = len(serialized_message)
        self.output_file.write(struct.pack('=I', size))
        self.output_file.write(serialized_message)
        print(f"Wrote message of {size} bytes!")

if __name__ == "__main__": 
    if len(sys.argv) < 2:
        raise Exception("Need to pass in output file!")

    s = SequenceGenerator(sys.argv[1])
    s.setSerialize(True)

    for i in range(5000):
        key = i*1000*random.random()
        s.createWrite(int(key), int(key))
    
    for i in range(2500):
        key = i*1000*random.random()
        s.createRead(int(key), int(key))