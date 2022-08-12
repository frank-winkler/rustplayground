use crate::utils::array as custom_array;

#[derive(Debug)]
pub struct Dish {
    name : String,
}
impl Dish {
    pub fn new(name: String) -> Self {
        Dish {
            name : name
        }
    }

    pub fn call_ready() {
        // Notify the frontofhouse about the dish being ready
    }
}

#[derive(Debug, PartialEq, Eq)]
enum IngredientState {
    Raw(),
    Cut(i32),
    Sliced(i32),
    Cubed(i32),
    Fried(i16),
}
// Define cooking preparation activities
pub trait Slicing {
    fn set_state(&mut self, state: IngredientState);

    fn slice(&mut self, thickness: i32) {
        self.set_state(IngredientState::Sliced(thickness))
    }
}

pub trait Frying {
    fn set_state(&mut self, state: IngredientState);

    fn fry(&mut self, duration: i16) {
        self.set_state(IngredientState::Fried(duration))
    }
}

// STRUCTS -----------
#[derive(Debug)]
pub struct Ingredient {
    name: String,
    state: Vec<IngredientState>
}
impl PartialEq for Ingredient {
    fn eq(&self, other: &Ingredient) -> bool {
        self.name == other.name
    }
}
impl Slicing for Ingredient {
    fn set_state(&mut self, state: IngredientState) {
        if ! self.state.contains(&state) {
            self.state.push(state)
        }
    }
}
impl Frying for Ingredient {
    fn set_state(&mut self, state: IngredientState) {
        if self.state.contains(&IngredientState::Raw()) {
            let index = self.state.iter().position(
                |x| *x == IngredientState::Raw()
            ).unwrap();
            self.state.remove(index);
        }
        if ! self.state.contains(&state) {
            self.state.push(state)
        }
    }
}
impl Ingredient {
    pub fn new(name: String) -> Self {
        Ingredient {
            name: name,
            state: vec![IngredientState::Raw()]
        }
    }
}

