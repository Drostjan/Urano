#![allow(dead_code)]
#![allow(unused_assignments)]

pub mod task;
pub mod colors;

use task::*;
use colors::Color;

pub struct Urano {
    tasks: Vec<Task>,
    raimbow: Color,
}

impl Urano {
    pub fn new() -> Urano {
        Urano {
            tasks: Vec::new(),
            raimbow: Color::new(),
        }
    }

    pub fn add_task(&mut self, task: String){
        self.tasks.push(Task::new(task));
    }

    pub fn list_task(&self){
        println!("{:#?}", self.tasks.clone());
    }

    pub fn remove_task(&mut self,task: String) {
        for t in self.tasks.clone() {
            if t.content == task {
                self.tasks.remove(t.id);
            }
        }
    }

   
}