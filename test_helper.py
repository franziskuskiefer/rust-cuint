import sys

if not len(sys.argv) == 3:
    print("ERROR; Usage: python test_helper.py number1 number2")
    exit(1)

print(hex(int(sys.argv[1], 16) + int(sys.argv[2], 16)))
