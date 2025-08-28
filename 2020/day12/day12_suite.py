class Ship:
    def __init__(self):
        self.position = [0, 0]  # [east, north]
        self.waypoint = [10, 1]  # [east, north], RELATIVE TO SHIP

    def turn_waypoint(self, command):
        direction, angle = command

        # Reference : sens trigo
        if direction == 'R':
            direction = 'L'
            angle = 360 - angle

        if direction == 'L':
            while angle > 0:
                east, north = self.waypoint
                self.waypoint[0] = - north  # east = - north
                self.waypoint[1] = east  # north = east
                angle -= 90
        else:
            raise ValueError(f"In TURN_WAYPOINT function, direction should be 'R' or 'L', not: {direction}")

    def move_waypoint(self, command):
        direction, distance = command

        if direction == 'N':
            self.waypoint[1] += distance
        elif direction == 'S':
            self.waypoint[1] -= distance
        elif direction == 'E':
            self.waypoint[0] += distance
        elif direction == 'W':
            self.waypoint[0] -= distance
        else:
            raise ValueError(f"In MOVE_WAYPOINT function, direction should be 'N', 'S', 'E' or 'W', not: {direction}")

    def move_ship(self, command):
        _, nb_times = command
        self.position[0] += nb_times * self.waypoint[0]
        self.position[1] += nb_times * self.waypoint[1]

    def follow_instruction(self, command):
        direction, _ = command
        if direction in ['L', 'R']:
            self.turn_waypoint(command)
        elif direction in ['N', 'S', 'E', 'W']:
            self.move_waypoint(command)
        elif direction == 'F':
            self.move_ship(command)
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
    print(f"ship position: {ship.position}, waypoint relative position: {ship.waypoint}")

    for ins in instructions:
        ship.follow_instruction(ins)
        #print(f"ship position: {ship.position}, waypoint relative position: {ship.waypoint}")

    print(f"ship position: {ship.position}, waypoint relative position: {ship.waypoint}")
    print(ship.manhattan_distance())


main()
