use crate::gridmath::*;
use rand::Rng;

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct World {
    particles: [Particle; WORLD_WIDTH as usize * WORLD_HEIGHT as usize],
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParticleType {
    Boundary,
    Air,
    Sand,
    Water,
    Stone,
}

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub particle_type: ParticleType
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Particle{particle_type}
    }
}

impl Default for Particle {
    fn default() -> Self { Particle{particle_type: ParticleType::Air} }
}

impl World {
    pub fn new() -> Self {
        let created: World = World {
            particles: [Particle::default(); WORLD_WIDTH as usize * WORLD_HEIGHT as usize],
        };

        return created;
    }

    pub fn contains(&self, pos: GridVec) -> bool {
        return pos.x >= 0 && pos.x < WORLD_WIDTH && pos.y >= 0 && pos.y < WORLD_HEIGHT;
    }

    pub fn clamp(&self, pos: GridVec) -> GridVec {
        if self.contains(pos) { return pos; }

        let mut modified = pos;
        if pos.x < 0 { modified.x = 0; }
        else if pos.x >= WORLD_WIDTH { modified.x = WORLD_WIDTH - 1; }
        if pos.y < 0 { modified.y = 0; }
        else if pos.y >= WORLD_HEIGHT { modified.y = WORLD_HEIGHT - 1; }

        return modified;
    }

    fn get_index(pos: GridVec) -> usize {
        return pos.y as usize * WORLD_WIDTH as usize + pos.x as usize;
    }

    pub fn get_particle(&self, pos: GridVec) -> Particle {
        if pos.x <= 0 || pos.x >= WORLD_WIDTH - 2 || pos.y <= 0 || pos.y >= WORLD_HEIGHT - 2 {
            return Particle { particle_type: ParticleType::Boundary };
        }
        return self.particles[World::get_index(pos)];
    }

    pub fn replace_particle(&mut self, pos: GridVec, new_val: Particle) {
        if pos.x <= 0 || pos.x >= WORLD_WIDTH - 2 || pos.y <= 0 || pos.y >= WORLD_HEIGHT - 2 {
            return;
        }
        self.particles[World::get_index(pos)] = new_val;
    }

    pub fn add_particle(&mut self, pos: GridVec, new_val: Particle) {
        if pos.x <= 0 || pos.x >= WORLD_WIDTH - 2 || pos.y <= 0 || pos.y >= WORLD_HEIGHT - 2 {
            return;
        }
        if self.get_particle(pos).particle_type == ParticleType::Air {
            self.particles[World::get_index(pos)] = new_val;
        }
    }

    pub fn clear_circle(&mut self, pos: GridVec, radius: i32) {
        self.place_circle(pos, radius, Particle{particle_type:ParticleType::Air}, true);
    }

    pub fn place_circle(&mut self, pos: GridVec, radius: i32, new_val: Particle, replace: bool) {
        let left = self.clamp(pos + GridVec::new(-radius, 0));
        let right = self.clamp(pos + GridVec::new(radius, 0));
        let bottom = self.clamp(pos + GridVec::new(0, -radius));
        let top = self.clamp(pos + GridVec::new(0, radius));

        for y in bottom.y..top.y {
            for x in left.x..right.x {
                if replace { self.replace_particle(GridVec{x, y}, new_val.clone()); }
                else { self.add_particle(GridVec{x, y}, new_val.clone()); }
            }
        }
    }

    fn test_vec(&self, base_pos: GridVec, test_vec: GridVec, replace_water: bool) -> bool {
        let test_pos = base_pos + test_vec;
        if !self.contains(test_pos) { return false; }

        let material_at_test = self.get_particle(test_pos).particle_type;

        if material_at_test == ParticleType::Air { return true; }
        else if replace_water && material_at_test == ParticleType::Water { return true; }
        return false;
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..WORLD_HEIGHT {
            // flip processing order for a random half of rows
            let flip = rng.gen_bool(0.5);
            for mut x in 0..WORLD_WIDTH {
                if flip { x = WORLD_WIDTH - x - 1; }

                let base_pos = GridVec{x, y};
                let cur_part = self.get_particle(base_pos);
                if cur_part.particle_type == ParticleType::Sand {
                    if y >= 1 {
                        let available_moves = vec![GridVec{x: 1, y: -1}, GridVec{x: -1, y: -1}, GridVec{x: 0, y: -1}];
                        let mut possible_moves = Vec::<GridVec>::new();
                        
                        for vec in available_moves {
                            if self.test_vec(base_pos, vec, true) {
                                possible_moves.push(vec.clone());
                            }
                        }

                        if possible_moves.len() > 0 {
                            let chosen_vec = possible_moves[rng.gen_range(0..possible_moves.len())];
                            let chosen_pos = base_pos + chosen_vec;
                            self.replace_particle(base_pos, self.get_particle(chosen_pos));
                            self.replace_particle(chosen_pos, cur_part);
                        }
                    }
                }
                else if cur_part.particle_type == ParticleType::Water {
                    let available_moves = vec![GridVec{x: 1, y: -1}, GridVec{x: 0, y: -1}, GridVec{x: -1, y: -1}, GridVec{x: 0, y: -2} ];
                    let mut possible_moves = Vec::<GridVec>::new();
                        
                    for vec in available_moves {
                        if self.test_vec(base_pos, vec, false) {
                            possible_moves.push(vec.clone());
                        }
                    }

                    if possible_moves.len() <= 1 {
                        let available_moves_2 = vec![ GridVec{x: 1, y: 0}, GridVec{x: -1, y: 0}, GridVec{x: 2, y: 0}, GridVec{x: -2, y: 0}, GridVec{x: 3, y: 0}, GridVec{x: -3, y: 0} ];
                        for vec in available_moves_2 {
                            if self.test_vec(base_pos, vec, false) {
                                possible_moves.push(vec.clone());
                            }
                        }
                    }

                    if possible_moves.len() > 0 {
                        let chosen_vec = possible_moves[rng.gen_range(0..possible_moves.len())];
                        let chosen_pos = base_pos + chosen_vec;
                        self.replace_particle(base_pos, self.get_particle(chosen_pos));
                        self.replace_particle(chosen_pos, cur_part);
                    }
                }
            }
        }
    }
}