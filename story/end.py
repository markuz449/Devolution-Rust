import sys

arg_length = len(sys.argv)
game_over = []
for i in range (1, arg_length):
    with open(sys.argv[i], 'r') as f:

        file = f.read()     
        if "[Game Over]" in file:
            game_over.append(sys.argv[i])
        f.close()

print("Game Over Files:")
for j in range (0, len(game_over)):
    print(game_over[j])