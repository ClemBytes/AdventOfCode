class Ship:
    def __init__(self):
        self.position = [0, 0]  # [east, north]
        self.orientation = "E"

    def turn(self, command):
        direction, angle = command
        order = ["E", "N", "W", "S"] * 2

        # Reference : sens trigo
        if direction == 'R':
            direction = 'L'
            angle = 360 - angle

        if direction == 'L':
            while angle > 0:
                self.orientation = order[order.index(self.orientation) + 1]
                angle -= 90
        else:
            raise ValueError(f"In TURN function, direction should be 'R' or 'L', not: {direction}")

    def move(self, command):
        direction, distance = command

        if direction == 'N':
            self.position[1] += distance
        elif direction == 'S':
            self.position[1] -= distance
        elif direction == 'E':
            self.position[0] += distance
        elif direction == 'W':
            self.position[0] -= distance
        elif direction == 'F':
            self.move((self.orientation, distance))
        else:
            raise ValueError(f"In MOVE function, direction should be 'N', 'S', 'E', 'W' or 'F', not: {direction}")

    def follow_instruction(self, command):
        direction, _ = command
        if direction in ['L', 'R']:
            self.turn(command)
        elif direction in ['N', 'S', 'E', 'W', 'F']:
            self.move(command)
        else:
            raise ValueError(f"Unknown command: '{command}'")

    def manhattan_distance(self):
        east, north = self.position
        return abs(east) + abs(north)


def main():
    # Read input
    f = open("./input")
    instructions = [(line[0], int(line[1:])) for line in f]

    ship = Ship()
    print(ship.position, ship.orientation)

    for ins in instructions:
        ship.follow_instruction(ins)

    print(ship.position, ship.orientation)
    print(ship.manhattan_distance())


main()
