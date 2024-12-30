use thiserror::Error;

pub enum ListElement<Value, Separator> {
    Value(Value),
    Separator(Separator),
}

#[derive(Debug, Error)]
pub enum TrailingSeparatedListError {
    #[error("Consecutive values are not allowed, you must put a separator first")]
    ConsecutiveValues,
    #[error("Consecutive separators are not allowed")]
    ConsecutiveSeparators,
}

pub struct TrailingSeparatedList<Value, Separator> {
    elements: Vec<ListElement<Value, Separator>>,
}

impl<Value, Separator> TrailingSeparatedList<Value, Separator> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push_value(&mut self, value: Value) -> Result<(), TrailingSeparatedListError> {
        match self.elements.last() {
            Some(ListElement::Separator(_)) | None => {}
            Some(ListElement::Value(_)) => {
                return Err(TrailingSeparatedListError::ConsecutiveValues)
            }
        }

        self.elements.push(ListElement::Value(value));
        Ok(())
    }

    pub fn push_separator(
        &mut self,
        separator: Separator,
    ) -> Result<(), TrailingSeparatedListError> {
        match self.elements.last() {
            Some(ListElement::Separator(_)) => {
                return Err(TrailingSeparatedListError::ConsecutiveSeparators)
            }
            Some(ListElement::Value(_)) | None => {}
        }

        self.elements.push(ListElement::Separator(separator));
        Ok(())
    }

    pub fn is_empty_values(&self) -> bool {
        match self.elements.first() {
            None => true,
            Some(ListElement::Separator(_)) => self.elements.len() == 1,
            Some(ListElement::Value(_)) => false,
        }
    }

    pub fn len_values(&self) -> usize {
        match self.elements.first() {
            None => 0,
            Some(ListElement::Separator(_)) => self.elements.len() / 2,
            Some(ListElement::Value(_)) => (self.elements.len() + 1) / 2,
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        let start_index = match self.elements.first() {
            None => 0,
            Some(ListElement::Separator(_)) => 1,
            Some(ListElement::Value(_)) => 0,
        };

        self.elements
            .iter()
            .skip(start_index)
            .step_by(2)
            .map(|element| match element {
                ListElement::Value(value) => value,
                ListElement::Separator(_) => panic!("This shouldn't be possible"),
            })
    }

    pub fn iter_elements(&self) -> impl Iterator<Item = &ListElement<Value, Separator>> {
        self.elements.iter()
    }
}

pub struct StrictSeparatedList<Value, Separator> {
    elements: Vec<ListElement<Value, Separator>>,
}

#[derive(Debug, Error)]
pub enum StrictSeparatedListError {
    #[error("Consecutive values are not allowed, you must put a separator first")]
    ConsecutiveValues,
    #[error("Sequences must start with a value")]
    MustStartWithValue,
}

impl<Value, Separator> StrictSeparatedList<Value, Separator> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push_value(&mut self, value: Value) -> Result<(), StrictSeparatedListError> {
        match self.elements.last() {
            Some(ListElement::Separator(_)) | None => {}
            Some(ListElement::Value(_)) => return Err(StrictSeparatedListError::ConsecutiveValues),
        }

        self.elements.push(ListElement::Value(value));
        Ok(())
    }

    pub fn push_separator_value_pair(
        &mut self,
        separator: Separator,
        value: Value,
    ) -> Result<(), StrictSeparatedListError> {
        match self.elements.last() {
            None => return Err(StrictSeparatedListError::MustStartWithValue),
            Some(ListElement::Separator(_)) => {
                panic!("It shouldn't be possible to push a separator without a value")
            }
            Some(ListElement::Value(_)) => {}
        }

        self.elements.push(ListElement::Separator(separator));
        self.elements.push(ListElement::Value(value));
        Ok(())
    }

    pub fn is_empty_values(&self) -> bool {
        self.elements.len() == 0
    }

    pub fn len_values(&self) -> usize {
        (self.elements.len() + 1) / 2
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.elements
            .iter()
            .step_by(2)
            .map(|element| match element {
                ListElement::Value(value) => value,
                ListElement::Separator(_) => panic!("This shouldn't be possible"),
            })
    }

    pub fn iter_elements(&self) -> impl Iterator<Item = &ListElement<Value, Separator>> {
        self.elements.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_separated_list_len_values() {
        let mut list = TrailingSeparatedList::<i32, char>::new();
        assert_eq!(list.len_values(), 0);

        list.push_value(1).unwrap();
        assert_eq!(list.len_values(), 1);

        list.push_separator(',').unwrap();
        assert_eq!(list.len_values(), 1);

        list.push_value(2).unwrap();
        assert_eq!(list.len_values(), 2);

        list.push_separator(',').unwrap();
        assert_eq!(list.len_values(), 2);

        list.push_value(3).unwrap();
        assert_eq!(list.len_values(), 3);

        // Attempt to push another separator should work
        list.push_separator(',').unwrap();
        assert_eq!(list.len_values(), 3);
    }
}
