use std::{collections::VecDeque, vec};

#[derive(Debug)]
pub struct Room {
    pub room_number: u32,
    pub details: String,
}

impl Room {
    fn new(room_number: u32, details: String) -> Self {
        Room {
            room_number,
            details,
        }
    }
}

#[derive(Debug)]
pub struct QueueItem<'a> {
    pub queue_number: u32,
    pub room: &'a Room,
}

impl<'a> QueueItem<'a> {
    fn new(queue_number: u32, room: &'a Room) -> Self {
        QueueItem { queue_number, room }
    }
}

#[derive(Debug, Default)]
pub struct Queue<'a> {
    pub rooms: Vec<Room>,
    pub current_number: u32,
    pub items: VecDeque<QueueItem<'a>>,
}

impl<'a> Queue<'a> {
    pub fn new() -> Self {
        Queue {
            rooms: vec![],
            current_number: 0,
            items: VecDeque::new(),
        }
    }

    pub fn add_room(&mut self, room_number: u32, details: String) -> &Self {
        let room_exists = self
            .rooms
            .iter()
            .any(|room| room.room_number == room_number);

        if !room_exists {
            let room = Room::new(room_number, details);
            self.rooms.push(room);
        } else {
            println!("Room number already exists.")
        }

        self
    }

    pub fn increment(&'a mut self, room_number: u32) -> &Self {
        if let Some(room) = self
            .rooms
            .iter()
            .find(|room| room.room_number == room_number)
        {
            self.current_number += 1;
            let q_item = QueueItem::new(self.current_number, room);
            self.items.push_back(q_item);
        } else {
            println!("Room not found.");
        }

        self
    }
}
