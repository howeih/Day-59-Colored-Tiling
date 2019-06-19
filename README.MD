Day 59: Colored Tiling
<br>
We have a stock of tiles with colored edges at our disposal and there’s a wall we need to tile up. However, there are two catches. The stock is limited and if two tiles touch each other, their edges must match on color.
<br>
![image]("images/ex1.png")
run:
```

fn main() {
    let mut wall = Wall::new(12, 12);
    wall.solve(1, 1);
    wall.print_wall();

}

```

result:
```
U U G U G U B U G U B U 
B G B G B B B G B B B U 
U B B B B B B B B B B B 
G B B B B B B B B B G U 
U G B B B B B B B B B G 
G B B B B B B B B B G U 
U G B B B B B B B B B G 
G B B B B B B B B B B U 
U G B B B B B B B B B B 
B B B B B B B B B B G U 
U B B B G B G B G B B G 
U B U G U G U G U B U U 


```