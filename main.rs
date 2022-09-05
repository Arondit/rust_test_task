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
        Point{
            x_coordinate: point.x_coordinate + 1, 
            y_coordinate: point.y_coordinate + 1,
        },
        Point{
            x_coordinate: point.x_coordinate + 1, 
            y_coordinate: point.y_coordinate - 1,
        },
        Point{
            x_coordinate: point.x_coordinate - 1, 
            y_coordinate: point.y_coordinate + 1,
        },
        Point{
            x_coordinate: point.x_coordinate - 1, 
            y_coordinate: point.y_coordinate - 1,
        },
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
