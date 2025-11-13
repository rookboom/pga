# 3D Projective Geometric Algebra

This project is a stab at creating a type safe geometric algebra library. It allows one to lean on the compiler while reasoning through geometric computations. For example:

## Joining two points in a line:

```rust
   let p0 = Point3::new(0.0, 0.0, 0.0);
   let p1 = Point3::new(1.0, 0.0, 0.0);
   let line: Line = p0 ^ p1;
```

## Joining three points in a plane:
```rust 
   let p0 = Point3::new(0.0, 0.0, 0.0);
   let p1 = Point3::new(1.0, 0.0, 0.0);
   let p2 = Point3::new(1.0, 1.0, 0.0);
   let line: Plane = p0 ^ p1 ^ p2;
```

## Line and a plane meet in a point or direction when coplanar:
```rust 
   let p0 = Point3::new(0.0, 0.0, 0.0);
   let p1 = Point3::new(1.0, 0.0, 0.0);
   let p2 = Point3::new(0.0, 1.0, 0.0);
   let line: Line = p0 ^ p1;
   let plane: Plane = line ^ p2;
   let point4: Point4 = plane & line;
   let point4: Point4 = plane & line;
   match point4.into() {
      PointOrDirection::Point(p) => {
            println!("Line and plane meet at finite point {:?}", p);
      }
      PointOrDirection::Direction(d) => {
            println!("Line and plane meet at infinite point in direction {:?}", d);
      }
   }
```

See more examples and visualizations [here](https://rookboom.github.io/pga/).
