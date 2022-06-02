ORDER = 6

tens = 1
for i in range(ORDER):
    with open(f"lines{tens}.txt", "w") as fout:
        fout.writelines([f"{j}\n" for j in range(tens)]) 
    tens *= 10
