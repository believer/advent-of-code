let data = [
  "......#...........#...#........",
  ".#.....#...##.......#.....##...",
  "......#.#....#.................",
  "..............#.#.......#......",
  ".....#.#...##...#.#..#..#..#..#",
  ".......##...#..#...........#...",
  ".......#.##.#...#.#.........#..",
  "..#...##............##......#.#",
  ".......#.......##......##.##.#.",
  "...#...#........#....#........#",
  "#............###.#......#.....#",
  "..#........#....#..#..........#",
  "..#..##....#......#..#......#..",
  "........#......#......#..#..#..",
  "..#...#....#..##.......#.#.....",
  ".....#.#......#..#....#.##.#..#",
  "......###.....#..#..........#..",
  ".#................#.#..........",
  ".........#..#...#......##......",
  "##...#....#...#.#...#.##..#....",
  "...##...#....#.........###.....",
  ".#.#....#.........##...........",
  "....#.#..#..#...........#......",
  "..#..#.#....#....#...#.........",
  "..........##.....#.##..........",
  "..#.#....#..##......#.#.....##.",
  "..#...#.##......#..........#...",
  "......#....#..#.....#.....#...#",
  "#.#...##.#.##.........#..#.....",
  "...#.#.#.........#.....#.#.#...",
  "..#.........#...............#..",
  "#..##.....#.........#....#.....",
  "...#....##..##...........##..#.",
  "......##.................#.#...",
  "##.......#....#.#.#.....#......",
  "....#.#...#.................##.",
  "#...#.........##.....#.........",
  "#....#.###..#.....##.#....#....",
  "#..#....#...#....#.#.#.........",
  ".......#...........#....#.....#",
  "#...#.............#........#...",
  ".......#.....#...#..#.........#",
  ".##.....##.....##.......#......",
  "....##...##.......#..#.#.....#.",
  ".##.........#......#........##.",
  ".......#...#...###.#..#........",
  "..#..###......##..##...........",
  ".#..#......##..#.#.........#...",
  "...#.......#........#...#.#....",
  "...#....#..#....#.....##.......",
  "............#......#..........#",
  ".#.......#......#.#....#..#.#..",
  "##.........#.#.#..........#....",
  "....##.....#...................",
  ".......#..#........#...........",
  "....##.#..#......###.......#...",
  "....#....#...#.#......#...#...#",
  ".......#.....##..#....#...#....",
  "#...#........#.........#..##...",
  "...........##.........#.#...#..",
  "....................#....#.##..",
  ".#..#..#.........#....#..#..##.",
  "......................#........",
  "..###....#.......#.....###.##..",
  "......#......#.......#.....#..#",
  ".....#...#.##...#......#....#..",
  ".....#.....##.............#....",
  "....#......##..#....#.......#..",
  ".##....#..##......###....#..#..",
  "...###.#.............##...#.#..",
  ".....#.....#.....#...#..#.#....",
  "..#.#.....###......#.......#...",
  "..........#.##......#.........#",
  "..##..#.......................#",
  "........#......#............#..",
  "#..#..#..#.#......#..#....#....",
  "...##......#.............#....#",
  "...........#..#..##.......#....",
  ".....#.........#.#..#..........",
  "##...#.......#.#....#..#..#....",
  "#.#.#...........#.##.#.#..###..",
  "#..#...........#.........##....",
  "............#.#..............#.",
  ".#....#....##.#...........#..#.",
  "....#...#..#...#....#....#.....",
  "....#....#...#..#......#.......",
  ".#.#.........#.......#.##......",
  ".#..##...#........#...........#",
  "##...#..#...#...#.....#...#....",
  "....###.#..#.......##.#..#...#.",
  "...##.......####...##.#........",
  "#....#....#.#............#..#..",
  "#.#.#...#...................##.",
  "##......#...........#..........",
  "#..#..#....#.#...#......#......",
  ".##...#.....#...#........#.....",
  "..#............#..............#",
  "###........#..#....#...#......#",
  "###..##......#.##...........#..",
  "........#......#..#.....#......",
  "...#..........#..#...........#.",
  "....#..#..#....#........#....#.",
  ".#.................#####..##..#",
  ".....#...##..#..........#.##...",
  "..#..............#...####......",
  ".....#.##..................#.#.",
  "...#.#..#..#........#..........",
  "...........#....#.#..#.........",
  ".....##.......#......#..#.#.#..",
  "...#.............##...#........",
  "...............#.......##.##.##",
  ".....#........#........#.#..#..",
  "...#..#.........#...##...###...",
  "...#.#.............###.#.....#.",
  ".#..........#......###.#.#.....",
  "....##..##.............###.....",
  "..#..#.#...##...#.......##.....",
  "..........###........#.....#.#.",
  "#.#....#..#..#......#...#...#..",
  ".........#......##.......#.#..#",
  "...#.....#.........##..#..#....",
  ".....##.#..##.##..##...........",
  "...#.#.##....#..#..#......#..#.",
  "#....#....#.............#...##.",
  "#......#..#.####.#.##.#....##..",
  "##.#.#....##..................#",
  ".....##......#.......##.......#",
  "..#......#.#..#...##......##...",
  "..#....##....#.........#..##...",
  ".###.....#....##...........#...",
  ".........#......#.#........#...",
  "...#...#..#.#....######.#..#...",
  "###......#.#.#.........##.#....",
  ".....#...#.........#...#.......",
  "....#.............#.#.........#",
  "..##...#...#.......#......#....",
  ".....#...#.#...#...#..#........",
  ".#......#......................",
  "...###..#..#....#...##.#.......",
  ".#.#.....##...#...#.....#...##.",
  ".....###..###....##............",
  ".....##....#..#.....#.##.......",
  "#........#.........#...#..#....",
  "...#.#.........#..#.......#.#..",
  "....#.#....##.....#..........#.",
  ".#..#....#..#.#..#..#.........#",
  "#...#....#..............#......",
  ".........#.....#.##...##...###.",
  ".....#....##............#..#...",
  ".....#.#...........#..#....#...",
  ".#..........#...#......#.....#.",
  ".#...........#.....#..#........",
  "..............#......##...#..#.",
  "...#.........#..#....#..##...##",
  "..##...#..................#....",
  "#.....#.................#......",
  "...#......#..#..........#.#....",
  "......#..#.....#.....##...#..#.",
  "......#........#..........#....",
  "...##.##....#..##.#..........#.",
  "..........#..#.#.##............",
  "..##........................#..",
  ".....#.#.#......#....#....##...",
  "#....#.........#........#......",
  ".##.......#...#...#........##..",
  "....##......#....#.#..........#",
  "..#.......#..............#.....",
  ".....#......#.#...#..#.#.#....#",
  ".....#..#........#.##.##.......",
  "##........#..........#.........",
  ".....#..##....#.#......###..##.",
  "#.#...##.........#.#.....#..#..",
  "#....#.#...#........#.....#..#.",
  "........................#......",
  "....###......#............#...#",
  "...#..##......#..##.........#..",
  ".............#...#......#..#..#",
  "....#......#....#...........#..",
  "..#.#.####.#.....##........#..#",
  "#..#...#..#..#.......#.#..#....",
  "..#..#..#....#.#.........##..#.",
  ".......#......#.#............#.",
  "...#.............#.#.....#.....",
  "...#.#.........##...#.#.......#",
  "........#...#...........##...#.",
  "..........#....#......#....##..",
  "..........#...........#........",
  "...#..#...#..........#......#..",
  "......#......#....#.....#..#.#.",
  "........##.................#..#",
  ".#........#.#...........#......",
  "#...#........#.#.#.....#.#.#...",
  ".........#........#..#..#....#.",
  "##........#..........#....#..#.",
  ".#.##...........#..#.#..##....#",
  ".......#.#....#..#......#......",
  "..#.....#........##..#......###",
  "..#...#..................#....#",
  "......#...#..#.##.......#......",
  "........#...#.#................",
  ".........#............#........",
  "..#.....##....#.#..##..........",
  "#.....#..........#....#........",
  "....#.#...#...##....#.....##...",
  "..#.#.......#.............#...#",
  "...##..............#......#....",
  "#......#...#................##.",
  ".#.#...#.#..#..................",
  "...##.......#...........#.#.#..",
  "#......#.#.#........#.##...####",
  ".......#..#.#.........#.#.##..#",
  "..............#....#.........#.",
  "...........#.#..#....##......#.",
  "#.............#...##..#.......#",
  ".........#............#...#.##.",
  ".......#.........#.#.....#..#..",
  "........................#.#.##.",
  "#......#.#......#.........#....",
  "...#.......#.......#.....#.....",
  "#..#....#................#...#.",
  "........#.#..##......#.........",
  "#..#...##....##....##.........#",
  ".......#...#...###.............",
  "#.#..#........#.#.#............",
  "#.....#........##.........#.#..",
  ".#..........#....#.#....###....",
  ".#.....#...#.#........#..#.##..",
  "...#.##......#..#.............#",
  "..##..#.#...................#..",
  ".....#....#...#.#...#...#......",
  ".....#..#.#....#.#.............",
  "#.#....#.#.##..###..........#..",
  "........#.#.............#..#...",
  ".........#.......#.............",
  ".##.#............##...#........",
  "......#................#.......",
  "...............#..#...........#",
  "...#.......#...#.##.....#....#.",
  "##..##..#..........#...........",
  ".##.#.......#...#..#...#...#...",
  "....#..#...........#....#.##...",
  ".#........#........#....#......",
  ".......#...#.##.#..#.#..#......",
  ".#..#......#....#...##....#.#..",
  "......#...##.#.....##.###.....#",
  ".#....#..#......#...#.#.....#..",
  "#............#....##...##.##...",
  "#...#.#....#...#.......##...##.",
  "#...........#.##..#....#.....#.",
  "...#..#...#.........#.......#..",
  ".#....#.....#............#.#..#",
  ".#.....#.#...#.#....##......###",
  "..#..#.#.#...#..#.............#",
  "...#...#..#....#........#...##.",
  ".......#.....#...##...........#",
  "#.##.................#...##...#",
  "..............##........#.....#",
  "............#...#..#.......#.#.",
  "#.#.....#.........#...#......#.",
  "#.###..#......#..#..#...#.....#",
  ".....#.......#.................",
  "........#..#......#.#...#......",
  "#.......#..#........#...#..#...",
  "..#...#.......##.............#.",
  "#.......#.......##...#.........",
  ".........#....#.#..##.....#...#",
  "..#.....#.#.......#....#.......",
  "...#.......#.....#..##.#..#....",
  "....#.......#.#.#..............",
  ".#..#......#........#.#..##..##",
  "....#...#.##.#...#....##...#...",
  "#..##..#.....#.......#.........",
  "....#..#..#.#............#.....",
  "#.......##...##..##............",
  "...............................",
  "....#.......#.##...#.....#.#...",
  "...#........#....#.#..#..#.....",
  "##.......#.....##.#.#....#....#",
  "#.............#...........#.##.",
  "#...........#.#..........#.....",
  "#..#....#....#.#.........#.#...",
  "......#.#.#..#.#.#.............",
  "...#.....#........##....#......",
  "..#...#...#.#.......#......#...",
  ".##........#...#..#..........#.",
  "..#...........#..##.....##.....",
  "............#..#.#...#.....#...",
  "..........#....##.......#......",
  "....#....#.................#..#",
  "....#...............#.........#",
  "..#.#...#......#..........##...",
  ".....#...........#.........#..#",
  ".......#.....##.....#.#........",
  ".#.#..........#....#...........",
  ".#..##....#........#....#......",
  "....#.#..#.......#..#.........#",
  "..#....#.....#......#..#.......",
  "......#........#.......#...#.#.",
  ".......#.......#....#.....##...",
  "....##........#..#...#.#..#...#",
  ".#......#...........##....#....",
  "##....##......#.......#.......#",
  ".##....#.##......#.......##..#.",
  "...#..#.#.#.......#..#.###.....",
  "..........##....#..#.##........",
  "...#........###.#..#........#..",
  ".....#....#..##....#.....#....#",
  "#..........#..........#.#....#.",
  "..#....#.....#..............#..",
  "#..................#......#.##.",
  ".#...#.#.....#.........##......",
  "...#...........#.....#......#..",
  "......#.....#.#..##......##....",
  "...#....###..#.....#..#..##..##",
  "......#.......##..#..#.........",
  "#..#.#....#.#..#..........##.#.",
  "..#..#..##..#.#.#.#.....#......",
  "..#.#...#..#.....###.#.........",
  "##.#.#......#........#.####....",
  ".............#..#..#....#......",
  "...##..........#.......#.#....#",
  "..#.....................#......",
  "..#..#...##...#.##........#....",
]