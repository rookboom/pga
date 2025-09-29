# 3D Projective Geometric Algebra

This project is just some experimentation with geometric algebra with [visualizations](https://<username>.github.io/<repository-name>/) using Bevy.

The library allows you do type safe computations like:

## Joining two points in a line:

```
   let p0 = Point::new(0.0, 0.0, 0.0);
   let p1 = Point::new(1.0, 0.0, 0.0);
   let line: ZeroOr<Line> = p0 & p1;
```

## Joining three points in a plane:
``` 
   let p0 = Point::new(0.0, 0.0, 0.0);
   let p1 = Point::new(1.0, 0.0, 0.0);
   let p2 = Point::new(1.0, 1.0, 0.0);
   let line: ZeroOr<Plane> = p0 & p1 & p2;
```

## Line and a plane meet in a point:
``` 
   let p0 = Point::new(0.0, 0.0, 0.0);
   let p1 = Point::new(1.0, 0.0, 0.0);
   let p2 = Point::new(0.0, 1.0, 0.0);
   let line: Line = (p0 & p1).unwrap();
   let plane: Plane = line & p2;
```