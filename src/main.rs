use std::fmt;
use chrono::Datelike;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_cost_is_0() {
        assert_eq!(PizzaOrder::new_order().total(), 0.0);
    }

    #[test]
    fn test_two_pepperoni_costs_20() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 20.0);
    }

    #[test]
    fn test_zero_pepperoni_costs_0() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Pepperoni, 0);
        assert_eq!(order.total(), 0.0);
    }

    #[test]
    fn test_10_pepperoni_costs_95() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Pepperoni, 10);
        assert_eq!(order.total(), 95.0);
    }

    #[test]
    fn test_three_brie_costs_45() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::BrieChickenAndMushroom, 3);
        assert_eq!(order.total(), 45.0);
    }

    #[test]
    fn test_three_brie_added_separately_costs_45() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        assert_eq!(order.total(), 45.0);
    }

    #[test]
    fn test_one_mighty_veg_costs_12() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::MightyVeg, 1);
        assert_eq!(order.total(), 12.0);
    }

    #[test]
    fn test_two_pepperoni_costs_18_on_mondays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Mon);
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 18.0);
    }

    #[test]
    fn test_two_pepperoni_costs_20_on_tuesdays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Tue);
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 20.0);
    }

    #[test]
    fn test_one_pepperoni_costs_10_point_5_on_sundays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Sun);
        order.add(PizzaType::Pepperoni, 1);
        assert_eq!(order.total(), 10.5);
    }

    #[test]
    fn test_one_brie_costs_15_on_mondays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Mon);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        assert_eq!(order.total(), 15.0);
    }

    #[test]
    fn test_receipt_works() {
        // Why would anyone want anything other than a Brie and Veg Pepperoni sandwhich?
        let mut order = PizzaOrder::new_order();
        order.add(PizzaType::Pepperoni, 1);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        order.add(PizzaType::MightyVeg, 1);
        order.add(PizzaType::Pepperoni, 1);
        order.get_receipt();
    }
}

#[derive(PartialEq, Clone, Copy)]
enum PizzaType {
    Pepperoni,
    BrieChickenAndMushroom,
    MightyVeg,
}

impl fmt::Display for PizzaType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            PizzaType::Pepperoni => {
                write!(f, "Pepperoni")
            }
            PizzaType::BrieChickenAndMushroom => {
                write!(f, "Brie, Chicken and Mushroom")
            }
            PizzaType::MightyVeg => {
                write!(f, "Mighty Veg")
            }
        }
    }
}

#[derive(Clone, Copy)]
struct OrderLine {
    pizza_type: PizzaType,
    quantity: i64,
}

impl fmt::Display for OrderLine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.quantity, self.pizza_type)
    }
}

struct PizzaOrder {
    running_total: f64,
    current_day: chrono::Weekday,
    total_pizzas: i64,
    order_lines: Vec<OrderLine>,
}

fn get_daily_discount(current_day: chrono::Weekday) -> f64 {
    match current_day {
        chrono::Weekday::Mon => 0.9,
        chrono::Weekday::Tue => 1.0,
        chrono::Weekday::Wed => 1.0,
        chrono::Weekday::Thu => 1.0,
        chrono::Weekday::Fri => 1.0,
        chrono::Weekday::Sat => 1.0,
        chrono::Weekday::Sun => 1.05,
    }
}

impl PizzaType {
    fn get_cost(&self) -> f64 {
        match self {
            PizzaType::Pepperoni => 10.0,
            PizzaType::BrieChickenAndMushroom => 15.0,
            PizzaType::MightyVeg => 12.0,
        }
    }
}

// Implementation block, all `PizzaOrder` associated functions & methods go in here
impl PizzaOrder {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, PizzaOrder.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn new_order() -> PizzaOrder {
        let current_time = chrono::offset::Local::now();
        let today = current_time.date().weekday();
        return PizzaOrder {
            running_total: 0.0,
            current_day: today,
            total_pizzas: 0,
            order_lines: vec![],
        };
    }

    fn new_dateless_order() -> PizzaOrder {
        return PizzaOrder {
            running_total: 0.0,
            current_day: chrono::Weekday::Tue,
            total_pizzas: 0,
            order_lines: vec![],
        };
    }

    fn set_day(&mut self, new_current_day: chrono::Weekday) -> () {
        self.current_day = new_current_day
    }

    fn bonus_discount_multiplier(&self) -> f64 {
        if self.total_pizzas > 5 {
            return 0.95;
        }
        return 1.0;
    }

    fn total(&self) -> f64 {
        return self.running_total * self.bonus_discount_multiplier();
    }

    fn get_order_line_cost(&self, order_line: &OrderLine) -> f64 {
        let pizza_cost = order_line.pizza_type.get_cost();
        let discount_multiplier: f64;
        if self.current_day == chrono::Weekday::Mon {
            match order_line.pizza_type {
                PizzaType::BrieChickenAndMushroom => {
                    discount_multiplier = 1.0;
                }
                _ => {
                    discount_multiplier = get_daily_discount(self.current_day);
                }
            }
        } else {
            discount_multiplier = get_daily_discount(self.current_day);
        }
        return pizza_cost * discount_multiplier * order_line.quantity as f64;
    }

    // Another associated function, taking two arguments:
    fn add(&mut self, pizza_type: PizzaType, quantity: i64) -> () {
        let new_order_line = OrderLine {
            pizza_type,
            quantity,
        };
        self.order_lines.push(new_order_line);
        self.running_total += self.get_order_line_cost(&new_order_line);
        self.total_pizzas += quantity;
    }

    fn get_receipt(&self) -> () {
        for order_line in &self.order_lines {
            println!("{}: ${}", order_line, self.get_order_line_cost(&order_line));
        }
    }
}
