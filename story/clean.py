import sys

arg_length = len(sys.argv)
re_check = []
for i in range (1, arg_length):
    with open(sys.argv[i], 'r') as f:
        start_speech = 0
        end_speech = 0

        file = f.read()
        if ". . ." in file:
            re_check.append(sys.argv[i])
        f.close()

if len(re_check) == 0:
    print("There are no files to fix")
else:
    print("Story files to re check:")
    for j in range (0, len(re_check)):
        print(re_check[j])