# 3D Projective Geometric Algebra

This project is a stab at creating a type safe geometric algebra library. It allows one to lean on the compiler while reasoning through geometric computations. For example:

## Joining two points in a line:

```
   let p0 = Point3::new(0.0, 0.0, 0.0);
   let p1 = Point3::new(1.0, 0.0, 0.0);
   let line: Line = p0 ^ p1;
```

## Joining three points in a plane:
``` 
   let p0 = Point3::new(0.0, 0.0, 0.0);
   let p1 = Point3::new(1.0, 0.0, 0.0);
   let p2 = Point3::new(1.0, 1.0, 0.0);
   let line: Plane = p0 ^ p1 ^ p2;
```

## Line and a plane meet in a point or direction when coplanar:
``` 
   let p0 = Point3::new(0.0, 0.0, 0.0);
   let p1 = Point3::new(1.0, 0.0, 0.0);
   let p2 = Point3::new(0.0, 1.0, 0.0);
   let line: Line = p0 ^ p1;
   let plane: Plane = line ^ p2;
   let point4: Point4 = plane & line;
   let p: PointOrDirection = point4.into();
```

See visualizations [here](https://rookboom.github.io/pga/).
