use crate::{polygon, Coordinate, CoordinateType, Polygon};
use num_traits::Float;

/// An _axis-aligned_ bounded 2D rectangle whose area is
/// defined by minimum and maximum `Coordinate`s.
///
/// The constructors and setters ensure the maximum
/// `Coordinate` is greater than or equal to the minimum.
/// Thus, a `Rect`s width, height, and area is guaranteed to
/// be greater than or equal to zero.
///
/// **Note.** While `Rect` implements `MapCoords` and
/// `RotatePoint` algorithmic traits, the usage is expected
/// to maintain the axis alignment. In particular, only
/// rotation by integer multiples of 90 degrees, will
/// preserve the original shape. In other cases, the min,
/// and max points are rotated or transformed, and a new
/// rectangle is created (with coordinate swaps to ensure
/// min < max).
///
/// # Examples
///
/// ```
/// use geo_types::{Coordinate, Rect};
///
/// let rect = Rect::new(
///     Coordinate { x: 0., y: 4.},
///     Coordinate { x: 3., y: 10.},
/// );
///
/// assert_eq!(3., rect.width());
/// assert_eq!(6., rect.height());
/// assert_eq!(
///     Coordinate { x: 1.5, y: 7. },
///     rect.center()
/// );
/// ```
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T>
where
    T: CoordinateType,
{
    min: Coordinate<T>,
    max: Coordinate<T>,
}

impl<T: CoordinateType> Rect<T> {
    /// Constructor to creates a new rectangle from coordinates, where `min` denotes to the
    /// coordinates of the bottom-left corner, and `max` denotes to the coordinates of the
    /// top-right corner
    ///
    /// # Panics
    ///
    /// Panics if `min`'s x/y coordinate is larger than that of the `max`'s.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{Coordinate, Rect};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 0., y: 0. },
    ///     Coordinate { x: 10., y: 20. }
    /// );
    /// ```
    pub fn new<C>(c1: C, c2: C) -> Rect<T>
    where
        C: Into<Coordinate<T>>,
    {
        let c1 = c1.into();
        let c2 = c2.into();
        let (min_x, max_x) = if c1.x < c2.x {
            (c1.x, c2.x)
        } else {
            (c2.x, c1.x)
        };
        let (min_y, max_y) = if c1.y < c2.y {
            (c1.y, c2.y)
        } else {
            (c2.y, c1.y)
        };
        Rect {
            min: Coordinate { x: min_x, y: min_y },
            max: Coordinate { x: max_x, y: max_y },
        }
    }

    #[deprecated(
        since = "0.6.2",
        note = "Use `Rect::new` instead, since `Rect::try_new` will never Error"
    )]
    pub fn try_new<C>(c1: C, c2: C) -> Result<Rect<T>, InvalidRectCoordinatesError>
    where
        C: Into<Coordinate<T>>,
    {
        Ok(Rect::new(c1, c2))
    }

    /// Returns the minimum `Coordinate` (bottom-left corner) of the `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geo_types::{Coordinate, Rect};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 5., y: 5. },
    ///     Coordinate { x: 15., y: 15. },
    /// );
    ///
    /// assert_eq!(rect.min(), Coordinate { x: 5., y: 5. });
    /// ```
    pub fn min(self) -> Coordinate<T> {
        self.min
    }

    /// Set the `Rect`’s minimum coordinate.
    ///
    /// # Panics
    ///
    /// Panics if `min`’s x/y is greater than the maximum coordinate’s x/y.
    pub fn set_min<C>(&mut self, min: C)
    where
        C: Into<Coordinate<T>>,
    {
        self.min = min.into();
        self.assert_valid_bounds();
    }

    /// Returns the maximum `Coordinate` (top-right corner) of the `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geo_types::{Coordinate, Rect};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 5., y: 5. },
    ///     Coordinate { x: 15., y: 15. },
    /// );
    ///
    /// assert_eq!(rect.max(), Coordinate { x: 15., y: 15. });
    /// ```
    pub fn max(self) -> Coordinate<T> {
        self.max
    }

    /// Set the `Rect`’s maximum coordinate.
    ///
    /// # Panics
    ///
    /// Panics if `max`’s x/y is less than the minimum coordinate’s x/y.
    pub fn set_max<C>(&mut self, max: C)
    where
        C: Into<Coordinate<T>>,
    {
        self.max = max.into();
        self.assert_valid_bounds();
    }

    /// Returns the width of the `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geo_types::{Coordinate, Rect};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 5., y: 5. },
    ///     Coordinate { x: 15., y: 15. },
    /// );
    ///
    /// assert_eq!(rect.width(), 10.);
    /// ```
    pub fn width(self) -> T {
        self.max().x - self.min().x
    }

    /// Returns the height of the `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geo_types::{Coordinate, Rect};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 5., y: 5. },
    ///     Coordinate { x: 15., y: 15. },
    /// );
    ///
    /// assert_eq!(rect.height(), 10.);
    /// ```
    pub fn height(self) -> T {
        self.max().y - self.min().y
    }

    /// Create a `Polygon` from the `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geo_types::{Coordinate, Rect, polygon};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 0., y: 0. },
    ///     Coordinate { x: 10., y: 20. },
    /// );
    ///
    /// assert_eq!(
    ///     rect.to_polygon(),
    ///     polygon![
    ///         (x: 0., y: 0.),
    ///         (x: 0., y: 20.),
    ///         (x: 10., y: 20.),
    ///         (x: 10., y: 0.),
    ///         (x: 0., y: 0.),
    ///     ],
    /// );
    /// ```
    pub fn to_polygon(self) -> Polygon<T> {
        polygon![
            (x: self.min.x, y: self.min.y),
            (x: self.min.x, y: self.max.y),
            (x: self.max.x, y: self.max.y),
            (x: self.max.x, y: self.min.y),
            (x: self.min.x, y: self.min.y),
        ]
    }

    fn assert_valid_bounds(&self) {
        if !self.has_valid_bounds() {
            panic!(RECT_INVALID_BOUNDS_ERROR);
        }
    }

    fn has_valid_bounds(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y
    }
}

