import sys
import time
from rustat import TestClass

#print(test_func(1_000_000_000))

temp = TestClass()

amount = 1_000_000_000
step = amount / 25
next_step = step

temp.run(amount)

print("  [", end="")
sys.stdout.flush()

percent_done = 0
while percent_done < 100:
    if temp.get_count() >= next_step:
        next_step += step
        percent_done += 4
        print("#", end="")
        sys.stdout.flush()
        #print(f"{percent_done}% - {temp.get_count()}", end="")
    time.sleep(.017)
print("]")
print(f"  Finished counting to {amount}!")