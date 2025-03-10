import sys
import time
import rustat
import pygame
from concurrent.futures import ThreadPoolExecutor

import rustat_controller as RC
import rustat_display as RD


class Rustat:
    menu = None
    display = None
    running = False

    def __init__(self):
        self.menu = RC.RustatController()
        self.display = RD.RustatDisplay()

        self.running = True
    

    def main_display_loop(self):
        while self.running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.running = False

            self.display.tick()

        pygame.quit()


temp = Rustat()
#temp.main_display_loop()

with ThreadPoolExecutor() as executor:
    futures = executor.submit(temp.main_display_loop)





#print(test_func(1_000_000_000))

temp = rustat.load_file("test_prices")

amount = 50_000_000
step = amount / 100
next_step = step

temp.run(amount)


### MAIN GAME LOOP PROTOTYPING ###
# while true
# 
# 
# 
#
#
#
#
###


def getMenuRequest():
    return 7

i = getMenuRequest()
if i != 0:
    print(i)

print("  [", end="")
sys.stdout.flush()

percent_done = 0
while percent_done < 100:
    while temp.get_count() >= next_step and percent_done < 100:
        next_step += step
        percent_done += 1
        print("#", end="")
        sys.stdout.flush()
    
    time.sleep(.017)


print("]")
print(f"  Finished counting to {amount}!")

temp.set_in_dict("test1", 27)
print(temp.get_from_dict("test1"))