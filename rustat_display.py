import pygame


class RustatDisplay:
    window = None
    clock = None
    
    def __init__(self):
        self.window = pygame.display.set_mode((800, 800))
        self.clock = pygame.time.Clock()
    

    def tick(self):
        self.clock.tick(30)