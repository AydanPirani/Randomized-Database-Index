import subprocess
import sys
import tempfile
import struct

PROTO_FILE = 'proto/operation.proto'

class SequenceGenerator:
    def __init__(self, output_file):
        self.tmp_dir = tempfile.TemporaryDirectory(delete=False)
        self.output_file = open(output_file, "wb+")

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


    def createRead(self, key, serialize=False):
        import operation_pb2
        read_op = operation_pb2.ReadOp()
        read_op.key = key

        if serialize:
            self.serializeOp(read_op)
        return read_op


    def createWrite(self, key, val, serialize=False):
        import operation_pb2
        write_op = operation_pb2.WriteOp()
        write_op.key = key
        write_op.value = val

        if serialize:
            self.serializeOp(write_op)
        return write_op    


    def serializeOp(self, op):
        serialized_message = op.SerializeToString()
        size = len(serialized_message)
        self.output_file.write(struct.pack('>I', size))
        self.output_file.write(serialized_message)
        print(f"Wrote message of {size} bytes!")


if __name__ == "__main__": 
    if len(sys.argv) < 2:
        raise Exception("Need to pass in output file!")

    s = SequenceGenerator(sys.argv[1])
    # op = s.createRead(12, True)
    op = s.createWrite(12, 12, True)
    