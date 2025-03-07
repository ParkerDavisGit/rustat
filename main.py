import sys
import time
import rustat

#print(test_func(1_000_000_000))

temp = rustat.load_file("test_prices")

amount = 2_000_000_00
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
    
    time.sleep(.017)

print("]")
print(f"  Finished counting to {amount}!")

temp.set_in_dict("test1", 27)
print(temp.get_from_dict("test1"))