impl<T: CoordinateType + Float> Rect<T> {
    /// Returns the center `Coordinate` of the `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geo_types::{Coordinate, Rect};
    ///
    /// let rect = Rect::new(
    ///     Coordinate { x: 5., y: 5. },
    ///     Coordinate { x: 15., y: 15. },
    /// );
    ///
    /// assert_eq!(
    ///     rect.center(),
    ///     Coordinate { x: 10., y: 10. }
    /// );
    /// ```
    pub fn center(self) -> Coordinate<T> {
        let two = T::one() + T::one();
        (
            (self.max.x + self.min.x) / two,
            (self.max.y + self.min.y) / two,
        )
            .into()
    }
}

static RECT_INVALID_BOUNDS_ERROR: &str = "Failed to create Rect: 'min' coordinate's x/y value must be smaller or equal to the 'max' x/y value";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvalidRectCoordinatesError;

impl std::error::Error for InvalidRectCoordinatesError {}

impl std::fmt::Display for InvalidRectCoordinatesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", RECT_INVALID_BOUNDS_ERROR)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Coordinate;

    #[test]
    fn rect() {
        let rect = Rect::new((10, 10), (20, 20));
        assert_eq!(rect.min, Coordinate { x: 10, y: 10 });
        assert_eq!(rect.max, Coordinate { x: 20, y: 20 });

        let rect = Rect::new((20, 20), (10, 10));
        assert_eq!(rect.min, Coordinate { x: 10, y: 10 });
        assert_eq!(rect.max, Coordinate { x: 20, y: 20 });

        let rect = Rect::new((10, 20), (20, 10));
        assert_eq!(rect.min, Coordinate { x: 10, y: 10 });
        assert_eq!(rect.max, Coordinate { x: 20, y: 20 });
    }

    #[test]
    fn rect_width() {
        let rect = Rect::new((10, 10), (20, 20));
        assert_eq!(rect.width(), 10);
    }

    #[test]
    fn rect_height() {
        let rect = Rect::new((10., 10.), (20., 20.));
        assert_relative_eq!(rect.height(), 10.);
    }

    #[test]
    fn rect_center() {
        assert_relative_eq!(
            Rect::new((0., 10.), (10., 90.)).center(),
            Coordinate::from((5., 50.))
        );
        assert_relative_eq!(
            Rect::new((-42., -42.), (42., 42.)).center(),
            Coordinate::from((0., 0.))
        );
        assert_relative_eq!(
            Rect::new((0., 0.), (0., 0.)).center(),
            Coordinate::from((0., 0.))
        );
    }
}
