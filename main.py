import time
from rustat import test_class

#print(test_func(1_000_000_000))

temp = test_class(1987)

amount = 1_000_000_000
step = amount / 20
next_step = step

temp.run(amount)

percent_done = 0
while percent_done < 100:
    if temp.get_count() >= next_step:
        next_step += step
        percent_done += 5
        print(f"{percent_done}% - {temp.get_count()}")
    time.sleep(.017)