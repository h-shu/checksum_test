import capnp
from crc32c import crc32c
import os

# 1. Load the schema
# Assuming the file above is saved as 'schema.capnp'
addressbook = capnp.load('addressbook.capnp')

# 2. Create an instance of the struct
my_data = addressbook.MyData.new_message()
my_data.a = 64
my_data.b = 128
my_data.c = 123.321
my_data.d = 123.123
my_data.e = True
my_data.f = "abc"
my_data.g = "def"
my_data.h = "ghi"

# 3. Serialize the struct to bytes
    # 'to_bytes_packed' is efficient for storage/transmission
binary_data = my_data.to_bytes()

# 4. Generate CRC32 Hash
# zlib.crc32 returns a signed 32-bit integer; 
hash_value = crc32c(binary_data)

print(f"Serialized Bytes: {binary_data.hex()}")
print(f"CRC32 Hash: {hash_value:08x}")
