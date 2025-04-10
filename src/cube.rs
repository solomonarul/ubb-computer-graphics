pub const VERTICES: [glam::Vec4; 8]  = [
    // Front face
    glam::vec4( 0.5,  0.5,  0.5, 1.0), // 0: front top right
    glam::vec4(-0.5,  0.5,  0.5, 1.0), // 1: front top left
    glam::vec4(-0.5, -0.5,  0.5, 1.0), // 2: front bottom left
    glam::vec4( 0.5, -0.5,  0.5, 1.0), // 3: front bottom right
    
    // Back face
    glam::vec4( 0.5,  0.5, -0.5, 1.0), // 4: back top right
    glam::vec4(-0.5,  0.5, -0.5, 1.0), // 5: back top left
    glam::vec4(-0.5, -0.5, -0.5, 1.0), // 6: back bottom left
    glam::vec4( 0.5, -0.5, -0.5, 1.0), // 7: back bottom right
];

pub const INDICES: [u16; 36] = [
    // Front face
    0, 1, 2,
    0, 2, 3,
    
    // Back face
    4, 6, 5,
    4, 7, 6,
    
    // Top face
    0, 5, 1,
    0, 4, 5,
    
    // Bottom face
    3, 2, 6,
    3, 6, 7,
    
    // Right face
    0, 3, 7,
    0, 7, 4,
    
    // Left face
    1, 6, 2,
    1, 5, 6,
];