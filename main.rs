use std::collections::HashSet;
use std::convert::TryInto;


///Возвращает сумму цифр положительного числа
///Для получения суммы цифр любого числа можно добавить модуль
fn cifers_sum(number: i32) -> i32 {
    if number == 0 {
        return 0;
    }
    return number % 10 + cifers_sum(number / 10);
}


///Может ли муравей зайти в эту точку
fn point_can_be_visited(point: Point) -> bool {
    let x_cifers_sum = cifers_sum(point.x_coordinate);
    let y_cifers_sum = cifers_sum(point.y_coordinate);
    return x_cifers_sum + y_cifers_sum <= 25;
}


///Возвращает четырех "соседей" точки
fn point_neighbours(point: Point) -> [Point; 4] {
    return [
        Point{x_coordinate: point.x_coordinate + 1, y_coordinate: point.y_coordinate},
        Point{x_coordinate: point.x_coordinate - 1, y_coordinate: point.y_coordinate},
        Point{x_coordinate: point.x_coordinate, y_coordinate: point.y_coordinate + 1},
        Point{x_coordinate: point.x_coordinate, y_coordinate: point.y_coordinate - 1},
    ];
}


#[derive(Hash, Eq, Debug)]
struct Point {
    x_coordinate: i32,
    y_coordinate: i32,
}

impl Copy for Point { }

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x_coordinate == other.x_coordinate && self.y_coordinate==other.y_coordinate;
    }
}


///Находит количество точек, которые может посетить муравей, начав движение в start_point
///Общее решение, подходит для любой точки с положительными координатами
///Не использует знание о конкретной начальной точке
fn find_count_of_visited(start_point: Point) -> i32 { 
    let mut queue: Vec<Point> = Vec::new();
    queue.push(start_point);
    let mut visited: HashSet<Point> = HashSet::new();
    
    while !queue.is_empty() {
        let point = queue.pop().unwrap();
        if point_can_be_visited(point) && !visited.contains(&point) {
            let neighbours = point_neighbours(point);
            queue.extend_from_slice(&neighbours);
            visited.insert(point);
        }
    }

    return visited.len().try_into().unwrap();
}


fn main() {
    let start_point = Point{
        x_coordinate: 1000,
        y_coordinate: 1000,
    };

    let count = find_count_of_visited(start_point);
    
    println!(
        "Муравей может посетить {} клеток, если он начал в клетке ({}, {})",
        count,
        start_point.x_coordinate,
        start_point.y_coordinate,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    ///Arrange: Точка с координатами, сумма цифр которых равна 26
    ///Act: Для этой точки вызывается point_can_be_visited
    ///Assert: Точка не может быть посещена, так как сумма больше 25
    fn test_can_not_visit_point_26() {
        let point = crate::Point{x_coordinate: 8882, y_coordinate: 0};

        assert_eq!(crate::point_can_be_visited(point), false);
    }

    #[test]
    ///Arrange: Точка с координатами, сумма цифр которых равна 25
    ///Act: Для этой точки вызывается point_can_be_visited
    ///Assert: Точка не может быть посещена, так как сумма не больше 25
    fn test_can_visit_point_25() {
        let point = crate::Point{x_coordinate: 8881, y_coordinate: 0};

        assert_eq!(crate::point_can_be_visited(point), true);
    }

    #[test]
    ///Arrange: Точка с координатами, сумма цифр которых равна 24
    ///Act: Для этой точки вызывается point_can_be_visited
    ///Assert: Точка не может быть посещена, так как сумма меньше 25
    fn test_can_visit_point_24() {
        let point = crate::Point{x_coordinate: 888, y_coordinate: 0};

        assert_eq!(crate::point_can_be_visited(point), true);
    }

    #[test]
    ///Arrange: Точка в середине сетки координат
    ///Act: Для этой точки вызывается point_neighbours
    ///Assert: Соседи точки - точки слева, справа, сверху, снизу
    fn test_get_neighbours() {
        let point = crate::Point{x_coordinate: 0, y_coordinate: 0};

        let expected_neighbours = [
            crate::Point{x_coordinate: 1, y_coordinate: 0},
            crate::Point{x_coordinate: -1, y_coordinate: 0},
            crate::Point{x_coordinate: 0, y_coordinate: 1},
            crate::Point{x_coordinate: 0, y_coordinate: -1},
        ];

        assert_eq!(crate::point_neighbours(point), expected_neighbours);
    }

    #[test]
    ///Arrange: Точка с координатами (1000, 1000)
    ///Act: Для этой точки вызывается find_count_of_visited
    ///Assert: Из точки (1000, 1000) муравей может посетить 148848 клеток
    fn test_visited_count_for_1000() {
        let start_point = crate::Point{
            x_coordinate: 1000,
            y_coordinate: 1000,
        };
    
        let count = crate::find_count_of_visited(start_point);

        assert_eq!(count, 148848);
    }
}
