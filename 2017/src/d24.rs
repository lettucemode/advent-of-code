use std::cmp::max;

pub fn solve(input: String) -> (u32, u32) {
    let mut component_bag: Vec<Component> = vec![Component::new(0, 0)];
    component_bag[0].a_open = false;
    component_bag.append(&mut parse_components(input));
    let mut bridge: Vec<usize> = vec![0usize];
    let mut max_strength: u32 = 0;
    let mut max_strength_with_length: (u32, u32) = (0, 0);

    recurse(
        &mut component_bag,
        &mut bridge,
        &mut max_strength,
        &mut max_strength_with_length,
    );
    (max_strength, max_strength_with_length.0)
}

fn recurse(
    component_bag: &mut Vec<Component>,
    bridge: &mut Vec<usize>,
    max_strength: &mut u32,
    max_strength_with_length: &mut (u32, u32),
) {
    let next_links = next_links(&component_bag, &bridge);
    if next_links.len() == 0 {
        let strength = bridge.iter().fold(0u32, |acc, i| {
            acc + component_bag[*i].a as u32 + component_bag[*i].b as u32
        });
        *max_strength = max(*max_strength, strength);
        if bridge.len() as u32 >= max_strength_with_length.1 {
            max_strength_with_length.0 = max(strength, max_strength_with_length.0);
            max_strength_with_length.1 = bridge.len() as u32;
        }
    } else {
        let curr = bridge[bridge.len() - 1];
        for link in next_links {
            bridge.push(link);
            // arbitrary scopes to get around the multi-mutable reference problem
            {
                let (curr_component, link_component) = mut_two(curr, link, component_bag);
                curr_component.link_to(link_component);
            }
            recurse(
                component_bag,
                bridge,
                max_strength,
                max_strength_with_length,
            );
            {
                let (curr_component, link_component) = mut_two(curr, link, component_bag);
                curr_component.unlink(link_component);
            }
            bridge.pop();
        }
    }
}

// can't get mutable references to two things in a Vec at once in the year of our lord 2020!
// this is extremely stupid. Enough to sour me on the whole language, tbh
fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = if first_index < second_index {
        second_index
    } else {
        first_index
    };
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

fn next_links(component_bag: &Vec<Component>, bridge: &Vec<usize>) -> Vec<usize> {
    let curr = bridge[bridge.len() - 1];
    component_bag
        .iter()
        .enumerate()
        .filter(|&(i, c)| component_bag[curr].could_fit(c) && !bridge.contains(&i))
        .map(|(i, _)| i)
        .collect()
}

fn parse_components(input: String) -> Vec<Component> {
    let mut components = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split("/").collect();
        components.push(Component::new(
            parts[0].parse::<u8>().unwrap(),
            parts[1].parse::<u8>().unwrap(),
        ));
    }

    components
}

#[derive(Debug, Clone)]
struct Component {
    a: u8,
    b: u8,
    a_open: bool,
    b_open: bool,
}

impl Component {
    fn new(a: u8, b: u8) -> Component {
        Component {
            a,
            b,
            a_open: true,
            b_open: true,
        }
    }

    fn link_to(&mut self, c: &mut Component) {
        if self.a_open && c.a_open && self.a == c.a {
            self.a_open = false;
            c.a_open = false;
        } else if self.a_open && c.b_open && self.a == c.b {
            self.a_open = false;
            c.b_open = false;
        } else if self.b_open && c.a_open && self.b == c.a {
            self.b_open = false;
            c.a_open = false;
        } else if self.b_open && c.b_open && self.b == c.b {
            self.b_open = false;
            c.b_open = false;
        }
    }

    fn unlink(&mut self, c: &mut Component) {
        if !self.a_open && !c.a_open && self.a == c.a {
            self.a_open = true;
            c.a_open = true;
        } else if !self.a_open && !c.b_open && self.a == c.b {
            self.a_open = true;
            c.b_open = true;
        } else if !self.b_open && !c.a_open && self.b == c.a {
            self.b_open = true;
            c.a_open = true;
        } else if !self.b_open && !c.b_open && self.b == c.b {
            self.b_open = true;
            c.b_open = true;
        }
    }

    fn could_fit(&self, c: &Component) -> bool {
        (self.a_open && c.a_open && self.a == c.a)
            || (self.a_open && c.b_open && self.a == c.b)
            || (self.b_open && c.a_open && self.b == c.a)
            || (self.b_open && c.b_open && self.b == c.b)
    }
}
